use clap::Parser;
use log::error;
use crate::args::{Cli, Commands};

mod cli;
mod args;

fn main() {
    let args = Cli::parse();

    if args.config.is_none() { error!("No config file provided"); return }
    if args.command.is_none() { error!("No command provided"); return }

    match args.command.unwrap() {
        Commands::Bake { product } => {
            println!("Baking a {}", product);
        }
    }
}
