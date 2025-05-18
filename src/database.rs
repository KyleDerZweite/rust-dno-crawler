use serde::{Deserialize, Serialize};
use serde_json;
use std::path::Path;
use rusqlite::{Connection, Result, params, OptionalExtension, ffi};

const DB_FILE: &str = "assets/data.db";
const HLZF_VOLTAGE_LEVELS: [&str; 4] = ["hs/ms", "ms", "ms/ns", "ns"];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub key: String,
    pub dno_name: Vec<String>,
    pub description: String,
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Crawl {
    pub key: String,
    pub crawl_type: String, 
    pub netzentgelte_source_url: Option<String>, 
    pub hlzf_source_url: Option<String>, 
    pub netzentgelte_file_pattern: Option<String>,
    pub hlzf_file_pattern: Option<String>,
    pub auto_crawl: bool,
    pub auto_crawl_increment: bool,
    pub auto_crawl_years: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HlzfData {
    pub key: String,
    pub year: i32,
    pub update_timestamp: i64,
    pub value_id: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetzentgelteData {
    pub key: String,
    pub year: i32,
    pub update_timestamp: i64,
    pub voltage_level: String,
    pub value_id: String,
    pub value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataSourceYearly {
    pub key: String,
    pub year: i32,
    pub source_type: String,
    pub hlzf_url: Option<String>,
    pub netzentgelte_url: Option<String>,
    pub hlzf_file: Option<String>,
    pub netzentgelte_file: Option<String>,
}

fn create_tables_if_not_exist(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS metadata (
            key TEXT PRIMARY KEY,
            dno_name TEXT NOT NULL, -- JSON string for Vec<String>
            description TEXT,
            region TEXT
        );

        CREATE TABLE IF NOT EXISTS crawl (
            key TEXT PRIMARY KEY,
            crawl_type TEXT NOT NULL,
            netzentgelte_source_url TEXT,
            hlzf_source_url TEXT,
            netzentgelte_file_pattern TEXT,
            hlzf_file_pattern TEXT,
            auto_crawl INTEGER NOT NULL DEFAULT 0, -- 0 for false, 1 for true
            auto_crawl_increment INTEGER NOT NULL DEFAULT 0, -- 0 for false, 1 for true
            auto_crawl_years TEXT NOT NULL -- JSON string for Vec<i32>
        );

        CREATE TABLE IF NOT EXISTS hlzf_data (
            key TEXT NOT NULL,
            year INTEGER NOT NULL,
            update_timestamp INTEGER NOT NULL,
            value_id TEXT NOT NULL,
            value TEXT, -- Storing time strings or NULL
            PRIMARY KEY (key, year, value_id)
        );

        CREATE TABLE IF NOT EXISTS netzentgelte_data (
            key TEXT NOT NULL,
            year INTEGER NOT NULL,
            update_timestamp INTEGER NOT NULL,
            voltage_level TEXT NOT NULL,
            value_id TEXT NOT NULL,
            value REAL, -- Storing numeric values or NULL
            PRIMARY KEY (key, year, voltage_level, value_id)
        );

        -- Removed old data_source table
        -- New table for yearly data source info
        CREATE TABLE IF NOT EXISTS data_source_yearly (
            key TEXT NOT NULL,
            year INTEGER NOT NULL,
            source_type TEXT,
            hlzf_url TEXT,
            netzentgelte_url TEXT,
            hlzf_file TEXT,
            netzentgelte_file TEXT,
            PRIMARY KEY (key, year)
        );
        "#,
    )?;
    println!("Database tables checked/created.");
    Ok(())
}

pub fn initialize_database() -> Result<Connection> {
    let db_path = Path::new(DB_FILE);
    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| rusqlite::Error::SqliteFailure(
                ffi::Error::new(ffi::SQLITE_IOERR),
                Some(format!("Failed to create database directory: {}", e))
            ))?;
    }

    let conn = Connection::open(db_path)?;
    create_tables_if_not_exist(&conn)?;
    Ok(conn)
}

// --- Read/Write functions for metadata table ---
pub fn insert_metadata(conn: &Connection, metadata: &Metadata) -> Result<usize> {
    let dno_name_json = serde_json::to_string(&metadata.dno_name)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    conn.execute(
        "INSERT INTO metadata (key, dno_name, description, region) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(key) DO UPDATE SET
         dno_name=excluded.dno_name, description=excluded.description, region=excluded.region",
        params![
            metadata.key.clone(),
            dno_name_json,
            metadata.description.clone(),
            metadata.region.clone()
        ],
    )
}

pub fn get_metadata_by_key(conn: &Connection, key: &str) -> Result<Option<Metadata>> {
    let mut stmt = conn
        .prepare("SELECT key, dno_name, description, region FROM metadata WHERE key = ?1")?;
    stmt.query_row(params![key], |row| {
        let dno_name_json: String = row.get(1)?;
        let dno_name: Vec<String> = serde_json::from_str(&dno_name_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?;
        Ok(Metadata {
            key: row.get(0)?,
            dno_name,
            description: row.get(2)?,
            region: row.get(3)?,
        })
    }).optional()
}

// --- Read/Write functions for crawl table ---
pub fn insert_crawl(conn: &Connection, crawl: &Crawl) -> Result<usize> {
    let auto_crawl_years_json = serde_json::to_string(&crawl.auto_crawl_years)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    conn.execute(
        "INSERT INTO crawl (key, crawl_type, netzentgelte_source_url, hlzf_source_url, netzentgelte_file_pattern, hlzf_file_pattern, auto_crawl, auto_crawl_increment, auto_crawl_years)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
         ON CONFLICT(key) DO UPDATE SET
         crawl_type=excluded.crawl_type, netzentgelte_source_url=excluded.netzentgelte_source_url, hlzf_source_url=excluded.hlzf_source_url,
         netzentgelte_file_pattern=excluded.netzentgelte_file_pattern, hlzf_file_pattern=excluded.hlzf_file_pattern,
         auto_crawl=excluded.auto_crawl, auto_crawl_increment=excluded.auto_crawl_increment,
         auto_crawl_years=excluded.auto_crawl_years",
        params![
            crawl.key.clone(),
            crawl.crawl_type.clone(),
            crawl.netzentgelte_source_url.clone(),
            crawl.hlzf_source_url.clone(),
            crawl.netzentgelte_file_pattern.clone(),
            crawl.hlzf_file_pattern.clone(),
            if crawl.auto_crawl { 1 } else { 0 },
            if crawl.auto_crawl_increment { 1 } else { 0 },
            auto_crawl_years_json
        ],
    )
}

pub fn get_crawl_by_key(conn: &Connection, key: &str) -> Result<Option<Crawl>> {
    let mut stmt = conn
        .prepare("SELECT key, crawl_type, netzentgelte_source_url, hlzf_source_url, netzentgelte_file_pattern, hlzf_file_pattern, auto_crawl, auto_crawl_increment, auto_crawl_years FROM crawl WHERE key = ?1")?;
    stmt.query_row(params![key], |row| {
        let auto_crawl_years_json: String = row.get(8)?;
        let auto_crawl_years: Vec<i32> = serde_json::from_str(&auto_crawl_years_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(e)))?;
        let auto_crawl_int: i32 = row.get(6)?;
        let auto_crawl_increment_int: i32 = row.get(7)?;
        Ok(Crawl {
            key: row.get(0)?,
            crawl_type: row.get(1)?,
            netzentgelte_source_url: row.get(2)?,
            hlzf_source_url: row.get(3)?,
            netzentgelte_file_pattern: row.get(4)?,
            hlzf_file_pattern: row.get(5)?,
            auto_crawl: auto_crawl_int == 1,
            auto_crawl_increment: auto_crawl_increment_int == 1,
            auto_crawl_years,
        })
    }).optional()
}

// --- Read/Write functions for hlzf_data table ---
pub fn insert_hlzf_data(conn: &Connection, data: &HlzfData) -> Result<usize> {
    conn.execute(
        "INSERT INTO hlzf_data (key, year, update_timestamp, value_id, value)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(key, year, value_id) DO UPDATE SET
         update_timestamp=excluded.update_timestamp, value=excluded.value",
        params![
            data.key.clone(),
            data.year,
            data.update_timestamp,
            data.value_id.clone(),
            data.value.clone()
        ],
    )
}

pub fn get_hlzf_data(conn: &Connection, key: &str, year: i32, value_id: &str) -> Result<Option<HlzfData>> {
    let mut stmt = conn
        .prepare("SELECT key, year, update_timestamp, value_id, value FROM hlzf_data WHERE key = ?1 AND year = ?2 AND value_id = ?3")?;
    stmt.query_row(params![key, year, value_id], |row| {
        Ok(HlzfData {
            key: row.get(0)?,
            year: row.get(1)?,
            update_timestamp: row.get(2)?,
            value_id: row.get(3)?,
            value: row.get(4)?,
        })
    }).optional()
}

// --- Read/Write functions for netzentgelte_data table ---
pub fn insert_netzentgelte_data(conn: &Connection, data: &NetzentgelteData) -> Result<usize> {
    conn.execute(
        "INSERT INTO netzentgelte_data (key, year, update_timestamp, voltage_level, value_id, value)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(key, year, voltage_level, value_id) DO UPDATE SET
         update_timestamp=excluded.update_timestamp, value=excluded.value",
        params![
            data.key.clone(),
            data.year,
            data.update_timestamp,
            data.voltage_level.clone(),
            data.value_id.clone(),
            data.value
        ],
    )
}

pub fn get_netzentgelte_data(conn: &Connection, key: &str, year: i32, voltage_level: &str, value_id: &str) -> Result<Option<NetzentgelteData>> {
    let mut stmt = conn
        .prepare("SELECT key, year, update_timestamp, voltage_level, value_id, value FROM netzentgelte_data WHERE key = ?1 AND year = ?2 AND voltage_level = ?3 AND value_id = ?4")?;
    stmt.query_row(params![key, year, voltage_level, value_id], |row| {
        Ok(NetzentgelteData {
            key: row.get(0)?,
            year: row.get(1)?,
            update_timestamp: row.get(2)?,
            voltage_level: row.get(3)?,
            value_id: row.get(4)?,
            value: row.get(5)?,
        })
    }).optional()
}

// --- Read/Write functions for data_source_yearly table ---
pub fn insert_data_source_yearly(conn: &Connection, data: &DataSourceYearly) -> Result<usize> {
    conn.execute(
        "INSERT INTO data_source_yearly (key, year, source_type, hlzf_url, netzentgelte_url, hlzf_file, netzentgelte_file)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(key, year) DO UPDATE SET
         source_type=excluded.source_type, hlzf_url=excluded.hlzf_url, netzentgelte_url=excluded.netzentgelte_url,
         hlzf_file=excluded.hlzf_file, netzentgelte_file=excluded.netzentgelte_file",
        params![
            data.key.clone(),
            data.year,
            data.source_type.clone(),
            data.hlzf_url.clone(),
            data.netzentgelte_url.clone(),
            data.hlzf_file.clone(),
            data.netzentgelte_file.clone()
        ],
    )
}

pub fn get_data_source_yearly_by_key_year(conn: &Connection, key: &str, year: i32) -> Result<Option<DataSourceYearly>> {
    let mut stmt = conn
        .prepare("SELECT key, year, source_type, hlzf_url, netzentgelte_url, hlzf_file, netzentgelte_file FROM data_source_yearly WHERE key = ?1 AND year = ?2")?;
    stmt.query_row(params![key, year], |row| {
        Ok(DataSourceYearly {
            key: row.get(0)?,
            year: row.get(1)?,
            source_type: row.get(2)?,
            hlzf_url: row.get(3)?,
            netzentgelte_url: row.get(4)?,
            hlzf_file: row.get(5)?,
            netzentgelte_file: row.get(6)?,
        })
    }).optional()
}


pub(crate) fn run_database() -> Result<()> {
    println!("Initializing database...");
    let conn = initialize_database()?;
    println!("Database initialized successfully at '{}'.", DB_FILE);

    let dno_key = "netze-bw".to_string();

    // Example: Insert Metadata for "netze-bw"
    let netze_bw_metadata = Metadata {
        key: dno_key.clone(),
        dno_name: vec!["Netze BW".to_string(), "Netze BW GmbH".to_string()],
        description: "Netzbetreiber in Baden-Württemberg".to_string(),
        region: "Baden-Württemberg".to_string(),
    };
    match insert_metadata(&conn, &netze_bw_metadata) {
        Ok(changes) => println!("Inserted/Updated metadata for key '{}', changes: {}", netze_bw_metadata.key, changes),
        Err(e) => eprintln!("Failed to insert/update metadata: {:?}", e),
    }
    if let Ok(Some(md)) = get_metadata_by_key(&conn, &dno_key) {
        println!("Retrieved metadata: {:?}", md);
    }

    // Example: Insert Crawl info for "netze-bw"
    let netze_bw_crawl = Crawl {
        key: dno_key.clone(),
        crawl_type: "file".to_string(),
        netzentgelte_source_url: Some("https://www.netze-bw.de/unternehmen/veroeffentlichungen#3-1".to_string()),
        hlzf_source_url: Some("https://www.netze-bw.de/unternehmen/veroeffentlichungen#3-1".to_string()),
        netzentgelte_file_pattern: Some("[Nn]etzentgelte[ -_][Ss]trom[ -_]{year}.pdf".to_string()),
        hlzf_file_pattern: Some("[Rr]egelungen[ -_]für[ -_]die[ -_][Nn]utzung[ -_]des[ -_][Ss]tromverteilnetzes[ -_]{year}.pdf".to_string()),
        auto_crawl: true,
        auto_crawl_increment: true,
        auto_crawl_years: vec![2016,2017,2018,2019,2020,2021,2022,2023,2024,2025],
    };
    match insert_crawl(&conn, &netze_bw_crawl) {
        Ok(changes) => println!("Inserted/Updated crawl for key '{}', changes: {}", netze_bw_crawl.key, changes),
        Err(e) => eprintln!("Failed to insert/update crawl: {:?}", e),
    }
    if let Ok(Some(cr)) = get_crawl_by_key(&conn, &dno_key) {
        println!("Retrieved crawl: {:?}", cr);
    }

    // Example: Insert data for year 2024 for "netze-bw"
    let year_2024 = 2024;
    let latest_update_timestamp_2024: i64 = 1745056892; // Example timestamp for 2025-04-20T12:41:32 UTC

    // Insert HlzfData for 2024
    let hlzf_entries_2024 = vec![
        HlzfData { key: dno_key.clone(), year: year_2024, update_timestamp: latest_update_timestamp_2024, value_id: "Winter_1_Start".to_string(), value: Some("06:00:00".to_string()) },
        HlzfData { key: dno_key.clone(), year: year_2024, update_timestamp: latest_update_timestamp_2024, value_id: "Winter_1_Ende".to_string(), value: Some("22:00:00".to_string()) },
        HlzfData { key: dno_key.clone(), year: year_2024, update_timestamp: latest_update_timestamp_2024, value_id: "Sommer_1_Start".to_string(), value: None },
    ];
    for entry in hlzf_entries_2024 {
        match insert_hlzf_data(&conn, &entry) {
            Ok(changes) => println!("Inserted/Updated hlzf_data for key '{}', year {}, value_id '{}', changes: {}", entry.key, entry.year, entry.value_id, changes),
            Err(e) => eprintln!("Failed to insert/update hlzf_data: {:?}", e),
        }
    }
    if let Ok(Some(hd)) = get_hlzf_data(&conn, &dno_key, year_2024, "Winter_1_Start") {
        println!("Retrieved HLZF data: {:?}", hd);
    }

    // Insert NetzentgelteData for 2024
    let netzentgelte_hs_2024 = vec![
        NetzentgelteData { key: dno_key.clone(), year: year_2024, update_timestamp: latest_update_timestamp_2024, voltage_level: "hs".to_string(), value_id: "Leistung".to_string(), value: Some(58.21) },
        NetzentgelteData { key: dno_key.clone(), year: year_2024, update_timestamp: latest_update_timestamp_2024, voltage_level: "hs".to_string(), value_id: "Arbeit".to_string(), value: Some(1.26) },
    ];
    let netzentgelte_ms_ns_2024 = vec![
         NetzentgelteData { key: dno_key.clone(), year: year_2024, update_timestamp: latest_update_timestamp_2024, voltage_level: "ms/ns".to_string(), value_id: "Leistung".to_string(), value: Some(142.11) },
    ];

    for entry in netzentgelte_hs_2024.iter().chain(netzentgelte_ms_ns_2024.iter()) {
         match insert_netzentgelte_data(&conn, entry) {
            Ok(changes) => println!("Inserted/Updated netzentgelte_data for key '{}', year {}, vl '{}', value_id '{}', changes: {}", entry.key, entry.year, entry.voltage_level, entry.value_id, changes),
            Err(e) => eprintln!("Failed to insert/update netzentgelte_data: {:?}", e),
        }
    }
     if let Ok(Some(nd)) = get_netzentgelte_data(&conn, &dno_key, year_2024, "hs", "Leistung") {
        println!("Retrieved Netzentgelte data: {:?}", nd);
    }

    // Insert DataSourceYearly for 2024
    let data_source_2024 = DataSourceYearly {
        key: dno_key.clone(),
        year: year_2024,
        source_type: "file".to_string(),
        hlzf_url: None,
        netzentgelte_url: None,
        hlzf_file: Some("assets/netze-bw/Netzentgelte Strom 2024.pdf".to_string()), 
        netzentgelte_file: Some("assets/netze-bw/Regelungen für die Nutzung des Stromverteilnetzes 2024.pdf".to_string()),
    };
    match insert_data_source_yearly(&conn, &data_source_2024) {
        Ok(changes) => println!("Inserted/Updated data_source_yearly for key '{}', year {}, changes: {}", data_source_2024.key, data_source_2024.year, changes),
        Err(e) => eprintln!("Failed to insert/update data_source_yearly: {:?}", e),
    }
    if let Ok(Some(dsy)) = get_data_source_yearly_by_key_year(&conn, &dno_key, year_2024) {
        println!("Retrieved DataSourceYearly: {:?}", dsy);
    }

    Ok(())
}
