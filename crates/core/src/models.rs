use chrono::{DateTime, Utc, NaiveTime, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

// Custom enum types
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Pending,
    User,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "crawl_type", rename_all = "lowercase")]
pub enum CrawlType {
    File,
    Table,
    Api,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "data_type", rename_all = "lowercase")]
pub enum DataType {
    Netzentgelte,
    Hlzf,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "season", rename_all = "lowercase")]
pub enum Season {
    Winter,
    Fruehling,
    Sommer,
    Herbst,
}

// DNO (Distribution Network Operator) model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Dno {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub official_name: Option<String>,
    pub description: Option<String>,
    pub region: Option<String>,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDno {
    pub slug: String,
    pub name: String,
    pub official_name: Option<String>,
    pub description: Option<String>,
    pub region: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDno {
    pub slug: Option<String>,
    pub name: Option<String>,
    pub official_name: Option<String>,
    pub description: Option<String>,
    pub region: Option<String>,
    pub website: Option<String>,
}

// DNO crawl configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DnoCrawlConfig {
    pub id: Uuid,
    pub dno_id: Uuid,
    pub crawl_type: CrawlType,
    pub netzentgelte_source_url: Option<String>,
    pub hlzf_source_url: Option<String>,
    pub netzentgelte_file_pattern: Option<String>,
    pub hlzf_file_pattern: Option<String>,
    pub auto_crawl: bool,
    pub auto_crawl_interval: Option<String>,
    pub auto_crawl_years: Option<Vec<i32>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDnoCrawlConfig {
    pub dno_id: Uuid,
    pub crawl_type: CrawlType,
    pub netzentgelte_source_url: Option<String>,
    pub hlzf_source_url: Option<String>,
    pub netzentgelte_file_pattern: Option<String>,
    pub hlzf_file_pattern: Option<String>,
    pub auto_crawl: bool,
    pub auto_crawl_interval: Option<String>,
    pub auto_crawl_years: Option<Vec<i32>>,
}

// Netzentgelte data model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NetzentgelteData {
    pub id: Uuid,
    pub dno_id: Uuid,
    pub year: i32,
    pub voltage_level: String,
    pub leistung: Option<rust_decimal::Decimal>,
    pub arbeit: Option<rust_decimal::Decimal>,
    pub leistung_unter_2500h: Option<rust_decimal::Decimal>,
    pub arbeit_unter_2500h: Option<rust_decimal::Decimal>,
    pub verification_status: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verification_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNetzentgelteData {
    pub dno_id: Uuid,
    pub year: i32,
    pub voltage_level: String,
    pub leistung: Option<rust_decimal::Decimal>,
    pub arbeit: Option<rust_decimal::Decimal>,
    pub leistung_unter_2500h: Option<rust_decimal::Decimal>,
    pub arbeit_unter_2500h: Option<rust_decimal::Decimal>,
}

// HLZF data model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HlzfData {
    pub id: Uuid,
    pub dno_id: Uuid,
    pub year: i32,
    pub season: Season,
    pub period_number: i32,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub verification_status: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verification_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHlzfData {
    pub dno_id: Uuid,
    pub year: i32,
    pub season: Season,
    pub period_number: i32,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
}

// Data sources model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DataSource {
    pub id: Uuid,
    pub dno_id: Uuid,
    pub year: i32,
    pub data_type: DataType,
    pub source_type: CrawlType,
    pub source_url: Option<String>,
    pub file_path: Option<String>,
    pub file_hash: Option<String>,
    pub extracted_at: DateTime<Utc>,
    pub confidence: Option<rust_decimal::Decimal>,
    pub page_number: Option<i32>,
    pub extraction_method: Option<String>,
    pub extraction_region: Option<serde_json::Value>,
    pub ocr_text: Option<String>,
    pub extraction_log: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataSource {
    pub dno_id: Uuid,
    pub year: i32,
    pub data_type: DataType,
    pub source_type: CrawlType,
    pub source_url: Option<String>,
    pub file_path: Option<String>,
    pub file_hash: Option<String>,
    pub confidence: Option<rust_decimal::Decimal>,
    pub page_number: Option<i32>,
    pub extraction_method: Option<String>,
    pub extraction_region: Option<serde_json::Value>,
    pub ocr_text: Option<String>,
    pub extraction_log: Option<serde_json::Value>,
}

// User model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub role: UserRole,
    pub profile_picture_url: Option<String>,
    pub is_active: bool,
    pub email_verified: bool,
    pub verification_status: Option<String>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub role: Option<UserRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub role: Option<UserRole>,
    pub profile_picture_url: Option<String>,
    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,
    pub verification_status: Option<String>,
    pub approved_by: Option<Uuid>,
}

// User settings model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSettings {
    pub user_id: Uuid,
    pub language: String,
    pub timezone: String,
    pub email_notifications: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserSettings {
    pub user_id: Uuid,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub email_notifications: Option<bool>,
}

// API Keys model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    #[serde(skip_serializing)]
    pub key_hash: String,
    pub masked_key: String,
    pub last_used: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKey {
    pub user_id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub masked_key: String,
    pub expires_at: Option<DateTime<Utc>>,
}

// Query logs model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QueryLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub query: String,
    pub interpretation: Option<String>,
    pub response_time_ms: Option<i32>,
    pub source_ip: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQueryLog {
    pub user_id: Option<Uuid>,
    pub query: String,
    pub interpretation: Option<String>,
    pub response_time_ms: Option<i32>,
    pub source_ip: Option<String>,
}

// Crawl jobs model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CrawlJob {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub dno_id: Uuid,
    pub year: i32,
    pub data_type: DataType,
    pub status: JobStatus,
    pub progress: i32,
    pub current_step: Option<String>,
    pub error_message: Option<String>,
    pub priority: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCrawlJob {
    pub user_id: Option<Uuid>,
    pub dno_id: Uuid,
    pub year: i32,
    pub data_type: DataType,
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCrawlJob {
    pub status: Option<JobStatus>,
    pub progress: Option<i32>,
    pub current_step: Option<String>,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

// Crawl job steps model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CrawlJobStep {
    pub id: Uuid,
    pub job_id: Uuid,
    pub step_name: String,
    pub status: JobStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCrawlJobStep {
    pub job_id: Uuid,
    pub step_name: String,
    pub details: Option<serde_json::Value>,
}

// System logs model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SystemLog {
    pub id: Uuid,
    pub level: String,
    pub service: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
    pub trace_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSystemLog {
    pub level: String,
    pub service: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
    pub trace_id: Option<String>,
}

// Automated jobs model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AutomatedJob {
    pub id: Uuid,
    pub name: String,
    pub job_type: String,
    pub schedule: String,
    pub enabled: bool,
    pub config: serde_json::Value,
    pub last_run: Option<DateTime<Utc>>,
    pub last_status: Option<JobStatus>,
    pub next_run: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAutomatedJob {
    pub name: String,
    pub job_type: String,
    pub schedule: String,
    pub enabled: bool,
    pub config: serde_json::Value,
    pub next_run: Option<DateTime<Utc>>,
}

// Data entry history model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DataEntryHistory {
    pub id: Uuid,
    pub entry_type: String,
    pub entry_id: Uuid,
    pub version: i32,
    pub changed_by: Option<Uuid>,
    pub changed_at: DateTime<Utc>,
    pub changes: String,
    pub data_before: Option<serde_json::Value>,
    pub data_after: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataEntryHistory {
    pub entry_type: String,
    pub entry_id: Uuid,
    pub version: i32,
    pub changed_by: Option<Uuid>,
    pub changes: String,
    pub data_before: Option<serde_json::Value>,
    pub data_after: Option<serde_json::Value>,
}

// Metrics model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Metric {
    pub id: Uuid,
    pub metric_name: String,
    pub metric_type: String,
    pub value: f64,
    pub labels: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMetric {
    pub metric_name: String,
    pub metric_type: String,
    pub value: f64,
    pub labels: Option<serde_json::Value>,
}

// Sessions model for authentication
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(skip_serializing)]
    pub token_hash: String,
    #[serde(skip_serializing)]
    pub refresh_token_hash: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: Option<DateTime<Utc>>,
    pub ip_address: Option<std::net::IpAddr>,
    pub user_agent: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSession {
    pub user_id: Uuid,
    pub token_hash: String,
    pub refresh_token_hash: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: Option<DateTime<Utc>>,
    pub ip_address: Option<std::net::IpAddr>,
    pub user_agent: Option<String>,
}

// Authentication DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserPublic,
    pub tokens: TokenPair,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub profile_picture_url: Option<String>,
    pub is_active: bool,
    pub email_verified: bool,
    pub verification_status: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserPublic {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
            profile_picture_url: user.profile_picture_url,
            is_active: user.is_active,
            email_verified: user.email_verified,
            verification_status: user.verification_status,
            created_at: user.created_at,
        }
    }
}

// DTOs for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnoWithData {
    #[serde(flatten)]
    pub dno: Dno,
    pub netzentgelte_data: Vec<NetzentgelteData>,
    pub hlzf_data: Vec<HlzfData>,
    pub data_sources: Vec<DataSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithSettings {
    #[serde(flatten)]
    pub user: User,
    pub settings: Option<UserSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlJobWithSteps {
    #[serde(flatten)]
    pub job: CrawlJob,
    pub steps: Vec<CrawlJobStep>,
}

// Search result DTOs
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NetzentgelteDataWithDno {
    // Netzentgelte data fields
    pub id: Uuid,
    pub dno_id: Uuid,
    pub year: i32,
    pub voltage_level: String,
    pub leistung: Option<rust_decimal::Decimal>,
    pub arbeit: Option<rust_decimal::Decimal>,
    pub leistung_unter_2500h: Option<rust_decimal::Decimal>,
    pub arbeit_unter_2500h: Option<rust_decimal::Decimal>,
    pub verification_status: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verification_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    // DNO data fields (prefixed)
    pub dno_id_full: Uuid,
    pub dno_slug: String,
    pub dno_name: String,
    pub dno_official_name: Option<String>,
    pub dno_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HlzfDataWithDno {
    // HLZF data fields
    pub id: Uuid,
    pub dno_id: Uuid,
    pub year: i32,
    pub season: Season,
    pub voltage_level: String,
    pub ht: Option<rust_decimal::Decimal>,
    pub nt: Option<rust_decimal::Decimal>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub verification_status: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verification_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    // DNO data fields (prefixed)
    pub dno_id_full: Uuid,
    pub dno_slug: String,
    pub dno_name: String,
    pub dno_official_name: Option<String>,
    pub dno_region: Option<String>,
}

// Dashboard and statistics DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub queries_today: u32,
    pub queries_this_month: u32,
    pub total_dnos: u32,
    pub total_data_entries: u32,
    pub available_years: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnoInfo {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableFilters {
    pub years: Vec<i32>,
    pub dnos: Vec<DnoInfo>,
    pub regions: Vec<String>,
    pub data_types: Vec<String>,
}


// API request/response DTOs for search endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchByDnoRequest {
    pub dno_name: Option<String>,
    pub dno_id: Option<Uuid>,
    pub year: Option<i32>,
    pub data_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchByYearRequest {
    pub year: i32,
    pub dno_name: Option<String>,
    pub dno_id: Option<Uuid>,
    pub data_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchByDataTypeRequest {
    pub data_type: String,
    pub dno_name: Option<String>,
    pub dno_id: Option<Uuid>,
    pub year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub dno_name: Option<String>,
    pub dno_id: Option<Uuid>,
    pub year: Option<i32>,
    pub data_type: Option<String>,
    pub region: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub dno: DnoInfo,
    pub year: i32,
    pub data_type: String,
    pub status: String,
    pub data: serde_json::Value,
    pub source: Option<SourceInfo>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub id: Uuid,
    pub file_type: String,
    pub file_url: Option<String>,
    pub page: Option<i32>,
    pub extracted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub total: u32,
    pub results: Vec<SearchResult>,
    pub filters_applied: serde_json::Value,
    pub available_years: Vec<i32>,
    pub available_dnos: Vec<DnoInfo>,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
    pub has_more: bool,
}


// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessResponse {
    pub status: String,
    pub services: ServiceStatus,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub database: String,
    pub cache: Option<String>,
    pub storage: Option<String>,
}