use axum::{
    extract::State,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared::{SearchQuery};
use uuid::Uuid;
use crate::AppState;

pub async fn search(
    State(state): State<AppState>,
    Json(query): Json<SearchQuery>,
) -> Result<Json<Value>, Json<Value>> {
    let start_time = std::time::Instant::now();
    
    match state.search_service.search(&query.query, query.limit).await {
        Ok(results) => {
            let search_time = start_time.elapsed();
            Ok(Json(json!({
                "success": true,
                "data": {
                    "query": query.query,
                    "results": results,
                    "total": results.len(),
                    "search_time": format!("{:.3}s", search_time.as_secs_f64())
                }
            })))
        },
        Err(err) => {
            tracing::error!("Search failed: {}", err);
            Err(Json(json!({
                "success": false,
                "error": {
                    "code": "SEARCH_FAILED",
                    "message": format!("Search failed: {}", err)
                }
            })))
        }
    }
}

pub async fn search_history(
    State(_state): State<AppState>,
) -> Json<Value> {
    // TODO: Implement search history retrieval
    let mock_history = vec![
        json!({
            "id": Uuid::new_v4(),
            "query": "German DNO data",
            "timestamp": chrono::Utc::now(),
            "results_count": 15
        }),
        json!({
            "id": Uuid::new_v4(),
            "query": "Netzbetreiber Deutschland",
            "timestamp": chrono::Utc::now(),
            "results_count": 23
        })
    ];

    Json(json!({
        "success": true,
        "data": mock_history
    }))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GatherRequest {
    pub dno: String,
    pub years: String,
    pub max_urls: Option<u32>,
    pub max_time: Option<u64>,
}

pub async fn gather(
    State(_state): State<AppState>,
    Json(request): Json<GatherRequest>,
) -> Json<Value> {
    use std::process::Command;
    
    // Execute the CLI gather command
    let years = request.years;
    let dno = request.dno;
    let max_urls = request.max_urls.unwrap_or(10);
    let max_time = request.max_time.unwrap_or(300);
    
    match Command::new("cargo")
        .args(&[
            "run", "--bin", "crawler", "--", "gather", 
            &dno, "--years", &years, "--json", 
            "--max-urls", &max_urls.to_string(),
            "--max-time", &max_time.to_string()
        ])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                match serde_json::from_str::<Value>(&stdout) {
                    Ok(json_result) => Json(json!({
                        "success": true,
                        "data": json_result
                    })),
                    Err(_) => Json(json!({
                        "success": true,
                        "data": {
                            "raw_output": stdout.to_string()
                        }
                    }))
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "CRAWLER_FAILED",
                        "message": stderr.to_string()
                    }
                }))
            }
        },
        Err(err) => Json(json!({
            "success": false,
            "error": {
                "code": "EXECUTION_FAILED",
                "message": format!("Failed to execute crawler: {}", err)
            }
        }))
    }
}