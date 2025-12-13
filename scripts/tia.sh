#!/bin/bash
# DEPYLER-0954: Test Impact Analysis (TIA)
#
# Runs only tests affected by recent code changes.
# Target: 50-80% faster CI cycles by avoiding redundant test execution.
#
# Usage:
#   ./scripts/tia.sh              # Tests for changes since last commit
#   ./scripts/tia.sh HEAD~5       # Tests for changes in last 5 commits
#   ./scripts/tia.sh main         # Tests for changes since main branch
#   ./scripts/tia.sh --all        # Run all tests (bypass TIA)
#
# Toyota Way: Muda (waste) elimination - don't run tests unaffected by changes

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
BASE_REF="${1:-HEAD~1}"
RUN_ALL=false

if [[ "$1" == "--all" ]]; then
    RUN_ALL=true
fi

if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "Usage: $0 [BASE_REF|--all]"
    echo ""
    echo "Options:"
    echo "  BASE_REF    Git ref to compare against (default: HEAD~1)"
    echo "  --all       Run all tests (bypass TIA)"
    echo ""
    echo "Examples:"
    echo "  $0              # Changes since last commit"
    echo "  $0 HEAD~5       # Changes in last 5 commits"
    echo "  $0 main         # Changes since main branch"
    echo "  $0 --all        # Run all tests"
    exit 0
fi

echo -e "${BLUE}🔍 DEPYLER-0954: Test Impact Analysis${NC}"
echo ""

if $RUN_ALL; then
    echo -e "${YELLOW}Running ALL tests (--all flag)${NC}"
    cargo test --workspace
    exit $?
fi

# Get changed files
echo -e "${BLUE}Analyzing changes since ${BASE_REF}...${NC}"
CHANGED_FILES=$(git diff --name-only "$BASE_REF" -- '*.rs' 2>/dev/null || echo "")

if [[ -z "$CHANGED_FILES" ]]; then
    echo -e "${GREEN}✅ No Rust files changed - skipping tests${NC}"
    exit 0
fi

echo -e "${BLUE}Changed files:${NC}"
echo "$CHANGED_FILES" | head -20
TOTAL_CHANGED=$(echo "$CHANGED_FILES" | wc -l)
if [[ $TOTAL_CHANGED -gt 20 ]]; then
    echo "  ... and $((TOTAL_CHANGED - 20)) more"
fi
echo ""

# Build dependency map: changed file -> affected packages
declare -A AFFECTED_PACKAGES
declare -A AFFECTED_TESTS

for file in $CHANGED_FILES; do
    # Extract package from path
    if [[ "$file" =~ ^crates/([^/]+)/ ]]; then
        pkg="${BASH_REMATCH[1]}"
        AFFECTED_PACKAGES["$pkg"]=1
    fi

    # Map source files to their test files
    # Convention: src/foo.rs -> tests/*foo* or tests that import foo
    if [[ "$file" =~ ^crates/([^/]+)/src/(.+)\.rs$ ]]; then
        pkg="${BASH_REMATCH[1]}"
        module="${BASH_REMATCH[2]}"
        # Remove path separators for module name
        module_name=$(basename "$module")
        AFFECTED_TESTS["$module_name"]=1
    fi

    # Direct test file changes
    if [[ "$file" =~ ^crates/([^/]+)/tests/(.+)\.rs$ ]]; then
        test_name="${BASH_REMATCH[2]}"
        AFFECTED_TESTS["$test_name"]=1
    fi
done

# Build test filter
PACKAGES="${!AFFECTED_PACKAGES[@]}"
TESTS="${!AFFECTED_TESTS[@]}"

echo -e "${BLUE}Affected packages:${NC} ${PACKAGES:-none}"
echo -e "${BLUE}Affected modules:${NC} ${TESTS:-none}"
echo ""

# Strategy selection based on scope
if [[ ${#AFFECTED_PACKAGES[@]} -eq 0 ]]; then
    echo -e "${GREEN}✅ No crate packages affected - skipping tests${NC}"
    exit 0
fi

# Count total tests for comparison
TOTAL_TESTS=$(cargo test --workspace --no-run 2>&1 | grep -c "test$" || echo "0")

# Build cargo test command
TEST_CMD="cargo test"

# If only specific packages affected, scope to them
if [[ ${#AFFECTED_PACKAGES[@]} -lt 5 ]]; then
    for pkg in "${!AFFECTED_PACKAGES[@]}"; do
        TEST_CMD="$TEST_CMD -p $pkg"
    done
else
    # Too many packages - run workspace tests
    TEST_CMD="$TEST_CMD --workspace"
fi

# Add lib tests only if we have specific modules
if [[ ${#AFFECTED_TESTS[@]} -gt 0 && ${#AFFECTED_TESTS[@]} -lt 10 ]]; then
    # Build test name filter
    FILTER=""
    for test in "${!AFFECTED_TESTS[@]}"; do
        if [[ -n "$FILTER" ]]; then
            FILTER="$FILTER\|$test"
        else
            FILTER="$test"
        fi
    done
    # Use -- to pass filter to test binary
    TEST_CMD="$TEST_CMD -- --test-threads=4"
fi

echo -e "${BLUE}Running TIA-selected tests...${NC}"
echo -e "${YELLOW}Command: $TEST_CMD${NC}"
echo ""

# Time the execution
START_TIME=$(date +%s)
$TEST_CMD
EXIT_CODE=$?
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
if [[ $EXIT_CODE -eq 0 ]]; then
    echo -e "${GREEN}✅ TIA tests passed in ${DURATION}s${NC}"

    # Report savings estimate
    SELECTED_PKGS=${#AFFECTED_PACKAGES[@]}
    if [[ $SELECTED_PKGS -lt 5 ]]; then
        SAVINGS=$((100 - (SELECTED_PKGS * 20)))
        echo -e "${GREEN}📊 Estimated CI time savings: ~${SAVINGS}%${NC}"
    fi
else
    echo -e "${RED}❌ TIA tests failed${NC}"
fi

exit $EXIT_CODE
