use axum::{extract::{Query, State}, http::StatusCode, response::Json, Extension};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{AppState, AuthenticatedUser};
use core::models::*;

/// Search for data by DNO name or ID
pub async fn search_by_dno(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<SearchByDnoRequest>,
) -> Result<Json<Value>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    // Determine search parameters
    let dno_id = request.dno_id;
    let dno_name = request.dno_name.as_deref();
    let year = request.year;
    let data_type = request.data_type.as_deref().unwrap_or("all");

    // Get DNO if searching by name using cached repository
    let target_dno = if let Some(name) = dno_name {
        match state.dno_repo.get_dno_by_name(name).await {
            Ok(Some(dno)) => Some(dno),
            Ok(None) => {
                return Ok(Json(json!({
                    "total": 0,
                    "results": [],
                    "filters_applied": {
                        "dno_name": name,
                        "year": year,
                        "data_type": data_type
                    },
                    "available_years": [],
                    "available_dnos": []
                })));
            }
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else if let Some(id) = dno_id {
        match state.dno_repo.get_dno_by_id(id).await {
            Ok(dno) => dno,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        None
    };

    let final_dno_id = target_dno.as_ref().map(|d| d.id).or(dno_id);
    let final_dno_name = target_dno.as_ref().map(|d| d.name.as_str()).or(dno_name);

    // Search data based on type
    let mut search_results = Vec::new();
    let mut total_count = 0i64;

    match data_type {
        "netzentgelte" => {
            let netzentgelte_data = state.search_repo.search_netzentgelte_data(
                final_dno_id,
                final_dno_name,
                year,
                Some("verified"),
                Some(50),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            total_count = state.search_repo.count_netzentgelte_data(
                final_dno_id,
                final_dno_name,
                year,
                Some("verified"),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            for entry in netzentgelte_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "netzentgelte".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "netzentgelte": {
                            "voltage_level": entry.voltage_level,
                            "leistung": entry.leistung,
                            "arbeit": entry.arbeit,
                            "leistung_unter_2500h": entry.leistung_unter_2500h,
                            "arbeit_unter_2500h": entry.arbeit_unter_2500h
                        }
                    }),
                    source: None, // TODO: Add source info
                    last_updated: entry.updated_at,
                });
            }
        }
        "hlzf" => {
            let hlzf_data = state.search_repo.search_hlzf_data(
                final_dno_id,
                final_dno_name,
                year,
                Some("verified"),
                Some(50),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            for entry in hlzf_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "hlzf".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "hlzf": {
                            "season": entry.season,
                            "voltage_level": entry.voltage_level,
                            "ht": entry.ht,
                            "nt": entry.nt,
                            "start_date": entry.start_date,
                            "end_date": entry.end_date
                        }
                    }),
                    source: None, // TODO: Add source info
                    last_updated: entry.updated_at,
                });
            }
        }
        _ => {
            // Search both types using cached repository
            let netzentgelte_data = state.search_repo.search_netzentgelte_data(
                final_dno_id,
                final_dno_name,
                year,
                Some("verified"),
                Some(25),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let hlzf_data = state.search_repo.search_hlzf_data(
                final_dno_id,
                final_dno_name,
                year,
                Some("verified"),
                Some(25),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Add netzentgelte results
            for entry in netzentgelte_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "netzentgelte".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "netzentgelte": {
                            "voltage_level": entry.voltage_level,
                            "leistung": entry.leistung,
                            "arbeit": entry.arbeit,
                            "leistung_unter_2500h": entry.leistung_unter_2500h,
                            "arbeit_unter_2500h": entry.arbeit_unter_2500h
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }

            // Add hlzf results  
            for entry in hlzf_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "hlzf".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "hlzf": {
                            "season": entry.season,
                            "voltage_level": entry.voltage_level,
                            "ht": entry.ht,
                            "nt": entry.nt,
                            "start_date": entry.start_date,
                            "end_date": entry.end_date
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }

            total_count = search_results.len() as i64;
        }
    }

    // Get available filters using cached repository
    let available_filters = state.search_repo.get_available_years_and_dnos()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log query
    let response_time = start_time.elapsed().as_millis() as i32;
    let query_text = format!("Search by DNO: {} (year: {:?}, type: {})", 
        final_dno_name.unwrap_or("unknown"), year, data_type);
    
    let log = CreateQueryLog {
        user_id: Some(user.id),
        query: query_text,
        interpretation: Some(format!("DNO search for {}", data_type)),
        response_time_ms: Some(response_time),
        source_ip: None, // TODO: Extract from request
    };
    
    let _ = core::database::log_query(&state.database, log).await;

    Ok(Json(json!({
        "total": total_count,
        "results": search_results,
        "filters_applied": {
            "dno_name": final_dno_name,
            "dno_id": final_dno_id,
            "year": year,
            "data_type": data_type
        },
        "available_years": available_filters.years,
        "available_dnos": available_filters.dnos
    })))
}

/// Search for data by year
pub async fn search_by_year(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<SearchByYearRequest>,
) -> Result<Json<Value>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    let year = request.year;
    let dno_name = request.dno_name.as_deref();
    let dno_id = request.dno_id;
    let data_type = request.data_type.as_deref().unwrap_or("all");

    let mut search_results = Vec::new();
    let mut total_count = 0i64;

    match data_type {
        "netzentgelte" => {
            let netzentgelte_data = state.search_repo.search_netzentgelte_data(
                dno_id,
                dno_name,
                Some(year),
                Some("verified"),
                Some(50),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            total_count = state.search_repo.count_netzentgelte_data(
                dno_id,
                dno_name,
                Some(year),
                Some("verified"),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            for entry in netzentgelte_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "netzentgelte".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "netzentgelte": {
                            "voltage_level": entry.voltage_level,
                            "leistung": entry.leistung,
                            "arbeit": entry.arbeit,
                            "leistung_unter_2500h": entry.leistung_unter_2500h,
                            "arbeit_unter_2500h": entry.arbeit_unter_2500h
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }
        }
        "hlzf" => {
            let hlzf_data = state.search_repo.search_hlzf_data(
                dno_id,
                dno_name,
                Some(year),
                Some("verified"),
                Some(50),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            for entry in hlzf_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "hlzf".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "hlzf": {
                            "season": entry.season,
                            "voltage_level": entry.voltage_level,
                            "ht": entry.ht,
                            "nt": entry.nt,
                            "start_date": entry.start_date,
                            "end_date": entry.end_date
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }
            total_count = search_results.len() as i64;
        }
        _ => {
            // Search both
            let netzentgelte_data = state.search_repo.search_netzentgelte_data(
                dno_id,
                dno_name,
                Some(year),
                Some("verified"),
                Some(25),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let hlzf_data = state.search_repo.search_hlzf_data(
                dno_id,
                dno_name,
                Some(year),
                Some("verified"),
                Some(25),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Process results (similar to above)
            for entry in netzentgelte_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "netzentgelte".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "netzentgelte": {
                            "voltage_level": entry.voltage_level,
                            "leistung": entry.leistung,
                            "arbeit": entry.arbeit,
                            "leistung_unter_2500h": entry.leistung_unter_2500h,
                            "arbeit_unter_2500h": entry.arbeit_unter_2500h
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }

            for entry in hlzf_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "hlzf".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "hlzf": {
                            "season": entry.season,
                            "voltage_level": entry.voltage_level,
                            "ht": entry.ht,
                            "nt": entry.nt,
                            "start_date": entry.start_date,
                            "end_date": entry.end_date
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }
            total_count = search_results.len() as i64;
        }
    }

    let available_filters = state.search_repo.get_available_years_and_dnos()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log query
    let response_time = start_time.elapsed().as_millis() as i32;
    let log = CreateQueryLog {
        user_id: Some(user.id),
        query: format!("Search by year: {} (type: {})", year, data_type),
        interpretation: Some(format!("Year-based search for {}", data_type)),
        response_time_ms: Some(response_time),
        source_ip: None,
    };
    let _ = core::database::log_query(&state.database, log).await;

    Ok(Json(json!({
        "total": total_count,
        "results": search_results,
        "filters_applied": {
            "year": year,
            "dno_name": dno_name,
            "dno_id": dno_id,
            "data_type": data_type
        },
        "available_years": available_filters.years,
        "available_dnos": available_filters.dnos
    })))
}

/// Search for data by data type (netzentgelte or hlzf)
pub async fn search_by_data_type(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<SearchByDataTypeRequest>,
) -> Result<Json<Value>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    let data_type = &request.data_type;
    let dno_name = request.dno_name.as_deref();
    let dno_id = request.dno_id;
    let year = request.year;

    let mut search_results = Vec::new();
    let total_count;

    match data_type.as_str() {
        "netzentgelte" => {
            let netzentgelte_data = state.search_repo.search_netzentgelte_data(
                dno_id,
                dno_name,
                year,
                Some("verified"),
                Some(50),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            total_count = state.search_repo.count_netzentgelte_data(
                dno_id,
                dno_name,
                year,
                Some("verified"),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            for entry in netzentgelte_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "netzentgelte".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "netzentgelte": {
                            "voltage_level": entry.voltage_level,
                            "leistung": entry.leistung,
                            "arbeit": entry.arbeit,
                            "leistung_unter_2500h": entry.leistung_unter_2500h,
                            "arbeit_unter_2500h": entry.arbeit_unter_2500h
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }
        }
        "hlzf" => {
            let hlzf_data = state.search_repo.search_hlzf_data(
                dno_id,
                dno_name,
                year,
                Some("verified"),
                Some(50),
                Some(0),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            for entry in hlzf_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "hlzf".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "hlzf": {
                            "season": entry.season,
                            "voltage_level": entry.voltage_level,
                            "ht": entry.ht,
                            "nt": entry.nt,
                            "start_date": entry.start_date,
                            "end_date": entry.end_date
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }
            total_count = search_results.len() as i64;
        }
        _ => {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let available_filters = state.search_repo.get_available_years_and_dnos()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log query
    let response_time = start_time.elapsed().as_millis() as i32;
    let log = CreateQueryLog {
        user_id: Some(user.id),
        query: format!("Search by data type: {}", data_type),
        interpretation: Some(format!("Data type search for {}", data_type)),
        response_time_ms: Some(response_time),
        source_ip: None,
    };
    let _ = core::database::log_query(&state.database, log).await;

    Ok(Json(json!({
        "total": total_count,
        "results": search_results,
        "filters_applied": {
            "data_type": data_type,
            "dno_name": dno_name,
            "dno_id": dno_id,
            "year": year
        },
        "available_years": available_filters.years,
        "available_dnos": available_filters.dnos
    })))
}

/// Search with multiple filters using query parameters
pub async fn search_with_filters(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(filters): Query<SearchFilters>,
) -> Result<Json<Value>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    let dno_name = filters.dno_name.as_deref();
    let dno_id = filters.dno_id;
    let year = filters.year;
    let data_type = filters.data_type.as_deref().unwrap_or("all");
    let limit = filters.limit.map(|l| l as i64).unwrap_or(50);
    let offset = filters.offset.map(|o| o as i64).unwrap_or(0);

    let mut search_results = Vec::new();
    let mut total_count = 0i64;

    match data_type {
        "netzentgelte" => {
            let netzentgelte_data = state.search_repo.search_netzentgelte_data(
                dno_id,
                dno_name,
                year,
                Some("verified"),
                Some(limit),
                Some(offset),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            total_count = state.search_repo.count_netzentgelte_data(
                dno_id,
                dno_name,
                year,
                Some("verified"),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            for entry in netzentgelte_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "netzentgelte".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "netzentgelte": {
                            "voltage_level": entry.voltage_level,
                            "leistung": entry.leistung,
                            "arbeit": entry.arbeit,
                            "leistung_unter_2500h": entry.leistung_unter_2500h,
                            "arbeit_unter_2500h": entry.arbeit_unter_2500h
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }
        }
        "hlzf" => {
            let hlzf_data = state.search_repo.search_hlzf_data(
                dno_id,
                dno_name,
                year,
                Some("verified"),
                Some(limit),
                Some(offset),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            for entry in hlzf_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "hlzf".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "hlzf": {
                            "season": entry.season,
                            "voltage_level": entry.voltage_level,
                            "ht": entry.ht,
                            "nt": entry.nt,
                            "start_date": entry.start_date,
                            "end_date": entry.end_date
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }
            total_count = search_results.len() as i64;
        }
        _ => {
            // Mixed search - limit per type
            let half_limit = limit / 2;
            
            let netzentgelte_data = state.search_repo.search_netzentgelte_data(
                dno_id,
                dno_name,
                year,
                Some("verified"),
                Some(half_limit),
                Some(offset / 2),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let hlzf_data = state.search_repo.search_hlzf_data(
                dno_id,
                dno_name,
                year,
                Some("verified"),
                Some(half_limit),
                Some(offset / 2),
            ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Add both result types
            for entry in netzentgelte_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "netzentgelte".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "netzentgelte": {
                            "voltage_level": entry.voltage_level,
                            "leistung": entry.leistung,
                            "arbeit": entry.arbeit,
                            "leistung_unter_2500h": entry.leistung_unter_2500h,
                            "arbeit_unter_2500h": entry.arbeit_unter_2500h
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }

            for entry in hlzf_data {
                search_results.push(SearchResult {
                    id: entry.id,
                    dno: DnoInfo {
                        id: entry.dno_id_full,
                        name: entry.dno_name,
                        slug: entry.dno_slug,
                        region: entry.dno_region,
                    },
                    year: entry.year,
                    data_type: "hlzf".to_string(),
                    status: entry.verification_status.unwrap_or_else(|| "unverified".to_string()),
                    data: json!({
                        "hlzf": {
                            "season": entry.season,
                            "voltage_level": entry.voltage_level,
                            "ht": entry.ht,
                            "nt": entry.nt,
                            "start_date": entry.start_date,
                            "end_date": entry.end_date
                        }
                    }),
                    source: None,
                    last_updated: entry.updated_at,
                });
            }

            total_count = search_results.len() as i64;
        }
    }

    let available_filters = state.search_repo.get_available_years_and_dnos()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log query
    let response_time = start_time.elapsed().as_millis() as i32;
    let query_text = format!("Filter search: DNO={:?}, year={:?}, type={}", 
        dno_name, year, data_type);
    let log = CreateQueryLog {
        user_id: Some(user.id),
        query: query_text,
        interpretation: Some(format!("Filtered search with {} results", search_results.len())),
        response_time_ms: Some(response_time),
        source_ip: None,
    };
    let _ = core::database::log_query(&state.database, log).await;

    Ok(Json(json!({
        "total": total_count,
        "results": search_results,
        "pagination": {
            "limit": limit,
            "offset": offset,
            "total": total_count,
            "has_more": (offset + limit) < total_count
        },
        "filters_applied": {
            "dno_name": dno_name,
            "dno_id": dno_id,
            "year": year,
            "data_type": data_type,
            "region": filters.region,
            "limit": limit,
            "offset": offset
        },
        "available_filters": {
            "years": available_filters.years,
            "data_types": ["netzentgelte", "hlzf"],
            "regions": available_filters.regions
        }
    })))
}