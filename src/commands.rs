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
    // Validate arguments
    if args.len() < 3 {
        return Err("Error: No path provided\nUsage: dayshift set <path>".into());
    }

    // Extract the path from the arguments
    let path = std::path::PathBuf::from(&args[2]);

    // Check if the path exists
    if !path.exists() {
        return Err(format!("Error: Path does not exist - {}", &args[2]).into());
    }

    // Check if the path is an image file
    if helpers::is_image_file(&path) {
        // Set the image file as the wallpaper and exit early
        wallpaper::set(&path)?;
        return Ok(format!("Wallpaper set to: {}", &args[2]));
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
    wallpaper::set(wallpaper)?;
    return Ok(format!("Wallpaper set to: {}", wallpaper.display()));
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
