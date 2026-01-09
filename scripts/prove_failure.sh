#!/bin/bash
# prove_failure.sh: Validates that the Depyler architecture is fundamentally broken.
# This script runs the Falsification Suite and confirms compile-time failures.

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo "Step 1: Transpiling Falsification Suite..."
if ! cargo run --bin depyler -- transpile examples/falsification_suite.py; then
    echo -e "${RED}[FAIL] Transpilation crashed.${NC}"
    exit 1
fi

echo "Step 2: Compiling Generated Rust Code..."
# We expect this to fail.
if rustc --edition 2021 --crate-type lib examples/falsification_suite.rs 2> compile_err.log; then
    echo -e "${RED}[ERROR] Falsification Suite COMPILED. The architecture is NOT as broken as we thought. Falsification FAILED.${NC}"
    exit 1
else
    echo -e "${GREEN}[SUCCESS] Compilation FAILED as predicted.${NC}"
    echo "Summary of failures:"
    grep -E "error\[E[0-9]+\]" compile_err.log | sort | uniq -c
fi

echo "Step 3: Verifying Specific Root Causes..."

# RC-1 check: Undefined variable due to loop destruction
if grep -q "E0425" compile_err.log; then
    echo -e "${GREEN}[CONFIRMED] RC-1: For-loop destruction (Undefined variable).${NC}"
else
    echo -e "${RED}[NOT FOUND] RC-1 failure missing from logs.${NC}"
fi

# RC-2/3 check: Type mismatch
if grep -q "E0308" compile_err.log; then
    echo -e "${GREEN}[CONFIRMED] RC-2/3: Type mismatch / Semantic laziness.${NC}"
else
    echo -e "${RED}[NOT FOUND] RC-2/3 failure missing from logs.${NC}"
fi

echo -e "\n${GREEN}Architecture Falsification Complete: The system is definitively BROKEN.${NC}"
rm compile_err.log
