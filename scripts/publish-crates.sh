#!/bin/bash
# Script to publish all Depyler crates to crates.io in dependency order
# Run this script after fixing linker issues

set -e  # Exit on error

echo "Publishing Depyler v1.0.1 crates to crates.io..."
echo "Make sure you are logged in with: cargo login"
echo ""

# Function to publish a crate
publish_crate() {
    local crate_name=$1
    echo "Publishing $crate_name..."
    cd crates/$crate_name
    
    # Dry run first
    if cargo publish --dry-run; then
        echo "Dry run successful, publishing $crate_name..."
        cargo publish
        echo "Successfully published $crate_name"
        echo "Waiting 30 seconds for crates.io to index..."
        sleep 30
    else
        echo "Failed to publish $crate_name"
        exit 1
    fi
    
    cd ../..
}

# Publish in dependency order
publish_crate "depyler-annotations"
publish_crate "depyler-core"
publish_crate "depyler-analyzer"
publish_crate "depyler-verify"
publish_crate "depyler-quality"
publish_crate "depyler-mcp"
publish_crate "depyler-wasm"
publish_crate "depyler"

echo ""
echo "✅ All crates published successfully!"
echo "View at: https://crates.io/crates/depyler"