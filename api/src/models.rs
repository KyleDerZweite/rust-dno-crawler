use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct DataResponse {
    pub data: Vec<DataItem>,
}

#[derive(Serialize, FromRow)]
pub struct DataItem {
    pub id: Uuid,
    pub name: String,
    pub value: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
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