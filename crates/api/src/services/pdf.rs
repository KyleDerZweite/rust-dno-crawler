use shared::{
    PdfAnalysisRequest, PdfAnalysisResponse, PdfAnalysisResult, PdfAnalysisType, AppError,
};
use sqlx::SqlitePool;
use std::path::Path;
use tokio::fs;
use uuid::Uuid;
use chrono::Utc;
use sha2::{Sha256, Digest};
use std::process::Command;

#[derive(Clone)]
pub struct PdfAnalysisService {
    db: SqlitePool,
    ollama_url: String,
}

impl PdfAnalysisService {
    pub fn new(db: SqlitePool, ollama_url: String) -> Self {
        Self { db, ollama_url }
    }

    pub async fn analyze_pdf(&self, request: PdfAnalysisRequest) -> Result<PdfAnalysisResponse, AppError> {
        let start_time = std::time::Instant::now();
        
        // Check if file exists
        if !Path::new(&request.file_path).exists() {
            return Err(AppError::NotFound("PDF file not found".to_string()));
        }

        // Calculate file hash
        let file_hash = self.calculate_file_hash(&request.file_path).await?;

        // Check cache unless force reanalyze is requested
        if !request.force_reanalyze.unwrap_or(false) {
            if let Ok(cached_result) = self.get_cached_analysis(&file_hash, &request.analysis_type).await {
                let processing_time = start_time.elapsed().as_millis() as i32;
                return Ok(PdfAnalysisResponse {
                    id: cached_result.id,
                    extracted_data: cached_result.extracted_data,
                    confidence_score: cached_result.confidence_score,
                    processing_time_ms: processing_time,
                    was_cached: true,
                    model_used: cached_result.model_used,
                });
            }
        }

        // Perform new analysis
        let extracted_data = self.perform_llava_analysis(&request.file_path, &request.analysis_type).await?;
        let processing_time = start_time.elapsed().as_millis() as i32;

        // Determine confidence score based on analysis type and extracted data
        let confidence_score = self.calculate_confidence_score(&extracted_data, &request.analysis_type);

        // Store result in cache
        let result_id = Uuid::new_v4();
        let now = Utc::now();
        
        let analysis_result = PdfAnalysisResult {
            id: result_id,
            file_path: request.file_path.clone(),
            file_hash: file_hash.clone(),
            analysis_type: request.analysis_type,
            model_used: "llava:7b".to_string(),
            extracted_data: extracted_data.clone(),
            confidence_score: Some(confidence_score),
            processing_time_ms: Some(processing_time),
            created_at: now,
            updated_at: now,
        };

        self.cache_analysis_result(&analysis_result).await?;

        Ok(PdfAnalysisResponse {
            id: result_id,
            extracted_data,
            confidence_score: Some(confidence_score),
            processing_time_ms: processing_time,
            was_cached: false,
            model_used: "llava:7b".to_string(),
        })
    }

    async fn calculate_file_hash(&self, file_path: &str) -> Result<String, AppError> {
        let contents = fs::read(file_path).await
            .map_err(|e| AppError::Internal(format!("Failed to read file: {}", e)))?;
        
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        let hash = format!("{:x}", hasher.finalize());
        Ok(hash)
    }

    async fn get_cached_analysis(&self, file_hash: &str, analysis_type: &PdfAnalysisType) -> Result<PdfAnalysisResult, AppError> {
        let analysis_type_str = match analysis_type {
            PdfAnalysisType::Netzentgelte => "netzentgelte",
            PdfAnalysisType::Hlzf => "hlzf",
            PdfAnalysisType::General => "general",
        };

        let row = sqlx::query!(
            r#"
            SELECT id, file_path, file_hash, analysis_type, model_used, 
                   extracted_data, confidence_score, processing_time_ms,
                   created_at, updated_at
            FROM pdf_analysis_results 
            WHERE file_hash = ? AND analysis_type = ?
            "#,
            file_hash,
            analysis_type_str
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| AppError::NotFound("Cached analysis not found".to_string()))?;

        let analysis_type = match row.analysis_type.as_str() {
            "netzentgelte" => PdfAnalysisType::Netzentgelte,
            "hlzf" => PdfAnalysisType::Hlzf,
            "general" => PdfAnalysisType::General,
            _ => return Err(AppError::Internal("Invalid analysis type in database".to_string())),
        };

        let extracted_data: serde_json::Value = serde_json::from_str(&row.extracted_data)
            .map_err(|e| AppError::Internal(format!("Failed to parse extracted data: {}", e)))?;

        Ok(PdfAnalysisResult {
            id: Uuid::parse_str(&row.id)
                .map_err(|e| AppError::Internal(format!("Invalid UUID in database: {}", e)))?,
            file_path: row.file_path,
            file_hash: row.file_hash,
            analysis_type,
            model_used: row.model_used,
            extracted_data,
            confidence_score: row.confidence_score,
            processing_time_ms: row.processing_time_ms.map(|x| x as i32),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| AppError::Internal(format!("Invalid datetime in database: {}", e)))?
                .with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map_err(|e| AppError::Internal(format!("Invalid datetime in database: {}", e)))?
                .with_timezone(&Utc),
        })
    }

    async fn perform_llava_analysis(&self, file_path: &str, analysis_type: &PdfAnalysisType) -> Result<serde_json::Value, AppError> {
        // Convert PDF to images first (since llava works with images)
        let image_path = self.convert_pdf_to_image(file_path).await?;

        // Prepare prompt based on analysis type
        let prompt = self.get_analysis_prompt(analysis_type);

        // Call Ollama with llava:7b model
        let client = reqwest::Client::new();
        let request_body = serde_json::json!({
            "model": "llava:7b",
            "prompt": prompt,
            "images": [self.encode_image_base64(&image_path).await?],
            "stream": false
        });

        let response = client
            .post(&format!("{}/api/generate", self.ollama_url))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Failed to call Ollama: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::External(format!("Ollama returned error: {}", response.status())));
        }

        let ollama_response: serde_json::Value = response.json().await
            .map_err(|e| AppError::External(format!("Failed to parse Ollama response: {}", e)))?;

        // Extract the response text and parse it
        let response_text = ollama_response["response"]
            .as_str()
            .ok_or_else(|| AppError::External("No response text from Ollama".to_string()))?;

        // Parse the structured response
        let extracted_data = self.parse_llava_response(response_text, analysis_type)?;

        // Clean up temporary image file
        let _ = fs::remove_file(&image_path).await;

        Ok(extracted_data)
    }

    async fn convert_pdf_to_image(&self, pdf_path: &str) -> Result<String, AppError> {
        let output_path = format!("/tmp/pdf_page_{}_{}.png", 
            Uuid::new_v4().to_string(),
            std::process::id());

        // Use pdftoppm to convert first page to PNG
        let output = Command::new("pdftoppm")
            .args(&[
                "-f", "1", // First page only
                "-l", "1", // Last page (same as first)
                "-png",    // PNG format
                pdf_path,
                &output_path[..output_path.len()-4] // Remove .png extension as pdftoppm adds it
            ])
            .output()
            .map_err(|e| AppError::Internal(format!("Failed to execute pdftoppm: {}. Make sure poppler-utils is installed.", e)))?;

        if !output.status.success() {
            return Err(AppError::Internal(format!("pdftoppm failed: {}", 
                String::from_utf8_lossy(&output.stderr))));
        }

        // pdftoppm adds -1.png suffix for single page
        let actual_output_path = format!("{}-1.png", &output_path[..output_path.len()-4]);
        
        if !Path::new(&actual_output_path).exists() {
            return Err(AppError::Internal("PDF to image conversion failed".to_string()));
        }

        Ok(actual_output_path)
    }

    async fn encode_image_base64(&self, image_path: &str) -> Result<String, AppError> {
        let image_data = fs::read(image_path).await
            .map_err(|e| AppError::Internal(format!("Failed to read image: {}", e)))?;
        
        use base64::Engine;
        Ok(base64::engine::general_purpose::STANDARD.encode(image_data))
    }

    fn get_analysis_prompt(&self, analysis_type: &PdfAnalysisType) -> String {
        match analysis_type {
            PdfAnalysisType::Netzentgelte => {
                r#"Analyze this German electrical grid tariff document (Netzentgelte). 
Extract all numerical values for different voltage levels (Hochspannung/HS, Mittelspannung/MS, Niederspannung/NS).
Look for values labeled as "Leistung" (power) and "Arbeit" (energy).
Return the data in this exact JSON format:
{
  "voltage_levels": {
    "hs": {"Leistung": <value>, "Arbeit": <value>},
    "ms": {"Leistung": <value>, "Arbeit": <value>},
    "ns": {"Leistung": <value>, "Arbeit": <value>},
    "ms_ns": {"Leistung": <value>, "Arbeit": <value>}
  },
  "units": {
    "Leistung": "€/kW",
    "Arbeit": "ct/kWh"
  },
  "year": <extracted_year>
}
Only include voltage levels that are present in the document. Use null for missing values."#.to_string()
            },
            PdfAnalysisType::Hlzf => {
                r#"Analyze this German electrical grid document for HLZF (High/Low Tariff Time Frames).
Look for time periods for Winter and Summer seasons, specifically start and end times.
Return the data in this exact JSON format:
{
  "time_frames": {
    "Winter_1_Start": "HH:MM:SS",
    "Winter_1_Ende": "HH:MM:SS",
    "Winter_2_Start": "HH:MM:SS",
    "Winter_2_Ende": "HH:MM:SS",
    "Sommer_1_Start": "HH:MM:SS",
    "Sommer_1_Ende": "HH:MM:SS",
    "Sommer_2_Start": "HH:MM:SS",
    "Sommer_2_Ende": "HH:MM:SS"
  },
  "year": <extracted_year>
}
Use null for time frames that are not defined in the document."#.to_string()
            },
            PdfAnalysisType::General => {
                r#"Analyze this document and extract any relevant information about German electrical grid data.
Look for DNO names, years, tariff information, time frames, or any other structured data.
Return the data in JSON format with appropriate structure based on the content found."#.to_string()
            }
        }
    }

    fn parse_llava_response(&self, response_text: &str, analysis_type: &PdfAnalysisType) -> Result<serde_json::Value, AppError> {
        // Try to extract JSON from the response text
        let json_start = response_text.find('{');
        let json_end = response_text.rfind('}');

        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &response_text[start..=end];
            match serde_json::from_str::<serde_json::Value>(json_str) {
                Ok(data) => Ok(data),
                Err(_) => {
                    // Fallback: create a simple structure with the raw response
                    Ok(serde_json::json!({
                        "raw_response": response_text,
                        "analysis_type": match analysis_type {
                            PdfAnalysisType::Netzentgelte => "netzentgelte",
                            PdfAnalysisType::Hlzf => "hlzf",
                            PdfAnalysisType::General => "general",
                        },
                        "parsed": false
                    }))
                }
            }
        } else {
            // No JSON found, return raw response
            Ok(serde_json::json!({
                "raw_response": response_text,
                "analysis_type": match analysis_type {
                    PdfAnalysisType::Netzentgelte => "netzentgelte",
                    PdfAnalysisType::Hlzf => "hlzf",
                    PdfAnalysisType::General => "general",
                },
                "parsed": false
            }))
        }
    }

    fn calculate_confidence_score(&self, extracted_data: &serde_json::Value, analysis_type: &PdfAnalysisType) -> f64 {
        // Simple confidence scoring based on the presence of expected fields
        match analysis_type {
            PdfAnalysisType::Netzentgelte => {
                if let Some(voltage_levels) = extracted_data.get("voltage_levels") {
                    let mut score = 0.0;
                    let mut total_fields = 0;
                    
                    for (_, level_data) in voltage_levels.as_object().unwrap_or(&serde_json::Map::new()) {
                        if let Some(level_obj) = level_data.as_object() {
                            total_fields += 2; // Leistung and Arbeit
                            if level_obj.get("Leistung").is_some() { score += 1.0; }
                            if level_obj.get("Arbeit").is_some() { score += 1.0; }
                        }
                    }
                    
                    if total_fields > 0 { score / total_fields as f64 } else { 0.0 }
                } else {
                    0.0
                }
            },
            PdfAnalysisType::Hlzf => {
                if let Some(time_frames) = extracted_data.get("time_frames") {
                    let expected_fields = ["Winter_1_Start", "Winter_1_Ende", "Sommer_1_Start", "Sommer_1_Ende"];
                    let mut found = 0;
                    
                    for field in &expected_fields {
                        if time_frames.get(field).is_some() {
                            found += 1;
                        }
                    }
                    
                    found as f64 / expected_fields.len() as f64
                } else {
                    0.0
                }
            },
            PdfAnalysisType::General => {
                // For general analysis, any structured data gets a baseline score
                if extracted_data.get("parsed").and_then(|v| v.as_bool()).unwrap_or(false) {
                    0.7
                } else {
                    0.3
                }
            }
        }
    }

    async fn cache_analysis_result(&self, result: &PdfAnalysisResult) -> Result<(), AppError> {
        let analysis_type_str = match result.analysis_type {
            PdfAnalysisType::Netzentgelte => "netzentgelte",
            PdfAnalysisType::Hlzf => "hlzf",
            PdfAnalysisType::General => "general",
        };

        let extracted_data_str = serde_json::to_string(&result.extracted_data)
            .map_err(|e| AppError::Internal(format!("Failed to serialize extracted data: {}", e)))?;

        let id_str = result.id.to_string();
        let created_at_str = result.created_at.to_rfc3339();
        let updated_at_str = result.updated_at.to_rfc3339();

        sqlx::query!(
            r#"
            INSERT INTO pdf_analysis_results 
            (id, file_path, file_hash, analysis_type, model_used, extracted_data, 
             confidence_score, processing_time_ms, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id_str,
            result.file_path,
            result.file_hash,
            analysis_type_str,
            result.model_used,
            extracted_data_str,
            result.confidence_score,
            result.processing_time_ms,
            created_at_str,
            updated_at_str
        )
        .execute(&self.db)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }
}