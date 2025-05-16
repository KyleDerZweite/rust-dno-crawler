use reqwest::Client;

#[tokio::main]
pub async fn search_xng(keyword: &str) -> anyhow::Result<()> {
    
    let client = Client::new();
    let resp = client
        .get("http://65.109.38.216:10000/search")
        .query(&[("q", &keyword), ("format", &"json")])
        .send()
        .await?
        .error_for_status()?;

    let json: serde_json::Value = resp.json().await?;
    println!("{}", serde_json::to_string_pretty(&json)?);
    
    Ok(())
}