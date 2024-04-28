// Library
use crate::config;
use crate::helpers;
use crate::time;
use crate::wallpaper;

// ---
// GET
// ---

/// Get the current wallpaper path
pub fn get() {
    let filepath = wallpaper::get().unwrap();
    println!("{}", filepath);
}

// ---
// SET
// ---

/// Set the wallpaper
pub fn set(args: Vec<String>) {
    // Validate arguments
    if args.len() < 3 {
        eprintln!("Usage: {} set <path>", args[0]);
        std::process::exit(1);
    }

    // Extract the path from the arguments
    let path = std::path::PathBuf::from(&args[2]);

    // Check if the path exists
    if !path.exists() {
        eprintln!("Error: Path does not exist - {}", &args[2]);
        std::process::exit(1);
    }

    // Check if the path is an image file
    if helpers::is_image_file(&path) {
        // Set the image file as the wallpaper and exit early
        wallpaper::set(path.to_str().unwrap()).unwrap();
        println!("Wallpaper set to: {}", path.to_str().unwrap());
        std::process::exit(0);
    }

    // Read the config file
    let config = config::Config::read(&path).unwrap();
    println!("Configuration: {:?}", config);

    // Retrieve the wallpapers from the directory
    let wallpapers = helpers::get_wallpapers(&config.path);

    // Divide the day into segments
    let day = time::Day::new_with(config.start, config.end);
    let segments = day.divide(wallpapers.len() as u32);

    // Get the segment for the current time
    let segment = time::get_current_segment(&segments);

    // Set the Wallpaper
    let wallpaper = &wallpapers[segment];
    wallpaper::set(wallpaper.to_str().unwrap()).unwrap();
    println!(
        "Wallpaper set to: {} for ({})",
        wallpaper.to_str().unwrap(),
        segments[segment]
    );
}

// -------
// UNKNOWN
// -------

/// Unknown command handler
pub fn unknown(command: &str) {
    eprintln!("Error: Unknown command '{}'\n", command);
    help();
    std::process::exit(1);
}

// ----
// HELP
// ----

/// Display the help message
pub fn help() {
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
    println!("{}", msg);
}

// -------
// VERSION
// -------

/// Display the version number
pub fn version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}
