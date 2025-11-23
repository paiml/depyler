# DEPYLER-0466: Clap Args Field Borrowing in Function Calls

## Status: ❌ FAILED (Approach Too Aggressive)
- **Created**: 2025-11-22
- **Attempted**: 2025-11-22
- **Result**: Made situation worse (6 → 8 errors)
- **Priority**: P0 (CRITICAL - blocks compilation)
- **Type**: Bug Fix Attempt
- **Impact**: HIGH - Affects function calls with String/Value arguments
- **Parent**: DEPYLER-0465 (String parameter borrowing - COMPLETE)

## Problem Statement

After fixing DEPYLER-0465, config_manager still had 6 compilation errors. Analysis showed that function calls weren't properly borrowing arguments:

**Remaining Errors (6 total)**:
1. Line 120: HashMap inside json!() macro (E0308)
2. Line 132: Wrong value type for dict insertion (E0308)
3. Line 156: `get_nested_value(config, key)` expects `(&Value, &str)`, got `(Value, String)` (E0308)
4. Line 164: Option<Value> doesn't implement Display (E0277)
5. Line 168: `set_nested_value(config, key, value)` expects `(&Value, &str, &mut str)` (E0308)
6. Line 169: `save_config(args.config, config)` expects `(String, &Value)`, got `(String, Value)` (E0308)

**Errors 3, 5, 6 Related to Borrowing**:
- `config` variable (serde_json::Value) should be borrowed → `&config`
- `args.key` and `args.value` (String from Clap) should be borrowed → `&key`, `&value`
- But `args.config` should NOT be borrowed (function expects owned String)

## Approach Attempted

Extended auto-borrowing logic in `expr_gen.rs` to handle `HirExpr::Attribute` (field accesses like `args.key`):

**Files Modified**:
- `crates/depyler-core/src/rust_gen/expr_gen.rs` (lines 2447-2458)

**Code Added**:
```rust
// DEPYLER-0466: Auto-borrow field accesses (args.key, args.value, etc.)
// Clap struct fields are typically String types that should be borrowed
HirExpr::Attribute { value, .. } => {
    // Check if this is accessing a Clap args struct (args.field)
    if let HirExpr::Var(_base_var) = &**value {
        // If we have argparse tracked, borrow field accesses
        // (Clap generates String fields that should be borrowed)
        self.ctx.argparser_tracker.has_parsers()
    } else {
        false
    }
}
```

**Logic**: If expression is a field access from the args struct, auto-borrow it.

## Result: FAILED (Situation Worse)

**Before Fix**: 6 errors
**After Fix**: 8 errors (+2 errors, +33%)

**New Errors Introduced**:
```
error[E0308]: mismatched types
 --> config_manager.rs:144:25
  |
144 |             save_config(&args.config, &DEFAULT_CONFIG);
    |             ----------- ^^^^^^^^^^^^ expected `String`, found `&String`

error[E0308]: mismatched types
 --> config_manager.rs:149:30
  |
149 |     let config = load_config(&args.config)?;
    |                  ----------- ^^^^^^^^^^^^ expected `String`, found `&String`
```

**Generated Code (Incorrect)**:
```rust
// Line 144
save_config(&args.config, &DEFAULT_CONFIG);  // ❌ Should be args.config (owned)

// Line 149
let config = load_config(&args.config)?;  // ❌ Should be args.config (owned)

// Line 156
let mut value = get_nested_value(config, &key)?;  // ❌ Should be &config (borrowed)

// Line 168
set_nested_value(config, &key, &value);  // ❌ Should be &config (borrowed)

// Line 169
save_config(&args.config, config);  // ❌ args.config is correct, but config should be &config
```

## Root Cause Analysis

**Why the fix failed**:

1. **Too Broad**: Auto-borrows ALL args.* fields unconditionally
2. **No Function Signature Awareness**: Can't distinguish between:
   - `load_config(path: String)` - expects owned String
   - `get_nested_value(config: &Value, key: &str)` - expects borrowed &str
3. **Function Signatures Unavailable**: `function_param_borrows` only tracks current function parameters, not callees

**The Core Problem**:
To correctly auto-borrow, we need to know the callee's parameter types:
- If param is `&str` and arg is `String` → borrow with `&`
- If param is `String` and arg is `String` → pass as-is
- If param is `&Value` and arg is `Value` → borrow with `&`

But we don't have this information at code generation time because:
- Functions are generated one at a time
- Cross-function type information isn't tracked
- `function_param_borrows` only has current function's info

## What We Learned

**Heuristics That Failed**:
1. ❌ "Borrow all args.* fields" - too broad, breaks owned String params
2. ❌ "Check argparser_tracker.has_parsers()" - doesn't distinguish between field types

**What Would Work (But Not Implemented)**:
1. ✅ **Function Signature Database**: Track all function signatures in a first pass, then use during codegen
2. ✅ **Type-Aware Auto-Borrowing**: Check actual parameter types of callee functions
3. ✅ **Explicit Borrow Annotations**: Add metadata to HIR indicating when to borrow

**Simpler Alternative**:
- Change parameter types: `get_nested_value(&config: &Value)` → `config: Value` (take ownership)
- But this changes Rust idioms (Value should be borrowed)

## Next Steps

**Option 1**: Implement two-pass compilation:
1. **Pass 1**: Collect all function signatures → populate signature database
2. **Pass 2**: Code generation with signature lookup for auto-borrowing

**Option 2**: Change Python-to-Rust parameter type mapping:
- Python `str` param → Rust `&str` (always borrowed)
- Python dict param → Rust `&Value` (always borrowed)
- Adjust call sites accordingly

**Option 3**: Manual fixes in generated code:
- Accept that some patterns require post-generation fixes
- Focus on reducing errors, not eliminating them

**Option 4**: Smarter heuristics:
- Detect variable names (`config`, `key`, `value`) and infer borrowing needs
- Risky but might work for common patterns

## Decision: Revert and Revisit

**Immediate Action**: Revert the Attribute auto-borrowing change
**Reason**: It makes the situation worse, not better
**Future Work**: Consider Option 1 (two-pass compilation) or Option 2 (parameter type changes)

## Files Modified (To Be Reverted)

1. `crates/depyler-core/src/rust_gen/expr_gen.rs`
   - Lines 2447-2458: Remove Attribute handling for auto-borrowing
   - Line 2499-2500: Keep type annotation fix (unrelated)

## Lessons Learned

1. **Measure Impact**: Always test that a fix actually reduces errors, not increases them
2. **Heuristics Have Limits**: Without type information, auto-borrowing is guesswork
3. **Two-Pass May Be Necessary**: Complex type-aware transformations need full context
4. **Document Failures**: Failed approaches are as valuable as successes
5. **Incremental Progress**: A failed attempt that teaches us something is still progress

## Related Issues

- **Parent**: DEPYLER-0435 (reprorusted 100% compilation)
- **Previous**: DEPYLER-0465 (String parameter borrowing - COMPLETE)
- **Blocker for**: config_manager (still 6 errors after revert)

## Metrics

- **Errors Before**: 6
- **Errors After**: 8 (+2, +33%)
- **Fix Complexity**: Low (10 lines of code)
- **Build Time**: 41s
- **Test Result**: ❌ FAILED
