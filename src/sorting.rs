use std::{error::Error, fs, io, path::Path};

/// Sorts the entries of a given directory into directories and files.
///
/// # Parameters
/// - `path`: A reference to a `Path` representing the directory whose entries will be sorted.
///
/// # Returns
/// A `Vec<fs::DirEntry>` containing the sorted directory entries. Directories will appear before files in the vector.
pub fn sort_entities(path: &Path) -> Result<Vec<fs::DirEntry>, io::Error> {
    let mut dirs = vec![];
    let mut files = vec![];

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.filter_map(|e| Some(e.unwrap())) {
                if entry.path().is_file() {
                    files.push(entry);
                } else {
                    dirs.push(entry);
                }
            }
        }
        Err(e) => return Err(e),
    }
    Ok(dirs.into_iter().chain(files.into_iter()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_sort_entities() {
        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        // Create subdirectories
        let sub_dir1 = temp_path.join("dir1");
        let sub_dir2 = temp_path.join("dir2");
        fs::create_dir_all(&sub_dir1).unwrap();
        fs::create_dir_all(&sub_dir2).unwrap();

        // Create files
        let file1 = temp_path.join("file1.txt");
        let file2 = temp_path.join("file2.log");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        // Call the function
        let sorted_entries = sort_entities(temp_path).unwrap();

        // Collect paths for comparison
        let sorted_paths: Vec<PathBuf> = sorted_entries.iter().map(|e| e.path()).collect();

        // Expected sorted order: Directories first, then files
        let expected_paths = vec![sub_dir1, sub_dir2, file1, file2];

        assert_eq!(sorted_paths, expected_paths);
    }

    #[test]
    fn test_sort_entities_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        let sorted_entries = sort_entities(temp_path).unwrap();
        assert!(sorted_entries.is_empty());
    }

    #[test]
    fn test_sort_entities_invalid_path() {
        let invalid_path = Path::new("/this/path/does/not/exist");
        let result = sort_entities(invalid_path);
        assert!(result.is_err());
    }
}
