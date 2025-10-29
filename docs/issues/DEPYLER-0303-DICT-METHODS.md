# DEPYLER-0303: Dictionary/HashMap Method Translation Gaps

**Discovered**: 2025-10-29 during Example 09 (Dictionary Operations) validation
**Status**: 🛑 BLOCKING - 14 compilation errors
**Priority**: P1 (fundamental data structure - high ROI)
**Estimate**: 4-6 hours (medium complexity, multiple issues)

## Overview

Transpiled Example 09 (26 dict functions) revealed **14 compilation errors** due to incorrect HashMap method translations, type mismatches, and ownership issues.

## Discovery Context

**Example**: python-to-rust-conversion-examples/examples/09_dictionary_operations/
**Functions**: 26 dictionary manipulation functions
**Success Rate**: 46% (12/26 functions compile)
**Error Rate**: 54% (14/26 functions fail)

**Validation Command**:
```bash
rustc --crate-type lib .../column_b/src/lib.rs 2>&1 | grep "error\[" | wc -l
# Result: 14 errors
```

## Error Categories

### Category 1: Option/None Type Confusion (1 error - Easy)

#### Error 1: `.is_none()` called on `i32`

**Python**:
```python
def get_without_default(d: dict[str, int], key: str) -> int:
    result = d.get(key)
    if result is None:
        return -1
    return result
```

**Generated Code** (WRONG):
```rust
pub fn get_without_default(d: &HashMap<String, i32>, key: &str) -> i32 {
    let result = d.get(key).cloned().unwrap_or_default();
    if result.is_none() {  // ❌ Error: i32 doesn't have is_none()
        return -1;
    }
    result
}
```

**Error Message**:
```
error[E0599]: no method named `is_none` found for type `i32` in the current scope
```

**Root Cause**: Transpiler translates `d.get(key)` to `.get(key).cloned().unwrap_or_default()` which returns `i32`, not `Option<i32>`. The subsequent `is None` check then tries to call `.is_none()` on the unwrapped value.

**Fix**: Don't unwrap `.get()` when followed by `is None` check:
```rust
// Correct:
pub fn get_without_default(d: &HashMap<String, i32>, key: &str) -> i32 {
    let result = d.get(key).cloned();  // Returns Option<i32>
    if result.is_none() {
        return -1;
    }
    result.unwrap()
}

// Or better:
pub fn get_without_default(d: &HashMap<String, i32>, key: &str) -> i32 {
    d.get(key).cloned().unwrap_or(-1)
}
```

**Complexity**: Easy (pattern recognition in codegen)

---

### Category 2: HashMap Key Type Mismatches (3 errors - Medium)

#### Error 2: `&String` vs `&str` in `.contains_key()` and `.remove()`

**Python**:
```python
def remove_entry_pop(d: dict[str, int], key: str) -> dict[str, int]:
    if key in d:
        d.pop(key)
    return d
```

**Generated Code** (WRONG):
```rust
pub fn remove_entry_pop(d: HashMap<String, i32>, key: &str) -> HashMap<String, i32> {
    let _cse_temp_0 = d.contains_key(&key);  // ❌ Error
    if _cse_temp_0 {
        d.remove(&key).expect("KeyError: key not found");  // ❌ Error
    }
    d
}
```

**Error Messages**:
```
error[E0277]: the trait bound `String: Borrow<&str>` is not satisfied
   --> line 60
    |
 60 |     let _cse_temp_0 = d.contains_key(&key);
```

**Root Cause**: HashMap with `String` keys needs `&str` for lookups, but transpiler passes `&&str` (double reference). Rust's `Borrow` trait allows `HashMap<String, V>` to accept `&str` for lookups, but not `&&str`.

**Fix**:
```rust
// Python: key in d
// Wrong: d.contains_key(&key)  // key is &str, &key is &&str
// Correct: d.contains_key(key)  // key is &str (correct)

// Python: d.pop(key)
// Wrong: d.remove(&key)  // key is &str, &key is &&str
// Correct: d.remove(key)  // key is &str (correct)
```

**Affected Functions**: 3 errors
- `remove_entry_pop`: `.contains_key(&key)`, `.remove(&key)`
- `pop_entry`: `.remove(&key)`
- `pop_entry_no_default`: `.contains_key(&key)`, `.remove(&key)`

**Complexity**: Medium (need to detect when key is already a reference)

---

#### Error 3: `Cow<'_, str>` in `.contains_key()` (Type over-complication)

**Python**:
```python
def has_key(d: dict[str, int], key: str) -> bool:
    return key in d
```

**Generated Code** (WRONG):
```rust
pub fn has_key(d: &HashMap<String, i32>, key: Cow<'_, str>) -> bool {
    d.contains_key(&key)  // ❌ Error
}
```

**Error Message**:
```
error[E0277]: the trait bound `String: Borrow<Cow<'_, str>>` is not satisfied
```

**Root Cause**: Transpiler unnecessarily generates `Cow<'_, str>` for string parameters. Should be simple `&str`.

**Fix**:
```rust
// Correct:
pub fn has_key(d: &HashMap<String, i32>, key: &str) -> bool {
    d.contains_key(key)
}
```

**Complexity**: Easy (fix signature generation for string parameters)

---

### Category 3: Iterator Type Mismatches (2 errors - Medium)

#### Error 4: `.insert()` with references in `for` loop

**Python**:
```python
def update_dict(d1: dict[str, int], d2: dict[str, int]) -> dict[str, int]:
    d1.update(d2)
    return d1
```

**Generated Code** (WRONG):
```rust
pub fn update_dict(d1: HashMap<String, i32>, d2: &HashMap<String, i32>) -> HashMap<String, i32> {
    for (k, v) in d2 {  // k and v are &String and &i32
        d1.insert(k, v);  // ❌ Error: expected String, i32 found &String, &i32
    }
    d1
}
```

**Error Message**:
```
error[E0308]: arguments to this method are incorrect
  --> line 94
   |
94 |         d1.insert(k, v);
   |            ^^^^^^ - - argument of type `&i32` is incorrect
   |                  |
   |                  argument of type `&String` is incorrect
```

**Root Cause**: Iterating over `&HashMap` yields `(&K, &V)`, but `.insert()` expects `(K, V)`. Need to clone.

**Fix**:
```rust
// Correct:
pub fn update_dict(d1: HashMap<String, i32>, d2: &HashMap<String, i32>) -> HashMap<String, i32> {
    for (k, v) in d2 {
        d1.insert(k.clone(), *v);  // Clone String, dereference i32
    }
    d1
}
```

**Affected Functions**: 2 errors
- `update_dict`: `.insert(k, v)` with `(&String, &i32)`
- `merge_dicts`: `.insert(k, v)` with `(&String, &i32)`

**Complexity**: Medium (need to detect iterator reference types and add `.clone()` / `*` dereference)

---

#### Error 5: `.zip()` returns references not owned values

**Python**:
```python
def create_from_lists(keys: list[str], values: list[int]) -> dict[str, int]:
    return dict(zip(keys, values))
```

**Generated Code** (WRONG):
```rust
pub fn create_from_lists(keys: Vec<String>, values: Vec<i32>) -> HashMap<String, i32> {
    keys.iter()
        .zip(values.iter())
        .into_iter()
        .collect::<HashMap<_, _>>()  // ❌ Error: expected HashMap<String, i32>, found HashMap<&String, &i32>
}
```

**Error Message**:
```
error[E0308]: mismatched types
   --> line 120
   |
120 |     keys.iter()
    |     ^^^^^^^^^^^ expected HashMap<String, i32>, found HashMap<&String, &i32>
```

**Root Cause**: `.iter()` yields references, so `.zip()` creates `(&String, &i32)` pairs. Need `.cloned()` to convert to owned values.

**Fix**:
```rust
// Correct:
pub fn create_from_lists(keys: Vec<String>, values: Vec<i32>) -> HashMap<String, i32> {
    keys.into_iter()  // Consume vectors, yield owned values
        .zip(values.into_iter())
        .collect::<HashMap<_, _>>()
}

// Or with borrowing:
pub fn create_from_lists(keys: &Vec<String>, values: &Vec<i32>) -> HashMap<String, i32> {
    keys.iter()
        .cloned()  // Convert &String to String
        .zip(values.iter().cloned())  // Convert &i32 to i32
        .collect::<HashMap<_, _>>()
}
```

**Complexity**: Medium (need to choose `.into_iter()` vs `.iter().cloned()` based on ownership)

---

### Category 4: Missing Operator Support (1 error - Easy)

#### Error 6: HashMap pipe operator `|` not supported

**Python**:
```python
def merge_with_pipe(d1: dict[str, int], d2: dict[str, int]) -> dict[str, int]:
    return d1 | d2  # Python 3.9+ dict merge operator
```

**Generated Code** (WRONG):
```rust
pub fn merge_with_pipe(d1: HashMap<String, i32>, d2: &HashMap<String, i32>) -> HashMap<String, i32> {
    d1 | d2  // ❌ Error: no operator | for HashMap
}
```

**Error Message**:
```
error[E0369]: no implementation for `HashMap<String, i32> | &HashMap<String, i32>`
   --> line 145
   |
145 |     d1 | d2
    |     -- ^ -- &HashMap<String, i32>
    |     |
    |     HashMap<String, i32>
```

**Root Cause**: Rust HashMap doesn't support `|` operator. Python's `|` is dict merge (Python 3.9+).

**Fix**: Translate to `.extend()` or manual iteration:
```rust
// Correct:
pub fn merge_with_pipe(d1: HashMap<String, i32>, d2: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut result = d1;
    for (k, v) in d2 {
        result.insert(k.clone(), *v);
    }
    result
}

// Or use drain if d2 is owned:
pub fn merge_with_pipe(d1: HashMap<String, i32>, d2: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut result = d1;
    result.extend(d2);
    result
}
```

**Complexity**: Easy (add case in binary operator handler for dict `|`)

---

### Category 5: Type Inference Issues (1 error - Easy)

#### Error 7: `.sum::<f64>()` on `&i32` iterator

**Python**:
```python
def average_values(d: dict[str, int]) -> float:
    if len(d) == 0:
        return 0.0
    return sum(d.values()) / len(d)
```

**Generated Code** (WRONG):
```rust
pub fn average_values(d: &HashMap<String, i32>) -> Result<f64, ZeroDivisionError> {
    if d.len() == 0 {
        return Ok(0.0);
    }
    Ok(
        (d.values().cloned().collect::<Vec<_>>().iter().sum::<f64>() as f64)
            / (d.len() as f64),
    )
}
```

**Error Message**:
```
error[E0277]: a value of type `f64` cannot be made by summing an iterator over elements of type `&i32`
   --> line 194
   |
194 |         (d.values().cloned().collect::<Vec<_>>().iter().sum::<f64>() as f64)
```

**Root Cause**: `.cloned()` produces `Vec<i32>`, then `.iter()` borrows it again yielding `&i32`, but `.sum::<f64>()` expects iterator of numeric types that can be summed into `f64`.

**Fix**:
```rust
// Correct Option 1: Don't re-borrow after cloned
Ok(
    (d.values().cloned().sum::<i32>() as f64) / (d.len() as f64)
)

// Correct Option 2: Use map to convert
Ok(
    d.values().map(|&v| v as f64).sum::<f64>() / (d.len() as f64)
)
```

**Complexity**: Easy (remove redundant `.collect().iter()` pattern)

---

### Category 6: Ownership/Mutability Issues (2 errors - Medium)

#### Error 8: Immutable HashMap in mutating methods

**Python**:
```python
def add_entry(d: dict[str, int], key: str, value: int) -> dict[str, int]:
    d[key] = value
    return d
```

**Generated Code** (WRONG):
```rust
pub fn add_entry(d: HashMap<String, i32>, key: String, value: i32) -> HashMap<String, i32> {
    d.insert(key, value);  // ❌ Error: cannot borrow `d` as mutable
    d
}
```

**Error Messages**:
```
error[E0596]: cannot borrow `d` as mutable, as it is not declared as mutable
  --> line 53
   |
53 |     d.insert(key, value);
```

**Root Cause**: Parameter `d` is not marked as `mut`, but `.insert()` requires mutable reference.

**Fix**:
```rust
// Correct:
pub fn add_entry(mut d: HashMap<String, i32>, key: String, value: i32) -> HashMap<String, i32> {
    d.insert(key, value);
    d
}
```

**Affected Functions**: 2 errors
- `add_entry`: `.insert()` on immutable HashMap
- `clear_dict`: `.clear()` on immutable HashMap

**Complexity**: Medium (need to detect mutating methods and add `mut` to parameters)

---

## Error Summary by Function

| Function | Error Type | Category | Difficulty |
|----------|-----------|----------|------------|
| `get_without_default` | `.is_none()` on i32 | Option confusion | Easy |
| `remove_entry_pop` | `&&str` vs `&str` (2 errors) | Key type mismatch | Medium |
| `pop_entry` | `&&str` vs `&str` | Key type mismatch | Medium |
| `pop_entry_no_default` | `&&str` vs `&str` (2 errors) | Key type mismatch | Medium |
| `add_entry` | Immutable HashMap | Ownership/mutability | Medium |
| `clear_dict` | Immutable HashMap | Ownership/mutability | Medium |
| `update_dict` | `.insert(&K, &V)` | Iterator references | Medium |
| `has_key` | `Cow<'_, str>` | Type over-complication | Easy |
| `create_from_lists` | `.zip()` yields refs | Iterator references | Medium |
| `merge_dicts` | `.insert(&K, &V)` | Iterator references | Medium |
| `merge_with_pipe` | `HashMap \| HashMap` | Missing operator | Easy |
| `average_values` | `.sum::<f64>()` on `&i32` | Type inference | Easy |

**Total**: 14 errors across 12 functions (46% failure rate)

---

## Recommended Fix Priority

### P0 (High ROI - 5 errors, 1-2 hours)

1. ✅ **Fix `&&str` vs `&str` in key lookups** (3 errors - Easy)
   - Don't add `&` when parameter is already `&str`
   - Affects: `remove_entry_pop`, `pop_entry`, `pop_entry_no_default`

2. ✅ **Fix immutable HashMap parameters** (2 errors - Easy)
   - Add `mut` to parameters for mutating methods (`.insert()`, `.clear()`)
   - Affects: `add_entry`, `clear_dict`

**Time**: 1-2 hours → 5 errors fixed (2.5-5 errors/hour - HIGH ROI)

---

### P1 (Medium ROI - 4 errors, 2-3 hours)

3. ⚠️ **Fix Option unwrapping with None checks** (1 error - Medium)
   - Don't unwrap `.get()` when followed by `is None` check
   - Affects: `get_without_default`

4. ⚠️ **Fix iterator reference types in loops** (2 errors - Medium)
   - Add `.clone()` / `*` dereference for `(&K, &V)` in `.insert()`
   - Affects: `update_dict`, `merge_dicts`

5. ⚠️ **Fix Cow<str> over-complication** (1 error - Easy)
   - Use simple `&str` instead of `Cow<'_, str>` for string parameters
   - Affects: `has_key`

**Time**: 2-3 hours → 4 errors fixed (1.3-2 errors/hour - MODERATE ROI)

---

### P2 (Lower ROI - 3 errors, 1-2 hours)

6. ⚠️ **Fix zip iterator ownership** (1 error - Medium)
   - Use `.into_iter()` instead of `.iter()` for owned zip
   - Affects: `create_from_lists`

7. ⚠️ **Add dict merge operator support** (1 error - Easy)
   - Translate Python `d1 | d2` to `.extend()` pattern
   - Affects: `merge_with_pipe`

8. ⚠️ **Fix sum type inference** (1 error - Easy)
   - Remove redundant `.collect().iter()` pattern
   - Affects: `average_values`

**Time**: 1-2 hours → 3 errors fixed (1.5-3 errors/hour - MODERATE ROI)

---

## Implementation Plan

### Phase 1: Quick Wins (1-2 hours, 5 errors)

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`

1. **Fix key reference handling** (30 min):
   ```rust
   // In membership test (key in dict):
   if parameter_already_ref(key) {
       Ok(parse_quote! { #dict.contains_key(#key) })  // Don't add &
   } else {
       Ok(parse_quote! { #dict.contains_key(&#key) })  // Add &
   }

   // In .pop() → .remove():
   if parameter_already_ref(key) {
       Ok(parse_quote! { #dict.remove(#key) })  // Don't add &
   } else {
       Ok(parse_quote! { #dict.remove(&#key) })  // Add &
   }
   ```

2. **Fix mutability detection** (1 hour):
   ```rust
   // Detect mutating methods on HashMap parameters
   fn needs_mut_param(method: &str) -> bool {
       matches!(method, "insert" | "remove" | "clear" | "extend" | "drain")
   }

   // In function signature generation:
   if is_hashmap_param(param) && modifies_param_in_body(param, body) {
       parse_quote! { mut #param }  // Add mut
   }
   ```

---

### Phase 2: Medium Wins (2-3 hours, 4 errors)

3. **Fix Option unwrapping patterns** (1 hour):
   - Detect `.get()` followed by `is None` check
   - Don't unwrap, keep as Option

4. **Fix iterator reference cloning** (1 hour):
   - Detect `for (k, v) in &hashmap`
   - Add `.clone()` for `K`, `*` for `V` in `.insert()`

5. **Fix Cow parameter generation** (30 min):
   - Use `&str` instead of `Cow<'_, str>` for string parameters

---

### Phase 3: Remaining Issues (1-2 hours, 3 errors)

6. **Fix zip ownership** (30 min):
   - Use `.into_iter()` for owned values in zip

7. **Add dict merge operator** (30 min):
   - Translate `d1 | d2` to `.extend()` pattern

8. **Fix sum type inference** (30 min):
   - Remove `.collect().iter()` after `.cloned()`

---

## Testing Strategy

**Test Cases Needed**:
```python
# Key reference handling
assert has_key({"a": 1}, "a") == True

# Mutability
d = {"a": 1}
add_entry(d, "b", 2)
assert d == {"a": 1, "b": 2}

# Option handling
assert get_without_default({}, "x") == -1

# Iterator cloning
d1 = {"a": 1}
d2 = {"b": 2}
result = update_dict(d1, d2)
assert result == {"a": 1, "b": 2}

# Dict merge operator
assert merge_with_pipe({"a": 1}, {"b": 2}) == {"a": 1, "b": 2}

# Zip ownership
keys = ["a", "b"]
values = [1, 2]
assert create_from_lists(keys, values) == {"a": 1, "b": 2}
```

---

## ROI Analysis

**Time Investment**: 4-6 hours (all phases)
**Error Reduction**: 14 errors → 0 errors (100%)
**Functions Fixed**: 12/26 functions (46% → 100%)
**Strategic Value**: Dictionaries are fundamental - highest frequency after lists/strings

**Quick Wins (Phase 1)**:
- 1-2 hours → 5 errors fixed (2.5-5 errors/hour)
- 19% improvement
- **HIGH ROI**

**Complete Fix (All Phases)**:
- 6 hours → 14 errors fixed (2.3 errors/hour)
- 100% Example 09 success
- **MODERATE ROI**

---

## Dependencies

**Required**:
- `is_hashmap_type()` / `is_dict_base()` heuristics (may need enhancement)
- Mutability analysis for function parameters
- Reference depth detection (distinguish `&str` from `&&str`)

**Blockers**: None

---

## Related Issues

- **DEPYLER-0298**: Complex comprehension targets (tuple unpacking `for k, v in d.items()`) - not fixed here
- **DEPYLER-0302**: String method gaps (similar pattern of missing method translations)
- **DEPYLER-0299**: List comprehension iterator issues (similar `.iter()` vs `.into_iter()` patterns)

---

## Recommendation

**Immediate Action**: Fix Phase 1 (Quick Wins) - 1-2 hours, 5 errors
- High ROI (2.5-5 errors/hour)
- Low risk (simple fixes)
- Unblocks 19% of Example 09 functions immediately

**Strategic**: Complete Phase 1 + Phase 2 (3-5 hours, 9 errors)
- Good ROI (1.8-3 errors/hour)
- Addresses most common use cases
- 35% of Example 09 functions fixed

**Defer**: Phase 3 can wait until other high-priority issues addressed

---

## Conclusion

Example 09 validation successfully applied STOP THE LINE protocol, discovering 14 dictionary/HashMap method translation gaps. This validates the Matrix Project strategy of discovering patterns before fixing.

**Next Steps**:
1. ✅ Document bugs (this ticket)
2. ⏸️ Continue Matrix Project to discover more patterns
3. 🎯 Batch-fix dict methods in dedicated sprint
4. 📋 Re-validate Example 09 after fixes

**Status**: Documented, ready for implementation
