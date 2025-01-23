use clap::Parser;
use rusty_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.commands {
        Commands::FileSearch => {
            println!("File search command not yet implemented.");
        }
    }

    Ok(())
}
