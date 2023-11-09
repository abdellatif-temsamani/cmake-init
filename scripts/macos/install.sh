#!/bin/sh

# building cmake-init
cargo build --release

# creating necessary directories
mkdir -p "$HOME/.local/bin/"
mkdir -p "$HOME/Library/Application Support/cmake-init/templates"


# copying files
cp ./target/release/cmake-init "$HOME/.local/bin/"
cp -r ./templates "$HOME/Library/Application Support/cmake-init/"

echo "export PATH=$PATH:~/.local/bin" >> ~/.bashrc

echo "Installation complete!"
