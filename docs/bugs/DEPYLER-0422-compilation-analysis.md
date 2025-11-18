# DEPYLER-0422: Systematic Compilation Error Analysis

**Status:** 46/142 examples compiling (32.4%)
**Errors Remaining:** 1297 errors across 96 files
**Date:** 2025-11-18

## Executive Summary

After fixing DEPYLER-0420 (array repeat) and DEPYLER-0421 (truthiness), we have 46/142 files compiling. This document provides systematic analysis of the remaining 1297 errors to guide further fixes toward 100% compilation.

## Error Distribution

| Error Code | Count | Description | Transpiler Bug? |
|------------|-------|-------------|-----------------|
| E0308 | 324 | Type mismatch | YES |
| E0277 | 297 | Trait bound not satisfied | MIXED |
| E0433 | 147 | Unresolved import | NO (external crates) |
| E0599 | 123 | Method not found | YES |
| E0412 | 76 | Cannot find type | YES |
| E0425 | 69 | Cannot find value | YES |
| E0423 | 55 | Expected function, found module | YES |
| E0609 | 50 | No field on type | YES |
| E0432 | 46 | Unresolved import | NO (external crates) |
| E0282 | 37 | Type annotations needed | YES |

## Files by Error Count

- **0 errors (compiling):** 46 files (32.4%)
- **1 error:** 16 files (quick wins)
- **2-5 errors:** 30 files
- **6-10 errors:** 13 files
- **10+ errors:** 37 files (complex cases)

## Five-Whys Analysis: Top 3 Transpiler Bugs

### 1. E0308: Type Mismatches (324 errors)

#### Pattern A: Expected bool, found collection (truthiness)

**Sample Error:**
```
error[E0308]: mismatched types
  --> showcase_type_inference.rs:93:27
   |
93 |     let _cse_temp_0 = a && b;
   |                           ^ expected `bool`, found `&str`
```

**Five-Whys:**
1. **Why:** Boolean operators `&&` and `||` receive non-bool operands
2. **Why:** Truthiness conversion not applied to binary logical operators
3. **Why:** `apply_truthiness_conversion()` only handles if/while conditions
4. **Why:** Binary operator codegen doesn't check for truthiness needs
5. **ROOT CAUSE:** Logical operators (`&&`, `||`) missing truthiness conversion in `expr_gen.rs`

**Solution:** Apply truthiness conversion to `&&` and `||` operands

**Files Affected:** 5+ files (showcase_type_inference.rs, type_inference_demo.rs, etc.)

---

#### Pattern B: Vec vs Array type mismatch

**Sample Error:**
```
error[E0308]: mismatched types
  --> array_test.rs:31:6
   |
31 |     (z, o, f)
   |      ^ expected `Vec<i32>`, found `[i32; 10]`
```

**Five-Whys:**
1. **Why:** Function returns array but signature says Vec
2. **Why:** Return type inference says Vec but code generates array
3. **Why:** `zeros(10)` function call generates `[0; 10]` array
4. **Why:** `zeros()` function not recognized as Vec constructor
5. **ROOT CAUSE:** numpy-style functions (zeros, ones, full) not mapped to Vec constructors

**Solution:** Map `zeros(n)`, `ones(n)`, `full(n, val)` → `vec![val; n]`

**Files Affected:** 2 files (array_test.rs, test_pickle_module.rs)

---

#### Pattern C: &str vs String ownership

**Sample Error:**
```
error[E0308]: mismatched types
  --> lambda_demo.rs:X:Y
   |
   | expected `String`, found `&str`
```

**Five-Whys:**
1. **Why:** String reference where owned String expected
2. **Why:** String literals generate `&str` in some contexts
3. **Why:** Ownership not tracked in variable type map
4. **Why:** No conversion from `&str` → `String` when needed
5. **ROOT CAUSE:** Missing `.to_string()` conversions in assignments/returns

**Solution:** Add automatic `.to_string()` when assigning `&str` to `String` variable

**Files Affected:** 5+ files (lambda_demo.rs, simulation_combined.rs, etc.)

---

### 2. E0277: Trait Bound Not Satisfied (297 errors)

#### Pattern: Slice indexing with wrong integer type

**Sample Error:**
```
error[E0277]: the type `[f64]` cannot be indexed by `&i32`
  --> data_analysis_combined.rs:133:29
   |
133 |             sorted_data.get(&mid).cloned().unwrap_or_default(),
   |                             ^^^^ expected `usize`, found `&i32`
```

**Five-Whys:**
1. **Why:** `.get()` called with `&i32` instead of `usize`
2. **Why:** Integer variable used as index without cast
3. **Why:** Python int → Rust i32, but indexing needs usize
4. **Why:** Index expressions not automatically cast to usize
5. **ROOT CAUSE:** Missing `as usize` casts in index/slice operations

**Solution:** Automatically cast integer indices to `usize` in `.get()` calls

**Files Affected:** Many files with array/slice indexing

---

### 3. E0599: Method Not Found (123 errors)

**Five-Whys:** (Need specific samples to analyze)

---

## Recommended Fix Priority

### Phase 1: High-Impact Fixes (Target: +20 files)
1. **E0308 Pattern A:** Logical operators truthiness → Fix 5+ files
2. **E0308 Pattern B:** numpy function mapping → Fix 2 files
3. **E0308 Pattern C:** &str → String conversion → Fix 5+ files
4. **E0277 Pattern:** Index type casting → Fix many files

### Phase 2: Medium-Impact Fixes (Target: +30 files)
5. E0599: Method not found issues
6. E0412: Type resolution issues
7. E0425: Value/variable resolution

### Phase 3: Complex Cases (Target: +50 files)
8. Files with 10+ errors (systematic re-transpilation)
9. Edge cases and unique bugs

## Testing Strategy

For each fix:
1. Use `--trace` flag to see transpilation decisions
2. Use `--explain` flag for error context
3. Create minimal test case (e.g., `test_logical_operators.py`)
4. Verify fix with `cargo test --workspace`
5. Re-transpile affected examples
6. Count compilation progress

## Success Metrics

- **Phase 1 Target:** 66/142 files compiling (46%)
- **Phase 2 Target:** 96/142 files compiling (68%)
- **Phase 3 Target:** 142/142 files compiling (100%)

## Notes

- Skip E0433/E0432 (external crate imports) - not transpiler bugs
- Focus on systematic patterns, not one-off fixes
- Each fix should help multiple files compile
- Use five-whys for root cause, not symptoms

---

**Next Steps:** Implement Phase 1 fixes systematically, starting with logical operators truthiness conversion.
