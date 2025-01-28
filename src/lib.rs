mod cli;
mod utils;

pub mod prelude {
    pub use crate::cli::{
        Cli, Commands, FileSearch, PasswordCommands, PasswordManager, PasswordManagerCommands,
    };
    pub use crate::utils::config::Config;
    pub use crate::utils::errors::FileSearchError;
}
