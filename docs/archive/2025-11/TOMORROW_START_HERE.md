# 🌅 START HERE - Friday Morning (2025-11-08)

## 🛑 P0 BLOCKER - RELEASE CANNOT PROCEED

**Status**: 3 test failures blocking Friday release
**Time Available**: ~6 hours before release freeze
**Estimated Work**: 4-6 hours to fix all issues

## 🎯 Your Mission

Fix these 3 failing tests to achieve ZERO DEFECTS:

1. `test_DEPYLER_0269_multiple_reference_parameters_compiles`
2. `test_DEPYLER_0270_dict_result_unwrapping_compiles`
3. `test_DEPYLER_0270_multiple_result_accesses_compiles`

## 📋 Quick Action Plan

### Step 1: Run Tests to Confirm State (5 min)

```bash
cd /home/noah/src/depyler

# Quick test of the 3 failures
cargo nextest run --workspace \
  "test_DEPYLER_0269_multiple_reference_parameters" \
  "test_DEPYLER_0270_dict_result_unwrapping" \
  "test_DEPYLER_0270_multiple_result_accesses"
```

### Step 2: Fix #1 - Display Trait (1-2 hours)

**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs`

**Current Issue**:
```python
result = merge(a, b)  # Function return value not tracked
print(result)         # Uses {} instead of {:?}
```

**Solution**: Track function return types in assignments

```rust
// Add to codegen_assign_stmt around line 820
HirExpr::Call { func, .. } => {
    // Look up function return type from HIR
    // If return type is List/Dict/Set, track it
    if let Some(ret_type) = get_function_return_type(func) {
        if matches!(ret_type, Type::List(_) | Type::Dict(_, _) | Type::Set(_)) {
            ctx.var_types.insert(var_name.clone(), ret_type);
        }
    }
}
```

**Test After**: Run test #1 to verify fix

### Step 3: Fix #2 - Result<(), E> Edge Case (30 min)

**File**: `crates/depyler-core/src/rust_gen/func_gen.rs`

**Current Issue**: `Ok(())` not added to some main() functions

**Debug**:
```bash
# Transpile the failing test case
cargo run --release --bin depyler -- transpile \
  tests/python/depyler_0270_dict_result_unwrapping.py \
  > /tmp/debug_output.rs

# Check generated main()
tail -20 /tmp/debug_output.rs
```

**Look for**:
- Is `can_fail` being set correctly?
- Is `ret_type` recognized as `Type::None`?
- Why isn't the condition at line 816 triggering?

**Fix**: Add debug logging or adjust condition

### Step 4: Fix #3 - Auto-Borrowing (2-3 hours)

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`

**Current Issue**: Auto-borrowing adds `&` but function expects owned

**Solution**: Check function signature before auto-borrowing

```rust
// Around line 2008-2040, modify auto-borrowing logic:
let borrowed_args: Vec<syn::Expr> = hir_args
    .iter()
    .zip(args.iter())
    .map(|(hir_arg, arg_expr)| {
        let should_borrow = match hir_arg {
            HirExpr::Var(var_name) => {
                if let Some(var_type) = self.ctx.var_types.get(var_name) {
                    if matches!(var_type, Type::List(_) | Type::Dict(_, _) | Type::Set(_)) {
                        // NEW: Check if function expects borrowed or owned
                        // Look up function signature
                        // Only borrow if param type is &Vec not Vec
                        function_expects_borrowed(func, param_index)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false
        };

        if should_borrow {
            parse_quote! { &#arg_expr }
        } else {
            arg_expr.clone()
        }
    })
    .collect();
```

**Helper Function Needed**:
```rust
fn function_expects_borrowed(func_name: &str, param_idx: usize) -> bool {
    // Look up function in HIR
    // Check parameter type at param_idx
    // Return true if type is &Vec/&HashMap/&HashSet
    // Return false if type is Vec/HashMap/HashSet
}
```

### Step 5: Verify All Tests Pass (15 min)

```bash
# Run full test suite
make coverage

# Check for ZERO failures
# Expected: 2133/2133 tests pass (100%)
```

### Step 6: Commit and Push (10 min)

```bash
git add -A
git commit -m "[DEPYLER-0269/0270] Fix final 3 test failures - ZERO DEFECTS achieved

Fixes:
1. Display trait for function return values
2. Result<(), E> edge case in main()
3. Auto-borrowing function signature lookup

Tests: 2133/2133 pass (100%)
Status: Release unblocked ✅

Closes: DEPYLER-0269, DEPYLER-0270"

git push origin main
```

## 📚 Reference Documents

- **Detailed Analysis**: `STOP_THE_LINE_STATUS.md`
- **Test Locations**:
  - `tests/depyler_0269_function_borrowing_test.rs`
  - `tests/depyler_0270_result_unwrapping_test.rs`
- **Previous Commits**:
  - db3224f: Result<(), E> fix + type tracking
  - dd751e2: Vec concatenation fix
  - 2539c40: Documentation (tonight)

## ⚠️ If You Get Stuck

### Fallback Option 1: Skip Failing Tests
```bash
# Mark tests as ignored temporarily
#[ignore]
#[test]
fn test_DEPYLER_0269_multiple_reference_parameters_compiles() { ... }
```

### Fallback Option 2: Extend Deadline
- Push release to NEXT Friday (Nov 15)
- Document as known issues in CHANGELOG
- Create DEPYLER-0275 ticket for post-release fix

## 🎯 Success Criteria

- [ ] All 3 tests pass
- [ ] `make coverage` shows 100% pass rate (2133/2133)
- [ ] Changes committed and pushed
- [ ] STOP_THE_LINE_STATUS.md updated to RESOLVED
- [ ] CHANGELOG.md updated with fixes
- [ ] Ready for Friday release ✅

## 💪 You Got This!

You fixed 7/10 tests yesterday (70%). The remaining 3 are well-documented with clear solutions. Estimated 4-6 hours = doable before release freeze.

**Start with Fix #2** (Result edge case) - it's the quickest win and will boost momentum!

Good luck! 🚀
