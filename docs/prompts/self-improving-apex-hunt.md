# The Self-Improving Apex Hunt Protocol

**Role:** Apex Hunt Team
**Objective:** Converge on 80% Single-Shot Compilation Rate for `reprorusted-python-cli`.
**Context:** `depyler-oracle` now **auto-retrains** on code changes (DEPYLER-MLOPS-001). Manual retraining is obsolete.
**Tooling:** `depyler report`, `depyler localize`, `typeshed_ingest`.

## PROTOCOL: EXECUTE ONE RAPID PDCA CYCLE (Idempotent)

### 1. PLAN (Self-Improving Diagnosis)
*   **Check State:** Does `examples/repro_fix.py` exist?
*   **IF PASSING (or missing):**
    *   **Action 1:** Remove `examples/repro_fix.py`.
    *   **Action 2 (Turbo Scan):** Run `depyler report --corpus ../reprorusted-python-cli --format rich`.
        *   *Note:* The oracle will now *automatically* check its state and retrain if the codebase has changed since the last run, ensuring diagnostics are fresh.
    *   **Target Selection:** Pick the P0 Critical failure from the fresh report.
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
    *   **Commit:** `fix(core): Resolve [Pattern Name] (Refs DEPYLER-204)`
    *   **Reflect:** "Fix committed. Oracle will auto-learn this pattern on next run."
    *   **Loop:** Ready for next "Plan" phase.
