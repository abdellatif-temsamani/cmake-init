#!/bin/sh

# building cmake-init
cargo build --release

# creating necessary directories
mkdir -p "$HOME/.local/bin/"
mkdir -p "$HOME/.local/share/cmake-init/"


# copying files
cp ./target/release/cmake-init ~/.local/bin/
cp -r ./templates ~/.local/share/cmake-init/

echo "export PATH=$PATH:~/.local/bin" >> ~/.bashrc

echo "Installation complete!"
