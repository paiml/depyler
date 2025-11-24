# DEPYLER-0496: Binary Operations on Result-Returning Expressions Missing ? Operator

**Status**: PLANNED
**Priority**: P0-CRITICAL (STOP THE LINE)
**Assigned**: Claude
**Created**: 2025-11-23T21:30:00+00:00

## Problem Statement

When binary operations (e.g., `+`, `-`, `*`) are performed on Result-returning function calls, the transpiler generates code that tries to operate on Result types directly instead of using the `?` operator to propagate errors.

### Current Behavior (BROKEN)

**Python**:
```python
def fibonacci_memoized(n: int, memo=None) -> int:
    if n <= 0:
        return 0
    elif n == 1:
        return 1

    result = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo)
    memo[n] = result
    return result
```

**Generated Rust** (INCORRECT):
```rust
pub fn fibonacci_memoized(n: i32, memo: &HashMap<i32, i32>) -> Result<i32, IndexError> {
    if n <= 0 {
        return Ok(0);
    } else if n == 1 {
        return Ok(1);
    }

    // ❌ WRONG: Cannot add Result<i32> + Result<i32>
    let result = fibonacci_memoized(n - 1, &memo) + fibonacci_memoized(n - 2, &memo);
    memo.insert(n, result);
    Ok(result)
}
```

**Compilation Error**:
```
error[E0369]: cannot add `Result<i32, IndexError>` to `Result<i32, IndexError>`
   --> fibonacci.rs:146:56
    |
146 |     let _cse_temp_3 = fibonacci_memoized(n - 1, &memo) + fibonacci_memoized(n - 2, &memo);
    |                       -------------------------------- ^ -------------------------------- Result<i32, IndexError>
    |                       |
    |                       Result<i32, IndexError>
```

### Expected Behavior (CORRECT)

**Generated Rust** (CORRECT):
```rust
pub fn fibonacci_memoized(n: i32, memo: &HashMap<i32, i32>) -> Result<i32, IndexError> {
    if n <= 0 {
        return Ok(0);
    } else if n == 1 {
        return Ok(1);
    }

    // ✅ CORRECT: Use ? operator to unwrap Results before operating
    let result = fibonacci_memoized(n - 1, &memo)? + fibonacci_memoized(n - 2, &memo)?;
    memo.insert(n, result);
    Ok(result)
}
```

## Root Cause Analysis

### Hypothesis 1: Binary Operation Codegen Doesn't Check Result Return Types

The binary operation code generation (`codegen_binop`) likely:
1. Generates code for left operand: `fibonacci_memoized(n - 1, &memo)`
2. Generates code for right operand: `fibonacci_memoized(n - 2, &memo)`
3. Combines with operator: `left + right`
4. **MISSING**: Doesn't check if operands are Result types that need `?`

**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` - `codegen_binop()` or `codegen_call()`

### Hypothesis 2: Type Context Not Propagated to Binary Operations

The codegen may not track:
- Whether current function returns `Result<T, E>`
- Whether sub-expressions return Result types
- When to insert `?` operator automatically

**Location**: `CodeGenContext` may need `in_result_context: bool` flag

### Hypothesis 3: Result Wrapping Happens Too Late

The function call codegen may wrap results after binary operations complete:
1. Generate: `let x = f()`
2. Generate: `let y = g()`
3. Generate: `let z = x + y` ← Error here (x and y are Results)
4. Only then wrap: `Ok(z)`

**Should be**:
1. Generate: `let x = f()?` ← Unwrap Result
2. Generate: `let y = g()?` ← Unwrap Result
3. Generate: `let z = x + y` ← Now works (x and y are unwrapped)
4. Wrap: `Ok(z)`

## Impact

**Severity**: P0-CRITICAL - Breaks ANY recursive function with error handling

**Affected Code**:
- ✅ `fibonacci_memoized` in examples/test_project/fibonacci.py
- ✅ Any function with binary operations on Result-returning calls
- ✅ Recursive functions with error propagation
- ✅ 1 compilation error (E0369) in fibonacci.rs

**Scope**:
- Error handling is CRITICAL for Rust safety
- This bug makes Result-returning recursive functions completely unusable
- Blocks adoption for any non-trivial error handling code

## Test Plan

### Phase 1: RED - Failing Tests

Create comprehensive test suite in `crates/depyler-core/tests/depyler_0496_result_binop.rs`:

**Test Cases**:
1. ✅ `test_result_returning_binop_addition` - `f()? + g()?`
2. ✅ `test_result_returning_binop_subtraction` - `f()? - g()?`
3. ✅ `test_result_returning_binop_multiplication` - `f()? * g()?`
4. ✅ `test_fibonacci_memoized_specific` - Actual fibonacci_memoized case
5. ✅ `test_nested_result_binops` - `(f()? + g()?) * h()?`
6. ✅ `test_compilation_no_e0369` - Generated code compiles

**Assertions**:
- Function calls in binary operations have `?` suffix
- Generated code pattern: `func_call()? OPERATOR func_call()?`
- No E0369 errors in rustc output
- Generated code compiles successfully

### Phase 2: GREEN - Implementation

**Fix Location**: One or more of:
1. `expr_gen.rs::codegen_binop()` - Add Result type detection
2. `expr_gen.rs::codegen_call()` - Auto-append `?` in Result context
3. `CodeGenContext` - Add `result_context` tracking

**Expected Changes**:
- Detect when binary operation operands return Result types
- Automatically append `?` operator to Result-returning calls in binop
- Maintain proper error propagation through expression tree

**Algorithm**:
```rust
fn codegen_binop(left: Expr, op: BinOp, right: Expr, ctx: &mut CodeGenContext) -> TokenStream {
    let left_code = codegen_expr(left, ctx);
    let right_code = codegen_expr(right, ctx);

    // NEW: Check if operands are Result types
    let left_is_result = expr_returns_result(left, ctx);
    let right_is_result = expr_returns_result(right, ctx);

    // NEW: Append ? if needed
    let left_final = if left_is_result { quote! { #left_code? } } else { left_code };
    let right_final = if right_is_result { quote! { #right_code? } } else { right_code };

    quote! { #left_final #op #right_final }
}
```

### Phase 3: REFACTOR - Quality Gates

**Quality Checks**:
- ✅ All 6 new tests pass
- ✅ No regressions in existing error handling tests
- ✅ fibonacci.rs compiles without E0369 error
- ✅ Complexity ≤10 (PMAT enforcement)
- ✅ No clippy warnings
- ✅ Test coverage ≥80%

**Verification**:
```bash
# Run new tests
cargo test depyler_0496_result_binop

# Verify no regressions
cargo test --workspace

# Re-transpile fibonacci.py
cargo run --bin depyler -- transpile examples/test_project/fibonacci.py

# Verify compilation
rustc --crate-type=lib --deny=warnings examples/test_project/fibonacci.rs
```

## Files to Modify

**Primary**:
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Fix binop Result handling
- `crates/depyler-core/src/codegen_context.rs` - Add Result context tracking (if needed)

**Tests**:
- `crates/depyler-core/tests/depyler_0496_result_binop.rs` (NEW)

**Examples**:
- `examples/test_project/fibonacci.rs` - Re-transpile after fix

## Acceptance Criteria

- [ ] Create comprehensive failing tests (RED phase)
- [ ] `fibonacci_memoized(n-1) + fibonacci_memoized(n-2)` generates `?` operators
- [ ] All binary operations on Result expressions use `?`
- [ ] fibonacci_memoized compiles without E0369 error
- [ ] No regression in error handling tests
- [ ] Test coverage ≥80%
- [ ] Complexity ≤10
- [ ] Error count: 13 → 12 (eliminate 1 E0369 error)

## Timeline

**Estimated Effort**: 2 hours
- Phase 1 (RED): 30 minutes
- Phase 2 (GREEN): 60 minutes
- Phase 3 (REFACTOR): 30 minutes

## Related Issues

- **DEPYLER-0495**: Generator return type (COMPLETED)
- **DEPYLER-0497**: Format macro Display trait (PLANNED)
- **DEPYLER-0498**: Option type mismatches (PLANNED)

## Notes

This is a **STOP THE LINE** bug - it breaks error handling, a CRITICAL Rust safety feature. Must be fixed before any other work.

The `?` operator is fundamental to idiomatic Rust error handling. Without it, Result-returning functions cannot be composed in expressions.
