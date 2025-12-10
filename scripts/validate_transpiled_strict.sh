#!/bin/bash
#
# Strict validation of transpiled examples using direct rustc compilation
# This catches issues that cargo clippy misses (examples/ not in workspace)
#
# Ticket: DEPYLER-0095
# Purpose: Find transpiler code generation issues

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo "Finding all transpiled .rs files in examples/..."
echo ""

# Find all .rs files that have corresponding .py files (transpiled)
TOTAL=0
PASSED=0
FAILED=0
WARNINGS_FOUND=0

declare -a FAILED_FILES
declare -a WARNING_COUNTS

for rs_file in examples/**/*.rs examples/*.rs; do
    # Skip if file doesn't exist
    [ -f "$rs_file" ] || continue
    
    # Check if corresponding .py exists (i.e., it's transpiled)
    py_file="${rs_file%.rs}.py"
    
    if [ ! -f "$py_file" ]; then
        # Skip manually written .rs files
        continue
    fi
    
    TOTAL=$((TOTAL + 1))
    basename=$(basename "$rs_file")
    
    printf "[%2d] Checking %-50s ... " "$TOTAL" "$(basename $rs_file)"
    
    # Check for Cargo.toml in the same directory
    dir=$(dirname "$rs_file")
    manifest="$dir/Cargo.toml"
    
    if [ -f "$manifest" ]; then
        # Use cargo check if manifest exists (handles dependencies)
        # We use --quiet to suppress progress bars but keep warnings
        # We need to capture stderr because cargo prints warnings there
        if cargo check --manifest-path "$manifest" --quiet --message-format=short > "/tmp/depyler_check_$$" 2>"/tmp/depyler_err_$$"; then
            # Cargo check succeeded (no errors), now check for warnings in stderr
            warning_count=$(grep "warning:" "/tmp/depyler_err_$$" | wc -l | tr -d ' ')
        else
            # Cargo check failed (errors occurred)
            # Treat this as a failure, even if we are only counting warnings normally
            warning_count="ERROR"
        fi
    else
        # Fallback to rustc for standalone files (no dependencies)
        rustc --crate-type lib "$rs_file" -o "/tmp/depyler_check_$$" 2>"/tmp/depyler_err_$$" || true
        warning_count=$(grep "^warning:" "/tmp/depyler_err_$$" 2>/dev/null | wc -l | tr -d ' ')
    fi

    [ -z "$warning_count" ] && warning_count=0

    if [ "$warning_count" = "ERROR" ]; then
        echo -e "${RED}❌ BUILD FAIL${NC}"
        FAILED=$((FAILED + 1))
        FAILED_FILES+=("$rs_file")
        WARNING_COUNTS+=("Build Error")
        
        # Show build errors
        echo -e "${YELLOW}  Build errors:${NC}"
        cat "/tmp/depyler_err_$$" | head -5 | sed 's/^/    /'
        
    elif [ "$warning_count" -eq 0 ]; then
        echo -e "${GREEN}✅ PASS${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}❌ FAIL${NC} ($warning_count warnings)"
        
        FAILED=$((FAILED + 1))
        FAILED_FILES+=("$rs_file")
        WARNING_COUNTS+=("$warning_count")
        WARNINGS_FOUND=$((WARNINGS_FOUND + warning_count))
        
        # Show first 3 warnings
        echo -e "${YELLOW}  Sample warnings:${NC}"
        grep "warning:" "/tmp/depyler_err_$$" | head -3 | sed 's/^/    /'
    fi
    
    # Cleanup
    rm -f "/tmp/depyler_check_$$" "/tmp/depyler_err_$$"
done

echo ""
echo "=========================================="
echo "📊 STRICT VALIDATION RESULTS"
echo "=========================================="
echo "Total transpiled examples: $TOTAL"

if [ $TOTAL -gt 0 ]; then
    echo -e "Passed: ${GREEN}$PASSED${NC} ($((PASSED * 100 / TOTAL))%)"
    echo -e "Failed: ${RED}$FAILED${NC} ($((FAILED * 100 / TOTAL))%)"
else
    echo "No transpiled examples found!"
    exit 0
fi

echo "Total warnings found: $WARNINGS_FOUND"
echo ""

if [ $FAILED -gt 0 ]; then
    echo "=========================================="
    echo -e "${RED}🛑 STOP THE LINE${NC}"
    echo "=========================================="
    echo ""
    echo "Transpiler has code generation quality issues!"
    echo ""
    echo "Failed files:"
    for i in "${!FAILED_FILES[@]}"; do
        printf "  %2d. %-50s %s warnings\n" $((i+1)) "${FAILED_FILES[$i]}" "${WARNING_COUNTS[$i]}"
    done
    echo ""
    echo "📋 See ticket: DEPYLER-0095 in docs/execution/roadmap.md"
    echo "📖 See protocol: 'Stop the Line' section in CLAUDE.md"
    echo ""
    echo "Next steps:"
    echo "  1. Analyze warnings (run: rustc --crate-type lib <file.rs>)"
    echo "  2. Fix TRANSPILER code generation (not output files!)"
    echo "  3. Re-transpile all examples: depyler transpile examples/**/*.py"
    echo "  4. Re-run: make validate-transpiled-strict"
    echo ""
    exit 1
else
    echo "=========================================="
    echo -e "${GREEN}✅ ALL EXAMPLES PASS STRICT VALIDATION!${NC}"
    echo "=========================================="
    echo ""
    echo "The transpiler generates perfect, idiomatic Rust code!"
    echo ""
fi
