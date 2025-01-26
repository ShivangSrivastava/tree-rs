use std::{
    borrow::Cow,
    collections::HashSet,
    env,
    fs::{self},
    path::Path,
    process::exit,
};

use colorized::{Color, Colors};

/// Sorts the entries of a given directory into directories and files.
///
/// # Parameters
/// - `path`: A reference to a `Path` representing the directory whose entries will be sorted.
///
/// # Returns
/// A `Vec<fs::DirEntry>` containing the sorted directory entries. Directories will appear before files in the vector.
fn sort_entities(path: &Path) -> Vec<fs::DirEntry> {
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

/// Recursively prints the directory structure in a tree format.
///
/// # Parameters
/// - `path`: A reference to a `Path` representing the directory to be printed.
/// - `prefix`: A string used for formatting the tree structure (for indentation).
/// - `exclude`: A reference to a vector of `Cow<'_, str>` containing the names of directories to exclude from the tree traversal.
fn print_tree(path: &Path, prefix: &str, exclude: &HashSet<Cow<'_, str>>) {
    let entries = sort_entities(path);

    let total_entries = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let raw_file_name = entry.file_name();
        let file_name = raw_file_name.to_string_lossy();

        let is_last = i == total_entries - 1;
        let is_dir = entry.path().is_dir();
        let is_hidden = file_name.starts_with(".");
        let is_excluded = !exclude.contains(&file_name);

        let connector = if is_last { "└── " } else { "├── " };

        let formatted_name = if is_hidden {
            if is_dir {
                format!("{}/", file_name).color(Colors::BrightBlackFg)
            } else {
                file_name.to_string().color(Colors::BrightBlackFg)
            }
        } else if is_dir {
            format!("{}/", file_name).color(Colors::BlueFg)
        } else {
            file_name.to_string()
        };
        println!("{}{} {}", prefix, connector, formatted_name,);

        if entry.path().is_dir() {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            if is_excluded {
                print_tree(&entry.path(), &new_prefix, exclude);
            }
        }
    }
}

/// The entry point of the program that prints the directory tree of the current working directory.
fn main() {
    let path = Path::new(".");
    let exclude: HashSet<Cow<'_, str>> = vec![
        "target",
        ".git",
        ".venv",
        "node_modules",
        "build",
        ".gradle",
        "__pycache__",
        ".cache",
        ".config",
        ".dart_tool",
        ".mypy_cache",
        ".firebase",
        ".idea",
    ]
    .into_iter()
    .map(|e| Cow::from(e))
    .collect();
    let cwd = env::current_dir().expect("Failed to get current directory");
    if let Some(project_name) = cwd.file_name() {
        println!("{}", project_name.to_string_lossy());
    } else {
        println!("No folder name found");
        exit(1);
    }
    print_tree(path, "", &exclude);
}
