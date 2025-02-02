use std::{borrow::Cow, collections::HashSet, path::Path};

use colorized::{Color, Colors};

use crate::sorting::sort_entities;

/// Recursively prints the directory structure in a tree format.
///
/// # Parameters
/// - `path`: A reference to a `Path` representing the directory to be printed.
/// - `prefix`: A string used for formatting the tree structure (for indentation).
/// - `exclude`: A reference to a vector of `Cow<'_, str>` containing the names of directories to exclude from the tree traversal.
pub fn print_tree(path: &Path, prefix: &str, exclude: &HashSet<Cow<'_, str>>) {
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
