use crate::{
    cache::{CacheLayer, CacheKeys},
    database, AppError, User, CreateUser, UpdateUser, Session, CreateSession,
};
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};
use uuid::Uuid;

/// Repository for user and authentication operations with Redis caching
#[derive(Clone)]
pub struct UserRepository<C: CacheLayer> {
    db: PgPool,
    cache: Arc<C>,
    session_ttl: Duration,
    user_ttl: Duration,
}

impl<C: CacheLayer> UserRepository<C> {
    pub fn new(db: PgPool, cache: Arc<C>) -> Self {
        Self {
            db,
            cache,
            session_ttl: Duration::from_secs(3600), // 1 hour
            user_ttl: Duration::from_secs(1800),    // 30 minutes
        }
    }

    /// Create a new user (no caching for create operations)
    pub async fn create_user(&self, user: CreateUser) -> Result<User, AppError> {
        let created_user = database::create_user(&self.db, user).await?;
        
        // Cache the newly created user
        let user_id_key = CacheKeys::user_by_id(created_user.id);
        let user_email_key = CacheKeys::user_by_email(&created_user.email);
        
        if let Err(e) = self.cache.set(&user_id_key, &created_user, Some(self.user_ttl)).await {
            warn!("Failed to cache new user by ID: {}", e);
        }
        
        if let Err(e) = self.cache.set(&user_email_key, &created_user, Some(self.user_ttl)).await {
            warn!("Failed to cache new user by email: {}", e);
        }

        debug!("Created and cached new user: {}", created_user.id);
        Ok(created_user)
    }

    /// Get user by email with caching
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let cache_key = CacheKeys::user_by_email(email);

        // Try cache first
        match self.cache.get::<User>(&cache_key).await {
            Ok(Some(user)) => {
                debug!("Cache HIT for user by email: {}", email);
                return Ok(Some(user));
            }
            Ok(None) => {
                debug!("Cache MISS for user by email: {}", email);
            }
            Err(e) => {
                warn!("Cache error for user by email {}: {}", email, e);
            }
        }

        // Cache miss - fetch from database
        let user = database::get_user_by_email(&self.db, email).await?;

        // Cache the result
        if let Some(ref user) = user {
            // Cache by email
            if let Err(e) = self.cache.set(&cache_key, user, Some(self.user_ttl)).await {
                warn!("Failed to cache user by email: {}", e);
            }
            
            // Also cache by ID for faster lookups
            let id_key = CacheKeys::user_by_id(user.id);
            if let Err(e) = self.cache.set(&id_key, user, Some(self.user_ttl)).await {
                warn!("Failed to cache user by ID: {}", e);
            }
        } else {
            // Cache negative result with shorter TTL
            if let Err(e) = self.cache.set(&cache_key, &Option::<User>::None, Some(Duration::from_secs(300))).await {
                warn!("Failed to cache negative user result: {}", e);
            }
        }

        Ok(user)
    }

    /// Get user by ID with caching
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError> {
        let cache_key = CacheKeys::user_by_id(user_id);

        // Try cache first
        match self.cache.get::<User>(&cache_key).await {
            Ok(Some(user)) => {
                debug!("Cache HIT for user by ID: {}", user_id);
                return Ok(Some(user));
            }
            Ok(None) => {
                debug!("Cache MISS for user by ID: {}", user_id);
            }
            Err(e) => {
                warn!("Cache error for user by ID {}: {}", user_id, e);
            }
        }

        // Cache miss - fetch from database
        let user = database::get_user_by_id(&self.db, user_id).await?;

        // Cache the result
        if let Some(ref user) = user {
            // Cache by ID
            if let Err(e) = self.cache.set(&cache_key, user, Some(self.user_ttl)).await {
                warn!("Failed to cache user by ID: {}", e);
            }
            
            // Also cache by email for cross-reference
            let email_key = CacheKeys::user_by_email(&user.email);
            if let Err(e) = self.cache.set(&email_key, user, Some(self.user_ttl)).await {
                warn!("Failed to cache user by email: {}", e);
            }
        } else {
            // Cache negative result with shorter TTL
            if let Err(e) = self.cache.set(&cache_key, &Option::<User>::None, Some(Duration::from_secs(300))).await {
                warn!("Failed to cache negative user result: {}", e);
            }
        }

        Ok(user)
    }

    /// Update user and invalidate cache
    pub async fn update_user(&self, user_id: Uuid, updates: UpdateUser) -> Result<User, AppError> {
        let updated_user = database::update_user(&self.db, user_id, updates).await?;

        // Invalidate cache entries
        let id_key = CacheKeys::user_by_id(user_id);
        let email_key = CacheKeys::user_by_email(&updated_user.email);

        if let Err(e) = self.cache.delete(&id_key).await {
            warn!("Failed to invalidate user cache by ID: {}", e);
        }
        
        if let Err(e) = self.cache.delete(&email_key).await {
            warn!("Failed to invalidate user cache by email: {}", e);
        }

        // Cache the updated user
        if let Err(e) = self.cache.set(&id_key, &updated_user, Some(self.user_ttl)).await {
            warn!("Failed to cache updated user by ID: {}", e);
        }
        
        if let Err(e) = self.cache.set(&email_key, &updated_user, Some(self.user_ttl)).await {
            warn!("Failed to cache updated user by email: {}", e);
        }

        debug!("Updated and re-cached user: {}", user_id);
        Ok(updated_user)
    }

    /// Create session with caching
    pub async fn create_session(&self, session: CreateSession) -> Result<Session, AppError> {
        let created_session = database::create_session(&self.db, session).await?;

        // Cache the session by token hash
        let token_key = CacheKeys::session_by_token(&created_session.token_hash);
        if let Err(e) = self.cache.set(&token_key, &created_session, Some(self.session_ttl)).await {
            warn!("Failed to cache new session by token: {}", e);
        }

        // Cache by refresh token if available
        if let Some(ref refresh_token_hash) = created_session.refresh_token_hash {
            let refresh_key = CacheKeys::session_by_refresh_token(refresh_token_hash);
            if let Err(e) = self.cache.set(&refresh_key, &created_session, Some(self.session_ttl)).await {
                warn!("Failed to cache new session by refresh token: {}", e);
            }
        }

        debug!("Created and cached new session for user: {}", created_session.user_id);
        Ok(created_session)
    }

    /// Get session by token hash with caching
    pub async fn get_session_by_token_hash(&self, token_hash: &str) -> Result<Option<Session>, AppError> {
        let cache_key = CacheKeys::session_by_token(token_hash);

        // Try cache first
        match self.cache.get::<Session>(&cache_key).await {
            Ok(Some(session)) => {
                debug!("Cache HIT for session by token");
                return Ok(Some(session));
            }
            Ok(None) => {
                debug!("Cache MISS for session by token");
            }
            Err(e) => {
                warn!("Cache error for session by token: {}", e);
            }
        }

        // Cache miss - fetch from database
        let session = database::get_session_by_token_hash(&self.db, token_hash).await?;

        // Cache the result
        if let Some(ref session) = session {
            if let Err(e) = self.cache.set(&cache_key, session, Some(self.session_ttl)).await {
                warn!("Failed to cache session by token: {}", e);
            }
        } else {
            // Cache negative result with shorter TTL
            if let Err(e) = self.cache.set(&cache_key, &Option::<Session>::None, Some(Duration::from_secs(60))).await {
                warn!("Failed to cache negative session result: {}", e);
            }
        }

        Ok(session)
    }

    /// Get session by refresh token hash with caching
    pub async fn get_session_by_refresh_token_hash(&self, refresh_token_hash: &str) -> Result<Option<Session>, AppError> {
        let cache_key = CacheKeys::session_by_refresh_token(refresh_token_hash);

        // Try cache first
        match self.cache.get::<Session>(&cache_key).await {
            Ok(Some(session)) => {
                debug!("Cache HIT for session by refresh token");
                return Ok(Some(session));
            }
            Ok(None) => {
                debug!("Cache MISS for session by refresh token");
            }
            Err(e) => {
                warn!("Cache error for session by refresh token: {}", e);
            }
        }

        // Cache miss - fetch from database
        let session = database::get_session_by_refresh_token_hash(&self.db, refresh_token_hash).await?;

        // Cache the result
        if let Some(ref session) = session {
            if let Err(e) = self.cache.set(&cache_key, session, Some(self.session_ttl)).await {
                warn!("Failed to cache session by refresh token: {}", e);
            }
        } else {
            // Cache negative result with shorter TTL
            if let Err(e) = self.cache.set(&cache_key, &Option::<Session>::None, Some(Duration::from_secs(60))).await {
                warn!("Failed to cache negative session result: {}", e);
            }
        }

        Ok(session)
    }

    /// Invalidate session and remove from cache
    pub async fn invalidate_session(&self, session_id: Uuid) -> Result<(), AppError> {
        database::invalidate_session(&self.db, session_id).await?;

        // Invalidate cache - we need to remove all possible cache entries
        // Since we don't have the exact token hashes, we'll use pattern invalidation
        if let Err(e) = self.cache.invalidate_pattern("auth:session:").await {
            warn!("Failed to invalidate session cache: {}", e);
        }

        debug!("Invalidated session and cache: {}", session_id);
        Ok(())
    }

    /// Update session last used timestamp and refresh cache
    pub async fn update_session_last_used(&self, session_id: Uuid) -> Result<(), AppError> {
        database::update_session_last_used(&self.db, session_id).await?;

        // Invalidate cache for this session to force refresh
        if let Err(e) = self.cache.invalidate_pattern("auth:session:").await {
            warn!("Failed to invalidate session cache after update: {}", e);
        }

        Ok(())
    }

    /// Get user query history with pagination caching
    pub async fn get_user_query_history(&self, user_id: Uuid, page: i64, limit: i64) -> Result<Vec<crate::QueryLog>, AppError> {
        let cache_key = CacheKeys::user_query_history(user_id, page);

        // Try cache first
        match self.cache.get::<Vec<crate::QueryLog>>(&cache_key).await {
            Ok(Some(history)) => {
                debug!("Cache HIT for user query history: user={}, page={}", user_id, page);
                return Ok(history);
            }
            Ok(None) => {
                debug!("Cache MISS for user query history: user={}, page={}", user_id, page);
            }
            Err(e) => {
                warn!("Cache error for user query history: {}", e);
            }
        }

        // Cache miss - fetch from database
        let history = database::get_user_query_history(&self.db, user_id, Some(limit), Some(page * limit)).await?;

        // Cache the result with shorter TTL since query history changes frequently
        if let Err(e) = self.cache.set(&cache_key, &history, Some(Duration::from_secs(600))).await {
            warn!("Failed to cache user query history: {}", e);
        }

        Ok(history)
    }

    /// Invalidate all cached data for a user
    pub async fn invalidate_user_cache(&self, user_id: Uuid, email: &str) -> Result<(), AppError> {
        let id_key = CacheKeys::user_by_id(user_id);
        let email_key = CacheKeys::user_by_email(email);

        if let Err(e) = self.cache.delete(&id_key).await {
            warn!("Failed to invalidate user cache by ID: {}", e);
        }
        
        if let Err(e) = self.cache.delete(&email_key).await {
            warn!("Failed to invalidate user cache by email: {}", e);
        }

        // Invalidate all user-related caches
        let user_pattern = format!("auth:user:{}", user_id);
        if let Err(e) = self.cache.invalidate_pattern(&user_pattern).await {
            warn!("Failed to invalidate user pattern cache: {}", e);
        }

        debug!("Invalidated all cache for user: {}", user_id);
        Ok(())
    }

    /// Warm up user-related caches (minimal implementation)
    pub async fn warm_cache(&self) -> Result<(), AppError> {
        debug!("User cache warm-up - nothing to pre-cache yet");
        Ok(())
    }
}