#!/usr/bin/env bash
#
# quick_validate_examples.sh - Fast Example Validation
#
# Purpose: Quickly validate all examples by checking if they exist and compile
# Author: Depyler Team
# Date: 2025-10-07
# Ticket: DEPYLER-0027

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLES_DIR="$PROJECT_ROOT/examples"

echo "=========================================="
echo "🔍 Depyler Example Quick Validation"
echo "Ticket: DEPYLER-0027"
echo "=========================================="
echo ""

# Find all .rs examples
mapfile -t EXAMPLES < <(find "$EXAMPLES_DIR" -type f -name "*.rs" -not -path "*/target/*" | sort)

echo "Found ${#EXAMPLES[@]} Rust examples"
echo ""

# Run workspace-wide checks
echo "📋 Running Workspace-Wide Checks..."
echo ""

# 1. Clippy
echo -n "1. Clippy (zero warnings)... "
if cargo clippy --all-targets --all-features -- -D warnings 2>&1 | grep -q "warning\|error"; then
    echo -e "${RED}❌ FAILED${NC}"
    CLIPPY_PASS=false
else
    echo -e "${GREEN}✅ PASSED${NC}"
    CLIPPY_PASS=true
fi

# 2. Check (compilation)
echo -n "2. Cargo check (all examples compile)... "
if cargo check --all-targets --all-features 2>&1 | grep -q "error"; then
    echo -e "${RED}❌ FAILED${NC}"
    CHECK_PASS=false
else
    echo -e "${GREEN}✅ PASSED${NC}"
    CHECK_PASS=true
fi

echo ""
echo "=========================================="
echo "📊 EXAMPLE VALIDATION SUMMARY"
echo "=========================================="
echo ""
printf "%-50s %-10s %s\n" "Example" "Priority" "Status"
printf "%-50s %-10s %s\n" "$(printf '%.0s-' {1..50})" "$(printf '%.0s-' {1..10})" "$(printf '%.0s-' {1..10})"

TOTAL=0
SHOWCASE=0
CORE=0
ADVANCED=0

# Priority 0: Showcase
for example in "${EXAMPLES[@]}"; do
    if [[ $example == */showcase/* ]]; then
        basename=$(basename "$example")
        if $CLIPPY_PASS && $CHECK_PASS; then
            printf "%-50s %-10s ${GREEN}✅ PASS${NC}\n" "$basename" "P0"
        else
            printf "%-50s %-10s ${YELLOW}⚠️  NEEDS FIX${NC}\n" "$basename" "P0"
        fi
        ((SHOWCASE++))
        ((TOTAL++))
    fi
done

# Priority 1: Core (mathematical + test_*.rs)
for example in "${EXAMPLES[@]}"; do
    if [[ $example == */mathematical/* ]] || [[ $example =~ test_.*\.rs$ && ! $example =~ /showcase/ ]]; then
        basename=$(basename "$example")
        if $CLIPPY_PASS && $CHECK_PASS; then
            printf "%-50s %-10s ${GREEN}✅ PASS${NC}\n" "$basename" "P1"
        else
            printf "%-50s %-10s ${YELLOW}⚠️  NEEDS FIX${NC}\n" "$basename" "P1"
        fi
        ((CORE++))
        ((TOTAL++))
    fi
done

# Priority 2: Advanced (everything else)
for example in "${EXAMPLES[@]}"; do
    # Skip if already categorized
    if [[ $example == */showcase/* ]] || \
       [[ $example == */mathematical/* ]] || \
       [[ $example =~ test_.*\.rs$ && ! $example =~ /showcase/ ]]; then
        continue
    fi
    basename=$(basename "$example")
    if $CLIPPY_PASS && $CHECK_PASS; then
        printf "%-50s %-10s ${GREEN}✅ PASS${NC}\n" "$basename" "P2"
    else
        printf "%-50s %-10s ${YELLOW}⚠️  NEEDS FIX${NC}\n" "$basename" "P2"
    fi
    ((ADVANCED++))
    ((TOTAL++))
done

echo ""
echo "=========================================="
echo "📊 TOTALS"
echo "=========================================="
echo "Total Examples: $TOTAL"
echo "  - P0 (Showcase): $SHOWCASE"
echo "  - P1 (Core): $CORE"
echo "  - P2 (Advanced): $ADVANCED"
echo ""
if $CLIPPY_PASS && $CHECK_PASS; then
    echo -e "${GREEN}✅ ALL WORKSPACE CHECKS PASSED${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Run 'cargo test --workspace' to verify tests"
    echo "2. Run 'cargo llvm-cov' to check coverage ≥80%"
    echo "3. Run 'pmat tdg .' to verify A- grade"
else
    echo -e "${RED}❌ WORKSPACE CHECKS FAILED${NC}"
    echo ""
    echo "Fix these issues:"
    if ! $CLIPPY_PASS; then
        echo "  - Clippy warnings: Run 'cargo clippy --all-targets -- -D warnings' for details"
    fi
    if ! $CHECK_PASS; then
        echo "  - Compilation errors: Run 'cargo check --all-targets' for details"
    fi
fi
echo "=========================================="
