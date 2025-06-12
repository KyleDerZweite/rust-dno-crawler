use api::routes::create_api_routes;
use auth::jwt::JwtService;
use core::database::Database;
use axum::Router;
use std::{env, net::SocketAddr};
use tower_http::cors::CorsLayer;
use tracing::{info, error, Level};
use tracing_subscriber;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenv::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:data.db".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key".to_string()),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Load configuration
    let config = Config::from_env()?;
    info!("Starting API server");

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

    // Initialize JWT Service
    let jwt_service = JwtService::new(&config.jwt_secret);

    // Create API routes
    let api_routes = create_api_routes(database, jwt_service);

    // Combine all routes
    let app = Router::new()
        .nest("/api", api_routes)
        .layer(CorsLayer::permissive());

    // Start server
    let addr = SocketAddr::new(
        config.server_host.parse()?,
        config.server_port,
    );

    info!("API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}