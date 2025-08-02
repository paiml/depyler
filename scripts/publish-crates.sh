#!/bin/bash
# Publish all Depyler crates to crates.io in dependency order

set -euo pipefail

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}=== Publishing Depyler Crates to crates.io ===${NC}"

# Function to publish with retry
publish_crate() {
    local crate=$1
    echo -e "\n${YELLOW}Publishing $crate...${NC}"
    cd crates/$crate
    
    # Retry logic for crates.io rate limits
    for i in {1..3}; do
        if cargo publish --no-verify; then
            echo -e "${GREEN}✅ Successfully published $crate${NC}"
            break
        else
            if [ $i -eq 3 ]; then
                echo -e "${RED}❌ Failed to publish $crate after 3 attempts${NC}"
                exit 1
            fi
            echo -e "${YELLOW}Attempt $i failed, waiting 30s...${NC}"
            sleep 30
        fi
    done
    
    cd ../..
    sleep 30  # Wait for crates.io indexing
}

# Check if we have CARGO_REGISTRY_TOKEN
if [ -z "${CARGO_REGISTRY_TOKEN:-}" ]; then
    echo -e "${RED}Error: CARGO_REGISTRY_TOKEN not set${NC}"
    echo "Please set your crates.io token:"
    echo "  export CARGO_REGISTRY_TOKEN=<your-token>"
    exit 1
fi

# Publish in dependency order
publish_crate depyler-annotations
publish_crate depyler-core
publish_crate depyler-analyzer
publish_crate depyler-verify
publish_crate depyler-quality
publish_crate depyler-mcp
publish_crate depyler-wasm
publish_crate depyler

echo -e "\n${GREEN}✨ All crates published successfully!${NC}"

# Verify installation
echo -e "\n${YELLOW}Waiting for crates.io propagation...${NC}"
sleep 60

echo -e "\n${YELLOW}Testing installation...${NC}"
if cargo install depyler --force; then
    echo -e "${GREEN}✅ Installation successful${NC}"
    depyler --version
else
    echo -e "${YELLOW}⚠️  Installation might need more time to propagate${NC}"
fi

echo -e "\n${BLUE}View published crates at:${NC}"
echo "https://crates.io/crates/depyler"