#!/bin/bash
# Publishing script for kawat crates with rate-limiting delays
# Publishes crates in dependency order with 30-second delays between publishes

set -e

DELAY=30  # seconds between publishes
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if logged in to crates.io
check_login() {
    if [ ! -f ~/.cargo/credentials.toml ]; then
        log_error "Not logged in to crates.io. Run 'cargo login' first."
        exit 1
    fi
}

# Publish a single crate
publish_crate() {
    local crate_name=$1
    local crate_path=$2
    
    log_info "Publishing $crate_name..."
    
    cd "$crate_path"
    
    if cargo publish; then
        log_info "✓ $crate_name published successfully"
        return 0
    else
        log_error "✗ Failed to publish $crate_name"
        return 1
    fi
}

# Wait with countdown
wait_with_countdown() {
    local seconds=$1
    echo -n "Waiting $seconds seconds before next publish"
    
    for ((i=seconds; i>0; i--)); do
        echo -n "."
        sleep 1
    done
    echo " done"
}

main() {
    log_info "Starting kawat crate publishing process..."
    check_login
    
    cd "$WORKSPACE_ROOT"
    
    # Define crates in publishing order (dependency order)
    declare -a CRATES=(
        "htmldate-rs:crates/htmldate-rs"
        "kawat-html:crates/kawat-html"
        "kawat-xpath:crates/kawat-xpath"
        "kawat-extract:crates/kawat-extract"
        "kawat-metadata:crates/kawat-metadata"
        "kawat-dedup:crates/kawat-dedup"
        "kawat-output:crates/kawat-output"
        "kawat-readability:crates/kawat-readability"
        "kawat-justext:crates/kawat-justext"
        "kawat-core:crates/kawat-core"
        "kawat:crates/kawat"
    )
    
    local failed_crates=()
    
    for i in "${!CRATES[@]}"; do
        IFS=':' read -r crate_name crate_path <<< "${CRATES[$i]}"
        
        if publish_crate "$crate_name" "$crate_path"; then
            # Wait before next publish (except for the last one)
            if [ $((i + 1)) -lt ${#CRATES[@]} ]; then
                wait_with_countdown $DELAY
            fi
        else
            failed_crates+=("$crate_name")
        fi
    done
    
    # Summary
    echo ""
    log_info "Publishing complete!"
    
    if [ ${#failed_crates[@]} -eq 0 ]; then
        log_info "All crates published successfully! 🎉"
        return 0
    else
        log_error "Failed to publish the following crates:"
        for crate in "${failed_crates[@]}"; do
            echo "  - $crate"
        done
        return 1
    fi
}

main "$@"
