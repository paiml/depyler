# DEPYLER-0266: Boolean Conversion Bug - Cannot Use `!` on Borrowed Collections

## Status
**ACTIVE** - P0 BLOCKING

## Priority
**P0 - CRITICAL** - Prevents compilation of empty collection checks

## Discovery
- **Date**: 2025-10-26
- **Context**: DEPYLER-0265 test suite (iterator dereferencing tests)
- **Discoverer**: Blocked 3 of 4 tests for DEPYLER-0265

## Summary
Python's `if not collection:` generates Rust code `if !collection` which fails because you cannot apply the unary `!` operator to borrowed collection types like `&Vec<T>`. The correct Rust idiom is `collection.is_empty()`.

## Impact
**BLOCKS**: All Python code using `if not collection:` empty checks including:
- `if not numbers:` (list empty check)
- `if not items:` (collection truthiness)
- Guard clauses with empty checks

**Severity**: CRITICAL - affects fundamental Python empty-checking patterns

## Reproduction

### Python Input
```python
def find_min(numbers: list[int]) -> int:
    """Find minimum value in a list."""
    if not numbers:
        return 0
    min_val = numbers[0]
    for num in numbers:
        if num < min_val:
            min_val = num
    return min_val
```

### Generated Rust (BROKEN)
```rust
pub fn find_min<'a>(numbers: &'a Vec<i32>) -> Result<i32, IndexError> {
    if !numbers {  // ERROR: cannot apply unary operator `!`
        return Ok(0 as i32);
    }
    // ...
}
```

### Compilation Error
```
error[E0600]: cannot apply unary operator `!` to type `&'a Vec<i32>`
  --> generated.rs:20:8
   |
20 |     if !numbers {
   |        ^^^^^^^^ cannot apply unary operator `!`
```

## Root Cause Analysis

### Location
Unknown - likely in expression generation for `not` operator (UnaryOp)

### Current Behavior
```rust
// Python: if not collection:
// Generated: if !collection {
// Problem: ! operator requires bool, but collection is &Vec<T>
```

### Expected Behavior
```rust
// Option 1: Use .is_empty()
if numbers.is_empty() {
    return Ok(0);
}

// Option 2: Check length
if numbers.len() == 0 {
    return Ok(0);
}

// Option 3: Pattern match
if let [] = &numbers[..] {
    return Ok(0);
}
```

## Evidence

### Error from DEPYLER-0265 tests
All 3 failing tests show the same error:
```
error[E0600]: cannot apply unary operator `!` to type `&'a Vec<i32>`
error[E0600]: cannot apply unary operator `!` to type `&'a Vec<String>`
```

### Python Semantics
In Python, `not collection` checks if the collection is empty:
- `not []` → `True`
- `not [1, 2]` → `False`
- Python truthiness: empty collections are falsy

### Rust Semantics
Rust doesn't have truthiness for collections:
- `!` operator requires `bool` type
- Collections don't implement `Not` trait
- Idiomatic: `collection.is_empty()`

## Recommended Fix

### Strategy
**Detect `not` operator on collection types and generate `.is_empty()` instead of `!`**

### Implementation Location
Likely in expression codegen - need to find where UnaryOp (Not) is handled

### Fix Approach
```rust
// When generating UnaryOp::Not on collection type:
// 1. Detect if operand is a collection (Vec, HashMap, HashSet, etc.)
// 2. Generate .is_empty() instead of !
// 3. For other types (bool), use ! as normal

// Example:
if is_collection_type(operand_type) {
    quote! { #operand.is_empty() }
} else {
    quote! { !#operand }
}
```

## Test Cases Required

### Test 1: List empty check
```python
def is_empty_list(items: list[int]) -> bool:
    if not items:
        return True
    return False
```

### Test 2: String empty check
```python
def is_empty_string(text: str) -> bool:
    if not text:
        return True
    return False
```

### Test 3: Dict empty check
```python
def is_empty_dict(mapping: dict[str, int]) -> bool:
    if not mapping:
        return True
    return False
```

### Test 4: Guard clause pattern
```python
def process_items(items: list[int]) -> int:
    if not items:
        return 0
    return sum(items)
```

## Success Criteria

1. ✅ Generated code uses `.is_empty()` instead of `!` for collections
2. ✅ All 4 test cases compile without errors
3. ✅ Boolean `not` still works (e.g., `not flag`)
4. ✅ No regressions in existing test suite

## Related Bugs
- DEPYLER-0265: Iterator dereferencing (FIXED) - tests blocked by this bug
- DEPYLER-0267: Index access bug (TBD)
- DEPYLER-0268: Index negation bug (TBD)

## Files to Modify
- TBD: Expression codegen for UnaryOp::Not
- `tests/depyler_0266_boolean_conversion_test.rs` - Comprehensive tests
- `crates/depyler/Cargo.toml` - Test registration

## Estimated Complexity
**Medium** - Requires type-aware expression generation

## Dependencies
None - can be fixed independently

## Timeline
**Target**: Same session (EXTREME TDD - 30-60 minutes)

---
**Created**: 2025-10-26
**Updated**: 2025-10-26
**Reporter**: DEPYLER-0265 test suite
**Assignee**: Claude Code
