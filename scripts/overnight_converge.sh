#!/usr/bin/env bash
# overnight_converge.sh - Robust overnight CITL convergence runner
# shellcheck disable=SC2155
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEPYLER_DIR="$(dirname "$SCRIPT_DIR")"
CORPUS_DIR="${DEPYLER_CORPUS_DIR:-/home/noah/src/reprorusted-python-cli/examples}"
CHECKPOINT_DIR="${DEPYLER_DIR}/nightly_checkpoints"
LOG_DIR="${DEPYLER_DIR}/logs"
TIMESTAMP="${DEPYLER_TIMESTAMP:-$(date +%Y%m%d_%H%M%S)}"
LOG_FILE="${LOG_DIR}/converge_${TIMESTAMP}.log"
PID_FILE="${LOG_DIR}/converge.pid"

# Configuration
MAX_ITERATIONS="${DEPYLER_MAX_ITERATIONS:-1000}"
TARGET_RATE="${DEPYLER_TARGET_RATE:-100}"
FIX_CONFIDENCE="${DEPYLER_FIX_CONFIDENCE:-0.7}"
PARALLEL_JOBS="${DEPYLER_PARALLEL_JOBS:-$(nproc)}"
MAX_RETRIES="${DEPYLER_MAX_RETRIES:-3}"
RETRY_DELAY="${DEPYLER_RETRY_DELAY:-60}"

mkdir -p "$CHECKPOINT_DIR" "$LOG_DIR"

log() {
    local ts="${DEPYLER_LOG_TIMESTAMP:-$(date '+%Y-%m-%d %H:%M:%S')}"
    echo "[$ts] $*" | tee -a "$LOG_FILE"
}

cleanup() {
    log "Cleaning up..."
    rm -f "$PID_FILE"
}
trap cleanup EXIT

check_existing() {
    if [[ -f "$PID_FILE" ]]; then
        local old_pid
        old_pid=$(cat "$PID_FILE")
        if kill -0 "$old_pid" 2>/dev/null; then
            log "ERROR: Converge already running (PID: $old_pid)"
            exit 1
        fi
        rm -f "$PID_FILE"
    fi
}

build_depyler() {
    log "Building depyler in release mode..."
    cd "$DEPYLER_DIR"
    if ! cargo build --release --bin depyler 2>&1 | tee -a "$LOG_FILE"; then
        log "ERROR: Build failed"
        return 1
    fi
    log "Build complete"
}

run_converge() {
    local attempt=$1
    log "Starting convergence loop (attempt $attempt/$MAX_RETRIES)..."

    cd "$DEPYLER_DIR"
    ./target/release/depyler converge \
        -i "$CORPUS_DIR" \
        -t "$TARGET_RATE" \
        -m "$MAX_ITERATIONS" \
        --auto-fix \
        --fix-confidence "$FIX_CONFIDENCE" \
        --checkpoint-dir "$CHECKPOINT_DIR" \
        -p "$PARALLEL_JOBS" \
        -v 2>&1 | tee -a "$LOG_FILE"

    return "${PIPESTATUS[0]}"
}

summarize() {
    log "=== CONVERGENCE SUMMARY ==="
    log "Log file: $LOG_FILE"
    log "Checkpoints: $CHECKPOINT_DIR"

    # Extract final stats from log
    if grep -q "Rate:" "$LOG_FILE"; then
        local final_rate
        final_rate=$(grep "Rate:" "$LOG_FILE" | tail -1)
        log "Final: $final_rate"
    fi

    # Count iterations
    local iterations
    iterations=$(grep -c "Iteration" "$LOG_FILE" 2>/dev/null || echo "0")
    log "Total iterations: $iterations"

    log "==========================="
}

main() {
    log "=== OVERNIGHT CONVERGE STARTED ==="
    log "Corpus: $CORPUS_DIR"
    log "Max iterations: $MAX_ITERATIONS"
    log "Parallel jobs: $PARALLEL_JOBS"
    log "Fix confidence: $FIX_CONFIDENCE"

    check_existing
    echo $$ >"$PID_FILE"

    build_depyler || exit 1

    local attempt=1
    local success=false

    while [[ $attempt -le $MAX_RETRIES ]]; do
        if run_converge "$attempt"; then
            success=true
            break
        fi

        log "Attempt $attempt failed, retrying in ${RETRY_DELAY}s..."
        sleep "$RETRY_DELAY"
        ((attempt++)) || true
    done

    summarize

    if $success; then
        log "=== CONVERGE COMPLETED SUCCESSFULLY ==="
    else
        log "=== CONVERGE FAILED AFTER $MAX_RETRIES ATTEMPTS ==="
        exit 1
    fi
}

main "$@"
