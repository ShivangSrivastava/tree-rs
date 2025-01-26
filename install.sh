
if ! command -v cargo &>/dev/null; then
  echo "Installing rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

echo "Cloning repository..."

git clone https://github.com/ShivangSrivastava/tree-rs

cd tree-rs

echo "Building project..."
cargo build --release

echo "Adding to /usr/local/bin/"
sudo cp target/release/tree /usr/local/bin/

echo "Now you are ready to go"

# Check if the current shell is bash
if [ -n "$BASH_VERSION" ]; then
    source ~/.bashrc
# Check if the current shell is zsh
elif [ -n "$ZSH_VERSION" ]; then
    source ~/.zshrc
else
    echo "Unsupported shell, reload shell manually."
fi
