use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct UserSql {
    pub id : i64,
    pub username: String,
    pub password: String
}

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}