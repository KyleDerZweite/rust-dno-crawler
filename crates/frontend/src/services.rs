// API service layer for communicating with the backend
use shared::{Dno, SearchQuery, SearchResult, CrawlJob, AppError};
use reqwest::Client;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    
    pub async fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>, AppError> {
        let url = format!("{}/api/v1/search", self.base_url);
        let response = self.client
            .post(&url)
            .json(&query)
            .send()
            .await
            .map_err(AppError::Http)?;
            
        if !response.status().is_success() {
            return Err(AppError::Http(reqwest::Error::from(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("HTTP {}", response.status())
                )
            )));
        }
        
        let results = response.json().await.map_err(AppError::Http)?;
        Ok(results)
    }
    
    pub async fn get_dnos(&self) -> Result<Vec<Dno>, AppError> {
        let url = format!("{}/api/v1/dnos", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(AppError::Http)?;
            
        if !response.status().is_success() {
            return Err(AppError::Http(reqwest::Error::from(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("HTTP {}", response.status())
                )
            )));
        }
        
        let dnos = response.json().await.map_err(AppError::Http)?;
        Ok(dnos)
    }
    
    pub async fn create_crawl_job(&self, url: String) -> Result<CrawlJob, AppError> {
        let api_url = format!("{}/api/v1/crawl/jobs", self.base_url);
        let payload = serde_json::json!({ "url": url });
        
        let response = self.client
            .post(&api_url)
            .json(&payload)
            .send()
            .await
            .map_err(AppError::Http)?;
            
        if !response.status().is_success() {
            return Err(AppError::Http(reqwest::Error::from(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("HTTP {}", response.status())
                )
            )));
        }
        
        let job = response.json().await.map_err(AppError::Http)?;
        Ok(job)
    }
}