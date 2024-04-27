// Library
mod commands;
mod config;
mod helpers;
mod time;
mod wallpaper;

// ----
// MAIN
// ----

/// The entrypoint of the application
fn main() {
    // Read the command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Validate arguments
    if args.len() < 2 {
        eprintln!("Usage: {} <command> <args...>", args[0]);
        std::process::exit(1);
    }

    // Extract the command from the arguments
    let command = &args[1];

    // Match the command and execute the corresponding handler function
    match command.as_str() {
        "get" => commands::get(args),
        "set" => commands::set(args),
        _ => commands::unknown(args),
    }
}
