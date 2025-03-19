@echo off

REM Create releases directory
mkdir releases

REM Windows (64-bit)
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
mkdir releases\windows
copy target\x86_64-pc-windows-msvc\release\breakout.exe releases\windows\
xcopy /E /I res releases\windows\res

REM Linux (64-bit)
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
mkdir releases\linux
copy target\x86_64-unknown-linux-gnu\release\breakout releases\linux\
xcopy /E /I res releases\linux\res

REM macOS (64-bit)
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
mkdir releases\macos
copy target\x86_64-apple-darwin\release\breakout releases\macos\
xcopy /E /I res releases\macos\res