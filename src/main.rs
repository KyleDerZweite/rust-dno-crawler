mod api;
mod auth;
mod config;
mod core;
mod website;

use crate::{
    auth::{backend::AuthBackend, jwt::JwtService},
    config::Config,
    core::database::Database,
    website::handlers,
};
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
    // Initialisiere Logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Lade Konfiguration
    let config = Config::from_env()?;
    let current_date = chrono::Utc::now().format("%B %d, %Y").to_string();
    info!("Starting DNO Crawler server - Current time: {current_date}");

    // Initialisiere Datenbank
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

    // Initialisiere JWT Service für API
    let jwt_service = JwtService::new(&config.jwt_secret);

    // Session store für axum-login
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_expiry(Expiry::OnInactivity(Duration::from_secs(3600).try_into()?)); // 1 hour

    // Auth backend
    let auth_backend = AuthBackend::new(database.clone());
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    // Website state
    let web_state = handlers::WebState { database: database.clone() };

    // Website routes mit axum-login - Updated with proper authentication-aware handlers
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

    // API routes (JWT-basiert)
    let api_routes = api::routes::create_api_routes(database, jwt_service);

    // Static file serving
    let static_routes = Router::new()
        .nest_service("/public", ServeDir::new("./public"));

    // Kombiniere alle Routes
    let app = Router::new()
        .nest("/api", api_routes)
        .merge(website_routes)
        .merge(static_routes)
        .layer(CorsLayer::permissive());

    // Starte Server
    let addr = SocketAddr::new(
        config.server_host.parse()?,
        config.server_port,
    );

    info!("DNO Crawler server with axum-login listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}