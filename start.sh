#!/bin/bash

set -e

BIN_PATH="./target/release/sapphire"

if [ -f "$BIN_PATH" ]; then
    echo "Running binary..."
    $BIN_PATH
else
    echo "Binary not found. Running build.sh to build the project..."
    ./tools/build.sh
    if [ -f "$BIN_PATH" ]; then
        echo "Build succeeded. Running binary..."
        $BIN_PATH
    else
        echo "Build failed. Binary still not found."
        exit 1
    fi
fi
