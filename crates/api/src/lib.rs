pub mod routes;
pub mod middleware;

use sqlx::PgPool;
use std::sync::Arc;

// Re-export commonly used types
pub use routes::api_routes;
pub use middleware::{AuthenticatedUser, UserRole};

// Re-export cache types
pub use core::cache::RedisCache;
pub use core::repository::{UserRepository, SearchRepository, DnoRepository};

#[derive(Clone)]
pub struct AppState {
    pub database: PgPool,
    pub config: Arc<AppConfig>,
    pub jwt_secret: String,
    pub cache: Arc<RedisCache>,
    pub user_repo: UserRepository<RedisCache>,
    pub search_repo: SearchRepository<RedisCache>,
    pub dno_repo: DnoRepository<RedisCache>,
}

impl AppState {
    pub fn new(
        database: PgPool, 
        config: AppConfig, 
        jwt_secret: String,
        cache: Arc<RedisCache>
    ) -> Self {
        // Create repository instances with shared cache
        let user_repo = UserRepository::new(database.clone(), cache.clone());
        let search_repo = SearchRepository::new(database.clone(), cache.clone());
        let dno_repo = DnoRepository::new(database.clone(), cache.clone());

        Self {
            database,
            config: Arc::new(config),
            jwt_secret,
            cache,
            user_repo,
            search_repo,
            dno_repo,
        }
    }

    /// Initialize Redis cache from configuration
    pub async fn init_cache(config: &core::CacheConfig) -> Result<Arc<RedisCache>, core::AppError> {
        let redis_config = core::RedisCacheConfig::from_env()
            .map_err(|e| core::AppError::Config(format!("Redis config error: {}", e)))?;
        
        let cache = RedisCache::new(redis_config).await
            .map_err(|e| core::AppError::Cache(format!("Failed to connect to Redis: {}", e)))?;
        
        Ok(Arc::new(cache))
    }

    /// Warm up caches with commonly accessed data
    pub async fn warm_caches(&self) -> Result<(), core::AppError> {
        tracing::info!("Starting cache warm-up");
        
        // Warm up repositories in parallel
        let (user_result, search_result, dno_result) = tokio::join!(
            async { self.user_repo.warm_cache().await },
            async { self.search_repo.warm_cache().await },
            async { self.dno_repo.warm_cache().await }
        );
        
        if let Err(e) = user_result {
            tracing::warn!("User cache warm-up failed: {}", e);
        }
        
        if let Err(e) = search_result {
            tracing::warn!("Search cache warm-up failed: {}", e);
        }
        
        if let Err(e) = dno_result {
            tracing::warn!("DNO cache warm-up failed: {}", e);
        }
        
        tracing::info!("Cache warm-up completed");
        Ok(())
    }

    /// Get cache health information
    pub async fn cache_health(&self) -> Result<serde_json::Value, core::AppError> {
        let cache_health = self.cache.health_check().await
            .map_err(|e| core::AppError::Cache(format!("Cache health check failed: {}", e)))?;
        
        Ok(serde_json::to_value(cache_health)?)
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub cors_origins: Vec<String>,
    pub rate_limit_per_minute: u32,
    pub rate_limit_per_hour: u32,
    pub jwt_access_token_expiry: i64,
    pub jwt_refresh_token_expiry: i64,
    pub upload_max_size: u64,
    pub storage_path: String,
    pub temp_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_host: "0.0.0.0".to_string(),
            server_port: 3000,
            cors_origins: vec![
                "http://localhost:3000".to_string(),
                "http://localhost:8000".to_string(),
                "http://localhost:5173".to_string(),
            ],
            rate_limit_per_minute: 60,
            rate_limit_per_hour: 1000,
            jwt_access_token_expiry: 3600, // 1 hour
            jwt_refresh_token_expiry: 2592000, // 30 days
            upload_max_size: 52428800, // 50MB
            storage_path: "./storage".to_string(),
            temp_path: "./temp".to_string(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            cors_origins: std::env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000,http://localhost:8000,http://localhost:5173".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            rate_limit_per_minute: std::env::var("RATE_LIMIT_PER_MINUTE")
                .unwrap_or_else(|_| "60".to_string())
                .parse()?,
            rate_limit_per_hour: std::env::var("RATE_LIMIT_PER_HOUR")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()?,
            jwt_access_token_expiry: std::env::var("JWT_ACCESS_TOKEN_EXPIRY")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()?,
            jwt_refresh_token_expiry: std::env::var("JWT_REFRESH_TOKEN_EXPIRY")
                .unwrap_or_else(|_| "2592000".to_string())
                .parse()?,
            upload_max_size: std::env::var("UPLOAD_MAX_SIZE")
                .unwrap_or_else(|_| "52428800".to_string())
                .parse()?,
            storage_path: std::env::var("STORAGE_PATH")
                .unwrap_or_else(|_| "./storage".to_string()),
            temp_path: std::env::var("TEMP_PATH")
                .unwrap_or_else(|_| "./temp".to_string()),
        })
    }
}