# DEPYLER-0319: Numeric Type Conversion in Aggregations

**Date Created**: 2025-10-31
**Status**: 📋 ANALYSIS - Ready for Implementation
**Priority**: P3 - Low (edge case, workaround exists)
**Estimate**: 1-2 hours
**Related**: DEPYLER-0304 (discovered during 09_dictionary_operations validation)

## Problem Statement

When aggregating (sum, mean, etc.) integer iterators into float results, transpiler doesn't insert necessary type conversion, causing compilation errors.

**Discovery Context**: Found during DEPYLER-0304 campaign when validating `average_values()` in 09_dictionary_operations.

## Example

**Python**:
```python
def average_values(d: dict[str, int]) -> float:
    """Average of all values."""
    if len(d) == 0:
        return 0.0
    return sum(d.values()) / len(d)  # sum returns int, division converts to float
```

**Generated Rust** (WRONG):
```rust
pub fn average_values(d: &HashMap<String, i32>) -> Result<f64, ZeroDivisionError> {
    let _cse_temp_0 = d.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0.0);
    }
    Ok((d.values().cloned().sum::<f64>() as f64) / (d.len() as i32 as f64))
    //                        ^^^^^^^^-- ❌ Cannot sum i32 iterator as f64
}
```

**Error**:
```
error[E0277]: a value of type `f64` cannot be made by summing an iterator over elements of type `i32`
    --> lib.rs:199:35
     |
 199 |     Ok((d.values().cloned().sum::<f64>() as f64) / (d.len() as i32 as f64))
     |                             ---   ^^^ value of type `f64` cannot be made by summing a `std::iter::Iterator<Item=i32>`
     |
     = help: the trait `Sum<i32>` is not implemented for `f64`
```

**Correct Rust**:
```rust
pub fn average_values(d: &HashMap<String, i32>) -> Result<f64, ZeroDivisionError> {
    let _cse_temp_0 = d.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(0.0);
    }
    // Need to map i32 to f64 BEFORE summing
    Ok((d.values().map(|&x| x as f64).sum::<f64>()) / (d.len() as f64))
    //            ^^^^^^^^^^^^^^^^^^-- ✅ Convert each element first
}
```

## Root Cause Analysis

### Current Behavior
Transpiler sees:
1. Return type is `f64`
2. Tries to generate `.sum::<f64>()`
3. Doesn't realize iterator elements are `i32`
4. Results in type mismatch

### Missing Logic
Need to detect iterator element type vs aggregation result type mismatch and insert conversion:

```rust
// Pattern detection:
iterator_element_type = i32
aggregation_result_type = f64
if iterator_element_type != aggregation_result_type {
    // Insert: .map(|x| x as aggregation_result_type)
}
```

## Implementation Strategy

### Approach 1: Type-Aware Aggregation (Recommended)

**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` (sum() translation)

**Steps**:
1. When translating `sum()` builtin:
   - Detect target type from context (return type, variable annotation, etc.)
   - Detect source iterator element type
   - If mismatch: insert `.map(|x| x as TargetType)` before `.sum()`

**Code Change** (approximate):
```rust
// In expr_gen.rs, builtin function handling
"sum" => {
    let iterator = &args[0];
    let iterator_elem_type = infer_iterator_element_type(iterator)?;
    let target_type = infer_target_type_from_context()?;  // From return type or variable
    
    if iterator_elem_type != target_type && needs_conversion(iterator_elem_type, target_type) {
        // Insert type conversion
        Ok(parse_quote! { 
            #iterator.map(|x| x as #target_type).sum::<#target_type>()
        })
    } else {
        // No conversion needed
        Ok(parse_quote! { #iterator.sum::<#target_type>() })
    }
}
```

### Approach 2: Explicit Type Annotation (Simpler, Less General)

Just detect `sum(int_iterator)` in float context and always map to f64:

```rust
if target_type == f64 && iterator_elem_type.is_integer() {
    Ok(parse_quote! { #iterator.map(|x| x as f64).sum::<f64>() })
}
```

## Affected Operations

### Aggregations
- ✅ `sum()` with type conversion (i32 → f64, i64 → f64, etc.)
- ❓ `mean()` (if we add this)
- ❓ `std()`, `variance()` (future statistical functions)

### Other Potential Cases
- ⚠️ Mixed arithmetic: `int + float` (may already be handled)
- ⚠️ Comparisons: `int < float` (probably works via coercion)

## Testing Strategy

### Test Cases

**Test 1**: Average of integers
```python
def test_average(values: list[int]) -> float:
    return sum(values) / len(values)
```

**Test 2**: Sum with explicit float return
```python
def test_sum_float(values: list[int]) -> float:
    return sum(values)  # Implicitly converts to float
```

**Test 3**: Mixed types (edge case)
```python
def test_mixed(ints: list[int], floats: list[float]) -> float:
    return sum(ints) + sum(floats)
```

### Success Criteria

✅ **All test cases compile** without E0277 errors
✅ **Generated code includes `.map(|x| x as f64)`** when needed
✅ **Zero regressions** in existing tests
✅ **09_dictionary_operations** compiles with 1 fewer error (from 4 to 3)

## Implementation Checklist

- [ ] Implement iterator element type inference
- [ ] Implement target type detection from context
- [ ] Add type conversion insertion logic
- [ ] Test with sum() → f64 case
- [ ] Test with sum() → i32 case (no conversion)
- [ ] Verify 09_dictionary_operations compiles
- [ ] Run full test suite
- [ ] Update CHANGELOG.md
- [ ] Commit with detailed message

## Priority Justification

**P3 - Low** because:
- ⚠️ Edge case (most aggregations stay in same type)
- ✅ Workaround exists (manually specify type or use intermediate variable)
- ⚠️ Only affects statistical/math-heavy code
- ⚠️ Not blocking critical functionality

## Estimated Impact

**After Fix**:
- 09_dictionary_operations: 4 errors → 3 errors (25% reduction)
- Enables correct transpilation of averaging/statistical functions
- Improves Python → Rust idiom compatibility

---
**Status**: Ready for implementation
**Next Step**: Implement type inference for iterator elements and aggregation targets
