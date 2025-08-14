#!/bin/bash
set -e

echo "Publishing depyler v2.3.0 to crates.io..."
echo "This will publish all workspace crates in dependency order."
echo

# Function to publish a crate
publish_crate() {
    local crate=$1
    echo "Publishing $crate..."
    cargo publish -p $crate --no-verify
    echo "Published $crate. Waiting 30 seconds for crates.io to index..."
    sleep 30
    echo
}

# Publish in dependency order
publish_crate depyler-annotations
publish_crate depyler-core
publish_crate depyler-analyzer
publish_crate depyler-verify
publish_crate depyler-mcp
publish_crate depyler-quality
publish_crate depyler-wasm
publish_crate depyler

echo "All crates published successfully!"
echo "Release v2.3.0 is now available on crates.io"