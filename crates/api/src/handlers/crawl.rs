use axum::{
    extract::{State, Path},
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared::{AppError, CrawlJob, CrawlStatus};
use uuid::Uuid;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateCrawlJobRequest {
    pub url: String,
    pub priority: Option<String>,
}

pub async fn list_jobs(
    State(_state): State<AppState>,
) -> Json<Value> {
    // TODO: Implement actual job retrieval from database
    let mock_jobs = vec![
        json!({
            "id": Uuid::new_v4(),
            "url": "https://example-dno.de",
            "status": "completed",
            "created_at": chrono::Utc::now(),
            "completed_at": chrono::Utc::now()
        }),
        json!({
            "id": Uuid::new_v4(),
            "url": "https://another-dno.de",
            "status": "in_progress",
            "created_at": chrono::Utc::now(),
            "started_at": chrono::Utc::now()
        })
    ];

    Json(json!({
        "success": true,
        "data": mock_jobs
    }))
}

pub async fn create_job(
    State(_state): State<AppState>,
    Json(req): Json<CreateCrawlJobRequest>,
) -> Json<Value> {
    // TODO: Implement actual job creation and queue
    let job_id = Uuid::new_v4();
    
    Json(json!({
        "success": true,
        "data": {
            "id": job_id,
            "url": req.url,
            "status": "pending",
            "created_at": chrono::Utc::now()
        },
        "message": "Crawl job created successfully"
    }))
}

pub async fn get_job(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<Value> {
    // TODO: Implement job retrieval
    Json(json!({
        "success": true,
        "data": {
            "id": id,
            "url": "https://example.com",
            "status": "completed",
            "created_at": chrono::Utc::now(),
            "completed_at": chrono::Utc::now(),
            "result": {
                "content": "Mock crawled content",
                "links": ["https://link1.com", "https://link2.com"],
                "metadata": {
                    "title": "Mock Page Title",
                    "description": "Mock page description"
                }
            }
        }
    }))
}

pub async fn job_status(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<Value> {
    // TODO: Implement job status check
    Json(json!({
        "success": true,
        "data": {
            "id": id,
            "status": "completed",
            "progress": 100,
            "message": "Job completed successfully"
        }
    }))
}