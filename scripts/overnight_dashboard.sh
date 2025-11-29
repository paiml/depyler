#!/bin/bash
# shellcheck disable=SC2046,SC2155
# Overnight Session Dashboard
# Combines monitor, andon, and renacer outputs
set -euo pipefail

readonly STATUS_FILE="${DEPYLER_STATUS_FILE:-/tmp/depyler_overnight_status.json}"
readonly ALERT_FILE="${DEPYLER_ALERT_FILE:-/tmp/depyler_andon_alerts.jsonl}"
readonly LOG_FILE="${DEPYLER_LOG_FILE:-/tmp/depyler_overnight.log}"
readonly COMMIT_FILE="${DEPYLER_COMMIT_FILE:-/tmp/depyler_commits.log}"
readonly DEPYLER_DIR="${DEPYLER_DIR:-/home/noah/src/depyler}"

clear
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           DEPYLER OVERNIGHT SESSION DASHBOARD                ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Status section
echo "┌─ STATUS ─────────────────────────────────────────────────────┐"
if [ -f "$STATUS_FILE" ]; then
    jq -r '"│ Last Update:  \(.timestamp)"' "$STATUS_FILE" 2>/dev/null || echo "│ Status: Unknown"
    jq -r '"│ Session:      \(.session_id)"' "$STATUS_FILE" 2>/dev/null || true
    jq -r '"│ Last Tool:    \(.last_tool)"' "$STATUS_FILE" 2>/dev/null || true
    jq -r '"│ Commits:      \(.commits_since_start)"' "$STATUS_FILE" 2>/dev/null || true
    jq -r '"│ Modified:     \(.modified_files) files"' "$STATUS_FILE" 2>/dev/null || true
else
    echo "│ No status file found - session may not have started"
fi
echo "└──────────────────────────────────────────────────────────────┘"
echo ""

# Git section
echo "┌─ GIT ────────────────────────────────────────────────────────┐"
if cd "$DEPYLER_DIR" 2>/dev/null; then
    echo "│ Branch: $(git branch --show-current)"
    echo "│ Latest: $(git log -1 --oneline 2>/dev/null || echo 'none')"
    echo "│ Uncommitted: $(git diff --stat HEAD 2>/dev/null | tail -1 || echo '0')"
fi
echo "└──────────────────────────────────────────────────────────────┘"
echo ""

# Alerts section
echo "┌─ ANDON ALERTS ──────────────────────────────────────────────┐"
if [ -f "$ALERT_FILE" ]; then
    tail -5 "$ALERT_FILE" | while read -r line; do
        level=$(printf '%s' "$line" | jq -r '.level // "info"')
        msg=$(printf '%s' "$line" | jq -r '.message // ""')
        case "$level" in
            error|critical) printf "│ ❌ %s\n" "$msg" ;;
            warning)        printf "│ ⚠️  %s\n" "$msg" ;;
            *)              printf "│ ℹ️  %s\n" "$msg" ;;
        esac
    done
else
    echo "│ No alerts"
fi
echo "└──────────────────────────────────────────────────────────────┘"
echo ""

# Recent activity
echo "┌─ RECENT ACTIVITY ────────────────────────────────────────────┐"
if [ -f "$LOG_FILE" ]; then
    tail -5 "$LOG_FILE" | sed 's/^/│ /'
else
    echo "│ No activity log"
fi
echo "└──────────────────────────────────────────────────────────────┘"
echo ""

# Commits
echo "┌─ COMMITS ───────────────────────────────────────────────────┐"
if [ -f "$COMMIT_FILE" ]; then
    wc -l <"$COMMIT_FILE" | xargs printf "│ Total commits detected: %s\n"
    tail -3 "$COMMIT_FILE" | sed 's/^/│ /'
else
    echo "│ No commits yet"
fi
echo "└──────────────────────────────────────────────────────────────┘"
echo ""

# Decision Traces (CITL Integration - Spec Section 6.2)
readonly TRACE_FILE="${DEPYLER_DECISION_FILE:-/tmp/depyler_decisions.msgpack}"
readonly TRACE_JSONL="${TRACE_FILE%.msgpack}.jsonl"

echo "┌─ DECISION TRACES ─────────────────────────────────────────────┐"
if [ -f "$TRACE_FILE" ]; then
    # Get file stats
    trace_size=$(stat -c%s "$TRACE_FILE" 2>/dev/null || stat -f%z "$TRACE_FILE" 2>/dev/null || echo "0")
    trace_mtime=$(stat -c%Y "$TRACE_FILE" 2>/dev/null || stat -f%m "$TRACE_FILE" 2>/dev/null || echo "0")
    trace_age=$(($(date +%s) - trace_mtime))

    printf "│ Trace file: %s\n" "$TRACE_FILE"
    printf "│ Size: %s bytes\n" "$trace_size"
    printf "│ Age: %ds ago\n" "$trace_age"

    # Check for renacer stats (if available)
    if command -v renacer >/dev/null 2>&1; then
        trace_count=$(renacer stats "$TRACE_FILE" 2>/dev/null | grep -E "^count:" | awk '{print $2}' || echo "unknown")
        printf "│ Decisions captured: %s\n" "$trace_count"
    fi
elif [ -f "$TRACE_JSONL" ]; then
    # Fallback: count lines in JSONL file
    jsonl_count=$(wc -l < "$TRACE_JSONL" 2>/dev/null || echo "0")
    printf "│ Trace file (JSONL fallback): %s\n" "$TRACE_JSONL"
    printf "│ Decisions captured: %s\n" "$jsonl_count"

    # Show category distribution from JSONL
    if [ "$jsonl_count" -gt 0 ]; then
        echo "│ Category distribution:"
        jq -r '.category' "$TRACE_JSONL" 2>/dev/null | sort | uniq -c | sort -rn | head -5 | while read -r count cat; do
            printf "│   %s: %s\n" "$cat" "$count"
        done || echo "│   (unable to parse)"
    fi
else
    echo "│ No decision traces"
    echo "│ Enable with: cargo build --features decision-tracing"
fi
echo "└──────────────────────────────────────────────────────────────┘"

echo ""
echo "Refresh: watch -n10 $0"
