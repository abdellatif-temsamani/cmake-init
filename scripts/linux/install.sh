#!/bin/sh

shell="$HOME/.bashrc"

if [ -z "$ZSH_VERSION" ]; then
    shell="$HOME/.zshrc"
else
    shell="$HOME/.bashrc"

fi

printf "\033[1;32m"
echo "Installing cmake-init"
printf "\033[0m\n"

# building cmake-init
cargo build --release

# creating necessary directories
mkdir -p "$HOME/.local/bin/"
mkdir -p "$HOME/.local/share/cmake-init/"


# copying files
cp ./target/release/cmake-init ~/.local/bin/
cp -r ./templates ~/.local/share/cmake-init/

echo "export PATH=$PATH:~/.local/bin" >> "$shell"

printf "\033[1;32m"
echo "Installation complete!"
printf "\033[0m\n"
