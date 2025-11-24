# DEPYLER-REFACTOR-001: expr_gen.rs God File Split

**Status**: Planned
**Priority**: P1-High (Architectural Risk)
**Estimated Effort**: 2-3 weeks
**Quality Standard**: EXTREME TDD (bashrs-level)

## Problem Statement

`crates/depyler-core/src/rust_gen/expr_gen.rs` is a **12,772-line God File** with:
- **158 functions** in a single file
- **59 type conversion functions** (bool, int, float, str, range, array, set, dict, list, etc.)
- Touched in almost every commit → high merge conflict risk
- Low cohesion, high coupling

## Quality Requirements (NON-NEGOTIABLE)

### EXTREME TDD Protocol
1. ✅ **Test First** - Write comprehensive tests BEFORE refactoring
2. ✅ **TDG Quality** - All new modules ≤10 complexity (A+ grade)
3. ✅ **95% Coverage Minimum** (not 80% - this is critical refactoring)
4. ✅ **Mutation Testing** - All new code validated with `pmat analyze mutate`
5. ✅ **Property-Based Testing** - Use proptest for invariants
6. ✅ **Zero Regressions** - All 545+ unit tests must pass at each step

### Quality Gates (BLOCKING)
```bash
# MANDATORY before each commit during refactor:
cargo test --workspace                          # 100% pass
cargo clippy --all-targets -- -D warnings       # ZERO warnings
cargo llvm-cov --all-features --fail-under-lines 95  # ≥95%
pmat tdg check-quality --min-grade A-          # TDG ≤2.0
pmat analyze mutate <new-module.rs>            # Mutation coverage
cargo test --doc                                # Doctest coverage
```

## Refactoring Strategy

### Phase 1: Establish Golden Test Harness (Week 1)
**Goal**: Create comprehensive test coverage BEFORE touching code

#### 1.1 Extract Current Behavior (Day 1-2)
```bash
# Create baseline test suite that captures ALL current behavior
cargo test -p depyler-core --lib > baseline_tests.txt

# Property tests for expression generation invariants:
# - All generated Rust code must compile
# - All generated code must pass clippy
# - Semantically equivalent Python/Rust behavior (Golden Trace)
```

**Deliverable**:
- `tests/refactor_expr_gen_baseline.rs` - Comprehensive test suite
- 95% coverage of expr_gen.rs functionality
- Property tests for invariants
- Mutation tests showing ≥80% kill rate

#### 1.2 Create Module Extraction Tests (Day 3-4)
For each target module, create tests FIRST:

```rust
// tests/refactor_builtin_conversions_test.rs
#[test]
fn test_int_conversion_preserves_behavior() {
    // Test that split module has identical behavior
}

#[proptest]
fn prop_type_conversions_compile(input: PythonExpr) {
    // Property: All type conversions generate valid Rust
}
```

**Acceptance Criteria**:
- ✅ Test suite for each target module written FIRST
- ✅ Tests fail (module doesn't exist yet - RED phase)
- ✅ Tests cover edge cases, error paths, success paths
- ✅ Property tests for invariants

### Phase 2: Extract Modules (Week 2-3)
**Method**: RED-GREEN-REFACTOR for EACH module

#### Module Breakdown

**2.1 `builtin_conversions.rs` (~1,500 lines)**
- `convert_int_cast`, `convert_float_cast`, `convert_str_conversion`
- `convert_bool_cast`, `convert_len_call`
- All `convert_X_builtin` functions

**Tests FIRST**:
```bash
# Day 1: Write tests (RED)
cargo test test_builtin_conversions_baseline  # MUST FAIL

# Day 2: Extract module (GREEN)
# Move functions, adjust visibility, fix imports
cargo test test_builtin_conversions_baseline  # MUST PASS

# Day 3: Quality gates (REFACTOR)
pmat tdg builtin_conversions.rs --min-grade A-  # ≤10 complexity
cargo llvm-cov --lib -- builtin_conversions     # ≥95% coverage
pmat analyze mutate builtin_conversions.rs      # Mutation tests
```

**2.2 `collection_constructors.rs` (~1,200 lines)**
- `convert_set_constructor`, `convert_frozenset_constructor`
- `convert_dict_builtin`, `convert_list_builtin`
- `convert_counter_builtin`, `convert_deque_builtin`

**2.3 `array_initialization.rs` (~800 lines)**
- `convert_array_init_call`, `convert_array_small_literal`
- `convert_array_large_literal`, `convert_array_dynamic_size`

**2.4 `range_expressions.rs` (~600 lines)**
- `convert_range_call`, `convert_range_with_step`
- `convert_range_negative_step`, `convert_range_positive_step`

**2.5 `operators.rs` (~1,000 lines)**
- `convert_binary`, `convert_unary`
- All operator-related logic

**2.6 `call_resolution.rs` (~1,500 lines)**
- `convert_call`, `try_convert_map_with_zip`
- Function call resolution logic

**Remaining in `expr_gen.rs`**: ~6,000 lines (core expression conversion)

### Phase 3: Validation & Documentation (Final Days)

#### 3.1 Full Test Suite Validation
```bash
# Run ENTIRE test suite
cargo test --workspace

# Coverage must be ≥95%
cargo llvm-cov --all-features --workspace --fail-under-lines 95

# All new modules TDG A+ grade
pmat tdg crates/depyler-core/src/rust_gen/builtin_conversions.rs --min-grade A-
pmat tdg crates/depyler-core/src/rust_gen/collection_constructors.rs --min-grade A-
# ... for all new modules
```

#### 3.2 Performance Regression Check
```bash
# Use renacer to profile transpilation performance
./scripts/profile_transpiler.sh examples/benchmark.py > baseline_perf.txt

# After refactor, compare
./scripts/profile_transpiler.sh examples/benchmark.py > refactor_perf.txt
diff baseline_perf.txt refactor_perf.txt

# MUST NOT REGRESS: Performance within 5% of baseline
```

#### 3.3 Documentation
- Update architecture docs with new module structure
- Add comprehensive rustdoc for each new module
- Update CLAUDE.md with new structure

## Commit Strategy

Each module extraction is ONE atomic commit:

```bash
git commit -m "[REFACTOR] DEPYLER-REFACTOR-001: Extract builtin_conversions module (Refs DEPYLER-REFACTOR-001)

Phase 2.1: Extract type conversion builtins to separate module

Extracted Functions (15):
- convert_int_cast, convert_float_cast, convert_str_conversion
- convert_bool_cast, convert_len_call
- ... (list all)

Quality Metrics:
- TDG Score: 0.8 (A+) ✅
- Complexity: Max 8 (≤10) ✅
- Coverage: 96.2% (≥95%) ✅
- Mutation Kill Rate: 82% ✅
- All 545 tests passing ✅
- Zero regressions ✅

Test Results:
  test result: ok. 563 passed; 0 failed; 0 ignored

Reduces expr_gen.rs from 12,772 → 11,272 lines (-11.7%)

🤖 Generated with Claude Code

Co-Authored-By: Claude <noreply@anthropic.com>"
```

## Risk Mitigation

### Risk 1: Breaking Changes
**Mitigation**:
- Test-first approach ensures behavior preservation
- Golden Trace validation for semantic equivalence
- Each commit is atomic and revertible

### Risk 2: Performance Regression
**Mitigation**:
- Profile before/after with renacer
- Benchmark transpilation time for all examples
- Performance budgets enforced in tests

### Risk 3: Merge Conflicts During Refactor
**Mitigation**:
- Do refactor in dedicated branch
- Small, frequent commits
- Rebase daily from main
- Complete refactor in 2-3 weeks maximum

## Success Criteria

✅ **All modules ≤2,500 lines** (50% reduction from 12,772)
✅ **All new modules TDG A+ grade** (≤10 complexity)
✅ **95% test coverage maintained** (not 80%)
✅ **Zero test regressions** (all 545+ tests pass)
✅ **Mutation coverage ≥80%** (validated with pmat)
✅ **Performance within 5%** of baseline
✅ **Property tests validate invariants**
✅ **Comprehensive rustdoc** for all public APIs

## Timeline

- **Week 1**: Golden test harness (95% coverage, property tests, mutation tests)
- **Week 2**: Extract 3 modules (builtin_conversions, collection_constructors, array_initialization)
- **Week 3**: Extract 3 modules (range_expressions, operators, call_resolution)
- **Final Days**: Validation, documentation, performance verification

## References

- Thrashing Audit Report: `/home/noah/src/depyler/trashing-report.md`
- Similar Refactoring: `../bashrs` (EXTREME quality standard)
- Quality Enforcement: `CLAUDE.md` A+ Code Standard section
- Mutation Testing: Sprint 61 (pmat analyze mutate)

---

**REMEMBER**: Test FIRST, quality ALWAYS, zero compromises on the 95% coverage requirement.
