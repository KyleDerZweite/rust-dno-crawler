use async_trait::async_trait;
use std::time::Duration;
use thiserror::Error;

pub mod redis_cache;
pub mod metrics;

pub use redis_cache::RedisCache;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Connection pool error: {0}")]
    Pool(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Key not found: {0}")]
    NotFound(String),
    
    #[error("Cache operation timeout")]
    Timeout,
}

/// Trait defining cache operations for the DNO data gatherer system
#[async_trait]
pub trait CacheLayer: Send + Sync + Clone {
    /// Get a value from cache by key
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: serde::de::DeserializeOwned + Send;

    /// Set a value in cache with optional TTL
    async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), CacheError>
    where
        T: serde::Serialize + Send + Sync;

    /// Delete a key from cache
    async fn delete(&self, key: &str) -> Result<(), CacheError>;

    /// Check if a key exists in cache
    async fn exists(&self, key: &str) -> Result<bool, CacheError>;

    /// Invalidate multiple keys matching a pattern
    async fn invalidate_pattern(&self, pattern: &str) -> Result<u64, CacheError>;

    /// Get multiple keys at once
    async fn mget<T>(&self, keys: &[String]) -> Result<Vec<Option<T>>, CacheError>
    where
        T: serde::de::DeserializeOwned + Send;

    /// Set multiple key-value pairs at once
    async fn mset<T>(&self, items: &[(String, T)], ttl: Option<Duration>) -> Result<(), CacheError>
    where
        T: serde::Serialize + Send + Sync;

    /// Increment a numeric value (for counters, rate limiting)
    async fn incr(&self, key: &str, delta: i64, ttl: Option<Duration>) -> Result<i64, CacheError>;
}

/// Cache key utilities for consistent naming
pub struct CacheKeys;

impl CacheKeys {
    /// User authentication cache keys
    pub fn user_by_id(user_id: uuid::Uuid) -> String {
        format!("auth:user:id:{}", user_id)
    }

    pub fn user_by_email(email: &str) -> String {
        format!("auth:user:email:{}", Self::hash_email(email))
    }

    pub fn session_by_token(token_hash: &str) -> String {
        format!("auth:session:token:{}", token_hash)
    }

    pub fn session_by_refresh_token(refresh_token_hash: &str) -> String {
        format!("auth:session:refresh:{}", refresh_token_hash)
    }

    /// Search cache keys with filter-based hashing
    pub fn search_netzentgelte(filters: &SearchFilters) -> String {
        let filter_hash = Self::hash_search_filters(filters);
        format!("search:netzentgelte:{}", filter_hash)
    }

    pub fn search_hlzf(filters: &SearchFilters) -> String {
        let filter_hash = Self::hash_search_filters(filters);
        format!("search:hlzf:{}", filter_hash)
    }

    pub fn search_count_netzentgelte(filters: &SearchFilters) -> String {
        let filter_hash = Self::hash_search_filters(filters);
        format!("search:count:netzentgelte:{}", filter_hash)
    }

    /// Dashboard and analytics cache keys
    pub fn dashboard_stats(user_role: &str) -> String {
        let window = chrono::Utc::now().timestamp() / 900; // 15-minute windows
        format!("stats:dashboard:{}:{}", user_role, window)
    }

    pub fn available_filters() -> String {
        let window = chrono::Utc::now().timestamp() / 3600; // 1-hour windows
        format!("filters:available:{}", window)
    }

    /// DNO reference data cache keys
    pub fn dno_by_id(dno_id: uuid::Uuid) -> String {
        format!("reference:dno:id:{}", dno_id)
    }

    pub fn dno_by_name(name: &str) -> String {
        format!("reference:dno:name:{}", Self::normalize_name(name))
    }

    pub fn dno_by_slug(slug: &str) -> String {
        format!("reference:dno:slug:{}", slug.to_lowercase())
    }

    pub fn all_dnos() -> String {
        "reference:dnos:all".to_string()
    }

    /// Query history cache keys
    pub fn user_query_history(user_id: uuid::Uuid, page: i64) -> String {
        format!("history:user:{}:page:{}", user_id, page)
    }

    /// Rate limiting cache keys
    pub fn rate_limit_ip(ip: &str) -> String {
        let window = chrono::Utc::now().timestamp() / 60; // 1-minute windows
        format!("rate_limit:ip:{}:{}", ip, window)
    }

    pub fn rate_limit_user(user_id: uuid::Uuid) -> String {
        let window = chrono::Utc::now().timestamp() / 60; // 1-minute windows
        format!("rate_limit:user:{}:{}", user_id, window)
    }

    // Helper functions for key generation
    fn hash_email(email: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(email.to_lowercase().trim());
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    fn hash_search_filters(filters: &SearchFilters) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        
        // Create deterministic hash from search parameters
        hasher.update(filters.dno_id.map(|id| id.to_string()).unwrap_or_default());
        hasher.update(filters.dno_name.as_deref().unwrap_or(""));
        hasher.update(filters.year.map(|y| y.to_string()).unwrap_or_default());
        hasher.update(filters.data_type.as_deref().unwrap_or(""));
        hasher.update(filters.region.as_deref().unwrap_or(""));
        hasher.update(filters.limit.map(|l| l.to_string()).unwrap_or_default());
        hasher.update(filters.offset.map(|o| o.to_string()).unwrap_or_default());
        
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    fn normalize_name(name: &str) -> String {
        name.to_lowercase()
            .trim()
            .replace(' ', "_")
            .replace(|c: char| !c.is_alphanumeric() && c != '_', "")
    }
}

/// Search filters struct for cache key generation
#[derive(Debug, Clone)]
pub struct SearchFilters {
    pub dno_id: Option<uuid::Uuid>,
    pub dno_name: Option<String>,
    pub year: Option<i32>,
    pub data_type: Option<String>,
    pub region: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Cache configuration structure for Redis connection
#[derive(Debug, Clone)]
pub struct RedisCacheConfig {
    pub redis_url: String,
    pub max_connections: u32,
    pub connection_timeout: u64,
    pub default_ttl: Duration,
    pub session_ttl: Duration,
    pub found_data_ttl: Duration,
    pub not_found_ttl: Duration,
}

impl RedisCacheConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            redis_url: std::env::var("APP_REDIS_URL")?,
            max_connections: std::env::var("REDIS_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
            connection_timeout: std::env::var("REDIS_CONNECTION_TIMEOUT")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            default_ttl: Duration::from_secs(3600), // 1 hour default
            session_ttl: Duration::from_secs(
                std::env::var("CACHE_TTL_SESSION")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap_or(3600)
            ),
            found_data_ttl: Duration::from_secs(
                std::env::var("CACHE_TTL_FOUND")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()
                    .unwrap_or(86400)
            ),
            not_found_ttl: Duration::from_secs(
                std::env::var("CACHE_TTL_NOT_FOUND")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap_or(3600)
            ),
        })
    }
}