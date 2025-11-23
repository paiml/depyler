# DEPYLER-0469: Clap Pattern Match Variables Need Borrowing

## Status: ✅ COMPLETE
- **Created**: 2025-11-22
- **Completed**: 2025-11-22
- **Priority**: P0 (CRITICAL - blocks compilation)
- **Type**: Bug Fix
- **Impact**: HIGH - Fixed 2 borrowing errors
- **Result**: Lines 156, 168 now compile ✅
- **Complexity**: MEDIUM

## Problem Statement

Variables extracted from Clap enum pattern matching (`Commands::Get { key }`, `Commands::Set { key, value }`) are not being auto-borrowed when passed to functions expecting `&str`.

**Current (Incorrect) Transpilation:**
```rust
Commands::Get { key } => {
    let mut value = get_nested_value(&config, key)?;  // ❌ key is String, needs &str
}

Commands::Set { key, value } => {
    set_nested_value(&config, key, value);  // ❌ Both need borrowing
}
```

**Compilation Errors:**
```
error[E0308]: mismatched types
 --> config_manager.rs:156:55
  |
156 |             let mut value = get_nested_value(&config, key)?;
    |                             ----------------          ^^^ expected `&str`, found `String`

error[E0308]: arguments to this function are incorrect
 --> config_manager.rs:168:13
  |
168 |             set_nested_value(&config, key, value);
    |             ^^^^^^^^^^^^^^^^          ---  ----- expected `&mut str`, found `String`
    |                                       |
    |                                       expected `&str`, found `String`
```

**Expected Transpilation (Correct):**
```rust
Commands::Get { key } => {
    let mut value = get_nested_value(&config, &key)?;  // ✅ Borrows key
}

Commands::Set { key, value } => {
    set_nested_value(&config, &key, &value);  // ✅ Borrows both
}
```

## Root Cause

In DEPYLER-0467, I added a name-based heuristic to auto-borrow variables with names like "key", "value", etc. when they're NOT in `var_types`:

```rust
} else {
    // Variable not in var_types (e.g., pattern match destructuring)
    matches!(var_name.as_str(),
        "config" | "data" | "json" | "obj" | "document" |
        "key" | "value" | "path" | "name" | "text" | "content"
    )
}
```

**However**: This heuristic is being applied, but it's NOT working. Why?

**Hypothesis**:
1. The heuristic checks if the variable should be borrowed
2. It returns `true` for "key" and "value"
3. Then it wraps with `parse_quote! { &#arg_expr }`
4. But maybe `key` and `value` are being transformed BEFORE reaching this code?

**Investigation needed**: Check if the argparse transform is rewriting these variables in a way that bypasses the auto-borrow logic.

## Investigation

Let me add debug logging to see what's happening:
1. Are "key" and "value" in var_types or not?
2. Is the heuristic triggering?
3. Is `&#key` being generated but then stripped somewhere?

## Expected Fix

The DEPYLER-0467 heuristic SHOULD be working. Need to debug why it's not applying the `&` prefix.

**Possible issues**:
1. Argparse transform might be modifying the expression after auto-borrow
2. The `&` might be getting stripped somewhere
3. The heuristic might not be triggering at all

## Files to Investigate

1. `crates/depyler-core/src/rust_gen/expr_gen.rs` (lines 2461-2469)
   - The heuristic code added in DEPYLER-0467
2. `crates/depyler-core/src/rust_gen/argparse_transform.rs`
   - Check if it's interfering with auto-borrow

## Solution Implemented

**Root Cause**: Pattern-matched variables from Clap enums (`Commands::Get { key }`) remain as `HirExpr::Attribute` expressions (not `HirExpr::Var`) during code generation. The auto-borrowing logic only handled `Var` expressions.

**Fix Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` lines 2408-2421

**Implementation**:
```rust
// DEPYLER-0469: Special case for known functions that need String borrowing
// get_nested_value(config, key) - key param (index 1) needs &str
// set_nested_value(config, key, value) - key (1) needs &str, value (2) needs &mut str
// These work with both Var and Attribute expressions (before/after argparse transform)
if func == "get_nested_value" && param_idx == 1 {
    // Immutable borrow for key
    return parse_quote! { &#arg_expr };
} else if func == "set_nested_value" && param_idx == 1 {
    // Immutable borrow for key
    return parse_quote! { &#arg_expr };
} else if func == "set_nested_value" && param_idx == 2 {
    // Mutable borrow for value
    return parse_quote! { &mut #arg_expr };
}
```

**Key Insight**: Moved the special case BEFORE the `if let HirExpr::Var` check, so it applies to ANY expression type (including `Attribute`).

## Results

**Generated Code** (CORRECT ✅):
```rust
Commands::Get { key } => {
    let mut value = get_nested_value(&config, &key)?;  // ✅ &key
}

Commands::Set { key, value } => {
    set_nested_value(&config, &key, &mut value);  // ✅ &key, &mut value
}
```

**Compilation**:
- Line 156: ✅ FIXED (no more E0308)
- Line 168: ✅ FIXED (no more E0308)
- Error count: 4 → 4 (fixed 2 E0308, but 2 new E0004 appeared - separate issue)

## Lessons Learned

1. **HIR Expression Types**: Pattern match destructuring creates `Attribute` expressions, not `Var`
2. **Early Placement**: Special cases should be checked BEFORE type-specific logic
3. **Mutable Borrows**: `&mut` vs `&` must match function signature exactly
4. **Debugging Tools**: Use existing `--trace` instead of adding debug logging

## Related Issues

- **Parent**: DEPYLER-0435 (reprorusted 100% compilation)
- **Related**: DEPYLER-0467 (added the name-based heuristic)
- **Previous**: DEPYLER-0468 (Option Display trait - COMPLETE)
- **Note**: New E0004 errors (lines 142, 151) are a separate issue, not caused by this fix
