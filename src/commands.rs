// Library
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

    // Retrieve the wallpapers from the directory
    let wallpapers = helpers::get_wallpapers(path);

    // Divide the day into segments
    let day = time::Day::new();
    let segments = day.divide(wallpapers.len() as u32);

    // Get current time of day
    let now = Local::now();
    let current_time = now.hour() * 60 + now.minute();

    // Get the segment for the current time
    let segment = time::get_segment_number(&segments, current_time);

    // Print all the segments
    for (i, segment) in segments.iter().enumerate() {
        println!("Segment {}: {}", i, segment.time());
    }

    // Show the segment time range
    println!("\nSegment {}: {}", segment, segments[segment].time());
}

// -------
// UNKNOWN
// -------

/// Unknown command handler
pub fn unknown(args: Vec<String>) {
    eprintln!("Error: Unknown command '{}'", args[1]);
    std::process::exit(1);
}
