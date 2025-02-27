use clap::Parser;
pub use commands::{Commands, PasswordCommands, PasswordManagerCommands};
pub use file_search::FileSearch;
pub use password::PasswordManager;

mod commands;
mod file_search;
mod password;

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
