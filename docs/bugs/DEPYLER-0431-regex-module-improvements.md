# DEPYLER-0431: re (regex) Module Improvements

## Status: IN PROGRESS (Analysis Complete, Ready for RED)
- **Created**: 2025-11-19
- **Priority**: P1 (HIGH - MEDIUM Priority)
- **Type**: Feature Gap
- **Parent**: DEPYLER-0435 (reprorusted-python-cli 100% compilation)
- **Blocks**: pattern_matcher (46 errors)
- **Estimated Effort**: 2-3 hours
- **Actual Effort**: TBD

## Problem Statement

The pattern_matcher.py example fails to compile with 46 errors, of which 15 (33%) are due to missing or incorrect implementations of regex (re) module operations.

### Error Breakdown (pattern_matcher.py)

**Total**: 46 errors
- **DEPYLER-0431 scope**: 15/46 (33%)
  - re.IGNORECASE constant: 4 errors
  - Option<Match> method calls: 7 errors (`.as_str()`, `.groups()`, `.start()`)
  - Option<Match> boolean conversion: 2 errors
  - compiled.r#match() method: 1 error
  - enumerate() destructuring: 2 errors (indirect, related to match variable scope)
- **Out of scope**: 31/46 (67%)
  - Argparse subcommand field access: 11 errors (DEPYLER-0425 related)
  - Type inference: 20 errors (serde_json::Value vs &str, return types)

### Issue 1: re.IGNORECASE Constant (4 errors)

**Current (WRONG)**:
```python
flags = re.IGNORECASE if ignore_case else 0
match = re.match(pattern, text, flags)
```

**Generated Rust (INCORRECT)**:
```rust
let flags = if ignore_case { re.IGNORECASE } else { 0 };  // ❌ E0423: expected value, found crate `re`
let match = regex::Regex::new(pattern).unwrap().find(text);
```

**Expected (CORRECT)**:
```rust
// Option 1: Use RegexBuilder for case-insensitive matching
let regex = if ignore_case {
    regex::RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .unwrap()
} else {
    regex::Regex::new(pattern).unwrap()
};
let match_result = regex.find(text);

// Option 2: Ignore flags parameter (simpler for basic cases)
let match_result = regex::Regex::new(pattern).unwrap().find(text);
```

**Root Cause**:
- `re.IGNORECASE` is a Python constant that doesn't exist in Rust's regex crate
- The flags parameter is being transpiled literally instead of being integrated into regex construction
- Rust's regex crate uses builder pattern for options, not flags

### Issue 2: Option<Match> Method Calls (7 errors)

**Current (WRONG)**:
```python
match = re.search(pattern, text)
if match:
    print(f"Match: {match.group(0)}")
    print(f"Groups: {match.groups()}")
    print(f"Position: {match.start()}")
```

**Generated Rust (INCORRECT)**:
```rust
let match = regex::Regex::new(pattern).unwrap().find(text);  // Returns Option<Match>
if match {  // ❌ E0308: Option<Match> is not bool
    println!("Match: {}", match.as_str());  // ❌ E0599: no method `as_str` on Option<T>
    println!("Groups: {}", match.groups());  // ❌ E0599: no method `groups` on Option<T>
    println!("Position: {}", match.start());  // ❌ E0599: no method `start` on Option<T>
}
```

**Expected (CORRECT)**:
```rust
let match_result = regex::Regex::new(pattern).unwrap().find(text);
if let Some(m) = match_result {
    println!("Match: {}", m.as_str());  // ✅ Works on Match
    // Note: .groups() doesn't exist in regex crate, need workaround
    println!("Position: {}", m.start());  // ✅ Works on Match
}
```

**Root Cause**:
- `regex::Regex::find()` returns `Option<Match>`, not `Match`
- Methods like `.as_str()`, `.start()` exist on `Match`, not `Option<Match>`
- Python's `match` object is never None when used in `if match:`, but Rust's is `Option<Match>`
- Transpiler needs to detect this pattern and generate `if let Some(m) = match`

### Issue 3: Option<Match> Boolean Conversion (2 errors)

**Current (WRONG)**:
```python
match = re.match(pattern, text)
if match:  # Python: truthy check (None is falsy)
    ...
```

**Generated Rust (INCORRECT)**:
```rust
let match = regex::Regex::new(pattern).unwrap().find(text);
if match {  // ❌ E0308: expected `bool`, found `Option<Match>`
    ...
}
```

**Expected (CORRECT)**:
```rust
let match_result = regex::Regex::new(pattern).unwrap().find(text);
if let Some(m) = match_result {  // ✅ Pattern matching
    ...
}
// OR
if match_result.is_some() {  // ✅ Boolean method
    ...
}
```

**Root Cause**: Python's truthy/falsy semantics don't directly map to Rust's type system

### Issue 4: compiled.r#match() Method (1 error)

**Current (WRONG)**:
```python
compiled = re.compile(pattern)
match = compiled.match(email)
```

**Generated Rust (INCORRECT)**:
```rust
let compiled = regex::Regex::new(pattern).unwrap();
let match = compiled.r#match(email);  // ❌ E0599: no method named `r#match`
```

**Expected (CORRECT)**:
```rust
let compiled = regex::Regex::new(pattern).unwrap();
let match_result = compiled.find(email);  // ✅ Use .find() instead of .match()
```

**Root Cause**:
- Rust's regex crate uses `.find()` for pattern matching, not `.match()`
- Python's `re.match()` only matches at start of string, but Rust's `.find()` searches anywhere
- Should use `.find()` with `^` anchor in pattern for equivalent behavior

### Issue 5: match.groups() Method (NOT IMPLEMENTED)

**Python API**:
```python
match = re.search(r"(\d+)-(\d+)", "123-456")
groups = match.groups()  # Returns ("123", "456")
```

**Rust Challenge**:
```rust
// regex crate doesn't have .groups() method
// Need to extract all capture groups manually
let caps = regex.captures(text).unwrap();
let groups = (1..caps.len())
    .map(|i| caps.get(i).map(|m| m.as_str()).unwrap_or(""))
    .collect::<Vec<_>>();
```

**Root Cause**: Fundamental API difference between Python `re` and Rust `regex` crate

### Issue 6: enumerate() Destructuring in For Loops (2 errors)

**Current (WRONG)**:
```python
for i, match in enumerate(matches, 1):
    print(f"  {i}. {match}")
```

**Generated Rust (INCORRECT)**:
```rust
for (_i, _match) in matches.into_iter().enumerate().map(|(i, x)| ((i + 1) as i32, x)) {
    println!("  {}. {}", i, r#match);  // ❌ E0425: cannot find value `i` or `r#match`
}
```

**Expected (CORRECT)**:
```rust
for (i, match_str) in matches.into_iter().enumerate().map(|(i, x)| ((i + 1) as i32, x)) {
    println!("  {}. {}", i, match_str);  // ✅ Use the correct variable names
}
```

**Root Cause**: Variable names `_i` and `_match` are prefixed with `_` but then referenced without prefix

## Root Cause Analysis

### Core Issues

1. **Flags vs Builder Pattern**
   - Python uses integer flags (`re.IGNORECASE`)
   - Rust uses builder pattern (`RegexBuilder::new().case_insensitive(true)`)
   - Transpiler needs to detect flags usage and convert to builder

2. **Option<Match> Handling**
   - Python's `match` object is truthy/falsy
   - Rust's `find()` returns `Option<Match>`
   - Transpiler needs to generate `if let Some(m)` pattern

3. **API Differences**
   - Python: `compiled.match()`, `compiled.search()`
   - Rust: `compiled.find()` for both (with `^` anchor for match-at-start)
   - Need to map Python methods to appropriate Rust equivalents

4. **Missing .groups() API**
   - Python has `.groups()` returning all capture groups
   - Rust requires manual extraction from `Captures`
   - Need to implement helper or inline logic

## Files Affected

### Primary Implementation:
- `crates/depyler-core/src/rust_gen/expr_gen.rs`
  - Update: `try_convert_re_method()` for better Match handling
  - Add: Option<Match> detection and conversion logic
  - Fix: `compiled.match()` → `compiled.find()`

### Test Files:
- `crates/depyler-core/tests/depyler_0431_regex_improvements.rs` (NEW)

## Test Plan

### Unit Tests (depyler_0431_regex_improvements.rs)

```rust
#[test]
fn test_DEPYLER_0431_01_re_search_option_handling() {
    // Python: match = re.search(pattern, text); if match: ...
    // Expected: if let Some(m) = regex.find(text)
}

#[test]
fn test_DEPYLER_0431_02_match_as_str() {
    // Python: match.group(0)
    // Expected: m.as_str() where m: Match (not Option<Match>)
}

#[test]
fn test_DEPYLER_0431_03_match_start() {
    // Python: match.start()
    // Expected: m.start() where m: Match
}

#[test]
fn test_DEPYLER_0431_04_compiled_match() {
    // Python: compiled.match(text)
    // Expected: compiled.find(text)
}

#[test]
fn test_DEPYLER_0431_05_match_groups() {
    // Python: match.groups()
    // Expected: Extract all capture groups manually
}

#[test]
fn test_DEPYLER_0431_06_re_ignorecase_flag() {
    // Python: re.match(pattern, text, re.IGNORECASE)
    // Expected: RegexBuilder::new(pattern).case_insensitive(true) OR ignore flags
}

#[test]
fn test_DEPYLER_0431_07_pattern_matcher_integration() {
    // Full pattern_matcher.py transpilation
    // Verify 15/46 errors fixed
}
```

### Integration Tests

1. **pattern_matcher.py compilation**: 46 errors → ~31 errors (15 fixed, 33% reduction)
2. **Regex operations**: All re.search/match/findall work
3. **Option<Match> handling**: Proper pattern matching

## Implementation Plan

### Phase 1: RED - Write Failing Tests ✅
```bash
# Create test file
touch crates/depyler-core/tests/depyler_0431_regex_improvements.rs

# Add 7 tests (6 unit + 1 integration)
cargo test test_DEPYLER_0431  # MUST FAIL initially
```

### Phase 2: GREEN - Implement Fixes

**Step 1: Fix Option<Match> handling in re.search/match**

Detect pattern:
```python
match = re.search(pattern, text)
if match:
    ... match.group(0) ...
```

Generate:
```rust
if let Some(m) = regex::Regex::new(pattern).unwrap().find(text) {
    ... m.as_str() ...
}
```

**Step 2: Fix compiled.r#match() → compiled.find()**

In `try_convert_re_method()`:
```rust
"match" => {
    // Python re.match() → Rust .find() with ^ anchor
    // Or just use .find() if pattern already has ^
    let text_arg = &args[0];
    Some(quote! { #object_expr.find(#text_arg) })
}
```

**Step 3: Handle re.IGNORECASE (simplified approach)**

For now, detect and remove flags parameter:
```rust
// Python: re.match(pattern, text, flags)
// Rust: regex::Regex::new(pattern).unwrap().find(text)
// Ignore flags for initial implementation
```

**Step 4: Implement .groups() workaround**

Detect `.groups()` call and generate extraction logic:
```rust
"groups" => {
    // Generate code to extract all capture groups
    Some(quote! {
        {
            let caps = #object_expr.captures(text).unwrap();
            (1..caps.len())
                .map(|i| caps.get(i).map(|m| m.as_str().to_string()).unwrap_or_default())
                .collect::<Vec<_>>()
        }
    })
}
```

### Phase 3: REFACTOR - Clean Up + Edge Cases
- Handle pattern anchoring for `.match()` vs `.search()`
- Ensure complexity ≤10, test coverage ≥80%
- Handle edge cases (no match, empty groups, etc.)

## Verification Checklist

- [ ] All 7 unit tests passing
- [ ] pattern_matcher.py errors: 46 → ~31 (15 fixed, 33% reduction)
- [ ] Regex operations work correctly
- [ ] Option<Match> handled properly (no more E0308/E0599 errors)
- [ ] Complexity ≤10 (pmat analyze complexity)
- [ ] Coverage ≥80% (cargo llvm-cov)
- [ ] No clippy warnings (cargo clippy -D warnings)

## Success Criteria

**MUST ACHIEVE**:
1. ✅ pattern_matcher.py: 46 errors → ~31 errors (15 fixed, 33% reduction)
2. ✅ `re.search()`, `re.match()` return `Option<Match>` handled correctly
3. ✅ `match.group(0)`, `match.start()` work (on unwrapped Match)
4. ✅ `compiled.match()` → `compiled.find()` conversion
5. ✅ All quality gates pass (complexity, coverage, clippy)

**Compilation Progress**:
- Current: 4/13 (30.8%)
- After DEPYLER-0431: pattern_matcher closer to compilation (depends on fixing argparse + type inference)
- Target (after MEDIUM tickets): 10-11/13 (77-85%)

## Time Tracking

- **Debug & Analysis**: 1 hour (DONE)
- **RED Phase**: 30-45 min (estimated)
- **GREEN Phase**: 1.5-2 hours (estimated)
- **REFACTOR Phase**: 30 min (estimated)
- **Total**: 2.5-3.5 hours

## Related Tickets

- **DEPYLER-0428**: Exception flow (COMPLETE)
- **DEPYLER-0430**: os/sys/platform (COMPLETE)
- **DEPYLER-0435**: Master ticket (IN PROGRESS)
- **DEPYLER-0432**: sys.stdin/stdout (NOT STARTED)

## References

- Rust regex crate: https://docs.rs/regex/latest/regex/
- Python re module: https://docs.python.org/3/library/re.html
- Regex crate Match type: https://docs.rs/regex/latest/regex/struct.Match.html
- Regex crate Captures: https://docs.rs/regex/latest/regex/struct.Captures.html

---

## Debugging Notes

### Error Count by Category

```
Total: 46 errors (pattern_matcher.py)
├── DEPYLER-0431 scope: 15 (33%)
│   ├── re.IGNORECASE: 4 (E0423)
│   ├── Option<Match> methods: 7 (E0599)
│   ├── Option<Match> bool: 2 (E0308)
│   ├── compiled.r#match: 1 (E0599)
│   └── enumerate vars: 2 (E0425) - indirect
└── Out of scope: 31 (67%)
    ├── Argparse subcommands: 11 (field access)
    └── Type inference: 20 (serde_json::Value, return types)
```

### Already Implemented ✅

- `re.compile(pattern)` → `regex::Regex::new(pattern).unwrap()`
- `re.findall(pattern, text)` → `.find_iter(text).map(|m| m.as_str()).collect()`
- `re.sub(pattern, replacement, text)` → `.replace_all(text, replacement)`
- `regex::Regex::new()` basic usage

### Missing Implementations ❌

**Match Object Handling**:
- `if match:` → `if let Some(m) = match_result` (Option pattern)
- `match.group(0)` → `m.as_str()` (on Match, not Option<Match>)
- `match.groups()` → manual capture group extraction
- `match.start()` → `m.start()` (on Match, not Option<Match>)

**API Differences**:
- `compiled.match(text)` → `compiled.find(text)`
- `re.IGNORECASE` flag → RegexBuilder or ignore

**Variable Scoping**:
- `for (_i, _match)` → use variables without underscore prefix

---

**STATUS**: Analysis complete, ready for RED phase
**NEXT STEP**: `pmat prompt show continue DEPYLER-0431` to begin RED phase
