#!/bin/bash
# Corpus Analysis Report - Just Fucking Works Edition
# Usage: ./scripts/corpus-report.sh [corpus_path]

set -euo pipefail

CORPUS="${1:-/home/noah/src/reprorusted-python-cli/examples}"
REPORT_FILE="/tmp/corpus-report-$(date +%Y%m%d-%H%M%S).md"

echo "=== DEPYLER CORPUS ANALYSIS ==="
echo "Corpus: $CORPUS"
echo "Report: $REPORT_FILE"
echo ""

# Initialize counters
PASS=0
FAIL=0
TOTAL=0
declare -A ERRORS
declare -A SAMPLES

# Find all Cargo.toml directories (transpiled projects)
DIRS=$(find "$CORPUS" -name "Cargo.toml" -exec dirname "{}" \; 2>/dev/null | sort -u)

echo "Analyzing $(echo "$DIRS" | wc -l) transpiled projects..."
echo ""

for dir in $DIRS; do
    ((TOTAL++))
    name=$(basename "$dir")

    # Run cargo build
    if output=$(cd "$dir" && cargo build --release 2>&1); then
        ((PASS++))
        echo -ne "\r[$PASS pass / $FAIL fail] $name                    "
    else
        ((FAIL++))
        # Extract first error code
        errcode=$(echo "$output" | grep -oP 'error\[E\d+\]' | head -1 | grep -oP 'E\d+' || echo "UNKNOWN")
        ((ERRORS[$errcode]++)) || ERRORS[$errcode]=1

        # Store sample if first occurrence
        if [[ -z "${SAMPLES[$errcode]:-}" ]]; then
            errmsg=$(echo "$output" | grep -A1 "error\[E" | head -2 | tr '\n' ' ')
            SAMPLES[$errcode]="$name: $errmsg"
        fi

        echo -ne "\r[$PASS pass / $FAIL fail] $name                    "
    fi
done

echo ""
echo ""

# Calculate rate
if [ $TOTAL -gt 0 ]; then
    RATE=$(echo "scale=1; $PASS * 100 / $TOTAL" | bc)
else
    RATE="0.0"
fi

# Generate report
cat > "$REPORT_FILE" << EOF
# Depyler Corpus Analysis Report

**Generated**: $(date -Iseconds)
**Corpus**: $CORPUS

## Executive Summary

| Metric | Value |
|--------|-------|
| Total Projects | $TOTAL |
| Compiles (PASS) | $PASS |
| Fails | $FAIL |
| **Single-Shot Rate** | **${RATE}%** |

## Andon Status

EOF

if (( $(echo "$RATE >= 80" | bc -l) )); then
    echo "**GREEN** - Target met (>= 80%)" >> "$REPORT_FILE"
elif (( $(echo "$RATE >= 50" | bc -l) )); then
    echo "**YELLOW** - Below target (50-80%)" >> "$REPORT_FILE"
else
    echo "**RED** - Critical (< 50%)" >> "$REPORT_FILE"
fi

cat >> "$REPORT_FILE" << EOF

## Error Taxonomy (Prioritized Blockers)

| Priority | Error | Count | Impact | Description |
|----------|-------|-------|--------|-------------|
EOF

# Sort errors by count and add descriptions
for code in "${!ERRORS[@]}"; do
    echo "$code ${ERRORS[$code]}"
done | sort -k2 -rn | while read code count; do
    # Calculate impact percentage
    impact=$(echo "scale=1; $count * 100 / $FAIL" | bc 2>/dev/null || echo "0")

    # Error descriptions
    case $code in
        E0425) desc="Cannot find value in scope (undefined variable/function)" ;;
        E0412) desc="Cannot find type in scope (missing generic/type)" ;;
        E0308) desc="Mismatched types (type inference failure)" ;;
        E0277) desc="Trait not implemented (missing impl)" ;;
        E0432) desc="Unresolved import (missing crate)" ;;
        E0599) desc="Method not found (wrong type/missing impl)" ;;
        E0433) desc="Failed to resolve (unresolved module)" ;;
        E0423) desc="Expected value, found type" ;;
        E0369) desc="Binary operation not supported" ;;
        *) desc="See rustc --explain $code" ;;
    esac

    # Priority based on count
    if [ "$count" -ge 20 ]; then
        prio="P0-CRITICAL"
    elif [ "$count" -ge 10 ]; then
        prio="P1-HIGH"
    elif [ "$count" -ge 5 ]; then
        prio="P2-MEDIUM"
    else
        prio="P3-LOW"
    fi

    echo "| $prio | $code | $count | ${impact}% | $desc |"
done >> "$REPORT_FILE"

cat >> "$REPORT_FILE" << EOF

## Actionable Fix Items

### P0 Critical (Fix This Week)

EOF

# Top 3 errors with samples
i=0
for code in "${!ERRORS[@]}"; do
    echo "$code ${ERRORS[$code]}"
done | sort -k2 -rn | head -3 | while read code count; do
    ((i++))
    sample="${SAMPLES[$code]:-No sample}"

    cat >> "$REPORT_FILE" << EOF
#### $i. Fix $code ($count occurrences)

**Sample**: \`$sample\`

**Root Cause**:
EOF

    case $code in
        E0425)
            cat >> "$REPORT_FILE" << 'EOF'
Variables/functions referenced but not defined in Rust scope.
- `asyncio` -> needs tokio runtime mapping
- `as_of` -> date parameter not propagated
- Loop variables not in scope

**Action**: Update `codegen.rs` to properly declare variables before use.
EOF
            ;;
        E0412)
            cat >> "$REPORT_FILE" << 'EOF'
Generic type parameters `T` used without declaration.
- Python duck typing -> Rust needs explicit generics
- `UnionType` not mapped to Rust enum

**Action**: Add generic parameter detection in `type_inference.rs`.
EOF
            ;;
        E0308)
            cat >> "$REPORT_FILE" << 'EOF'
Type coercion failures between i32/i64, String/&str.
- Python int -> needs consistent Rust int type
- String literals vs owned strings

**Action**: Standardize numeric types in `rust_type_mapper.rs`.
EOF
            ;;
        *)
            echo "See \`rustc --explain $code\` for details." >> "$REPORT_FILE"
            ;;
    esac

    echo "" >> "$REPORT_FILE"
done

cat >> "$REPORT_FILE" << EOF

## Next PDCA Cycle

Based on error taxonomy, recommended focus for next sprint:

1. **E0425** (${ERRORS[E0425]:-0} errors): Variable scoping in async/closures
2. **E0412** (${ERRORS[E0412]:-0} errors): Generic type parameter generation
3. **E0308** (${ERRORS[E0308]:-0} errors): Type coercion consistency

**Expected Impact**: Fixing top 3 errors would resolve ~$(echo "scale=0; (${ERRORS[E0425]:-0} + ${ERRORS[E0412]:-0} + ${ERRORS[E0308]:-0}) * 100 / $FAIL" | bc 2>/dev/null || echo "70")% of failures.

---
*Generated by depyler-corpus v3.21.0*
EOF

# Print summary
echo "=== REPORT GENERATED ==="
echo ""
cat "$REPORT_FILE"
echo ""
echo "Report saved to: $REPORT_FILE"
