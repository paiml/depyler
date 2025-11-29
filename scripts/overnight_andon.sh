#!/bin/bash
# Andon Alert System for Overnight Sessions
# Implements Toyota Way 自働化 (Jidoka) - stop on defects
# shellcheck disable=SC2032,SC2155
set -euo pipefail

readonly ALERT_FILE="${DEPYLER_ALERT_FILE:-/tmp/depyler_andon_alerts.jsonl}"
readonly STATUS_FILE="${DEPYLER_STATUS_FILE:-/tmp/depyler_overnight_status.json}"

# Alert levels
alert() {
    local level="$1"
    local message="$2"
    local timestamp="${DEPYLER_TIMESTAMP:-$(date -Iseconds)}"

    local emoji=""
    case "$level" in
        info)     emoji="ℹ️" ;;
        warning)  emoji="⚠️" ;;
        error)    emoji="❌" ;;
        critical) emoji="🛑" ;;
        *)        emoji="?" ;;
    esac

    # Write to JSONL log
    printf '{"timestamp":"%s","level":"%s","message":"%s"}\n' \
        "$timestamp" "$level" "$message" >>"$ALERT_FILE"

    # Also write to stderr for visibility
    printf '%s [%s] %s: %s\n' "$emoji" "$timestamp" "$level" "$message" >&2
}

# Check for stalled progress (no commits in 30 min)
check_stall() {
    if [ ! -f "$STATUS_FILE" ]; then
        return 0
    fi

    local commits
    commits="$(jq -r '.commits_since_start // "0"' "$STATUS_FILE" 2>/dev/null || echo "0")"

    if [ "$commits" = "0" ]; then
        local file_age
        file_age="$(stat -c %Y "$STATUS_FILE" 2>/dev/null || echo "0")"
        local now="${DEPYLER_NOW:-$(date +%s)}"
        local age
        age=$((now - file_age))

        if [ "$age" -gt 1800 ]; then
            alert "warning" "No commits in 30+ minutes - session may be stuck"
        fi
    fi
}

# Check test status
check_tests() {
    if [ ! -f "$STATUS_FILE" ]; then
        return 0
    fi

    local test_status
    test_status="$(jq -r '.test_status // "unknown"' "$STATUS_FILE" 2>/dev/null || echo "unknown")"

    if [[ "$test_status" == *"FAILED"* ]]; then
        alert "error" "Tests are failing: $test_status"
    fi
}

# Check for clippy warnings
check_clippy() {
    local clippy_out
    local depyler_dir="${DEPYLER_DIR:-/home/noah/src/depyler}"

    if clippy_out="$(cd "$depyler_dir" && cargo clippy --package depyler-core -- -D warnings 2>&1)"; then
        alert "info" "Clippy clean"
    else
        local warn_count
        warn_count="$(printf '%s' "$clippy_out" | grep -c "warning:" || echo "0")"
        if [ "$warn_count" -gt 0 ]; then
            alert "warning" "Clippy has $warn_count warnings"
        fi
    fi
}

# Main monitoring loop
main() {
    alert "info" "Andon monitor started"

    while true; do
        check_stall
        check_tests
        sleep 300  # Check every 5 minutes
    done
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
