mod routes;
mod handlers;
mod middleware;
mod database;
mod services;

use axum::{
    Router,
    routing::{get, post},
    http::StatusCode,
    response::Json,
    middleware as axum_middleware,
};
use serde_json::{json, Value};
use shared::{Config, AppError};
use services::{SearchService, CrawlService, OllamaService, SearchOrchestrator, PdfAnalysisService};
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
    
    // Initialize services
    let search_service = SearchService::new(Some(config.external.searxng.url.clone()));
    let crawl_service = CrawlService::new();
    let ai_service = OllamaService::new(config.external.ollama.url.clone(), config.external.ollama.model.clone());
    let pdf_service = PdfAnalysisService::new(db_pool.clone(), config.external.ollama.url.clone());
    let search_orchestrator = SearchOrchestrator::new(
        ai_service.clone(),
        search_service.clone(),
        crawl_service.clone(),
        db_pool.clone(),
    );

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
            search_service,
            crawl_service,
            ai_service,
            search_orchestrator,
            pdf_service,
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
    pub search_service: SearchService,
    pub crawl_service: CrawlService,
    pub ai_service: OllamaService,
    pub search_orchestrator: SearchOrchestrator,
    pub pdf_service: PdfAnalysisService,
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "dno-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now()
    }))
}
