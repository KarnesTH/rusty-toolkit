use clap::Parser;
pub use commands::Commands;
pub use file_search::FileSearch;

mod commands;
mod file_search;

#[derive(Parser, Debug)]
#[clap(
    name = "rusty-toolkit",
    version = "0.1.0",
    about = "A modular CLI utility suite written in Rust for file management, password tools, downloads, and system monitoring."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}
