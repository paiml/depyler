# DEPYLER-0338: re Module find() Argument Count Error

**Status**: 🔴 BLOCKING
**Priority**: P1 (HIGH)
**Severity**: Medium
**Created**: 2025-11-05
**Component**: Transpiler Core / Stdlib Mapping
**Affects**: Python code using re module find/search methods

---

## Problem Statement

The transpiler fails with error `find() requires exactly one argument` when processing Python files that use the `re` module. This prevents transpilation of any code using regular expression functionality.

### Error Message
```
Error: find() requires exactly one argument

Stack backtrace:
   0: anyhow::error::<impl anyhow::Error>::msg
             at /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/backtrace.rs:27:14
   1: anyhow::__private::format_err
```

### Reproduction

**Test File**: `examples/test_re_module.py`

**Current Behavior**: Transpilation fails with argument count error
**Expected Behavior**: Should transpile re module functions to Rust regex equivalents

---

## Root Cause Analysis

**Likely Cause**: Confusion between:
1. **String `.find()` method** (takes 1 argument: substring to find)
2. **re module functions** (like `re.search()`, `re.findall()` which take pattern + string)

The transpiler appears to be incorrectly applying string method validation rules to re module function calls.

**Hypothesis**:
The AST converter sees `find` or `findall` and assumes it's a string method, enforcing the single-argument rule, instead of recognizing it as an `re` module function that takes multiple arguments.

---

## Investigation Needed

### Questions to Answer
1. Where is the "find() requires exactly one argument" error raised?
2. Is this in string method handling or re module handling?
3. Does the transpiler correctly distinguish between:
   - `string.find(substring)`
   - `re.search(pattern, string)`
   - `re.findall(pattern, string)`

### Files to Inspect
1. `crates/depyler-core/src/ast_bridge/converters.rs` - Method call handling
2. `crates/depyler-core/src/stdlib/` - Stdlib module mappings
3. Search for "find() requires" error string

---

## Solution Design (Preliminary)

### Expected Translations

| Python (re module) | Rust Equivalent |
|--------------------|-----------------|
| `re.search(pattern, string)` | `regex::Regex::new(pattern)?.find(string)` |
| `re.match(pattern, string)` | `regex::Regex::new(pattern)?.is_match(string)` |
| `re.findall(pattern, string)` | `regex::Regex::new(pattern)?.find_iter(string).collect()` |
| `re.sub(pattern, repl, string)` | `regex::Regex::new(pattern)?.replace_all(string, repl)` |

### Implementation Strategy

1. **Add re module mapping** to stdlib mappings
2. **Distinguish context**: Check if `find`/`findall` is called on `re` module vs string
3. **Update argument validation**: Don't apply string method rules to re module functions
4. **Add Rust regex dependency**: Ensure `regex` crate is available in generated code

---

## Implementation Plan (EXTREME TDD)

### Phase 1: RED - Add Failing Tests

Create `crates/depyler-core/tests/test_re_module.rs`:

```rust
#[test]
fn test_re_search_basic() {
    let python = r#"
import re
result = re.search(r'\d+', 'age: 42')
"#;
    let result = transpile(python);
    assert!(result.is_ok());
    assert!(result.unwrap().contains("regex::Regex"));
}

#[test]
fn test_re_findall() {
    let python = r#"
import re
numbers = re.findall(r'\d+', 'one 1 two 2 three 3')
"#;
    let result = transpile(python);
    assert!(result.is_ok());
}

#[test]
fn test_re_vs_string_find() {
    let python = r#"
import re
# String method
idx = "hello".find("l")
# Re module function
match = re.search(r'l+', "hello")
"#;
    let result = transpile(python);
    assert!(result.is_ok());
    // Should handle both differently
}
```

### Phase 2: GREEN - Implement Fix

**Step 1**: Locate the error source
```bash
grep -r "find() requires exactly one argument" crates/
```

**Step 2**: Update method call handling
- Check if method is called on `re` module
- Apply different validation rules for module functions vs instance methods

**Step 3**: Add re module mapping
- Map `re.search` → regex find
- Map `re.findall` → regex find_iter
- Map `re.sub` → regex replace_all
- etc.

### Phase 3: REFACTOR - Optimize

1. Create dedicated `re` module handler
2. Add comprehensive regex pattern translation
3. Handle compiled regex objects (`re.compile()`)
4. Document limitations (e.g., Python regex features not in Rust)

---

## Test Plan

### Unit Tests (Minimum 5)
1. ✅ `re.search(pattern, string)`
2. ✅ `re.match(pattern, string)`
3. ✅ `re.findall(pattern, string)`
4. ✅ `re.sub(pattern, repl, string)`
5. ✅ Distinguish `re.find*` from `string.find()`

### Integration Tests (Minimum 2)
1. ✅ Full `test_re_module.py` transpiles successfully
2. ✅ Generated Rust compiles and runs

---

## Validation Criteria

### Functional
- [ ] test_re_module.py transpiles without errors
- [ ] Generated Rust code compiles
- [ ] Regex operations produce correct results
- [ ] String `find()` still works correctly (no regression)

### Code Quality
- [ ] Cyclomatic Complexity ≤ 10
- [ ] Test Coverage ≥ 85%
- [ ] All clippy warnings resolved

---

## Impact Assessment

**Affected Components**:
- Method call handling
- Stdlib module mappings
- re module support

**Downstream Impact**:
- **Immediate**: test_re_module.py can transpile
- **Short-term**: All code using re module
- **Long-term**: Enables regex-heavy applications

**Risk Level**: Medium (need to avoid breaking string `.find()` method)

---

## Workaround (Temporary)

Until fixed, users can:
1. Avoid using `re` module in Python code
2. Use string methods where possible
3. Post-process generated Rust to add regex manually

---

## Related Issues

- DEPYLER-0337: `is` operator not supported (found during same validation session)
- Potentially related to other stdlib module mapping issues

---

## Resolution Timeline

- **Filed**: 2025-11-05
- **Target Fix**: 2025-11-06 (P1 - next day)
- **Actual Fix**: TBD
- **Verified**: TBD

---

## Lessons Learned

1. **Stdlib validation importance**: Need systematic validation of all stdlib modules
2. **Method vs function distinction**: Transpiler must correctly distinguish module functions from instance methods
3. **Test coverage gaps**: Should have tests for all validated stdlib modules

---

## References

- Python re module docs: https://docs.python.org/3/library/re.html
- Rust regex crate: https://docs.rs/regex/latest/regex/
- STDLIB_COVERAGE.md: re module marked as validated but transpilation failing

---

**Assigned To**: TBD
**Reviewed By**: TBD
**Merged In**: TBD
