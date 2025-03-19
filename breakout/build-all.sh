#!/bin/bash

# Create a directory for the releases
mkdir -p releases

# Windows (64-bit)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
mkdir -p releases/windows
cp target/x86_64-pc-windows-gnu/release/breakout.exe releases/windows/
cp -r res releases/windows/

# Linux (64-bit)
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
mkdir -p releases/linux
cp target/x86_64-unknown-linux-gnu/release/breakout releases/linux/
cp -r res releases/linux/

# macOS (64-bit)
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
mkdir -p releases/macos
cp target/x86_64-apple-darwin/release/breakout releases/macos/
cp -r res releases/macos/

# Create ZIP archives
cd releases
zip -r breakout-windows.zip windows/
zip -r breakout-linux.zip linux/
zip -r breakout-macos.zip macos/
cd ..