#!/usr/bin/env bash
# Profile Depyler test suite using Renacer
# Usage: ./scripts/profile_tests.sh [test_name] [--slow-only]

set -euo pipefail

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Check if renacer is installed
if ! command -v renacer &> /dev/null; then
    echo -e "${RED}❌ Renacer not found!${NC}"
    echo "Install with: cargo install renacer"
    exit 1
fi

TEST_NAME="${1:-}"
MODE="${2:---default}"

if [ "$MODE" == "--slow-only" ]; then
    echo -e "${GREEN}🐌 Finding slow tests (>100ms)...${NC}"
    echo ""

    if [ -n "$TEST_NAME" ]; then
        renacer --function-time -- cargo test "$TEST_NAME" 2>&1 | \
            grep -E "test.*ok" | \
            awk '{if ($4 > 0.1) print}' | \
            sort -k4 -rn
    else
        renacer --function-time -- cargo test --workspace 2>&1 | \
            grep -E "test.*ok" | \
            awk '{if ($4 > 0.1) print}' | \
            sort -k4 -rn | \
            head -20
    fi
else
    if [ -n "$TEST_NAME" ]; then
        echo -e "${GREEN}🔍 Profiling test: $TEST_NAME${NC}"
        echo ""
        renacer --function-time --source -- cargo test "$TEST_NAME" -- --nocapture
    else
        echo -e "${GREEN}🔍 Profiling entire test suite...${NC}"
        echo ""
        renacer --function-time -- cargo test --workspace
    fi
fi

echo ""
echo -e "${GREEN}✅ Test profiling complete!${NC}"
