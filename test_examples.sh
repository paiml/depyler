#!/bin/bash

# Test script to verify examples work correctly

echo "Testing Depyler Examples..."
echo "=========================="

DEPYLER_BIN="cargo run --bin depyler --"
SUCCESS_COUNT=0
FAIL_COUNT=0
TESTED_EXAMPLES=(
    "examples/showcase/binary_search.py"
    "examples/mathematical/basic_math.py"
    "examples/array_test.py"
    "examples/dict_assign.py"
    "examples/simple_set.py"
    "examples/showcase/calculate_sum.py"
    "examples/showcase/classify_number.py"
    "examples/simple_class_test.py"
    "examples/test_imports.py"
    "examples/test_list_append.py"
)

for example in "${TESTED_EXAMPLES[@]}"; do
    echo -n "Testing $example... "
    if $DEPYLER_BIN transpile "$example" >/dev/null 2>&1; then
        echo "✓ SUCCESS"
        ((SUCCESS_COUNT++))
    else
        echo "✗ FAILED"
        ((FAIL_COUNT++))
    fi
done

echo ""
echo "Summary:"
echo "========"
echo "Successful: $SUCCESS_COUNT"
echo "Failed: $FAIL_COUNT"
echo "Total: $((SUCCESS_COUNT + FAIL_COUNT))"

if [[ $FAIL_COUNT -gt 0 ]]; then
    exit 1
fi