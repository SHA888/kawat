#!/bin/bash
# Verify which workspace crates are already published on crates.io

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="${1:-$(grep '^version' "$WORKSPACE_ROOT/Cargo.toml" | head -1 | sed 's/.*= "\([^"]*\)".*/\1/')}"

CRATES=(
    htmldate-rs
    kawat-html
    kawat-xpath
    kawat-extract
    kawat-metadata
    kawat-dedup
    kawat-output
    kawat-readability
    kawat-justext
    kawat-core
    kawat
    kawat-cli
)

echo "Checking crates.io for version $VERSION..."
echo ""

published=()
missing=()

for crate in "${CRATES[@]}"; do
    result=$(cargo search "$crate" --limit 5 2>/dev/null || true)
    version=$(echo "$result" | grep "^$crate =" | head -1 | sed 's/.*= "\([^"]*\)".*/\1/' || true)

    if [ "$version" = "$VERSION" ]; then
        printf "  ✓ %-22s %s\n" "$crate" "$version"
        published+=("$crate")
    elif [ -n "$version" ]; then
        printf "  ⚠ %-22s %s (want %s)\n" "$crate" "$version" "$VERSION"
    else
        printf "  ✗ %-22s not found\n" "$crate"
        missing+=("$crate")
    fi
done

echo ""
echo "Published: ${#published[@]}/${#CRATES[@]}"
echo "Missing:   ${#missing[@]}/${#CRATES[@]}"

if [ ${#missing[@]} -gt 0 ]; then
    echo ""
    echo "Missing crates:"
    for crate in "${missing[@]}"; do
        echo "  - $crate"
    done
    exit 1
fi

echo ""
echo "All crates published at version $VERSION"
