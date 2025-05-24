use reqwest::Client;
use serde_json::json; // Ensure serde_json is imported if not already

// Make the function public
pub async fn call_ollama(model: &str, prompt: &str) -> anyhow::Result<String> {
    let body = json!({ // Use the imported json macro
        "model": model,
        "prompt": prompt,
        "stream": false // Ensure stream is false for a single completion
    });
    let resp = Client::new()
        .post("http://localhost:11434/api/generate") // Common Ollama generate endpoint
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let error_body = resp.text().await?;
        return Err(anyhow::anyhow!("Ollama API request failed: {}", error_body));
    }
    
    let json_resp: serde_json::Value = resp.json().await?;
    // Ollama's /api/generate endpoint typically returns a JSON object with a "response" field for the full text.
    // The /completions endpoint might be different or for older/specific setups.
    // Adjust based on your Ollama version and how deepseek is integrated.
    // Assuming the response structure for /api/generate:
    // { "model": "...", "created_at": "...", "response": "...", "done": true, ... }
    Ok(json_resp["response"].as_str().unwrap_or("").to_string())
}
