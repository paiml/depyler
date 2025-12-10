# Corpus Convergence Protocol

This chapter describes how to achieve 100% single-shot compilation rate on real-world Python corpora using Depyler's integrated toolchain.

## Overview

The convergence protocol systematically improves transpilation success rates by combining:

- **Compilation Cache** (DEPYLER-CACHE-001): O(1) lookups for unchanged files
- **Explain Command** (Issue #214): Correlates Rust errors with transpiler decisions
- **Oracle ML** (#211): Online retraining from corrections
- **UTOL** (Toyota Way): Automated self-correcting convergence loop

## Quick Start

```bash
# 1. Build release binary for speed
cargo build --release --bin depyler

# 2. Warm the compilation cache
depyler cache warm --input-dir /path/to/corpus

# 3. Run automated convergence (Toyota Way)
depyler utol --corpus /path/to/corpus --target-rate 0.80

# 4. Or use oracle continuous improvement
depyler oracle improve --corpus /path/to/corpus --target-rate 1.0
```

## The Convergence Loop

### Phase A: Setup & Baseline

```bash
# Define environment
export DEPYLER_BIN="/path/to/depyler/target/release/depyler"
export CORPUS_DIR="/path/to/corpus/examples"
export LOG_DIR="/tmp/depyler_convergence"
mkdir -p "$LOG_DIR"

# Warm cache for O(1) subsequent lookups
"$DEPYLER_BIN" cache warm --input-dir "$CORPUS_DIR"

# Show cache statistics
"$DEPYLER_BIN" cache stats
```

### Phase B: Measurement

Run the scanner to assess current compilation success rate:

```bash
PASS=0; FAIL=0; TOTAL=0
for dir in "$CORPUS_DIR"/example_*; do
    [ -d "$dir" ] || continue
    TOTAL=$((TOTAL + 1))

    py_source=$(find "$dir" -maxdepth 1 -name "*.py" ! -name "test_*" | head -n 1)
    [ -z "$py_source" ] && continue

    "$DEPYLER_BIN" transpile "$py_source" --output "$dir/out.rs" --cargo-toml

    if cargo check --manifest-path "$dir/Cargo.toml" --quiet 2>/dev/null; then
        echo "PASS: $(basename "$dir")"; ((PASS++))
    else
        echo "FAIL: $(basename "$dir")"; ((FAIL++))
    fi
done
echo "Results: $PASS / $TOTAL ($(( 100 * PASS / TOTAL ))%)"
```

### Phase C: Diagnosis (Explainability)

For each failing example:

```bash
# Transpile with decision tracing
"$DEPYLER_BIN" transpile "$FAILING_PY" --output /tmp/debug.rs --trace-output /tmp/debug.trace.json

# Correlate errors with transpiler decisions
"$DEPYLER_BIN" explain /tmp/debug.rs --trace /tmp/debug.trace.json --verbose
```

The explain command shows:
- Which transpiler decisions led to the error
- Type inference misses
- Method mapping failures
- Suggested fixes

### Phase D: Resolution

**CRITICAL RULE**: Never modify generated `.rs` files. Fix the transpiler.

1. Create reproduction test in `crates/depyler-core/tests/repro_fail.rs`
2. Fix transpiler logic in `crates/depyler-core/src/`
3. Verify: `cargo test --package depyler-core`

### Phase E: Verification

Re-run the measurement cycle (Phase B) to confirm:
- PASS count increased
- FAIL count decreased
- No new regressions

### Phase F: Online Retraining

After fixing errors, retrain the Oracle model:

```bash
# Train on corrected corpus
"$DEPYLER_BIN" oracle train --corpus "$CORPUS_DIR"

# Or run continuous improvement until target
"$DEPYLER_BIN" oracle improve --corpus "$CORPUS_DIR" --target-rate 1.0
```

## UTOL: Automated Convergence

Instead of manual phases, use UTOL (Unified Training Oracle Loop):

```bash
"$DEPYLER_BIN" utol --corpus "$CORPUS_DIR" --target-rate 0.80 --max-iterations 50
```

UTOL implements Toyota Way principles:
- **Jidoka** (Autonomation): Stop on defect, fix before proceeding
- **Kaizen** (Continuous Improvement): Small incremental fixes
- **Andon** (Visual Feedback): Clear pass/fail indicators
- **Heijunka** (Load Leveling): Prioritize high-impact errors

## Command Reference

| Command | Description |
|---------|-------------|
| `depyler cache warm --input-dir DIR` | Pre-populate cache with transpilation results |
| `depyler cache stats` | Show cache hit/miss rates and size |
| `depyler cache gc --max-size-mb 500` | Garbage collect old cache entries |
| `depyler cache clear --force` | Delete all cached results |
| `depyler explain FILE --trace TRACE` | Correlate errors with decisions |
| `depyler oracle train --corpus DIR` | Train ML model on corpus |
| `depyler oracle improve --corpus DIR` | Continuous improvement loop |
| `depyler oracle classify --error-log FILE` | Classify errors from log |
| `depyler utol --corpus DIR --target-rate 0.80` | Automated convergence |

## Completion Criteria

The convergence campaign is complete when:

1. **Pass Rate**: >= 80% (mandatory), 100% (stretch goal)
2. **Clean Log**: Error log contains only known limitations
3. **Validation**: All compiled examples pass strict validation

## Example: reprorusted-python-cli Corpus

```bash
# Target corpus
export CORPUS_DIR="/home/noah/src/reprorusted-python-cli/examples"

# Run convergence
depyler utol --corpus "$CORPUS_DIR" --target-rate 0.80

# Or manual protocol
# See: docs/prompts/converge_reprorusted_100.md
```

## Five-Whys: Why This Protocol?

1. **Why use cache?** Unchanged files skip transpilation (O(1) vs O(n))
2. **Why explain command?** Shows WHY transpiler made wrong decisions
3. **Why oracle retrain?** Model learns from corrections, improves future
4. **Why UTOL?** Automates the entire loop with Toyota Way discipline
5. **Why document this?** Critical workflow must not slip through cracks

## See Also

- [Oracle: ML Error Classification](./oracle.md)
- [Hunt Mode: Automated Calibration](./hunt-mode.md)
- [Corpus Report: Scientific Analysis](./corpus-report.md)
