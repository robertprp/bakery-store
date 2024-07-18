mod cli;
mod server;
mod helpers;
mod schema;
mod queries;
mod mutations;
mod subscriptions;
mod types;

use std::path::Path;
use clap::Parser;
use log::info;
use service::config::ConfigService;

static LOG_TARGET: &str = "graphql";

fn main() {
    let default_config_file_path = Path::new("config.yaml");
    dotenv::dotenv().ok().expect("Failed to load .env file");
    let cli = cli::Cli::parse();

    let config = ConfigService::from_file(cli.config.as_deref().unwrap_or(default_config_file_path)).unwrap();

    match &cli.command {
        Some(cli::Commands::Start) => {
            info!("Welcome to the bakery GraphQL server!");
            cli::start::start(config)
        }
        Some(cli::Commands::Version) => {
            cli::version::version(config)
        }
        None => {
            info!("No command provided.");
        }
    }
}
