use crate::{
    api::{handlers as api_handlers, middleware::{AuthState, jwt_auth}},
    core::database::Database,
    auth::jwt::JwtService,
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn create_api_routes(database: Database, jwt_service: JwtService) -> Router {
    let api_state = api_handlers::ApiState {
        database: database.clone(),
        jwt_service: jwt_service.clone(),
    };

    let auth_state = AuthState {
        database,
        jwt_service,
    };

    Router::new()
        // Public routes - nur für Token-Anfrage
        .route("/auth/login", post(api_handlers::login))

        // Protected routes - für DNO-Daten mit Token
        .route("/dno/data", get(api_handlers::get_dno_data))
        .route("/me", get(api_handlers::me))
        .layer(middleware::from_fn_with_state(
            auth_state,
            jwt_auth,
        ))
        .with_state(api_state)
}