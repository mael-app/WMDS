mod cli;
mod profiler;
pub mod docker;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::commands::Commands::Profile { path } => {
            profiler::run_profile(&path);
        }
    }
}
