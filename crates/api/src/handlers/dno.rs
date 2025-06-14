use axum::{
    extract::State,
    response::Json,
};
use serde_json::{json, Value};
use shared::{DnoQueryRequest, DnoQueryResponse, DnoQueryMetadata, AppError};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;
use crate::AppState;

pub async fn query_dno_data(
    State(state): State<AppState>,
    Json(request): Json<DnoQueryRequest>,
) -> Result<Json<Value>, Json<Value>> {
    let start_time = std::time::Instant::now();
    
    tracing::info!("Processing DNO query: {}", request.query);
    
    match process_dno_query(&state, &request).await {
        Ok(response) => {
            let processing_time = start_time.elapsed().as_millis() as i32;
            Ok(Json(json!({
                "success": true,
                "data": response,
                "metadata": {
                    "processing_time_ms": processing_time,
                    "timestamp": Utc::now(),
                    "api_version": "2.5"
                }
            })))
        },
        Err(err) => {
            tracing::error!("DNO query failed: {}", err);
            Err(Json(json!({
                "success": false,
                "error": {
                    "code": "DNO_QUERY_FAILED",
                    "message": format!("DNO query failed: {}", err)
                }
            })))
        }
    }
}

pub async fn analyze_pdf(
    State(state): State<AppState>,
    Json(request): Json<shared::PdfAnalysisRequest>,
) -> Result<Json<Value>, Json<Value>> {
    tracing::info!("Analyzing PDF: {}", request.file_path);
    
    match state.pdf_service.analyze_pdf(request).await {
        Ok(response) => {
            Ok(Json(json!({
                "success": true,
                "data": response
            })))
        },
        Err(err) => {
            tracing::error!("PDF analysis failed: {}", err);
            Err(Json(json!({
                "success": false,
                "error": {
                    "code": "PDF_ANALYSIS_FAILED",
                    "message": format!("PDF analysis failed: {}", err)
                }
            })))
        }
    }
}

async fn process_dno_query(
    state: &AppState,
    request: &DnoQueryRequest,
) -> Result<DnoQueryResponse, AppError> {
    let start_time = std::time::Instant::now();
    let mut cache_hits = 0;

    // Parse the query to extract DNO and years
    let parse_result = state.ai_service.parse_dno_query(&request.query).await?;
    
    tracing::info!("Parsed DNO query: {:?}", parse_result);

    // Try to get data from database first
    let mut hlzf_data = HashMap::new();
    let mut netzentgelte_data = HashMap::new();
    let mut sources = Vec::new();

    for year in &parse_result.years {
        // Get HLZF data for this year
        if should_fetch_hlzf(&parse_result.data_types) {
            match get_hlzf_data(&state.db, &parse_result.dno_key, *year).await {
                Ok((hlzf_year_data, hlzf_sources)) => {
                    if !hlzf_year_data.is_empty() {
                        hlzf_data.insert(year.to_string(), hlzf_year_data);
                        sources.extend(hlzf_sources);
                        cache_hits += 1;
                    }
                },
                Err(e) => tracing::warn!("Failed to get HLZF data for {} {}: {}", parse_result.dno_key, year, e),
            }
        }

        // Get Netzentgelte data for this year
        if should_fetch_netzentgelte(&parse_result.data_types) {
            match get_netzentgelte_data(&state.db, &parse_result.dno_key, *year).await {
                Ok((netzentgelte_year_data, netzentgelte_sources)) => {
                    if !netzentgelte_year_data.is_empty() {
                        netzentgelte_data.insert(year.to_string(), netzentgelte_year_data);
                        sources.extend(netzentgelte_sources);
                        cache_hits += 1;
                    }
                },
                Err(e) => tracing::warn!("Failed to get Netzentgelte data for {} {}: {}", parse_result.dno_key, year, e),
            }
        }
    }

    // If we don't have data for all requested years, try to find and analyze PDFs
    let missing_years: Vec<i32> = parse_result.years.iter()
        .filter(|&year| {
            let has_hlzf = hlzf_data.contains_key(&year.to_string());
            let has_netzentgelte = netzentgelte_data.contains_key(&year.to_string());
            
            match &parse_result.data_types[0] {
                shared::DnoDataType::Hlzf => !has_hlzf,
                shared::DnoDataType::Netzentgelte => !has_netzentgelte,
                shared::DnoDataType::Both => !has_hlzf || !has_netzentgelte,
            }
        })
        .cloned()
        .collect();

    if !missing_years.is_empty() {
        tracing::info!("Missing data for years: {:?}, attempting to find PDFs", missing_years);
        
        // Try to find PDFs for missing years and analyze them
        // This would integrate with the crawling system to find and download PDFs
        // For now, we'll return what we have
    }

    let processing_time = start_time.elapsed().as_millis() as i32;

    // Create metadata
    let metadata = DnoQueryMetadata {
        dno_name: parse_result.dno_name,
        original_query: request.query.clone(),
        extracted_years: parse_result.years,
        sources: sources.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect(),
        confidence: parse_result.confidence,
        processing_time_ms: processing_time,
        cache_hits,
        query_timestamp: Utc::now(),
    };

    Ok(DnoQueryResponse {
        metadata,
        hlzf_data,
        netzentgelte_data,
    })
}

fn should_fetch_hlzf(data_types: &[shared::DnoDataType]) -> bool {
    data_types.iter().any(|dt| matches!(dt, shared::DnoDataType::Hlzf | shared::DnoDataType::Both))
}

fn should_fetch_netzentgelte(data_types: &[shared::DnoDataType]) -> bool {
    data_types.iter().any(|dt| matches!(dt, shared::DnoDataType::Netzentgelte | shared::DnoDataType::Both))
}

async fn get_hlzf_data(
    db: &sqlx::SqlitePool,
    dno_key: &str,
    year: i32,
) -> Result<(HashMap<String, Option<String>>, Vec<String>), AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT value_id, value, source_file
        FROM hlzf_data 
        WHERE key = ? AND year = ?
        ORDER BY value_id
        "#,
        dno_key,
        year
    )
    .fetch_all(db)
    .await
    .map_err(|e| AppError::Database(e))?;

    let mut data = HashMap::new();
    let mut sources = Vec::new();

    for row in rows {
        data.insert(row.value_id, row.value);
        if let Some(source) = row.source_file {
            sources.push(source);
        }
    }

    Ok((data, sources))
}

async fn get_netzentgelte_data(
    db: &sqlx::SqlitePool,
    dno_key: &str,
    year: i32,
) -> Result<(HashMap<String, HashMap<String, Option<f64>>>, Vec<String>), AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT voltage_level, value_id, value, source_file
        FROM netzentgelte_data 
        WHERE key = ? AND year = ?
        ORDER BY voltage_level, value_id
        "#,
        dno_key,
        year
    )
    .fetch_all(db)
    .await
    .map_err(|e| AppError::Database(e))?;

    let mut data: HashMap<String, HashMap<String, Option<f64>>> = HashMap::new();
    let mut sources = Vec::new();

    for row in rows {
        let voltage_data = data.entry(row.voltage_level).or_insert_with(HashMap::new);
        voltage_data.insert(row.value_id, row.value);
        
        if let Some(source) = row.source_file {
            sources.push(source);
        }
    }

    Ok((data, sources))
}

// Learning system endpoints

pub async fn get_query_learning_stats(
    State(state): State<AppState>,
) -> Json<Value> {
    match get_learning_statistics(&state.db).await {
        Ok(stats) => Json(json!({
            "success": true,
            "data": stats
        })),
        Err(err) => {
            tracing::error!("Failed to get learning stats: {}", err);
            Json(json!({
                "success": false,
                "error": format!("Failed to get learning stats: {}", err)
            }))
        }
    }
}

async fn get_learning_statistics(db: &sqlx::SqlitePool) -> Result<Value, AppError> {
    let total_queries = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM query_learning"
    )
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Database(e))?
    .unwrap_or(0);

    let avg_success_rate = sqlx::query_scalar!(
        "SELECT AVG(success_rate) as avg_rate FROM query_learning"
    )
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Database(e))?
    .unwrap_or(0.0);

    let most_common_dnos = sqlx::query!(
        r#"
        SELECT extracted_dno, COUNT(*) as count 
        FROM query_learning 
        WHERE extracted_dno IS NOT NULL 
        GROUP BY extracted_dno 
        ORDER BY count DESC 
        LIMIT 10
        "#
    )
    .fetch_all(db)
    .await
    .map_err(|e| AppError::Database(e))?;

    Ok(json!({
        "total_queries": total_queries,
        "average_success_rate": avg_success_rate,
        "most_common_dnos": most_common_dnos.into_iter().map(|row| json!({
            "dno": row.extracted_dno,
            "count": row.count
        })).collect::<Vec<_>>()
    }))
}