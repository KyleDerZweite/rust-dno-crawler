-- Enhanced crawler intelligence and learning system
-- This migration adds comprehensive learning, pattern recognition, and job management

-- Crawler intelligence patterns for learning system
CREATE TABLE IF NOT EXISTS crawler_intelligence (
    id TEXT PRIMARY KEY,
    dno_key TEXT NOT NULL,
    pattern_type TEXT NOT NULL, -- 'url', 'navigation', 'content', 'file_naming', 'structural'
    pattern_signature TEXT NOT NULL, -- Unique hash of the pattern
    confidence_score REAL NOT NULL DEFAULT 0.0,
    success_count INTEGER DEFAULT 0,
    failure_count INTEGER DEFAULT 0,
    avg_success_time_ms INTEGER,
    last_success_at TEXT,
    last_failure_at TEXT,
    pattern_metadata TEXT NOT NULL, -- JSON with pattern details
    admin_verified INTEGER DEFAULT 0, -- 0 = not reviewed, 1 = verified, -1 = rejected
    admin_flagged INTEGER DEFAULT 0, -- 0 = not flagged, 1 = flagged for review
    admin_notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Live crawl sessions for real-time tracking
CREATE TABLE IF NOT EXISTS live_crawl_sessions (
    session_id TEXT PRIMARY KEY,
    dno_name TEXT NOT NULL,
    dno_key TEXT NOT NULL,
    year INTEGER NOT NULL,
    status TEXT NOT NULL, -- 'queued', 'initializing', 'searching', 'crawling', 'extracting', 'completed', 'failed', 'paused'
    priority INTEGER NOT NULL DEFAULT 5, -- 1-10 scale
    progress_percentage REAL DEFAULT 0.0,
    current_phase TEXT,
    current_url TEXT,
    pages_visited INTEGER DEFAULT 0,
    files_downloaded INTEGER DEFAULT 0,
    data_extracted INTEGER DEFAULT 0,
    errors_encountered INTEGER DEFAULT 0,
    estimated_completion TEXT,
    worker_thread_id TEXT,
    parent_session_id TEXT, -- For sub-sessions
    created_by_user TEXT, -- NULL for automated jobs
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Real-time log streaming for live monitoring
CREATE TABLE IF NOT EXISTS live_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    log_level TEXT NOT NULL, -- 'trace', 'debug', 'info', 'warn', 'error'
    message TEXT NOT NULL,
    context TEXT, -- JSON with additional context
    phase TEXT,
    url TEXT,
    worker_id TEXT,
    file_path TEXT,
    FOREIGN KEY (session_id) REFERENCES live_crawl_sessions(session_id)
);

-- Crawl paths for reverse crawling and pattern learning
CREATE TABLE IF NOT EXISTS crawl_paths (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    dno_key TEXT NOT NULL,
    year INTEGER NOT NULL,
    path_sequence TEXT NOT NULL, -- JSON array of navigation steps
    success_endpoints TEXT NOT NULL, -- JSON array of successful URLs
    files_discovered TEXT NOT NULL, -- JSON array of files found
    extraction_methods TEXT NOT NULL, -- JSON array of successful extraction methods
    total_time_ms INTEGER NOT NULL,
    max_depth_reached INTEGER NOT NULL,
    success_confidence REAL NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES live_crawl_sessions(session_id)
);

-- Enhanced data sources with full provenance
CREATE TABLE IF NOT EXISTS data_sources_v2 (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    source_type TEXT NOT NULL, -- 'webpage', 'pdf', 'image', 'api', 'text'
    original_url TEXT NOT NULL,
    final_url TEXT NOT NULL, -- After redirects
    local_file_path TEXT, -- Relative path in assets/
    file_hash TEXT, -- SHA-256 for deduplication
    content_type TEXT,
    file_size INTEGER,
    extraction_confidence REAL,
    extraction_method TEXT, -- 'pdf_analysis', 'ocr', 'table_extraction', 'text_parsing'
    data_extracted TEXT, -- JSON with structured data
    crawl_path_id TEXT, -- How we found this source
    discovered_at TEXT NOT NULL,
    last_verified_at TEXT,
    is_active INTEGER DEFAULT 1,
    admin_flagged INTEGER DEFAULT 0,
    admin_verification_status TEXT DEFAULT 'pending', -- 'pending', 'verified', 'rejected'
    admin_notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES live_crawl_sessions(session_id),
    FOREIGN KEY (crawl_path_id) REFERENCES crawl_paths(id)
);

-- DNO knowledge graph for relationship mapping
CREATE TABLE IF NOT EXISTS dno_knowledge_graph (
    id TEXT PRIMARY KEY,
    dno_key TEXT NOT NULL,
    relationship_type TEXT NOT NULL, -- 'parent_company', 'subsidiary', 'partner', 'similar_structure'
    related_dno_key TEXT NOT NULL,
    confidence REAL NOT NULL,
    evidence TEXT, -- JSON with supporting evidence
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Pattern performance tracking
CREATE TABLE IF NOT EXISTS pattern_performance (
    id TEXT PRIMARY KEY,
    pattern_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    execution_time_ms INTEGER NOT NULL,
    success INTEGER NOT NULL, -- 0 = failure, 1 = success
    data_quality_score REAL, -- Quality of extracted data
    error_message TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (pattern_id) REFERENCES crawler_intelligence(id),
    FOREIGN KEY (session_id) REFERENCES live_crawl_sessions(session_id)
);

-- Job queue for orchestrated crawling
CREATE TABLE IF NOT EXISTS crawl_job_queue (
    id TEXT PRIMARY KEY,
    dno_name TEXT NOT NULL,
    dno_key TEXT NOT NULL,
    year INTEGER NOT NULL,
    priority INTEGER NOT NULL DEFAULT 5,
    job_type TEXT NOT NULL, -- 'user_request', 'automated_discovery', 'historical_backfill', 'verification'
    strategy_preference TEXT, -- JSON with preferred strategies
    constraints TEXT, -- JSON with crawl constraints
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    scheduled_for TEXT, -- When to execute (for delayed jobs)
    created_by_user TEXT, -- NULL for automated jobs
    status TEXT NOT NULL DEFAULT 'queued', -- 'queued', 'assigned', 'in_progress', 'completed', 'failed', 'cancelled'
    assigned_worker TEXT,
    session_id TEXT, -- FK to live_crawl_sessions when started
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES live_crawl_sessions(session_id)
);

-- Admin flagging and quality control
CREATE TABLE IF NOT EXISTS data_quality_flags (
    id TEXT PRIMARY KEY,
    flagged_table TEXT NOT NULL, -- Which table the flagged data is in
    flagged_record_id TEXT NOT NULL, -- ID of the flagged record
    flag_type TEXT NOT NULL, -- 'incorrect_data', 'suspicious_pattern', 'verification_needed', 'false_positive'
    severity TEXT NOT NULL, -- 'low', 'medium', 'high', 'critical'
    admin_user_id TEXT NOT NULL,
    reason TEXT NOT NULL,
    impact_analysis TEXT, -- JSON describing what patterns/data are affected
    resolution_status TEXT DEFAULT 'open', -- 'open', 'investigating', 'resolved', 'false_alarm'
    resolution_notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Website structure learning
CREATE TABLE IF NOT EXISTS website_structures (
    id TEXT PRIMARY KEY,
    dno_key TEXT NOT NULL,
    domain TEXT NOT NULL,
    structure_type TEXT NOT NULL, -- 'navigation_menu', 'archive_pattern', 'file_organization', 'url_scheme'
    structure_data TEXT NOT NULL, -- JSON with structure details
    confidence REAL NOT NULL,
    last_verified_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Automated discovery tracking
CREATE TABLE IF NOT EXISTS discovery_tracking (
    id TEXT PRIMARY KEY,
    discovery_type TEXT NOT NULL, -- 'new_dno', 'new_data_source', 'archive_discovery', 'pattern_evolution'
    discovered_entity TEXT NOT NULL, -- What was discovered
    discovery_source TEXT NOT NULL, -- How it was discovered
    confidence REAL NOT NULL,
    verification_status TEXT DEFAULT 'pending',
    metadata TEXT, -- JSON with discovery details
    created_at TEXT NOT NULL
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_crawler_intelligence_dno_key ON crawler_intelligence(dno_key);
CREATE INDEX IF NOT EXISTS idx_crawler_intelligence_pattern_type ON crawler_intelligence(pattern_type);
CREATE INDEX IF NOT EXISTS idx_crawler_intelligence_confidence ON crawler_intelligence(confidence_score);
CREATE INDEX IF NOT EXISTS idx_crawler_intelligence_admin_flagged ON crawler_intelligence(admin_flagged);

CREATE INDEX IF NOT EXISTS idx_live_crawl_sessions_status ON live_crawl_sessions(status);
CREATE INDEX IF NOT EXISTS idx_live_crawl_sessions_dno_year ON live_crawl_sessions(dno_key, year);
CREATE INDEX IF NOT EXISTS idx_live_crawl_sessions_created_at ON live_crawl_sessions(created_at);

CREATE INDEX IF NOT EXISTS idx_live_logs_session_id ON live_logs(session_id);
CREATE INDEX IF NOT EXISTS idx_live_logs_timestamp ON live_logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_live_logs_log_level ON live_logs(log_level);

CREATE INDEX IF NOT EXISTS idx_crawl_paths_dno_year ON crawl_paths(dno_key, year);
CREATE INDEX IF NOT EXISTS idx_crawl_paths_session_id ON crawl_paths(session_id);

CREATE INDEX IF NOT EXISTS idx_data_sources_v2_session_id ON data_sources_v2(session_id);
CREATE INDEX IF NOT EXISTS idx_data_sources_v2_file_hash ON data_sources_v2(file_hash);
CREATE INDEX IF NOT EXISTS idx_data_sources_v2_admin_flagged ON data_sources_v2(admin_flagged);

CREATE INDEX IF NOT EXISTS idx_crawl_job_queue_status ON crawl_job_queue(status);
CREATE INDEX IF NOT EXISTS idx_crawl_job_queue_priority ON crawl_job_queue(priority);
CREATE INDEX IF NOT EXISTS idx_crawl_job_queue_scheduled_for ON crawl_job_queue(scheduled_for);
CREATE INDEX IF NOT EXISTS idx_crawl_job_queue_dno_year ON crawl_job_queue(dno_key, year);

CREATE INDEX IF NOT EXISTS idx_data_quality_flags_flagged_table_record ON data_quality_flags(flagged_table, flagged_record_id);
CREATE INDEX IF NOT EXISTS idx_data_quality_flags_resolution_status ON data_quality_flags(resolution_status);

CREATE INDEX IF NOT EXISTS idx_website_structures_dno_key ON website_structures(dno_key);
CREATE INDEX IF NOT EXISTS idx_website_structures_domain ON website_structures(domain);

CREATE INDEX IF NOT EXISTS idx_discovery_tracking_type ON discovery_tracking(discovery_type);
CREATE INDEX IF NOT EXISTS idx_discovery_tracking_verification ON discovery_tracking(verification_status);