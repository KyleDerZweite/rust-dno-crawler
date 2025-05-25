use serde::Deserialize;
use crate::core::{
    errors::AppError,
    models::{CreateUserRequest, User},
};
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, AppError> {
        // Erstelle Datenbankordner falls nicht vorhanden
        if database_url.contains("sqlite:") && !database_url.contains(":memory:") {
            let db_path = database_url.replace("sqlite:", "");
            if let Some(parent) = std::path::Path::new(&db_path).parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| AppError::InternalServerError(format!("Failed to create database directory: {}", e)))?;
            }
        }

        let pool = SqlitePool::connect(database_url).await
            .map_err(|e| AppError::Database(e))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT UNIQUE NOT NULL,
                hashed_password TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'guest',
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
            .execute(&pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(Database { pool })
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, AppError> {
        let user = User::new(request.email, &request.password, request.role)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        
        sqlx::query!(
            "INSERT INTO users (id, email, hashed_password, role, created_at) VALUES (?, ?, ?, ?, ?)",
            user.id,
            user.email,
            user.hashed_password,
            user.role,
            user.created_at
        )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, hashed_password, role, created_at FROM users WHERE email = ?",
            email
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, hashed_password, role, created_at FROM users WHERE id = ?",
            id
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn list_users(
        &self,
        page: Option<usize>,
        limit: Option<usize>,
        role_filter: Option<&str>
    ) -> Result<Vec<User>, AppError> {
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        let users = match role_filter {
            Some(role) => {
                sqlx::query_as!(
                    User,
                    "SELECT id, email, hashed_password, role, created_at FROM users WHERE role = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
                    role,
                    limit as i64,
                    offset as i64
                )
                    .fetch_all(&self.pool)
                    .await
                    .map_err(|e| AppError::Database(e))?
            }
            None => {
                sqlx::query_as!(
                    User,
                    "SELECT id, email, hashed_password, role, created_at FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?",
                    limit as i64,
                    offset as i64
                )
                    .fetch_all(&self.pool)
                    .await
                    .map_err(|e| AppError::Database(e))?
            }
        };

        Ok(users)
    }

    pub async fn count_users(&self, role_filter: Option<&str>) -> Result<usize, AppError> {
        let count = match role_filter {
            Some(role) => {
                sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM users WHERE role = ?",
                    role
                )
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|e| AppError::Database(e))?
            }
            None => {
                sqlx::query_scalar!("SELECT COUNT(*) FROM users")
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|e| AppError::Database(e))?
            }
        };

        Ok(count as usize)
    }

    pub async fn update_user(&self, id: &str, request: UpdateUserRequest) -> Result<User, AppError> {
        // Lade aktuellen Benutzer
        let mut user = self.get_user_by_id(id).await?
            .ok_or_else(|| AppError::BadRequest("User not found".to_string()))?;

        // Update Felder wenn vorhanden
        if let Some(email) = request.email {
            user.email = email;
        }
        if let Some(role) = request.role {
            user.role = role;
        }

        // Speichere Änderungen
        sqlx::query!(
            "UPDATE users SET email = ?, role = ? WHERE id = ?",
            user.email,
            user.role,
            id
        )
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn delete_user(&self, id: &str) -> Result<(), AppError> {
        let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::BadRequest("User not found".to_string()));
        }

        Ok(())
    }
}