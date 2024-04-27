// External Library
use chrono::NaiveTime;
use serde::Deserialize;

/// Configuration for the application
#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

// Default Configuration Values
impl Default for Config {
    fn default() -> Config {
        Config {
            start: NaiveTime::from_hms_opt(0, 0, 0).unwrap_or_default(),
            end: NaiveTime::from_hms_opt(23, 59, 59).unwrap_or_default(),
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
