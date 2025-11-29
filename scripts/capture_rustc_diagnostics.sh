#!/bin/bash
# Capture rustc JSON diagnostics for CITL training
# shellcheck disable=SC2094,SC2155
set -euo pipefail

readonly DIAG_FILE="${DEPYLER_DIAG_FILE:-/tmp/depyler_rustc_diagnostics.jsonl}"
readonly TRAINING_DIR="${DEPYLER_TRAINING_DIR:-/tmp/depyler_training_signals}"

mkdir -p "$TRAINING_DIR"

# Read hook input
INPUT="$(cat)"

# Extract command from hook input
CMD="$(printf '%s' "$INPUT" | jq -r '.tool_input.command // ""' 2>/dev/null || echo "")"

# Only process cargo build commands
if [[ "$CMD" == *"cargo build"* ]] || [[ "$CMD" == *"cargo check"* ]]; then
    # Extract manifest path if present
    MANIFEST=""
    if [[ "$CMD" == *"--manifest-path"* ]]; then
        MANIFEST="$(printf '%s' "$CMD" | grep -oP '(?<=--manifest-path )[^ ]+' || echo "")"
    fi

    # Get output from tool result
    OUTPUT="$(printf '%s' "$INPUT" | jq -r '.tool_output // ""' 2>/dev/null || echo "")"

    # If there are errors, capture structured diagnostics
    if printf '%s' "$OUTPUT" | grep -q "error\[E"; then
        TIMESTAMP="${DEPYLER_TIMESTAMP:-$(date -Iseconds)}"

        # Extract error codes and messages
        ERRORS="$(printf '%s' "$OUTPUT" | grep -oE 'error\[E[0-9]+\][^$]*' | head -10 || echo "")"

        # Write training signal
        jq -nc \
            --arg ts "$TIMESTAMP" \
            --arg manifest "$MANIFEST" \
            --arg cmd "$CMD" \
            --arg errors "$ERRORS" \
            '{timestamp: $ts, manifest: $manifest, command: $cmd, errors: $errors}' \
            >>"$DIAG_FILE"
    fi
fi
