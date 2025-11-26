#!/bin/bash
# Systematic corpus testing using verificar-generated programs
# This prevents thrashing by testing comprehensively and categorizing failures

set -euo pipefail

CORPUS_FILE="${1:-corpus_d3_c50.json}"
OUTPUT_DIR="test_results_$(date +%Y%m%d_%H%M%S)"
DEPYLER="/home/noah/src/depyler/target/release/depyler"

mkdir -p "$OUTPUT_DIR"

echo "🧪 Verificar Corpus Testing"
echo "============================"
echo "Corpus: $CORPUS_FILE"
echo "Output: $OUTPUT_DIR"
echo ""

# Statistics
TOTAL=0
TRANSPILE_SUCCESS=0
TRANSPILE_FAIL=0
COMPILE_SUCCESS=0
COMPILE_FAIL=0

# Error categories
declare -A ERROR_CATEGORIES

# Extract and test each program
jq -c '.[]' "$CORPUS_FILE" | while IFS= read -r program; do
    TOTAL=$((TOTAL + 1))

    # Extract program details
    CODE=$(echo "$program" | jq -r '.code')
    DEPTH=$(echo "$program" | jq -r '.ast_depth')
    FEATURES=$(echo "$program" | jq -r '.features | join(",")')

    TEST_FILE="$OUTPUT_DIR/test_${TOTAL}.py"
    RUST_FILE="$OUTPUT_DIR/test_${TOTAL}.rs"
    LOG_FILE="$OUTPUT_DIR/test_${TOTAL}.log"

    # Write Python program
    echo "$CODE" > "$TEST_FILE"

    echo "[$TOTAL] Testing depth=$DEPTH features=$FEATURES"

    # Step 1: Transpile
    if $DEPYLER transpile "$TEST_FILE" -o "$RUST_FILE" > "$LOG_FILE" 2>&1; then
        TRANSPILE_SUCCESS=$((TRANSPILE_SUCCESS + 1))
        echo "  ✅ Transpilation succeeded"

        # Step 2: Compile
        if rustc --crate-type lib "$RUST_FILE" -o "$OUTPUT_DIR/test_${TOTAL}.rlib" >> "$LOG_FILE" 2>&1; then
            COMPILE_SUCCESS=$((COMPILE_SUCCESS + 1))
            echo "  ✅ Compilation succeeded"
            echo "PASS" > "$OUTPUT_DIR/test_${TOTAL}.status"
        else
            COMPILE_FAIL=$((COMPILE_FAIL + 1))
            echo "  ❌ Compilation failed"

            # Categorize compilation error
            if grep -q "error\[E0308\]" "$LOG_FILE"; then
                ERROR_CATEGORIES["E0308_type_mismatch"]=$((${ERROR_CATEGORIES["E0308_type_mismatch"]:-0} + 1))
            fi
            if grep -q "error\[E0369\]" "$LOG_FILE"; then
                ERROR_CATEGORIES["E0369_cannot_add"]=$((${ERROR_CATEGORIES["E0369_cannot_add"]:-0} + 1))
            fi
            if grep -q "error\[E0425\]" "$LOG_FILE"; then
                ERROR_CATEGORIES["E0425_cannot_find"]=$((${ERROR_CATEGORIES["E0425_cannot_find"]:-0} + 1))
            fi
            if grep -q "error\[E0277\]" "$LOG_FILE"; then
                ERROR_CATEGORIES["E0277_trait_not_implemented"]=$((${ERROR_CATEGORIES["E0277_trait_not_implemented"]:-0} + 1))
            fi

            echo "COMPILE_FAIL" > "$OUTPUT_DIR/test_${TOTAL}.status"
        fi
    else
        TRANSPILE_FAIL=$((TRANSPILE_FAIL + 1))
        echo "  ❌ Transpilation failed"
        echo "TRANSPILE_FAIL" > "$OUTPUT_DIR/test_${TOTAL}.status"
    fi

    echo ""
done

# Summary report
SUMMARY_FILE="$OUTPUT_DIR/SUMMARY.txt"
{
    echo "Verificar Corpus Testing Summary"
    echo "================================="
    echo ""
    echo "Total programs tested: $TOTAL"
    echo ""
    echo "Transpilation:"
    echo "  Success: $TRANSPILE_SUCCESS ($(awk "BEGIN {printf \"%.1f\", ($TRANSPILE_SUCCESS/$TOTAL)*100}")%)"
    echo "  Failure: $TRANSPILE_FAIL ($(awk "BEGIN {printf \"%.1f\", ($TRANSPILE_FAIL/$TOTAL)*100}")%)"
    echo ""
    echo "Compilation (of transpiled programs):"
    echo "  Success: $COMPILE_SUCCESS ($(awk "BEGIN {printf \"%.1f\", ($COMPILE_SUCCESS/$TRANSPILE_SUCCESS)*100}")%)"
    echo "  Failure: $COMPILE_FAIL ($(awk "BEGIN {printf \"%.1f\", ($COMPILE_FAIL/$TRANSPILE_SUCCESS)*100}")%)"
    echo ""
    echo "Overall pass rate: $(awk "BEGIN {printf \"%.1f\", ($COMPILE_SUCCESS/$TOTAL)*100}")%"
    echo ""
    echo "Error Categories:"
    for category in "${!ERROR_CATEGORIES[@]}"; do
        count="${ERROR_CATEGORIES[$category]}"
        echo "  $category: $count"
    done
} | tee "$SUMMARY_FILE"

echo ""
echo "📊 Results saved to: $OUTPUT_DIR"
echo "📄 Summary: $SUMMARY_FILE"
