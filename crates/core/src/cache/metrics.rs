use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Cache metrics for monitoring and performance analysis
#[derive(Debug, Clone)]
pub struct CacheMetrics {
    hits: Arc<AtomicU64>,
    misses: Arc<AtomicU64>,
    errors: Arc<AtomicU64>,
    total_latency_ms: Arc<AtomicU64>,
    operations: Arc<AtomicU64>,
}

impl Default for CacheMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheMetrics {
    pub fn new() -> Self {
        Self {
            hits: Arc::new(AtomicU64::new(0)),
            misses: Arc::new(AtomicU64::new(0)),
            errors: Arc::new(AtomicU64::new(0)),
            total_latency_ms: Arc::new(AtomicU64::new(0)),
            operations: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Record a cache hit
    pub fn record_hit(&self, latency: Duration) {
        self.hits.fetch_add(1, Ordering::Relaxed);
        self.operations.fetch_add(1, Ordering::Relaxed);
        self.total_latency_ms.fetch_add(latency.as_millis() as u64, Ordering::Relaxed);
    }

    /// Record a cache miss
    pub fn record_miss(&self, latency: Duration) {
        self.misses.fetch_add(1, Ordering::Relaxed);
        self.operations.fetch_add(1, Ordering::Relaxed);
        self.total_latency_ms.fetch_add(latency.as_millis() as u64, Ordering::Relaxed);
    }

    /// Record a cache error
    pub fn record_error(&self, latency: Duration) {
        self.errors.fetch_add(1, Ordering::Relaxed);
        self.operations.fetch_add(1, Ordering::Relaxed);
        self.total_latency_ms.fetch_add(latency.as_millis() as u64, Ordering::Relaxed);
    }

    /// Get current cache statistics
    pub fn get_stats(&self) -> CacheStats {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let errors = self.errors.load(Ordering::Relaxed);
        let operations = self.operations.load(Ordering::Relaxed);
        let total_latency_ms = self.total_latency_ms.load(Ordering::Relaxed);

        let hit_rate = if operations > 0 {
            (hits as f64 / operations as f64) * 100.0
        } else {
            0.0
        };

        let miss_rate = if operations > 0 {
            (misses as f64 / operations as f64) * 100.0
        } else {
            0.0
        };

        let error_rate = if operations > 0 {
            (errors as f64 / operations as f64) * 100.0
        } else {
            0.0
        };

        let avg_latency_ms = if operations > 0 {
            total_latency_ms as f64 / operations as f64
        } else {
            0.0
        };

        CacheStats {
            hits,
            misses,
            errors,
            operations,
            hit_rate,
            miss_rate,
            error_rate,
            avg_latency_ms,
        }
    }

    /// Reset all metrics (useful for testing)
    pub fn reset(&self) {
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
        self.errors.store(0, Ordering::Relaxed);
        self.operations.store(0, Ordering::Relaxed);
        self.total_latency_ms.store(0, Ordering::Relaxed);
    }
}

/// Cache statistics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub errors: u64,
    pub operations: u64,
    pub hit_rate: f64,
    pub miss_rate: f64,
    pub error_rate: f64,
    pub avg_latency_ms: f64,
}

/// Operation timer for measuring cache operation latency
pub struct OperationTimer {
    start: Instant,
    metrics: Arc<CacheMetrics>,
}

impl OperationTimer {
    pub fn new(metrics: Arc<CacheMetrics>) -> Self {
        Self {
            start: Instant::now(),
            metrics,
        }
    }

    /// Complete the operation as a hit
    pub fn hit(self) {
        let latency = self.start.elapsed();
        self.metrics.record_hit(latency);
    }

    /// Complete the operation as a miss
    pub fn miss(self) {
        let latency = self.start.elapsed();
        self.metrics.record_miss(latency);
    }

    /// Complete the operation as an error
    pub fn error(self) {
        let latency = self.start.elapsed();
        self.metrics.record_error(latency);
    }
}

/// Key-specific metrics for detailed analysis
#[derive(Debug, Clone, Serialize)]
pub struct KeyMetrics {
    pub pattern: String,
    pub hits: u64,
    pub misses: u64,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub avg_size_bytes: f64,
}

/// Cache metrics aggregator for different key patterns
#[derive(Debug, Clone)]
pub struct DetailedCacheMetrics {
    global: CacheMetrics,
    key_patterns: std::collections::HashMap<String, CacheMetrics>,
}

impl DetailedCacheMetrics {
    pub fn new() -> Self {
        Self {
            global: CacheMetrics::new(),
            key_patterns: std::collections::HashMap::new(),
        }
    }

    pub fn record_operation(&mut self, key: &str, result: CacheOperationResult, latency: Duration) {
        // Record global metrics
        match result {
            CacheOperationResult::Hit => self.global.record_hit(latency),
            CacheOperationResult::Miss => self.global.record_miss(latency),
            CacheOperationResult::Error => self.global.record_error(latency),
        }

        // Extract pattern from key (e.g., "auth:user:*", "search:netzentgelte:*")
        let pattern = extract_key_pattern(key);
        
        // Record pattern-specific metrics
        let pattern_metrics = self.key_patterns
            .entry(pattern)
            .or_insert_with(CacheMetrics::new);

        match result {
            CacheOperationResult::Hit => pattern_metrics.record_hit(latency),
            CacheOperationResult::Miss => pattern_metrics.record_miss(latency),
            CacheOperationResult::Error => pattern_metrics.record_error(latency),
        }
    }

    pub fn get_global_stats(&self) -> CacheStats {
        self.global.get_stats()
    }

    pub fn get_pattern_stats(&self, pattern: &str) -> Option<CacheStats> {
        self.key_patterns.get(pattern).map(|metrics| metrics.get_stats())
    }

    pub fn get_all_pattern_stats(&self) -> Vec<(String, CacheStats)> {
        self.key_patterns
            .iter()
            .map(|(pattern, metrics)| (pattern.clone(), metrics.get_stats()))
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CacheOperationResult {
    Hit,
    Miss,
    Error,
}

/// Extract cache key pattern for metrics grouping
fn extract_key_pattern(key: &str) -> String {
    let parts: Vec<&str> = key.split(':').collect();
    if parts.len() >= 3 {
        format!("{}:{}:*", parts[0], parts[1])
    } else if parts.len() >= 2 {
        format!("{}:*", parts[0])
    } else {
        "*".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_metrics() {
        let metrics = CacheMetrics::new();
        
        // Test initial state
        let stats = metrics.get_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.operations, 0);
        assert_eq!(stats.hit_rate, 0.0);

        // Record some operations
        metrics.record_hit(Duration::from_millis(10));
        metrics.record_hit(Duration::from_millis(20));
        metrics.record_miss(Duration::from_millis(30));

        let stats = metrics.get_stats();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.operations, 3);
        assert_eq!(stats.hit_rate, 66.66666666666666);
        assert_eq!(stats.miss_rate, 33.333333333333336);
        assert_eq!(stats.avg_latency_ms, 20.0);
    }

    #[test]
    fn test_key_pattern_extraction() {
        assert_eq!(extract_key_pattern("auth:user:123"), "auth:user:*");
        assert_eq!(extract_key_pattern("search:netzentgelte:abc123"), "search:netzentgelte:*");
        assert_eq!(extract_key_pattern("simple:key"), "simple:*");
        assert_eq!(extract_key_pattern("single"), "*");
    }
}