use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Simple program to bake a product
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Bake {
        #[arg(short, long)]
        product: String,
    },
}