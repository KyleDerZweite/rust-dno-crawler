use super::{CacheLayer, CacheError, RedisCacheConfig};
use async_trait::async_trait;
use bb8_redis::{bb8, redis::AsyncCommands, RedisConnectionManager};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, error, warn};

pub type RedisPool = bb8::Pool<RedisConnectionManager>;

/// Redis implementation of the CacheLayer trait
#[derive(Clone)]
pub struct RedisCache {
    pool: RedisPool,
    config: RedisCacheConfig,
}

impl RedisCache {
    /// Create a new Redis cache instance with connection pooling
    pub async fn new(config: RedisCacheConfig) -> Result<Self, CacheError> {
        let manager = RedisConnectionManager::new(config.redis_url.clone())
            .map_err(|e| CacheError::Pool(format!("Failed to create Redis manager: {}", e)))?;

        let pool = bb8::Pool::builder()
            .max_size(config.max_connections)
            .min_idle(Some(config.max_connections / 4)) // Keep 25% of connections idle
            .connection_timeout(Duration::from_secs(config.connection_timeout))
            .build(manager)
            .await
            .map_err(|e| CacheError::Pool(format!("Failed to create connection pool: {}", e)))?;

        // Test the connection
        {
            let mut conn = pool.get().await
                .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;
            let _: String = redis::cmd("PING").query_async(&mut *conn).await?;
        }

        debug!("Redis cache initialized successfully with {} max connections", config.max_connections);

        Ok(Self { pool, config })
    }

    /// Create cache key with DNO prefix for namespace separation
    fn make_key(&self, key: &str) -> String {
        format!("dno:{}", key)
    }

    /// Get TTL based on data type and configuration
    fn get_ttl(&self, custom_ttl: Option<Duration>) -> Duration {
        custom_ttl.unwrap_or(self.config.default_ttl)
    }
}

#[async_trait]
impl CacheLayer for RedisCache {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: serde::de::DeserializeOwned + Send,
    {
        let start = std::time::Instant::now();
        let cache_key = self.make_key(key);

        let mut conn = self.pool.get().await
            .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;

        let data: Option<String> = conn.get(&cache_key).await?;

        let result = match data {
            Some(json) => {
                match serde_json::from_str::<T>(&json) {
                    Ok(value) => {
                        debug!("Cache HIT for key: {} ({}ms)", key, start.elapsed().as_millis());
                        Some(value)
                    }
                    Err(e) => {
                        warn!("Cache deserialization error for key {}: {}", key, e);
                        // Delete corrupted data
                        let _: () = conn.del(&cache_key).await.unwrap_or(());
                        return Err(CacheError::Serialization(e));
                    }
                }
            }
            None => {
                debug!("Cache MISS for key: {} ({}ms)", key, start.elapsed().as_millis());
                None
            }
        };

        Ok(result)
    }

    async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), CacheError>
    where
        T: serde::Serialize + Send + Sync,
    {
        let start = std::time::Instant::now();
        let cache_key = self.make_key(key);

        let mut conn = self.pool.get().await
            .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;

        let json = serde_json::to_string(value)?;
        let ttl_seconds = self.get_ttl(ttl).as_secs();

        let _: () = conn.set_ex(&cache_key, json, ttl_seconds).await?;

        debug!("Cache SET for key: {} with TTL {}s ({}ms)", 
               key, ttl_seconds, start.elapsed().as_millis());

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let cache_key = self.make_key(key);

        let mut conn = self.pool.get().await
            .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;

        let deleted: i32 = conn.del(&cache_key).await?;

        if deleted > 0 {
            debug!("Cache DELETE for key: {}", key);
        }

        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, CacheError> {
        let cache_key = self.make_key(key);

        let mut conn = self.pool.get().await
            .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;

        let exists: bool = conn.exists(&cache_key).await?;
        Ok(exists)
    }

    async fn invalidate_pattern(&self, pattern: &str) -> Result<u64, CacheError> {
        let start = std::time::Instant::now();
        let cache_pattern = self.make_key(&format!("{}*", pattern));

        let mut conn = self.pool.get().await
            .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;

        // Get all keys matching the pattern
        let keys: Vec<String> = conn.keys(&cache_pattern).await?;
        let key_count = keys.len() as u64;

        if !keys.is_empty() {
            let _: () = conn.del(&keys).await?;
            debug!("Cache INVALIDATE pattern: {} ({} keys, {}ms)", 
                   pattern, key_count, start.elapsed().as_millis());
        }

        Ok(key_count)
    }

    async fn mget<T>(&self, keys: &[String]) -> Result<Vec<Option<T>>, CacheError>
    where
        T: serde::de::DeserializeOwned + Send,
    {
        if keys.is_empty() {
            return Ok(vec![]);
        }

        let start = std::time::Instant::now();
        let cache_keys: Vec<String> = keys.iter().map(|k| self.make_key(k)).collect();

        let mut conn = self.pool.get().await
            .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;

        let data: Vec<Option<String>> = conn.mget(&cache_keys).await?;

        let mut results = Vec::with_capacity(data.len());
        let mut hits = 0;

        for (i, value) in data.into_iter().enumerate() {
            match value {
                Some(json) => {
                    match serde_json::from_str::<T>(&json) {
                        Ok(parsed) => {
                            results.push(Some(parsed));
                            hits += 1;
                        }
                        Err(e) => {
                            warn!("Cache deserialization error for key {}: {}", keys[i], e);
                            // Delete corrupted data
                            let corrupted_key = &cache_keys[i];
                            let _: () = conn.del(corrupted_key).await.unwrap_or(());
                            results.push(None);
                        }
                    }
                }
                None => results.push(None),
            }
        }

        debug!("Cache MGET for {} keys: {} hits ({}ms)", 
               keys.len(), hits, start.elapsed().as_millis());

        Ok(results)
    }

    async fn mset<T>(&self, items: &[(String, T)], ttl: Option<Duration>) -> Result<(), CacheError>
    where
        T: serde::Serialize + Send + Sync,
    {
        if items.is_empty() {
            return Ok(());
        }

        let start = std::time::Instant::now();
        let ttl_seconds = self.get_ttl(ttl).as_secs();

        let mut conn = self.pool.get().await
            .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;

        // Use pipeline for efficiency
        let mut pipe = redis::pipe();

        for (key, value) in items {
            let cache_key = self.make_key(key);
            let json = serde_json::to_string(value)?;
            pipe.set_ex(&cache_key, json, ttl_seconds);
        }

        pipe.query_async::<_, ()>(&mut *conn).await?;

        debug!("Cache MSET for {} keys with TTL {}s ({}ms)", 
               items.len(), ttl_seconds, start.elapsed().as_millis());

        Ok(())
    }

    async fn incr(&self, key: &str, delta: i64, ttl: Option<Duration>) -> Result<i64, CacheError> {
        let cache_key = self.make_key(key);

        let mut conn = self.pool.get().await
            .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;

        // Increment and set expiry if it's a new key
        let result: i64 = conn.incr(&cache_key, delta).await?;

        if result == delta && ttl.is_some() {
            // This is a new key, set TTL
            let ttl_seconds = self.get_ttl(ttl).as_secs();
            let _: () = conn.expire(&cache_key, ttl_seconds as i64).await?;
        }

        Ok(result)
    }
}

/// Cached result wrapper to track cache metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResult<T> {
    pub data: T,
    pub cached_at: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

impl<T> CachedResult<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            cached_at: chrono::Utc::now(),
            version: "1.0".to_string(),
        }
    }

    pub fn is_expired(&self, max_age: Duration) -> bool {
        let age = chrono::Utc::now() - self.cached_at;
        age.to_std().unwrap_or(Duration::MAX) > max_age
    }
}

/// Health check for Redis cache
impl RedisCache {
    pub async fn health_check(&self) -> Result<CacheHealth, CacheError> {
        let start = std::time::Instant::now();

        let mut conn = self.pool.get().await
            .map_err(|e| CacheError::Pool(format!("Failed to get connection: {}", e)))?;

        // Test basic operations
        let test_key = "health:check";
        let test_value = "ok";

        // SET
        let _: () = conn.set_ex(test_key, test_value, 60).await?;

        // GET
        let result: String = conn.get(test_key).await?;
        if result != test_value {
            return Err(CacheError::Redis(redis::RedisError::from((
                redis::ErrorKind::ResponseError,
                "Health check failed",
                "GET returned wrong value".to_string(),
            ))));
        }

        // DELETE
        let _: () = conn.del(test_key).await?;

        let latency = start.elapsed();

        Ok(CacheHealth {
            status: "healthy".to_string(),
            latency_ms: latency.as_millis() as u64,
            pool_size: self.pool.state().connections,
            max_pool_size: self.pool.state().connections, // TODO: Get actual max size from config
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CacheHealth {
    pub status: String,
    pub latency_ms: u64,
    pub pool_size: u32,
    pub max_pool_size: u32,
}