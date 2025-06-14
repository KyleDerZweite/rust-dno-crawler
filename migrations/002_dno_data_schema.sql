-- DNO metadata table for storing DNO information and aliases
CREATE TABLE IF NOT EXISTS dno_metadata (
    key TEXT PRIMARY KEY,
    dno_names TEXT NOT NULL, -- JSON array of DNO name variations
    description TEXT,
    region TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- HLZF (High/Low tariff time frames) data table
CREATE TABLE IF NOT EXISTS hlzf_data (
    key TEXT NOT NULL,
    year INTEGER NOT NULL,
    update_timestamp INTEGER NOT NULL,
    value_id TEXT NOT NULL, -- e.g., "Winter_1_Start", "Winter_1_Ende", "Sommer_1_Start"
    value TEXT, -- Time strings like "06:00:00" or NULL
    source_file TEXT, -- PDF source file path
    created_at TEXT NOT NULL,
    PRIMARY KEY (key, year, value_id)
);

-- Netzentgelte (Grid fees) data table
CREATE TABLE IF NOT EXISTS netzentgelte_data (
    key TEXT NOT NULL,
    year INTEGER NOT NULL,
    update_timestamp INTEGER NOT NULL,
    voltage_level TEXT NOT NULL, -- "hs", "ms", "ns", "ms_ns"
    value_id TEXT NOT NULL, -- "Leistung", "Arbeit", etc.
    value REAL, -- Numeric values or NULL
    unit TEXT, -- "€/kW", "ct/kWh", etc.
    source_file TEXT, -- PDF source file path
    created_at TEXT NOT NULL,
    PRIMARY KEY (key, year, voltage_level, value_id)
);

-- Data source yearly table for tracking PDF sources per DNO per year
CREATE TABLE IF NOT EXISTS data_source_yearly (
    key TEXT NOT NULL,
    year INTEGER NOT NULL,
    source_type TEXT, -- "file", "url", etc.
    hlzf_url TEXT,
    netzentgelte_url TEXT,
    hlzf_file TEXT,
    netzentgelte_file TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (key, year)
);

-- Crawl configuration table for auto-crawl patterns
CREATE TABLE IF NOT EXISTS crawl_config (
    key TEXT PRIMARY KEY,
    crawl_type TEXT NOT NULL,
    netzentgelte_source_url TEXT,
    hlzf_source_url TEXT,
    netzentgelte_file_pattern TEXT, -- Regex pattern for file names
    hlzf_file_pattern TEXT, -- Regex pattern for file names
    auto_crawl INTEGER NOT NULL DEFAULT 0, -- 0 for false, 1 for true
    auto_crawl_increment INTEGER NOT NULL DEFAULT 0, -- 0 for false, 1 for true
    auto_crawl_years TEXT NOT NULL, -- JSON array of years
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Query learning table for storing successful query chains
CREATE TABLE IF NOT EXISTS query_learning (
    id TEXT PRIMARY KEY,
    original_query TEXT NOT NULL,
    extracted_dno TEXT,
    extracted_years TEXT, -- JSON array of years
    search_terms TEXT, -- JSON array of search terms used
    success_rate REAL NOT NULL DEFAULT 0.0,
    execution_time_ms INTEGER,
    result_confidence REAL,
    source_files TEXT, -- JSON array of source files found
    created_at TEXT NOT NULL,
    last_used_at TEXT NOT NULL
);

-- PDF analysis results table for caching llava:7b analysis
CREATE TABLE IF NOT EXISTS pdf_analysis_results (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL UNIQUE,
    file_hash TEXT NOT NULL, -- SHA-256 hash of file content
    analysis_type TEXT NOT NULL, -- "netzentgelte", "hlzf", "general"
    model_used TEXT NOT NULL, -- "llava:7b"
    extracted_data TEXT NOT NULL, -- JSON with extracted structured data
    confidence_score REAL,
    processing_time_ms INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Query execution history for learning optimization
CREATE TABLE IF NOT EXISTS query_execution_history (
    id TEXT PRIMARY KEY,
    query_learning_id TEXT NOT NULL,
    execution_path TEXT NOT NULL, -- JSON describing the execution path taken
    cache_hits INTEGER NOT NULL DEFAULT 0,
    database_queries INTEGER NOT NULL DEFAULT 0,
    pdf_analyses INTEGER NOT NULL DEFAULT 0,
    web_searches INTEGER NOT NULL DEFAULT 0,
    total_time_ms INTEGER NOT NULL,
    success INTEGER NOT NULL, -- 0 for failure, 1 for success
    error_message TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (query_learning_id) REFERENCES query_learning(id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_dno_metadata_region ON dno_metadata(region);
CREATE INDEX IF NOT EXISTS idx_hlzf_data_key_year ON hlzf_data(key, year);
CREATE INDEX IF NOT EXISTS idx_netzentgelte_data_key_year ON netzentgelte_data(key, year);
CREATE INDEX IF NOT EXISTS idx_netzentgelte_data_voltage ON netzentgelte_data(voltage_level);
CREATE INDEX IF NOT EXISTS idx_data_source_yearly_key_year ON data_source_yearly(key, year);
CREATE INDEX IF NOT EXISTS idx_crawl_config_auto_crawl ON crawl_config(auto_crawl);
CREATE INDEX IF NOT EXISTS idx_query_learning_dno ON query_learning(extracted_dno);
CREATE INDEX IF NOT EXISTS idx_query_learning_success_rate ON query_learning(success_rate);
CREATE INDEX IF NOT EXISTS idx_pdf_analysis_file_hash ON pdf_analysis_results(file_hash);
CREATE INDEX IF NOT EXISTS idx_pdf_analysis_type ON pdf_analysis_results(analysis_type);
CREATE INDEX IF NOT EXISTS idx_query_execution_success ON query_execution_history(success);
CREATE INDEX IF NOT EXISTS idx_query_execution_time ON query_execution_history(total_time_ms);