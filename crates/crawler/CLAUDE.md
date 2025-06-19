# Crawler Crate - AI-Driven DNO Data Extraction

## Overview
The `crawler` crate provides a command-line interface for intelligent data gathering from German Distribution Network Operators (DNOs). It features AI-driven crawling with reinforcement learning, SearXNG integration, and Puppeteer-based web scraping.

## Architecture

### 🤖 AI-Driven Approach
The crawler uses artificial intelligence to:
- **Learn from Success**: Adapts strategies based on successful data extractions
- **Pattern Recognition**: Identifies URL patterns and content indicators for DNO data
- **Quality Assessment**: Evaluates extracted data quality and completeness
- **Adaptive Strategy**: Balances exploration of new methods vs. exploitation of proven ones

### 🔍 Search Integration
- **SearXNG**: Privacy-respecting search engine for discovering DNO resources
- **Query Optimization**: Smart query construction for German DNO-specific searches
- **Result Filtering**: AI-powered filtering of search results for relevance

### 🌐 Web Scraping
- **Puppeteer MCP**: Advanced web page interaction and data extraction
- **JavaScript Execution**: Dynamic content handling for modern web applications
- **Screenshot Capture**: Visual verification of extraction regions

## CLI Interface

### Commands

#### `search` - Test SearXNG Connectivity
```bash
# Basic search test
crawler search "Netze BW Netzentgelte 2024"

# JSON output for scripting
crawler search "test query" --json
```

#### `ai-gather` - AI-Driven Data Gathering
```bash
# Primary AI crawling command
crawler ai-gather "Netze BW" --storage-types "netzentgelte,hlzf" --json

# Specific year targeting
crawler ai-gather "Bayernwerk" --years 2024,2023 --data-types netzentgelte

# High priority extraction
crawler ai-gather "EnBW" --priority high --max-time 1800

# Multiple DNOs batch processing
crawler ai-gather "Netze BW,Bayernwerk,EnBW" --data-types all --json
```

### Options & Flags

#### Global Options
- `--json` - Output results in JSON format for API integration
- `--verbose` - Enable detailed logging for debugging
- `--config` - Specify custom configuration file

#### AI Gather Options
- `--storage-types` - Target data types: `netzentgelte`, `hlzf`, `all`
- `--years` - Specific years to target (default: current year)
- `--max-time` - Maximum execution time in seconds
- `--priority` - Extraction priority: `low`, `normal`, `high`
- `--dry-run` - Simulate extraction without saving data
- `--force-refresh` - Bypass cache and force fresh extraction

## AI Agent System

### 🧠 Learning Algorithm
The AI agent implements epsilon-greedy reinforcement learning:

```rust
pub struct AiAgent {
    // Exploration vs exploitation balance (0.0 to 1.0)
    epsilon: f64,
    
    // Success/failure tracking per DNO
    dno_success_rates: HashMap<String, f64>,
    
    // URL pattern effectiveness
    pattern_rewards: HashMap<String, f64>,
    
    // Strategy adaptation weights
    strategy_weights: Vec<f64>,
}

impl AiAgent {
    // Decide whether to explore new strategy or exploit known good one
    pub fn choose_strategy(&mut self, dno: &str) -> CrawlStrategy {
        if rand::random::<f64>() < self.epsilon {
            self.explore_new_strategy(dno)  // Try something new
        } else {
            self.exploit_best_strategy(dno) // Use proven method
        }
    }
    
    // Update learning based on extraction success/failure
    pub fn update_reward(&mut self, strategy: CrawlStrategy, success: bool, quality_score: f64) {
        // Reinforcement learning update
    }
}
```

### 📊 Data Quality Evaluation
The AI evaluates extracted data on multiple dimensions:

#### German Data Validation
- **Address Validation**: German postal codes, state abbreviations
- **Phone Number Format**: German phone number patterns
- **Email Domains**: Validate against known DNO domains
- **Currency Format**: Euro amounts with proper decimal handling

#### Completeness Scoring
```rust
pub struct QualityScore {
    pub overall_score: f64,        // 0.0 to 1.0
    pub completeness: f64,         // All required fields present
    pub accuracy: f64,             // Data format correctness
    pub consistency: f64,          // Internal data consistency
    pub freshness: f64,            // Data recency score
}

impl QualityScore {
    pub fn evaluate_netzentgelte(&self, data: &NetzentgelteData) -> QualityScore {
        // Comprehensive quality assessment
        // - Check for all voltage levels (HS, HS/MS, MS, MS/NS, NS)
        // - Validate price ranges (reasonable €/kW and €/kWh values)
        // - Ensure completeness of Leistung/Arbeit pairs
        // - Verify year consistency
    }
}
```

### 🎯 Strategy Patterns
The AI learns and adapts multiple extraction strategies:

#### URL Discovery Patterns
- **Official Website Crawling**: Start from DNO homepage
- **Document Portals**: Target known document download sections
- **Search Result Mining**: Leverage SearXNG for document discovery
- **Regulatory Portals**: Check energy authority databases

#### Content Extraction Methods
- **PDF Text Extraction**: OCR and text parsing for tariff documents
- **Table Recognition**: HTML table parsing for structured data
- **Form Interaction**: Automated form submission for data requests
- **API Discovery**: Attempt to find and use JSON/XML APIs

## SearXNG Integration

### 🔍 Search Strategy
```rust
pub struct SearxngClient {
    base_url: String,
    categories: Vec<String>,  // files, general, news
    engines: Vec<String>,     // bing, google, duckduckgo
}

impl SearxngClient {
    pub async fn search_dno_data(&self, dno: &str, data_type: DataType, year: i32) -> SearchResults {
        let queries = self.generate_search_queries(dno, data_type, year);
        
        for query in queries {
            let results = self.execute_search(&query).await?;
            let filtered = self.filter_relevant_results(results, dno, data_type);
            
            if !filtered.is_empty() {
                return Ok(filtered);
            }
        }
    }
    
    fn generate_search_queries(&self, dno: &str, data_type: DataType, year: i32) -> Vec<String> {
        match data_type {
            DataType::Netzentgelte => vec![
                format!("{} Netzentgelte {} filetype:pdf", dno, year),
                format!("{} Strompreise {} site:{}.de", dno, year, dno.to_lowercase()),
                format!("\"{}\" \"Entgelte\" {} PDF", dno, year),
            ],
            DataType::Hlzf => vec![
                format!("{} Hauptlastzeiten {} filetype:pdf", dno, year),
                format!("{} HLZF {} regelungen", dno, year),
                format!("\"{}\" \"Lastzeiten\" {}", dno, year),
            ],
            DataType::All => {
                // Combine strategies for comprehensive search
            }
        }
    }
}
```

### 📈 Result Scoring
Each search result is scored based on:
- **URL Relevance**: Domain matching, path analysis
- **Title Matching**: Keyword density and relevance
- **File Type**: Preference for PDF documents
- **Recency**: Publication date proximity to target year

## Puppeteer MCP Integration

### 🌐 Web Automation
```rust
pub struct PuppeteerCrawler {
    mcp_client: MCPClient,
}

impl PuppeteerCrawler {
    pub async fn extract_from_page(&self, url: &str) -> ExtractionResult {
        // Navigate to the page
        self.mcp_client.navigate(url).await?;
        
        // Wait for dynamic content
        self.wait_for_content().await?;
        
        // Take screenshot for verification
        let screenshot = self.mcp_client.screenshot("page_capture").await?;
        
        // Extract data using multiple strategies
        let strategies = vec![
            self.extract_from_tables().await,
            self.extract_from_pdfs().await,
            self.extract_from_forms().await,
        ];
        
        // Combine and validate results
        self.merge_extraction_results(strategies)
    }
    
    async fn extract_from_tables(&self) -> Vec<DataPoint> {
        // Use CSS selectors to find data tables
        let tables = self.mcp_client.evaluate(r#"
            Array.from(document.querySelectorAll('table')).map(table => ({
                headers: Array.from(table.querySelectorAll('th')).map(th => th.textContent),
                rows: Array.from(table.querySelectorAll('tr')).map(tr => 
                    Array.from(tr.querySelectorAll('td')).map(td => td.textContent)
                )
            }))
        "#).await?;
        
        // Parse tables for Netzentgelte/HLZF data
        self.parse_table_data(tables)
    }
}
```

### 📸 Visual Verification
- **Screenshot Capture**: Visual proof of extraction regions
- **Element Highlighting**: Mark extracted data sources
- **Error Documentation**: Capture failed extraction attempts

## Data Processing Pipeline

### 🔄 Processing Flow
1. **Search Phase**: Use SearXNG to discover relevant resources
2. **Filtering Phase**: AI-powered relevance filtering
3. **Extraction Phase**: Puppeteer-based content extraction
4. **Validation Phase**: German data format validation
5. **Quality Assessment**: Multi-dimensional quality scoring
6. **Storage Phase**: Database insertion with source tracking

### 🧹 Data Cleaning
```rust
pub struct DataCleaner {
    german_patterns: GermanDataPatterns,
}

impl DataCleaner {
    pub fn clean_netzentgelte(&self, raw_data: &str) -> CleanedData {
        // Remove common PDF artifacts
        let cleaned = self.remove_pdf_artifacts(raw_data);
        
        // Extract monetary values
        let prices = self.extract_euro_amounts(&cleaned);
        
        // Identify voltage levels
        let voltage_levels = self.extract_voltage_levels(&cleaned);
        
        // Structure the data
        self.structure_netzentgelte_data(prices, voltage_levels)
    }
    
    fn extract_euro_amounts(&self, text: &str) -> Vec<EuroAmount> {
        // Regex patterns for German number formats
        // "58,21 €/kW" or "1,26 €/kWh" or "58.21 EUR/kW"
        let patterns = [
            r"(\d{1,3}(?:[.,]\d{3})*[.,]\d{2})\s*€/k?W",
            r"(\d{1,3}(?:[.,]\d{3})*[.,]\d{2})\s*EUR/k?W",
            // Additional German currency patterns
        ];
        
        // Extract and validate amounts
    }
}
```

## Configuration

### 📄 Configuration Files
```toml
# crawler.toml
[searxng]
url = "http://localhost:8080"
categories = ["files", "general"]
engines = ["bing", "duckduckgo", "google"]
timeout_seconds = 30

[puppeteer]
mcp_url = "http://localhost:3001"
headless = true
timeout_seconds = 60
screenshot_quality = 90

[ai_agent]
epsilon = 0.1  # 10% exploration, 90% exploitation
learning_rate = 0.01
memory_size = 1000
model_save_interval = 100

[data_validation]
min_quality_score = 0.7
require_all_voltage_levels = true
validate_german_formats = true

[output]
json_pretty = true
include_source_info = true
include_confidence_scores = true
```

### 🔐 Environment Variables
```bash
# Required
SEARXNG_URL=http://localhost:8080
DATABASE_URL=postgresql://user:pass@localhost/dno_crawler

# Optional
PUPPETEER_MCP_URL=http://localhost:3001
CRAWLER_LOG_LEVEL=debug
CRAWLER_USER_AGENT="DNO-Crawler/1.0"
```

## Output Formats

### 📊 JSON Output Example
```json
{
  "extraction_id": "550e8400-e29b-41d4-a716-446655440000",
  "dno": {
    "name": "Netze BW",
    "slug": "netze-bw"
  },
  "year": 2024,
  "data_type": "netzentgelte",
  "status": "success",
  "quality_score": {
    "overall": 0.95,
    "completeness": 0.98,
    "accuracy": 0.92,
    "consistency": 0.96
  },
  "data": {
    "netzentgelte": {
      "hs": {
        "leistung": 58.21,
        "arbeit": 1.26,
        "leistung_unter_2500h": 2.56,
        "arbeit_unter_2500h": 7.14
      },
      "hs_ms": {
        "leistung": 79.84,
        "arbeit": 1.42
      }
    }
  },
  "source": {
    "type": "pdf",
    "url": "https://netze-bw.de/docs/netzentgelte-2024.pdf",
    "file_hash": "abc123...",
    "extraction_method": "table_recognition",
    "confidence": 0.95,
    "page_number": 12,
    "extraction_region": {
      "x": 100, "y": 200, "width": 400, "height": 300
    }
  },
  "ai_metadata": {
    "strategy_used": "pdf_table_extraction",
    "learning_iteration": 156,
    "time_to_extract_ms": 2340
  },
  "extraction_log": [
    {
      "timestamp": "2024-01-15T10:30:00Z",
      "step": "searxng_search",
      "duration_ms": 850,
      "results_found": 12
    },
    {
      "timestamp": "2024-01-15T10:30:01Z", 
      "step": "relevance_filtering",
      "duration_ms": 200,
      "candidates_remaining": 3
    }
  ]
}
```

## Testing & Validation

### 🧪 Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_searxng_integration() {
        let client = SearxngClient::new("http://localhost:8080");
        let results = client.search_dno_data("Netze BW", DataType::Netzentgelte, 2024).await;
        assert!(results.is_ok());
    }
    
    #[test]
    fn test_german_data_validation() {
        let cleaner = DataCleaner::new();
        let cleaned = cleaner.clean_netzentgelte("58,21 €/kW Leistungspreis HS");
        assert_eq!(cleaned.amount, 58.21);
        assert_eq!(cleaned.unit, "€/kW");
        assert_eq!(cleaned.voltage_level, "HS");
    }
}
```

### 🎯 Integration Tests
- End-to-end extraction workflows
- AI agent learning validation
- Quality score accuracy tests
- Multi-DNO batch processing

## Performance & Monitoring

### 📈 Metrics Collection
- Extraction success rates per DNO
- Average quality scores
- Processing time distributions
- AI learning convergence rates
- SearXNG response times

### 🔧 Optimization Strategies
- Parallel processing for multiple DNOs
- Caching of search results and extractions
- Smart retry logic with exponential backoff
- Resource usage monitoring and throttling

## Future Enhancements

### 🚀 Planned Features
- **Multi-language Support**: Extend beyond German DNOs
- **Advanced OCR**: Improved PDF text extraction
- **Machine Learning Models**: Deep learning for content recognition
- **Distributed Crawling**: Multi-node crawler deployment
- **Real-time Monitoring**: Live dashboard for extraction status

This crawler represents the cutting edge of AI-driven data extraction, combining traditional web scraping with modern machine learning techniques to provide intelligent, adaptive data gathering for the German energy sector.