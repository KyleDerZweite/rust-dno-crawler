use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{json, Value};
use crate::AppState;

pub async fn natural_query(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual natural language query logic here
    // For now, fallback to mock
    _natural_query(State(state)).await
}

pub async fn _natural_query(State(_state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "query_id": "990e8400-e29b-41d4-a716-446655440000",
        "interpretation": {
            "dno": "Netze BW",
            "year": 2024,
            "data_type": "netzentgelte",
            "confidence": 0.95
        },
        "status": "found",
        "storage": {
            "netzentgelte": {
                "hs": {"leistung": 58.21, "arbeit": 1.26},
                "ms": {"leistung": 109.86, "arbeit": 1.73}
            }
        },
        "source": {
            "file": "netzentgelte-2024.pdf",
            "url": "https://netze-bw.de/...",
            "page": 12,
            "extracted_at": "2024-01-15T10:00:00Z"
        },
        "response": {
            "text": "Hier sind die Netzentgelte von Netze BW für 2024:",
            "format": "json"
        },
        "job_id": null
    })))
}