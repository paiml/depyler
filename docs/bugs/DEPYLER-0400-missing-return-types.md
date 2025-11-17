# DEPYLER-0400: Missing Return Type Annotations in Function Signatures

## Metadata
- **Ticket**: DEPYLER-0400
- **Priority**: P0 (STOP ALL WORK - Compilation Failure)
- **Impact**: 28 files (19.7% of examples)
- **Discovery Date**: 2025-11-17
- **Status**: In Progress

## Problem Statement

Functions with return values are transpiled without return type annotations, causing compilation failures.

### Example

**Python Input** (`examples/array_test.py`):
```python
def test_array_literals():
    arr1 = [1, 2, 3, 4, 5]
    arr2 = [0, 0, 0, 0]
    arr3 = [True, False, True]
    arr4 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    return (arr1, arr2, arr3, arr4)
```

**Current Transpiled Output** (`examples/array_test.rs`):
```rust
pub fn test_array_literals() {  // ← Missing return type!
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![0, 0, 0, 0];
    let arr3 = vec![true, false, true];
    let arr4 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    (arr1, arr2, arr3, arr4)
}
```

**Compilation Error**:
```
error[E0308]: mismatched types
 --> examples/array_test.rs:8:5
  |
3 | pub fn test_array_literals() {
  |                             - help: try adding a return type: `-> (Vec<i32>, Vec<i32>, Vec<bool>, Vec<i32>)`
...
8 |     (arr1, arr2, arr3, arr4)
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `(Vec<{integer}>, ..., ..., ...)`
```

**Expected Output**:
```rust
pub fn test_array_literals() -> (Vec<i32>, Vec<i32>, Vec<bool>, Vec<i32>) {
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![0, 0, 0, 0];
    let arr3 = vec![true, false, true];
    let arr4 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    (arr1, arr2, arr3, arr4)
}
```

## Root Cause

**Location**: `crates/depyler-core/src/rust_gen/func_gen.rs`

The function generation code infers return types from Python type hints but does NOT synthesize return types when:
1. Function has explicit return statement
2. Last statement is an expression (implicit return)
3. No Python type hint provided

The HIR (`crates/depyler-core/src/hir.rs`) captures Python return statements and expressions, but the Rust code generator does not analyze the function body to infer the return type.

## Solution

### Approach: Synthesize Return Types from HIR Analysis

1. **Detect Return Statements**: Scan function body HIR for `HirStmt::Return` statements
2. **Infer Type**: Use existing type inference to determine return value type
3. **Generate Annotation**: Add `-> Type` to function signature if return type != `()`

### Implementation Plan

1. Modify `codegen_function()` in `func_gen.rs:200-400`
2. Add helper `infer_return_type(body: &[HirStmt]) -> Option<Type>`
3. Check if function has returns before assuming `-> ()`
4. Generate proper return type annotation

## Test Strategy

### Unit Tests
```rust
#[test]
fn test_depyler_0400_explicit_return() {
    // Python: def f(): return 42
    // Rust:   pub fn f() -> i32 { 42 }
}

#[test]
fn test_depyler_0400_implicit_return_tuple() {
    // Python: def f(): return (1, 2, 3)
    // Rust:   pub fn f() -> (i32, i32, i32) { (1, 2, 3) }
}

#[test]
fn test_depyler_0400_no_return() {
    // Python: def f(): print("hello")
    // Rust:   pub fn f() { println!("hello"); }
}
```

### Integration Tests
Re-transpile and compile 28 affected files:
- `array_test.rs`
- `dict_assign.rs`
- `test_iterator.rs`
- ... (25 more)

Expected: 28 files should gain return types and compile further (may still have other errors).

## Impact Analysis

**Before Fix**:
- Compilation: 12/142 (8.5%)
- Blocked: 28 files by E0308 missing return type

**After Fix** (estimated):
- Compilation: ~30-40/142 (21-28%)
- Unblocked: 28 files
- May reveal secondary errors in some files

## Affected Files (28 total)

Sample (first 10):
1. `examples/array_test.rs` - Returns tuple of Vecs
2. `examples/dict_assign.rs` - Returns HashMap
3. `examples/test_iterator.rs` - Returns i32
4. `examples/array_functions_test.rs` - Returns Vec
5. `examples/basic_lambda.rs` - Returns closure result
6. `examples/test_nested_access.rs` - Returns nested value
7. `examples/test_performance_warnings.rs` - Returns String
8. `examples/test_property.rs` - Returns property value
9. `examples/test_frozenset.rs` - Returns HashSet
10. `examples/test_iterator.rs` - Returns sum

## References
- HIR Return Statement: `crates/depyler-core/src/hir.rs:250-260`
- Function Codegen: `crates/depyler-core/src/rust_gen/func_gen.rs:200-400`
- Type Inference: `crates/depyler-core/src/type_inference/mod.rs`

## Notes
- This is a code quality issue, not a semantic bug
- Python functions without type hints should still work
- Rust compiler helpfully suggests the correct return type
- Fix will significantly improve compilation success rate
