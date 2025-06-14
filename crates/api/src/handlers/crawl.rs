use axum::{
    extract::{State, Path},
    response::Json,
};
use serde::{Deserialize};
use serde_json::{json, Value};
use shared::{CrawlJob, CrawlStatus};
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
    State(state): State<AppState>,
    Json(req): Json<CreateCrawlJobRequest>,
) -> Result<Json<Value>, Json<Value>> {
    let job_id = Uuid::new_v4();
    
    // Start crawling immediately (in a real implementation, this would be queued)
    match state.crawl_service.crawl(&req.url).await {
        Ok(result) => {
            // In a real implementation, save to database here
            Ok(Json(json!({
                "success": true,
                "data": {
                    "id": job_id,
                    "url": req.url,
                    "status": "completed",
                    "created_at": chrono::Utc::now(),
                    "completed_at": chrono::Utc::now(),
                    "result": {
                        "title": result.title,
                        "status_code": result.status_code,
                        "links_found": result.links.len(),
                        "emails_found": result.emails.len(),
                        "phones_found": result.phone_numbers.len(),
                        "metadata": result.metadata
                    }
                },
                "message": "Crawl job completed successfully"
            })))
        },
        Err(err) => {
            tracing::error!("Crawl failed for {}: {}", req.url, err);
            Err(Json(json!({
                "success": false,
                "error": {
                    "code": "CRAWL_FAILED",
                    "message": format!("Crawl failed: {}", err)
                }
            })))
        }
    }
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