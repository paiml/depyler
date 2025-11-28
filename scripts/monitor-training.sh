#!/bin/bash
# Real-time training monitor for depyler oracle improve
# Usage: ./monitor-training.sh [monitor.json path]

MONITOR_FILE="${1:-/home/noah/src/reprorusted-python-cli/examples/.depyler-improve/monitor.json}"

echo "Monitoring: $MONITOR_FILE"
echo "Press Ctrl+C to stop"
echo ""

while true; do
    if [[ -f "$MONITOR_FILE" ]]; then
        # Check if file was modified in last 2 minutes
        if [[ $(find "$MONITOR_FILE" -mmin -2 2>/dev/null) ]]; then
            clear
            echo "═══════════════════════════════════════════════════════════"
            echo "         DEPYLER TRAINING MONITOR (Live)"
            echo "═══════════════════════════════════════════════════════════"
            echo ""

            # Parse and display metrics
            jq -r '
                "Epoch:      \(.epoch)/\(.max_epochs)",
                "Transpile:  \(.transpile_ok)/\(.total_files) (\(.transpile_ok/.total_files*100 | floor)%)",
                "Compile:    \(.compile_ok)/\(.total_files) (\(.compile_rate*100 | floor)%)",
                "Target:     \(.target_rate*100 | floor)%",
                "Delta:      \(.delta)",
                "No Progress: \(.no_progress_count)/3",
                "",
                "─── Top Errors ───────────────────────────────────────────"
            ' "$MONITOR_FILE" 2>/dev/null

            # Show top 10 error codes
            jq -r '.error_distribution | to_entries | sort_by(-.value) | .[0:10] | .[] | "  \(.key | split(":")[3] // .key): \(.value)"' "$MONITOR_FILE" 2>/dev/null

            echo ""
            echo "───────────────────────────────────────────────────────────"
            echo "Updated: $(stat -c %y "$MONITOR_FILE" | cut -d. -f1)"
        else
            echo -ne "\rWaiting for updates... (last update > 2min ago)"
        fi
    else
        echo -ne "\rWaiting for job to start..."
    fi
    sleep 5
done
