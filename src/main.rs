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
                        info!("Adding a new password");
                        pw.add_password(service, username, password, url, notes)?;

                        println!("New Password added.");
                    }
                    PasswordManagerCommands::Remove { id } => {
                        info!("Removing a Password");
                        pw.remove_password(id)?;

                        println!("Password removed.");
                    }
                    PasswordManagerCommands::List => {
                        info!("Listing all Passwords");
                        pw.list_passwords()?;
                    }
                    PasswordManagerCommands::Update {
                        id,
                        service,
                        username,
                        password,
                        url,
                        notes,
                    } => {
                        info!("Updating a Password");
                        pw.update_password(id, service, username, password, url, notes)?;

                        println!("Password updated.");
                    }
                    PasswordManagerCommands::Show { id } => {
                        info!("Showing a Password");
                        pw.show_password(id)?;
                    }
                    PasswordManagerCommands::Search { query } => {
                        info!("Searching for a Password");
                        pw.search_password(query)?;
                    }
                    PasswordManagerCommands::Export { path } => {
                        info!("Exporting Passwords");
                        pw.export_passwords(path)?;
                    }
                    PasswordManagerCommands::Import { path } => {
                        info!("Importing Passwords");
                        pw.import_passwords(path)?;
                    }
                    PasswordManagerCommands::GenerateImportTemplate { path } => {
                        info!("Generating Import Template");
                        pw.generate_import_template(path)?;
                    }
                }
            }
        },
    }

    Ok(())
}
