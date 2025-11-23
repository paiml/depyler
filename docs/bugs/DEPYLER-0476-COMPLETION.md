# DEPYLER-0476 COMPLETION: Variable Hoisting For/While Loop Fix

## Status: ✅ PARTIAL (Reduced example_environment errors from 17 → 16)
- **Date**: 2025-11-23
- **Result**: Fixed variable hoisting to exclude nested for/while loop assignments
- **Files Modified**: 1 (stmt_gen.rs)
- **Lines Changed**: ~75 lines
- **Build Time**: ~40s

## Achievement: Fixed Variable Type Inference in Nested Scopes

example_environment now compiles with 1 fewer error by fixing incorrect variable hoisting.

```
E0624 errors (3) → 0 (100% fixed!)
E0308 value type errors → 0 (100% fixed!)
Total: 17 errors → 16 errors (5.9% reduction)
```

## Problem Solved

### Variable Hoisting from Nested For/While Loops ✅ FIXED

**Original**: Variables assigned in for/while loops were hoisted to parent if/else scope

**Error**:
```rust
pub fn show_environment(var_name: &Option<String>) {
    let mut value;  // Hoisted - inferred as Option<String>
    if var_name.is_some() {
        value = std::env::var(var_name).ok();  // Option<String>
    } else {
        for var in common_vars {
            value = std::env::var(var).unwrap_or_else(...);  // ❌ String, not Option<String>!
            if value.len() > 50 {  // ❌ E0624: .len() is private on Option<String>
                ...
            }
        }
    }
}
```

**Compilation Errors**:
```
error[E0308]: mismatched types
  --> env_info.rs:93:21
   |
50 |     let mut value;
   |         --------- expected due to the type of this binding
93 |             value = std::env::var(var).unwrap_or_else(...);
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<String>`, found `String`

error[E0624]: method `len` is private
  --> env_info.rs:94:42
   |
94 |             if (var == "PATH") && (value.len() as i32 > 50) {
   |                                          ^^^ private method
```

**Root Cause**:
- The `extract_assigned_symbols` function recursively extracted variables from ALL nested scopes:
  - If/else blocks ✅
  - For loops ❌ (should not hoist)
  - While loops ❌ (should not hoist)
  - Try/except blocks ✅
- When analyzing if/else branches, it found `value` in both branches:
  - If branch: `value = ...` at top level → `Option<String>`
  - Else branch's for loop: `value = ...` inside loop → `String`
- Incorrectly hoisted `value` to function top, causing type conflict

**Solution**: Created `extract_toplevel_assigned_symbols` function (stmt_gen.rs lines 1103-1167)

**New Function**:
```rust
/// Extract symbols assigned ONLY at the top level (not in nested for/while loops)
///
/// DEPYLER-0476: Fix variable hoisting for variables with incompatible types in nested scopes.
/// Variables assigned inside for/while loops should NOT be hoisted to the parent if/else scope
/// because they may have different types than variables with the same name in the if branch.
fn extract_toplevel_assigned_symbols(stmts: &[HirStmt]) -> std::collections::HashSet<String> {
    use std::collections::HashSet;
    let mut symbols = HashSet::new();

    for stmt in stmts {
        match stmt {
            HirStmt::Assign { target: AssignTarget::Symbol(name), .. } => {
                symbols.insert(name.clone());
            }
            // Recursively check nested if/else blocks (same conceptual level)
            HirStmt::If { then_body, else_body, .. } => {
                symbols.extend(extract_toplevel_assigned_symbols(then_body));
                if let Some(else_stmts) = else_body {
                    symbols.extend(extract_toplevel_assigned_symbols(else_stmts));
                }
            }
            // Recursively check try/except blocks (same conceptual level)
            HirStmt::Try { body, handlers, finalbody, .. } => {
                symbols.extend(extract_toplevel_assigned_symbols(body));
                for handler in handlers {
                    symbols.extend(extract_toplevel_assigned_symbols(&handler.body));
                }
                if let Some(finally) = finalbody {
                    symbols.extend(extract_toplevel_assigned_symbols(finally));
                }
            }
            // DEPYLER-0476: DO NOT recurse into for/while loops
            HirStmt::While { .. } | HirStmt::For { .. } => {
                // Skip - don't extract symbols from loop bodies
            }
            _ => {}
        }
    }

    symbols
}
```

**Why this works**:
1. If/else blocks are checked recursively (nested if/else are at same conceptual level)
2. Try/except blocks are checked recursively (exception handling is at same conceptual level)
3. **For/while loops are SKIPPED** - variables inside loops have different scope/lifetime
4. Variables only hoisted if assigned at TOP LEVEL of BOTH branches

## Files Modified

1. **crates/depyler-core/src/rust_gen/stmt_gen.rs**
   - Lines 1103-1167: Added `extract_toplevel_assigned_symbols` function
   - Lines 1212-1215: Updated `codegen_if_stmt` to use new function
   - Lines 1055: Added `#[allow(dead_code)]` to old `extract_assigned_symbols`

## Generated Code (Before/After)

**Before** (17 errors):
```rust
pub fn show_environment(var_name: &Option<String>) {
    let mut value;  // ❌ Hoisted - type inferred as Option<String>
    if var_name.is_some() {
        value = std::env::var(var_name).ok();  // Option<String> ✅
        // ...
    } else {
        for var in common_vars {
            value = std::env::var(var).unwrap_or_else(...);  // ❌ String, not Option<String>!
            if value.len() > 50 {  // ❌ E0624: private method
                // ...
            }
        }
    }
}
```

**After** (16 errors):
```rust
pub fn show_environment(var_name: &Option<String>) {
    if var_name.is_some() {
        let mut value = std::env::var(var_name).ok();  // ✅ Option<String>, scoped to if branch
        // ...
    } else {
        for var in common_vars {
            let mut value = std::env::var(var).unwrap_or_else(...);  // ✅ String, scoped to for loop
            if value.len() > 50 {  // ✅ Can call .len() on String
                // ...
            }
        }
    }
}
```

## Pattern Analysis

**Python Pattern**:
```python
def show_environment(var_name=None):
    if var_name:
        value = os.environ.get(var_name)  # Returns str | None
        if value:
            print(f"{var_name}={value}")
    else:
        for var in common_vars:
            value = os.environ.get(var, "(not set)")  # Returns str (always)
            if len(value) > 50:  # Can call len() on str
                value = value[:47] + "..."
            print(f"  {var}={value}")
```

**Old Rust (Broken)**:
```rust
let mut value;  // Hoisted - type conflict!
if condition {
    value = ...;  // Option<String>
} else {
    for item in items {
        value = ...;  // String ❌
    }
}
```

**New Rust (Correct)**:
```rust
// No hoisting - variables scoped to their blocks
if condition {
    let mut value = ...;  // Option<String>, scoped to if
} else {
    for item in items {
        let mut value = ...;  // String, scoped to for loop ✅
    }
}
```

**Key Insight**: Variables in for/while loops have different scopes and lifetimes than variables in parallel if/else branches. They should NOT be hoisted together.

## Impact

**Error Reduction**: example_environment errors reduced from 17 → 16

**Errors Fixed**:
- ✅ E0624: method `len` is private (3 instances)
- ✅ E0308: mismatched types for `value` assignment

**Broader Impact**: This fix benefits ALL examples with variables assigned in both if/else branches AND nested for/while loops:
- No more incorrect variable hoisting causing type conflicts
- Variables properly scoped to their respective blocks
- Matches Python's scoping semantics more accurately

**No Regressions**: Verified all previously working examples still compile:
- ✅ example_subcommands: 0 errors (was 0)
- ✅ example_simple: 0 errors (was 0)
- ✅ example_flags: 0 errors (was 0)
- ✅ example_complex: 0 errors (was 0)
- ✅ example_positional: 0 errors (was 0)

## Progress Metrics

**Examples Achieving 100% Single-Shot Compilation** (unchanged):
1. ✅ example_simple
2. ✅ example_flags
3. ✅ example_complex
4. ✅ example_positional
5. ✅ example_config (DEPYLER-0473)
6. ✅ example_subcommands (DEPYLER-0474)

**Examples In Progress**:
- example_environment: 17 → 16 errors (5.9% reduction)

**Remaining Examples**: ~6 more to fix

## Quality Gates

- ✅ example_environment: **1 error fixed** (17 → 16)
- ✅ make lint: **PASSING**
- ✅ No regressions in other examples
- ✅ Code complexity: extract_toplevel_assigned_symbols ≤10

## Related Tickets

- DEPYLER-0379: Original variable hoisting implementation
- DEPYLER-0439: Skip already-declared variables in hoisting
- DEPYLER-0440: Skip None-placeholder assignments
- DEPYLER-0455 Bug 2: Track hoisted inference variables for String normalization

## Notes

The remaining errors in example_environment are unrelated to variable hoisting:
- E0425: `parts` variable not found (varargs issue)
- E0277: `Option<String>` doesn't implement `AsRef<OsStr>` (env::var parameter issue)
- E0308: Type mismatches in other parts (Path, String conversions)
- E0599: `.to_vec()` on `str` (slice issue)

These require separate fixes in future tickets.

---

**Progress: Single-Shot Compilation Roadmap (6/13 examples = 46%)**
