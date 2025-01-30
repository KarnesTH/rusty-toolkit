use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub logging: LogConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            logging: LogConfig {
                level: "info".to_string(),
            },
            database: DatabaseConfig {
                db_name: "pass.db".to_string(),
            },
        }
    }
}

impl Config {
    /// Load the configuration from the config directory.
    ///
    /// If the configuration file does not exist, it will be created with the default configuration.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Config` struct or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the configuration file cannot be read or written.
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let config_path = config_dir.join("config.toml");

        let config = if config_path.exists() {
            let config_str = std::fs::read_to_string(&config_path)?;
            toml::from_str(&config_str)?
        } else {
            let default_config = Config::default();
            let config_str = toml::to_string_pretty(&default_config)?;
            std::fs::write(&config_path, &config_str)?;

            default_config
        };

        Ok(config)
    }

    /// Get the configuration directory.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configuration directory path or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the configuration directory cannot be found or created.
    pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir().ok_or("Could not find config directory")?;
        let app_dir = config_dir.join("karnes-development/rusty-toolkit");

        if !app_dir.exists() {
            std::fs::create_dir_all(&app_dir)?;
        }

        Ok(app_dir)
    }

    /// Get the log directory.
    ///
    /// # Returns
    ///
    /// A `Result` containing the log directory path or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the log directory cannot be found or created.
    fn get_log_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let log_dir = config_dir.join("logs");

        if !log_dir.exists() {
            std::fs::create_dir_all(&log_dir)?;
        }

        Ok(log_dir)
    }

    /// Setup the logger with the configuration settings.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the log file cannot be opened or written to.
    pub fn setup_logger(&self) -> Result<(), Box<dyn std::error::Error>> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let log_dir = Self::get_log_dir()?;
        let log_file = log_dir.join(format!("rusty-toolkit-{}.log", today));

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        let level = match self.logging.level.as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info,
        };

        Builder::new()
            .write_style(WriteStyle::Always)
            .filter(None, level)
            .target(env_logger::Target::Pipe(Box::new(file)))
            .init();

        Ok(())
    }

    /// Get the database path.
    ///
    /// If the database file does not exist, it will be created with the necessary schema.
    ///
    /// # Returns
    ///
    /// A `Result` containing the database path or an error.
    ///
    /// # Errors
    ///
    /// An error will be returned if the database file cannot be opened or created.
    pub fn get_db_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let db_path = config_dir.join(&self.database.db_name);

        Ok(db_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert_eq!(config.logging.level, "info");
        assert_eq!(config.database.db_name, "pass.db");
    }

    #[test]
    fn test_load_config() {
        let config = Config::load().unwrap();

        assert_eq!(config.logging.level, "info");
        assert_eq!(config.database.db_name, "pass.db");
    }

    #[test]
    fn test_setup_logger() {
        let config = Config::default();
        let result = config.setup_logger();

        assert!(result.is_ok());
    }
}
