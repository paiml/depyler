#!/usr/bin/env bash
# DEPYLER-0970: Trace compilation with Renacer for debugging type inference
# Usage: ./scripts/trace_compile.sh <python_file> [options]
#
# This script:
# 1. Transpiles Python to Rust with source maps
# 2. Attempts to compile with Renacer tracing
# 3. Dumps type inference telemetry on failure
# 4. Correlates errors back to Python source

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Renacer binary location (fallback to system PATH)
RENACER="${RENACER:-/home/noah/src/renacer/target/release/renacer}"
if [[ ! -f "$RENACER" ]]; then
    RENACER='renacer'
fi

# Check if renacer is available
check_renacer() {
    if ! command -v "$RENACER" >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  Renacer not found. Building...${NC}"
        if [[ -d '/home/noah/src/renacer' ]]; then
            (cd /home/noah/src/renacer && cargo build --release --quiet)
            RENACER='/home/noah/src/renacer/target/release/renacer'
        else
            echo -e "${RED}❌ Cannot find renacer source. Install with: cargo install renacer${NC}"
            exit 1
        fi
    fi
}

# Check arguments
if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <python_file> [--strict|--trace-syscalls|--telemetry-only]"
    echo ''
    echo 'Options:'
    echo '  --strict           Enable strict mode (panic on Unknown types)'
    echo '  --trace-syscalls   Trace syscalls during compilation'
    echo '  --telemetry-only   Only dump telemetry, do not trace'
    echo '  --verbose          Verbose output with full traces'
    echo ''
    echo 'Examples:'
    echo "  $0 examples/example_subprocess/task_runner.py"
    echo "  $0 examples/example_argparse/cli.py --strict"
    echo ''
    echo 'Environment:'
    echo '  DEPYLER_STRICT=1   Enable strict type checking'
    echo '  RUST_LOG=debug     Enable debug logging'
    exit 1
fi

PYTHON_FILE="$1"
MODE="${2:---default}"

if [[ ! -f "$PYTHON_FILE" ]]; then
    echo -e "${RED}❌ File not found: $PYTHON_FILE${NC}"
    exit 1
fi

# Create temp directory for build artifacts
WORK_DIR=$(mktemp -d)
# SEC011: Validate WORK_DIR before using in trap
if [[ -z "${WORK_DIR}" || "${WORK_DIR}" == "/" ]]; then
    echo -e "${RED}❌ Invalid WORK_DIR: ${WORK_DIR}${NC}"
    exit 1
fi
cleanup() {
    # SEC011: Validate before rm -rf
    local dir="${WORK_DIR:-}"
    if [[ -n "$dir" && "$dir" != "/" && -d "$dir" ]]; then
        rm -rf "$dir"
    fi
}
trap cleanup EXIT

BASENAME=$(basename "$PYTHON_FILE" .py)
RS_FILE="${WORK_DIR}/${BASENAME}.rs"
SOURCEMAP="${WORK_DIR}/${BASENAME}.rs.sourcemap.json"
# Telemetry is dumped to /tmp by the transpiler, not this file
COMPILE_LOG="${WORK_DIR}/compile.log"

echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     DEPYLER-0970: Compile Trace with Renacer                 ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ''
echo -e "📄 Input:    ${YELLOW}$PYTHON_FILE${NC}"
echo -e "📂 Work Dir: $WORK_DIR"
echo -e "🔧 Mode:     $MODE"
echo ''

# Build depyler if needed
echo -e "${YELLOW}📦 Building depyler...${NC}"
cargo build --release --quiet 2>/dev/null || cargo build --release

# Set environment for strict mode if requested
if [[ "$MODE" == '--strict' ]]; then
    export DEPYLER_STRICT=1
    export RUST_BACKTRACE=1
fi

# Step 1: Transpile with source map
echo -e "${YELLOW}🔄 Step 1: Transpiling Python → Rust...${NC}"
if ./target/release/depyler transpile "$PYTHON_FILE" --source-map -o "$RS_FILE" 2>&1; then
    echo -e "${GREEN}   ✅ Transpilation successful${NC}"
    echo -e "   📄 Generated: $RS_FILE"
    if [[ -f "$SOURCEMAP" ]]; then
        echo -e "   🗺️  Source map: $SOURCEMAP"
    fi
else
    TRANSPILE_EXIT=$?
    echo -e "${RED}   ❌ Transpilation failed (exit $TRANSPILE_EXIT)${NC}"

    # Dump telemetry if available
    if [[ -f '/tmp/depyler_unknown_types.json' ]]; then
        echo -e "${YELLOW}   📊 Type inference telemetry:${NC}"
        head -50 /tmp/depyler_unknown_types.json
    fi
    exit "$TRANSPILE_EXIT"
fi

# Show generated Rust code
echo ''
echo -e "${CYAN}Generated Rust (first 50 lines):${NC}"
head -50 "$RS_FILE" | nl -ba
echo '...'
echo ''

# Step 2: Attempt compilation
echo -e "${YELLOW}🔧 Step 2: Compiling Rust...${NC}"

# Check if Renacer tracing is requested
case "$MODE" in
    --trace-syscalls)
        check_renacer
        echo -e '   🔍 Tracing syscalls with Renacer...'
        if [[ -f "$SOURCEMAP" ]]; then
            "$RENACER" --transpiler-map "$SOURCEMAP" -T -- rustc --edition 2021 "$RS_FILE" -o "${WORK_DIR}/${BASENAME}" 2>&1 | tee "$COMPILE_LOG"
        else
            "$RENACER" -T -- rustc --edition 2021 "$RS_FILE" -o "${WORK_DIR}/${BASENAME}" 2>&1 | tee "$COMPILE_LOG"
        fi
        ;;

    --telemetry-only)
        echo -e '   📊 Compiling with telemetry capture only...'
        rustc --edition 2021 "$RS_FILE" -o "${WORK_DIR}/${BASENAME}" 2>&1 | tee "$COMPILE_LOG" || true
        ;;

    *)
        echo -e '   🔨 Standard compilation...'
        rustc --edition 2021 "$RS_FILE" -o "${WORK_DIR}/${BASENAME}" 2>&1 | tee "$COMPILE_LOG" || true
        ;;
esac

# Step 3: Analyze results
echo ''
if [[ -f "${WORK_DIR}/${BASENAME}" ]]; then
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  ✅ COMPILATION SUCCESSFUL                                   ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ''
    echo -e "Binary: ${WORK_DIR}/${BASENAME}"
    file "${WORK_DIR}/${BASENAME}"
else
    echo -e "${RED}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║  ❌ COMPILATION FAILED                                       ║${NC}"
    echo -e "${RED}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ''

    # Extract error codes
    echo -e "${YELLOW}Error Analysis:${NC}"
    # shellcheck disable=SC2062  # Pattern is correct for grep -E
    grep -oE 'error\[E[0-9]+\]' "$COMPILE_LOG" | sort | uniq -c | sort -rn || true
    echo ''

    # Show errors with context
    echo -e "${YELLOW}Errors with context:${NC}"
    # shellcheck disable=SC2062
    grep -A3 'error\[E' "$COMPILE_LOG" 2>/dev/null | head -30 || true
    echo ''

    # If source map exists, correlate errors to Python
    if [[ -f "$SOURCEMAP" && "$MODE" == '--trace-syscalls' ]]; then
        echo -e "${YELLOW}Python Source Correlation (via source map):${NC}"
        # Parse error line numbers and map to Python
        # shellcheck disable=SC2062,SC2154  # Pattern correct, loc assigned by read
        grep -oE "${BASENAME}\.rs:[0-9]+" "$COMPILE_LOG" 2>/dev/null | while IFS= read -r loc; do
            line_num="${loc##*:}"
            echo "  Rust line ${line_num} → Python: (see source map)"
        done || true
    fi

    # Check for type inference issues
    if grep -q 'serde_json::Value' "$COMPILE_LOG" || grep -q 'Unknown' "$COMPILE_LOG"; then
        echo -e "${RED}⚠️  Type inference issue detected!${NC}"
        echo '   This may indicate incomplete type inference.'
        echo '   Try running with --strict to fail fast on Unknown types.'
    fi
fi

# Dump telemetry summary
if [[ -f '/tmp/depyler_unknown_types.json' ]]; then
    echo ''
    echo -e "${CYAN}Type Inference Telemetry Summary:${NC}"
    if command -v jq >/dev/null 2>&1; then
        jq -r '.[] | "  \(.expr_kind): \(.context // "no context")"' /tmp/depyler_unknown_types.json 2>/dev/null | head -20 || head -20 /tmp/depyler_unknown_types.json
    else
        head -20 /tmp/depyler_unknown_types.json
    fi
fi

echo ''
echo -e "${GREEN}Done! Work directory: $WORK_DIR${NC}"
