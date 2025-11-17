# DEPYLER-0380: String Literal Conversion and os Module Transpilation Bugs

**Status**: 🔴 CRITICAL - Blocks compilation of generated Rust code
**Ticket**: DEPYLER-0380
**Discovered**: 2025-11-17
**Reporter**: Claude Code (continuing DEPYLER-0379 fixes)
**Severity**: P0 (STOP ALL WORK)

## Executive Summary

Three critical transpiler bugs prevent generated Rust code from compiling:

1. **String Literal Bug**: String literals assigned to `String` typed variables don't get `.to_string()` conversion
2. **os.getenv() Bug**: `os.getenv(var, default)` transpiles to `std::env::var()` with incorrect signature and no default handling
3. **os.environ Membership Bug**: `var in os.environ` transpiles to invalid `std::env::vars.contains_key()` call

All bugs cause **100% compilation failure** for affected code patterns.

## Problem Statement

### Bug #1: String Literal to String Conversion

**What happens**: When a string literal is assigned to a variable with type annotation `str`, the transpiler doesn't add `.to_string()` conversion.

**Example**:
```python
# Python source
def test_sys_version_info() -> str:
    version: str = "Python 3.x"
    return version
```

**Generated (broken) Rust**:
```rust
pub fn test_sys_version_info() -> Result<String, Box<dyn std::error::Error>> {
    let version: String = "Python 3.x";  // ❌ ERROR: expected `String`, found `&str`
    Ok(version)
}
```

**Error**:
```
error[E0308]: mismatched types
  --> examples/test_os_sys_module.rs:34:27
   |
34 |     let version: String = "Python 3.x";
   |                  ------   ^^^^^^^^^^^^ expected `String`, found `&str`
```

### Bug #2: os.getenv() with Default Value

**What happens**: Python's `os.getenv(var, default)` transpiles to `std::env::var(var, default)` which is invalid - Rust's `std::env::var()` takes one argument and returns `Result<String, VarError>`.

**Example**:
```python
# Python source
def test_env_variable_access() -> str:
    home: str = os.getenv("HOME", "/home/user")
    return home
```

**Generated (broken) Rust**:
```rust
pub fn test_env_variable_access() -> Result<String, Box<dyn std::error::Error>> {
    let home: String = std::env::var("HOME".to_string(), "/home/user");
    //                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //                 ERROR: this function takes 1 argument but 2 arguments were supplied
    //                 ERROR: expected `String`, found `Result<String, VarError>`
    Ok(home)
}
```

**Errors**:
```
error[E0061]: this function takes 1 argument but 2 arguments were supplied
   --> examples/test_os_sys_module.rs:59:24
    |
 59 |     let home: String = std::env::var("HOME".to_string(), "/home/user");
    |                        ^^^^^^^^^^^^^                     ------------ unexpected argument #2

error[E0308]: mismatched types
   --> examples/test_os_sys_module.rs:59:24
    |
 59 |     let home: String = std::env::var("HOME".to_string(), "/home/user");
    |               ------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `String`, found `Result<String, VarError>`
```

### Bug #3: os.environ Membership Test

**What happens**: Python's `var in os.environ` transpiles to `std::env::vars.contains_key(&var)`, but `vars` is a function, not a HashMap.

**Example**:
```python
# Python source
def test_env_variable_exists() -> bool:
    var_name: str = "PATH"
    exists: bool = var_name in os.environ
    return exists
```

**Generated (broken) Rust**:
```rust
pub fn test_env_variable_exists() -> Result<bool, Box<dyn std::error::Error>> {
    let var_name: String = "PATH";  // ❌ First error: string literal
    let _cse_temp_0 = std::env::vars.contains_key(&var_name);
    //                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //                ❌ ERROR: `vars` is a function, not a HashMap
    let exists: bool = _cse_temp_0;
    Ok(exists)
}
```

**Error**:
```
error[E0599]: no method named `contains_key` found for fn item `fn() -> Vars {vars}` in the current scope
  --> examples/test_os_sys_module.rs:67:38
   |
67 |     let _cse_temp_0 = std::env::vars.contains_key(&var_name);
   |                                      ^^^^^^^^^^^^ method not found in `fn() -> Vars {vars}`
```

## Root Cause Analysis

### Bug #1: String Literal Conversion

**Location**: Likely in expression code generation (`crates/depyler-core/src/rust_gen/expr_gen.rs`) or type coercion logic

**Root cause**: When generating code for a literal expression assigned to a typed variable, the transpiler doesn't check if type coercion is needed (e.g., `&str` → `String`).

**Why it's wrong**: Rust requires explicit conversion from string literals (`&str`) to owned `String` type via `.to_string()` or `String::from()`.

**Correct approach**:
```rust
// Check target type during assignment
if target_type == RustType::String && expr_type == RustType::StrLiteral {
    code = quote! { #expr.to_string() };
}
```

### Bug #2: os.getenv() Transpilation

**Location**: Stdlib function mapping, likely in `crates/depyler-core/src/stdlib/os.rs` or function call codegen

**Root cause**: The transpiler incorrectly maps `os.getenv(var, default)` directly to `std::env::var(var, default)` without handling:
1. Rust's `std::env::var()` only takes one argument
2. It returns `Result<String, VarError>` that needs unwrapping
3. Default value must be handled via `.unwrap_or_else()` or `.unwrap_or()`

**Why it's wrong**: Python's `os.getenv()` returns the default value if the variable doesn't exist, while Rust's `std::env::var()` returns a Result.

**Correct approach**:
```rust
// os.getenv("VAR", "default") should transpile to:
std::env::var("VAR").unwrap_or_else(|_| "default".to_string())
```

### Bug #3: os.environ Membership Test

**Location**: `in` operator codegen for `os.environ`, likely in operator handling or stdlib mapping

**Root cause**: The transpiler maps `var in os.environ` to `std::env::vars.contains_key(&var)`, but:
1. `std::env::vars` is a function that returns an iterator, not a HashMap
2. Should use `std::env::var(&var).is_ok()` to check existence

**Why it's wrong**: Rust doesn't have a global `environ` dictionary - environment variables are accessed via functions.

**Correct approach**:
```rust
// var in os.environ should transpile to:
std::env::var(&var).is_ok()
// or
std::env::var(var).is_ok()  // if var is already a &str
```

## Impact Assessment

**Affected Files**:
- `examples/test_os_sys_module.py` - Multiple functions affected
- Any Python code using:
  - String literals assigned to `str` typed variables
  - `os.getenv()` with default values
  - `var in os.environ` membership tests

**Compilation Status**:
- ❌ 0% success rate for affected patterns
- Blocks all os module examples from compiling
- Combined with DEPYLER-0379 (now fixed), blocks complete example file

**User Impact**:
- **CRITICAL**: Cannot transpile common Python stdlib patterns
- **BLOCKS**: Standard library os module usage
- **DEGRADES**: User confidence in transpiler reliability

## Test Cases

### Test Case 1: String Literal Assignment

**Input** (`test_string_literal_assignment.py`):
```python
def test_string_literal() -> str:
    message: str = "Hello, World!"
    return message
```

**Expected Rust**:
```rust
pub fn test_string_literal() -> Result<String, Box<dyn std::error::Error>> {
    let message: String = "Hello, World!".to_string();
    Ok(message)
}
```

**Current (broken) Rust**:
```rust
pub fn test_string_literal() -> Result<String, Box<dyn std::error::Error>> {
    let message: String = "Hello, World!";  // ❌ ERROR: expected `String`, found `&str`
    Ok(message)
}
```

### Test Case 2: os.getenv() with Default

**Input** (`test_env_getenv.py`):
```python
import os

def get_config_dir() -> str:
    config: str = os.getenv("XDG_CONFIG_HOME", "~/.config")
    return config
```

**Expected Rust**:
```rust
pub fn get_config_dir() -> Result<String, Box<dyn std::error::Error>> {
    let config: String = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| "~/.config".to_string());
    Ok(config)
}
```

**Current (broken) Rust**:
```rust
pub fn get_config_dir() -> Result<String, Box<dyn std::error::Error>> {
    let config: String = std::env::var("XDG_CONFIG_HOME".to_string(), "~/.config");
    // ❌ ERROR: takes 1 argument but 2 supplied
    // ❌ ERROR: expected `String`, found `Result<String, VarError>`
    Ok(config)
}
```

### Test Case 3: Environment Variable Existence Check

**Input** (`test_env_exists.py`):
```python
import os

def has_display() -> bool:
    return "DISPLAY" in os.environ
```

**Expected Rust**:
```rust
pub fn has_display() -> Result<bool, Box<dyn std::error::Error>> {
    Ok(std::env::var("DISPLAY").is_ok())
}
```

**Current (broken) Rust**:
```rust
pub fn has_display() -> Result<bool, Box<dyn std::error::Error>> {
    let _cse_temp_0 = std::env::vars.contains_key("DISPLAY");
    // ❌ ERROR: no method `contains_key` for function
    Ok(_cse_temp_0)
}
```

## Solution Design

### Bug #1: String Literal Type Coercion

**Approach**: Add type coercion logic during assignment/expression generation

**Algorithm**:
1. **Detect pattern**: String literal assigned to `String` typed variable
2. **Check types**: Compare inferred expression type with target type
3. **Insert coercion**: If `&str` → `String`, wrap with `.to_string()`

**Implementation** (pseudocode):
```rust
// In codegen for assignment or initialization
fn generate_assignment(&mut self, target_type: &RustType, expr: &HirExpr) -> TokenStream {
    let mut expr_code = self.codegen_expr(expr);
    let expr_type = self.infer_type(expr);

    // Apply type coercion if needed
    if needs_to_string_conversion(&expr_type, target_type) {
        expr_code = quote! { #expr_code.to_string() };
    }

    expr_code
}

fn needs_to_string_conversion(from: &RustType, to: &RustType) -> bool {
    matches!(
        (from, to),
        (RustType::Str | RustType::StrLiteral, RustType::String)
    )
}
```

### Bug #2: os.getenv() Function Mapping

**Approach**: Map `os.getenv(var, default)` to proper Rust idiom with Result handling

**Strategy**:
1. **Detect pattern**: `os.getenv(key)` or `os.getenv(key, default)`
2. **Single argument**: `std::env::var(key)?` (propagate error)
3. **With default**: `std::env::var(key).unwrap_or_else(|_| default.to_string())`

**Implementation** (pseudocode):
```rust
// In stdlib/os.rs or function call codegen
fn transpile_os_getenv(&mut self, args: &[HirExpr]) -> TokenStream {
    match args.len() {
        1 => {
            // os.getenv("VAR") -> std::env::var("VAR")?
            let key = self.codegen_expr(&args[0]);
            quote! { std::env::var(#key)? }
        }
        2 => {
            // os.getenv("VAR", "default") -> std::env::var("VAR").unwrap_or_else(|_| "default".to_string())
            let key = self.codegen_expr(&args[0]);
            let default = self.codegen_expr(&args[1]);
            quote! {
                std::env::var(#key).unwrap_or_else(|_| #default.to_string())
            }
        }
        _ => panic!("os.getenv takes 1 or 2 arguments"),
    }
}
```

### Bug #3: os.environ Membership Test

**Approach**: Map `var in os.environ` to `std::env::var(var).is_ok()`

**Strategy**:
1. **Detect pattern**: `in` operator with `os.environ` as right operand
2. **Transpile to**: `std::env::var(key).is_ok()`

**Implementation** (pseudocode):
```rust
// In binary operator codegen for `in`
fn codegen_in_operator(&mut self, left: &HirExpr, right: &HirExpr) -> TokenStream {
    // Check if right side is os.environ
    if is_os_environ_access(right) {
        let key = self.codegen_expr(left);
        return quote! { std::env::var(#key).is_ok() };
    }

    // ... other `in` operator handling
}

fn is_os_environ_access(expr: &HirExpr) -> bool {
    matches!(
        expr,
        HirExpr::Attribute {
            object: box HirExpr::Symbol(sym),
            attr,
            ..
        } if sym == "os" && attr == "environ"
    )
}
```

## Implementation Plan

### Phase 1: Bug #1 - String Literal Coercion Fix

**Files to modify**:
1. `crates/depyler-core/src/rust_gen/expr_gen.rs` - Expression code generation
2. `crates/depyler-core/src/rust_gen/stmt_gen.rs` - Assignment handling

**Steps**:
1. Add type coercion detection for `&str` → `String`
2. Wrap string literals with `.to_string()` when assigned to `String` type
3. Add test cases for various string literal patterns
4. Ensure no regressions in existing string handling

**Complexity**: ≤10 per function (meets quality standard)

### Phase 2: Bug #2 - os.getenv() Mapping Fix

**Files to modify**:
1. `crates/depyler-core/src/stdlib/os.rs` - OS module stdlib mappings (or create if not exists)
2. `crates/depyler-core/src/rust_gen/expr_gen.rs` - Function call handling

**Steps**:
1. Add stdlib mapping for `os.getenv()`
2. Handle single argument case: `std::env::var(key)?`
3. Handle default argument case: `.unwrap_or_else(|_| default.to_string())`
4. Test both variations

**Complexity**: ≤10 per function

### Phase 3: Bug #3 - os.environ Membership Test Fix

**Files to modify**:
1. `crates/depyler-core/src/rust_gen/expr_gen.rs` - Binary operator handling for `in`

**Steps**:
1. Detect `var in os.environ` pattern
2. Map to `std::env::var(var).is_ok()`
3. Add test cases for membership tests
4. Verify correctness with actual environment variable checks

**Complexity**: ≤10 per function

### Phase 4: Comprehensive Testing

**Test coverage**:
1. Unit tests for string literal coercion
2. Unit tests for os.getenv() transpilation
3. Unit tests for os.environ membership tests
4. Integration test: Re-transpile `test_os_sys_module.py`
5. Verify generated code compiles with `rustc --deny warnings`
6. Regression tests for existing functionality

**Success criteria**:
- ✅ All test cases compile successfully
- ✅ Generated Rust passes `rustc --deny warnings`
- ✅ No regressions in existing tests (497 tests still passing)
- ✅ Code coverage ≥80%

## Verification Plan

### Step 1: Create failing tests
```bash
# Add test cases to test suite
cargo test test_DEPYLER_0380 --lib
# Should fail initially
```

### Step 2: Implement fixes
```bash
# Fix transpiler code
# Run tests continuously
cargo watch -x 'test test_DEPYLER_0380'
```

### Step 3: Re-transpile affected examples
```bash
# Re-generate examples
depyler transpile examples/test_os_sys_module.py

# Verify compilation
rustc --crate-type lib --deny warnings examples/test_os_sys_module.rs
```

### Step 4: Comprehensive validation
```bash
# Run full test suite
cargo test --workspace --all-features

# Check coverage
cargo llvm-cov --all-features --workspace --fail-under-lines 80

# Quality gates
pmat analyze tdg --path crates --threshold 2.0 --critical-only
cargo clippy --all-targets -- -D warnings
```

## Rollback Plan

If fixes introduce regressions:
1. Revert commits for this ticket
2. Re-enable affected tests with `#[ignore]` attribute
3. Document known limitations in README
4. Schedule proper fix for next sprint

## References

- **Related bugs**:
  - DEPYLER-0379 (variable scope and tuple type) - Fixed in same session
- **Documentation**:
  - `docs/processes/stop-the-line.md` - Process followed
  - Python os module docs: https://docs.python.org/3/library/os.html
  - Rust std::env docs: https://doc.rust-lang.org/std/env/

## Lessons Learned

### What went well:
- ✅ DEPYLER-0379 fixes enabled discovery of these additional bugs
- ✅ Systematic compilation checking revealed all issues
- ✅ Clear error messages from rustc

### What needs improvement:
- ❌ Should have had test coverage for stdlib os module patterns
- ❌ String literal type coercion should have been in initial implementation
- ❌ Need comprehensive stdlib function mapping tests

### Process improvements:
1. Add os/sys module patterns to systematic test matrix
2. Create comprehensive stdlib function mapping test suite
3. Add property tests for type coercion
4. Ensure all Python stdlib modules have transpilation tests before marking as "supported"

## Timeline

- **2025-11-17 15:00**: Bugs discovered during DEPYLER-0379 verification
- **2025-11-17 15:15**: STOP THE LINE initiated
- **2025-11-17 15:30**: Bug document created (DEPYLER-0380)
- **2025-11-17 15:45**: Fix implementation begins
- **2025-11-17 TBD**: Testing and verification
- **2025-11-17 TBD**: Fixes committed and pushed

---

**Remember**: Fix the transpiler, not the generated code. Never bypass this protocol.
