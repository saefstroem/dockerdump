use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

/// Searches for files in the given directory that match the search string
pub fn search_files(dir: &Path, search_str: &str) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .to_lowercase()
                .contains(&search_str.to_lowercase())
        })
        .map(|e| e.path().to_owned())
        .collect()
}

/// Copy file or directory to destination
pub fn copy_path(src: &Path, dest_dir: &Path) -> std::io::Result<()> {
    let dest = dest_dir.join(src.file_name().unwrap());
    if src.is_dir() {
        fs::create_dir_all(&dest)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            copy_path(&entry.path(), &dest)?;
        }
    } else {
        fs::copy(src, dest)?;
    }
    Ok(())
}
