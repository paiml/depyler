# DEPYLER-0473: Dict .get()/.contains_key() Regression from DEPYLER-0472

## Status: 🚧 IN PROGRESS
- **Created**: 2025-11-23
- **Priority**: P0 (CRITICAL - blocks 12 tests)
- **Type**: Bug Fix - Test Regression
- **Root Cause**: DEPYLER-0472 json context changes affected dict key borrowing
- **Impact**: 12 test failures + 3 config_manager E0382 errors

## Problem Statement

DEPYLER-0472 fixed E0308 type conversion errors but introduced regressions in dict key handling:

**Test Failures** (12 tests):
1. `test_dict_contains_key_no_double_ref` - expects `.contains_key()`, gets `.get().is_some()`
2. `test_regression_dict_operations_still_work` - dict operations broken
3-12. Various validator and type system tests failing

**config_manager Errors** (3 E0382):
```rust
// Error 1 & 2: Lines 116/120/122
if !current.get(k).is_some() {  // Moves k
    current.as_object_mut().unwrap().insert(k, ...);  // ❌ Use after move
}

// Error 3: Lines 107/126
let base = keys;  // Moves keys
...
let base = &keys;  // ❌ Borrow after move
```

## Root Cause

**Problem 1**: Tests expect `.contains_key(&key)` but code generates `.get(&key).is_some()` (DEPYLER-0449 pattern)

**Problem 2**: Dict keys (`k`, `keys`) moved instead of borrowed when used with serde_json::Value

## Solution

### Fix 1: Update Tests or Revert to .contains_key()
Either:
- A) Update tests to accept `.get().is_some()` as equivalent
- B) Generate `.contains_key()` for dict membership tests

### Fix 2: Borrow Dict Keys for .get()
When calling `.get(k)` on HashMap/serde_json::Value, always borrow the key:
```rust
if !current.get(&k).is_some() {  // ✅ Borrow k
    current.as_object_mut().unwrap().insert(k, ...);  // ✅ Can still use k
}
```

## Implementation Plan

1. **RED**: Run failing tests, capture output
2. **GREEN**: Fix dict key borrowing in convert_index()
3. **GREEN**: Ensure .get() always borrows keys
4. **REFACTOR**: Verify all 12 tests pass
5. **VERIFY**: Ensure config_manager compiles (3 E0382 fixed)

## Files to Modify

1. `crates/depyler-core/src/rust_gen/expr_gen.rs`
   - Line ~9760-9776: Dict `.get()` key borrowing
   - Line ~193-258: `in` operator `.get().is_some()` key borrowing

2. `crates/depyler-core/tests/depyler_0303_dict_methods_test.rs`
   - Update assertions if needed

## Success Criteria

- ✅ All 12 test failures resolved
- ✅ 3 config_manager E0382 errors fixed
- ✅ config_manager compiles (0 errors)
- ✅ No new regressions
- ✅ make lint passes
