# Transpiler Bug: Variable Scoping in Loops

**Date Discovered**: 2025-10-08
**Date Fixed**: 2025-10-08
**Severity**: CRITICAL (Correctness Issue)
**Status**: FIXED

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

**Specific Problem**: `propagate_constants_program` in optimizer.rs
- Treats ALL variables with constant initial values as immutable constants
- Breaks accumulator patterns:
  - `total = 0` → treated as constant
  - `total += n` inside loop → becomes `total = 0 + n` (wrong!)
  - `return total` → becomes `return 0` (wrong!)

## Solution

**Fix Implemented**: Added mutation tracking to constant propagation pass

**Changes Made** (`crates/depyler-core/src/optimizer.rs`):

1. **Three-Pass Approach**:
   - **Pass 1**: Count assignments per variable (`collect_mutated_vars_function`)
   - **Pass 2**: Collect constants, but skip mutated variables (`collect_constants_function`)
   - **Pass 3**: Propagate constants (existing logic)

2. **Mutation Detection**:
   ```rust
   fn collect_mutated_vars_function(&self, func: &HirFunction, mutated_vars: &mut HashSet<String>) {
       let mut assignments = HashMap::new();
       self.count_assignments_stmt(&func.body, &mut assignments);

       // Any variable assigned more than once is mutated
       for (var, count) in assignments {
           if count > 1 {
               mutated_vars.insert(var);
           }
       }
   }
   ```

3. **Recursive Assignment Counting**:
   - Traverses all statements (including nested loops, conditionals)
   - Counts how many times each variable is assigned
   - Marks variables with >1 assignment as mutated

4. **Modified Constant Collection**:
   ```rust
   // Only treat as constant if variable is never mutated AND value is constant
   if !mutated_vars.contains(name) && self.is_constant_expr(value) {
       constants.insert(name.clone(), value.clone());
   }
   ```

**Test Results**:
- ✅ Minimal test case (untyped): CORRECT output
- ✅ Typed test case with List[int]: CORRECT output
- ✅ calculate_sum.py: CORRECT output (compiles cleanly)
- ✅ All 370 core tests: PASSING
- ✅ 76/130 examples: Successfully re-transpiled (54 failures unrelated to this bug)

**Impact**:
- **Before Fix**: Accumulator patterns broken (always returned 0)
- **After Fix**: All variable scoping correct, loops work as expected
- **No Regressions**: All existing tests continue to pass

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

## Verification Steps Completed

1. ✅ Verified HIR is correct using `depyler inspect --repr hir --format debug`
2. ✅ Confirmed variable declarations being moved during optimization (constant propagation)
3. ✅ Identified optimizer as root cause (not codegen)
4. ✅ Tested with simpler case (no type annotation) - confirmed same bug
5. ✅ Created minimal test cases to isolate issue
6. ✅ Fixed optimizer constant propagation with mutation tracking
7. ✅ Re-transpiled all 130 examples (76 successful, 54 fail on unrelated unsupported features)

## Related Files

- `examples/showcase/calculate_sum.py` (input)
- `examples/showcase/calculate_sum.rs` (output, now CORRECT)
- `crates/depyler-core/src/optimizer.rs` (ROOT CAUSE - fixed constant propagation)
- `crates/depyler-core/src/ast_bridge/converters.rs` (AnnAssign, AugAssign handlers - working correctly)
- `crates/depyler-core/src/rust_gen.rs` (code generation - working correctly)
