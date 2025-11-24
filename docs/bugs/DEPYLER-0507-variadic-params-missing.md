# DEPYLER-0507: Variadic parameters (*args) completely missing from function signatures

## Problem Statement

Functions with variadic parameters (`*args`) are transpiled without any parameters at all, causing compilation failures with "variable not found" errors.

**Minimal Reproducer**:
```python
def join_paths(*parts):
    result = os.path.join(*parts)
    print(f"Joined path: {result}")
    return result
```

**Transpiled (INCORRECT)**:
```rust
pub fn join_paths() -> String {
    let result = if parts.is_empty() {  // ❌ parts not defined!
        String::new()
    } else {
        parts.join(std::path::MAIN_SEPARATOR_STR)
    };
    println!("{}", format!("Joined path: {}", result));
    result.to_string()
}
```

**Expected (CORRECT)**:
```rust
pub fn join_paths(parts: &[String]) -> String {
    let result = if parts.is_empty() {
        String::new()
    } else {
        parts.join(std::path::MAIN_SEPARATOR_STR)
    };
    println!("{}", format!("Joined path: {}", result));
    result.to_string()
}
```

**Error**: `cannot find value 'parts' in this scope`

**Discovery**: Systematic testing of `reprorusted-python-cli/examples/example_environment/env_info.py`

## Five-Whys Root Cause Analysis

**1. Why does compilation fail?**
- Variable `parts` is referenced but not defined in function scope

**2. Why is `parts` not defined?**
- Function signature has zero parameters: `pub fn join_paths()` instead of `pub fn join_paths(parts: &[String])`

**3. Why was the parameter not added to signature?**
- Function conversion code in `ast_bridge/mod.rs` doesn't handle `ast::ArgWithDefault` with `is_vararg: true`
- Variadic parameters are skipped during parameter conversion

**4. Why are variadic parameters skipped?**
- The `convert_function_params()` function likely filters out parameters where `is_vararg == true`
- OR: HIR `HirParam` marks `is_vararg` but codegen ignores it
- OR: Parameter conversion doesn't check `is_vararg` field at all

**5. ROOT CAUSE: Incomplete parameter conversion logic**
- Either:
  - A) HIR correctly represents `is_vararg` but Rust codegen doesn't emit `&[T]` slice parameters
  - B) HIR conversion skips variadic parameters entirely (doesn't add to `params` vector)
  - C) `is_vararg` field exists but is never set to `true` during conversion

## Evidence Collection

**Source File**: `reprorusted-python-cli/examples/example_environment/env_info.py:88`
```python
def join_paths(*parts):
    """
    Join path components and display result

    Args:
        parts: Path components to join

    Depyler: proven to terminate
    """
    result = os.path.join(*parts)
    print(f"Joined path: {result}")
    return result
```

**Transpiled File**: `reprorusted-python-cli/examples/example_environment/env_info.rs:191`
```rust
pub fn join_paths() -> String {
    let result = if parts.is_empty() {
        String::new()
    } else {
        parts.join(std::path::MAIN_SEPARATOR_STR)
    };
    println!("{}", format!("Joined path: {}", result));
    result.to_string()
}
```

**Compilation Error**:
```
error[E0425]: cannot find value `parts` in this scope
   --> env_info.rs:194:21
    |
194 |     let result = if parts.is_empty() {
    |                     ^^^^^ not found in this scope
```

**Golden Trace Context**:
- Python execution: `python env_info.py join /home user docs` works correctly
- Syscalls: Standard file operations (getdents64, openat, read) - no anomalies
- Expected behavior: Path components joined with platform separator

## Hypothesis Testing

**Hypothesis**: HIR conversion drops variadic parameters

**Test**:
```python
def test_function(*args):
    return len(args)
```

**Expected HIR**:
```rust
HirFunc {
    name: "test_function",
    params: vec![
        HirParam {
            name: "args",
            ty: Type::Unknown,
            default: None,
            is_vararg: true,  // ← Should be true
        }
    ],
    body: ...
}
```

**Investigation Required**:
1. Check `crates/depyler-core/src/ast_bridge/mod.rs` - `convert_function_params()` logic
2. Check if `is_vararg` field is populated from `rustpython_ast::Arguments`
3. Check `crates/depyler-core/src/rust_gen.rs` - parameter codegen logic
4. Verify HIR→Rust conversion handles `is_vararg: true` parameters

## Impact Assessment

**Severity**: **CRITICAL (P0)** - Breaks all functions with variadic parameters

**Affected Functions**:
- Any function with `*args` parameter
- Common patterns: `join_paths(*parts)`, `print_all(*items)`, `concat(*strings)`
- Real-world usage: CLI tools, path utilities, variadic wrappers

**Workaround**: None (requires transpiler fix)

**Examples Affected**:
- `reprorusted-python-cli/examples/example_environment/env_info.py`
- Potentially many more using variadic parameters

## Solution Design

### Option A: Add Slice Parameter to Signature (Recommended)

**Approach**: Convert `*args` → `args: &[T]` in Rust

**HIR Change**: None needed (already has `is_vararg` field)

**Codegen Change**: In `rust_gen.rs`, when generating function signature:
```rust
for param in &func.params {
    if param.is_vararg {
        // Generate slice parameter
        format!("{}: &[{}]", param.name, infer_element_type(param))
    } else {
        // Normal parameter
        format!("{}: {}", param.name, rust_type(param.ty))
    }
}
```

**Element Type Inference**:
- If all uses are strings: `&[String]`
- If mixed types: `&[Box<dyn Any>]` (dynamic)
- Default: `&[String]` (most common)

**Pros**:
- Idiomatic Rust (slice parameters common)
- Zero allocation (no Vec)
- Efficient (stack-allocated slice)

**Cons**:
- Caller must provide slice: `join_paths(&["a", "b"])`
- Type inference may be wrong (need fallback)

### Option B: Vec Parameter

**Approach**: Convert `*args` → `args: Vec<T>`

**Pros**:
- More flexible (owned data)
- Easier type inference

**Cons**:
- Requires allocation
- Less idiomatic
- Caller syntax: `join_paths(vec!["a", "b"])`

### Option C: Macro-based Variadic

**Approach**: Generate Rust macro for true variadics

```rust
macro_rules! join_paths {
    ($($part:expr),*) => {{
        let parts = vec![$($part.to_string()),*];
        join_paths_impl(&parts)
    }};
}
```

**Pros**:
- True variadic behavior
- Clean call site: `join_paths!("a", "b", "c")`

**Cons**:
- Complex codegen
- Macros not first-class
- Harder to debug

**RECOMMENDATION**: Option A (slice parameters) - Most idiomatic and efficient

## Implementation Plan

### Phase 1: Fix HIR Conversion (if needed)

**File**: `crates/depyler-core/src/ast_bridge/mod.rs`

**Task**: Ensure variadic parameters are added to `params` vector

```rust
fn convert_function_params(args: &ast::Arguments) -> Result<Vec<HirParam>> {
    let mut params = Vec::new();

    // Regular args
    for arg in &args.args {
        params.push(HirParam {
            name: arg.def.arg.to_string(),
            ty: extract_type(&arg.def.annotation),
            default: None,
            is_vararg: false,
        });
    }

    // VARARG (*args)
    if let Some(vararg) = &args.vararg {
        params.push(HirParam {
            name: vararg.arg.to_string(),
            ty: Type::Unknown,  // Inferred later
            default: None,
            is_vararg: true,  // ← CRITICAL
        });
    }

    Ok(params)
}
```

### Phase 2: Fix Rust Codegen

**File**: `crates/depyler-core/src/rust_gen.rs`

**Task**: Emit slice parameters for `is_vararg: true`

```rust
fn generate_function_signature(func: &HirFunc) -> String {
    let mut sig = format!("pub fn {}(", func.name);

    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            sig.push_str(", ");
        }

        if param.is_vararg {
            // Variadic parameter → slice
            let elem_type = infer_vararg_type(param, func);
            sig.push_str(&format!("{}: &[{}]", param.name, elem_type));
        } else {
            // Normal parameter
            sig.push_str(&format!("{}: {}", param.name, rust_type(&param.ty)));
        }
    }

    sig.push_str(")");
    sig
}

fn infer_vararg_type(param: &HirParam, func: &HirFunc) -> String {
    // Analyze function body to infer element type
    // 1. Check for method calls (e.g., .join() → String)
    // 2. Check for iteration patterns
    // 3. Default to String (most common)
    "String".to_string()
}
```

### Phase 3: Handle Call Sites

**File**: `crates/depyler-core/src/ast_bridge/converters.rs`

**Task**: Convert `func(*args)` calls to slice notation

```rust
// Python: join_paths(*parts)
// Rust: join_paths(parts)  // Already a slice from parent scope

// Python: join_paths("a", "b", "c")
// Rust: join_paths(&["a", "b", "c"])
```

**Logic**:
- If call uses `*arg` unpacking AND arg is already slice → pass directly
- If call uses multiple positional args → wrap in slice literal

## Test Plan

### RED Phase Tests

**File**: `crates/depyler-core/tests/depyler_0507_variadic_params.rs`

**Test 1**: Simple variadic function
```python
def concat(*args):
    return "".join(args)

result = concat("a", "b", "c")
```

**Expected Rust**:
```rust
pub fn concat(args: &[String]) -> String {
    args.join("")
}

let result = concat(&["a".to_string(), "b".to_string(), "c".to_string()]);
```

**Test 2**: Variadic with regular params
```python
def format_msg(prefix, *parts):
    return prefix + ": " + " ".join(parts)

msg = format_msg("INFO", "server", "started")
```

**Expected Rust**:
```rust
pub fn format_msg(prefix: String, parts: &[String]) -> String {
    format!("{}: {}", prefix, parts.join(" "))
}

let msg = format_msg("INFO".to_string(), &["server".to_string(), "started".to_string()]);
```

**Test 3**: Unpacking variadic args
```python
def join_paths(*parts):
    return os.path.join(*parts)

path = join_paths("home", "user", "docs")
```

**Expected Rust**:
```rust
pub fn join_paths(parts: &[String]) -> String {
    parts.join(std::path::MAIN_SEPARATOR_STR)
}

let path = join_paths(&["home".to_string(), "user".to_string(), "docs".to_string()]);
```

### Integration Test

**File**: `reprorusted-python-cli/examples/example_environment/env_info.py`

**Command**: `cargo run -- join /home user docs`

**Expected**:
- ✅ Compiles without errors
- ✅ Output: `Joined path: /home/user/docs` (Unix) or `home\user\docs` (Windows)
- ✅ Golden trace matches Python behavior

## Success Criteria

1. ✅ All variadic functions compile without "variable not found" errors
2. ✅ Rust signatures correctly include slice parameters
3. ✅ Call sites correctly wrap args in slices
4. ✅ `env_info.py` compiles and runs
5. ✅ Zero regressions in existing tests
6. ✅ Clippy passes with zero warnings

## Related Issues

- DEPYLER-0382: Starred expressions in function calls (partially related)
- DEPYLER-0494: Generator variable scoping (similar missing parameter issue)

## References

- Python AST: `ast::Arguments::vararg` field
- HIR: `HirParam::is_vararg` field
- Rust idiom: Slice parameters (`&[T]`) for variadic functions
- Codegen: `crates/depyler-core/src/rust_gen.rs`
