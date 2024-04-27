// Library
use serde::Deserialize;

/// Configuration for the application
#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    offset: u32,
}

// Default Configuration Values
impl Default for Config {
    fn default() -> Config {
        Config { offset: 0 }
    }
}

impl Config {
    /// Read the configuration from the config file in the given path
    pub fn read(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let path = std::path::Path::new(path).join("dayshift.config.json");
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
