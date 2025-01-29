pub struct Encryption {
    pub key: String,
}

impl Encryption {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn encrypt(&self, data: &str) -> String {
        data.to_string()
    }

    pub fn decrypt(&self, data: &str) -> String {
        data.to_string()
    }

    pub fn generate_key() -> String {
        "key".to_string()
    }

    pub fn verify_key(&self, key: &str) -> bool {
        key == self.key
    }
}
