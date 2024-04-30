// Library
use crate::helpers;
use crate::theme;
use crate::time::{self, CurrentSegment};
use crate::wallpaper;

// ---
// GET
// ---

/// Get the current wallpaper path
pub fn get() -> Result<String, Box<dyn std::error::Error>> {
    wallpaper::get()
}

// ---
// SET
// ---

/// Set the wallpaper
pub fn set(args: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut args = args;

    // Validate arguments
    if args.len() < 3 {
        return Err("Error: No path provided\nUsage: dayshift set <path>".into());
    }

    // Check for the `--dry-run` flag, and remove it from the arguments
    let is_dry_run = if args.contains(&String::from("--dry-run")) {
        let idx = args.iter().position(|arg| arg == "--dry-run").unwrap();
        args.remove(idx);
        true
    } else {
        false
    };

    // Extract the path from the arguments
    let path = std::path::PathBuf::from(&args[2]);

    // Check if the path exists
    if !path.exists() {
        return Err(format!("Error: Path does not exist - {}", &args[2]).into());
    }

    // Check if the path is an image file
    if helpers::is_image_file(&path) {
        // Set the image file as the wallpaper and exit early
        let msg = format!("Wallpaper set to: {}", &args[2]);
        if is_dry_run {
            return Ok(format!("Dry Run: {}", msg));
        }
        wallpaper::set(&path)?;
        return Ok(msg);
    }

    // Check if the path is a directory
    if !path.is_dir() {
        return Err(format!("Error: Path is not a directory - {}", &args[2]).into());
    }

    // Read the theme config file
    let config = theme::Config::read(&path)?;

    // Retrieve the wallpapers from the directory
    let wallpapers = helpers::get_wallpapers(&config.path);

    // Divide the timeframe into segments
    let timeframe = time::TimeFrame::new(config.start, config.duration);
    let segments = timeframe.divide(wallpapers.len() as u32);

    // Get the segment index for the current time
    let idx = segments.current_index();

    // Set the Wallpaper
    let wallpaper = &wallpapers[idx];
    let msg = format!("Wallpaper set to: {}", wallpaper.display());
    if is_dry_run {
        return Ok(format!("Dry Run: {}", msg));
    }
    wallpaper::set(wallpaper)?;
    return Ok(msg);
}

// -------
// UNKNOWN
// -------

/// Unknown command handler
pub fn unknown(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let msg = format!("Error: Unknown command - {}", command);
    Err(format!("{}\n{}", msg, help()?).into())
}

// ----
// HELP
// ----

/// Display the help message
pub fn help() -> Result<String, Box<dyn std::error::Error>> {
    let msg = r#"
Usage: dayshift <command> <args...>

Commands:
    get             Get the current wallpaper path
    set             Set the wallpaper theme
    help            Display this help message
    version         Display the version number

Options:
    -h, --help      Display the help message
    -v, --version   Display the version number

Examples:
    dayshift get
    dayshift set /path/to/wallpapers
    dayshift help
    dayshift version

Note:
    The 'set' command requires a valid directory path containing wallpapers.
"#;
    Ok(String::from(msg))
}

// -------
// VERSION
// -------

/// Display the version number
pub fn version() -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!("{}", env!("CARGO_PKG_VERSION")))
}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let result = get();
        assert!(&result.is_ok());
        let path = std::path::PathBuf::from(&result.unwrap());
        assert!(path.exists());
    }

    #[test]
    fn test_set() {
        // Load the theme path from the .env file
        dotenv::dotenv().ok();
        let path = std::env::var("THEME_PATH").unwrap();
        let args = vec![
            String::from("dayshift"),
            String::from("set"),
            String::from("--dry-run"),
            path,
        ];
        let result = set(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unknown() {
        let result = unknown("unknown");
        assert!(result.is_err());
    }

    #[test]
    fn test_help() {
        let result = help();
        assert!(result.is_ok());
    }

    #[test]
    fn test_version() {
        let result = version();
        assert!(result.is_ok());
        assert!(result.unwrap() == env!("CARGO_PKG_VERSION"))
    }
}
