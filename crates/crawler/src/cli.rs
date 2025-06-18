use clap::Subcommand;
use chrono::Datelike;
use crate::ai_agent::IntelligentGatheringAgent;
use crate::evaluation_engine::DataEvaluationEngine;

#[derive(Subcommand)]
pub enum Commands {
    /// AI-driven intelligent data gathering (primary method)
    AiGather {
        /// DNO name
        dno: String,
        /// Data types to gather (comma-separated: netzentgelte,hlzf,contact)
        #[arg(long, default_value = "netzentgelte")]
        data_types: String,
        /// Target years (comma-separated)
        #[arg(long)]
        years: Option<String>,
        /// Return structured JSON output
        #[arg(long)]
        json: bool,
        /// Maximum time in seconds
        #[arg(long, default_value = "120")]
        max_time: u64,
        /// Priority mode (speed, quality, completeness)
        #[arg(long, default_value = "quality")]
        priority: String,
    },
    /// Simple search for testing SearXNG connectivity
    Search {
        /// Search query
        query: String,
        /// Return structured JSON output
        #[arg(long)]
        json: bool,
    },
}

pub async fn handle_search(query: String, _json_output: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing SearXNG connectivity with query: {}", query);
    
    // Use SearXNG instance - check for environment variable or use default localhost
    let searxng_url = std::env::var("SEARXNG_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    
    // Simple connectivity test
    let client = reqwest::Client::new();
    let search_url = format!("{}/search", searxng_url);
    
    let response = client
        .get(&search_url)
        .query(&[("q", &query), ("format", &"json".to_string())])
        .send()
        .await?;
    
    if response.status().is_success() {
        let results: serde_json::Value = response.json().await?;
        println!("✅ SearXNG connectivity test successful");
        println!("📊 Found {} results", results["results"].as_array().map(|a| a.len()).unwrap_or(0));
    } else {
        println!("❌ SearXNG connectivity test failed: {}", response.status());
    }
    
    Ok(())
}

pub async fn handle_ai_gather(
    dno: String,
    data_types: String,
    years: Option<String>,
    json_output: bool,
    max_time: u64,
    priority: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if !json_output {
        println!("🤖 AI-driven data gathering for: {}", dno);
        println!("📊 Data types: {}", data_types);
        println!("⚙️  Priority: {}, Max time: {}s", priority, max_time);
    }

    // Parse data types
    let target_data_types: Vec<String> = data_types
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Parse years or use default
    let target_years: Vec<i32> = match years {
        Some(years_str) => years_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect(),
        None => {
            let current_year = chrono::Utc::now().year();
            vec![current_year - 1, current_year, current_year + 1]
        }
    };

    if !json_output {
        println!("📅 Target years: {:?}", target_years);
    }

    // Initialize AI agent
    let storage_path = format!("ai_model_{}.json", dno.to_lowercase().replace(" ", "_"));
    let mut ai_agent = IntelligentGatheringAgent::new(storage_path);

    // Execute AI-driven data gathering
    let start_time = std::time::Instant::now();
    let gathered_data = ai_agent.gather_data_intelligently(
        &dno,
        target_data_types.clone(),
        target_years.clone()
    ).await?;

    // Evaluate data quality
    let mut evaluation_engine = DataEvaluationEngine::new();
    let evaluation = evaluation_engine.evaluate_gathered_data(
        &gathered_data,
        &target_data_types,
        &dno
    ).await?;

    let processing_time = start_time.elapsed().as_secs();
    let ai_metrics = ai_agent.get_performance_metrics();

    if json_output {
        let result = serde_json::json!({
            "success": true,
            "dno": dno,
            "data_types": target_data_types,
            "target_years": target_years,
            "gathered_data": gathered_data,
            "evaluation": evaluation,
            "ai_metrics": ai_metrics,
            "processing_time_seconds": processing_time,
            "metadata": {
                "ai_engine": "intelligent_gathering_agent",
                "crawler_version": "2.0.0-ai",
                "generated_at": chrono::Utc::now()
            }
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("✅ AI data gathering completed in {}s", processing_time);
        println!("📊 Found {} data fields", gathered_data.len());
        println!("🎯 Overall evaluation score: {:.2}", evaluation.overall_score);
        println!("🤖 AI confidence: {:.2}", ai_metrics.get("average_reward").unwrap_or(&0.0));
        
        if !evaluation.recommendations.is_empty() {
            println!("\n💡 Recommendations:");
            for rec in &evaluation.recommendations {
                println!("  • {}", rec);
            }
        }
    }

    Ok(())
}