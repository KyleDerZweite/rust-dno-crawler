use axum::{extract::State, http::StatusCode, response::Json, Extension};
use serde_json::{json, Value};
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::{AppState, AuthenticatedUser, middleware::{generate_jwt_token, hash_password, verify_password}};
use core::models::*;

pub async fn login(
    State(state): State<AppState>, 
    Json(request): Json<LoginRequest>
) -> Result<Json<Value>, StatusCode> {
    // Input validation
    if request.email.is_empty() || request.password.is_empty() {
        return Ok(Json(json!({
            "error": "validation_error",
            "message": "Email and password are required",
            "details": {},
            "request_id": Uuid::new_v4().to_string()
        })));
    }

    // Get user by email using cached repository
    let user = match state.user_repo.get_user_by_email(&request.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok(Json(json!({
                "error": "invalid_credentials",
                "message": "Invalid email or password",
                "details": {},
                "request_id": Uuid::new_v4().to_string()
            })));
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Check if user is active
    if !user.is_active || user.deleted_at.is_some() {
        return Ok(Json(json!({
            "error": "account_disabled",
            "message": "Account has been disabled",
            "details": {},
            "request_id": Uuid::new_v4().to_string()
        })));
    }

    // Verify password
    let password_valid = match verify_password(&request.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if !password_valid {
        return Ok(Json(json!({
            "error": "invalid_credentials",
            "message": "Invalid email or password",
            "details": {},
            "request_id": Uuid::new_v4().to_string()
        })));
    }

    // Generate session and tokens
    let session_id = Uuid::new_v4();
    let access_token_expiry = Duration::seconds(state.config.jwt_access_token_expiry);
    let refresh_token_expiry = Duration::seconds(state.config.jwt_refresh_token_expiry);

    let access_token = match generate_jwt_token(
        &user,
        session_id,
        &state.jwt_secret,
        state.config.jwt_access_token_expiry,
    ) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let refresh_token = match generate_jwt_token(
        &user,
        session_id,
        &state.jwt_secret,
        state.config.jwt_refresh_token_expiry,
    ) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Hash tokens for storage
    let access_token_hash = format!("{:x}", md5::compute(&access_token));
    let refresh_token_hash = format!("{:x}", md5::compute(&refresh_token));

    // Create session in database
    let session = CreateSession {
        user_id: user.id,
        token_hash: access_token_hash,
        refresh_token_hash: Some(refresh_token_hash),
        expires_at: Utc::now() + access_token_expiry,
        refresh_expires_at: Some(Utc::now() + refresh_token_expiry),
        ip_address: None, // TODO: Extract from request
        user_agent: None, // TODO: Extract from request
    };

    if let Err(_) = state.user_repo.create_session(session).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Prepare response
    let user_public = UserPublic::from(user.clone());
    let tokens = TokenPair {
        access_token,
        refresh_token,
        expires_in: state.config.jwt_access_token_expiry,
    };

    let message = match user.role {
        UserRole::Pending => Some("Account pending approval. Contact admin for verification.".to_string()),
        _ => None,
    };

    Ok(Json(json!({
        "user": user_public,
        "tokens": tokens,
        "message": message
    })))
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>
) -> Result<Json<Value>, StatusCode> {
    // Input validation
    if request.email.is_empty() || request.password.is_empty() || request.name.is_empty() {
        return Ok(Json(json!({
            "error": "validation_error",
            "message": "Email, password, and name are required",
            "details": {},
            "request_id": Uuid::new_v4().to_string()
        })));
    }

    // Basic email validation
    if !request.email.contains('@') {
        return Ok(Json(json!({
            "error": "validation_error",
            "message": "Invalid email format",
            "details": {},
            "request_id": Uuid::new_v4().to_string()
        })));
    }

    // Password strength validation (basic)
    if request.password.len() < 8 {
        return Ok(Json(json!({
            "error": "validation_error",
            "message": "Password must be at least 8 characters long",
            "details": {},
            "request_id": Uuid::new_v4().to_string()
        })));
    }

    // Check if user already exists using cached repository
    match state.user_repo.get_user_by_email(&request.email).await {
        Ok(Some(_)) => {
            return Ok(Json(json!({
                "error": "user_exists",
                "message": "User with this email already exists",
                "details": {},
                "request_id": Uuid::new_v4().to_string()
            })));
        }
        Ok(None) => {}, // Good, user doesn't exist
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    // Hash password
    let password_hash = match hash_password(&request.password) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Create user (default role is pending)
    let create_user = CreateUser {
        email: request.email,
        password_hash,
        name: request.name,
        role: Some(UserRole::Pending),
    };

    let user = match state.user_repo.create_user(create_user).await {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Generate session and tokens
    let session_id = Uuid::new_v4();
    let access_token_expiry = Duration::seconds(state.config.jwt_access_token_expiry);
    let refresh_token_expiry = Duration::seconds(state.config.jwt_refresh_token_expiry);

    let access_token = match generate_jwt_token(
        &user,
        session_id,
        &state.jwt_secret,
        state.config.jwt_access_token_expiry,
    ) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let refresh_token = match generate_jwt_token(
        &user,
        session_id,
        &state.jwt_secret,
        state.config.jwt_refresh_token_expiry,
    ) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Hash tokens for storage
    let access_token_hash = format!("{:x}", md5::compute(&access_token));
    let refresh_token_hash = format!("{:x}", md5::compute(&refresh_token));

    // Create session in database
    let session = CreateSession {
        user_id: user.id,
        token_hash: access_token_hash,
        refresh_token_hash: Some(refresh_token_hash),
        expires_at: Utc::now() + access_token_expiry,
        refresh_expires_at: Some(Utc::now() + refresh_token_expiry),
        ip_address: None,
        user_agent: None,
    };

    if let Err(_) = state.user_repo.create_session(session).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Prepare response
    let user_public = UserPublic::from(user);
    let tokens = TokenPair {
        access_token,
        refresh_token,
        expires_in: state.config.jwt_access_token_expiry,
    };

    Ok(Json(json!({
        "user": user_public,
        "tokens": tokens,
        "message": "Account created successfully. Awaiting admin approval to access full features."
    })))
}

pub async fn refresh(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement token refresh logic
    // This would extract refresh token from header, validate it, and issue new access token
    Ok(Json(json!({
        "error": "not_implemented",
        "message": "Token refresh not yet implemented",
        "details": {},
        "request_id": Uuid::new_v4().to_string()
    })))
}

pub async fn logout(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>
) -> Result<Json<Value>, StatusCode> {
    // Invalidate user's session using cached repository
    if let Err(_) = state.user_repo.invalidate_session(user.session_id).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(json!({
        "message": "Logged out successfully"
    })))
}