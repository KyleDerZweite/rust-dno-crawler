use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use shared::{
    CrawlSessionRequest, CrawlSessionResponse, CrawlSessionStatus, LiveCrawlSession, 
    LiveLog, Priority, JobType, CrawlConstraints
};
use std::collections::HashMap;
use tracing::{error, info};
use uuid::Uuid;

use crate::{AppState, AppError};

#[derive(Debug, Deserialize)]
pub struct StartCrawlRequest {
    pub dno_name: String,
    pub dno_key: String,
    pub year: i32,
    pub priority: Option<Priority>,
    pub constraints: Option<CrawlConstraints>,
    pub scheduled_for: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct StartCrawlResponse {
    pub session_id: Uuid,
    pub status: CrawlSessionStatus,
    pub estimated_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub queue_position: usize,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SessionQuery {
    pub limit: Option<u32>,
    pub status: Option<CrawlSessionStatus>,
}

#[derive(Debug, Deserialize)]
pub struct LogQuery {
    pub limit: Option<u32>,
    pub level: Option<String>,
    pub since: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct SessionListResponse {
    pub sessions: Vec<LiveCrawlSession>,
    pub total_count: usize,
    pub active_count: usize,
    pub queued_count: usize,
}

#[derive(Debug, Serialize)]
pub struct SessionLogResponse {
    pub logs: Vec<LiveLog>,
    pub session_id: Uuid,
    pub total_logs: usize,
}

/// Start a new crawl session
pub async fn start_crawl_session(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request): Json<StartCrawlRequest>,
) -> Result<Json<StartCrawlResponse>, AppError> {
    info!("Starting crawl session for {} {}", request.dno_name, request.year);

    // Create crawl session request
    let session_request = CrawlSessionRequest {
        dno_name: request.dno_name.clone(),
        dno_key: request.dno_key.clone(),
        year: request.year,
        priority: request.priority.map(|p| p.into()),
        created_by_user: Some(user_id.to_string()),
        strategy_preference: request.constraints.map(|c| serde_json::to_value(c).unwrap_or(serde_json::Value::Null)),
        constraints: None,
        scheduled_for: request.scheduled_for,
    };

    // Submit to orchestrator (placeholder - will integrate with actual orchestrator)
    let session_id = Uuid::new_v4();
    
    // For now, create a mock response
    let response = StartCrawlResponse {
        session_id,
        status: CrawlSessionStatus::Queued,
        estimated_start_time: Some(chrono::Utc::now() + chrono::Duration::minutes(2)),
        queue_position: 1,
        message: format!("Crawl session queued for {} {}", request.dno_name, request.year),
    };

    info!("Created crawl session {} for {} {}", session_id, request.dno_name, request.year);
    Ok(Json(response))
}

/// Get all active crawl sessions
pub async fn get_active_sessions(
    State(state): State<AppState>,
    Query(query): Query<SessionQuery>,
) -> Result<Json<SessionListResponse>, AppError> {
    info!("Retrieving active crawl sessions");

    // Placeholder implementation - will integrate with actual orchestrator
    let sessions = vec![];
    
    let response = SessionListResponse {
        sessions: sessions.clone(),
        total_count: sessions.len(),
        active_count: sessions.iter().filter(|s| matches!(s.status, 
            CrawlSessionStatus::Initializing | CrawlSessionStatus::Searching | 
            CrawlSessionStatus::Crawling | CrawlSessionStatus::Extracting)).count(),
        queued_count: sessions.iter().filter(|s| matches!(s.status, CrawlSessionStatus::Queued)).count(),
    };

    Ok(Json(response))
}

/// Get specific session status
pub async fn get_session_status(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<LiveCrawlSession>, AppError> {
    info!("Retrieving status for session {}", session_id);

    // Placeholder implementation
    Err(AppError::NotFound("Session not found".to_string()))
}

/// Get session logs
pub async fn get_session_logs(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Query(query): Query<LogQuery>,
) -> Result<Json<SessionLogResponse>, AppError> {
    info!("Retrieving logs for session {}", session_id);

    // Placeholder implementation
    let logs = vec![];
    
    let response = SessionLogResponse {
        logs: logs.clone(),
        session_id,
        total_logs: logs.len(),
    };

    Ok(Json(response))
}

/// Cancel a crawl session
pub async fn cancel_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    info!("Cancelling session {}", session_id);

    // Placeholder implementation - will integrate with actual orchestrator
    Ok(Json(serde_json::json!({
        "message": "Session cancelled successfully",
        "session_id": session_id
    })))
}

/// Pause a crawl session
pub async fn pause_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    info!("Pausing session {}", session_id);

    // Placeholder implementation
    Ok(Json(serde_json::json!({
        "message": "Session paused successfully",
        "session_id": session_id
    })))
}

/// Resume a paused crawl session
pub async fn resume_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    info!("Resuming session {}", session_id);

    // Placeholder implementation
    Ok(Json(serde_json::json!({
        "message": "Session resumed successfully",
        "session_id": session_id
    })))
}

/// Submit automated discovery job
pub async fn submit_automated_job(
    State(state): State<AppState>,
    Json(request): Json<AutomatedJobRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    info!("Submitting automated job for {} {}", request.dno_key, request.year);

    // Placeholder implementation
    let job_id = Uuid::new_v4();

    Ok(Json(serde_json::json!({
        "job_id": job_id,
        "message": "Automated job submitted successfully",
        "dno_key": request.dno_key,
        "year": request.year,
        "job_type": request.job_type
    })))
}

/// Get orchestrator statistics
pub async fn get_orchestrator_stats(
    State(state): State<AppState>,
) -> Result<Json<OrchestratorStats>, AppError> {
    info!("Retrieving orchestrator statistics");

    // Placeholder implementation
    let stats = OrchestratorStats {
        total_sessions: 0,
        active_sessions: 0,
        queued_sessions: 0,
        completed_today: 0,
        failed_today: 0,
        average_completion_time_minutes: 0.0,
        worker_utilization: 0.0,
        queue_health: "healthy".to_string(),
    };

    Ok(Json(stats))
}

#[derive(Debug, Deserialize)]
pub struct AutomatedJobRequest {
    pub dno_key: String,
    pub year: i32,
    pub job_type: JobType,
}

#[derive(Debug, Serialize)]
pub struct OrchestratorStats {
    pub total_sessions: u32,
    pub active_sessions: u32,
    pub queued_sessions: u32,
    pub completed_today: u32,
    pub failed_today: u32,
    pub average_completion_time_minutes: f64,
    pub worker_utilization: f64,
    pub queue_health: String,
}