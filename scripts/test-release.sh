#!/bin/bash
# Test script for local release workflow testing

set -e

echo "Testing release process locally..."

# Build for all targets
targets=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
)

for target in "${targets[@]}"; do
    echo "Building for $target..."
    cargo build --release --target "$target"
done

# Create release directory
mkdir -p release

# Create archives
echo "Creating release archives..."
for target in "${targets[@]}"; do
    staging="release/kawat-$target"
    mkdir -p "$staging"
    
    if [[ "$target" == *"windows"* ]]; then
        cp "target/$target/release/kawat.exe" "$staging/"
        cd release
        7z a "kawat-$target.zip" "kawat-$target/"
        cd ..
    else
        cp "target/$target/release/kawat" "$staging/"
        tar czf "release/kawat-$target.tar.gz" -C "release" "kawat-$target/"
    fi
done

echo "Release artifacts created in release/ directory:"
ls -la release/

# Checksums
echo "Generating checksums..."
cd release
sha256sum * > SHA256SUMS
cat SHA256SUMS
cd ..

echo "Local release test complete!"
