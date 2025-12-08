# The Self-Evolving Apex Hunt Protocol

**Role:** Apex Hunt Team (Autonomous MLOps)
**Objective:** Converge on 80% Single-Shot Compilation Rate for `reprorusted-python-cli`.
**Context:** `depyler-oracle` now **automatically monitors its own performance and retrains when drift is detected**. Manual retraining is obsolete. Model is always fresh.
**Tooling:** `depyler report`, `depyler localize`, `typeshed_ingest`.

## PROTOCOL: EXECUTE ONE RAPID PDCA CYCLE (Idempotent)

### 1. PLAN (Self-Monitoring Diagnosis)
*   **Check State:** Does `examples/repro_fix.py` exist?
*   **IF PASSING (or missing):**
    *   **Action 1:** Remove `examples/repro_fix.py`.
    *   **Action 2 (Turbo Scan & Self-Healing AI Insight):** Run `depyler report --corpus ../reprorusted-python-cli --format rich`.
        *   *Note:* The oracle will now **automatically monitor for drift and retrain if needed** during the report generation. This ensures diagnostics are always fresh and reflect the current codebase.
    *   **Target Selection (AI-Prioritized):** Based on the `depyler report`'s:
        *   **Andon Status:** Observe `print_drift_status()` output. If `DriftDetected`, prioritize retraining.
        *   **Top Error Taxonomies** (e.g., `E0425`, `E0412`, `E0308`, or `TRANSPILE`).
        *   **Semantic Classification:** Are there failing Core/Stdlib/External clusters?
        *   **PageRank/Clustering:** Are there "super-spreader" errors or high-density failure clusters?
    *   **Create:** Synthesize a minimal `examples/repro_fix.py` that isolates the chosen P0 failure.
    *   **Verify:** Run `./target/release/depyler compile examples/repro_fix.py --cargo-first`. Confirm failure (Red State). **STOP.**

### 2. DO (Precision Repair)
*   **IF FAILING (Active Red State):**
    *   **Log:** `pmat work "Apex Hunt: Repairing [Error Code]"`
    *   **Strategy:**
        *   **Missing API:** Use **Typeshed Ingester**.
        *   **Semantic Bug:** Use **Fault Localization** (`depyler localize`).
    *   **Execute:** Modify `crates/depyler-core` to implement the fix.

### 3. CHECK (Verify)
*   **Run:** `./target/release/depyler compile examples/repro_fix.py --cargo-first`.
*   **Constraint:** Must pass `cargo check` without errors.

### 4. ACT (Standardize & Track)
*   **IF GREEN:**
    *   **Log:** `pmat work "Hunt Mode: Fixed [Pattern Name] - Cycle Complete"`
    *   **Commit:** `fix(core): Resolve [Pattern Name] based on AI-driven analysis (Refs DEPYLER-204)`
    *   **Reflect:** "Fix committed. Oracle automatically learned this pattern."
    *   **Loop:** Ready for next "Plan" phase.
