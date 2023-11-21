#!/bin/sh

printf "\033[1;31m"
echo "Uninstalling cmake-init..."
printf "\033[0m\n"

# creating necessary directories
rm -v"$HOME/.local/bin/cmake-init"
rm -fm "$HOME/Library/Application Support/cmake-init"

printf "\033[1;31m"
echo "Uninstallation complete!"
printf "\033[0m\n"
