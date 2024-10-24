use std::{
    fs,
    path::{Path, PathBuf},
};

use colored::Colorize;
use dialoguer::{MultiSelect, Select};

use crate::search::metadata::get_file_metadata;

/// Browse directory contents and allow selection
pub fn browse_directory(current_dir: &Path, base_dir: &Path) -> Vec<PathBuf> {
    let mut selected = Vec::new();
    let mut current_path = current_dir.to_path_buf();

    loop {
        // Read directory contents
        let mut entries: Vec<_> = fs::read_dir(&current_path)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();

        // Sort entries (directories first, then alphabetically)
        entries.sort_by(|a, b| {
            let a_is_dir = a.path().is_dir();
            let b_is_dir = b.path().is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        // Create display items
        let mut display_items = vec!["[<-] Parent Directory".blue().to_string()]; // Parent directory option
        display_items.extend(entries.iter().map(|e| {
            let path = e.path();
            let metadata = get_file_metadata(&path);
            let file_name = e.file_name();
            let name = file_name.to_string_lossy();
            let prefix = if path.is_dir() { "[DIR] " } else { "[FILE] " };
            format!("{}{} {}", prefix, metadata, name)
        }));

        // Add options menu
        display_items.push("[Done browsing]".yellow().to_string());
        display_items.push("[Select files in current directory]".green().to_string());

        let path_or_slash = {
            let path = current_path
                .strip_prefix(base_dir)
                .unwrap_or(&current_path)
                .to_str()
                .unwrap();
            if path.is_empty() {
                "/"
            } else {
                path
            }
        };

        let selection = Select::new()
            .with_prompt(format!("Browsing: {}", path_or_slash))
            .items(&display_items)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                // Parent directory
                if current_path != base_dir {
                    if let Some(parent) = current_path.parent() {
                        current_path = parent.to_path_buf();
                    }
                }
            }
            i if i == entries.len() + 1 => {
                // Done browsing
                break;
            }
            i if i == entries.len() + 2 => {
                // Select files in current directory
                let file_items: Vec<String> = entries
                    .iter()
                    .map(|e| {
                        let path = e.path();
                        let metadata = get_file_metadata(&path);
                        let file_name = e.file_name();
                        let name = file_name.to_string_lossy();
                        let prefix = if path.is_dir() { "[DIR] " } else { "[FILE] " };
                        format!("{}{} {}", prefix, metadata, name)
                    })
                    .collect();

                if let Ok(selections) = MultiSelect::new()
                    .with_prompt("Select files (Space to select, Enter to confirm)")
                    .items(&file_items)
                    .interact()
                {
                    for idx in selections {
                        let path = entries[idx].path();
                        if !selected.contains(&path) {
                            selected.push(path);
                        }
                    }
                }
            }
            i => {
                // Navigate into directory or select file
                let path = entries[i - 1].path();
                if path.is_dir() {
                    current_path = path;
                }
            }
        }
    }

    selected
}
