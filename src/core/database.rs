use crate::core::{
    errors::AppError,
    models::{CreateUserRequest, User},
};
use sqlx::{Pool, Sqlite, SqlitePool};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Clone, Debug)]
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

        // Erstelle Tabellen wenn sie nicht existieren
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'user',
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                last_login DATETIME
            )
            "#,
        )
            .execute(&pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(Database { pool })
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, AppError> {
        let user = User::new(request.name, request.email, &request.password, request.role)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        
        let sql_user_clone = user.clone();
        
        sqlx::query(
            "INSERT INTO users (id,name, email, password_hash, role, created_at, last_login) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
            .bind(sql_user_clone.id)
            .bind(sql_user_clone.name)
            .bind(sql_user_clone.email)
            .bind(sql_user_clone.password_hash)
            .bind(sql_user_clone.role)
            .bind(sql_user_clone.created_at)
            .bind(sql_user_clone.last_login)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;
        
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, role, created_at, last_login FROM users WHERE email = ?",
        )
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, role, created_at, last_login FROM users WHERE id = ?",
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn update_last_login(&self, user_id: &str) -> Result<(), AppError> {
        let now = chrono::Utc::now().naive_utc();
        sqlx::query(
            "UPDATE users SET last_login = ? WHERE id = ?",
        )
            .bind(now)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(())
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
                sqlx::query_as::<_, User>(
                    "SELECT id, name, email, password_hash, role, created_at, last_login FROM users WHERE role = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
                )
                    .bind(role)
                    .bind(limit as i64)
                    .bind(offset as i64)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(|e| AppError::Database(e))?
            }
            None => {
                sqlx::query_as::<_, User>(
                    "SELECT id, name, email, password_hash, role, created_at, last_login FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?",
                )
                    .bind(limit as i64)
                    .bind(offset as i64)
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
                sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM users WHERE role = ?",
                )
                    .bind(role)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|e| AppError::Database(e))?
            }
            None => {
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
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

        let sql_user_clone = user.clone();
        
        // Speichere Änderungen
        sqlx::query(
            "UPDATE users SET email = ?, role = ? WHERE id = ?",
        )
            .bind(sql_user_clone.email)
            .bind(sql_user_clone.role)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    pub async fn delete_user(&self, id: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::BadRequest("User not found".to_string()));
        }

        Ok(())
    }
}
