#!/bin/bash
# DEPYLER-0596: Extract training data from reprorusted-python-cli examples
# Generates errors.jsonl with compilation errors for oracle training
# bashrs compliant - NASA quality with embedded unit tests
set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
OUTPUT_DIR="${REPO_ROOT}/training_corpus"
ERRORS_FILE="${OUTPUT_DIR}/errors.jsonl"
REPRORUSTED_DIR="${REPRORUSTED_DIR:-/home/noah/src/reprorusted-python-cli}"

# Grep pattern for extracting compiler messages
readonly COMPILER_MSG_PATTERN='{"reason":"compiler-message"[^}]*}'

# ============================================================================
# HELPER FUNCTIONS (Testable)
# ============================================================================

# Compute MD5 hash (first 16 chars)
compute_hash() {
    local input="$1"
    printf '%s' "${input}" | md5sum | cut -c1-16
}

# Escape JSON string (quotes and limit to 500 chars)
escape_json() {
    local input="$1"
    local escaped
    escaped="${input//\"/\\\"}"
    printf '%s' "${escaped}" | head -c 500
}

# Validate directory exists
validate_dir() {
    local dir="$1"
    [[ -d "${dir}" ]]
}

# Validate file exists
validate_file() {
    local file="$1"
    [[ -f "${file}" ]]
}

# Safe cleanup of temp directory
safe_cleanup() {
    local dir="$1"
    if [[ -n "${dir}" && -d "${dir}" ]]; then
        if [[ "${dir}" == /tmp/* || "${dir}" == /var/* ]]; then
            rm -rf "${dir:?}"
            return 0
        fi
    fi
    return 1
}

# Generate Cargo.toml content
generate_cargo_toml() {
    local name="$1"
    cat <<EOF
[package]
name = "${name}"
version = "0.1.0"
edition = "2021"

[lib]
path = "lib.rs"
EOF
}

# Format error as JSON
format_error_json() {
    local error_code="$1"
    local message="$2"
    local context="$3"
    local file="$4"
    local hash="$5"
    printf '{"error_code": "%s", "message": "%s", "context": "%s", "file": "%s", "hash": "%s"}\n' \
        "${error_code}" "${message}" "${context}" "${file}" "${hash}"
}

# ============================================================================
# UNIT TESTS (bashrs test discovers test_* functions)
# ============================================================================

test_compute_hash_returns_16_chars() {
    local result
    result=$(compute_hash "test input")
    [[ ${#result} -eq 16 ]] || return 1
    echo "PASS: compute_hash returns 16 character hash"
}

test_compute_hash_deterministic() {
    local hash1 hash2
    hash1=$(compute_hash "same input")
    hash2=$(compute_hash "same input")
    [[ "${hash1}" == "${hash2}" ]] || return 1
    echo "PASS: compute_hash is deterministic"
}

test_compute_hash_different_inputs() {
    local hash1 hash2
    hash1=$(compute_hash "input1")
    hash2=$(compute_hash "input2")
    [[ "${hash1}" != "${hash2}" ]] || return 1
    echo "PASS: compute_hash produces different hashes for different inputs"
}

test_escape_json_escapes_quotes() {
    local result
    result=$(escape_json 'test "quotes" here')
    [[ "${result}" == 'test \"quotes\" here' ]] || return 1
    echo "PASS: escape_json escapes double quotes"
}

test_escape_json_handles_empty() {
    local result
    result=$(escape_json "")
    [[ "${result}" == "" ]] || return 1
    echo "PASS: escape_json handles empty string"
}

test_escape_json_truncates_long() {
    local long_input result
    long_input=$(printf 'x%.0s' {1..600})
    result=$(escape_json "${long_input}")
    [[ ${#result} -le 500 ]] || return 1
    echo "PASS: escape_json truncates to 500 chars"
}

test_validate_dir_existing() {
    validate_dir "/tmp" || return 1
    echo "PASS: validate_dir returns true for existing directory"
}

test_validate_dir_nonexistent() {
    ! validate_dir "/nonexistent_dir_12345" || return 1
    echo "PASS: validate_dir returns false for nonexistent directory"
}

test_validate_file_existing() {
    local tmpfile
    tmpfile=$(mktemp)
    validate_file "${tmpfile}" || { rm -f "${tmpfile}"; return 1; }
    rm -f "${tmpfile}"
    echo "PASS: validate_file returns true for existing file"
}

test_validate_file_nonexistent() {
    ! validate_file "/nonexistent_file_12345" || return 1
    echo "PASS: validate_file returns false for nonexistent file"
}

test_safe_cleanup_tmp_dir() {
    local tmpdir
    tmpdir=$(mktemp -d)
    safe_cleanup "${tmpdir}" || return 1
    [[ ! -d "${tmpdir}" ]] || return 1
    echo "PASS: safe_cleanup removes /tmp directory"
}

test_safe_cleanup_rejects_non_tmp() {
    ! safe_cleanup "/home/test" || return 1
    echo "PASS: safe_cleanup rejects non-tmp directories"
}

test_safe_cleanup_handles_empty() {
    ! safe_cleanup "" || return 1
    echo "PASS: safe_cleanup handles empty input"
}

test_generate_cargo_toml_format() {
    local result
    result=$(generate_cargo_toml "test_project")
    [[ "${result}" == *'name = "test_project"'* ]] || return 1
    [[ "${result}" == *'edition = "2021"'* ]] || return 1
    echo "PASS: generate_cargo_toml produces valid format"
}

test_format_error_json_structure() {
    local result
    result=$(format_error_json "E0001" "test msg" "context" "file.py" "abc123")
    [[ "${result}" == *'"error_code": "E0001"'* ]] || return 1
    [[ "${result}" == *'"message": "test msg"'* ]] || return 1
    [[ "${result}" == *'"file": "file.py"'* ]] || return 1
    echo "PASS: format_error_json produces valid JSON structure"
}

test_compiler_msg_pattern_valid() {
    local test_input='{"reason":"compiler-message","data":"test"}'
    local result
    result=$(grep -oE "${COMPILER_MSG_PATTERN}" <<<"${test_input}" 2>/dev/null) || true
    [[ -n "${result}" ]] || return 1
    echo "PASS: COMPILER_MSG_PATTERN matches valid input"
}

# Run all tests
run_tests() {
    local failed=0
    local passed=0

    echo "=== Running Unit Tests ==="
    echo ""

    # List of test functions (explicit to avoid shell injection)
    local -a test_functions=(
        test_compute_hash_returns_16_chars
        test_compute_hash_deterministic
        test_compute_hash_different_inputs
        test_escape_json_escapes_quotes
        test_escape_json_handles_empty
        test_escape_json_truncates_long
        test_validate_dir_existing
        test_validate_dir_nonexistent
        test_validate_file_existing
        test_validate_file_nonexistent
        test_safe_cleanup_tmp_dir
        test_safe_cleanup_rejects_non_tmp
        test_safe_cleanup_handles_empty
        test_generate_cargo_toml_format
        test_format_error_json_structure
        test_compiler_msg_pattern_valid
    )

    local test_func
    for test_func in "${test_functions[@]}"; do
        if "${test_func}"; then
            ((++passed))
        else
            echo "FAIL: ${test_func}"
            ((++failed))
        fi
    done

    echo ""
    echo "=== Test Summary ==="
    echo "Passed: ${passed}"
    echo "Failed: ${failed}"
    echo ""

    [[ "${failed}" -eq 0 ]]
}

# ============================================================================
# MAIN FUNCTION
# ============================================================================

main() {
    # Create output directory
    mkdir -p "${OUTPUT_DIR}"

    # Temporary files with safe cleanup
    local TEMP_DIR
    TEMP_DIR=$(mktemp -d)
    # shellcheck disable=SC2064
    trap "safe_cleanup '${TEMP_DIR}'" EXIT

    echo "=== Depyler Oracle Training Data Extraction ==="
    echo "Output: ${ERRORS_FILE}"
    echo ""

    # Build depyler if needed
    if [[ ! -f "${REPO_ROOT}/target/release/depyler" ]]; then
        echo "Building depyler..."
        cargo build --release -p depyler --quiet
    fi

    local DEPYLER="${REPO_ROOT}/target/release/depyler"

    # Find all Python files and store in array
    local -a PYTHON_FILES=()
    while IFS= read -r line; do
        PYTHON_FILES+=("${line}")
    done < <(find "${REPRORUSTED_DIR}/examples" -name "*.py" -type f \
        ! -name "test_*.py" ! -name "__init__.py" 2>/dev/null | sort)

    local TOTAL=${#PYTHON_FILES[@]}
    if [[ "${TOTAL}" -eq 0 ]]; then
        echo "Error: No Python files found in ${REPRORUSTED_DIR}/examples"
        exit 1
    fi

    local COUNT=0
    local ERRORS=0

    # Clear output file
    >"${ERRORS_FILE}"

    # Process each Python file
    local pyfile prefix REL_PATH FILE_STEM PROJ_DIR RS_OUTPUT TRANS_OUTPUT
    for pyfile in "${PYTHON_FILES[@]}"; do
        COUNT=$((COUNT+1))

        # Get relative path for file identification
        prefix="${REPRORUSTED_DIR}/"
        REL_PATH="${pyfile#"$prefix"}"
        FILE_STEM=$(basename "${pyfile}" .py)

        # Create temp project directory
        PROJ_DIR="${TEMP_DIR}/${FILE_STEM}"
        mkdir -p "${PROJ_DIR}"

        # Transpile to temp project directory
        RS_OUTPUT="${PROJ_DIR}/lib.rs"
        if ! TRANS_OUTPUT=$("${DEPYLER}" transpile "${pyfile}" -o "${RS_OUTPUT}" 2>&1); then
            TRANS_OUTPUT="${TRANS_OUTPUT:-}"
        fi

        if [[ ! -f "${RS_OUTPUT}" ]]; then
            # Transpilation failed - extract error
            if [[ -n "${TRANS_OUTPUT}" ]]; then
                local ERR_MSG ERR_HASH
                ERR_MSG=$(escape_json "$(head -1 <<<"${TRANS_OUTPUT}")")
                ERR_HASH=$(compute_hash "${ERR_MSG}")
                format_error_json "TRANS" "${ERR_MSG}" "transpilation" "${REL_PATH}" "${ERR_HASH}" >>"${ERRORS_FILE}"
                ERRORS=$((ERRORS+1))
            fi
            continue
        fi

        # Generate minimal Cargo.toml
        generate_cargo_toml "${FILE_STEM}" >"${PROJ_DIR}/Cargo.toml"

        # Try to compile with cargo check
        local COMPILE_OUT
        if ! COMPILE_OUT=$(cd "${PROJ_DIR}" && cargo check --message-format=json 2>&1); then
            COMPILE_OUT="${COMPILE_OUT:-}"
        fi

        # Extract compilation errors using jq
        local json_line ERR_CODE ERR_MSG_RAW ERR_MSG ERR_LEVEL ERR_HASH CONTEXT_RAW CONTEXT
        while IFS= read -r json_line; do
            [[ -z "${json_line}" ]] && continue

            ERR_CODE=$(jq -r '.message.code.code // "UNKNOWN"' <<<"${json_line}" 2>/dev/null) || ERR_CODE="UNKNOWN"
            ERR_MSG_RAW=$(jq -r '.message.message // ""' <<<"${json_line}" 2>/dev/null) || ERR_MSG_RAW=""
            ERR_MSG=$(escape_json "${ERR_MSG_RAW}")
            ERR_LEVEL=$(jq -r '.message.level // "error"' <<<"${json_line}" 2>/dev/null) || ERR_LEVEL="error"

            if [[ "${ERR_LEVEL}" == "error" && -n "${ERR_MSG}" ]]; then
                ERR_HASH=$(compute_hash "${ERR_CODE}:${ERR_MSG}")
                CONTEXT_RAW=$(jq -r '.message.spans[0].text[0].text // ""' <<<"${json_line}" 2>/dev/null) || CONTEXT_RAW=""
                CONTEXT=$(escape_json "${CONTEXT_RAW}" | head -c 200)
                format_error_json "${ERR_CODE}" "${ERR_MSG}" "${CONTEXT}" "${REL_PATH}" "${ERR_HASH}" >>"${ERRORS_FILE}"
                ERRORS=$((ERRORS+1))
            fi
        done < <(grep -oE "${COMPILER_MSG_PATTERN}" <<<"${COMPILE_OUT}" 2>/dev/null || true)

        # Clean up project directory safely
        safe_cleanup "${PROJ_DIR}" || true

        # Progress
        if [[ $((COUNT % 50)) -eq 0 ]]; then
            echo "Processed ${COUNT}/${TOTAL} files..."
        fi
    done

    # Deduplicate by hash
    local UNIQUE_FILE="${TEMP_DIR}/unique_errors.jsonl"
    sort -u -t'"' -k12 "${ERRORS_FILE}" >"${UNIQUE_FILE}"
    mv "${UNIQUE_FILE}" "${ERRORS_FILE}"

    local UNIQUE_COUNT
    UNIQUE_COUNT=$(wc -l < "${ERRORS_FILE}")

    echo ""
    echo "=== Extraction Complete ==="
    echo "Total files processed: ${TOTAL}"
    echo "Unique errors extracted: ${UNIQUE_COUNT}"
    echo "Output: ${ERRORS_FILE}"
}

# ============================================================================
# ENTRY POINT
# ============================================================================

# Detect if running under bashrs test harness (scripts extracted to temp with bashrs_test prefix)
_is_bashrs_test() {
    [[ "${0:-}" == *bashrs_test* ]]
}

# Detect if script is being sourced vs executed directly
# shellcheck disable=SC2317  # False positive: code IS reachable when executed
_depyler_main_entry() {
    # When sourced, this function is defined but never called
    if [[ "${1:-}" == "--test" ]]; then
        run_tests
        exit $?
    fi
    main
}

# Only call entry point when executed directly (not sourced or under bashrs test)
# Using BASH_SOURCE comparison - works for both bash and zsh
if [[ "${BASH_SOURCE[0]}" == "${0}" ]] && ! _is_bashrs_test; then
    _depyler_main_entry "${1:-}"
fi
