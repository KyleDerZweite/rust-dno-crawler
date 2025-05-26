use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::core::errors::AppError;
use crate::auth::password::PasswordService;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
            last_login: user.last_login,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // user_id als String
    pub exp: usize,     // Ablaufzeit (epoch seconds)
    pub iat: usize,     // Ausstellzeit
    pub role: String,   // Rollenbasierte Authentifizierung
}

impl User {
    pub fn new(email: String, password: &str, role: Option<String>) -> Result<Self, AppError> {
        let password_service = PasswordService::new();
        let password_hash = password_service.hash_password(password)?;
        let id = Uuid::new_v4().to_string();

        Ok(User {
            id,
            email,
            password_hash,
            role: role.unwrap_or_else(|| "user".to_string()),
            created_at: chrono::Utc::now().naive_utc(),
            last_login: None,
        })
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, AppError> {
        let password_service = PasswordService::new();
        password_service.verify_password(password, &self.password_hash)
    }

    pub fn update_last_login(&mut self) {
        self.last_login = Some(chrono::Utc::now().naive_utc());
    }

    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    pub fn can_access_api(&self) -> bool {
        self.role == "user" || self.role == "admin"
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum UserRoles {
    Guest,
    User,
    Admin,
}

impl std::fmt::Display for UserRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRoles::Guest => write!(f, "guest"),
            UserRoles::User => write!(f, "user"),
            UserRoles::Admin => write!(f, "admin"),
        }
    }
}

impl From<String> for UserRoles {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => UserRoles::Admin,
            "user" => UserRoles::User,
            "guest" | _ => UserRoles::Guest,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Metadata {
    pub key: String,
    pub dno_name: String, // Stored as JSON string in DB
    pub description: Option<String>,
    pub region: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Crawl {
    pub key: String,
    pub crawl_type: String,
    pub netzentgelte_source_url: Option<String>,
    pub hlzf_source_url: Option<String>,
    pub netzentgelte_file_pattern: Option<String>,
    pub hlzf_file_pattern: Option<String>,
    pub auto_crawl: bool,
    pub auto_crawl_increment: bool,
    pub auto_crawl_years: String, // Stored as JSON string in DB
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct HlzfData {
    pub key: String,
    pub year: i32,
    pub update_timestamp: i64,
    pub value_id: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct NetzentgelteData {
    pub key: String,
    pub year: i32,
    pub update_timestamp: i64,
    pub voltage_level: String,
    pub value_id: String,
    pub value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DataSourceYearly {
    pub key: String,
    pub year: i32,
    pub source_type: Option<String>,
    pub hlzf_url: Option<String>,
    pub netzentgelte_url: Option<String>,
    pub hlzf_file: Option<String>,
    pub netzentgelte_file: Option<String>,
}

// Helper functions for JSON serialization/deserialization
impl Metadata {
    pub fn get_dno_names(&self) -> Vec<String> {
        serde_json::from_str(&self.dno_name).unwrap_or_default()
    }

    pub fn set_dno_names(&mut self, names: Vec<String>) {
        self.dno_name = serde_json::to_string(&names).unwrap_or_default();
    }
}

impl Crawl {
    pub fn get_auto_crawl_years(&self) -> Vec<i32> {
        serde_json::from_str(&self.auto_crawl_years).unwrap_or_default()
    }

    pub fn set_auto_crawl_years(&mut self, years: Vec<i32>) {
        self.auto_crawl_years = serde_json::to_string(&years).unwrap_or_default();
    }
}