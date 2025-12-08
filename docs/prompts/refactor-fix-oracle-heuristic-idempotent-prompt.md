# Depyler Refactor-Fix Oracle Heuristic Idempotent Prompt

**Role:** Principal Compiler Architect & Automation Engineer
**Mission:** Achieve 80% Single-Shot Compilation (Target: DEPYLER-204).
**Current Status:** 23.6% (Critical Gap).
**Method:** "Extreme TDD" with Dynamic Tool Selection.

**PROTOCOL (Execute Idempotently):**

1.  **PHASE 1: DIAGNOSTIC CHECK (The "Oracle" Decision)**
    *   **Check Active Failure:** Does `examples/repro_fix.py` exist?
        *   *Run:* `depyler transpile examples/repro_fix.py` && `rustc --crate-type lib --edition 2021 examples/repro_fix.rs`
        *   **IF FAIL:** You are in **REPAIR MODE**. Proceed to Phase 2.
        *   **IF PASS (or missing):** You are in **HUNT MODE**.
            *   *Cleanup:* Remove `examples/repro_fix.*`.
            *   *Scan:* Read `docs/qa/tech-debt-dec3.md` or `benchmarks/results/` for the *next highest-frequency* failure (e.g., `sys.argv`, `serde` imports, `try/except`, `dict` mutation).
            *   *Create:* Write a minimal `examples/repro_fix.py` isolating *only* that pattern.
            *   *Verify:* Confirm it fails `rustc` (TDD Red). **STOP. Let the next cycle fix it.**

2.  **PHASE 2: INTELLIGENT REPAIR (The Fix)**
    *   **Analyze:** Why is `repro_fix.py` failing? (e.g., "Missing method `get` on tuple").
    *   **Select Strategy:**
        *   *Option A (Codegen):* Modify `crates/depyler-core/src/codegen/` (likely `stmt_gen.rs` or `expr_gen.rs`) to generate correct Rust.
        *   *Option B (Type System):* If type inference is wrong, adjust `type_mapper.rs`.
        *   *Option C (Fallback):* If complex, fallback to `Any` or `serde_json::Value` to ensure compilation.
    *   **Execute:** Edit the code. *Prioritize compilability over perfect performance.*

3.  **PHASE 3: VERIFY & COMMIT**
    *   **Verify:** Run the reproduction case again.
    *   **If Passing:**
        *   Commit: `fix(codegen): Resolve [Pattern Name] to boost compile rate`.
        *   *Loop:* The next run will see it passing and switch back to **HUNT MODE**.

**COMMAND:**
Assess `examples/repro_fix.py`.
If it fails, **FIX IT** (edit `depyler-core`).
If it passes, **BREAK IT** (create new repro from Tech Debt report).
**GO.**
