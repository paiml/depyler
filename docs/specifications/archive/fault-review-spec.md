# Specification: Fault Localization & Review Strategy

**Version:** 1.0.0
**Status:** Implementation
**Tooling:** Tarantula SBFL (Spectrum-Based Fault Localization)

## 1. Objective

Implement an automated fault localization pipeline for `depyler` to pinpoint the exact Rust codegen lines responsible for transpilation failures. This replaces "guess-and-check" debugging with **Spectrum-Based Fault Localization (SBFL)**, leveraging the `tarantula` crate.

## 2. Methodology (The "Tarantula" Process)

SBFL statistically correlates code coverage of passing vs. failing tests to calculate a "suspiciousness score" for each line of code.

### 2.1 The Algorithm (Ochiai Metric)

For each line $s$ in `depyler-core`:
$$ Suspiciousness(s) = \frac{\frac{failed(s)}{total\_failed}}{\frac{failed(s)}{total\_failed} + \frac{passed(s)}{total\_passed}} $$

*   $failed(s)$: Number of failing test cases that executed line $s$.
*   $passed(s)$: Number of passing test cases that executed line $s$.

Lines with high scores are executed frequently in failures but rarely in successes—strong candidates for the root cause.

## 3. Integration Plan

### 3.1 Instrumentation
1.  **Coverage Trace:** Use `minicov` or `llvm-cov` to capture execution traces of `depyler-core` during `depyler transpile`.
2.  **Granularity:** Statement-level or Basic Block-level coverage.

### 3.2 The `depyler localize` Command

New CLI subcommand to run the analysis:

```bash
depyler localize --target examples/repro_fix.py
```

**Workflow:**
1.  **Pass Run:** Runs known-good examples (e.g., `examples/simple_*.py`). Captures coverage $C_{pass}$.
2.  **Fail Run:** Runs the failing target (e.g., `examples/repro_fix.py`). Captures coverage $C_{fail}$.
3.  **Analysis:** Computes Ochiai scores for all lines in `crates/depyler-core`.
4.  **Report:** Outputs top 10 suspicious locations.

### 3.3 Expected Output Format

```text
Fault Localization Report
Target: examples/repro_fix.py (E0308 Type Mismatch)

Top Suspicious Locations:
1. crates/depyler-core/src/rust_gen/expr_gen.rs:1405 (Score: 1.00)
   > parse_quote! { serde_json::Value }
   * Reason: Executed in failure, never in passing baseline.

2. crates/depyler-core/src/type_mapper.rs:156 (Score: 0.92)
   > RustType::Custom("serde_json::Value".to_string())
```

## 4. Strategic Value

*   **Hunt Mode Acceleration:** Instantly directs the Hunt Mode agent to the file and line number needing repair.
*   **Root Cause Validation:** Verifies that a fix actually changes the execution path of the failure.
*   **Automated Debugging:** Reduces human analysis time from minutes to seconds.

## 5. Implementation Steps

1.  [ ] Add `tarantula` dependency to `depyler-oracle`.
2.  [ ] Create `crates/depyler/src/localize_cmd.rs`.
3.  [ ] Instrument `depyler-core` with a tracing feature flag (`feature = "trace-decisions"`).
4.  [ ] Wire up `depyler localize` to run the comparison.

## 6. References

[1] **Jones, J. A., & Harrold, M. J.** (2005). "Empirical evaluation of the tarantula automatic fault-localization technique." *ASE 2005*.
[2] **Abreu, R., et al.** (2007). "On the accuracy of spectrum-based fault localization." *Testing: Academic and Industrial Conference*.
