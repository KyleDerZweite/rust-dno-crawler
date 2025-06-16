mod cli;
mod crawler;
mod crawler_orchestrator;
mod extractors;
mod learning_engine;
// mod master_orchestrator; // Temporarily disabled due to compilation issues
mod reverse_crawler;
mod source_manager;
mod sources;

use clap::Parser;
use tracing::info;
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
        cli::Commands::Reverse { dno_key, years, max_depth, max_time, aggressive } => {
            info!("Starting reverse crawl for DNO: {}", dno_key);
            cli::handle_reverse(dno_key, years, max_depth, max_time, aggressive).await?;
        }
    }

    Ok(())
}
