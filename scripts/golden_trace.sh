#!/bin/bash
# DEPYLER-0956: Golden Trace Validation
#
# Validates transpiled Rust code produces semantically equivalent output
# to the original Python by comparing syscall-level execution traces.
#
# Uses Renacer (https://github.com/paiml/renacer) for trace capture.
#
# Usage:
#   ./scripts/golden_trace.sh capture <python_file> <output_dir>  # Capture baseline
#   ./scripts/golden_trace.sh validate <rust_binary> <output_dir> # Validate against baseline
#   ./scripts/golden_trace.sh compare <trace1.json> <trace2.json> # Compare traces
#   ./scripts/golden_trace.sh ci <example_dir>                    # Full CI validation
#
# Toyota Way: Jidoka (stop on defect) - fail CI on semantic divergence

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Renacer path (prefer local install, then cargo install)
RENACER="${RENACER:-$(which renacer 2>/dev/null || echo '/home/noah/src/renacer/target/debug/renacer')}"

# Check if renacer is available
check_renacer() {
    if [[ ! -x "$RENACER" ]]; then
        echo -e "${YELLOW}⚠️  Renacer not found. Install with: cargo install renacer${NC}"
        echo "   Or set RENACER environment variable to the path"
        return 1
    fi
    return 0
}

# Capture Python golden trace
capture_python_trace() {
    local python_file="$1"
    local output_dir="$2"

    if [[ ! -f "$python_file" ]]; then
        echo -e "${RED}❌ Python file not found: $python_file${NC}"
        return 1
    fi

    mkdir -p "$output_dir"
    local trace_file="$output_dir/golden_python.json"

    echo -e "${BLUE}📸 Capturing Python trace for: $python_file${NC}"

    # Capture syscall trace
    "$RENACER" --format json -- python3 "$python_file" > "$trace_file" 2>&1 || true

    # Capture output
    python3 "$python_file" > "$output_dir/python_output.txt" 2>&1 || true

    echo -e "${GREEN}✅ Trace captured: $trace_file${NC}"
}

# Capture Rust trace and validate
capture_rust_trace() {
    local rust_binary="$1"
    local output_dir="$2"

    if [[ ! -x "$rust_binary" ]]; then
        echo -e "${RED}❌ Rust binary not found or not executable: $rust_binary${NC}"
        return 1
    fi

    local trace_file="$output_dir/golden_rust.json"

    echo -e "${BLUE}📸 Capturing Rust trace for: $rust_binary${NC}"

    # Capture syscall trace
    "$RENACER" --format json -- "$rust_binary" > "$trace_file" 2>&1 || true

    # Capture output
    "$rust_binary" > "$output_dir/rust_output.txt" 2>&1 || true

    echo -e "${GREEN}✅ Trace captured: $trace_file${NC}"
}

# Compare Python and Rust outputs
compare_outputs() {
    local output_dir="$1"
    local python_output="$output_dir/python_output.txt"
    local rust_output="$output_dir/rust_output.txt"

    if [[ ! -f "$python_output" || ! -f "$rust_output" ]]; then
        echo -e "${YELLOW}⚠️  Missing output files for comparison${NC}"
        return 1
    fi

    echo -e "${BLUE}🔍 Comparing outputs...${NC}"

    if diff -q "$python_output" "$rust_output" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Output equivalence: PASS${NC}"
        return 0
    else
        echo -e "${RED}❌ Output equivalence: FAIL${NC}"
        echo -e "${YELLOW}Differences:${NC}"
        diff "$python_output" "$rust_output" | head -20
        return 1
    fi
}

# Compare trace patterns (semantic equivalence)
compare_traces() {
    local trace1="$1"
    local trace2="$2"

    if [[ ! -f "$trace1" || ! -f "$trace2" ]]; then
        echo -e "${YELLOW}⚠️  Missing trace files${NC}"
        return 1
    fi

    echo -e "${BLUE}🔍 Analyzing trace patterns...${NC}"

    # Extract syscall names for pattern comparison
    local python_calls=$(grep -oP '"syscall_name":\s*"[^"]+' "$trace1" 2>/dev/null | sort | uniq -c | sort -rn | head -10)
    local rust_calls=$(grep -oP '"syscall_name":\s*"[^"]+' "$trace2" 2>/dev/null | sort | uniq -c | sort -rn | head -10)

    echo "Python syscall patterns:"
    echo "$python_calls" | head -5
    echo ""
    echo "Rust syscall patterns:"
    echo "$rust_calls" | head -5

    # Calculate performance difference
    local python_count=$(wc -l < "$trace1" 2>/dev/null || echo 0)
    local rust_count=$(wc -l < "$trace2" 2>/dev/null || echo 0)

    if [[ $python_count -gt 0 && $rust_count -gt 0 ]]; then
        local ratio=$(echo "scale=2; $python_count / $rust_count" | bc 2>/dev/null || echo "N/A")
        echo ""
        echo -e "${BLUE}📊 Syscall ratio (Python/Rust): ${ratio}x${NC}"
    fi
}

# Full CI validation for an example
ci_validation() {
    local example_dir="$1"
    local status=0

    echo -e "${BLUE}🧪 DEPYLER-0956: Golden Trace CI Validation${NC}"
    echo "Example: $example_dir"
    echo "================================"

    # Find Python file
    local python_file=$(find "$example_dir" -name "*.py" -type f | head -1)
    if [[ -z "$python_file" ]]; then
        echo -e "${YELLOW}⚠️  No Python file found in $example_dir${NC}"
        return 1
    fi

    # Create output directory
    local trace_dir="$example_dir/.golden_traces"
    mkdir -p "$trace_dir"

    # Step 1: Capture Python baseline
    echo ""
    echo -e "${BLUE}Step 1: Capturing Python baseline...${NC}"
    capture_python_trace "$python_file" "$trace_dir" || status=1

    # Step 2: Transpile Python to Rust
    echo ""
    echo -e "${BLUE}Step 2: Transpiling Python to Rust...${NC}"
    local rs_file="$trace_dir/transpiled.rs"
    ./target/release/depyler transpile "$python_file" -o "$rs_file" 2>/dev/null || {
        echo -e "${YELLOW}⚠️  Transpilation failed, skipping trace validation${NC}"
        return 1
    }

    # Step 3: Compile Rust
    echo ""
    echo -e "${BLUE}Step 3: Compiling Rust...${NC}"
    local rust_binary="$trace_dir/transpiled"
    rustc "$rs_file" -o "$rust_binary" 2>/dev/null || {
        echo -e "${YELLOW}⚠️  Rust compilation failed, skipping trace validation${NC}"
        return 1
    }

    # Step 4: Capture Rust trace
    echo ""
    echo -e "${BLUE}Step 4: Capturing Rust trace...${NC}"
    capture_rust_trace "$rust_binary" "$trace_dir" || status=1

    # Step 5: Compare
    echo ""
    echo -e "${BLUE}Step 5: Validating semantic equivalence...${NC}"
    compare_outputs "$trace_dir" || status=1

    echo ""
    echo -e "${BLUE}Step 6: Analyzing trace patterns...${NC}"
    compare_traces "$trace_dir/golden_python.json" "$trace_dir/golden_rust.json"

    echo ""
    echo "================================"
    if [[ $status -eq 0 ]]; then
        echo -e "${GREEN}✅ Golden Trace Validation: PASSED${NC}"
    else
        echo -e "${RED}❌ Golden Trace Validation: FAILED${NC}"
    fi

    return $status
}

# Print usage
usage() {
    echo "Usage: $0 <command> [args]"
    echo ""
    echo "Commands:"
    echo "  capture <python_file> <output_dir>   Capture Python golden trace"
    echo "  validate <rust_binary> <output_dir>  Capture and validate Rust trace"
    echo "  compare <trace1.json> <trace2.json>  Compare two trace files"
    echo "  ci <example_dir>                     Full CI validation for an example"
    echo ""
    echo "Environment:"
    echo "  RENACER  Path to renacer binary (default: auto-detect)"
}

# Main
case "${1:-}" in
    capture)
        check_renacer && capture_python_trace "$2" "$3"
        ;;
    validate)
        check_renacer && capture_rust_trace "$2" "$3"
        ;;
    compare)
        compare_traces "$2" "$3"
        ;;
    ci)
        check_renacer && ci_validation "$2"
        ;;
    --help|-h)
        usage
        exit 0
        ;;
    *)
        echo -e "${BLUE}🔍 DEPYLER-0956: Golden Trace Validation${NC}"
        echo ""
        usage
        exit 1
        ;;
esac
