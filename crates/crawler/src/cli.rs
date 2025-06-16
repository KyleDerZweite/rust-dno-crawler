use clap::Subcommand;
use shared::{CrawlStatus, Dno, ContactInfo, DataSource, DataSourceType};
use serde_json::json;
use uuid::Uuid;
use std::path::Path;
use chrono::Datelike;
use crate::reverse_crawler::{ReverseCrawler, ReverseCrawlerConfig};
use crate::source_manager::SourceManager;
use crate::learning_engine::LearningEngine;

#[derive(Subcommand)]
pub enum Commands {
    /// Search for DNO data using SearXNG
    Search {
        /// Search query
        query: String,
    },
    /// Crawl a specific URL
    Crawl {
        /// URL to crawl
        url: String,
    },
    /// Process a batch of URLs from a file
    Batch {
        /// File containing URLs (one per line)
        file: String,
    },
    /// Generate mock data for testing
    Mock,
    /// Reverse crawl from successful endpoints to discover historical data
    Reverse {
        /// DNO key to reverse crawl
        dno_key: String,
        /// Known years (comma-separated, e.g., "2023,2024")
        #[arg(long)]
        years: Option<String>,
        /// Maximum depth for reverse crawling
        #[arg(long, default_value = "5")]
        max_depth: u32,
        /// Maximum crawl time in seconds
        #[arg(long, default_value = "300")]
        max_time: u64,
        /// Enable aggressive archive discovery
        #[arg(long)]
        aggressive: bool,
    },
}

pub async fn handle_search(query: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Searching for: {}", query);
    
    // TODO: Implement actual SearXNG integration
    // For now, return mock results
    let mock_results = vec![
        json!({
            "title": format!("Mock result for '{}'", query),
            "url": "https://example-dno.de/data",
            "snippet": "Mock search result snippet with relevant DNO information...",
            "source": "Mock DNO Website"
        }),
        json!({
            "title": format!("Another result for '{}'", query),
            "url": "https://another-dno.de/info",
            "snippet": "Another mock search result with DNO data...",
            "source": "Another DNO Website"
        })
    ];
    
    println!("📋 Found {} results:", mock_results.len());
    for (i, result) in mock_results.iter().enumerate() {
        println!("  {}. {}", i + 1, result["title"].as_str().unwrap());
        println!("     URL: {}", result["url"].as_str().unwrap());
        println!("     Snippet: {}", result["snippet"].as_str().unwrap());
        println!();
    }
    
    Ok(())
}

pub async fn handle_crawl(url: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("🕷️  Crawling URL: {}", url);
    
    // TODO: Implement actual crawling with reqwest + scraper
    // For now, simulate crawling
    println!("⏳ Fetching page content...");
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    println!("✅ Page crawled successfully!");
    println!("📄 Title: Mock Page Title");
    println!("🔗 Found 5 links");
    println!("📊 Extracted 3 data points");
    
    // Mock extracted data
    let extracted_data = json!({
        "title": "Mock DNO Page",
        "contact_email": "info@mock-dno.de",
        "region": "Nordrhein-Westfalen",
        "services": ["Netzanschluss", "Störungsdienst", "Zählerwesen"]
    });
    
    println!("📋 Extracted Data:");
    println!("{}", serde_json::to_string_pretty(&extracted_data)?);
    
    Ok(())
}

pub async fn handle_batch(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("📁 Processing batch file: {}", file_path);
    
    if !Path::new(&file_path).exists() {
        return Err(format!("File not found: {}", file_path).into());
    }
    
    // TODO: Implement actual file reading and batch processing
    // For now, simulate batch processing
    let mock_urls = vec![
        "https://avacon-netz.de",
        "https://bayernwerk-netz.de",
        "https://eon-netz.de",
        "https://westfalen-weser-netz.de",
    ];
    
    println!("📋 Found {} URLs to process", mock_urls.len());
    
    for (i, url) in mock_urls.iter().enumerate() {
        println!("⏳ Processing {} ({}/{})", url, i + 1, mock_urls.len());
        
        // Simulate processing time
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        println!("✅ Completed: {}", url);
    }
    
    println!("🎉 Batch processing completed!");
    
    Ok(())
}

pub async fn handle_mock() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎭 Generating mock DNO data...");
    
    let mock_dnos = vec![
        Dno {
            id: Uuid::new_v4(),
            name: "Avacon Netz GmbH".to_string(),
            region: "Niedersachsen".to_string(),
            website: Some("https://www.avacon-netz.de".to_string()),
            contact_info: Some(ContactInfo {
                email: Some("info@avacon-netz.de".to_string()),
                phone: Some("+49 5361 189-0".to_string()),
                address: None,
            }),
            data_sources: vec![
                DataSource {
                    id: Uuid::new_v4(),
                    url: "https://www.avacon-netz.de/de/netz/netzanschluss".to_string(),
                    source_type: DataSourceType::Website,
                    last_crawled: None,
                    status: CrawlStatus::Pending,
                    metadata: json!({"category": "netzanschluss"}),
                }
            ],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Dno {
            id: Uuid::new_v4(),
            name: "Bayernwerk Netz GmbH".to_string(),
            region: "Bayern".to_string(),
            website: Some("https://www.bayernwerk-netz.de".to_string()),
            contact_info: Some(ContactInfo {
                email: Some("info@bayernwerk-netz.de".to_string()),
                phone: Some("+49 941 201-0".to_string()),
                address: None,
            }),
            data_sources: vec![
                DataSource {
                    id: Uuid::new_v4(),
                    url: "https://www.bayernwerk-netz.de/de/netz/netzanschluss".to_string(),
                    source_type: DataSourceType::Website,
                    last_crawled: None,
                    status: CrawlStatus::Pending,
                    metadata: json!({"category": "netzanschluss"}),
                }
            ],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];
    
    println!("📋 Generated {} mock DNOs:", mock_dnos.len());
    for dno in &mock_dnos {
        println!("  • {} ({})", dno.name, dno.region);
        if let Some(website) = &dno.website {
            println!("    Website: {}", website);
        }
        if let Some(contact) = &dno.contact_info {
            if let Some(email) = &contact.email {
                println!("    Email: {}", email);
            }
        }
        println!("    Data sources: {}", dno.data_sources.len());
        println!();
    }
    
    // Save to JSON file
    let json_output = serde_json::to_string_pretty(&mock_dnos)?;
    tokio::fs::write("mock_dnos.json", json_output).await?;
    
    println!("💾 Mock data saved to: mock_dnos.json");
    
    Ok(())
}

pub async fn handle_reverse(
    dno_key: String,
    years: Option<String>,
    max_depth: u32,
    max_time: u64,
    aggressive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Starting reverse crawl for DNO: {}", dno_key);
    
    // Parse years if provided
    let known_years: Vec<i32> = if let Some(years_str) = years {
        years_str
            .split(',')
            .filter_map(|y| y.trim().parse::<i32>().ok())
            .collect()
    } else {
        // Default to current year if no years specified
        vec![chrono::Utc::now().year()]
    };
    
    println!("📅 Known years: {:?}", known_years);
    
    // Configure reverse crawler
    let config = ReverseCrawlerConfig {
        max_reverse_depth: max_depth,
        max_crawl_time_seconds: max_time,
        aggressive_archive_discovery: aggressive,
        ..Default::default()
    };
    
    println!("⚙️  Configuration:");
    println!("   Max depth: {}", config.max_reverse_depth);
    println!("   Max time: {}s", config.max_crawl_time_seconds);
    println!("   Aggressive discovery: {}", config.aggressive_archive_discovery);
    
    // Initialize components
    println!("🔧 Initializing crawler components...");
    
    // For now, we'll use a temporary directory for the source manager
    let temp_dir = std::env::temp_dir().join("dno-reverse-crawler");
    std::fs::create_dir_all(&temp_dir)?;
    
    let source_manager = SourceManager::new(&temp_dir)?;
    let learning_engine = LearningEngine::new();
    
    // Create reverse crawler
    let mut reverse_crawler = ReverseCrawler::new(source_manager, learning_engine, Some(config));
    
    println!("🚀 Starting reverse crawl...");
    
    // Start reverse crawl
    let result = reverse_crawler
        .discover_historical_data(&dno_key, &known_years)
        .await;
    
    match result {
        Ok(crawl_result) => {
            println!("✅ Reverse crawl completed successfully!");
            println!();
            println!("📊 Results Summary:");
            println!("   Session ID: {}", crawl_result.session_id);
            println!("   Crawl duration: {:?}", crawl_result.crawl_duration);
            println!("   URLs analyzed: {}", crawl_result.analyzed_urls.len());
            println!("   URLs discovered: {}", crawl_result.discovered_urls.len());
            println!("   Patterns learned: {}", crawl_result.learned_patterns.len());
            println!("   Temporal patterns: {}", crawl_result.temporal_patterns.len());
            println!("   Archive structures: {}", crawl_result.archive_structures.len());
            println!("   Files stored: {}", crawl_result.files_stored.len());
            println!("   HTTP requests made: {}", crawl_result.requests_made);
            println!("   Success rate: {:.2}%", crawl_result.success_rate * 100.0);
            println!("   Overall confidence: {:.2}", crawl_result.overall_confidence);
            
            // Show discovered URLs
            if !crawl_result.discovered_urls.is_empty() {
                println!();
                println!("🔗 Discovered URLs:");
                for (i, discovered_url) in crawl_result.discovered_urls.iter().take(10).enumerate() {
                    println!("   {}. {}", i + 1, discovered_url.url);
                    println!("      Method: {:?}", discovered_url.discovery_method);
                    println!("      Confidence: {:.2}", discovered_url.confidence);
                    if let Some(status) = discovered_url.status_code {
                        println!("      Status: {}", status);
                    }
                    if let Some(content_type) = &discovered_url.content_type {
                        println!("      Content-Type: {}", content_type);
                    }
                    if let Some(temporal_data) = &discovered_url.temporal_data {
                        if let Some(year) = temporal_data.year {
                            println!("      Year: {}", year);
                        }
                    }
                    println!();
                }
                
                if crawl_result.discovered_urls.len() > 10 {
                    println!("   ... and {} more URLs", crawl_result.discovered_urls.len() - 10);
                }
            }
            
            // Show learned patterns
            if !crawl_result.learned_patterns.is_empty() {
                println!();
                println!("🧠 Learned URL Patterns:");
                for (i, pattern) in crawl_result.learned_patterns.iter().take(5).enumerate() {
                    println!("   {}. Template: {}", i + 1, pattern.template);
                    println!("      Confidence: {:.2}", pattern.confidence);
                    println!("      Success rate: {:.2}", pattern.success_rate);
                    println!("      Variables: {:?}", pattern.variables.keys().collect::<Vec<_>>());
                    println!();
                }
                
                if crawl_result.learned_patterns.len() > 5 {
                    println!("   ... and {} more patterns", crawl_result.learned_patterns.len() - 5);
                }
            }
            
            // Show archive structures
            if !crawl_result.archive_structures.is_empty() {
                println!();
                println!("🗄️  Archive Structures:");
                for (i, archive) in crawl_result.archive_structures.iter().take(3).enumerate() {
                    println!("   {}. Base URL: {}", i + 1, archive.base_url);
                    println!("      Organization: {:?}", archive.temporal_organization);
                    println!("      Confidence: {:.2}", archive.confidence);
                    println!("      Directory structure: {} levels", archive.directory_structure.len());
                    println!("      File patterns: {} patterns", archive.file_patterns.len());
                    println!();
                }
                
                if crawl_result.archive_structures.len() > 3 {
                    println!("   ... and {} more structures", crawl_result.archive_structures.len() - 3);
                }
            }
            
            // Show stored files
            if !crawl_result.files_stored.is_empty() {
                println!();
                println!("💾 Files Downloaded and Stored:");
                for (i, file_path) in crawl_result.files_stored.iter().take(5).enumerate() {
                    println!("   {}. {}", i + 1, file_path);
                }
                
                if crawl_result.files_stored.len() > 5 {
                    println!("   ... and {} more files", crawl_result.files_stored.len() - 5);
                }
            }
            
            println!();
            println!("🎉 Reverse crawl completed! Check the stored files for discovered historical data.");
        }
        Err(e) => {
            println!("❌ Reverse crawl failed: {}", e);
            println!("💡 Tips:");
            println!("   - Make sure the DNO key is correct");
            println!("   - Ensure there are existing successful crawls for this DNO");
            println!("   - Try with different years or less aggressive settings");
            return Err(e.into());
        }
    }
    
    Ok(())
}