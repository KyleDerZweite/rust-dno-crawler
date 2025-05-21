mod components;
mod backend;
mod router;
pub mod app;

use dioxus::prelude::*;

#[cfg(feature = "server")]
pub async fn start_server() -> Result<()> {
    println!("Starting website server...");
    
    // Initialize the website server
    
    Ok(())
}

