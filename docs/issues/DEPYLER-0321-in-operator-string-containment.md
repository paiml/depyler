# DEPYLER-0321: `in` Operator Generates `.contains_key()` for Strings

**Date Created**: 2025-10-31
**Status**: 📋 ANALYSIS - Ready for Implementation
**Priority**: P1 - Critical (blocks all string containment operations)
**Estimate**: 30 minutes (extend DEPYLER-0304 Phase 2A fix)
**Related**: DEPYLER-0304 Phase 2A (same bug, different context), DEPYLER-0323

## Problem Statement

The `in` operator for strings generates `.contains_key()` (HashMap method) instead of `.contains()` (string method), causing compilation errors for all string containment checks.

**Discovery Context**: Found during 08_string_operations Matrix validation (2025-10-31).

## Examples

### Example 1: Simple String Containment

**Python**:
```python
def contains_substring(s: str, substring: str) -> bool:
    """Check if substring is in string."""
    return substring in s
```

**Generated Rust** (WRONG):
```rust
pub fn contains_substring(s: &str, substring: Cow<'_, str>) -> bool {
    s.contains_key(&substring)  // ❌ ERROR: no method `contains_key` for &str
}
```

**Error**:
```
error[E0599]: no method named `contains_key` found for reference `&str` in the current scope
  --> src/lib.rs:135:7
   |
135 |     s.contains_key(&substring)
   |       ^^^^^^^^^^^^ method not found in `&str`
```

**Correct Rust**:
```rust
pub fn contains_substring(s: &str, substring: &str) -> bool {
    s.contains(substring)  // ✅ Correct string method
}
```

### Example 2: Character in String (Loop Context)

**Python**:
```python
def count_vowels(s: str) -> int:
    """Count vowels in string."""
    count = 0
    for char in s:
        if char in "aeiouAEIOU":  # Check if char is vowel
            count += 1
    return count
```

**Generated Rust** (WRONG):
```rust
pub fn count_vowels(s: &str) -> i32 {
    let mut count = 0;
    for char in s.iter().cloned() {  // ❌ Also wrong, but different issue (DEPYLER-0323)
        if "aeiouAEIOU".contains_key(&char) {  // ❌ ERROR: .contains_key() for string
            count = count + 1;
        }
    }
    count
}
```

**Error**:
```
error[E0599]: no method named `contains_key` found for reference `&'static str` in the current scope
  --> src/lib.rs:281:25
   |
281 |         if "aeiouAEIOU".contains_key(&char) {
   |                         ^^^^^^^^^^^^ method not found in `&str`
```

**Correct Rust**:
```rust
pub fn count_vowels(s: &str) -> i32 {
    let mut count = 0;
    for char in s.chars() {  // ✅ Use .chars() (separate issue: DEPYLER-0323)
        if "aeiouAEIOU".contains(char) {  // ✅ Use .contains() for string
            count += 1;
        }
    }
    count
}
```

## Root Cause Analysis

### Current Behavior
In `expr_gen.rs` lines 123-165 (modified in DEPYLER-0304 Phase 2A):

```rust
// CURRENT CODE (from DEPYLER-0304 Phase 2A):
BinOp::In | BinOp::NotIn => {
    let is_negated = matches!(op, BinOp::NotIn);
    let left_expr = self.generate_expr(left, ctx)?;
    let right_expr = self.generate_expr(right, ctx)?;

    let is_string = matches!(
        self.infer_type(right, ctx),
        Some(RustType::String | RustType::Str)
    );
    let is_set = matches!(self.infer_type(right, ctx), Some(RustType::HashSet(_)));

    let contains_expr = if is_string || is_set {
        Ok(parse_quote! { #right_expr.contains(&#left_expr) })  // ✅ Works for Set
    } else {
        // HashMap/dict uses .contains_key()
        Ok(parse_quote! { #right_expr.contains_key(#left_expr) })  // ✅ Fixed in Phase 2A
    }?;

    // ...
}
```

**Problem**: The `is_string` check is present BUT the logic uses `.contains(&#left_expr)` with `&` prefix, which doesn't work correctly for all string cases.

### Why This Happens

1. **Phase 2A Fix Was Incomplete**: DEPYLER-0304 Phase 2A focused on HashMap double-borrowing but didn't fully test string containment
2. **Reference Handling**: For strings, `.contains()` takes `&str` or `char`, but the transpiler adds `&` prefix
3. **Type Detection**: `is_string` check works, but the generated code isn't quite right

### Correct Logic

```rust
// CORRECT FIX:
BinOp::In | BinOp::NotIn => {
    let is_negated = matches!(op, BinOp::NotIn);
    let left_expr = self.generate_expr(left, ctx)?;
    let right_expr = self.generate_expr(right, ctx)?;

    // Type-aware containment check
    let right_type = self.infer_type(right, ctx);
    let left_type = self.infer_type(left, ctx);

    let contains_expr = match right_type {
        Some(RustType::String | RustType::Str) => {
            // String containment: "hello".contains("ell") or "hello".contains('e')
            match left_type {
                Some(RustType::Char) => {
                    // char in string: no & needed
                    Ok(parse_quote! { #right_expr.contains(#left_expr) })
                }
                Some(RustType::String | RustType::Str) => {
                    // substring in string: needs & for str
                    Ok(parse_quote! { #right_expr.contains(&#left_expr) })
                }
                _ => Ok(parse_quote! { #right_expr.contains(&#left_expr) })
            }
        }
        Some(RustType::HashSet(_)) => {
            // Set containment: set.contains(&item)
            Ok(parse_quote! { #right_expr.contains(&#left_expr) })
        }
        Some(RustType::HashMap(_, _)) | Some(RustType::Dict(_, _)) => {
            // HashMap/dict containment: map.contains_key(key)
            // Use smart reference handling from Phase 2A
            Ok(parse_quote! { #right_expr.contains_key(#left_expr) })
        }
        _ => {
            // Default: assume collection with .contains()
            Ok(parse_quote! { #right_expr.contains(&#left_expr) })
        }
    }?;

    if is_negated {
        Ok(parse_quote! { !(#contains_expr) })
    } else {
        Ok(contains_expr)
    }
}
```

## Implementation Strategy

### Approach: Extend DEPYLER-0304 Phase 2A Fix

**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` (lines 123-165)

**Steps**:
1. Refactor `BinOp::In`/`BinOp::NotIn` handler to use comprehensive type detection
2. Add case for `RustType::String | RustType::Str` that emits `.contains()`
3. Keep HashMap case using `.contains_key()` (Phase 2A fix)
4. Keep Set case using `.contains()`
5. Handle char vs str distinction for left operand

**Code Change**:
```rust
// In expr_gen.rs, BinOp::In / BinOp::NotIn handler
BinOp::In | BinOp::NotIn => {
    let is_negated = matches!(op, BinOp::NotIn);
    let left_expr = self.generate_expr(left, ctx)?;
    let right_expr = self.generate_expr(right, ctx)?;

    // DEPYLER-0321: Type-aware containment method selection
    let right_type = self.infer_type(right, ctx);
    let contains_expr = match right_type {
        Some(RustType::String | RustType::Str) => {
            // String uses .contains() not .contains_key()
            parse_quote! { #right_expr.contains(&#left_expr) }
        }
        Some(RustType::HashSet(_)) => {
            // Set uses .contains()
            parse_quote! { #right_expr.contains(&#left_expr) }
        }
        Some(RustType::HashMap(_, _)) | Some(RustType::Dict(_, _)) => {
            // HashMap uses .contains_key() (DEPYLER-0304 Phase 2A)
            // Smart reference handling: no & prefix
            parse_quote! { #right_expr.contains_key(#left_expr) }
        }
        _ => {
            // Default: try .contains() for other collections
            parse_quote! { #right_expr.contains(&#left_expr) }
        }
    };

    if is_negated {
        Ok(parse_quote! { !(#contains_expr) })
    } else {
        Ok(contains_expr)
    }
}
```

## Testing Strategy

### Test Cases

**Test 1**: Substring containment
```python
def test_substring_in_string(s: str, sub: str) -> bool:
    return sub in s
```
Expected Rust: `s.contains(sub)` ✅

**Test 2**: Char in string
```python
def test_char_in_string(s: str, c: str) -> bool:
    return c in s  # c is single char
```
Expected Rust: `s.contains(c)` ✅

**Test 3**: Not in (negation)
```python
def test_not_in(s: str, sub: str) -> bool:
    return sub not in s
```
Expected Rust: `!(s.contains(sub))` ✅

**Test 4**: Loop with char check
```python
def test_loop_char_check(s: str) -> int:
    count = 0
    for char in s:
        if char in "aeiou":
            count += 1
    return count
```
Expected Rust: `"aeiou".contains(char)` ✅

### Success Criteria

✅ **08_string_operations compiles** with 2 fewer errors (20 → 18)
✅ **Generated code uses `.contains()`** for string types
✅ **Generated code uses `.contains_key()`** for HashMap types (preserved from Phase 2A)
✅ **Zero regressions** in existing tests (453/458 maintained)
✅ **09_dictionary_operations still compiles** with same error count (4 errors)

## Implementation Checklist

- [ ] Refactor BinOp::In/NotIn handler with type-aware dispatch
- [ ] Add test case for substring in string
- [ ] Add test case for char in string
- [ ] Add test case for string not in string
- [ ] Verify HashMap containment still works (DEPYLER-0304 regression check)
- [ ] Verify Set containment still works
- [ ] Retranspile 08_string_operations and verify compilation
- [ ] Run full test suite (ensure 453/458 pass rate)
- [ ] Update CHANGELOG.md
- [ ] Commit with detailed message

## Priority Justification

**P1 - Critical** because:
- ✅ Blocks ALL string containment operations (fundamental feature)
- ✅ Affects 08_string_operations (2 errors)
- ✅ Likely affects other Matrix examples with string operations
- ✅ Quick fix (30 min) - extends existing Phase 2A work
- ✅ Same pattern as DEPYLER-0304 Phase 2A (proven fix approach)

## Estimated Impact

**After Fix**:
- 08_string_operations: 20 errors → 18 errors (10% reduction)
- Enables proper string containment checks across ALL examples
- Completes the `.contains_key()` vs `.contains()` fix started in DEPYLER-0304

**Combined with DEPYLER-0323** (str.iter() → .chars()):
- 08_string_operations: 20 errors → 12 errors (40% reduction)
- Two quick fixes (1.5 hours total) for major impact

---
**Status**: Ready for implementation
**Next Step**: Refactor BinOp::In/NotIn handler with type-aware dispatch
