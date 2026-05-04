pub mod commands;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(name = "wmds")]
#[command(about = "A simple docker build profiler")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
