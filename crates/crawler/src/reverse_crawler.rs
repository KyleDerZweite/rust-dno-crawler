use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc, Datelike};
use regex::Regex;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;
use tokio::time::sleep;
use anyhow::{Result, anyhow};
use tracing::{info, warn, debug, trace};

use crate::source_manager::{SourceManager, FileMetadata};
use crate::learning_engine::{LearningEngine, LearnedPattern};
use shared::SourceType;

/// Comprehensive reverse crawling engine for discovering historical data and archive patterns
#[derive(Debug)]
pub struct ReverseCrawler {
    /// Source manager for tracking discovered content
    source_manager: SourceManager,
    /// Learning engine for pattern recognition
    learning_engine: LearningEngine,
    /// HTTP client for crawling
    client: reqwest::Client,
    /// Current session ID
    session_id: String,
    /// Configuration settings
    config: ReverseCrawlerConfig,
    /// Pattern cache for performance
    pattern_cache: HashMap<String, Vec<UrlPattern>>,
    /// Temporal pattern recognition engine
    temporal_engine: TemporalPatternEngine,
    /// Archive structure discovery
    archive_discovery: ArchiveDiscovery,
    /// URL reconstruction engine
    url_reconstructor: UrlReconstructor,
}

/// Configuration for the reverse crawler
#[derive(Debug, Clone)]
pub struct ReverseCrawlerConfig {
    /// Maximum depth to crawl backwards
    pub max_reverse_depth: u32,
    /// Maximum time to spend on reverse crawling (seconds)
    pub max_crawl_time_seconds: u64,
    /// Maximum number of URLs to test per pattern
    pub max_urls_per_pattern: usize,
    /// Delay between requests (milliseconds)
    pub request_delay_ms: u64,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Years to look back for historical data
    pub historical_years_back: u32,
    /// Confidence threshold for accepting patterns
    pub pattern_confidence_threshold: f64,
    /// Enable aggressive archive discovery
    pub aggressive_archive_discovery: bool,
}

/// Temporal pattern for recognizing time-based URL structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPattern {
    /// Pattern ID
    pub id: String,
    /// Pattern type (year, month, date, etc.)
    pub pattern_type: TemporalPatternType,
    /// Regular expression for matching
    pub regex: String,
    /// Format string for reconstruction
    pub format_template: String,
    /// Confidence score
    pub confidence: f64,
    /// Success count
    pub success_count: u32,
    /// Total attempts
    pub total_attempts: u32,
    /// Examples of successful matches
    pub examples: Vec<String>,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
}

/// Types of temporal patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalPatternType {
    /// Year-based patterns (2024, 24, etc.)
    Year,
    /// Month-based patterns (01, Jan, January, etc.)
    Month,
    /// Date-based patterns (2024-01-01, etc.)
    Date,
    /// Quarter-based patterns (Q1, quarter1, etc.)
    Quarter,
    /// Archive-specific patterns
    Archive,
    /// Version or revision patterns
    Version,
}

/// URL pattern for reconstruction and prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlPattern {
    /// Pattern ID
    pub id: String,
    /// Base URL template
    pub template: String,
    /// Variable parts and their types
    pub variables: HashMap<String, VariableType>,
    /// Pattern confidence
    pub confidence: f64,
    /// Success rate
    pub success_rate: f64,
    /// Associated DNO key
    pub dno_key: String,
    /// Last successful use
    pub last_success: Option<DateTime<Utc>>,
    /// Examples that match this pattern
    pub examples: Vec<String>,
}

/// Types of variables in URL patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    Year,
    Month,
    Day,
    Quarter,
    Version,
    DocumentType,
    FileExtension,
}

/// Archive discovery and analysis engine
#[derive(Debug)]
pub struct ArchiveDiscovery {
    /// Known archive patterns
    archive_patterns: Vec<ArchivePattern>,
    /// Directory structure templates
    directory_templates: Vec<DirectoryTemplate>,
    /// File naming conventions
    file_conventions: Vec<FileNamingConvention>,
}

/// Archive pattern for systematic discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivePattern {
    /// Pattern ID
    pub id: String,
    /// Pattern name
    pub name: String,
    /// URL structure template
    pub url_template: String,
    /// Expected directory structure
    pub directory_structure: Vec<String>,
    /// File naming patterns
    pub file_patterns: Vec<String>,
    /// Confidence score
    pub confidence: f64,
    /// Associated DNO keys
    pub dno_keys: Vec<String>,
}

/// Directory structure template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryTemplate {
    /// Template ID
    pub id: String,
    /// Directory structure pattern
    pub structure: String,
    /// Variable mappings
    pub variables: HashMap<String, String>,
    /// Success rate
    pub success_rate: f64,
}

/// File naming convention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNamingConvention {
    /// Convention ID
    pub id: String,
    /// File name pattern
    pub pattern: String,
    /// Expected file types
    pub file_types: Vec<String>,
    /// Associated data types
    pub data_types: Vec<String>,
    /// Confidence score
    pub confidence: f64,
}

/// Temporal pattern recognition engine
#[derive(Debug)]
pub struct TemporalPatternEngine {
    /// Known temporal patterns
    patterns: Vec<TemporalPattern>,
    /// Pattern matchers (compiled regexes)
    matchers: HashMap<String, Regex>,
}

/// URL reconstruction engine
#[derive(Debug)]
pub struct UrlReconstructor {
    /// Base URL patterns
    base_patterns: Vec<UrlPattern>,
    /// Reconstruction strategies
    strategies: Vec<ReconstructionStrategy>,
}

/// Strategy for URL reconstruction
#[derive(Debug, Clone)]
pub struct ReconstructionStrategy {
    /// Strategy name
    pub name: String,
    /// Priority (higher = more preferred)
    pub priority: u32,
    /// Function to apply the strategy
    pub apply: fn(&str, &HashMap<String, String>) -> Vec<String>,
}

/// Result of reverse crawling operation
#[derive(Debug, Clone)]
pub struct ReverseCrawlResult {
    /// Session ID
    pub session_id: String,
    /// Starting URLs that were analyzed
    pub analyzed_urls: Vec<String>,
    /// Successfully discovered URLs
    pub discovered_urls: Vec<DiscoveredUrl>,
    /// Learned patterns
    pub learned_patterns: Vec<UrlPattern>,
    /// Temporal patterns discovered
    pub temporal_patterns: Vec<TemporalPattern>,
    /// Archive structures identified
    pub archive_structures: Vec<ArchiveStructure>,
    /// Total crawl time
    pub crawl_duration: Duration,
    /// Number of HTTP requests made
    pub requests_made: u32,
    /// Success rate
    pub success_rate: f64,
    /// Files downloaded and stored
    pub files_stored: Vec<String>,
    /// Confidence in results
    pub overall_confidence: f64,
}

/// Discovered URL with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredUrl {
    /// URL
    pub url: String,
    /// Discovery method
    pub discovery_method: DiscoveryMethod,
    /// Confidence score
    pub confidence: f64,
    /// Response status
    pub status_code: Option<u16>,
    /// Content type
    pub content_type: Option<String>,
    /// File size
    pub content_length: Option<u64>,
    /// Last modified date
    pub last_modified: Option<DateTime<Utc>>,
    /// Associated temporal data
    pub temporal_data: Option<TemporalData>,
    /// Discovery timestamp
    pub discovered_at: DateTime<Utc>,
}

/// Method used to discover a URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Pattern-based reconstruction
    PatternReconstruction,
    /// Temporal analysis
    TemporalAnalysis,
    /// Archive traversal
    ArchiveTraversal,
    /// Link following
    LinkFollowing,
    /// Directory listing
    DirectoryListing,
    /// Sitemap parsing
    SitemapParsing,
}

/// Temporal data associated with a URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalData {
    /// Year
    pub year: Option<i32>,
    /// Month
    pub month: Option<u32>,
    /// Day
    pub day: Option<u32>,
    /// Quarter
    pub quarter: Option<u32>,
    /// Version
    pub version: Option<String>,
}

/// Archive structure discovered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveStructure {
    /// Structure ID
    pub id: String,
    /// Base URL
    pub base_url: String,
    /// Directory structure
    pub directory_structure: Vec<String>,
    /// File patterns
    pub file_patterns: Vec<String>,
    /// Temporal organization
    pub temporal_organization: TemporalOrganization,
    /// Confidence score
    pub confidence: f64,
    /// Associated DNO
    pub dno_key: String,
}

/// How archives are organized temporally
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalOrganization {
    /// Organized by year
    ByYear,
    /// Organized by year/month
    ByYearMonth,
    /// Organized by date
    ByDate,
    /// Organized by quarter
    ByQuarter,
    /// Organized by version
    ByVersion,
    /// No clear temporal organization
    None,
}

impl Default for ReverseCrawlerConfig {
    fn default() -> Self {
        Self {
            max_reverse_depth: 5,
            max_crawl_time_seconds: 300, // 5 minutes
            max_urls_per_pattern: 100,
            request_delay_ms: 1000, // 1 second between requests
            max_concurrent_requests: 5,
            historical_years_back: 10,
            pattern_confidence_threshold: 0.7,
            aggressive_archive_discovery: false,
        }
    }
}

impl ReverseCrawler {
    /// Create a new reverse crawler instance
    pub fn new(
        source_manager: SourceManager,
        learning_engine: LearningEngine,
        config: Option<ReverseCrawlerConfig>,
    ) -> Self {
        let config = config.unwrap_or_default();
        let session_id = Uuid::new_v4().to_string();
        
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .user_agent("DNO-Reverse-Crawler/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            source_manager,
            learning_engine,
            client,
            session_id,
            config,
            pattern_cache: HashMap::new(),
            temporal_engine: TemporalPatternEngine::new(),
            archive_discovery: ArchiveDiscovery::new(),
            url_reconstructor: UrlReconstructor::new(),
        }
    }

    /// Start reverse crawling from successful endpoints
    pub async fn reverse_crawl_from_successful_endpoints(
        &mut self,
        dno_key: &str,
        successful_urls: Vec<String>,
    ) -> Result<ReverseCrawlResult> {
        let start_time = Instant::now();
        info!("Starting reverse crawl for DNO: {} with {} successful URLs", dno_key, successful_urls.len());

        let mut result = ReverseCrawlResult {
            session_id: self.session_id.clone(),
            analyzed_urls: successful_urls.clone(),
            discovered_urls: Vec::new(),
            learned_patterns: Vec::new(),
            temporal_patterns: Vec::new(),
            archive_structures: Vec::new(),
            crawl_duration: Duration::from_secs(0),
            requests_made: 0,
            success_rate: 0.0,
            files_stored: Vec::new(),
            overall_confidence: 0.0,
        };

        // Step 1: Analyze successful URLs to learn patterns
        let patterns = self.analyze_url_patterns(&successful_urls, dno_key).await?;
        result.learned_patterns = patterns.clone();
        info!("Learned {} URL patterns from successful endpoints", patterns.len());

        // Step 2: Discover temporal patterns
        let temporal_patterns = self.discover_temporal_patterns(&successful_urls).await?;
        result.temporal_patterns = temporal_patterns.clone();
        info!("Discovered {} temporal patterns", temporal_patterns.len());

        // Step 3: Reconstruct historical URLs
        let historical_urls = self.reconstruct_historical_urls(&patterns, &temporal_patterns, dno_key).await?;
        info!("Generated {} historical URL candidates", historical_urls.len());

        // Step 4: Test historical URLs and discover active content
        let discovered_urls = self.test_and_discover_urls(historical_urls, &mut result.requests_made).await?;
        result.discovered_urls = discovered_urls.clone();
        info!("Successfully discovered {} active historical URLs", discovered_urls.len());

        // Step 5: Discover archive structures
        let archive_structures = self.discover_archive_structures(&successful_urls, &discovered_urls, dno_key).await?;
        result.archive_structures = archive_structures;
        info!("Identified {} archive structures", result.archive_structures.len());

        // Step 6: Download and store discovered content
        let stored_files = self.download_and_store_content(&discovered_urls, dno_key).await?;
        result.files_stored = stored_files;
        info!("Downloaded and stored {} files", result.files_stored.len());

        // Step 7: Learn from results and update patterns
        self.learn_from_results(&result, dno_key).await?;

        // Calculate final metrics
        result.crawl_duration = start_time.elapsed();
        result.success_rate = if result.requests_made > 0 {
            discovered_urls.len() as f64 / result.requests_made as f64
        } else {
            0.0
        };
        result.overall_confidence = self.calculate_overall_confidence(&result);

        info!("Reverse crawl completed in {:?} with {:.2}% success rate", 
              result.crawl_duration, result.success_rate * 100.0);

        Ok(result)
    }

    /// Discover historical data by analyzing successful patterns and extrapolating backwards
    pub async fn discover_historical_data(
        &mut self,
        dno_key: &str,
        known_years: &[i32],
    ) -> Result<ReverseCrawlResult> {
        info!("Discovering historical data for DNO: {} based on known years: {:?}", dno_key, known_years);

        // Get all successful files for this DNO
        let successful_files = self.get_successful_files_for_dno(dno_key).await?;
        let successful_urls: Vec<String> = successful_files.iter()
            .map(|f| f.source_url.clone())
            .collect();

        if successful_urls.is_empty() {
            warn!("No successful URLs found for DNO: {}", dno_key);
            return Err(anyhow!("No successful URLs found for DNO: {}", dno_key));
        }

        // Start reverse crawling from successful endpoints
        let mut result = self.reverse_crawl_from_successful_endpoints(dno_key, successful_urls).await?;

        // Enhance with historical extrapolation
        let historical_urls = self.extrapolate_historical_urls(dno_key, known_years, &result.learned_patterns).await?;
        
        if !historical_urls.is_empty() {
            info!("Generated {} additional historical URL candidates through extrapolation", historical_urls.len());
            
            let mut additional_requests = 0;
            let additional_discovered = self.test_and_discover_urls(historical_urls, &mut additional_requests).await?;
            
            result.discovered_urls.extend(additional_discovered.clone());
            result.requests_made += additional_requests;
            
            // Download additional discovered content
            let additional_files = self.download_and_store_content(&additional_discovered, dno_key).await?;
            result.files_stored.extend(additional_files);
            
            info!("Discovered {} additional historical URLs through extrapolation", additional_discovered.len());
        }

        // Recalculate metrics
        result.success_rate = if result.requests_made > 0 {
            result.discovered_urls.len() as f64 / result.requests_made as f64
        } else {
            0.0
        };
        result.overall_confidence = self.calculate_overall_confidence(&result);

        Ok(result)
    }

    /// Analyze URL patterns from successful endpoints
    async fn analyze_url_patterns(&mut self, urls: &[String], dno_key: &str) -> Result<Vec<UrlPattern>> {
        debug!("Analyzing URL patterns for {} URLs", urls.len());
        
        let mut patterns = Vec::new();
        let mut url_components: Vec<UrlComponents> = Vec::new();

        // Parse all URLs into components
        for url_str in urls {
            if let Ok(parsed_url) = self.parse_url_components(url_str) {
                url_components.push(parsed_url);
            }
        }

        if url_components.is_empty() {
            return Ok(patterns);
        }

        // Group URLs by similar structure
        let grouped_urls = self.group_urls_by_structure(&url_components);

        for (_pattern_key, group) in grouped_urls {
            if group.len() < 2 {
                continue; // Need at least 2 URLs to identify a pattern
            }

            // Extract pattern from group
            if let Some(pattern) = self.extract_url_pattern(&group, dno_key) {
                patterns.push(pattern);
            }
        }

        debug!("Extracted {} URL patterns", patterns.len());
        Ok(patterns)
    }

    /// Discover temporal patterns in URLs
    async fn discover_temporal_patterns(&mut self, urls: &[String]) -> Result<Vec<TemporalPattern>> {
        debug!("Discovering temporal patterns in {} URLs", urls.len());
        
        let mut temporal_patterns = Vec::new();
        
        // Check each URL for temporal indicators
        for url in urls {
            if let Some(patterns) = self.temporal_engine.analyze_url(url).await? {
                temporal_patterns.extend(patterns);
            }
        }

        // Consolidate and validate patterns
        let consolidated_patterns = self.temporal_engine.consolidate_patterns(temporal_patterns).await?;
        
        debug!("Discovered {} consolidated temporal patterns", consolidated_patterns.len());
        Ok(consolidated_patterns)
    }

    /// Reconstruct historical URLs based on learned patterns
    async fn reconstruct_historical_urls(
        &self,
        patterns: &[UrlPattern],
        temporal_patterns: &[TemporalPattern],
        dno_key: &str,
    ) -> Result<Vec<String>> {
        debug!("Reconstructing historical URLs using {} patterns and {} temporal patterns", 
               patterns.len(), temporal_patterns.len());

        let mut historical_urls = Vec::new();
        let current_year = Utc::now().year();
        let start_year = current_year - self.config.historical_years_back as i32;

        for pattern in patterns {
            for temporal_pattern in temporal_patterns {
                // Generate URLs for historical years
                for year in start_year..=current_year {
                    if let Some(urls) = self.url_reconstructor.reconstruct_urls_for_year(
                        pattern, 
                        temporal_pattern, 
                        year
                    ).await? {
                        historical_urls.extend(urls);
                    }
                }
            }
        }

        // Remove duplicates and limit results
        historical_urls.sort();
        historical_urls.dedup();
        historical_urls.truncate(self.config.max_urls_per_pattern * patterns.len());

        debug!("Generated {} unique historical URL candidates", historical_urls.len());
        Ok(historical_urls)
    }

    /// Test URLs and discover active content
    async fn test_and_discover_urls(
        &self,
        urls: Vec<String>,
        requests_made: &mut u32,
    ) -> Result<Vec<DiscoveredUrl>> {
        debug!("Testing {} URL candidates", urls.len());
        
        let mut discovered_urls = Vec::new();
        let mut futures = Vec::new();
        
        // Create batches to respect concurrency limits
        for chunk in urls.chunks(self.config.max_concurrent_requests) {
            for url in chunk {
                futures.push(self.test_single_url(url.clone()));
                *requests_made += 1;
                
                // Add delay between requests
                if self.config.request_delay_ms > 0 {
                    sleep(Duration::from_millis(self.config.request_delay_ms)).await;
                }
            }
            
            // Process batch
            let results = futures::future::join_all(futures.drain(..)).await;
            
            for result in results {
                if let Ok(Some(discovered_url)) = result {
                    discovered_urls.push(discovered_url);
                }
            }
        }

        debug!("Successfully discovered {} active URLs out of {} tested", discovered_urls.len(), urls.len());
        Ok(discovered_urls)
    }

    /// Test a single URL to see if it's active and contains relevant content
    async fn test_single_url(&self, url: String) -> Result<Option<DiscoveredUrl>> {
        trace!("Testing URL: {}", url);
        
        match self.client.head(&url).send().await {
            Ok(response) => {
                let status_code = response.status().as_u16();
                
                if response.status().is_success() {
                    let content_type = response.headers()
                        .get("content-type")
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string());
                    
                    let content_length = response.headers()
                        .get("content-length")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok());
                    
                    let last_modified = response.headers()
                        .get("last-modified")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| DateTime::parse_from_rfc2822(s).ok())
                        .map(|dt| dt.with_timezone(&Utc));

                    // Extract temporal data from URL
                    let temporal_data = self.extract_temporal_data_from_url(&url);

                    let discovered_url = DiscoveredUrl {
                        url: url.clone(),
                        discovery_method: DiscoveryMethod::PatternReconstruction,
                        confidence: 0.8, // Base confidence for pattern-based discovery
                        status_code: Some(status_code),
                        content_type,
                        content_length,
                        last_modified,
                        temporal_data,
                        discovered_at: Utc::now(),
                    };

                    trace!("Successfully discovered URL: {} (status: {})", url, status_code);
                    Ok(Some(discovered_url))
                } else {
                    trace!("URL returned non-success status {}: {}", status_code, url);
                    Ok(None)
                }
            }
            Err(e) => {
                trace!("Failed to test URL {}: {}", url, e);
                Ok(None)
            }
        }
    }

    /// Discover archive structures from successful and discovered URLs
    async fn discover_archive_structures(
        &mut self,
        successful_urls: &[String],
        discovered_urls: &[DiscoveredUrl],
        dno_key: &str,
    ) -> Result<Vec<ArchiveStructure>> {
        debug!("Discovering archive structures for DNO: {}", dno_key);
        
        let mut all_urls = successful_urls.to_vec();
        all_urls.extend(discovered_urls.iter().map(|d| d.url.clone()));
        
        let archive_structures = self.archive_discovery.analyze_urls(&all_urls, dno_key).await?;
        
        debug!("Discovered {} archive structures", archive_structures.len());
        Ok(archive_structures)
    }

    /// Download and store discovered content
    async fn download_and_store_content(
        &mut self,
        discovered_urls: &[DiscoveredUrl],
        dno_key: &str,
    ) -> Result<Vec<String>> {
        debug!("Downloading and storing content from {} discovered URLs", discovered_urls.len());
        
        let mut stored_files = Vec::new();
        
        for discovered_url in discovered_urls {
            // Only download PDF and document files to avoid clutter
            if let Some(content_type) = &discovered_url.content_type {
                if content_type.contains("pdf") || 
                   content_type.contains("document") || 
                   content_type.contains("spreadsheet") {
                    
                    if let Ok(file_path) = self.download_and_store_file(discovered_url, dno_key).await {
                        stored_files.push(file_path);
                    }
                }
            }
        }
        
        debug!("Successfully stored {} files", stored_files.len());
        Ok(stored_files)
    }

    /// Download and store a single file
    async fn download_and_store_file(
        &mut self,
        discovered_url: &DiscoveredUrl,
        dno_key: &str,
    ) -> Result<String> {
        trace!("Downloading file: {}", discovered_url.url);
        
        let response = self.client.get(&discovered_url.url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to download file: HTTP {}", response.status()));
        }
        
        let content = response.bytes().await?;
        let content_type = discovered_url.content_type.as_deref().unwrap_or("application/octet-stream");
        
        // Determine year from temporal data or URL
        let year = discovered_url.temporal_data.as_ref()
            .and_then(|td| td.year)
            .unwrap_or_else(|| self.extract_year_from_url(&discovered_url.url).unwrap_or(Utc::now().year()));
        
        // Generate filename
        let filename = self.generate_filename_from_url(&discovered_url.url, content_type);
        
        // Store file
        let metadata = self.source_manager.store_file(
            dno_key,
            year,
            &filename,
            &content,
            &discovered_url.url,
            &discovered_url.url,
            self.determine_source_type(content_type),
            content_type,
        ).await?;
        
        trace!("Stored file: {} -> {}", discovered_url.url, metadata.absolute_path.display());
        Ok(metadata.absolute_path.to_string_lossy().to_string())
    }

    /// Learn from crawling results and update patterns
    async fn learn_from_results(&mut self, result: &ReverseCrawlResult, dno_key: &str) -> Result<()> {
        debug!("Learning from reverse crawl results for DNO: {}", dno_key);
        
        // Learn URL patterns
        for pattern in &result.learned_patterns {
            let learned_pattern = LearnedPattern {
                pattern_type: "url_structure".to_string(),
                pattern_data: serde_json::to_value(pattern)?,
                confidence: pattern.confidence,
                metadata: HashMap::new(),
            };
            
            self.learning_engine.learn_pattern(learned_pattern).await?;
        }
        
        // Learn temporal patterns
        for temporal_pattern in &result.temporal_patterns {
            let learned_pattern = LearnedPattern {
                pattern_type: "temporal_pattern".to_string(),
                pattern_data: serde_json::to_value(temporal_pattern)?,
                confidence: temporal_pattern.confidence,
                metadata: HashMap::new(),
            };
            
            self.learning_engine.learn_pattern(learned_pattern).await?;
        }
        
        // Learn archive structures
        for archive_structure in &result.archive_structures {
            let learned_pattern = LearnedPattern {
                pattern_type: "archive_structure".to_string(),
                pattern_data: serde_json::to_value(archive_structure)?,
                confidence: archive_structure.confidence,
                metadata: HashMap::new(),
            };
            
            self.learning_engine.learn_pattern(learned_pattern).await?;
        }
        
        debug!("Successfully learned {} patterns from reverse crawl results", 
               result.learned_patterns.len() + result.temporal_patterns.len() + result.archive_structures.len());
        
        Ok(())
    }

    /// Extrapolate historical URLs based on known years and patterns
    async fn extrapolate_historical_urls(
        &self,
        dno_key: &str,
        known_years: &[i32],
        patterns: &[UrlPattern],
    ) -> Result<Vec<String>> {
        debug!("Extrapolating historical URLs for DNO: {} with known years: {:?}", dno_key, known_years);
        
        let mut historical_urls = Vec::new();
        let current_year = Utc::now().year();
        
        // Determine year range to explore
        let min_known_year = known_years.iter().min().unwrap_or(&current_year);
        let max_known_year = known_years.iter().max().unwrap_or(&current_year);
        
        // Extend range backwards and forwards
        let start_year = (*min_known_year - self.config.historical_years_back as i32).max(2000);
        let end_year = (*max_known_year + 2).min(current_year + 1);
        
        for pattern in patterns {
            for year in start_year..=end_year {
                if let Some(urls) = self.generate_urls_for_year(pattern, year).await? {
                    historical_urls.extend(urls);
                }
            }
        }
        
        // Remove duplicates and limit results
        historical_urls.sort();
        historical_urls.dedup();
        historical_urls.truncate(self.config.max_urls_per_pattern);
        
        debug!("Generated {} historical URLs through extrapolation", historical_urls.len());
        Ok(historical_urls)
    }

    /// Get successful files for a DNO from source manager
    async fn get_successful_files_for_dno(&self, dno_key: &str) -> Result<Vec<FileMetadata>> {
        // This would typically query the source manager for successful files
        // For now, we'll return the files from all years for this DNO
        let mut all_files = Vec::new();
        
        // Get current year and look back several years
        let current_year = Utc::now().year();
        for year in (current_year - 5)..=current_year {
            let files = self.source_manager.get_files_for_dno_year(dno_key, year);
            all_files.extend(files.into_iter().cloned());
        }
        
        // Filter for active, successful files
        let successful_files: Vec<FileMetadata> = all_files.into_iter()
            .filter(|f| f.is_active && f.extracted_data.is_some())
            .collect();
        
        Ok(successful_files)
    }

    /// Calculate overall confidence in results
    fn calculate_overall_confidence(&self, result: &ReverseCrawlResult) -> f64 {
        let mut confidence_scores = Vec::new();
        
        // Add pattern confidences
        for pattern in &result.learned_patterns {
            confidence_scores.push(pattern.confidence);
        }
        
        // Add temporal pattern confidences
        for temporal_pattern in &result.temporal_patterns {
            confidence_scores.push(temporal_pattern.confidence);
        }
        
        // Add archive structure confidences
        for archive in &result.archive_structures {
            confidence_scores.push(archive.confidence);
        }
        
        // Add success rate as a confidence factor
        confidence_scores.push(result.success_rate);
        
        if confidence_scores.is_empty() {
            0.0
        } else {
            confidence_scores.iter().sum::<f64>() / confidence_scores.len() as f64
        }
    }

    // Helper methods (implementation details)
    
    /// Parse URL into components for analysis
    fn parse_url_components(&self, url_str: &str) -> Result<UrlComponents> {
        let url = Url::parse(url_str)?;
        
        Ok(UrlComponents {
            scheme: url.scheme().to_string(),
            host: url.host_str().unwrap_or("").to_string(),
            port: url.port(),
            path: url.path().to_string(),
            query: url.query().map(|q| q.to_string()),
            fragment: url.fragment().map(|f| f.to_string()),
        })
    }
    
    /// Group URLs by similar structure
    fn group_urls_by_structure(&self, components: &[UrlComponents]) -> HashMap<String, Vec<UrlComponents>> {
        let mut groups = HashMap::new();
        
        for component in components {
            // Create a structure key by normalizing the path
            let structure_key = self.create_structure_key(&component.path);
            groups.entry(structure_key).or_insert_with(Vec::new).push(component.clone());
        }
        
        groups
    }
    
    /// Create a structure key for grouping similar URLs
    fn create_structure_key(&self, path: &str) -> String {
        // Replace numbers with placeholders to group similar structures
        let re = Regex::new(r"\d+").unwrap();
        re.replace_all(path, "{NUM}").to_string()
    }
    
    /// Extract URL pattern from a group of similar URLs
    fn extract_url_pattern(&self, group: &[UrlComponents], dno_key: &str) -> Option<UrlPattern> {
        if group.is_empty() {
            return None;
        }
        
        let first = &group[0];
        let mut template = format!("{}://{}{}", first.scheme, first.host, first.path);
        let mut variables = HashMap::new();
        
        // Find variable parts by comparing URLs in the group
        // This is a simplified version - a real implementation would be more sophisticated
        let re = Regex::new(r"\d{4}").unwrap(); // Look for years
        if let Some(captures) = re.find(&template) {
            template.replace_range(captures.range(), "{year}");
            variables.insert("year".to_string(), VariableType::Year);
        }
        
        Some(UrlPattern {
            id: Uuid::new_v4().to_string(),
            template,
            variables,
            confidence: 0.8,
            success_rate: 1.0, // All URLs in group were successful
            dno_key: dno_key.to_string(),
            last_success: Some(Utc::now()),
            examples: group.iter().map(|c| format!("{}://{}{}", c.scheme, c.host, c.path)).collect(),
        })
    }
    
    /// Extract temporal data from URL
    fn extract_temporal_data_from_url(&self, url: &str) -> Option<TemporalData> {
        let year_re = Regex::new(r"(\d{4})").unwrap();
        let month_re = Regex::new(r"/(\d{1,2})/").unwrap(); // Month in path segments like /03/
        
        let year = year_re.find(url)
            .and_then(|m| m.as_str().parse::<i32>().ok())
            .filter(|&y| y >= 2000 && y <= 2030); // Reasonable year range
        
        // Look for month after finding a year
        let month = if year.is_some() {
            // First try to find month in path format like /2024/03/
            month_re.find(url)
                .and_then(|m| m.as_str().trim_matches('/').parse::<u32>().ok())
                .filter(|&m| m >= 1 && m <= 12)
                .or_else(|| {
                    // If not found, try to find any 2-digit number that could be a month
                    let two_digit_re = Regex::new(r"(\d{2})").unwrap();
                    for captures in two_digit_re.find_iter(url) {
                        if let Ok(num) = captures.as_str().parse::<u32>() {
                            if num >= 1 && num <= 12 {
                                return Some(num);
                            }
                        }
                    }
                    None
                })
        } else {
            None
        };
        
        if year.is_some() || month.is_some() {
            Some(TemporalData {
                year,
                month,
                day: None,
                quarter: None,
                version: None,
            })
        } else {
            None
        }
    }
    
    /// Extract year from URL
    fn extract_year_from_url(&self, url: &str) -> Option<i32> {
        let year_re = Regex::new(r"(\d{4})").unwrap();
        year_re.find(url)
            .and_then(|m| m.as_str().parse::<i32>().ok())
            .filter(|&y| y >= 2000 && y <= 2030)
    }
    
    /// Generate filename from URL
    fn generate_filename_from_url(&self, url: &str, content_type: &str) -> String {
        let url_path = Url::parse(url).ok()
            .and_then(|u| u.path_segments().map(|s| s.last().unwrap_or("file").to_string()))
            .unwrap_or_else(|| "file".to_string());
        
        let extension = if url_path.contains('.') {
            "".to_string()
        } else {
            match content_type {
                ct if ct.contains("pdf") => ".pdf".to_string(),
                ct if ct.contains("spreadsheet") => ".xlsx".to_string(),
                ct if ct.contains("document") => ".doc".to_string(),
                _ => ".bin".to_string(),
            }
        };
        
        format!("reverse_crawl_{}_{}{}", 
                Utc::now().format("%Y%m%d_%H%M%S"),
                url_path.replace('/', "_").replace('?', "_"),
                extension)
    }
    
    /// Determine source type from content type
    fn determine_source_type(&self, content_type: &str) -> SourceType {
        match content_type {
            ct if ct.contains("pdf") => SourceType::Pdf,
            ct if ct.contains("image") => SourceType::Image,
            ct if ct.contains("text") => SourceType::Text,
            _ => SourceType::Webpage,
        }
    }
    
    /// Generate URLs for a specific year using a pattern
    async fn generate_urls_for_year(&self, pattern: &UrlPattern, year: i32) -> Result<Option<Vec<String>>> {
        if pattern.variables.contains_key("year") {
            let url = pattern.template.replace("{year}", &year.to_string());
            Ok(Some(vec![url]))
        } else {
            Ok(None)
        }
    }
}

/// URL components for analysis
#[derive(Debug, Clone)]
struct UrlComponents {
    scheme: String,
    host: String,
    port: Option<u16>,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}

impl TemporalPatternEngine {
    fn new() -> Self {
        let patterns = vec![
            TemporalPattern {
                id: Uuid::new_v4().to_string(),
                pattern_type: TemporalPatternType::Year,
                regex: r"(\d{4})".to_string(),
                format_template: "{}".to_string(),
                confidence: 0.9,
                success_count: 0,
                total_attempts: 0,
                examples: Vec::new(),
                last_seen: Utc::now(),
            },
            TemporalPattern {
                id: Uuid::new_v4().to_string(),
                pattern_type: TemporalPatternType::Year,
                regex: r"(\d{2})".to_string(),
                format_template: "20{}".to_string(),
                confidence: 0.7,
                success_count: 0,
                total_attempts: 0,
                examples: Vec::new(),
                last_seen: Utc::now(),
            },
        ];
        
        let mut matchers = HashMap::new();
        for pattern in &patterns {
            if let Ok(regex) = Regex::new(&pattern.regex) {
                matchers.insert(pattern.id.clone(), regex);
            }
        }
        
        Self { patterns, matchers }
    }
    
    async fn analyze_url(&self, url: &str) -> Result<Option<Vec<TemporalPattern>>> {
        let mut found_patterns = Vec::new();
        
        for pattern in &self.patterns {
            if let Some(regex) = self.matchers.get(&pattern.id) {
                if regex.is_match(url) {
                    found_patterns.push(pattern.clone());
                }
            }
        }
        
        if found_patterns.is_empty() {
            Ok(None)
        } else {
            Ok(Some(found_patterns))
        }
    }
    
    async fn consolidate_patterns(&self, patterns: Vec<TemporalPattern>) -> Result<Vec<TemporalPattern>> {
        // Group patterns by type and consolidate
        let mut consolidated = HashMap::new();
        
        for pattern in patterns {
            let key = format!("{:?}", pattern.pattern_type);
            consolidated.entry(key).or_insert_with(Vec::new).push(pattern);
        }
        
        let mut result = Vec::new();
        for (_, group) in consolidated {
            if let Some(best_pattern) = group.into_iter().max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap()) {
                result.push(best_pattern);
            }
        }
        
        Ok(result)
    }
}

impl ArchiveDiscovery {
    fn new() -> Self {
        Self {
            archive_patterns: Vec::new(),
            directory_templates: Vec::new(),
            file_conventions: Vec::new(),
        }
    }
    
    async fn analyze_urls(&self, urls: &[String], dno_key: &str) -> Result<Vec<ArchiveStructure>> {
        let mut structures = Vec::new();
        
        // Group URLs by domain and analyze structure
        let mut domain_groups: HashMap<String, Vec<String>> = HashMap::new();
        
        for url in urls {
            if let Ok(parsed_url) = Url::parse(url) {
                if let Some(host) = parsed_url.host_str() {
                    domain_groups.entry(host.to_string()).or_insert_with(Vec::new).push(url.clone());
                }
            }
        }
        
        for (domain, domain_urls) in domain_groups {
            if let Some(structure) = self.analyze_domain_structure(&domain, &domain_urls, dno_key).await? {
                structures.push(structure);
            }
        }
        
        Ok(structures)
    }
    
    async fn analyze_domain_structure(&self, domain: &str, urls: &[String], dno_key: &str) -> Result<Option<ArchiveStructure>> {
        if urls.len() < 2 {
            return Ok(None);
        }
        
        // Extract common path prefixes
        let paths: Vec<String> = urls.iter()
            .filter_map(|url| Url::parse(url).ok())
            .map(|u| u.path().to_string())
            .collect();
        
        let common_prefix = self.find_common_path_prefix(&paths);
        let directory_structure = self.extract_directory_structure(&paths);
        let file_patterns = self.extract_file_patterns(&paths);
        let temporal_organization = self.determine_temporal_organization(&paths);
        
        let structure = ArchiveStructure {
            id: Uuid::new_v4().to_string(),
            base_url: format!("https://{}{}", domain, common_prefix),
            directory_structure,
            file_patterns,
            temporal_organization,
            confidence: 0.8,
            dno_key: dno_key.to_string(),
        };
        
        Ok(Some(structure))
    }
    
    fn find_common_path_prefix(&self, paths: &[String]) -> String {
        if paths.is_empty() {
            return "/".to_string();
        }
        
        let first = &paths[0];
        let mut prefix = String::new();
        
        for (i, ch) in first.chars().enumerate() {
            if paths.iter().all(|p| p.chars().nth(i) == Some(ch)) {
                prefix.push(ch);
            } else {
                break;
            }
        }
        
        // Ensure we end at a path boundary
        if let Some(last_slash) = prefix.rfind('/') {
            prefix.truncate(last_slash + 1);
        }
        
        prefix
    }
    
    fn extract_directory_structure(&self, paths: &[String]) -> Vec<String> {
        let mut structures = HashSet::new();
        
        for path in paths {
            let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
            for i in 1..=parts.len() {
                let structure = parts[..i].join("/");
                structures.insert(structure);
            }
        }
        
        let mut result: Vec<String> = structures.into_iter().collect();
        result.sort();
        result
    }
    
    fn extract_file_patterns(&self, paths: &[String]) -> Vec<String> {
        let mut patterns = HashSet::new();
        
        for path in paths {
            if let Some(filename) = path.split('/').last() {
                // Extract file extension
                if let Some(dot_pos) = filename.rfind('.') {
                    let extension = &filename[dot_pos..];
                    patterns.insert(format!("*{}", extension));
                }
                
                // Extract naming patterns (simplified)
                let pattern = Regex::new(r"\d+").unwrap().replace_all(filename, "*");
                patterns.insert(pattern.to_string());
            }
        }
        
        patterns.into_iter().collect()
    }
    
    fn determine_temporal_organization(&self, paths: &[String]) -> TemporalOrganization {
        let year_regex = Regex::new(r"\d{4}").unwrap();
        let month_regex = Regex::new(r"\d{2}").unwrap();
        
        let has_years = paths.iter().any(|p| year_regex.is_match(p));
        let has_months = paths.iter().any(|p| month_regex.is_match(p));
        
        if has_years && has_months {
            TemporalOrganization::ByYearMonth
        } else if has_years {
            TemporalOrganization::ByYear
        } else {
            TemporalOrganization::None
        }
    }
}

impl UrlReconstructor {
    fn new() -> Self {
        Self {
            base_patterns: Vec::new(),
            strategies: vec![
                ReconstructionStrategy {
                    name: "year_substitution".to_string(),
                    priority: 10,
                    apply: |template, variables| {
                        if let Some(year) = variables.get("year") {
                            vec![template.replace("{year}", year)]
                        } else {
                            Vec::new()
                        }
                    },
                },
            ],
        }
    }
    
    async fn reconstruct_urls_for_year(
        &self,
        pattern: &UrlPattern,
        _temporal_pattern: &TemporalPattern,
        year: i32,
    ) -> Result<Option<Vec<String>>> {
        let mut variables = HashMap::new();
        variables.insert("year".to_string(), year.to_string());
        
        let urls = (self.strategies[0].apply)(&pattern.template, &variables);
        
        if urls.is_empty() {
            Ok(None)
        } else {
            Ok(Some(urls))
        }
    }
}

// Include tests
#[path = "reverse_crawler_test.rs"]
mod tests;