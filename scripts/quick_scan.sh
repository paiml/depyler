#!/bin/bash
# Quick corpus scan for failures

pass_count=0
fail_count=0
transpile_fail=0
errors_by_type=""

for dir in ../reprorusted-python-cli/examples/example_*/; do
  name=$(basename "$dir")
  # Find main Python file (not test files)
  py_file=$(find "$dir" -maxdepth 1 -name "*.py" ! -name "test_*.py" ! -name "*_test.py" 2>/dev/null | head -1)
  if [ -n "$py_file" ]; then
    result=$(./target/release/depyler compile "$py_file" 2>&1)
    if echo "$result" | grep -q "Binary created"; then
      echo "PASS: $name"
      ((pass_count++))
    elif echo "$result" | grep -q "error\[E"; then
      error=$(echo "$result" | grep -oE "error\[E[0-9]+\]" | head -1)
      echo "FAIL: $name - $error"
      errors_by_type="$errors_by_type $error"
      ((fail_count++))
    elif echo "$result" | grep -qiE "transpilation.*fail|parse.*error|unsupported|Unsupported"; then
      hint=$(echo "$result" | grep -iE "unsupported|error" | head -1 | cut -c1-60)
      echo "FAIL: $name - TRANSPILE: $hint"
      ((transpile_fail++))
    else
      # Check for any error indication
      if echo "$result" | grep -qi "error"; then
        error_hint=$(echo "$result" | grep -i "error" | head -1 | cut -c1-60)
        echo "FAIL: $name - $error_hint"
        ((fail_count++))
      else
        echo "????: $name"
        # Debug: show first 3 lines
        echo "  --> $(echo "$result" | head -3)"
      fi
    fi
  fi
done

total=$((pass_count + fail_count + transpile_fail))
pass_rate=$(echo "scale=1; $pass_count * 100 / $total" | bc)

echo ""
echo "=== Summary ==="
echo "PASS: $pass_count"
echo "FAIL (cargo): $fail_count"
echo "FAIL (transpile): $transpile_fail"
echo "Total: $total"
echo "Pass rate: ${pass_rate}%"
echo ""
echo "=== Error Types ==="
echo "$errors_by_type" | tr ' ' '\n' | sort | uniq -c | sort -rn | head -10
