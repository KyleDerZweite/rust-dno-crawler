use crate::{
    cache::{CacheLayer, CacheKeys},
    database, AppError, Dno, CreateDno, UpdateDno,
};
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};
use uuid::Uuid;

/// Repository for DNO operations with Redis caching
#[derive(Clone)]
pub struct DnoRepository<C: CacheLayer> {
    db: PgPool,
    cache: Arc<C>,
    dno_ttl: Duration,
}

impl<C: CacheLayer> DnoRepository<C> {
    pub fn new(db: PgPool, cache: Arc<C>) -> Self {
        Self {
            db,
            cache,
            dno_ttl: Duration::from_secs(14400), // 4 hours - DNO data rarely changes
        }
    }

    /// Get all DNOs with caching
    pub async fn get_all_dnos(&self) -> Result<Vec<Dno>, AppError> {
        let cache_key = CacheKeys::all_dnos();

        // Try cache first
        match self.cache.get::<Vec<Dno>>(&cache_key).await {
            Ok(Some(dnos)) => {
                debug!("Cache HIT for all DNOs: {} entries", dnos.len());
                return Ok(dnos);
            }
            Ok(None) => {
                debug!("Cache MISS for all DNOs");
            }
            Err(e) => {
                warn!("Cache error for all DNOs: {}", e);
            }
        }

        // Cache miss - fetch from database
        let dnos = database::get_all_dnos(&self.db).await?;

        // Cache the result
        if let Err(e) = self.cache.set(&cache_key, &dnos, Some(self.dno_ttl)).await {
            warn!("Failed to cache all DNOs: {}", e);
        }

        // Also cache individual DNOs for faster lookups
        for dno in &dnos {
            let id_key = CacheKeys::dno_by_id(dno.id);
            let name_key = CacheKeys::dno_by_name(&dno.name);
            let slug_key = CacheKeys::dno_by_slug(&dno.slug);

            if let Err(e) = self.cache.set(&id_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by ID {}: {}", dno.id, e);
            }
            
            if let Err(e) = self.cache.set(&name_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by name {}: {}", dno.name, e);
            }
            
            if let Err(e) = self.cache.set(&slug_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by slug {}: {}", dno.slug, e);
            }
        }

        debug!("Cached all DNOs: {} entries", dnos.len());
        Ok(dnos)
    }

    /// Get DNO by ID with caching
    pub async fn get_dno_by_id(&self, dno_id: Uuid) -> Result<Option<Dno>, AppError> {
        let cache_key = CacheKeys::dno_by_id(dno_id);

        // Try cache first
        match self.cache.get::<Dno>(&cache_key).await {
            Ok(Some(dno)) => {
                debug!("Cache HIT for DNO by ID: {}", dno_id);
                return Ok(Some(dno));
            }
            Ok(None) => {
                debug!("Cache MISS for DNO by ID: {}", dno_id);
            }
            Err(e) => {
                warn!("Cache error for DNO by ID {}: {}", dno_id, e);
            }
        }

        // Cache miss - fetch from database
        let dno = database::get_dno_by_id(&self.db, dno_id).await?;

        // Cache the result
        if let Some(ref dno) = dno {
            // Cache by ID
            if let Err(e) = self.cache.set(&cache_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by ID: {}", e);
            }
            
            // Also cache by name and slug for cross-reference
            let name_key = CacheKeys::dno_by_name(&dno.name);
            let slug_key = CacheKeys::dno_by_slug(&dno.slug);
            
            if let Err(e) = self.cache.set(&name_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by name: {}", e);
            }
            
            if let Err(e) = self.cache.set(&slug_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by slug: {}", e);
            }
        } else {
            // Cache negative result with shorter TTL
            if let Err(e) = self.cache.set(&cache_key, &Option::<Dno>::None, Some(Duration::from_secs(300))).await {
                warn!("Failed to cache negative DNO result: {}", e);
            }
        }

        Ok(dno)
    }

    /// Get DNO by name with caching (handles ILIKE pattern matching)
    pub async fn get_dno_by_name(&self, name: &str) -> Result<Option<Dno>, AppError> {
        let cache_key = CacheKeys::dno_by_name(name);

        // Try cache first
        match self.cache.get::<Dno>(&cache_key).await {
            Ok(Some(dno)) => {
                debug!("Cache HIT for DNO by name: {}", name);
                return Ok(Some(dno));
            }
            Ok(None) => {
                debug!("Cache MISS for DNO by name: {}", name);
            }
            Err(e) => {
                warn!("Cache error for DNO by name {}: {}", name, e);
            }
        }

        // Cache miss - fetch from database
        let dno = database::get_dno_by_name(&self.db, name).await?;

        // Cache the result
        if let Some(ref dno) = dno {
            // Cache by name
            if let Err(e) = self.cache.set(&cache_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by name: {}", e);
            }
            
            // Also cache by ID and slug for cross-reference
            let id_key = CacheKeys::dno_by_id(dno.id);
            let slug_key = CacheKeys::dno_by_slug(&dno.slug);
            
            if let Err(e) = self.cache.set(&id_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by ID: {}", e);
            }
            
            if let Err(e) = self.cache.set(&slug_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by slug: {}", e);
            }
        } else {
            // Cache negative result with shorter TTL
            if let Err(e) = self.cache.set(&cache_key, &Option::<Dno>::None, Some(Duration::from_secs(300))).await {
                warn!("Failed to cache negative DNO result: {}", e);
            }
        }

        Ok(dno)
    }

    /// Get DNO by slug with caching
    pub async fn get_dno_by_slug(&self, slug: &str) -> Result<Option<Dno>, AppError> {
        let cache_key = CacheKeys::dno_by_slug(slug);

        // Try cache first
        match self.cache.get::<Dno>(&cache_key).await {
            Ok(Some(dno)) => {
                debug!("Cache HIT for DNO by slug: {}", slug);
                return Ok(Some(dno));
            }
            Ok(None) => {
                debug!("Cache MISS for DNO by slug: {}", slug);
            }
            Err(e) => {
                warn!("Cache error for DNO by slug {}: {}", slug, e);
            }
        }

        // Cache miss - fetch from database
        let dno = database::get_dno_by_slug(&self.db, slug).await?;

        // Cache the result
        if let Some(ref dno) = dno {
            // Cache by slug
            if let Err(e) = self.cache.set(&cache_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by slug: {}", e);
            }
            
            // Also cache by ID and name for cross-reference
            let id_key = CacheKeys::dno_by_id(dno.id);
            let name_key = CacheKeys::dno_by_name(&dno.name);
            
            if let Err(e) = self.cache.set(&id_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by ID: {}", e);
            }
            
            if let Err(e) = self.cache.set(&name_key, dno, Some(self.dno_ttl)).await {
                warn!("Failed to cache DNO by name: {}", e);
            }
        } else {
            // Cache negative result with shorter TTL
            if let Err(e) = self.cache.set(&cache_key, &Option::<Dno>::None, Some(Duration::from_secs(300))).await {
                warn!("Failed to cache negative DNO result: {}", e);
            }
        }

        Ok(dno)
    }

    /// Create a new DNO and invalidate cache
    pub async fn create_dno(&self, dno: CreateDno) -> Result<Dno, AppError> {
        let created_dno = database::create_dno(&self.db, dno).await?;

        // Invalidate the all DNOs cache
        if let Err(e) = self.cache.delete(&CacheKeys::all_dnos()).await {
            warn!("Failed to invalidate all DNOs cache: {}", e);
        }

        // Cache the new DNO
        let id_key = CacheKeys::dno_by_id(created_dno.id);
        let name_key = CacheKeys::dno_by_name(&created_dno.name);
        let slug_key = CacheKeys::dno_by_slug(&created_dno.slug);

        if let Err(e) = self.cache.set(&id_key, &created_dno, Some(self.dno_ttl)).await {
            warn!("Failed to cache new DNO by ID: {}", e);
        }
        
        if let Err(e) = self.cache.set(&name_key, &created_dno, Some(self.dno_ttl)).await {
            warn!("Failed to cache new DNO by name: {}", e);
        }
        
        if let Err(e) = self.cache.set(&slug_key, &created_dno, Some(self.dno_ttl)).await {
            warn!("Failed to cache new DNO by slug: {}", e);
        }

        debug!("Created and cached new DNO: {}", created_dno.id);
        Ok(created_dno)
    }

    /// Update DNO and refresh cache
    pub async fn update_dno(&self, dno_id: Uuid, updates: UpdateDno) -> Result<Dno, AppError> {
        // Get the old DNO first to invalidate old cache entries
        let old_dno = self.get_dno_by_id(dno_id).await?;

        let updated_dno = database::update_dno(&self.db, dno_id, updates).await?;

        // Invalidate old cache entries
        if let Some(old_dno) = old_dno {
            let old_name_key = CacheKeys::dno_by_name(&old_dno.name);
            let old_slug_key = CacheKeys::dno_by_slug(&old_dno.slug);
            
            if let Err(e) = self.cache.delete(&old_name_key).await {
                warn!("Failed to invalidate old DNO name cache: {}", e);
            }
            
            if let Err(e) = self.cache.delete(&old_slug_key).await {
                warn!("Failed to invalidate old DNO slug cache: {}", e);
            }
        }

        // Invalidate the all DNOs cache
        if let Err(e) = self.cache.delete(&CacheKeys::all_dnos()).await {
            warn!("Failed to invalidate all DNOs cache: {}", e);
        }

        // Cache the updated DNO
        let id_key = CacheKeys::dno_by_id(updated_dno.id);
        let name_key = CacheKeys::dno_by_name(&updated_dno.name);
        let slug_key = CacheKeys::dno_by_slug(&updated_dno.slug);

        if let Err(e) = self.cache.set(&id_key, &updated_dno, Some(self.dno_ttl)).await {
            warn!("Failed to cache updated DNO by ID: {}", e);
        }
        
        if let Err(e) = self.cache.set(&name_key, &updated_dno, Some(self.dno_ttl)).await {
            warn!("Failed to cache updated DNO by name: {}", e);
        }
        
        if let Err(e) = self.cache.set(&slug_key, &updated_dno, Some(self.dno_ttl)).await {
            warn!("Failed to cache updated DNO by slug: {}", e);
        }

        debug!("Updated and re-cached DNO: {}", updated_dno.id);
        Ok(updated_dno)
    }

    /// Delete DNO and invalidate cache
    pub async fn delete_dno(&self, dno_id: Uuid) -> Result<(), AppError> {
        // Get the DNO first to invalidate cache entries
        let dno = self.get_dno_by_id(dno_id).await?;

        database::delete_dno(&self.db, dno_id).await?;

        // Invalidate all related cache entries
        if let Some(dno) = dno {
            let id_key = CacheKeys::dno_by_id(dno.id);
            let name_key = CacheKeys::dno_by_name(&dno.name);
            let slug_key = CacheKeys::dno_by_slug(&dno.slug);
            
            if let Err(e) = self.cache.delete(&id_key).await {
                warn!("Failed to invalidate deleted DNO ID cache: {}", e);
            }
            
            if let Err(e) = self.cache.delete(&name_key).await {
                warn!("Failed to invalidate deleted DNO name cache: {}", e);
            }
            
            if let Err(e) = self.cache.delete(&slug_key).await {
                warn!("Failed to invalidate deleted DNO slug cache: {}", e);
            }
        }

        // Invalidate the all DNOs cache
        if let Err(e) = self.cache.delete(&CacheKeys::all_dnos()).await {
            warn!("Failed to invalidate all DNOs cache: {}", e);
        }

        // Also invalidate search-related caches that depend on DNO data
        if let Err(e) = self.cache.invalidate_pattern("filters:available:").await {
            warn!("Failed to invalidate available filters cache: {}", e);
        }

        debug!("Deleted DNO and invalidated cache: {}", dno_id);
        Ok(())
    }

    /// Warm up DNO cache by pre-loading all DNOs
    pub async fn warm_cache(&self) -> Result<(), AppError> {
        debug!("Starting DNO cache warm-up");
        
        // Pre-load all DNOs (this will cache individual DNOs as well)
        let _ = self.get_all_dnos().await?;
        
        debug!("DNO cache warm-up completed");
        Ok(())
    }

    /// Invalidate all DNO-related caches
    pub async fn invalidate_all_caches(&self) -> Result<(), AppError> {
        // Invalidate all DNO reference caches
        if let Err(e) = self.cache.invalidate_pattern("reference:dno").await {
            warn!("Failed to invalidate DNO reference caches: {}", e);
        }

        // Also invalidate dependent caches
        if let Err(e) = self.cache.invalidate_pattern("search:").await {
            warn!("Failed to invalidate search caches: {}", e);
        }

        if let Err(e) = self.cache.invalidate_pattern("filters:available:").await {
            warn!("Failed to invalidate available filters cache: {}", e);
        }

        debug!("Invalidated all DNO-related caches");
        Ok(())
    }
}