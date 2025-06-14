use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub external: ExternalConfig,
    pub crawler: CrawlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry: u64,
    pub refresh_token_expiry: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalConfig {
    pub searxng: SearxngConfig,
    pub ollama: OllamaConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearxngConfig {
    pub url: String,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub url: String,
    pub model: String,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlerConfig {
    pub max_concurrent: usize,
    pub delay_between_requests: u64,
    pub user_agent: String,
    pub timeout: u64,
    pub max_retries: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, crate::AppError> {
        Ok(Self {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .map_err(|e| crate::AppError::Config(format!("Invalid SERVER_PORT: {}", e)))?,
                cors_origins: env::var("CORS_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .map_err(|_| crate::AppError::Config("DATABASE_URL is required".to_string()))?,
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                min_connections: env::var("DB_MIN_CONNECTIONS")
                    .unwrap_or_else(|_| "1".to_string())
                    .parse()
                    .unwrap_or(1),
                connect_timeout: env::var("DB_CONNECT_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
                idle_timeout: env::var("DB_IDLE_TIMEOUT")
                    .unwrap_or_else(|_| "600".to_string())
                    .parse()
                    .unwrap_or(600),
            },
            auth: AuthConfig {
                jwt_secret: env::var("JWT_SECRET")
                    .map_err(|_| crate::AppError::Config("JWT_SECRET is required".to_string()))?,
                jwt_expiry: env::var("JWT_EXPIRY")
                    .unwrap_or_else(|_| "900".to_string()) // 15 minutes
                    .parse()
                    .unwrap_or(900),
                refresh_token_expiry: env::var("REFRESH_TOKEN_EXPIRY")
                    .unwrap_or_else(|_| "604800".to_string()) // 7 days
                    .parse()
                    .unwrap_or(604800),
            },
            external: ExternalConfig {
                searxng: SearxngConfig {
                    url: env::var("SEARXNG_URL")
                        .unwrap_or_else(|_| "http://localhost:8888".to_string()),
                    timeout: env::var("SEARXNG_TIMEOUT")
                        .unwrap_or_else(|_| "30".to_string())
                        .parse()
                        .unwrap_or(30),
                },
                ollama: OllamaConfig {
                    url: env::var("OLLAMA_URL")
                        .unwrap_or_else(|_| "http://localhost:11434".to_string()),
                    model: env::var("OLLAMA_MODEL")
                        .unwrap_or_else(|_| "llama3".to_string()),
                    timeout: env::var("OLLAMA_TIMEOUT")
                        .unwrap_or_else(|_| "60".to_string())
                        .parse()
                        .unwrap_or(60),
                },
            },
            crawler: CrawlerConfig {
                max_concurrent: env::var("CRAWLER_MAX_CONCURRENT")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                delay_between_requests: env::var("CRAWLER_DELAY")
                    .unwrap_or_else(|_| "1000".to_string())
                    .parse()
                    .unwrap_or(1000),
                user_agent: env::var("CRAWLER_USER_AGENT")
                    .unwrap_or_else(|_| "DNO-Data-Gatherer/0.0.1".to_string()),
                timeout: env::var("CRAWLER_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
                max_retries: env::var("CRAWLER_MAX_RETRIES")
                    .unwrap_or_else(|_| "3".to_string())
                    .parse()
                    .unwrap_or(3),
            },
        })
    }
}