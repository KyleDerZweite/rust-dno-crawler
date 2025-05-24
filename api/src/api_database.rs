use core::errors::AppError;
use crate::models::{DataItem, User};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct DatabasePool {
    pub pool: PgPool,
}

impl DatabasePool {
    pub async fn new(database_url: &str) -> Result<Self, AppError> {
        let pool = Pool::<Postgres>::connect(database_url).await?;
        Ok(DatabasePool { pool })
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, created_at FROM users WHERE username = $1"
        )
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn get_data_by_name(&self, name: &str) -> Result<Vec<DataItem>, AppError> {
        let data = sqlx::query_as::<_, DataItem>(
            "SELECT id, name, value, created_at FROM data_items WHERE name ILIKE $1 ORDER BY created_at DESC"
        )
            .bind(format!("%{}%", name))
            .fetch_all(&self.pool)
            .await?;

        Ok(data)
    }

    // Helper function to create a user (for testing)
    pub async fn create_user(&self, username: &str, password: &str) -> Result<User, AppError> {
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        let id = Uuid::new_v4();

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3) RETURNING id, username, password_hash, created_at"
        )
            .bind(id)
            .bind(username)
            .bind(password_hash)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
}