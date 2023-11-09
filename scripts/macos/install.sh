#!/bin/sh


printf "\033[1;32m"
echo "Installing cmake-init"
printf "\033[0m\n"

# building cmake-init
cargo build --release

# creating necessary directories
mkdir -p "$HOME/.local/bin/"
mkdir -p "$HOME/Library/Application Support/cmake-init/"


# copying files
cp ./target/release/cmake-init "$HOME/.local/bin/"
cp -r ./templates "$HOME/Library/Application Support/cmake-init/"

echo "Installation complete!"

printf "\033[1;32m"
echo "Installation complete!"
printf "\033[0m\n"
