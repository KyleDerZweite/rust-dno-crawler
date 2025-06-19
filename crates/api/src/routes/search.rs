use axum::{extract::{Query, State}, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct SearchByDnoRequest {
    pub dno_name: Option<String>,
    pub dno_id: Option<Uuid>,
    pub year: Option<i32>,
    pub data_type: Option<String>, // "netzentgelte", "hlzf", or "all"
}

#[derive(Debug, Deserialize)]
pub struct SearchByYearRequest {
    pub year: i32,
    pub dno_name: Option<String>,
    pub dno_id: Option<Uuid>,
    pub data_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchByDataTypeRequest {
    pub data_type: String, // "netzentgelte" or "hlzf"
    pub dno_name: Option<String>,
    pub dno_id: Option<Uuid>,
    pub year: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SearchFilters {
    pub dno_name: Option<String>,
    pub dno_id: Option<Uuid>,
    pub year: Option<i32>,
    pub data_type: Option<String>,
    pub region: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub dno: DnoInfo,
    pub year: i32,
    pub data_type: String,
    pub status: String,
    pub data: Value,
    pub source: SourceInfo,
    pub last_updated: String,
}

#[derive(Debug, Serialize)]
pub struct DnoInfo {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub region: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SourceInfo {
    pub id: Uuid,
    pub file_type: String,
    pub file_url: String,
    pub page: Option<i32>,
    pub extracted_at: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub total: u32,
    pub results: Vec<SearchResult>,
    pub filters_applied: Value,
    pub available_years: Vec<i32>,
    pub available_dnos: Vec<DnoInfo>,
}

/// Search for data by DNO name or ID
pub async fn search_by_dno(
    State(state): State<AppState>,
    Json(request): Json<SearchByDnoRequest>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual database search logic
    // For now, return mock data based on request
    _search_by_dno_mock(State(state), Json(request)).await
}

async fn _search_by_dno_mock(
    State(_state): State<AppState>,
    Json(request): Json<SearchByDnoRequest>,
) -> Result<Json<Value>, StatusCode> {
    let dno_name = request.dno_name.unwrap_or_else(|| "Netze BW".to_string());
    let year = request.year.unwrap_or(2024);
    let data_type = request.data_type.unwrap_or_else(|| "netzentgelte".to_string());

    Ok(Json(json!({
        "total": 1,
        "results": [{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "dno": {
                "id": "123e4567-e89b-12d3-a456-426614174000",
                "name": dno_name,
                "slug": "netze-bw",
                "region": "Baden-Württemberg"
            },
            "year": year,
            "data_type": data_type,
            "status": "verified",
            "data": {
                "netzentgelte": {
                    "hs": {"leistung": 58.21, "arbeit": 1.26},
                    "ms": {"leistung": 109.86, "arbeit": 1.73}
                }
            },
            "source": {
                "id": "660e8400-e29b-41d4-a716-446655440000",
                "file_type": "pdf",
                "file_url": "/files/pdf/660e8400-e29b-41d4-a716-446655440000",
                "page": 12,
                "extracted_at": "2024-01-15T10:00:00Z"
            },
            "last_updated": "2024-01-15T10:00:00Z"
        }],
        "filters_applied": {
            "dno_name": dno_name,
            "year": year,
            "data_type": data_type
        },
        "available_years": [2022, 2023, 2024],
        "available_dnos": [{
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "name": dno_name,
            "slug": "netze-bw",
            "region": "Baden-Württemberg"
        }]
    })))
}

/// Search for data by year
pub async fn search_by_year(
    State(state): State<AppState>,
    Json(request): Json<SearchByYearRequest>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual database search logic
    // For now, return mock data based on request
    _search_by_year_mock(State(state), Json(request)).await
}

async fn _search_by_year_mock(
    State(_state): State<AppState>,
    Json(request): Json<SearchByYearRequest>,
) -> Result<Json<Value>, StatusCode> {
    let year = request.year;
    let data_type = request.data_type.unwrap_or_else(|| "netzentgelte".to_string());

    Ok(Json(json!({
        "total": 2,
        "results": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "dno": {
                    "id": "123e4567-e89b-12d3-a456-426614174000",
                    "name": "Netze BW",
                    "slug": "netze-bw",
                    "region": "Baden-Württemberg"
                },
                "year": year,
                "data_type": data_type.clone(),
                "status": "verified",
                "data": {
                    "netzentgelte": {
                        "hs": {"leistung": 58.21, "arbeit": 1.26},
                        "ms": {"leistung": 109.86, "arbeit": 1.73}
                    }
                },
                "source": {
                    "id": "660e8400-e29b-41d4-a716-446655440000",
                    "file_type": "pdf",
                    "file_url": "/files/pdf/660e8400-e29b-41d4-a716-446655440000",
                    "page": 12,
                    "extracted_at": "2024-01-15T10:00:00Z"
                },
                "last_updated": "2024-01-15T10:00:00Z"
            },
            {
                "id": "660e8400-e29b-41d4-a716-446655440001",
                "dno": {
                    "id": "223e4567-e89b-12d3-a456-426614174001",
                    "name": "Bayernwerk",
                    "slug": "bayernwerk",
                    "region": "Bayern"
                },
                "year": year,
                "data_type": data_type,
                "status": "verified",
                "data": {
                    "netzentgelte": {
                        "hs": {"leistung": 61.45, "arbeit": 1.35},
                        "ms": {"leistung": 115.20, "arbeit": 1.82}
                    }
                },
                "source": {
                    "id": "770e8400-e29b-41d4-a716-446655440001",
                    "file_type": "pdf",
                    "file_url": "/files/pdf/770e8400-e29b-41d4-a716-446655440001",
                    "page": 8,
                    "extracted_at": "2024-01-16T14:30:00Z"
                },
                "last_updated": "2024-01-16T14:30:00Z"
            }
        ],
        "filters_applied": {
            "year": year,
            "data_type": data_type
        },
        "available_years": [2022, 2023, 2024],
        "available_dnos": [
            {
                "id": "123e4567-e89b-12d3-a456-426614174000",
                "name": "Netze BW",
                "slug": "netze-bw",
                "region": "Baden-Württemberg"
            },
            {
                "id": "223e4567-e89b-12d3-a456-426614174001",
                "name": "Bayernwerk",
                "slug": "bayernwerk",
                "region": "Bayern"
            }
        ]
    })))
}

/// Search for data by data type (netzentgelte or hlzf)
pub async fn search_by_data_type(
    State(state): State<AppState>,
    Json(request): Json<SearchByDataTypeRequest>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual database search logic
    // For now, return mock data based on request
    _search_by_data_type_mock(State(state), Json(request)).await
}

async fn _search_by_data_type_mock(
    State(_state): State<AppState>,
    Json(request): Json<SearchByDataTypeRequest>,
) -> Result<Json<Value>, StatusCode> {
    let data_type = request.data_type;
    let year = request.year.unwrap_or(2024);

    let mock_data = match data_type.as_str() {
        "netzentgelte" => json!({
            "netzentgelte": {
                "hs": {"leistung": 58.21, "arbeit": 1.26},
                "ms": {"leistung": 109.86, "arbeit": 1.73}
            }
        }),
        "hlzf" => json!({
            "hlzf": {
                "winter": {"ht": 145.50, "nt": 98.20},
                "sommer": {"ht": 132.80, "nt": 89.40}
            }
        }),
        _ => json!({})
    };

    Ok(Json(json!({
        "total": 1,
        "results": [{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "dno": {
                "id": "123e4567-e89b-12d3-a456-426614174000",
                "name": "Netze BW",
                "slug": "netze-bw",
                "region": "Baden-Württemberg"
            },
            "year": year,
            "data_type": data_type,
            "status": "verified",
            "data": mock_data,
            "source": {
                "id": "660e8400-e29b-41d4-a716-446655440000",
                "file_type": "pdf",
                "file_url": "/files/pdf/660e8400-e29b-41d4-a716-446655440000",
                "page": 12,
                "extracted_at": "2024-01-15T10:00:00Z"
            },
            "last_updated": "2024-01-15T10:00:00Z"
        }],
        "filters_applied": {
            "data_type": data_type,
            "year": year
        },
        "available_years": [2022, 2023, 2024],
        "available_dnos": [{
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "name": "Netze BW", 
            "slug": "netze-bw",
            "region": "Baden-Württemberg"
        }]
    })))
}

/// Search with multiple filters using query parameters
pub async fn search_with_filters(
    State(state): State<AppState>,
    Query(filters): Query<SearchFilters>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual database search logic
    // For now, return mock data based on filters
    _search_with_filters_mock(State(state), Query(filters)).await
}

async fn _search_with_filters_mock(
    State(_state): State<AppState>,
    Query(filters): Query<SearchFilters>,
) -> Result<Json<Value>, StatusCode> {
    let dno_name = filters.dno_name.unwrap_or_else(|| "Netze BW".to_string());
    let year = filters.year.unwrap_or(2024);
    let data_type = filters.data_type.unwrap_or_else(|| "netzentgelte".to_string());
    let limit = filters.limit.unwrap_or(10);
    let offset = filters.offset.unwrap_or(0);

    Ok(Json(json!({
        "total": 150,
        "results": [{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "dno": {
                "id": "123e4567-e89b-12d3-a456-426614174000", 
                "name": dno_name,
                "slug": "netze-bw",
                "region": filters.region.unwrap_or_else(|| "Baden-Württemberg".to_string())
            },
            "year": year,
            "data_type": data_type,
            "status": "verified",
            "data": {
                "netzentgelte": {
                    "hs": {"leistung": 58.21, "arbeit": 1.26},
                    "ms": {"leistung": 109.86, "arbeit": 1.73}
                }
            },
            "source": {
                "id": "660e8400-e29b-41d4-a716-446655440000",
                "file_type": "pdf",
                "file_url": "/files/pdf/660e8400-e29b-41d4-a716-446655440000",
                "page": 12,
                "extracted_at": "2024-01-15T10:00:00Z"
            },
            "last_updated": "2024-01-15T10:00:00Z"
        }],
        "pagination": {
            "limit": limit,
            "offset": offset,
            "total": 150,
            "has_more": offset + limit < 150
        },
        "filters_applied": {
            "dno_name": dno_name,
            "year": year,
            "data_type": data_type,
            "region": filters.region,
            "limit": limit,
            "offset": offset
        },
        "available_filters": {
            "years": [2022, 2023, 2024],
            "data_types": ["netzentgelte", "hlzf"],
            "regions": ["Baden-Württemberg", "Bayern", "Nordrhein-Westfalen", "Niedersachsen"]
        }
    })))
}