mod account;
mod admin;
mod auth;
mod dashboard;
mod files;
mod health;
mod metrics;
mod query;
mod websocket;

use axum::{
    Router,
    routing::{get, post, put, delete, patch},
};
use crate::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        // Public endpoints (no auth required)
        .route("/health", get(health::health_check))
        .route("/ready", get(health::readiness_check))
        .nest("/auth", auth_routes())
        // User authenticated endpoints
        .nest("/query", query_routes())
        .nest("/dashboard", dashboard_routes())
        .nest("/account", account_routes())
        // Admin only endpoints
        .nest("/admin", admin_routes())
        .nest("/metrics", metrics_routes())
        .nest("/files", files_routes())
        .route("/ws", get(websocket::websocket_handler))
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/refresh", post(auth::refresh))
        .route("/logout", post(auth::logout))
}

fn query_routes() -> Router<AppState> {
    use axum::middleware;
    use crate::middleware::user_auth_middleware;
    
    Router::new()
        .route("/natural", post(query::natural_query))
        .layer(middleware::from_fn_with_state(AppState::default(), user_auth_middleware))
}

fn dashboard_routes() -> Router<AppState> {
    use axum::middleware;
    use crate::middleware::user_auth_middleware;
    
    Router::new()
        .route("/stats", get(dashboard::get_stats))
        .route("/history", get(dashboard::get_history))
        .route("/history/:id", delete(dashboard::delete_history))
        .layer(middleware::from_fn_with_state(AppState::default(), user_auth_middleware))
}

fn account_routes() -> Router<AppState> {
    use axum::middleware;
    use crate::middleware::{user_auth_middleware, pending_allowed_middleware};
    
    Router::new()
        // Profile GET is allowed for pending users (read-only)
        .route("/profile", get(account::get_profile))
        .layer(middleware::from_fn_with_state(AppState::default(), pending_allowed_middleware))
        .merge(
            Router::new()
                // All other account endpoints require user/admin role
                .route("/profile", patch(account::update_profile))
                .route("/change-email", post(account::change_email))
                .route("/change-password", post(account::change_password))
                .route("/profile-picture", post(account::upload_profile_picture))
                .route("/profile-picture", delete(account::delete_profile_picture))
                .route("/api-keys", get(account::list_api_keys))
                .route("/api-keys", post(account::create_api_key))
                .route("/api-keys/:id", delete(account::delete_api_key))
                .route("/", delete(account::delete_account))
                .layer(middleware::from_fn_with_state(AppState::default(), user_auth_middleware))
        )
}

fn admin_routes() -> Router<AppState> {
    use axum::middleware;
    use crate::middleware::admin_auth_middleware;
    
    Router::new()
        .route("/overview", get(admin::get_overview))
        .route("/users", get(admin::list_users))
        .route("/users/:id", patch(admin::update_user))
        .route("/users/:id", delete(admin::delete_user))
        .route("/users/:id/approve", post(admin::approve_user))
        .route("/users/:id/reject", post(admin::reject_user))
        .route("/data-entries", get(admin::list_data_entries))
        .route("/data-entries/:id", get(admin::get_data_entry))
        .route("/data-entries/:id/source", get(admin::get_data_entry_source))
        .route("/data-entries/:id/verify", post(admin::verify_data_entry))
        .route("/data-entries/:id", patch(admin::update_data_entry))
        .route("/data-entries/:id", delete(admin::delete_data_entry))
        .route("/data-entries/bulk", post(admin::bulk_data_entries))
        .route("/crawl-settings", get(admin::get_crawl_settings))
        .route("/crawl-settings", patch(admin::update_crawl_settings))
        .route("/queries", get(admin::get_queries))
        .route("/cache/status", get(admin::get_cache_status))
        .route("/cache/clear", post(admin::clear_cache))
        .route("/jobs/automated", get(admin::list_automated_jobs))
        .route("/jobs/automated", post(admin::create_automated_job))
        .route("/logs", get(admin::get_logs))
        .route("/crawl/trigger", post(admin::trigger_crawl))
        .route("/metrics/dashboard", get(admin::get_metrics_dashboard))
        .route("/metrics/query", post(admin::query_metrics))
        .route("/metrics/export", get(admin::export_metrics))
        .route("/metrics/timeseries", get(admin::get_timeseries))
        .layer(middleware::from_fn_with_state(AppState::default(), admin_auth_middleware))
}

fn metrics_routes() -> Router<AppState> {
    use axum::middleware;
    use crate::middleware::admin_auth_middleware;
    
    Router::new()
        .route("/", get(metrics::get_prometheus_metrics))
        .layer(middleware::from_fn_with_state(AppState::default(), admin_auth_middleware))
}

fn files_routes() -> Router<AppState> {
    use axum::middleware;
    use crate::middleware::user_auth_middleware;
    
    Router::new()
        .route("/:type/:id", get(files::download_file))
        .layer(middleware::from_fn_with_state(AppState::default(), user_auth_middleware))
}