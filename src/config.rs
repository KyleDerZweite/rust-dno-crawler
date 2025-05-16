use std::fs;
use std::io::Write;
use std::path::Path;
use crate::data_store::DataStore;
use serde_json;
 
/*
- metadata
key, dno_name (best case array of strings), description, region

- crawl
key, type, netzentgelte_url, hlzf_url, netzentgelte_pattern, hlzf_pattern, auto_crawl, auto_crawl_increment, auto_crawl_years (best case array of integer)

- hlzf_data
key, year, update_timestamp, value_id, value

- netzentgelte_data
key, year, update_timestamp, voltage_level, value_id, value

- data_source
key, type, hlzf_url, netzentgelte_url, hlzf_file, netzentgelte_file
*/

/// Relative path to your JSON config
const CONFIG_PATH: &str = "C:/Users/leleg/RustroverProjects/rust-dno-crawler/config/data_store.json";
 
/// Load DataStore from JSON file
pub fn load_data_store() -> Result<DataStore, Box<dyn std::error::Error>> {
    let path = Path::new(CONFIG_PATH);
    let content = fs::read_to_string(path)?;
    // Until here it worked
    let ds = serde_json::from_str::<DataStore>(&content)?;
    println!("DataStore loaded: {}", serde_json::to_string_pretty(&ds)?);
    Ok(ds)
}
 
/// Save DataStore back to JSON (pretty-printed)
pub fn save_data_store(ds: &DataStore) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(CONFIG_PATH);
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let serialized = serde_json::to_string_pretty(ds)?;
    let mut file = fs::File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
