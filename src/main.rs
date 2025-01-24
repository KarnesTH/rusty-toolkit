use clap::Parser;
use rusty_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.commands {
        Commands::FileSearch { path, name } => {
            let mut file_search = FileSearch::new(path, name)?;
            file_search.run()?;
        }
    }

    Ok(())
}
