use crate::{
    auth::jwt::JwtService,
    core::{
        database::{Database, UpdateUserRequest},
        errors::AppError,
        models::{LoginRequest, UserResponse},
    },
};
use axum::{
    extract::{State, Query},
    response::Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone)]
pub struct ApiState {
    pub database: Database,
    pub jwt_service: JwtService,
}

#[derive(Deserialize)]
pub struct DnoDataQuery {
    pub dno_name: String,
    pub year: u32,
}

#[derive(Serialize)]
pub struct DnoDataResponse {
    pub dno_name: String,
    pub year: u32,
    pub data: Vec<DnoEntry>,
    pub retrieved_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct DnoEntry {
    pub id: String,
    pub name: String,
    pub value: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn login(
    State(state): State<ApiState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<Value>, AppError> {
    // Finde Benutzer
    let user = state
        .database
        .get_user_by_email(&request.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Überprüfe Passwort
    if !user.verify_password(&request.password)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
    {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Überprüfe ob Benutzer API-Zugriff hat
    if user.role == "guest" {
        return Err(AppError::Forbidden("API access denied for guest users".to_string()));
    }

    // Erstelle Token
    let token = state.jwt_service.create_token(&user)?;

    Ok(Json(json!({
        "access_token": token,
        "token_type": "Bearer",
        "user": {
            "email": user.email,
            "role": user.role
        }
    })))
}

pub async fn me(Extension(user): Extension<UserResponse>) -> Json<UserResponse> {
    Json(user)
}

pub async fn get_dno_data(
    State(_state): State<ApiState>,
    Query(query): Query<DnoDataQuery>,
    Extension(_user): Extension<UserResponse>,
) -> Result<Json<DnoDataResponse>, AppError> {
    // Hier würde die echte DNO-Daten-Abfrage implementiert werden
    // Für jetzt geben wir Beispieldaten zurück

    let sample_data = vec![
        DnoEntry {
            id: "1".to_string(),
            name: format!("{}_data_entry_1", query.dno_name),
            value: "Sample Value 1".to_string(),
            timestamp: chrono::Utc::now(),
        },
        DnoEntry {
            id: "2".to_string(),
            name: format!("{}_data_entry_2", query.dno_name),
            value: "Sample Value 2".to_string(),
            timestamp: chrono::Utc::now(),
        },
    ];

    let response = DnoDataResponse {
        dno_name: query.dno_name,
        year: query.year,
        data: sample_data,
        retrieved_at: chrono::Utc::now(),
    };

    Ok(Json(response))
}