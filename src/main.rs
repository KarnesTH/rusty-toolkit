use clap::Parser;
use log::info;
use rusty_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    config.setup_logger()?;

    info!("Starting rusty-toolkit...");
    let cli = Cli::parse();

    match cli.commands {
        Commands::FileSearch { path, name } => {
            info!(
                "Starting file search with path: {:?}, name: {:?}",
                path, name
            );
            let mut file_search = FileSearch::new(path, name)?;
            file_search.run()?;
        }
    }

    Ok(())
}
