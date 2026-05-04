use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Profile a docker build
    Profile {
        /// Path to the project (where Dockerfile is located)
        path: String,
        /// Is the output must be in json format?
        #[arg(long)] json: bool,
    },
}
