mod cli;
mod crawler;
mod sources;
mod extractors;

use clap::Parser;
use shared::Config;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "crawler")]
#[command(about = "DNO Data Crawler - Extract data from German Distribution Network Operators")]
struct Cli {
    #[command(subcommand)]
    command: cli::Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "crawler=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    
    match cli.command {
        cli::Commands::Search { query } => {
            info!("Searching for: {}", query);
            cli::handle_search(query).await?;
        }
        cli::Commands::Crawl { url } => {
            info!("Crawling URL: {}", url);
            cli::handle_crawl(url).await?;
        }
        cli::Commands::Batch { file } => {
            info!("Processing batch file: {}", file);
            cli::handle_batch(file).await?;
        }
        cli::Commands::Mock => {
            info!("Generating mock data");
            cli::handle_mock().await?;
        }
    }

    Ok(())
}
