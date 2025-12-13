#!/bin/bash
# Harvest real transpilation errors from verificar corpus
# These are ACTUAL rustc errors from transpiled Python, not synthetic

set -euo pipefail

CORPUS_DIR="target/verificar/corpus"
OUTPUT_DIR="target/verificar/output"
REAL_ERRORS_DIR="training_corpus/real_errors"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
ERRORS_FILE="${REAL_ERRORS_DIR}/${TIMESTAMP}.jsonl"
MAIN_CORPUS="training_corpus/errors.jsonl"

echo "🔍 Harvesting real transpilation errors..."
echo "   Source: ${CORPUS_DIR}"
echo "   Output: ${ERRORS_FILE}"
echo ""

# Ensure directories exist
mkdir -p "$REAL_ERRORS_DIR" "$OUTPUT_DIR"

# Build depyler if needed
if [ ! -f "target/release/depyler" ]; then
    echo "📦 Building depyler (release)..."
    cargo build --release -p depyler
fi

# Generate verificar corpus if not exists
if [ ! -d "$CORPUS_DIR" ] || [ -z "$(ls -A $CORPUS_DIR 2>/dev/null)" ]; then
    echo "📝 Generating verificar corpus first..."
    make verificar-generate 2>/dev/null || {
        echo "⚠️  verificar not available, using examples instead"
        CORPUS_DIR="examples"
    }
fi

# Count files
TOTAL_FILES=$(find "$CORPUS_DIR" -name "*.py" -type f 2>/dev/null | wc -l)
echo "📊 Found $TOTAL_FILES Python files to process"
echo ""

# Initialize counters
TRANSPILE_SUCCESS=0
TRANSPILE_FAIL=0
COMPILE_SUCCESS=0
COMPILE_FAIL=0
ERRORS_HARVESTED=0

# Process each Python file
for py in $(find "$CORPUS_DIR" -name "*.py" -type f 2>/dev/null | head -500); do
    name=$(basename "$py" .py)
    rs="${OUTPUT_DIR}/${name}.rs"

    # Try to transpile
    if ./target/release/depyler transpile "$py" -o "$rs" 2>/dev/null; then
        ((TRANSPILE_SUCCESS++)) || true

        # Try to compile the generated Rust
        ERROR=$(rustc --edition 2021 --crate-type lib "$rs" -o /dev/null 2>&1 || true)

        if [ -n "$ERROR" ] && [ "$ERROR" != "" ]; then
            ((COMPILE_FAIL++)) || true

            # Extract error category
            CATEGORY="Unknown"
            if echo "$ERROR" | grep -q "cannot borrow"; then
                CATEGORY="BorrowChecker"
            elif echo "$ERROR" | grep -q "lifetime"; then
                CATEGORY="LifetimeError"
            elif echo "$ERROR" | grep -q "expected\|found\|mismatched"; then
                CATEGORY="TypeMismatch"
            elif echo "$ERROR" | grep -q "cannot find"; then
                CATEGORY="MissingImport"
            elif echo "$ERROR" | grep -q "trait.*not implemented"; then
                CATEGORY="TraitBound"
            elif echo "$ERROR" | grep -q "syntax\|expected"; then
                CATEGORY="SyntaxError"
            fi

            # Escape JSON
            ERROR_ESCAPED=$(echo "$ERROR" | head -20 | tr '\n' ' ' | sed 's/"/\\"/g' | cut -c1-500)

            # Write to JSONL
            echo "{\"source\":\"real_transpilation\",\"file\":\"$py\",\"category\":\"$CATEGORY\",\"error\":\"$ERROR_ESCAPED\",\"timestamp\":\"$(date -Iseconds)\"}" >> "$ERRORS_FILE"

            ((ERRORS_HARVESTED++)) || true
        else
            ((COMPILE_SUCCESS++)) || true
        fi
    else
        ((TRANSPILE_FAIL++)) || true
    fi

    # Progress indicator every 50 files
    PROCESSED=$((TRANSPILE_SUCCESS + TRANSPILE_FAIL))
    if [ $((PROCESSED % 50)) -eq 0 ] && [ $PROCESSED -gt 0 ]; then
        echo "   Processed $PROCESSED files, harvested $ERRORS_HARVESTED errors..."
    fi
done

echo ""
echo "=== Harvest Complete ==="
echo ""
echo "📊 Results:"
echo "   Files processed: $((TRANSPILE_SUCCESS + TRANSPILE_FAIL))"
echo "   Transpile success: $TRANSPILE_SUCCESS"
echo "   Transpile fail: $TRANSPILE_FAIL"
echo "   Compile success: $COMPILE_SUCCESS"
echo "   Compile fail (errors harvested): $COMPILE_FAIL"
echo ""
echo "🎯 Errors harvested: $ERRORS_HARVESTED"
echo "   Output: $ERRORS_FILE"

# Merge into main corpus if we got errors
if [ -f "$ERRORS_FILE" ] && [ -s "$ERRORS_FILE" ]; then
    echo ""
    echo "📥 Merging into main corpus..."

    BEFORE=$(wc -l < "$MAIN_CORPUS" 2>/dev/null || echo 0)

    # Append and deduplicate
    cat "$ERRORS_FILE" >> "$MAIN_CORPUS"
    sort -u "$MAIN_CORPUS" -o "$MAIN_CORPUS"

    AFTER=$(wc -l < "$MAIN_CORPUS")
    NEW=$((AFTER - BEFORE))

    echo "   Before: $BEFORE errors"
    echo "   After: $AFTER errors"
    echo "   New unique: $NEW errors"
    echo ""
    echo "✅ Real errors merged into corpus!"
else
    echo ""
    echo "⚠️  No errors harvested (all files compiled successfully or transpilation failed)"
fi

echo ""
echo "💡 Next: Run 'make train-oracle' to retrain with real errors"
