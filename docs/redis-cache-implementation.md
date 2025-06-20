# Redis Cache Implementation for DNO Crawler

## Architecture Overview

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Client    │────▶│     API     │────▶│    Redis    │
└─────────────┘     │   (Axum)    │     │   (Cache)   │
                    └──────┬──────┘     └──────┬──────┘
                           │                    │
                           └────────┬───────────┘
                                    ▼
                            ┌─────────────┐
                            │  PostgreSQL  │
                            │  (Database)  │
                            └─────────────┘
```

## 1. Dependencies (Cargo.toml)

```toml
[dependencies]
# Database
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono"] }

# Redis
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }
bb8 = "0.8"
bb8-redis = "0.15"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1", features = ["full"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"
```

## 2. Cache Layer Implementation

### Cache Trait

```rust
// src/cache/mod.rs
use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait CacheLayer: Send + Sync {
    type Error;
    
    async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>, Self::Error>;
    async fn set<T: serde::Serialize>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), Self::Error>;
    async fn delete(&self, key: &str) -> Result<(), Self::Error>;
    async fn exists(&self, key: &str) -> Result<bool, Self::Error>;
    async fn invalidate_pattern(&self, pattern: &str) -> Result<(), Self::Error>;
}
```

### Redis Implementation

```rust
// src/cache/redis_cache.rs
use bb8_redis::{bb8, RedisConnectionManager};
use redis::AsyncCommands;
use std::time::Duration;
use async_trait::async_trait;

pub type RedisPool = bb8::Pool<RedisConnectionManager>;

#[derive(Clone)]
pub struct RedisCache {
    pool: RedisPool,
    default_ttl: Duration,
}

impl RedisCache {
    pub async fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = RedisConnectionManager::new(redis_url)?;
        let pool = bb8::Pool::builder()
            .max_size(100)
            .min_idle(Some(10))
            .build(manager)
            .await?;
            
        Ok(Self {
            pool,
            default_ttl: Duration::from_secs(3600), // 1 hour default
        })
    }
    
    fn make_key(&self, key: &str) -> String {
        format!("dno:{}", key)
    }
}

#[async_trait]
impl CacheLayer for RedisCache {
    type Error = redis::RedisError;
    
    async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>, Self::Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            redis::RedisError::from((redis::ErrorKind::IoError, "Pool error", e.to_string()))
        })?;
        
        let key = self.make_key(key);
        let data: Option<String> = conn.get(&key).await?;
        
        match data {
            Some(json) => {
                let value = serde_json::from_str(&json)
                    .map_err(|e| redis::RedisError::from((
                        redis::ErrorKind::TypeError,
                        "Deserialization error",
                        e.to_string()
                    )))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
    
    async fn set<T: serde::Serialize>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), Self::Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            redis::RedisError::from((redis::ErrorKind::IoError, "Pool error", e.to_string()))
        })?;
        
        let key = self.make_key(key);
        let json = serde_json::to_string(value)
            .map_err(|e| redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization error",
                e.to_string()
            )))?;
        
        let ttl = ttl.unwrap_or(self.default_ttl);
        conn.setex(&key, json, ttl.as_secs() as i64).await?;
        
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            redis::RedisError::from((redis::ErrorKind::IoError, "Pool error", e.to_string()))
        })?;
        
        let key = self.make_key(key);
        conn.del(&key).await?;
        Ok(())
    }
    
    async fn exists(&self, key: &str) -> Result<bool, Self::Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            redis::RedisError::from((redis::ErrorKind::IoError, "Pool error", e.to_string()))
        })?;
        
        let key = self.make_key(key);
        conn.exists(&key).await
    }
    
    async fn invalidate_pattern(&self, pattern: &str) -> Result<(), Self::Error> {
        let mut conn = self.pool.get().await.map_err(|e| {
            redis::RedisError::from((redis::ErrorKind::IoError, "Pool error", e.to_string()))
        })?;
        
        let pattern = format!("dno:{}*", pattern);
        let keys: Vec<String> = conn.keys(&pattern).await?;
        
        if !keys.is_empty() {
            conn.del(&keys).await?;
        }
        
        Ok(())
    }
}
```

## 3. Repository Pattern with Cache

```rust
// src/repository/dno_repository.rs
use sqlx::PgPool;
use std::sync::Arc;
use crate::cache::{CacheLayer, RedisCache};

#[derive(Clone)]
pub struct DnoRepository {
    db: PgPool,
    cache: Arc<RedisCache>,
}

impl DnoRepository {
    pub fn new(db: PgPool, cache: Arc<RedisCache>) -> Self {
        Self { db, cache }
    }
    
    // Get DNO data with cache
    pub async fn get_netzentgelte(
        &self,
        dno_slug: &str,
        year: i32
    ) -> Result<Option<NetzentgelteData>, sqlx::Error> {
        // Create cache key
        let cache_key = format!("netzentgelte:{}:{}", dno_slug, year);
        
        // Try cache first
        if let Ok(Some(cached)) = self.cache.get::<NetzentgelteData>(&cache_key).await {
            tracing::debug!("Cache hit for key: {}", cache_key);
            return Ok(Some(cached));
        }
        
        // Cache miss - fetch from database
        tracing::debug!("Cache miss for key: {}", cache_key);
        
        let data = sqlx::query_as!(
            NetzentgelteData,
            r#"
            SELECT 
                n.*, 
                d.name as dno_name,
                d.slug as dno_slug
            FROM netzentgelte_data n
            JOIN dnos d ON n.dno_id = d.id
            WHERE d.slug = $1 AND n.year = $2
            AND n.deleted_at IS NULL
            "#,
            dno_slug,
            year
        )
        .fetch_optional(&self.db)
        .await?;
        
        // Cache the result
        if let Some(ref data) = data {
            let ttl = if data.verification_status == "verified" {
                Some(Duration::from_secs(86400)) // 24 hours for verified data
            } else {
                Some(Duration::from_secs(3600)) // 1 hour for unverified
            };
            
            if let Err(e) = self.cache.set(&cache_key, data, ttl).await {
                tracing::warn!("Failed to cache data: {}", e);
            }
        }
        
        Ok(data)
    }
    
    // Update with cache invalidation
    pub async fn update_netzentgelte(
        &self,
        id: Uuid,
        update: UpdateNetzentgelte
    ) -> Result<NetzentgelteData, sqlx::Error> {
        let updated = sqlx::query_as!(
            NetzentgelteData,
            r#"
            UPDATE netzentgelte_data
            SET 
                leistung = COALESCE($2, leistung),
                arbeit = COALESCE($3, arbeit),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#,
            id,
            update.leistung,
            update.arbeit
        )
        .fetch_one(&self.db)
        .await?;
        
        // Invalidate cache
        let cache_pattern = format!("netzentgelte:*:{}", updated.year);
        if let Err(e) = self.cache.invalidate_pattern(&cache_pattern).await {
            tracing::warn!("Failed to invalidate cache: {}", e);
        }
        
        Ok(updated)
    }
    
    // Bulk query with smart caching
    pub async fn get_dno_coverage_stats(&self) -> Result<CoverageStats, sqlx::Error> {
        let cache_key = "stats:coverage";
        
        // Try cache first
        if let Ok(Some(cached)) = self.cache.get::<CoverageStats>(&cache_key).await {
            return Ok(cached);
        }
        
        // Expensive query
        let stats = sqlx::query_as!(
            CoverageStats,
            r#"
            SELECT 
                COUNT(DISTINCT d.id) as total_dnos,
                COUNT(DISTINCT n.id) as total_entries,
                COUNT(DISTINCT CASE WHEN n.verification_status = 'verified' THEN n.id END) as verified_entries,
                COUNT(DISTINCT CASE WHEN n.year = EXTRACT(YEAR FROM CURRENT_DATE) THEN d.id END) as current_year_coverage
            FROM dnos d
            LEFT JOIN netzentgelte_data n ON d.id = n.dno_id
            WHERE n.deleted_at IS NULL
            "#
        )
        .fetch_one(&self.db)
        .await?;
        
        // Cache for 5 minutes
        let _ = self.cache.set(&cache_key, &stats, Some(Duration::from_secs(300))).await;
        
        Ok(stats)
    }
}
```

## 4. Cache-Aside Pattern for Natural Language Queries

```rust
// src/services/query_service.rs
use crate::cache::RedisCache;
use sha2::{Sha256, Digest};

pub struct QueryService {
    db: PgPool,
    cache: Arc<RedisCache>,
    nlp_service: NlpService,
}

impl QueryService {
    pub async fn process_natural_query(&self, query: &str) -> Result<QueryResult, Error> {
        // Create deterministic cache key from query
        let query_hash = self.hash_query(query);
        let cache_key = format!("query:{}", query_hash);
        
        // Check cache
        if let Ok(Some(cached)) = self.cache.get::<QueryResult>(&cache_key).await {
            tracing::info!("Query cache hit");
            return Ok(cached);
        }
        
        // Process query
        let interpretation = self.nlp_service.interpret(query).await?;
        
        // Fetch data based on interpretation
        let result = match interpretation.data_type {
            DataType::Netzentgelte => {
                let data = self.fetch_netzentgelte(
                    &interpretation.dno,
                    interpretation.year
                ).await?;
                
                QueryResult {
                    interpretation,
                    data: serde_json::to_value(data)?,
                    cached: false,
                }
            }
            DataType::Hlzf => {
                let data = self.fetch_hlzf(
                    &interpretation.dno,
                    interpretation.year
                ).await?;
                
                QueryResult {
                    interpretation,
                    data: serde_json::to_value(data)?,
                    cached: false,
                }
            }
        };
        
        // Cache the result
        let ttl = if result.interpretation.confidence > 0.9 {
            Some(Duration::from_secs(3600)) // 1 hour for high confidence
        } else {
            Some(Duration::from_secs(300)) // 5 minutes for low confidence
        };
        
        let _ = self.cache.set(&cache_key, &result, ttl).await;
        
        Ok(result)
    }
    
    fn hash_query(&self, query: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(query.to_lowercase().trim());
        format!("{:x}", hasher.finalize())
    }
}
```

## 5. Session Management with Redis

```rust
// src/auth/session.rs
use axum_sessions::{SessionLayer, SessionStore};
use redis::Client;

pub struct RedisSessionStore {
    client: Client,
}

impl RedisSessionStore {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn load(&self, session_id: &str) -> Result<Option<Session>, Error> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("session:{}", session_id);
        let data: Option<String> = conn.get(&key).await?;
        
        match data {
            Some(json) => Ok(serde_json::from_str(&json)?),
            None => Ok(None),
        }
    }
    
    async fn store(&self, session: &Session) -> Result<(), Error> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("session:{}", session.id());
        let json = serde_json::to_string(session)?;
        
        // Set with 24 hour TTL
        conn.setex(&key, json, 86400).await?;
        Ok(())
    }
    
    async fn destroy(&self, session_id: &str) -> Result<(), Error> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("session:{}", session_id);
        conn.del(&key).await?;
        Ok(())
    }
}
```

## 6. Rate Limiting with Redis

```rust
// src/middleware/rate_limit.rs
use axum::{
    extract::{State, ConnectInfo},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;

pub async fn rate_limit_middleware(
    State(cache): State<Arc<RedisCache>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let key = format!("rate_limit:{}", addr.ip());
    let window = chrono::Utc::now().timestamp() / 60; // 1 minute window
    let window_key = format!("{}:{}", key, window);
    
    // Get current count
    let count: i32 = cache.get(&window_key).await
        .unwrap_or(Some(0))
        .unwrap_or(0);
    
    // Check limit
    let limit = 60; // 60 requests per minute
    if count >= limit {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    // Increment counter
    let _ = cache.set(&window_key, &(count + 1), Some(Duration::from_secs(60))).await;
    
    Ok(next.run(request).await)
}
```

## 7. Pub/Sub for Cache Invalidation

```rust
// src/cache/invalidation.rs
use redis::aio::PubSub;

pub struct CacheInvalidator {
    pubsub: PubSub,
    cache: Arc<RedisCache>,
}

impl CacheInvalidator {
    pub async fn new(redis_url: &str, cache: Arc<RedisCache>) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let pubsub = client.get_async_connection().await?.into_pubsub();
        
        Ok(Self { pubsub, cache })
    }
    
    pub async fn listen(mut self) {
        let _ = self.pubsub.subscribe("cache:invalidate").await;
        
        while let Ok(msg) = self.pubsub.on_message().next().await {
            let pattern: String = msg.get_payload().unwrap_or_default();
            
            if let Err(e) = self.cache.invalidate_pattern(&pattern).await {
                tracing::error!("Failed to invalidate cache pattern {}: {}", pattern, e);
            }
        }
    }
    
    pub async fn publish_invalidation(redis_url: &str, pattern: &str) -> Result<(), redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let mut conn = client.get_async_connection().await?;
        
        conn.publish("cache:invalidate", pattern).await?;
        Ok(())
    }
}
```

## 8. Application State Setup

```rust
// src/main.rs
#[derive(Clone)]
struct AppState {
    db: PgPool,
    cache: Arc<RedisCache>,
    dno_repo: DnoRepository,
    query_service: QueryService,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    // Database pool
    let db_url = std::env::var("DATABASE_URL")?;
    let db = PgPool::connect(&db_url).await?;
    
    // Redis cache
    let redis_url = std::env::var("REDIS_URL")?;
    let cache = Arc::new(RedisCache::new(&redis_url).await?);
    
    // Start cache invalidation listener
    let invalidator = CacheInvalidator::new(&redis_url, cache.clone()).await?;
    tokio::spawn(invalidator.listen());
    
    // Initialize repositories and services
    let dno_repo = DnoRepository::new(db.clone(), cache.clone());
    let query_service = QueryService::new(db.clone(), cache.clone());
    
    let state = AppState {
        db,
        cache,
        dno_repo,
        query_service,
    };
    
    // Build router
    let app = Router::new()
        .route("/api/v1/query/natural", post(query_handler))
        .route("/api/v1/dnos/:slug/:year", get(get_dno_data))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware
        ))
        .with_state(state);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

## 9. Cache Warming Strategy

```rust
// src/jobs/cache_warmer.rs
pub async fn warm_cache_job(state: AppState) {
    let interval = Duration::from_secs(3600); // Run every hour
    
    loop {
        tokio::time::sleep(interval).await;
        
        // Get most queried DNOs
        let popular_dnos = sqlx::query!(
            r#"
            SELECT DISTINCT interpreted_dno, interpreted_year
            FROM query_logs
            WHERE created_at > NOW() - INTERVAL '7 days'
            GROUP BY interpreted_dno, interpreted_year
            ORDER BY COUNT(*) DESC
            LIMIT 50
            "#
        )
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
        
        // Pre-cache popular data
        for record in popular_dnos {
            if let (Some(dno), Some(year)) = (record.interpreted_dno, record.interpreted_year) {
                let _ = state.dno_repo.get_netzentgelte(&dno, year).await;
            }
        }
        
        tracing::info!("Cache warming completed");
    }
}
```

## 10. Monitoring & Metrics

```rust
// src/metrics/cache_metrics.rs
use prometheus::{IntCounter, Histogram, Registry};

pub struct CacheMetrics {
    pub hits: IntCounter,
    pub misses: IntCounter,
    pub errors: IntCounter,
    pub latency: Histogram,
}

impl CacheMetrics {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let hits = IntCounter::new("cache_hits_total", "Total cache hits")?;
        let misses = IntCounter::new("cache_misses_total", "Total cache misses")?;
        let errors = IntCounter::new("cache_errors_total", "Total cache errors")?;
        let latency = Histogram::new("cache_operation_duration_seconds", "Cache operation latency")?;
        
        registry.register(Box::new(hits.clone()))?;
        registry.register(Box::new(misses.clone()))?;
        registry.register(Box::new(errors.clone()))?;
        registry.register(Box::new(latency.clone()))?;
        
        Ok(Self { hits, misses, errors, latency })
    }
}
```

## Best Practices

1. **Cache Keys**: Use consistent, hierarchical naming (e.g., `type:identifier:version`)
2. **TTL Strategy**: Verified data = 24h, unverified = 1h, stats = 5m
3. **Error Handling**: Always fall back to database on cache errors
4. **Monitoring**: Track hit/miss rates to optimize cache usage
5. **Invalidation**: Use pub/sub for distributed cache invalidation
6. **Serialization**: Use JSON for flexibility, consider MessagePack for performance
7. **Connection Pooling**: Use bb8 for efficient connection management