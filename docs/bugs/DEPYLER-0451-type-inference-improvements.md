# DEPYLER-0451: Type Inference Improvements

**Status**: 🔴 RED PHASE (Analysis Complete, Implementation Pending)
**Created**: 2025-11-21
**Priority**: P0 (CRITICAL - STOP THE LINE)
**Parent**: DEPYLER-0435 (reprorusted-python-cli 100% compilation)
**Effort Estimate**: 12-16 hours (HIGH complexity)

---

## Executive Summary

62% of remaining compilation errors (45/73) are **type inference** issues where depyler defaults to `serde_json::Value` instead of inferring concrete types from usage context. This causes cascading E0308 (mismatched types), E0282 (type annotations needed), E0277 (trait bounds), and E0425 (undefined variable) errors.

**Impact**: Fixing type inference will resolve 45+ errors across 3 examples, bringing compilation rate from 30% → ~70%+ (9-10/13 examples).

---

## Problem Statement

###Current Behavior

Depyler's type inference system currently:
1. **Defaults to `serde_json::Value`** for unannotated parameters/variables
2. **Fails to propagate types** from usage context (method calls, operators)
3. **Doesn't infer from control flow** (if/match patterns, assignments)

**Example 1: CSV Reader Iteration**
```python
def process_csv(filepath):
    reader = csv.DictReader(open(filepath))
    for row in reader:  # row should be Dict<String, String>
        print(row['name'])
```

**Current (WRONG)**:
```rust
pub fn process_csv(filepath: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let reader = csv::Reader::from_path(filepath)?;  // ❌ E0277: Value doesn't impl AsRef<Path>
    for row in reader.iter() {  // ❌ E0599: Reader doesn't have .iter()
        let _cse_temp_0 = row.get("name");  // row type unknown
        println!("{}", _cse_temp_0);
    }
    Ok(())
}
```

**Expected (CORRECT)**:
```rust
pub fn process_csv(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new().from_path(filepath)?;
    for result in reader.deserialize() {  // Correct csv iteration pattern
        let row: HashMap<String, String> = result?;
        println!("{}", row.get("name").unwrap_or(&String::new()));
    }
    Ok(())
}
```

**Example 2: File Operations**
```python
def read_lines(filepath):
    with open(filepath) as f:
        for line in f:
            process(line)
```

**Current (WRONG)**:
```rust
pub fn read_lines(filepath: serde_json::Value) -> Result<(), std::io::Error> {
    let f = std::fs::File::open(filepath)?;  // ❌ E0277: Value doesn't impl AsRef<Path>
    for line in f.iter() {  // ❌ E0599: File doesn't have .iter()
        process(line);
    }
    Ok(())
}
```

**Expected (CORRECT)**:
```rust
pub fn read_lines(filepath: &str) -> Result<(), std::io::Error> {
    let f = std::fs::File::open(filepath)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {  // Correct file iteration
        let line = line?;
        process(&line);
    }
    Ok(())
}
```

### Error Manifestations

**1. E0308: Mismatched Types** (17 occurrences)
- `serde_json::Value` passed where concrete type expected
- `bool` vs `Value` in conditionals
- String vs `&str` vs `Value`

**2. E0282: Type Annotations Needed** (5 occurrences)
- Variables with inferred type `Value` but used as concrete type
- Ambiguous tuple/struct destructuring

**3. E0277: Trait Bound Not Satisfied** (9 occurrences)
- `Value: AsRef<Path>` for file paths
- `Value: Ord` for comparisons
- `Value: Display` for formatting

**4. E0425: Cannot Find Value** (11 occurrences)
- Variables not inferred due to type propagation failures
- Scope issues from incorrect type handling

---

## Root Cause Analysis

### Current Type Inference Flow

1. **Function Parameters**: Default to `Value` if no type hint
2. **Local Variables**: Inferred from RHS expression OR default to `Value`
3. **Return Types**: Inferred from explicit return OR `Type::None`

**Gaps**:
- ❌ No **backward type propagation** (usage → definition)
- ❌ No **flow-sensitive typing** (if guards, match arms)
- ❌ No **context-aware inference** (stdlib API signatures)

### File Locations

**Type Inference System**:
- `crates/depyler-core/src/type_hints.rs` - Core type inference logic
- `crates/depyler-core/src/ast_bridge/types.rs` - Type representations
- `crates/depyler-core/src/rust_gen/func_gen.rs` - Function signature generation
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Expression type checking

---

## Solution Design

### Phase 1: Backward Type Propagation (4-6 hours)

**Goal**: Infer parameter types from usage context

**Implementation**:
1. **Scan function body** for parameter usage patterns
2. **Collect constraints** (method calls, operators, stdlib APIs)
3. **Unify constraints** to most specific type
4. **Fallback to Value** only if ambiguous

**Example**:
```python
def process(filepath):  # Parameter type unknown
    with open(filepath) as f:  # Usage: filepath used in open() → must be str-like
        ...
```

**Constraint Collection**:
- `open(filepath)` → `filepath: impl AsRef<Path>` → **infer &str**
- `reader.iter()` → `reader: impl Iterator` → **check csv::Reader methods**

**Files**:
- `type_hints.rs`: Add `infer_from_usage()` function
- `func_gen.rs`: Call `infer_from_usage()` before generating signature

### Phase 2: Flow-Sensitive Typing (3-4 hours)

**Goal**: Narrow types based on control flow

**Implementation**:
1. **Track type refinements** in if/match guards
2. **Propagate narrowed types** to dominated blocks
3. **Handle type guards** (isinstance, hasattr patterns)

**Example**:
```python
def handle(value):
    if isinstance(value, str):
        return value.upper()  # value: str in this block
    else:
        return str(value)  # value: Any in this block
```

**Files**:
- `types.rs`: Add `TypeEnvironment` for scoped type tracking
- `stmt_gen.rs`: Update if/match handling to track type refinements

### Phase 3: Context-Aware Stdlib API Inference (3-4 hours)

**Goal**: Infer types from stdlib API signatures

**Implementation**:
1. **Define API signatures** for common stdlib patterns:
   - `csv.DictReader()` → returns `Iterator<HashMap<String, String>>`
   - `open()` → takes `impl AsRef<Path>`, returns `File`
   - `subprocess.run()` → takes `&[&str]`, returns `CompletedProcess`
2. **Use signatures** during type inference
3. **Generate correct Rust equivalents** based on inferred types

**Files**:
- `type_hints.rs`: Add stdlib API signature database
- `expr_gen.rs`: Use signatures when transpiling calls

### Phase 4: Type Propagation Through Assignments (2-3 hours)

**Goal**: Propagate types through variable assignments

**Implementation**:
1. **Track variable definitions** and uses
2. **Propagate types** from RHS → LHS in assignments
3. **Unify types** across multiple assignments

**Example**:
```python
result = None  # Initial: Type::None
if condition:
    result = get_string()  # Type: str
else:
    result = "default"  # Type: str
# Unified: result: str (not Option<str> yet, keep simple)
```

---

## Test Plan

### Test Suite: `depyler_0451_type_inference.rs`

**Test 1: File Path Parameter Inference**
```python
def read_file(filepath):
    with open(filepath) as f:
        return f.read()
```
**Expected**: `filepath: &str` (not `Value`)

**Test 2: CSV Reader Type Inference**
```python
def process_csv(path):
    reader = csv.DictReader(open(path))
    for row in reader:
        print(row['name'])
```
**Expected**:
- `path: &str`
- `row: HashMap<String, String>`
- Correct csv iteration pattern

**Test 3: Integer Parameter Inference**
```python
def increment(x):
    return x + 1
```
**Expected**: `x: i32` (from + operator with literal 1)

**Test 4: String Parameter Inference**
```python
def greet(name):
    return f"Hello, {name}"
```
**Expected**: `name: &str` (from f-string usage)

**Test 5: List Parameter Inference**
```python
def sum_list(items):
    total = 0
    for item in items:
        total += item
    return total
```
**Expected**: `items: &[i32]` (from += with int literal)

**Test 6: Mixed Type Inference**
```python
def process(data, flag):
    if flag:
        return data.upper()
    return data
```
**Expected**:
- `data: &str` (from .upper() method)
- `flag: bool` (from if condition)

**Test 7: Backward Propagation from Return**
```python
def get_number():
    x = compute()
    return x + 1
```
**Expected**: `compute()` returns int-like, `x: i32`

---

## Implementation Phases

### RED Phase (2 hours)
1. Create test suite with 7+ failing tests
2. Document expected vs actual output
3. Commit: `[RED] DEPYLER-0451: Add type inference test suite`

### GREEN Phase (10-12 hours)
1. Implement Phase 1: Backward type propagation (4-6h)
2. Implement Phase 2: Flow-sensitive typing (3-4h)
3. Implement Phase 3: Context-aware stdlib (3-4h)
4. Implement Phase 4: Assignment propagation (2-3h)
5. Iterate until tests pass
6. Commit: `[GREEN] DEPYLER-0451: Type inference improvements`

### REFACTOR Phase (2-3 hours)
1. Optimize type unification algorithm
2. Add caching for repeated inference
3. Ensure complexity ≤10
4. Document type inference algorithm
5. Commit: `[REFACTOR] DEPYLER-0451: Optimize type inference`

---

## Expected Impact

### Error Reduction

**csv_filter** (15 errors):
- E0277: `Value: AsRef<Path>` (2x) → **FIXED**
- E0599: `.iter()` on Reader (2x) → **FIXED** (correct iteration pattern)
- E0308: mismatched types (2x) → **FIXED**
- E0282: type annotations (1x) → **FIXED**
- **Expected**: 15 → 8 errors (47% reduction)

**log_analyzer** (26 errors):
- E0308: mismatched types (15x) → **12 FIXED** (3 remain for generators)
- E0277: `Value: Ord` (2x) → **FIXED**
- E0425: undefined variables (4x) → **FIXED** (type propagation)
- **Expected**: 26 → 8 errors (69% reduction)

**stream_processor** (32 errors):
- E0277: trait bounds (9x) → **FIXED**
- E0599: method not found (6x) → **FIXED**
- E0425: undefined variables (4x) → **FIXED**
- E0308: mismatched types (2x) → **FIXED**
- **Expected**: 32 → 11 errors (66% reduction)

### Compilation Rate

**Before**: 4/13 (30.8%)
**After**: **10-11/13 (77-85%)** 🚀

**Newly Compiling**:
- csv_filter (likely) ✅
- log_analyzer (likely) ✅
- stream_processor (likely) ✅
- config_manager (partial)
- env_info (partial)
- stdlib_integration (partial)

---

## Risks and Mitigations

### Risk 1: Inference Ambiguity

**Issue**: Multiple valid types for same variable
**Example**: `x` used in both int and str contexts

**Mitigation**:
- Prefer more specific types
- Fall back to `Value` with clear warning
- Add `// Type inference ambiguous: ...` comment

### Risk 2: Performance Regression

**Issue**: Type inference adds analysis overhead

**Mitigation**:
- Cache inference results
- Use lazy evaluation
- Add `--no-type-inference` flag for debugging

### Risk 3: Breaking Changes

**Issue**: Existing code depends on `Value` types

**Mitigation**:
- Feature flag: `--enable-type-inference` (default: on)
- Gradual rollout
- Comprehensive test suite

---

## Success Criteria

1. **Test Suite**: 7/7 tests passing ✅
2. **Error Reduction**: ≥60% across csv_filter, log_analyzer, stream_processor
3. **Compilation Rate**: ≥10/13 examples (77%)
4. **Quality Gates**: Complexity ≤10, TDG ≤2.0, SATD=0
5. **No Regressions**: All existing tests pass

---

## Related Tickets

- **DEPYLER-0435** (parent): reprorusted-python-cli 100% compilation
- **DEPYLER-0429**: Deferred due to type inference issues
- **DEPYLER-0431**: Regex type inference issues
- **DEPYLER-0436**: Argparse type validator inference (partial solution)

---

## Next Steps

1. Create test suite (RED phase)
2. Implement backward type propagation
3. Iterate on remaining phases
4. Re-test all examples
5. Update parent ticket progress

**Start Command**: `pmat prompt show continue DEPYLER-0451`
