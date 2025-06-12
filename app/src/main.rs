// Core application - The cornerstone main binary
use auth::{backend::AuthBackend, jwt::JwtService};
use core::{database::Database, config::Config};
use website::handlers;
use api::routes::create_api_routes;

use axum::{routing::{get, post}, Router};
use axum_login::{
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use std::{net::SocketAddr, time::Duration};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, error, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Load configuration
    let config = Config::from_env()?;
    let current_date = chrono::Utc::now().format("%B %d, %Y").to_string();
    info!("Starting DNO Crawler server - Current time: {current_date}");

    // Initialize database
    let database = match Database::new(&config.database_url).await {
        Ok(db) => {
            info!("Database connected successfully");
            db
        },
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Err(anyhow::anyhow!("Database connection failed: {}", e));
        }
    };

    // Initialize JWT Service for API
    let jwt_service = JwtService::new(&config.jwt_secret);

    // Session store for axum-login
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_expiry(Expiry::OnInactivity(Duration::from_secs(3600).try_into()?)); // 1 hour

    // Auth backend
    let auth_backend = AuthBackend::new(database.clone());
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    // Website state
    let web_state = handlers::WebState { database: database.clone() };

    // Website routes with axum-login - Updated with proper authentication-aware handlers
    let website_routes = Router::new()
        .route("/", get(handlers::home_page)) // Updated to use the new home_page handler
        .route("/register", get(handlers::register_page))
        .route("/register", post(handlers::register))
        .route("/login", get(handlers::login_page))
        .route("/login", post(handlers::login))
        .route("/logout", post(handlers::logout))
        .route("/dashboard", get(handlers::dashboard))
        .route("/user-management", get(handlers::user_management_page))
        .route("/privacy", get(handlers::privacy_page))
        .route("/terms", get(handlers::terms_page))
        .route("/contact", get(handlers::contact_page))
        .fallback(handlers::error_404_page)
        .layer(auth_layer)
        .with_state(web_state);

    // API routes (JWT-based)
    let api_routes = create_api_routes(database, jwt_service);

    // Static file serving
    let static_routes = Router::new()
        .nest_service("/public", ServeDir::new("./public"));

    // Combine all routes
    let app = Router::new()
        .nest("/api", api_routes)
        .merge(website_routes)
        .merge(static_routes)
        .layer(CorsLayer::permissive());

    // Start server
    let addr = SocketAddr::new(
        config.server_host.parse()?,
        config.server_port,
    );

    info!("DNO Crawler server with axum-login listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}