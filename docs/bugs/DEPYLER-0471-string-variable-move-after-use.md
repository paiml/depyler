# DEPYLER-0471: String Variable Moved and Used After Move

## Status: 🚧 IN PROGRESS
- **Created**: 2025-11-22
- **Priority**: P0 (CRITICAL - blocks compilation)
- **Type**: Bug Fix
- **Impact**: MEDIUM - Fixes 1 of 3 remaining errors
- **Expected**: -1 error (-33% of remaining!)
- **Complexity**: EASY

## Problem Statement

A String variable (`args.config`) is moved in one statement and then used in a subsequent statement, causing a "use after move" error.

**Current (Incorrect) Transpilation:**
```rust
Commands::Init {} => {
    save_config(args.config, &DEFAULT_CONFIG);  // Moves args.config
    println!("{}", format!("Initialized config at {}", args.config));  // ❌ Use after move
    return Ok(());
}
```

**Compilation Error:**
```
error[E0382]: borrow of moved value: `args.config`
   --> config_manager.rs:145:64
    |
144 |             save_config(args.config, &DEFAULT_CONFIG);
    |                         ----------- value moved here
145 |             println!("{}", format!("Initialized config at {}", args.config));
    |                                                                ^^^^^^^^^^^ value borrowed here after move
```

**Expected Transpilation (Correct):**
```rust
Commands::Init {} => {
    save_config(args.config.clone(), &DEFAULT_CONFIG);  // ✅ Clone before move
    println!("{}", format!("Initialized config at {}", args.config));
    return Ok(());
}
```

OR (better long-term):
```rust
// Change function signature
pub fn save_config(path: &str, config: &serde_json::Value) -> Result<(), std::io::Error> {
    // ...
}

Commands::Init {} => {
    save_config(&args.config, &DEFAULT_CONFIG);  // ✅ Borrow instead of move
    println!("{}", format!("Initialized config at {}", args.config));
}
```

## Root Cause

The `save_config` function signature is:
```rust
pub fn save_config(path: String, config: &serde_json::Value) -> Result<(), std::io::Error>
```

It takes `String` (owned), not `&str`, so passing `args.config` moves the value. But `args.config` is used again in the next line.

**Python Source:**
```python
if args.action == "init":
    save_config(args.config, DEFAULT_CONFIG)
    print(f"Initialized config at {args.config}")
    return
```

In Python, strings are immutable and passed by reference, so there's no move. The transpiler needs to recognize this pattern and either:
1. Clone before the move
2. Change function signatures to accept `&str`

## Solutions

### Solution 1: Clone Before Move (QUICK FIX)
Detect when a variable is used multiple times and clone before moves.

**Implementation**: Expression generation
```rust
// When generating function call arguments
if variable_used_again_after_this_call {
    parse_quote! { #var.clone() }
} else {
    parse_quote! { #var }
}
```

**Pros**: Simple, works immediately
**Cons**: Runtime overhead (unnecessary clones)

### Solution 2: Change Function Signatures (BETTER)
Generate string parameters as `&str` instead of `String` when they're only read.

**Implementation**: Function generation
```rust
// In func_gen.rs, when parameter type is String and only used for reading
Type::String => {
    if param_only_read_not_mutated {
        parse_quote! { &str }
    } else {
        parse_quote! { String }
    }
}
```

**Pros**: Idiomatic Rust, no runtime overhead
**Cons**: Requires analysis of function body

### Solution 3: Auto-Borrow at Call Site (HYBRID)
Keep function signature as `String`, but pass `&args.config` and let Rust's `.to_string()` conversion handle it.

Wait, this won't work because `String` parameter requires owned value, not reference.

## Decision: Solution 1 (Clone Before Move)

**Rationale**:
- Quickest fix to unblock compilation
- Can be improved later with Solution 2
- Safe and correct
- Minimal code changes

## Implementation

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`

**Approach**:
1. Track variable usage across statements
2. When generating argument expressions, check if variable is used later
3. If yes, wrap with `.clone()`

**Alternative (simpler)**: For Clap pattern match variables, always clone when passing to functions that take owned values.

## Expected Result

**Before**:
```rust
save_config(args.config, &DEFAULT_CONFIG);  // ❌ E0382
```

**After**:
```rust
save_config(args.config.clone(), &DEFAULT_CONFIG);  // ✅ Compiles
```

**Error Reduction**: 3 → 2 errors (-1, -33%)

## Files to Modify

1. `crates/depyler-core/src/rust_gen/expr_gen.rs`
   - Add `.clone()` when passing String variables that are used later

OR

2. `crates/depyler-core/src/rust_gen/argparse_transform.rs`
   - For pattern match variables, track usage and clone when needed

## Related Issues

- **Parent**: DEPYLER-0435 (reprorusted 100% compilation)
- **Previous**: DEPYLER-0470 (match exhaustiveness - COMPLETE)
- **Related**: DEPYLER-0469 (auto-borrowing patterns)

## Success Criteria

- ✅ Line 144-145 compile (no E0382)
- ✅ 3 → 2 errors (-33%)
- ✅ No regressions in other examples
- ✅ Build time < 45s
- ✅ Code complexity ≤10
