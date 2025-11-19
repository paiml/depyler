#!/bin/bash
# Validate and purify all shell scripts using bashrs
# Part of DEPYLER-XXXX: Enforce bashrs validation on all Makefiles and shell scripts

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Summary counters
TOTAL_SCRIPTS=0
SCRIPTS_WITH_ERRORS=0
SCRIPTS_WITH_WARNINGS=0
SCRIPTS_PURIFIED=0
SCRIPTS_FAILED=0

echo "========================================"
echo "🔍 bashrs Shell Script Validation"
echo "========================================"
echo ""

# Find all shell scripts (excluding target/, node_modules/, backups)
SCRIPTS=$(find . -name "*.sh" \
    -not -path "./target/*" \
    -not -path "./playground/node_modules/*" \
    -not -path "./.git/*" \
    -not -path "*/.*" \
    -type f)

# Process each script
for script in $SCRIPTS; do
    TOTAL_SCRIPTS=$((TOTAL_SCRIPTS + 1))
    echo -e "${BLUE}[$TOTAL_SCRIPTS]${NC} Processing: $script"

    # Lint the script
    LINT_OUTPUT=$(bashrs lint "$script" 2>&1 || true)

    # Count errors and warnings
    ERROR_COUNT=$(echo "$LINT_OUTPUT" | grep -c "\[error\]" || true)
    WARNING_COUNT=$(echo "$LINT_OUTPUT" | grep -c "\[warning\]" || true)

    if [ "$ERROR_COUNT" -gt 0 ]; then
        echo -e "  ${RED}✗ ERRORS: $ERROR_COUNT${NC}"
        SCRIPTS_WITH_ERRORS=$((SCRIPTS_WITH_ERRORS + 1))
    fi

    if [ "$WARNING_COUNT" -gt 0 ]; then
        echo -e "  ${YELLOW}⚠ WARNINGS: $WARNING_COUNT${NC}"
        SCRIPTS_WITH_WARNINGS=$((SCRIPTS_WITH_WARNINGS + 1))
    fi

    if [ "$ERROR_COUNT" -eq 0 ] && [ "$WARNING_COUNT" -eq 0 ]; then
        echo -e "  ${GREEN}✓ Clean${NC}"
        continue
    fi

    # Save lint report
    echo "$LINT_OUTPUT" > "${script}.lint-report.txt"
    echo -e "  ${BLUE}📄 Report: ${script}.lint-report.txt${NC}"

    # Purify the script
    echo -e "  ${BLUE}🔧 Purifying...${NC}"
    if bashrs purify "$script" -o "${script}.purified" 2>&1 | grep -v "INFO"; then
        # Validate purified version
        bash -n "${script}.purified" 2>/dev/null || {
            echo -e "  ${RED}✗ Purified version has syntax errors${NC}"
            SCRIPTS_FAILED=$((SCRIPTS_FAILED + 1))
            rm -f "${script}.purified"
            continue
        }

        # Compare sizes
        ORIGINAL_LINES=$(wc -l < "$script")
        PURIFIED_LINES=$(wc -l < "${script}.purified")

        echo -e "  ${GREEN}✓ Purified: $ORIGINAL_LINES → $PURIFIED_LINES lines${NC}"
        SCRIPTS_PURIFIED=$((SCRIPTS_PURIFIED + 1))
    else
        echo -e "  ${RED}✗ Purification failed${NC}"
        SCRIPTS_FAILED=$((SCRIPTS_FAILED + 1))
    fi

    echo ""
done

# Summary
echo "========================================"
echo "📊 Summary"
echo "========================================"
echo -e "Total scripts:       ${BLUE}$TOTAL_SCRIPTS${NC}"
echo -e "Scripts with errors: ${RED}$SCRIPTS_WITH_ERRORS${NC}"
echo -e "Scripts with warnings: ${YELLOW}$SCRIPTS_WITH_WARNINGS${NC}"
echo -e "Scripts purified:    ${GREEN}$SCRIPTS_PURIFIED${NC}"
echo -e "Failed to purify:    ${RED}$SCRIPTS_FAILED${NC}"
echo ""

if [ "$SCRIPTS_WITH_ERRORS" -eq 0 ]; then
    echo -e "${GREEN}✅ No scripts with errors!${NC}"
else
    echo -e "${YELLOW}⚠️  $SCRIPTS_WITH_ERRORS scripts have errors (see .lint-report.txt files)${NC}"
fi

echo ""
echo "Next steps:"
echo "1. Review .lint-report.txt files for details"
echo "2. Review .purified versions of scripts"
echo "3. Replace originals with purified versions if satisfactory"
echo "4. Run: find . -name '*.sh.purified' -exec bash -c 'mv \"\$1\" \"\${1%.purified}\"' _ {} \;"
