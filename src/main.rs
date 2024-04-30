// Library
mod commands;
mod helpers;
mod theme;
mod time;
mod wallpaper;

// ----
// MAIN
// ----

/// The entrypoint of the application
fn main() {
    // Read the command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Run the application
    match run(args) {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

/// Run the command-line application
fn run(args: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    // Validate arguments
    if args.len() < 2 {
        commands::help()?;
        return Err("Error: No command provided".into());
    }

    // Extract the command from the arguments
    let command = &args[1];

    // Match the command and execute the corresponding handler function
    match command.as_str() {
        "get" => commands::get(),
        "set" => commands::set(args),

        "help" => commands::help(),
        "--help" => commands::help(),
        "-h" => commands::help(),

        "version" => commands::version(),
        "--version" => commands::version(),
        "-v" => commands::version(),

        _ => commands::unknown(command),
    }
}
