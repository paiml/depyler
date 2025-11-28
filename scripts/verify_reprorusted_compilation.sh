#!/bin/bash
# verify_reprorusted_compilation.sh
# QA script to verify all reprorusted examples compile
# Usage: ./scripts/verify_reprorusted_compilation.sh

set -e

EXAMPLES_DIR="/home/noah/src/reprorusted-python-cli/examples"
REPORT_FILE="/tmp/reprorusted_qa_report_$(date +%Y%m%d_%H%M%S).txt"

echo "=============================================="
echo "  Reprorusted Compilation Verification"
echo "=============================================="
echo "Date: $(date)"
echo "Depyler Version: $(cargo metadata --format-version 1 --no-deps 2>/dev/null | jq -r '.packages[] | select(.name=="depyler") | .version')"
echo ""
echo "Report: $REPORT_FILE"
echo ""

# Redirect all output to report file as well
exec > >(tee -a "$REPORT_FILE") 2>&1

failed=0
passed=0

echo "=== Step 1: Build All Examples ==="
echo ""

for example in simple complex config csv_filter environment flags io_streams log_analyzer positional regex stdlib subcommands subprocess; do
    dir="$EXAMPLES_DIR/example_$example"
    if [ -f "$dir/Cargo.toml" ]; then
        echo -n "Building example_$example... "
        if cargo build --manifest-path "$dir/Cargo.toml" 2>&1 | grep -q "^error"; then
            echo "FAIL"
            failed=$((failed + 1))
        else
            echo "PASS"
            passed=$((passed + 1))
        fi
    fi
done

echo ""
echo "=== Step 2: Verify Specific Fixes ==="
echo ""

# Check stdlib fix
echo -n "Checking stdlib_integration has_hash fix... "
if grep -q "has_hash = args.hash.is_some()" "$EXAMPLES_DIR/example_stdlib/stdlib_integration.rs"; then
    echo "PASS"
else
    echo "FAIL - Missing has_hash pre-computation"
    failed=$((failed + 1))
fi

# Check log_analyzer doesn't use serde_json::Value excessively
echo -n "Checking log_analyzer proper typing... "
value_count=$(grep -c "serde_json::Value" "$EXAMPLES_DIR/example_log_analyzer/log_analyzer.rs" 2>/dev/null || echo "0")
if [ "$value_count" -eq 0 ]; then
    echo "PASS (0 serde_json::Value in signatures)"
else
    echo "WARN ($value_count occurrences of serde_json::Value)"
fi

# Check log_analyzer uses Args::command()
echo -n "Checking log_analyzer uses Args::command()... "
if grep -q "Args::command().print_help()" "$EXAMPLES_DIR/example_log_analyzer/log_analyzer.rs"; then
    echo "PASS"
else
    echo "FAIL - Should use Args::command().print_help()"
    failed=$((failed + 1))
fi

echo ""
echo "=== Step 3: Clippy Check (errors only) ==="
echo ""

clippy_errors=0
for example in simple complex config csv_filter environment flags io_streams log_analyzer positional regex stdlib subcommands subprocess; do
    dir="$EXAMPLES_DIR/example_$example"
    if [ -f "$dir/Cargo.toml" ]; then
        errors=$(cargo clippy --manifest-path "$dir/Cargo.toml" 2>&1 | grep -c "^error" || true)
        if [ "$errors" -gt 0 ]; then
            echo "example_$example: $errors clippy errors"
            clippy_errors=$((clippy_errors + errors))
        fi
    fi
done

if [ "$clippy_errors" -eq 0 ]; then
    echo "All examples pass clippy (no errors)"
fi

echo ""
echo "=== Step 4: Oracle Classification Test ==="
echo ""

# Test oracle can classify borrow checker errors
echo "Testing oracle E0382 classification..."
cd /home/noah/src/depyler

# Quick oracle test via cargo test
if cargo test --package depyler-oracle test_corpus_statistics -- --nocapture 2>&1 | grep -q "ok"; then
    echo "Oracle corpus test: PASS"
else
    echo "Oracle corpus test: FAIL"
    failed=$((failed + 1))
fi

echo ""
echo "=============================================="
echo "  SUMMARY"
echo "=============================================="
echo "Examples passed: $passed/13"
echo "Failures: $failed"
echo "Report saved: $REPORT_FILE"
echo ""

if [ "$failed" -eq 0 ]; then
    echo "STATUS: ALL CHECKS PASSED"
    exit 0
else
    echo "STATUS: $failed CHECK(S) FAILED"
    exit 1
fi
