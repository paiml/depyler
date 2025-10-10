# DEPYLER-0140: Refactor HirStmt::to_rust_tokens

**Priority**: P0 (Critical Technical Debt)
**File**: `crates/depyler-core/src/rust_gen.rs:1165`
**Current Complexity**: Cyclomatic 129, Cognitive 296
**Target**: ≤10 cyclomatic, ≤10 cognitive
**Effort**: ~80 hours
**Status**: PLANNED

## Problem Analysis

The `HirStmt::to_rust_tokens` function is **2678 lines** long and handles 12 different statement types in a single massive match expression. This creates:

- **Unmaintainability**: Function is too large to understand
- **Untestability**: Cannot unit test individual statement handlers
- **Complexity**: Cyclomatic 129 (12.9x over limit)
- **Cognitive Load**: 296 cognitive complexity (29.6x over limit)

## Statement Types (12 total)

1. `HirStmt::Assign` - Variable/index/attribute assignment (~300 lines)
2. `HirStmt::Return` - Return statements (~50 lines)
3. `HirStmt::If` - If/elif/else logic (~200 lines)
4. `HirStmt::While` - While loops (~100 lines)
5. `HirStmt::For` - For loops (~250 lines)
6. `HirStmt::Expr` - Expression statements (~30 lines)
7. `HirStmt::Raise` - Exception raising (~100 lines)
8. `HirStmt::Break` - Break statements (~20 lines)
9. `HirStmt::Continue` - Continue statements (~20 lines)
10. `HirStmt::With` - Context managers (~150 lines)
11. `HirStmt::Try` - Try/except blocks (~400 lines)
12. `HirStmt::Pass` - No-op (~10 lines)

## Refactoring Strategy

### Phase 1: Extract Simple Handlers (16 hours)
Extract the simplest 4 statement types into separate functions:

```rust
// BEFORE (current):
impl RustCodeGen for HirStmt {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<TokenStream> {
        match self {
            HirStmt::Pass => Ok(quote! {}),
            HirStmt::Break { .. } => { /* 20 lines */ },
            HirStmt::Continue { .. } => { /* 20 lines */ },
            HirStmt::Expr(expr) => { /* 30 lines */ },
            // ... 8 more massive cases
        }
    }
}

// AFTER (target):
impl RustCodeGen for HirStmt {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<TokenStream> {
        match self {
            HirStmt::Pass => codegen_pass_stmt(),
            HirStmt::Break { label } => codegen_break_stmt(label),
            HirStmt::Continue { label } => codegen_continue_stmt(label),
            HirStmt::Expr(expr) => codegen_expr_stmt(expr, ctx),
            HirStmt::Assign { .. } => codegen_assign_stmt(self, ctx),
            HirStmt::Return(expr) => codegen_return_stmt(expr, ctx),
            HirStmt::If { .. } => codegen_if_stmt(self, ctx),
            HirStmt::While { .. } => codegen_while_stmt(self, ctx),
            HirStmt::For { .. } => codegen_for_stmt(self, ctx),
            HirStmt::Raise { .. } => codegen_raise_stmt(self, ctx),
            HirStmt::With { .. } => codegen_with_stmt(self, ctx),
            HirStmt::Try { .. } => codegen_try_stmt(self, ctx),
        }
    }
}

// New helper functions (each ≤30 lines, complexity ≤10)
fn codegen_pass_stmt() -> Result<TokenStream> {
    Ok(quote! {})
}

fn codegen_break_stmt(label: &Option<String>) -> Result<TokenStream> {
    // Implementation extracted from original
}

fn codegen_continue_stmt(label: &Option<String>) -> Result<TokenStream> {
    // Implementation extracted from original
}

fn codegen_expr_stmt(expr: &HirExpr, ctx: &mut CodeGenContext) -> Result<TokenStream> {
    // Implementation extracted from original
}
```

### Phase 2: Extract Medium Handlers (24 hours)
Extract the medium-complexity handlers:

```rust
fn codegen_return_stmt(expr: &Option<Box<HirExpr>>, ctx: &mut CodeGenContext) -> Result<TokenStream>

fn codegen_while_stmt(condition: &HirExpr, body: &[HirStmt], ctx: &mut CodeGenContext) -> Result<TokenStream>

fn codegen_raise_stmt(exc: &Option<Box<HirExpr>>, ctx: &mut CodeGenContext) -> Result<TokenStream>

fn codegen_with_stmt(items: &[WithItem], body: &[HirStmt], ctx: &mut CodeGenContext) -> Result<TokenStream>
```

### Phase 3: Extract Complex Handlers (40 hours)
The hard ones - each needs decomposition:

#### 3a. Assign Statement (~300 lines → 4 functions)
```rust
fn codegen_assign_stmt(stmt: &HirStmt, ctx: &mut CodeGenContext) -> Result<TokenStream> {
    match extract_assign_parts(stmt) {
        (AssignTarget::Symbol(s), value, ann) => codegen_assign_symbol(s, value, ann, ctx),
        (AssignTarget::Index { .. }, value, ann) => codegen_assign_index(base, index, value, ctx),
        (AssignTarget::Attribute { .. }, value, ann) => codegen_assign_attribute(obj, attr, value, ctx),
        (AssignTarget::Tuple(targets), value, ann) => codegen_assign_tuple(targets, value, ctx),
    }
}

fn codegen_assign_symbol(...) -> Result<TokenStream> { /* ≤50 lines, complexity ≤8 */ }
fn codegen_assign_index(...) -> Result<TokenStream> { /* ≤50 lines, complexity ≤8 */ }
fn codegen_assign_attribute(...) -> Result<TokenStream> { /* ≤30 lines, complexity ≤5 */ }
fn codegen_assign_tuple(...) -> Result<TokenStream> { /* ≤70 lines, complexity ≤10 */ }
```

#### 3b. If Statement (~200 lines → 3 functions)
```rust
fn codegen_if_stmt(stmt: &HirStmt, ctx: &mut CodeGenContext) -> Result<TokenStream> {
    // Dispatch to helpers
}

fn codegen_if_condition(cond: &HirExpr, ctx: &mut CodeGenContext) -> Result<TokenStream>
fn codegen_if_chain(elif_blocks: &[...], ctx: &mut CodeGenContext) -> Result<TokenStream>
fn codegen_if_else(else_block: &[HirStmt], ctx: &mut CodeGenContext) -> Result<TokenStream>
```

#### 3c. For Statement (~250 lines → 4 functions)
```rust
fn codegen_for_stmt(...) -> Result<TokenStream>
fn codegen_for_range(...) -> Result<TokenStream>  // for x in range(n)
fn codegen_for_iterator(...) -> Result<TokenStream>  // for x in iter
fn codegen_for_unpacking(...) -> Result<TokenStream>  // for (a, b) in pairs
```

#### 3d. Try Statement (~400 lines → 5 functions)
```rust
fn codegen_try_stmt(...) -> Result<TokenStream>
fn codegen_try_handlers(...) -> Result<TokenStream>  // except clauses
fn codegen_try_finally(...) -> Result<TokenStream>  // finally block
fn codegen_exception_matching(...) -> Result<TokenStream>  // match exception types
fn codegen_exception_binding(...) -> Result<TokenStream>  // as e syntax
```

## Implementation Plan

### Week 1-2: Simple Handlers (16h)
- [ ] Extract Pass, Break, Continue, Expr
- [ ] Add unit tests for each
- [ ] Verify all existing tests pass
- [ ] Commit: "DEPYLER-0140: Extract simple statement handlers (4/12)"

### Week 3-4: Medium Handlers (24h)
- [ ] Extract Return, While, Raise, With
- [ ] Add unit tests for each
- [ ] Verify all existing tests pass
- [ ] Commit: "DEPYLER-0140: Extract medium statement handlers (8/12)"

### Week 5-8: Complex Handlers (40h)
- [ ] Week 5: Refactor Assign (4 sub-functions)
- [ ] Week 6: Refactor If (3 sub-functions)
- [ ] Week 7: Refactor For (4 sub-functions)
- [ ] Week 8: Refactor Try (5 sub-functions)
- [ ] Commit: "DEPYLER-0140: Complete statement handler extraction (12/12)"

### Week 9: Validation & Cleanup (8h)
- [ ] Run `pmat analyze complexity` - verify all ≤10
- [ ] Run full test suite - ensure 100% pass rate
- [ ] Run benchmarks - ensure no performance regression
- [ ] Update documentation
- [ ] Final commit: "DEPYLER-0140: Refactoring complete - complexity 129→7"

## Success Criteria

- ✅ Main `to_rust_tokens` function: cyclomatic ≤10 (target: ~7)
- ✅ All extracted functions: cyclomatic ≤10
- ✅ All extracted functions: cognitive ≤10
- ✅ All extracted functions: ≤50 lines
- ✅ 100% test pass rate maintained throughout
- ✅ Zero performance regression
- ✅ PMAT quality gate passes

## Testing Strategy

For each extracted function:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_pass_stmt() {
        let result = codegen_pass_stmt().unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_codegen_break_stmt_simple() {
        let result = codegen_break_stmt(&None).unwrap();
        assert_eq!(result.to_string(), "break");
    }

    #[test]
    fn test_codegen_break_stmt_with_label() {
        let result = codegen_break_stmt(&Some("outer".to_string())).unwrap();
        assert_eq!(result.to_string(), "break 'outer");
    }

    // ... comprehensive tests for each handler
}
```

## Risks & Mitigation

**Risk 1**: Breaking existing functionality
**Mitigation**: Extract one function at a time, run full test suite after each

**Risk 2**: Performance regression due to function call overhead
**Mitigation**: Mark helpers with `#[inline]`, run benchmarks

**Risk 3**: Time overrun (80 hours is aggressive)
**Mitigation**: Prioritize simple/medium handlers first, defer Try/For if needed

**Risk 4**: Incomplete test coverage masking bugs
**Mitigation**: Add property tests and integration tests before refactoring

## Dependencies

None - this is pure refactoring with no external dependencies.

## Notes

- This refactoring will reduce the file from 3843 lines to ~2500 lines
- The extracted functions can be reused if we add more code generation backends
- Future refactorings can apply the same pattern to `HirFunction::to_rust_tokens` (complexity 106)

---

**Last Updated**: 2025-10-10
**Status**: PLANNED - Awaiting sprint allocation
