# DEPYLER-0497: Five-Whys Analysis - Format Macro Display Trait Issue

**Date**: 2025-11-24
**Method**: Five-Whys + Golden Trace Validation
**Golden Trace**: fibonacci_golden.json (9,293 syscalls from Python execution)

---

## Problem Statement

When `Result<T>`, `Option<T>`, or `Vec<T>` types are used in `format!` macro with `{}` placeholder, the transpiler generates code that fails to compile because these types don't implement `Display` trait.

---

## Five-Whys Analysis: Error #6 (Vec Display)

### Compilation Error
```rust
error[E0277]: `Vec<i32>` doesn't implement `std::fmt::Display`
   --> fibonacci.rs:198:56
    |
198 |         format!("\nFirst {} Fibonacci numbers: {}", n, fibonacci_sequence(n))
    |                                                --      ^^^^^^^^^^^^^^^^^^^^^ `Vec<i32>` cannot be formatted with the default formatter
```

### Five-Whys

**Why #1**: Why does `format!("{}", fibonacci_sequence(n))` fail to compile?
- **Answer**: `fibonacci_sequence()` returns `Vec<i32>`, and `Vec<T>` doesn't implement `Display` trait

**Why #2**: Why is {} placeholder used instead of {:?}?
- **Answer**: Transpiler converts Python f-strings to Rust format! with {} for all types

**Why #3**: Why doesn't transpiler detect that Vec needs {:?}?
- **Answer**: No type-aware format placeholder selection in expr_gen.rs

**Why #4**: Why is type information not available during format! codegen?
- **Answer**: Type inference happens but results aren't used to select format placeholder

**Why #5 (ROOT CAUSE)**: Why aren't type-specific formatting rules applied?
- **Answer**: Format macro generation uses simple string substitution without type checking

### Golden Trace Evidence

From `fibonacci_golden.json` (Python output):
```
First 10 Fibonacci numbers: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

**Python behavior**:
- Python f-strings call `__str__()` method on objects
- Lists implement `__str__()` which formats as: `[item1, item2, ...]`
- No type errors - Python duck typing handles this automatically

**Rust requirement**:
- `{}` requires `Display` trait implementation
- `Vec<T>` only implements `Debug` trait (requires `{:?}`)
- Type mismatch causes compilation error

**Conclusion**: Python's duck typing masks the Display vs Debug distinction. Rust requires explicit trait bounds.

---

## Five-Whys Analysis: Error #8 (Option Display)

### Compilation Error
```rust
error[E0277]: `Option<i32>` doesn't implement `std::fmt::Display`
   --> fibonacci.rs:210:74
    |
210 |             format!("\n{} is at index {} in Fibonacci sequence", target, index)
    |                                       --                                 ^^^^^ `Option<i32>` cannot be formatted with the default formatter
```

### Five-Whys

**Why #1**: Why does `format!("{}", index)` fail where `index: Option<i32>`?
- **Answer**: `Option<T>` doesn't implement `Display` trait (only `Debug`)

**Why #2**: Why is Python behavior different?
- **Answer**: Python's None and int values both implement `__str__()`

**Why #3**: Why doesn't transpiler unwrap Option before formatting?
- **Answer**: No special handling for Option types in format expression generation

**Why #4**: Why is the Python → Rust mapping incorrect?
- **Answer**: Python directly formats values; Rust needs explicit unwrap or {:?}

**Why #5 (ROOT CAUSE)**: Why no pattern match or unwrap_or() insertion?
- **Answer**: Format macro codegen doesn't analyze wrapped types (Option/Result)

### Golden Trace Evidence

From `fibonacci_golden.json` (Python output):
```
21 is at index 8 in Fibonacci sequence
```

**Python behavior**:
- `find_fibonacci_index(21)` returns `8` (int, not Optional)
- Python f-string formats int directly: `f"{index}"`
- If None, would print `"None"` (Python's default)

**Rust requirement**:
- `find_fibonacci_index(21)` returns `Option<i32>`
- Cannot use `{}` with Option directly
- Must either:
  1. Use `{:?}` → prints `Some(8)` (not semantically equivalent)
  2. Use `.unwrap()` → prints `8` (matches Python but unsafe)
  3. Use `.unwrap_or_default()` → prints `8` or `0` (safe)

**Semantic Gap**: Python implicitly unwraps Optional to show inner value. Rust requires explicit handling.

---

## Five-Whys Analysis: Result Type (Error not in fibonacci.rs but general issue)

### Potential Error
```rust
error[E0277]: `Result<i32, IndexError>` doesn't implement `std::fmt::Display`
```

### Five-Whys

**Why #1**: Why would `format!("{}", result)` fail for Result types?
- **Answer**: `Result<T, E>` doesn't implement `Display` (only `Debug`)

**Why #2**: Why is this different from Python?
- **Answer**: Python doesn't have Result type - uses exceptions

**Why #3**: Why doesn't transpiler propagate errors with `?`?
- **Answer**: Format argument evaluation doesn't insert `?` operator

**Why #4**: Why isn't Result unwrapped in format context?
- **Answer**: Format macro sees expression as-is without Result handling

**Why #5 (ROOT CAUSE)**: Why no automatic error propagation?
- **Answer**: No context-aware Result handling in format expression codegen

### Golden Trace Evidence

From `fibonacci_golden.json` (Python output - after DEPYLER-0498 fix):
```
Fibonacci(10) memoized: 55
```

**Python behavior**:
- `fibonacci_memoized(10)` returns `55` (int)
- If error occurs, exception is raised (not returned)
- f-string never sees error values

**Rust requirement**:
- `fibonacci_memoized(10)` returns `Result<i32, IndexError>`
- Must use `?` operator to propagate: `fibonacci_memoized(10)?`
- Or use `{:?}` to show Result wrapper: `Ok(55)`

**Key Insight**: Python exceptions vs Rust Result types require different handling in format contexts.

---

## Root Cause Summary

**Primary Root Cause**: Format macro generation in `expr_gen.rs` uses type-blind string substitution.

**Contributing Factors**:
1. No type-aware placeholder selection (Display vs Debug)
2. No Option/Result unwrapping logic in format arguments
3. Type inference results not propagated to format codegen
4. Python's duck typing masks Display/Debug distinction

---

## Solution Approaches

### Option 1: Type-Aware Format Placeholder Selection (RECOMMENDED)

**Principle**: Detect type and auto-select {:?} for non-Display types

```rust
// In expr_gen.rs: convert_format_string()
fn select_format_placeholder(expr_type: &Type) -> &str {
    match expr_type {
        Type::Int | Type::Float | Type::String | Type::Bool => "{}", // Display
        Type::List(_) | Type::Tuple(_) | Type::Set(_) => "{:?}",     // Debug
        Type::Optional(_) | Type::Result(_) => "{:?}",                // Debug
        _ => "{:?}" // Default to Debug for safety
    }
}
```

**Pros**:
- Minimal code changes
- Safe default (Debug works for all types)
- Matches Python's automatic __str__() behavior

**Cons**:
- Output shows wrappers: `Ok(55)` instead of `55`
- Not semantically equivalent to Python

### Option 2: Smart Unwrapping (BETTER for semantic equivalence)

**Principle**: Unwrap Option/Result in safe contexts

```rust
// For Option<T>: use .unwrap_or_default() or pattern match
format!("{}", index.unwrap_or_default())

// For Result<T>: propagate with ? if function returns Result
format!("{}", fibonacci_memoized(n)?)
```

**Pros**:
- Output matches Python semantics (no wrappers)
- Golden trace validation passes

**Cons**:
- Requires function signature analysis (does main() return Result?)
- More complex codegen logic

### Option 3: Hybrid Approach (BEST)

**Principle**: Combine type detection + context-aware unwrapping

1. **For collections (Vec/HashMap/HashSet)**: Use `{:?}` (matches Python list repr)
2. **For Option in Display context**:
   - Use `.unwrap_or_default()` if safe
   - Pattern match if None case matters
3. **For Result in error-propagating context**:
   - Insert `?` operator if function returns Result
   - Use `{:?}` otherwise

---

## Implementation Plan

### Phase 1: RED - Create Failing Tests

Create `crates/depyler-core/tests/depyler_0497_format_display.rs`:

```rust
#[test]
fn test_format_vec_uses_debug() {
    let python = r#"
def main():
    nums = [1, 2, 3]
    print(f"Numbers: {nums}")
"#;
    let rust = transpile(python);
    assert!(rust.contains("{:?}"), "Vec should use Debug formatter");
    assert!(compile_rust(&rust).is_ok());
}

#[test]
fn test_format_option_unwraps() {
    let python = r#"
def find() -> int | None:
    return 42

def main():
    result = find()
    print(f"Result: {result}")
"#;
    let rust = transpile(python);
    // Should either use {:?} or .unwrap_or()
    assert!(compile_rust(&rust).is_ok());
}
```

### Phase 2: GREEN - Implement Fix

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`

**Location**: `convert_format_string()` method

**Change**: Add type-aware placeholder selection

### Phase 3: REFACTOR - Validate with Golden Trace

```bash
# Re-transpile fibonacci.py
depyler transpile fibonacci.py -o fibonacci_fixed.rs

# Compile
rustc fibonacci_fixed.rs -o fibonacci_fixed

# Capture trace
renacer --format json -- ./fibonacci_fixed > fibonacci_rust_fixed.json

# Compare outputs
diff <(head -31 fibonacci_golden.json) <(head -31 fibonacci_rust_fixed.json)

# Expected: Semantically equivalent output
```

---

## Success Criteria

1. ✅ fibonacci.rs compiles without E0277 errors
2. ✅ Golden trace validation passes (output matches Python)
3. ✅ All format! tests pass
4. ✅ No regressions in existing format! usage
5. ✅ Quality gates pass (complexity ≤10, TDG A-)

---

## References

- **Golden Trace**: fibonacci_golden.json (9,293 syscalls)
- **Specification**: DEPYLER-0497-format-macro-display-trait.md
- **Related**: DEPYLER-0498 (fixed Result unwrapping, now need format! fix)
- **Rust std::fmt**: https://doc.rust-lang.org/std/fmt/

---

**Next Action**: Write failing tests (RED phase) in `depyler_0497_format_display.rs`
