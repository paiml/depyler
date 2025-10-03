# DEPYLER-0021: Baseline Mutation Testing Analysis

**Date**: 2025-10-03
**Target**: depyler-core package
**Tool**: cargo-mutants v25.3.1
**Goal**: Achieve ≥90% mutation kill rate

---

## Configuration

### `.cargo/mutants.toml`
```toml
timeout_multiplier = 5.0
minimum_test_timeout = 120
exclude_globs = ["**/tests/**", "**/*_test.rs", "**/examples/**"]
additional_cargo_test_args = ["--release"]
```

### Baseline Test Strategy

Starting with `ast_bridge.rs` as it's critical for transpilation correctness:
- **File**: `crates/depyler-core/src/ast_bridge.rs`
- **Mutations Found**: 164
- **Criticality**: HIGH (Python AST → HIR conversion)

---

## Baseline Results

### ast_bridge.rs (Disk Space Limitation)

**Command**:
```bash
cargo mutants --file crates/depyler-core/src/ast_bridge.rs --jobs 8 --timeout 120 --json
```

**Status**: ❌ FAILED - Disk space exhausted

**Issue**: `/tmp` directory ran out of space (13G/16G used)
- cargo-mutants creates separate build for each mutation
- With `--release` flag + 8 parallel jobs, space requirements exceeded available
- Error: "No space left on device (os error 28)"

**Root Cause**:
```
tmpfs            16G   13G  3.4G  79% /tmp
```

164 mutations × 8 parallel builds × ~200MB/build ≈ 25GB required
Available: 3.4GB

### Initial Observations

Sample mutations being tested:
1. `replace AstBridge::python_to_hir -> Result<HirModule> with Ok(Default::default())`
2. `delete match arm ast::Mod::Module(m) in AstBridge::python_to_hir`
3. `delete match arm ast::Stmt::FunctionDef(f) in AstBridge::convert_module`
4. `replace != with == in AstBridge::try_convert_type_alias`
5. `delete ! in AstBridge::try_convert_protocol`

These are exactly the types of mutations our specification predicted would be critical.

---

## Full Package Scope

### depyler-core Total Mutations

```bash
cargo mutants --list -p depyler-core | wc -l
# Result: 2,714 mutations
```

**Files with Most Mutations** (estimated from list output):
1. `codegen.rs` - Code generation (likely 500+ mutations)
2. `ast_bridge.rs` - AST conversion (164 mutations confirmed)
3. `direct_rules.rs` - Expression/statement conversion (likely 400+ mutations)
4. `rust_gen.rs` - Rust type generation (likely 300+ mutations)
5. Various helpers and utilities

### Prioritization Strategy

**Phase 1**: Critical Files (Week 1)
- [x] ast_bridge.rs (164 mutations) - IN PROGRESS
- [ ] codegen.rs
- [ ] direct_rules.rs

**Phase 2**: Type System (Week 2)
- [ ] rust_gen.rs
- [ ] type_hints.rs
- [ ] annotation_aware_type_mapper.rs

**Phase 3**: Full Package (Week 3)
- [ ] Remaining files
- [ ] Integration testing
- [ ] Final ≥90% validation

---

## Results to Document

### For Each File/Phase:
1. **Baseline Metrics**:
   - Total mutations tested
   - Caught mutations (count & %)
   - Missed mutations (count & %)
   - Timeout mutations (count & %)

2. **Analysis**:
   - Which mutations survived?
   - Why did they survive?
   - What tests are missing?

3. **Action Plan**:
   - Tests to write (EXTREME TDD)
   - Expected kill rate improvement
   - Time estimate

---

## Next Steps

1. ✅ Install cargo-mutants
2. ✅ Configure .cargo/mutants.toml
3. 🔄 Run baseline on ast_bridge.rs
4. ⏳ Analyze results
5. ⏳ Write tests to kill missed mutations
6. ⏳ Re-run until ≥90%
7. ⏳ Expand to next critical file

---

**Status**: Baseline test running
**Updated**: 2025-10-03 (live document)
