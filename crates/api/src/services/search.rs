use async_trait::async_trait;
use shared::{SearchResult, AppError};
use serde_json::Value;

#[async_trait]
pub trait SearchSource {
    async fn search(&self, query: &str, limit: Option<u32>) -> Result<Vec<SearchResult>, AppError>;
    fn name(&self) -> &'static str;
}

#[derive(Clone)]
pub struct SearxngSource {
    base_url: String,
    client: reqwest::Client,
}

impl SearxngSource {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl SearchSource for SearxngSource {
    async fn search(&self, query: &str, limit: Option<u32>) -> Result<Vec<SearchResult>, AppError> {
        let url = format!("{}/search", self.base_url);
        let limit = limit.unwrap_or(10);
        
        let params = [
            ("q", query),
            ("format", "json"),
            ("categories", "general"),
        ];
        
        tracing::info!("Searching SearXNG for: {}", query);
        
        let response = self.client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(AppError::Http)?;
            
        if !response.status().is_success() {
            return Err(AppError::ServiceUnavailable(
                format!("SearXNG returned status: {}", response.status())
            ));
        }
        
        let json: Value = response.json().await.map_err(AppError::Http)?;
        
        let results = json["results"].as_array()
            .ok_or_else(|| AppError::ServiceUnavailable("Invalid SearXNG response format".to_string()))?;
            
        let search_results: Vec<SearchResult> = results
            .iter()
            .take(limit as usize)
            .filter_map(|result| {
                let title = result["title"].as_str()?;
                let url = result["url"].as_str()?;
                let content = result["content"].as_str().unwrap_or("");
                
                Some(SearchResult {
                    id: uuid::Uuid::new_v4(),
                    title: title.to_string(),
                    url: url.to_string(),
                    snippet: content.to_string(),
                    source: "SearXNG".to_string(),
                    relevance_score: 1.0, // SearXNG doesn't provide relevance scores
                    found_at: chrono::Utc::now(),
                })
            })
            .collect();
            
        tracing::info!("Found {} results from SearXNG", search_results.len());
        Ok(search_results)
    }
    
    fn name(&self) -> &'static str {
        "SearXNG"
    }
}

#[derive(Clone)]
pub struct MockSearchSource;

#[async_trait]
impl SearchSource for MockSearchSource {
    async fn search(&self, query: &str, limit: Option<u32>) -> Result<Vec<SearchResult>, AppError> {
        let limit = limit.unwrap_or(10);
        
        // Generate mock results
        let results: Vec<SearchResult> = (0..limit.min(5))
            .map(|i| SearchResult {
                id: uuid::Uuid::new_v4(),
                title: format!("Mock result {} for '{}'", i + 1, query),
                url: format!("https://mock-dno-{}.de/page", i + 1),
                snippet: format!("This is a mock search result snippet for query '{}'. It contains relevant information about German DNOs.", query),
                source: format!("Mock DNO {}", i + 1),
                relevance_score: 1.0 - (i as f64 * 0.1),
                found_at: chrono::Utc::now(),
            })
            .collect();
            
        tracing::info!("Generated {} mock search results", results.len());
        Ok(results)
    }
    
    fn name(&self) -> &'static str {
        "Mock Search"
    }
}

#[derive(Clone)]
pub struct SearchService {
    searxng: Option<SearxngSource>,
    mock: MockSearchSource,
}

impl SearchService {
    pub fn new(searxng_url: Option<String>) -> Self {
        Self {
            searxng: searxng_url.map(|url| SearxngSource::new(url)),
            mock: MockSearchSource,
        }
    }
    
    pub async fn search(&self, query: &str, limit: Option<u32>) -> Result<Vec<SearchResult>, AppError> {
        // Try SearXNG first if available, fall back to mock
        if let Some(searxng) = &self.searxng {
            match searxng.search(query, limit).await {
                Ok(results) => return Ok(results),
                Err(e) => {
                    tracing::warn!("SearXNG search failed, falling back to mock: {}", e);
                }
            }
        }
        
        // Use mock source as fallback
        self.mock.search(query, limit).await
    }
}