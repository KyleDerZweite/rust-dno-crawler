use scraper::{Html, Selector};
use shared::AppError;
use serde_json::{Value, Map};

#[derive(Clone)]
pub struct CrawlService {
    client: reqwest::Client,
    max_retries: u32,
    delay_ms: u64,
}

#[derive(Debug, Clone)]
pub struct CrawlResult {
    pub url: String,
    pub status_code: u16,
    pub title: Option<String>,
    pub content: String,
    pub links: Vec<String>,
    pub emails: Vec<String>,
    pub phone_numbers: Vec<String>,
    pub metadata: Value,
}

impl CrawlService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("DNO Data Gatherer Bot/1.0")
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap(),
            max_retries: 3,
            delay_ms: 1000,
        }
    }
    
    pub async fn crawl(&self, url: &str) -> Result<CrawlResult, AppError> {
        tracing::info!("Starting crawl of: {}", url);
        
        // Validate URL
        let parsed_url = url::Url::parse(url)
            .map_err(|e| AppError::Validation(format!("Invalid URL: {}", e)))?;
        
        // Fetch the page with retries
        let response = self.fetch_with_retries(url).await?;
        let status_code = response.status().as_u16();
        let html_content = response.text().await.map_err(AppError::Http)?;
        
        // Parse HTML
        let document = Html::parse_document(&html_content);
        
        // Extract data
        let title = self.extract_title(&document);
        let links = self.extract_links(&document, &parsed_url);
        let emails = self.extract_emails(&document);
        let phone_numbers = self.extract_phone_numbers(&document);
        let metadata = self.extract_metadata(&document);
        
        tracing::info!("Successfully crawled {} (status: {}, links: {}, emails: {}, phones: {})", 
                      url, status_code, links.len(), emails.len(), phone_numbers.len());
        
        Ok(CrawlResult {
            url: url.to_string(),
            status_code,
            title,
            content: html_content,
            links,
            emails,
            phone_numbers,
            metadata,
        })
    }
    
    async fn fetch_with_retries(&self, url: &str) -> Result<reqwest::Response, AppError> {
        let mut last_error = AppError::Internal("No attempts made".to_string());
        
        for attempt in 1..=self.max_retries {
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        tracing::info!("Successfully fetched {} on attempt {}", url, attempt);
                        return Ok(response);
                    } else {
                        tracing::warn!("HTTP {} for {} on attempt {}", response.status(), url, attempt);
                        last_error = AppError::ServiceUnavailable(
                            format!("HTTP {}", response.status())
                        );
                    }
                }
                Err(e) => {
                    tracing::warn!("Request failed for {} on attempt {}: {}", url, attempt, e);
                    last_error = AppError::Http(e);
                }
            }
            
            if attempt < self.max_retries {
                tokio::time::sleep(std::time::Duration::from_millis(self.delay_ms)).await;
            }
        }
        
        tracing::error!("Failed to fetch {} after {} attempts", url, self.max_retries);
        Err(last_error)
    }
    
    fn extract_title(&self, document: &Html) -> Option<String> {
        document
            .select(&Selector::parse("title").unwrap())
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|s| !s.is_empty())
    }
    
    fn extract_links(&self, document: &Html, base_url: &url::Url) -> Vec<String> {
        let selector = Selector::parse("a[href]").unwrap();
        document
            .select(&selector)
            .filter_map(|el| {
                el.value().attr("href").and_then(|href| {
                    base_url.join(href).ok().map(|url| url.to_string())
                })
            })
            .filter(|url| url.starts_with("http"))
            .collect()
    }
    
    fn extract_emails(&self, document: &Html) -> Vec<String> {
        let text = document.root_element().text().collect::<String>();
        if let Ok(email_regex) = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b") {
            email_regex
                .find_iter(&text)
                .map(|m| m.as_str().to_string())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    fn extract_phone_numbers(&self, document: &Html) -> Vec<String> {
        let text = document.root_element().text().collect::<String>();
        if let Ok(phone_regex) = regex::Regex::new(r"\+49[\s\-]?[0-9\s\-\(\)]{8,}|0[0-9\s\-\(\)]{8,}") {
            phone_regex
                .find_iter(&text)
                .map(|m| m.as_str().trim().to_string())
                .filter(|p| p.len() > 8)
                .collect()
        } else {
            Vec::new()
        }
    }
    
    fn extract_metadata(&self, document: &Html) -> Value {
        let mut metadata = Map::new();
        
        // Extract meta description
        if let Ok(desc_selector) = Selector::parse("meta[name='description']") {
            if let Some(desc) = document.select(&desc_selector).next() {
                if let Some(content) = desc.value().attr("content") {
                    metadata.insert("description".to_string(), Value::String(content.to_string()));
                }
            }
        }
        
        // Extract meta keywords
        if let Ok(keywords_selector) = Selector::parse("meta[name='keywords']") {
            if let Some(keywords) = document.select(&keywords_selector).next() {
                if let Some(content) = keywords.value().attr("content") {
                    metadata.insert("keywords".to_string(), Value::String(content.to_string()));
                }
            }
        }
        
        // Extract headings
        let headings: Vec<String> = ["h1", "h2", "h3"]
            .iter()
            .flat_map(|tag| {
                if let Ok(selector) = Selector::parse(tag) {
                    document
                        .select(&selector)
                        .map(|el| el.text().collect::<String>().trim().to_string())
                        .filter(|s| !s.is_empty())
                        .take(5) // Limit to 5 headings per level
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            })
            .collect();
        
        if !headings.is_empty() {
            metadata.insert("headings".to_string(), 
                           Value::Array(headings.into_iter().map(Value::String).collect()));
        }
        
        Value::Object(metadata)
    }
}

impl Default for CrawlService {
    fn default() -> Self {
        Self::new()
    }
}