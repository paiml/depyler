# Transpiler Bug: Dictionary Access Treated as Array Indexing

**Date Discovered**: 2025-10-08
**Severity**: CRITICAL (Compilation Failure)
**Status**: Documented, Not Fixed

## Summary

The transpiler incorrectly treats ALL index operations (`base[index]`) as array/list access requiring `usize` keys. Dictionary/HashMap access with string keys fails because the transpiler casts string keys to `usize`.

## Minimal Reproduction

**Python Input** (`examples/showcase/process_config.py:5-6`):
```python
if "debug" in config:
    return config["debug"]
```

**Expected Rust Output**:
```rust
if config.contains_key("debug") {
    return Ok(config.get("debug").copied())
}
```

**Actual Rust Output** (INCORRECT):
```rust
let _cse_temp_0 = config.contains_key(& "debug");  // ❌ Extra &
if _cse_temp_0 {
    return Ok(config.get("debug" as usize).copied().unwrap_or_default());  // ❌ as usize!
}
```

## Compilation Errors

```
error[E0277]: the trait bound `String: Borrow<&str>` is not satisfied
  --> process_config.rs:20:43
   |
20 |     let _cse_temp_0 = config.contains_key(& "debug");
   |                              ------------ ^^^^^^^^^ the trait `Borrow<&str>` is not implemented

error[E0308]: mismatched types
  --> process_config.rs:22:26
   |
22 |     return Ok(config.get("debug" as usize).copied().unwrap_or_default());
   |                      --- ^^^^^^^^^^^^^^^^ expected `&_`, found `usize`
```

## Problems

1. **Wrong Cast**: Casting string key `"debug"` to `usize` makes no sense
2. **Type Error**: HashMap expects `&str` key, not `usize`
3. **Extra Reference**: `contains_key(& "debug")` creates `& &str` instead of `&str`
4. **Generic Bug**: Affects ALL dictionary operations

## Root Cause

**Location**: `crates/depyler-core/src/rust_gen.rs:1917-1924`

```rust
fn convert_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
    let base_expr = base.to_rust_expr(self.ctx)?;
    let index_expr = index.to_rust_expr(self.ctx)?;
    // V1: Safe indexing with bounds checking
    Ok(parse_quote! {
        #base_expr.get(#index_expr as usize).copied().unwrap_or_default()  // ❌ ALWAYS casts to usize
    })
}
```

The function ALWAYS generates `as usize`, regardless of whether the base is a Vec (needs usize) or a HashMap (needs key type).

## Impact

**Severity**: CRITICAL - Prevents all dictionary operations from compiling
**Examples Affected**:
- `examples/showcase/process_config.py` ❌ (dict access fails)
- Any code using dictionaries/HashMaps
- Any code using string keys

**Patterns Affected**:
- `config["key"]` → BROKEN
- `"key" in dict` → BROKEN
- `dict.get("key")` → BROKEN
- Array access like `arr[0]` → Works (by accident, since it needs usize)

## Fix Required

### Solution: Type-Aware Index Generation

Need to determine if base is a Vec/list or HashMap/dict:

```rust
fn convert_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
    let base_expr = base.to_rust_expr(self.ctx)?;
    let index_expr = index.to_rust_expr(self.ctx)?;

    // NEW: Infer type of base expression
    let base_type = self.infer_expr_type(base)?;

    match base_type {
        Type::Vec(_) | Type::List(_) => {
            // Array/list indexing: needs usize
            Ok(parse_quote! {
                #base_expr.get(#index_expr as usize).copied().unwrap_or_default()
            })
        }
        Type::HashMap(_, _) | Type::Dict(_, _) => {
            // Dictionary access: use key type directly
            Ok(parse_quote! {
                #base_expr.get(&#index_expr).copied()
            })
        }
        _ => {
            // Fallback: try usize (for backward compatibility)
            Ok(parse_quote! {
                #base_expr.get(#index_expr as usize).copied().unwrap_or_default()
            })
        }
    }
}
```

### Additional Fixes Needed

1. **contains_key Fix**: Remove extra `&` in `contains_key(& "debug")`
   - Location: Likely in method call conversion
   - Fix: `contains_key("key")` not `contains_key(& "key")`

2. **Return Type Fix**: Line 25 returns `Ok(())` instead of `Ok(None)`
   - This is a separate bug in None/unit conversion

## Estimated Effort

- **Complexity**: HIGH (requires type inference)
- **Files to Modify**: 2-3 files
- **Lines Changed**: ~50-100 lines
- **Test Cases**: ~15 new tests
- **Time Estimate**: 4-6 hours
- **Dependencies**: Need robust type inference for expressions

## Workaround

None available - requires transpiler fix.

## Next Steps

1. Create ticket: DEPYLER-XXXX for dict access type discrimination
2. Implement expression type inference (if not already available)
3. Update `convert_index` to use type information
4. Write test cases for Vec vs HashMap access
5. Fix contains_key extra reference bug
6. Fix None vs () return type bug
7. Re-transpile and validate all examples

## Related Files

- `crates/depyler-core/src/rust_gen.rs:1917` (convert_index - ROOT CAUSE)
- `crates/depyler-core/src/hir.rs:337` (HirExpr::Index definition)
- `examples/showcase/process_config.py` (affected example)

## Related Bugs

- TRANSPILER_BUG_type_annotations.md - Type annotation support
- Both bugs require type inference improvements
