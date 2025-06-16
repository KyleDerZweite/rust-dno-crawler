use axum::{
    extract::{State, Path, Query},
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared::{
    DataSourceV2, SourceType, ExtractionMethod, CrawlPath, NavigationStep,
    DiscoveryTracking, DiscoveryType, DiscoveryVerificationStatus
};
use tracing::{info, warn, error};
use uuid::Uuid;
use crate::AppState;

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct ReverseCrawlRequest {
    pub source_urls: Vec<String>,
    pub target_dno_key: String,
    pub target_year: i32,
    pub analysis_depth: Option<u32>, // How deep to analyze the source structure
    pub extract_patterns: Option<bool>, // Whether to learn patterns from the source
}

#[derive(Debug, Deserialize)]
pub struct SourceAnalysisRequest {
    pub source_url: String,
    pub analysis_type: String, // "structure", "content", "patterns", "all"
    pub include_metadata: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub discovery_type: Option<String>,
    pub verification_status: Option<String>,
    pub confidence_threshold: Option<f64>,
    pub days_back: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ReverseCrawlResult {
    pub crawl_id: String,
    pub discovered_sources: Vec<DataSourceV2>,
    pub learned_patterns: Vec<String>, // Pattern IDs
    pub navigation_paths: Vec<CrawlPath>,
    pub confidence_score: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SourceAnalysisResult {
    pub source_url: String,
    pub analysis_type: String,
    pub structure_analysis: Option<Value>,
    pub content_analysis: Option<Value>,
    pub pattern_analysis: Option<Value>,
    pub metadata: Option<Value>,
    pub confidence_score: f64,
    pub recommendations: Vec<String>,
}

/// Trigger reverse crawling from known good sources
pub async fn trigger_reverse_crawl(
    State(state): State<AppState>,
    Json(req): Json<ReverseCrawlRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    info!("Triggering reverse crawl for {} sources targeting {} {}", 
          req.source_urls.len(), req.target_dno_key, req.target_year);

    match state.reverse_crawl_service.trigger_reverse_crawl(req).await {
        Ok(result) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "crawl_id": result.crawl_id,
                    "discovered_sources_count": result.discovered_sources.len(),
                    "learned_patterns_count": result.learned_patterns.len(),
                    "navigation_paths_count": result.navigation_paths.len(),
                    "confidence_score": result.confidence_score,
                    "recommendations": result.recommendations,
                    "status": "completed"
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to trigger reverse crawl: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "REVERSE_CRAWL_FAILED",
                        "message": format!("Failed to trigger reverse crawl: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Analyze a specific source for reverse engineering
pub async fn analyze_source(
    State(state): State<AppState>,
    Json(req): Json<SourceAnalysisRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    info!("Analyzing source: {} (type: {})", req.source_url, req.analysis_type);

    match state.reverse_crawl_service.analyze_source(req).await {
        Ok(result) => {
            Ok(Json(json!({
                "success": true,
                "data": result,
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to analyze source: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_ANALYSIS_FAILED",
                        "message": format!("Failed to analyze source: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get discovered sources from reverse crawling
pub async fn get_discovered_sources(
    State(state): State<AppState>,
    Query(params): Query<DiscoveryQueryParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.reverse_crawl_service.get_discovered_sources(params.into()).await {
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
            error!("Failed to get discovered sources: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "DISCOVERED_SOURCES_FETCH_FAILED",
                        "message": format!("Failed to fetch discovered sources: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get reverse crawl history and results
pub async fn get_reverse_crawl_history(
    State(state): State<AppState>,
    Query(params): Query<DiscoveryQueryParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.reverse_crawl_service.get_crawl_history(params.into()).await {
        Ok(history) => {
            let total = history.len();
            let offset = params.offset.unwrap_or(0) as usize;
            let limit = params.limit.unwrap_or(50) as usize;
            
            let paginated_history: Vec<_> = history
                .into_iter()
                .skip(offset)
                .take(limit)
                .collect();

            Ok(Json(json!({
                "success": true,
                "data": paginated_history,
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
            error!("Failed to get reverse crawl history: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "REVERSE_CRAWL_HISTORY_FETCH_FAILED",
                        "message": format!("Failed to fetch reverse crawl history: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get navigation paths discovered during reverse crawling
pub async fn get_navigation_paths(
    State(state): State<AppState>,
    Path(crawl_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.reverse_crawl_service.get_navigation_paths(crawl_id.clone()).await {
        Ok(paths) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "crawl_id": crawl_id,
                    "navigation_paths": paths,
                    "path_count": paths.len(),
                    "average_depth": paths.iter().map(|p| p.max_depth_reached).sum::<i32>() as f64 / paths.len() as f64,
                    "success_rate": paths.iter().filter(|p| p.success_confidence > 0.8).count() as f64 / paths.len() as f64
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get navigation paths: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "NAVIGATION_PATHS_FETCH_FAILED",
                        "message": format!("Failed to fetch navigation paths: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Compare sources to find similarities for reverse engineering
pub async fn compare_sources(
    State(state): State<AppState>,
    Json(source_urls): Json<Vec<String>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    info!("Comparing {} sources for similarities", source_urls.len());

    match state.reverse_crawl_service.compare_sources(source_urls).await {
        Ok(comparison) => {
            Ok(Json(json!({
                "success": true,
                "data": comparison,
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to compare sources: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "SOURCE_COMPARISON_FAILED",
                        "message": format!("Failed to compare sources: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get discovery tracking data
pub async fn get_discovery_tracking(
    State(state): State<AppState>,
    Query(params): Query<DiscoveryQueryParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.reverse_crawl_service.get_discovery_tracking(params.into()).await {
        Ok(discoveries) => {
            let total = discoveries.len();
            let offset = params.offset.unwrap_or(0) as usize;
            let limit = params.limit.unwrap_or(50) as usize;
            
            let paginated_discoveries: Vec<_> = discoveries
                .into_iter()
                .skip(offset)
                .take(limit)
                .collect();

            // Calculate some statistics
            let verified_count = paginated_discoveries.iter()
                .filter(|d| d.verification_status == DiscoveryVerificationStatus::Verified)
                .count();
            
            let pending_count = paginated_discoveries.iter()
                .filter(|d| d.verification_status == DiscoveryVerificationStatus::Pending)
                .count();

            Ok(Json(json!({
                "success": true,
                "data": paginated_discoveries,
                "metadata": {
                    "total": total,
                    "offset": offset,
                    "limit": limit,
                    "verified_count": verified_count,
                    "pending_count": pending_count,
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get discovery tracking: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "DISCOVERY_TRACKING_FETCH_FAILED",
                        "message": format!("Failed to fetch discovery tracking: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Verify a discovered entity
pub async fn verify_discovery(
    State(state): State<AppState>,
    Path(discovery_id): Path<String>,
    Json(verification_status): Json<DiscoveryVerificationStatus>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.reverse_crawl_service.verify_discovery(discovery_id.clone(), verification_status).await {
        Ok(()) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "discovery_id": discovery_id,
                    "verification_status": verification_status,
                    "verified_at": chrono::Utc::now()
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to verify discovery: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "DISCOVERY_VERIFICATION_FAILED",
                        "message": format!("Failed to verify discovery: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get reverse crawl analytics and insights
pub async fn get_reverse_crawl_analytics(
    State(state): State<AppState>,
    Query(params): Query<DiscoveryQueryParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // In a real implementation, this would gather comprehensive analytics
    let analytics_data = json!({
        "summary": {
            "total_reverse_crawls": 45,
            "successful_discoveries": 127,
            "learned_patterns": 89,
            "verified_sources": 103,
            "pending_verification": 24
        },
        "discovery_trends": {
            "discoveries_per_week": 15.7,
            "verification_rate": 81.9,
            "pattern_extraction_success": 72.4,
            "source_quality_score": 87.3
        },
        "source_types": {
            "pdf": 45,
            "webpage": 82,
            "api": 12,
            "image": 8,
            "text": 3
        },
        "extraction_methods": {
            "pdf_analysis": 58,
            "table_extraction": 34,
            "text_parsing": 28,
            "ocr": 7
        },
        "top_performing_sources": [
            {
                "source_url": "https://example-dno.de/data",
                "success_rate": 94.2,
                "patterns_learned": 12,
                "confidence_score": 0.89
            }
        ],
        "recommendations": [
            "Focus on PDF sources for better data extraction",
            "Improve OCR accuracy for image-based sources",
            "Investigate low-performing webpage sources"
        ]
    });

    Ok(Json(json!({
        "success": true,
        "data": analytics_data,
        "metadata": {
            "timestamp": chrono::Utc::now(),
            "version": "1.0.0"
        }
    })))
}

/// Export reverse crawl results
pub async fn export_reverse_crawl_results(
    State(state): State<AppState>,
    Path(crawl_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.reverse_crawl_service.export_crawl_results(crawl_id.clone()).await {
        Ok(export_data) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "crawl_id": crawl_id,
                    "export_format": "json",
                    "export_data": export_data,
                    "exported_at": chrono::Utc::now()
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to export reverse crawl results: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "REVERSE_CRAWL_EXPORT_FAILED",
                        "message": format!("Failed to export reverse crawl results: {}", err)
                    }
                }))
            ))
        }
    }
}

// Additional route handler functions that match the routes.rs definitions

/// Start a reverse crawl session (alias for trigger_reverse_crawl)
pub async fn start_reverse_crawl(
    state: State<AppState>,
    req: Json<ReverseCrawlRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    trigger_reverse_crawl(state, req).await
}

/// Get reverse crawl session status
pub async fn get_reverse_crawl_status(
    State(_state): State<AppState>,
    Path(session_id): Path<String>,
) -> Json<Value> {
    // TODO: Implement actual session status retrieval
    info!("Getting reverse crawl status for session: {}", session_id);
    
    Json(json!({
        "success": true,
        "data": {
            "session_id": session_id,
            "status": "completed",
            "progress_percentage": 100.0,
            "current_phase": "analysis_complete",
            "discovered_sources": 15,
            "learned_patterns": 8,
            "errors_encountered": 2,
            "started_at": chrono::Utc::now() - chrono::Duration::hours(2),
            "completed_at": chrono::Utc::now(),
            "estimated_completion": null
        }
    }))
}

/// Discover historical data for a specific DNO
pub async fn discover_historical_data(
    State(_state): State<AppState>,
    Path(dno_key): Path<String>,
    Json(discovery_params): Json<Value>,
) -> Json<Value> {
    // TODO: Implement actual historical data discovery
    info!("Discovering historical data for DNO: {} with params: {:?}", dno_key, discovery_params);
    
    let mock_discoveries = vec![
        json!({
            "year": 2020,
            "sources_found": 3,
            "source_types": ["pdf", "webpage"],
            "confidence_score": 0.87,
            "data_completeness": 0.92,
            "urls": [
                "https://example.de/archive/2020/netzentgelte.pdf",
                "https://example.de/tariff-history/2020"
            ]
        }),
        json!({
            "year": 2021,
            "sources_found": 5,
            "source_types": ["pdf", "webpage", "api"],
            "confidence_score": 0.94,
            "data_completeness": 0.98,
            "urls": [
                "https://example.de/archive/2021/netzentgelte.pdf",
                "https://example.de/api/tariffs/2021"
            ]
        })
    ];
    
    Json(json!({
        "success": true,
        "data": {
            "dno_key": dno_key,
            "discovery_session_id": Uuid::new_v4(),
            "discovered_years": mock_discoveries,
            "total_sources_found": 8,
            "average_confidence": 0.905,
            "recommendations": [
                "High confidence data available for 2021-2022",
                "Investigate additional sources for pre-2020 data",
                "API endpoints available for recent years"
            ],
            "discovered_at": chrono::Utc::now()
        }
    }))
}

/// Get historical patterns for a specific DNO
pub async fn get_historical_patterns(
    State(_state): State<AppState>,
    Path(dno_key): Path<String>,
) -> Json<Value> {
    // TODO: Implement actual historical pattern retrieval
    info!("Getting historical patterns for DNO: {}", dno_key);
    
    let mock_patterns = vec![
        json!({
            "pattern_id": "hist_001",
            "pattern_type": "archive_url",
            "pattern_signature": format!("https://*.{}.de/archive/{{year}}/netzentgelte.pdf", dno_key),
            "years_successful": [2018, 2019, 2020, 2021, 2022],
            "years_failed": [2017],
            "confidence_score": 0.89,
            "last_successful": "2022",
            "data_quality": 0.94,
            "extraction_method": "pdf_analysis"
        }),
        json!({
            "pattern_id": "hist_002",
            "pattern_type": "legacy_navigation",
            "pattern_signature": "navigate[text='Archiv'] -> select[year] -> download[type='tariff']",
            "years_successful": [2015, 2016, 2017, 2018],
            "years_failed": [],
            "confidence_score": 0.76,
            "last_successful": "2018",
            "data_quality": 0.82,
            "extraction_method": "webpage_scraping"
        })
    ];
    
    Json(json!({
        "success": true,
        "data": {
            "dno_key": dno_key,
            "historical_patterns": mock_patterns,
            "pattern_coverage": {
                "earliest_year": 2015,
                "latest_year": 2022,
                "coverage_percentage": 87.5,
                "gaps": [2017]
            },
            "success_metrics": {
                "average_confidence": 0.825,
                "average_data_quality": 0.88,
                "pattern_reliability": 0.92
            },
            "recommendations": [
                "Archive URL pattern highly reliable for recent years",
                "Legacy navigation pattern good for historical data",
                "Investigate alternative sources for 2017 gap"
            ]
        }
    }))
}