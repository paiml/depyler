# Mutation Testing Specification Session Summary

**Date**: 2025-10-03
**Duration**: ~4 hours
**Focus**: DEPYLER-0020 - Mutation Testing Infrastructure Setup
**Status**: ✅ **COMPLETED**

---

## Accomplishment

### DEPYLER-0020: Mutation Testing Infrastructure Setup ✅

Created comprehensive mutation testing specification based on pforge's proven methodology, adapted specifically for Depyler's Python→Rust transpilation domain.

**Deliverable**: `docs/specifications/mutant.md`
- **Size**: 23KB (950 lines)
- **Quality**: Production-ready specification
- **Source**: Adapted from `pforge-book/src/ch09-04-mutation-testing.md`

---

## Key Specification Features

### 1. Depyler-Specific Mutation Strategies

Identified 4 critical testing areas with tailored mutation approaches:

#### AST → HIR Conversion (`depyler-core/ast_bridge.rs`)
- **Risk**: Incorrect Python AST interpretation
- **Mutations**: Operator mapping, error handling, match arm logic
- **Strategy**: Comprehensive operator tests, error path validation

#### Type Inference (`depyler-analyzer/type_flow.rs`)
- **Risk**: Incorrect Rust type generation
- **Mutations**: Type resolution, ownership inference
- **Strategy**: Exact type validation, boundary testing

#### Code Generation (`depyler-core/codegen.rs`)
- **Risk**: Generated Rust doesn't compile or has wrong semantics
- **Mutations**: Token generation, scope management, control flow
- **Strategy**: Compilation validation, semantic correctness checks

#### Lambda Conversion (`depyler/src/lib.rs`)
- **Risk**: AWS Lambda generation fails or produces incorrect handler
- **Mutations**: Event type inference, runtime generation
- **Strategy**: Event-specific handler validation (leverages existing DEPYLER-0011 tests)

### 2. Mutation Operators with Kill Strategies

Documented 5 critical mutation operators with Depyler-specific examples:

1. **Replace Function Return Values**
   - Example: Type inference always returns `Type::Unknown`
   - Kill Strategy: Test all type branches with exact assertions

2. **Negate Boolean Conditions**
   - Example: `if expr.has_type_annotation()` → `if !expr.has_type_annotation()`
   - Kill Strategy: Test both true/false branches

3. **Change Comparison Operators**
   - Example: `scope_depth > 0` → `scope_depth >= 0`
   - Kill Strategy: Boundary condition tests

4. **Delete Statements**
   - Example: Remove validation calls
   - Kill Strategy: Test that invalid input fails

5. **Replace Binary Operators**
   - Example: `a + b` → `a - b`
   - Kill Strategy: Test with values where operators differ

### 3. Complete Configuration

#### `.cargo/mutants.toml`
```toml
timeout = 300  # 5 minutes (transpiler tests can be slow)

exclude_globs = [
    "**/tests/**",
    "**/*_test.rs",
    "**/examples/**",
]

crates = [
    "depyler-core",      # HIGHEST PRIORITY
    "depyler-analyzer",
    "depyler",
    "depyler-verify",
]

skip_crates = [
    "depyler-mcp",       # External API
    "depyler-ruchy",     # Separate project
]

test_args = ["--release"]
test_threads = 0  # Auto-detect CPU count
```

#### GitHub Actions Workflow
- Two parallel jobs: `mutation-test-core` and `mutation-test-analyzer`
- ≥90% mutation score enforcement (fails CI if below target)
- Weekly full runs + PR runs on critical paths
- Mutation report artifacts uploaded

### 4. EXTREME TDD Integration

Mutation testing fits perfectly into existing workflow:

```bash
# STEP 1: Baseline check
pmat tdg . --min-grade A- --format=table
cargo mutants --list --file src/target_file.rs

# STEP 2-4: Write tests FIRST, implement, verify (existing TDD)

# STEP 5: Run mutation testing on new code
cargo mutants --file src/target_file.rs

# STEP 6: If mutations survive, strengthen tests
# (Add specific test to kill mutation)

# STEP 7: Re-run until ≥90% killed
cargo mutants --file src/target_file.rs

# STEP 8: Full quality gate (existing)
pmat tdg . --min-grade A- --fail-on-violation
```

### 5. Performance Optimization

Strategies for handling Depyler's large test suite (596+ tests):

- **Parallel Execution**: `--test-threads=0` (auto-detect cores)
- **Incremental Testing**: Only changed files in PRs
- **Baseline Filtering**: Exclude tests, examples, generated code
- **Timeout Tuning**: 30s for unit tests, 300s for integration
- **Caching**: sccache for faster compilation

### 6. Quality Framework Integration

Mutation testing as the 5th quality pillar:

| Metric | Current Target | Mutation Testing Impact |
|--------|---------------|------------------------|
| TDG Score | A+ (≥95/100) | Validates quality is real |
| Test Coverage | ≥80% | Proves coverage is effective |
| Complexity | ≤10 per function | Lower complexity = easier to test |
| SATD | 0 violations | No technical debt hiding test gaps |
| **Mutation Score** | **≥90%** | **Ultimate test validation** |

---

## Roadmap Impact

### Sprint 5 Created with 4 Tickets

**DEPYLER-0020**: Mutation Testing Infrastructure Setup ✅
- Status: **COMPLETED** (this session)
- Deliverable: Comprehensive specification

**DEPYLER-0021**: Achieve 90% Mutation Score - Core Transpilation
- Status: **PENDING**
- Time: 16-24h (EXTREME TDD)
- Target: depyler-core crate
- Focus: AST conversion, code generation, expression handling

**DEPYLER-0022**: Achieve 90% Mutation Score - Type Analysis
- Status: **PENDING**
- Time: 8-12h (EXTREME TDD)
- Target: depyler-analyzer crate
- Focus: Type inference, ownership analysis, lifetime tracking

**DEPYLER-0023**: Mutation Testing Documentation & Integration
- Status: **PENDING**
- Time: 2-4h
- Tasks: Developer guide, troubleshooting docs, quality dashboard update

**Total Sprint 5 Estimate**: 2-3 weeks

---

## Files Modified

### Created:
- `docs/specifications/mutant.md` (NEW - 23KB)

### Updated:
- `docs/execution/roadmap.md` - Added Sprint 5 section
- `CHANGELOG.md` - Added DEPYLER-0020 entry

### Commits:
1. `999b969` - [SPEC] Add comprehensive mutation testing specification
2. `359f010` - docs: Update roadmap and changelog for DEPYLER-0020 completion

**All changes pushed to GitHub** ✅

---

## Specification Highlights

### Target: ≥90% Mutation Kill Rate

```
Mutation Score = (Killed Mutants / Total Mutants) × 100%

Depyler target: ≥ 90%
```

**Why 90%, not 100%?**
- 90%: ✅ Excellent test quality (TARGET)
- 95%: ⚠️ Very good, but significant effort
- 100%: ❌ Not worth the effort (acceptable mutations exist)

### Acceptable Mutations (Examples)

Some mutations are OK to miss:

1. **Logging Statements**: Tests shouldn't depend on logs
2. **Performance Optimizations**: Result same, just slower
3. **Error Messages**: Tests check variant, not message text
4. **Debug Assertions**: Development aids, not production logic

### Real-World Example from Specification

```rust
// Original: Type inference
fn infer_type(&mut self, expr: &HirExpr) -> Type {
    match expr {
        HirExpr::Literal(lit) => match lit {
            Literal::Int(_) => Type::I32,
            Literal::Float(_) => Type::F64,
            Literal::Str(_) => Type::String,
            Literal::Bool(_) => Type::Bool,
        }
        _ => Type::Unknown
    }
}

// Mutation: Replace Type::I32 with Type::I64
// Mutation: Replace Type::String with Type::Unit
// Mutation: Delete match arms (return Type::Unknown always)

// Test that KILLS these mutations:
#[test]
fn test_literal_type_inference_exact_types() {
    assert_eq!(infer_type(int_42), Type::I32);      // Not I64 ✅
    assert_eq!(infer_type(str_hello), Type::String); // Not Unit ✅
    assert_eq!(infer_type(float_3_14), Type::F64);
    assert_eq!(infer_type(bool_true), Type::Bool);
}
```

---

## Best Practices Documented

### 1. Run Regularly, Not Every Commit

```bash
# Local: Manual runs on changed files
cargo mutants --file src/my_changes.rs

# CI: Weekly full runs
schedule:
  - cron: '0 0 * * 0'  # Sunday

# CI: PR runs on critical crates only
```

### 2. Focus on Critical Code

**Critical Priority**:
- `depyler-core/src/codegen.rs` - Code generation (bugs ship to users)
- `depyler-core/src/ast_bridge.rs` - AST conversion (correctness critical)
- `depyler-analyzer/src/type_flow.rs` - Type inference (safety critical)

**Medium Priority**:
- `depyler/src/lib.rs` - CLI and Lambda conversion
- `depyler-verify/src/contracts.rs` - Contract verification

**Low Priority**:
- `depyler-mcp/src/server.rs` - MCP protocol (external API)
- `depyler-ruchy/src/interpreter.rs` - Ruchy runtime (separate project)

### 3. Track Metrics Over Time

```bash
# Save mutation scores
cargo mutants --json > reports/mutation-$(date +%Y%m%d).json

# Compare over time
jq -r '.mutation_score' reports/mutation-*.json |
    awk '{sum+=$1; count++} END {print "Avg:", sum/count "%"}'
```

### 4. Makefile Integration

```makefile
mutants:           # Full run (slow, use for releases)
mutants-core:      # Core crates only (regular development)
mutants-quick:     # Changed files only (TDD workflow)
mutants-report:    # Generate readable report
```

---

## Key Insights from pforge Methodology

### What We Adapted

1. **Target Metrics**: ≥90% mutation kill rate (proven effective in pforge)
2. **Mutation Operators**: Same 5 categories work for transpilers
3. **CI/CD Strategy**: Weekly full + PR critical paths
4. **Performance Tricks**: Parallel execution, incremental testing
5. **Best Practices**: 90% target, focus on critical code, track metrics

### What We Customized for Depyler

1. **Testing Focus**: Transpilation correctness vs generic Rust code
2. **Critical Areas**: AST conversion, type inference, code generation
3. **Test Strategies**: Semantic validation (does generated Rust compile?)
4. **Crate Priorities**: depyler-core > depyler-analyzer > rest
5. **Acceptable Mutations**: Transpiler-specific (e.g., intermediate representations)

---

## Next Steps (Sprint 5 Execution)

### Immediate (Next Session)

**Option 1**: Begin DEPYLER-0021 Implementation
```bash
# Install cargo-mutants
cargo install cargo-mutants --locked

# Run baseline on depyler-core
cargo mutants -p depyler-core --list

# Identify missed mutations
cargo mutants -p depyler-core --json > baseline.json

# Begin EXTREME TDD to kill mutations
```

**Option 2**: Address Security Vulnerabilities First
- 2 Dependabot alerts (1 critical, 1 moderate)
- Time: 30 minutes - 1 hour
- Then proceed to DEPYLER-0021

**Option 3**: Continue Sprint 4 Remaining Tickets
- DEPYLER-0012: stmt_to_rust_tokens_with_scope (complexity 25→≤10)
- DEPYLER-0013: lambda_test_command (complexity 18→≤10)

### Short-term (Next Week)

1. **Complete DEPYLER-0021**: 90% mutation score on depyler-core
2. **Complete DEPYLER-0022**: 90% mutation score on depyler-analyzer
3. **Complete DEPYLER-0023**: Documentation and integration

### Medium-term (Next Month)

1. **Maintain**: Weekly mutation testing in CI
2. **Track**: Mutation score trends over time
3. **Improve**: Add tests for any new surviving mutations

---

## Quality Status

### Current Metrics
```
TDG Score:        99.1/100 (A+)    ✅ Excellent
Max Complexity:   20               🟡 Target: ≤10
SATD Violations:  0                ✅ Zero tolerance achieved
Test Count:       596+             ✅ Growing
Coverage:         70.16%           🟡 Target: 80%
Mutation Score:   TBD              🎯 Target: 90% (new)
```

### Sprint 4 Summary
- **Completed**: 2/6 tickets (DEPYLER-0011, DEPYLER-0015)
- **Time**: ~3.5 hours actual vs ~12 hours estimated (71% savings)
- **Quality**: TDG A+ maintained, zero SATD achieved
- **Complexity**: Max reduced from 31→20 (35% improvement)

### Sprint 5 Status
- **Planned**: 4 tickets (DEPYLER-0020 through DEPYLER-0023)
- **Completed**: 1/4 (DEPYLER-0020 - this session)
- **Remaining**: 3 tickets (26-40h estimated, ~8-12h via EXTREME TDD)

---

## References

### Created Documentation
- **Primary**: `docs/specifications/mutant.md` (23KB specification)
- **Roadmap**: `docs/execution/roadmap.md` (Sprint 5 added)
- **Changelog**: `CHANGELOG.md` (DEPYLER-0020 entry)

### Source Material
- **pforge**: `../pforge/pforge-book/src/ch09-04-mutation-testing.md`
- **cargo-mutants**: https://mutants.rs/

### Related Work
- **Sprint 4**: `SPRINT-4-SESSION-SUMMARY.md`
- **Overall Status**: `FINAL_STATUS_REPORT.md`
- **Project Context**: `CLAUDE.md`, `deep_context.md`

---

## Conclusion

DEPYLER-0020 successfully completed with production-ready mutation testing specification that:

1. **Adapts proven methodology** from pforge for transpilation domain
2. **Provides concrete strategies** for killing mutations in critical areas
3. **Integrates seamlessly** with existing EXTREME TDD workflow
4. **Sets clear path** for achieving ≥90% mutation kill rate
5. **Defines 3 implementation tickets** with realistic time estimates

**Project Health**: ✅ Excellent (TDG A+, Zero SATD, Specification Complete)

**Recommended Next Action**: Begin DEPYLER-0021 (Core Transpilation Mutation Testing) or address security vulnerabilities first.

---

**Prepared by**: Claude Code
**Date**: 2025-10-03
**Session Type**: Specification Development
**Next Update**: After DEPYLER-0021 completion
