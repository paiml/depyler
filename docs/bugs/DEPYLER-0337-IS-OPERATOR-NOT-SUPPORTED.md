# DEPYLER-0337: Python `is` Operator Not Supported in Transpiler

**Status**: 🔴 BLOCKING
**Priority**: P0 (CRITICAL)
**Severity**: High
**Created**: 2025-11-05
**Component**: Transpiler Core / AST Bridge
**Affects**: All Python code using `is` operator for identity comparison

---

## Problem Statement

The transpiler fails with error `'is' operator not yet supported (use == for value comparison)` when encountering Python's `is` operator. This is a fundamental Python operator used for identity comparison (checking if two references point to the same object), distinct from `==` which checks value equality.

### Error Message
```
Error: 'is' operator not yet supported (use == for value comparison)

Stack backtrace:
   at ./crates/depyler-core/src/ast_bridge.rs:1166:27
```

### Reproduction

**Python Code**:
```python
def test_boolean_check():
    assert config.has_section('existing') is True
    assert config.has_option('section', 'opt') is False
    assert value is None
```

**Current Behavior**: Transpilation fails with error
**Expected Behavior**: Should transpile to Rust equivalent

---

## Root Cause Analysis

**Location**: `crates/depyler-core/src/ast_bridge.rs:1166`
**Function**: `convert_cmpop`

The `is` operator is recognized by the Python AST parser but not handled in the comparison operator converter. The code explicitly returns an error instead of implementing the translation.

**Why This Exists**:
The transpiler's comparison operator handler doesn't have a mapping for Python's `is` operator to Rust. The error message incorrectly suggests using `==`, but `is` and `==` have different semantics in Python.

---

## Solution Design

### Python `is` vs Rust Equivalents

| Python Pattern | Semantic Meaning | Rust Equivalent |
|----------------|------------------|-----------------|
| `x is None` | Check if x is None | `x.is_none()` or `matches!(x, None)` |
| `x is not None` | Check if x is not None | `x.is_some()` or `!matches!(x, None)` |
| `x is True` | Check if x is True singleton | `x == true` (after bool conversion) |
| `x is False` | Check if x is False singleton | `x == false` (after bool conversion) |
| `x is y` | Check object identity | Complex - depends on type |

### Implementation Strategy

1. **Add `Is` and `IsNot` to comparison operator mapping**
2. **Special-case `None` comparisons** → translate to `Option` methods
3. **Special-case `True`/`False` comparisons** → translate to boolean equality
4. **General object identity** → use reference equality for supported types

### Rust Translation Rules

```python
# Pattern 1: is None
if x is None:        → if x.is_none() {

# Pattern 2: is not None
if x is not None:    → if x.is_some() {

# Pattern 3: is True/False (in assertions)
assert x is True     → assert_eq!(x, true);
assert x is False    → assert_eq!(x, false);

# Pattern 4: General identity (complex)
if obj1 is obj2:     → if std::ptr::eq(obj1, obj2) {  // for Rc/Arc
```

---

## Implementation Plan (EXTREME TDD)

### Phase 1: RED - Add Failing Tests

Create `crates/depyler-core/tests/test_is_operator.rs`:

```rust
#[test]
fn test_is_none_comparison() {
    let python = "x is None";
    let result = transpile(python);
    assert!(result.contains("x.is_none()"));
}

#[test]
fn test_is_not_none_comparison() {
    let python = "x is not None";
    let result = transpile(python);
    assert!(result.contains("x.is_some()"));
}

#[test]
fn test_is_true_comparison() {
    let python = "assert x is True";
    let result = transpile(python);
    assert!(result.contains("== true"));
}

#[test]
fn test_is_false_comparison() {
    let python = "assert x is False";
    let result = transpile(python);
    assert!(result.contains("== false"));
}
```

### Phase 2: GREEN - Implement Minimal Solution

**File**: `crates/depyler-core/src/ast_bridge.rs`

```rust
fn convert_cmpop(op: &CmpOp) -> Result<String> {
    match op {
        CmpOp::Eq => Ok("==".to_string()),
        CmpOp::NotEq => Ok("!=".to_string()),
        CmpOp::Lt => Ok("<".to_string()),
        CmpOp::LtE => Ok("<=".to_string()),
        CmpOp::Gt => Ok(">".to_string()),
        CmpOp::GtE => Ok(">=".to_string()),
        CmpOp::Is => Ok("IS".to_string()),      // Marker for special handling
        CmpOp::IsNot => Ok("IS_NOT".to_string()), // Marker for special handling
        CmpOp::In => Ok("IN".to_string()),
        CmpOp::NotIn => Ok("NOT_IN".to_string()),
    }
}

// In convert_compare function
fn convert_compare(expr: &Expr) -> Result<RuchyExpr> {
    // ... existing code ...

    // Special handling for Is/IsNot with None
    if op_str == "IS" {
        if is_none_literal(right) {
            return Ok(method_call(left_expr, "is_none", vec![]));
        }
        if is_true_literal(right) {
            return Ok(binary_op(left_expr, "==", bool_literal(true)));
        }
        if is_false_literal(right) {
            return Ok(binary_op(left_expr, "==", bool_literal(false)));
        }
    }

    if op_str == "IS_NOT" {
        if is_none_literal(right) {
            return Ok(method_call(left_expr, "is_some", vec![]));
        }
        if is_true_literal(right) {
            return Ok(binary_op(left_expr, "!=", bool_literal(true)));
        }
        if is_false_literal(right) {
            return Ok(binary_op(left_expr, "!=", bool_literal(false)));
        }
    }

    // ... rest of logic ...
}
```

### Phase 3: REFACTOR - Optimize and Document

1. Extract helper functions: `is_none_literal()`, `is_true_literal()`, `is_false_literal()`
2. Add comprehensive documentation
3. Ensure cyclomatic complexity ≤ 10
4. Add property tests for edge cases
5. Update user documentation

---

## Test Plan

### Unit Tests (Minimum 5)
1. ✅ `is None` comparison
2. ✅ `is not None` comparison
3. ✅ `is True` comparison
4. ✅ `is False` comparison
5. ✅ General object `is` comparison (error or limitation documented)

### Integration Tests (Minimum 3)
1. ✅ configparser test suite with `is True`/`is False`
2. ✅ Option type handling with `is None`
3. ✅ Nested conditions with `is`/`is not`

### Property Tests (Minimum 1)
1. ✅ Any `x is None` always translates to `.is_none()`

---

## Validation Criteria

### Code Quality
- [ ] Cyclomatic Complexity ≤ 10
- [ ] Cognitive Complexity ≤ 10
- [ ] Test Coverage ≥ 85%
- [ ] No TODO/FIXME/HACK comments
- [ ] All clippy warnings resolved

### Functional
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] configparser test suite transpiles successfully
- [ ] Generated Rust compiles with `--deny warnings`

---

## Impact Assessment

**Affected Components**:
- AST Bridge (crates/depyler-core/src/ast_bridge.rs)
- Comparison operator converter

**Downstream Impact**:
- **Immediate**: configparser test suite can transpile
- **Short-term**: All stdlib tests using `is` operator
- **Long-term**: Any Python code using identity comparison

**Risk Level**: Low (isolated change to comparison operator handling)

---

## Related Issues

- None (first occurrence)

---

## Resolution Timeline

- **Filed**: 2025-11-05
- **Target Fix**: 2025-11-05 (P0 - same day)
- **Actual Fix**: TBD
- **Verified**: TBD

---

## Lessons Learned

1. **Operator completeness**: Need comprehensive operator support matrix
2. **Testing gap**: Should have transpiler unit tests for all Python operators
3. **Documentation**: Operator mapping should be documented in specification

---

## References

- Python `is` operator docs: https://docs.python.org/3/reference/expressions.html#is
- Rust equality comparison: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
- Rust Option methods: https://doc.rust-lang.org/std/option/enum.Option.html

---

**Assigned To**: TBD
**Reviewed By**: TBD
**Merged In**: TBD
