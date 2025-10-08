# Transpiler Bug: Type Annotations Not Preserved

**Date Discovered**: 2025-10-08
**Severity**: HIGH (Type Safety / Correctness Issue)
**Status**: Documented, Not Fixed

## Summary

The transpiler does not preserve or use type annotations from Python annotated assignments (`AnnAssign`). This causes type mismatches when the annotation differs from the inferred type of the expression.

## Minimal Reproduction

**Python Input** (`examples/showcase/binary_search.py:6`):
```python
right: int = len(arr) - 1
```

**Expected Rust Output**:
```rust
let mut right: i32 = (arr.len() - 1) as i32;
// or
let mut right: i32 = (arr.len() as i32) - 1;
```

**Actual Rust Output** (INCORRECT):
```rust
let _cse_temp_0 = arr.len();  // usize
let mut right = _cse_temp_0 - 1;  // inferred as usize, should be i32
```

## Problems

1. **Type Annotation Ignored**: The `int` annotation is completely discarded
2. **Wrong Type Inferred**: `right` is inferred as `usize` instead of `i32`
3. **Compilation Errors**: Leads to "mismatched types" errors downstream
4. **Example**: `binary_search.rs:24` - "expected `i32`, found `usize`"

## Root Cause

**Location**: `crates/depyler-core/src/ast_bridge/converters.rs:56-64`

```rust
fn convert_ann_assign(a: ast::StmtAnnAssign) -> Result<HirStmt> {
    let target = extract_assign_target(&a.target)?;
    let value = if let Some(v) = a.value {
        super::convert_expr(*v)?
    } else {
        bail!("Annotated assignment without value not supported")
    };
    Ok(HirStmt::Assign { target, value })  // ❌ Annotation discarded!
}
```

The `a.annotation` field is never used. The function treats annotated assignments identically to regular assignments.

## Impact

**Severity**: HIGH - Causes compilation failures
**Examples Affected**:
- `examples/showcase/binary_search.py` ❌ (type error: usize vs i32)
- Any code with explicit type annotations that differ from inferred types

**Patterns Affected**:
- `x: int = len(...)` - len() returns usize, annotation says int
- `x: float = some_int_expr` - needs conversion
- `x: str = ...` - needs conversion from &str

## Fix Required

### 1. Update HIR (`crates/depyler-core/src/hir.rs`)

Add type annotation field to `HirStmt::Assign`:

```rust
pub enum HirStmt {
    Assign {
        target: AssignTarget,
        value: HirExpr,
        type_annotation: Option<Type>,  // NEW
    },
    // ... rest
}
```

### 2. Update AST Converter (`crates/depyler-core/src/ast_bridge/converters.rs`)

Extract and convert the type annotation:

```rust
fn convert_ann_assign(a: ast::StmtAnnAssign) -> Result<HirStmt> {
    let target = extract_assign_target(&a.target)?;
    let value = if let Some(v) = a.value {
        super::convert_expr(*v)?
    } else {
        bail!("Annotated assignment without value not supported")
    };

    // NEW: Convert type annotation
    let type_annotation = Some(convert_type_annotation(&a.annotation)?);

    Ok(HirStmt::Assign { target, value, type_annotation })
}

fn convert_type_annotation(ann: &ast::Expr) -> Result<Type> {
    match ann {
        ast::Expr::Name(n) if n.id == "int" => Ok(Type::I32),
        ast::Expr::Name(n) if n.id == "float" => Ok(Type::F64),
        ast::Expr::Name(n) if n.id == "str" => Ok(Type::String),
        // ... handle List[T], Dict[K,V], etc.
        _ => bail!("Unsupported type annotation: {ann:?}")
    }
}
```

### 3. Update Code Generator (`crates/depyler-core/src/rust_gen.rs`)

Use type annotation to insert conversions:

```rust
HirStmt::Assign { target, value, type_annotation } => {
    let value_tokens = self.expr_to_tokens(value)?;

    // NEW: Insert conversion if type annotation differs from value type
    let value_with_conversion = if let Some(target_type) = type_annotation {
        let value_type = self.infer_expr_type(value)?;
        if needs_conversion(value_type, target_type) {
            insert_conversion(value_tokens, value_type, target_type)
        } else {
            value_tokens
        }
    } else {
        value_tokens
    };

    // ... rest of assignment generation
}
```

### 4. Update Pattern Matches

All files that match on `HirStmt::Assign` need updating:
- `crates/depyler-core/src/borrowing_context.rs`
- `crates/depyler-core/src/codegen.rs`
- `crates/depyler-core/src/direct_rules.rs`
- `crates/depyler-core/src/lifetime_analysis.rs`
- `crates/depyler-core/src/optimizer.rs`
- `crates/depyler-analyzer/src/type_flow.rs`

Add the new field:
```rust
HirStmt::Assign { target, value, type_annotation } => {
    // Existing logic, potentially using type_annotation
}
```

## Estimated Effort

- **Complexity**: MEDIUM-HIGH
- **Files to Modify**: ~8 files
- **Lines Changed**: ~100-200 lines
- **Test Cases**: ~20 new tests
- **Time Estimate**: 4-8 hours

## Workaround

Manual type conversion in generated code:
```rust
// Find: let mut right = _cse_temp_0 - 1;
// Replace: let mut right: i32 = (_cse_temp_0 - 1) as i32;
```

**Status**: Not practical for automated transpilation

## Next Steps

1. Create ticket: DEPYLER-XXXX for type annotation support
2. Write test cases for different type conversions
3. Implement HIR changes with TDD
4. Update AST converter
5. Update code generator with conversion logic
6. Re-transpile all examples
7. Validate all examples compile cleanly

## Related Files

- `crates/depyler-core/src/hir.rs` (HIR definition)
- `crates/depyler-core/src/ast_bridge/converters.rs` (AST→HIR converter)
- `crates/depyler-core/src/rust_gen.rs` (code generator)
- `examples/showcase/binary_search.py` (affected example)
