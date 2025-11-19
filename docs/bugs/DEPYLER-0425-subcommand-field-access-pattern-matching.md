# DEPYLER-0425: Subcommand Field Access Requires Pattern Matching

**Status**: 🔴 CRITICAL
**Priority**: P0 (STOP ALL WORK)
**Severity**: Compilation failure
**Created**: 2025-11-18
**Component**: argparse_transform (subcommand handling)

---

## Problem Statement

When generating handlers for argparse subcommands, depyler generates code that tries to access subcommand-specific fields (like `url`, `remote`) directly on the `Args` struct via `args.url`, `args.remote`, etc.

However, these fields are actually inside the `Commands` enum variants (`Commands::Clone { url }`, `Commands::Push { remote }`, etc.), not on `Args` directly. The `Args` struct only has `args.command: Commands`.

**Impact**: All subcommand examples fail compilation with E0609 errors ("no field `X` on type `&Args`").

**Affected Examples**:
- `git_clone.py` (6 errors)
- `complex_cli.py` (subcommand portions)

---

## Root Cause Analysis

**File**: `crates/depyler-core/src/rust_gen/argparse_transform.rs`

**Problem**: When transpiling handler functions that receive `args` parameter, the code generator doesn't recognize that subcommand-specific fields need pattern matching to extract.

**Current Generated Code** (WRONG):
```rust
pub fn handle_clone(args: &Args) {
    if args.verbose {
        println!("{}", format!("Clone: {:?}", args.url));  // ❌ ERROR: no field `url`
    }
}
```

**Expected Generated Code** (CORRECT):
```rust
pub fn handle_clone(args: &Args) {
    if let Commands::Clone { url } = &args.command {
        if args.verbose {
            println!("{}", format!("Clone: {:?}", url));  // ✅ OK: extracted via pattern matching
        }
    }
}
```

**Why This Happens**:
1. Handler functions receive `args: &Args` parameter
2. Python code accesses `args.url`, `args.remote`, etc. (flat namespace)
3. Transpiler generates Rust code with same field access pattern
4. But in Rust, these fields are inside `args.command` enum variant
5. Requires pattern matching: `if let Commands::Clone { url } = &args.command`

---

## Minimal Reproduction

**Python Source** (`test_subcommand_access.py`):
```python
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    clone_parser = subparsers.add_parser("clone")
    clone_parser.add_argument("url", help="Repository URL")

    args = parser.parse_args()

    if args.command == "clone":
        handle_clone(args)

def handle_clone(args):
    print(f"Clone: {args.url}")  # ❌ Transpiles to args.url (wrong)

if __name__ == "__main__":
    main()
```

**Generated Rust** (CURRENT - WRONG):
```rust
#[derive(clap::Subcommand)]
enum Commands {
    Clone { url: String },
}

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

pub fn handle_clone(args: &Args) {
    println!("{}", format!("Clone: {:?}", args.url));  // ❌ ERROR: no field `url`
}
```

**Compilation Error**:
```
error[E0609]: no field `url` on type `&Args`
  --> test_subcommand_access.rs:15:52
   |
15 |     println!("{}", format!("Clone: {:?}", args.url));
   |                                                ^^^ unknown field
   |
   = note: available fields are: `command`
```

---

## Solution Design

### Option 1: Pattern Matching in Handler Body (RECOMMENDED)

**Pros**:
- Clean handler signatures (`args: &Args`)
- Clear pattern matching at usage site
- Handles missing variant gracefully

**Cons**:
- Adds boilerplate to handler body
- Requires tracking which fields belong to which variant

**Implementation**:
```rust
pub fn handle_clone(args: &Args) {
    if let Commands::Clone { url } = &args.command {
        if args.verbose {
            println!("{}", format!("Clone: {:?}", url));
        }
    }
}
```

### Option 2: Variant-Specific Parameter Types

**Pros**:
- Type-safe: handler only receives relevant fields
- No pattern matching needed in body

**Cons**:
- Changes handler signature (more invasive)
- Requires creating per-variant structs

**Implementation**:
```rust
pub fn handle_clone(global: &GlobalArgs, cmd: &CloneArgs) {
    if global.verbose {
        println!("{}", format!("Clone: {:?}", cmd.url));
    }
}
```

### Option 3: Direct Variant Passing

**Pros**:
- Minimal changes to handler signature
- Type-safe via enum

**Cons**:
- Changes handler semantics (receives enum variant, not full Args)

**Implementation**:
```rust
pub fn handle_clone(args: &Args, cmd: &Commands) {
    if let Commands::Clone { url } = cmd {
        if args.verbose {
            println!("{}", format!("Clone: {:?}", url));
        }
    }
}
```

**DECISION**: Use **Option 1** (Pattern Matching in Handler Body) because:
1. Minimal changes to existing transpilation logic
2. Preserves handler signature semantics
3. Clear and idiomatic Rust
4. Matches argparse's flat namespace behavior

---

## Implementation Plan

### Phase 1: Detection (TDD - RED)

**Test**: `crates/depyler-core/tests/depyler_0425_subcommand_field_access.rs`

```rust
#[test]
fn test_DEPYLER_0425_subcommand_field_extraction() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    clone_parser = subparsers.add_parser("clone")
    clone_parser.add_argument("url")

    args = parser.parse_args()
    if args.command == "clone":
        handle_clone(args)

def handle_clone(args):
    print(f"Clone: {args.url}")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_and_compile(python);
    assert!(result.compiles, "Subcommand field access must compile");
    assert!(result.rust_code.contains("if let Commands::Clone { url }"));
}
```

### Phase 2: Field Access Analysis (GREEN)

**File**: `crates/depyler-core/src/rust_gen/argparse_transform.rs`

**Add**:
```rust
/// Analyze which fields in a handler function belong to subcommand variants
fn analyze_subcommand_fields(
    func: &FunctionDef,
    subcommands: &HashMap<String, Vec<String>>,  // variant -> field names
) -> HashMap<String, Vec<String>> {
    // Map: field_name -> variant_name
    let mut field_to_variant = HashMap::new();

    // Walk function body, find all `args.FIELD` accesses
    // Match against subcommand field names
    // Return mapping

    field_to_variant
}
```

### Phase 3: Pattern Match Generation (GREEN)

**File**: `crates/depyler-core/src/rust_gen/func_gen.rs`

**Modify**: `generate_handler_body()`

```rust
fn generate_handler_body(func: &FunctionDef, args_param: &str) -> String {
    // 1. Detect if this is a subcommand handler (receives args: &Args)
    // 2. Analyze which subcommand fields are used
    // 3. Wrap body in pattern matching:

    let variant_name = detect_variant_for_handler(func);
    let fields = extract_fields_for_variant(func, variant_name);

    format!(r#"
        if let Commands::{variant_name} {{ {fields} }} = &{args_param}.command {{
            {original_body}
        }}
    "#)
}
```

### Phase 4: Field Reference Rewriting (REFACTOR)

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`

**Modify**: `generate_attribute()` for `args.field` expressions

```rust
fn generate_attribute(&mut self, attr: &Attribute) -> String {
    // Current: args.url -> "args.url"
    // New: args.url -> "url" (if inside pattern match block)

    if self.in_subcommand_handler && attr.value is "args" {
        // Just the field name (extracted via pattern matching)
        return attr.attr.clone();
    }

    // Otherwise, normal attribute access
    format!("{}.{}", self.generate_expr(attr.value), attr.attr)
}
```

---

## Test Plan

### Unit Tests

1. **Single subcommand field access**:
   - Input: `args.url` in `handle_clone`
   - Output: `if let Commands::Clone { url } = &args.command { ... url ... }`

2. **Multiple fields in one variant**:
   - Input: `args.url`, `args.branch` in same handler
   - Output: `if let Commands::Clone { url, branch } = ...`

3. **Global + subcommand fields**:
   - Input: `args.verbose` (global), `args.url` (subcommand)
   - Output: `args.verbose` unchanged, `url` extracted via pattern

4. **Multiple subcommands**:
   - Input: `handle_clone(args)`, `handle_push(args)` with different fields
   - Output: Each handler has correct variant pattern match

### Integration Tests

1. **git_clone.py** (6 errors → 0 errors):
   - Compile transpiled output
   - Run: `./git_clone clone https://github.com/test/repo`
   - Verify: "Clone: https://github.com/test/repo"

2. **complex_cli.py** (subcommand portions):
   - Compile transpiled output
   - Verify all subcommand handlers work

### Property Tests

```rust
#[quickcheck]
fn prop_subcommand_fields_always_extracted(
    num_variants: usize,
    fields_per_variant: usize,
) -> bool {
    // Generate random subcommand definitions
    // Verify all field accesses in handlers use pattern matching
    // Check compilation succeeds
}
```

---

## Quality Gates

### Before Fix
- ❌ git_clone: 6 compilation errors
- ❌ complex_cli: 13 compilation errors (partial)
- ⚠️ TDG: argparse_transform.rs ≈ 1.8

### After Fix (REQUIRED)
- ✅ git_clone: 0 errors, compiles successfully
- ✅ complex_cli: subcommand errors fixed (remaining errors are separate bugs)
- ✅ TDG: argparse_transform.rs ≤ 2.0
- ✅ Complexity: ≤10 for all new functions
- ✅ Coverage: ≥80% for modified code
- ✅ All existing tests pass

---

## Verification Commands

```bash
# 1. Add failing test
cd /home/noah/src/depyler
cat > crates/depyler-core/tests/depyler_0425_subcommand_field_access.rs << 'EOF'
[test content]
EOF

# 2. Run test (MUST FAIL)
cargo test test_DEPYLER_0425  # Should fail

# 3. Implement fix in argparse_transform.rs
# ... code changes ...

# 4. Run test (MUST PASS)
cargo test test_DEPYLER_0425  # Should pass

# 5. Re-transpile git_clone example
cd /home/noah/src/reprorusted-python-cli/examples/example_subcommands
depyler transpile git_clone.py -o git_clone.rs

# 6. Verify compilation
cargo clean && cargo build --release

# 7. Verify execution
./target/release/git_clone clone https://github.com/test/repo

# 8. Quality gates
cd /home/noah/src/depyler
pmat analyze tdg --path crates/depyler-core/src/rust_gen/argparse_transform.rs --threshold 2.0
pmat analyze complexity --file crates/depyler-core/src/rust_gen/argparse_transform.rs --max-cyclomatic 10
cargo test --workspace
cargo clippy -- -D warnings
```

---

## Rollout Strategy

### Phase 1: Core Fix
1. Implement field-to-variant mapping in argparse_transform.rs
2. Add pattern match wrapper generation in func_gen.rs
3. Update field reference generation in expr_gen.rs

### Phase 2: Re-transpile Examples
1. git_clone.py → git_clone.rs (verify 6 → 0 errors)
2. complex_cli.py → complex_cli.rs (partial fix)

### Phase 3: Documentation
1. Update CHANGELOG.md with DEPYLER-0425 fix
2. Add example to docs/argparse-patterns.md
3. Update roadmap.yaml

---

## Dependencies

**Blocks**:
- DEPYLER-0426 (complex_cli remaining errors)
- Full reprorusted-python-cli success (3/13 → 4/13+)

**Blocked By**:
- None (highest priority)

**Related**:
- DEPYLER-0424 (handler function Args type) - prerequisite (DONE)

---

## Lessons Learned

1. **Python's Flat Namespace vs Rust's Type Safety**:
   - Python argparse: `args.url` works regardless of subcommand
   - Rust clap: Must pattern match enum variant first
   - Transpiler must bridge this semantic gap

2. **Handler Signature Preservation**:
   - Maintaining `fn handle_clone(args: &Args)` signature is important
   - Matches Python semantics (all handlers receive same `args` object)
   - Pattern matching inside body is the Rust-idiomatic solution

3. **Field Provenance Tracking**:
   - Need to track which fields belong to which enum variant
   - Requires global context (argparse parser definition)
   - Cannot be determined from handler function alone

---

## Success Criteria

- [ ] Test `test_DEPYLER_0425_subcommand_field_access` passes
- [ ] git_clone.py transpiles and compiles (0 errors)
- [ ] Execution: `./git_clone clone URL` works correctly
- [ ] complex_cli.py subcommand portions compile
- [ ] TDG ≤ 2.0, Complexity ≤ 10, Coverage ≥ 80%
- [ ] Zero clippy warnings
- [ ] All existing tests pass

---

**Estimated Effort**: 3-4 hours
**Actual Effort**: TBD
**Complexity**: High (requires cross-module context tracking)

---

## References

- Python argparse docs: https://docs.python.org/3/library/argparse.html#sub-commands
- Rust clap derive docs: https://docs.rs/clap/latest/clap/_derive/index.html#subcommands
- DEPYLER-0424: Handler function Args type (prerequisite)
