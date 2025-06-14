use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dno {
    pub id: Uuid,
    pub name: String,
    pub region: String,
    pub website: Option<String>,
    pub contact_info: Option<ContactInfo>,
    pub data_sources: Vec<DataSource>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub id: Uuid,
    pub url: String,
    pub source_type: DataSourceType,
    pub last_crawled: Option<DateTime<Utc>>,
    pub status: CrawlStatus,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSourceType {
    Website,
    Api,
    Document,
    Feed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrawlStatus {
    Pending,
    InProgress,
    Completed,
    Failed { error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub filters: SearchFilters,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub region: Option<String>,
    pub source_type: Option<DataSourceType>,
    pub date_range: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub source: String,
    pub relevance_score: f64,
    pub found_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlJob {
    pub id: Uuid,
    pub url: String,
    pub status: CrawlStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<CrawlResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlResult {
    pub content: String,
    pub extracted_data: serde_json::Value,
    pub links: Vec<String>,
    pub metadata: PageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub language: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
}

// DNO-specific data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DnoDataType {
    Netzentgelte,
    Hlzf,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnoQueryParseResult {
    pub dno_name: String,
    pub dno_key: String, // Normalized key like "netze-bw"
    pub years: Vec<i32>,
    pub data_types: Vec<DnoDataType>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnoMetadata {
    pub key: String,
    pub dno_names: Vec<String>, // Multiple name variations
    pub description: Option<String>,
    pub region: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HlzfData {
    pub key: String,
    pub year: i32,
    pub update_timestamp: i64,
    pub value_id: String, // "Winter_1_Start", "Winter_1_Ende", etc.
    pub value: Option<String>, // Time strings like "06:00:00"
    pub source_file: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetzentgelteData {
    pub key: String,
    pub year: i32,
    pub update_timestamp: i64,
    pub voltage_level: VoltageLevel,
    pub value_id: String, // "Leistung", "Arbeit", etc.
    pub value: Option<f64>,
    pub unit: Option<String>, // "€/kW", "ct/kWh", etc.
    pub source_file: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoltageLevel {
    #[serde(rename = "hs")]
    HighVoltage,
    #[serde(rename = "ms")]
    MediumVoltage,
    #[serde(rename = "ns")]
    LowVoltage,
    #[serde(rename = "ms_ns")]
    MediumLowVoltage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceYearly {
    pub key: String,
    pub year: i32,
    pub source_type: Option<String>,
    pub hlzf_url: Option<String>,
    pub netzentgelte_url: Option<String>,
    pub hlzf_file: Option<String>,
    pub netzentgelte_file: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlConfig {
    pub key: String,
    pub crawl_type: String,
    pub netzentgelte_source_url: Option<String>,
    pub hlzf_source_url: Option<String>,
    pub netzentgelte_file_pattern: Option<String>,
    pub hlzf_file_pattern: Option<String>,
    pub auto_crawl: bool,
    pub auto_crawl_increment: bool,
    pub auto_crawl_years: Vec<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLearning {
    pub id: Uuid,
    pub original_query: String,
    pub extracted_dno: Option<String>,
    pub extracted_years: Vec<i32>,
    pub search_terms: Vec<String>,
    pub success_rate: f64,
    pub execution_time_ms: Option<i32>,
    pub result_confidence: Option<f64>,
    pub source_files: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfAnalysisResult {
    pub id: Uuid,
    pub file_path: String,
    pub file_hash: String,
    pub analysis_type: PdfAnalysisType,
    pub model_used: String,
    pub extracted_data: serde_json::Value,
    pub confidence_score: Option<f64>,
    pub processing_time_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PdfAnalysisType {
    #[serde(rename = "netzentgelte")]
    Netzentgelte,
    #[serde(rename = "hlzf")]
    Hlzf,
    #[serde(rename = "general")]
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryExecutionHistory {
    pub id: Uuid,
    pub query_learning_id: Uuid,
    pub execution_path: serde_json::Value, // JSON describing execution path
    pub cache_hits: i32,
    pub database_queries: i32,
    pub pdf_analyses: i32,
    pub web_searches: i32,
    pub total_time_ms: i32,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

// DNO query request and response structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnoQueryRequest {
    pub query: String, // Natural language query like "Netze BW 2024 & 2025"
    pub max_years: Option<u32>, // Limit number of years to return
    pub include_sources: Option<bool>, // Whether to include source file information
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnoQueryResponse {
    pub metadata: DnoQueryMetadata,
    pub hlzf_data: HashMap<String, HashMap<String, Option<String>>>, // year -> value_id -> value
    pub netzentgelte_data: HashMap<String, HashMap<String, HashMap<String, Option<f64>>>>, // year -> voltage_level -> value_id -> value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnoQueryMetadata {
    pub dno_name: String,
    pub original_query: String,
    pub extracted_years: Vec<i32>,
    pub sources: Vec<String>,
    pub confidence: f64,
    pub processing_time_ms: i32,
    pub cache_hits: i32,
    pub query_timestamp: DateTime<Utc>,
}

// PDF analysis request structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfAnalysisRequest {
    pub file_path: String,
    pub analysis_type: PdfAnalysisType,
    pub force_reanalyze: Option<bool>, // Skip cache and force re-analysis
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfAnalysisResponse {
    pub id: Uuid,
    pub extracted_data: serde_json::Value,
    pub confidence_score: Option<f64>,
    pub processing_time_ms: i32,
    pub was_cached: bool,
    pub model_used: String,
}