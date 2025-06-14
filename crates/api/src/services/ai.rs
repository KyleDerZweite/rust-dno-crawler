use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared::{AppError, DnoDataType, DnoQueryParseResult};

#[derive(Clone)]
pub struct OllamaService {
    client: reqwest::Client,
    base_url: String,
    model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    response: String,
    done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedQuery {
    pub original_query: String,
    pub search_terms: Vec<String>,
    pub dno_names: Vec<String>,
    pub years: Vec<String>,
    pub data_types: Vec<String>,
    pub intent: QueryIntent,
}

// DnoQueryParseResult moved to shared/models.rs

// DnoDataType moved to shared/models.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryIntent {
    NetworkData,
    ContactInfo,
    ServiceAreas,
    Tariffs,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub message: String,
    pub data: Value,
    pub sources: Vec<String>,
}

impl OllamaService {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap(),
            base_url,
            model,
        }
    }

    pub async fn parse_dno_query(&self, user_query: &str) -> Result<DnoQueryParseResult, AppError> {
        let prompt = format!(
            r#"Du bist ein Experte für deutsche Netzbetreiber. Analysiere die folgende Anfrage und extrahiere DNO-spezifische Informationen.

Benutzeranfrage: "{}"

Antworte AUSSCHLIESSLICH mit gültigem JSON in folgendem Format:
{{
    "dno_name": "Netze BW",
    "dno_key": "netze-bw",
    "years": [2024, 2025],
    "data_types": ["Netzentgelte", "Hlzf"],
    "confidence": 0.95
}}

Bekannte DNO Namen und ihre Keys:
- "Netze BW" -> "netze-bw"
- "Bayernwerk" -> "bayernwerk" 
- "E.DIS" -> "e-dis"
- "Avacon" -> "avacon"
- "Westnetz" -> "westnetz"
- "Stromnetz Berlin" -> "stromnetz-berlin"

Data Types: ["Netzentgelte", "Hlzf", "Both"]
Confidence: 0.0 bis 1.0

Antworte NUR mit dem JSON, keine zusätzlichen Erklärungen."#,
            user_query
        );

        let response = self.query_ollama(&prompt).await?;
        
        // Try to parse the JSON response
        match serde_json::from_str::<DnoQueryParseResult>(&response) {
            Ok(result) => Ok(result),
            Err(_) => {
                // Fallback: try to extract JSON from response
                if let Some(json_start) = response.find('{') {
                    if let Some(json_end) = response.rfind('}') {
                        let json_str = &response[json_start..=json_end];
                        match serde_json::from_str::<DnoQueryParseResult>(json_str) {
                            Ok(result) => Ok(result),
                            Err(e) => {
                                tracing::error!("Failed to parse DNO query response: {}", e);
                                tracing::error!("Raw response: {}", response);
                                
                                // Fallback to manual parsing
                                Ok(self.fallback_parse_dno_query(user_query))
                            }
                        }
                    } else {
                        Ok(self.fallback_parse_dno_query(user_query))
                    }
                } else {
                    Ok(self.fallback_parse_dno_query(user_query))
                }
            }
        }
    }

    pub async fn process_query(&self, user_query: &str) -> Result<ProcessedQuery, AppError> {
        let prompt = format!(
            r#"Du bist ein Experte für deutsche Netzbetreiber (DNOs). Analysiere die folgende Benutzeranfrage und extrahiere strukturierte Informationen.

Benutzeranfrage: "{}"

Antworte AUSSCHLIESSLICH mit gültigem JSON in folgendem Format:
{{
    "original_query": "{}",
    "search_terms": ["Begriff1", "Begriff2"],
    "dno_names": ["Netzbetreiber1", "Netzbetreiber2"],
    "years": ["2024", "2025"],
    "data_types": ["Netzentgelte", "Anschlussbedingungen"],
    "intent": "NetworkData"
}}

Mögliche Intents: NetworkData, ContactInfo, ServiceAreas, Tariffs, General

Bekannte DNO Namen: Netze BW, Avacon, Bayernwerk, Stromnetz Berlin, Westnetz, E.DIS, 50Hertz, Amprion, TenneT, TransnetBW

Antworte NUR mit dem JSON, keine zusätzlichen Erklärungen."#,
            user_query, user_query
        );

        let response = self.query_ollama(&prompt).await?;
        
        // Try to parse the JSON response
        match serde_json::from_str::<ProcessedQuery>(&response) {
            Ok(processed) => Ok(processed),
            Err(_) => {
                // Fallback: try to extract JSON from response
                if let Some(json_start) = response.find('{') {
                    if let Some(json_end) = response.rfind('}') {
                        let json_str = &response[json_start..=json_end];
                        match serde_json::from_str::<ProcessedQuery>(json_str) {
                            Ok(processed) => Ok(processed),
                            Err(e) => {
                                tracing::error!("Failed to parse AI response: {}", e);
                                tracing::error!("Raw response: {}", response);
                                
                                // Fallback to manual parsing
                                Ok(self.fallback_parse_query(user_query))
                            }
                        }
                    } else {
                        Ok(self.fallback_parse_query(user_query))
                    }
                } else {
                    Ok(self.fallback_parse_query(user_query))
                }
            }
        }
    }

    pub async fn generate_response(&self, query: &ProcessedQuery, data: &Value) -> Result<AIResponse, AppError> {
        let prompt = format!(
            r#"Du bist ein hilfreicher Assistent für deutsche Netzbetreiber-Daten. 

Ursprüngliche Anfrage: "{}"
Gefundene Daten: {}

Erstelle eine freundliche, informative Antwort auf Deutsch. Erkläre kurz, was gefunden wurde und präsentiere die Daten strukturiert.

Format der Antwort:
- Kurze Einleitung (1-2 Sätze)
- Dann die Daten präsentieren
- Abschluss mit Hinweis auf Quellen

Antworte in einem natürlichen, hilfreichen Ton."#,
            query.original_query,
            serde_json::to_string_pretty(data).unwrap_or_else(|_| "Keine Daten verfügbar".to_string())
        );

        let response = self.query_ollama(&prompt).await?;

        Ok(AIResponse {
            message: response,
            data: data.clone(),
            sources: vec![], // TODO: Extract from actual crawl results
        })
    }

    async fn query_ollama(&self, prompt: &str) -> Result<String, AppError> {
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let url = format!("{}/api/generate", self.base_url);
        
        tracing::info!("Querying Ollama at: {}", url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::ServiceUnavailable(format!("Ollama request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::ServiceUnavailable(
                format!("Ollama returned status: {}", response.status())
            ));
        }

        let ollama_response: OllamaResponse = response
            .json()
            .await
            .map_err(|e| AppError::ServiceUnavailable(format!("Failed to parse Ollama response: {}", e)))?;

        Ok(ollama_response.response.trim().to_string())
    }

    fn fallback_parse_dno_query(&self, user_query: &str) -> DnoQueryParseResult {
        let query_lower = user_query.to_lowercase();
        
        // DNO name mapping
        let dno_mappings = [
            ("netze bw", "Netze BW", "netze-bw"),
            ("bayernwerk", "Bayernwerk", "bayernwerk"),
            ("e.dis", "E.DIS", "e-dis"),
            ("avacon", "Avacon", "avacon"),
            ("westnetz", "Westnetz", "westnetz"),
            ("stromnetz berlin", "Stromnetz Berlin", "stromnetz-berlin"),
        ];

        // Find matching DNO
        let (dno_name, dno_key) = dno_mappings
            .iter()
            .find(|(search_name, _, _)| query_lower.contains(search_name))
            .map(|(_, name, key)| (name.to_string(), key.to_string()))
            .unwrap_or(("Unknown DNO".to_string(), "unknown".to_string()));

        // Extract years (2020-2030)
        let years: Vec<i32> = (2020..=2030)
            .filter(|year| query_lower.contains(&year.to_string()))
            .collect();

        // Determine data types
        let data_types = if query_lower.contains("netzentgelt") && query_lower.contains("hlzf") {
            vec![DnoDataType::Both]
        } else if query_lower.contains("netzentgelt") {
            vec![DnoDataType::Netzentgelte]
        } else if query_lower.contains("hlzf") || query_lower.contains("tarif") {
            vec![DnoDataType::Hlzf]
        } else {
            vec![DnoDataType::Both] // Default to both if unclear
        };

        // Calculate confidence
        let mut confidence = 0.5; // Base confidence
        if dno_name != "Unknown DNO" { confidence += 0.3; }
        if !years.is_empty() { confidence += 0.2; }

        DnoQueryParseResult {
            dno_name,
            dno_key,
            years,
            data_types,
            confidence,
        }
    }

    fn fallback_parse_query(&self, user_query: &str) -> ProcessedQuery {
        let query_lower = user_query.to_lowercase();
        
        // Extract common DNO names
        let dno_names = [
            "netze bw", "avacon", "bayernwerk", "stromnetz berlin", 
            "westnetz", "e.dis", "50hertz", "amprion", "tennet", "transnetbw"
        ]
        .iter()
        .filter(|&name| query_lower.contains(name))
        .map(|s| s.to_string())
        .collect();

        // Extract years (2020-2030)
        let years: Vec<String> = (2020..=2030)
            .filter(|year| query_lower.contains(&year.to_string()))
            .map(|year| year.to_string())
            .collect();

        // Determine intent based on keywords
        let intent = if query_lower.contains("netzentgelt") || query_lower.contains("tarif") {
            QueryIntent::Tariffs
        } else if query_lower.contains("kontakt") || query_lower.contains("telefon") || query_lower.contains("email") {
            QueryIntent::ContactInfo
        } else if query_lower.contains("gebiet") || query_lower.contains("region") {
            QueryIntent::ServiceAreas
        } else if query_lower.contains("netz") || query_lower.contains("daten") || query_lower.contains("werte") {
            QueryIntent::NetworkData
        } else {
            QueryIntent::General
        };

        ProcessedQuery {
            original_query: user_query.to_string(),
            search_terms: vec![user_query.to_string()],
            dno_names,
            years,
            data_types: vec!["Allgemeine Daten".to_string()],
            intent,
        }
    }
}

impl Default for OllamaService {
    fn default() -> Self {
        Self::new("http://localhost:11434".to_string(), "llama3".to_string())
    }
}