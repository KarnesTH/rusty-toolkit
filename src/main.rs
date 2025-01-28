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
        Commands::Password { subcommand } => match subcommand {
            PasswordCommands::Generate { length } => {
                info!("Generating password with length: {:?}", length);
                let password = PasswordManager::generate_password(length)?;
                println!("Generated password: {}", password);
                info!("Generating Password successfully");
            }
            PasswordCommands::Manage { subcommand } => match subcommand {
                PasswordManagerCommands::Add { name, password } => {
                    println!("Password add functinality not implemented yet");
                    info!("Add name: {}, with password: {}", name, password);
                    todo!("Implement Password add functionality");
                }
                PasswordManagerCommands::Remove { name } => {
                    println!("Password remove functinality not implemented yet");
                    info!("Remove name: {}", name);
                    todo!("Implement Password remove functionality");
                }
                PasswordManagerCommands::List => {
                    println!("Password list functinality not implemented yet");
                    todo!("Implement Password list functionality");
                }
            },
        },
    }

    Ok(())
}
