use reqwest::Client;
use scraper::{Html, Selector};
use shared::{CrawlResult, PageMetadata, AppError};
use std::time::Duration;
use tracing::{info, warn, error};
use url::Url;

pub struct WebCrawler {
    client: Client,
    max_retries: u32,
    delay_between_requests: Duration,
    user_agent: String,
}

impl WebCrawler {
    pub fn new(max_retries: u32, delay_ms: u64, user_agent: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(&user_agent)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            max_retries,
            delay_between_requests: Duration::from_millis(delay_ms),
            user_agent,
        }
    }

    pub async fn crawl(&self, url: &str) -> Result<CrawlResult, AppError> {
        info!("Starting crawl for URL: {}", url);
        
        let response = self.fetch_with_retries(url).await?;
        let content = response.text().await
            .map_err(|e| AppError::Http(e))?;
        
        let document = Html::parse_document(&content);
        
        let metadata = self.extract_metadata(&document, url);
        let links = self.extract_links(&document, url);
        let extracted_data = self.extract_structured_data(&document);

        Ok(CrawlResult {
            content,
            extracted_data,
            links,
            metadata,
        })
    }

    async fn fetch_with_retries(&self, url: &str) -> Result<reqwest::Response, AppError> {
        let mut last_error = AppError::Internal("No attempts made".to_string());
        
        for attempt in 1..=self.max_retries {
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("Successfully fetched {} on attempt {}", url, attempt);
                        return Ok(response);
                    } else {
                        warn!("HTTP {} for {} on attempt {}", response.status(), url, attempt);
                        last_error = AppError::ServiceUnavailable(
                            format!("HTTP {}", response.status())
                        );
                    }
                }
                Err(e) => {
                    warn!("Request failed for {} on attempt {}: {}", url, attempt, e);
                    last_error = AppError::Http(e);
                }
            }
            
            if attempt < self.max_retries {
                tokio::time::sleep(self.delay_between_requests).await;
            }
        }
        
        error!("Failed to fetch {} after {} attempts", url, self.max_retries);
        Err(last_error)
    }

    fn extract_metadata(&self, document: &Html, _url: &str) -> PageMetadata {
        let title = document
            .select(&Selector::parse("title").unwrap())
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|s| !s.is_empty());

        let description = document
            .select(&Selector::parse("meta[name='description'], meta[property='og:description']").unwrap())
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        let keywords = document
            .select(&Selector::parse("meta[name='keywords']").unwrap())
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(|s| s.split(',').map(|k| k.trim().to_string()).collect())
            .unwrap_or_default();

        let language = document
            .select(&Selector::parse("html").unwrap())
            .next()
            .and_then(|el| el.value().attr("lang"))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        PageMetadata {
            title,
            description,
            keywords,
            language,
            last_modified: None, // TODO: Extract from HTTP headers or meta tags
        }
    }

    fn extract_links(&self, document: &Html, base_url: &str) -> Vec<String> {
        let base = Url::parse(base_url).ok();
        let selector = Selector::parse("a[href]").unwrap();
        
        document
            .select(&selector)
            .filter_map(|el| el.value().attr("href"))
            .filter_map(|href| {
                if let Some(base) = &base {
                    base.join(href).ok().map(|url| url.to_string())
                } else {
                    Some(href.to_string())
                }
            })
            .filter(|url| url.starts_with("http"))
            .collect()
    }

    fn extract_structured_data(&self, document: &Html) -> serde_json::Value {
        let mut data = serde_json::Map::new();
        
        // Extract contact information
        if let Some(email) = self.extract_email(document) {
            data.insert("email".to_string(), serde_json::Value::String(email));
        }
        
        if let Some(phone) = self.extract_phone(document) {
            data.insert("phone".to_string(), serde_json::Value::String(phone));
        }
        
        // Extract headings
        let headings = self.extract_headings(document);
        if !headings.is_empty() {
            data.insert("headings".to_string(), serde_json::Value::Array(
                headings.into_iter().map(serde_json::Value::String).collect()
            ));
        }
        
        serde_json::Value::Object(data)
    }

    fn extract_email(&self, document: &Html) -> Option<String> {
        let text = document.root_element().text().collect::<String>();
        let email_regex = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").ok()?;
        
        email_regex.find(&text).map(|m| m.as_str().to_string())
    }

    fn extract_phone(&self, document: &Html) -> Option<String> {
        let text = document.root_element().text().collect::<String>();
        let phone_regex = regex::Regex::new(r"\+?[0-9\s\-\(\)]{8,}").ok()?;
        
        phone_regex.find(&text).map(|m| m.as_str().trim().to_string())
    }

    fn extract_headings(&self, document: &Html) -> Vec<String> {
        let selector = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();
        
        document
            .select(&selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}