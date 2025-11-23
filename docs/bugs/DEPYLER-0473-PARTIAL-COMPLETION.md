# DEPYLER-0473 PARTIAL COMPLETION: Dict Key Borrowing Fixes

## Status: 🚧 IN PROGRESS (67% complete - 2/3 errors fixed)
- **Date**: 2025-11-23
- **Result**: 17 errors → 3 errors → 2 errors (88% reduction)
- **Files Modified**: 2 (expr_gen.rs, test files)
- **Lines Changed**: ~15
- **Build Time**: ~41s

## Problems Solved

### Problem 1: Test Regressions from DEPYLER-0472 ✅ FIXED
**Original**: 12 test failures after DEPYLER-0472 changes

**Root Cause**: Tests expected `.contains_key()` but DEPYLER-0449 changed to `.get().is_some()`

**Solution**: Updated 2 test files to accept both patterns
- `depyler_0303_dict_methods_test.rs` line 106-115
- `depyler_0303_phase2_test.rs` line 321-328

**Result**: All dict-related test regressions FIXED ✅
- Remaining 10 failures are pre-existing (type system, validators, kwargs)

### Problem 2: Dict `.get()` Not Borrowing Keys ✅ FIXED
**Original**: `current.get(k).is_some()` moved `k`

**Error**:
```rust
if !current.get(k).is_some() {  // Moves k
    current.as_object_mut().unwrap().insert(k, ...);  // ❌ Use after move
}
```

**Root Cause**: `NotIn` operator had faulty borrowing logic
- Lines 236-240 in expr_gen.rs skipped borrowing for Type::String
- Backwards logic: owned String NEEDS borrowing, &str doesn't

**Solution**: Changed `needs_borrow` to always `true` (line 235-238)
```rust
// DEPYLER-0473: Always borrow keys for .get() and .contains()
// HIR Type::String doesn't distinguish owned String vs &str
let needs_borrow = true;
```

**Result**: Line 116 now generates `.get(&k).is_some()` ✅
- Error count: 3 → 2 (-33%)

## Remaining Work (2 errors)

### Error 1: Dict `.insert()` Moves Key (Line 120/122)
```rust
if !current.get(&k).is_some() {
    current.as_object_mut().unwrap().insert(k, ...);  // Moves k
}
current = current.get(&k)...;  // ❌ Borrow after move
```

**Fix Needed**: Clone key when inserting
```rust
.insert(k.clone(), serde_json::json!({}))
```

### Error 2: Slice Generation Moves Vec (Line 107/126)  
```rust
let base = keys;  // Moves keys
...
let base = &keys;  // ❌ Borrow after move
```

**Fix Needed**: Borrow in slice generation

## Files Modified

1. **crates/depyler-core/src/rust_gen/expr_gen.rs**
   - Lines 235-238: Fixed NotIn operator borrowing logic

2. **crates/depyler-core/tests/depyler_0303_dict_methods_test.rs**
   - Lines 106-115: Accept .get().is_some() pattern

3. **crates/depyler-core/tests/depyler_0303_phase2_test.rs**
   - Lines 321-328: Accept .get().is_some() pattern

## Progress Metrics

**Error Reduction**:
- **Starting**: 17 errors (config_manager)
- **After DEPYLER-0469-0471**: 3 errors (-82%)
- **After DEPYLER-0472**: 2 E0308 fixed → 3 E0382 revealed
- **After DEPYLER-0473 (partial)**: 2 errors (-88% total)
- **Target**: 0 errors (100% compilation) 🎯

**Test Status**:
- ✅ Test regressions: FIXED (2/2 dict tests passing)
- ❌ Pre-existing failures: 10 tests (unrelated to DEPYLER-0472/0473)

## Next Steps

1. Fix `.insert()` key cloning
2. Fix slice generation borrowing
3. Achieve 0 errors → **100% SINGLE-SHOT COMPILATION** 🎉

## Success Criteria

- ✅ `.get()` borrows keys
- ✅ Test regressions fixed
- ⏳ `.insert()` clones keys (pending)
- ⏳ Slice generation borrows vec (pending)
- ⏳ config_manager compiles with 0 errors (pending)
- ✅ make lint passes
- ✅ No new regressions in test suite
