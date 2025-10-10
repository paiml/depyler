# Showcase Example Issues - v3.14.0

## Summary

**Status**: 4/6 examples compile cleanly (67%)
- ✅ binary_search.rs - 0 warnings
- ✅ calculate_sum.rs - 0 warnings
- ✅ process_config.rs - 0 warnings
- ⚠️ classify_number.rs - 1 warning (unused import)
- ❌ contracts_example.rs - 2 compilation errors
- ❌ annotated_example.rs - 3 compilation errors

## Issue 1: classify_number.rs - Unused Import (Minor)

**Severity**: P3 (Warning only)
**File**: examples/showcase/classify_number.rs
**Error**: `warning: unused import: 'std::borrow::Cow'`

**Root Cause**:
- Transpiler adds `use std::borrow::Cow;` unconditionally
- Located in `rust_gen.rs:285` controlled by `ctx.needs_cow`
- Lifetime analysis is setting `needs_cow = true` even when not needed
- Triggered in 4 locations (lines 834, 986, 3689, 4054)

**Impact**: Cosmetic - generates clippy warning but code works correctly

**Fix**: Requires refining lifetime/borrowing strategy analysis to only set `needs_cow` when actually generating `Cow<'a, str>` types

**Recommendation**: Document for v3.15.0, not critical for v3.14.0

---

## Issue 2: contracts_example.rs - Type Inference Bug (Critical)

**Severity**: P0 (Compilation failure)
**File**: examples/showcase/contracts_example.rs

**Errors**:
1. `error[E0277]: cannot add '&f64' to '{integer}'` (line 50)
2. `error[E0308]: mismatched types - expected 'f64', found integer` (line 53)

**Root Cause**:
```python
# Python source (list_sum function)
def list_sum(numbers: list[float]) -> float:
    total = 0  # <-- Problem: Python infers this as int
    for num in numbers:
        total = total + num  # Python auto-converts int + float
    return total
```

```rust
// Generated Rust (WRONG)
pub fn list_sum<'a>(numbers: & 'a Vec<f64>) -> f64 {
    let mut total = 0;  // <-- Inferred as {integer} (i32 by default)
    for num in numbers.iter() {
        total = total + num;  // ERROR: i32 + &f64 not allowed
    }
    return total;  // ERROR: expected f64, found integer
}
```

**Fix Needed**: Transpiler must:
1. Detect that `total` is assigned from f64 values
2. Initialize `total` as `0.0` (not `0`) to make it f64
3. Or explicitly annotate: `let mut total: f64 = 0.0;`

**Location**: Type inference in `rust_gen.rs` or `type_mapper.rs`

---

## Issue 3: annotated_example.rs - Multiple Type Issues (Critical)

**Severity**: P0 (Compilation failure)
**File**: examples/showcase/annotated_example.rs

**Errors**:

### 3.1: Missing fnv Crate Dependency
```
error[E0432]: unresolved import `fnv`
 --> annotated_example.rs:2:9
2 | use fnv::FnvHashMap;
  |     ^^^ use of undeclared crate or module `fnv`
```

**Root Cause**: Transpiler generates `use fnv::FnvHashMap` but fnv is not in dependencies
- `ctx.needs_fnv_hashmap` is set to true somewhere
- Generated code requires external crate

**Fix**: Either:
- Add fnv to workspace dependencies, OR
- Use std::collections::HashMap instead, OR
- Make fnv import conditional/optional

### 3.2: Lifetime/Ownership Mismatch
```
error[E0308]: mismatched types
49 | pub fn process_text<'a>(text: & 'a str) -> & 'a str {
   |                                              ------- expected '&'a str'
50 |     return text.to_uppercase();
   |            ^^^^^^^^^^^^^^^^^^^ expected '&str', found 'String'
```

**Root Cause**:
- Python: `return text.upper()` returns a new string
- Rust: `text.to_uppercase()` returns `String`, not `&str`
- Function signature specifies returning `&'a str` (borrowed)

**Fix**: Transpiler should recognize string transformation methods return `String`, not `&str`

### 3.3: Type Conversion Bug
```
error[E0308]: mismatched types
77 |     return Ok(Some(_cse_temp_1));
   |                    ^^^^^^^^^^^ expected `f64`, found `i32`
```

**Root Cause**: Similar to Issue #2 - integer literal not inferred as f64

**Fix**: Same as Issue #2 - better type inference for numeric literals in f64 context

---

## Recommendation for v3.15.0

### Immediate (v3.15.0 Phase 1):
1. **DEPYLER-TBD**: Fix numeric type inference for literals in typed contexts
   - When `total: f64` is assigned `0`, generate `0.0`
   - Priority: P0 (blocks 2 showcase examples)

2. **DEPYLER-TBD**: Fix string method return types
   - Methods like `.upper()`, `.lower()` return owned String
   - Should generate `String` return type, not `&str`
   - Priority: P0 (blocks annotated_example)

### Short-term (v3.15.0 Phase 2):
3. **DEPYLER-TBD**: Fix fnv dependency handling
   - Add fnv to workspace deps OR use std HashMap
   - Priority: P1 (workaround available)

### Long-term (v3.15.0 Phase 3):
4. **DEPYLER-TBD**: Improve Cow import detection
   - Only add `use std::borrow::Cow` when actually used
   - Priority: P3 (cosmetic)

## Testing Strategy

For each fix:
1. Add failing test case first
2. Implement fix in transpiler
3. Re-transpile ALL showcase examples
4. Verify all 6 compile cleanly
5. Run validation suite

**Target**: 6/6 showcase examples compile with 0 warnings
