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
        .nest("/intelligent", intelligent_routes())
        .nest("/dno", dno_data_routes())
        .nest("/pdf", pdf_routes())
        .nest("/learning", learning_routes())
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

fn intelligent_routes() -> Router<AppState> {
    Router::new()
        .route("/search", post(handlers::intelligent::intelligent_search))
        .route("/query", post(handlers::intelligent::query_ai_only))
        .route("/health", get(handlers::intelligent::health_ai))
}

fn dno_data_routes() -> Router<AppState> {
    Router::new()
        .route("/query", post(handlers::dno::query_dno_data))
}

fn pdf_routes() -> Router<AppState> {
    Router::new()
        .route("/analyze", post(handlers::dno::analyze_pdf))
}

fn learning_routes() -> Router<AppState> {
    Router::new()
        .route("/stats", get(handlers::dno::get_query_learning_stats))
}