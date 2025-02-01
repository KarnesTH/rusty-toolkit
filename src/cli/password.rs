use std::collections::HashMap;

use csv::Writer;
use inquire::{validator::Validation, Confirm, Password, Text};
use log::info;
use ring::rand::{SecureRandom, SystemRandom};
use serde::Serialize;

use crate::prelude::{Config, Database, Encryption, PasswordEntry};

#[derive(Debug)]
pub struct PasswordManager {
    pub length: usize,
    pub database: Database,
    pub encryption: Encryption,
}

#[derive(Serialize, Debug)]
struct PasswordExport {
    service: String,
    username: String,
    password: String,
    url: String,
    notes: String,
    created_at: String,
    updated_at: String,
}

impl PasswordManager {
    /// Create a new `PasswordManager` instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `PasswordManager` instance or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the master password is invalid.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load()?;
        let config_dir = Config::get_config_dir()?;
        let master_file = config_dir.join("master.key");

        let (salt, master_password) = if !master_file.exists() {
            let rng = SystemRandom::new();
            let mut salt = [0u8; 16];
            rng.fill(&mut salt).unwrap();

            let password = if Confirm::new("Do you want to generate a password? ")
                .with_default(true)
                .prompt()?
            {
                Self::generate_password(Some(16))?
            } else {
                Password::new("Please enter your master password:").prompt()?
            };

            println!(
                "The master password is: {}. Please take it secure!",
                password
            );

            let encryption = Encryption::new(&password, &salt);
            let verification_data = encryption.encrypt(&password).unwrap();

            let mut file_content = Vec::new();
            file_content.extend_from_slice(&salt);
            file_content.extend_from_slice(&verification_data);
            std::fs::write(&master_file, file_content)?;

            (salt, password)
        } else {
            let file_content = std::fs::read(&master_file)?;
            let salt: [u8; 16] = file_content[..16].try_into()?;
            let verification_data = &file_content[16..];

            let password = Password::new("Please enter your master password:")
                .without_confirmation()
                .prompt()?;

            let encryption = Encryption::new(&password, &salt);

            if let Ok(decrypted) = encryption.decrypt(verification_data) {
                if decrypted != password {
                    return Err("Invalid master password".into());
                }
            } else {
                return Err("Invalid master password".into());
            }

            (salt, password)
        };

        Ok(Self {
            length: 16,
            database: Database::new(config.get_db_path()?, &master_password, &salt)?,
            encryption: Encryption::new(&master_password, &salt),
        })
    }

    /// Generate a new password.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the password to generate.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated password as a `String`.
    ///
    /// # Errors
    ///
    /// An error will be returned if the password cannot be generated.
    pub fn generate_password(length: Option<usize>) -> Result<String, Box<dyn std::error::Error>> {
        let charset: &[u8] =
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+";

        let password_length = if let Some(length) = length {
            if !Self::is_valid_password_length(&length.to_string()) {
                return Err("Invalid password length".into());
            }
            length
        } else {
            info!("promts the user to input a password length");
            let validator = |input: &str| {
                if let Ok(length) = input.parse::<usize>() {
                    if (8..=64).contains(&length) {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid(
                            "Password length must be greater than 8 and less than 64".into(),
                        ))
                    }
                } else {
                    Ok(Validation::Invalid("Invalid password length".into()))
                }
            };
            let length = Text::new("Please enter your password length:")
                .with_validator(validator)
                .prompt()?;
            if let Ok(length) = length.parse::<usize>() {
                length
            } else {
                return Err("Invalid password length".into());
            }
        };

        let rng = SystemRandom::new();
        let mut password = String::with_capacity(password_length);

        for _ in 0..password_length {
            let mut byte = [0u8; 1];
            rng.fill(&mut byte).unwrap();
            let index = byte[0] as usize % charset.len();
            password.push(charset[index] as char);
        }

        if Self::is_valid_password(password.as_str()) {
            Ok(password)
        } else {
            Self::generate_password(length)
        }
    }

    /// Check if the password is valid.
    ///
    /// # Arguments
    ///
    /// * `password` - The password to check.
    ///
    /// # Returns
    ///
    /// A `bool` indicating if the password is valid.
    fn is_valid_password(password: &str) -> bool {
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;
        let mut has_special = false;

        for c in password.chars() {
            if c.is_lowercase() {
                has_lower = true;
            } else if c.is_uppercase() {
                has_upper = true;
            } else if c.is_ascii_digit() {
                has_digit = true;
            } else {
                has_special = true;
            }
        }

        has_lower && has_upper && has_digit && has_special
    }

    /// Check if the password length is valid.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the password to check.
    ///
    /// # Returns
    ///
    /// A `bool` indicating if the password length is valid.
    fn is_valid_password_length(length: &str) -> bool {
        if let Ok(length) = length.parse::<usize>() {
            (8..=64).contains(&length)
        } else {
            false
        }
    }

    /// Add a new password to the password manager.
    ///
    /// # Arguments
    ///
    /// * `service` - The name of the service the password is for.
    /// * `username` - The name of the password to add.
    /// * `password` - The password to add.
    /// * `url` - The URL for the service.
    /// * `notes` - Additional notes about the password.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the password cannot be added.
    pub fn add_password(
        &self,
        service: Option<String>,
        username: Option<String>,
        password: Option<String>,
        url: Option<String>,
        notes: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let input_data = Self::get_user_data(service, username, password, url, notes)?;

        let entry = PasswordEntry::new(
            input_data["service"].clone(),
            input_data["username"].clone(),
            input_data["password"].clone(),
            input_data["url"].clone(),
            input_data["notes"].clone(),
        )?;

        self.database.create(&entry)?;

        Ok(())
    }

    /// Get user input for the password manager.
    ///
    /// # Arguments
    ///
    /// * `service` - The name of the service the password is for.
    /// * `username` - The name of the password to add.
    /// * `password` - The password to add.
    /// * `url` - The URL for the service.
    /// * `notes` - Additional notes about the password.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `HashMap` of the user input or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the user input cannot be retrieved.
    fn get_user_data(
        service: Option<String>,
        username: Option<String>,
        password: Option<String>,
        url: Option<String>,
        notes: Option<String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut input = HashMap::new();
        let service = if let Some(service) = service {
            service
        } else {
            Text::new("Please enter the service name:").prompt()?
        };

        let username = if let Some(username) = username {
            username
        } else {
            Text::new("Please enter the username:").prompt()?
        };

        let password = if let Some(password) = password {
            password
        } else if Confirm::new("Do you want to generate a password? (y/n)")
            .with_default(true)
            .prompt()?
        {
            Self::generate_password(Some(16))?
        } else {
            Password::new("Please enter the password:").prompt()?
        };

        let url = if let Some(url) = url {
            url
        } else {
            Text::new("Please enter the URL:").prompt()?
        };

        let notes = if Confirm::new("Do you want to add notes? (y/n)")
            .with_default(false)
            .prompt()?
        {
            if let Some(notes) = notes {
                notes
            } else {
                Text::new("Please enter the notes:").prompt()?
            }
        } else {
            "".to_string()
        };

        input.insert("service".to_string(), service);
        input.insert("username".to_string(), username);
        input.insert("password".to_string(), password);
        input.insert("url".to_string(), url);
        input.insert("notes".to_string(), notes);

        Ok(input)
    }

    /// List all passwords in the password manager.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the passwords cannot be listed.
    pub fn list_passwords(&self) -> Result<(), Box<dyn std::error::Error>> {
        let passwords = self.database.read()?;

        if passwords.is_empty() {
            println!("No passwords found.");
            return Ok(());
        }

        println!("ID\tService\tUsername\tURL\tNotes");
        for password in passwords {
            println!(
                "{:?}\t{}\t{}\t{}\t{}",
                password.id, password.service, password.username, password.url, password.notes
            );
        }

        Ok(())
    }

    /// Remove a password from the password manager.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the password to remove.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the password cannot be removed.
    pub fn remove_password(&self, id: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
        let id = if let Some(id) = id {
            id
        } else {
            let id = Text::new("Please enter the ID of the password to remove:").prompt()?;
            if let Ok(id) = id.parse::<i32>() {
                id
            } else {
                return Err("Invalid ID".into());
            }
        };

        self.database.delete(id)?;
        Ok(())
    }

    /// Update a password in the password manager.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the password to update.
    /// * `service` - The name of the service the password is for.
    /// * `username` - The name of the password to add.
    /// * `password` - The password to add.
    /// * `url` - The URL for the service.
    /// * `notes` - Additional notes about the password.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the password cannot be updated.
    pub fn update_password(
        &self,
        id: Option<i32>,
        service: Option<String>,
        username: Option<String>,
        password: Option<String>,
        url: Option<String>,
        notes: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = if let Some(id) = id {
            id
        } else {
            let id = Text::new("Please enter the ID of the password to update:").prompt()?;
            if let Ok(id) = id.parse::<i32>() {
                id
            } else {
                return Err("Invalid ID".into());
            }
        };

        let input_data = Self::get_user_data(service, username, password, url, notes)?;

        let entry = PasswordEntry::new(
            input_data["service"].clone(),
            input_data["username"].clone(),
            input_data["password"].clone(),
            input_data["url"].clone(),
            input_data["notes"].clone(),
        )?;

        self.database.update(id, entry)?;

        Ok(())
    }

    /// Show a password from the password manager.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the password to show.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the password cannot be shown.
    pub fn show_password(&self, id: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
        let id = if let Some(id) = id {
            id
        } else {
            let id = Text::new("Please enter the ID of the password to show:").prompt()?;
            if let Ok(id) = id.parse::<i32>() {
                id
            } else {
                return Err("Invalid ID".into());
            }
        };

        let password = self.database.read_by_id(id)?;

        println!(
            "ID: {:#?}\nService: {}\nUsername: {}\nPassword: {}\nURL: {}\nNotes: {}",
            password.id,
            password.service,
            password.username,
            password.password,
            password.url,
            password.notes
        );

        Ok(())
    }

    /// Search for a password in the password manager.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to search for.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the password cannot be found.
    pub fn search_password(&self, query: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let query = if let Some(query) = query {
            query
        } else {
            Text::new("Please enter the query to search for:").prompt()?
        };

        let passwords = self.database.search(&query)?;

        if passwords.is_empty() {
            println!("No passwords found.");
            return Ok(());
        }

        println!("ID\tService\tUsername\tURL\tNotes");
        for password in passwords {
            println!(
                "{:#?}\t{}\t{}\t{}\t{}",
                password.id, password.service, password.username, password.url, password.notes
            );
        }

        Ok(())
    }

    /// Export all passwords to a CSV file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to export the passwords to.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the passwords cannot be exported.
    pub fn export_passwords(&self, path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let path = if let Some(path) = path {
            path
        } else {
            Text::new("Please enter the path to export the passwords to:").prompt()?
        };

        let passwords = self.database.read()?;
        let mut writer = Writer::from_path(path.clone())?;

        writer.write_record(&[
            "Service",
            "Username",
            "Password",
            "URL",
            "Notes",
            "Created At",
            "Updated At",
        ])?;

        for password in passwords {
            let export = PasswordExport {
                service: password.service,
                username: password.username,
                password: password.password,
                url: password.url,
                notes: password.notes,
                created_at: password.created_at.to_string(),
                updated_at: password.updated_at.to_string(),
            };

            writer.serialize(export)?;
        }

        writer.flush()?;

        println!("Passwords successfully exported to: {}", path);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        let password = PasswordManager::generate_password(Some(16)).unwrap();
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_is_valid_password() {
        let password = "Password123!";
        assert!(PasswordManager::is_valid_password(password));
    }

    #[test]
    fn test_is_valid_password_length() {
        assert!(PasswordManager::is_valid_password_length("16"));
    }
}
