#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use crate::source_manager::SourceManager;
    use crate::learning_engine::LearningEngine;
    use crate::reverse_crawler::{
        ReverseCrawler, ReverseCrawlerConfig, TemporalPatternEngine, 
        TemporalPatternType, ArchiveDiscovery
    };

    #[tokio::test]
    async fn test_reverse_crawler_creation() {
        let temp_dir = TempDir::new().unwrap();
        let source_manager = SourceManager::new(temp_dir.path()).unwrap();
        let learning_engine = LearningEngine::new();
        
        let reverse_crawler = ReverseCrawler::new(
            source_manager,
            learning_engine,
            None, // Use default config
        );
        
        assert_eq!(reverse_crawler.config.max_reverse_depth, 5);
        assert_eq!(reverse_crawler.config.max_crawl_time_seconds, 300);
        assert!(!reverse_crawler.config.aggressive_archive_discovery);
    }

    #[tokio::test]
    async fn test_temporal_pattern_engine() {
        let engine = TemporalPatternEngine::new();
        
        // Test URL with year pattern
        let test_url = "https://example.com/docs/2024/report.pdf";
        let patterns = engine.analyze_url(test_url).await.unwrap();
        
        assert!(patterns.is_some());
        let patterns = patterns.unwrap();
        assert!(!patterns.is_empty());
        
        // Should find year pattern
        assert!(patterns.iter().any(|p| matches!(p.pattern_type, TemporalPatternType::Year)));
    }

    #[tokio::test]
    async fn test_url_components_parsing() {
        let temp_dir = TempDir::new().unwrap();
        let source_manager = SourceManager::new(temp_dir.path()).unwrap();
        let learning_engine = LearningEngine::new();
        let reverse_crawler = ReverseCrawler::new(source_manager, learning_engine, None);
        
        let test_url = "https://example.com/path/to/file.pdf?year=2024";
        let components = reverse_crawler.parse_url_components(test_url).unwrap();
        
        assert_eq!(components.scheme, "https");
        assert_eq!(components.host, "example.com");
        assert_eq!(components.path, "/path/to/file.pdf");
        assert_eq!(components.query, Some("year=2024".to_string()));
    }

    #[tokio::test]
    async fn test_temporal_data_extraction() {
        let temp_dir = TempDir::new().unwrap();
        let source_manager = SourceManager::new(temp_dir.path()).unwrap();
        let learning_engine = LearningEngine::new();
        let reverse_crawler = ReverseCrawler::new(source_manager, learning_engine, None);
        
        // Test URL with year
        let test_url = "https://example.com/docs/2024/report.pdf";
        let temporal_data = reverse_crawler.extract_temporal_data_from_url(test_url);
        
        assert!(temporal_data.is_some());
        let temporal_data = temporal_data.unwrap();
        assert_eq!(temporal_data.year, Some(2024));
        
        // Test URL with year and month
        let test_url2 = "https://example.com/docs/2024/03/report.pdf";
        let temporal_data2 = reverse_crawler.extract_temporal_data_from_url(test_url2);
        
        assert!(temporal_data2.is_some());
        let temporal_data2 = temporal_data2.unwrap();
        assert_eq!(temporal_data2.year, Some(2024));
        assert_eq!(temporal_data2.month, Some(3));
    }

    #[tokio::test]
    async fn test_pattern_extraction() {
        let temp_dir = TempDir::new().unwrap();
        let source_manager = SourceManager::new(temp_dir.path()).unwrap();
        let learning_engine = LearningEngine::new();
        let mut reverse_crawler = ReverseCrawler::new(source_manager, learning_engine, None);
        
        // Test with similar URLs
        let test_urls = vec![
            "https://example.com/docs/2023/report.pdf".to_string(),
            "https://example.com/docs/2024/report.pdf".to_string(),
            "https://example.com/docs/2025/report.pdf".to_string(),
        ];
        
        let patterns = reverse_crawler.analyze_url_patterns(&test_urls, "test-dno").await.unwrap();
        
        assert!(!patterns.is_empty());
        let pattern = &patterns[0];
        assert!(pattern.template.contains("{year}") || pattern.template.contains("example.com"));
        assert_eq!(pattern.dno_key, "test-dno");
        assert!(pattern.confidence > 0.0);
    }

    #[tokio::test]
    async fn test_archive_discovery() {
        let archive_discovery = ArchiveDiscovery::new();
        
        let test_urls = vec![
            "https://example.com/archive/2023/docs/file1.pdf".to_string(),
            "https://example.com/archive/2024/docs/file2.pdf".to_string(),
            "https://example.com/archive/2024/reports/file3.pdf".to_string(),
        ];
        
        let structures = archive_discovery.analyze_urls(&test_urls, "test-dno").await.unwrap();
        
        assert!(!structures.is_empty());
        let structure = &structures[0];
        assert!(structure.base_url.contains("example.com"));
        assert_eq!(structure.dno_key, "test-dno");
        assert!(!structure.directory_structure.is_empty());
    }

    #[test]
    fn test_reverse_crawler_config() {
        let default_config = ReverseCrawlerConfig::default();
        assert_eq!(default_config.max_reverse_depth, 5);
        assert_eq!(default_config.max_crawl_time_seconds, 300);
        assert_eq!(default_config.max_urls_per_pattern, 100);
        assert_eq!(default_config.request_delay_ms, 1000);
        assert_eq!(default_config.max_concurrent_requests, 5);
        assert_eq!(default_config.historical_years_back, 10);
        assert_eq!(default_config.pattern_confidence_threshold, 0.7);
        assert!(!default_config.aggressive_archive_discovery);
        
        let custom_config = ReverseCrawlerConfig {
            max_reverse_depth: 3,
            aggressive_archive_discovery: true,
            ..Default::default()
        };
        assert_eq!(custom_config.max_reverse_depth, 3);
        assert!(custom_config.aggressive_archive_discovery);
    }
}