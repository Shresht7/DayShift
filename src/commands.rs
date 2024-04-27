// Library
use crate::config;
use crate::helpers;
use crate::time;
use crate::wallpaper;

// External Library
use chrono::{Local, Timelike};

// ---
// GET
// ---

/// Get the current wallpaper path
pub fn get(_: Vec<String>) {
    let filepath = wallpaper::get().unwrap();
    println!("Wallpaper Path: {}", filepath);
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
    let path = &args[2];

    // Check if the path is a valid directory
    if !std::path::Path::new(path).exists() {
        eprintln!("Error: Path '{}' does not exist", path);
        std::process::exit(1);
    }

    // Read the config file
    let config = config::Config::read(path).unwrap();
    println!("Config: {:?}", config);

    // Retrieve the wallpapers from the directory
    let wallpapers = helpers::get_wallpapers(path);

    // Divide the day into segments
    let day = time::Day::new_with(config.start, config.end); // TODO: Allow custom day start and end times
    let segments = day.divide(wallpapers.len() as u32);

    // Get current time of day
    let now = Local::now();
    let current_time = now.num_seconds_from_midnight();

    // Get the segment for the current time
    let segment = time::get_segment_number(&segments, current_time);

    // Set the Wallpaper
    let wallpaper = &wallpapers[segment];
    wallpaper::set(wallpaper.to_str().unwrap()).unwrap();
    println!(
        "Wallpaper set to: {} for {}",
        wallpaper.to_str().unwrap(),
        segments[segment].time()
    );
}

// -------
// UNKNOWN
// -------

/// Unknown command handler
pub fn unknown(args: Vec<String>) {
    eprintln!("Error: Unknown command '{}'", args[1]);
    std::process::exit(1);
}
