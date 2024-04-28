// ----------------
// HELPER FUNCTIONS
// ----------------

/// Retrieve the wallpapers from the directory
pub fn get_wallpapers(path: &std::path::PathBuf) -> Vec<std::path::PathBuf> {
    let mut wallpapers = Vec::new();

    // Read the directory and store the paths that match the criteria
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if matches_criteria(&path) {
            wallpapers.push(path);
        }
    }

    return wallpapers;
}

/// Check if the path matches the criteria for a numbered wallpaper
fn matches_criteria(path: &std::path::PathBuf) -> bool {
    // Extract the extension and basename of the file
    let ext = path.extension().unwrap().to_str().unwrap();
    let basename = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_end_matches(&format!(".{}", ext));

    // Check if the path is a image file
    if !is_image_file(path) {
        return false;
    }
    // Ignore files that are not numbered
    if basename.parse::<u32>().is_err() {
        return false;
    }

    // If all the criteria are met, return true
    return true;
}

/// Check if the path is an image file (jpg, jpeg, png)
pub fn is_image_file(path: &std::path::PathBuf) -> bool {
    let valid_extensions = ["jpg", "jpeg", "png"];

    // Check if the path is a file
    if !path.is_file() {
        return false;
    }

    // Check if the extension is valid
    let ext = path.extension();
    match ext {
        Some(ext) => {
            let ext = ext.to_str().unwrap();
            return valid_extensions.contains(&ext);
        }
        None => return false,
    }
}
