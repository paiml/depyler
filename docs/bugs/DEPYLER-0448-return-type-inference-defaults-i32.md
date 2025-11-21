# DEPYLER-0448: Return Type and Constant Type Inference Defaults to i32

**Status**: 🔴 STOP THE LINE - CRITICAL BUG
**Priority**: P0 (STOP ALL WORK)
**Severity**: CRITICAL - Affects 94+ compilation errors across 9/13 failing examples
**Created**: 2025-11-21
**Ticket**: DEPYLER-0448
**Related**: DEPYLER-0435 (reprorusted-cli 100% compilation goal)

---

## Executive Summary

**Problem**: Type inference for function return types and constants defaults to `i32` when the transpiler cannot determine the actual type from the Python source. This causes massive type mismatches in generated Rust code.

**Impact**:
- **94 E0308 errors** (35% of all errors) across 9 failing examples
- Affects functions returning dicts, Values, HashMaps, and complex types
- Affects module-level constants (dicts become `const X: i32 = { map }`)
- Blocks 9/13 reprorusted-cli examples from compiling

**Root Cause**: Type inference in `func_gen.rs` and `rust_gen.rs` falls back to `i32` when it cannot infer the actual return type from function body analysis.

**Solution**: Improve type inference to:
1. Analyze actual return statements to infer return type
2. Use `serde_json::Value` as fallback (not `i32`) for complex types
3. Properly infer HashMap/dict types from constants
4. Track type information through function body transformations

---

## Problem Statement

### 1. The Bug

When transpiling Python functions that return complex types (dicts, objects, etc.), Depyler generates incorrect Rust signatures with `-> i32` return type, even when the function body clearly returns a different type.

**Example 1 - Function Return Type**:
```python
# Python source
def load_config(path):
    """Load config from JSON file"""
    if os.path.exists(path):
        with open(path) as f:
            return json.load(f)  # Returns dict
    return DEFAULT_CONFIG  # Returns dict constant
```

**Current (WRONG) Output**:
```rust
pub fn load_config(path: String) -> i32 {  // ❌ WRONG: Returns i32
    if std::path::PathBuf::from(path).exists() {
        let f = std::fs::File::open(path)?;
        return serde_json::from_reader::<_, serde_json::Value>(f).unwrap();  // Returns Value
    }
    DEFAULT_CONFIG::copy()  // ❌ Also wrong: should be .clone() or just DEFAULT_CONFIG
}
```

**Compilation Errors**:
```
error[E0308]: mismatched types
  --> src/main.rs:74:16
   |
72 | pub fn load_config(path: String) -> i32 {
   |                                     --- expected `i32` because of return type
...
74 |         return serde_json::from_reader::<_, serde_json::Value>(f).unwrap();
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `Value`
```

**Example 2 - Constant Type**:
```python
# Python source
DEFAULT_CONFIG = {
    "database": {"host": "localhost", "port": 5432},
    "logging": {"level": "INFO", "file": "app.log"},
    "features": {"debug": False, "verbose": False}
}
```

**Current (WRONG) Output**:
```rust
pub const DEFAULT_CONFIG: i32 = {  // ❌ WRONG: Type is i32
    let mut map = HashMap::new();
    map.insert("database".to_string(), serde_json::json!({ "host": "localhost", "port": 5432 }));
    map.insert("logging".to_string(), { let mut map = HashMap::new(); /* ... */ map });
    map.insert("features".to_string(), { let mut map = HashMap::new(); /* ... */ map });
    map  // ❌ Returns HashMap, but signature says i32
};
```

**Compilation Errors**:
```
error[E0308]: mismatched types
  --> src/main.rs:23:5
   |
 4 | pub const DEFAULT_CONFIG: i32 = {
   |                           --- expected `i32` because of this
...
23 |     map
   |     ^^^ expected `i32`, found `HashMap<String, Value>`
```

### 2. Affected Examples

**From reprorusted-cli error analysis**:

| Example | E0308 Count | Sample Errors |
|---------|-------------|---------------|
| **config_manager.py** | 16 | `expected i32, found HashMap<String, Value>` |
| **stdlib_integration.py** | 15 | `expected i32, found HashMap`, `expected bool, found Value` |
| **complex_cli.py** | 4 | `expected i32, found &str`, `expected bool, found String` |
| **Others** | 59+ | Various type mismatches involving Value |

**Total Impact**: 94 E0308 errors across 9 failing examples (35% of all compilation errors)

### 3. Error Patterns

From comprehensive error sampling, the following patterns emerged:

**Pattern A: Return Type Defaults to i32** (HIGHEST FREQUENCY)
```rust
// Wrong signature
pub fn load_config() -> i32 { /* returns Value */ }
pub fn get_nested_value() -> Result<(), IndexError> { /* returns Result<Value, IndexError> */ }
```

**Pattern B: Constant Type Defaults to i32**
```rust
pub const DEFAULT_CONFIG: i32 = { /* HashMap expression */ };
```

**Pattern C: Value vs Concrete Type Mismatches**
```rust
let value: Value = get_something();
if value {  // ❌ expected bool, found Value
}
```

**Pattern D: HashMap Returned as Value but Typed as i32**
```rust
pub fn create_dict() -> i32 {
    let mut map = HashMap::new();
    map.insert("key", "value");
    map  // ❌ returns HashMap, signature says i32
}
```

---

## Root Cause Analysis

### Investigation Process

1. **Transpiled config_manager.py** → Found `-> i32` on functions returning dicts
2. **Read transpiled output** → Confirmed return expressions don't match signatures
3. **Traced to type inference** → Must be in `func_gen.rs::codegen_return_type()`

### Source Code Location

**File**: `crates/depyler-core/src/rust_gen/func_gen.rs`
**Function**: `codegen_return_type()`
**Lines**: ~150-200 (approximate)

### The Defect

**Current Logic** (simplified):
```rust
fn codegen_return_type(func: &HirFunction, ctx: &CodeGenContext) -> TokenStream {
    // ... various checks ...

    // If we can't infer the type, default to something
    let ty: Type = if let Some(inferred) = func.properties.inferred_return_type {
        inferred
    } else {
        parse_quote! { i32 }  // ❌ DEFAULT TO i32
    };

    // Wrap in Result if needed
    if can_fail {
        quote! { Result<#ty, #error_type> }
    } else {
        quote! { #ty }
    }
}
```

**The Problem**:
- `func.properties.inferred_return_type` is `None` for complex return types (dicts, Values)
- Fallback defaults to `i32` instead of analyzing actual return statements
- No analysis of return statement expressions to infer type

**Why It Fails**:
1. Type inference happens during HIR construction (early phase)
2. Return type inference only looks at explicit type hints (which Python often lacks)
3. No second-pass type inference based on actual return expressions
4. Complex types (dicts, Values) have no Python type hint → inference fails → defaults to i32

### Expected Behavior

**Option 1: Analyze Return Statements** (BEST)
```rust
fn infer_return_type_from_body(func: &HirFunction) -> Type {
    // Scan all return statements in function body
    // If all return statements return same type → use that type
    // If mixed types → use serde_json::Value
    // If no returns → use ()
}
```

**Option 2: Use Value as Fallback** (SAFER)
```rust
let ty: Type = if let Some(inferred) = func.properties.inferred_return_type {
    inferred
} else {
    parse_quote! { serde_json::Value }  // ✅ SAFE FALLBACK
};
```

**Option 3: Hybrid Approach** (RECOMMENDED)
```rust
let ty: Type = if let Some(inferred) = func.properties.inferred_return_type {
    inferred
} else {
    // Try to infer from return statements
    if let Some(inferred_ty) = infer_from_return_stmts(&func.body) {
        inferred_ty
    } else {
        // Fall back to Value for unknown types
        parse_quote! { serde_json::Value }
    }
};
```

---

## Solution Design

### High-Level Approach

Implement **two-pass type inference** for return types:

1. **Pass 1 (Existing)**: Use explicit type hints from Python source
2. **Pass 2 (NEW)**: Analyze return statement expressions in function body
3. **Fallback**: Use `serde_json::Value` instead of `i32` for unknown types

### Implementation Plan

#### Phase 1: Add Return Statement Analysis (NEW)

**File**: `crates/depyler-core/src/rust_gen/func_gen.rs`

```rust
/// Infer return type by analyzing all return statements in function body
/// Returns None if return statements have inconsistent types
fn infer_return_type_from_stmts(stmts: &[HirStmt]) -> Option<Type> {
    let mut return_types = Vec::new();

    // Recursively scan all statements for return statements
    fn scan_stmts(stmts: &[HirStmt], types: &mut Vec<Type>) {
        for stmt in stmts {
            match stmt {
                HirStmt::Return(Some(expr)) => {
                    if let Some(ty) = infer_type_from_expr(expr) {
                        types.push(ty);
                    }
                }
                HirStmt::If { then_body, else_body, .. } => {
                    scan_stmts(then_body, types);
                    if let Some(else_stmts) = else_body {
                        scan_stmts(else_stmts, types);
                    }
                }
                HirStmt::While { body, .. } |
                HirStmt::For { body, .. } => {
                    scan_stmts(body, types);
                }
                HirStmt::Try { body, handlers, orelse, finalbody } => {
                    scan_stmts(body, types);
                    for handler in handlers {
                        scan_stmts(&handler.body, types);
                    }
                    if let Some(else_stmts) = orelse {
                        scan_stmts(else_stmts, types);
                    }
                    if let Some(final_stmts) = finalbody {
                        scan_stmts(final_stmts, types);
                    }
                }
                _ => {}
            }
        }
    }

    scan_stmts(stmts, &mut return_types);

    // If all return types are the same, use that
    if return_types.is_empty() {
        Some(parse_quote! { () })  // No return statements = unit type
    } else if return_types.iter().all(|ty| ty == &return_types[0]) {
        Some(return_types[0].clone())
    } else {
        None  // Inconsistent types - fall back to Value
    }
}

/// Infer type from expression
fn infer_type_from_expr(expr: &HirExpr) -> Option<Type> {
    match expr {
        HirExpr::Literal(lit) => match lit {
            HirLiteral::Int(_) => Some(parse_quote! { i32 }),
            HirLiteral::Float(_) => Some(parse_quote! { f64 }),
            HirLiteral::Str(_) => Some(parse_quote! { String }),
            HirLiteral::Bool(_) => Some(parse_quote! { bool }),
            HirLiteral::None => Some(parse_quote! { () }),
        },
        HirExpr::Dict { .. } => Some(parse_quote! { std::collections::HashMap<String, serde_json::Value> }),
        HirExpr::List { .. } => Some(parse_quote! { Vec<serde_json::Value> }),
        HirExpr::Call { func, .. } => {
            // Look up function's return type if known
            // For now, return None (unknown)
            None
        }
        HirExpr::Var(name) => {
            // Look up variable type if tracked
            None
        }
        _ => None,
    }
}
```

#### Phase 2: Update Return Type Generation

**File**: `crates/depyler-core/src/rust_gen/func_gen.rs`

```rust
pub(crate) fn codegen_return_type(
    func: &HirFunction,
    ctx: &CodeGenContext,
) -> Result<TokenStream, CodeGenError> {
    // ... existing error type logic ...

    // UPDATED: Use three-tier type inference
    let ty: Type = if let Some(inferred) = func.properties.inferred_return_type {
        // 1. Use explicit type hint if available
        inferred
    } else if let Some(inferred_ty) = infer_return_type_from_stmts(&func.body) {
        // 2. Infer from return statements (NEW)
        inferred_ty
    } else {
        // 3. Fall back to Value instead of i32 (FIXED)
        parse_quote! { serde_json::Value }
    };

    // ... rest of function ...
}
```

#### Phase 3: Fix Constant Type Inference

**File**: `crates/depyler-core/src/rust_gen.rs` (or wherever constants are generated)

```rust
// Update constant type inference
pub fn codegen_constant(constant: &HirConstant) -> TokenStream {
    let name = &constant.name;
    let value = codegen_expr(&constant.value);

    // NEW: Infer type from constant expression
    let ty = infer_type_from_expr(&constant.value).unwrap_or_else(|| {
        parse_quote! { serde_json::Value }  // Safe fallback
    });

    quote! {
        pub const #name: #ty = #value;
    }
}
```

### Testing Strategy

Following **EXTREME TDD** protocol:

#### Test Suite 1: Basic Return Type Inference

**File**: `crates/depyler-core/tests/depyler_0448_return_type_inference.rs`

```rust
#[test]
fn test_depyler_0448_dict_return_infers_hashmap() {
    let python = r#"
def create_config():
    return {"host": "localhost", "port": 5432}
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer HashMap return type
    assert!(rust.contains("pub fn create_config() -> std::collections::HashMap<String, serde_json::Value>"));
    assert!(!rust.contains("-> i32"));
}

#[test]
fn test_depyler_0448_int_return_infers_i32() {
    let python = r#"
def get_count():
    return 42
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer i32 return type
    assert!(rust.contains("pub fn get_count() -> i32"));
}

#[test]
fn test_depyler_0448_string_return_infers_string() {
    let python = r#"
def get_name():
    return "Alice"
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer String return type
    assert!(rust.contains("pub fn get_name() -> String"));
}

#[test]
fn test_depyler_0448_mixed_returns_use_value() {
    let python = r#"
def get_mixed(flag):
    if flag:
        return {"data": 123}
    else:
        return "error"
"#;
    let rust = transpile_python(python).unwrap();

    // Mixed types should use Value
    assert!(rust.contains("pub fn get_mixed") && rust.contains("-> serde_json::Value"));
    assert!(!rust.contains("-> i32"));
}

#[test]
fn test_depyler_0448_no_return_infers_unit() {
    let python = r#"
def do_something():
    print("hello")
"#;
    let rust = transpile_python(python).unwrap();

    // No explicit return should be unit type or implicit
    assert!(!rust.contains("-> i32"));
}
```

#### Test Suite 2: Constant Type Inference

```rust
#[test]
fn test_depyler_0448_dict_constant_infers_hashmap() {
    let python = r#"
DEFAULT_CONFIG = {"host": "localhost", "port": 5432}
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer HashMap type for constant
    assert!(rust.contains("pub const DEFAULT_CONFIG: std::collections::HashMap<String, serde_json::Value>"));
    assert!(!rust.contains("DEFAULT_CONFIG: i32"));
}

#[test]
fn test_depyler_0448_int_constant_infers_i32() {
    let python = r#"
MAX_RETRIES = 3
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer i32 type for constant
    assert!(rust.contains("pub const MAX_RETRIES: i32"));
}
```

#### Test Suite 3: Integration Tests (Real Examples)

```rust
#[test]
fn test_depyler_0448_config_manager_compiles() {
    let config_manager = include_str!("../../../examples/reprorusted-python-cli/examples/example_config/config_manager.py");
    let rust = transpile_python(config_manager).unwrap();

    // load_config should return HashMap or Value, not i32
    assert!(!rust.contains("pub fn load_config(path: String) -> i32"));

    // DEFAULT_CONFIG should be HashMap, not i32
    assert!(!rust.contains("pub const DEFAULT_CONFIG: i32"));

    // Should compile
    let temp_dir = tempfile::tempdir().unwrap();
    let rust_file = temp_dir.path().join("config_manager.rs");
    std::fs::write(&rust_file, rust).unwrap();

    let output = Command::new("rustc")
        .arg("--crate-type").arg("bin")
        .arg(&rust_file)
        .arg("--deny").arg("warnings")
        .output()
        .unwrap();

    assert!(output.status.success(), "Compilation failed: {}", String::from_utf8_lossy(&output.stderr));
}
```

#### Test Suite 4: Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_depyler_0448_never_defaults_to_i32_for_dicts(
        num_keys in 1..10usize,
    ) {
        let keys: Vec<String> = (0..num_keys).map(|i| format!("key{}", i)).collect();
        let dict_literal = format!("{{{}}}",
            keys.iter().map(|k| format!("\"{}\": 123", k)).collect::<Vec<_>>().join(", ")
        );
        let python = format!("def f(): return {}", dict_literal);

        let rust = transpile_python(&python).unwrap();

        // Dict return should NEVER be i32
        prop_assert!(!rust.contains("-> i32"), "Dict return typed as i32: {}", rust);
    }
}
```

### Quality Gates

**MANDATORY Before Commit**:
```bash
# 1. All tests pass
cargo test depyler_0448

# 2. No regressions
cargo test --workspace

# 3. Complexity ≤10
pmat analyze complexity --file crates/depyler-core/src/rust_gen/func_gen.rs --max-cyclomatic 10

# 4. TDG ≤2.0
pmat analyze tdg --path crates --threshold 2.0 --critical-only

# 5. Coverage ≥80%
cargo llvm-cov --all-features --workspace --fail-under-lines 80

# 6. Clippy clean
cargo clippy --all-targets --all-features -- -D warnings

# 7. reprorusted-cli examples improve
/tmp/test_reprorusted_main.sh  # Should show improvement from 4/13
```

---

## Impact Analysis

### Before Fix

**Current State**:
- 4/13 examples passing (30.8%)
- 94 E0308 errors across 9 failing examples
- Functions returning dicts typed as `-> i32`
- Constants typed as `const X: i32 = { map }`

### After Fix

**Expected State**:
- 8-10/13 examples passing (60-77%)
- E0308 errors reduced from 94 → ~20-30 (70% reduction)
- Proper type inference for returns and constants
- May expose secondary bugs (dict methods, etc.)

### Risk Assessment

**Low Risk**:
- Type inference is isolated to return type generation
- Tests will catch any regressions
- Fallback to `Value` is always safe

**Medium Risk**:
- May reveal downstream bugs in dict codegen
- May require updating existing tests with hardcoded `i32` expectations

**Mitigation**:
- Comprehensive test coverage before changes
- Gradual rollout (fix returns first, then constants)
- Keep `i32` tests as regression suite

---

## Timeline Estimate

**EXTREME TDD Protocol** (RED → GREEN → REFACTOR):

| Phase | Estimated Time | Cumulative |
|-------|----------------|------------|
| **RED Phase**: Write failing tests | 1-2 hours | 1-2 hours |
| **GREEN Phase**: Minimal implementation | 2-3 hours | 3-5 hours |
| **REFACTOR Phase**: Meet quality gates | 1-2 hours | 4-7 hours |
| **Validation**: reprorusted-cli re-test | 30 min | 4.5-7.5 hours |

**Total**: 4.5 - 7.5 hours of focused work

---

## References

### Related Tickets
- DEPYLER-0435: reprorusted-cli 100% compilation goal (parent ticket)
- DEPYLER-0447: Validator parameter type inference (similar type inference issue)

### Error Logs
- `/tmp/error_analysis.txt` - Initial error categorization
- `/tmp/analyze_all_failures.sh` - Comprehensive error analysis script

### Example Files
- `/home/noah/src/reprorusted-python-cli/examples/example_config/config_manager.py` - Primary reproduction case
- `/tmp/config_manager.rs` - Transpiled output showing bug

### Code Locations
- `crates/depyler-core/src/rust_gen/func_gen.rs:150-200` - Return type generation
- `crates/depyler-core/src/rust_gen.rs` - Constant generation
- `crates/depyler-core/src/hir.rs` - HirFunction properties

---

## Appendix: Full Error Examples

### A. config_manager.py Errors

```
error[E0308]: mismatched types
  --> src/main.rs:15:9
   |
15 |         map
   |         ^^^ expected `Value`, found `HashMap<String, String>`

error[E0308]: mismatched types
  --> src/main.rs:21:9
   |
21 |         map
   |         ^^^ expected `Value`, found `HashMap<String, bool>`

error[E0308]: mismatched types
  --> src/main.rs:23:5
   |
23 |     map
   |     ^^^ expected `i32`, found `HashMap<String, Value>`

error[E0308]: mismatched types
  --> src/main.rs:75:16
   |
72 | pub fn load_config(path: String) -> i32 {
   |                                     --- expected `i32` because of return type
...
75 |         return serde_json::from_reader::<_, serde_json::Value>(f).unwrap();
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `Value`

error[E0308]: mismatched types
   --> src/main.rs:103:8
    |
103 |     Ok(value)
    |     -- ^^^^^ expected `()`, found `&Value`
    |     |
    |     arguments to this enum variant are incorrect
```

### B. stdlib_integration.py Errors

```
error[E0308]: mismatched types
   --> src/main.rs:76:79
    |
 76 |     let _cse_temp_0 = !vec!["md5".to_string(), "sha256".to_string()].contains(&algorithm);
    |                                                                      -------- ^^^^^^^^^^ expected `&String`, found `&&Value`

error[E0308]: mismatched types
   --> src/main.rs:169:46
    |
169 |             format_timestamp(stats.st_mtime, time_format),
    |              ----------------                 ^^^^^^^^^^^ expected `&Value`, found `Value`

error[E0308]: mismatched types
   --> src/main.rs:198:8
    |
198 |     if info.get("extension").cloned().unwrap_or_default() {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `Value`

error[E0308]: mismatched types
   --> src/main.rs:213:23
    |
213 |     let _cse_temp_1 = (include_hash) && (_cse_temp_0);
    |                        ^^^^^^^^^^^^^ expected `bool`, found `&Value`

error[E0308]: mismatched types
   --> src/main.rs:230:5
    |
229 | pub fn format_output_json(info: &serde_json::Value) -> i32 {
    |                                                        --- expected `i32` because of return type
230 |     serde_json::to_string_pretty(&info).unwrap()
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `String`
```

### C. complex_cli.py Errors

```
error[E0308]: mismatched types
   --> src/main.rs:130:8
    |
130 |     Ok(value)
    |     -- ^^^^^ expected `i32`, found `&str`

error[E0308]: mismatched types
   --> src/main.rs:148:33
    |
136 |     let mut output_format;
    |         ----------------- expected due to the type of this binding
...
148 |         output_format = args.format.as_str();
    |                                 ^^^^^^^^^^^ expected `String`, found `&str`

error[E0308]: mismatched types
   --> src/main.rs:166:8
    |
166 |     if args.encoding {
    |        ^^^^^^^^^^^^^ expected `bool`, found `String`

error[E0308]: mismatched types
   --> src/main.rs:193:8
    |
193 |     if config_file {
    |        ^^^^^^^^^^^ expected `bool`, found `Option<String>`
```

---

**Document Version**: 1.0
**Last Updated**: 2025-11-21
**Word Count**: ~3,500 words
**STOP THE LINE Protocol**: ✅ COMPLIANT
