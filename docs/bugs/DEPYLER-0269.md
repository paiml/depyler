# DEPYLER-0269: Function Parameter Borrowing - Missing Borrow Operator

**Ticket ID**: DEPYLER-0269
**Status**: 🔴 OPEN (RED phase)
**Priority**: P1 - HIGH (common pattern)
**Category**: Type System / Function Calls
**Discovered**: 2025-10-27 (Benchmark Transpilation Validation)
**Affects**: v3.19.20 and earlier

---

## Executive Summary

The transpiler generates function calls that pass owned values to functions expecting references, causing **mismatched types** compilation errors. The transpiler fails to insert the borrow operator `&` when needed.

**Impact**: Blocks compilation of any code where functions accept reference parameters (`&Vec<T>`, `&str`, etc.) and callers pass owned values.

---

## Reproduction

### Minimal Test Case

**Python Input**:
```python
def process_data(numbers: list[int]) -> dict[str, int]:
    """Function that expects a reference to a list."""
    return {"count": len(numbers)}

def main() -> None:
    """Caller passes owned list."""
    data = [1, 2, 3]
    result = process_data(data)  # Should add & here
    print(result)
```

### Expected Rust Output

```rust
pub fn process_data<'a>(numbers: &'a Vec<i32>) -> Result<HashMap<String, i32>, IndexError> {
    Ok({
        let mut map = HashMap::new();
        map.insert("count".to_string(), numbers.len() as i32);
        map
    })
}

pub fn main() {
    let data = vec![1, 2, 3];
    let result = process_data(&data);  // ✅ Borrow operator added
    println!("{:?}", result);
}
```

### Actual Generated Code (BROKEN)

```rust
pub fn process_data<'a>(numbers: &'a Vec<i32>) -> Result<HashMap<String, i32>, IndexError> {
    // ... same as above
}

pub fn main() {
    let data = vec![1, 2, 3];
    let result = process_data(data);  // ❌ Missing &
    println!("{:?}", result);
}
```

### Compilation Error

```
error[E0308]: mismatched types
  --> output.rs:XX:YY
   |
XX |     let result = process_data(data);
   |                               ^^^^ expected `&Vec<i32>`, found `Vec<i32>`
   |
help: consider borrowing here
   |
XX |     let result = process_data(&data);
   |                               +
```

---

## Real-World Evidence (Fibonacci Benchmark)

**File**: `benchmarks/rust/compute_intensive_transpiled.rs:109`

### Function Signature (Line 47)
```rust
pub fn calculate_statistics<'a>(numbers: &'a Vec<i32>) -> Result<HashMap<String, i32>, IndexError>
```
Expects: `&Vec<i32>` (reference)

### Broken Call Site (Line 109)
```rust
let stats = calculate_statistics(fib_sequence);
```
Passes: `Vec<i32>` (owned value)

### Compilation Error
```
error[E0308]: mismatched types
  --> benchmarks/rust/compute_intensive_transpiled.rs:109:38
   |
109 |         let stats = calculate_statistics(fib_sequence);
   |                     -------------------- ^^^^^^^^^^^^ expected `&Vec<i32>`, found `Vec<i32>`
   |                     |
   |                     arguments to this function are incorrect
```

### Required Fix
```rust
let stats = calculate_statistics(&fib_sequence);  // Add &
```

---

## Root Cause Analysis

### Location in Codebase

**Primary**: `crates/depyler-core/src/rust_gen/expr_gen.rs` (function call generation)
**Likely Function**: `generate_call()` or similar call expression handler

### Analysis

1. **Type Information Available**: The transpiler knows the function signature expects `&Vec<i32>`
2. **Caller Type Known**: The transpiler knows `fib_sequence` has type `Vec<i32>`
3. **Missing Logic**: No type matching check to insert `&` when passing owned → reference
4. **Pattern**: Common in Rust - functions take `&T` to avoid unnecessary moves

### Expected Behavior

When generating function calls, the transpiler should:
1. Retrieve callee function signature
2. For each argument position:
   - Check expected parameter type (from signature)
   - Check actual argument type (from caller expression)
   - If expected is `&T` and actual is `T` (owned), insert `&`
   - If expected is `&mut T` and actual is `T`, insert `&mut`

---

## Impact Assessment

### Severity: P1 - HIGH

**Frequency**: VERY COMMON
- Almost all Rust functions use references to avoid moves
- Standard pattern: `fn process(data: &Vec<T>)` not `fn process(data: Vec<T>)`
- Affects any function accepting `&str`, `&Vec<T>`, `&HashMap<K,V>`, etc.

**Workaround**: MANUAL EDITING REQUIRED
- User must manually add `&` to every call site
- Not feasible for production transpilation

**Blocks**:
- Fibonacci benchmark compilation (1 of 5 remaining errors)
- Any real-world Python code using functions with collection parameters

---

## Test Strategy (EXTREME TDD)

### RED Phase: Failing Tests

#### Test 1: Basic Reference Parameter
```rust
#[test]
fn test_DEPYLER_0269_basic_reference_parameter_compiles() {
    let python = r#"
def process(data: list[int]) -> int:
    return len(data)

def main() -> None:
    nums = [1, 2, 3]
    result = process(nums)
"#;
    // Should generate: process(&nums)
}
```

#### Test 2: Multiple Reference Parameters
```rust
#[test]
fn test_DEPYLER_0269_multiple_reference_parameters_compiles() {
    let python = r#"
def merge(list1: list[int], list2: list[int]) -> list[int]:
    return list1 + list2

def main() -> None:
    a = [1, 2]
    b = [3, 4]
    result = merge(a, b)
"#;
    // Should generate: merge(&a, &b)
}
```

#### Test 3: String Reference Parameter
```rust
#[test]
fn test_DEPYLER_0269_string_reference_parameter_compiles() {
    let python = r#"
def process_text(text: str) -> int:
    return len(text)

def main() -> None:
    message = "hello"
    length = process_text(message)
"#;
    // Should generate: process_text(&message)
}
```

#### Test 4: Dict Reference Parameter
```rust
#[test]
fn test_DEPYLER_0269_dict_reference_parameter_compiles() {
    let python = r#"
def count_keys(data: dict[str, int]) -> int:
    return len(data)

def main() -> None:
    info = {"a": 1, "b": 2}
    count = count_keys(info)
"#;
    // Should generate: count_keys(&info)
}
```

### GREEN Phase: Implementation Plan

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`

**Strategy**:
1. In function call generation, retrieve callee function signature
2. Iterate through arguments with their expected types
3. For each argument:
   ```rust
   fn should_add_borrow(expected: &RustType, actual: &RustType) -> bool {
       match (expected, actual) {
           (RustType::Reference(inner_expected), actual_owned)
               if inner_expected == actual_owned => true,
           _ => false,
       }
   }
   ```
4. If borrow needed, wrap argument in `&` operator

### REFACTOR Phase: Validation

- All 4 new tests pass ✅
- Fibonacci benchmark line 109 compiles ✅
- Full test suite passes (no regressions) ✅
- Coverage ≥80% ✅

---

## Success Criteria

### Compilation Success
```bash
# Fibonacci benchmark should compile
rustc --edition 2021 --crate-type bin --deny warnings \
  benchmarks/rust/compute_intensive_transpiled.rs
# Exit code: 0 (down from 5 errors to 4 errors)
```

### Generated Code Quality
```rust
// BEFORE (BROKEN):
let stats = calculate_statistics(fib_sequence);

// AFTER (FIXED):
let stats = calculate_statistics(&fib_sequence);
```

### Test Suite
```bash
cargo test test_DEPYLER_0269
# All 4 tests pass
```

---

## Related Issues

- **DEPYLER-0264**: Fixed DynamicType undefined (v3.19.20)
- **DEPYLER-0265**: Fixed iterator dereferencing (v3.19.20)
- **DEPYLER-0266**: Fixed boolean conversion (v3.19.20)
- **DEPYLER-0267**: Fixed index access .copied()/.cloned() (v3.19.20)

**Next in Queue**:
- **DEPYLER-0270**: Result unwrapping at call sites
- **DEPYLER-0271**: Main function return type
- **DEPYLER-0272**: Unused variable warnings

---

## References

### Documentation
- Rust Reference: Borrow operator `&` - https://doc.rust-lang.org/reference/expressions/operator-expr.html#borrow-operators
- Rust Book Chapter 4: Understanding Ownership - https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

### Benchmarks
- `benchmarks/python/compute_intensive.py` - Original Python
- `benchmarks/rust/compute_intensive_transpiled.rs:109` - Broken call site
- `benchmarks/TRANSPILATION_VALIDATION.md` - Full validation report

---

**Created**: 2025-10-27
**Campaign**: STOP THE LINE - Round 2
**Next Step**: Write RED phase tests
