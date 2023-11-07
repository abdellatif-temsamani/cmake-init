@echo off

cargo build --release

mkdir "%APPDATA%\\cmake-init\\templates"
xcopy /s /y "templates" "%APPDATA%\\cmake-init\\templates"


REM cope target\release\cmake-init.exe to %USERPROFILE%\bin
mkdir "%USERPROFILE%\\bin"
xcopy /s /y "target\\release\\cmake-init.exe" "%USERPROFILE%\\bin\\cmake-init.exe"


REM add %USERPROFILE%\bin to PATH
setx PATH "%USERPROFILE%\\bin;%PATH%"
