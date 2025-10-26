# DEPYLER-0264: DynamicType Undefined for Untyped List Parameters

**Date**: 2025-10-26
**Priority**: P0 - BLOCKING
**Status**: DISCOVERED (Performance Benchmarking Campaign)
**Discovered By**: Performance benchmarking - compute_intensive.py transpilation

---

## Bug Summary

When transpiling Python functions with untyped `list` parameters (e.g., `def func(numbers: list)`), the transpiler generates `Vec<DynamicType>` in Rust, but **DynamicType is never defined or imported**. Generated code fails to compile with "cannot find type `DynamicType` in this scope".

## Impact

**BLOCKING**: Prevents transpilation of any Python code using untyped collections:
- Functions with `list` parameters (no element type specified)
- Functions with `dict` parameters (no key/value types specified)
- Functions with `set` parameters (no element type specified)

**Severity**: P0 - All TDD Book examples use typed collections (`list[int]`, `dict[str, float]`) to work around this bug

## Reproduction

### Input (Python):
```python
def calculate_statistics(numbers: list) -> dict:
    """Calculate basic statistics on a list of numbers."""
    if not numbers:
        return {"count": 0, "sum": 0, "min": 0, "max": 0}

    count = len(numbers)
    total = 0
    min_val = numbers[0]
    max_val = numbers[0]

    for num in numbers:
        total += num
        if num < min_val:
            min_val = num
        if num > max_val:
            max_val = num

    return {
        "count": count,
        "sum": total,
        "min": min_val,
        "max": max_val
    }
```

### Output (Generated Rust):
```rust
pub fn calculate_statistics<'a>(
    numbers: &'a Vec<DynamicType>,  // ❌ DynamicType not defined!
) -> Result<HashMap<DynamicType, DynamicType>, IndexError> {
    // ...
}
```

### Compilation Result:
```
error[E0412]: cannot find type `DynamicType` in this scope
  --> benchmarks/rust/compute_intensive.rs:48:22
   |
48 |     numbers: &'a Vec<DynamicType>,
   |                      ^^^^^^^^^^^ not found in this scope
```

---

## Root Cause Analysis

### Type Extraction Phase

**File**: `crates/depyler-core/src/ast_bridge/type_extraction.rs:88`

```rust
fn try_extract_collection_type(name: &str) -> Option<Type> {
    Some(match name {
        "list" => Type::List(Box::new(Type::Unknown)),  // ← Untyped list becomes List(Unknown)
        "dict" => Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)),
        "set" => Type::Set(Box::new(Type::Unknown)),
        _ => return None,
    })
}
```

### Type Mapping Phase

**File**: `crates/depyler-core/src/type_mapper.rs:122`

```rust
impl TypeMapper {
    pub fn map_type(&self, py_type: &PythonType) -> RustType {
        match py_type {
            PythonType::Unknown => RustType::Custom("DynamicType".to_string()), // ← Becomes DynamicType!
            // ...
            PythonType::List(inner) => RustType::Vec(Box::new(self.map_type(inner))), // ← Vec<DynamicType>
        }
    }
}
```

### The Problem Chain

1. Python: `def func(numbers: list)` (no type parameter)
2. Type Extraction: `Type::List(Box::new(Type::Unknown))`
3. Type Mapping: `RustType::Vec(Box::new(RustType::Custom("DynamicType")))`
4. Code Generation: `Vec<DynamicType>`
5. **FAILURE**: `DynamicType` is never defined or imported

---

## Evidence

### Codebase Search

```bash
$ grep -r "pub type DynamicType" crates/
# NO RESULTS - DynamicType is never defined!

$ grep -r "DynamicType" crates/ | wc -l
61  # Used in 61 places but never defined
```

### Working Examples Use Typed Collections

**File**: `tdd-book/tests/test_statistics.py`
```python
def mean(data: list[float]) -> float:  # ✅ Typed list - works!
    # ...
```

**Generated Rust**:
```rust
pub fn mean<'a>(data: &'a Vec<f64>) -> f64 {  // ✅ Vec<f64> - compiles!
```

### Examples That Don't Compile

**File**: `examples/debugging_workflow.rs` (line 23):
```rust
pub fn find_max<'a>(numbers: &'a Vec<DynamicType>) -> Result<i32, IndexError>
//                              ^^^^^^^^^^^ ERROR: not found in this scope
```

**Test Files**:
- `crates/depyler-core/tests/lambda_collections_test.rs` - Tests only check transpilation succeeds, NOT rustc compilation
- None of the lambda/collection tests actually compile the generated Rust!

---

## Fix Options

### Option 1: Default to Sensible Rust Type (RECOMMENDED)

Map `Type::Unknown` to a concrete Rust type instead of undefined `DynamicType`:

**File**: `crates/depyler-core/src/type_mapper.rs:122`

```rust
impl TypeMapper {
    pub fn map_type(&self, py_type: &PythonType) -> RustType {
        match py_type {
            // OLD:
            // PythonType::Unknown => RustType::Custom("DynamicType".to_string()),

            // NEW: Use serde_json::Value for maximum flexibility
            PythonType::Unknown => {
                // Add import tracking
                self.track_import("serde_json", "Value");
                RustType::Custom("serde_json::Value".to_string())
            }
        }
    }
}
```

**Pros**:
- ✅ Compiles immediately
- ✅ Matches existing pattern (lines 156-159 use `serde_json::Value` for untyped Dict)
- ✅ Runtime-checked, Python-like behavior
- ✅ Works with any Python type

**Cons**:
- ⚠️ Performance overhead (runtime type checking)
- ⚠️ Loses static type safety benefits

### Option 2: Emit Compiler Warning + Use Generic

Warn user about untyped collections and emit generic parameter:

```rust
pub fn calculate_statistics<'a, T>(numbers: &'a Vec<T>) -> HashMap<String, T>
where T: Default + PartialOrd + std::ops::AddAssign + Clone
```

**Pros**:
- ✅ Maintains type safety
- ✅ Compiler helps catch bugs
- ✅ Better performance

**Cons**:
- ❌ Requires trait bound inference (complex)
- ❌ May not work for all use cases

### Option 3: Require Type Annotations (STRICTEST)

Fail transpilation with helpful error:

```
ERROR: Untyped collection parameter not supported
  --> example.py:1:34
   |
1  | def calculate_statistics(numbers: list) -> dict:
   |                                  ^^^^ help: specify element type: `list[int]` or `list[float]`
```

**Pros**:
- ✅ Forces best practices
- ✅ No runtime overhead
- ✅ Clear error messages

**Cons**:
- ❌ Breaks existing Python code
- ❌ Not backward compatible

---

## Recommendation

**Fix**: Option 1 (serde_json::Value) for immediate unblocking
**Future**: Add linter warning suggesting typed collections for better performance

This matches existing transpiler pattern (lines 156-159 of type_mapper.rs already use serde_json::Value for untyped dict values).

---

## Test Plan

1. **Unit Test**: Add `test_untyped_list_parameter()` to `crates/depyler-core/tests/type_mapping_test.rs`
2. **Integration Test**: Add `test_DEPYLER_0264_untyped_list_compiles()` to `tests/bug_regression_tests.rs`
3. **Property Test**: Verify ALL untyped collections (list, dict, set) transpile to compilable Rust
4. **Compilation Test**: Run `rustc --crate-type lib` on generated output
5. **Behavior Test**: Verify runtime behavior matches Python (using quickcheck)

---

## Related Issues

- Similar issue with untyped `dict` parameters (generates `HashMap<DynamicType, DynamicType>`)
- Similar issue with untyped `set` parameters (generates `HashSet<DynamicType>`)
- **DEPYLER-0266**: Result unwrapping for dict return types (related - dict handling)

---

## References

- **Discovery**: Performance Benchmarking Campaign (2025-10-26)
- **File**: `benchmarks/python/compute_intensive.py`
- **Generated**: `benchmarks/rust/compute_intensive.rs`
- **Compilation Errors**: 13 total (this is error #1, #2, #3 - multiple instances)
- **Workaround**: Manual Rust implementation used for benchmarking
