use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Search for files on the system.
    FileSearch,
}
