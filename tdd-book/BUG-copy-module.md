# BUG REPORT: copy.copy() fails for lists

**Discovered**: 2025-10-23
**Test Suite**: tdd-book/tests/test_copy.py
**Severity**: P1 - MAJOR
**Category**: stdlib_correctness

## Problem

`copy.copy()` for lists generates invalid Rust code with error:
```
Error: copy() takes no arguments
```

## Test Evidence

**Test File**: tests/test_copy.py
**Results**: 5/6 passing (83.3%)

**Passed** ✅:
- test_copy_shallow_dict - copy.copy() works for dicts
- test_copy_deepcopy_nested - copy.deepcopy() works for lists
- test_copy_shallow_shares_nested - copy.copy() works when sharing nested
- test_copy_deepcopy_dict - copy.deepcopy() works for dicts
- test_copy_multiple_references - copy.deepcopy() works with multiple refs

**Failed** ❌:
- test_copy_shallow_list - copy.copy() for lists

## Failing Code

```python
import copy

def test_shallow_copy() -> int:
    original = [1, 2, 3]
    copied = copy.copy(original)  # ❌ Fails here
    copied.append(4)
    return len(original)
```

## Analysis

**Hypothesis**: The transpiler is likely generating:
```rust
let copied = original.copy();  // ❌ Wrong - copy() is method, not function
```

Instead of:
```rust
let copied = copy::copy(&original);  // ✅ Correct - function call
```

OR:
```rust
let copied = original.clone();  // ✅ Rust idiomatic
```

**Why dict works but list fails**: Unknown - needs investigation of type-specific codegen.

## Impact

- `copy.copy()` is fundamental Python stdlib function
- Affects all shallow copy operations
- `copy.deepcopy()` works fine (5/5 tests passed)
- Partial implementation creates confusing behavior

## Recommended Fix

1. Check `import copy` handling in import resolution
2. Verify `copy.copy()` transpiles to correct Rust pattern
3. Add type-aware logic for `.clone()` vs `copy::copy()`
4. Ensure both list and dict work consistently

## Next Ticket

Should create: **DEPYLER-0XXX: Fix copy.copy() transpilation for lists**

---

**Discovery Method**: TDD Book validation (OPTION 1 strategy)
**Expected Outcome**: Will discover 5-10 more bugs like this across remaining 7 modules
