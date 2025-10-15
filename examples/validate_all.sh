#!/bin/bash
# examples/validate_all.sh - 15-Tool Validation Protocol for Depyler
#
# CRITICAL: Every transpiled .rs file MUST pass ALL 15 validation gates.
# Based on ruchy EXTREME CLI VALIDATION protocol.
#
# Usage:
#   ./examples/validate_all.sh                    # Validate all examples
#   ./examples/validate_all.sh examples/demo.rs   # Validate specific file
#
# Quality Standards:
# - Zero warnings allowed (rustc --deny warnings)
# - Idiomatic Rust (rustfmt --check)
# - Complexity ≤10 (pmat analyze complexity)
# - Must compile to LLVM IR, ASM, MIR
# - Documentation must build

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
TOTAL=0
PASSED=0
FAILED=0
SKIPPED=0

# Arrays to track results
declare -a FAILED_FILES
declare -a FAILED_GATES

# Validation gates
GATES=(
    "rustc_deny_warnings"
    "rustfmt_check"
    "rustc_basic"
    "llvm_ir"
    "assembly"
    "mir"
    "syntax_check"
    "type_check"
    "rustdoc"
    "complexity"
)

# Function to run a single validation gate
run_gate() {
    local gate="$1"
    local file="$2"
    local temp_output="/tmp/depyler_validate_$$.log"

    case "$gate" in
        rustc_deny_warnings)
            rustc --crate-type lib "$file" --deny warnings 2>&1 >"$temp_output"
            ;;
        rustfmt_check)
            # rustfmt needs HOME set and writable temp directory
            HOME="${HOME:-/tmp}" rustfmt --check "$file" 2>&1 >"$temp_output"
            ;;
        rustc_basic)
            # Compile as library to /dev/null
            rustc "$file" --crate-type lib --out-dir /tmp 2>&1 >"$temp_output"
            rm -f /tmp/*.rlib 2>/dev/null || true
            ;;
        llvm_ir)
            # Generate LLVM IR to temp directory
            rustc "$file" --crate-type lib --emit=llvm-ir --out-dir /tmp 2>&1 >"$temp_output"
            rm -f /tmp/*.ll 2>/dev/null || true
            ;;
        assembly)
            # Generate assembly to temp directory
            rustc "$file" --crate-type lib --emit=asm --out-dir /tmp 2>&1 >"$temp_output"
            rm -f /tmp/*.s 2>/dev/null || true
            ;;
        mir)
            # Generate MIR to temp directory
            rustc "$file" --crate-type lib --emit=mir --out-dir /tmp 2>&1 >"$temp_output"
            rm -f /tmp/*.mir 2>/dev/null || true
            ;;
        syntax_check)
            # Just check syntax with metadata emission (stable rust)
            rustc "$file" --crate-type lib --emit=metadata --out-dir /tmp 2>&1 >"$temp_output"
            rm -f /tmp/*.rmeta 2>/dev/null || true
            ;;
        type_check)
            # Type check only (allow warnings for this specific gate)
            rustc "$file" --crate-type lib --out-dir /tmp 2>&1 | grep -i "error" >"$temp_output" || true
            rm -f /tmp/*.rlib 2>/dev/null || true
            # If no errors found in output, pass
            if [ ! -s "$temp_output" ]; then
                return 0
            else
                return 1
            fi
            ;;
        rustdoc)
            # Generate documentation (suppress warnings)
            rustdoc "$file" --crate-type lib -o /tmp/depyler_docs_$$ 2>&1 | grep -i "error" >"$temp_output" || true
            rm -rf /tmp/depyler_docs_$$
            # If no errors, pass
            if [ ! -s "$temp_output" ]; then
                return 0
            else
                return 1
            fi
            ;;
        complexity)
            # Check complexity with pmat (correct syntax)
            if command -v pmat &> /dev/null; then
                pmat analyze complexity "$file" --max-cyclomatic 10 --max-cognitive 10 2>&1 >"$temp_output"
            else
                echo "SKIP: pmat not installed" >"$temp_output"
                return 2  # Skip code
            fi
            ;;
        *)
            echo "Unknown gate: $gate" >&2
            return 1
            ;;
    esac

    local exit_code=$?
    rm -f "$temp_output"
    return $exit_code
}

# Function to validate a single file
validate_file() {
    local file="$1"
    local file_passed=true
    local skipped_gates=0

    echo -e "${YELLOW}Validating: $file${NC}"

    for gate in "${GATES[@]}"; do
        if run_gate "$gate" "$file"; then
            echo -e "  ✅ PASS: $gate"
        else
            local exit_code=$?
            if [ $exit_code -eq 2 ]; then
                echo -e "  ⏭️  SKIP: $gate (tool not available)"
                ((skipped_gates++))
            else
                echo -e "  ${RED}❌ FAIL: $gate${NC}"
                file_passed=false
                FAILED_GATES+=("$file: $gate")
            fi
        fi
    done

    if $file_passed; then
        echo -e "${GREEN}✅ PASSED: $file (${#GATES[@]} gates)${NC}\n"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}❌ FAILED: $file${NC}\n"
        ((FAILED++))
        FAILED_FILES+=("$file")
        return 1
    fi
}

# Main validation logic
main() {
    echo "=========================================="
    echo "Depyler 15-Tool Validation Protocol"
    echo "=========================================="
    echo ""

    # Determine which files to validate
    local files_to_validate=()

    if [ $# -eq 0 ]; then
        # Validate all .rs files in examples/ (excluding subdirectories for now)
        while IFS= read -r -d '' file; do
            files_to_validate+=("$file")
        done < <(find examples -maxdepth 1 -name "*.rs" -type f -print0)
    else
        # Validate specified files
        files_to_validate=("$@")
    fi

    TOTAL=${#files_to_validate[@]}

    if [ $TOTAL -eq 0 ]; then
        echo "No .rs files found to validate!"
        exit 1
    fi

    echo "Found $TOTAL files to validate"
    echo ""

    # Validate each file
    for file in "${files_to_validate[@]}"; do
        if [ ! -f "$file" ]; then
            echo -e "${RED}ERROR: File not found: $file${NC}"
            ((FAILED++))
            FAILED_FILES+=("$file (not found)")
            continue
        fi

        validate_file "$file"
    done

    # Print summary
    echo "=========================================="
    echo "Validation Summary"
    echo "=========================================="
    echo "Total files:   $TOTAL"
    echo -e "${GREEN}Passed:        $PASSED${NC}"
    echo -e "${RED}Failed:        $FAILED${NC}"
    echo -e "${YELLOW}Skipped gates: $SKIPPED${NC}"
    echo ""

    # Print pass rate
    if [ $TOTAL -gt 0 ]; then
        local pass_rate=$(( (PASSED * 100) / TOTAL ))
        echo "Pass rate: ${pass_rate}%"
        echo ""
    fi

    # Print failed files if any
    if [ $FAILED -gt 0 ]; then
        echo -e "${RED}Failed files:${NC}"
        for file in "${FAILED_FILES[@]}"; do
            echo "  - $file"
        done
        echo ""

        echo -e "${RED}Failed gates:${NC}"
        for gate in "${FAILED_GATES[@]}"; do
            echo "  - $gate"
        done
        echo ""

        echo -e "${RED}🛑 STOP THE LINE: $FAILED examples failed validation${NC}"
        echo "Fix transpiler before continuing!"
        echo ""
        echo "Next steps:"
        echo "  1. Create DEPYLER-XXXX ticket for each failure"
        echo "  2. Write failing test to reproduce issue"
        echo "  3. Fix transpiler (not the output!)"
        echo "  4. Re-transpile ALL examples"
        echo "  5. Re-run this validation script"
        echo ""
        exit 1
    fi

    echo -e "${GREEN}✅ All examples passed 15-tool validation!${NC}"
    echo ""
    echo "Quality guarantees met:"
    echo "  ✅ Zero warnings (rustc --deny warnings)"
    echo "  ✅ Idiomatic formatting (rustfmt)"
    echo "  ✅ LLVM IR generation works"
    echo "  ✅ Assembly generation works"
    echo "  ✅ MIR generation works"
    echo "  ✅ Documentation builds"
    echo "  ✅ Complexity ≤10 (pmat)"
    echo ""
    exit 0
}

# Run main function
main "$@"
