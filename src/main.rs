use std::{borrow::Cow, env, fs, path::Path, process::exit};

use colorized::{Color, Colors};

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

fn print_tree(path: &Path, prefix: &str, exclude: &Vec<Cow<'_, str>>) {
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

        // if file_name.starts_with(".") {
        //     println!(
        //         "{}{} {}",
        //         prefix,
        //         connector,
        //         file_name.to_string().color(Colors::BrightBlackFg)
        //     );
        // } else if entry.path().is_dir() {
        //     println!(
        //         "{}{} {}/",
        //         prefix,
        //         connector,
        //         file_name.to_string().color(Colors::BlueFg)
        //     );
        // } else {
        //     println!("{}{} {}", prefix, connector, file_name);
        // }
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

fn main() {
    let path = Path::new(".");
    let exclude: Vec<Cow<'_, str>> = vec!["target", ".git", ".venv", "node_modules", "build"]
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
