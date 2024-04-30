// ----------------
// HELPER FUNCTIONS
// ----------------

/// Retrieve the wallpapers from the directory
pub fn get_wallpapers(path: &std::path::PathBuf) -> Vec<std::path::PathBuf> {
    let mut wallpapers = Vec::new();

    // Define the glob pattern for the directory
    let mut pattern = path.clone();
    if !path.ends_with(".png") || !path.ends_with(".jpg") {
        pattern.push("*");
    }

    // Iterate over the files in the directory
    let pattern = pattern.to_str().unwrap();
    for entry in glob::glob(pattern).unwrap() {
        match entry {
            Ok(path) => {
                // Check if the path matches the criteria
                if matches_criteria(&path).unwrap_or(false) {
                    wallpapers.push(path);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    return wallpapers;
}

/// Check if the path matches the criteria for a numbered wallpaper
fn matches_criteria(path: &std::path::PathBuf) -> Option<bool> {
    // Extract the extension and basename of the file
    let ext = path.extension()?.to_str()?;
    let basename = path
        .file_name()?
        .to_str()?
        .trim_end_matches(&format!(".{}", ext));

    // Check if the path is a image file
    if !is_image_file(&path) {
        return Some(false);
    }

    // Ignore files that are not numbered
    if basename.parse::<u32>().is_err() {
        return Some(false);
    }

    // If all the criteria are met, return true
    return Some(true);
}

// IS IMAGE FILE
// -------------

/// Valid image file extensions
const VALID_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

/// Check if the path is an image file (jpg, jpeg, png)
pub fn is_image_file(path: &std::path::Path) -> bool {
    // Check if the path is a file
    if !path.is_file() {
        return false;
    }
    // Check if the extension is valid
    match path.extension().and_then(|s| s.to_str()) {
        Some(extension) if VALID_EXTENSIONS.contains(&extension) => true,
        _ => false,
    }
}
