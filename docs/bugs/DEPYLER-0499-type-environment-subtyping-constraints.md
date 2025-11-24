# DEPYLER-0499: TypeEnvironment with Subtyping Constraints

**Type**: Feature (Phase 1 of Type System Tracking Enhancement)
**Priority**: P0-CRITICAL
**Status**: IN PROGRESS
**Related Spec**: docs/specifications/type-system-tracking-enhancement-exit-local-optimization-toyota-way.md

---

## Problem Statement

Current type tracking uses 7 fragmented HashMaps with ad-hoc heuristics, creating O(exp) search space. Need unified TypeEnvironment with constraint-based inference.

**Critical Gap Identified in Spec Review**:
- Spec proposed Hindley-Milner unification (T1 = T2)
- Python requires subtyping constraints (T1 <: T2)
- Pure HM fails for numeric tower (i32 <: i64), container variance, Option lifting

## Root Cause (Five-Whys)

1. Why? Type errors require multiple commits
2. Why? Each fix is ad-hoc (heuristic-based)
3. Why? No constraint solver
4. Why? Type info fragmented across 7+ HashMaps
5. **ROOT**: O(exp) search space - ad-hoc HashMap lookups with unification instead of subtyping

## Solution: TypeEnvironment with Subtyping-Aware Constraints

### Core Innovation

**Replace**: Equality unification (T1 = T2)
**With**: Subtyping constraints (T1 <: T2) + worklist solver

### Constraint Types

```rust
pub enum ConstraintKind {
    Eq,                  // T1 == T2 (identity)
    Subtype,             // T1 <: T2 (NEW - subtyping relation)
    Supertype,           // T1 :> T2 (NEW - reverse subtyping)
    Callable,            // T1 callable with args → T2
    HasField(String),    // T1 has field with type T2
    Arithmetic,          // T1 and T2 support arithmetic
}
```

### Subtyping Rules for Python→Rust

**Numeric Tower**:
- `i32 <: i64 <: f64`
- `u32 <: u64`

**Container Variance**:
- `Vec<T>` is covariant: `Vec<T> <: Vec<U>` if `T <: U`
- `HashMap<K, V>` is covariant in V: `HashMap<K, T> <: HashMap<K, U>` if `T <: U`

**Option Lifting**:
- `T <: Option<T>` (can lift value into Some)
- `Option<T> <: Option<U>` if `T <: U`

**Result Lifting**:
- `T <: Result<T, E>` (can lift value into Ok)
- `Result<T, E> <: Result<U, E>` if `T <: U`

### Academic Foundation

**Bidirectional Type Checking** (Dunfield & Krishnaswami, 2013):
- Synthesis (⇒): Infer type from term
- Checking (⇐): Verify term against expected type
- Handles subtyping + higher-rank polymorphism

**SSA for Flow-Sensitivity** (Cytron et al., 1991):
- Variable reassignment creates new version: `x_0: i64`, `x_1: String`
- Not mutable type updates

## Implementation Plan (TDD)

### RED Phase: Failing Tests

```rust
#[test]
fn test_subtype_i32_to_i64() {
    let mut env = TypeEnvironment::new();

    // i32 <: i64
    let result = env.check_subtype(&Type::Int32, &Type::Int64);
    assert!(result.is_ok(), "i32 should be subtype of i64");
}

#[test]
fn test_subtype_not_reflexive() {
    let mut env = TypeEnvironment::new();

    // i64 NOT <: i32
    let result = env.check_subtype(&Type::Int64, &Type::Int32);
    assert!(result.is_err(), "i64 should NOT be subtype of i32");
}

#[test]
fn test_subtype_option_lift() {
    let mut env = TypeEnvironment::new();

    // i64 <: Option<i64>
    let result = env.check_subtype(&Type::Int64, &Type::Optional(Box::new(Type::Int64)));
    assert!(result.is_ok(), "T should be subtype of Option<T>");
}
```

### GREEN Phase: Minimal Implementation

Core data structures:
1. `TypeEnvironment` - central hub
2. `TypeConstraint` - constraint representation with Subtype kind
3. `SubtypeRelation` - encodes subtyping rules
4. `ConstraintSolver` - worklist algorithm (not unification)

### REFACTOR Phase: Quality Standards

- Complexity ≤10 (pmat analyze complexity)
- Coverage ≥85% (cargo llvm-cov)
- TDG grade A- (pmat tdg check-quality)

## Success Criteria

1. Unit tests pass (≥20 tests)
2. Subtyping rules correctly encode Python semantics
3. Worklist solver handles transitive subtyping (i32 <: i64 <: f64 → i32 <: f64)
4. SSA variable versioning (x_0, x_1) for flow-sensitive analysis
5. Quality gates pass

## Files to Create

- `crates/depyler-core/src/type_system/mod.rs` (new module)
- `crates/depyler-core/src/type_system/type_environment.rs`
- `crates/depyler-core/src/type_system/constraint.rs`
- `crates/depyler-core/src/type_system/subtyping.rs`
- `crates/depyler-core/src/type_system/solver.rs`
- `crates/depyler-core/tests/type_environment_tests.rs`

## References

1. Dunfield, J., & Krishnaswami, N. (2013). Complete and Easy Bidirectional Typechecking for Higher-Rank Polymorphism. ICFP.
2. Cytron, R., et al. (1991). Efficiently computing static single assignment form. ACM TOPLAS.
3. Pierce, B. C. (2002). Types and Programming Languages. MIT Press. (Chapter 15: Subtyping)

---

**Next Steps**:
1. Run `pmat work start DEPYLER-0499`
2. Create module structure
3. Write failing tests (RED)
4. Implement minimal solution (GREEN)
5. Refactor to quality standards
