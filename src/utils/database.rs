use std::path::PathBuf;

use chrono::{DateTime, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswortEntry {
    pub id: i32,
    pub service: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Database {
    pub connection: Connection,
    pub path: PathBuf,
}

impl Database {
    pub fn new(path: PathBuf) -> Self {
        let conn = Connection::open(&path).unwrap();
        Self {
            connection: conn,
            path,
        }
    }

    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub fn read(&self) -> Result<Vec<PasswortEntry>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }

    pub fn update(&self, _entry: PasswortEntry) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub fn delete(&self, _id: i32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
