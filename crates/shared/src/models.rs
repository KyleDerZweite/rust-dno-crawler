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

// Enhanced crawler intelligence system models

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlIntelligence {
    pub id: String,
    pub dno_key: String,
    pub pattern_type: PatternType,
    pub pattern_signature: String,
    pub confidence_score: f64,
    pub success_count: i32,
    pub failure_count: i32,
    pub avg_success_time_ms: Option<i32>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_failure_at: Option<DateTime<Utc>>,
    pub pattern_metadata: serde_json::Value,
    pub admin_verification_status: AdminVerificationStatus,
    pub admin_flagged: bool,
    pub admin_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "navigation")]
    Navigation,
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "file_naming")]
    FileNaming,
    #[serde(rename = "structural")]
    Structural,
}

impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            PatternType::Url => "URL Pattern",
            PatternType::Navigation => "Navigation Pattern",
            PatternType::Content => "Content Pattern",
            PatternType::FileNaming => "File Naming Pattern",
            PatternType::Structural => "Structural Pattern",
        };
        write!(f, "{}", display_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdminVerificationStatus {
    #[serde(rename = "not_reviewed")]
    NotReviewed,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveCrawlSession {
    pub session_id: String,
    pub dno_name: String,
    pub dno_key: String,
    pub year: i32,
    pub status: CrawlSessionStatus,
    pub priority: i32,
    pub progress_percentage: f64,
    pub current_phase: Option<String>,
    pub current_url: Option<String>,
    pub pages_visited: i32,
    pub files_downloaded: i32,
    pub data_extracted: i32,
    pub errors_encountered: i32,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub worker_thread_id: Option<String>,
    pub parent_session_id: Option<String>,
    pub created_by_user: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrawlSessionStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "searching")]
    Searching,
    #[serde(rename = "crawling")]
    Crawling,
    #[serde(rename = "extracting")]
    Extracting,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "paused")]
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveLog {
    pub id: i64,
    pub session_id: String,
    pub timestamp: DateTime<Utc>,
    pub log_level: LogLevel,
    pub message: String,
    pub context: Option<serde_json::Value>,
    pub phase: Option<String>,
    pub url: Option<String>,
    pub worker_id: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlPath {
    pub id: String,
    pub session_id: String,
    pub dno_key: String,
    pub year: i32,
    pub path_sequence: Vec<NavigationStep>,
    pub success_endpoints: Vec<String>,
    pub files_discovered: Vec<String>,
    pub extraction_methods: Vec<String>,
    pub total_time_ms: i32,
    pub max_depth_reached: i32,
    pub success_confidence: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationStep {
    pub step_type: String,
    pub url: String,
    pub action: Option<String>,
    pub selector: Option<String>,
    pub input_value: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceV2 {
    pub id: String,
    pub session_id: String,
    pub source_type: SourceType,
    pub original_url: String,
    pub final_url: String,
    pub local_file_path: Option<String>,
    pub file_hash: Option<String>,
    pub content_type: Option<String>,
    pub file_size: Option<i64>,
    pub extraction_confidence: Option<f64>,
    pub extraction_method: Option<ExtractionMethod>,
    pub data_extracted: Option<serde_json::Value>,
    pub crawl_path_id: Option<String>,
    pub discovered_at: DateTime<Utc>,
    pub last_verified_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub admin_flagged: bool,
    pub admin_verification_status: AdminDataVerificationStatus,
    pub admin_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    #[serde(rename = "webpage")]
    Webpage,
    #[serde(rename = "pdf")]
    Pdf,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtractionMethod {
    #[serde(rename = "pdf_analysis")]
    PdfAnalysis,
    #[serde(rename = "ocr")]
    Ocr,
    #[serde(rename = "table_extraction")]
    TableExtraction,
    #[serde(rename = "text_parsing")]
    TextParsing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdminDataVerificationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnoKnowledgeGraph {
    pub id: String,
    pub dno_key: String,
    pub relationship_type: RelationshipType,
    pub related_dno_key: String,
    pub confidence: f64,
    pub evidence: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    #[serde(rename = "parent_company")]
    ParentCompany,
    #[serde(rename = "subsidiary")]
    Subsidiary,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "similar_structure")]
    SimilarStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternPerformance {
    pub id: String,
    pub pattern_id: String,
    pub session_id: String,
    pub execution_time_ms: i32,
    pub success: bool,
    pub data_quality_score: Option<f64>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlJobQueue {
    pub id: String,
    pub dno_name: String,
    pub dno_key: String,
    pub year: i32,
    pub priority: i32,
    pub job_type: JobType,
    pub strategy_preference: Option<serde_json::Value>,
    pub constraints: Option<serde_json::Value>,
    pub retry_count: i32,
    pub max_retries: i32,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub created_by_user: Option<String>,
    pub status: JobStatus,
    pub assigned_worker: Option<String>,
    pub session_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobType {
    #[serde(rename = "user_request")]
    UserRequest,
    #[serde(rename = "automated_discovery")]
    AutomatedDiscovery,
    #[serde(rename = "historical_backfill")]
    HistoricalBackfill,
    #[serde(rename = "verification")]
    Verification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityFlag {
    pub id: String,
    pub flagged_table: String,
    pub flagged_record_id: String,
    pub flag_type: FlagType,
    pub severity: FlagSeverity,
    pub admin_user_id: String,
    pub reason: String,
    pub impact_analysis: Option<serde_json::Value>,
    pub resolution_status: FlagResolutionStatus,
    pub resolution_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FlagType {
    #[serde(rename = "incorrect_data")]
    IncorrectData,
    #[serde(rename = "suspicious_pattern")]
    SuspiciousPattern,
    #[serde(rename = "verification_needed")]
    VerificationNeeded,
    #[serde(rename = "false_positive")]
    FalsePositive,
    #[serde(rename = "low_confidence")]
    LowConfidence,
    #[serde(rename = "data_quality_issue")]
    DataQualityIssue,
    #[serde(rename = "manual_review")]
    ManualReview,
    #[serde(rename = "security_concern")]
    SecurityConcern,
    #[serde(rename = "performance_issue")]
    PerformanceIssue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlagSeverity {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlagResolutionStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "investigating")]
    Investigating,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "false_alarm")]
    FalseAlarm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteStructure {
    pub id: String,
    pub dno_key: String,
    pub domain: String,
    pub structure_type: StructureType,
    pub structure_data: serde_json::Value,
    pub confidence: f64,
    pub last_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureType {
    #[serde(rename = "navigation_menu")]
    NavigationMenu,
    #[serde(rename = "archive_pattern")]
    ArchivePattern,
    #[serde(rename = "file_organization")]
    FileOrganization,
    #[serde(rename = "url_scheme")]
    UrlScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryTracking {
    pub id: String,
    pub discovery_type: DiscoveryType,
    pub discovered_entity: String,
    pub discovery_source: String,
    pub confidence: f64,
    pub verification_status: DiscoveryVerificationStatus,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryType {
    #[serde(rename = "new_dno")]
    NewDno,
    #[serde(rename = "new_data_source")]
    NewDataSource,
    #[serde(rename = "archive_discovery")]
    ArchiveDiscovery,
    #[serde(rename = "pattern_evolution")]
    PatternEvolution,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscoveryVerificationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "rejected")]
    Rejected,
}

// Request/response models for the enhanced crawler system

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlSessionRequest {
    pub dno_name: String,
    pub dno_key: String,
    pub year: i32,
    pub priority: Option<i32>,
    pub created_by_user: Option<String>,
    pub strategy_preference: Option<serde_json::Value>,
    pub constraints: Option<serde_json::Value>,
    pub scheduled_for: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlSessionResponse {
    pub session_id: String,
    pub status: CrawlSessionStatus,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub progress_percentage: f64,
    pub current_phase: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSessionUpdate {
    pub session_id: String,
    pub status: CrawlSessionStatus,
    pub progress_percentage: f64,
    pub current_phase: Option<String>,
    pub current_url: Option<String>,
    pub pages_visited: i32,
    pub files_downloaded: i32,
    pub data_extracted: i32,
    pub errors_encountered: i32,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternLearningRequest {
    pub dno_key: String,
    pub pattern_type: PatternType,
    pub pattern_data: serde_json::Value,
    pub success: bool,
    pub execution_time_ms: i32,
    pub data_quality_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternLearningResponse {
    pub pattern_id: String,
    pub confidence_score: f64,
    pub recommendation: PatternRecommendation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternRecommendation {
    #[serde(rename = "use")]
    Use,
    #[serde(rename = "avoid")]
    Avoid,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "verify")]
    Verify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlJobRequest {
    pub dno_name: String,
    pub dno_key: String,
    pub year: i32,
    pub priority: Option<i32>,
    pub job_type: JobType,
    pub strategy_preference: Option<serde_json::Value>,
    pub constraints: Option<serde_json::Value>,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub created_by_user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlJobResponse {
    pub job_id: String,
    pub status: JobStatus,
    pub session_id: Option<String>,
    pub estimated_start_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminFlagRequest {
    pub flagged_table: String,
    pub flagged_record_id: String,
    pub flag_type: FlagType,
    pub severity: FlagSeverity,
    pub reason: String,
    pub impact_analysis: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminFlagResponse {
    pub flag_id: String,
    pub status: FlagResolutionStatus,
    pub created_at: DateTime<Utc>,
}

// Additional types needed by handlers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl From<Priority> for i32 {
    fn from(priority: Priority) -> Self {
        match priority {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Critical => 4,
        }
    }
}

impl From<i32> for Priority {
    fn from(value: i32) -> Self {
        match value {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            4 => Priority::Critical,
            _ => Priority::Medium,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlConstraints {
    pub max_pages: Option<u32>,
    pub max_depth: Option<u32>,
    pub timeout_ms: Option<u32>,
    pub rate_limit_ms: Option<u32>,
    pub allowed_domains: Option<Vec<String>>,
    pub blocked_domains: Option<Vec<String>>,
}

// Use FlagSeverity as the main type, and create Severity as an alias
pub type Severity = FlagSeverity;

// Additional types needed by handlers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsightsParams {
    pub dno_key: Option<String>,
    pub pattern_type: Option<PatternType>,
    pub time_range: Option<DateRange>,
    pub min_confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTestRequest {
    pub pattern_id: String,
    pub test_dno_key: String,
    pub test_year: i32,
    pub simulation_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseCrawlRequest {
    pub target_url: String,
    pub dno_key: String,
    pub max_depth: Option<u32>,
    pub extraction_hints: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAnalysisRequest {
    pub source_url: String,
    pub analysis_type: String,
    pub deep_analysis: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryQueryParams {
    pub discovery_type: Option<DiscoveryType>,
    pub status: Option<DiscoveryVerificationStatus>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceQueryParams {
    pub source_type: Option<SourceType>,
    pub status: Option<AdminDataVerificationStatus>,
    pub dno_key: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMetricsParams {
    pub time_range: Option<DateRange>,
    pub group_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceUpdateRequest {
    pub source_type: Option<SourceType>,
    pub extraction_method: Option<ExtractionMethod>,
    pub admin_notes: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkSourceRequest {
    pub operation: String,
    pub source_ids: Vec<String>,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInventoryParams {
    pub format: Option<String>,
    pub include_content: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlHistoryParams {
    pub dno_key: Option<String>,
    pub time_range: Option<DateRange>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceComparisonRequest {
    pub source_urls: Vec<String>,
    pub comparison_metrics: Vec<String>,
}

// Admin and Authentication Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAction {
    pub id: Uuid,
    pub admin_user_id: String,
    pub admin_username: String,
    pub action_type: String,
    pub entity_type: String,
    pub entity_id: String,
    pub details: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminFlag {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub flag_type: FlagType,
    pub flagged_by: String,
    pub flagged_by_username: String,
    pub reason: String,
    pub status: String, // "pending", "approved", "rejected"
    pub preserve_learning: bool,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub reviewed_by: Option<String>,
    pub review_notes: Option<String>,
}


// Job Management Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatedJobConfig {
    pub job_id: Uuid,
    pub job_type: String,
    pub schedule: Option<String>, // Cron expression
    pub enabled: bool,
    pub config: serde_json::Value,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecution {
    pub execution_id: Uuid,
    pub job_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: String, // "running", "completed", "failed", "cancelled"
    pub results: Option<serde_json::Value>,
    pub error_details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobControl {
    pub job_id: Uuid,
    pub action: String, // "start", "stop", "pause", "resume"
    pub requested_by: String,
    pub requested_at: DateTime<Utc>,
    pub reason: Option<String>,
}