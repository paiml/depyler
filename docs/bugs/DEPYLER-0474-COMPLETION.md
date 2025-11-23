# DEPYLER-0474 COMPLETION: Subcommand Partial Move Fix

## Status: ✅ COMPLETE (100% - All errors fixed!)
- **Date**: 2025-11-23
- **Result**: 3 errors → 0 errors (100% reduction) 🎉
- **Files Modified**: 1 (stmt_gen.rs)
- **Lines Changed**: ~15
- **Build Time**: ~40s

## Achievement: 100% Single-Shot Compilation

example_subcommands (git_clone) now compiles **on first transpilation attempt** with **ZERO errors**!

```
3 initial E0382 errors → 0 errors (100% success)
```

## Problem Solved

### Subcommand Pattern Matching Partial Move ✅ FIXED

**Original**: Pattern matching extracted fields from enum, causing partial move

**Error**:
```rust
match args.command {
    Commands::Clone { url } => {  // Moves url from args.command
        handle_clone(&args);  // ❌ Borrow of partially moved value: `args`
    }
    ...
}
```

**Compilation Errors**:
```
error[E0382]: borrow of partially moved value: `args`
  --> git_clone.rs:40:26
   |
39 |         Commands::Clone { url } => {
   |                           --- value partially moved here
40 |             handle_clone(&args);
   |                          ^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because value has type `std::string::String`,
           which does not implement the `Copy` trait
```

**Root Cause**:
- Match pattern `Commands::Clone { url }` extracted `url` field by value
- This partially moved `args.command`
- Then tried to borrow `&args`, but `args` was partially moved
- Rust ownership rules prevent borrowing after partial move

**Solution**: Match by reference and ignore field bindings (stmt_gen.rs lines 3509, 3487-3491)

**Fix 1 - Match by reference** (line 3509):
```rust
// DEPYLER-0474: Match by reference to avoid partial move errors
match &args.command {  // ✅ Borrow, don't move
    Commands::Clone { .. } => {
        handle_clone(&args);  // ✅ Can borrow args since we didn't move it
    }
    ...
}
```

**Fix 2 - Ignore field bindings** (line 3487-3491):
```rust
// DEPYLER-0474: Don't extract field bindings, use `..` to ignore them
// Handler functions re-extract fields from &args, so bindings here are unused
quote! {
    Commands::#variant_name { .. } => {  // ✅ Ignore all fields
        #(#body_stmts)*
    }
}
```

**Why this works**:
1. Matching `&args.command` borrows the enum instead of moving it
2. Pattern `{ .. }` ignores all fields, so nothing is extracted
3. Handler functions like `handle_clone(&args)` can borrow args
4. Inside handlers, fields are re-extracted using `if let Commands::Clone { url } = &args.command { ... }`

## Files Modified

1. **crates/depyler-core/src/rust_gen/stmt_gen.rs**
   - Lines 3509: Changed `match args.command {` to `match &args.command {`
   - Lines 3487-3491: Changed pattern from `{ #(#field_bindings),* }` to `{ .. }`
   - Lines 3457-3458: Removed unused field_bindings variable extraction
   - Lines 3506-3507: Added documentation comments

## Generated Code (Before/After)

**Before** (3 E0382 errors):
```rust
pub fn main() {
    let args = Args::parse();
    match args.command {  // ❌ Moves args.command
        Commands::Clone { url } => {  // ❌ Moves url field
            handle_clone(&args);  // ❌ Borrow after partial move
        }
        Commands::Push { remote } => {  // ❌ Moves remote field
            handle_push(&args);  // ❌ Borrow after partial move
        }
        ...
    }
}
```

**After** (0 errors):
```rust
pub fn main() {
    let args = Args::parse();
    let _cse_temp_0 = matches!(args.command, Commands::Clone { .. });
    match &args.command {  // ✅ Borrows args.command
        Commands::Clone { .. } => {  // ✅ Ignores all fields
            handle_clone(&args);  // ✅ Can borrow args
        }
        Commands::Push { .. } => {  // ✅ Ignores all fields
            handle_push(&args);  // ✅ Can borrow args
        }
        ...
    }
}

pub fn handle_clone(args: &Args) {
    if let Commands::Clone { url } = &args.command {  // ✅ Re-extracts field
        println!("{}", format!("Clone: {}", url));
    }
}
```

## Pattern Analysis

**Python Pattern**:
```python
if args.command == "clone":
    handle_clone(args)  # Pass args to handler
```

**Old Rust (Broken)**:
```rust
match args.command {
    Commands::Clone { url } => {  // Extract field in match
        handle_clone(&args);  // ❌ But still pass full args
    }
}
```

**New Rust (Correct)**:
```rust
match &args.command {
    Commands::Clone { .. } => {  // Just dispatch
        handle_clone(&args);  // ✅ Pass full args
    }
}
```

**Key Insight**: Python passes the entire `args` object to handlers, which then access specific fields. The Rust match should just dispatch to the right handler, not extract fields.

## Impact

**Single-Shot Compilation**: example_subcommands is now the **second example** to achieve 100% single-shot compilation!

**Broader Impact**: This fix benefits ALL subcommand patterns:
- No more partial move errors in subcommand dispatch
- Cleaner generated code (no unused bindings)
- Matches Python's dispatch semantics more accurately

## Progress Metrics

**Examples Achieving 100% Single-Shot Compilation**:
1. ✅ example_config (DEPYLER-0473)
2. ✅ example_subcommands (DEPYLER-0474) 🆕
3. ✅ example_simple (already working)
4. ✅ example_flags (already working)
5. ✅ example_complex (already working)

**Remaining Examples**: ~8 more to fix

## Quality Gates

- ✅ example_subcommands: **0 compilation errors**
- ✅ make lint: **PASSING**
- ✅ No new test regressions
- ✅ Cleaner code (removed unused field_bindings extraction)

## Related Tickets

- DEPYLER-0456: Subcommand transpilation bugs (enum generation, field access)
- DEPYLER-0470: Match exhaustiveness (wildcard arm)
- DEPYLER-0473: Dict key borrowing fixes (similar borrowing pattern)

---

**🎉 MILESTONE: Second 100% Single-Shot Compilation Example! 🎉**
