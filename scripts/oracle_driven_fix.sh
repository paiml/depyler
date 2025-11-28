#!/bin/bash
# Oracle-Driven Transpilation Fix Loop
# Runs until 100% compilation or no progress is made

set -e

DEPYLER=/home/noah/src/depyler/target/release/depyler
EXAMPLES_DIR=/home/noah/src/reprorusted-python-cli/examples
MAX_ITERATIONS=20
ERROR_CORPUS=/tmp/error_corpus.jsonl

echo "=== Oracle-Driven Transpilation Fix Loop ==="
echo "Target: 100% compilation of $EXAMPLES_DIR"
echo ""

# Function to transpile all examples
transpile_all() {
    echo "Transpiling all examples..."
    for dir in "$EXAMPLES_DIR"/*/; do
        py_file=$(find "$dir" -maxdepth 1 -name "*_tool.py" -o -name "*_cli.py" -o -name "*_flat.py" -o -name "*_simple.py" 2>/dev/null | grep -v "test_" | head -1)
        if [ -z "$py_file" ]; then
            py_file=$(find "$dir" -maxdepth 1 -name "*.py" ! -name "test_*" 2>/dev/null | head -1)
        fi
        if [ -n "$py_file" ]; then
            $DEPYLER transpile "$py_file" -o "$dir/src/main.rs" 2>/dev/null || true
            if [ -f "$dir/src/Cargo.toml" ]; then
                cp "$dir/src/Cargo.toml" "$dir/Cargo.toml"
                sed -i 's|path = "main.rs"|path = "src/main.rs"|' "$dir/Cargo.toml"
            fi
        fi
    done
}

# Function to count compilation results and collect errors
count_and_collect() {
    local total=0
    local pass=0
    rm -f "$ERROR_CORPUS"

    for dir in "$EXAMPLES_DIR"/*/; do
        if [ -f "$dir/Cargo.toml" ]; then
            total=$((total + 1))
            errors=$(cargo check --manifest-path "$dir/Cargo.toml" 2>&1)
            if [ $? -eq 0 ]; then
                pass=$((pass + 1))
            else
                # Extract error codes and messages for oracle
                echo "$errors" | grep -E "^error\[E[0-9]+\]" | while read -r line; do
                    example=$(basename "$dir")
                    echo "{\"example\":\"$example\",\"error\":\"$line\"}" >> "$ERROR_CORPUS"
                done
            fi
        fi
    done

    echo "$pass/$total"
}

# Function to categorize errors
categorize_errors() {
    if [ -f "$ERROR_CORPUS" ]; then
        echo "Error categories:"
        grep -oP 'error\[E[0-9]+\]' "$ERROR_CORPUS" 2>/dev/null | sort | uniq -c | sort -rn | head -10
    fi
}

# Main loop
prev_pass=0
for i in $(seq 1 $MAX_ITERATIONS); do
    echo ""
    echo "=== Iteration $i ==="

    # Step 1: Transpile
    transpile_all

    # Step 2: Count and collect errors
    result=$(count_and_collect)
    pass=$(echo "$result" | cut -d'/' -f1)
    total=$(echo "$result" | cut -d'/' -f2)
    rate=$((pass * 100 / total))

    echo "Compilation: $pass/$total ($rate%)"

    # Step 3: Check for 100%
    if [ "$pass" -eq "$total" ]; then
        echo ""
        echo "🎉 SUCCESS! 100% compilation achieved!"
        exit 0
    fi

    # Step 4: Check for progress
    if [ "$pass" -eq "$prev_pass" ] && [ $i -gt 1 ]; then
        echo "No progress made. Stopping."
        echo ""
        categorize_errors
        exit 1
    fi
    prev_pass=$pass

    # Step 5: Categorize errors for oracle
    categorize_errors

    # Step 6: Train oracle on new errors (if corpus exists)
    if [ -f "$ERROR_CORPUS" ] && [ -s "$ERROR_CORPUS" ]; then
        echo "Training oracle on $(wc -l < "$ERROR_CORPUS") errors..."
        # $DEPYLER oracle train --corpus "$ERROR_CORPUS" 2>/dev/null || true
    fi

    echo "Waiting for manual fixes or oracle suggestions..."
    echo "(In the future, oracle will auto-apply fixes)"

    # For now, exit after showing current state
    # In production, the oracle would suggest fixes here
    break
done

echo ""
echo "Final compilation rate: $pass/$total ($rate%)"
