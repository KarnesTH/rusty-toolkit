use inquire::{validator::Validation, Confirm, Password, Text};
use log::info;
use ring::rand::{SecureRandom, SystemRandom};

use crate::prelude::{Config, Database, Encryption};

#[derive(Debug)]
pub struct PasswordManager {
    pub length: usize,
    pub database: Database,
    pub encryption: Encryption,
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

        let master_password = if !master_file.exists() {
            let password = if Confirm::new("Do you want to generate a password? (y/n)")
                .with_default(true)
                .prompt()?
            {
                Self::generate_password(Some(16))?
            } else {
                Password::new("Please enter your master password:").prompt()?
            };

            let encryption = Encryption::new(&password);

            let verification_data = encryption.encrypt(&password).unwrap();
            std::fs::write(&master_file, verification_data)?;

            password
        } else {
            let password = Password::new("Please enter your master password:")
                .without_confirmation()
                .prompt()?;

            let encryption = Encryption::new(&password);
            let verification_data = std::fs::read(&master_file)?;

            if let Ok(decrypted) = encryption.decrypt(&verification_data) {
                if decrypted != password {
                    return Err("Invalid master password".into());
                }
            } else {
                return Err("Invalid master password".into());
            }

            password
        };

        let encryption = Encryption::new(&master_password);

        Ok(Self {
            length: 16,
            database: Database::new(config.get_db_path()?, master_password.as_str())?,
            encryption,
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
