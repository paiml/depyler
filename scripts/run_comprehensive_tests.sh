#!/bin/bash

# Depyler Comprehensive Test Runner
# NASA/SQLite-style exhaustive testing

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
COVERAGE_THRESHOLD=85
MAX_TEST_TIME=300  # 5 minutes max
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo -e "${BLUE}=== Depyler Comprehensive Test Suite ===${NC}"
echo "Starting comprehensive testing with NASA/SQLite standards..."
echo "Coverage threshold: ${COVERAGE_THRESHOLD}%"
echo "Maximum test time: ${MAX_TEST_TIME}s"
echo "Temporary directory: ${TEMP_DIR}"
echo

# Function to run test with timeout
run_test_with_timeout() {
    local test_name="$1"
    local test_command="$2"
    local max_time="$3"
    
    echo -e "${YELLOW}Running: ${test_name}${NC}"
    
    if timeout "$max_time" bash -c "$test_command"; then
        echo -e "${GREEN}✅ ${test_name} passed${NC}"
        return 0
    else
        echo -e "${RED}❌ ${test_name} failed${NC}"
        return 1
    fi
}

# Initialize counters
total_tests=0
passed_tests=0
failed_tests=0

# Test categories
declare -A test_categories=(
    ["Unit Tests"]="cargo test --lib --workspace --all-features"
    ["Integration Tests"]="cargo test --test transpilation_tests"
    ["Property Tests"]="cargo test --test semantic_equivalence"
    ["Compilation Tests"]="cargo test --test rustc_compilation"
    ["Fixture Tests"]="cargo test fixture"
    ["Benchmark Tests"]="cargo bench --no-run"
)

# Run test categories
for category in "${!test_categories[@]}"; do
    total_tests=$((total_tests + 1))
    
    if run_test_with_timeout "$category" "${test_categories[$category]}" "$MAX_TEST_TIME"; then
        passed_tests=$((passed_tests + 1))
    else
        failed_tests=$((failed_tests + 1))
    fi
    echo
done

# Coverage analysis
echo -e "${BLUE}=== Coverage Analysis ===${NC}"
total_tests=$((total_tests + 1))

if run_test_with_timeout "Coverage Generation" "cargo llvm-cov --workspace --html --summary-only" "$MAX_TEST_TIME"; then
    # Extract coverage percentage
    coverage_output=$(cargo llvm-cov --workspace --summary-only 2>/dev/null || echo "0.0%")
    coverage_percent=$(echo "$coverage_output" | grep -o '[0-9]*\.[0-9]*%' | head -1 | sed 's/%//')
    
    if (( $(echo "$coverage_percent >= $COVERAGE_THRESHOLD" | bc -l) )); then
        echo -e "${GREEN}✅ Coverage: ${coverage_percent}% (≥ ${COVERAGE_THRESHOLD}%)${NC}"
        passed_tests=$((passed_tests + 1))
    else
        echo -e "${RED}❌ Coverage: ${coverage_percent}% (< ${COVERAGE_THRESHOLD}%)${NC}"
        failed_tests=$((failed_tests + 1))
    fi
else
    echo -e "${RED}❌ Coverage analysis failed${NC}"
    failed_tests=$((failed_tests + 1))
fi

# Code quality checks
echo -e "${BLUE}=== Code Quality Checks ===${NC}"

quality_checks=(
    "Rustfmt Check:cargo fmt --check"
    "Clippy Analysis:cargo clippy --workspace --all-features -- -D warnings"
    "Security Audit:cargo audit"
)

for check in "${quality_checks[@]}"; do
    IFS=':' read -r check_name check_command <<< "$check"
    total_tests=$((total_tests + 1))
    
    if run_test_with_timeout "$check_name" "$check_command" 60; then
        passed_tests=$((passed_tests + 1))
    else
        failed_tests=$((failed_tests + 1))
    fi
done

# Generate comprehensive report
echo -e "${BLUE}=== Test Report Generation ===${NC}"
report_file="$TEMP_DIR/test_report.md"

cat > "$report_file" << EOF
# Depyler Comprehensive Test Report

**Generated:** $(date)
**Total Tests:** $total_tests
**Passed:** $passed_tests
**Failed:** $failed_tests
**Success Rate:** $(( passed_tests * 100 / total_tests ))%

## Coverage Analysis
- **Coverage:** ${coverage_percent:-N/A}%
- **Threshold:** ${COVERAGE_THRESHOLD}%
- **Status:** $([ "$passed_tests" -eq "$total_tests" ] && echo "✅ PASSED" || echo "❌ FAILED")

## Test Categories
EOF

for category in "${!test_categories[@]}"; do
    echo "- $category" >> "$report_file"
done

echo "
## Quality Gates
- Code formatting (rustfmt)
- Static analysis (clippy)  
- Security audit (cargo-audit)
- Coverage threshold (${COVERAGE_THRESHOLD}%)

## Files Tested
- $(find tests/fixtures/python_samples -name "*.py" | wc -l) Python fixture files
- $(find tests/fixtures/expected_rust -name "*.rs" | wc -l) Expected Rust output files
- $(find tests -name "*.rs" | wc -l) Rust test files
" >> "$report_file"

# Copy report to project root
cp "$report_file" "./test_report.md"

# Final summary
echo -e "${BLUE}=== Final Summary ===${NC}"
echo "Total tests executed: $total_tests"
echo "Tests passed: $passed_tests"
echo "Tests failed: $failed_tests"
echo "Success rate: $(( passed_tests * 100 / total_tests ))%"

if [ "$failed_tests" -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed! Depyler meets NASA/SQLite quality standards.${NC}"
    echo "Test report saved to: test_report.md"
    exit 0
else
    echo -e "${RED}❌ $failed_tests test(s) failed. Quality gate not met.${NC}"
    echo "Test report saved to: test_report.md"
    exit 1
fi