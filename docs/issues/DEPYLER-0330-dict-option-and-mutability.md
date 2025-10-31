# DEPYLER-0330: Fix dict.get() Option Handling & Parameter Mutability Detection

**Status**: 🔴 Open
**Priority**: P1 - Critical (blocks 70% pass rate milestone)
**Estimate**: 30-60 minutes
**Type**: Transpiler Bug - Code Generation
**Affects**: 09_dictionary_operations (3 remaining errors)

## Summary

Two distinct issues in 09_dictionary_operations preventing compilation:
1. **Option handling**: `.get()` result unwrapped then checked with `.is_none()`
2. **Parameter mutability**: Functions calling `.remove()` need `&mut HashMap` parameters

**Impact**: Blocking 9/13 compilation (69.2% pass rate / 70% milestone)

---

## Issue #1: dict.get() with None Check

### Error
```
error[E0599]: no method named `is_none` found for type `i32` in the current scope
  --> src/lib.rs:43:15
   |
43 |     if result.is_none() {
   |               ^^^^^^^
```

### Python Source (Line 33-38)
```python
def get_without_default(d: dict[str, int], key: str) -> int:
    """Get value or None if key doesn't exist."""
    result = d.get(key)
    if result is None:
        return -1
    return result
```

### Generated Rust (WRONG)
```rust
pub fn get_without_default(d: &HashMap<String, i32>, key: &str) -> i32 {
    let result = d.get(key).cloned().unwrap_or_default();  // ❌ Unwraps to i32
    if result.is_none() {  // ❌ i32 doesn't have .is_none()
        return -1;
    }
    result
}
```

### Expected Rust (CORRECT)
```rust
pub fn get_without_default(d: &HashMap<String, i32>, key: &str) -> i32 {
    d.get(key).cloned().unwrap_or(-1)  // ✅ One expression, correct default
}
```

OR:
```rust
pub fn get_without_default(d: &HashMap<String, i32>, key: &str) -> i32 {
    match d.get(key).cloned() {
        Some(v) => v,
        None => -1,
    }
}
```

### Root Cause

**Pattern**: Python code that assigns `d.get(key)` then checks `is None`

Transpiler currently:
1. Generates `.get().cloned().unwrap_or_default()` (unwraps the Option)
2. Generates `if result.is_none()` check (but result is already unwrapped!)

**Should**:
1. Keep the Option: `let result = d.get(key).cloned();`
2. Check the Option: `if result.is_none()`
3. OR: Use `.unwrap_or(-1)` directly without intermediate variable

### Fix Strategy

**Option A: Detect pattern and optimize** (RECOMMENDED)
```rust
// In statement generation for dict.get() assignment + None check
if is_dict_get_with_none_check(stmt, next_stmt) {
    // Generate: d.get(key).cloned().unwrap_or(default_value)
    // Skip the intermediate variable and None check
}
```

**Option B: Don't unwrap when None check follows**
```rust
// In dict.get() translation
if followed_by_none_check() {
    // Generate: let result = d.get(key).cloned();  // Keep Option
} else {
    // Generate: let result = d.get(key).cloned().unwrap_or_default();
}
```

---

## Issue #2: .remove() Needs Mutable Parameter

### Errors (2 occurrences)
```
error[E0596]: cannot borrow `*d` as mutable, as it is behind a `&` reference
  --> src/lib.rs:69:5
   |
69 |     d.remove(key).unwrap_or(-1)
   |     ^ `d` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference
   |
68 | pub fn pop_entry(d: &mut HashMap<String, i32>, key: &str) -> i32 {
   |                      +++
```

### Python Source (Line 58-60)
```python
def pop_entry(d: dict[str, int], key: str) -> int:
    """Remove and return value."""
    return d.pop(key, -1)
```

### Generated Rust (WRONG)
```rust
pub fn pop_entry(d: &HashMap<String, i32>, key: &str) -> i32 {
    d.remove(key).unwrap_or(-1)  // ❌ .remove() needs &mut
}
```

### Expected Rust (CORRECT)
```rust
pub fn pop_entry(d: &mut HashMap<String, i32>, key: &str) -> i32 {
    //                 ^^^ Added mut
    d.remove(key).unwrap_or(-1)  // ✅ Can mutate now
}
```

### Root Cause

**Pattern**: Python dict methods that modify the dictionary

Methods requiring `&mut`:
- `d.pop(key)` → `d.remove(key)` - Requires &mut
- `d.clear()` → `d.clear()` - Requires &mut
- `d.update(other)` → `d.extend()` - Requires &mut
- `d[key] = value` → `d.insert(key, value)` - Requires &mut

Transpiler currently generates `&HashMap` for all dict parameters.

**Should**: Analyze function body to detect mutation and generate `&mut HashMap`.

### Fix Strategy

**Option A: Analyze function body for mutations** (RECOMMENDED)
```rust
// In function parameter type generation
fn infer_param_mutability(func: &HirFunction, param: &HirParam) -> bool {
    // Check if param is mutated in function body
    for stmt in &func.body {
        if calls_mutating_method(stmt, &param.name) {
            return true;  // Generate &mut
        }
    }
    false  // Generate &
}

fn calls_mutating_method(stmt: &HirStmt, param_name: &str) -> bool {
    // Check for: param.remove(), param.clear(), param.insert(), etc.
    match stmt {
        HirStmt::Expr(HirExpr::MethodCall { object, method, .. }) => {
            if let HirExpr::Var(var) = object.as_ref() {
                if var == param_name && is_mutating_method(method) {
                    return true;
                }
            }
        }
        // ... check other statement types
    }
    false
}

fn is_mutating_method(method: &str) -> bool {
    matches!(method, "remove" | "clear" | "insert" | "extend" | "push" | "pop")
}
```

**Option B: Use Python type hints** (if available)
```python
# If Python has mutable annotation (unlikely):
def pop_entry(d: MutableMapping[str, int], key: str) -> int:
    ...
```

**Option C: Heuristic based on method name**
- If function name contains "pop", "remove", "clear", "update" → assume mutation
- Not reliable, prefer Option A

---

## Affected Code Locations

### Issue #1: get_without_default
- File: `09_dictionary_operations/column_b/src/lib.rs`
- Line: 41-47
- Function: `get_without_default`

### Issue #2: pop_entry (2 functions)
- File: `09_dictionary_operations/column_b/src/lib.rs`
- Lines: 68-70 (`pop_entry`)
- Lines: 74-80 (`pop_entry_no_default`)

---

## Implementation Plan

### Phase 1: Fix Issue #2 (Mutability Detection) - 20 min

**Easier to implement**: Just need to detect mutating methods and add `mut` to parameter.

1. Add `is_mutating_method()` helper in `rust_gen/func_gen.rs`
2. Add `param_is_mutated()` analyzer
3. Modify parameter type generation to use `&mut` when needed
4. Test on `pop_entry` functions

### Phase 2: Fix Issue #1 (Option Handling) - 30 min

**More complex**: Need to detect pattern across statements.

1. Add pattern detection for `result = dict.get()`  followed by `if result is None:`
2. Optimize to single expression: `.unwrap_or(default)`
3. OR: Keep Option and generate proper None check
4. Test on `get_without_default`

### Total Estimate: 50 minutes

---

## Test Cases

### Test Case 1: get_without_default
```python
def get_without_default(d: dict[str, int], key: str) -> int:
    result = d.get(key)
    if result is None:
        return -1
    return result
```

**Expected Rust**:
```rust
pub fn get_without_default(d: &HashMap<String, i32>, key: &str) -> i32 {
    d.get(key).cloned().unwrap_or(-1)
}
```

### Test Case 2: pop_entry
```python
def pop_entry(d: dict[str, int], key: str) -> int:
    return d.pop(key, -1)
```

**Expected Rust**:
```rust
pub fn pop_entry(d: &mut HashMap<String, i32>, key: &str) -> i32 {
    d.remove(key).unwrap_or(-1)
}
```

### Test Case 3: pop_entry_no_default
```python
def pop_entry_no_default(d: dict[str, int], key: str) -> int:
    if key in d:
        return d.pop(key)
    return -1
```

**Expected Rust**:
```rust
pub fn pop_entry_no_default(d: &mut HashMap<String, i32>, key: &str) -> i32 {
    if d.contains_key(key) {
        return d.remove(key).expect("KeyError: key not found");
    }
    -1
}
```

---

## Success Criteria

✅ 09_dictionary_operations compiles: 3 errors → 0 errors
✅ `get_without_default` generates `.unwrap_or(-1)` or proper match
✅ `pop_entry` and `pop_entry_no_default` have `&mut HashMap` parameters
✅ Matrix Project: 8/13 → **9/13 compiling (69.2%)**
✅ **70% pass rate milestone achieved!** 🎉
✅ No regressions in existing tests

---

## Files to Modify

1. `crates/depyler-core/src/rust_gen/func_gen.rs` - Parameter mutability
2. `crates/depyler-core/src/rust_gen/stmt_gen.rs` or `expr_gen.rs` - Option handling
3. `python-to-rust-conversion-examples/examples/09_dictionary_operations/column_b/src/lib.rs` - Retranspile

---

## Related Issues

- DEPYLER-0304: HashMap auto-borrowing (related to parameter references)
- DEPYLER-0326: .contains_key() borrowing (same file)
- DEPYLER-0328: sum() type inference (same file)

---

## Priority Justification

**P1 - Critical** because:
1. Blocks 70% pass rate milestone (currently 61.5%)
2. Only 3 errors remaining in this example
3. Estimated 50 minutes to complete
4. High impact for low effort (quick win)
5. Validates parameter mutability detection (important feature)

---

**Next Step**: Implement Phase 1 (mutability detection) first as it's simpler and fixes 2/3 errors.
