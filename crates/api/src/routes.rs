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
        .nest("/dno", dno_data_routes())
        .nest("/admin", admin_routes())
        .nest("/jobs", job_routes())
        .nest("/ai", ai_crawl_routes())
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
        .route("/gather", post(handlers::search::gather))
}

fn crawl_routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", get(handlers::crawl::list_jobs))
        .route("/jobs", post(handlers::crawl::create_job))
        .route("/jobs/:id", get(handlers::crawl::get_job))
        .route("/jobs/:id/status", get(handlers::crawl::job_status))
}

fn dno_data_routes() -> Router<AppState> {
    Router::new()
        .route("/query", post(handlers::dno::query_dno_data))
        .route("/analyze", post(handlers::dno::analyze_pdf))
        .route("/stats", get(handlers::dno::get_query_learning_stats))
}

fn admin_routes() -> Router<AppState> {
    use axum::middleware;
    use crate::middleware::admin_auth_middleware;
    
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
        .layer(middleware::from_fn(admin_auth_middleware))
}

fn job_routes() -> Router<AppState> {
    use axum::middleware;
    use crate::middleware::admin_auth_middleware;
    
    Router::new()
        .route("/", get(handlers::jobs::get_automated_jobs))
        .route("/", post(handlers::jobs::create_automated_job))
        .route("/:job_id", put(handlers::jobs::update_automated_job))
        .route("/:job_id", delete(handlers::jobs::delete_automated_job))
        .route("/:job_id/control", post(handlers::jobs::control_automated_job))
        .route("/:job_id/history", get(handlers::jobs::get_job_execution_history))
        .route("/system/status", get(handlers::jobs::get_job_system_status))
        .layer(middleware::from_fn(admin_auth_middleware))
}

fn ai_crawl_routes() -> Router<AppState> {
    Router::new()
        .route("/crawl", post(handlers::ai_crawl::ai_crawl_intelligent))
        .route("/query", post(handlers::ai_crawl::query_intelligent))
        .route("/status/:dno", get(handlers::ai_crawl::get_ai_status))
        .route("/retrain/:dno", post(handlers::ai_crawl::retrain_ai_model))
        .route("/evaluation/:dno", get(handlers::ai_crawl::get_data_evaluation))
}