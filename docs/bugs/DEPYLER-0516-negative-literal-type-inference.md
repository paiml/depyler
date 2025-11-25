# DEPYLER-0516: Negative Literal Type Inference Bug (E0308)

**Status**: IN PROGRESS
**Priority**: P0-CRITICAL (47% of verificar corpus failures)
**Assigned**: Claude
**Created**: 2025-11-25
**Source**: Verificar Corpus Testing

## Problem Statement

Negative integer literals and parenthesized expressions default to `serde_json::Value` type instead of concrete integer types like `i32`, causing E0308 type mismatch errors.

**Impact**: 15/32 failures (47%) in verificar corpus testing

### Current Behavior (BROKEN)

**Python**:
```python
x = -1
```

**Generated Rust** (INCORRECT):
```rust
pub const x: serde_json::Value = -1;
```

**Compilation Error**:
```
error[E0308]: mismatched types
 --> test.rs:1:38
  |
1 | pub const x: serde_json::Value = -1;
  |                                  ^^ expected `serde_json::Value`, found integer
  |
  = note: expected enum `serde_json::Value`
             found type `{integer}`
```

### Expected Behavior (CORRECT)

**Generated Rust** (CORRECT):
```rust
pub const x: i32 = -1;
```

## Root Cause Analysis

### Hypothesis 1: Type Inference Defaults to serde_json::Value

The type inference system appears to:
1. Encounter untyped literal: `-1`
2. Default to most general type: `serde_json::Value`
3. Fails to recognize it's a concrete integer literal

**Location**: `crates/depyler-core/src/type_system/` or `crates/depyler-core/src/rust_gen/expr_gen.rs`

### Hypothesis 2: Unary Negation Not Handled

Unary `-` operator may not be considered during type inference:
1. Positive literals work: `x = 1` → `i32`
2. Negative literals fail: `x = -1` → `serde_json::Value`
3. Parenthesized fail too: `x = (-1)` → `serde_json::Value`

### Hypothesis 3: Constant vs Variable Inference

Module-level constants may use different type inference than function-local variables.

## Test Cases (from Verificar Corpus)

All these **FAIL** with E0308:

1. `x = -1` → `pub const x: serde_json::Value = -1;`
2. `x = -2` → `pub const x: serde_json::Value = -2;`
3. `x = -10` → `pub const x: serde_json::Value = -10;`
4. `x = (-0)` → `pub const x: serde_json::Value = (-0);`
5. `x = (-1)` → `pub const x: serde_json::Value = (-1);`
6. `x = (--1)` → `pub const x: serde_json::Value = (--1);`
7. `x = None` → `pub const x: serde_json::Value = None;` (related issue)

All these **PASS**:

1. `x = 0` → `pub const x: i32 = 0;` ✅
2. `x = 1` → `pub const x: i32 = 1;` ✅
3. `x = 2` → `pub const x: i32 = 2;` ✅
4. `x = 10` → `pub const x: i32 = 10;` ✅
5. `x = True` → `pub const x: bool = true;` ✅

## Impact

**Severity**: P0-CRITICAL - Blocks 47% of verificar corpus

**Affected Code**:
- ✅ 15/32 verificar corpus failures
- ✅ Module-level constant assignments
- ✅ Negative literals in any context
- ✅ Parenthesized expressions with negation

**Scope**:
- This is a fundamental type inference bug
- Affects all Python code with negative literals
- Likely impacts reprorusted examples too

## Test Plan

### Phase 1: RED - Failing Tests

Create test file `crates/depyler-core/tests/depyler_0516_negative_literal_type.rs`:

```rust
#[test]
fn test_negative_integer_literal() {
    let python = "x = -1";
    let rust = transpile(python);

    // Should generate: pub const x: i32 = -1;
    assert!(rust.contains("i32"));
    assert!(!rust.contains("serde_json::Value"));

    // Should compile
    assert!(compiles(&rust));
}

#[test]
fn test_parenthesized_negative() {
    let python = "x = (-1)";
    let rust = transpile(python);

    assert!(rust.contains("i32"));
    assert!(compiles(&rust));
}

#[test]
fn test_double_negative() {
    let python = "x = (--1)";
    let rust = transpile(python);

    assert!(rust.contains("i32"));
    assert!(compiles(&rust));
}

#[test]
fn test_various_negatives() {
    for val in &["-1", "-2", "-10", "-100"] {
        let python = format!("x = {}", val);
        let rust = transpile(&python);

        assert!(rust.contains("i32"), "Failed for: {}", val);
        assert!(compiles(&rust), "Failed to compile: {}", val);
    }
}
```

### Phase 2: GREEN - Implementation

**Fix Location**: One or more of:
1. `type_system/inference.rs` - Add handling for UnaryOp::Neg
2. `rust_gen/expr_gen.rs` - Detect negative literals directly
3. `hir.rs` - Preserve type info through UnaryOp nodes

**Expected Changes**:
```rust
// In type inference
fn infer_unary_expr(&mut self, op: UnaryOp, operand: &HirExpr) -> Type {
    match op {
        UnaryOp::Neg | UnaryOp::Pos => {
            let operand_ty = self.infer_expr(operand);
            // Negation preserves numeric type
            match operand_ty {
                Type::Int | Type::Unknown => Type::Int,
                Type::Float => Type::Float,
                ty => ty  // Pass through
            }
        }
        // ... other ops
    }
}
```

### Phase 3: REFACTOR - Quality Gates

**Quality Checks**:
- ✅ All 4+ new tests pass
- ✅ No regressions in existing tests
- ✅ Complexity ≤10 (PMAT enforcement)
- ✅ No clippy warnings
- ✅ Test coverage ≥80%

**Verification**:
```bash
# Run new tests
cargo test depyler_0516_negative_literal_type

# Verify no regressions
cargo test --workspace

# Re-run verificar corpus
cd tests/verificar_corpus
python3 test_corpus_v2.py corpus_d3_c50.json

# Expected improvement: 36% → 60%+ pass rate
```

## Files to Modify

**Primary**:
- `crates/depyler-core/src/type_system/inference.rs` - Add UnaryOp::Neg handling
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Type generation for negatives

**Tests**:
- `crates/depyler-core/tests/depyler_0516_negative_literal_type.rs` (NEW)

**Verification**:
- `tests/verificar_corpus/corpus_d3_c50.json` - Re-run corpus

## Acceptance Criteria

- [ ] Create comprehensive failing tests (RED phase)
- [ ] `x = -1` generates `i32` not `serde_json::Value`
- [ ] All negative literals handled: -1, -2, -10, etc.
- [ ] Parenthesized negatives work: (-1), (--1)
- [ ] No regression in existing tests
- [ ] Test coverage ≥80%
- [ ] Complexity ≤10
- [ ] Verificar corpus: 36% → 60%+ pass rate

## Timeline

**Estimated Effort**: 2 hours
- Phase 1 (RED): 30 minutes
- Phase 2 (GREEN): 60 minutes
- Phase 3 (REFACTOR): 30 minutes

## Related Issues

- **Verificar Corpus Testing**: Identified this as #1 issue (47% of failures)
- **DEPYLER-0496**: Result binop error propagation (COMPLETED)
- **GH-70**: Nested function type inference (COMPLETED)

## Notes

This is a **STOP THE LINE** bug identified through systematic testing:
- Discovered via verificar corpus testing framework
- 15/32 failures tracked to this single root cause
- Fixing this will improve overall pass rate from 36% → ~60%
- Critical for reprorusted examples to compile

The verificar testing framework provides:
- ✅ Reproducible test cases
- ✅ Quantitative metrics
- ✅ Clear verification path
- ✅ No more thrashing

**Next Steps After Fix**:
1. Re-run verificar corpus (expect 36% → 60%+)
2. Test reprorusted examples (expect some to pass)
3. Move to next issue: E0425 undefined variables (31% of failures)
