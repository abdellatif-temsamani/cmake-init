@echo off

echo "Installing cmake-init"

cargo build --release

mkdir "%APPDATA%\\cmake-init\\templates"
xcopy /s /y "templates" "%APPDATA%\\cmake-init\\templates"

mkdir "%USERPROFILE%\\bin"
xcopy /s /y "target\\release\\cmake-init.exe" "%USERPROFILE%\\bin\\cmake-init.exe"

echo "to add cmake-init to your path, run the following command:"
echo "setx PATH "%USERPROFILE%\\bin;%PATH%""

echo "Installation complete!"
