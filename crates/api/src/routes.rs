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
        .nest("/orchestrator", orchestrator_routes())
        .nest("/admin", admin_routes())
        .nest("/patterns", pattern_routes())
        .nest("/reverse", reverse_crawl_routes())
        .nest("/sources", source_routes())
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

fn orchestrator_routes() -> Router<AppState> {
    Router::new()
        .route("/sessions", post(handlers::orchestrator::start_crawl_session))
        .route("/sessions", get(handlers::orchestrator::get_active_sessions))
        .route("/sessions/:id", get(handlers::orchestrator::get_session_status))
        .route("/sessions/:id/logs", get(handlers::orchestrator::get_session_logs))
        .route("/sessions/:id/cancel", post(handlers::orchestrator::cancel_session))
        .route("/sessions/:id/pause", post(handlers::orchestrator::pause_session))
        .route("/sessions/:id/resume", post(handlers::orchestrator::resume_session))
        .route("/automated", post(handlers::orchestrator::submit_automated_job))
        .route("/stats", get(handlers::orchestrator::get_orchestrator_stats))
}

fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(handlers::admin::get_admin_dashboard))
        .route("/flags", get(handlers::admin::get_data_flags))
        .route("/flags", post(handlers::admin::flag_data))
        .route("/flags/:id/resolve", post(handlers::admin::resolve_flag))
        .route("/patterns", get(handlers::admin::get_patterns_for_verification))
        .route("/patterns/:id/verify", post(handlers::admin::verify_pattern))
        .route("/patterns/bulk-verify", post(handlers::admin::bulk_verify_patterns))
        .route("/sources", get(handlers::admin::get_flagged_sources))
        .route("/sources/:id/verify", post(handlers::admin::verify_source))
        .route("/audit", get(handlers::admin::get_audit_log))
}

fn pattern_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::patterns::get_patterns))
        .route("/:id", get(handlers::patterns::get_pattern))
        .route("/:id", put(handlers::patterns::update_pattern))
        .route("/:id", delete(handlers::patterns::delete_pattern))
        .route("/dno/:dno_key", get(handlers::patterns::get_patterns_for_dno))
        .route("/recommendations/:dno_key/:year", get(handlers::patterns::get_pattern_recommendations))
        .route("/performance", get(handlers::patterns::get_pattern_performance))
}

fn reverse_crawl_routes() -> Router<AppState> {
    Router::new()
        .route("/start", post(handlers::reverse_crawl::start_reverse_crawl))
        .route("/sessions/:id", get(handlers::reverse_crawl::get_reverse_crawl_status))
        .route("/discover/:dno_key", post(handlers::reverse_crawl::discover_historical_data))
        .route("/patterns/:dno_key", get(handlers::reverse_crawl::get_historical_patterns))
}

fn source_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::sources::get_sources))
        .route("/:id", get(handlers::sources::get_source))
        .route("/:id/download", get(handlers::sources::download_source))
        .route("/:id/verify", post(handlers::sources::verify_source_integrity))
        .route("/dno/:dno_key/:year", get(handlers::sources::get_sources_for_dno_year))
        .route("/metadata/:id", get(handlers::sources::get_source_metadata))
        .route("/deduplicate", post(handlers::sources::deduplicate_sources))
}