# DEPYLER-0452: CSV/Stdlib API Codegen Fix

**Status**: 🔴 RED PHASE (Analysis Complete, Implementation Pending)
**Created**: 2025-11-21
**Priority**: P0 (CRITICAL - HIGH IMPACT)
**Parent**: DEPYLER-0435 (reprorusted-python-cli 100% compilation)
**Predecessor**: DEPYLER-0451 (Type Inference - completed, minimal impact)
**Effort Estimate**: 8-12 hours (MEDIUM-HIGH complexity)

---

## Executive Summary

40% of remaining compilation errors (16/74 in csv_filter alone) are **stdlib API codegen mismatches** where depyler generates incorrect Rust API calls for Python stdlib functions (csv, file I/O). This causes E0599 (method not found), E0609 (no such field), and E0277 (trait bounds) errors.

**Impact**: Fixing CSV API codegen will resolve 16 errors in csv_filter, bringing it from **non-compiling → COMPILING** ✅. Additional fixes for file I/O patterns will impact stream_processor and log_analyzer.

**Expected Compilation Rate**: 4/13 (30.8%) → **6-7/13 (46-54%)** (+2-3 examples)

---

## Problem Statement

### Current Behavior

**Python Code**:
```python
import csv

def filter_csv(input_file, column, value):
    with open(input_file) as f:
        reader = csv.DictReader(f)
        fieldnames = reader.fieldnames  # Property access

        for row in reader:  # Direct iteration
            if row[column] == value:
                yield row
```

**Current Rust (WRONG)**:
```rust
let f = std::fs::File::open(input_file)?;
let reader = csv::ReaderBuilder::new()
    .has_headers(true)
    .from_reader(f);

let fieldnames = reader.fieldnames;  // ❌ E0609: no field `fieldnames`
let filtered_rows = reader.iter()    // ❌ E0599: no method `iter()`
    .copied()
    .filter(|row| row.get(column as usize)  // ❌ Wrong API
        .cloned()
        .unwrap_or_default() == value)
    .map(|row| row);
```

**Correct Rust (EXPECTED)**:
```rust
let f = std::fs::File::open(input_file)?;
let mut reader = csv::ReaderBuilder::new()  // ✅ mut required
    .has_headers(true)
    .from_reader(f);

let fieldnames = reader.headers()?.clone();  // ✅ Use headers() method

for result in reader.deserialize::<HashMap<String, String>>() {  // ✅ Correct iteration
    let row: HashMap<String, String> = result?;
    if row.get(column) == Some(&value) {  // ✅ Correct HashMap API
        // yield equivalent (generator transformation)
    }
}
```

### Error Manifestations

**csv_filter (16 errors)**:
```
error[E0609]: no field `fieldnames` on type `Reader<File>`
  --> csv_filter.rs:XX:YY
   |
   | let fieldnames = reader.fieldnames;
   |                          ^^^^^^^^^^

error[E0599]: no method named `iter` found for struct `Reader<R>`
  --> csv_filter.rs:XX:YY
   |
   | let filtered_rows = reader.iter()
   |                            ^^^^
```

**Root Causes**:
1. ❌ Python `reader.fieldnames` → Rust `reader.fieldnames` (WRONG: use `reader.headers()`)
2. ❌ Python `for row in reader` → Rust `reader.iter()` (WRONG: use `reader.deserialize()`)
3. ❌ Python `row[column]` → Rust `row.get(column as usize)` (WRONG: DictReader uses string keys)
4. ❌ Generator expressions not transformed to proper Rust iterators

---

## Root Cause Analysis

### File Locations

**CSV Codegen**:
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Expression translation
- `crates/depyler-core/src/rust_gen/stmt_gen.rs` - Statement/loop translation
- `crates/depyler-core/src/stdlib_mappings.rs` - Stdlib API mapping (TO BE CREATED)

### Current CSV Codegen Logic

**Problem 1: Attribute Access**
```rust
// In expr_gen.rs
HirExpr::Attribute { base, attr } => {
    // Generic attribute translation
    format!("{}.{}", self.generate_expr(base)?, attr)
}
```
❌ **Issue**: Treats all attributes uniformly, doesn't know `reader.fieldnames` → `reader.headers()?`

**Problem 2: Iteration Pattern**
```rust
// In stmt_gen.rs
HirStmt::For { target, iter, body } => {
    // Generic iteration translation
    format!("for {} in {}.iter() {{", target, iter)
}
```
❌ **Issue**: Assumes `.iter()` method exists on all iterables, doesn't handle CSV Reader's `.deserialize()`

**Problem 3: Item Access**
```rust
// In expr_gen.rs
HirExpr::Index { base, index } => {
    // Generic indexing translation
    format!("{}.get({} as usize)", base, index)
}
```
❌ **Issue**: Assumes numeric indexing, doesn't know DictReader uses string keys

---

## Solution Design

### Phase 1: Stdlib API Mapping Database (3-4 hours)

**Goal**: Create stdlib API signature database for correct code generation

**Implementation**:
1. Create `crates/depyler-core/src/stdlib_mappings.rs`
2. Define `StdlibApiMapping` struct:
   ```rust
   struct StdlibApiMapping {
       module: &'static str,
       class: &'static str,
       method: &'static str,
       rust_pattern: RustPattern,
   }

   enum RustPattern {
       MethodCall { method: &'static str, args: Vec<&'static str> },
       FieldAccess { field: &'static str },
       CustomCodegen { template: &'static str },
   }
   ```
3. Add mappings for csv module:
   ```rust
   const CSV_MAPPINGS: &[StdlibApiMapping] = &[
       StdlibApiMapping {
           module: "csv",
           class: "DictReader",
           method: "fieldnames",
           rust_pattern: RustPattern::MethodCall {
               method: "headers",
               args: vec![],
           },
       },
       StdlibApiMapping {
           module: "csv",
           class: "DictReader",
           method: "__iter__",
           rust_pattern: RustPattern::CustomCodegen {
               template: "deserialize::<HashMap<String, String>>()",
           },
       },
   ];
   ```

**Files**:
- **NEW**: `stdlib_mappings.rs` (~200 lines)
- **MODIFIED**: `expr_gen.rs` (integrate mappings)
- **MODIFIED**: `stmt_gen.rs` (use mappings for iteration)

### Phase 2: CSV-Specific Codegen (3-4 hours)

**Goal**: Apply stdlib mappings to CSV operations

**Implementation**:
1. Detect CSV Reader usage in HIR
2. Transform attribute access:
   - `reader.fieldnames` → `reader.headers()?.clone()`
3. Transform iteration:
   - `for row in reader` → `for result in reader.deserialize::<HashMap<String, String>>() { let row = result?; ...}`
4. Transform item access on dict rows:
   - `row[column]` → `row.get(column)`

**Example Transformation**:
```python
# Python
reader = csv.DictReader(f)
fieldnames = reader.fieldnames
for row in reader:
    print(row['name'])
```

```rust
// Rust (generated)
let mut reader = csv::ReaderBuilder::new()
    .has_headers(true)
    .from_reader(f);
let fieldnames = reader.headers()?.clone();

for result in reader.deserialize::<HashMap<String, String>>() {
    let row: HashMap<String, String> = result?;
    println!("{}", row.get("name").unwrap_or(&String::new()));
}
```

**Files**:
- `expr_gen.rs`: Add `generate_csv_attribute()` method
- `stmt_gen.rs`: Add `generate_csv_iteration()` method

### Phase 3: File I/O Pattern Fixes (2-3 hours)

**Goal**: Fix file iteration patterns

**Current (WRONG)**:
```python
with open(filepath) as f:
    for line in f:
        process(line)
```

**Generated (WRONG)**:
```rust
let f = std::fs::File::open(filepath)?;
for line in f.iter() {  // ❌ File doesn't have iter()
    process(line);
}
```

**Expected (CORRECT)**:
```rust
let f = std::fs::File::open(filepath)?;
let reader = BufReader::new(f);
for line_result in reader.lines() {
    let line = line_result?;
    process(&line);
}
```

**Implementation**:
1. Detect `for x in file_var` patterns
2. Generate `BufReader::new()` wrapper
3. Use `.lines()` iterator
4. Handle `Result<String, Error>` unwrapping

**Files**:
- `stmt_gen.rs`: Add `generate_file_iteration()` method

### Phase 4: Generator Expression Transformation (Optional, 2-3 hours)

**Goal**: Transform Python generators to Rust iterators

**Current (WRONG)**:
```python
filtered = (row for row in reader if row['age'] > 18)
```

**Generated (WRONG)**:
```rust
let filtered = reader.iter()  // ❌ Wrong API + generator semantics lost
    .filter(|row| row.get("age") > 18)
    .map(|row| row);
```

**Expected (CORRECT)**:
```rust
let filtered = reader
    .deserialize::<HashMap<String, String>>()
    .filter_map(|result| result.ok())
    .filter(|row| row.get("age")
        .and_then(|v| v.parse::<i32>().ok())
        .map(|age| age > 18)
        .unwrap_or(false));
```

**Note**: This may be deferred to DEPYLER-0455 if too complex.

---

## Test Plan

### Test Suite: `depyler_0452_csv_api.rs`

**Test 1: CSV DictReader Creation**
```python
def read_csv(filepath):
    with open(filepath) as f:
        reader = csv.DictReader(f)
        return list(reader)
```
**Expected**: Correct `ReaderBuilder` + `deserialize()` pattern

**Test 2: Fieldnames Access**
```python
def get_headers(filepath):
    reader = csv.DictReader(open(filepath))
    return reader.fieldnames
```
**Expected**: `reader.headers()?.clone()`

**Test 3: Row Iteration**
```python
def print_rows(filepath):
    for row in csv.DictReader(open(filepath)):
        print(row)
```
**Expected**: `for result in reader.deserialize() { let row = result?; ... }`

**Test 4: Row Item Access**
```python
def get_name(filepath):
    reader = csv.DictReader(open(filepath))
    for row in reader:
        print(row['name'])
```
**Expected**: `row.get("name").unwrap_or(&String::new())`

**Test 5: CSV Filtering**
```python
def filter_csv(filepath, column, value):
    reader = csv.DictReader(open(filepath))
    return [row for row in reader if row[column] == value]
```
**Expected**: Correct iteration + filtering + collect pattern

**Test 6: File Line Iteration**
```python
def read_lines(filepath):
    with open(filepath) as f:
        for line in f:
            process(line)
```
**Expected**: `BufReader::new(f).lines()` pattern

---

## Expected Impact

### Error Reduction

**csv_filter (16 errors → 5 errors, 69% reduction)**:
- E0609: fieldnames (2x) → **FIXED**
- E0599: iter() (2x) → **FIXED**
- E0308: type mismatches (2x) → **FIXED**
- E0606: invalid cast (1x) → **FIXED**
- E0282: type annotations (3x) → **FIXED**
- E0277: trait bounds (2x) → **FIXED**
- **Remaining**: Closure capture (1x), generator yield (1x), misc (3x)

**stream_processor (32 errors → 18 errors, 44% reduction)**:
- File iteration patterns (4x) → **FIXED**
- E0423: file macro usage (4x) → **FIXED**
- E0425: undefined vars (6x) → **PARTIALLY FIXED**
- **Remaining**: Generators, subprocess, tempfile

**log_analyzer (26 errors → 20 errors, 23% reduction)**:
- File patterns (2x) → **FIXED**
- Generator patterns (4x) → **PARTIALLY FIXED**
- **Remaining**: itertools, group_by, generators

### Compilation Rate

**Before**: 4/13 examples (30.8%)
**After**: **6-7/13 examples (46-54%)** 🚀

**Newly Compiling**:
- csv_filter: 16 → 5 errors (likely **COMPILING** ✅)
- possibly: example_simple, example_flags (if minor fixes complete them)

**Improved but Not Compiling**:
- stream_processor: 32 → 18 errors (progress toward compilation)
- log_analyzer: 26 → 20 errors (progress toward compilation)

---

## Implementation Phases

### RED Phase (2 hours)
1. Create test suite `depyler_0452_csv_api.rs` with 6+ failing tests
2. Document expected vs actual CSV API patterns
3. Commit: `[RED] DEPYLER-0452: Add CSV API codegen test suite`

### GREEN Phase (6-8 hours)
1. Implement Phase 1: Stdlib mapping database (3-4h)
2. Implement Phase 2: CSV-specific codegen (3-4h)
3. Implement Phase 3: File I/O patterns (2-3h)
4. (Optional) Phase 4: Generator transformation (2-3h)
5. Iterate until tests pass
6. Commit: `[GREEN] DEPYLER-0452: CSV/stdlib API codegen fix`

### REFACTOR Phase (2-3 hours)
1. Extract common patterns to mapping system
2. Add more stdlib mappings (json, subprocess, etc.)
3. Optimize lookup performance
4. Document mapping system
5. Commit: `[REFACTOR] DEPYLER-0452: Generalize stdlib mapping system`

---

## Risks and Mitigations

### Risk 1: Generator Transformation Complexity

**Issue**: Python generators are fundamentally different from Rust iterators
**Example**: `yield` requires full coroutine transformation

**Mitigation**:
- Defer complex generator cases to DEPYLER-0455
- Focus on simple list comprehension → iterator chains
- Use `.collect()` for now instead of true lazy evaluation

### Risk 2: Type Inference for HashMap Values

**Issue**: `row.get("name")` returns `Option<&String>`, not `String`
**Requires**: Proper Option handling in generated code

**Mitigation**:
- Add `.unwrap_or(&String::new())` or `.unwrap_or_default()`
- Generate Result propagation where appropriate
- Leverage DEPYLER-0450 (Result wrapping) work

### Risk 3: Performance Regression

**Issue**: Stdlib mapping lookups add overhead
**Solution**: Cache mappings in HashMap, O(1) lookup

---

## Success Criteria

1. **Test Suite**: 6/6 tests passing ✅
2. **Error Reduction**: csv_filter 16 → ≤5 errors (≥69% reduction)
3. **Compilation**: csv_filter **COMPILING** ✅
4. **Quality Gates**: Complexity ≤10, TDG ≤2.0, SATD=0
5. **No Regressions**: All existing tests pass

---

## Related Tickets

- **DEPYLER-0435** (parent): reprorusted-python-cli 100% compilation
- **DEPYLER-0451** (predecessor): Type inference (complete, minimal impact)
- **DEPYLER-0450**: Result return wrapping (leveraged for error handling)
- **DEPYLER-0453** (next): Closure environment capture
- **DEPYLER-0454** (next): Auto-detect missing dependencies

---

## Next Steps

1. Create test suite (RED phase)
2. Implement stdlib mapping system
3. Apply CSV-specific transformations
4. Fix file I/O patterns
5. Validate csv_filter compilation
6. Update parent ticket progress

**Start Command**: `pmat work continue DEPYLER-0452`
