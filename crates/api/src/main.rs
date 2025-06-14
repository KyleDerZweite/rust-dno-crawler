mod routes;
mod handlers;
mod middleware;
mod database;

use axum::{
    Router,
    routing::{get, post},
    http::StatusCode,
    response::Json,
    middleware as axum_middleware,
};
use serde_json::{json, Value};
use shared::{Config, AppError};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    info!("Starting API server with config: {:?}", config.server);

    // Initialize database
    let db_pool = database::create_pool(&config.database).await?;
    database::run_migrations(&db_pool).await?;

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    // Build the application router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .nest("/api/v1", routes::api_routes())
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(axum_middleware::from_fn(middleware::logging))
        )
        .with_state(AppState {
            db: db_pool,
            config: config.clone(),
        });

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
        .await?;
    
    info!("API server listening on {}:{}", config.server.host, config.server.port);
    
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Config,
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "dno-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now()
    }))
}
