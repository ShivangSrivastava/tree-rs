use std::{fs, path::Path};

/// Sorts the entries of a given directory into directories and files.
///
/// # Parameters
/// - `path`: A reference to a `Path` representing the directory whose entries will be sorted.
///
/// # Returns
/// A `Vec<fs::DirEntry>` containing the sorted directory entries. Directories will appear before files in the vector.
pub fn sort_entities(path: &Path) -> Vec<fs::DirEntry> {
    let mut dirs = vec![];
    let mut files = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| Some(e.unwrap())) {
            if entry.path().is_file() {
                files.push(entry);
            } else {
                dirs.push(entry);
            }
        }
    }
    dirs.into_iter().chain(files.into_iter()).collect()
}
