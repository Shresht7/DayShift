// External Library
use chrono::NaiveTime;
use serde::Deserialize;

/// Name of the configuration file
const CONFIG_FILE: &str = "dayshift.config.json";

/// Configuration for the application
#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub selection: SelectionMode,
}

/// Selection Mode
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SelectionMode {
    Random,     // Select randomly
    Sequential, // Select in order
}

// Default Configuration Values
impl Default for Config {
    fn default() -> Config {
        Config {
            start: NaiveTime::from_hms_opt(0, 0, 0).unwrap_or_default(),
            end: NaiveTime::from_hms_opt(23, 59, 59).unwrap_or_default(),
            selection: SelectionMode::Random,
        }
    }
}

impl Config {
    /// Read the configuration from the config file in the given path
    pub fn read(path: &std::path::Path) -> Result<Config, Box<dyn std::error::Error>> {
        let path = path.join(CONFIG_FILE);
        let contents = std::fs::read_to_string(path).unwrap_or(String::from("{}"));
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
