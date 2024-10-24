use std::{fs, path::Path};

use colored::Colorize;

/// Get human-readable file size
pub fn format_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Get basic file metadata string
pub fn get_file_metadata(path: &Path) -> String {
    let metadata = match fs::metadata(path) {
        Ok(meta) => meta,
        Err(_) => return "[Unknown] [????]".to_string(),
    };
    let size = metadata.len();
    let file_type = if path.is_dir() {
        "DIR".blue().bold()
    } else {
        "FILE".green().bold()
    };
    format!("[{}] [{}]", format_size(size), file_type)
}
