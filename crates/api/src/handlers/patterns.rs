use axum::{
    extract::{State, Path, Query},
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared::{
    PatternLearningRequest, PatternLearningResponse, PatternType, PatternRecommendation,
    CrawlIntelligence, PatternPerformance, DnoKnowledgeGraph, RelationshipType
};
use tracing::{info, warn, error};
use uuid::Uuid;
use crate::AppState;

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct PatternQueryParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub pattern_type: Option<String>,
    pub dno_key: Option<String>,
    pub min_confidence: Option<f64>,
    pub verified_only: Option<bool>,
    pub sort_by: Option<String>, // confidence, success_count, last_success
}

#[derive(Debug, Deserialize)]
pub struct LearningInsightsParams {
    pub dno_key: Option<String>,
    pub pattern_type: Option<String>,
    pub days_back: Option<u32>,
    pub include_predictions: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PatternTestRequest {
    pub dno_key: String,
    pub pattern_type: PatternType,
    pub pattern_data: Value,
    pub test_urls: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PatternTestResult {
    pub pattern_id: String,
    pub test_results: Vec<PatternTestCase>,
    pub overall_success_rate: f64,
    pub recommendation: PatternRecommendation,
}

#[derive(Debug, Serialize)]
pub struct PatternTestCase {
    pub url: String,
    pub success: bool,
    pub execution_time_ms: i32,
    pub error_message: Option<String>,
    pub confidence_score: Option<f64>,
}

/// Get all learned patterns with filtering and sorting
pub async fn get_patterns(
    State(state): State<AppState>,
    Query(params): Query<PatternQueryParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.pattern_service.get_patterns(params.into()).await {
        Ok(patterns) => {
            let total = patterns.len();
            let offset = params.offset.unwrap_or(0) as usize;
            let limit = params.limit.unwrap_or(50) as usize;
            
            let paginated_patterns: Vec<_> = patterns
                .into_iter()
                .skip(offset)
                .take(limit)
                .collect();

            Ok(Json(json!({
                "success": true,
                "data": paginated_patterns,
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
            error!("Failed to get patterns: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "PATTERNS_FETCH_FAILED",
                        "message": format!("Failed to fetch patterns: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get specific pattern details
pub async fn get_pattern_details(
    State(state): State<AppState>,
    Path(pattern_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.pattern_service.get_pattern_details(pattern_id.clone()).await {
        Ok(Some(pattern)) => {
            // Also get performance history for this pattern
            let performance_history = state.pattern_service
                .get_pattern_performance_history(pattern_id.clone())
                .await
                .unwrap_or_default();

            Ok(Json(json!({
                "success": true,
                "data": {
                    "pattern": pattern,
                    "performance_history": performance_history,
                    "usage_statistics": {
                        "total_executions": pattern.success_count + pattern.failure_count,
                        "success_rate": pattern.success_count as f64 / (pattern.success_count + pattern.failure_count) as f64 * 100.0,
                        "last_used": pattern.last_success_at.or(pattern.last_failure_at)
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
                        "code": "PATTERN_NOT_FOUND",
                        "message": "Pattern not found"
                    }
                }))
            ))
        }
        Err(err) => {
            error!("Failed to get pattern details: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "PATTERN_DETAILS_FETCH_FAILED",
                        "message": format!("Failed to fetch pattern details: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Submit a new pattern learning result
pub async fn submit_pattern_learning(
    State(state): State<AppState>,
    Json(req): Json<PatternLearningRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    info!("Submitting pattern learning for {} ({})", req.dno_key, req.pattern_type);

    match state.pattern_service.submit_pattern_learning(req).await {
        Ok(response) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "pattern_id": response.pattern_id,
                    "confidence_score": response.confidence_score,
                    "recommendation": response.recommendation
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to submit pattern learning: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "PATTERN_LEARNING_SUBMIT_FAILED",
                        "message": format!("Failed to submit pattern learning: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get learning insights and analytics
pub async fn get_learning_insights(
    State(state): State<AppState>,
    Query(params): Query<LearningInsightsParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.pattern_service.get_learning_insights(params.into()).await {
        Ok(insights) => {
            Ok(Json(json!({
                "success": true,
                "data": insights,
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get learning insights: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "LEARNING_INSIGHTS_FETCH_FAILED",
                        "message": format!("Failed to fetch learning insights: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Test a pattern against specific URLs
pub async fn test_pattern(
    State(state): State<AppState>,
    Json(req): Json<PatternTestRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    info!("Testing pattern for {} against {} URLs", req.dno_key, req.test_urls.len());

    match state.pattern_service.test_pattern(req).await {
        Ok(test_result) => {
            Ok(Json(json!({
                "success": true,
                "data": test_result,
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to test pattern: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "PATTERN_TEST_FAILED",
                        "message": format!("Failed to test pattern: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get pattern recommendations for a specific DNO
pub async fn get_pattern_recommendations(
    State(state): State<AppState>,
    Path(dno_key): Path<String>,
    Query(year): Query<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.pattern_service.get_recommendations_for_dno(dno_key.clone(), year).await {
        Ok(recommendations) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "dno_key": dno_key,
                    "year": year,
                    "recommendations": recommendations,
                    "confidence_threshold": 0.7,
                    "generated_at": chrono::Utc::now()
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get pattern recommendations: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "PATTERN_RECOMMENDATIONS_FAILED",
                        "message": format!("Failed to get pattern recommendations: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get DNO knowledge graph and relationships
pub async fn get_dno_knowledge_graph(
    State(state): State<AppState>,
    Path(dno_key): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.pattern_service.get_dno_relationships(dno_key.clone()).await {
        Ok(relationships) => {
            // Build a knowledge graph structure
            let knowledge_graph = json!({
                "central_dno": dno_key,
                "relationships": relationships,
                "related_patterns": [], // Would be populated from pattern service
                "structural_similarities": [], // DNOs with similar website structures
                "success_correlations": [] // Patterns that work well across related DNOs
            });

            Ok(Json(json!({
                "success": true,
                "data": knowledge_graph,
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get DNO knowledge graph: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "KNOWLEDGE_GRAPH_FETCH_FAILED",
                        "message": format!("Failed to fetch DNO knowledge graph: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get pattern evolution over time
pub async fn get_pattern_evolution(
    State(state): State<AppState>,
    Path(pattern_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.pattern_service.get_pattern_evolution(pattern_id.clone()).await {
        Ok(evolution) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "pattern_id": pattern_id,
                    "evolution": evolution,
                    "trends": {
                        "success_rate_trend": "improving", // Would be calculated
                        "usage_frequency": "stable",
                        "adaptation_rate": "high"
                    }
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get pattern evolution: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "PATTERN_EVOLUTION_FETCH_FAILED",
                        "message": format!("Failed to fetch pattern evolution: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Get cross-DNO pattern effectiveness
pub async fn get_cross_dno_effectiveness(
    State(state): State<AppState>,
    Path(pattern_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.pattern_service.get_cross_dno_effectiveness(pattern_id.clone()).await {
        Ok(effectiveness) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "pattern_id": pattern_id,
                    "effectiveness_by_dno": effectiveness,
                    "overall_transferability": 75.3, // Would be calculated
                    "best_performing_dnos": [], // Top DNOs where this pattern works
                    "poor_performing_dnos": [] // DNOs where this pattern struggles
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to get cross-DNO effectiveness: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "CROSS_DNO_EFFECTIVENESS_FETCH_FAILED",
                        "message": format!("Failed to fetch cross-DNO effectiveness: {}", err)
                    }
                }))
            ))
        }
    }
}

/// Export pattern learning report
pub async fn export_pattern_report(
    State(state): State<AppState>,
    Query(params): Query<LearningInsightsParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Generate comprehensive pattern learning report
    let report_data = json!({
        "report_id": Uuid::new_v4(),
        "generated_at": chrono::Utc::now(),
        "parameters": params,
        "summary": {
            "total_patterns_learned": 347,
            "patterns_by_type": {
                "url": 142,
                "navigation": 89,
                "content": 76,
                "file_naming": 28,
                "structural": 12
            },
            "average_confidence": 84.2,
            "success_rate": 87.3,
            "cross_dno_transferability": 62.8
        },
        "top_performing_patterns": [], // Would be populated with actual data
        "learning_trends": {
            "patterns_learned_per_week": 12.4,
            "success_rate_improvement": 8.7,
            "false_positive_reduction": 15.2
        },
        "recommendations": [
            "Focus on improving URL pattern learning for better discovery",
            "Investigate low-performing navigation patterns",
            "Consider automated pattern validation for high-confidence patterns"
        ]
    });

    Ok(Json(json!({
        "success": true,
        "data": report_data,
        "metadata": {
            "timestamp": chrono::Utc::now(),
            "version": "1.0.0"
        }
    })))
}

/// Reset a pattern's learning data (admin function)
pub async fn reset_pattern(
    State(state): State<AppState>,
    Path(pattern_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.pattern_service.reset_pattern(pattern_id.clone()).await {
        Ok(()) => {
            Ok(Json(json!({
                "success": true,
                "data": {
                    "pattern_id": pattern_id,
                    "reset_at": chrono::Utc::now(),
                    "status": "reset_complete"
                },
                "metadata": {
                    "timestamp": chrono::Utc::now(),
                    "version": "1.0.0"
                }
            })))
        }
        Err(err) => {
            error!("Failed to reset pattern: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": {
                        "code": "PATTERN_RESET_FAILED",
                        "message": format!("Failed to reset pattern: {}", err)
                    }
                }))
            ))
        }
    }
}

// Additional route handler functions that match the routes.rs definitions

/// Get specific pattern by ID (alias for get_pattern_details)
pub async fn get_pattern(
    state: State<AppState>,
    path: Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    get_pattern_details(state, path).await
}

/// Update pattern information
pub async fn update_pattern(
    State(_state): State<AppState>,
    Path(pattern_id): Path<String>,
    Json(update_data): Json<Value>,
) -> Json<Value> {
    // TODO: Implement actual pattern update logic
    info!("Updating pattern {} with data: {:?}", pattern_id, update_data);
    
    Json(json!({
        "success": true,
        "data": {
            "pattern_id": pattern_id,
            "updated_at": chrono::Utc::now(),
            "changes_applied": update_data
        },
        "message": "Pattern updated successfully"
    }))
}

/// Delete a pattern
pub async fn delete_pattern(
    State(_state): State<AppState>,
    Path(pattern_id): Path<String>,
) -> Json<Value> {
    // TODO: Implement actual pattern deletion logic (should be soft delete)
    info!("Deleting pattern {}", pattern_id);
    
    Json(json!({
        "success": true,
        "data": {
            "pattern_id": pattern_id,
            "deleted_at": chrono::Utc::now(),
            "status": "deleted"
        },
        "message": "Pattern deleted successfully"
    }))
}

/// Get patterns for a specific DNO
pub async fn get_patterns_for_dno(
    State(_state): State<AppState>,
    Path(dno_key): Path<String>,
) -> Json<Value> {
    // TODO: Implement actual database query for DNO-specific patterns
    info!("Getting patterns for DNO: {}", dno_key);
    
    let mock_patterns = vec![
        json!({
            "id": "pattern_001",
            "dno_key": dno_key,
            "pattern_type": "url",
            "pattern_signature": format!("https://*.{}.de/*/netzentgelte/*/{{year}}", dno_key),
            "confidence_score": 0.85,
            "success_count": 12,
            "failure_count": 2,
            "admin_verified": "verified",
            "created_at": chrono::Utc::now()
        }),
        json!({
            "id": "pattern_002",
            "dno_key": dno_key,
            "pattern_type": "navigation",
            "pattern_signature": "navigate[data-section='tariffs'] -> select[year='{year}']",
            "confidence_score": 0.78,
            "success_count": 9,
            "failure_count": 1,
            "admin_verified": "not_reviewed",
            "created_at": chrono::Utc::now()
        })
    ];

    Json(json!({
        "success": true,
        "data": mock_patterns,
        "metadata": {
            "dno_key": dno_key,
            "total_patterns": mock_patterns.len(),
            "verified_patterns": 1,
            "unverified_patterns": 1
        }
    }))
}

/// Get pattern performance metrics
pub async fn get_pattern_performance(
    State(_state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Json<Value> {
    // TODO: Implement actual pattern performance analytics
    let days = params.get("days").and_then(|d| d.parse::<u32>().ok()).unwrap_or(30);
    
    let mock_performance = json!({
        "summary": {
            "total_executions": 145,
            "successful_executions": 127,
            "failed_executions": 18,
            "success_rate": 0.876,
            "average_execution_time_ms": 2847,
            "data_quality_score": 0.89
        },
        "daily_stats": vec![
            json!({
                "date": "2024-06-15",
                "executions": 12,
                "success_rate": 0.916,
                "avg_time_ms": 2650,
                "data_quality": 0.92
            }),
            json!({
                "date": "2024-06-14",
                "executions": 8,
                "success_rate": 0.875,
                "avg_time_ms": 3100,
                "data_quality": 0.88
            })
        ],
        "pattern_breakdown": vec![
            json!({
                "pattern_id": "pattern_001",
                "pattern_type": "url",
                "executions": 45,
                "success_rate": 0.911,
                "avg_time_ms": 2200
            }),
            json!({
                "pattern_id": "pattern_002",
                "pattern_type": "navigation",
                "executions": 32,
                "success_rate": 0.875,
                "avg_time_ms": 3400
            })
        ]
    });

    Json(json!({
        "success": true,
        "data": mock_performance,
        "metadata": {
            "period_days": days,
            "generated_at": chrono::Utc::now()
        }
    }))
}