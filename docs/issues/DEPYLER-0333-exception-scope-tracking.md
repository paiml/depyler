# DEPYLER-0333: Exception Scope Tracking Architecture

**Date**: 2025-11-04
**Status**: 🚧 IN PROGRESS - EXTREME TDD Protocol
**Priority**: P0 (Blocks 05_error_handling completion)
**Estimate**: 4-6 hours (430 LOC)
**Complexity**: HIGH (architectural change)

## Executive Summary

The transpiler currently lacks the ability to track whether code is executing inside a try/except block. This causes 3 compilation errors in the Matrix Project's 05_error_handling example:

1. **E0599** (1 error): Spurious `.unwrap()` on non-Result types
2. **E0308** (2 errors): Result<T> returned when T expected in caught exceptions

**Impact**: Once resolved, Matrix Project pass rate increases from 75% → 83% (9/12 → 10/12)

## Problem Statement

### Issue #1: Spurious .unwrap() on Non-Result Types

**Python Code**:
```python
def process_data(data: str) -> int:
    try:
        result = int(data)  # Can raise ValueError
        return result.upper()  # ❌ ERROR: trying to call .upper() on int
    except ValueError:
        return -1
```

**Current Generated Rust** (WRONG):
```rust
pub fn process_data(data: &str) -> Result<i32, ValueError> {
    let result = data.parse::<i32>()?;  // Returns Result<i32, _>
    Ok(result.unwrap().to_uppercase())  // ❌ E0599: no method `unwrap` found for type `i32`
}
```

**Root Cause**: Transpiler sees `int(data)` can raise ValueError, so generates `Result<i32, ValueError>`. But then incorrectly adds `.unwrap()` on the `i32` value.

**Expected Rust**:
```rust
pub fn process_data(data: &str) -> Result<i32, ValueError> {
    let result = data.parse::<i32>()?;
    // No .unwrap() needed - result is already i32
    Ok(result)  // Simplified - actual code would fail differently
}
```

### Issue #2: Result<T> Returned When T Expected in Try/Except

**Python Code**:
```python
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b  # Can raise ZeroDivisionError
    except ZeroDivisionError:
        return 0  # Exception is caught internally
```

**Current Generated Rust** (WRONG):
```rust
pub fn safe_divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        return Err(ZeroDivisionError::new("division by zero"));  // ❌ E0308: expected `i32`, found `Result<_, ZeroDivisionError>`
    }
    a / b
}
```

**Root Cause**: Transpiler sees `a // b` can raise, generates exception handling, but doesn't track that the exception is caught internally. Function returns `i32`, not `Result<i32, _>`.

**Expected Rust**:
```rust
pub fn safe_divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 0;  // ✅ Return fallback value, not Err
    }
    a / b
}
```

### Issue #3: raise Statements in Non-Result Functions

**Python Code**:
```python
def validate_positive(n: int) -> int:
    if n < 0:
        raise ValueError("must be positive")
    return n * 2
```

**Current Generated Rust** (WRONG):
```rust
pub fn validate_positive(n: i32) -> i32 {
    if n < 0 {
        return Err(ValueError::new("must be positive"));  // ❌ E0308: expected `i32`, found `Result<_, ValueError>`
    }
    n * 2
}
```

**Root Cause**: `raise` always generates `return Err()`, even in functions that don't return Result.

**Expected Rust** (Option 1 - Panic):
```rust
pub fn validate_positive(n: i32) -> i32 {
    if n < 0 {
        panic!("ValueError: must be positive");  // ✅ Panic in non-Result functions
    }
    n * 2
}
```

**Expected Rust** (Option 2 - Change signature):
```rust
pub fn validate_positive(n: i32) -> Result<i32, ValueError> {  // ✅ Change return type
    if n < 0 {
        return Err(ValueError::new("must be positive"));
    }
    Ok(n * 2)
}
```

## Proposed Solution: ExceptionScope Tracking

### HIR Enhancement

**Add ExceptionScope to HIR context**:

```rust
// crates/depyler-core/src/hir.rs

#[derive(Debug, Clone, PartialEq)]
pub enum ExceptionScope {
    /// Code outside any try/except block - exceptions propagate to caller
    Unhandled,

    /// Code inside try block - exceptions are caught
    /// Contains list of exception types that are caught by handlers
    TryCaught {
        handled_types: Vec<String>,  // ["ValueError", "ZeroDivisionError"]
    },

    /// Code inside except/finally block - exceptions may propagate
    Handler,
}

pub struct Context {
    // ... existing fields ...

    /// Stack of exception scopes (for nested try blocks)
    pub exception_scopes: Vec<ExceptionScope>,

    /// Whether current function can fail (needs Result return type)
    pub current_function_can_fail: bool,
}

impl Context {
    pub fn current_exception_scope(&self) -> &ExceptionScope {
        self.exception_scopes.last().unwrap_or(&ExceptionScope::Unhandled)
    }

    pub fn is_in_try_block(&self) -> bool {
        matches!(self.current_exception_scope(), ExceptionScope::TryCaught { .. })
    }

    pub fn is_exception_handled(&self, exception_type: &str) -> bool {
        if let ExceptionScope::TryCaught { handled_types } = self.current_exception_scope() {
            handled_types.contains(&exception_type.to_string())
        } else {
            false
        }
    }
}
```

### Property Analyzer Updates

**Track exception scopes during analysis**:

```rust
// crates/depyler-core/src/properties.rs

impl HirStmt {
    pub fn collect_exception_scopes(&self, ctx: &mut Context) -> Result<()> {
        match self {
            HirStmt::Try { body, handlers, .. } => {
                // Extract handled exception types from handlers
                let handled_types: Vec<String> = handlers
                    .iter()
                    .filter_map(|h| h.exception_type.clone())
                    .collect();

                // Push TryCaught scope
                ctx.exception_scopes.push(ExceptionScope::TryCaught { handled_types });

                // Analyze body statements
                for stmt in body {
                    stmt.collect_exception_scopes(ctx)?;
                }

                // Pop scope after try block
                ctx.exception_scopes.pop();

                // Analyze handlers (with Handler scope)
                for handler in handlers {
                    ctx.exception_scopes.push(ExceptionScope::Handler);
                    for stmt in &handler.body {
                        stmt.collect_exception_scopes(ctx)?;
                    }
                    ctx.exception_scopes.pop();
                }

                Ok(())
            }
            _ => {
                // Recursively analyze nested statements
                self.visit_nested_stmts(|stmt| stmt.collect_exception_scopes(ctx))
            }
        }
    }
}
```

### Codegen Updates

**Use exception scope in code generation**:

```rust
// crates/depyler-core/src/rust_gen/stmt_gen.rs

fn codegen_raise(&mut self, exception: &HirExpr) -> Result<syn::Stmt> {
    let exception_expr = exception.to_rust_expr(self.ctx)?;

    if self.ctx.current_function_can_fail {
        // Function returns Result - use return Err()
        Ok(parse_quote! {
            return Err(#exception_expr);
        })
    } else if self.ctx.is_in_try_block() {
        // Inside try block but function doesn't return Result
        // Convert to panic! (exceptions caught by outer try)
        Ok(parse_quote! {
            panic!("{}", #exception_expr);
        })
    } else {
        // Not in try block and function doesn't return Result
        // This should have been caught earlier - function signature needs Result
        bail!("raise statement in non-Result function outside try block")
    }
}

// crates/depyler-core/src/rust_gen/expr_gen.rs

fn convert_generic_call(&mut self, ...) -> Result<syn::Expr> {
    // ... existing code ...

    if function_can_raise {
        let exception_type = self.get_exception_type(func_name)?;

        if self.ctx.is_exception_handled(&exception_type) {
            // Exception is caught by surrounding try/except
            // Use .unwrap_or(default) instead of ?
            Ok(parse_quote! {
                #base_expr.unwrap_or_default()
            })
        } else if self.ctx.current_function_can_fail {
            // Exception propagates to caller - use ?
            Ok(parse_quote! {
                #base_expr?
            })
        } else {
            // Function doesn't handle exceptions - should panic or change signature
            Ok(parse_quote! {
                #base_expr.expect("Unhandled exception in non-Result function")
            })
        }
    }

    // ... rest of code ...
}
```

## Implementation Checklist

### Phase 1: RED - Write Comprehensive Failing Tests (1-2 hours)

- [ ] **Unit Tests** (crates/depyler-core/tests/depyler_0333_exception_scope_test.rs):
  - [ ] Test 1: Simple try/except with caught exception (safe_divide pattern)
  - [ ] Test 2: Nested try/except blocks with different exception types
  - [ ] Test 3: Try/except with finally block
  - [ ] Test 4: raise in try block (should NOT propagate Result if caught)
  - [ ] Test 5: raise outside try block (should panic! or change signature)
  - [ ] Test 6: Multiple exception types in handlers
  - [ ] Test 7: Bare except clause (catches all exceptions)
  - [ ] Test 8: Exception re-raising from handler
  - [ ] Test 9: Function call that can raise inside try block
  - [ ] Test 10: Mixed Result-returning and panic-on-error functions

- [ ] **Property Tests** (same file, #[cfg(test)] mod property_tests):
  - [ ] Property 1: All try blocks have matching exception scopes
  - [ ] Property 2: Exception scopes are properly nested (stack discipline)
  - [ ] Property 3: Caught exceptions never generate ? operator
  - [ ] Property 4: Unhandled exceptions in Result functions use ?
  - [ ] Property 5: raise statements in non-Result functions outside try blocks fail compilation

- [ ] **Integration Tests** (validate 05_error_handling example):
  - [ ] Transpile 05_error_handling/column_a/column_a.py
  - [ ] Compile generated Rust (should have 0 errors, currently 3)
  - [ ] Run generated tests

### Phase 2: GREEN - Minimal Implementation (2-3 hours)

- [ ] **HIR Enhancement** (crates/depyler-core/src/hir.rs):
  - [ ] Add ExceptionScope enum (Unhandled, TryCaught, Handler)
  - [ ] Add exception_scopes: Vec<ExceptionScope> to Context
  - [ ] Add current_function_can_fail: bool to Context
  - [ ] Add helper methods (current_exception_scope, is_in_try_block, is_exception_handled)

- [ ] **Property Analyzer** (crates/depyler-core/src/properties.rs):
  - [ ] Add collect_exception_scopes() method to HirStmt
  - [ ] Implement scope tracking for Try statements
  - [ ] Implement scope tracking for nested statements
  - [ ] Integrate into existing property analysis pipeline

- [ ] **Codegen Updates** (crates/depyler-core/src/rust_gen):
  - [ ] stmt_gen.rs: Update codegen_raise() to check exception scope
  - [ ] expr_gen.rs: Update convert_generic_call() for scope-aware error handling
  - [ ] func_gen.rs: Set current_function_can_fail based on function signature

- [ ] **Verify Tests Pass**:
  - [ ] Run unit tests - should all pass
  - [ ] Run integration tests - 05_error_handling should compile

### Phase 3: REFACTOR - Quality & Testing (1 hour)

- [ ] **Mutation Testing**:
  - [ ] Run cargo mutants on exception scope code
  - [ ] Target: ≥75% mutation kill rate
  - [ ] Add tests to kill surviving mutants

- [ ] **Fuzz Testing**:
  - [ ] Create fuzz target for exception scope analysis
  - [ ] Fuzz with random try/except structures
  - [ ] Run for 10 minutes, verify no panics

- [ ] **Code Quality**:
  - [ ] PMAT TDG: Ensure ≤2.0 (A- grade)
  - [ ] Complexity: All functions ≤10 (pmat analyze complexity)
  - [ ] SATD: Zero violations (pmat analyze satd)
  - [ ] Dead code: Zero warnings
  - [ ] Clippy: Zero warnings with -D warnings

- [ ] **Coverage**:
  - [ ] Run cargo llvm-cov
  - [ ] Verify ≥80% coverage on new code
  - [ ] Add tests for uncovered branches

### Phase 4: VALIDATE - Full Quality Gates (30 minutes)

- [ ] **Regression Testing**:
  - [ ] Run full test suite: cargo test --workspace
  - [ ] Verify 661+ tests passing (zero regressions)
  - [ ] Verify Matrix Project: 9/12 → 10/12 passing (75% → 83%)

- [ ] **Matrix Project Validation**:
  - [ ] Transpile 05_error_handling/column_a/column_a.py
  - [ ] Compile with rustc --deny warnings
  - [ ] Verify 0 compilation errors (was 3)

- [ ] **Performance**:
  - [ ] Benchmark transpilation time (should not increase >5%)
  - [ ] Profile memory usage (should not increase significantly)

- [ ] **Documentation**:
  - [ ] Update CHANGELOG.md with DEPYLER-0333 entry
  - [ ] Update roadmap.yaml with completion status
  - [ ] Document ExceptionScope API in code comments

## Success Criteria

✅ **Functional**:
- 05_error_handling example compiles with 0 errors (was 3)
- All 3 error patterns resolved (E0599, E0308 x2)
- Matrix Project pass rate: 75% → 83% (9/12 → 10/12)

✅ **Quality**:
- Test suite: 661+ passing (zero regressions)
- New tests: ≥15 comprehensive tests (10 unit + 5 property)
- Mutation score: ≥75% kill rate
- Coverage: ≥80% on new code
- TDG: A- grade (≤2.0)
- Complexity: ≤10 per function
- Clippy: Zero warnings

✅ **Performance**:
- Transpilation time: <5% increase
- Memory usage: <10% increase

## Risk Assessment

### High Risk

**Risk**: Breaking existing exception handling
- **Mitigation**: Comprehensive regression tests, test all existing try/except examples
- **Detection**: Full test suite must pass (661+ tests)

**Risk**: Complex nested try/except blocks
- **Mitigation**: Property tests for nested structures, fuzz testing
- **Detection**: Property test failures

### Medium Risk

**Risk**: Performance degradation from scope tracking
- **Mitigation**: Benchmark before/after, profile if needed
- **Detection**: Transpilation benchmarks

**Risk**: Edge cases with exception re-raising
- **Mitigation**: Specific tests for re-raising patterns
- **Detection**: Integration tests

### Low Risk

**Risk**: Documentation gaps
- **Mitigation**: Comprehensive code comments, update CHANGELOG
- **Detection**: Code review

## Timeline

**Total**: 4-6 hours

- **Phase 1 (RED)**: 1-2 hours - Write failing tests
- **Phase 2 (GREEN)**: 2-3 hours - Implement solution
- **Phase 3 (REFACTOR)**: 1 hour - Quality & testing
- **Phase 4 (VALIDATE)**: 30 minutes - Final validation

## Dependencies

**Unblocked**:
- ✅ DEPYLER-0327 complete (Try block analysis in property analyzer)
- ✅ Exception type generation working (ValueError, ZeroDivisionError)
- ✅ Result type handling infrastructure exists

**Unblocks**:
- 05_error_handling Matrix Project example (3 errors → 0)
- Matrix Project 75% → 83% pass rate
- Advanced exception handling patterns

## Related Issues

- **DEPYLER-0327**: Try block analysis (COMPLETE) - Foundation for this work
- **DEPYLER-0294**: Missing Result unwrapping (BLOCKED BY THIS) - Subset of Issue #2
- **DEPYLER-0296**: Return type mismatches in exception paths (BLOCKED BY THIS) - Issue #3

## Notes

This is an **architectural change** requiring careful implementation and testing. The EXTREME TDD protocol is essential:

1. **Write tests first** - No implementation without failing tests
2. **Property tests** - Validate invariants (scope nesting, exception handling correctness)
3. **Mutation tests** - Ensure tests actually validate behavior
4. **Fuzz tests** - Find edge cases with random structures
5. **Full quality gates** - PMAT, coverage, complexity, clippy

**Scientific Method**: We don't guess, we prove. Every behavior must be validated by tests.

---

**Status**: Ready for implementation with EXTREME TDD protocol
**Next Step**: Phase 1 - RED - Write comprehensive failing tests
