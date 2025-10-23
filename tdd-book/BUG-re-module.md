# BUG REPORT: re module - Match object handling causes PANIC

**Discovered**: 2025-10-23
**Test Suite**: tdd-book/tests/test_re.py
**Severity**: P1 - MAJOR (Transpiler Crash)
**Category**: transpiler_crash

## Problem

re module operations that return Match objects cause **transpiler panic**:
```
thread 'main' panicked at expr_gen.rs:34:16:
unexpected end of input, expected an expression
```

## Test Evidence

**Test File**: tests/test_re.py
**Results**: 2/6 passing (33.3% - PARTIAL FAILURE)

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

## Next Ticket

Should create: **DEPYLER-0XXX: Implement Match object type and truthiness for re module**

This will require:
1. Add Match object type to type system
2. Implement Option<regex::Captures> mapping
3. Implement truthiness check for Match object (is_some())
4. Handle match.group(), match.groups(), match.span()
5. Implement compiled pattern (Regex) type
6. Add re module to import resolution

**Estimated Effort**: 6-12 hours (new type + pattern implementation)

---

**Discovery Method**: TDD Book validation (OPTION 1 strategy)
**Bug Severity Progression**: P1 (copy) → P0 (struct) → P1 (re)
**Expected Outcome**: More bugs in memoryview, sys, textwrap modules
