#![allow(non_snake_case)]

use dioxus::prelude::*;
// Import the workspace crates
use auth;
use core;
use api;
use website;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the core components
    let core_instance = core::init().await?;
    
    // Start the authentication server
    let auth_server = auth::start_server().await?;
    
    // Initialize API client
    let api_client = api::create_client()?;
    
    // Start the website - this will depend on whether you are using web, desktop, or mobile features
    #[cfg(feature = "server")]
    website::start_server().await?;
    
    #[cfg(not(feature = "server"))]
    launch(website::app::App);
    
    println!("Dno-crawler initialized successfully!");
    
    Ok(())
}
