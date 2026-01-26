#!/bin/bash
# Verify semantic parity for test files
# DEPYLER-1361: Build the green list

set -e

GREEN_LIST=""
FAIL_LIST=""

verify_file() {
    local py_file="$1"
    local base=$(basename "$py_file" .py)
    local dir=$(dirname "$py_file")
    local rs_file="$dir/$base.rs"
    local bin_file="$dir/${base}_bin"

    echo "Testing: $base"

    # Get Python output
    python3 "$py_file" > /tmp/py_out.txt 2>&1 || { echo "  Python FAILED"; return 1; }

    # Transpile
    cargo run --release --bin depyler -- transpile "$py_file" -o "$rs_file" 2>/dev/null || { echo "  Transpile FAILED"; return 1; }

    # Compile
    rustc "$rs_file" -o "$bin_file" --edition 2021 2>/dev/null || { echo "  Compile FAILED"; return 1; }

    # Get Rust output
    "$bin_file" > /tmp/rs_out.txt 2>&1 || { echo "  Rust run FAILED"; return 1; }

    # Compare
    if diff -q /tmp/py_out.txt /tmp/rs_out.txt > /dev/null 2>&1; then
        echo "  PASS (semantic parity)"
        GREEN_LIST="$GREEN_LIST $base"
        return 0
    else
        echo "  FAIL (semantic mismatch)"
        echo "  Python: $(cat /tmp/py_out.txt)"
        echo "  Rust:   $(cat /tmp/rs_out.txt)"
        FAIL_LIST="$FAIL_LIST $base"
        return 1
    fi
}

echo "=============================================="
echo "SEMANTIC PARITY VERIFICATION (DEPYLER-1361)"
echo "=============================================="
echo

cd /Users/noahgift/src/depyler

for f in examples/semantic_test_*.py; do
    verify_file "$f" || true
    echo
done

echo "=============================================="
echo "GREEN LIST (Verified Semantic Parity)"
echo "=============================================="
for f in $GREEN_LIST; do
    echo "  + $f"
done

if [ -n "$FAIL_LIST" ]; then
    echo
    echo "FAIL LIST (Semantic Mismatch)"
    echo "=============================================="
    for f in $FAIL_LIST; do
        echo "  - $f"
    done
fi

echo
echo "Total: $(echo $GREEN_LIST | wc -w | tr -d ' ') passed"
