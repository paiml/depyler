# DEPYLER-0379: Variable Scope in If/Else Blocks and Tuple Type Mapping Bugs

**Status**: 🔴 CRITICAL - Blocks compilation of generated Rust code
**Ticket**: DEPYLER-0379
**Discovered**: 2025-11-17
**Reporter**: Claude Code (via --trace flag analysis)
**Severity**: P0 (STOP ALL WORK)

## Executive Summary

Two critical transpiler bugs prevent generated Rust code from compiling:

1. **Variable Scope Bug**: Variables defined within if/else blocks are not accessible outside their scope, causing "cannot find value in this scope" errors
2. **Tuple Type Bug**: Python's `tuple` type annotation is literally transpiled as `tuple` instead of proper Rust tuple syntax like `(T1, T2)`

Both bugs cause **100% compilation failure** for affected code patterns.

## Problem Statement

### Bug #1: Variable Scope in If/Else Blocks

**What happens**: When a variable is assigned in both branches of an if/else statement in Python, the transpiler generates separate Rust variables in each block scope, making them inaccessible outside the if/else.

**Example**:
```python
# Python source (test_path_basename function)
if last_slash >= 0:
    basename: str = path[last_slash + 1:]
else:
    basename: str = path

return basename  # Should work - basename is defined in both branches
```

**Generated (broken) Rust**:
```rust
if _cse_temp_0 {
    let mut basename: String = { ... };  // Scoped to if block
} else {
    let mut basename: String = path;     // Scoped to else block
}
Ok(basename)  // ERROR: cannot find value `basename` in this scope
```

**Error**:
```
error[E0425]: cannot find value `basename` in this scope
   --> examples/test_os_sys_module.rs:126:8
    |
126 |     Ok(basename)
    |        ^^^^^^^^
    |
help: the binding `basename` is available in a different scope in the same function
```

### Bug #2: Tuple Type Mapping

**What happens**: Python `tuple` type annotations are transpiled literally as the identifier `tuple` instead of Rust tuple syntax.

**Example**:
```python
def test_path_split() -> tuple:
    # Returns (dirname, basename)
    return (dirname, basename)
```

**Generated (broken) Rust**:
```rust
pub fn test_path_split() -> Result<tuple, IndexError> {
    //                              ^^^^^
    //                              ERROR: cannot find type `tuple` in this scope
```

**Error**:
```
error[E0412]: cannot find type `tuple` in this scope
   --> examples/test_os_sys_module.rs:161:36
    |
161 | pub fn test_path_split() -> Result<tuple, IndexError> {
    |                                    ^^^^^ not found in this scope
```

## Root Cause Analysis

### Bug #1: Variable Scope

**Location**: Likely in the if/else statement code generation in `crates/depyler-core/src/codegen.rs`

**Root cause**: The transpiler generates variable declarations inside each branch block instead of:
1. Declaring the variable before the if statement with a default/placeholder value
2. Assigning to the existing variable within each branch

**Why it's wrong**: In Rust, variables defined in a block are only accessible within that block. Python's semantics allow variables assigned in any branch to be accessible after the if/else statement.

**Correct approach**:
```rust
// Declare before if/else
let mut basename: String;

// Assign in branches
if _cse_temp_0 {
    basename = { ... };  // Assignment, not declaration
} else {
    basename = path;     // Assignment, not declaration
}

Ok(basename)  // Now accessible
```

### Bug #2: Tuple Type Mapping

**Location**: Type inference/mapping code, likely in `crates/depyler-core/src/type_inference.rs` or type conversion utilities

**Root cause**: The type system doesn't have a mapping for Python's `tuple` annotation to Rust's tuple syntax.

**Why it's wrong**:
- Python: `tuple` (generic type, elements can be of any type)
- Rust: `(T1, T2, ...)` (specific tuple with typed elements)

**Challenges**:
1. Python's `tuple` without type parameters is too generic
2. Need to infer element types from return value or provide default like `(String, String)`
3. Should ideally use tuple type hints like `tuple[str, str]` for precise mapping

## Impact Assessment

**Affected Files** (discovered via grep):
- `examples/test_os_sys_module.py` - Multiple functions affected
- Any Python code using:
  - If/else with variable assignments in both branches
  - `tuple` type annotations without specific element types

**Compilation Status**:
- ❌ 0% success rate for affected patterns
- Blocks all os/sys module examples from compiling

**User Impact**:
- **CRITICAL**: Cannot transpile common Python patterns
- **BLOCKS**: Standard library os/sys module examples
- **DEGRADES**: User confidence in transpiler reliability

## Test Cases

### Test Case 1: If/Else Variable Scope

**Input** (`test_if_else_scope.py`):
```python
def test_variable_scope() -> str:
    condition: bool = True
    if condition:
        result: str = "if_branch"
    else:
        result: str = "else_branch"
    return result
```

**Expected Rust**:
```rust
pub fn test_variable_scope() -> Result<String, Box<dyn std::error::Error>> {
    let condition: bool = true;
    let mut result: String;
    if condition {
        result = "if_branch".to_string();
    } else {
        result = "else_branch".to_string();
    }
    Ok(result)
}
```

**Current (broken) Rust**:
```rust
pub fn test_variable_scope() -> Result<String, Box<dyn std::error::Error>> {
    let condition: bool = true;
    if condition {
        let mut result: String = "if_branch".to_string();  // Scoped!
    } else {
        let mut result: String = "else_branch".to_string(); // Scoped!
    }
    Ok(result)  // ERROR: cannot find value `result`
}
```

### Test Case 2: Tuple Type Annotation

**Input** (`test_tuple_return.py`):
```python
def get_pair() -> tuple:
    return ("first", "second")

def get_typed_pair() -> tuple[str, int]:
    return ("value", 42)
```

**Expected Rust**:
```rust
// For generic tuple (best effort)
pub fn get_pair() -> Result<(String, String), Box<dyn std::error::Error>> {
    Ok(("first".to_string(), "second".to_string()))
}

// For typed tuple
pub fn get_typed_pair() -> Result<(String, i32), Box<dyn std::error::Error>> {
    Ok(("value".to_string(), 42))
}
```

**Current (broken) Rust**:
```rust
pub fn get_pair() -> Result<tuple, Box<dyn std::error::Error>> {
    //                       ^^^^^ ERROR: cannot find type `tuple`
}
```

## Solution Design

### Bug #1: Variable Hoisting Strategy

**Approach**: Implement "variable hoisting" for if/else blocks

**Algorithm**:
1. **Detect pattern**: Variable assigned in both if and else branches
2. **Hoist declaration**: Create `let mut var: Type;` before if statement
3. **Convert to assignment**: Change `let mut var = value;` to `var = value;` in branches
4. **Handle initialization**: If type inference requires it, use `Default::default()` or specific placeholder

**Implementation** (pseudocode):
```rust
// In codegen for If statement
fn generate_if_else(&mut self, if_stmt: &IfStatement) -> String {
    // 1. Analyze both branches for assigned variables
    let if_vars = self.find_assigned_vars(&if_stmt.if_body);
    let else_vars = self.find_assigned_vars(&if_stmt.else_body);

    // 2. Find common variables (assigned in both branches)
    let hoisted_vars = if_vars.intersection(&else_vars);

    // 3. Generate hoisted declarations
    let mut code = String::new();
    for var in hoisted_vars {
        let var_type = self.infer_type(var);
        code.push_str(&format!("let mut {}: {};\n", var.name, var_type));
    }

    // 4. Generate if/else with assignments (not declarations)
    code.push_str(&self.generate_if_with_assignments(if_stmt, &hoisted_vars));

    code
}
```

### Bug #2: Tuple Type Mapping

**Approach**: Map Python `tuple` to Rust tuple syntax with type inference

**Strategy**:
1. **For generic `tuple`**: Infer from return value structure
   - `return (a, b)` → `(TypeOf(a), TypeOf(b))`
   - Default to `(String, String)` if inference fails
2. **For typed `tuple[T1, T2]`**: Direct mapping
   - `tuple[str, int]` → `(String, i32)`
3. **For empty `tuple`**: `()`

**Implementation** (pseudocode):
```rust
fn map_python_type_to_rust(&self, py_type: &PythonType) -> String {
    match py_type {
        PythonType::Tuple { element_types: None } => {
            // Generic tuple - try to infer from context
            if let Some(inferred) = self.infer_tuple_types_from_usage() {
                format!("({})", inferred.join(", "))
            } else {
                // Fallback: 2-tuple of strings (most common case)
                "(String, String)".to_string()
            }
        }
        PythonType::Tuple { element_types: Some(types) } => {
            let rust_types: Vec<_> = types.iter()
                .map(|t| self.map_python_type_to_rust(t))
                .collect();
            format!("({})", rust_types.join(", "))
        }
        _ => // ... other type mappings
    }
}
```

## Implementation Plan

### Phase 1: Bug #1 - Variable Scope Fix

**Files to modify**:
1. `crates/depyler-core/src/codegen.rs` - If/else code generation
2. `crates/depyler-core/src/analysis/control_flow.rs` - Variable usage analysis

**Steps**:
1. Add variable usage analysis for if/else blocks
2. Implement hoisting logic for common variables
3. Modify if/else codegen to use assignments instead of declarations
4. Add test cases for various patterns

**Complexity**: ≤10 per function (meets quality standard)

### Phase 2: Bug #2 - Tuple Type Mapping

**Files to modify**:
1. `crates/depyler-core/src/type_inference.rs` - Type mapping
2. `crates/depyler-core/src/types.rs` - Type definitions

**Steps**:
1. Add `PythonType::Tuple` variant if not present
2. Implement tuple type inference from return values
3. Add mapping from `tuple[T1, T2]` to `(T1, T2)`
4. Handle edge cases (empty tuple, single element)

**Complexity**: ≤10 per function

### Phase 3: Comprehensive Testing

**Test coverage**:
1. Unit tests for hoisting algorithm
2. Unit tests for tuple type mapping
3. Integration test: Re-transpile `test_os_sys_module.py`
4. Property tests for edge cases
5. Regression tests for existing functionality

**Success criteria**:
- ✅ All test cases compile successfully
- ✅ Generated Rust passes `rustc --deny warnings`
- ✅ No regressions in existing tests
- ✅ Code coverage ≥80%

## Verification Plan

### Step 1: Create failing tests
```bash
# Add test cases to test suite
cargo test test_DEPYLER_0379 --lib
# Should fail initially
```

### Step 2: Implement fixes
```bash
# Fix transpiler code
# Run tests continuously
cargo watch -x 'test test_DEPYLER_0379'
```

### Step 3: Re-transpile affected examples
```bash
# Re-generate all examples
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

- **Related bugs**: None (new pattern discovery)
- **Similar issues**:
  - DEPYLER-0279 (dict codegen bugs) - Similar "stop the line" protocol
  - DEPYLER-0280 (duplicate mod tests) - Comprehensive bug documentation example
- **Documentation**:
  - `docs/processes/stop-the-line.md` - Process followed
  - Python scoping rules: https://docs.python.org/3/reference/executionmodel.html#naming-and-binding
  - Rust scoping rules: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

## Lessons Learned

### What went well:
- ✅ --trace flag immediately revealed the issue
- ✅ Compilation failure caught bugs before runtime
- ✅ Clear error messages from rustc

### What needs improvement:
- ❌ Should have had test coverage for if/else variable patterns
- ❌ Type mapping for `tuple` should have been in initial implementation
- ❌ Need property tests for control flow patterns

### Process improvements:
1. Add if/else variable patterns to systematic test matrix
2. Create comprehensive type mapping test suite
3. Add "compile all examples" to pre-commit hooks
4. Use --trace flag more proactively during development

## Timeline

- **2025-11-17 14:00**: Bugs discovered via --trace analysis
- **2025-11-17 14:15**: STOP THE LINE initiated
- **2025-11-17 14:30**: Bug document created (DEPYLER-0379)
- **2025-11-17 14:45**: Fix implementation begins
- **2025-11-17 TBD**: Testing and verification
- **2025-11-17 TBD**: Fixes committed and pushed

---

**Remember**: Fix the transpiler, not the generated code. Never bypass this protocol.
