# DEPYLER-0280: Duplicate `mod tests` Blocks in Generated Code

**Status**: 🔴 BLOCKING - STOP THE LINE
**Severity**: P0 - Critical (Prevents compilation)
**Discovered**: 2025-10-28
**Reporter**: Matrix Testing Project validation
**Type**: Code Generation Bug

---

## 1. Executive Summary

The test generation system creates multiple `#[cfg(test)] mod tests {}` blocks with identical names, causing Rust compilation to fail with "the name `tests` is defined multiple times" errors.

**Impact**:
- ❌ Generated code does not compile
- ❌ Blocks all testing workflows
- ❌ Breaks Matrix Testing Project validation
- ❌ Affects all files with multiple functions

**Discovery Context**:
- Found during Matrix Testing Project re-transpilation with v3.19.21
- Occurred when transpiling `01_basic_types/column_a.py` (10 functions)
- Generated 6 separate `mod tests` blocks (one per tested function)

---

## 2. Problem Description

### 2.1 Reproduction Steps

```bash
# Step 1: Transpile Python file with multiple functions
cargo run --bin depyler -- transpile \
  examples/01_basic_types/column_a/column_a.py \
  --output examples/01_basic_types/column_b/src/lib.rs \
  --verify --gen-tests

# Step 2: Attempt to compile
cd examples/01_basic_types/column_b
cargo build

# Result: Compilation fails with duplicate module errors
```

### 2.2 Error Output

```
error[E0428]: the name `tests` is defined multiple times
  --> src/lib.rs:94:1
   |
71 | mod tests {
   | --------- previous definition of the module `tests` here
...
94 | mod tests {
   | ^^^^^^^^^ `tests` redefined here
   |
   = note: `tests` must be defined only once in the type namespace of this module

error[E0428]: the name `tests` is defined multiple times
   --> src/lib.rs:111:1
    |
 71 | mod tests {
    | --------- previous definition of the module `tests` here
...
111 | mod tests {
    | ^^^^^^^^^ `tests` redefined here
```

### 2.3 Minimal Test Case

**Input** (`test_multiple_funcs.py`):
```python
def add(a: int, b: int) -> int:
    return a + b

def subtract(a: int, b: int) -> int:
    return a - b

def multiply(a: int, b: int) -> int:
    return a * b
```

**Current Output** (BROKEN):
```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn subtract(a: i32, b: i32) -> i32 { a - b }
pub fn multiply(a: i32, b: i32) -> i32 { a * b }

#[cfg(test)]
mod tests {  // ❌ First definition
    use super::*;
    #[test]
    fn test_add_examples() {
        assert_eq!(add(1, 2), 3);
    }
}

#[cfg(test)]
mod tests {  // ❌ Duplicate! Compilation error
    use super::*;
    #[test]
    fn test_subtract_examples() {
        assert_eq!(subtract(5, 3), 2);
    }
}

#[cfg(test)]
mod tests {  // ❌ Another duplicate!
    use super::*;
    #[test]
    fn test_multiply_examples() {
        assert_eq!(multiply(3, 4), 12);
    }
}
```

---

## 3. Root Cause Analysis

### 3.1 Code Location

**File**: `/home/noah/src/depyler/crates/depyler-core/src/test_generation.rs`

**Function**: `generate_test_cases()` - lines 380-600 (approximate)

### 3.2 Current Implementation

The test generator iterates over each function and generates:
```rust
#[cfg(test)]
mod tests {
    // test code for this function
}
```

This works for **single-function files** but breaks for **multi-function files**.

### 3.3 Why This Happens

1. `generate_test_cases()` is called once per function
2. Each call generates a standalone `mod tests {}` block
3. All blocks have the same name: `tests`
4. Rust modules must have unique names within a scope
5. Result: Compilation error

### 3.4 Architecture Issue

The test generation is **per-function** rather than **per-module**. This violates Rust's module naming rules.

**Current Architecture**:
```
generate_test_cases(func1) → mod tests { test1 }
generate_test_cases(func2) → mod tests { test2 }  // ❌ Duplicate name!
generate_test_cases(func3) → mod tests { test3 }  // ❌ Duplicate name!
```

---

## 4. Solution Design

### 4.1 Option A: Single `mod tests` Block (RECOMMENDED)

Generate **one** `mod tests` block per file containing **all** test functions.

**Implementation**:
```rust
// Collect all test functions first
let mut all_test_items = Vec::new();
for func in functions {
    let test_items = generate_test_items_for_function(func)?;
    all_test_items.extend(test_items);
}

// Generate single mod tests block
quote! {
    #[cfg(test)]
    mod tests {
        use super::*;
        use quickcheck::{quickcheck, TestResult};

        #(#all_test_items)*
    }
}
```

**Pros**:
- ✅ Idiomatic Rust (standard pattern)
- ✅ Clean, readable test organization
- ✅ Single `use super::*;` import
- ✅ Easy to understand

**Cons**:
- ⚠️ Requires refactoring test generation flow

### 4.2 Option B: Unique Module Names per Function

Generate unique module names: `mod tests_add`, `mod tests_subtract`, etc.

**Implementation**:
```rust
let test_mod_name = format_ident!("tests_{}", func.name);

quote! {
    #[cfg(test)]
    mod #test_mod_name {
        use super::*;
        #(#test_items)*
    }
}
```

**Pros**:
- ✅ Minimal code change
- ✅ Preserves per-function generation

**Cons**:
- ❌ Non-idiomatic (unusual pattern)
- ❌ Multiple `use super::*;` imports
- ❌ Clutters module namespace

### 4.3 Recommended Solution: Option A

**Rationale**:
1. **Idiomatic**: Standard Rust testing pattern
2. **Quality**: Cleaner, more maintainable
3. **Extensibility**: Easier to add module-level test setup
4. **Convention**: Matches cargo's test template

---

## 5. Implementation Plan

### 5.1 Changes Required

**File**: `crates/depyler-core/src/test_generation.rs`

**Current Structure**:
```rust
impl HirModule {
    pub fn generate_tests(&self) -> Result<TokenStream> {
        let mut tests = Vec::new();
        for func in &self.functions {
            // ❌ Generates separate mod tests for each function
            let test_block = generate_test_cases(func)?;
            tests.push(test_block);
        }
        Ok(quote! { #(#tests)* })
    }
}
```

**New Structure**:
```rust
impl HirModule {
    pub fn generate_tests(&self) -> Result<TokenStream> {
        let mut test_items = Vec::new();

        // Step 1: Generate test items for all functions
        for func in &self.functions {
            let items = generate_test_items_for_function(func)?;
            test_items.extend(items);
        }

        // Step 2: Wrap all items in single mod tests block
        Ok(quote! {
            #[cfg(test)]
            mod tests {
                use super::*;
                use quickcheck::{quickcheck, TestResult};

                #(#test_items)*
            }
        })
    }
}

// Refactored helper (no longer generates mod wrapper)
fn generate_test_items_for_function(func: &HirFunction) -> Result<Vec<syn::Item>> {
    let mut items = Vec::new();

    // Generate property test
    if should_generate_property_test(func) {
        items.push(generate_property_test(func)?);
    }

    // Generate example test
    if should_generate_example_test(func) {
        items.push(generate_example_test(func)?);
    }

    Ok(items)
}
```

### 5.2 Detailed Changes

#### Change 1: Rename `generate_test_cases()` → `generate_test_items_for_function()`

**Before**:
```rust
pub fn generate_test_cases(func: &HirFunction) -> Result<TokenStream> {
    // ... generates mod tests { ... }
}
```

**After**:
```rust
pub fn generate_test_items_for_function(func: &HirFunction) -> Result<Vec<syn::Item>> {
    // ... generates test functions only (no mod wrapper)
}
```

#### Change 2: Update Return Type

**Before**: Returns `TokenStream` containing `mod tests { ... }`
**After**: Returns `Vec<syn::Item>` containing test functions

#### Change 3: Module-Level Generation

Add new function to wrap all test items:

```rust
pub fn generate_tests_module(functions: &[HirFunction]) -> Result<TokenStream> {
    let mut test_items = Vec::new();

    for func in functions {
        let items = generate_test_items_for_function(func)?;
        test_items.extend(items);
    }

    Ok(quote! {
        #[cfg(test)]
        mod tests {
            use super::*;
            use quickcheck::{quickcheck, TestResult};

            #(#test_items)*
        }
    })
}
```

### 5.3 Migration Path

1. **Step 1**: Create `generate_test_items_for_function()` (new function)
2. **Step 2**: Update call sites to use new function
3. **Step 3**: Add module-level wrapper
4. **Step 4**: Remove old `generate_test_cases()` function
5. **Step 5**: Run full test suite

---

## 6. Testing Strategy

### 6.1 Unit Tests

**Test File**: `crates/depyler-core/tests/test_generation_tests.rs`

```rust
#[test]
fn test_single_mod_tests_block_for_multiple_functions() {
    let source = r#"
def add(a: int, b: int) -> int:
    return a + b

def sub(a: int, b: int) -> int:
    return a - b
"#;

    let result = transpile_with_tests(source).unwrap();

    // Verify: Only ONE mod tests block
    let mod_count = result.matches("mod tests").count();
    assert_eq!(mod_count, 1, "Should generate exactly one mod tests block");

    // Verify: Contains tests for both functions
    assert!(result.contains("test_add_examples"));
    assert!(result.contains("test_sub_examples"));

    // Verify: Code compiles
    assert!(rust_code_compiles(&result));
}

#[test]
fn test_empty_file_generates_no_test_module() {
    let source = r#"
# Empty file with no functions
"#;

    let result = transpile_with_tests(source).unwrap();
    assert!(!result.contains("mod tests"));
}

#[test]
fn test_ten_functions_single_mod_tests() {
    let source = generate_n_functions(10);
    let result = transpile_with_tests(&source).unwrap();

    let mod_count = result.matches("mod tests").count();
    assert_eq!(mod_count, 1);

    // Verify compilation
    assert!(rust_code_compiles(&result));
}
```

### 6.2 Integration Tests

**Test**: Re-transpile Matrix Project example
```bash
cargo run --bin depyler -- transpile \
  examples/01_basic_types/column_a/column_a.py \
  --output /tmp/test_output.rs \
  --verify --gen-tests

# Verify compilation
rustc --crate-type lib /tmp/test_output.rs

# Verify tests run
rustc --test /tmp/test_output.rs -o /tmp/test_runner
/tmp/test_runner
```

### 6.3 Regression Tests

Add test case to prevent recurrence:

```rust
#[test]
fn test_no_duplicate_mod_tests_regression() {
    // DEPYLER-0280 regression test
    let source = include_str!("../../test_data/multiple_functions.py");
    let result = transpile_with_tests(source).unwrap();

    // Count mod tests occurrences
    let mod_tests_count = result.matches("mod tests {").count();

    assert_eq!(
        mod_tests_count, 1,
        "REGRESSION: DEPYLER-0280 - Multiple mod tests blocks detected. \
         Should generate exactly one mod tests block per file."
    );
}
```

---

## 7. Expected Behavior (After Fix)

**Input** (`test_multiple_funcs.py`):
```python
def add(a: int, b: int) -> int:
    return a + b

def subtract(a: int, b: int) -> int:
    return a - b

def multiply(a: int, b: int) -> int:
    return a * b
```

**Expected Output** (CORRECT):
```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn subtract(a: i32, b: i32) -> i32 { a - b }
pub fn multiply(a: i32, b: i32) -> i32 { a * b }

#[cfg(test)]
mod tests {  // ✅ Single module
    use super::*;
    use quickcheck::{quickcheck, TestResult};

    #[test]
    fn test_add_examples() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_subtract_examples() {
        assert_eq!(subtract(5, 3), 2);
    }

    #[test]
    fn test_multiply_examples() {
        assert_eq!(multiply(3, 4), 12);
    }
}
```

---

## 8. Verification Checklist

After implementing the fix:

- [ ] Unit tests pass (`cargo test -p depyler-core test_generation`)
- [ ] Integration test: Re-transpile `01_basic_types` example
- [ ] Compilation succeeds: `cargo build` in transpiled project
- [ ] Tests pass: `cargo test` in transpiled project
- [ ] Regression test added to prevent recurrence
- [ ] Full test suite passes (0 regressions)
- [ ] Matrix Project validation continues

---

## 9. Impact Assessment

### 9.1 Files Affected

**Primary**:
- `crates/depyler-core/src/test_generation.rs` (main fix)

**Secondary**:
- `crates/depyler-core/src/rust_gen/mod.rs` (call site update)
- `crates/depyler-core/tests/test_generation_tests.rs` (new tests)

**Test Data**:
- `crates/depyler-core/test_data/multiple_functions.py` (regression test)

### 9.2 Breaking Changes

**None** - This is a pure bug fix. Generated code structure changes, but:
- ✅ Test APIs unchanged
- ✅ Function signatures unchanged
- ✅ Test behavior identical
- ✅ Only affects generated code (internal)

### 9.3 Performance Impact

**Minimal** - Slightly more efficient (one module vs many):
- 🟢 Reduced AST nodes (single `mod tests` instead of N)
- 🟢 Faster compilation (fewer module boundaries)
- 🟢 Smaller generated code size

---

## 10. Related Issues

**None** - This is a new issue discovered during Matrix Testing validation.

**Relationship to Recent Fixes**:
- DEPYLER-0269 (Test generation type mismatch) - Same file, different bug
- DEPYLER-0279 (Dictionary codegen) - Different file

**Pattern**: Test generation subsystem needs comprehensive review.

---

## 11. Post-Fix Actions

### 11.1 Immediate

1. ✅ Document bug (this document)
2. 🔄 Implement fix
3. 🔄 Verify fix with regression test
4. 🔄 Re-transpile Matrix Project example
5. 🔄 Continue Matrix Project validation

### 11.2 Short-Term

1. Review test generation architecture for other issues
2. Add comprehensive test coverage for test generation
3. Update documentation on test generation patterns

### 11.3 Long-Term

1. Consider property-based testing for test generator itself
2. Add mutation testing for test generation code
3. Explore formal verification of test generator correctness

---

## 12. References

**Rust Module System**:
- [Rust Reference: Modules](https://doc.rust-lang.org/reference/items/modules.html)
- [Rust Book: Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

**Testing Patterns**:
- [Rust By Example: Testing](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
- [Cargo Book: Tests](https://doc.rust-lang.org/cargo/guide/tests.html)

**Similar Issues**:
- None found in public Rust transpiler projects

---

## 13. Timeline

**Discovery**: 2025-10-28 12:37 UTC (during Matrix Project validation)
**Documentation**: 2025-10-28 12:40 UTC (this document)
**Fix Implementation**: IN PROGRESS
**Verification**: PENDING
**Resolution**: PENDING

---

**Status**: 🔴 BLOCKING - Actively being fixed following Stop the Line protocol.
