#!/bin/bash
# DEPYLER-1318: Full Ambiguity Corpus Training Pipeline
#
# This script runs the complete training pipeline:
# 1. Generate hostile Python code (2,000 files)
# 2. Vectorize failures through depyler transpiler
# 3. Train Oracle model on failure vectors
#
# Usage:
#   ./scripts/train_ambiguity_pipeline.sh
#   ./scripts/train_ambiguity_pipeline.sh --count 5000  # More files
#   ./scripts/train_ambiguity_pipeline.sh --skip-generate  # Skip step 1

set -euo pipefail

# Configuration
CORPUS_DIR="training_corpus/ambiguity_v1"
VECTORS_FILE="training_corpus/ambiguity_vectors.ndjson"
MODEL_OUTPUT="${HOME}/.depyler/depyler_oracle_v3.23.apr"
FILE_COUNT=2000
SKIP_GENERATE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --count)
            FILE_COUNT="$2"
            shift 2
            ;;
        --skip-generate)
            SKIP_GENERATE=true
            shift
            ;;
        --output)
            MODEL_OUTPUT="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  DEPYLER-1318: Ambiguity Corpus Training Pipeline          ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Configuration:"
echo "  Corpus dir:   ${CORPUS_DIR}"
echo "  Vectors file: ${VECTORS_FILE}"
echo "  Model output: ${MODEL_OUTPUT}"
echo "  File count:   ${FILE_COUNT}"
echo ""

# Step 1: Generate corpus
if [ "$SKIP_GENERATE" = false ]; then
    echo "═══ Step 1/3: Generating Ambiguity Corpus ═══"
    python3 scripts/generate_ambiguity_corpus.py \
        --output "${CORPUS_DIR}" \
        --count "${FILE_COUNT}"
    echo ""
else
    echo "═══ Step 1/3: Skipping corpus generation ═══"
    if [ ! -d "${CORPUS_DIR}" ]; then
        echo "Error: Corpus directory not found: ${CORPUS_DIR}"
        exit 1
    fi
    echo "Using existing corpus: ${CORPUS_DIR}"
    echo ""
fi

# Step 2: Vectorize failures
echo "═══ Step 2/3: Vectorizing Failures ═══"
cargo run --release --bin depyler -- graph vectorize \
    --corpus "${CORPUS_DIR}" \
    --output "${VECTORS_FILE}"
echo ""

# Count vectors
VECTOR_COUNT=$(wc -l < "${VECTORS_FILE}" | tr -d ' ')
echo "Generated ${VECTOR_COUNT} failure vectors"
echo ""

# Check minimum threshold
if [ "${VECTOR_COUNT}" -lt 1500 ]; then
    echo "Warning: Only ${VECTOR_COUNT} vectors (< 1500 threshold)"
    echo "Consider increasing file count or adding more hostile patterns"
fi

# Step 3: Train Oracle
echo "═══ Step 3/3: Training Oracle ═══"
cargo run --release --example train_ambiguity_corpus -p depyler-oracle -- \
    --vectors "${VECTORS_FILE}" \
    --output "${MODEL_OUTPUT}" \
    --balance \
    --max-per-class 2500
echo ""

# Verify output
if [ -f "${MODEL_OUTPUT}" ]; then
    MODEL_SIZE=$(ls -lh "${MODEL_OUTPUT}" | awk '{print $5}')
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║  Pipeline Complete                                         ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    echo "Model saved: ${MODEL_OUTPUT} (${MODEL_SIZE})"
    echo "Vectors:     ${VECTOR_COUNT}"
    echo ""
    echo "To use this model, copy to ~/.depyler/depyler_oracle.apr"
else
    echo "Error: Model file not created"
    exit 1
fi
