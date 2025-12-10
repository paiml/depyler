# DEPYLER-CORPUS-100: Convergence Protocol

**Objective:** Achieve 100% single-shot compilation rate on the `reprorusted-python-cli` corpus.
**Target:** `../reprorusted-python-cli/examples`
**Constraint:** Idempotent, Scientific Method, Transpiler-Only Fixes.

---

## 1. Setup & Baseline (Idempotent)

Ensure the environment is clean and tools are ready.

```bash
# 1. Build release binary for speed
cd /home/noah/src/depyler
cargo build --release --bin depyler

# 2. Define environment variables
export DEPYLER_BIN="/home/noah/src/depyler/target/release/depyler"
export CORPUS_DIR="/home/noah/src/reprorusted-python-cli/examples"
export LOG_DIR="/tmp/depyler_convergence"
mkdir -p "$LOG_DIR"

# 3. Clean previous run artifacts
rm -f "$LOG_DIR/baseline_results.txt" "$LOG_DIR/error_log.txt"

# 4. Warm the compilation cache (DEPYLER-CACHE-001)
# This pre-populates cache with transpilation results for O(1) subsequent lookups
"$DEPYLER_BIN" cache warm --input-dir "$CORPUS_DIR"

# 5. Show cache statistics
"$DEPYLER_BIN" cache stats
```

## 2. Measurement Cycle (The "Scanner")

Run this block to assess current status. It is designed to be idempotent and safe.

```bash
echo "Starting Corpus Scan..."
PASS=0
FAIL=0
TOTAL=0

# Iterate through all example directories
for dir in "$CORPUS_DIR"/example_*; do
    [ -d "$dir" ] || continue
    TOTAL=$((TOTAL + 1))
    
    # Identify source file (first .py file that isn't a test)
    py_source=$(find "$dir" -maxdepth 1 -name "*.py" ! -name "test_*" | head -n 1)
    
    if [ -z "$py_source" ]; then
        echo "⚠️  SKIP: $dir (No Python source found)"
        continue
    fi

    # Transpile FRESH (Overwrites previous output)
    # NOTE: We output to 'out.rs' as per reprorusted convention
    "$DEPYLER_BIN" transpile "$py_source" --output "$dir/out.rs" --cargo-toml > /dev/null 2>&1
    
    # Validation Gate
    if [ -f "$dir/Cargo.toml" ]; then
        # Use cargo check to handle dependencies (clap, serde, etc.)
        if cargo check --manifest-path "$dir/Cargo.toml" --quiet 2>/dev/null; then
            echo "✅ PASS: $(basename "$dir")"
            ((PASS++))
        else
            echo "❌ FAIL: $(basename "$dir")"
            ((FAIL++))
            # Log specific error for triage
            echo "=== $(basename "$dir") ===" >> "$LOG_DIR/error_log.txt"
            cargo check --manifest-path "$dir/Cargo.toml" --message-format=short 2>&1 | grep "error\[" | head -n 3 >> "$LOG_DIR/error_log.txt"
        fi
    else
        # Fallback for simple files (though Depyler should generate Cargo.toml)
        if rustc --crate-type lib "$dir/out.rs" --out-dir /tmp > /dev/null 2>&1; then
             echo "✅ PASS: $(basename "$dir")"
             ((PASS++))
        else
             echo "❌ FAIL: $(basename "$dir")"
             ((FAIL++))
             echo "=== $(basename "$dir") ===" >> "$LOG_DIR/error_log.txt"
             rustc --crate-type lib "$dir/out.rs" --out-dir /tmp 2>&1 | grep "error\[" | head -n 3 >> "$LOG_DIR/error_log.txt"
        fi
    fi
done

echo "---------------------------------------------------"
echo "Results: $PASS / $TOTAL Passed ($(( 100 * PASS / TOTAL ))%)"
echo "See details in $LOG_DIR/error_log.txt"
```

## 3. The Convergence Loop (The "Fixer")

Execute this process for **EACH** failure class identified in `$LOG_DIR/error_log.txt`.

### Phase A: Identification (Group & Prioritize)
1.  **Group Errors:** Run this to find the highest impact error code.
    ```bash
    grep "error\[" "$LOG_DIR/error_log.txt" | awk '{print $1}' | sort | uniq -c | sort -nr
    ```
2.  **Consult Oracle:** Check if this is a known pattern or regression.
    ```bash
    "$DEPYLER_BIN" oracle classify --error-log "$LOG_DIR/error_log.txt"
    ```

### Phase B: Reproduction (Minimal Case)
1.  Create a reproduction file in `crates/depyler-core/tests/repro_fail.rs`.
2.  Extract the failing Python snippet (e.g., `pathlib.Path("foo").glob("*")`).
3.  Add it as a test case.

### Phase C: Diagnosis (Explainability - Issue #214)
1.  **Trace:** Run the transpiler with tracing enabled on the failing example.
    ```bash
    "$DEPYLER_BIN" transpile "$FAILING_PY_PATH" --output /tmp/debug.rs --trace-output /tmp/debug.trace.json
    ```
2.  **Explain:** Use the explain command to correlate errors with transpiler decisions.
    ```bash
    "$DEPYLER_BIN" explain /tmp/debug.rs --trace /tmp/debug.trace.json --verbose
    ```
3.  **Analyze:** The explain output shows WHY type inference missed or wrong method mapped.

### Phase D: Resolution (Transpiler Fix)
**CRITICAL RULE:** NEVER modify the generated `.rs` file. Modify the transpiler logic (`expr_gen.rs`, `stmt_gen.rs`, etc.).

1.  **Apply Fix:** Edit the Rust code in `crates/depyler-core/src/...`.
2.  **Verify Local:** Run the reproduction test.
    ```bash
    cargo test --package depyler-core --test repro_fail
    ```

### Phase E: Verification (Regression Check)
1.  Rerun the **Measurement Cycle** (Section 2).
2.  Ensure `PASS` count increased and `FAIL` count decreased.
3.  Ensure no *new* regressions appeared.

### Phase F: Online Retraining (Oracle Improve - #211)
After fixing errors, retrain the Oracle model to learn from corrections:
```bash
# Train oracle on updated corpus (learns from fixes)
"$DEPYLER_BIN" oracle train --corpus "$CORPUS_DIR"

# Or use continuous improvement loop until 100%
"$DEPYLER_BIN" oracle improve --corpus "$CORPUS_DIR" --target-rate 1.0
```

## 3.1 Alternative: UTOL Automated Loop (Toyota Way)

Instead of manual Phase A-F, use UTOL for automated convergence:
```bash
# Unified Training Oracle Loop - self-correcting compilation feedback
"$DEPYLER_BIN" utol --corpus "$CORPUS_DIR" --target-rate 0.80 --max-iterations 50
```
UTOL implements: Jidoka (stop on defect), Kaizen (continuous improvement), Andon (visual feedback).

## 4. Completion Criteria

The campaign is complete when:
1.  **Pass Rate:** ≥ 80% (Mandatory), 100% (Stretch).
2.  **Clean Log:** `$LOG_DIR/error_log.txt` contains only "known limitation" errors (if any).
3.  **Validation:** `scripts/validate_transpiled_strict.sh` passes on all compiled examples.

## 5. Quick Commands Reference

| Task | Command |
|------|---------|
| **Scan** | (Copy/Paste Section 2) |
| **Trace** | `DEPYLER_TRACE=1 depyler transpile ...` |
| **Test Core** | `cargo test -p depyler-core` |
| **Check Oracle** | `depyler oracle show` |
| **Cache Stats** | `depyler cache stats` |
| **Cache Warm** | `depyler cache warm --input-dir $CORPUS_DIR` |
| **Cache Clear** | `depyler cache clear --force` |
| **Cache GC** | `depyler cache gc --max-size-mb 500` |
| **Explain** | `depyler explain <out.rs> --trace <trace.json>` |
| **Oracle Train** | `depyler oracle train --corpus $CORPUS_DIR` |
| **Oracle Improve** | `depyler oracle improve --corpus $CORPUS_DIR` |
| **UTOL** | `depyler utol --corpus $CORPUS_DIR --target-rate 0.80` |
