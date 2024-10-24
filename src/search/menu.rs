use std::{
    fs,
    path::{Path, PathBuf},
};

use colored::Colorize;
use dialoguer::{Input, MultiSelect, Select};

use crate::{
    clean::cleanup,
    search::{
        io::{copy_path, search_files},
        metadata::get_file_metadata,
    },
};

use super::browse::browse_directory;

fn format_paths_for_display(paths: &[PathBuf], temp_dir: &Path) -> Vec<String> {
    // Format paths for display by stripping the temp directory and adding metadata
    let display_paths: Vec<String> = paths
        .iter()
        .map(|p| {
            let relative_path = p.strip_prefix(&temp_dir).unwrap_or(p).display().to_string();
            let metadata = get_file_metadata(p);
            format!("{} {}", metadata, relative_path)
        })
        .collect();
    display_paths
}

/// Interactive file search and selection interface
pub async fn interactive_search(temp_dir: PathBuf) -> std::io::Result<()> {
    let mut selected_files: Vec<PathBuf> = Vec::new();

    loop {
        // Show current selection status
        if !selected_files.is_empty() {
            println!("\n{}", "Currently selected files:".blue().bold());
            for file in &selected_files {
                // Strip the temp directory as it's not relevant to the user
                println!(
                    "  {}",
                    file.strip_prefix(&temp_dir).unwrap_or(file).display()
                );
            }
        }

        // Main menu
        let menu_items = vec![
            "Browse files".to_string(),
            "Search for files".to_string(),
            "Edit selected files".to_string(),
            "Extract selected files".to_string(),
            "Exit".to_string(),
        ];

        // Select action
        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&menu_items)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                // Browse files
                let new_selections = browse_directory(&temp_dir, &temp_dir);
                for path in new_selections {
                    // If anything new was selected, add it to our list
                    if !selected_files.contains(&path) {
                        selected_files.push(path);
                    }
                }
            }
            1 => {
                // Search for files
                let search_term: String = Input::new()
                    .with_prompt("Enter search term")
                    .interact_text()
                    .unwrap();

                // Search for files matching the search term
                let matches = search_files(&temp_dir, &search_term);

                // If no matches, inform the user and continue
                if matches.is_empty() {
                    println!("{}", "No files found matching your search.".yellow());
                    continue;
                }

                println!(
                    "\n{} {} {}",
                    "Found".green(),
                    matches.len(),
                    "matching file(s):".green()
                );

                // Prompt user to select files, storing the selections as the index of the matches
                // which in this case is the same as the index of the display_paths
                let selections = MultiSelect::new()
                    .with_prompt("Select files to add (Space to select, Enter to confirm)")
                    .items(&format_paths_for_display(&matches, &temp_dir))
                    .interact()
                    .unwrap();

                // Add selected files to our list
                for idx in selections {
                    // If the file isn't already selected, add it
                    let path = &matches[idx];
                    if !selected_files.contains(path) {
                        selected_files.push(path.clone());
                    }
                }
            }
            2 => {
                // View/remove selected files
                if selected_files.is_empty() {
                    println!("{}", "No files currently selected.".yellow());
                    continue;
                }

                let selections = MultiSelect::new()
                    .with_prompt("Select files to remove (Space to select, Enter to remove)")
                    .items(&format_paths_for_display(&selected_files, &temp_dir))
                    .interact()
                    .unwrap();

                // Remove selected files from our list (in reverse order to maintain indices)
                for idx in selections.iter().rev() {
                    selected_files.remove(*idx);
                }
            }
            3 => {
                // Extract selected files
                if selected_files.is_empty() {
                    println!("{}", "No files selected for extraction.".yellow());
                    continue;
                }

                // 
                let output_dir: String = Input::new()
                    .with_prompt("Enter output directory")
                    .default(".".into())
                    .interact_text()
                    .unwrap();

                let output_path = Path::new(&output_dir);
                if !output_path.exists() {
                    fs::create_dir_all(output_path)?;
                }

                println!("\n{}", "Extracting files...".green());
                for file in &selected_files {
                    match copy_path(file, output_path) {
                        Ok(_) => println!(
                            "✓ Extracted: {}",
                            file.strip_prefix(&temp_dir).unwrap_or(file).display()
                        ),
                        Err(e) => println!("✗ Failed to extract {}: {}", file.display(), e),
                    }
                }
                println!("{}", "Extraction complete.".green());
            }
            4 => {
                // Exit
                cleanup(temp_dir.to_str().unwrap())?;
                println!("{}", "suh!".bold());
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
