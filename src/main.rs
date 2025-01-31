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
            PasswordCommands::Manage { subcommand } => {
                let pw = PasswordManager::new()?;

                match subcommand {
                    PasswordManagerCommands::Add {
                        service,
                        username,
                        password,
                        url,
                        notes,
                    } => {
                        pw.add_password(service, username, password, url, notes)?;

                        println!("New Password added.");
                    }
                    PasswordManagerCommands::Remove { id } => {
                        pw.remove_password(id)?;

                        println!("Password removed.");
                    }
                    PasswordManagerCommands::List => {
                        pw.list_passwords()?;
                    }
                }
            }
        },
    }

    Ok(())
}
