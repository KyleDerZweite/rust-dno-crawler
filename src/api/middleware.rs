use crate::{
    auth::jwt::JwtService,
    core::{database::Database, models::UserResponse},
};
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct AuthState {
    pub database: Database,
    pub jwt_service: JwtService,
}

pub async fn jwt_auth(
    State(state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extrahiere Authorization Header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extrahiere Token
    let token = JwtService::extract_token_from_header(auth_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Verifiziere Token
    let claims = state
        .jwt_service
        .verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Lade Benutzer aus Datenbank
    let user = state
        .database
        .get_user_by_id(&claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Überprüfe ob Benutzer API-Zugriff hat (nicht guest)
    if user.role == "guest" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Füge Benutzer zu Request hinzu
    request.extensions_mut().insert(UserResponse::from(user));

    Ok(next.run(request).await)
}

pub async fn admin_auth(
    State(state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Führe zuerst normale JWT-Auth durch
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = JwtService::extract_token_from_header(auth_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = state
        .jwt_service
        .verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = state
        .database
        .get_user_by_id(&claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Überprüfe Admin-Berechtigung
    if user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Füge Benutzer zu Request hinzu
    request.extensions_mut().insert(UserResponse::from(user));

    Ok(next.run(request).await)
}