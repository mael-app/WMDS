use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Profile a docker build
    Profile {
        /// Path to the project (where Dockerfile is located)
        path: String,
    },
}
