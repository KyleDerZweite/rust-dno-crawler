use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::Value;

const SEARCH_ENGINE_URL: &str = "http://65.109.38.216:10000/search"; // SearXNG URL

#[tokio::main]
pub async fn search_xng(keyword: &str) -> Result<()> {
    let client = Client::new();
    let resp = client
        .get(SEARCH_ENGINE_URL)
        .query(&[("q", keyword), ("format", "json")])
        .send()
        .await?
        .error_for_status()?;

    let json: Value = resp.json().await?;
    let results = json["results"]
        .as_array()
        .ok_or_else(|| anyhow!("Invalid response: missing results array"))?;

    let filtered: Vec<&Value> = results
        .iter()
        .filter(|r| r["score"].as_f64().unwrap_or(0.0) > 1.0)
        .collect();

    if filtered.is_empty() {
        Err(anyhow!("No results with score > 1.0"))
    } else {
        println!("{}", serde_json::to_string_pretty(&filtered)?);
        Ok(())
    }
}