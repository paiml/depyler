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

## Root Cause Analysis

The transpiler correctly handles:
- ✅ Annotated assignments (`AnnAssign -> Assign`)
- ✅ Augmented assignments (`total += n -> total = total + n`)
- ✅ For loop conversion

But somewhere in the pipeline, statements are being reordered or scoped incorrectly.

**Suspected Component**: Code generation or HIR transformation phase

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
