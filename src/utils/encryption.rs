use std::num::NonZeroU32;

use ring::{
    aead::{self, Nonce},
    pbkdf2,
    rand::{SecureRandom, SystemRandom},
};

#[derive(Debug)]
pub struct Encryption {
    key: aead::LessSafeKey,
}

impl Encryption {
    /// Create a new `Encryption` instance.
    ///
    /// # Arguments
    ///
    /// * `master_password` - The master password to use for encryption.
    ///
    /// # Returns
    ///
    /// A new `Encryption` instance.
    ///
    /// # Panics
    ///
    /// Panics if the key cannot be created.
    pub fn new(master_password: &str) -> Self {
        let rng = SystemRandom::new();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt).expect("RNG failed");

        let mut key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            &salt,
            master_password.as_bytes(),
            &mut key,
        );

        let unbound_key =
            aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &key).expect("Failed to create key");

        Self {
            key: aead::LessSafeKey::new(unbound_key),
        }
    }

    /// Encrypt data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to encrypt.
    ///
    /// # Returns
    ///
    /// A `Result` containing the encrypted data as a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// An error will be returned if the data cannot be encrypted.
    pub fn encrypt(&self, data: &str) -> Result<Vec<u8>, ring::error::Unspecified> {
        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes)?;
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

        let mut in_out = data.as_bytes().to_vec();
        self.key
            .seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)?;

        let mut result = Vec::with_capacity(nonce_bytes.len() + in_out.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&in_out);

        Ok(result)
    }

    /// Decrypt data.
    ///
    /// # Arguments
    ///
    /// * `encrypted_data` - The encrypted data to decrypt.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decrypted data as a `String`.
    ///
    /// # Errors
    ///
    /// An error will be returned if the data cannot be decrypted.
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<String, ring::error::Unspecified> {
        if encrypted_data.len() < 12 {
            return Err(ring::error::Unspecified);
        }

        let nonce = Nonce::assume_unique_for_key(
            encrypted_data[..12]
                .try_into()
                .map_err(|_| ring::error::Unspecified)?,
        );
        let mut in_out = encrypted_data[12..].to_vec();

        let plain_text = self
            .key
            .open_in_place(nonce, aead::Aad::empty(), &mut in_out)?;

        String::from_utf8(plain_text.to_vec()).map_err(|_| ring::error::Unspecified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let master_password = "password";
        let data = "data";

        let encryption = Encryption::new(master_password);
        let encrypted_data = encryption.encrypt(data).unwrap();
        let decrypted_data = encryption.decrypt(&encrypted_data).unwrap();

        assert_eq!(data, decrypted_data);
    }

    #[test]
    fn test_decrypt_invalid_data() {
        let encryption = Encryption::new("password");
        let result = encryption.decrypt(&[0u8; 8]);
        assert!(result.is_err());
    }

    #[test]
    fn test_different_passwords() {
        let data = "data";
        let encryption1 = Encryption::new("password1");
        let encryption2 = Encryption::new("password2");

        let encrypted_data = encryption1.encrypt(data).unwrap();
        let result = encryption2.decrypt(&encrypted_data);
        assert!(result.is_err());
    }
}
