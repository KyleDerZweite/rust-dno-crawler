use crate::{config::DatabaseConfig, AppError};
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

// DNO management functions
pub async fn get_all_dnos(pool: &PgPool) -> Result<Vec<Dno>, AppError> {
    let result = sqlx::query_as!(
        Dno,
        r#"
        SELECT id, slug, name, official_name, description, region, website,
               created_at, updated_at, deleted_at
        FROM dnos 
        WHERE deleted_at IS NULL
        ORDER BY name ASC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn get_dno_by_id(pool: &PgPool, dno_id: Uuid) -> Result<Option<Dno>, AppError> {
    let result = sqlx::query_as!(
        Dno,
        r#"
        SELECT id, slug, name, official_name, description, region, website,
               created_at, updated_at, deleted_at
        FROM dnos 
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        dno_id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn get_dno_by_name(pool: &PgPool, name: &str) -> Result<Option<Dno>, AppError> {
    let result = sqlx::query_as!(
        Dno,
        r#"
        SELECT id, slug, name, official_name, description, region, website,
               created_at, updated_at, deleted_at
        FROM dnos 
        WHERE (name ILIKE $1 OR official_name ILIKE $1) AND deleted_at IS NULL
        "#,
        format!("%{}%", name)
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn get_dno_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Dno>, AppError> {
    let result = sqlx::query_as!(
        Dno,
        r#"
        SELECT id, slug, name, official_name, description, region, website,
               created_at, updated_at, deleted_at
        FROM dnos 
        WHERE slug = $1 AND deleted_at IS NULL
        "#,
        slug
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn create_dno(pool: &PgPool, dno: CreateDno) -> Result<Dno, AppError> {
    let result = sqlx::query_as!(
        Dno,
        r#"
        INSERT INTO dnos (slug, name, official_name, description, region, website)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, slug, name, official_name, description, region, website,
                  created_at, updated_at, deleted_at
        "#,
        dno.slug,
        dno.name,
        dno.official_name,
        dno.description,
        dno.region,
        dno.website
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn update_dno(pool: &PgPool, dno_id: Uuid, updates: UpdateDno) -> Result<Dno, AppError> {
    let result = sqlx::query_as!(
        Dno,
        r#"
        UPDATE dnos 
        SET slug = COALESCE($2, slug),
            name = COALESCE($3, name),
            official_name = COALESCE($4, official_name),
            description = COALESCE($5, description),
            region = COALESCE($6, region),
            website = COALESCE($7, website),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, slug, name, official_name, description, region, website,
                  created_at, updated_at, deleted_at
        "#,
        dno_id,
        updates.slug,
        updates.name,
        updates.official_name,
        updates.description,
        updates.region,
        updates.website
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn delete_dno(pool: &PgPool, dno_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE dnos SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1",
        dno_id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(())
}

// Netzentgelte data search functions
pub async fn search_netzentgelte_data(
    pool: &PgPool,
    dno_id: Option<Uuid>,
    dno_name: Option<&str>,
    year: Option<i32>,
    verification_status: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<NetzentgelteDataWithDno>, AppError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT 
            n.id, n.dno_id, n.year, n.voltage_level,
            n.leistung, n.arbeit, n.leistung_unter_2500h, n.arbeit_unter_2500h,
            n.verification_status, n.verified_by, n.verified_at, n.verification_notes,
            n.created_at, n.updated_at, n.deleted_at,
            d.id as dno_id_full, d.slug as dno_slug, d.name as dno_name, 
            d.official_name as dno_official_name, d.region as dno_region
        FROM netzentgelte_data n
        JOIN dnos d ON n.dno_id = d.id
        WHERE n.deleted_at IS NULL AND d.deleted_at IS NULL
        "#
    );

    let _has_where = true;

    if let Some(dno_id) = dno_id {
        query_builder.push(" AND n.dno_id = ");
        query_builder.push_bind(dno_id);
    }

    if let Some(dno_name) = dno_name {
        query_builder.push(" AND (d.name ILIKE ");
        query_builder.push_bind(format!("%{}%", dno_name));
        query_builder.push(" OR d.official_name ILIKE ");
        query_builder.push_bind(format!("%{}%", dno_name));
        query_builder.push(")");
    }

    if let Some(year) = year {
        query_builder.push(" AND n.year = ");
        query_builder.push_bind(year);
    }

    if let Some(status) = verification_status {
        query_builder.push(" AND n.verification_status = ");
        query_builder.push_bind(status);
    }

    query_builder.push(" ORDER BY n.created_at DESC, d.name ASC LIMIT ");
    query_builder.push_bind(limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let query = query_builder.build_query_as::<NetzentgelteDataWithDno>();
    let result = query.fetch_all(pool).await.map_err(AppError::Database)?;

    Ok(result)
}

pub async fn count_netzentgelte_data(
    pool: &PgPool,
    dno_id: Option<Uuid>,
    dno_name: Option<&str>,
    year: Option<i32>,
    verification_status: Option<&str>,
) -> Result<i64, AppError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT COUNT(*)
        FROM netzentgelte_data n
        JOIN dnos d ON n.dno_id = d.id
        WHERE n.deleted_at IS NULL AND d.deleted_at IS NULL
        "#
    );

    if let Some(dno_id) = dno_id {
        query_builder.push(" AND n.dno_id = ");
        query_builder.push_bind(dno_id);
    }

    if let Some(dno_name) = dno_name {
        query_builder.push(" AND (d.name ILIKE ");
        query_builder.push_bind(format!("%{}%", dno_name));
        query_builder.push(" OR d.official_name ILIKE ");
        query_builder.push_bind(format!("%{}%", dno_name));
        query_builder.push(")");
    }

    if let Some(year) = year {
        query_builder.push(" AND n.year = ");
        query_builder.push_bind(year);
    }

    if let Some(status) = verification_status {
        query_builder.push(" AND n.verification_status = ");
        query_builder.push_bind(status);
    }

    let query = query_builder.build_query_scalar::<i64>();
    let result = query.fetch_one(pool).await.map_err(AppError::Database)?;

    Ok(result)
}

// HLZF data search functions
pub async fn search_hlzf_data(
    pool: &PgPool,
    dno_id: Option<Uuid>,
    dno_name: Option<&str>,
    year: Option<i32>,
    verification_status: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<HlzfDataWithDno>, AppError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT 
            h.id, h.dno_id, h.year, h.season, h.voltage_level,
            h.ht, h.nt, h.start_date, h.end_date,
            h.verification_status, h.verified_by, h.verified_at, h.verification_notes,
            h.created_at, h.updated_at, h.deleted_at,
            d.id as dno_id_full, d.slug as dno_slug, d.name as dno_name, 
            d.official_name as dno_official_name, d.region as dno_region
        FROM hlzf_data h
        JOIN dnos d ON h.dno_id = d.id
        WHERE h.deleted_at IS NULL AND d.deleted_at IS NULL
        "#
    );

    if let Some(dno_id) = dno_id {
        query_builder.push(" AND h.dno_id = ");
        query_builder.push_bind(dno_id);
    }

    if let Some(dno_name) = dno_name {
        query_builder.push(" AND (d.name ILIKE ");
        query_builder.push_bind(format!("%{}%", dno_name));
        query_builder.push(" OR d.official_name ILIKE ");
        query_builder.push_bind(format!("%{}%", dno_name));
        query_builder.push(")");
    }

    if let Some(year) = year {
        query_builder.push(" AND h.year = ");
        query_builder.push_bind(year);
    }

    if let Some(status) = verification_status {
        query_builder.push(" AND h.verification_status = ");
        query_builder.push_bind(status);
    }

    query_builder.push(" ORDER BY h.created_at DESC, d.name ASC LIMIT ");
    query_builder.push_bind(limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let query = query_builder.build_query_as::<HlzfDataWithDno>();
    let result = query.fetch_all(pool).await.map_err(AppError::Database)?;

    Ok(result)
}

// Dashboard and analytics functions
pub async fn get_dashboard_stats(pool: &PgPool, user_id: Uuid) -> Result<DashboardStats, AppError> {
    // Get user's query count for today
    let queries_today = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM query_logs 
        WHERE user_id = $1 AND DATE(created_at) = CURRENT_DATE
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?
    .unwrap_or(0);

    // Get user's query count for this month
    let queries_this_month = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM query_logs 
        WHERE user_id = $1 AND DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE)
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?
    .unwrap_or(0);

    // Get total DNO count
    let total_dnos = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM dnos WHERE deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?
    .unwrap_or(0);

    // Get total data entries
    let netzentgelte_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM netzentgelte_data WHERE deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?
    .unwrap_or(0);

    let hlzf_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM hlzf_data WHERE deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?
    .unwrap_or(0);

    // Get available years
    let available_years = sqlx::query_scalar!(
        r#"
        SELECT DISTINCT year 
        FROM (
            SELECT year FROM netzentgelte_data WHERE deleted_at IS NULL
            UNION
            SELECT year FROM hlzf_data WHERE deleted_at IS NULL
        ) AS years
        ORDER BY year DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(DashboardStats {
        queries_today: queries_today as u32,
        queries_this_month: queries_this_month as u32,
        total_dnos: total_dnos as u32,
        total_data_entries: (netzentgelte_count + hlzf_count) as u32,
        available_years,
    })
}

pub async fn get_available_years_and_dnos(pool: &PgPool) -> Result<AvailableFilters, AppError> {
    // Get available years
    let years = sqlx::query_scalar!(
        r#"
        SELECT DISTINCT year 
        FROM (
            SELECT year FROM netzentgelte_data WHERE deleted_at IS NULL
            UNION
            SELECT year FROM hlzf_data WHERE deleted_at IS NULL
        ) AS years
        ORDER BY year DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    // Get available DNOs
    let dnos = sqlx::query_as!(
        DnoInfo,
        r#"
        SELECT DISTINCT d.id, d.name, d.slug, d.region
        FROM dnos d
        WHERE d.deleted_at IS NULL
        AND (
            EXISTS (SELECT 1 FROM netzentgelte_data n WHERE n.dno_id = d.id AND n.deleted_at IS NULL)
            OR
            EXISTS (SELECT 1 FROM hlzf_data h WHERE h.dno_id = d.id AND h.deleted_at IS NULL)
        )
        ORDER BY d.name ASC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    // Get available regions
    let regions = sqlx::query_scalar!(
        r#"
        SELECT DISTINCT region
        FROM dnos
        WHERE deleted_at IS NULL AND region IS NOT NULL
        ORDER BY region ASC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(AvailableFilters {
        years,
        dnos,
        regions: regions.into_iter().filter_map(|r| r).collect(),
        data_types: vec!["netzentgelte".to_string(), "hlzf".to_string()],
    })
}

// Query logging functions
pub async fn log_query(pool: &PgPool, log: CreateQueryLog) -> Result<QueryLog, AppError> {
    let result = sqlx::query_as!(
        QueryLog,
        r#"
        INSERT INTO query_logs (user_id, query, interpretation, response_time_ms, source_ip)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, query, interpretation, response_time_ms, source_ip, created_at
        "#,
        log.user_id,
        log.query,
        log.interpretation,
        log.response_time_ms,
        log.source_ip
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

pub async fn get_user_query_history(
    pool: &PgPool,
    user_id: Uuid,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<QueryLog>, AppError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let result = sqlx::query_as!(
        QueryLog,
        r#"
        SELECT id, user_id, query, interpretation, response_time_ms, source_ip, created_at
        FROM query_logs
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        user_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result)
}

// Transaction helpers
pub async fn begin_transaction(pool: &PgPool) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, AppError> {
    pool.begin().await.map_err(AppError::Database)
}

// Health check function
pub async fn health_check(pool: &PgPool) -> Result<(), AppError> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;
    
    Ok(())
}
