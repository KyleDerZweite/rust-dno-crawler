mod cli;
mod ai_agent;
mod evaluation_engine;
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
        cli::Commands::Search { query, json } => {
            info!("Testing SearXNG connectivity with query: {}", query);
            cli::handle_search(query, json).await?;
        }
        cli::Commands::AiGather { dno, data_types, years, json, max_time, priority } => {
            info!("AI-driven data gathering for DNO: {}", dno);
            cli::handle_ai_gather(dno, data_types, years, json, max_time, priority).await?;
        }
    }

    Ok(())
}
