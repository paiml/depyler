#!/bin/bash
# DEPYLER-0596: Extract training data from OIP (Organizational Intelligence Plugin)
# Converts OIP format to depyler oracle format
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${REPO_ROOT}/training_corpus"
OIP_OUTPUT="${OUTPUT_DIR}/oip_data.json"
REPRORUSTED_DIR="${REPRORUSTED_DIR:-/home/noah/src/reprorusted-python-cli}"
OIP_BINARY="/home/noah/src/organizational-intelligence-plugin/target/release/oip"

# Validate paths don't contain path traversal
if [[ "${OUTPUT_DIR}" == *".."* ]]; then
    echo "Error: Invalid path - potential path traversal detected"
    exit 1
fi

# Create output directory (path validated above)
# bashrs: SEC010 false positive - OUTPUT_DIR is constructed from REPO_ROOT which uses pwd
install -d "${OUTPUT_DIR}"

echo "=== OIP Training Data Extraction ==="
echo "Output: ${OIP_OUTPUT}"
echo ""

# Check if OIP exists
if [ ! -f "${OIP_BINARY}" ]; then
    echo "Warning: OIP binary not found at ${OIP_BINARY}"
    echo "Building OIP..."
    (cd /home/noah/src/organizational-intelligence-plugin && cargo build --release --quiet 2>/dev/null) || {
        echo "OIP build failed, skipping OIP extraction"
        echo "[]" > "${OIP_OUTPUT}"
        exit 0
    }
fi

# Extract training data using OIP
echo "Running OIP extract-training-data..."
if "${OIP_BINARY}" extract-training-data \
    --repo "${REPRORUSTED_DIR}" \
    -o "${OIP_OUTPUT}" 2>/dev/null; then
    echo "OIP extraction successful"
else
    echo "OIP extraction failed or not supported, creating empty file"
    echo "[]" > "${OIP_OUTPUT}"
fi

# Display stats
if [ -f "${OIP_OUTPUT}" ]; then
    RECORDS=$(jq 'length' "${OIP_OUTPUT}" 2>/dev/null || echo "0")
    echo ""
    echo "=== OIP Extraction Complete ==="
    echo "Records extracted: ${RECORDS}"
    echo "Output: ${OIP_OUTPUT}"
fi
