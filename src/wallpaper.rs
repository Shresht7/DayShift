// Standard Library
use std::os::windows::ffi::OsStrExt;

// External Library
use winapi::um::winuser;

// TODO: Add support for other platforms

// ---
// GET
// ---

/// Get the current wallpaper path using the Windows API
pub fn get() -> Result<String, Box<dyn std::error::Error>> {
    unsafe {
        // Create a buffer to store the wallpaper path (maximum length is 260 in Windows)
        let buffer: [u16; 260] = std::mem::zeroed();

        // Try to get the wallpaper path using the Windows API
        let ok = winuser::SystemParametersInfoW(
            winuser::SPI_GETDESKWALLPAPER,                  // Action code
            buffer.len() as u32,                            // Maximum buffer size
            buffer.as_ptr() as *mut winapi::ctypes::c_void, // Buffer pointer to store the path
            0,                                              // No flags needed for this operation
        ) == 1; // Returns 1 if successful

        // If the operation failed, return the last OS error
        if !ok {
            return Err(std::io::Error::last_os_error().into());
        }

        // Convert the buffer to a valid UTF-16 string and remove the trailing null character
        let filepath = String::from_utf16(&buffer)?.trim_end_matches('\x00').into();

        // Return the wallpaper path
        Ok(filepath)
    }
}

// ---
// SET
// ---

/// Set the wallpaper path using the Windows API
pub fn set(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Convert the path to a UTF-16 string
        let path: Vec<u16> = std::ffi::OsStr::new(path)
            .encode_wide()
            .chain(std::iter::once(0)) // Append a null character
            .collect();

        // Try to set the wallpaper path using the Windows API
        let ok = winuser::SystemParametersInfoW(
            winuser::SPI_SETDESKWALLPAPER,                          // Action code
            0,                                                      // Not used
            path.as_ptr() as *mut winapi::ctypes::c_void,           // Path pointer to the wallpaper
            winuser::SPIF_UPDATEINIFILE | winuser::SPIF_SENDCHANGE, // Update the .ini file and send a change notification
        ) == 1; // Returns 1 if successful

        // If the operation failed, return the last OS error
        if !ok {
            return Err(std::io::Error::last_os_error().into());
        }

        // Return success
        Ok(())
    }
}
