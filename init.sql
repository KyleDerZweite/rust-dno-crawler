-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create custom types
CREATE TYPE user_role AS ENUM ('user', 'admin');
CREATE TYPE job_status AS ENUM ('pending', 'running', 'completed', 'failed', 'cancelled');
CREATE TYPE crawl_type AS ENUM ('file', 'table', 'api');
CREATE TYPE data_type AS ENUM ('netzentgelte', 'hlzf', 'all');
CREATE TYPE season AS ENUM ('winter', 'fruehling', 'sommer', 'herbst');

-- DNO (Distribution Network Operator) table
CREATE TABLE dnos (
                      id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                      slug VARCHAR(255) UNIQUE NOT NULL, -- e.g., 'netze-bw'
                      name VARCHAR(255) NOT NULL, -- e.g., 'Netze BW'
                      official_name VARCHAR(255), -- e.g., 'Netze BW GmbH'
                      description TEXT,
                      region VARCHAR(255),
                      website VARCHAR(500),
                      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                      updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_dnos_slug ON dnos(slug);
CREATE INDEX idx_dnos_region ON dnos(region);

-- DNO crawl configuration
CREATE TABLE dno_crawl_configs (
                                   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                                   dno_id UUID NOT NULL REFERENCES dnos(id) ON DELETE CASCADE,
                                   crawl_type crawl_type NOT NULL,
                                   netzentgelte_source_url TEXT,
                                   hlzf_source_url TEXT,
                                   netzentgelte_file_pattern TEXT,
                                   hlzf_file_pattern TEXT,
                                   auto_crawl BOOLEAN DEFAULT false,
                                   auto_crawl_interval VARCHAR(50), -- 'daily', 'weekly', 'monthly', 'yearly'
                                   auto_crawl_years INTEGER[],
                                   created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                                   updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                                   UNIQUE(dno_id)
);

-- Netzentgelte data table
CREATE TABLE netzentgelte_data (
                                   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                                   dno_id UUID NOT NULL REFERENCES dnos(id) ON DELETE CASCADE,
                                   year INTEGER NOT NULL,
                                   voltage_level VARCHAR(10) NOT NULL, -- 'hs', 'hs/ms', 'ms', 'ms/ns', 'ns'
                                   leistung DECIMAL(10, 2),
                                   arbeit DECIMAL(10, 2),
                                   leistung_unter_2500h DECIMAL(10, 2),
                                   arbeit_unter_2500h DECIMAL(10, 2),
                                   created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                                   updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                                   UNIQUE(dno_id, year, voltage_level)
);

CREATE INDEX idx_netzentgelte_dno_year ON netzentgelte_data(dno_id, year);

-- HLZF (Hauptlastzeiten) data table
CREATE TABLE hlzf_data (
                           id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                           dno_id UUID NOT NULL REFERENCES dnos(id) ON DELETE CASCADE,
                           year INTEGER NOT NULL,
                           season season NOT NULL,
                           period_number INTEGER NOT NULL CHECK (period_number BETWEEN 1 AND 4),
                           start_time TIME,
                           end_time TIME,
                           created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                           updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                           UNIQUE(dno_id, year, season, period_number)
);

CREATE INDEX idx_hlzf_dno_year ON hlzf_data(dno_id, year);

-- Data sources tracking
CREATE TABLE data_sources (
                              id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                              dno_id UUID NOT NULL REFERENCES dnos(id) ON DELETE CASCADE,
                              year INTEGER NOT NULL,
                              data_type data_type NOT NULL,
                              source_type crawl_type NOT NULL,
                              source_url TEXT,
                              file_path TEXT,
                              file_hash VARCHAR(64), -- SHA256 hash for deduplication
                              extracted_at TIMESTAMPTZ NOT NULL,
                              confidence DECIMAL(3, 2) CHECK (confidence >= 0 AND confidence <= 1),
                              page_number INTEGER,
                              created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                              UNIQUE(dno_id, year, data_type)
);

CREATE INDEX idx_data_sources_dno_year ON data_sources(dno_id, year);

-- Users table
CREATE TABLE users (
                       id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                       email VARCHAR(255) UNIQUE NOT NULL,
                       password_hash VARCHAR(255) NOT NULL,
                       name VARCHAR(255) NOT NULL,
                       role user_role DEFAULT 'user',
                       profile_picture_url VARCHAR(500),
                       is_active BOOLEAN DEFAULT true,
                       email_verified BOOLEAN DEFAULT false,
                       created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                       updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                       deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users(email);

-- User settings
CREATE TABLE user_settings (
                               user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
                               language VARCHAR(5) DEFAULT 'de',
                               timezone VARCHAR(50) DEFAULT 'Europe/Berlin',
                               email_notifications BOOLEAN DEFAULT true,
                               created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                               updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- API Keys
CREATE TABLE api_keys (
                          id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                          user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                          name VARCHAR(255) NOT NULL,
                          key_hash VARCHAR(255) NOT NULL UNIQUE,
                          masked_key VARCHAR(50) NOT NULL, -- e.g., 'dnk_live_...xyz'
                          last_used TIMESTAMPTZ,
                          expires_at TIMESTAMPTZ,
                          created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);

-- Query logs
CREATE TABLE query_logs (
                            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                            user_id UUID REFERENCES users(id) ON DELETE SET NULL,
                            query_text TEXT NOT NULL,
                            interpreted_dno VARCHAR(255),
                            interpreted_year INTEGER,
                            interpreted_data_type data_type,
                            confidence DECIMAL(3, 2),
                            status VARCHAR(50) NOT NULL, -- 'found', 'not_found', 'crawling', 'error'
                            response_time_ms INTEGER,
                            result_from_cache BOOLEAN DEFAULT false,
                            ip_address INET,
                            user_agent TEXT,
                            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_query_logs_user_id ON query_logs(user_id);
CREATE INDEX idx_query_logs_created_at ON query_logs(created_at);

-- Crawl jobs
CREATE TABLE crawl_jobs (
                            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                            user_id UUID REFERENCES users(id) ON DELETE SET NULL,
                            dno_id UUID NOT NULL REFERENCES dnos(id) ON DELETE CASCADE,
                            year INTEGER NOT NULL,
                            data_type data_type NOT NULL,
                            status job_status DEFAULT 'pending',
                            progress INTEGER DEFAULT 0 CHECK (progress >= 0 AND progress <= 100),
                            current_step VARCHAR(255),
                            error_message TEXT,
                            priority INTEGER DEFAULT 5, -- 1-10, higher = more priority
                            started_at TIMESTAMPTZ,
                            completed_at TIMESTAMPTZ,
                            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_crawl_jobs_status ON crawl_jobs(status);
CREATE INDEX idx_crawl_jobs_user_id ON crawl_jobs(user_id);
CREATE INDEX idx_crawl_jobs_dno_year ON crawl_jobs(dno_id, year);

-- Crawl job steps
CREATE TABLE crawl_job_steps (
                                 id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                                 job_id UUID NOT NULL REFERENCES crawl_jobs(id) ON DELETE CASCADE,
                                 step_name VARCHAR(255) NOT NULL,
                                 status job_status DEFAULT 'pending',
                                 started_at TIMESTAMPTZ,
                                 completed_at TIMESTAMPTZ,
                                 duration_seconds INTEGER,
                                 details JSONB,
                                 created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_crawl_job_steps_job_id ON crawl_job_steps(job_id);

-- System logs
CREATE TABLE system_logs (
                             id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                             level VARCHAR(10) NOT NULL, -- 'debug', 'info', 'warn', 'error'
                             service VARCHAR(50) NOT NULL, -- 'api', 'crawler', 'scheduler'
                             message TEXT NOT NULL,
                             context JSONB,
                             trace_id VARCHAR(255),
                             created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_system_logs_level ON system_logs(level);
CREATE INDEX idx_system_logs_service ON system_logs(service);
CREATE INDEX idx_system_logs_created_at ON system_logs(created_at);

-- Automated jobs
CREATE TABLE automated_jobs (
                                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                                name VARCHAR(255) NOT NULL UNIQUE,
                                job_type VARCHAR(50) NOT NULL, -- 'scheduled_crawl', 'cleanup', 'export'
                                schedule VARCHAR(255) NOT NULL, -- Cron expression
                                enabled BOOLEAN DEFAULT true,
                                config JSONB NOT NULL,
                                last_run TIMESTAMPTZ,
                                last_status job_status,
                                next_run TIMESTAMPTZ,
                                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create update timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply update trigger to tables with updated_at
CREATE TRIGGER update_dnos_updated_at BEFORE UPDATE ON dnos
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_dno_crawl_configs_updated_at BEFORE UPDATE ON dno_crawl_configs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_netzentgelte_data_updated_at BEFORE UPDATE ON netzentgelte_data
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_hlzf_data_updated_at BEFORE UPDATE ON hlzf_data
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_settings_updated_at BEFORE UPDATE ON user_settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_crawl_jobs_updated_at BEFORE UPDATE ON crawl_jobs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_automated_jobs_updated_at BEFORE UPDATE ON automated_jobs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert example data from the JSON
INSERT INTO dnos (slug, name, official_name, description, region) VALUES
    ('netze-bw', 'Netze BW', 'Netze BW GmbH', 'Netzbetreiber in Baden-Württemberg', 'Baden-Württemberg');

-- Get the DNO ID for further inserts
DO $$
DECLARE
dno_id UUID;
BEGIN
SELECT id INTO dno_id FROM dnos WHERE slug = 'netze-bw';

-- Insert crawl config
INSERT INTO dno_crawl_configs (
    dno_id, crawl_type,
    netzentgelte_source_url, hlzf_source_url,
    netzentgelte_file_pattern, hlzf_file_pattern,
    auto_crawl, auto_crawl_interval, auto_crawl_years
) VALUES (
             dno_id, 'file',
             'https://www.netze-bw.de/unternehmen/veroeffentlichungen#3-1',
             'https://www.netze-bw.de/unternehmen/veroeffentlichungen#3-1',
             '[Nn]etzentgelte[ -_][Ss]trom[ -_]{year}.pdf',
             '[Rr]egelungen[ -_]für[ -_]die[ -_][Nn]utzung[ -_]des[ -_][Ss]tromverteilnetzes[ -_]{year}.pdf',
             true, 'yearly', ARRAY[2024]
         );

-- Insert 2024 Netzentgelte data
INSERT INTO netzentgelte_data (dno_id, year, voltage_level, leistung, arbeit, leistung_unter_2500h, arbeit_unter_2500h) VALUES
                                                                                                                            (dno_id, 2024, 'hs', 58.21, 1.26, 2.56, 7.14),
                                                                                                                            (dno_id, 2024, 'hs/ms', 79.84, 1.42, 3.14, 8.28),
                                                                                                                            (dno_id, 2024, 'ms', 109.86, 1.73, 4.72, 10.97),
                                                                                                                            (dno_id, 2024, 'ms/ns', 142.11, 2.63, 6.24, 14.58),
                                                                                                                            (dno_id, 2024, 'ns', 169.42, 3.15, 7.05, 16.97);

-- Insert HLZF data (only winter periods from the JSON)
INSERT INTO hlzf_data (dno_id, year, season, period_number, start_time, end_time) VALUES
    (dno_id, 2024, 'winter', 1, '06:00:00', '22:00:00');

-- Insert data source information
INSERT INTO data_sources (dno_id, year, data_type, source_type, file_path, extracted_at) VALUES
                                                                                             (dno_id, 2024, 'netzentgelte', 'file', 'dno-assets/netze-bw/Netzentgelte Strom 2024.pdf', '2025-04-20T12:41:32Z'),
                                                                                             (dno_id, 2024, 'hlzf', 'file', 'dno-assets/netze-bw/Regelungen für die Nutzung des Stromverteilnetzes 2024.pdf', '2025-04-20T12:41:32Z');
END $$;