mod crawler;
mod ollama_client;
mod web_search;
mod database;
use std::{
    fs,
};

const ASSET_DIR: &str = "assets";

fn main() {
    fs::create_dir_all(ASSET_DIR).expect("Failed to create asset directory");
    
    // let _ = web_search::search_xng("RheinNetz GmbH (RNG)").unwrap_or_else(
    //     |err| {
    //         eprintln!("Error: {}", err);
    //         std::process::exit(1);
    //     }
    // );
    
    let _ = database::main().unwrap_or_else(
        |err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    );
    
}
