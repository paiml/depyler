# Overnight Autonomous Session Process

Autonomous Claude Code sessions for continuous transpiler improvement.

## Overview

Run Claude Code unattended for 6+ hours to fix transpiler errors systematically. Each session targets error clusters in priority order, committing fixes with DEPYLER-XXXX ticket refs.

## Prerequisites

1. **Monitoring hooks** in `.claude/settings.json`
2. **Scripts** in `scripts/overnight_*.sh`
3. **Clean git state** on master branch

## Monitoring Infrastructure

### Hooks (`.claude/settings.json`)

```json
{
  "hooks": {
    "PostToolUse": [
      {"matcher": "*", "hooks": [{"type": "command", "command": "/home/noah/src/depyler/scripts/overnight_monitor.sh"}]},
      {"matcher": "Bash", "hooks": [{"type": "command", "command": "/home/noah/src/depyler/scripts/capture_rustc_diagnostics.sh"}]}
    ],
    "Stop": [...],
    "SessionStart": [...]
  }
}
```

> **Note**: Use absolute paths. Hooks run from arbitrary CWD.

### Status Files

| File | Purpose |
|------|---------|
| `/tmp/depyler_overnight_status.json` | Session state (commits, last tool, modified files) |
| `/tmp/depyler_andon_alerts.jsonl` | Error/warning alerts |
| `/tmp/depyler_rustc_diagnostics.jsonl` | Structured compiler errors for training |
| `/tmp/depyler_overnight.log` | Activity log |

### Dashboard

```bash
# From depyler directory:
watch -n10 ./scripts/overnight_dashboard.sh
```

## Prompt Template

```
You are running an OVERNIGHT AUTONOMOUS SESSION for the depyler Python→Rust transpiler.

## CRITICAL RULES
- NEVER stop after one fix - continue to next error
- NEVER ask for confirmation - decide autonomously
- ALWAYS commit working changes with DEPYLER-06XX refs
- ALWAYS work on master branch (NEVER create branches)
- Track progress in TodoWrite

## PHASE 1: COMMIT PENDING WORK
Check git status, commit any uncommitted changes.

## PHASE 2: GET ERROR BASELINE
DON'T use `depyler converge` (broken). Use direct transpilation:

cargo build --release

```bash
for dir in ../reprorusted-python-cli/examples/example_*/; do
    for py in "$dir"/*.py; do
        [[ "$py" == *test_* ]] && continue
        [[ -f "$py" ]] || continue
        ./target/release/depyler transpile "$py" 2>&1
    done
done 2>&1 | grep -oE "error\[E[0-9]+\]" | sort | uniq -c | sort -rn
```

## PHASE 3: FIX LOOP
For each error type (highest count first):
1. Sample the error
2. Trace with transpile output
3. Locate fix in depyler-core
4. Implement fix
5. Validate (cargo test, clippy)
6. Commit
7. CONTINUE to next error

## WHEN TO STOP
- All errors with count > 50 addressed
- Tests broken and cannot fix
- 6+ hours elapsed
```

## Error → Fix Location Map

| Error | Fix Location |
|-------|--------------|
| E0433 | `rust_gen/import_gen.rs`, `cargo_toml_gen.rs` |
| E0599 | `rust_gen/expr_gen.rs`, `builtin_conversions.rs` |
| E0308 | `type_mapper.rs`, `rust_gen/expr_gen.rs` |
| E0277 | `rust_gen/expr_gen.rs` (Display→Debug) |
| E0425 | `rust_gen/func_gen.rs`, `stmt_gen.rs` |
| E0382 | `borrowing_context.rs` |

## Training Signal Capture

The `capture_rustc_diagnostics.sh` hook captures:
- Timestamp
- Manifest path
- Cargo command
- Structured error messages

Output: `/tmp/depyler_rustc_diagnostics.jsonl`

This supplements the 4-tier diagnostic system in `compilation_trainer.rs`.

## Success Metrics

- 5+ commits per session
- Tests pass (2 pre-existing failures OK)
- Zero clippy warnings
- Top error count reduced by 50%

## Lessons Learned

1. **Don't use `depyler converge`** — uses `rustc` directly instead of `cargo build`, missing dependencies
2. **Direct bash loops** work reliably for error collection
3. **Session context exhaustion** — previously ~4-5 hours, but OVERNIGHT_V2 achieved 13+ hours
4. **Hooks must exist before session starts** — sessions don't reload hooks mid-run
5. **New session = fresh start** — don't resume, paste full prompt
6. **OVERNIGHT_V2 prompt is production-ready** — 240% of target commits achieved
7. **Codegen fixes cluster by domain** — string handling, containers, Python runtime are distinct categories
8. **Optimizer/lint fixes are bonus commits** — DCE and clippy work happens organically
9. **~1 commit/hour is sustainable rate** — 12 commits in 13 hours

## Enhancement Ideas

### 1. Renacer Decision Traces

Extend renacer's `decision_trace` module to capture codegen decision trees during transpilation. Log which HIR→Rust codegen paths are taken and correlate with error outcomes.

```rust
// Proposed: Add decision points in expr_gen.rs
renacer::decision_trace!(
    "binop",
    lhs_type = %lhs_ty,
    rhs_type = %rhs_ty,
    operator = %op,
    chosen_path = "numeric_promotion"
);
```

### 2. aprender CITL Pattern Library

Integrate aprender's `PatternLibrary` with HNSW index for error-fix pattern matching. Store (Python AST, error code, fix diff) triples as embeddings for retrieval-augmented fix generation.

### 3. Rust Test Generation

Currently depyler generates source but no tests. Add test generation from Python doctests/type hints:
- Parse Python docstrings for examples
- Generate `#[test]` functions
- Training signal: test pass/fail ratio per pattern

### 4. Synthetic Example Augmentation

Use mutation-based augmentation to expand training corpus:
- Variable renaming (semantic-preserving)
- Expression reordering
- Type annotation injection
- Error injection for negative examples

### 5. Organizational Defect Patterns (OIP)

Use organizational-intelligence-plugin to track defect patterns over time:
- Which error codes recur across sessions?
- Which fix patterns have highest success rate?
- Temporal trends in error distribution

---

## References

CITL and program synthesis literature:

1. **Gulwani, S., Polozov, O., & Singh, R.** (2017). Program Synthesis. *Foundations and Trends in Programming Languages*, 4(1-2), 1-119. doi:10.1561/2500000010

2. **Allamanis, M., Barr, E.T., Devanbu, P., & Sutton, C.** (2018). A Survey of Machine Learning for Big Code and Naturalness. *ACM Computing Surveys*, 51(4), 1-37. doi:10.1145/3212695

3. **Gupta, R., Pal, S., Kanade, A., & Shevade, S.** (2017). DeepFix: Fixing Common C Language Errors by Deep Learning. *AAAI Conference on Artificial Intelligence*, 1345-1351.

4. **Lewis, P., Perez, E., Piktus, A., et al.** (2020). Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks. *NeurIPS*, 33, 9459-9474.

5. **Settles, B.** (2009). Active Learning Literature Survey. *Computer Sciences Technical Report 1648*, University of Wisconsin-Madison.

6. **Bengio, Y., Louradour, J., Collobert, R., & Weston, J.** (2009). Curriculum Learning. *ICML*, 41-48. doi:10.1145/1553374.1553380

7. **Yasunaga, M. & Liang, P.** (2020). Graph-based, Self-Supervised Program Repair from Diagnostic Feedback. *ICML*, 10799-10808.

8. **Chen, M., Tworek, J., Jun, H., et al.** (2021). Evaluating Large Language Models Trained on Code. *arXiv:2107.03374*.

9. **Wong, W.E., Gao, R., Li, Y., Abreu, R., & Wotawa, F.** (2016). A Survey on Software Fault Localization. *IEEE TSE*, 42(8), 707-740. doi:10.1109/TSE.2016.2521368

10. **Gazzola, L., Micucci, D., & Mariani, L.** (2019). Automatic Software Repair: A Survey. *IEEE TSE*, 45(1), 34-67. doi:10.1109/TSE.2017.2755013

---

## Historical Results

| Date | Duration | Commits | Tickets | Key Fixes |
|------|----------|---------|---------|-----------|
| 2025-11-28/29 | ~13 hrs | **12** | DEPYLER-0616→0627 | enum constants, exit codes, tuple `in`, await detection, datetime heuristics, encode/decode, strptime, HashMap/HashSet paths, dunder vars, DCE optimization |

### Session 2025-11-28/29 Details (OVERNIGHT_V2)

**Performance: 240% of target (12 commits vs 5 target)**

| Ticket | Category | Fix Description |
|--------|----------|-----------------|
| DEPYLER-0616 | codegen | Enum constant detection in direct_rules |
| DEPYLER-0617 | codegen | main() exit codes with is_main_function flag |
| DEPYLER-0618 | codegen | Tuple containment check for `in` operator |
| DEPYLER-0619 | codegen | Await expressions in variable usage detection |
| DEPYLER-0620 | codegen | Expand datetime object detection heuristics |
| DEPYLER-0621 | codegen | Route encode/decode methods through string handler |
| DEPYLER-0622 | codegen | Borrow format string in strptime for &str |
| DEPYLER-0623 | codegen | Fully qualified paths for HashMap/HashSet |
| DEPYLER-0624 | codegen | Python `__file__` and `__name__` dunder variables |
| DEPYLER-0625 | lint | Clippy warnings in oracle |
| DEPYLER-0626 | lint | Clippy warnings in tests |
| DEPYLER-0627 | optimizer | Assert/Raise in DCE variable usage detection |

**Pattern Categories Fixed:**
- String handling: encode/decode, strptime borrowing
- Container types: tuple `in`, HashMap/HashSet qualified paths
- Python runtime: `__file__`, `__name__`, async/await
- Code quality: DCE optimization, clippy compliance

**Success Factors:**
1. OVERNIGHT_V2 prompt bypasses broken `depyler converge`
2. Direct bash loops for error collection proved reliable
3. Session ran 13+ hours without context exhaustion
4. Commit rate: ~1 commit/hour average
