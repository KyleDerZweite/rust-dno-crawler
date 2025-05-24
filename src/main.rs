#![allow(non_snake_case)]

// Import the workspace crates
use core;
use api;
use website;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the core components
    let core_instance = core::init().await?;
    
    // Initialize API client
    let api_client = api::create_client()?;
    
    let web_client = website::create_client().await?;

    println!("Dno-crawler initialized successfully!");
    Ok(())
}
