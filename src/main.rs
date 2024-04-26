// Library
mod wallpaper;

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

    // Match the command and execute the corresponding function
    match command.as_str() {
        "get" => {
            let filepath = wallpaper::get().unwrap();
            println!("Wallpaper Path: {}", filepath);
        }
        "set" => {
            if args.len() < 3 {
                eprintln!("Usage: {} set <path>", args[0]);
                std::process::exit(1);
            }

            let path = &args[2];
            wallpaper::set(path).unwrap();
            println!("Wallpaper set to: {}", path);
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            std::process::exit(1);
        }
    }
}
