// Library
use crate::helpers;
use crate::wallpaper;

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

    // Print the count of wallpapers found
    println!("Found {} wallpapers", wallpapers.len());
}

// -------
// UNKNOWN
// -------

/// Unknown command handler
pub fn unknown(args: Vec<String>) {
    eprintln!("Error: Unknown command '{}'", args[1]);
    std::process::exit(1);
}
