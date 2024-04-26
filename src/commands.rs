use crate::wallpaper;

/// Get the current wallpaper path
pub fn get(_: Vec<String>) {
    let filepath = wallpaper::get().unwrap();
    println!("Wallpaper Path: {}", filepath);
}

/// Set the wallpaper
pub fn set(args: Vec<String>) {
    // Validate arguments
    if args.len() < 3 {
        eprintln!("Usage: {} set <path>", args[0]);
        std::process::exit(1);
    }

    // Extract the path from the arguments and set the wallpaper
    let path = &args[2];
    wallpaper::set(path).unwrap();
    println!("Wallpaper set to: {}", path);
}

/// Unknown command handler
pub fn unknown(args: Vec<String>) {
    eprintln!("Error: Unknown command '{}'", args[1]);
    std::process::exit(1);
}
