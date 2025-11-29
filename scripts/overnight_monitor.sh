#!/bin/bash
# Overnight Session Monitor for Depyler
# Writes status to /tmp/depyler_overnight_status.json
# shellcheck disable=SC2032,SC2155
set -euo pipefail

# Use fixed paths (intentional, not user input)
readonly STATUS_FILE="${DEPYLER_STATUS_FILE:-/tmp/depyler_overnight_status.json}"
readonly LOG_FILE="${DEPYLER_LOG_FILE:-/tmp/depyler_overnight.log}"
readonly TEST_CACHE="${DEPYLER_TEST_CACHE:-/tmp/depyler_test_status}"
readonly DEPYLER_DIR="${DEPYLER_DIR:-/home/noah/src/depyler}"

# Read JSON from stdin
INPUT="$(cat)"

# Extract fields using jq (single quotes for jq filters are correct)
TOOL_NAME="$(printf '%s' "$INPUT" | jq -r '.tool_name // "unknown"')"
HOOK_EVENT="$(printf '%s' "$INPUT" | jq -r '.hook_event_name // "unknown"')"
SESSION_ID="$(printf '%s' "$INPUT" | jq -r '.session_id // "unknown"')"
TIMESTAMP="${DEPYLER_TIMESTAMP:-$(date -Iseconds)}"

# Count commits (using if-then-else for proper error handling)
COMMIT_COUNT="0"
if cd "${DEPYLER_DIR}" 2>/dev/null; then
    COMMIT_COUNT="$(git rev-list --count HEAD ^db41c45a 2>/dev/null || echo "0")"
fi

# Get latest commit message
LATEST_COMMIT="none"
if cd "${DEPYLER_DIR}" 2>/dev/null; then
    LATEST_COMMIT="$(git log -1 --oneline 2>/dev/null || echo "none")"
fi

# Count modified files
MODIFIED_FILES="0"
if cd "${DEPYLER_DIR}" 2>/dev/null; then
    MODIFIED_FILES="$(git diff --name-only HEAD 2>/dev/null | wc -l || echo "0")"
fi

# Check if tests pass (cached, updated every 5 min)
TEST_STATUS="unknown"
if [ -f "$TEST_CACHE" ]; then
    CURRENT_TIME="${DEPYLER_CURRENT_TIME:-$(date +%s)}"
    CACHE_MTIME="$(stat -c %Y "$TEST_CACHE" 2>/dev/null || echo "0")"
    CACHE_AGE=$((CURRENT_TIME - CACHE_MTIME))
    if [ "$CACHE_AGE" -lt 300 ]; then
        TEST_STATUS="$(cat "$TEST_CACHE")"
    fi
fi

# Write status JSON
cat >"$STATUS_FILE" <<EOF
{
  "timestamp": "$TIMESTAMP",
  "session_id": "$SESSION_ID",
  "last_event": "$HOOK_EVENT",
  "last_tool": "$TOOL_NAME",
  "commits_since_start": "$COMMIT_COUNT",
  "latest_commit": "$LATEST_COMMIT",
  "modified_files": "$MODIFIED_FILES",
  "test_status": "$TEST_STATUS"
}
EOF

# Append to log
printf '[%s] %s: %s\n' "$TIMESTAMP" "$HOOK_EVENT" "$TOOL_NAME" >>"$LOG_FILE"

# Alert on commits
if [[ "$TOOL_NAME" == "Bash" ]]; then
    COMMAND="$(printf '%s' "$INPUT" | jq -r '.tool_input.command // ""')"
    if [[ "$COMMAND" =~ git\ commit ]]; then
        printf '[%s] COMMIT DETECTED\n' "$TIMESTAMP" >>/tmp/depyler_commits.log
    fi
fi

exit 0
