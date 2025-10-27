# DEPYLER-0269: Investigation Summary & Implementation Plan

**Date**: 2025-10-27
**Status**: RED Phase Complete, GREEN Phase Deferred
**Complexity**: HIGH (Requires architectural changes)

---

## Investigation Summary

### What Was Completed

✅ **RED Phase**: Created comprehensive failing test suite (6 tests, 303 lines)
- Basic reference parameter test
- Multiple reference parameters test
- String reference parameter test
- Dict reference parameter test
- Property-based reference parameter test
- Diagnostic test for current bug verification

✅ **Bug Ticket**: Complete documentation with root cause analysis (188 lines)

✅ **Test Registration**: Added to Cargo.toml

### Root Cause Confirmed

The transpiler generates function signatures with reference parameters (`&Vec<T>`), but call sites pass owned values without the borrow operator `&`:

```python
def process(data: list[int]) -> int:
    return len(data)

def main() -> None:
    nums = [1, 2, 3]
    result = process(nums)  # Should be process(&nums)
```

**Generated (BROKEN)**:
```rust
pub fn process<'a>(data: &'a Vec<i32>) -> i32 { ... }

pub fn main() {
    let nums = vec![1, 2, 3];
    let result = process(nums);  // ❌ Missing &
}
```

---

## Implementation Challenges Discovered

### Challenge 1: Variable Type Tracking

**Problem**: `ctx.var_types` only tracks function **parameters**, not local variables.

**Evidence**:
- `func_gen.rs:131` populates `ctx.var_types` for parameters only
- Local variables (like `nums` in `main()`) are not tracked
- Simple type-based approach won't work without comprehensive tracking

### Challenge 2: Function Signature Lookup

**Current Architecture**:
- Function signatures generated in `func_gen.rs`
- Call sites generated in `expr_gen.rs`
- No easy way to look up "what does function X expect?" during call generation

**What's Needed**:
- Store all function signatures in transpilation context
- Look up signature during call generation
- Match argument types with parameter types

### Challenge 3: Type Flow Integration

**Existing Infrastructure**:
- `depyler-analyzer/src/type_flow.rs` has `TypeEnvironment` with function signatures
- Not currently used during code generation
- Would require integration between analyzer and codegen

---

## Attempted Approaches

###  Approach 1: Type-Based Borrowing (FAILED)

**Attempt**: Check `ctx.var_types` for argument type, add `&` if collection type

**Code Location**: `expr_gen.rs:546`

**Why It Failed**:
```rust
if let HirExpr::Var(var_name) = arg {
    if let Some(var_type) = self.ctx.var_types.get(var_name) {
        // Only works for function parameters, not local vars!
    }
}
```

**Result**: `ctx.var_types` empty for local variables, logic never triggers

---

## Recommended Implementation Strategies

### Strategy 1: Comprehensive Variable Tracking (PREFERRED)

**Approach**: Track ALL variable types during statement generation, not just parameters

**Changes Required**:
1. Modify `stmt_gen.rs` to add assignments to `ctx.var_types`:
   ```rust
   HirStmt::Assign { target, value, .. } => {
       if let AssignTarget::Symbol(var_name) = target {
           let inferred_type = infer_type_from_expr(value);
           ctx.var_types.insert(var_name.clone(), inferred_type);
       }
   }
   ```

2. Update `expr_gen.rs` to use this information:
   ```rust
   if let HirExpr::Var(var_name) = arg {
       if let Some(var_type) = ctx.var_types.get(var_name) {
           if matches!(var_type, Type::List(_) | Type::Dict(_, _) | ...) {
               expr = parse_quote! { &#expr };
           }
       }
   }
   ```

**Pros**:
- Complete solution for all variable types
- Works for local variables, parameters, and closure captures
- Minimal architectural changes

**Cons**:
- Requires careful type inference for all expressions
- Must handle edge cases (mutable borrows, already-borrowed vars, etc.)

### Strategy 2: Function Signature Lookup

**Approach**: Store function signatures in context, look them up during calls

**Changes Required**:
1. Add `function_signatures: HashMap<String, FunctionSignature>` to `CodeGenContext`
2. Populate during module-level function generation
3. Look up during call generation to determine parameter types

**Pros**:
- More precise - knows exactly what function expects
- Can handle mixed parameter types (some borrowed, some owned)

**Cons**:
- Requires two-pass generation (collect signatures, then generate calls)
- More architectural changes

### Strategy 3: Type Flow Integration

**Approach**: Use existing `type_flow::TypeEnvironment` from analyzer

**Changes Required**:
1. Pass `TypeEnvironment` through to codegen context
2. Query during call generation for both function signature and argument types

**Pros**:
- Leverages existing type inference infrastructure
- Most robust long-term solution

**Cons**:
- Largest architectural change
- Requires analyzer-codegen integration

---

## Recommendation

**Implement Strategy 1** (Comprehensive Variable Tracking) because:

1. **Minimal Disruption**: Small, focused changes to existing code
2. **Complete Solution**: Handles all variable types (params, locals, etc.)
3. **Incremental**: Can be implemented file-by-file
4. **TDD-Friendly**: Easy to test with existing RED phase tests

**Implementation Steps**:
1. Add `track_variable_type()` helper to `CodeGenContext`
2. Update assignment statement generation to track types
3. Update call expression generation to use tracked types
4. Run RED phase tests to verify fix
5. Run full regression suite

**Estimated Complexity**: Medium (4-6 complexity for main logic, well within A+ standard)

---

## Files to Modify

1. **crates/depyler-core/src/rust_gen/context.rs**
   - Add `track_variable_type()` method
   - Ensure `var_types` persists across scopes correctly

2. **crates/depyler-core/src/rust_gen/stmt_gen.rs**
   - Track variable types in assignment statements
   - Handle tuple unpacking, index assignment, etc.

3. **crates/depyler-core/src/rust_gen/expr_gen.rs**
   - Use `ctx.var_types` to add `&` when needed
   - Lines 538-541: Modify argument conversion logic

---

## Test Coverage

**RED Phase Tests** (Already Created):
- ✅ `test_DEPYLER_0269_basic_reference_parameter_compiles`
- ✅ `test_DEPYLER_0269_multiple_reference_parameters_compiles`
- ✅ `test_DEPYLER_0269_string_reference_parameter_compiles`
- ✅ `test_DEPYLER_0269_dict_reference_parameter_compiles`
- ✅ `test_DEPYLER_0269_reference_parameter_types` (property-based)
- ✅ `test_DEPYLER_0269_verify_current_bug` (diagnostic)

**Additional Tests Needed** (GREEN Phase):
- Edge case: Already-borrowed variables (`&&T` should not happen)
- Edge case: Mutable borrows (`&mut` when needed)
- Edge case: Primitive types (should work with or without `&` due to Copy)
- Regression: Ensure existing tests still pass

---

## Success Criteria

1. **All 6 RED phase tests pass** ✅
2. **Generated code compiles** (rustc --deny warnings)
3. **Benchmark line 109 fixed** (`calculate_statistics(&fib_sequence)`)
4. **Zero regressions** in existing test suite
5. **Complexity ≤10** for all modified functions (A+ standard)

---

## Next Steps

**For Next Session**:
1. Implement Strategy 1 (Comprehensive Variable Tracking)
2. Start with simple case: track `let var = expr` assignments
3. Expand to handle tuple unpacking, index assignments
4. Add `&` in call generation based on tracked types
5. Verify all RED phase tests pass
6. Run full regression suite

**Estimated Time**: 1-2 hours for complete GREEN + REFACTOR phases

---

**Created**: 2025-10-27
**Investigator**: Claude Code (Anthropic)
**Commits**: RED phase committed as 2e2634a
