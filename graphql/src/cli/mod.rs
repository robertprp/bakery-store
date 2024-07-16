pub(crate) mod start;
pub(crate) mod version;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(
    name = "pushpin",
    version = version(),
    about = "GraphQL server for bakery store."
)]
pub struct Cli {
    #[arg(long, value_name = "file", help = "Config file.")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Start,
    Version
}

