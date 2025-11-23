# DEPYLER-0425: Subcommand Field Extraction Analysis

**Status**: 🔍 ANALYSIS COMPLETE - Ready for implementation
**Date**: 2025-11-23
**Complexity**: MEDIUM - HIR analysis + pattern generation update
**Impact**: Fixes 3 errors in example_environment (13 → 10 errors)

---

## Problem Statement

Subcommand match arms generate `Commands::Variant { .. }` which ignores all fields, causing handler functions that access individual fields to fail with E0425 (cannot find value in scope).

### Example from example_environment

**Python Source**:
```python
elif args.command == "env":
    show_environment(args.variable)  # Pass specific field
elif args.command == "path":
    check_path(args.target)  # Pass specific field
elif args.command == "join":
    join_paths(*args.parts)  # Pass specific field (varargs)
```

**Current Transpilation** (broken):
```rust
match &args.command {
    Commands::Env { .. } => {  // ❌ Doesn't extract `variable`
        show_environment(variable);  // ❌ E0425: cannot find value `variable`
    }
    Commands::Path { .. } => {  // ❌ Doesn't extract `target`
        check_path(target);  // ❌ E0425: cannot find value `target`
    }
    Commands::Join { .. } => {  // ❌ Doesn't extract `parts`
        join_paths(parts);  // ❌ E0425: cannot find value `parts`
    }
}
```

**Required Transpilation**:
```rust
match &args.command {
    Commands::Env { variable } => {  // ✅ Extract field
        show_environment(variable);  // ✅ Compiles
    }
    Commands::Path { target } => {  // ✅ Extract field
        check_path(target);  // ✅ Compiles
    }
    Commands::Join { parts } => {  // ✅ Extract field
        join_paths(parts);  // ✅ Compiles
    }
}
```

---

## Root Cause Analysis

### Location

**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs`
**Function**: `try_generate_subcommand_match()` (lines 3438-3566)
**Specific Lines**: 3543-3549 (pattern generation)

### Current Implementation

```rust
// DEPYLER-0474: Don't extract field bindings, use `..` to ignore them
// Handler functions re-extract fields from &args, so bindings here are unused
quote! {
    Commands::#variant_name { .. } => {
        #(#body_stmts)*
    }
}
```

**Problem**: The comment says "Handler functions re-extract fields from &args", but this is **INCORRECT**. Handler functions receive individual fields as arguments, not `&args`.

### Incorrect Assumption

**DEPYLER-0474** assumed all handlers follow **Pattern A**:
```python
def handle_clone(args):  # Takes full args object
    url = args.url  # Extract fields inside handler
    print(url)

if args.command == "clone":
    handle_clone(args)  # Pass entire args object
```

**Reality**: example_environment uses **Pattern B**:
```python
def show_environment(variable):  # Takes individual field
    print(variable)  # Uses field directly

if args.command == "env":
    show_environment(args.variable)  # Pass specific field
```

### Why This Matters

**Pattern A**: Handler extracts fields → `{ .. }` works
**Pattern B**: Caller extracts fields → `{ field }` required

Currently ALL patterns use `{ .. }` (Pattern A), breaking Pattern B cases.

---

## Examples Breakdown

### Example 1: show_environment (variable)

**Python**:
```python
def show_environment(variable=None):
    """Display environment variable value"""
    if variable:
        value = os.getenv(variable)
        ...

# Handler call
if args.command == "env":
    show_environment(args.variable)  # ← Extracts args.variable
```

**Current HIR** (at match arm generation):
```rust
HirStmt::If {
    condition: Binary {  // args.command == "env"
        left: Attribute { object: "args", attr: "command" },
        op: Eq,
        right: Literal("env")
    },
    then_body: [
        Expr(Call {  // show_environment(args.variable)
            func: "show_environment",
            args: [
                Attribute { object: "args", attr: "variable" }  // ← Key: accesses variable field
            ]
        })
    ]
}
```

**Analysis**: The call accesses `args.variable`, so we must extract `variable` in the pattern.

---

### Example 2: check_path (target)

**Python**:
```python
def check_path(target):
    """Check if path exists and display info"""
    exists = os.path.exists(target)
    ...

# Handler call
if args.command == "path":
    check_path(args.target)  # ← Extracts args.target
```

**HIR Analysis**: Call accesses `args.target` → must extract `target`

---

### Example 3: join_paths (parts)

**Python**:
```python
def join_paths(*parts):
    """Join path components"""
    result = os.path.join(*parts)
    ...

# Handler call
if args.command == "join":
    join_paths(*args.parts)  # ← Extracts args.parts (varargs expansion)
```

**HIR Analysis**: Call accesses `args.parts` → must extract `parts`

---

## Solution Design

### Approach: Pattern Detection via HIR Analysis

**Goal**: Detect which fields are accessed in the match arm body and extract only those fields.

### Algorithm

1. **Analyze HIR body** before converting to Rust tokens
2. **Find all `args.field` accesses**:
   - Look for `HirExpr::Attribute { object: "args", attr: field_name }`
   - Collect unique field names
3. **Determine pattern**:
   - If no fields accessed → use `{ .. }` (Pattern A)
   - If fields accessed → use `{ field1, field2, ... }` (Pattern B)
4. **Generate appropriate pattern**:
   ```rust
   Commands::Variant { field1, field2 } => { ... }
   ```

### Implementation Plan

#### Step 1: Create Field Detection Helper

Add helper function to extract accessed fields from HIR:

```rust
/// DEPYLER-0425: Extract subcommand fields accessed in handler body
/// Analyzes HIR statements to find args.field attribute accesses
fn extract_accessed_subcommand_fields(
    body: &[HirStmt],
    args_var: &str,  // Usually "args"
) -> Vec<String> {
    let mut fields = std::collections::HashSet::new();
    extract_fields_recursive(body, args_var, &mut fields);
    let mut result: Vec<_> = fields.into_iter().collect();
    result.sort();  // Deterministic order
    result
}

fn extract_fields_recursive(
    stmts: &[HirStmt],
    args_var: &str,
    fields: &mut std::collections::HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Expr(expr) => extract_fields_from_expr(expr, args_var, fields),
            HirStmt::Assign { value, .. } => extract_fields_from_expr(value, args_var, fields),
            HirStmt::If { then_body, else_body, .. } => {
                extract_fields_recursive(then_body, args_var, fields);
                if let Some(else_stmts) = else_body {
                    extract_fields_recursive(else_stmts, args_var, fields);
                }
            }
            // ... other statement types
            _ => {}
        }
    }
}

fn extract_fields_from_expr(
    expr: &HirExpr,
    args_var: &str,
    fields: &mut std::collections::HashSet<String>,
) {
    match expr {
        // Pattern: args.field
        HirExpr::Attribute { object, attr } => {
            if let HirExpr::Var(var) = object.as_ref() {
                if var == args_var {
                    fields.insert(attr.clone());
                }
            }
        }
        // Recurse into nested expressions
        HirExpr::Call { args, .. } => {
            for arg in args {
                extract_fields_from_expr(arg, args_var, fields);
            }
        }
        HirExpr::Binary { left, right, .. } => {
            extract_fields_from_expr(left, args_var, fields);
            extract_fields_from_expr(right, args_var, fields);
        }
        // ... other expression types
        _ => {}
    }
}
```

**Complexity**: O(n) where n = number of HIR nodes in body (linear scan)

---

#### Step 2: Update Pattern Generation

Modify `try_generate_subcommand_match()` at line 3523-3550:

```rust
let arms: Vec<proc_macro2::TokenStream> = branches
    .iter()
    .map(|(cmd_name, body)| {
        let variant_name = format_ident!("{}", to_pascal_case_subcommand(cmd_name));

        // DEPYLER-0425: Detect which fields are accessed in the body
        let accessed_fields = extract_accessed_subcommand_fields(body, "args");

        // Generate body statements
        ctx.enter_scope();
        let body_stmts: Vec<_> = body
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()
            .unwrap_or_default();
        ctx.exit_scope();

        // DEPYLER-0425: Generate pattern based on accessed fields
        if accessed_fields.is_empty() {
            // Pattern A: No fields accessed, use { .. }
            quote! {
                Commands::#variant_name { .. } => {
                    #(#body_stmts)*
                }
            }
        } else {
            // Pattern B: Fields accessed, extract them
            let field_idents: Vec<syn::Ident> = accessed_fields
                .iter()
                .map(|f| format_ident!("{}", f))
                .collect();

            quote! {
                Commands::#variant_name { #(#field_idents),* } => {
                    #(#body_stmts)*
                }
            }
        }
    })
    .collect();
```

**Changes**:
1. Call `extract_accessed_subcommand_fields(body, "args")` BEFORE converting to tokens
2. Check if `accessed_fields.is_empty()`
3. Generate appropriate pattern based on result

---

#### Step 3: Handle Varargs Expansion

**Special Case**: `join_paths(*args.parts)`

**HIR**:
```rust
HirExpr::Call {
    func: "join_paths",
    args: [
        HirExpr::Starred(
            HirExpr::Attribute { object: "args", attr: "parts" }
        )
    ]
}
```

**Detection**: Also check for `HirExpr::Starred` wrapping attribute access.

**Update `extract_fields_from_expr()`**:
```rust
HirExpr::Starred(inner) => {
    // Varargs expansion: *args.field
    extract_fields_from_expr(inner, args_var, fields);
}
```

---

## Testing Strategy

### Unit Test

```rust
#[test]
fn test_extract_accessed_subcommand_fields() {
    // Create HIR for: show_environment(args.variable)
    let body = vec![HirStmt::Expr(HirExpr::Call {
        func: "show_environment".to_string(),
        args: vec![HirExpr::Attribute {
            object: Box::new(HirExpr::Var("args".to_string())),
            attr: "variable".to_string(),
        }],
    })];

    let fields = extract_accessed_subcommand_fields(&body, "args");
    assert_eq!(fields, vec!["variable"]);
}

#[test]
fn test_extract_multiple_fields() {
    // Test multiple field accesses
    // ...
}

#[test]
fn test_varargs_expansion() {
    // Test *args.parts pattern
    // ...
}
```

### Integration Test

**File**: `examples/example_environment/env_info.py`

**Before**:
```
Error count: 13
- 3 E0425: variable, target, parts not found
```

**After**:
```
Error count: 10 ✅
- 0 E0425 for subcommand fields ✅
```

**Verification**:
```bash
cd /home/noah/src/reprorusted-python-cli/examples/example_environment
cargo check 2>&1 | grep -c "E0425"
# Expected: 0 (down from 3)
```

---

## Files to Modify

### 1. stmt_gen.rs (Primary Changes)

**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs`

**Add** (near top of file, around line 50):
- `extract_accessed_subcommand_fields()` function (~30 lines)
- `extract_fields_recursive()` helper (~40 lines)
- `extract_fields_from_expr()` helper (~50 lines)

**Modify** (line 3523-3550):
- Update pattern generation in `try_generate_subcommand_match()`
- Add field detection before token generation
- Conditional pattern generation (Pattern A vs Pattern B)

**Total**: ~130 lines added/modified

---

## Edge Cases

### Edge Case 1: No Field Access (Pattern A)

**Python**:
```python
if args.command == "system":
    show_system_info()  # No args.field access
```

**Expected**:
```rust
Commands::System { .. } => {  // ✅ Correct - no fields needed
    show_system_info();
}
```

**Handling**: `accessed_fields.is_empty()` → use `{ .. }`

---

### Edge Case 2: Multiple Field Access

**Python**:
```python
if args.command == "deploy":
    deploy(args.env, args.region, args.dry_run)
```

**Expected**:
```rust
Commands::Deploy { env, region, dry_run } => {  // ✅ Extract all 3
    deploy(env, region, dry_run);
}
```

**Handling**: Collect all unique field names, generate comma-separated list

---

### Edge Case 3: Field Access in Nested Blocks

**Python**:
```python
if args.command == "config":
    if args.verbose:
        print(args.config_file)
    load_config(args.config_file)
```

**Expected**:
```rust
Commands::Config { verbose, config_file } => {  // ✅ Extract both
    if verbose {
        println!("{}", config_file);
    }
    load_config(config_file);
}
```

**Handling**: Recursive HIR traversal (covered by `extract_fields_recursive()`)

---

### Edge Case 4: Varargs Expansion

**Python**:
```python
if args.command == "join":
    join_paths(*args.parts)
```

**Expected**:
```rust
Commands::Join { parts } => {  // ✅ Extract parts
    join_paths(parts);  // Note: No * in Rust (parts is already Vec)
}
```

**Handling**: Detect `HirExpr::Starred` wrapping attribute access

---

## Impact Analysis

### Direct Impact: example_environment

**Before**: 13 errors
- 3 E0425: `variable`, `target`, `parts` not found

**After**: 10 errors
- 0 E0425 for subcommand fields ✅

**Reduction**: 23% (3/13 errors fixed)

---

### Single-Shot Compilation Progress

**Before**: 46% (6/13 examples)
**After**: 46% (6/13 examples - still has 10 other errors)

**Next Milestone**: After fixing remaining 10 errors in example_environment → 54% (7/13)

---

### Broader Impact

**Affected Pattern**: Any argparse program with subcommands that pass individual fields to handlers (Pattern B)

**Examples Potentially Affected**:
- example_environment ✅ (confirmed - fixes 3 errors)
- Any future examples using Pattern B subcommands

**No Regressions Expected**:
- Pattern A (no field access) → still generates `{ .. }` ✅
- Non-subcommand examples → unchanged ✅

---

## Risks & Mitigations

### Risk 1: Over-extraction

**Issue**: Extracting fields that aren't actually needed
**Example**: `args.command` itself shouldn't be extracted (it's the match discriminant)

**Mitigation**:
- Filter out `command` field (or `dest_field`)
- Only extract fields from subcommand variants (not Args struct itself)

### Risk 2: Missing Field Accesses

**Issue**: HIR analysis doesn't find all field accesses (e.g., in lambdas, comprehensions)

**Mitigation**:
- Comprehensive recursive traversal of all HIR expression types
- Test with complex examples (nested blocks, comprehensions)
- Fallback: If field access fails at runtime, user gets clear compiler error

### Risk 3: Field Name Conflicts

**Issue**: Extracted field shadows local variable
**Example**:
```python
if args.command == "config":
    config = load_config()  # Local variable
    deploy(args.config)  # Subcommand field
```

**Mitigation**:
- Rust's shadowing rules handle this correctly
- Later assignment shadows extracted field
- No special handling needed

### Risk 4: Wrong args Variable

**Issue**: Function has multiple parameters named differently
**Example**:
```python
def main(arguments):  # Not 'args'
    if arguments.command == "deploy":
        ...
```

**Mitigation**:
- Detect args variable name from pattern match expression
- Pass detected name to `extract_accessed_subcommand_fields()`
- Currently hardcoded to "args" (acceptable for argparse convention)

---

## Estimated Effort

**Implementation**: 3-4 hours
- Helper functions: 1.5 hours
- Pattern generation update: 1 hour
- Testing: 1 hour
- Documentation: 0.5 hours

**Complexity**: Medium
- HIR traversal: Well-understood pattern
- Pattern generation: Simple conditional logic
- No new concepts, just refining existing code

---

## Success Criteria

1. ✅ example_environment: 13 → 10 errors (3 fixed)
2. ✅ Zero E0425 errors for subcommand fields (`variable`, `target`, `parts`)
3. ✅ make lint passes (no clippy warnings)
4. ✅ No regressions in 6 passing examples
5. ✅ Pattern A (no field access) still works correctly

---

## Next Steps After DEPYLER-0425

**Remaining example_environment errors**: 10

**Categories**:
1. Type conversions (Path, OsStr, Option<String>) - 8 errors
2. Other issues - 2 errors

**Next Priority Task**: Type conversion improvements or tackle another failing example (example_io_streams, example_csv_filter)

---

**Analysis Status**: ✅ COMPLETE - Ready for implementation
**Recommended Action**: Proceed with implementation following the plan above
