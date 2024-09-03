use std::path::Path;
use clap::Parser;
use log::{error, info};
use service::config::ConfigService;
use crate::args::{Cli, Commands};
use dotenv::dotenv;
mod cli;
mod args;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    info!("Starting indexer...");
    let args = Cli::parse();

    if args.command.is_none() { error!("No command provided"); return }

    let config_service = match args.config {
        Some(path) => {
            let path_str = path.to_str().unwrap();
            let full_path = Path::new(path_str);

            ConfigService::from_file(full_path).unwrap()
        }
        None => {
            ConfigService::default()
        }
    };
    match args.command.unwrap() {
        Commands::Bake { product } => {
            info!("Starting bake for product: {}", product);
            cli::bake::start(config_service, product);
        }
    }
}
