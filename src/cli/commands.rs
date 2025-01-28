use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Search for files on the system.
    FileSearch {
        /// The path to search for files in.
        #[arg(short, long)]
        path: Option<String>,
        /// The name of the file to search for.
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Password management commands.
    Password {
        #[command(subcommand)]
        subcommand: PasswordCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum PasswordCommands {
    /// Generate a new password.
    Generate {
        /// The length of the password to generate.
        #[arg(short, long)]
        length: Option<usize>,
    },
}
