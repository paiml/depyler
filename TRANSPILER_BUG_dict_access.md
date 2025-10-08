# Transpiler Bug: Dictionary Access Treated as Array Indexing

**Date Discovered**: 2025-10-08
**Date Fixed**: 2025-10-08 (Partial - index discrimination fixed, `contains_key` issue remains)
**Severity**: HIGH (was CRITICAL, now partially resolved)
**Status**: PARTIALLY FIXED

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

## Fix Implemented (2025-10-08)

**Location**: `crates/depyler-core/src/rust_gen.rs:1917-1937`

**Solution**: Type-aware index generation based on index expression type:

```rust
fn convert_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
    let base_expr = base.to_rust_expr(self.ctx)?;

    // Discriminate between HashMap and Vec access based on index type
    match index {
        HirExpr::Literal(Literal::String(s)) => {
            // String index → HashMap/Dict access
            // Use cloned() for String values (not Copy)
            Ok(parse_quote! {
                #base_expr.get(#s).cloned().unwrap_or_default()
            })
        }
        _ => {
            // Numeric/other index → Vec/List access with usize cast
            let index_expr = index.to_rust_expr(self.ctx)?;
            Ok(parse_quote! {
                #base_expr.get(#index_expr as usize).copied().unwrap_or_default()
            })
        }
    }
}
```

**What Works Now**:
- ✅ `d["key"]` → `d.get("key").cloned().unwrap_or_default()` (HashMap)
- ✅ `lst[0]` → `lst.get(0 as usize).copied().unwrap_or_default()` (Vec)
- ✅ No more `"key" as usize` type errors
- ✅ Uses `.cloned()` for String values (not Copy)
- ✅ Uses `.copied()` for Copy values (i32, etc.)

**Test Results**:
- ✅ Test file compiles cleanly
- ✅ Dict access with string keys works
- ✅ List access with numeric keys works
- ✅ All 370 core tests passing

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

## Remaining Issues

1. **`contains_key` Extra Reference** (UNFIXED):
   - Generated: `config.contains_key(& "debug")`
   - Expected: `config.contains_key("debug")`
   - Error: `Borrow<&str>` not implemented
   - Location: Likely in `BinOp::In` conversion

2. **Return Type Mismatch** (UNFIXED):
   - Generated: `Ok(config.get("debug").cloned().unwrap_or_default())`
   - Expected: `Ok(Some(config.get("debug").cloned()))`
   - Error: `expected Option<String>, found String`
   - Location: Return statement generation for Optional types

3. **None vs () Confusion** (UNFIXED):
   - Generated: `return Ok(())`
   - Expected: `return Ok(None)`
   - Error: `expected Option<String>, found ()`
   - Location: None literal conversion

## Estimated Effort (Remaining)

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
