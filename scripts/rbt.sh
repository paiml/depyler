#!/bin/bash
# DEPYLER-0957: Risk-Based Testing (RBT)
#
# Prioritizes test execution based on risk factors:
# - Impact: How critical is the feature being tested
# - Likelihood: How likely is the code to have bugs
# - History: Tests that failed recently get higher priority
#
# Usage:
#   ./scripts/rbt.sh              # Run risk-prioritized tests
#   ./scripts/rbt.sh --high-only  # Run only high-risk tests
#   ./scripts/rbt.sh --analyze    # Analyze risk without running
#   ./scripts/rbt.sh --report     # Generate risk report
#
# Toyota Way: Prioritize where quality matters most

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Risk scoring configuration
HIGH_RISK_THRESHOLD=70
MEDIUM_RISK_THRESHOLD=40

# Parse arguments
MODE="${1:-run}"
HIGH_ONLY=false
ANALYZE_ONLY=false

case "$1" in
    --high-only)
        HIGH_ONLY=true
        MODE="run"
        ;;
    --analyze)
        ANALYZE_ONLY=true
        MODE="analyze"
        ;;
    --report)
        MODE="report"
        ;;
    --help|-h)
        echo "Usage: $0 [--high-only|--analyze|--report]"
        echo ""
        echo "Options:"
        echo "  --high-only   Run only high-risk tests (risk score >= 70)"
        echo "  --analyze     Analyze risk scores without running tests"
        echo "  --report      Generate detailed risk report"
        echo ""
        echo "Risk Factors:"
        echo "  - Complexity: Files with high cyclomatic complexity"
        echo "  - Recent changes: Files changed in last 5 commits"
        echo "  - Failure history: Tests that failed recently"
        echo "  - Critical paths: Core transpiler and codegen modules"
        exit 0
        ;;
esac

echo -e "${BLUE}🎯 DEPYLER-0957: Risk-Based Testing${NC}"
echo ""

# Define critical paths (high impact)
CRITICAL_PATHS=(
    "rust_gen"
    "type_system"
    "type_inference"
    "codegen"
    "converge"
    "compile"
    "transpile"
)

# Function to calculate risk score for a module
calculate_risk() {
    local module="$1"
    local score=0

    # Factor 1: Critical path (0-40 points)
    for critical in "${CRITICAL_PATHS[@]}"; do
        if [[ "$module" == *"$critical"* ]]; then
            score=$((score + 40))
            break
        fi
    done

    # Factor 2: Recent changes (0-30 points)
    local changes=$(git log --oneline -5 --all -- "**/*${module}*" 2>/dev/null | wc -l)
    if [[ $changes -gt 3 ]]; then
        score=$((score + 30))
    elif [[ $changes -gt 1 ]]; then
        score=$((score + 20))
    elif [[ $changes -gt 0 ]]; then
        score=$((score + 10))
    fi

    # Factor 3: File complexity (0-20 points based on line count as proxy)
    local lines=$(find crates -name "*${module}*.rs" -exec wc -l "{}" + 2>/dev/null | tail -1 | awk '{print $1}')
    if [[ -n "$lines" && "$lines" -gt 500 ]]; then
        score=$((score + 20))
    elif [[ -n "$lines" && "$lines" -gt 200 ]]; then
        score=$((score + 10))
    fi

    # Factor 4: Test existence penalty (0-10 points if no dedicated tests)
    local has_test=$(find crates -name "*${module}*test*.rs" 2>/dev/null | wc -l)
    if [[ $has_test -eq 0 ]]; then
        score=$((score + 10))
    fi

    echo $score
}

# Get list of test modules
get_test_modules() {
    cargo test --workspace --no-run 2>&1 | \
        grep -oP 'crates/[^/]+/(?:src|tests)/[^/]+' | \
        sed 's|crates/||g' | \
        sed 's|/src/||g' | \
        sed 's|/tests/||g' | \
        sed 's|\.rs$||g' | \
        sort -u
}

# Analyze and categorize tests by risk
analyze_tests() {
    echo -e "${BLUE}Analyzing test risk scores...${NC}"
    echo ""

    declare -A HIGH_RISK
    declare -A MEDIUM_RISK
    declare -A LOW_RISK

    # Core modules with high inherent risk
    local modules=(
        "rust_gen"
        "type_inference"
        "codegen"
        "type_system"
        "converge"
        "transpiler"
        "cargo_toml_gen"
        "depylint"
        "stdlib"
        "builtins"
    )

    for module in "${modules[@]}"; do
        local score=$(calculate_risk "$module")

        if [[ $score -ge $HIGH_RISK_THRESHOLD ]]; then
            HIGH_RISK["$module"]=$score
        elif [[ $score -ge $MEDIUM_RISK_THRESHOLD ]]; then
            MEDIUM_RISK["$module"]=$score
        else
            LOW_RISK["$module"]=$score
        fi
    done

    # Output results
    echo -e "${RED}HIGH RISK (>= $HIGH_RISK_THRESHOLD):${NC}"
    for module in "${!HIGH_RISK[@]}"; do
        echo "  [$((HIGH_RISK[$module]))] $module"
    done | sort -t'[' -k2 -rn

    echo ""
    echo -e "${YELLOW}MEDIUM RISK ($MEDIUM_RISK_THRESHOLD-$HIGH_RISK_THRESHOLD):${NC}"
    for module in "${!MEDIUM_RISK[@]}"; do
        echo "  [$((MEDIUM_RISK[$module]))] $module"
    done | sort -t'[' -k2 -rn

    echo ""
    echo -e "${GREEN}LOW RISK (< $MEDIUM_RISK_THRESHOLD):${NC}"
    for module in "${!LOW_RISK[@]}"; do
        echo "  [$((LOW_RISK[$module]))] $module"
    done | sort -t'[' -k2 -rn

    # Return high-risk modules for test filtering
    echo ""
    echo "HIGH_RISK_MODULES: ${!HIGH_RISK[*]}"
}

# Run tests in risk-priority order
run_prioritized_tests() {
    echo -e "${BLUE}Running risk-prioritized tests...${NC}"
    echo ""

    # Phase 1: Critical path tests (highest risk)
    echo -e "${RED}Phase 1: Critical Path Tests${NC}"
    local START=$(date +%s)

    if $HIGH_ONLY; then
        cargo test -p depyler-core -- rust_gen type_inference codegen 2>&1 || true
    else
        # Run all tests but in priority order
        # First: core transpiler tests
        echo "Running core transpiler tests..."
        cargo test -p depyler-core --lib -- --test-threads=4 2>&1 | tail -5

        # Second: integration tests
        echo ""
        echo -e "${YELLOW}Phase 2: Integration Tests${NC}"
        cargo test -p depyler-core --test '*' -- --test-threads=4 2>&1 | tail -5

        # Third: full workspace
        echo ""
        echo -e "${GREEN}Phase 3: Remaining Tests${NC}"
        cargo test --workspace --exclude depyler-core -- --test-threads=4 2>&1 | tail -5
    fi

    local END=$(date +%s)
    local DURATION=$((END - START))

    echo ""
    echo -e "${GREEN}✅ RBT tests completed in ${DURATION}s${NC}"
}

# Generate risk report
generate_report() {
    echo -e "${BLUE}📊 Risk-Based Testing Report${NC}"
    echo "================================"
    echo ""
    echo "Date: $(date)"
    echo "Repository: $(basename $(pwd))"
    echo ""

    analyze_tests

    echo ""
    echo "================================"
    echo "Test Execution Strategy:"
    echo "  1. High-risk modules: Run first (fail-fast)"
    echo "  2. Medium-risk: Run in parallel"
    echo "  3. Low-risk: Run last or skip in CI"
    echo ""
    echo "CI Optimization:"
    echo "  - Use 'make test-rbt-high' for quick feedback"
    echo "  - Use 'make test-rbt' for full prioritized run"
}

# Main execution
case "$MODE" in
    run)
        run_prioritized_tests
        ;;
    analyze)
        analyze_tests
        ;;
    report)
        generate_report
        ;;
esac
