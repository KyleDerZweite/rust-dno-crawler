use axum::{
    Router,
    routing::{get, post, put, delete},
};
use crate::{handlers, AppState};

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/dnos", dno_routes())
        .nest("/search", search_routes())
        .nest("/crawl", crawl_routes())
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(handlers::auth::login))
        .route("/register", post(handlers::auth::register))
        .route("/refresh", post(handlers::auth::refresh_token))
        .route("/logout", post(handlers::auth::logout))
}

fn dno_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::dnos::list_dnos))
        .route("/", post(handlers::dnos::create_dno))
        .route("/:id", get(handlers::dnos::get_dno))
        .route("/:id", put(handlers::dnos::update_dno))
        .route("/:id", delete(handlers::dnos::delete_dno))
}

fn search_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::search::search))
        .route("/history", get(handlers::search::search_history))
}

fn crawl_routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", get(handlers::crawl::list_jobs))
        .route("/jobs", post(handlers::crawl::create_job))
        .route("/jobs/:id", get(handlers::crawl::get_job))
        .route("/jobs/:id/status", get(handlers::crawl::job_status))
}