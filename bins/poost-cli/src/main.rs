use anyhow::{Context, Result};
use clap::Parser;
use poost_core::config::PoostConfig;
use poost_server::{app_state::AppState, server::run_server};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "poost-cli")]
#[command(author, version, about = "Poost Server CLI", long_about = None)]
struct Args {
    /// Path to the YAML configuration file
    #[arg(short, long, default_value = "poost-config.yaml")]
    config: PathBuf,
}

fn load_config(path: &PathBuf) -> Result<PoostConfig> {
    let config_content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let config: PoostConfig = serde_yaml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

    Ok(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Parse CLI arguments
    let args = Args::parse();

    tracing::info!("Loading config from: {}", args.config.display());

    // Load and parse the config
    let config = load_config(&args.config)?;

    tracing::info!("Server URL: {}", config.server_url);
    tracing::info!(
        "Program instances to load: {}",
        config.program_instances.len()
    );

    // Initialize the app state with the config
    tracing::info!("Initializing application state...");
    let app_state = AppState::init(&config).await;

    tracing::info!("Starting Poost server at {}", config.server_url);

    // Run the server
    run_server(&config, app_state).await?;

    Ok(())
}
