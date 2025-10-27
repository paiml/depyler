# DEPYLER-0268: Index Negation Bug - Cannot Negate `usize`

## Status
**CLOSED - NON-ISSUE** (Bug does not exist; tests retained as regression protection)

## Resolution (2025-10-27)
**BUG DOES NOT EXIST** - Analysis revealed the transpiler correctly handles negative indices:
- **Literal indices** (e.g., `-1`): Directly converts to `usize` offset without negation
- **Runtime indices** (e.g., `idx: i32`): Uses `(-idx) as usize` which is CORRECT since idx is signed
- Generated code: `if idx < 0 { len.saturating_sub((-idx) as usize) }` ✅ COMPILES
- All tests pass without any code changes required

**Action Taken**: Tests retained as **regression protection** to ensure negative indexing continues to work correctly.

## Original Priority
**P0 - CRITICAL** - Would prevent compilation if bug existed (it doesn't)

## Discovery
- **Date**: 2025-10-26
- **Context**: DEPYLER-0265 test suite (for loop with String iteration)
- **Discoverer**: Fourth failing test in DEPYLER-0265 (string negative index)

## Summary
Negative index handling generates `(-idx) as usize` which fails compilation because Rust's `usize` type doesn't implement the `Neg` trait. The code attempts to negate an unsigned integer, which is a type error.

## Impact
**BLOCKS**: All Python code using negative indexing including:
- `items[-1]` (get last item)
- `items[-n]` (get nth item from end)
- Any collection with negative index access

**Severity**: CRITICAL - negative indexing is a fundamental Python pattern

## Reproduction

### Python Input
```python
def get_last(items: list[str]) -> str:
    """Get last item from list."""
    return items[-1]
```

### Generated Rust (BROKEN)
```rust
pub fn get_last(items: &Vec<String>) -> String {
    let actual_idx = items.len().saturating_sub((-1_i32) as usize);
    //                                            ^^^^^^^
    //                                            ERROR: cannot negate usize
    items.get(actual_idx).cloned().unwrap_or_default()
}
```

### Compilation Error
```
error[E0600]: cannot apply unary operator `-` to type `usize`
  --> generated.rs:XX:XX
   |
XX |     let actual_idx = items.len().saturating_sub((-idx) as usize);
   |                                                  ^^^^^^ cannot apply unary operator `-`
   |
   = note: the trait `Neg` is not implemented for `usize`
```

## Root Cause Analysis

### Location
`crates/depyler-core/src/rust_gen/expr_gen.rs` - line ~2143 (negative index handling)

### Current Behavior
```rust
// Python: items[-1]
// Generated (BROKEN):
base.len().saturating_sub((-idx) as usize)
// Problem: idx is already converted to usize before negation attempt
```

### Expected Behavior
```rust
// Strategy 1: Keep idx as signed integer until after negation
let offset = (-idx_i32).abs() as usize;
base.len().saturating_sub(offset)

// Strategy 2: Direct offset without negation (idx is already negative as i32)
let offset = idx_i32.unsigned_abs() as usize;
base.len().saturating_sub(offset)

// Strategy 3: Handle at type level (keep signed throughout)
if idx_i32 < 0 {
    base.len().saturating_sub(idx_i32.unsigned_abs() as usize)
} else {
    idx_i32 as usize
}
```

## Evidence

### Error from DEPYLER-0265 String test
```
error[E0600]: cannot apply unary operator `-` to type `usize`
    --> /tmp/test_depyler_0265_string.rs:XX:XX
     |
  XX |     let actual_idx = items.len().saturating_sub((-idx) as usize);
     |                                                  ^^^^^^ cannot apply unary operator `-`
```

### Rust Type System Constraints
- `usize` is **unsigned** - has no negative values
- `-` operator requires `Neg` trait
- `usize` does NOT implement `Neg` (by design)
- Must work with **signed** integers (i32, isize) for negation
- `.unsigned_abs()` or `.abs() as usize` are safe conversions

## Recommended Fix

### Strategy
**Use `.unsigned_abs()` to convert negative signed int to positive unsigned offset**

### Implementation Location
`crates/depyler-core/src/rust_gen/expr_gen.rs` - negative index handling in subscript operation

### Fix Approach

**Option 1: Use `.unsigned_abs()` (RECOMMENDED)**
```rust
// For literal negative indices (e.g., -1, -2):
// Python: items[-1]
// Input: idx is i32 literal (e.g., -1_i32)

// CURRENT (BROKEN):
base.get(base.len().saturating_sub((-idx) as usize)).cloned().unwrap_or_default()

// FIXED:
let offset = #idx.unsigned_abs() as usize;
base.get(base.len().saturating_sub(offset)).cloned().unwrap_or_default()

// OR inline:
base.get(base.len().saturating_sub((#idx).unsigned_abs() as usize)).cloned().unwrap_or_default()
```

**Option 2: Use `.abs()` for older Rust compatibility**
```rust
base.get(base.len().saturating_sub((#idx).abs() as usize)).cloned().unwrap_or_default()
```

### Code Change Required
```rust
// Find negative index literal handling in expr_gen.rs
// Current pattern (BROKEN):
quote! {
    #base.get(#base.len().saturating_sub((-#offset) as usize)).cloned().unwrap_or_default()
}

// Fixed pattern:
quote! {
    #base.get(#base.len().saturating_sub((#offset).unsigned_abs() as usize)).cloned().unwrap_or_default()
}
```

## Test Cases Required

### Test 1: Basic negative index (last item)
```python
def get_last(items: list[str]) -> str:
    return items[-1]
```

### Test 2: Negative index (second-to-last)
```python
def get_second_last(items: list[int]) -> int:
    return items[-2]
```

### Test 3: Runtime negative index
```python
def get_by_index(items: list[str], idx: int) -> str:
    """Handles both positive and negative indices at runtime."""
    return items[idx]
```

### Test 4: Negative index with Vec (nested collections)
```python
def get_last_row(matrix: list[list[int]]) -> list[int]:
    return matrix[-1]
```

### Test 5: Ensure positive indices still work
```python
def get_first(items: list[int]) -> int:
    return items[0]
```

## Success Criteria

1. ✅ Negative index literals compile without negation errors
2. ✅ Runtime negative indices handled correctly
3. ✅ Positive indices continue to work (no regression)
4. ✅ All 5 test cases compile and run correctly
5. ✅ No regressions in existing test suite
6. ✅ All 4 DEPYLER-0265 tests now pass

## Related Bugs
- DEPYLER-0265: Iterator dereferencing (FIXED) - blocked by this bug
- DEPYLER-0266: Boolean conversion (FIXED) - unblocked some DEPYLER-0265 tests
- DEPYLER-0267: Index access .cloned() (FIXED) - also blocked DEPYLER-0265 tests

## Files to Modify
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Fix negative index handling (~line 2143)
- `tests/depyler_0268_index_negation_test.rs` - Comprehensive tests (CREATE)
- `crates/depyler/Cargo.toml` - Test registration
- `CHANGELOG.md` - Document fix

## Estimated Complexity
**Low** - Simple fix (change negation strategy from `(-idx)` to `.unsigned_abs()`)

## Dependencies
None - can be fixed independently

## Timeline
**Target**: Same session (EXTREME TDD - 15-30 minutes)

---
**Created**: 2025-10-27
**Updated**: 2025-10-27
**Reporter**: DEPYLER-0265 test suite
**Assignee**: Claude Code
