@echo off

cargo build --release

echo "Uninstalling cmake-init..."

rmdir /s /q "%APPDATA%\\cmake-init\\templates"

rmdir /s /q "%USERPROFILE%\\bin\\cmake-init.exe"


echo "Uninstallation complete!"
