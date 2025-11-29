#!/bin/bash
# Renacer Trace Wrapper for Overnight Sessions
# Captures syscall traces for performance analysis
# shellcheck disable=SC2032,SC2155
set -euo pipefail

readonly TRACE_DIR="${DEPYLER_TRACE_DIR:-/tmp/depyler_traces}"
readonly DEPYLER_BIN="${DEPYLER_BIN:-/home/noah/src/depyler/target/release/depyler}"

mkdir -p "$TRACE_DIR"

# Trace a single transpilation with renacer
trace_transpile() {
    local py_file="$1"
    local basename
    basename="$(basename "$py_file" .py)"
    local ts="${DEPYLER_TRACE_TS:-$(date +%s)}"
    local trace_file="$TRACE_DIR/${basename}_${ts}.trace"

    # Run with renacer timing and summary
    renacer -T -c -- "$DEPYLER_BIN" transpile "$py_file" 2>&1 | tee "$trace_file"
}

# Trace compilation of generated Rust
trace_compile() {
    local rs_file="$1"
    local basename
    basename="$(basename "$rs_file" .rs)"
    local ts="${DEPYLER_TRACE_TS:-$(date +%s)}"
    local trace_file="$TRACE_DIR/${basename}_compile_${ts}.trace"

    # Get parent dir for cargo
    local dir
    dir="$(dirname "$rs_file")"

    if [ -f "$dir/Cargo.toml" ]; then
        renacer -T -c -- cargo build --manifest-path "$dir/Cargo.toml" 2>&1 | tee "$trace_file"
    fi
}

# Summarize traces
summarize() {
    echo "=== Trace Summary ==="
    echo "Traces collected: $(find "$TRACE_DIR" -name "*.trace" 2>/dev/null | wc -l)"

    if [ -d "$TRACE_DIR" ]; then
        echo "Recent traces:"
        # shellcheck disable=SC2012
        ls -lt "$TRACE_DIR"/*.trace 2>/dev/null | head -5 || echo "  No traces found"
    fi
}

# Command dispatch
case "${1:-help}" in
    transpile)
        shift
        trace_transpile "$@"
        ;;
    compile)
        shift
        trace_compile "$@"
        ;;
    summary)
        summarize
        ;;
    *)
        echo "Usage: $0 {transpile|compile|summary} [args...]"
        exit 1
        ;;
esac
