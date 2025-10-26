# BUG REPORT: re module - RESOLVED (Rust Keyword Collision)

**Discovered**: 2025-10-23
**Fixed**: 2025-10-26 (DEPYLER-0023)
**Test Suite**: tdd-book/tests/test_re.py
**Severity**: P1 - MAJOR (Transpiler Crash) → FIXED
**Category**: transpiler_crash → rust_keyword_collision
**Resolution Time**: 30 minutes (vs 6-12h estimate)

## Problem (RESOLVED)

re module operations caused **transpiler panic** - but NOT due to Match objects!

**Root Cause**: Variable named `match` (Rust strict keyword) caused panic at expr_gen.rs:34

## Test Evidence

**Test File**: tests/test_re.py
**Before Fix**: 2/6 passing (33.3% - PARTIAL FAILURE)
**After DEPYLER-0023**: **6/6 passing (100% - COMPLETE SUCCESS)**

**Failed** ❌:
- test_re_search_basic - Panic (Match object check)
- test_re_match_start - Panic (Match object check)
- test_re_compile - Panic (Match object check)
- test_re_groups - Panic (Match object check)

**Passed** ✅:
- test_re_findall - Works (returns list)
- test_re_sub - Works (returns string)

## Failing Code

**re.search with Match object**:
```python
import re

def test_search() -> int:
    text = "hello world"
    match = re.search(r"world", text)

    # ❌ Panic happens here
    if match:
        return 1
    else:
        return 0
```

**re.match with Match object**:
```python
import re

def test_match() -> int:
    text = "hello world"
    match = re.match(r"hello", text)

    # ❌ Panic happens here
    if match:
        return 1
    else:
        return 0
```

**re.compile with Match object**:
```python
import re

def test_compile() -> int:
    pattern = re.compile(r"[0-9]+")
    text = "there are 42 apples"
    match = pattern.search(text)

    # ❌ Panic happens here
    if match:
        return 1
    else:
        return 0
```

## Error Analysis

**Panic Location**: `expr_gen.rs:34:16`
**Error**: `unexpected end of input, expected an expression`
**Return Code**: 101 (panic/crash)

**Root Cause**: The transpiler cannot handle the **Match object type**:
- `re.search()` returns `Optional[Match]` - not recognized
- `re.match()` returns `Optional[Match]` - not recognized
- `pattern.search()` returns `Optional[Match]` - not recognized
- Checking `if match:` fails because Match object truthiness not implemented

**Working cases**:
- `re.findall()` returns `list[str]` - works fine
- `re.sub()` returns `str` - works fine

**Pattern**: Functions returning **Match objects** fail, functions returning **basic types** work.

## Impact

- **MAJOR**: Most common regex operations cause transpiler crash
- **Partial**: 33.3% of tests pass (basic operations work)
- **SEVERITY**: P1 - This is more severe than copy bug, less severe than struct bug
- re is a core Python stdlib module
- Used for: Text processing, validation, parsing, data extraction

## Comparison with other bugs

1. **struct module**: TOTAL failure (0/6 tests, P0 CRITICAL)
2. **re module**: MAJOR failure (2/6 tests, P1 MAJOR)
3. **copy.copy() for lists**: Minor failure (5/6 tests, P1 MAJOR)

The re bug is severe but not as bad as struct (which is completely unimplemented).

## Recommended Fix Priority

**P1 - MAJOR - HIGH PRIORITY**

This should be fixed after struct module but before copy.copy() because:
1. Transpiler crash vs simple failure (more severe than copy)
2. Affects majority of regex operations (66.7% failure rate)
3. Match object is fundamental to regex usage
4. More commonly used than struct module

## Resolution Summary (DEPYLER-0023)

**What We Learned**:
1. **TDD Book Misdiagnosis**: Report claimed "Match object not implemented" - WRONG!
2. **Actual Root Cause**: Variable named `match` (Rust keyword) caused parser failure
3. **Simple Fix**: Added `is_rust_keyword()` + `syn::Ident::new_raw()` for raw identifiers
4. **Impact**: Fixed ALL re module tests (2/6 → 6/6 passing = 100%)
5. **Time Saved**: 30 min vs 6-12h estimate (12-24x faster via correct root cause analysis)

**The Fix** (crates/depyler-core/src/rust_gen/expr_gen.rs):
```rust
fn is_rust_keyword(name: &str) -> bool {
    matches!(name, "match" | "type" | "impl" | /* ... all Rust keywords */)
}

fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
    let ident = if Self::is_rust_keyword(name) {
        syn::Ident::new_raw(name, proc_macro2::Span::call_site())  // r#match
    } else {
        syn::Ident::new(name, proc_macro2::Span::call_site())
    };
    Ok(parse_quote! { #ident })
}
```

**Lesson**: Always test simplest hypothesis first! Variable naming issue, not complex type system gap.

**Status**: RESOLVED - No Match object implementation needed. re module fully working!

---

**Discovery Method**: TDD Book validation (OPTION 1 strategy)
**Resolution**: DEPYLER-0023 (2025-10-26)
**Closes**: GitHub Issue #23
