// Library
use crate::time;

// External Library
use serde::Deserialize;

/// Configuration for the application
#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub start: u32,
    pub end: u32,
}

// Default Configuration Values
impl Default for Config {
    fn default() -> Config {
        Config {
            start: 0,
            end: time::SECONDS_IN_DAY,
        }
    }
}

impl Config {
    /// Read the configuration from the config file in the given path
    pub fn read(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let path = std::path::Path::new(path).join("dayshift.config.json");
        let contents = std::fs::read_to_string(path).unwrap_or(String::from("{}"));
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
