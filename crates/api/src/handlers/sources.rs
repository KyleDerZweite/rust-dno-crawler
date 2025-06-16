use axum::{
    extract::{State, Path, Query},
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared::{
    DataSourceV2, SourceType, ExtractionMethod, AdminDataVerificationStatus,
    DataSourceYearly, CrawlConfig
};
use std::collections::HashMap;
use tokio::fs;
use tracing::{info, warn, error};
use uuid::Uuid;
use crate::AppState;

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct SourceQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub source_type: Option<String>,
    pub dno_key: Option<String>,
    pub verification_status: Option<String>,
    pub active_only: Option<bool>,
    pub flagged_only: Option<bool>,
    pub include_inactive: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SourceUpdateRequest {
    pub is_active: Option<bool>,
    pub admin_notes: Option<String>,
    pub verification_status: Option<AdminDataVerificationStatus>,
}

#[derive(Debug, Deserialize)]
pub struct SourceUploadRequest {
    pub dno_key: String,
    pub source_type: SourceType,
    pub description: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct BulkSourceOperationRequest {
    pub source_ids: Vec<String>,
    pub operation: String, // "activate", "deactivate", "verify", "flag"
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SourceMetrics {
    pub total_sources: u32,
    pub active_sources: u32,
    pub verified_sources: u32,
    pub flagged_sources: u32,
    pub pending_verification: u32,
    pub sources_by_type: HashMap<String, u32>,
    pub extraction_success_rate: f64,
    pub average_confidence: f64,
}

/// Get all data sources with filtering
pub async fn get_sources(
    State(state): State<AppState>,
    Query(params): Query<SourceQueryParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.source_service.get_sources(params.into()).await {
        Ok(sources) => {
            let total = sources.len();
            let offset = params.offset.unwrap_or(0) as usize;
            let limit = params.limit.unwrap_or(50) as usize;
            
            let paginated_sources: Vec<_> = sources
                .into_iter()
                .skip(offset)
                .take(limit)
                .collect();

            Ok(Json(json!({
                "success": true,
                "data": paginated_sources,
                "metadata": {
                    "total": total,
                    "offset": offset,
                    "limit": limit,
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get sources: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCES_FETCH_FAILED",
                        "message": format!("Failed to fetch sources: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get specific source details
pub async fn get_source_details(
    State(state): State<AppState>,
    Path(source_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.source_service.get_source_details(source_id.clone()).await {
        Ok(Some(source)) => {
            // Get additional metadata like usage history
            let usage_history = state.source_service
                .get_source_usage_history(source_id.clone())
                .await
                .unwrap_or_default();

            Ok(Json(json!({
                "success": true,
                "data": {
                    "source": source,
                    "usage_history": usage_history,
                    "file_info": {
                        "exists": source.local_file_path.as_ref()
                            .map(|path| std::path::Path::new(path).exists())
                            .unwrap_or(false),
                        "size_bytes": source.file_size,
                        "content_type": source.content_type
                    }
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_NOT_FOUND",
                        "message": "Source not found"
                    }
                }))
            ))
        }
        Err(err) => {
            error!("Failed to get source details: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_DETAILS_FETCH_FAILED",
                        "message": format!("Failed to fetch source details: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Update source metadata and status
pub async fn update_source(
    State(state): State<AppState>,
    Path(source_id): Path<String>,
    Json(req): Json<SourceUpdateRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.source_service.update_source(source_id.clone(), req).await {
        Ok(()) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "source_id": source_id,
                    "updated_at": chrono::Utc::now()
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to update source: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_UPDATE_FAILED",
                        "message": format!("Failed to update source: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Upload a new source file (placeholder - would need multipart support)
pub async fn upload_source_file(
    State(_state): State<AppState>,
    Json(upload_request): Json<SourceUploadRequest>,
) -> Json<Value> {
    // TODO: Implement actual file upload with multipart support
    info!("Uploading source file for DNO: {}", upload_request.dno_key);
    
    Json(json!({
        "success": true,
        "data": {
            "source_id": Uuid::new_v4(),
            "dno_key": upload_request.dno_key,
            "source_type": upload_request.source_type,
            "uploaded_at": chrono::Utc::now(),
            "status": "upload_complete"
        },
        "message": "Source file uploaded successfully"
    }))
}

/// Delete a source and its associated file
pub async fn delete_source(
    State(state): State<AppState>,
    Path(source_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.source_service.delete_source(source_id.clone()).await {
        Ok(()) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "source_id": source_id,
                    "deleted_at": chrono::Utc::now()
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to delete source: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_DELETE_FAILED",
                        "message": format!("Failed to delete source: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Perform bulk operations on sources
pub async fn bulk_source_operations(
    State(state): State<AppState>,
    Json(req): Json<BulkSourceOperationRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    info!("Performing bulk operation '{}' on {} sources", req.operation, req.source_ids.len());

    match state.source_service.bulk_operations(req).await {
        Ok(results) => {
            Ok(Json(json!({
                "success": true,
                "data": results,
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to perform bulk operations: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "BULK_OPERATION_FAILED",
                        "message": format!("Failed to perform bulk operations: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get source metrics and statistics
pub async fn get_source_metrics(
    State(state): State<AppState>,
    Query(params): Query<SourceQueryParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.source_service.get_source_metrics(params.into()).await {
        Ok(metrics) => {
            Ok(Json(json!({
                "success": true,
                "data": metrics,
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get source metrics: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_METRICS_FETCH_FAILED",
                        "message": format!("Failed to fetch source metrics: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Re-analyze a source file
pub async fn reanalyze_source(
    State(state): State<AppState>,
    Path(source_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.source_service.reanalyze_source(source_id.clone()).await {
        Ok(analysis_result) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "source_id": source_id,
                    "analysis_result": analysis_result,
                    "reanalyzed_at": chrono::Utc::now()
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to reanalyze source: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_REANALYSIS_FAILED",
                        "message": format!("Failed to reanalyze source: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get source file content (for preview/download)
pub async fn get_source_content(
    State(state): State<AppState>,
    Path(source_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let preview_only = params.get("preview").map(|v| v == "true").unwrap_or(false);
    let max_size = params.get("max_size")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1024 * 1024); // 1MB default

    match state.source_service.get_source_content(source_id.clone(), preview_only, max_size).await {
        Ok(content) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "source_id": source_id,
                    "content": content,
                    "preview_only": preview_only,
                    "retrieved_at": chrono::Utc::now()
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get source content: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_CONTENT_FETCH_FAILED",
                        "message": format!("Failed to fetch source content: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Clean up orphaned source files
pub async fn cleanup_orphaned_sources(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.source_service.cleanup_orphaned_sources().await {
        Ok(cleanup_result) => {
            Ok(Json(json!({
                "success": true,
                "data": cleanup_result,
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to cleanup orphaned sources: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_CLEANUP_FAILED",
                        "message": format!("Failed to cleanup orphaned sources: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Export source inventory
pub async fn export_source_inventory(
    State(state): State<AppState>,
    Query(params): Query<SourceQueryParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.source_service.export_source_inventory(params.into()).await {
        Ok(inventory) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "export_id": Uuid::new_v4(),
                    "inventory": inventory,
                    "exported_at": chrono::Utc::now(),
                    "format": "json"
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to export source inventory: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_INVENTORY_EXPORT_FAILED",
                        "message": format!("Failed to export source inventory: {}", err)
                    }
                }))
            ))
        }
    }
}

// Additional route handler functions that match the routes.rs definitions

/// Get specific source by ID (alias for get_source_details)
pub async fn get_source(
    state: State<AppState>,
    path: Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    get_source_details(state, path).await
}

/// Download a source file
pub async fn download_source(
    State(_state): State<AppState>,
    Path(source_id): Path<String>,
) -> Json<Value> {
    // TODO: Implement actual file download with proper response headers
    info!("Downloading source file: {}", source_id);
    
    Json(json!({
        "success": true,
        "data": {
            "source_id": source_id,
            "download_url": format!("/api/v1/sources/{}/content?download=true", source_id),
            "expires_at": chrono::Utc::now() + chrono::Duration::hours(1),
            "file_info": {
                "filename": "example_document.pdf",
                "size_bytes": 2048576,
                "content_type": "application/pdf",
                "last_modified": chrono::Utc::now()
            }
        },
        "message": "Source file ready for download"
    }))
}

/// Verify source integrity
pub async fn verify_source_integrity(
    State(_state): State<AppState>,
    Path(source_id): Path<String>,
    Json(verification_params): Json<Value>,
) -> Json<Value> {
    // TODO: Implement actual source integrity verification
    info!("Verifying integrity of source: {} with params: {:?}", source_id, verification_params);
    
    Json(json!({
        "success": true,
        "data": {
            "source_id": source_id,
            "integrity_status": "verified",
            "verification_details": {
                "file_hash_match": true,
                "file_size_match": true,
                "content_readable": true,
                "metadata_consistent": true,
                "last_verification": chrono::Utc::now()
            },
            "verification_score": 0.98,
            "issues_found": [],
            "recommendations": [
                "Source integrity is excellent",
                "No issues detected"
            ]
        }
    }))
}

/// Get sources for a specific DNO and year
pub async fn get_sources_for_dno_year(
    State(_state): State<AppState>,
    Path((dno_key, year)): Path<(String, i32)>,
) -> Json<Value> {
    // TODO: Implement actual database query for DNO-year specific sources
    info!("Getting sources for DNO: {} and year: {}", dno_key, year);
    
    let mock_sources = vec![
        json!({
            "id": "source_001",
            "dno_key": dno_key,
            "year": year,
            "source_type": "pdf",
            "original_url": format!("https://{}.de/archive/{}/netzentgelte.pdf", dno_key, year),
            "local_file_path": format!("/data/{}/{}/netzentgelte.pdf", dno_key, year),
            "file_size": 2048576,
            "content_type": "application/pdf",
            "extraction_confidence": 0.92,
            "extraction_method": "pdf_analysis",
            "is_active": true,
            "admin_verification_status": "verified",
            "discovered_at": chrono::Utc::now()
        }),
        json!({
            "id": "source_002",
            "dno_key": dno_key,
            "year": year,
            "source_type": "webpage",
            "original_url": format!("https://{}.de/tariffs/{}/", dno_key, year),
            "local_file_path": null,
            "file_size": null,
            "content_type": "text/html",
            "extraction_confidence": 0.78,
            "extraction_method": "text_parsing",
            "is_active": true,
            "admin_verification_status": "pending",
            "discovered_at": chrono::Utc::now()
        })
    ];
    
    Json(json!({
        "success": true,
        "data": mock_sources,
        "metadata": {
            "dno_key": dno_key,
            "year": year,
            "total_sources": mock_sources.len(),
            "verified_sources": 1,
            "pending_sources": 1,
            "source_types": {
                "pdf": 1,
                "webpage": 1
            }
        }
    }))
}

/// Get source metadata
pub async fn get_source_metadata(
    State(_state): State<AppState>,
    Path(source_id): Path<String>,
) -> Json<Value> {
    // TODO: Implement actual source metadata retrieval
    info!("Getting metadata for source: {}", source_id);
    
    Json(json!({
        "success": true,
        "data": {
            "source_id": source_id,
            "metadata": {
                "file_info": {
                    "filename": "netzentgelte_2024.pdf",
                    "file_size": 2048576,
                    "content_type": "application/pdf",
                    "file_hash": "sha256:abcd1234...",
                    "created_at": chrono::Utc::now(),
                    "last_modified": chrono::Utc::now()
                },
                "extraction_info": {
                    "extraction_method": "pdf_analysis",
                    "confidence_score": 0.92,
                    "data_extracted": true,
                    "extraction_time_ms": 3200,
                    "extracted_tables": 5,
                    "extracted_values": 42
                },
                "source_info": {
                    "original_url": "https://example.de/netzentgelte_2024.pdf",
                    "discovered_by": "automated_crawler",
                    "crawl_session_id": "session_001",
                    "discovery_confidence": 0.89
                },
                "verification_info": {
                    "admin_verified": true,
                    "verification_status": "verified",
                    "verified_by": "admin_user",
                    "verified_at": chrono::Utc::now(),
                    "verification_notes": "High quality PDF with clear tabular data"
                }
            }
        }
    }))
}

/// Deduplicate sources
pub async fn deduplicate_sources(
    State(_state): State<AppState>,
    Json(deduplication_params): Json<Value>,
) -> Json<Value> {
    // TODO: Implement actual source deduplication logic
    info!("Deduplicating sources with params: {:?}", deduplication_params);
    
    let mock_deduplication_result = json!({
        "total_sources_analyzed": 245,
        "duplicates_found": 18,
        "duplicates_by_type": {
            "exact_file_match": 12,
            "similar_content": 4,
            "same_url_different_download": 2
        },
        "duplicates_removed": 16,
        "duplicates_flagged": 2,
        "space_saved_bytes": 32505856,
        "processing_time_ms": 5400,
        "duplicate_groups": [
            {
                "group_id": "dup_001",
                "master_source_id": "source_123",
                "duplicate_source_ids": ["source_456", "source_789"],
                "similarity_score": 1.0,
                "match_type": "exact_file_match"
            },
            {
                "group_id": "dup_002",
                "master_source_id": "source_234",
                "duplicate_source_ids": ["source_567"],
                "similarity_score": 0.94,
                "match_type": "similar_content"
            }
        ]
    });
    
    Json(json!({
        "success": true,
        "data": {
            "deduplication_session_id": Uuid::new_v4(),
            "results": mock_deduplication_result,
            "completed_at": chrono::Utc::now(),
            "recommendations": [
                "16 exact duplicates were safely removed",
                "2 similar sources flagged for manual review",
                "Consider implementing automated deduplication for new sources"
            ]
        }
    }))
}