#!/bin/bash

# Script to download wasm files from wasmi-benchmarks repository
# Repository: https://github.com/wasmi-labs/wasmi-benchmarks/
# Commit: b9385cae9bfb8cf84dbb13996d0b948ca5826b53
# Target: benches/res/wasm/*.wasm

# Configuration
REPO="wasmi-labs/wasmi-benchmarks"
COMMIT="b9385cae9bfb8cf84dbb13996d0b948ca5826b53"
TARGET_DIR="$(cd "$(dirname "$0")" && pwd)"

# List of wasm files to download
WASM_FILES=(
    "benches/res/wasm/bz2.wasm"
    "benches/res/wasm/pulldown-cmark.wasm"
    "benches/res/wasm/spidermonkey.wasm"
    "benches/res/wasm/ffmpeg.wasm"
    "benches/res/wasm/coremark-minimal.wasm"
    "benches/res/wasm/argon2.wasm"
    "benches/res/wasm/erc20.wasm"
)

# Create target directory if it doesn't exist
mkdir -p "$TARGET_DIR"

echo "Downloading wasm files from $REPO at commit $COMMIT..."
echo "Target directory: $TARGET_DIR"
echo

# Download each file
for file in "${WASM_FILES[@]}"; do
    filename=$(basename "$file")
    url="https://raw.githubusercontent.com/$REPO/$COMMIT/$file"
    
    echo "Downloading $filename..."
    
    if curl -fsSL "$url" -o "$TARGET_DIR/$filename"; then
        echo "  ✓ $filename downloaded successfully"
    else
        echo "  ✗ Failed to download $filename"
        exit 1
    fi
done

echo
echo "All wasm files downloaded successfully!"
