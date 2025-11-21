# DEPYLER-0447: Argparse Validator Return Type Inference Bug

## Status: IN PROGRESS
- **Created**: 2025-11-21
- **Priority**: P0 (CRITICAL - STOP THE LINE)
- **Type**: Bug/Transpiler
- **Parent**: DEPYLER-0435 (reprorusted-python-cli 100% compilation)
- **Effort**: 2-3 hours (estimated)
- **Blocks**: complex_cli (3/7 errors)

---

## Problem Statement

**Issue**: Argparse validator functions that return the original parameter (e.g., `email_address`) are incorrectly transpiled with wrong return type and parameter type.

**Impact**: complex_cli fails compilation with 3 errors related to `email_address` validator:
1. E0308: Parameter type mismatch (`serde_json::Value` instead of `&str`)
2. E0308: Return type mismatch (`i32` instead of `String`)
3. E0308: Error type mismatch (bare `String` instead of `ArgumentTypeError`)

**Observed in**: reprorusted-python-cli/examples/example_complex/complex_cli.py

---

## Root Cause Analysis

### Python Source Code

```python
def email_address(value):
    """Custom type for email address validation."""
    pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    if not re.match(pattern, value):
        raise argparse.ArgumentTypeError(f"Invalid email address: '{value}'")
    return value  # ← Returns original string parameter
```

### Current Transpiled Output (WRONG)

```rust
pub fn email_address(value: serde_json::Value) -> Result<i32, ArgumentTypeError> {
    //                       ^^^^^^^^^^^^^^^^^^^            ^^^
    //                       ERROR 1: Wrong param type      ERROR 2: Wrong return type
    let pattern = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$";
    if regex::Regex::new(pattern).unwrap().find(value).is_none() {
        return Err(format!("Invalid email address: '{}'", value));
        //         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //         ERROR 3: Returns String, not ArgumentTypeError
    }
    Ok(value)
}
```

**Compilation Errors**:
```
error[E0308]: mismatched types
   --> complex_cli.rs:125:49
    |
125 |     if regex::Regex::new(pattern).unwrap().find(value).is_none() {
    |                                            ---- ^^^^^ expected `&str`, found `Value`

error[E0308]: mismatched types
   --> complex_cli.rs:126:20
    |
126 |         return Err(format!("Invalid email address: '{}'", value));
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |                    expected `ArgumentTypeError`, found `String`

error[E0308]: mismatched types
   --> complex_cli.rs:128:8
    |
128 |     Ok(value)
    |        ^^^^^ expected `i32`, found `Value`
```

### Expected Transpiled Output (CORRECT)

```rust
pub fn email_address(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    //                       ^^^^               ^^^^^^
    //                       FIX 1: &str param  FIX 2: String return
    let pattern = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$";
    if regex::Regex::new(pattern).unwrap().find(value).is_none() {
        return Err(Box::new(ArgumentTypeError::new(
            format!("Invalid email address: '{}'", value)
        )));
        // FIX 3: Wrap in Box<dyn Error>
    }
    Ok(value.to_string())
    // FIX 4: Convert &str to String
}
```

---

## Comparison with Working Validators

### port_number (✅ WORKS)

**Python**:
```python
def port_number(value):
    port = int(value)  # ← Converts to int
    if port < 1 or port > 65535:
        raise argparse.ArgumentTypeError(...)
    return port  # ← Returns int
```

**Rust** (✅ Correct):
```rust
pub fn port_number(value: &str) -> Result<i32, Box<dyn std::error::Error>> {
    //                       ^^^^            ^^^
    match value.parse::<i32>() {
        Ok(port) => { /* validation */ Ok(port) }
        Err(_) => Err(Box::new(ArgumentTypeError::new(...)))
    }
}
```

### email_address (❌ BROKEN)

**Python**:
```python
def email_address(value):
    if not re.match(pattern, value):
        raise argparse.ArgumentTypeError(...)
    return value  # ← Returns original string (NO conversion)
```

**Rust** (❌ Wrong):
```rust
pub fn email_address(value: serde_json::Value) -> Result<i32, ArgumentTypeError> {
    // WHY i32?? Should be String!
    // WHY serde_json::Value?? Should be &str!
    // WHY bare ArgumentTypeError?? Should be Box<dyn Error>!
}
```

---

## Bug Analysis

### Issue 1: Return Type Inference Fails for Identity Functions

**Current behavior**: Transpiler infers return type as `i32` (default?)

**Why this happens**: The return type inference logic doesn't track that `return value` returns the SAME value as the input parameter when:
1. The parameter is NOT converted (no `int()`, `float()`, etc.)
2. The function performs validation but returns the original value

**Fix needed**: When a validator function:
- Has pattern `return <param_name>` (no conversion)
- Is used in `add_argument(..., type=validator_func)`
- Then infer return type as `String` (since argparse passes strings)

### Issue 2: Parameter Type Still Wrong Despite DEPYLER-0436

**Expected**: DEPYLER-0436 (commit a50cce9) should have fixed this

**Actual**: `email_address` still gets `serde_json::Value` parameter

**Hypothesis**: The fix in DEPYLER-0436 only applies to validators that:
- Call `int(value)` or similar conversion functions
- But NOT to validators that just validate and return the original value

**Evidence**: Compare transpiled output:
- `port_number`: `value: &str` ✅ (calls `int(value)`)
- `positive_int`: `value: &str` ✅ (calls `int(value)`)
- `email_address`: `value: serde_json::Value` ❌ (no conversion call)

### Issue 3: Error Wrapping Incomplete

**Current**: `return Err(format!(...))` generates bare `String`

**Expected**: DEPYLER-0438 should wrap in `Box::new(ArgumentTypeError::new(...))`

**Hypothesis**: The error wrapping logic triggers for some patterns but not others

---

## Files Affected

### Transpiler Source Files

1. **`crates/depyler-core/src/type_hints.rs`**
   - Function: `infer_return_type()` or similar
   - Issue: Doesn't handle identity return pattern (`return param_name`)
   - Fix: Detect when return value == parameter, infer as String for validators

2. **`crates/depyler-core/src/rust_gen/func_gen.rs`**
   - Function: Validator function signature generation
   - Issue: Parameter type detection incomplete
   - Fix: ALL argparse type validators should get `value: &str` parameter

3. **`crates/depyler-core/src/rust_gen/stmt_gen.rs`**
   - Function: Return statement generation
   - Issue: Doesn't convert `&str` to `String` when needed
   - Fix: Add `.to_string()` when returning `&str` as `Result<String, ...>`

4. **`crates/depyler-core/src/rust_gen/expr_gen.rs`**
   - Function: Error wrapping for `raise ArgumentTypeError`
   - Issue: Incomplete wrapping in some cases
   - Fix: Ensure ALL `raise ArgumentTypeError` wraps in Box + constructor

### Generated Files (affected examples)

- `/home/noah/src/reprorusted-python-cli/examples/example_complex/complex_cli.rs`

---

## Test Plan

### Test 1: Identity Validator Returns String

```python
def email_address(value):
    if not re.match(r"^[a-z]+@[a-z]+\.com$", value):
        raise argparse.ArgumentTypeError("Invalid email")
    return value  # Identity return
```

**Expected Rust**:
```rust
pub fn email_address(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    if !regex::Regex::new("^[a-z]+@[a-z]+\\.com$").unwrap().is_match(value) {
        return Err(Box::new(ArgumentTypeError::new("Invalid email".to_string())));
    }
    Ok(value.to_string())
}
```

### Test 2: Converting Validator Returns Converted Type

```python
def port_number(value):
    port = int(value)
    if port < 1:
        raise argparse.ArgumentTypeError("Bad port")
    return port  # Returns int
```

**Expected Rust** (already works):
```rust
pub fn port_number(value: &str) -> Result<i32, Box<dyn std::error::Error>> {
    match value.parse::<i32>() {
        Ok(port) => {
            if port < 1 {
                return Err(Box::new(ArgumentTypeError::new("Bad port".to_string())));
            }
            Ok(port)
        }
        Err(_) => Err(Box::new(ArgumentTypeError::new("Parse error".to_string())))
    }
}
```

### Test 3: Uppercase Validator Transforms String

```python
def uppercase_string(value):
    return value.upper()  # String method return
```

**Expected Rust**:
```rust
pub fn uppercase_string(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(value.to_uppercase())
}
```

---

## Implementation Plan

### Phase 1: RED (Failing Tests)

**File**: `crates/depyler-core/tests/depyler_0447_validator_return_type.rs`

```rust
#[test]
fn test_identity_validator_returns_string() {
    let py = r#"
import argparse
import re

def email_address(value):
    if not re.match(r"^[a-z]+@[a-z]+\.com$", value):
        raise argparse.ArgumentTypeError("Invalid email")
    return value

parser = argparse.ArgumentParser()
parser.add_argument("--email", type=email_address)
args = parser.parse_args()
"#;

    let (rust_code, _deps) = depyler_core::transpile(py).unwrap();

    // Should contain correct signature
    assert!(rust_code.contains("pub fn email_address(value: &str) -> Result<String"));
    assert!(rust_code.contains("Ok(value.to_string())"));
}

#[test]
fn test_converting_validator_returns_converted_type() {
    let py = r#"
import argparse

def port_number(value):
    port = int(value)
    if port < 1:
        raise argparse.ArgumentTypeError("Bad port")
    return port

parser = argparse.ArgumentParser()
parser.add_argument("--port", type=port_number)
"#;

    let (rust_code, _deps) = depyler_core::transpile(py).unwrap();

    // Should contain correct signature
    assert!(rust_code.contains("pub fn port_number(value: &str) -> Result<i32"));
}

#[test]
fn test_string_method_validator_returns_string() {
    let py = r#"
import argparse

def uppercase_string(value):
    return value.upper()

parser = argparse.ArgumentParser()
parser.add_argument("--name", type=uppercase_string)
"#;

    let (rust_code, _deps) = depyler_core::transpile(py).unwrap();

    assert!(rust_code.contains("pub fn uppercase_string(value: &str) -> Result<String"));
    assert!(rust_code.contains("Ok(value.to_uppercase())"));
}
```

**Run tests** (should FAIL):
```bash
cargo test depyler_0447 --lib
```

### Phase 2: GREEN (Minimal Fix)

#### Fix 1: Extend Parameter Type Detection

**File**: `crates/depyler-core/src/type_hints.rs` or `func_gen.rs`

**Current logic** (from DEPYLER-0436):
```rust
// Detects validators that call int(value), float(value), etc.
if function_calls_conversion(func_body) {
    param_type = "&str"
}
```

**Extended logic**:
```rust
// ALL argparse type validators get &str parameter
if is_argparse_type_validator(func_name, context) {
    param_type = "&str"  // Not just converters, ALL validators
}
```

#### Fix 2: Improve Return Type Inference

**File**: `crates/depyler-core/src/type_hints.rs`

**Add identity return detection**:
```rust
fn infer_validator_return_type(func: &HirFunction, context: &Context) -> Type {
    // Case 1: Function calls int(param) → returns i32
    if has_int_conversion(func.body) {
        return Type::Int
    }

    // Case 2: Function calls float(param) → returns f64
    if has_float_conversion(func.body) {
        return Type::Float
    }

    // Case 3: Function has "return param_name" (identity) → returns String
    if has_identity_return(func.body, func.params[0].name) {
        return Type::String  // NEW: Identity validators return String
    }

    // Case 4: Function calls string method → returns String
    if has_string_method_call(func.body) {
        return Type::String
    }

    // Default: String (argparse validators typically return String)
    Type::String
}

fn has_identity_return(body: &[HirStmt], param_name: &str) -> bool {
    // Check if any return statement returns the parameter unchanged
    for stmt in body {
        if let HirStmt::Return(Some(HirExpr::Var(name))) = stmt {
            if name == param_name {
                return true;
            }
        }
    }
    false
}
```

#### Fix 3: Add .to_string() for &str → String Conversion

**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs`

**Current**:
```rust
fn convert_return(expr: &HirExpr) -> TokenStream {
    quote! { Ok(#expr) }
}
```

**Fixed**:
```rust
fn convert_return(expr: &HirExpr, expected_return_type: &Type) -> TokenStream {
    // If returning &str but function signature is Result<String, ...>
    if expr_is_str_ref(expr) && expected_return_type == &Type::String {
        quote! { Ok(#expr.to_string()) }
    } else {
        quote! { Ok(#expr) }
    }
}
```

### Phase 3: REFACTOR (Quality Gates)

#### Quality Checks

1. **Cyclomatic Complexity**: ≤10
   ```bash
   pmat analyze complexity --file crates/depyler-core/src/type_hints.rs --max-cyclomatic 10
   ```

2. **TDG Score**: ≤2.0
   ```bash
   pmat analyze tdg --path crates/depyler-core/src/type_hints.rs --threshold 2.0
   ```

3. **Test Coverage**: ≥80%
   ```bash
   cargo llvm-cov --lib --fail-under-lines 80
   ```

4. **Clippy Clean**:
   ```bash
   cargo clippy --all-targets -- -D warnings
   ```

#### Refactoring Opportunities

- Extract `is_argparse_type_validator()` helper
- Extract `has_identity_return()` helper
- Add comprehensive comments for return type inference logic

---

## Verification

### Verification 1: Unit Tests Pass

```bash
cargo test depyler_0447 --lib
# Expected: 3/3 tests passing
```

### Verification 2: complex_cli Compiles

```bash
cd /home/noah/src/reprorusted-python-cli/examples/example_complex
depyler transpile complex_cli.py
cargo build --release 2>&1 | grep -c "^error\["
# Expected: 7 → 4 errors (3 fixed)
```

### Verification 3: No Regressions

```bash
cargo test --workspace
# Expected: All tests pass
```

### Verification 4: Other Validators Still Work

```bash
# Verify port_number and positive_int still compile correctly
grep "pub fn port_number" complex_cli.rs
grep "pub fn positive_int" complex_cli.rs
# Expected: Both have (value: &str) -> Result<i32, ...>
```

---

## Success Criteria

- [ ] All 3 unit tests passing
- [ ] complex_cli errors reduced from 7 → 4 (-3 errors)
- [ ] email_address signature: `(value: &str) -> Result<String, Box<dyn std::error::Error>>`
- [ ] No regressions in existing validators (port_number, positive_int)
- [ ] All quality gates passing (TDG ≤2.0, complexity ≤10, coverage ≥80%)
- [ ] Zero clippy warnings

---

## Related Tickets

- **Parent**: DEPYLER-0435 (reprorusted-python-cli 100% compilation master ticket)
- **Related**: DEPYLER-0436 (argparse validator parameter type inference - COMPLETE)
- **Related**: DEPYLER-0438 (error type generation - COMPLETE)
- **Follows**: DEPYLER-0446 (Option<T> handling - COMPLETE)

---

## Git Commits

**RED Phase**:
```bash
git add crates/depyler-core/tests/depyler_0447_validator_return_type.rs
git commit -m "[RED] DEPYLER-0447: Add failing tests for validator return type inference"
```

**GREEN Phase**:
```bash
git add crates/depyler-core/src/type_hints.rs crates/depyler-core/src/rust_gen/stmt_gen.rs
git commit -m "[GREEN] DEPYLER-0447: Fix identity validator return type inference

- Extend parameter type detection to ALL argparse validators
- Add identity return pattern detection (return param_name)
- Add .to_string() conversion for &str → String returns
- email_address now compiles with correct signature

Fixes 3/7 errors in complex_cli example.

Closes: DEPYLER-0447"
```

**REFACTOR Phase**:
```bash
git commit -m "[REFACTOR] DEPYLER-0447: Meet quality standards

- Extract helper functions for readability
- Add comprehensive comments
- Complexity ≤10, TDG ≤2.0, Coverage ≥80%
- Zero clippy warnings

Quality gates: ✅ ALL PASSING"
```

---

**Last Updated**: 2025-11-21
**Status**: Ready for RED phase implementation
**Next**: Run `cargo test depyler_0447 --lib` (expect failures) → Implement fixes
