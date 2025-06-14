use sqlx::SqlitePool;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::Utc;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://data.db".to_string());
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    println!("Connected to database, starting CSV import...");
    
    // Import data in order (respecting foreign key constraints)
    import_dno_metadata(&pool).await?;
    import_crawl_config(&pool).await?;
    import_data_source_yearly(&pool).await?;
    import_hlzf_data(&pool).await?;
    import_netzentgelte_data(&pool).await?;
    
    println!("CSV import completed successfully!");
    
    Ok(())
}

async fn import_dno_metadata(pool: &SqlitePool) -> Result<()> {
    println!("Importing DNO metadata...");
    
    let file = File::open("old_database_exports/metadata.csv")?;
    let reader = BufReader::new(file);
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line_num == 0 { continue; } // Skip header
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 4 {
            println!("Skipping malformed line {}: {}", line_num + 1, line);
            continue;
        }
        
        let key = parts[0];
        let dno_names_str = parts[1].trim_matches('"'); // Remove quotes
        let description = if parts[2].is_empty() { None } else { Some(parts[2]) };
        let region = if parts[3].is_empty() { None } else { Some(parts[3]) };
        
        // Parse the JSON array of DNO names
        let dno_names = if dno_names_str.starts_with('[') && dno_names_str.ends_with(']') {
            // Remove brackets and parse as JSON-like array
            let names_content = &dno_names_str[1..dno_names_str.len()-1];
            names_content
                .split("\",\"")
                .map(|s| s.trim_matches('"').to_string())
                .collect::<Vec<String>>()
        } else {
            vec![dno_names_str.to_string()]
        };
        
        let dno_names_json = serde_json::to_string(&dno_names)?;
        let now = Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO dno_metadata 
            (key, dno_names, description, region, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            key,
            dno_names_json,
            description,
            region,
            now,
            now
        )
        .execute(pool)
        .await?;
        
        println!("Imported DNO metadata for: {}", key);
    }
    
    Ok(())
}

async fn import_crawl_config(pool: &SqlitePool) -> Result<()> {
    println!("Importing crawl configuration...");
    
    let file = File::open("old_database_exports/crawl.csv")?;
    let reader = BufReader::new(file);
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line_num == 0 { continue; } // Skip header
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 9 {
            println!("Skipping malformed crawl line {}: {}", line_num + 1, line);
            continue;
        }
        
        let key = parts[0];
        let crawl_type = parts[1];
        let netzentgelte_source_url = if parts[2].is_empty() { None } else { Some(parts[2]) };
        let hlzf_source_url = if parts[3].is_empty() { None } else { Some(parts[3]) };
        let netzentgelte_file_pattern = if parts[4].is_empty() { None } else { Some(parts[4]) };
        let hlzf_file_pattern = if parts[5].is_empty() { None } else { Some(parts[5]) };
        let auto_crawl = parts[6] == "1";
        let auto_crawl_increment = parts[7] == "1";
        let auto_crawl_years_str = parts[8].trim_matches('"');
        
        let now = Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO crawl_config 
            (key, crawl_type, netzentgelte_source_url, hlzf_source_url, 
             netzentgelte_file_pattern, hlzf_file_pattern, auto_crawl, 
             auto_crawl_increment, auto_crawl_years, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            key,
            crawl_type,
            netzentgelte_source_url,
            hlzf_source_url,
            netzentgelte_file_pattern,
            hlzf_file_pattern,
            auto_crawl,
            auto_crawl_increment,
            auto_crawl_years_str,
            now,
            now
        )
        .execute(pool)
        .await?;
        
        println!("Imported crawl config for: {}", key);
    }
    
    Ok(())
}

async fn import_data_source_yearly(pool: &SqlitePool) -> Result<()> {
    println!("Importing yearly data sources...");
    
    let file = File::open("old_database_exports/data_source_yearly.csv")?;
    let reader = BufReader::new(file);
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line_num == 0 { continue; } // Skip header
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 7 {
            println!("Skipping malformed data source line {}: {}", line_num + 1, line);
            continue;
        }
        
        let key = parts[0];
        let year: i32 = parts[1].parse().unwrap_or_default();
        let source_type = if parts[2].is_empty() { None } else { Some(parts[2]) };
        let hlzf_url = if parts[3].is_empty() { None } else { Some(parts[3]) };
        let netzentgelte_url = if parts[4].is_empty() { None } else { Some(parts[4]) };
        let hlzf_file = if parts[5].is_empty() { None } else { Some(parts[5]) };
        let netzentgelte_file = if parts[6].is_empty() { None } else { Some(parts[6]) };
        
        let now = Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO data_source_yearly 
            (key, year, source_type, hlzf_url, netzentgelte_url, 
             hlzf_file, netzentgelte_file, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            key,
            year,
            source_type,
            hlzf_url,
            netzentgelte_url,
            hlzf_file,
            netzentgelte_file,
            now,
            now
        )
        .execute(pool)
        .await?;
        
        println!("Imported data source for: {} {}", key, year);
    }
    
    Ok(())
}

async fn import_hlzf_data(pool: &SqlitePool) -> Result<()> {
    println!("Importing HLZF data...");
    
    let file = File::open("old_database_exports/hlzf_data.csv")?;
    let reader = BufReader::new(file);
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line_num == 0 { continue; } // Skip header
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 5 {
            println!("Skipping malformed HLZF line {}: {}", line_num + 1, line);
            continue;
        }
        
        let key = parts[0];
        let year: i32 = parts[1].parse().unwrap_or_default();
        let update_timestamp: i64 = parts[2].parse().unwrap_or_default();
        let value_id = parts[3];
        let value = if parts[4].is_empty() { None } else { Some(parts[4]) };
        
        let now = Utc::now().to_rfc3339();
        
        // Try to find the source file from data_source_yearly table
        let source_file = sqlx::query_scalar!(
            "SELECT hlzf_file FROM data_source_yearly WHERE key = ? AND year = ?",
            key,
            year
        )
        .fetch_optional(pool)
        .await?
        .flatten();
        
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO hlzf_data 
            (key, year, update_timestamp, value_id, value, source_file, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            key,
            year,
            update_timestamp,
            value_id,
            value,
            source_file,
            now
        )
        .execute(pool)
        .await?;
    }
    
    println!("HLZF data import completed");
    Ok(())
}

async fn import_netzentgelte_data(pool: &SqlitePool) -> Result<()> {
    println!("Importing Netzentgelte data...");
    
    let file = File::open("old_database_exports/netzentgelte_data.csv")?;
    let reader = BufReader::new(file);
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line_num == 0 { continue; } // Skip header
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 6 {
            println!("Skipping malformed Netzentgelte line {}: {}", line_num + 1, line);
            continue;
        }
        
        let key = parts[0];
        let year: i32 = parts[1].parse().unwrap_or_default();
        let update_timestamp: i64 = parts[2].parse().unwrap_or_default();
        let voltage_level = parts[3];
        let value_id = parts[4];
        let value: Option<f64> = if parts[5].is_empty() { 
            None 
        } else { 
            parts[5].parse().ok() 
        };
        
        let now = Utc::now().to_rfc3339();
        
        // Try to find the source file from data_source_yearly table
        let source_file = sqlx::query_scalar!(
            "SELECT netzentgelte_file FROM data_source_yearly WHERE key = ? AND year = ?",
            key,
            year
        )
        .fetch_optional(pool)
        .await?
        .flatten();
        
        // Determine unit based on value_id (common patterns)
        let unit = match value_id {
            "Leistung" => Some("€/kW"),
            "Arbeit" => Some("ct/kWh"),
            _ => None,
        };
        
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO netzentgelte_data 
            (key, year, update_timestamp, voltage_level, value_id, value, unit, source_file, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            key,
            year,
            update_timestamp,
            voltage_level,
            value_id,
            value,
            unit,
            source_file,
            now
        )
        .execute(pool)
        .await?;
    }
    
    println!("Netzentgelte data import completed");
    Ok(())
}