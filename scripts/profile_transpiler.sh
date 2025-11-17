#!/usr/bin/env bash
# Profile Depyler transpiler using Renacer
# Usage: ./scripts/profile_transpiler.sh <python_file> [options]

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if renacer is installed
if ! command -v renacer &> /dev/null; then
    echo -e "${RED}❌ Renacer not found!${NC}"
    echo "Install with: cargo install renacer"
    exit 1
fi

# Check arguments
if [ $# -lt 1 ]; then
    echo "Usage: $0 <python_file> [--flamegraph|--io-only|--hot-functions]"
    echo ""
    echo "Options:"
    echo "  --flamegraph     Generate flamegraph SVG (requires flamegraph.pl in PATH)"
    echo "  --io-only        Show only I/O bottlenecks"
    echo "  --hot-functions  Show only functions >5% total time"
    echo ""
    echo "Examples:"
    echo "  $0 examples/benchmark.py"
    echo "  $0 examples/matrix_testing_project/07_algorithms/algorithms.py --flamegraph"
    echo "  $0 examples/example_stdlib.py --io-only"
    exit 1
fi

PYTHON_FILE="$1"
MODE="${2:---default}"

if [ ! -f "$PYTHON_FILE" ]; then
    echo -e "${RED}❌ File not found: $PYTHON_FILE${NC}"
    exit 1
fi

echo -e "${GREEN}🔍 Profiling Depyler transpilation${NC}"
echo "File: $PYTHON_FILE"
echo "Mode: $MODE"
echo ""

# Build release binary first
echo -e "${YELLOW}📦 Building release binary...${NC}"
cargo build --release --quiet

case "$MODE" in
    --flamegraph)
        if ! command -v flamegraph.pl &> /dev/null; then
            echo -e "${RED}❌ flamegraph.pl not found!${NC}"
            echo "Install from: https://github.com/brendangregg/FlameGraph"
            exit 1
        fi

        OUTPUT_FILE="transpile_$(basename "$PYTHON_FILE" .py)_flame.svg"
        echo -e "${YELLOW}🔥 Generating flamegraph: $OUTPUT_FILE${NC}"

        renacer --function-time -- cargo run --release -- transpile "$PYTHON_FILE" 2>&1 | \
            flamegraph.pl > "$OUTPUT_FILE"

        echo -e "${GREEN}✅ Flamegraph generated: $OUTPUT_FILE${NC}"

        # Try to open in browser
        if command -v xdg-open &> /dev/null; then
            xdg-open "$OUTPUT_FILE" 2>/dev/null || true
        elif command -v open &> /dev/null; then
            open "$OUTPUT_FILE" 2>/dev/null || true
        fi
        ;;

    --io-only)
        echo -e "${YELLOW}💾 Profiling I/O operations...${NC}"
        renacer --function-time --source -- cargo run --release -- transpile "$PYTHON_FILE" 2>&1 | \
            grep -A 100 "I/O Bottlenecks" || echo "No I/O bottlenecks detected"
        ;;

    --hot-functions)
        echo -e "${YELLOW}🔥 Finding hot functions (>5% total time)...${NC}"
        renacer --function-time -- cargo run --release -- transpile "$PYTHON_FILE" 2>&1 | \
            grep -A 20 "Top 10 Functions"
        ;;

    *)
        echo -e "${YELLOW}⏱️  Running full profile...${NC}"
        renacer --function-time --source -- cargo run --release -- transpile "$PYTHON_FILE"
        ;;
esac

echo ""
echo -e "${GREEN}✅ Profiling complete!${NC}"
