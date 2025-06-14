-- Users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- DNOs table
CREATE TABLE IF NOT EXISTS dnos (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    region TEXT NOT NULL,
    website TEXT,
    contact_email TEXT,
    contact_phone TEXT,
    address_street TEXT,
    address_city TEXT,
    address_postal_code TEXT,
    address_country TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Data sources table
CREATE TABLE IF NOT EXISTS data_sources (
    id TEXT PRIMARY KEY,
    dno_id TEXT NOT NULL,
    url TEXT NOT NULL,
    source_type TEXT NOT NULL,
    last_crawled TEXT,
    status TEXT NOT NULL,
    metadata TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (dno_id) REFERENCES dnos(id) ON DELETE CASCADE
);

-- Search queries table
CREATE TABLE IF NOT EXISTS search_queries (
    id TEXT PRIMARY KEY,
    user_id TEXT,
    query TEXT NOT NULL,
    filters TEXT NOT NULL,
    results_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Search results table
CREATE TABLE IF NOT EXISTS search_results (
    id TEXT PRIMARY KEY,
    query_id TEXT NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    snippet TEXT NOT NULL,
    source TEXT NOT NULL,
    relevance_score REAL NOT NULL,
    found_at TEXT NOT NULL,
    FOREIGN KEY (query_id) REFERENCES search_queries(id) ON DELETE CASCADE
);

-- Crawl jobs table
CREATE TABLE IF NOT EXISTS crawl_jobs (
    id TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    error_message TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0
);

-- Crawl results table
CREATE TABLE IF NOT EXISTS crawl_results (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    content TEXT NOT NULL,
    extracted_data TEXT NOT NULL,
    links TEXT NOT NULL,
    title TEXT,
    description TEXT,
    keywords TEXT,
    language TEXT,
    last_modified TEXT,
    FOREIGN KEY (job_id) REFERENCES crawl_jobs(id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_dnos_region ON dnos(region);
CREATE INDEX IF NOT EXISTS idx_data_sources_dno_id ON data_sources(dno_id);
CREATE INDEX IF NOT EXISTS idx_data_sources_status ON data_sources(status);
CREATE INDEX IF NOT EXISTS idx_search_queries_user_id ON search_queries(user_id);
CREATE INDEX IF NOT EXISTS idx_search_results_query_id ON search_results(query_id);
CREATE INDEX IF NOT EXISTS idx_crawl_jobs_status ON crawl_jobs(status);
CREATE INDEX IF NOT EXISTS idx_crawl_results_job_id ON crawl_results(job_id);