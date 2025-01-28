use ring::rand::{SecureRandom, SystemRandom};

#[derive(Debug)]
pub struct PasswordManager {
    pub length: usize,
}

impl PasswordManager {
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
    pub fn generate_password(length: usize) -> Result<String, Box<dyn std::error::Error>> {
        let charset: &[u8] =
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+";
        let rng = SystemRandom::new();
        let mut password = String::with_capacity(length);

        for _ in 0..length {
            let mut byte = [0u8; 1];
            rng.fill(&mut byte).unwrap();
            let index = byte[0] as usize % charset.len();
            password.push(charset[index] as char);
        }
        Ok(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        let password = PasswordManager::generate_password(16).unwrap();
        assert_eq!(password.len(), 16);
    }
}
