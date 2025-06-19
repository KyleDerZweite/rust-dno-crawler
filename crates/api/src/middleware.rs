use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{Json, Response},
};
use serde_json::{json, Value};
use crate::AppState;

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Pending,
    User,
    Admin,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
    pub email: String,
    pub role: UserRole,
    pub name: String,
}

/// Extract user from authorization header (mock implementation)
async fn extract_user_from_token(token: &str) -> Result<AuthenticatedUser, AuthError> {
    // TODO: Implement actual JWT token validation
    // For now, return mock data based on token pattern
    match token {
        "admin_token" => Ok(AuthenticatedUser {
            id: "admin-123".to_string(),
            email: "admin@example.com".to_string(),
            role: UserRole::Admin,
            name: "Admin User".to_string(),
        }),
        "user_token" => Ok(AuthenticatedUser {
            id: "user-123".to_string(),
            email: "user@example.com".to_string(),
            role: UserRole::User,
            name: "Regular User".to_string(),
        }),
        "pending_token" => Ok(AuthenticatedUser {
            id: "pending-123".to_string(),
            email: "pending@example.com".to_string(),
            role: UserRole::Pending,
            name: "Pending User".to_string(),
        }),
        _ => Err(AuthError::InvalidToken),
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    PendingApproval,
    InsufficientPermissions,
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
                    "request_id": "550e8400-e29b-41d4-a716-446655440000"
                }))
            ),
            AuthError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "invalid_token",
                    "message": "Invalid or expired authentication token",
                    "details": {},
                    "request_id": "550e8400-e29b-41d4-a716-446655440000"
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
                    "request_id": "550e8400-e29b-41d4-a716-446655440000"
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
                    "request_id": "550e8400-e29b-41d4-a716-446655440000"
                }))
            ),
        }
    }
}

/// Middleware that requires user authentication (user or admin role)
pub async fn user_auth_middleware(
    State(_state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let token = extract_bearer_token(&headers).map_err(|e| e.to_response(&None))?;
    let user = extract_user_from_token(&token).await.map_err(|e| e.to_response(&None))?;

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
    State(_state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let token = extract_bearer_token(&headers).map_err(|e| e.to_response(&None))?;
    let user = extract_user_from_token(&token).await.map_err(|e| e.to_response(&None))?;

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
    State(_state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let token = extract_bearer_token(&headers).map_err(|e| e.to_response(&None))?;
    let user = extract_user_from_token(&token).await.map_err(|e| e.to_response(&None))?;

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