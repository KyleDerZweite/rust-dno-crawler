use reqwest::Client;

async fn call_ollama(client: &Client,model: &str, prompt: &str) -> anyhow::Result<String> {
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
    });
    let resp = client
        .post("http://localhost:11434/completions")
        .json(&body)
        .send()
        .await?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json["choices"][0]["text"].as_str().unwrap_or("").to_string())
}
