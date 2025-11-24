# DEPYLER-0497: Option/Result Types in format! Macro Need Debug Formatting or Unwrap

**Status**: IN PROGRESS
**Priority**: P1-HIGH
**Assigned**: Claude
**Created**: 2025-11-23T21:30:00+00:00

## Problem Statement

When `Result<T>`, `Option<T>`, or `Vec<T>` types are used in `format!` macro with `{}` placeholder, the transpiler generates code that fails to compile because these types don't implement `Display` trait.

### Current Behavior (BROKEN)

**Python**:
```python
def main():
    n = 10
    print(f"Fibonacci({n}) memoized: {fibonacci_memoized(n)}")
    print(f"\nFirst {n} Fibonacci numbers: {fibonacci_sequence(n)}")

    target = 55
    index = find_fibonacci_index(target)
    print(f"\n{target} is at index {index} in Fibonacci sequence")
```

**Generated Rust** (INCORRECT):
```rust
pub fn main() {
    let n = 10;
    // ❌ WRONG: fibonacci_memoized returns Result<i32>, {} expects Display
    format!("Fibonacci({}) memoized: {}", n, fibonacci_memoized(n));

    // ❌ WRONG: fibonacci_sequence returns Vec<i32>, {} expects Display
    format!("\nFirst {} Fibonacci numbers: {}", n, fibonacci_sequence(n));

    let target = 55;
    let index = find_fibonacci_index(target);
    // ❌ WRONG: index is Option<i32>, {} expects Display
    format!("\n{} is at index {} in Fibonacci sequence", target, index);
}
```

**Compilation Errors**:
```
error[E0277]: `Result<i32, IndexError>` doesn't implement `std::fmt::Display`
   --> fibonacci.rs:194:50
    |
194 |         format!("Fibonacci({}) memoized: {}", n, fibonacci_memoized(n))
    |                                          --      ^^^^^^^^^^^^^^^^^^^^^ `Result<i32, IndexError>` cannot be formatted with the default formatter

error[E0277]: `Vec<i32>` doesn't implement `std::fmt::Display`
   --> fibonacci.rs:198:56
    |
198 |         format!("\nFirst {} Fibonacci numbers: {}", n, fibonacci_sequence(n))
    |                                                --      ^^^^^^^^^^^^^^^^^^^^^ `Vec<i32>` cannot be formatted with the default formatter

error[E0277]: `Option<i32>` doesn't implement `std::fmt::Display`
   --> fibonacci.rs:210:74
    |
210 |             format!("\n{} is at index {} in Fibonacci sequence", target, index)
    |                                       --                                 ^^^^^ `Option<i32>` cannot be formatted with the default formatter
```

### Expected Behavior (CORRECT)

**Generated Rust** (CORRECT - Option 1: Debug formatting):
```rust
pub fn main() {
    let n = 10;
    // ✅ CORRECT: Use {:?} for Result types
    format!("Fibonacci({}) memoized: {:?}", n, fibonacci_memoized(n));

    // ✅ CORRECT: Use {:?} for Vec types
    format!("\nFirst {} Fibonacci numbers: {:?}", n, fibonacci_sequence(n));

    let target = 55;
    let index = find_fibonacci_index(target);
    // ✅ CORRECT: Use {:?} for Option types
    format!("\n{} is at index {:?} in Fibonacci sequence", target, index);
}
```

**Generated Rust** (CORRECT - Option 2: Unwrap when safe):
```rust
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 10;
    // ✅ CORRECT: Unwrap Result with ? in Result-returning function
    format!("Fibonacci({}) memoized: {}", n, fibonacci_memoized(n)?);

    // ✅ CORRECT: Vec can use {:?} or format elements
    format!("\nFirst {} Fibonacci numbers: {:?}", n, fibonacci_sequence(n));

    let target = 55;
    let index = find_fibonacci_index(target);
    // ✅ CORRECT: Unwrap Option with .unwrap_or_default()
    format!("\n{} is at index {} in Fibonacci sequence", target, index.unwrap_or_default());

    Ok(())
}
```

## Root Cause Analysis

### Hypothesis 1: Format Macro Codegen Doesn't Check Argument Types

The format! macro code generation likely:
1. Generates format string: `format!("text {}", ...)`
2. Generates arguments without type checking
3. **MISSING**: Doesn't detect if arguments are Result/Option/Vec
4. **MISSING**: Doesn't modify `{}` to `{:?}` or wrap with unwrap

**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` - format macro handling

### Hypothesis 2: f-string Translation Preserves {} Without Type Context

Python f-strings use `{expr}` which works for any type. Rust's `{}` requires `Display` trait.

The transpiler may:
1. Parse f-string: `f"value: {result}"`
2. Convert to format!: `format!("value: {}", result)`
3. **MISSING**: Type-aware placeholder selection (`{}` vs `{:?}`)

### Hypothesis 3: No Type Information During Format Codegen

The format macro generation happens without access to type information:
- Can't determine if expression returns Result/Option/Vec
- Can't make intelligent choice between `{}` and `{:?}`
- Can't decide when to auto-unwrap

## Impact

**Severity**: P1-HIGH - Breaks display/printing for common types

**Affected Code**:
- ✅ Any `print()` or f-string with Result-returning function
- ✅ Any `print()` or f-string with Option values
- ✅ Any `print()` or f-string with Vec/List values
- ✅ 3 compilation errors (E0277) in fibonacci.rs

**Scope**:
- Printing and formatting is FUNDAMENTAL to debugging and UX
- This bug makes displaying common Rust types completely unusable
- Blocks adoption for any code with Result/Option/Vec output

## Test Plan

### Phase 1: RED - Failing Tests

Create comprehensive test suite in `crates/depyler-core/tests/depyler_0497_format_display.rs`:

**Test Cases**:
1. ✅ `test_format_result_type` - `format!("{:?}", result_fn())`
2. ✅ `test_format_option_type` - `format!("{:?}", option_val)`
3. ✅ `test_format_vec_type` - `format!("{:?}", vec_val)`
4. ✅ `test_print_result_function` - `print(f"{result_fn()}")`
5. ✅ `test_print_vec_variable` - `print(f"{vec_var}")`
6. ✅ `test_compilation_no_e0277` - Generated code compiles

**Assertions**:
- Format strings use `{:?}` for Result/Option/Vec types
- Or expressions are wrapped with `.unwrap_or_default()` / `?`
- No E0277 errors in rustc output
- Generated code compiles successfully

### Phase 2: GREEN - Implementation

**Fix Location**: One or more of:
1. `expr_gen.rs` - Format macro argument type detection
2. `stmt_gen.rs` - Print statement handling
3. Type system integration for format macro

**Expected Changes**:
- Detect if format argument is Result/Option/Vec type
- Automatically use `{:?}` placeholder instead of `{}`
- Or wrap expressions appropriately (unwrap_or, ?)

**Algorithm**:
```rust
fn codegen_format_arg(expr: &HirExpr, ctx: &CodeGenContext) -> (String, TokenStream) {
    let rust_expr = expr.to_rust_expr(ctx)?;

    // Check expression type
    let needs_debug_fmt = match get_expr_type(expr, ctx) {
        Type::Generic { base, .. } if base == "Result" || base == "Option" => true,
        Type::List(_) => true,  // Vec needs Debug
        _ => false,
    };

    let placeholder = if needs_debug_fmt { "{:?}" } else { "{}" };

    (placeholder, rust_expr)
}
```

### Phase 3: REFACTOR - Quality Gates

**Quality Checks**:
- ✅ All 6 new tests pass
- ✅ No regressions in existing format/print tests
- ✅ fibonacci.rs compiles without E0277 Display errors
- ✅ Complexity ≤10 (PMAT enforcement)
- ✅ No clippy warnings
- ✅ Test coverage ≥80%

**Verification**:
```bash
# Run new tests
cargo test depyler_0497_format_display

# Verify no regressions
cargo test --workspace

# Re-transpile fibonacci.py
cargo run --bin depyler -- transpile examples/test_project/fibonacci.py

# Verify compilation
rustc --crate-type=lib --deny=warnings examples/test_project/fibonacci.rs
```

## Files to Modify

**Primary**:
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Format macro codegen
- `crates/depyler-core/src/rust_gen/stmt_gen.rs` - Print statement handling (if needed)

**Tests**:
- `crates/depyler-core/tests/depyler_0497_format_display.rs` (NEW)

**Examples**:
- `examples/test_project/fibonacci.rs` - Re-transpile after fix

## Acceptance Criteria

- [ ] Create comprehensive failing tests (RED phase)
- [ ] Result/Option/Vec in format! use `{:?}` placeholder
- [ ] fibonacci.rs compiles without E0277 Display errors
- [ ] No regression in print/format tests
- [ ] Test coverage ≥80%
- [ ] Complexity ≤10
- [ ] Error count: 13 → 10 (eliminate 3 E0277 Display errors)

## Timeline

**Estimated Effort**: 2 hours
- Phase 1 (RED): 30 minutes
- Phase 2 (GREEN): 60 minutes
- Phase 3 (REFACTOR): 30 minutes

## Related Issues

- **DEPYLER-0496**: Result binop error propagation (COMPLETED)
- **DEPYLER-0498**: Option type mismatches (PLANNED)

## Notes

While this is P1-HIGH (not P0-CRITICAL), it's a fundamental usability issue. Display and printing are essential for debugging and user output.

The solution should prefer `{:?}` for non-Display types as it's simpler and more explicit than automatic unwrapping. Auto-unwrapping could hide errors.

## Design Decision: {:?} vs Unwrap

**Prefer {:?} Debug Formatting**:
- ✅ Safer: Doesn't panic
- ✅ More explicit: Shows Option(Some(5)) vs 5
- ✅ Simpler: No context needed about safety
- ✅ Idiomatic: Rust std lib uses Debug for these types

**Auto-unwrap only when**:
- Function already returns Result (use `?`)
- Clear None/Err handling present in Python code
