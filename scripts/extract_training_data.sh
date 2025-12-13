#!/bin/bash
# DEPYLER-0596: Extract training data from reprorusted-python-cli examples
# Generates errors.jsonl with compilation errors for oracle training
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${REPO_ROOT}/training_corpus"
ERRORS_FILE="${OUTPUT_DIR}/errors.jsonl"
REPRORUSTED_DIR="${REPRORUSTED_DIR:-/home/noah/src/reprorusted-python-cli}"

# Create output directory
mkdir -p "${OUTPUT_DIR}"

# Temporary files
TEMP_DIR=$(mktemp -d)
# shellcheck disable=SEC011
trap '[[ -n "$TEMP_DIR" ]] && [[ -d "$TEMP_DIR" ]] && [[ "$TEMP_DIR" == /tmp/* ]] && rm -rf "$TEMP_DIR"' EXIT

echo "=== Depyler Oracle Training Data Extraction ==="
echo "Output: ${ERRORS_FILE}"
echo ""

# Build depyler if needed
if [ ! -f "${REPO_ROOT}/target/release/depyler" ]; then
    echo "Building depyler..."
    cargo build --release -p depyler --quiet
fi

DEPYLER="${REPO_ROOT}/target/release/depyler"

# Find all Python files
PYTHON_FILES=$(find "${REPRORUSTED_DIR}/examples" -name "*.py" -type f \
    ! -name "test_*.py" ! -name "__init__.py" 2>/dev/null | sort)

TOTAL=$(echo "${PYTHON_FILES}" | wc -l)
COUNT=0
ERRORS=0

# Clear output file
> "${ERRORS_FILE}"

# Process each Python file
echo "${PYTHON_FILES}" | while read -r pyfile; do
    COUNT=$((COUNT + 1))

    # Get relative path for file identification
    prefix="${REPRORUSTED_DIR}/"
    REL_PATH="${pyfile#$prefix}"
    FILE_STEM=$(basename "${pyfile}" .py)

    # Create temp project directory
    PROJ_DIR="${TEMP_DIR}/${FILE_STEM}"
    mkdir -p "${PROJ_DIR}"

    # Transpile
    RUST_CODE=$("${DEPYLER}" transpile "${pyfile}" 2>/dev/null || true)

    if [ -z "${RUST_CODE}" ]; then
        # Transpilation failed - extract error
        TRANS_ERR=$("${DEPYLER}" transpile "${pyfile}" 2>&1 || true)
        if [ -n "${TRANS_ERR}" ]; then
            # Extract error message
            ERR_MSG=$(echo "${TRANS_ERR}" | head -1 | sed 's/"/\\"/g')
            ERR_HASH=$(echo "${ERR_MSG}" | md5sum | cut -c1-16)

            # Output JSON
            echo "{\"error_code\": \"TRANS\", \"message\": \"${ERR_MSG}\", \"context\": \"transpilation\", \"file\": \"${REL_PATH}\", \"hash\": \"${ERR_HASH}\"}" >> "${ERRORS_FILE}"
            ERRORS=$((ERRORS + 1))
        fi
        continue
    fi

    # Write Rust code to lib.rs
    echo "${RUST_CODE}" > "${PROJ_DIR}/lib.rs"

    # Generate minimal Cargo.toml
    cat > "${PROJ_DIR}/Cargo.toml" << EOF
[package]
name = "${FILE_STEM}"
version = "0.1.0"
edition = "2021"

[lib]
path = "lib.rs"
EOF

    # Try to compile with cargo check
    COMPILE_OUT=$(cd "${PROJ_DIR}" && cargo check --message-format=json 2>&1 || true)

    # Extract compilation errors
    echo "${COMPILE_OUT}" | grep -o '{"reason":"compiler-message".*}' 2>/dev/null | while read -r json_line; do
        # Parse error code and message
        ERR_CODE=$(echo "${json_line}" | jq -r '.message.code.code // "UNKNOWN"' 2>/dev/null || echo "UNKNOWN")
        ERR_MSG=$(echo "${json_line}" | jq -r '.message.message // ""' 2>/dev/null | sed 's/"/\\"/g' | head -c 500)
        ERR_LEVEL=$(echo "${json_line}" | jq -r '.message.level // "error"' 2>/dev/null || echo "error")

        if [ "${ERR_LEVEL}" = "error" ] && [ -n "${ERR_MSG}" ]; then
            ERR_HASH=$(echo "${ERR_CODE}:${ERR_MSG}" | md5sum | cut -c1-16)

            # Get context (first span if available)
            CONTEXT=$(echo "${json_line}" | jq -r '.message.spans[0].text[0].text // ""' 2>/dev/null | head -c 200 | sed 's/"/\\"/g')

            echo "{\"error_code\": \"${ERR_CODE}\", \"message\": \"${ERR_MSG}\", \"context\": \"${CONTEXT}\", \"file\": \"${REL_PATH}\", \"hash\": \"${ERR_HASH}\"}" >> "${ERRORS_FILE}"
            ERRORS=$((ERRORS + 1))
        fi
    done

    # Clean up project directory
    # shellcheck disable=SEC011
    [[ -n "$PROJ_DIR" ]] && [[ -d "$PROJ_DIR" ]] && [[ "$PROJ_DIR" == /tmp/* ]] && rm -rf "$PROJ_DIR"

    # Progress
    if [ $((COUNT % 50)) -eq 0 ]; then
        echo "Processed ${COUNT}/${TOTAL} files..."
    fi
done

# Deduplicate by hash
UNIQUE_FILE="${TEMP_DIR}/unique_errors.jsonl"
sort -u -t'"' -k12 "${ERRORS_FILE}" > "${UNIQUE_FILE}"
mv "${UNIQUE_FILE}" "${ERRORS_FILE}"

UNIQUE_COUNT=$(wc -l < "${ERRORS_FILE}")

echo ""
echo "=== Extraction Complete ==="
echo "Total files processed: ${TOTAL}"
echo "Unique errors extracted: ${UNIQUE_COUNT}"
echo "Output: ${ERRORS_FILE}"
