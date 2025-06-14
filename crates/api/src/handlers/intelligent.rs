use axum::{
    extract::State,
    response::Json,
};
use serde_json::{json, Value};
use crate::AppState;
use crate::services::{IntelligentSearchRequest, IntelligentSearchResponse};

pub async fn intelligent_search(
    State(state): State<AppState>,
    Json(request): Json<IntelligentSearchRequest>,
) -> Result<Json<Value>, Json<Value>> {
    tracing::info!("Intelligent search request: {}", request.query);
    
    match state.search_orchestrator.intelligent_search(request).await {
        Ok(response) => {
            tracing::info!("Intelligent search completed successfully");
            Ok(Json(json!({
                "success": true,
                "data": response
            })))
        }
        Err(err) => {
            tracing::error!("Intelligent search failed: {}", err);
            Err(Json(json!({
                "success": false,
                "error": {
                    "code": "INTELLIGENT_SEARCH_FAILED",
                    "message": format!("Search failed: {}", err)
                }
            })))
        }
    }
}

pub async fn query_ai_only(
    State(state): State<AppState>,
    Json(request): Json<IntelligentSearchRequest>,
) -> Result<Json<Value>, Json<Value>> {
    tracing::info!("AI-only query request: {}", request.query);
    
    match state.ai_service.process_query(&request.query).await {
        Ok(processed_query) => {
            tracing::info!("AI query processing completed");
            Ok(Json(json!({
                "success": true,
                "data": {
                    "processed_query": processed_query,
                    "original_query": request.query
                }
            })))
        }
        Err(err) => {
            tracing::error!("AI query processing failed: {}", err);
            Err(Json(json!({
                "success": false,
                "error": {
                    "code": "AI_PROCESSING_FAILED",
                    "message": format!("AI processing failed: {}", err)
                }
            })))
        }
    }
}

pub async fn health_ai(
    State(state): State<AppState>,
) -> Json<Value> {
    // Test AI connectivity
    let test_query = "Test Verbindung";
    
    match state.ai_service.process_query(test_query).await {
        Ok(_) => {
            Json(json!({
                "success": true,
                "ai_status": "healthy",
                "model": "llama3",
                "timestamp": chrono::Utc::now()
            }))
        }
        Err(err) => {
            Json(json!({
                "success": false,
                "ai_status": "unhealthy",
                "error": err.to_string(),
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}