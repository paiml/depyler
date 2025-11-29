# DEPYLER-0604: `depyler converge` - Automated Finish Line Command

## Status: PROPOSED
- **Created**: 2025-11-28
- **Priority**: HIGH
- **Type**: Feature/Epic
- **Effort**: 20-30 hours
- **ROI**: 10x+ (eliminates manual iteration loop)

## Problem Statement

Currently, getting all examples to compile requires a manual, iterative process:

1. Run `depyler oracle improve --input-dir ... --export-corpus gaps.json`
2. Manually analyze `gaps.json` to identify transpiler gaps vs model gaps
3. Create tickets for transpiler fixes (DEPYLER-0XXX)
4. Implement fixes in HIR/MIR/codegen
5. Retrain oracle on new patterns
6. Repeat until 100%

**Time sink**: Each iteration takes 2-4 hours of manual work. With 50+ blocking issues, this is 100-200 hours of manual effort.

**The insight**: 80% of this work is mechanical and can be automated.

## Proposed Solution

New subcommand: `depyler converge`

```bash
depyler converge \
    --input-dir ../reprorusted-python-cli/examples \
    --target-rate 100 \
    --max-iterations 50 \
    --auto-fix \
    --verbose
```

## Convergence Loop Algorithm

```
WHILE compilation_rate < target_rate AND iterations < max_iterations:

    1. COMPILE: Run all examples through transpiler + rustc
       - Collect all compilation errors
       - Hash errors for deduplication
       - Track error → example mapping

    2. CLASSIFY: For each unique error:
       - Run through oracle classifier
       - Categorize: TRANSPILER_GAP | MODEL_GAP | USER_ERROR | UNKNOWN
       - Extract: error_code, location, context, confidence

    3. CLUSTER: Group errors by root cause
       - E0599 (method not found) → missing stdlib mapping
       - E0308 (type mismatch) → type inference gap
       - E0277 (trait not impl) → missing codegen pattern
       - E0425 (unresolved name) → import/scope issue

    4. PRIORITIZE: Rank clusters by impact
       - impact_score = num_examples_blocked * confidence
       - Select top cluster for fixing

    5. FIX (dual-track):

       IF cluster.type == TRANSPILER_GAP:
           a. Generate fix template based on error pattern
           b. Identify target file (expr_gen.rs, stmt_gen.rs, etc.)
           c. Apply codegen patch (conservative: add new match arm)
           d. Run tests to verify no regression
           e. IF tests pass: commit fix
              ELSE: rollback, flag for manual review

       IF cluster.type == MODEL_GAP:
           a. Extract error patterns to training corpus
           b. Deduplicate against existing corpus
           c. Retrain oracle with new patterns
           d. Validate accuracy didn't regress

    6. VERIFY: Recompile affected examples
       - Track: errors_before → errors_after
       - Log: fix effectiveness (% reduction)
       - Update: compilation_rate

    7. REPORT: Emit progress
       - Iteration N: X/Y examples compiling (Z%)
       - Fixed: [list of error clusters resolved]
       - Remaining: [list of blocking issues]
       - ETA: estimated iterations to target

END WHILE

OUTPUT:
    - Final compilation rate
    - List of auto-fixed issues
    - List of issues requiring manual intervention
    - Training corpus delta (if model updated)
    - Suggested tickets for remaining gaps
```

## CLI Interface

```
depyler converge [OPTIONS]

OPTIONS:
    --input-dir <DIR>        Directory containing Python examples [required]
    --target-rate <PERCENT>  Target compilation rate (default: 100)
    --max-iterations <N>     Maximum convergence iterations (default: 50)
    --auto-fix               Automatically apply transpiler fixes
    --dry-run                Show what would be fixed without applying
    --retrain                Retrain oracle after each iteration
    --verbose                Show detailed progress
    --output <FILE>          Write convergence report to file
    --checkpoint <DIR>       Save/resume convergence state
    --fix-confidence <N>     Minimum confidence for auto-fix (default: 0.8)
    --skip-patterns <FILE>   Patterns to skip (known limitations)
    --parallel <N>           Parallel compilation jobs (default: num_cpus)

SUBCOMMANDS:
    converge status          Show current convergence state
    converge resume          Resume from checkpoint
    converge report          Generate detailed report from last run
    converge rollback        Undo last auto-fix batch
```

## Implementation Architecture

### Phase 1: Core Loop (8-10 hours)

```
crates/depyler/src/converge/
├── mod.rs              # Public API
├── loop.rs             # Main convergence loop
├── compiler.rs         # Batch compilation + error collection
├── classifier.rs       # Error classification (wraps oracle)
├── clusterer.rs        # Error clustering by root cause
└── reporter.rs         # Progress reporting
```

**Key structs**:
```rust
pub struct ConvergenceState {
    pub iteration: usize,
    pub examples: Vec<ExampleState>,
    pub error_clusters: Vec<ErrorCluster>,
    pub compilation_rate: f64,
    pub fixes_applied: Vec<AppliedFix>,
}

pub struct ErrorCluster {
    pub root_cause: RootCause,
    pub error_code: String,
    pub examples_blocked: Vec<PathBuf>,
    pub sample_errors: Vec<CompilationError>,
    pub fix_confidence: f64,
    pub suggested_fix: Option<FixTemplate>,
}

pub enum RootCause {
    TranspilerGap(TranspilerGapInfo),
    ModelGap(ModelGapInfo),
    UserError(UserErrorInfo),
    Unknown,
}
```

### Phase 2: Auto-Fix Engine (8-12 hours)

```
crates/depyler/src/converge/
├── fixer/
│   ├── mod.rs          # Fix orchestration
│   ├── templates.rs    # Fix templates by error type
│   ├── expr_fixes.rs   # expr_gen.rs patches
│   ├── stmt_fixes.rs   # stmt_gen.rs patches
│   ├── type_fixes.rs   # Type inference patches
│   └── validator.rs    # Post-fix validation
```

**Fix template example** (E0599: method not found):
```rust
pub struct MethodNotFoundFix {
    pub object_type: String,      // e.g., "serde_json::Value"
    pub method_name: String,      // e.g., "contains_key"
    pub python_method: String,    // e.g., "in" operator
    pub rust_replacement: String, // e.g., ".get().is_some()"
    pub location: FileLocation,   // expr_gen.rs:4293
}

impl MethodNotFoundFix {
    pub fn generate_patch(&self) -> String {
        format!(r#"
// DEPYLER-AUTO: {} → {} for {}
"{}" => {{
    // {} in Python → {} in Rust
    {}
}},
"#, self.python_method, self.method_name, self.object_type,
    self.python_method, self.python_method, self.rust_replacement,
    self.rust_replacement)
    }
}
```

### Phase 3: Model Integration (4-6 hours)

```
crates/depyler/src/converge/
├── oracle_bridge.rs    # Oracle integration
├── corpus_export.rs    # Export errors to training corpus
└── retrain.rs          # Trigger oracle retraining
```

**Corpus export format** (compatible with `oracle train`):
```json
{
  "errors": [
    {
      "hash": "e0599_value_contains_key",
      "error_code": "E0599",
      "message": "no method named `contains_key` found for struct `Value`",
      "category": "TRANSPILER_GAP",
      "subcategory": "missing_stdlib_mapping",
      "fix_applied": true,
      "fix_commit": "abc123"
    }
  ]
}
```

## Error Classification Heuristics

| Error Code | Pattern | Classification | Auto-Fix Strategy |
|------------|---------|----------------|-------------------|
| E0599 | method not found | TRANSPILER_GAP | Add stdlib mapping |
| E0308 | type mismatch | TRANSPILER_GAP | Fix type inference |
| E0277 | trait not impl | TRANSPILER_GAP | Add trait bound/impl |
| E0425 | unresolved name | TRANSPILER_GAP | Fix import generation |
| E0382 | use after move | TRANSPILER_GAP | Add .clone() |
| E0502 | borrow conflict | TRANSPILER_GAP | Restructure borrows |
| E0061 | wrong arg count | MODEL_GAP | Retrain on pattern |
| E0433 | unresolved import | TRANSPILER_GAP | Add use statement |

## Safety Constraints

1. **Never break passing examples**: Run full test suite before committing
2. **Conservative fixes only**: Add new match arms, don't modify existing
3. **Rollback on regression**: If tests fail, revert and flag for manual review
4. **Confidence threshold**: Only auto-fix if confidence > 0.8
5. **Human review for unknowns**: Flag UNKNOWN errors for manual triage
6. **Checkpoint frequently**: Save state every iteration for resume

## Success Metrics

| Metric | Target |
|--------|--------|
| Compilation rate | 100% |
| Auto-fix success rate | >80% |
| Iterations to converge | <20 |
| Time per iteration | <5 min |
| Manual intervention | <20% of issues |
| Regression rate | 0% |

## Example Output

```
$ depyler converge --input-dir ../reprorusted-python-cli/examples --verbose

🔄 Depyler Convergence Loop v0.1.0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Initial State
   Examples: 294
   Compiling: 186 (63.3%)
   Errors: 847 unique, 2,341 total

🔄 Iteration 1
   Clustered: 47 root causes
   Top cluster: E0599 HashMap::contains_key on Value (blocks 34 examples)
   Fix: Add .get().is_some() mapping in expr_gen.rs
   Applied: ✅ (commit abc123)
   Verified: 34 examples unblocked
   Rate: 186 → 220 (74.8%) [+11.5%]

🔄 Iteration 2
   Top cluster: E0308 Option<T> vs T mismatch (blocks 28 examples)
   Fix: Add unwrap_or_default() in type coercion
   Applied: ✅ (commit def456)
   Verified: 28 examples unblocked
   Rate: 220 → 248 (84.4%) [+9.6%]

...

🔄 Iteration 12
   Top cluster: E0277 String doesn't impl Error (blocks 3 examples)
   Fix: Wrap in custom error type
   Applied: ✅ (commit xyz789)
   Rate: 291 → 294 (100%) [+1.0%]

✅ Convergence Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📈 Summary
   Final rate: 294/294 (100%)
   Iterations: 12
   Auto-fixes: 38
   Manual flags: 4
   Time: 47 minutes

📝 Report saved to: convergence-2025-11-28.json
```

## Integration with Existing Commands

| Command | Relationship |
|---------|--------------|
| `oracle improve` | `converge` wraps this for model updates |
| `oracle train` | `converge` triggers after corpus export |
| `transpile` | `converge` uses for code generation |
| `compile` | `converge` uses for verification |
| `check` | `converge` uses for pre-validation |

## Dependencies

- `depyler-oracle` (classification)
- `depyler-core` (transpilation)
- `git2` (commit management)
- `indicatif` (progress bars)
- `rayon` (parallel compilation)

## Testing Strategy

1. **Unit tests**: Each component (classifier, clusterer, fixer)
2. **Integration tests**: Full loop on small example set
3. **Regression tests**: Ensure no examples regress
4. **Chaos tests**: Inject fake errors to test recovery

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Auto-fix introduces bugs | Comprehensive test suite + rollback |
| Infinite loop (no progress) | Max iterations + diminishing returns detection |
| Model accuracy regresses | Validation after each retrain |
| Too many manual flags | Improve heuristics based on patterns |
| Slow compilation | Parallel jobs + incremental builds |

## Related Tickets

- DEPYLER-0435: reprorusted-python-cli 100% compilation (manual approach)
- DEPYLER-0289: HashMap type inference
- DEPYLER-0294: Result unwrapping in try blocks
- DEPYLER-0305: File I/O std::fs

## Acceptance Criteria

- [ ] `depyler converge --help` shows all options
- [ ] Convergence loop runs to completion or max iterations
- [ ] Auto-fixes are applied with proper commits
- [ ] No regression in existing passing examples
- [ ] Checkpoint/resume works correctly
- [ ] Report shows actionable information
- [ ] Manual flags are accurate (not noise)
- [ ] Achieves 100% on reprorusted-python-cli examples

## References

- Oracle improve command: `crates/depyler/src/lib.rs:oracle_improve_command`
- Error classification: `crates/depyler-oracle/src/lib.rs`
- Codegen: `crates/depyler-core/src/rust_gen/`

---

**Next Action**: Review and approve, then `pmat prompt show continue DEPYLER-0604`
