# tree-rs

This project provides a command-line tool to print the structure of a directory in a tree format, excluding specified directories (e.g., `target`, `.git`, `.venv`, `node_modules`, `build`). The tool visually organizes directories and files, with different colors for files, directories, and hidden files. 

## Features

- Recursively traverses directories.
- Excludes specified directories (customizable).
- Supports colorized output for better readability.
- Differentiates between files, directories, and hidden files.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50 or later)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/ShivangSrivastava/tree-rs.git
   ```

2. Navigate to the project directory:
   ```bash
   cd tree-rs
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```
4. Add to /usr/local/bin
    ```bash
    sudo mv target/release/tree /usr/local/bin
    ```
    
### One-Step Installation
   ```bash
   curl https://raw.githubusercontent.com/ShivangSrivastava/tree-rs/refs/heads/main/install.sh | sh
   ```
    
## Usage

To run the tool, simply execute:

```bash
cargo run
```

This will print the directory tree of the current directory (`.`) excluding the following directories:

- `target`
- `.git`
- `.venv`
- `node_modules`
- `build`

### Example Output:

```bash
my_project
├── src/
│   ├── main.rs
│   └── utils.rs
├── target/
└── README.md
```

### Excluding Custom Directories:

You can customize the excluded directories by modifying the `exclude` vector in the `main()` function of the code.

```rust

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

```

## Dependencies

- [colorized](https://crates.io/crates/colorized) (for colorized output)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

