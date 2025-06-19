use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{json, Value};
use crate::AppState;

pub async fn login(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual login logic here
    // For now, fallback to mock
    _login(State(state)).await
}

pub async fn _login(State(_state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "user": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "email": "user@example.com",
            "name": "John Doe",
            "role": "user",
            "created_at": "2024-01-15T09:00:00Z"
        },
        "tokens": {
            "access_token": "eyJhbGciOiJIUzI1NiIs...",
            "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
            "expires_in": 3600
        }
    })))
}

pub async fn register(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual registration logic here
    // For now, fallback to mock
    _register(State(state)).await
}

pub async fn _register(State(_state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "user": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "email": "user@example.com",
            "name": "John Doe",
            "role": "pending",
            "created_at": "2024-01-15T09:00:00Z",
            "verification_status": "awaiting_approval"
        },
        "tokens": {
            "access_token": "eyJhbGciOiJIUzI1NiIs...",
            "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
            "expires_in": 3600
        },
        "message": "Account created successfully. Awaiting admin approval to access full features."
    })))
}

pub async fn refresh(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual refresh logic here
    // For now, fallback to mock
    _refresh(State(state)).await
}

pub async fn _refresh(State(_state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "access_token": "eyJhbGciOiJIUzI1NiIs...",
        "expires_in": 3600
    })))
}

pub async fn logout(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual logout logic here
    // For now, fallback to mock
    _logout(State(state)).await
}

pub async fn _logout(State(_state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "message": "Logged out successfully"
    })))
}