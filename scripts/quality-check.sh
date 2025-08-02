#!/bin/bash
# Quality check script for Depyler v1.0.1

set -euo pipefail

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔍 Running Depyler Quality Checks"
echo "================================"

# 1. Format check
echo -e "\n${YELLOW}Checking code formatting...${NC}"
if cargo fmt --all -- --check; then
    echo -e "${GREEN}✓ Code formatting is correct${NC}"
else
    echo -e "${RED}✗ Code needs formatting${NC}"
    echo "Run: cargo fmt --all"
    exit 1
fi

# 2. Clippy check
echo -e "\n${YELLOW}Running clippy...${NC}"
if cargo clippy --workspace -- -D warnings; then
    echo -e "${GREEN}✓ No clippy warnings${NC}"
else
    echo -e "${RED}✗ Clippy found issues${NC}"
    exit 1
fi

# 3. Test suite
echo -e "\n${YELLOW}Running test suite...${NC}"
if cargo test --workspace; then
    echo -e "${GREEN}✓ All tests passed${NC}"
else
    echo -e "${RED}✗ Some tests failed${NC}"
    exit 1
fi

# 4. Doc tests
echo -e "\n${YELLOW}Running doc tests...${NC}"
if cargo test --doc; then
    echo -e "${GREEN}✓ Doc tests passed${NC}"
else
    echo -e "${RED}✗ Doc tests failed${NC}"
    exit 1
fi

# 5. Build documentation
echo -e "\n${YELLOW}Building documentation...${NC}"
if cargo doc --workspace --no-deps; then
    echo -e "${GREEN}✓ Documentation builds successfully${NC}"
else
    echo -e "${RED}✗ Documentation build failed${NC}"
    exit 1
fi

# 6. Check examples compile
echo -e "\n${YELLOW}Checking examples...${NC}"
if cargo check --examples; then
    echo -e "${GREEN}✓ All examples compile${NC}"
else
    echo -e "${RED}✗ Some examples failed to compile${NC}"
    exit 1
fi

echo -e "\n${GREEN}✨ All quality checks passed!${NC}"
echo "Ready for release v1.0.1"