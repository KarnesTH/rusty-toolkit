mod cli;
mod utils;

pub mod prelude {
    pub use crate::cli::{Cli, Commands, FileSearch};
    pub use crate::utils::errors::FileSearchError;
}
