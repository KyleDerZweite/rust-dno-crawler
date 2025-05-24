pub mod auth;
pub mod database;
pub mod models;

pub use api_auth::*;
pub use api_database::*;
pub use models::*;

use core::errors::AppError;

use anyhow::Result;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use api_auth::{create_jwt_token, verify_jwt_token};
use models::{DataResponse, LoginRequest, TokenResponse, User};


pub fn create_client() -> Result<ApiClient> {
    println!("Creating API client...");

    // Initialize your API client here

    Ok(ApiClient {})
}

pub struct ApiClient {}

#[derive(Clone)]
pub struct AppState {
    db: DatabasePool,
    jwt_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://username:password@localhost/dbname".to_string());

    let db = DatabasePool::new(&database_url).await?;
    
    let jwt_secret =
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-super-secret-jwt-key".to_string());

    let app_state = AppState { db, jwt_secret };

    let app = Router::new()
        .route("/api/v1/get_token", get(get_token))
        .route("/api/v1/get_data", get(get_data))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_token(
    State(state): State<Arc<AppState>>,
    Query(params): Query<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    // Validate input
    if params.username.is_empty() || params.password.is_empty() {
        return Err(AppError::BadRequest(
            "Username and password are required".into(),
        ));
    }

    // Get user from database
    let user = state.db.get_user_by_username(&params.username).await?;

    match user {
        Some(user) => {
            // Verify password
            if bcrypt::verify(&params.password, &user.password_hash)? {
                // Create JWT token
                let token = create_jwt_token(&user.id.to_string(), &state.jwt_secret)?;
                Ok(Json(TokenResponse { token }))
            } else {
                Err(AppError::Unauthorized("Invalid credentials".into()))
            }
        }
        None => Err(AppError::Unauthorized("Invalid credentials".into())),
    }
}

async fn get_data(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<DataRequest>,
) -> Result<Json<DataResponse>, AppError> {
    // Extract and verify JWT token
    let auth_header = headers
        .get("authorization")
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".into()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid authorization header".into()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized(
            "Invalid authorization format".into(),
        ));
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix
    let _claims = verify_jwt_token(token, &state.jwt_secret)?;

    // Sanitize the dno_name parameter
    let sanitized_name = sanitize_dno_name(&params.dno_name)?;

    // Query the database
    let data = state.db.get_data_by_name(&sanitized_name).await?;

    Ok(Json(DataResponse { data }))
}

fn sanitize_dno_name(name: &str) -> Result<String, AppError> {
    // Remove any potentially dangerous characters
    let re = regex::Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
    let sanitized = re.replace_all(name.trim(), "").to_string();

    if sanitized.is_empty() {
        return Err(AppError::BadRequest("Invalid dno-name parameter".into()));
    }

    if sanitized.len() > 100 {
        return Err(AppError::BadRequest("dno-name parameter too long".into()));
    }

    Ok(sanitized)
}

#[derive(Deserialize)]
struct DataRequest {
    #[serde(rename = "dno-name")]
    dno_name: String,
}
