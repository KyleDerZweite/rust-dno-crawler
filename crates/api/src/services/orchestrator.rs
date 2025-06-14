use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared::AppError;
use sqlx::SqlitePool;
use std::collections::HashMap;

use super::ai::{OllamaService, ProcessedQuery, AIResponse};
use super::search::SearchService;
use super::crawl::CrawlService;

#[derive(Clone)]
pub struct SearchOrchestrator {
    ai_service: OllamaService,
    search_service: SearchService,
    crawl_service: CrawlService,
    db_pool: SqlitePool,
    cache: InMemoryCache,
}

#[derive(Clone)]
struct InMemoryCache {
    data: std::sync::Arc<tokio::sync::RwLock<HashMap<String, CachedResult>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedResult {
    data: Value,
    timestamp: chrono::DateTime<chrono::Utc>,
    expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntelligentSearchRequest {
    pub query: String,
    pub use_cache: Option<bool>,
    pub max_results: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntelligentSearchResponse {
    pub success: bool,
    pub ai_response: Option<AIResponse>,
    pub processed_query: ProcessedQuery,
    pub data_sources: Vec<DataSource>,
    pub cache_hit: bool,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub source_type: String, // "cache", "database", "crawler", "search"
    pub url: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub confidence: f64,
}

impl InMemoryCache {
    fn new() -> Self {
        Self {
            data: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    async fn get(&self, key: &str) -> Option<CachedResult> {
        let cache = self.data.read().await;
        cache.get(key).and_then(|result| {
            if result.expires_at > chrono::Utc::now() {
                Some(result.clone())
            } else {
                None
            }
        })
    }

    async fn set(&self, key: String, data: Value, ttl_seconds: i64) {
        let mut cache = self.data.write().await;
        let now = chrono::Utc::now();
        cache.insert(key, CachedResult {
            data,
            timestamp: now,
            expires_at: now + chrono::Duration::seconds(ttl_seconds),
        });
    }

    async fn cleanup_expired(&self) {
        let mut cache = self.data.write().await;
        let now = chrono::Utc::now();
        cache.retain(|_, v| v.expires_at > now);
    }
}

impl SearchOrchestrator {
    pub fn new(
        ai_service: OllamaService,
        search_service: SearchService,
        crawl_service: CrawlService,
        db_pool: SqlitePool,
    ) -> Self {
        Self {
            ai_service,
            search_service,
            crawl_service,
            db_pool,
            cache: InMemoryCache::new(),
        }
    }

    pub async fn intelligent_search(&self, request: IntelligentSearchRequest) -> Result<IntelligentSearchResponse, AppError> {
        let start_time = std::time::Instant::now();
        let use_cache = request.use_cache.unwrap_or(true);
        
        // Step 1: Process query with AI
        tracing::info!("Processing query with AI: {}", request.query);
        let processed_query = self.ai_service.process_query(&request.query).await?;
        
        // Step 2: Generate cache key
        let cache_key = self.generate_cache_key(&processed_query);
        
        // Step 3: Check cache first
        let mut data_sources = Vec::new();
        let mut final_data = None;
        let mut cache_hit = false;

        if use_cache {
            if let Some(cached) = self.cache.get(&cache_key).await {
                tracing::info!("Cache hit for key: {}", cache_key);
                final_data = Some(cached.data);
                cache_hit = true;
                data_sources.push(DataSource {
                    source_type: "cache".to_string(),
                    url: None,
                    timestamp: cached.timestamp,
                    confidence: 1.0,
                });
            }
        }

        // Step 4: If no cache hit, try database
        if final_data.is_none() {
            match self.search_database(&processed_query).await {
                Ok(Some(db_data)) => {
                    tracing::info!("Database hit for query");
                    final_data = Some(db_data.clone());
                    data_sources.push(DataSource {
                        source_type: "database".to_string(),
                        url: None,
                        timestamp: chrono::Utc::now(),
                        confidence: 0.9,
                    });
                    
                    // Cache the database result
                    if use_cache {
                        self.cache.set(cache_key.clone(), db_data, 3600).await; // 1 hour TTL
                    }
                }
                Ok(None) => {
                    tracing::info!("No database results found");
                }
                Err(e) => {
                    tracing::warn!("Database search failed: {}", e);
                }
            }
        }

        // Step 5: If still no data, use search + crawling
        if final_data.is_none() {
            match self.search_and_crawl(&processed_query, request.max_results).await {
                Ok(crawl_data) => {
                    tracing::info!("Search and crawl completed");
                    final_data = Some(crawl_data.clone());
                    data_sources.push(DataSource {
                        source_type: "crawler".to_string(),
                        url: None,
                        timestamp: chrono::Utc::now(),
                        confidence: 0.7,
                    });
                    
                    // Store in database for future use
                    if let Err(e) = self.store_in_database(&processed_query, &crawl_data).await {
                        tracing::warn!("Failed to store results in database: {}", e);
                    }
                    
                    // Cache the results
                    if use_cache {
                        self.cache.set(cache_key, crawl_data, 1800).await; // 30 minutes TTL
                    }
                }
                Err(e) => {
                    tracing::error!("Search and crawl failed: {}", e);
                    return Err(e);
                }
            }
        }

        // Step 6: Generate AI response
        let final_data = final_data.unwrap_or_else(|| json!({"message": "Keine Daten gefunden"}));
        let ai_response = self.ai_service.generate_response(&processed_query, &final_data).await?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(IntelligentSearchResponse {
            success: true,
            ai_response: Some(ai_response),
            processed_query,
            data_sources,
            cache_hit,
            processing_time_ms: processing_time,
        })
    }

    async fn search_database(&self, query: &ProcessedQuery) -> Result<Option<Value>, AppError> {
        // TODO: Implement actual database search based on processed query
        // For now, return None to indicate no database results
        
        // Example query structure:
        // SELECT data FROM search_results 
        // WHERE (dno_name IN (?) OR search_terms LIKE ?) 
        // AND (years IN (?) OR years IS NULL)
        // ORDER BY relevance DESC, created_at DESC
        // LIMIT 1
        
        tracing::info!("Searching database for: {:?}", query.dno_names);
        Ok(None)
    }

    async fn search_and_crawl(&self, query: &ProcessedQuery, max_results: Option<u32>) -> Result<Value, AppError> {
        let mut all_data = Vec::new();
        
        // Build search terms from processed query
        let search_terms = if query.dno_names.is_empty() {
            query.search_terms.clone()
        } else {
            query.dno_names.iter()
                .map(|name| format!("{} Deutschland Netzbetreiber", name))
                .collect()
        };

        for search_term in search_terms.iter().take(3) { // Limit to 3 searches
            match self.search_service.search(search_term, Some(5)).await {
                Ok(search_results) => {
                    for result in search_results.iter().take(2) { // Limit crawling
                        match self.crawl_service.crawl(&result.url).await {
                            Ok(crawl_result) => {
                                let extracted_data = json!({
                                    "source_url": result.url,
                                    "title": crawl_result.title,
                                    "dno_data": {
                                        "emails": crawl_result.emails,
                                        "phones": crawl_result.phone_numbers,
                                        "metadata": crawl_result.metadata
                                    },
                                    "search_result": {
                                        "title": result.title,
                                        "snippet": result.snippet,
                                        "source": result.source
                                    }
                                });
                                all_data.push(extracted_data);
                            }
                            Err(e) => {
                                tracing::warn!("Failed to crawl {}: {}", result.url, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Search failed for '{}': {}", search_term, e);
                }
            }
        }

        Ok(json!({
            "query": query.original_query,
            "processed_query": query,
            "results": all_data,
            "result_count": all_data.len(),
            "timestamp": chrono::Utc::now()
        }))
    }

    async fn store_in_database(&self, _query: &ProcessedQuery, _data: &Value) -> Result<(), AppError> {
        // TODO: Implement database storage
        // INSERT INTO search_results (query_hash, dno_names, years, data, created_at)
        // VALUES (?, ?, ?, ?, ?)
        tracing::info!("TODO: Store results in database");
        Ok(())
    }

    fn generate_cache_key(&self, query: &ProcessedQuery) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        query.dno_names.hash(&mut hasher);
        query.years.hash(&mut hasher);
        query.data_types.hash(&mut hasher);
        
        format!("search_{:x}", hasher.finish())
    }

    pub async fn cleanup_cache(&self) {
        self.cache.cleanup_expired().await;
    }
}