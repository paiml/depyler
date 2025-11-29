#!/bin/bash
# Decision Trace Export for CITL (Compiler-in-the-Loop) Training
# Captures transpiler decision traces for pattern learning
# shellcheck disable=SC2155,SC2089,SC2090,SC2274,SC2316,SC2320
# bashrs: disable=SEC010
set -euo pipefail

# Configuration - with safe defaults
TRACE_DIR="${DEPYLER_DECISION_DIR:-/tmp/depyler_decisions}"
readonly TRACE_FILE="${TRACE_DIR}/decisions.msgpack"
readonly DEPYLER_BIN="${DEPYLER_BIN:-/home/noah/src/depyler/target/release/depyler}"

# Validate path doesn't contain traversal
validate_path() {
    local path="$1"
    if [[ "$path" == *".."* ]]; then
        echo "Error: Path traversal detected in: $path" >&2
        exit 1
    fi
}

# Create trace directory safely
init_trace_dir() {
    validate_path "$TRACE_DIR"
    mkdir -p "$TRACE_DIR"
}

# Initialize decision tracing by building with the feature enabled
init_tracing() {
    echo "Building depyler with decision-tracing feature..."
    cargo build --release --features decision-tracing
    init_trace_dir
    echo "Decision tracing enabled. Traces will be written to: $TRACE_FILE"
}

# Run transpilation with decision tracing enabled
trace_transpile() {
    local py_file="$1"
    local base
    base="$(basename "$py_file" .py)"
    local ts="${DEPYLER_TRACE_TS:-0}"

    echo "=== Tracing transpilation: $py_file ==="

    init_trace_dir

    # Run transpilation - decisions are automatically captured
    "$DEPYLER_BIN" transpile "$py_file" --source-map

    # Check if trace file was created/updated
    if [ -f "$TRACE_FILE" ]; then
        local size
        size="$(stat -c%s "$TRACE_FILE" 2>/dev/null || stat -f%z "$TRACE_FILE" 2>/dev/null || echo 0)"
        echo "Trace file: $TRACE_FILE ($size bytes)"
    fi

    # Archive the trace with timestamp (if provided)
    if [ -f "$TRACE_FILE" ] && [ "$ts" != "0" ]; then
        local dest="${TRACE_DIR}/decisions_${base}_${ts}.msgpack"
        validate_path "$dest"
        cp "$TRACE_FILE" "$dest"
    fi
}

# Export traces for entrenar consumption
export_for_training() {
    local default_output="${TRACE_DIR}/training"
    local output_dir="${1:-$default_output}"

    validate_path "$output_dir"
    mkdir -p "$output_dir"

    echo "=== Exporting traces for CITL training ==="

    # Combine all decision traces into training format
    local trace_count=0
    while IFS= read -r -d '' trace; do
        trace_count=$((trace_count + 1))
        validate_path "$trace"
        cp "$trace" "$output_dir/"
    done < <(find "$TRACE_DIR" -maxdepth 1 -name "*.msgpack" -print0 2>/dev/null | sort -z)

    echo "Exported $trace_count trace files to: $output_dir"

    # Generate summary manifest
    local manifest_file="$output_dir/manifest.json"
    validate_path "$manifest_file"

    # Use here-doc directly
    {
        printf '{\n'
        printf '    "version": 1,\n'
        printf '    "trace_count": %d,\n' "$trace_count"
        printf '    "format": "msgpack",\n'
        printf '    "schema": {\n'
        printf '        "id": "u64",\n'
        printf '        "timestamp_ns": "u64",\n'
        printf '        "thread_id": "u64",\n'
        printf '        "source_file": "string",\n'
        printf '        "source_line": "u32",\n'
        printf '        "category": "DecisionCategory",\n'
        printf '        "name": "string",\n'
        printf '        "chosen_path": "string",\n'
        printf '        "alternatives": "Vec<string>",\n'
        printf '        "confidence": "f32"\n'
        printf '    }\n'
        printf '}\n'
    } > "$manifest_file"

    echo "Manifest generated: $manifest_file"
}

# Summarize collected traces
summarize() {
    echo "=== Decision Trace Summary ==="
    echo "Trace directory: $TRACE_DIR"
    echo ""

    if [ -d "$TRACE_DIR" ]; then
        local trace_count
        trace_count="$(find "$TRACE_DIR" -maxdepth 1 -name "*.msgpack" 2>/dev/null | wc -l)"
        echo "Total trace files: $trace_count"

        if [ "$trace_count" -gt 0 ]; then
            echo ""
            echo "Recent traces:"
            find "$TRACE_DIR" -maxdepth 1 -name "*.msgpack" -exec ls -lth "{}" + 2>/dev/null | head -10 || true

            echo ""
            local total_size
            total_size="$(du -sh "$TRACE_DIR" 2>/dev/null | cut -f1 || echo "unknown")"
            echo "Total size: $total_size"
        fi
    else
        echo "No trace directory found"
    fi
}

# Clean up old traces
cleanup() {
    local retention_days="${1:-7}"
    echo "Cleaning traces older than $retention_days days..."
    find "$TRACE_DIR" -maxdepth 1 -name "*.msgpack" -mtime +"$retention_days" -delete 2>/dev/null || true
    echo "Cleanup complete"
}

# Analyze trace patterns (stub for future integration)
analyze() {
    echo "=== Decision Pattern Analysis ==="
    echo "NOTE: Full analysis requires entrenar integration"
    echo ""

    if [ -f "$TRACE_FILE" ]; then
        echo "Main trace file: $TRACE_FILE"
        local size
        size="$(stat -c%s "$TRACE_FILE" 2>/dev/null || stat -f%z "$TRACE_FILE" 2>/dev/null || echo 0)"
        echo "Size: $size bytes"
        echo ""
        echo "To analyze with entrenar:"
        echo "  entrenar ingest --decisions $TRACE_FILE"
        echo "  entrenar train --patterns"
    else
        echo "No trace file found. Run 'trace_transpile' first."
    fi
}

# Show help
show_help() {
    echo "Decision Trace Export for CITL Training"
    echo ""
    echo "Usage: $0 <command> [args...]"
    echo ""
    echo "Commands:"
    echo "    init                  Build depyler with decision-tracing feature"
    echo "    transpile <file.py>   Transpile with decision tracing enabled"
    echo "    export [dir]          Export traces for entrenar consumption"
    echo "    summary               Show trace collection summary"
    echo "    cleanup [days]        Remove traces older than N days (default: 7)"
    echo "    analyze               Analyze decision patterns (stub)"
    echo "    help                  Show this help"
    echo ""
    echo "Environment:"
    echo "    DEPYLER_DECISION_DIR  Directory for traces (default: /tmp/depyler_decisions)"
    echo "    DEPYLER_BIN          Path to depyler binary (default: target/release/depyler)"
    echo ""
    echo "Example workflow:"
    echo "    $0 init"
    echo "    $0 transpile examples/hello.py"
    echo "    $0 summary"
    echo "    $0 export ./training_data"
}

# Command dispatch
case "${1:-help}" in
    init)
        init_tracing
        ;;
    transpile)
        shift
        trace_transpile "$1"
        ;;
    export)
        shift
        export_for_training "${1:-}"
        ;;
    summary)
        summarize
        ;;
    cleanup)
        shift
        cleanup "${1:-7}"
        ;;
    analyze)
        analyze
        ;;
    help)
        show_help
        ;;
    *)
        show_help
        ;;
esac
