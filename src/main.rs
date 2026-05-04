mod cli;
pub mod docker;
mod output;
pub mod parser;
mod profiler;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::commands::Commands::Profile { path , json} => {
            let result = profiler::run_profile(path);

            output::print(result, json);
        }
    }
}
