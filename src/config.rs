// Library
use crate::time::{CurrentSegment, TimeSegment};

// External Library
use chrono::{Duration, NaiveDateTime, NaiveTime};
use serde::Deserialize;

/// Name of the configuration file
const CONFIG_FILE: &str = "dayshift.config.json";

/// Configuration for the application
#[derive(Clone, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    /// Start time
    pub start: NaiveTime,
    /// Duration in hours
    pub duration: u32,
    /// Path to the wallpaper directory
    pub path: std::path::PathBuf,
    /// Wallpaper selection mode (random or sequential)
    pub selection: SelectionMode,
}

/// Selection Mode
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SelectionMode {
    Random,     // Select randomly
    Sequential, // Select in order
}

// Default Configuration Values
impl Default for Config {
    fn default() -> Config {
        Config {
            path: std::path::PathBuf::default(),
            start: NaiveTime::from_hms_opt(0, 0, 0).unwrap_or_default(),
            duration: 24,
            selection: SelectionMode::Random,
        }
    }
}

// Implement the TimeSegment trait for Config
impl TimeSegment for Config {
    fn start(&self) -> NaiveDateTime {
        let now = chrono::Local::now().naive_local();
        now.date().and_time(self.start)
    }
    fn end(&self) -> NaiveDateTime {
        self.start() + Duration::hours(self.duration as i64)
    }
}

impl Config {
    /// Read the configuration from the config file in the given path
    pub fn read(dir: &std::path::Path) -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = dir.join(CONFIG_FILE);
        let contents = std::fs::read_to_string(&config_path).unwrap_or(String::from("[]"));
        let configs: Vec<Config> = serde_json::from_str(&contents)?;

        // Get the current configuration based on the current time
        let mut config = configs.current().clone();

        // Update the path if it is relative
        if let Some(path) = config.path.to_str() {
            if config.path.is_relative() {
                config.path = dir.join(path);
            }
        }

        Ok(config)
    }
}
