mod api;
mod auth;
mod config;
mod core;
mod website;

use crate::{
    auth::jwt::JwtService,
    config::Config,
    core::database::Database,
};
use axum::Router;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, error, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialisiere Logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Lade Konfiguration
    let config = Config::from_env()?;
    info!("Starting DNO Crawler server with config: {:?}", config);

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

    // Initialisiere JWT Service
    let jwt_service = JwtService::new(&config.jwt_secret);

    // Erstelle API Routes
    let api_routes = api::routes::create_api_routes(database.clone(), jwt_service);

    // Erstelle Website Routes
    let website_routes = match website::handlers::create_website_routes(database, &config.session_secret).await {
        Ok(routes) => routes,
        Err(e) => {
            error!("Failed to create website routes: {}", e);
            return Err(anyhow::anyhow!("Website routes creation failed: {}", e));
        }
    };

    // Static file serving für CSS
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

    info!("DNO Crawler server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}