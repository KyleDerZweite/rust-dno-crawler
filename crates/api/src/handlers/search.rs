use axum::{
    extract::State,
    response::Json,
};
use serde_json::{json, Value};
use shared::{AppError, SearchQuery, SearchResult};
use uuid::Uuid;
use crate::AppState;

pub async fn search(
    State(_state): State<AppState>,
    Json(query): Json<SearchQuery>,
) -> Json<Value> {
    // TODO: Implement actual search via SearXNG
    // For now, return mock results
    let mock_results = vec![
        SearchResult {
            id: Uuid::new_v4(),
            title: format!("Search result for: {}", query.query),
            url: "https://example.com/result1".to_string(),
            snippet: "This is a mock search result snippet...".to_string(),
            source: "Mock Source".to_string(),
            relevance_score: 0.95,
            found_at: chrono::Utc::now(),
        },
        SearchResult {
            id: Uuid::new_v4(),
            title: format!("Another result for: {}", query.query),
            url: "https://example.com/result2".to_string(),
            snippet: "Another mock search result snippet...".to_string(),
            source: "Another Source".to_string(),
            relevance_score: 0.87,
            found_at: chrono::Utc::now(),
        }
    ];

    Json(json!({
        "success": true,
        "data": {
            "query": query.query,
            "results": mock_results,
            "total": mock_results.len(),
            "search_time": "0.123s"
        }
    }))
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