use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Search for files on the system.
    FileSearch {
        /// The path to search for files in.
        path: Option<String>,
        /// The name of the file to search for.
        name: Option<String>,
    },
}
