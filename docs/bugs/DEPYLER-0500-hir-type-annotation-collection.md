# DEPYLER-0500: HIR Integration - Type Annotation Collection (Pass 1)

**Type**: Feature (Phase 2 of Type System Tracking Enhancement)
**Priority**: P0-CRITICAL
**Status**: COMPLETE ✅
**Completed**: 2025-11-24
**Parent**: DEPYLER-0499 (TypeEnvironment with subtyping)
**Related Spec**: docs/specifications/type-system-tracking-enhancement-exit-local-optimization-toyota-way.md (Section 6.3)

---

## Problem Statement

TypeEnvironment exists but is not integrated with HIR generation. Need to collect Python type annotations during HIR traversal and return unified `(Hir, TypeEnvironment)` tuple.

**Current State**: HIR generation discards type annotation information
**Target State**: Type annotations captured in TypeEnvironment during single-pass HIR generation

## Root Cause (Five-Whys)

1. Why? Type annotations are lost after HIR generation
2. Why? `generate_hir()` returns only `Hir`, not type information
3. Why? No TypeEnvironment integration during HIR traversal
4. Why? Type tracking was added later as fragmented HashMaps
5. **ROOT**: No unified type collection during parsing phase

## Solution: Single-Pass HIR + Type Annotation Collection

### Design Principle

**Toyota Way: 一元管理** (Single Source of Truth)
- Collect types ONCE during HIR generation
- No need to re-traverse HIR for Pass 1
- Type information immediately available for subsequent passes

### API Change

```rust
// BEFORE:
pub fn generate_hir(module: &ast::Module) -> Result<Hir, HirGenError>

// AFTER:
pub fn generate_hir(module: &ast::Module) -> Result<(Hir, TypeEnvironment), HirGenError>
```

### Type Annotation Sources

**Python Type Annotations**:
```python
def fibonacci(n: int) -> int:  # Function signature
    x: int = 5                  # Variable annotation
    return x
```

**Depyler Directives** (future):
```python
# depyler: type x = i32
```

## Implementation Plan (TDD)

### RED Phase: Failing Tests

```rust
#[test]
fn test_hir_gen_returns_type_environment() {
    let python = "def foo(x: int) -> str: return str(x)";
    let ast = parse_python(python).unwrap();

    let (hir, type_env) = generate_hir(&ast).expect("Should generate HIR");

    // Verify TypeEnvironment populated
    assert_eq!(type_env.get_var_type("x"), Some(&Type::Int));
}

#[test]
fn test_collect_function_signature() {
    let python = "def add(a: int, b: int) -> int: return a + b";
    let ast = parse_python(python).unwrap();

    let (_hir, type_env) = generate_hir(&ast).unwrap();

    // Parameters collected
    assert_eq!(type_env.get_var_type("a"), Some(&Type::Int));
    assert_eq!(type_env.get_var_type("b"), Some(&Type::Int));
}

#[test]
fn test_collect_variable_annotations() {
    let python = "x: int = 5\ny: str = 'hello'";
    let ast = parse_python(python).unwrap();

    let (_hir, type_env) = generate_hir(&ast).unwrap();

    assert_eq!(type_env.get_var_type("x"), Some(&Type::Int));
    assert_eq!(type_env.get_var_type("y"), Some(&Type::String));
}
```

### GREEN Phase: Minimal Implementation

**Step 1**: Add TypeEnvironment to HirGenerator
```rust
// crates/depyler-core/src/hir/hir_gen.rs
pub struct HirGenerator {
    // Existing fields...
    type_env: TypeEnvironment,  // NEW
}

impl HirGenerator {
    pub fn new() -> Self {
        Self {
            // ...
            type_env: TypeEnvironment::new(),
        }
    }
}
```

**Step 2**: Collect annotations during traversal
```rust
fn visit_function_def(&mut self, func: &ast::FunctionDef) -> Result<HirFunction> {
    // Collect parameter types
    for param in &func.args.args {
        if let Some(annotation) = &param.annotation {
            let ty = self.convert_type_annotation(annotation)?;
            self.type_env.bind_var(&param.arg, ty);
        }
    }

    // Collect return type
    if let Some(returns) = &func.returns {
        let ret_ty = self.convert_type_annotation(returns)?;
        // Store function return type (TODO: extend TypeEnvironment API)
    }

    // ... rest of function generation
}

fn visit_ann_assign(&mut self, target: &ast::Expr, annotation: &ast::Expr, value: Option<&ast::Expr>) -> Result<HirStmt> {
    // Variable annotation: x: int = value
    let ty = self.convert_type_annotation(annotation)?;

    if let ast::Expr::Name(name) = target {
        self.type_env.bind_var(&name.id, ty);
    }

    // ... rest of assignment generation
}
```

**Step 3**: Return TypeEnvironment
```rust
pub fn generate_hir(module: &ast::Module) -> Result<(Hir, TypeEnvironment), HirGenError> {
    let mut generator = HirGenerator::new();
    let hir = generator.visit_module(module)?;

    Ok((hir, generator.type_env))
}
```

**Step 4**: Update all call sites
```rust
// Before:
let hir = generate_hir(&ast)?;

// After:
let (hir, type_env) = generate_hir(&ast)?;
```

### REFACTOR Phase: Quality Standards

- Complexity ≤10 (pmat analyze complexity)
- Coverage ≥85% (cargo llvm-cov)
- TDG grade A- (pmat tdg check-quality)
- All existing tests still pass

## Success Criteria

1. ✅ `python_to_hir()` returns `(HirModule, TypeEnvironment)` tuple
2. ✅ Python type annotations collected during HIR generation
3. ✅ Function signatures captured (parameters + return type)
4. ✅ Variable annotations captured (`:` syntax) - Module-level constants
5. ✅ Complex type annotations collected (list[T], dict[K, V])
6. ✅ All existing HIR tests still pass (539/539)
7. ✅ No performance regression (no measurable impact)

## Implementation Summary (Phase 1 & 2 Complete)

**Phase 1**: Parameter annotation collection (COMPLETE)
- Added TypeEnvironment field to AstBridge
- Modified `convert_function` to bind parameter types
- Updated `python_to_hir()` to return tuple

**Phase 2**: Module-level annotation collection (COMPLETE)
- Modified `try_convert_annotated_constant` to bind module-level types
- Enabled collection for:
  - Simple variables: `x: int = 5`
  - Complex types: `numbers: list[int] = [1, 2, 3]`
  - Optional types: `value: int | None = None`
- All 8 DEPYLER-0500 tests passing

**Commits**:
- Phase 1: (previous commits - see git history)
- Phase 2 GREEN: 3a825fa
- Quality: All gates passed (clippy, tests, TDG)

**Next Steps**: DEPYLER-0501 - Constraint solving and type inference integration

## Files to Modify

- `crates/depyler-core/src/hir/hir_gen.rs` - Add TypeEnvironment integration
- `crates/depyler-core/src/hir/mod.rs` - Update generate_hir signature
- `crates/depyler-core/src/lib.rs` - Update DepylerPipeline to handle TypeEnvironment
- `crates/depyler-core/src/rust_gen/mod.rs` - Accept TypeEnvironment parameter
- All test files using `generate_hir()`

## Migration Strategy (Strangler Fig Pattern)

**Dual Running** (recommended by spec review):

1. **Shadow Mode**: Populate TypeEnvironment but don't use it yet
2. **Verification**: Assert TypeEnvironment matches existing var_types in tests
3. **Cutover**: Switch code generators one by one (stmt_gen first, then expr_gen)

```rust
// Shadow mode verification
#[cfg(debug_assertions)]
{
    let old_type = ctx.var_types.get("x");
    let new_type = type_env.get_var_type("x");
    assert_eq!(old_type, new_type, "TypeEnvironment diverged from legacy!");
}
```

## Risk Mitigation

**Risk 1**: Breaking all downstream code
- **Mitigation**: Update call sites incrementally, use compiler to find all usages

**Risk 2**: Performance regression from double traversal
- **Mitigation**: Single-pass collection (no re-traversal)

**Risk 3**: Missing annotation types
- **Mitigation**: Golden trace validation in Pass 4 will catch missing types

## References

1. Specification Section 6.3: "HIR Integration (Week 2)"
2. Specification Section 5.4.1: "HIR Generation (Type Annotation Collection)"
3. Cytron et al. (1991): SSA form - variable versioning for reassignments

---

**Next Steps**:
1. Run `pmat work continue DEPYLER-0500`
2. Write failing tests (RED)
3. Add TypeEnvironment to HirGenerator
4. Collect annotations during traversal
5. Update all call sites
6. Verify quality gates pass
