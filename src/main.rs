use std::{borrow::Cow, collections::HashSet, env, path::Path, process::exit};

use display::print_tree;

mod display;

mod sorting;

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
