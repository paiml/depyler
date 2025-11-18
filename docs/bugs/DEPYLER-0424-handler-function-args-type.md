# DEPYLER-0424: Handler Function Args Parameter Type Incorrect

**Status**: In Progress
**Priority**: P0 (STOP ALL WORK - Compilation Failure)
**Severity**: Critical - Prevents all argparse subcommand examples from compiling
**Component**: argparse_transform, func_gen
**Affected Files**: 10/13 reprorusted-python-cli examples

---

## Problem Statement

When Python functions receive the result of `parser.parse_args()` as a parameter, the transpiler generates incorrect Rust function signatures using `&serde_json::Value` instead of `&Args`.

### Example Failure

**Python Input** (`git_clone.py`):
```python
def handle_clone(args):
    """Handle the 'clone' subcommand."""
    if args.verbose:
        print(f"Clone: {args.url}")
```

**Generated Rust** (INCORRECT):
```rust
pub fn handle_clone(args: &serde_json::Value) {
    if args.verbose {  // ERROR: no field `verbose` found for `&serde_json::Value`
        println!("{}", format!("Clone: {:?}", args.url));
    }
}
```

**Expected Rust** (CORRECT):
```rust
pub fn handle_clone(args: &Args) {
    if args.verbose {  // ✓ Args has field `verbose`
        println!("{}", format!("Clone: {:?}", args.url));
    }
}
```

### Compilation Errors

```
error[E0609]: no field `verbose` found for reference `&serde_json::Value`
  --> git_clone.rs:57:12
   |
57 |     if args.verbose {
   |            ^^^^^^^^ field not found in `&serde_json::Value`
```

**12 errors** in git_clone.rs alone. Affects 10 out of 13 examples in reprorusted-python-cli.

---

## Root Cause Analysis

### Detection Chain

1. **ArgumentParser Creation** (stmt_gen.rs:1448-1478):
   - Detects `parser = argparse.ArgumentParser(...)`
   - Creates `ArgParserInfo` and registers it

2. **parse_args Detection** (stmt_gen.rs:1481-1493):
   - Detects `args = parser.parse_args()`
   - Stores `args` as `args_var` in `ArgParserInfo`
   - Generates `let args = Args::parse();`

3. **Function Parameter Typing** (func_gen.rs:238-322):
   - When generating `handle_clone(args)`, looks up parameter type
   - Finds no type annotation (Python `args` parameter is untyped)
   - Maps to `Type::Unknown` → `serde_json::Value` (type_mapper.rs:124)

### The Bug

**File**: `crates/depyler-core/src/rust_gen/func_gen.rs`
**Function**: `codegen_single_param` (line 238)

The function parameter type mapping does NOT check if the parameter name matches the ArgParserTracker's `args_var`. It blindly maps untyped parameters to `serde_json::Value`.

```rust
// Current behavior (WRONG):
// param.name = "args"
// param.ty = Type::Unknown
// → rust_type = serde_json::Value  ❌

// Expected behavior (CORRECT):
// param.name = "args"
// ctx.argparser_tracker.get_first_parser().args_var = Some("args")
// → rust_type = &Args  ✓
```

### Why This Happens

1. **Scope Isolation**: The `Args` struct is generated inside `main()` function scope
2. **No Type Propagation**: Handler functions are defined at module level
3. **Missing Context**: `codegen_single_param` doesn't consult `ArgParserTracker`

---

## Solution Design

### Strategy

Add argparse-aware parameter type resolution to `func_gen.rs::codegen_single_param()`.

### Implementation

**Step 1**: Check ArgParserTracker before mapping parameter type

```rust
// In codegen_single_param (line 238)
fn codegen_single_param(
    param: &HirParam,
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let param_name = param.name.clone();
    let param_ident = syn::Ident::new(&param_name, proc_macro2::Span::call_site());

    // NEW: Check if this parameter is the argparse args variable
    let is_argparse_args = ctx
        .argparser_tracker
        .parsers
        .values()
        .any(|parser_info| {
            parser_info
                .args_var
                .as_ref()
                .map_or(false, |args_var| args_var == &param.name)
        });

    if is_argparse_args {
        // Use &Args instead of mapping from param.ty
        return Ok(quote! { #param_ident: &Args });
    }

    // ... rest of existing logic
}
```

**Step 2**: Ensure Args struct is accessible

- **Problem**: Args struct is defined inside `main()` function
- **Solution**: Hoist Args struct to module level OR pass by reference

**Current codegen** (stmt_gen.rs generates Args inside main):
```rust
pub fn main() {
    #[derive(clap::Parser)]
    struct Args { ... }  // ← Local to main()

    let args = Args::parse();
    handle_clone(args);  // ← Passing ownership
}

pub fn handle_clone(args: &Args) {  // ← ERROR: Args not in scope
    ...
}
```

**Fix Option 1**: Hoist Args to module level (RECOMMENDED)
```rust
#[derive(clap::Parser)]
struct Args { ... }  // ← Module-level

pub fn main() {
    let args = Args::parse();
    handle_clone(&args);  // ← Pass by reference
}

pub fn handle_clone(args: &Args) {  // ← ✓ Args is in scope
    ...
}
```

**Fix Option 2**: Make Args struct public and use fully qualified path
```rust
pub fn main() {
    #[derive(clap::Parser)]
    pub struct Args { ... }
    // Won't work - struct can't be pub inside function
}
```

**Conclusion**: Must hoist Args struct to module level.

---

## Testing Strategy

### Unit Tests

**File**: `crates/depyler-core/tests/test_argparse_handler_types.rs`

```rust
#[test]
fn test_DEPYLER_0424_handler_function_args_type() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    handle_command(args)

def handle_command(args):
    if args.verbose:
        print("Verbose mode")
"#;

    let rust = transpile_str(python).unwrap();

    // Args struct should be at module level
    assert!(rust.contains("struct Args"));
    assert!(!rust.contains("pub fn main() {") ||
            rust.find("struct Args").unwrap() < rust.find("pub fn main()").unwrap());

    // Handler should use &Args, not &serde_json::Value
    assert!(rust.contains("pub fn handle_command(args: &Args)"));
    assert!(!rust.contains("serde_json::Value"));
}
```

### Integration Tests

**Goal**: Verify all 13 reprorusted-python-cli examples compile

```bash
cd /home/noah/src/reprorusted-python-cli
./test_compile_proper.sh

# Expected: 13/13 passing (currently 3/13)
```

### Property Tests

**Invariants**:
1. If function parameter name == ArgParserInfo.args_var, type MUST be `&Args`
2. Args struct MUST be defined before any function that references it
3. Args::parse() call MUST pass reference to handler functions

---

## Estimated Scope

**Complexity**: Medium (≤10 complexity target met)
**Lines Changed**: ~30 lines
**Files Modified**:
- `crates/depyler-core/src/rust_gen/func_gen.rs` (parameter type resolution)
- `crates/depyler-core/src/rust_gen/stmt_gen.rs` (hoist Args struct generation)

**Test Files**:
- New: `crates/depyler-core/tests/test_argparse_handler_types.rs`
- Update: 13 examples in reprorusted-python-cli

---

## Implementation Checklist

- [ ] Add argparse args detection to `codegen_single_param`
- [ ] Hoist Args struct generation to module level
- [ ] Update function call sites to pass `&args` instead of `args`
- [ ] Add unit tests (test_argparse_handler_types.rs)
- [ ] Re-transpile all 13 reprorusted-python-cli examples
- [ ] Verify compilation: `./test_compile_proper.sh` → 13/13 passing
- [ ] Run quality gates (TDG ≤2.0, complexity ≤10, coverage ≥80%)
- [ ] Commit with proper format

---

## Related Issues

- **DEPYLER-0363**: Initial argparse → clap transformation
- **DEPYLER-0399**: Subcommand support implementation
- **DEPYLER-0400**: Missing return types (related typing issue)

---

## Quality Metrics

**Pre-Fix**:
- Compilation: 3/13 examples (23.1%)
- Coverage: N/A (tests can't run)

**Post-Fix Target**:
- Compilation: 13/13 examples (100%) ✓
- TDG Score: ≤2.0 ✓
- Cyclomatic Complexity: ≤10 ✓
- Test Coverage: ≥80% ✓

---

**Document Version**: 1.0
**Last Updated**: 2025-11-18
**Author**: Claude Code (Anthropic)
