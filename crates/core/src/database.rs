use crate::{DatabaseConfig, AppError};
use crate::models::*;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tracing::{info, error};
use uuid::Uuid;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, AppError> {
    info!("Connecting to PostgreSQL database: {}", config.url);
    
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout))
        .idle_timeout(Duration::from_secs(config.idle_timeout))
        .test_before_acquire(true)
        .connect(&config.url)
        .await
        .map_err(|e| {
            error!("Failed to connect to PostgreSQL database: {}", e);
            AppError::Database(e)
        })?;

    info!("PostgreSQL database connection pool created successfully");
    Ok(pool)
}

// User authentication functions
pub async fn create_user(pool: &PgPool, user: CreateUser) -> Result<User, AppError> {
    let result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash, name, role)
        VALUES ($1, $2, $3, $4)
        RETURNING id, email, password_hash, name, role as "role!: UserRole", 
                  profile_picture_url, is_active, email_verified, verification_status,
                  approved_by, approved_at, rejected_at, created_at, updated_at, deleted_at
        "#,
        user.email,
        user.password_hash,
        user.name,
        user.role.unwrap_or(UserRole::Pending) as UserRole
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    let result = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, name, role as "role!: UserRole", 
               profile_picture_url, is_active, email_verified, verification_status,
               approved_by, approved_at, rejected_at, created_at, updated_at, deleted_at
        FROM users 
        WHERE email = $1 AND deleted_at IS NULL
        "#,
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, AppError> {
    let result = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, name, role as "role!: UserRole", 
               profile_picture_url, is_active, email_verified, verification_status,
               approved_by, approved_at, rejected_at, created_at, updated_at, deleted_at
        FROM users 
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn update_user(pool: &PgPool, user_id: Uuid, updates: UpdateUser) -> Result<User, AppError> {
    let result = sqlx::query_as!(
        User,
        r#"
        UPDATE users 
        SET email = COALESCE($2, email),
            name = COALESCE($3, name),
            role = COALESCE($4, role),
            profile_picture_url = COALESCE($5, profile_picture_url),
            is_active = COALESCE($6, is_active),
            email_verified = COALESCE($7, email_verified),
            verification_status = COALESCE($8, verification_status),
            approved_by = COALESCE($9, approved_by),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, email, password_hash, name, role as "role!: UserRole", 
                  profile_picture_url, is_active, email_verified, verification_status,
                  approved_by, approved_at, rejected_at, created_at, updated_at, deleted_at
        "#,
        user_id,
        updates.email,
        updates.name,
        updates.role as Option<UserRole>,
        updates.profile_picture_url,
        updates.is_active,
        updates.email_verified,
        updates.verification_status,
        updates.approved_by
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn approve_user(pool: &PgPool, user_id: Uuid, approved_by: Uuid) -> Result<User, AppError> {
    let result = sqlx::query_as!(
        User,
        r#"
        UPDATE users 
        SET role = 'user',
            verification_status = 'approved',
            approved_by = $2,
            approved_at = CURRENT_TIMESTAMP,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, email, password_hash, name, role as "role!: UserRole", 
                  profile_picture_url, is_active, email_verified, verification_status,
                  approved_by, approved_at, rejected_at, created_at, updated_at, deleted_at
        "#,
        user_id,
        approved_by
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn reject_user(pool: &PgPool, user_id: Uuid) -> Result<User, AppError> {
    let result = sqlx::query_as!(
        User,
        r#"
        UPDATE users 
        SET verification_status = 'rejected',
            rejected_at = CURRENT_TIMESTAMP,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, email, password_hash, name, role as "role!: UserRole", 
                  profile_picture_url, is_active, email_verified, verification_status,
                  approved_by, approved_at, rejected_at, created_at, updated_at, deleted_at
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn list_users(pool: &PgPool, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, AppError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let result = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, name, role as "role!: UserRole", 
               profile_picture_url, is_active, email_verified, verification_status,
               approved_by, approved_at, rejected_at, created_at, updated_at, deleted_at
        FROM users 
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

// Session management functions
pub async fn create_session(pool: &PgPool, session: CreateSession) -> Result<Session, AppError> {
    let result = sqlx::query_as!(
        Session,
        r#"
        INSERT INTO sessions (user_id, token_hash, refresh_token_hash, expires_at, refresh_expires_at, ip_address, user_agent)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, user_id, token_hash, refresh_token_hash, expires_at, refresh_expires_at,
                  ip_address, user_agent, is_active, created_at, last_used
        "#,
        session.user_id,
        session.token_hash,
        session.refresh_token_hash,
        session.expires_at,
        session.refresh_expires_at,
        session.ip_address,
        session.user_agent
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn get_session_by_token_hash(pool: &PgPool, token_hash: &str) -> Result<Option<Session>, AppError> {
    let result = sqlx::query_as!(
        Session,
        r#"
        SELECT id, user_id, token_hash, refresh_token_hash, expires_at, refresh_expires_at,
               ip_address, user_agent, is_active, created_at, last_used
        FROM sessions 
        WHERE token_hash = $1 AND is_active = true AND expires_at > CURRENT_TIMESTAMP
        "#,
        token_hash
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn get_session_by_refresh_token_hash(pool: &PgPool, refresh_token_hash: &str) -> Result<Option<Session>, AppError> {
    let result = sqlx::query_as!(
        Session,
        r#"
        SELECT id, user_id, token_hash, refresh_token_hash, expires_at, refresh_expires_at,
               ip_address, user_agent, is_active, created_at, last_used
        FROM sessions 
        WHERE refresh_token_hash = $1 AND is_active = true AND refresh_expires_at > CURRENT_TIMESTAMP
        "#,
        refresh_token_hash
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn update_session_last_used(pool: &PgPool, session_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE sessions SET last_used = CURRENT_TIMESTAMP WHERE id = $1",
        session_id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(())
}

pub async fn invalidate_session(pool: &PgPool, session_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE sessions SET is_active = false WHERE id = $1",
        session_id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(())
}

pub async fn invalidate_user_sessions(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE sessions SET is_active = false WHERE user_id = $1",
        user_id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(())
}

// API Key management functions
pub async fn create_api_key(pool: &PgPool, api_key: CreateApiKey) -> Result<ApiKey, AppError> {
    let result = sqlx::query_as!(
        ApiKey,
        r#"
        INSERT INTO api_keys (user_id, name, key_hash, masked_key, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, name, key_hash, masked_key, last_used, expires_at, created_at
        "#,
        api_key.user_id,
        api_key.name,
        api_key.key_hash,
        api_key.masked_key,
        api_key.expires_at
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn get_api_keys_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<ApiKey>, AppError> {
    let result = sqlx::query_as!(
        ApiKey,
        "SELECT id, user_id, name, key_hash, masked_key, last_used, expires_at, created_at FROM api_keys WHERE user_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn delete_api_key(pool: &PgPool, api_key_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM api_keys WHERE id = $1 AND user_id = $2",
        api_key_id,
        user_id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(())
}
