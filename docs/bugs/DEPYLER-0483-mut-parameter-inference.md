# DEPYLER-0483: Incorrect `&mut` Parameter Type Inference

**Status**: 🔴 OPEN (Bug identified but not yet fixed in transpiler)
**Date**: 2025-11-23
**Severity**: P1 (BLOCK RELEASE) - Compilation error requiring manual fix
**Related**: None (new issue)
**Example**: example_config (`set_nested_value` function)

---

## Problem Statement

The transpiler incorrectly infers `&mut str` for function parameters when the parameter is **not actually mutated** in the function body. This causes compilation errors because the parameter is borrowed as mutable in the function signature but used immutably in the call site.

### Example

**Python**:
```python
def set_nested_value(config, key, value):
    """
    Set value in nested dict using dot notation
    """
    keys = key.split(".")
    current = config
    for k in keys[:-1]:
        if k not in current:
            current[k] = {}
        current = current[k]
    current[keys[-1]] = value  # value is used, not mutated
```

**Generated Rust** (INCORRECT):
```rust
pub fn set_nested_value<'c, 'a, 'b>(
    config: &'a serde_json::Value,
    key: &'b str,
    value: &'c mut str,  // ❌ WRONG: value is never mutated
) -> Result<(), IndexError> {
    // ...
    current[keys[-1]] = value;  // Just reads value, doesn't mutate
    Ok(())
}
```

**Call Site Error**:
```rust
Commands::Set { key, value } => {
    set_nested_value(&config, &key, &mut value);  // ❌ E0596: cannot borrow as mutable
    //                                  ^^^^ value is not declared as mutable
}
```

**Compilation Error**:
```
error[E0596]: cannot borrow `value` as mutable, as it is not declared as mutable
   --> config_manager.rs:169:45
    |
169 |             set_nested_value(&config, &key, &mut value);
    |                                             ^^^^^^^^^^ cannot borrow as mutable
```

**Expected Rust** (CORRECT):
```rust
pub fn set_nested_value<'a, 'b>(
    config: &'a serde_json::Value,
    key: &'b str,
    value: &str,  // ✅ Correct: immutable borrow
) -> Result<(), IndexError> {
    // ...
    current[keys[-1]] = value;
    Ok(())
}

// Call site
Commands::Set { key, value } => {
    set_nested_value(&config, &key, value);  // ✅ Works
}
```

---

## Root Cause Analysis

### Hypothesis
The transpiler may be incorrectly inferring mutability based on:
1. **Function signature analysis**: Misinterpreting `value` parameter as mutable
2. **Assignment context**: Seeing `current[keys[-1]] = value` and assuming `value` needs `&mut`
3. **Dict mutation confusion**: Confusing mutation of `current` (the dict) with mutation of `value` (the string being inserted)

### Location (Suspected)
Likely in parameter type inference or function signature generation:
- `/home/noah/src/depyler/crates/depyler-core/src/rust_gen/func_gen.rs`
- `/home/noah/src/depyler/crates/depyler-core/src/type_system/`

### Investigation Needed
1. Find where function parameter types are inferred
2. Check if assignment operations incorrectly mark parameters as mutable
3. Verify if the issue affects other examples

---

## Workaround (Temporary)

**Manual fix** required after transpilation:
```rust
// Change function signature from:
pub fn set_nested_value<'c, 'a, 'b>(
    config: &'a serde_json::Value,
    key: &'b str,
    value: &'c mut str,  // Remove 'c lifetime and mut
) -> Result<(), IndexError>

// To:
pub fn set_nested_value<'a, 'b>(
    config: &'a serde_json::Value,
    key: &'b str,
    value: &str,  // Immutable borrow
) -> Result<(), IndexError>

// And update call site from:
set_nested_value(&config, &key, &mut value)

// To:
set_nested_value(&config, &key, value)
```

---

## Impact

### Current Status
- **example_config**: ❌ Requires manual fix after transpilation
- **Other examples**: Unknown (needs investigation)

### Severity Justification
**P1 (BLOCK RELEASE)** because:
- ✅ Example compiles after transpilation (no syntax errors)
- ❌ Example fails to compile during `cargo build` (type error)
- ⚠️ Requires manual intervention to fix
- ⚠️ May affect multiple examples with similar patterns

**Not P0** because:
- The transpilation succeeds (not a transpiler crash)
- The generated code is close to correct (minor type fix needed)
- Workaround is straightforward

---

## Testing Strategy

### Test Case 1: Value-Only Parameter
```python
def use_value(data, value):
    data[0] = value  # value is read, not mutated
```

**Expected**:
```rust
fn use_value(data: &mut Vec<String>, value: &str) {  // ✅ value is &str
    data[0] = value.to_string();
}
```

### Test Case 2: Mutated Parameter
```python
def mutate_value(data, value):
    value = value.upper()  # value IS mutated
    data[0] = value
```

**Expected**:
```rust
fn mutate_value(data: &mut Vec<String>, value: &mut String) {  // ✅ value is &mut
    *value = value.to_uppercase();
    data[0] = value.clone();
}
```

### Test Case 3: Parameter Passed Through
```python
def pass_through(items, value):
    items.append(value)  // value is passed to method
```

**Expected**:
```rust
fn pass_through(items: &mut Vec<String>, value: &str) {  // ✅ value is &str
    items.push(value.to_string());
}
```

---

## Investigation Plan

### Step 1: Identify Affected Examples
```bash
# Search for functions with &mut parameters that shouldn't have them
grep -r "fn.*&mut str" examples/*/src/*.rs
grep -r "fn.*&'.*mut" examples/*/src/*.rs
```

### Step 2: Locate Type Inference Code
```bash
# Find where function parameter types are inferred
grep -r "infer_parameter_type\|param_type\|FunctionType" crates/depyler-core/src/
```

### Step 3: Add Test Cases
Create regression tests in `crates/depyler-core/tests/`:
- `depyler_0483_parameter_mutability.rs`

### Step 4: Fix Inference Logic
Modify parameter type inference to:
1. Track if parameter is **assigned to** (needs `&mut`)
2. Track if parameter is only **read from** (needs `&`)
3. Default to immutable borrow unless mutation is proven

---

## Related Issues

### Similar Bugs
- None identified yet (first instance of this pattern)

### May Be Related To
- **DEPYLER-0451**: Type inference improvements
- **DEPYLER-0455**: Type system overhaul

---

## Priority Justification

### Why P1 (Not P0)
- Does not block transpilation (code generates successfully)
- Compilation failure is clear and easily fixable
- Affects limited number of examples (potentially)

### Why Not P2
- Requires manual intervention (cannot ship as-is)
- May affect multiple examples (unknown scope)
- Creates poor user experience (manual fixes post-transpilation)

---

## Success Criteria

### Fix is Complete When:
1. ✅ example_config transpiles with correct parameter types
2. ✅ `cargo build` succeeds without manual fixes
3. ✅ Regression tests added for all parameter patterns
4. ✅ No other examples show similar issues

### Verification:
```bash
# Re-transpile example_config
depyler transpile examples/example_config/config_manager.py -o test_output.rs

# Check function signature
grep "fn set_nested_value" test_output.rs
# Expected: value: &str (NOT &mut str)

# Build without manual fixes
cd examples/example_config && cargo build
# Expected: Success
```

---

## Files To Investigate

### Source Code
1. `crates/depyler-core/src/rust_gen/func_gen.rs` - Function generation
2. `crates/depyler-core/src/type_system/inference.rs` - Type inference
3. `crates/depyler-core/src/rust_gen/stmt_gen.rs` - Statement generation (assignment tracking)

### Test Files
1. `crates/depyler-core/tests/depyler_0483_parameter_mutability.rs` (NEW)

### Examples
1. `examples/example_config/config_manager.py` - Primary example
2. Other examples (TBD based on investigation)

---

## Notes

### Why This Matters
Incorrect mutability inference creates **unnecessary friction** in the transpilation workflow:
- Users must manually fix type signatures
- Breaks "compile on first attempt" promise
- Reduces trust in transpiler correctness

### Python vs Rust Mutability
Python has no concept of `&` vs `&mut` - all references are mutable by default. The transpiler must:
1. **Analyze usage patterns** to determine if mutation occurs
2. **Default to immutable** unless proven mutable
3. **Be conservative** to avoid false positives

---

**Status**: 🔴 OPEN - Bug identified, awaiting transpiler fix
**Workaround**: Manual type signature correction
**Investigation**: Required
**Priority**: P1 (BLOCK RELEASE)
