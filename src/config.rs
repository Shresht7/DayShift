// External Library
use chrono::NaiveTime;
use serde::Deserialize;

/// Name of the configuration file
const CONFIG_FILE: &str = "dayshift.config.json";

/// Configuration for the application
#[derive(Clone, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    /// Start time
    pub start: NaiveTime,
    /// End time
    pub end: NaiveTime,
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
            end: NaiveTime::from_hms_opt(23, 59, 59).unwrap_or_default(),
            selection: SelectionMode::Random,
        }
    }
}

impl Config {
    /// Read the configuration from the config file in the given path
    pub fn read(dir: &std::path::PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = dir.join(CONFIG_FILE);
        let contents = std::fs::read_to_string(&config_path).unwrap_or(String::from("[]"));
        let configs: Vec<Config> = serde_json::from_str(&contents)?;

        // Get the current configuration based on the current time
        let mut config = Config::get_current(&configs);

        // Set the theme config path
        match &config.path {
            // If the path is relative, join it with the given directory
            path if path.is_relative() => {
                config.path = dir.join(path);
            }
            // If the path is absolute, use it as is
            path if path.is_absolute() => {
                config.path = path.clone();
            }
            // Default to the given directory
            _ => {
                config.path = dir.clone();
            }
        }

        return Ok(config);
    }

    /// Get the current configuration based on the current time
    fn get_current(configs: &Vec<Config>) -> Config {
        let time = chrono::Local::now().time();
        let mut config = Config::default();
        for c in configs.iter() {
            let start = c.start;
            let mut end = c.end;
            if end <= start && time > start {
                end += chrono::Duration::days(1);
            }
            println!("{} - {} - {}", start, end, time);
            if time >= start && time < end {
                config = c.clone();
            }
        }
        return config;
    }
}
