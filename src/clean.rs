use std::{fs, path::Path};

// Alternative cleanup approach
pub fn cleanup(temp_dir: &str) -> std::io::Result<()> {
    // Let user clean up when they're done, with proper error handling
    if Path::new(temp_dir).exists() {
        println!("Cleaning up {}", temp_dir);
        fs::remove_dir_all(temp_dir)?;
    }
    Ok(())
}
