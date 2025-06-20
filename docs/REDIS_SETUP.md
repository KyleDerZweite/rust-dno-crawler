# Redis Setup for DNO Crawler

## Redis Structure and Keys

### 1. Session Management
```
# User sessions (JWT tokens)
session:{user_id} -> {
    "access_token": "...",
    "refresh_token": "...",
    "expires_at": "2024-01-15T15:00:00Z"
}
TTL: 3600 seconds (1 hour for access token)

# Refresh tokens
refresh_token:{token_hash} -> {
    "user_id": "550e8400-...",
    "expires_at": "2024-02-15T15:00:00Z"
}
TTL: 30 days
```

### 2. Query Cache
```
# Natural language query cache
query_cache:{query_hash} -> {
    "interpretation": {
        "dno": "Netze BW",
        "year": 2024,
        "data_type": "netzentgelte"
    },
    "confidence": 0.95
}
TTL: 24 hours

# DNO data cache
dno_data:{dno_slug}:{year}:{data_type} -> {
    "data": { ... },
    "source": { ... },
    "extracted_at": "2024-01-15T10:00:00Z"
}
TTL: 24 hours for found data, 1 hour for not found
```

### 3. Rate Limiting
```
# API rate limiting per user
rate_limit:{user_id}:{window} -> count
Example: rate_limit:550e8400:2024-01-15-14 -> 45
TTL: 1 hour

# Global rate limiting per IP
rate_limit:ip:{ip_address}:{window} -> count
TTL: 1 hour
```

### 4. Job Queue and Status
```
# Job status
job:{job_id} -> {
    "status": "running",
    "progress": 65,
    "current_step": "extracting_data",
    "dno": "Netze BW",
    "year": 2024,
    "user_id": "550e8400-..."
}
TTL: 24 hours after completion

# Job queue (using Redis lists)
job_queue:high -> [job_id1, job_id2, ...]
job_queue:normal -> [job_id3, job_id4, ...]
job_queue:low -> [job_id5, job_id6, ...]

# Active jobs set
active_jobs -> SET of job_ids
```

### 5. Real-time Updates (Pub/Sub)
```
# WebSocket channels
channel:job:{job_id} -> publishes job updates
channel:user:{user_id} -> publishes user-specific updates
channel:admin:system -> publishes system-wide updates
```

### 6. Statistics and Metrics
```
# Daily statistics
stats:queries:{date} -> {
    "total": 3421,
    "cache_hits": 2805,
    "crawls": 616,
    "failed": 12
}
TTL: 7 days

# Real-time metrics
metrics:active_users -> SET of user_ids (sliding window)
metrics:crawler:queue_size -> current queue size
metrics:crawler:active_workers -> number of active workers
```

### 7. Temporary Data
```
# Email verification tokens
verify:email:{token} -> {
    "user_id": "550e8400-...",
    "new_email": "new@example.com"
}
TTL: 24 hours

# Password reset tokens
reset:password:{token} -> {
    "user_id": "550e8400-..."
}
TTL: 1 hour

# File upload sessions
upload:{session_id} -> {
    "user_id": "550e8400-...",
    "file_type": "profile_picture",
    "expires_at": "2024-01-15T15:00:00Z"
}
TTL: 1 hour
```

## Redis Configuration (redis.conf)

```conf
# Max memory and eviction policy
maxmemory 512mb
maxmemory-policy allkeys-lru

# Persistence
save 900 1
save 300 10
save 60 10000
appendonly yes
appendfsync everysec

# Security
requirepass your_redis_password
```

## Rust Redis Implementation Example

```rust
use redis::{Client, Commands, AsyncCommands};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CachedDnoData {
    data: serde_json::Value,
    source: DataSource,
    extracted_at: chrono::DateTime<chrono::Utc>,
}

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    // Cache DNO storage
    pub async fn cache_dno_data(
        &self,
        dno_slug: &str,
        year: i32,
        data_type: &str,
        data: &CachedDnoData,
        found: bool,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("dno_data:{}:{}:{}", dno_slug, year, data_type);
        let serialized = serde_json::to_string(data).unwrap();
        
        // TTL: 24 hours for found storage, 1 hour for not found
        let ttl = if found { 86400 } else { 3600 };
        
        conn.setex(key, serialized, ttl).await?;
        Ok(())
    }

    // Get cached DNO storage
    pub async fn get_dno_data(
        &self,
        dno_slug: &str,
        year: i32,
        data_type: &str,
    ) -> Result<Option<CachedDnoData>, redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("dno_data:{}:{}:{}", dno_slug, year, data_type);
        
        let data: Option<String> = conn.get(key).await?;
        Ok(data.and_then(|s| serde_json::from_str(&s).ok()))
    }

    // Rate limiting
    pub async fn check_rate_limit(
        &self,
        user_id: &str,
        limit: u32,
    ) -> Result<bool, redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let now = chrono::Utc::now();
        let window = format!("{}", now.format("%Y-%m-%d-%H"));
        let key = format!("rate_limit:{}:{}", user_id, window);
        
        let count: u32 = conn.incr(&key, 1).await?;
        if count == 1 {
            conn.expire(&key, 3600).await?;
        }
        
        Ok(count <= limit)
    }

    // Job queue operations
    pub async fn enqueue_job(
        &self,
        job_id: &str,
        priority: &str,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let queue_key = format!("job_queue:{}", priority);
        conn.lpush(queue_key, job_id).await?;
        Ok(())
    }

    pub async fn dequeue_job(
        &self,
        priority: &str,
    ) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let queue_key = format!("job_queue:{}", priority);
        conn.rpop(queue_key).await
    }

    // Pub/Sub for real-time updates
    pub async fn publish_job_update(
        &self,
        job_id: &str,
        update: &serde_json::Value,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let channel = format!("channel:job:{}", job_id);
        let message = serde_json::to_string(update).unwrap();
        conn.publish(channel, message).await?;
        Ok(())
    }
}
```

## Environment Variables

```env
# Redis connection
REDIS_URL=redis://default:your_redis_password@localhost:6379/0
REDIS_MAX_CONNECTIONS=100
REDIS_CONNECTION_TIMEOUT=5

# Cache TTLs (in seconds)
CACHE_TTL_FOUND=86400      # 24 hours
CACHE_TTL_NOT_FOUND=3600   # 1 hour
CACHE_TTL_SESSION=3600     # 1 hour
CACHE_TTL_REFRESH=2592000  # 30 days
```

## Key Naming Conventions

1. Use colons (`:`) as separators
2. Start with the data type (e.g., `session:`, `dno_data:`)
3. Include relevant identifiers in order of specificity
4. Use consistent naming across the application

## Cache Invalidation Strategy

1. **Time-based**: Use TTL for automatic expiration
2. **Event-based**: Clear cache when data is updated
3. **Manual**: Provide admin endpoints to clear specific caches
4. **Pattern-based**: Use Redis SCAN with patterns to clear related keys

## Monitoring

Monitor these Redis metrics:
- Memory usage
- Hit/miss ratio
- Evicted keys
- Connected clients
- Commands per second
- Slow queries

Use Redis INFO command or tools like Redis Commander for monitoring.