# DEPYLER-0504: Multiple generators in list comprehension not supported

## Problem Statement
List comprehensions with multiple `for` clauses (generators) fail with: `"Nested list comprehensions not yet supported"`

**Minimal Reproducer**:
```python
# Flattened comprehension (2 generators)
result = [i * j for i in range(3) for j in range(3)]

# With condition (2 generators + if)
result = [(i, j) for i in range(5) for j in range(5) if i < j]
```

**Error**: Rejected at conversion time (converters.rs:795-796)

## Five-Whys Root Cause Analysis

**1. Why does transpilation fail?**
- convert_list_comp() rejects comprehensions with `generators.len() != 1`

**2. Why is it rejected?**
- Line 795: `if lc.generators.len() != 1 { bail!("Nested list comprehensions not yet supported"); }`
- Check hardcoded to only allow single generator

**3. Why only single generator supported?**
- Current HIR `ListComp` structure only has fields for one generator:
  ```rust
  HirExpr::ListComp {
      element,
      target,     // String (single target like "x")
      iter,       // Box<HirExpr> (single iterator)
      condition,  // Option<Box<HirExpr>> (single condition)
  }
  ```

**4. Why doesn't HIR support multiple generators?**
- Original implementation (DEPYLER-0XXX) only handled simple case
- Multiple generators require either:
  - Vec<Generator> in HIR (architectural change)
  - Or desugaring into nested loops (code transformation)

**5. ROOT CAUSE: HIR architecture limitation**
- HIR `ListComp` node designed for single generator only
- No representation for multiple `for` clauses in same comprehension
- Python AST has `Vec<Comprehension>` (generators), HIR flattened it to single generator fields

**Architecture Issue**: Semantic mismatch between Python AST (supports N generators) and HIR (supports 1 generator).

## Solution Approaches

### Option A: HIR Schema Change (Recommended)
Change HIR to match Python semantics more closely:

```rust
// Current (BEFORE):
HirExpr::ListComp {
    element: Box<HirExpr>,
    target: String,
    iter: Box<HirExpr>,
    condition: Option<Box<HirExpr>>,
}

// Proposed (AFTER):
HirExpr::ListComp {
    element: Box<HirExpr>,
    generators: Vec<HirGenerator>,  // Support multiple for clauses
}

struct HirGenerator {
    target: AssignTarget,  // Supports tuple unpacking: for (i, j) in ...
    iter: Box<HirExpr>,
    conditions: Vec<HirExpr>,  // Support multiple if clauses per generator
}
```

**Pros**:
- More faithful to Python semantics
- Cleaner representation
- Easier to extend (e.g., tuple unpacking in for loop)

**Cons**:
- Requires HIR schema migration
- Affects all list/set/dict comprehension nodes
- Rust codegen needs nested loop generation

### Option B: Desugar to Nested Comprehensions
Transform `[f(x,y) for x in A for y in B]` → `[f(x,y) for x in A for y in [f2(x) for y in B]]`

**Pros**:
- No HIR schema changes
- Reuses existing single-generator code

**Cons**:
- More complex AST transformation
- Less efficient (creates intermediate lists)
- Harder to optimize later

### Option C: Desugar to Nested Loops
Transform to explicit for loops:
```python
[f(x,y) for x in A for y in B]
→
result = []
for x in A:
    for y in B:
        result.append(f(x,y))
```

**Pros**:
- No HIR schema changes
- Straightforward transformation

**Cons**:
- Loses comprehension semantics
- Harder to optimize (e.g., can't use Rust iterator chains)
- Verbose HIR

## Recommended Solution: Option A (HIR Schema Change)

**Rationale**:
1. Most faithful to Python semantics
2. Enables future enhancements (tuple unpacking, multiple conditions)
3. Clean separation of concerns
4. Better optimization opportunities

**Implementation Plan**:
1. Define `HirGenerator` struct
2. Update `HirExpr::ListComp`, `SetComp`, `DictComp` to use `Vec<HirGenerator>`
3. Update converters.rs to handle multiple generators
4. Update rust_gen to generate nested iterator chains
5. Update all tests

**Estimated Complexity**: Medium-High (affects HIR core, converters, codegen)

## Test Plan

**RED Phase Tests** (crates/depyler-core/tests/depyler_0504_multiple_generators.rs):
1. `test_flattened_list_comprehension()` - Basic 2-generator case
2. `test_filtered_nested_comprehension()` - 2 generators + condition
3. `test_nested_list_comprehension()` - Already passing (outer comprehension)

**Expected**: Tests 1-2 should transpile without "not yet supported" errors after fix.

## Scope Note

**This is a COMPLEX ticket** requiring:
- HIR schema changes (breaking change)
- Converter updates
- Codegen updates for nested iteration
- Extensive testing

**Decision**: Document for future work, prioritize simpler bugs first.

## Related Context

**Discovery**: Systematic bug discovery via `discover_bug_test.rs`
**Example File**: `examples/ast_converters_demo.py` line 197
**Impact**: Blocks flattened comprehensions (common Python pattern)

## References

- **Python AST**: `ast::ExprListComp` has `generators: Vec<Comprehension>`
- **Current HIR**: Single generator fields (target/iter/condition)
- **Similar Patterns**: Set/Dict comprehensions have same limitation
