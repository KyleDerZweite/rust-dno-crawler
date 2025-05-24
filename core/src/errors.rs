use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                eprintln!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::Jwt(_) => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AppError::Bcrypt(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::BadRequest(ref message) => (StatusCode::BAD_REQUEST, message.as_str()),
            AppError::Unauthorized(ref message) => (StatusCode::UNAUTHORIZED, message.as_str()),
            AppError::InternalServerError(ref message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.as_str())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}