use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{Json, Response},
};
use serde_json::{json, Value};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::AppState;

// Re-export UserRole from core crate
pub use core::models::UserRole;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub name: String,
    pub session_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub name: String,
    pub role: String,
    pub session_id: String,
    pub exp: i64,
    pub iat: i64,
}

/// Extract user from JWT token using cached repositories
async fn extract_user_from_token(
    token: &str,
    jwt_secret: &str,
    user_repo: &crate::UserRepository<crate::RedisCache>,
) -> Result<AuthenticatedUser, AuthError> {
    // Decode JWT token
    let validation = Validation::new(Algorithm::HS256);
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
    
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|_| AuthError::InvalidToken)?;

    let claims = token_data.claims;

    // Parse user ID
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AuthError::InvalidToken)?;

    // Parse session ID
    let session_id = Uuid::parse_str(&claims.session_id)
        .map_err(|_| AuthError::InvalidToken)?;

    // Verify session is still active using cached repository
    let session = user_repo.get_session_by_token_hash(&format!("{:x}", md5::compute(token)))
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    if session.is_none() {
        return Err(AuthError::InvalidToken);
    }

    // Get user from cached repository
    let user = user_repo.get_user_by_id(user_id)
        .await
        .map_err(|_| AuthError::InvalidToken)?
        .ok_or(AuthError::InvalidToken)?;

    // Check if user is still active
    if !user.is_active || user.deleted_at.is_some() {
        return Err(AuthError::InvalidToken);
    }

    // Parse role
    let role = match claims.role.as_str() {
        "pending" => UserRole::Pending,
        "user" => UserRole::User,
        "admin" => UserRole::Admin,
        _ => return Err(AuthError::InvalidToken),
    };

    // Update session last used timestamp
    let _ = user_repo.update_session_last_used(session_id).await;

    Ok(AuthenticatedUser {
        id: user_id,
        email: claims.email,
        role,
        name: claims.name,
        session_id,
    })
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    PendingApproval,
    InsufficientPermissions,
    DatabaseError,
}

impl AuthError {
    fn to_response(&self, current_role: Option<&UserRole>) -> (StatusCode, Json<Value>) {
        match self {
            AuthError::MissingToken => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "missing_token",
                    "message": "Authorization header with Bearer token is required",
                    "details": {},
                    "request_id": uuid::Uuid::new_v4().to_string()
                }))
            ),
            AuthError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "invalid_token",
                    "message": "Invalid or expired authentication token",
                    "details": {},
                    "request_id": uuid::Uuid::new_v4().to_string()
                }))
            ),
            AuthError::PendingApproval => (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "error": "access_denied",
                    "message": "Account pending approval. Contact admin for verification.",
                    "details": {
                        "role": "pending",
                        "verification_status": "awaiting_approval"
                    },
                    "request_id": uuid::Uuid::new_v4().to_string()
                }))
            ),
            AuthError::InsufficientPermissions => (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "error": "admin_required",
                    "message": "This endpoint requires admin privileges",
                    "details": {
                        "required_role": "admin",
                        "current_role": match current_role {
                            Some(UserRole::User) => "user",
                            Some(UserRole::Pending) => "pending",
                            Some(UserRole::Admin) => "admin",
                            None => "unknown"
                        }
                    },
                    "request_id": uuid::Uuid::new_v4().to_string()
                }))
            ),
            AuthError::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal_error",
                    "message": "Internal server error occurred",
                    "details": {},
                    "request_id": uuid::Uuid::new_v4().to_string()
                }))
            ),
        }
    }
}

/// Middleware that requires user authentication (user or admin role)
pub async fn user_auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let token = extract_bearer_token(&headers).map_err(|e| e.to_response(&None))?;
    let user = extract_user_from_token(&token, &state.jwt_secret, &state.user_repo)
        .await
        .map_err(|e| e.to_response(&None))?;

    // Check if user has sufficient permissions (user or admin)
    match user.role {
        UserRole::User | UserRole::Admin => {
            // Add user to request extensions for handlers to access
            request.extensions_mut().insert(user);
            Ok(next.run(request).await)
        }
        UserRole::Pending => Err(AuthError::PendingApproval.to_response(&Some(&user.role))),
    }
}

/// Middleware that requires admin authentication
pub async fn admin_auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let token = extract_bearer_token(&headers).map_err(|e| e.to_response(&None))?;
    let user = extract_user_from_token(&token, &state.jwt_secret, &state.user_repo)
        .await
        .map_err(|e| e.to_response(&None))?;

    // Check if user has admin permissions
    match user.role {
        UserRole::Admin => {
            // Add user to request extensions for handlers to access
            request.extensions_mut().insert(user);
            Ok(next.run(request).await)
        }
        UserRole::User | UserRole::Pending => {
            Err(AuthError::InsufficientPermissions.to_response(&Some(&user.role)))
        }
    }
}

/// Middleware that allows pending users to access specific endpoints (read-only profile)
pub async fn pending_allowed_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let token = extract_bearer_token(&headers).map_err(|e| e.to_response(&None))?;
    let user = extract_user_from_token(&token, &state.jwt_secret, &state.user_repo)
        .await
        .map_err(|e| e.to_response(&None))?;

    // Allow all authenticated users (including pending)
    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}

/// Extract Bearer token from Authorization header
fn extract_bearer_token(headers: &HeaderMap) -> Result<String, AuthError> {
    let auth_header = headers
        .get("authorization")
        .ok_or(AuthError::MissingToken)?
        .to_str()
        .map_err(|_| AuthError::InvalidToken)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::InvalidToken);
    }

    let token = auth_header.strip_prefix("Bearer ").unwrap();
    if token.is_empty() {
        return Err(AuthError::InvalidToken);
    }

    Ok(token.to_string())
}

/// Generate JWT token for user
pub fn generate_jwt_token(
    user: &core::models::User,
    session_id: Uuid,
    jwt_secret: &str,
    expires_in_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp();
    let expiration = now + expires_in_seconds;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        name: user.name.clone(),
        role: match user.role {
            UserRole::Pending => "pending".to_string(),
            UserRole::User => "user".to_string(),
            UserRole::Admin => "admin".to_string(),
        },
        session_id: session_id.to_string(),
        exp: expiration,
        iat: now,
    };

    let header = jsonwebtoken::Header::new(Algorithm::HS256);
    let encoding_key = jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref());

    jsonwebtoken::encode(&header, &claims, &encoding_key)
}

/// Hash password using bcrypt
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}