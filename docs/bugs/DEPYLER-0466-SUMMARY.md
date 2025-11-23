# DEPYLER-0466 Session Summary: Failed Auto-Borrowing Attempt

## Session Overview
- **Date**: 2025-11-22
- **Starting Point**: config_manager with 6 compilation errors (after DEPYLER-0465)
- **Goal**: Fix remaining function call type mismatches
- **Result**: Attempted fix failed, learned valuable lessons
- **Ending Point**: config_manager still at 6 errors (baseline maintained)

## Work Performed

### 1. Error Analysis
Analyzed the 6 remaining errors in config_manager:
- Line 120: HashMap inside json!() - unrelated to borrowing
- Line 132: Wrong value type - unrelated to borrowing
- Line 156: `get_nested_value(config, key)` - needs `&config, &key`
- Line 164: Display trait - unrelated to borrowing
- Line 168: `set_nested_value(config, key, value)` - needs `&config, &key, &value`
- Line 169: `save_config(args.config, config)` - needs `args.config, &config`

**Identified Pattern**: 3 of 6 errors related to missing `&` for function arguments

### 2. Implementation Attempt
Added auto-borrowing for `HirExpr::Attribute` (field accesses like `args.key`):

**Code Added** (`expr_gen.rs` lines 2447-2458):
```rust
HirExpr::Attribute { value, .. } => {
    if let HirExpr::Var(_base_var) = &**value {
        self.ctx.argparser_tracker.has_parsers()
    } else {
        false
    }
}
```

**Intention**: Auto-borrow ALL args.* field accesses

### 3. Testing and Discovery
- Rebuilt transpiler successfully
- Re-transpiled config_manager
- **Result**: 6 → 8 errors (+33% WORSE!)

**New Errors**:
- `&args.config` when function expects owned `String`
- Auto-borrowing was too aggressive

### 4. Root Cause Analysis
The fix failed because:
1. **No Function Signature Info**: Can't distinguish between:
   - `load_config(path: String)` - expects owned
   - `get_nested_value(key: &str)` - expects borrowed
2. **Heuristic Too Broad**: Borrowed ALL args.* fields unconditionally
3. **Wrong Assumption**: Not all Clap fields should be borrowed

### 5. Documentation
Created comprehensive failure documentation:
- `docs/bugs/DEPYLER-0466-clap-args-field-borrowing.md` (150 lines)
- Documented why it failed
- Outlined alternative approaches
- Preserved learnings for future work

### 6. Revert
- Removed Attribute auto-borrowing code
- Rebuilt and re-transpiled
- Confirmed back to 6 errors (baseline maintained)

## Key Learnings

### What Failed
❌ **Blanket auto-borrowing of args.* fields**
- Some functions expect owned String (`load_config`)
- Others expect borrowed &str (`get_nested_value`)
- Can't distinguish without function signature information

### What We Learned
1. **Type Information Is Critical**: Auto-borrowing requires knowing callee parameter types
2. **Heuristics Have Limits**: Name-based heuristics fail without type context
3. **Test Impact First**: Always verify a fix reduces errors before committing
4. **Document Failures**: Failed attempts teach us what NOT to do

### Alternative Approaches Identified
1. **Two-Pass Compilation**: Collect signatures first, then codegen
2. **Parameter Type Changes**: Map Python str → Rust &str (always borrowed)
3. **Smarter Heuristics**: Variable name + type inference
4. **Manual Fixes**: Accept some patterns need post-generation work

## Remaining Errors in config_manager (6 total)

After reverting DEPYLER-0466:

### Borrowing-Related (3 errors)
1. **Line 156**: `get_nested_value(config, key)`
   - Expected: `&config, &key`
   - Got: `config, key`
2. **Line 168**: `set_nested_value(config, key, value)`
   - Expected: `&config, &key, &mut value`
   - Got: `config, key, value`
3. **Line 169**: `save_config(args.config, config)`
   - Expected: `args.config, &config`
   - Got: `args.config, config`

**Pattern**: `config` variable (serde_json::Value) needs `&` prefix

### Non-Borrowing Related (3 errors)
4. **Line 120**: HashMap inside json!() macro
   - Root cause: Type mismatch in dict literal generation
5. **Line 132**: Wrong value type for dict insertion
   - Root cause: &mut str vs Value type confusion
6. **Line 164**: Option<Value> doesn't implement Display
   - Root cause: Missing unwrap before println!

## Next Steps Recommended

### High-Priority Fix: DEPYLER-0467 (Suggested)
**Target**: Fix `config` variable auto-borrowing

**Approach**:
Instead of fixing all borrowing at once, focus on ONE specific pattern:
- When a variable has type `Custom("serde_json::Value")`
- AND it's used as an argument to a function call
- AND the variable is NOT a parameter (it's a local variable)
- THEN add `&` prefix

**Expected Impact**: -3 errors (lines 156, 168, 169)

**Implementation**:
Modify `HirExpr::Var` handling to check if:
1. Variable type is `Type::Custom("serde_json::Value")`
2. Variable is declared in current scope (not a parameter)
3. Default to borrowing for Value types

### Medium-Priority: HashMap json!() Issues
**Target**: Fix lines 120, 132

**Root Cause**: Dict literal generation creates HashMap but json!() expects Value

**Approach**: Special case for dict literals inside json!() macro calls

### Low-Priority: Display Trait
**Target**: Fix line 164

**Simple Fix**: Add `.unwrap()` before printing Option<Value>

## Session Metrics

- **Time Spent**: ~2 hours (analysis, implementation, testing, documentation, revert)
- **Code Changes**: +11 lines (reverted), +150 lines docs
- **Build Time**: ~40s per build (3 builds total)
- **Errors Before**: 6
- **Errors After**: 6 (maintained baseline)
- **Errors During**: 8 (worst case, reverted)
- **Lines of Documentation**: 150+

## Conclusion

While DEPYLER-0466 failed to reduce errors, the session was productive:
- ✅ Learned why blanket auto-borrowing doesn't work
- ✅ Identified 3 specific errors that ARE fixable with targeted auto-borrowing
- ✅ Documented failure comprehensively for future reference
- ✅ Maintained baseline (didn't make things worse permanently)
- ✅ Outlined clear path forward for DEPYLER-0467

**Recommendation**: Close DEPYLER-0466 as "Won't Fix", create DEPYLER-0467 for targeted `config` variable borrowing fix.
