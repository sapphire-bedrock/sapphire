#!/bin/bash

set -e

if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "Cleaning previous builds..."
cargo clean

echo "Formatting code..."
cargo fmt

echo "Building in release mode..."
cargo build --release
