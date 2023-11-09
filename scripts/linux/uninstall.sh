#!/bin/sh

printf "\033[1;31m"
echo "Uninstalling cmake-init..."
printf "\033[0m\n"

# creating necessary directories
rm -v "$HOME/.local/bin/cmake-init" 2>/dev/null
rm -rvf "$HOME/.local/share/cmake-init/" 2>/dev/null

printf "\033[1;31m"
echo "Uninstallation complete!"
printf "\033[0m\n"
