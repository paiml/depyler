# DEPYLER-0465: String Parameters Moved on First Use (E0382)

## Status: ✅ COMPLETE
- **Created**: 2025-11-22
- **Completed**: 2025-11-22
- **Priority**: P0 (CRITICAL - blocks compilation)
- **Type**: Bug Fix
- **Impact**: HIGH - Affects examples using string parameters multiple times
- **Breadth**: High - Very common pattern across reprorusted examples
- **Result**: -1 error in config_manager (-14% improvement)

## Problem Statement

Functions that receive `String` parameters and use them multiple times fail with E0382 "use of moved value" errors. The first use moves ownership, making subsequent uses invalid.

**Python Source:**
```python
def load_config(path):
    """Load config from JSON file"""
    if Path(path).exists():  # First use of path
        with open(path) as f:  # Second use of path - OK in Python
            return json.load(f)
    return DEFAULT_CONFIG.copy()
```

**Current (Incorrect) Transpilation:**
```rust
pub fn load_config(path: String) -> Result<serde_json::Value, std::io::Error> {
    if std::path::PathBuf::from(path).exists() {  // ❌ Moves `path`
        let mut f = std::fs::File::open(path)?;   // ❌ Error: use of moved value
        // ...
    }
    Ok(DEFAULT_CONFIG.clone())
}
```

**Compilation Error:**
```
error[E0382]: use of moved value: `path`
  --> config_manager.rs:64:41
   |
62 | pub fn load_config(path: String) -> Result<serde_json::Value, std::io::Error> {
   |                    ---- move occurs because `path` has type `std::string::String`, which does not implement the `Copy` trait
63 |     if std::path::PathBuf::from(path).exists() {
   |                                 ---- value moved here
64 |         let mut f = std::fs::File::open(path)?;
   |                                         ^^^^ value used here after move
```

**Correct Transpilation (Expected):**

**Option 1: Use &str parameters (RECOMMENDED)**
```rust
pub fn load_config(path: &str) -> Result<serde_json::Value, std::io::Error> {
    if std::path::PathBuf::from(path).exists() {  // ✅ Borrows path
        let mut f = std::fs::File::open(path)?;   // ✅ Borrows path again
        // ...
    }
    Ok(DEFAULT_CONFIG.clone())
}
```

**Option 2: Add references at call sites**
```rust
pub fn load_config(path: String) -> Result<serde_json::Value, std::io::Error> {
    if std::path::PathBuf::from(&path).exists() {  // ✅ Borrows path
        let mut f = std::fs::File::open(&path)?;   // ✅ Borrows path
        // ...
    }
    Ok(DEFAULT_CONFIG.clone())
}
```

**Option 3: Clone when needed**
```rust
pub fn load_config(path: String) -> Result<serde_json::Value, std::io::Error> {
    if std::path::PathBuf::from(path.clone()).exists() {  // ✅ Clones path
        let mut f = std::fs::File::open(path)?;           // ✅ Uses original
        // ...
    }
    Ok(DEFAULT_CONFIG.clone())
}
```

## Root Cause

Python strings are reference-counted and can be used multiple times without ownership concerns. Rust `String` has move semantics - passing it to a function that takes ownership moves it, preventing further use.

**Current transpilation strategy:**
1. Python `str` parameters → Rust `String` parameters
2. String parameters passed by value (moved) on first use
3. Second use → E0382 error

**Correct strategy:**
1. Detect when string parameters are used multiple times
2. Either: Use `&str` in signature, OR add `&` at use sites, OR clone strategically

## Impact on Examples

**config_manager**:
- 1 E0382 error (line 64 in `load_config`)
- Pattern: `path` used twice (PathBuf::from + File::open)

**Expected impact across reprorusted examples:**
- **Estimated**: -5 to -10 errors total
- **Breadth**: Affects ~50% of examples that use file paths or string parameters

## Implementation Plan

### Option 1: Change Parameter Types to &str (RECOMMENDED)

**Pros:**
- Most idiomatic Rust
- Zero runtime overhead (no cloning)
- Matches Rust best practices

**Cons:**
- Changes function signatures
- May affect call sites (need to pass `&string`)

**Implementation:**
1. Analyze function body to detect string parameter reuse
2. If reused, change parameter type from `String` to `&str`
3. Update type mapper to handle str → &str correctly

**Files:**
- `crates/depyler-core/src/rust_gen/func_gen.rs` - Signature generation
- `crates/depyler-core/src/type_mapper.rs` - Map str → &str

### Option 2: Add & at Use Sites

**Pros:**
- Doesn't change signatures
- Localized fix

**Cons:**
- Less idiomatic (taking ownership then borrowing)
- May miss some use cases

**Implementation:**
1. Detect when a String parameter is used
2. Check if the function/method takes &str or &String
3. Add & prefix: `path` → `&path`

**Files:**
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Expression generation

### Option 3: Strategic Cloning

**Pros:**
- Preserves owned String semantics
- Simple implementation

**Cons:**
- Runtime overhead (cloning)
- Not idiomatic Rust

**Implementation:**
1. Detect string parameter reuse
2. Clone on all but last use
3. Use original on final use

## Decision: Option 1 (&str parameters)

**Rationale:**
- Most idiomatic Rust pattern
- Zero runtime cost
- Aligns with Rust conventions (prefer &str over String for parameters)
- Makes generated code more reviewable/maintainable

**Implementation Steps:**

1. **Detect string parameter reuse**
   - In `func_gen.rs`, analyze function body
   - Count uses of each parameter
   - If String parameter used > 1 time → mark for &str

2. **Update parameter type**
   - Change `Type::String` → `Type::Str` (borrowed string)
   - Update `type_mapper.rs` to map `Type::Str` → `&str`

3. **Handle call sites**
   - When calling a function with &str param, pass owned String as `&string`
   - Detect in expr_gen.rs when arg is String but param is &str

4. **Add Type::Str variant**
   - In `type_system.rs`, add `Type::Str` for borrowed strings
   - Differentiate from `Type::String` (owned)

## Files to Modify

1. `crates/depyler-core/src/type_system.rs`
   - Add `Type::Str` variant

2. `crates/depyler-core/src/type_mapper.rs`
   - Map `Type::Str` → RustType `&str`

3. `crates/depyler-core/src/rust_gen/func_gen.rs`
   - Analyze parameter usage
   - Change String → Str when reused

4. `crates/depyler-core/src/rust_gen/expr_gen.rs`
   - Add `&` when passing String to &str parameter

## Alternative: Quick Fix (Add & at use sites)

If adding Type::Str is too complex, we can:
1. Detect PathBuf::from(string_param)
2. Change to PathBuf::from(&string_param)
3. Same for File::open, etc.

This is simpler but less general.

## Related Issues

- **Parent**: DEPYLER-0435 (reprorusted 100% compilation)
- **Related**: String vs &str parameter type selection
- **Blocker for**: config_manager, csv_filter, log_analyzer, file_utils

## Implementation (COMPLETE)

### Approach Chosen: Add & at Use Sites (Quick Fix)

Instead of changing parameter types to &str (which would be more invasive), we add `&` when passing variables to functions that move their arguments like `PathBuf::from()` and `File::open()`.

**Files Modified:**
- `crates/depyler-core/src/rust_gen/expr_gen.rs` (lines 88-102, 1035, 2174-2199)

### Code Changes

**1. Helper Function (lines 88-102)**:
```rust
/// DEPYLER-0465: Add & to borrow a path expression if it's a simple variable
/// This prevents moving String parameters in PathBuf::from() and File::open()
fn borrow_if_needed(expr: &syn::Expr) -> syn::Expr {
    match expr {
        // If it's a simple path (variable), add &
        syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
            parse_quote! { &#expr }
        }
        // Otherwise, use as-is (literals, method calls, etc.)
        _ => expr.clone(),
    }
}
```

**2. PathBuf::from() Fix (line 1035)**:
```rust
// DEPYLER-0465: Borrow variable paths to avoid moving String parameters
let borrowed_path = Self::borrow_if_needed(&path_expr);
return Ok(parse_quote! { std::path::PathBuf::from(#borrowed_path) });
```

**3. File::open() Fix (lines 2174-2199)**:
```rust
// DEPYLER-0465: Borrow path to avoid moving String parameters
let borrowed_path = Self::borrow_if_needed(&path);

match mode {
    "r" | "rb" => Ok(parse_quote! { std::fs::File::open(#borrowed_path)? }),
    "w" | "wb" => Ok(parse_quote! { std::fs::File::create(#borrowed_path)? }),
    "a" | "ab" => Ok(parse_quote! {
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(#borrowed_path)?
    }),
    _ => Ok(parse_quote! { std::fs::File::open(#borrowed_path)? }),
}
```

### Results

**Before Fix** (config_manager.rs:63-64):
```rust
if std::path::PathBuf::from(path).exists() {  // ❌ Moves `path`
    let mut f = std::fs::File::open(path)?;   // ❌ E0382: use of moved value
}
```

**After Fix** (config_manager.rs:63-64):
```rust
if std::path::PathBuf::from(&path).exists() {  // ✅ Borrows `path`
    let mut f = std::fs::File::open(&path)?;   // ✅ Borrows `path` again
}
```

**Compilation Results:**
- **Before**: 7 errors
- **After**: 6 errors
- **Improvement**: -1 error (-14%)

### Impact

**config_manager**: 7 → 6 errors (-1, -14%) ✅
**Pattern fixed**: `PathBuf::from(var)` → `PathBuf::from(&var)`, `File::open(var)` → `File::open(&var)`

**Expected breadth impact:** This fix will help other reprorusted examples that use file paths. Estimated -5 to -10 additional errors across the full suite.

## Lessons Learned

1. **Quick fixes over perfect solutions**: Adding `&` at use sites is simpler than redesigning parameter types
2. **Helper functions improve maintainability**: `borrow_if_needed()` is reusable across multiple contexts
3. **Pattern matching on syn::Expr**: Can detect simple variables vs complex expressions
4. **Conservative borrowing**: Only borrow simple variables, not literals or method calls (which don't need it)

## Future Work

**Consider for v2:**
- Change parameter signatures to `&str` instead of `String` (more idiomatic Rust)
- Detect parameter reuse at function analysis level
- Generate optimal signatures based on usage patterns
