use shared::{DatabaseConfig, AppError};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::time::Duration;
use tracing::{info, error};

pub async fn create_pool(config: &DatabaseConfig) -> Result<SqlitePool, AppError> {
    info!("Connecting to database: {}", config.url);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout))
        .idle_timeout(Duration::from_secs(config.idle_timeout))
        .connect(&config.url)
        .await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            AppError::Database(e.to_string())
        })?;

    info!("Database connection pool created successfully");
    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), AppError> {
    info!("Running database migrations");
    
    sqlx::migrate!("../../migrations")
        .run(pool)
        .await
        .map_err(|e| {
            error!("Failed to run migrations: {}", e);
            AppError::Database(e.to_string())
        })?;

    info!("Database migrations completed successfully");
    Ok(())
}