# The Apex Hunt Protocol Prompt

**Role:** Apex Hunt Team
**Objective:** Converge on 80% Single-Shot Compilation Rate for `reprorusted-python-cli`.
**Context:** `DEPYLER-O1MAP-001` (Typeshed) is complete. `DEPYLER-REPORT-V2` (Rich Signal) is live.
**Tooling:** `depyler report`, `depyler localize`, `typeshed_ingest`, `depyler compile --cargo-first`.

## PROTOCOL: EXECUTE ONE RAPID PDCA CYCLE (Idempotent)

### 1. PLAN (Data-Driven Diagnosis)
*   **Check State:** Does `examples/repro_fix.py` exist?
*   **IF PASSING (or missing):**
    *   **Action 1:** Remove `examples/repro_fix.py`.
    *   **Action 2 (Turbo Scan & AI Insight):** Run `depyler report --corpus ../reprorusted-python-cli --format rich`. **This is the critical step to get the NEW baseline and P0 targets.**
    *   **Target Selection (AI-Prioritized):** Based on the `depyler report`'s:
        *   **Top Error Taxonomies** (e.g., `E0425`, `E0412`, `E0308`, or `TRANSPILE`).
        *   **Semantic Classification:** Are there failing Core/Stdlib/External clusters?
        *   **PageRank/Clustering:** Are there "super-spreader" errors or high-density failure clusters?
    *   **Create:** Synthesize a minimal `examples/repro_fix.py` that isolates the chosen P0 failure.
    *   **Verify:** Run `./target/release/depyler compile examples/repro_fix.py --cargo-first`. Confirm failure (Red State). **STOP.**

### 2. DO (Precision Repair)
*   **IF FAILING (Active Red State):**
    *   **Log:** `pmat work "Apex Hunt: Repairing [Error Code] from report"`
    *   **Strategy A (Missing API - If Applicable):**
        *   **Use Typeshed Ingester:** Locate the `.pyi` stub for the missing module/method. Run `typeshed_ingest` and copy the generated `ModuleMapping` code into `module_mapper.rs`.
    *   **Strategy B (Core Semantic Bug):**
        *   **Fault Localization:** If the error is complex, run `./target/release/depyler localize --target examples/repro_fix.py` to pinpoint the exact line in `depyler-core`.
        *   Fix logic in `stmt_gen.rs`, `expr_gen.rs`, `type_hints.rs`, or `borrowing.rs`.
    *   **Execute:** Modify `crates/depyler-core` to implement the fix.

### 3. CHECK (Verify)
*   **Run:** `./target/release/depyler compile examples/repro_fix.py --cargo-first`.
*   **Constraint:** Must pass `cargo check` without errors.

### 4. ACT (Standardize & Track)
*   **IF GREEN:**
    *   **Log:** `pmat work "Apex Hunt: Fixed [Pattern Name] - Cycle Complete"`
    *   **Commit:** `fix(core): Resolve [Pattern Name] based on AI-driven analysis (Refs DEPYLER-204)`
    *   **Reflect:** Note the fix impact and which strategy (Typeshed, Localize, Manual) was used.
    *   **Loop:** Ready for next "Plan" phase.
