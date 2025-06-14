use axum::{
    extract::State,
    response::Json,
};
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