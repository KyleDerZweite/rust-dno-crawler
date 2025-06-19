use crate::{
    cache::{CacheLayer, CacheKeys, SearchFilters},
    database, AppError, NetzentgelteDataWithDno, HlzfDataWithDno, AvailableFilters,
};
use chrono::Datelike;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};
use uuid::Uuid;

/// Repository for search operations with comprehensive Redis caching
#[derive(Clone)]
pub struct SearchRepository<C: CacheLayer> {
    db: PgPool,
    cache: Arc<C>,
    found_data_ttl: Duration,
    not_found_ttl: Duration,
    filters_ttl: Duration,
}

impl<C: CacheLayer> SearchRepository<C> {
    pub fn new(db: PgPool, cache: Arc<C>) -> Self {
        Self {
            db,
            cache,
            found_data_ttl: Duration::from_secs(86400), // 24 hours for found data
            not_found_ttl: Duration::from_secs(3600),   // 1 hour for not found
            filters_ttl: Duration::from_secs(3600),     // 1 hour for available filters
        }
    }

    /// Search netzentgelte data with caching
    pub async fn search_netzentgelte_data(
        &self,
        dno_id: Option<Uuid>,
        dno_name: Option<&str>,
        year: Option<i32>,
        verification_status: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<NetzentgelteDataWithDno>, AppError> {
        let filters = SearchFilters {
            dno_id,
            dno_name: dno_name.map(|s| s.to_string()),
            year,
            data_type: Some("netzentgelte".to_string()),
            region: None, // Not used in this search
            limit,
            offset,
        };

        let cache_key = CacheKeys::search_netzentgelte(&filters);

        // Try cache first
        match self.cache.get::<Vec<NetzentgelteDataWithDno>>(&cache_key).await {
            Ok(Some(data)) => {
                debug!("Cache HIT for netzentgelte search: {} results", data.len());
                return Ok(data);
            }
            Ok(None) => {
                debug!("Cache MISS for netzentgelte search");
            }
            Err(e) => {
                warn!("Cache error for netzentgelte search: {}", e);
            }
        }

        // Cache miss - fetch from database
        let data = database::search_netzentgelte_data(
            &self.db,
            dno_id,
            dno_name,
            year,
            verification_status,
            limit,
            offset,
        ).await?;

        // Cache the result with appropriate TTL
        let ttl = if data.is_empty() {
            self.not_found_ttl
        } else {
            self.found_data_ttl
        };

        if let Err(e) = self.cache.set(&cache_key, &data, Some(ttl)).await {
            warn!("Failed to cache netzentgelte search results: {}", e);
        }

        debug!("Cached netzentgelte search: {} results", data.len());
        Ok(data)
    }

    /// Search HLZF data with caching
    pub async fn search_hlzf_data(
        &self,
        dno_id: Option<Uuid>,
        dno_name: Option<&str>,
        year: Option<i32>,
        verification_status: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<HlzfDataWithDno>, AppError> {
        let filters = SearchFilters {
            dno_id,
            dno_name: dno_name.map(|s| s.to_string()),
            year,
            data_type: Some("hlzf".to_string()),
            region: None, // Not used in this search
            limit,
            offset,
        };

        let cache_key = CacheKeys::search_hlzf(&filters);

        // Try cache first
        match self.cache.get::<Vec<HlzfDataWithDno>>(&cache_key).await {
            Ok(Some(data)) => {
                debug!("Cache HIT for HLZF search: {} results", data.len());
                return Ok(data);
            }
            Ok(None) => {
                debug!("Cache MISS for HLZF search");
            }
            Err(e) => {
                warn!("Cache error for HLZF search: {}", e);
            }
        }

        // Cache miss - fetch from database
        let data = database::search_hlzf_data(
            &self.db,
            dno_id,
            dno_name,
            year,
            verification_status,
            limit,
            offset,
        ).await?;

        // Cache the result with appropriate TTL
        let ttl = if data.is_empty() {
            self.not_found_ttl
        } else {
            self.found_data_ttl
        };

        if let Err(e) = self.cache.set(&cache_key, &data, Some(ttl)).await {
            warn!("Failed to cache HLZF search results: {}", e);
        }

        debug!("Cached HLZF search: {} results", data.len());
        Ok(data)
    }

    /// Count netzentgelte data with caching
    pub async fn count_netzentgelte_data(
        &self,
        dno_id: Option<Uuid>,
        dno_name: Option<&str>,
        year: Option<i32>,
        verification_status: Option<&str>,
    ) -> Result<i64, AppError> {
        let filters = SearchFilters {
            dno_id,
            dno_name: dno_name.map(|s| s.to_string()),
            year,
            data_type: Some("netzentgelte".to_string()),
            region: None,
            limit: None,
            offset: None,
        };

        let cache_key = CacheKeys::search_count_netzentgelte(&filters);

        // Try cache first
        match self.cache.get::<i64>(&cache_key).await {
            Ok(Some(count)) => {
                debug!("Cache HIT for netzentgelte count: {}", count);
                return Ok(count);
            }
            Ok(None) => {
                debug!("Cache MISS for netzentgelte count");
            }
            Err(e) => {
                warn!("Cache error for netzentgelte count: {}", e);
            }
        }

        // Cache miss - fetch from database
        let count = database::count_netzentgelte_data(
            &self.db,
            dno_id,
            dno_name,
            year,
            verification_status,
        ).await?;

        // Cache the result
        let ttl = if count == 0 {
            self.not_found_ttl
        } else {
            self.found_data_ttl
        };

        if let Err(e) = self.cache.set(&cache_key, &count, Some(ttl)).await {
            warn!("Failed to cache netzentgelte count: {}", e);
        }

        debug!("Cached netzentgelte count: {}", count);
        Ok(count)
    }

    /// Get available years and DNOs with caching
    pub async fn get_available_years_and_dnos(&self) -> Result<AvailableFilters, AppError> {
        let cache_key = CacheKeys::available_filters();

        // Try cache first
        match self.cache.get::<AvailableFilters>(&cache_key).await {
            Ok(Some(filters)) => {
                debug!("Cache HIT for available filters");
                return Ok(filters);
            }
            Ok(None) => {
                debug!("Cache MISS for available filters");
            }
            Err(e) => {
                warn!("Cache error for available filters: {}", e);
            }
        }

        // Cache miss - fetch from database
        let filters = database::get_available_years_and_dnos(&self.db).await?;

        // Cache the result
        if let Err(e) = self.cache.set(&cache_key, &filters, Some(self.filters_ttl)).await {
            warn!("Failed to cache available filters: {}", e);
        }

        debug!("Cached available filters: {} years, {} DNOs", 
               filters.years.len(), filters.dnos.len());
        Ok(filters)
    }

    /// Get dashboard statistics with caching (minimal implementation for now)
    pub async fn get_dashboard_stats(&self, user_id: uuid::Uuid) -> Result<crate::DashboardStats, AppError> {
        let cache_key = format!("stats:dashboard:user:{}", user_id);

        // Try cache first
        match self.cache.get::<crate::DashboardStats>(&cache_key).await {
            Ok(Some(stats)) => {
                debug!("Cache HIT for dashboard stats (user: {})", user_id);
                return Ok(stats);
            }
            Ok(None) => {
                debug!("Cache MISS for dashboard stats (user: {})", user_id);
            }
            Err(e) => {
                warn!("Cache error for dashboard stats: {}", e);
            }
        }

        // Cache miss - fetch from database using correct signature
        let stats = database::get_dashboard_stats(&self.db, user_id).await?;

        // Cache the result with shorter TTL since dashboard stats change frequently
        if let Err(e) = self.cache.set(&cache_key, &stats, Some(Duration::from_secs(900))).await {
            warn!("Failed to cache dashboard stats: {}", e);
        }

        debug!("Cached dashboard stats for user: {}", user_id);
        Ok(stats)
    }

    /// Invalidate search caches when data is updated
    pub async fn invalidate_search_caches(&self, data_type: Option<&str>) -> Result<(), AppError> {
        match data_type {
            Some("netzentgelte") => {
                if let Err(e) = self.cache.invalidate_pattern("search:netzentgelte:").await {
                    warn!("Failed to invalidate netzentgelte search cache: {}", e);
                }
                if let Err(e) = self.cache.invalidate_pattern("search:count:netzentgelte:").await {
                    warn!("Failed to invalidate netzentgelte count cache: {}", e);
                }
            }
            Some("hlzf") => {
                if let Err(e) = self.cache.invalidate_pattern("search:hlzf:").await {
                    warn!("Failed to invalidate HLZF search cache: {}", e);
                }
            }
            _ => {
                // Invalidate all search caches
                if let Err(e) = self.cache.invalidate_pattern("search:").await {
                    warn!("Failed to invalidate all search caches: {}", e);
                }
            }
        }

        // Always invalidate available filters when data changes
        if let Err(e) = self.cache.invalidate_pattern("filters:available:").await {
            warn!("Failed to invalidate available filters cache: {}", e);
        }

        // Invalidate dashboard stats
        if let Err(e) = self.cache.invalidate_pattern("stats:dashboard:").await {
            warn!("Failed to invalidate dashboard stats cache: {}", e);
        }

        debug!("Invalidated search caches for data type: {:?}", data_type);
        Ok(())
    }

    /// Warm up cache with popular searches
    pub async fn warm_cache(&self) -> Result<(), AppError> {
        debug!("Starting cache warm-up for search operations");

        // Pre-cache available filters
        let _ = self.get_available_years_and_dnos().await;

        // Note: Dashboard stats are user-specific and cached on first request

        // Pre-cache recent year searches (current year and previous year)
        let current_year = chrono::Utc::now().year();
        let years_to_warm = [current_year, current_year - 1];

        for year in years_to_warm {
            // Search for both data types with basic filters
            let _ = self.search_netzentgelte_data(
                None, None, Some(year), Some("verified"), Some(50), Some(0)
            ).await;
            
            let _ = self.search_hlzf_data(
                None, None, Some(year), Some("verified"), Some(50), Some(0)
            ).await;
        }

        debug!("Cache warm-up completed");
        Ok(())
    }

    /// Get cache statistics for monitoring
    pub async fn get_cache_health(&self) -> Result<CacheHealthInfo, AppError> {
        // Test cache connectivity with a simple operation
        let test_key = "health:search:test";
        let test_value = "ok";

        let start = std::time::Instant::now();
        
        // Test SET
        self.cache.set(&test_key, &test_value, Some(Duration::from_secs(60))).await
            .map_err(|e| AppError::Cache(format!("Cache SET failed: {}", e)))?;

        // Test GET
        let retrieved: Option<String> = self.cache.get(&test_key).await
            .map_err(|e| AppError::Cache(format!("Cache GET failed: {}", e)))?;

        if retrieved.as_deref() != Some(test_value) {
            return Err(AppError::Cache("Cache health check failed: wrong value".to_string()));
        }

        // Test DELETE
        self.cache.delete(&test_key).await
            .map_err(|e| AppError::Cache(format!("Cache DELETE failed: {}", e)))?;

        let latency = start.elapsed();

        Ok(CacheHealthInfo {
            status: "healthy".to_string(),
            latency_ms: latency.as_millis() as u64,
            operations_tested: 3,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct CacheHealthInfo {
    pub status: String,
    pub latency_ms: u64,
    pub operations_tested: u32,
}