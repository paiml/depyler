# DEPYLER-0503: Constant in type annotation error

## Problem Statement
Transpilation fails with error: `Unsupported type annotation: Constant(ExprConstant { value: Int(0) })`

**Minimal Reproducer**:
```python
items = [1, 2, 3]
first = items[0]
```

**Error**: TypeExtractor::extract_type() is being called on the constant `0`, which is NOT a type annotation.

## Five-Whys Root Cause Analysis

**1. Why does transpilation fail?**
- TypeExtractor::extract_type() is called on `Constant(Int(0))` which is not a type annotation

**2. Why is TypeExtractor called on a constant?**
- try_convert_type_alias() treats `items[0]` as a generic type alias (like `Optional[int]`)
- It calls TypeExtractor::extract_type() on the entire subscript expression

**3. Why does try_convert_type_alias() treat `items[0]` as a type alias?**
- Line 393 in ast_bridge.rs matches on `ast::Expr::Subscript(_)` without checking the base
- ALL subscript expressions are treated as potential type aliases

**4. Why doesn't it distinguish between type subscripts and value subscripts?**
- The pattern match only checks `ast::Expr::Subscript(_)` (wildcard)
- No validation that the subscript base is actually a type name (e.g., `Optional`, `List`, `Dict`)

**5. ROOT CAUSE: Overly broad pattern matching in try_convert_type_alias()**
- Pattern: `ast::Expr::Subscript(_) => (TypeExtractor::extract_type(&assign.value)?, false)`
- Matches both:
  - Type subscripts: `Optional[int]`, `List[str]` (VALID type aliases)
  - Value subscripts: `items[0]`, `data["key"]` (NOT type aliases, regular indexing)

**Architecture Issue**: No distinction between syntactically identical but semantically different subscript operations.

## Solution

Add validation in try_convert_type_alias() to check if the subscript base is a known type name before treating it as a type alias.

**Fix Location**: `crates/depyler-core/src/ast_bridge.rs:393`

**Before** (BUGGY):
```rust
// Generic alias: UserId = Optional[int]
ast::Expr::Subscript(_) => (TypeExtractor::extract_type(&assign.value)?, false),
```

**After** (FIXED):
```rust
// Generic alias: UserId = Optional[int]
// DEPYLER-0503: Only treat subscripts with type base (Optional, List, etc.) as type aliases
// Regular value subscripts like items[0] should return None (not a type alias)
ast::Expr::Subscript(s) => {
    // Check if the base is a type name
    if let ast::Expr::Name(base_name) = s.value.as_ref() {
        if self.is_type_name(base_name.id.as_str()) {
            (TypeExtractor::extract_type(&assign.value)?, false)
        } else {
            return Ok(None); // Base is variable, not a type - not a type alias
        }
    } else {
        return Ok(None); // Complex base expression - not a type alias
    }
}
```

**Validation**:
- `Optional[int]` - base is `Optional` (type name) → extract type ✅
- `List[str]` - base is `List` (type name) → extract type ✅
- `items[0]` - base is `items` (variable) → return None ✅
- `data["key"]` - base is `data` (variable) → return None ✅

## Test Plan

**RED Phase Tests** (crates/depyler-core/tests/depyler_0503_dict_subscript.rs):
1. `test_list_subscript_with_int()` - Array indexing with integer key
2. `test_dict_with_int_keys()` - Dict literal with integer keys
3. `test_nested_dict_structure()` - Nested dict/list structures

**Expected**: All tests should transpile without "unsupported type annotation" errors.

## Related Context

**Discovery**: Systematic bug discovery via `discover_bug_test.rs` testing real examples
**Example File**: `examples/ast_converters_demo.py` exposed this bug
**Impact**: Blocks any Python code using subscript indexing on variables (extremely common pattern)

## Commit Structure

Following TDD RED → GREEN → REFACTOR:

1. **RED**: Tests created (already done in depyler_0503_dict_subscript.rs)
2. **GREEN**: Fix try_convert_type_alias() to validate subscript base (this commit)
3. **REFACTOR**: Quality gates (complexity, coverage, clippy)

## References

- **Stack Trace**: try_convert_type_alias (ast_bridge.rs:393) → extract_type → extract_type_params → error
- **Pattern**: Similar to DEPYLER-0501 (type extraction on non-type expressions)
- **Architecture**: Type alias detection heuristics need semantic validation, not just syntactic pattern matching
