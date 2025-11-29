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
echo "Refresh: watch -n10 $0"
