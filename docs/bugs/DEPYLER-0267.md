# DEPYLER-0267: Index Access Bug - `.copied()` Used for Non-Copy Types

## Status
**ACTIVE** - P0 BLOCKING

## Priority
**P0 - CRITICAL** - Prevents compilation of index access on non-Copy types (String)

## Discovery
- **Date**: 2025-10-26
- **Context**: DEPYLER-0265 test suite (for loop with String iteration)
- **Discoverer**: Last failing test in DEPYLER-0265 after fixing DEPYLER-0266

## Summary
Index access code generates `.copied()` for all types, but `.copied()` only works with `Copy` trait. Non-Copy types like `String` require `.cloned()` instead, causing compilation failures.

## Impact
**BLOCKS**: All Python code using index access on non-Copy types including:
- `strings[i]` where strings is `list[str]`
- Any collection indexing with Clone-only types
- Negative indexing on String collections

**Severity**: CRITICAL - affects fundamental list indexing patterns

## Reproduction

### Python Input
```python
def get_item(items: list[str], index: int) -> str:
    """Get item from string list."""
    if index >= 0:
        return items[index]
    else:
        return items[len(items) + index]
```

### Generated Rust (BROKEN)
```rust
pub fn get_item(items: &Vec<String>, index: i32) -> String {
    let actual_idx = if index >= 0 {
        index as usize
    } else {
        items.len().saturating_sub((-index) as usize)  // Also has DEPYLER-0268
    };
    items.get(actual_idx).copied().unwrap_or_default()  // ERROR!
    //                     ^^^^^^ String doesn't implement Copy
}
```

### Compilation Error
```
error[E0277]: the trait bound `String: Copy` is not satisfied
  --> generated.rs:31:30
   |
31 |         base.get(actual_idx).copied().unwrap_or_default()
   |                              ^^^^^^ the trait `Copy` is not implemented for `String`
   |
note: required by a bound in `Option::<&T>::copied`
help: consider removing this method call
```

## Root Cause Analysis

### Location
Unknown - likely in expression generation for index access

### Current Behavior
```rust
// Python: items[i]
// Generated: items.get(i).copied().unwrap_or_default()
// Problem: .copied() requires Copy trait
```

### Expected Behavior
```rust
// For Copy types (i32, f64, bool, etc.):
items.get(i).copied().unwrap_or_default()  // Use .copied()

// For Clone types (String, Vec, etc.):
items.get(i).cloned().unwrap_or_default()  // Use .cloned()
```

## Evidence

### Error from DEPYLER-0265 String test
```
error[E0277]: the trait bound `String: Copy` is not satisfied
    --> /tmp/test_depyler_0265_string.rs:31:30
     |
  31 |         base.get(actual_idx).copied().unwrap_or_default()
     |                              ^^^^^^ the trait `Copy` is not implemented for `String`
```

### Copy vs Clone in Rust
- **Copy types**: i8-i64, u8-u64, f32, f64, bool, char, tuples of Copy types
- **Clone-only types**: String, Vec<T>, HashMap<K,V>, most structs
- `.copied()` requires `T: Copy`
- `.cloned()` requires `T: Clone` (more general)

## Recommended Fix

### Strategy
**Detect element type and use `.cloned()` for non-Copy types, `.copied()` for Copy types**

### Implementation Location
Likely in index access expression generation (subscript operation)

### Fix Approach
```rust
// When generating index access:
// 1. Determine element type of collection
// 2. Check if type implements Copy (i32, f64, bool, char, etc.)
// 3. Use .copied() for Copy types, .cloned() for Clone types

// Simplified heuristic (since all Clone types also implement Clone):
// Use .cloned() universally (works for both Copy and Clone)
// OR: Use type checking to optimize Copy types

// Example:
if is_copy_type(element_type) {
    quote! { #collection.get(#index).copied().unwrap_or_default() }
} else {
    quote! { #collection.get(#index).cloned().unwrap_or_default() }
}
```

### Alternative: Always use .cloned()
```rust
// Simpler approach: .cloned() works for all Clone types (including Copy)
// Slightly less efficient for Copy types but always correct
items.get(i).cloned().unwrap_or_default()
```

## Test Cases Required

### Test 1: String list indexing
```python
def get_string(items: list[str], index: int) -> str:
    return items[index]
```

### Test 2: Vec list indexing (nested collections)
```python
def get_nested(matrix: list[list[int]], row: int) -> list[int]:
    return matrix[row]
```

### Test 3: Ensure Copy types still work
```python
def get_int(nums: list[int], index: int) -> int:
    return nums[index]
```

### Test 4: Negative indexing with String
```python
def get_last_string(items: list[str]) -> str:
    return items[-1]
```

## Success Criteria

1. ✅ Generated code uses `.cloned()` for non-Copy types (String, Vec, etc.)
2. ✅ Copy types continue to work (int, float, bool)
3. ✅ All 4 test cases compile without errors
4. ✅ No regressions in existing test suite

## Related Bugs
- DEPYLER-0265: Iterator dereferencing (FIXED) - same test blocked by this bug
- DEPYLER-0266: Boolean conversion (FIXED) - unblocked other DEPYLER-0265 tests
- DEPYLER-0268: Index negation bug (TBD) - also present in same code

## Files to Modify
- TBD: Expression codegen for index/subscript operations
- `tests/depyler_0267_index_access_test.rs` - Comprehensive tests
- `crates/depyler/Cargo.toml` - Test registration

## Estimated Complexity
**Low-Medium** - Simple fix (change .copied() to .cloned()), or add type checking

## Dependencies
None - can be fixed independently of DEPYLER-0268

## Timeline
**Target**: Same session (EXTREME TDD - 15-30 minutes)

---
**Created**: 2025-10-26
**Updated**: 2025-10-26
**Reporter**: DEPYLER-0265 test suite
**Assignee**: Claude Code
