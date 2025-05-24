mod models;

use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool, Error as SqlxError};
use std::path::Path;
use anyhow::{Context, Result};

const DB_FILE: &str = "assets/data.db";
const HLZF_VOLTAGE_LEVELS: [&str; 4] = ["hs/ms", "ms", "ms/ns", "ns"];

use sqlx::{Executor, Pool, Sqlite};

use tokio::sync::OnceCell;

use super::model::UserSql;

static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

async fn db() -> Pool<Sqlite> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite://db.sqlite")
        .await
        .unwrap();

    pool.execute(
        "
    CREATE TABLE IF NOT EXISTS users (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      username TEXT,
      password TEXT
    )
  ",
    )
        .await
        .unwrap();

    let rows: Vec<UserSql> = sqlx::query_as("SELECT * FROM users WHERE id = ?1")
        .bind(&1)
        .fetch_all(&pool)
        .await
        .unwrap();

    if rows.len() == 0 {
        sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
            .bind(&"guest")
            .bind(&"guest")
            .execute(&pool)
            .await
            .unwrap();
    }

    pool
}

pub async fn get_db() -> &'static Pool<Sqlite> {
    DB.get_or_init(db).await
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub key: String,
    pub dno_name: Vec<String>,
    pub description: String,
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Crawl {
    pub key: String,
    pub crawl_type: String, 
    pub netzentgelte_source_url: Option<String>, 
    pub hlzf_source_url: Option<String>, 
    pub netzentgelte_file_pattern: Option<String>,
    pub hlzf_file_pattern: Option<String>,
    pub auto_crawl: bool,
    pub auto_crawl_increment: bool,
    pub auto_crawl_years: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HlzfData {
    pub key: String,
    pub year: i32,
    pub update_timestamp: i64,
    pub value_id: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetzentgelteData {
    pub key: String,
    pub year: i32,
    pub update_timestamp: i64,
    pub voltage_level: String,
    pub value_id: String,
    pub value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataSourceYearly {
    pub key: String,
    pub year: i32,
    pub source_type: String,
    pub hlzf_url: Option<String>,
    pub netzentgelte_url: Option<String>,
    pub hlzf_file: Option<String>,
    pub netzentgelte_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AccountType {
    Guest,
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub user_id: Option<i64>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub verified: bool,
    pub blocked: bool,
    pub account_type: AccountType,
}

// Define a type alias for the sqlx pool
pub type DbPool = SqlitePool;

async fn create_tables_if_not_exist(pool: &DbPool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS metadata (
            key TEXT PRIMARY KEY,
            dno_name TEXT NOT NULL, -- JSON string for Vec<String>
            description TEXT,
            region TEXT
        );

        CREATE TABLE IF NOT EXISTS crawl (
            key TEXT PRIMARY KEY,
            crawl_type TEXT NOT NULL,
            netzentgelte_source_url TEXT,
            hlzf_source_url TEXT,
            netzentgelte_file_pattern TEXT,
            hlzf_file_pattern TEXT,
            auto_crawl INTEGER NOT NULL DEFAULT 0, -- 0 for false, 1 for true
            auto_crawl_increment INTEGER NOT NULL DEFAULT 0, -- 0 for false, 1 for true
            auto_crawl_years TEXT NOT NULL -- JSON string for Vec<i32>
        );

        CREATE TABLE IF NOT EXISTS hlzf_data (
            key TEXT NOT NULL,
            year INTEGER NOT NULL,
            update_timestamp INTEGER NOT NULL,
            value_id TEXT NOT NULL,
            value TEXT, -- Storing time strings or NULL
            PRIMARY KEY (key, year, value_id)
        );

        CREATE TABLE IF NOT EXISTS netzentgelte_data (
            key TEXT NOT NULL,
            year INTEGER NOT NULL,
            update_timestamp INTEGER NOT NULL,
            voltage_level TEXT NOT NULL,
            value_id TEXT NOT NULL,
            value REAL, -- Storing numeric values or NULL
            PRIMARY KEY (key, year, voltage_level, value_id)
        );

        CREATE TABLE IF NOT EXISTS data_source_yearly (
            key TEXT NOT NULL,
            year INTEGER NOT NULL,
            source_type TEXT,
            hlzf_url TEXT,
            netzentgelte_url TEXT,
            hlzf_file TEXT,
            netzentgelte_file TEXT,
            PRIMARY KEY (key, year)
        );

        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            verified INTEGER NOT NULL DEFAULT 0,
            blocked INTEGER NOT NULL DEFAULT 0,
            account_type TEXT NOT NULL CHECK(account_type IN ('Guest', 'User', 'Admin'))
        );
        "#,
    )
    .execute(pool)
    .await
    .context("Failed to create database tables")?;
    Ok(())
}

pub async fn create_pool() -> Result<DbPool> {
    let db_url = format!("sqlite:{}", DB_FILE);

    // Ensure the directory exists (blocking operation, run before async pool creation)
    let db_path_str = DB_FILE.to_string();
    tokio::fs::create_dir_all(Path::new(&db_path_str).parent().unwrap_or_else(|| Path::new(".")))
        .await
        .with_context(|| format!("Failed to create database directory for: {}", db_path_str))?;
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5) // Configure as needed
        .connect(&db_url)
        .await
        .with_context(|| format!("Failed to connect to SQLite database at {}", db_url))?;

    create_tables_if_not_exist(&pool).await?;

    Ok(pool)
}