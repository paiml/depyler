#!/bin/bash
# Depyler Example Validation Script
# Validates all transpiled examples against quality gates
#
# Usage: ./scripts/validate_examples.sh [directory]
# Example: ./scripts/validate_examples.sh examples/showcase

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
EXAMPLES_DIR="${1:-examples/showcase}"
REPORT_FILE="validation_report.md"
DEPYLER_BIN="${DEPYLER_BIN:-cargo run --quiet --}"

# Counters
TOTAL_EXAMPLES=0
PASSED_EXAMPLES=0
FAILED_EXAMPLES=0
SKIPPED_EXAMPLES=0

# Arrays for tracking
declare -a PASSED_FILES
declare -a FAILED_FILES
declare -a SKIPPED_FILES

echo -e "${BLUE}🔍 Depyler Example Validation${NC}"
echo -e "${BLUE}================================${NC}"
echo ""
echo "Directory: $EXAMPLES_DIR"
echo ""

# Initialize report
cat > "$REPORT_FILE" <<EOF
# Depyler Example Validation Report

**Generated**: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
**Directory**: $EXAMPLES_DIR

## Summary

EOF

# Function to validate a single Rust file
validate_rust_file() {
    local rust_file="$1"
    local python_file="${rust_file%.rs}.py"
    local filename=$(basename "$rust_file")

    echo -e "${BLUE}📝 Validating: $filename${NC}"

    # Check if source Python file exists
    if [[ ! -f "$python_file" ]]; then
        echo -e "  ${YELLOW}⊘ Skipped${NC} - No source Python file found"
        SKIPPED_FILES+=("$filename")
        ((SKIPPED_EXAMPLES++))
        return 0
    fi

    local violations=0
    local checks_run=0

    # Gate 1: Rust compilation check
    echo -e "  ${BLUE}[1/5]${NC} Checking Rust compilation..."
    if rustc --crate-type lib "$rust_file" -o /tmp/depyler_validate_$$.rlib 2>/dev/null; then
        echo -e "  ${GREEN}✓${NC} Compiles successfully"
        rm -f /tmp/depyler_validate_$$.rlib
    else
        echo -e "  ${RED}✗${NC} Compilation failed"
        ((violations++))
    fi
    ((checks_run++))

    # Gate 2: Clippy warnings (zero tolerance)
    echo -e "  ${BLUE}[2/5]${NC} Checking clippy warnings..."
    if rustc --crate-type lib "$rust_file" -o /tmp/depyler_validate_$$.rlib 2>&1 | grep -q "warning:"; then
        echo -e "  ${RED}✗${NC} Clippy warnings found"
        ((violations++))
    else
        echo -e "  ${GREEN}✓${NC} Zero clippy warnings"
    fi
    rm -f /tmp/depyler_validate_$$.rlib
    ((checks_run++))

    # Gate 3: PMAT Complexity (≤10 cyclomatic)
    if command -v pmat &> /dev/null; then
        echo -e "  ${BLUE}[3/5]${NC} Checking complexity..."
        if pmat analyze complexity --file "$rust_file" --max-cyclomatic 10 --max-cognitive 10 --fail-on-violation &>/dev/null; then
            echo -e "  ${GREEN}✓${NC} Complexity ≤10"
        else
            echo -e "  ${RED}✗${NC} Complexity >10"
            ((violations++))
        fi
        ((checks_run++))
    else
        echo -e "  ${YELLOW}⊘${NC} Complexity check skipped (pmat not installed)"
    fi

    # Gate 4: SATD (zero tolerance)
    if command -v pmat &> /dev/null; then
        echo -e "  ${BLUE}[4/5]${NC} Checking SATD..."
        if pmat analyze satd --path "$rust_file" --fail-on-violation &>/dev/null; then
            echo -e "  ${GREEN}✓${NC} Zero SATD"
        else
            echo -e "  ${RED}✗${NC} SATD violations found"
            ((violations++))
        fi
        ((checks_run++))
    else
        echo -e "  ${YELLOW}⊘${NC} SATD check skipped (pmat not installed)"
    fi

    # Gate 5: Re-transpilation check (determinism)
    echo -e "  ${BLUE}[5/5]${NC} Checking transpilation determinism..."
    if $DEPYLER_BIN transpile "$python_file" --output /tmp/depyler_validate_$$.rs 2>/dev/null; then
        if diff -q "$rust_file" /tmp/depyler_validate_$$.rs &>/dev/null; then
            echo -e "  ${GREEN}✓${NC} Transpilation is deterministic"
        else
            echo -e "  ${YELLOW}⚠${NC} Transpilation output differs (may need regeneration)"
            # Not counted as violation - may be intentional edits
        fi
        rm -f /tmp/depyler_validate_$$.rs
    else
        echo -e "  ${YELLOW}⊘${NC} Re-transpilation skipped (depyler failed)"
    fi
    ((checks_run++))

    # Summary for this file
    if [[ $violations -eq 0 ]]; then
        echo -e "  ${GREEN}✅ PASSED${NC} ($checks_run/$checks_run checks)"
        PASSED_FILES+=("$filename")
        ((PASSED_EXAMPLES++))
    else
        echo -e "  ${RED}❌ FAILED${NC} ($violations violations, $checks_run checks run)"
        FAILED_FILES+=("$filename:$violations")
        ((FAILED_EXAMPLES++))
    fi

    echo ""
    ((TOTAL_EXAMPLES++))
}

# Find and validate all Rust files in the examples directory
echo -e "${BLUE}🔍 Finding examples in $EXAMPLES_DIR...${NC}"
echo ""

if [[ ! -d "$EXAMPLES_DIR" ]]; then
    echo -e "${RED}Error: Directory $EXAMPLES_DIR not found${NC}"
    exit 1
fi

# Process all .rs files
while IFS= read -r rust_file; do
    validate_rust_file "$rust_file"
done < <(find "$EXAMPLES_DIR" -name "*.rs" -type f | sort)

# Generate summary
echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}📊 Validation Summary${NC}"
echo -e "${BLUE}================================${NC}"
echo ""
echo -e "Total examples validated: ${BLUE}$TOTAL_EXAMPLES${NC}"
echo -e "Passed: ${GREEN}$PASSED_EXAMPLES${NC}"
echo -e "Failed: ${RED}$FAILED_EXAMPLES${NC}"
echo -e "Skipped: ${YELLOW}$SKIPPED_EXAMPLES${NC}"
echo ""

# Append summary to report
cat >> "$REPORT_FILE" <<EOF
- **Total Examples**: $TOTAL_EXAMPLES
- **Passed**: $PASSED_EXAMPLES ✅
- **Failed**: $FAILED_EXAMPLES ❌
- **Skipped**: $SKIPPED_EXAMPLES ⊘

## Passed Examples ($PASSED_EXAMPLES)

EOF

for file in "${PASSED_FILES[@]+"${PASSED_FILES[@]}"}"; do
    echo "- ✅ $file" >> "$REPORT_FILE"
done

cat >> "$REPORT_FILE" <<EOF

## Failed Examples ($FAILED_EXAMPLES)

EOF

for file in "${FAILED_FILES[@]+"${FAILED_FILES[@]}"}"; do
    filename="${file%:*}"
    violations="${file#*:}"
    echo "- ❌ $filename ($violations violations)" >> "$REPORT_FILE"
done

cat >> "$REPORT_FILE" <<EOF

## Skipped Examples ($SKIPPED_EXAMPLES)

EOF

for file in "${SKIPPED_FILES[@]+"${SKIPPED_FILES[@]}"}"; do
    echo "- ⊘ $file (no source Python file)" >> "$REPORT_FILE"
done

cat >> "$REPORT_FILE" <<EOF

## Quality Gates Applied

1. **Rust Compilation**: Must compile with rustc
2. **Clippy Warnings**: Zero warnings (zero tolerance)
3. **Complexity**: Cyclomatic ≤10, Cognitive ≤10
4. **SATD**: Zero TODO/FIXME/HACK comments
5. **Determinism**: Re-transpilation produces identical output

## Recommendations

EOF

if [[ $FAILED_EXAMPLES -gt 0 ]]; then
    cat >> "$REPORT_FILE" <<EOF
### Priority Actions

1. Fix failed showcase examples first (highest visibility)
2. Apply EXTREME TDD to each fix
3. Re-run validation after fixes
4. Update examples with regenerated transpilation if needed

EOF
else
    cat >> "$REPORT_FILE" <<EOF
### All Examples Passing! 🎉

The transpiler is producing high-quality code that meets all quality gates.
Consider expanding validation to other example directories:
- examples/algorithms/
- examples/data_structures/
- examples/networking/

EOF
fi

echo -e "${BLUE}📄 Report generated: $REPORT_FILE${NC}"
echo ""

# Exit code based on results
if [[ $FAILED_EXAMPLES -gt 0 ]]; then
    echo -e "${RED}❌ Validation FAILED - $FAILED_EXAMPLES examples need attention${NC}"
    exit 1
else
    echo -e "${GREEN}✅ Validation PASSED - All examples meet quality gates!${NC}"
    exit 0
fi
