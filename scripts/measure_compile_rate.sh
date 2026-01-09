#!/bin/bash
# measure_compile_rate.sh: Measures the actual compile rate of the examples.

EXAMPLES_DIR="examples"
SUCCESS_COUNT=0
TOTAL_COUNT=0
TEMP_DIR=".compile_test_tmp"

mkdir -p "$TEMP_DIR"

echo "Measuring Post-Fix Compile Rate..."
echo "=================================="

# Find all .py files in examples/ (including subdirectories)
while IFS= read -r py_file; do
    ((TOTAL_COUNT++))
    echo -n "[$TOTAL_COUNT] Compiling $py_file... "
    
    # 1. Transpile
    if ! cargo run --quiet --bin depyler -- transpile "$py_file" --output "$TEMP_DIR/test.rs" >/dev/null 2>&1; then
        echo "✗ TRANSPILE FAIL"
        continue
    fi
    
    # 2. Compile with rustc
    if rustc --edition 2021 --crate-type lib "$TEMP_DIR/test.rs" --out-dir "$TEMP_DIR" >/dev/null 2>&1; then
        echo "✓ PASS"
        ((SUCCESS_COUNT++))
    else
        echo "✗ COMPILE FAIL"
    fi
done < <(find "$EXAMPLES_DIR" -maxdepth 2 -name "*.py")

echo ""
echo "Final Results:"
echo "=============="
echo "Successful: $SUCCESS_COUNT"
echo "Total:      $TOTAL_COUNT"
echo "Rate:       $(awk "BEGIN {printf \"%.2f\", $SUCCESS_COUNT/$TOTAL_COUNT*100}%")"

# Update the status document if success count changed
echo "Baseline was 17/60 (28%). Current: $SUCCESS_COUNT/$TOTAL_COUNT"

rm -rf "$TEMP_DIR"
