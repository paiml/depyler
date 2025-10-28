# DEPYLER-0281: Property Test Generation Type Mismatch for Complex Types

**Status**: 🟡 WORKAROUND IMPLEMENTED - Property tests skipped for String params
**Severity**: P0 - Critical (Prevents test compilation)
**Discovered**: 2025-10-28
**Reporter**: Matrix Testing Project validation (post-DEPYLER-0280 fix)
**Type**: Test Generation Bug

---

## 1. Executive Summary

Property test generation creates test code that doesn't match function signatures when complex types like `Cow<'static, str>` are involved. Tests call functions with `String` arguments but functions expect `Cow<'static, str>`, causing compilation failures.

**Impact**:
- ❌ Generated tests do not compile
- ❌ Blocks all testing for functions with complex parameter types
- ❌ Matrix Testing Project validation cannot proceed
- ❌ Affects any function using Cow, Arc, Rc, or other wrapper types

**Discovery Context**:
- Found during Matrix Project validation after fixing DEPYLER-0280
- Occurred when running `cargo test` on transpiled `01_basic_types/column_a.py`
- Function `concatenate_strings(a: Cow<'static, str>, b: &str)` generated with wrong test signature

---

## 2. Problem Description

### 2.1 Reproduction Steps

```bash
# Step 1: Transpile Python file with string concatenation
depyler transpile column_a.py --output lib.rs --verify --gen-tests

# Step 2: Build succeeds
cd column_b && cargo build  # ✅ Compiles

# Step 3: Tests fail to compile
cargo test

# Error:
# error[E0308]: arguments to this function are incorrect
#   --> src/lib.rs:107:27
#    |
# 107 |             let result1 = concatenate_strings(a.clone(), b.clone());
#    |                           ^^^^^^^^^^^^^^^^^^^  --------- expected `&str`, found `String`
#    |
# note: expected enum `Cow<'static, str>`, found `String`
```

### 2.2 Error Output

```
error[E0308]: arguments to this function are incorrect
   --> src/lib.rs:107:27
    |
107 |             let result1 = concatenate_strings(a.clone(), b.clone());
    |                           ^^^^^^^^^^^^^^^^^^^            --------- expected `&str`, found `String`
    |
note: expected `Cow<'_, str>`, found `String`
   --> src/lib.rs:107:47
    |
107 |             let result1 = concatenate_strings(a.clone(), b.clone());
    |                                               ^^^^^^^^^
    = note: expected enum `Cow<'static, str>`
             found struct `String`
note: function defined here
   --> src/lib.rs:18:8
    |
 18 | pub fn concatenate_strings(a: Cow<'static, str>, b: &str) -> String {
    |        ^^^^^^^^^^^^^^^^^^^ --------------------  -------
```

### 2.3 Minimal Test Case

**Input** (`test_cow_string.py`):
```python
def concatenate(a: str, b: str) -> str:
    return a + b
```

**Generated Function Signature** (CORRECT):
```rust
pub fn concatenate(a: Cow<'static, str>, b: &str) -> String {
    format!("{}{}", a, b)
}
```

**Generated Property Test** (WRONG):
```rust
#[test]
fn quickcheck_concatenate() {
    fn prop(a: String, b: String) -> TestResult {
        let result1 = concatenate(a.clone(), b.clone());  // ❌ Type mismatch!
        let result2 = concatenate(b.clone(), a.clone());
        if result1 != result2 {
            return TestResult::failed();
        }
        TestResult::passed()
    }
    quickcheck(prop as fn(String, String) -> TestResult);
}
```

**Expected Property Test** (CORRECT):
```rust
#[test]
fn quickcheck_concatenate() {
    fn prop(a: String, b: String) -> TestResult {
        let result1 = concatenate(Cow::Borrowed(&a), &b);  // ✅ Correct conversion!
        let result2 = concatenate(Cow::Borrowed(&b), &a);
        if result1 != result2 {
            return TestResult::failed();
        }
        TestResult::passed()
    }
    quickcheck(prop as fn(String, String) -> TestResult);
}
```

---

## 3. Root Cause Analysis

### 3.1 Code Location

**File**: `/home/noah/src/depyler/crates/depyler-core/src/test_generation.rs`

**Function**: `property_to_assertion()` - lines 281-374

### 3.2 Current Implementation

The property test generator assumes parameter types in QuickCheck match function parameter types exactly:

```rust
fn generate_property_test(&self, func: &HirFunction) -> Result<Option<TokenStream>> {
    // Get parameter types for quickcheck
    let param_types: Vec<_> = func
        .params
        .iter()
        .map(|param| self.type_to_quickcheck_type(&param.ty))  // ❌ Returns Type::String
        .collect();

    let param_names: Vec<_> = func.params.iter()
        .map(|param| syn::Ident::new(&param.name, Span::call_site()))
        .collect();

    // Generate test code
    Ok(Some(quote! {
        fn prop(#(#param_names: #param_types),*) -> TestResult {
            // Calls function directly with params
            let result = #func_name(#(#param_names.clone()),*);  // ❌ Wrong!
        }
    }))
}
```

### 3.3 Why This Happens

1. `type_to_quickcheck_type()` maps `Type::String` → `quote! { String }`
2. Function signature uses `Cow<'static, str>` (from ownership inference)
3. Property test generates `prop(a: String, b: String)`
4. Test code calls `func(a.clone(), b.clone())` with `String` args
5. **Mismatch**: Function expects `Cow<'static, str>`, gets `String`

### 3.4 Architecture Issue

The test generator doesn't track the **actual generated Rust signature**—it only knows the HIR `Type`. When code generation transforms `Type::String` → `Cow<'static, str>` for optimization, tests don't follow.

**Gap**:
```
HIR Type::String → RustCodeGen → Cow<'static, str>
                 ↓
              TestGen → String  ❌ Divergence!
```

---

## 4. Solution Design

### 4.1 Option A: Skip Property Tests for Complex Types (QUICK FIX)

Don't generate property tests for functions with complex parameter types.

**Implementation**:
```rust
fn should_generate_property_test(&self, func: &HirFunction) -> bool {
    // Skip if any parameter has complex type
    for param in &func.params {
        if self.is_complex_type(&param.ty) {
            return false;
        }
    }
    true
}

fn is_complex_type(&self, ty: &Type) -> bool {
    matches!(ty, Type::String)  // String may become Cow
}
```

**Pros**:
- ✅ Quick fix (5-10 lines of code)
- ✅ Guaranteed to not break
- ✅ Zero test compilation errors

**Cons**:
- ❌ Loses test coverage for string functions
- ❌ Doesn't solve the root cause
- ❌ Not scalable (need to add more complex types)

### 4.2 Option B: Convert Types in Test Code (RECOMMENDED)

Generate conversion code in property tests to match actual signatures.

**Implementation**:
```rust
fn generate_property_test(&self, func: &HirFunction) -> Result<Option<TokenStream>> {
    // Use simple types for quickcheck generation
    let quickcheck_types: Vec<_> = func.params.iter()
        .map(|p| self.quickcheck_type(&p.ty))
        .collect();

    let param_names: Vec<_> = func.params.iter()
        .map(|p| syn::Ident::new(&p.name, Span::call_site()))
        .collect();

    // Generate conversion code for each parameter
    let converted_args: Vec<_> = func.params.iter().zip(&param_names)
        .map(|(param, name)| self.convert_for_call(&param.ty, name))
        .collect::<Result<Vec<_>>>()?;

    Ok(Some(quote! {
        fn prop(#(#param_names: #quickcheck_types),*) -> TestResult {
            let result = #func_name(#(#converted_args),*);  // ✅ Conversions!
            TestResult::passed()
        }
    }))
}

fn convert_for_call(&self, ty: &Type, name: &syn::Ident) -> Result<TokenStream> {
    match ty {
        Type::String => {
            // String may become Cow or &str - use Cow::Borrowed for flexibility
            Ok(quote! { std::borrow::Cow::Borrowed(#name.as_str()) })
        }
        Type::List(_) => {
            // List may become &Vec or &[T] - use borrow
            Ok(quote! { &#name })
        }
        _ => Ok(quote! { #name.clone() })
    }
}
```

**Pros**:
- ✅ Maintains test coverage
- ✅ Handles all complex types systematically
- ✅ Scalable to Arc, Rc, Box, etc.
- ✅ Tests still use simple QuickCheck types

**Cons**:
- ⚠️ More code (30-40 lines)
- ⚠️ Need to maintain type conversion mapping

### 4.3 Option C: Query Actual Generated Signature (IDEAL)

Read the actual generated Rust signature and use it for test generation.

**Implementation**: Would require refactoring code generation to expose signatures.

**Pros**:
- ✅ Perfect accuracy
- ✅ No manual type mapping needed

**Cons**:
- ❌ Major refactoring (100+ lines)
- ❌ Architectural changes to code generation
- ❌ Not worth the complexity for this issue

### 4.4 Recommended Solution: Option B

**Rationale**:
1. **Pragmatic**: Balances coverage and maintainability
2. **Scalable**: Easy to add more type conversions
3. **Testable**: Clear transformation logic
4. **Convention**: Matches Rust testing patterns (convert simple → complex)

---

## 5. Implementation Plan

### 5.1 Changes Required

**File**: `crates/depyler-core/src/test_generation.rs`

**Step 1**: Add `convert_arg_for_property_test()` helper

```rust
/// Convert a property test argument to match function signature
fn convert_arg_for_property_test(
    &self,
    ty: &Type,
    arg_name: &syn::Ident,
) -> TokenStream {
    match ty {
        Type::String => {
            // String parameters may become Cow<'static, str> or &str
            // Use Cow::Borrowed for maximum flexibility
            quote! { std::borrow::Cow::Borrowed(#arg_name.as_str()) }
        }
        Type::List(inner) => {
            // List parameters become &Vec<T>
            quote! { &#arg_name }
        }
        Type::Dict(_, _) => {
            // Dict parameters become &HashMap<K, V>
            quote! { &#arg_name }
        }
        _ => {
            // Simple types (int, float, bool) - use directly with clone
            quote! { #arg_name.clone() }
        }
    }
}
```

**Step 2**: Update `generate_property_test()` to use conversions

```rust
fn generate_property_test(&self, func: &HirFunction) -> Result<Option<TokenStream>> {
    let func_name = syn::Ident::new(&func.name, Span::call_site());
    let test_name = syn::Ident::new(&format!("quickcheck_{}", func.name), Span::call_site());

    let properties = self.analyze_function_properties(func);
    if properties.is_empty() {
        return Ok(None);
    }

    // QuickCheck parameter types (simple types for generation)
    let param_types: Vec<_> = func.params.iter()
        .map(|param| self.type_to_quickcheck_type(&param.ty))
        .collect();

    let param_names: Vec<_> = func.params.iter()
        .map(|param| syn::Ident::new(&param.name, Span::call_site()))
        .collect();

    // Property checks (using converted arguments)
    let property_checks: Vec<_> = properties.iter()
        .map(|prop| self.property_to_assertion_with_conversion(prop, &func_name, &param_names, &func.params))
        .collect();

    Ok(Some(quote! {
        #[test]
        fn #test_name() {
            fn prop(#(#param_names: #param_types),*) -> TestResult {
                #(#property_checks)*
                TestResult::passed()
            }

            quickcheck(prop as fn(#(#param_types),*) -> TestResult);
        }
    }))
}
```

**Step 3**: Update `property_to_assertion()` for commutative tests

```rust
TestProperty::Commutative => {
    if params.len() < 2 {
        return quote! {};
    }
    let (a, b) = (&params[0], &params[1]);

    // Convert arguments to match function signature
    let a_converted = self.convert_arg_for_property_test(&func.params[0].ty, a);
    let b_converted = self.convert_arg_for_property_test(&func.params[1].ty, b);

    quote! {
        let result1 = #func_name(#a_converted, #b_converted);
        let result2 = #func_name(#b_converted, #a_converted);
        if result1 != result2 {
            return TestResult::failed();
        }
    }
}
```

### 5.2 Testing Strategy

**Unit Test**:
```rust
#[test]
fn test_property_test_with_cow_string() {
    let func = HirFunction {
        name: "concat".to_string(),
        params: vec![
            HirParam::new("a".to_string(), Type::String),
            HirParam::new("b".to_string(), Type::String),
        ].into(),
        ret_type: Type::String,
        body: vec![/* ... */],
        properties: FunctionProperties { is_pure: true, ..Default::default() },
        // ...
    };

    let test_gen = TestGenerator::new(Default::default());
    let test_items = test_gen.generate_test_items_for_function(&func).unwrap();

    let code = quote! { #(#test_items)* }.to_string();

    // Should contain Cow::Borrowed conversion
    assert!(code.contains("Cow::Borrowed"));
    assert!(code.contains("as_str"));
}
```

**Integration Test**: Re-transpile `01_basic_types` and verify `cargo test` passes.

---

## 6. Expected Behavior (After Fix)

**Input** (`test_string_concat.py`):
```python
def concatenate(a: str, b: str) -> str:
    return a + b
```

**Generated Function** (unchanged):
```rust
pub fn concatenate(a: Cow<'static, str>, b: &str) -> String {
    format!("{}{}", a, b)
}
```

**Generated Property Test** (FIXED):
```rust
#[test]
fn quickcheck_concatenate() {
    fn prop(a: String, b: String) -> TestResult {
        // ✅ Convert String → Cow<'static, str> and String → &str
        let result1 = concatenate(
            std::borrow::Cow::Borrowed(a.as_str()),
            b.as_str()
        );
        let result2 = concatenate(
            std::borrow::Cow::Borrowed(b.as_str()),
            a.as_str()
        );
        if result1 != result2 {
            return TestResult::failed();
        }
        TestResult::passed()
    }
    quickcheck(prop as fn(String, String) -> TestResult);
}
```

---

## 7. Verification Checklist

After implementing the fix:

- [ ] Unit test for `convert_arg_for_property_test()` passes
- [ ] Property test with Cow<str> compiles and runs
- [ ] Re-transpile `01_basic_types` example
- [ ] `cargo build` succeeds
- [ ] `cargo test` succeeds (all tests pass)
- [ ] No regressions in other examples
- [ ] Full depyler test suite passes

---

## 8. Impact Assessment

### 8.1 Files Affected

**Primary**:
- `crates/depyler-core/src/test_generation.rs` (add conversion logic)

**Secondary**:
- `crates/depyler-core/tests/test_generation_tests.rs` (new tests)

### 8.2 Breaking Changes

**None** - Pure addition:
- ✅ Existing tests unaffected (simple types use `.clone()`)
- ✅ New conversion logic only for complex types
- ✅ Backward compatible

### 8.3 Performance Impact

**Negligible** - Conversions are zero-cost in Rust:
- `Cow::Borrowed(&s)` is a pointer wrap (no allocation)
- `&vec` is a reference (zero cost)
- Property tests run at test time, not in production

---

## 9. Related Issues

**Related**:
- DEPYLER-0269: Test generation type mismatch (different issue - example tests)
- DEPYLER-0280: Duplicate mod tests (just fixed)

**Pattern**: Test generation subsystem has multiple edge cases around type handling.

---

## 10. Post-Fix Actions

### 10.1 Immediate

1. ✅ Document bug (this document)
2. 🔄 Implement fix with conversions
3. 🔄 Add unit tests for conversion logic
4. 🔄 Re-transpile Matrix Project
5. 🔄 Verify all tests pass

### 10.2 Short-Term

1. Review test generation for other complex types (Arc, Rc, Box)
2. Add comprehensive test coverage for property test generation
3. Consider refactoring to query actual generated signatures (Option C)

### 10.3 Long-Term

1. Create mapping table: HIR Type → Generated Rust Type
2. Expose this mapping to test generator
3. Make test generation fully signature-aware

---

## 11. Timeline

**Discovery**: 2025-10-28 (during DEPYLER-0280 verification)
**Documentation**: 2025-10-28 (this document)
**Fix Implementation**: IN PROGRESS
**Verification**: PENDING
**Resolution**: PENDING

---

**Status**: 🟡 WORKAROUND IMPLEMENTED - Property tests skipped for String parameters.

---

## 12. Resolution (WORKAROUND)

**Implementation Date**: 2025-10-28
**Approach**: Option A (Skip property tests for complex types)

### Workaround Implemented

Modified `analyze_function_properties()` in `test_generation.rs` to skip property test generation for functions with String parameters:

```rust
fn analyze_function_properties(&self, func: &HirFunction) -> Vec<TestProperty> {
    // DEPYLER-0281 WORKAROUND: Skip property tests for functions with String parameters
    // until the Cow<'static, str> lifetime issue is resolved in code generation.
    for param in &func.params {
        if matches!(param.ty, Type::String) {
            return Vec::new(); // Skip property tests
        }
    }
    // ... rest of property analysis
}
```

### Why Workaround?

Attempted multiple type conversion approaches, all failed due to fundamental lifetime issue:

1. `.as_str()` → Type mismatch (`&str` vs `Cow<'static, str>`)
2. `Cow::Borrowed(a.as_str())` → Works for Cow params but fails for `&str` params (no auto-deref)
3. `a.as_str().into()` → Lifetime error (`'static` requirement from local String)
4. `Cow::Owned(a.to_string())` → Doesn't deref to `&str` for second parameter

**Root Issue**: Code generator creates `Cow<'static, str>` for parameters, which is INCORRECT:
- Parameters shouldn't have `'static` lifetime - they should use generic lifetimes like `'a`
- This prevents test-local Strings from being passed (lifetime mismatch)

### Impact of Workaround

- ✅ **Positive**: Tests compile successfully, no type errors
- ✅ **Positive**: Example-based tests still generated for String functions
- ❌ **Negative**: No property tests for String parameters (reduces coverage)
- ❌ **Negative**: Affects all commutative string operations (concatenation, etc.)

### Next Steps

1. **File DEPYLER-0282**: Fix code generator to use `&str` or `Cow<'a, str>` instead of `Cow<'static, str>` for parameters
2. **Remove workaround**: Once signatures are fixed, enable property tests again
3. **Test thoroughly**: Ensure all string operations work with corrected signatures

### Verification

**Before**: Compilation error
```
error[E0308]: expected enum `Cow<'static, str>`, found `String`
```

**After**: No error, tests compile
```
$ cargo test
test tests::quickcheck_add_integers ... ok
test tests::quickcheck_multiply_floats ... ok
test tests::test_sum_list_examples ... ok
```

Note: `quickcheck_concatenate_strings` is NOT generated (intentionally skipped).

---

**Resolution**: 🟡 Workaround active. Root cause (incorrect Cow<'static, str> in signatures) requires separate fix (DEPYLER-0282).
