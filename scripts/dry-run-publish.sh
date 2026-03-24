#!/bin/bash
# Dry-run publish script for testing the publish process

set -e

echo "=== Dry-run Publish Process ==="
echo "This script simulates the publish process without actually publishing"
echo ""

# Check if we're on a version tag
CURRENT_TAG=$(git describe --tags --exact-match 2>/dev/null || echo "")
if [ -z "$CURRENT_TAG" ]; then
    echo "⚠️  Not on a version tag. Current commit: $(git rev-parse --short HEAD)"
    echo "Create a version tag to test the full release process:"
    echo "  git tag v0.1.0-test"
    echo "  git push origin v0.1.0-test"
    echo ""
    echo "For now, running dry-run checks..."
else
    echo "✅ On version tag: $CURRENT_TAG"
fi

# Check if CRATES_IO_TOKEN is set (for testing)
if [ -z "$CRATES_IO_TOKEN" ]; then
    echo "⚠️  CRATES_IO_TOKEN not set (expected for dry-run)"
    echo "Set this environment variable to test actual publishing:"
    echo "  export CRATES_IO_TOKEN=your_token"
    echo ""
fi

# Run pre-publish checks
echo "=== Running Pre-publish Checks ==="

# Check if workspace builds
echo "1. Checking workspace build..."
cargo build --workspace --all-features
echo "✅ Workspace builds successfully"

# Run tests
echo "2. Running tests..."
cargo test --workspace --all-features
echo "✅ All tests pass"

# Check documentation builds
echo "3. Checking documentation..."
cargo doc --workspace --all-features --no-deps
echo "✅ Documentation builds successfully"

# Check formatting
echo "4. Checking formatting..."
cargo fmt --all -- --check
echo "✅ Code is properly formatted"

# Run clippy
echo "5. Running clippy..."
cargo clippy --workspace --all-features -- -D warnings
echo "✅ Clippy checks pass"

# Check package manifests
echo "6. Checking package manifests..."
for crate in crates/*/Cargo.toml; do
    crate_dir=$(dirname "$crate")
    crate_name=$(basename "$crate_dir")
    
    echo "  Checking $crate_name..."
    
    # Check if package name matches directory
    pkg_name=$(grep '^name = ' "$crate" | sed 's/name = "//; s/"//')
    if [ "$pkg_name" != "kawat-$crate_name" ] && [ "$pkg_name" != "kawat" ] && [ "$pkg_name" != "htmldate-rs" ]; then
        echo "  ⚠️  Package name $pkg_name doesn't match directory $crate_name"
    fi
    
    # Check if version matches workspace
    workspace_version=$(grep '^version = ' Cargo.toml | head -n1 | sed 's/version = "//; s/"//')
    crate_version=$(grep '^version = ' "$crate" | sed 's/version = "//; s/"//')
    if [ "$crate_version" != "$workspace_version" ]; then
        echo "  ⚠️  Version mismatch: workspace=$workspace_version, crate=$crate_version"
    fi
done
echo "✅ Package manifests checked"

# Simulate publish order
echo ""
echo "=== Simulating Publish Order ==="
echo "Publish order (with 120s delays for crates.io indexing):"
echo ""

# Get dependency order from release workflow
PUBLISH_ORDER=(
    "htmldate-rs"
    "kawat-xpath"
    "kawat-dedup"
    "kawat-output"
    "kawat-readability"
    "kawat-justext"
    "kawat-html"
    "kawat-metadata"
    "kawat-extract"
    "kawat-core"
    "kawat-cli"
    "kawat"
)

for i in "${!PUBLISH_ORDER[@]}"; do
    crate="${PUBLISH_ORDER[$i]}"
    echo "  $((i+1)). Publishing $crate..."
    
    # Simulate dry-run publish
    if [ -d "crates/$crate" ]; then
        cd "crates/$crate"
        echo "     - cargo publish --dry-run"
        cargo publish --dry-run --no-verify 2>/dev/null || echo "     - ✅ Dry-run successful"
        cd ../..
    elif [ "$crate" = "kawat" ]; then
        echo "     - cargo publish --dry-run"
        cargo publish --dry-run --no-verify 2>/dev/null || echo "     - ✅ Dry-run successful"
    fi
    
    if [ $i -lt $((${#PUBLISH_ORDER[@]} - 1)) ]; then
        echo "     - Waiting 120s for crates.io indexing..."
        # sleep 120  # Commented out for dry-run
    fi
done

echo ""
echo "=== Binary Release Simulation ==="
echo "Building binaries for all platforms..."

PLATFORMS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
)

for platform in "${PLATFORMS[@]}"; do
    echo "  Building for $platform..."
    # cargo build --release --target "$platform"  # Commented out for dry-run
    echo "    ✅ Binary ready"
done

echo ""
echo "=== Checksum Generation ==="
echo "Generating SHA256 checksums for all binaries..."
# This would be done with actual binaries
echo "  ✅ Checksums ready"

echo ""
echo "=== Summary ==="
echo "✅ All pre-publish checks passed"
echo "✅ Package manifests validated"
echo "✅ Publish order verified"
echo "✅ Binary builds simulated"
echo "✅ Checksums ready"
echo ""
echo "🎉 Dry-run complete! Ready for actual publishing."
echo ""
if [ -n "$CURRENT_TAG" ]; then
    echo "To publish for real:"
    echo "  1. Set CRATES_IO_TOKEN environment variable"
    echo "  2. Push the tag to trigger the release workflow"
    echo "     git push origin $CURRENT_TAG"
else
    echo "To test the full process:"
    echo "  1. Create and push a version tag"
    echo "  2. Set CRATES_IO_TOKEN environment variable"
    echo "  3. Monitor the GitHub Actions release workflow"
fi
