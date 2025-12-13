#!/bin/bash

# Performance and Benchmarking Test Suite
# Runs comprehensive performance tests and generates reports

set -e

echo "=== Depyler Performance Test Suite ==="
echo "Starting comprehensive performance and integration testing..."
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to run test and measure time
run_timed_test() {
    local test_name=$1
    local test_command=$2
    
    print_status $BLUE "Running ${test_name}..."
    start_time=$(date +%s.%N)
    
    if bash -c "$test_command"; then
        end_time=$(date +%s.%N)
        duration=$(echo "$end_time - $start_time" | bc -l)
        print_status $GREEN "✓ ${test_name} completed in ${duration}s"
        echo "$test_name,$duration,PASS" >> performance_results.csv
    else
        end_time=$(date +%s.%N)
        duration=$(echo "$end_time - $start_time" | bc -l)
        print_status $RED "✗ ${test_name} failed in ${duration}s"
        echo "$test_name,$duration,FAIL" >> performance_results.csv
    fi
    echo
}

# Initialize results file
echo "Test Name,Duration (s),Result" > performance_results.csv

print_status $YELLOW "=== Phase 1: Property Test Benchmarks ==="
run_timed_test "Property Test Benchmarks" "cargo test --test property_test_benchmarks --release"

print_status $YELLOW "=== Phase 2: Integration Benchmarks ==="
run_timed_test "Integration Benchmarks" "cargo test --test integration_benchmarks --release"

print_status $YELLOW "=== Phase 3: Example Validation ==="
run_timed_test "Example Validation" "cargo test --test example_validation --release"

print_status $YELLOW "=== Phase 4: Coverage Analysis ==="
run_timed_test "Coverage Analysis" "cargo test --test coverage_analysis --release"

print_status $YELLOW "=== Phase 5: All Property Tests ==="
run_timed_test "All Property Tests" "make test-property"

print_status $YELLOW "=== Phase 6: Edge Case Coverage ==="
run_timed_test "Edge Case Coverage" "cargo test --test edge_case_coverage --release"

print_status $YELLOW "=== Phase 7: Error Path Coverage ==="
run_timed_test "Error Path Coverage" "cargo test --test error_path_coverage --release"

print_status $YELLOW "=== Phase 8: Boundary Value Tests ==="
run_timed_test "Boundary Value Tests" "cargo test --test boundary_value_tests --release"

print_status $YELLOW "=== Phase 9: Full Test Suite ==="
run_timed_test "Full Test Suite" "cargo test --workspace --release"

# Generate performance report
print_status $BLUE "Generating performance report..."

echo "=== PERFORMANCE TEST RESULTS ===" > performance_report.txt
echo "Generated at: $(date)" >> performance_report.txt
echo >> performance_report.txt

echo "Test Results Summary:" >> performance_report.txt
echo "====================" >> performance_report.txt

# Count pass/fail
total_tests=$(tail -n +2 performance_results.csv | wc -l)
passed_tests=$(tail -n +2 performance_results.csv | grep ",PASS" | wc -l)
failed_tests=$(tail -n +2 performance_results.csv | grep ",FAIL" | wc -l)

echo "Total Tests: $total_tests" >> performance_report.txt
echo "Passed: $passed_tests" >> performance_report.txt
echo "Failed: $failed_tests" >> performance_report.txt
echo >> performance_report.txt

echo "Detailed Results:" >> performance_report.txt
echo "=================" >> performance_report.txt
cat performance_results.csv >> performance_report.txt

# Calculate total time
total_time=$(tail -n +2 performance_results.csv | cut -d',' -f2 | paste -sd+ | bc -l)
echo >> performance_report.txt
echo "Total Execution Time: ${total_time}s" >> performance_report.txt

# Performance analysis
echo >> performance_report.txt
echo "Performance Analysis:" >> performance_report.txt
echo "====================" >> performance_report.txt

# Find slowest test
slowest_test=$(tail -n +2 performance_results.csv | sort -t',' -k2 -nr | head -1)
echo "Slowest Test: $slowest_test" >> performance_report.txt

# Find fastest test
fastest_test=$(tail -n +2 performance_results.csv | sort -t',' -k2 -n | head -1)
echo "Fastest Test: $fastest_test" >> performance_report.txt

# Check if under target time (5 minutes = 300 seconds)
target_time=300
if (( $(echo "$total_time < $target_time" | bc -l) )); then
    echo "✓ Performance Target: PASSED (${total_time}s < ${target_time}s)" >> performance_report.txt
else
    echo "✗ Performance Target: FAILED (${total_time}s >= ${target_time}s)" >> performance_report.txt
fi

print_status $GREEN "=== Performance Testing Complete ==="
print_status $BLUE "Results saved to: performance_report.txt"
print_status $BLUE "Raw data saved to: performance_results.csv"

# Display summary
echo
cat performance_report.txt

# Check overall success
if [ "$failed_tests" -eq 0 ]; then
    print_status $GREEN "🎉 All performance tests passed!"
    exit 0
else
    print_status $RED "❌ $failed_tests test(s) failed"
    exit 1
fi