use std::path::Path;
use clap::Parser;
use log::{error, info};
use service::config::ConfigService;
use crate::args::{Cli, Commands};
use dotenv::dotenv;
mod cli;
mod args;
mod tasks;

pub const LOG_TARGET: &str = "indexer";
fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    info!("Starting indexer...");
    let args = Cli::parse();

    if args.command.is_none() { error!("No command provided"); return }

    let config_service = ConfigService::from_file(Path::new(args.config.unwrap().to_str().unwrap_or("config.yaml"))).unwrap();

    match args.command.unwrap() {
        Commands::Bake { product } => {
            cli::bake::start(config_service, product);
        }
    }
}
