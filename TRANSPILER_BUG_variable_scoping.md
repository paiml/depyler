# Transpiler Bug: Variable Scoping in Loops

**Date**: 2025-10-08
**Severity**: CRITICAL (Correctness Issue)
**Status**: Discovered, Not Fixed

## Summary

The transpiler incorrectly moves variable declarations inside loops when they should be declared before the loop. This causes variables to be re-initialized on each iteration and makes them unavailable after the loop.

## Minimal Reproduction

**Python Input** (`examples/showcase/calculate_sum.py`):
```python
from typing import List

def calculate_sum(numbers: List[int]) -> int:
    """Calculate the sum of a list of integers."""
    total: int = 0      # Declared BEFORE loop
    for n in numbers:
        total += n      # Accumulated INSIDE loop
    return total        # Used AFTER loop
```

**Expected Rust Output**:
```rust
pub fn calculate_sum(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;        // Declared BEFORE loop
    for n in numbers.iter() {
        total = total + n;     // Accumulated INSIDE loop
    }
    return total;              // Used AFTER loop
}
```

**Actual Rust Output** (INCORRECT):
```rust
pub fn calculate_sum(numbers: &Vec<i32>) -> i32 {
    for n in numbers.iter() {
        let total = 0 + n;    // ❌ Declared INSIDE loop (re-initialized each iteration)
    }
    return 0;                 // ❌ Returns literal 0, not total
}
```

## Problems

1. **Variable Scope**: `total` is declared inside the loop instead of before it
2. **Lost Accumulation**: Variable is re-initialized on each iteration
3. **Incorrect Return**: Returns `0` instead of `total`
4. **Unused Variable**: `total` is never read, causing "unused variable" warning

## Impact

**Correctness**: BROKEN - Function always returns 0 instead of sum
**Test Results**: Tests likely fail or are incorrect
**Severity**: CRITICAL - This is a logic bug, not a style issue

## Root Cause Analysis (CONFIRMED)

**ROOT CAUSE FOUND**: The bug is in the **Optimizer**, not in code generation!

The transpiler correctly handles:
- ✅ AST → HIR conversion (`AnnAssign -> Assign`, `AugAssign -> Assign + Binary`)
- ✅ HIR structure is correct (verified via `depyler inspect --repr hir`)
- ✅ Code generation logic in `rust_gen.rs` is correct

**The Problem**: `crates/depyler-core/src/optimizer.rs`
- The optimizer runs 4 passes: constant propagation, dead code elimination, function inlining, CSE
- ALL are enabled by default (see `OptimizerConfig::default()`)
- One or more of these passes is incorrectly transforming loop variable assignments
- The optimization runs BEFORE code generation (line 429 in lib.rs)

**Evidence**:
1. HIR inspection shows CORRECT structure before optimization
2. Generated Rust code shows INCORRECT structure after optimization
3. Pipeline: `AST → HIR → OPTIMIZER → rust_gen → Rust`
4. Bug occurs between HIR and rust_gen, which is where optimizer runs

## Expected HIR

```rust
[
    Assign { target: Symbol("total"), value: Literal(0) },  // Before loop
    For {
        target: "n",
        iter: Var("numbers"),
        body: [
            Assign {
                target: Symbol("total"),
                value: Binary { op: Add, left: Var("total"), right: Var("n") }
            }
        ]
    },
    Return(Some(Var("total")))  // Return total
]
```

## Investigation Needed

1. Verify HIR is correct by adding debug output
2. Check if variable declarations are being moved during optimization
3. Review codegen scope tracking for variables used across loop boundaries
4. Test with simpler case (no type annotation, no augmented assignment)

## Workaround

None available - requires transpiler fix.

## Next Steps

1. Add debug logging to trace where `total` declaration moves
2. Create minimal test case without type annotations
3. Fix scope tracking in codegen or HIR transformation
4. Add regression test for this pattern
5. Re-transpile all examples after fix

## Related Files

- `examples/showcase/calculate_sum.py` (input)
- `examples/showcase/calculate_sum.rs` (incorrect output)
- `crates/depyler-core/src/ast_bridge/converters.rs` (AnnAssign, AugAssign handlers)
- `crates/depyler-core/src/codegen.rs` or `rust_gen.rs` (suspected location of bug)
