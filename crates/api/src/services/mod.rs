pub mod search;
pub mod crawl;
pub mod ai;
pub mod orchestrator;
pub mod pdf;
pub mod patterns;
pub mod reverse_crawl;

pub use search::{SearchService, SearchSource};
pub use crawl::{CrawlService, CrawlResult};
pub use ai::{OllamaService, ProcessedQuery, AIResponse, QueryIntent};
pub use orchestrator::{SearchOrchestrator, IntelligentSearchRequest, IntelligentSearchResponse};
pub use pdf::PdfAnalysisService;
pub use patterns::PatternService;
pub use reverse_crawl::ReverseCrawlService;