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
    /// Manage passwords in the password manager.
    Manage {
        #[command(subcommand)]
        subcommand: PasswordManagerCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum PasswordManagerCommands {
    /// Add a new password to the password manager.
    Add {
        /// The name of the service the password is for.
        #[arg(short, long)]
        service: Option<String>,
        /// The name of the password to add.
        #[arg(short, long)]
        username: Option<String>,
        /// The password to add.
        #[arg(short, long)]
        password: Option<String>,
        /// The URL for the service.
        #[arg(long)]
        url: Option<String>,
        /// Additional notes about the password.
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// Remove a password from the password manager.
    Remove {
        /// The name of the password to remove.
        #[arg(short, long)]
        id: Option<i32>,
    },
    /// List all passwords in the password manager.
    List,
    /// Update a password in the password manager.
    Update {
        /// The name of the password to update.
        #[arg(short, long)]
        id: Option<i32>,
        /// The name of the service the password is for.
        #[arg(short, long)]
        service: Option<String>,
        /// The name of the password to add.
        #[arg(short, long)]
        username: Option<String>,
        /// The password to add.
        #[arg(short, long)]
        password: Option<String>,
        /// The URL for the service.
        #[arg(long)]
        url: Option<String>,
        /// Additional notes about the password.
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// Show a password in the password manager.
    Show {
        /// The ID of the password to show.
        #[arg(short, long)]
        id: Option<i32>,
    },
    /// Search for a password in the password manager.
    Search {
        /// The query to search for.
        #[arg(short, long)]
        query: Option<String>,
    },
    /// Export the password manager to a file.
    Export {
        /// The path to export the password manager to.
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Import passwords from a file.
    Import {
        /// The path to import passwords from.
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Generate a import template.
    GenerateImportTemplate {
        /// The path to save the import template to.
        #[arg(short, long)]
        path: Option<String>,
    },
}
