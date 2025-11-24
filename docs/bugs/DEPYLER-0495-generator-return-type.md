# DEPYLER-0495: Generator Return Type Incorrect

**Status**: IN PROGRESS
**Priority**: P0-CRITICAL (STOP THE LINE)
**Assigned**: Claude
**Created**: 2025-11-23T21:15:00+00:00

## Problem Statement

Python generators with return type `Iterator[T]` are incorrectly transpiled to Rust `impl Iterator<Item = Iterator<T>>` instead of `impl Iterator<Item = T>`.

### Current Behavior (BROKEN)

**Python**:
```python
from typing import Iterator

def fibonacci_generator(limit: Optional[int] = None) -> Iterator[int]:
    a, b = 0, 1
    while limit is None or count < limit:
        yield a
        a, b = b, a + b
```

**Generated Rust** (INCORRECT):
```rust
pub fn fibonacci_generator(limit: &Option<i32>) -> impl Iterator<Item = Iterator<i32>> {
    FibonacciGeneratorState { ... }
}

impl Iterator for FibonacciGeneratorState {
    type Item = Iterator<i32>;  // ❌ WRONG!
    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

**Compilation Errors**:
```
error[E0107]: trait takes 0 generic arguments but 1 generic argument was supplied
  --> fibonacci.rs:83:73
   |
83 | pub fn fibonacci_generator(limit: &Option<i32>) -> impl Iterator<Item = Iterator<i32>> {
   |                                                                         ^^^^^^^^ expected 0 generic arguments

error[E0191]: the value of the associated type `Item` in `Iterator` must be specified
  --> fibonacci.rs:83:73
   |
83 | pub fn fibonacci_generator(limit: &Option<i32>) -> impl Iterator<Item = Iterator<i32>> {
   |                                                                         ^^^^^^^^^^^^^ associated type `Item` must be specified
```

### Expected Behavior (CORRECT)

**Generated Rust** (CORRECT):
```rust
pub fn fibonacci_generator(limit: &Option<i32>) -> impl Iterator<Item = i32> {
    FibonacciGeneratorState { ... }
}

impl Iterator for FibonacciGeneratorState {
    type Item = i32;  // ✅ CORRECT!
    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

## Root Cause Analysis

### Hypothesis 1: Type Mapper Incorrectly Nests Iterator

The type mapper may be wrapping the element type in an additional `Iterator<>` layer:
- Input: `Iterator[int]`
- Expected: `Iterator<Item = i32>`
- Actual: `Iterator<Item = Iterator<i32>>`

**Location**: Likely in `crates/depyler-core/src/type_mapper.rs` or generator codegen

### Hypothesis 2: Generator Codegen Confuses Iterator Type

The generator codegen may be using the wrong type when emitting:
1. Function return type signature
2. `impl Iterator for State` associated type

**Location**: `crates/depyler-core/src/rust_gen/generator_gen.rs`

## Impact

**Severity**: P0-CRITICAL - Breaks ALL generator functions

**Affected Code**:
- ✅ `fibonacci_generator` in examples/test_project/fibonacci.py
- ✅ Any generator function with `Iterator[T]` return type
- ✅ 4 compilation errors in fibonacci.rs (E0107, E0191)

**Scope**:
- Generators are core Python feature
- This bug makes generators completely unusable
- Blocks adoption for any code using generators

## Test Plan

### Phase 1: RED - Failing Tests

Create comprehensive test suite in `crates/depyler-core/tests/depyler_0495_generator_return_type.rs`:

**Test Cases**:
1. ✅ `test_generator_iterator_int` - `Iterator[int]` → `impl Iterator<Item = i32>`
2. ✅ `test_generator_iterator_string` - `Iterator[str]` → `impl Iterator<Item = String>`
3. ✅ `test_generator_iterator_tuple` - `Iterator[Tuple[int, str]]` → proper tuple type
4. ✅ `test_generator_iterator_optional` - `Iterator[Optional[int]]` → `Option<i32>`
5. ✅ `test_generator_compilation` - Generated code compiles with rustc

**Assertions**:
- Function signature contains `impl Iterator<Item = T>` (not nested Iterator)
- Impl block contains `type Item = T;` (not `Iterator<T>`)
- No E0107 or E0191 errors in rustc output
- Generated code compiles successfully

### Phase 2: GREEN - Implementation

**Fix Location**: Identify and fix in one of:
1. `type_mapper.rs` - Fix Iterator type mapping
2. `generator_gen.rs` - Fix return type emission
3. `func_gen.rs` - Fix function signature generation

**Expected Changes**:
- Iterator[T] maps to correct Rust type
- Generator state struct has correct Item associated type
- Function return type uses correct impl Iterator syntax

### Phase 3: REFACTOR - Quality Gates

**Quality Checks**:
- ✅ All 6 existing generator tests pass (no regressions)
- ✅ New tests pass (5/5)
- ✅ fibonacci.rs compiles without E0107/E0191 errors
- ✅ Complexity ≤10 (PMAT enforcement)
- ✅ No clippy warnings
- ✅ Test coverage ≥80%

**Verification**:
```bash
# Run new tests
cargo test depyler_0495_generator_return_type

# Verify no regressions
cargo test depyler_0494_generator_scoping

# Re-transpile fibonacci.py
cargo run --bin depyler -- transpile examples/test_project/fibonacci.py

# Verify compilation
rustc --crate-type=lib --deny=warnings examples/test_project/fibonacci.rs
```

## Files to Modify

**Primary**:
- `crates/depyler-core/src/type_mapper.rs` - Fix Iterator[T] mapping
- `crates/depyler-core/src/rust_gen/generator_gen.rs` - Fix return type emission

**Tests**:
- `crates/depyler-core/tests/depyler_0495_generator_return_type.rs` (NEW)

**Examples**:
- `examples/test_project/fibonacci.rs` - Re-transpile after fix

## Acceptance Criteria

- [x] Create comprehensive failing tests (RED phase)
- [ ] Iterator[int] transpiles to `impl Iterator<Item = i32>`
- [ ] Iterator[str] transpiles to `impl Iterator<Item = String>`
- [ ] Generated `type Item = T` (not `Iterator<T>`)
- [ ] fibonacci_generator compiles without E0107/E0191 errors
- [ ] No regression in 6 existing generator tests
- [ ] Test coverage ≥80%
- [ ] Complexity ≤10
- [ ] Error count: 20 → 16 (eliminate 4 Iterator errors)

## Timeline

**Estimated Effort**: 3 hours
- Phase 1 (RED): 45 minutes
- Phase 2 (GREEN): 90 minutes
- Phase 3 (REFACTOR): 45 minutes

## Related Issues

- **DEPYLER-0494**: Generator variable scoping (COMPLETED)
- **Remaining fibonacci.rs errors**: 16 errors after this fix (Display traits, type mismatches, etc.)

## Notes

This is a **STOP THE LINE** bug - it breaks a core Python feature (generators). Must be fixed before any other work.

The bug is likely a simple type mapping issue where `Iterator[T]` is being interpreted as "Iterator of Iterator<T>" instead of "Iterator with element type T".
