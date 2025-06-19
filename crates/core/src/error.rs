use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: admin access required")]
    Forbidden(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Too many requests")]
    TooManyRequests,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,           // 400
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,       // 401
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,             // 403
            AppError::NotFound(_) => StatusCode::NOT_FOUND,              // 404
            AppError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,  // 429
            _ => StatusCode::INTERNAL_SERVER_ERROR,                      // 500
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Database(_) => "database_error",
            AppError::Http(_) => "http_error",
            AppError::Json(_) => "json_error",
            AppError::Config(_) => "config_error",
            AppError::Unauthorized(_) => "unauthorized",
            AppError::Forbidden(_) => "forbidden",
            AppError::BadRequest(_) => "bad_request",
            AppError::NotFound(_) => "not_found",
            AppError::TooManyRequests => "too_many_requests",
            AppError::Io(_) => "io_error",
            AppError::InternalServerError(_) => "internal_server_error",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let request_id = Uuid::new_v4();
        
        let body = Json(json!({
            "error": self.error_code(),
            "message": self.to_string(),
            "details": {},
            "request_id": request_id
        }));
        
        (status, body).into_response()
    }
}