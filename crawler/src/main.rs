use crawler::crawler::*;
use tracing::{info, Level};
use tracing_subscriber;
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <website_url> [dno_name]", args[0]);
        eprintln!("Example: {} https://example-dno.com example-dno", args[0]);
        std::process::exit(1);
    }

    let website_url = &args[1];
    let dno_name = args.get(2).cloned().unwrap_or_else(|| "unknown".to_string());

    info!("Starting DNO crawler for: {}", website_url);
    info!("DNO Name: {}", dno_name);

    // Note: The actual crawling functionality would need to be implemented
    // based on the specific crawler functions from the original code
    println!("DNO Crawler CLI");
    println!("Target URL: {}", website_url);
    println!("DNO Name: {}", dno_name);
    
    // TODO: Implement the actual crawling logic here
    // This would involve calling the crawler functions and handling results
    
    Ok(())
}
