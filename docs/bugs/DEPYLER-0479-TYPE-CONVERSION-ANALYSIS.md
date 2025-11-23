# DEPYLER-0479: Type Conversion & Auto-Borrowing Analysis

**Status**: 🔴 IN PROGRESS
**Date**: 2025-11-23
**Priority**: P0 (STOP THE LINE)
**Blocking**: example_environment compilation (12 errors)

## Summary

Multiple type conversion issues prevent example_environment from compiling. These fall into 4 categories:
1. **Option<String> → &str conversion** (E0277) - Missing unwrap when passing optional params to std::env::var
2. **String slice codegen** (E0599) - Generates `.to_vec()` instead of `.to_string()`
3. **Incorrect type inference** (E0308) - Thinks `.unwrap_or_else()` result is still Option
4. **Missing auto-borrow** (E0308) - Doesn't insert `&` when passing String to functions expecting `&str`

## Problem Details

### Error 1: Option<String> → &str Conversion (E0277)

**Location**: `env_info.rs:51`

**Error**:
```
error[E0277]: the trait bound `Option<std::string::String>: AsRef<std::ffi::OsStr>` is not satisfied
  --> env_info.rs:51:39
   |
51 |         let mut value = std::env::var(var_name).ok();
   |                         ------------- ^^^^^^^^ the trait `AsRef<std::ffi::OsStr>` is not implemented for `Option<std::string::String>`
```

**Python Source** (`env_info.py:45`):
```python
def show_environment(var_name=None):
    if var_name:
        value = os.environ.get(var_name)  # var_name is optional
```

**Generated Rust** (broken):
```rust
pub fn show_environment(var_name: &Option<String>) {
    if var_name.is_some() {
        let mut value = std::env::var(var_name).ok();  // ❌ E0277
        //                              ^^^^^^^^ &Option<String>, but needs &str
```

**Expected Rust**:
```rust
pub fn show_environment(var_name: &Option<String>) {
    if var_name.is_some() {
        let mut value = std::env::var(var_name.as_ref().unwrap()).ok();  // ✅
        //                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ Unwrap Option to &String, auto-borrow to &str
```

**Root Cause**:
- Python optional parameter `var_name=None` transpiles to `var_name: &Option<String>`
- When passing to `std::env::var()`, transpiler doesn't unwrap the Option
- Need to detect when optional parameter is used in non-optional context and insert `.as_ref().unwrap()`

**Alternative Approach**:
Could also use if-let to destructure:
```rust
pub fn show_environment(var_name: &Option<String>) {
    if let Some(name) = var_name {
        let mut value = std::env::var(name).ok();  // ✅ Cleaner
```

---

### Error 2: String Slice Codegen (E0599)

**Location**: `env_info.rs:105`

**Error**:
```
error[E0599]: no method named `to_vec` found for type `str` in the current scope
   --> env_info.rs:105:54
    |
105 |                         base[..stop.min(base.len())].to_vec()
    |                                                      ^^^^^^ method not found in `str`
```

**Python Source** (`env_info.py:58`):
```python
if var == "PATH" and len(value) > 50:
    value = value[:47] + "..."  # String slicing
```

**Generated Rust** (broken):
```rust
if (var == "PATH") && (value.len() as i32 > 50) {
    value = format!(
        "{}{}",
        {
            let base = &value;
            let stop_idx = 47 as isize;
            let stop = if stop_idx < 0 {
                (base.len() as isize + stop_idx).max(0) as usize
            } else {
                stop_idx as usize
            };
            base[..stop.min(base.len())].to_vec()  // ❌ E0599
            //                           ^^^^^^^^ &str has no .to_vec()
        },
        "..."
    );
```

**Expected Rust**:
```rust
base[..stop.min(base.len())].to_string()  // ✅ Correct
```

**Root Cause**:
- String slicing codegen at `expr_gen.rs` (likely in `Subscript` handling)
- Incorrectly uses `.to_vec()` (for Vec<T> → Vec<T> conversion)
- Should use `.to_string()` (for &str → String conversion)

**Fix Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` - Subscript slice handling

---

### Error 3: Incorrect Type Inference After unwrap_or_else (E0308)

**Locations**: `env_info.rs:113-116`

**Errors**:
```
error[E0308]: mismatched types
   --> env_info.rs:114:25
    |
113 |                     match &value {
    |                           ------ this expression has type `&std::string::String`
114 |                         Some(v) => format!("{}", v),
    |                         ^^^^^^^ expected `String`, found `Option<_>`

error[E0308]: mismatched types
   --> env_info.rs:115:25
    |
115 |                         None => "None".to_string(),
    |                         ^^^^ expected `String`, found `Option<_>`
```

**Python Source** (`env_info.py:55-59`):
```python
for var in common_vars:
    value = os.environ.get(var, "(not set)")  # .get() with default returns str, not Optional[str]
    # ... later ...
    print(f"  {var}={value}")  # value is str, not Optional
```

**Generated Rust** (broken):
```rust
for var in common_vars.iter().cloned() {
    let mut value = std::env::var(var)
        .unwrap_or_else(|_| "(not set)".to_string().to_string());
    // value is String (not Option<String>)

    // ... later at line 113-116 ...
    println!("{}", format!("  {}={}", var, {
        match &value {  // ❌ Matching on &String as if it were &Option<String>
            Some(v) => format!("{}", v),  // ❌ E0308
            None => "None".to_string(),   // ❌ E0308
        }
    }));
```

**Expected Rust**:
```rust
for var in common_vars.iter().cloned() {
    let mut value = std::env::var(var)
        .unwrap_or_else(|_| "(not set)".to_string().to_string());
    // value is String

    // ... later ...
    println!("{}", format!("  {}={}", var, value));  // ✅ Just use value directly
```

**Root Cause**:
- Transpiler tracks `os.environ.get(var)` returns `Option<String>` ✅
- But when `.get(var, default)` is used (2-arg form), Python returns `str`, not `Optional[str]` ✅
- Transpiler correctly generates `.unwrap_or_else()` ✅
- **BUG**: Transpiler still thinks `value` has type `Option<String>` after the `unwrap_or_else()` ❌
- Later string formatting incorrectly pattern-matches on Option

**Fix Required**:
- Type inference must recognize that `unwrap_or_else()` unwraps `Result<T, E>` → `T`
- After `std::env::var(var).unwrap_or_else(...)`, type should be `String`, not `Option<String>`

**Fix Location**: `crates/depyler-core/src/type_inference/` or HIR → Rust type tracking

---

### Error 4: Missing Auto-Borrow for Path Operations (E0308)

**Locations**: `env_info.rs:145, 147, 160, 161`

**Errors**:
```
error[E0308]: mismatched types
    --> env_info.rs:145:52
     |
 145 |         format!("Exists: {}", std::path::Path::new(expanded).exists())
     |                               -------------------- ^^^^^^^^ expected `&_`, found `String`
     |                               |
     |                               arguments to this function are incorrect
     |
     = note: expected reference `&_`
                   found struct `std::string::String`
help: consider borrowing here
     |
 145 |         format!("Exists: {}", std::path::Path::new(&expanded).exists())
     |                                                    +
```

**Python Source** (`env_info.py:72-78`):
```python
def check_path(path):
    expanded = os.path.expanduser(path)

    print(f"Path: {path}")
    if path != expanded:
        print(f"Expanded: {expanded}")

    print(f"Exists: {os.path.exists(expanded)}")  # expanded is str
```

**Generated Rust** (broken):
```rust
pub fn check_path(path: String) {
    let expanded = ...;  // expanded: String

    // ...

    println!("{}", format!("Exists: {}",
        std::path::Path::new(expanded).exists()  // ❌ E0308
        //                   ^^^^^^^^ String, but Path::new expects &str
    ));

    if std::path::Path::new(expanded).exists() {  // ❌ E0308 (same issue)
        // ...
    }

    std::fs::canonicalize(expanded)  // ❌ E0308
    //                    ^^^^^^^^ String, but expects &Path
```

**Expected Rust**:
```rust
pub fn check_path(path: String) {
    let expanded = ...;  // expanded: String

    println!("{}", format!("Exists: {}",
        std::path::Path::new(&expanded).exists()  // ✅ Auto-borrow
        //                   ^
    ));

    if std::path::Path::new(&expanded).exists() {  // ✅
        // ...
    }

    std::fs::canonicalize(&expanded)  // ✅
    //                    ^
```

**Root Cause**:
- Rust has auto-deref coercion but NOT auto-borrow coercion for function arguments
- When passing `String` to function expecting `&str`, must explicitly borrow with `&`
- Transpiler doesn't insert `&` automatically

**Fix Required**:
- Detect when passing owned value (`String`, `PathBuf`, etc.) to function expecting reference (`&str`, `&Path`)
- Insert `&` prefix automatically in function call argument codegen
- Alternative: Use `.as_ref()` for trait-based conversion

**Fix Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` - Call expression argument handling

---

## Summary Table

| Error | Location | Category | Root Cause | Complexity |
|-------|----------|----------|------------|------------|
| E0277 | Line 51 | Option unwrap | Missing `.as_ref().unwrap()` on optional param | Medium |
| E0599 | Line 105 | Slice codegen | `.to_vec()` should be `.to_string()` | Low |
| E0308 | Lines 113-116 | Type inference | Type not updated after `unwrap_or_else()` | Medium |
| E0308 | Lines 145, 147, 160 | Auto-borrow | Missing `&` for owned → borrowed conversion | High |

**Total Errors**: 7 compilation errors (1 E0277, 1 E0599, 5 E0308)

---

## Implementation Plan

### Phase 1: Quick Wins (Low Complexity)

**Task 1.1**: Fix string slice `.to_vec()` → `.to_string()`
**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`
**Estimated Time**: 30 minutes
**Impact**: Fixes 1 error (line 105)

**Implementation**:
1. Find Subscript slice handling in `expr_gen.rs`
2. Detect when base type is `String` or `&str`
3. Generate `.to_string()` instead of `.to_vec()` for string slices
4. Add test case for `value[:47]` string slicing

---

### Phase 2: Type Inference (Medium Complexity)

**Task 2.1**: Fix type inference after `unwrap_or_else()`
**File**: `crates/depyler-core/src/type_inference/` or `ast_bridge`
**Estimated Time**: 2 hours
**Impact**: Fixes 2 errors (lines 114, 115)

**Implementation**:
1. Track method call return types in HIR
2. Recognize `Result<T, E>.unwrap_or_else()` returns `T`
3. Update variable type after unwrap in symbol table
4. Prevent incorrect Option pattern matching on non-Option values

**Alternative Quick Fix**:
Detect `os.environ.get(key, default)` (2-arg form) and generate direct `String` type instead of `Option<String>` → avoids unwrap entirely.

**Task 2.2**: Fix optional parameter unwrapping
**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`
**Estimated Time**: 1.5 hours
**Impact**: Fixes 1 error (line 51)

**Implementation**:
1. Detect when `Option<T>` parameter used in non-optional context
2. Insert `.as_ref().unwrap()` or use if-let destructuring
3. Track when optional parameters are known to be Some (e.g., inside `if var.is_some()` block)
4. Generate safe unwrap in that context

---

### Phase 3: Auto-Borrow (High Complexity)

**Task 3.1**: Implement auto-borrow for function arguments
**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`
**Estimated Time**: 3 hours
**Impact**: Fixes 4 errors (lines 145, 147, 160, 161)

**Implementation**:
1. Build function signature database (stdlib + known APIs)
2. For each function call argument:
   - Check argument type (from HIR/inference)
   - Check expected parameter type (from signature)
   - If mismatch: `String` → `&str`, `PathBuf` → `&Path`, etc.
   - Insert `&` prefix or `.as_ref()` call
3. Handle edge cases:
   - Move semantics (don't borrow if value consumed)
   - Multiple borrows (mutable vs immutable)
   - Lifetime constraints

**Challenges**:
- Requires robust type signature database for std library
- Must understand when borrowing is valid (not all owned → ref conversions work)
- May generate unnecessary borrows (performance impact)

**Alternative Simpler Approach**:
- Only handle common cases: `String` → `&str`, `PathBuf` → `&Path`
- Use `.as_ref()` method instead of `&` (more general, works with trait bounds)
- Example: `Path::new(expanded.as_ref())` instead of `Path::new(&expanded)`

---

## Testing Strategy

### Unit Tests (Per Fix)

**Test 1**: String slicing → `.to_string()`
```python
def truncate(s):
    return s[:10]
```
Expected Rust:
```rust
pub fn truncate(s: String) -> String {
    s[..10].to_string()  // Not .to_vec()
}
```

**Test 2**: Type inference after `unwrap_or_else()`
```python
import os
def get_var(key):
    value = os.environ.get(key, "default")
    return value  # Should infer String, not Option<String>
```

**Test 3**: Optional parameter unwrapping
```python
def show(name=None):
    if name:
        value = os.environ.get(name)  # Should unwrap `name` Option
```

**Test 4**: Auto-borrow for Path operations
```python
import os
def exists(p):
    return os.path.exists(p)  # Should generate Path::new(&p)
```

### Integration Test

Re-transpile `example_environment` and verify:
- Compilation succeeds (0 errors)
- All 7 type conversion errors fixed
- Clippy passes with `-D warnings`
- Runtime behavior matches Python original

---

## Risk Analysis

### Risk 1: Over-aggressive Auto-borrowing
**Probability**: Medium
**Impact**: High (incorrect code generation)
**Mitigation**:
- Conservative approach: only auto-borrow for known-safe cases
- Extensive testing with property-based tests
- Fallback: generate compile error hints instead of auto-fix

### Risk 2: Type Inference Complexity
**Probability**: Low
**Impact**: Medium (long implementation time)
**Mitigation**:
- Start with simple cases (`unwrap_or_else`, 2-arg `get()`)
- Incremental rollout per method
- Skip complex control flow initially

### Risk 3: Regressions in Other Examples
**Probability**: Medium
**Impact**: High
**Mitigation**:
- Run full test suite after each fix
- Check all 13 examples for regressions
- Quality gates must pass (make lint, coverage)

---

## Success Criteria

**Minimum (Phase 1)**:
- [ ] String slice generates `.to_string()` ✅
- [ ] example_environment: 12 → 11 errors

**Target (Phases 1-2)**:
- [ ] All type inference errors fixed
- [ ] example_environment: 12 → 7 errors (4 auto-borrow errors remain)
- [ ] No regressions in other examples

**Stretch Goal (All Phases)**:
- [ ] Auto-borrow implemented
- [ ] example_environment: 12 → 0 errors ✅ COMPILES
- [ ] Single-shot compilation: 46% → 54% (7/13 examples)

---

## Next Steps

1. **Start with Phase 1** (string slice fix) - Quick win, low risk
2. **Validate with re-transpile** - Confirm 1 error fixed
3. **Move to Phase 2.2** (optional param unwrapping) - Medium complexity
4. **Then Phase 2.1** (type inference) - Requires more analysis
5. **Finally Phase 3** (auto-borrow) if time permits - High complexity

**Estimated Total Time**: 4-7 hours (depending on scope)

---

**Document Status**: Analysis complete, ready for implementation
**Next Action**: Implement Phase 1 (string slice fix)
