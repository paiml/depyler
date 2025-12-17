# 8. Architectural Strategy: Two-Phase Type Inference

[← Back to TOC](../pareto-complete-single-shot.md)

---

## Overview

Replace flow-insensitive type inference with **bidirectional constraint-based inference**.

---

## Current Architecture (Problem)

```
Python AST → HIR → Type Assignment (local) → Rust CodeGen
                         ↓
               serde_json::Value fallback
                         ↓
               E0308, E0599, E0277 cascades
```

**Failure Mode**: Each expression typed independently. No information flows between sites.

---

## Proposed Architecture (Solution)

```
                    ┌─────────────────────────────┐
                    │    ConstraintCollector      │
                    │  (Phase 1: Gather)          │
                    └─────────────────────────────┘
                              ↓
Python AST → HIR → Constraints → TypeUnifier → Resolved HIR → Rust
                              ↑        ↓
                    ┌─────────────────────────────┐
                    │    TypeEnvironment          │
                    │  (Phase 2: Solve)           │
                    └─────────────────────────────┘
```

---

## Phase 1: Constraint Collection

```rust
pub struct ConstraintCollector {
    constraints: Vec<TypeConstraint>,
}

enum TypeConstraint {
    Equal(TypeVar, TypeVar),           // T1 = T2
    HasMethod(TypeVar, String, TypeVar), // T1.method() -> T2
    Callable(TypeVar, Vec<TypeVar>, TypeVar), // T1(args) -> ret
    Iterable(TypeVar, TypeVar),        // for x in T1: x: T2
}
```

**Forward Pass**: Collect constraints from definitions
**Backward Pass**: Collect constraints from usage sites

---

## Phase 2: Constraint Solving

```rust
pub struct TypeUnifier {
    substitutions: HashMap<TypeVar, Type>,
}

impl TypeUnifier {
    fn unify(&mut self, c: TypeConstraint) -> Result<(), TypeError> {
        match c {
            Equal(t1, t2) => self.unify_types(t1, t2),
            HasMethod(obj, name, ret) => {
                let obj_type = self.resolve(obj);
                let method_ret = obj_type.method_return(&name)?;
                self.unify_types(ret, method_ret)
            }
            // ...
        }
    }
}
```

---

## Key Improvements

| Feature | Before | After |
|---------|--------|-------|
| **Forward inference** | Definition only | Definition + propagation |
| **Backward inference** | None | Usage-site constraints |
| **Call-site specialization** | None | Generic instantiation |
| **Error recovery** | `Value` fallback | Constraint relaxation |

---

## Expected Impact

| Error Class | Current % | Expected After |
|-------------|-----------|----------------|
| E0308 (type mismatch) | 22% | <5% |
| E0599 (method not found) | 7% | <2% |
| E0277 (trait not impl) | 11% | <3% |
| E0425 (scope error) | 27% | <10% |

**Net convergence**: 22% → 80%

---

## Implementation Strategy (Piecemeal)

1. **Step 1**: Per-function constraint collection (low risk)
2. **Step 2**: Module-level constraints (medium risk)
3. **Step 3**: Global TypeEnvironment (higher risk)

Each step is independently testable and reversible.
