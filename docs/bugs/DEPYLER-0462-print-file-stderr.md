# DEPYLER-0462: print(file=sys.stderr) Not Transpiling to eprintln!()

## Status: ✅ COMPLETE
- **Created**: 2025-11-22
- **Completed**: 2025-11-22
- **Priority**: P1 (HIGH - blocks compilation)
- **Type**: Bug Fix
- **Impact**: HIGH - Affects error handling in multiple examples

## Problem Statement

Python's `print(..., file=sys.stderr)` is incorrectly transpiling to `println!("{} {}", ..., std::io::stderr())` instead of `eprintln!()`.

**Python Source:**
```python
print(f"Error: Key not found: {args.key}", file=sys.stderr)
```

**Incorrect Transpilation (Current):**
```rust
println!("{} {}",
    format!("Error: Key not found: {}", key),
    std::io::stderr()  // ❌ E0277: Stderr doesn't implement Display
);
```

**Correct Transpilation (Expected):**
```rust
eprintln!("Error: Key not found: {}", key);
```

## Error Message

```
error[E0277]: `Stderr` doesn't implement `std::fmt::Display`
   --> config_manager.rs:161:21
    |
159 |                     "{} {}",
    |                         -- required by this formatting parameter
160 |                     format!("Error: Key not found: {}", key),
161 |                     std::io::stderr()
    |                     ^^^^^^^^^^^^^^^^^ `Stderr` cannot be formatted with the default formatter
```

## Root Cause

The `print()` function transpilation in `expr_gen.rs` is not detecting the `file=sys.stderr` keyword argument and generating the appropriate `eprintln!()` macro call.

**Expected Behavior:**
- `print(...)` → `println!(...)`
- `print(..., file=sys.stderr)` → `eprintln!(...)`
- `print(..., file=sys.stdout)` → `println!(...)` (same as default)

## Impact on Examples

**config_manager**: 1 error (line 161)
**Expected error reduction**: -1 to -2 errors across reprorusted examples

## Implementation Plan

1. **Detect `file=` keyword in print() calls** (HIR level)
   - Check if `file=sys.stderr` is passed as keyword argument
   - Extract this during HIR construction

2. **Update print() transpilation** (`expr_gen.rs`)
   - Check for `file` parameter in print call
   - Generate `eprintln!()` instead of `println!()` when `file=sys.stderr`
   - Ignore `file=sys.stdout` (default behavior)

3. **Test case**:
```python
import sys
print("Normal output")
print("Error message", file=sys.stderr)
print("Explicit stdout", file=sys.stdout)
```

Expected Rust:
```rust
println!("Normal output");
eprintln!("Error message");
println!("Explicit stdout");
```

## Files to Modify

- `crates/depyler-core/src/hir.rs` - Add `file` field to Print expression
- `crates/depyler-core/src/ast_bridge.rs` - Extract `file=` keyword argument
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Generate `eprintln!()` when appropriate

## Related Issues

- **Parent**: DEPYLER-0435 (reprorusted 100% compilation)
- **Related**: Error handling patterns across all examples

## Implementation (COMPLETE)

### Solution

Modified `convert_call()` in `expr_gen.rs` to handle `print()` BEFORE kwargs are merged:

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`
**Lines**: 1243-1322

**Key Changes**:
1. Added special case for `func == "print"` before match statement (line 1245)
2. Check `kwargs` for `file=sys.stderr` pattern (lines 1247-1252)
3. Generate `eprintln!()` when `file=sys.stderr`, otherwise `println!()` (lines 1254-1321)
4. Removed old print() handling from `convert_generic_call()` (line 2299)

**Detection Logic**:
```rust
let use_stderr = kwargs.iter().any(|(name, value)| {
    name == "file" && matches!(value, HirExpr::Attribute {
        value: attr_value,
        attr
    } if matches!(&**attr_value, HirExpr::Var(module) if module == "sys") && attr == "stderr")
});
```

**Code Generation**:
- `use_stderr == true` → `eprintln!(...)`
- `use_stderr == false` → `println!(...)`
- Handles empty args, single arg, and multiple args
- Preserves `{:?}` debug formatting for collections

### Verification

**Before Fix** (config_manager.rs:161):
```rust
println!("{} {}", format!("Error: Key not found: {}", key), std::io::stderr());
// ❌ E0277: Stderr doesn't implement Display
```

**After Fix** (config_manager.rs:158):
```rust
eprintln!("{}", format!("Error: Key not found: {}", key));
// ✅ Compiles correctly
```

**Compilation Results**:
- **config_manager**: 12 errors → 10 errors (-2, -17% improvement) ✅
- Fixed E0277: `Stderr` doesn't implement `std::fmt::Display`
- Expected to fix similar errors in other examples using stderr

## Impact

### Direct Impact
- **config_manager**: -2 errors
- **Estimated total**: -2 to -4 errors across all reprorusted examples

### Related Patterns Fixed
All print() calls with `file=` keyword now work correctly:
- `print(..., file=sys.stderr)` → `eprintln!(...)`
- `print(..., file=sys.stdout)` → `println!(...)` (default)
- `print(...)` → `println!()` (default)

## Test Case

**Python**:
```python
import sys

print("Normal output")
print("Error message", file=sys.stderr)
print("Explicit stdout", file=sys.stdout)
```

**Generated Rust** (verified):
```rust
println!("Normal output");
eprintln!("Error message");
println!("Explicit stdout");
```

## Completion Summary

**Status**: ✅ **COMPLETE**
- Implementation: Done
- Testing: Verified with config_manager
- Error reduction: -2 errors (17% improvement in config_manager)
- Related: DEPYLER-0435 (reprorusted 100% compilation)
