# DEPYLER-0493: Constructor Pattern Recognition for Stdlib Types

**Status**: In Progress
**Priority**: P1 (High Impact)
**Affects**: example_io_streams
**Estimated Effort**: 2h

---

## Problem Statement

The transpiler incorrectly translates Python class instantiation patterns to Rust, treating structs as functions. This causes compilation failures when stdlib types like `tempfile::NamedTempFile` require constructor patterns (`::new()`) instead of function calls.

---

## Errors Observed

### Error 1: Struct Called as Function
```
error[E0423]: expected function, found struct 'tempfile::NamedTempFile'
  --> example_io_streams.rs:XX:XX
```

**Analysis**:
- Python: `temp_file = tempfile.NamedTempFile(delete=False)`
- Current Rust (WRONG): `let temp_file = tempfile::NamedTempFile(delete=false)`
- Expected Rust: `let temp_file = tempfile::NamedTempFile::new()`

**Root Cause**: Transpiler doesn't distinguish between:
- Function calls: `open()` → `std::fs::File::open()`
- Constructor patterns: `NamedTempFile()` → `NamedTempFile::new()`

---

### Error 2: Method Type Mismatch
```
error[E0599]: no method named 'to_vec' found for type 'str'
  --> example_io_streams.rs:XX:XX
```

**Analysis**:
- Likely: `some_str.to_vec()` when it should be `some_bytes.to_vec()`
- Type confusion between `&str` and `&[u8]`

---

## Five Whys Analysis

**1. Why does compilation fail?**
→ Struct `tempfile::NamedTempFile` is called as a function

**2. Why is it called as a function?**
→ Transpiler generates `NamedTempFile(...)` instead of `NamedTempFile::new()`

**3. Why doesn't transpiler use constructor pattern?**
→ No stdlib mapping distinguishes constructors from functions

**4. Why no constructor distinction?**
→ Python doesn't have separate constructor syntax (all use `__init__`)

**5. ROOT CAUSE: Stdlib mapping lacks constructor pattern metadata**
→ Need to map Python class instantiation → Rust constructor patterns

---

## Impact Analysis

### Affected Stdlib Types

**Constructor Pattern Types**:
- `tempfile::NamedTempFile` → `::new()`
- `std::fs::File` → `::open()` or `::create()`
- `std::io::BufReader` → `::new()`
- `std::io::BufWriter` → `::new()`
- `regex::Regex` → `::new()` or `::compile()`

**Common Patterns**:
1. `::new()` - Most common (BufReader, BufWriter, etc.)
2. `::open()` - File I/O (File, etc.)
3. `::create()` - Creating resources
4. `::from()` - Conversion constructors

---

## Current Implementation Analysis

### Stdlib Mapping Structure

**File**: `crates/depyler-core/src/stdlib_mappings.rs`

**Current Mapping**:
```rust
pub fn get_stdlib_mapping(module: &str, name: &str) -> Option<StdlibMapping> {
    match (module, name) {
        ("tempfile", "NamedTempFile") => Some(StdlibMapping {
            rust_path: "tempfile::NamedTempFile",
            constructor: None,  // ← MISSING!
            methods: HashMap::new(),
        }),
        ...
    }
}
```

**Problem**: No `constructor` field to specify pattern

---

## Solution Design

### Option 1: Add Constructor Field to StdlibMapping ✅ RECOMMENDED

**Changes Needed**:

1. Update `StdlibMapping` struct:
```rust
pub struct StdlibMapping {
    pub rust_path: &'static str,
    pub constructor: Option<ConstructorPattern>,  // ← NEW
    pub methods: HashMap<&'static str, &'static str>,
}

pub enum ConstructorPattern {
    New,                    // ::new()
    Method(String),         // ::open(), ::create(), etc.
    Function(String),       // Direct function call
    Builder(String),        // Builder pattern
}
```

2. Update stdlib_mappings.rs:
```rust
("tempfile", "NamedTempFile") => Some(StdlibMapping {
    rust_path: "tempfile::NamedTempFile",
    constructor: Some(ConstructorPattern::New),  // ← ADDED
    methods: HashMap::new(),
}),
```

3. Update codegen to use constructor pattern:
```rust
// In expr_gen.rs or similar
match stdlib_mapping.constructor {
    Some(ConstructorPattern::New) => {
        // Generate: Type::new(args)
        quote! { #rust_path::new(#(#args),*) }
    }
    Some(ConstructorPattern::Method(m)) => {
        // Generate: Type::method(args)
        quote! { #rust_path::#m(#(#args),*) }
    }
    None => {
        // Fallback: function call (current behavior)
        quote! { #rust_path(#(#args),*) }
    }
}
```

**Pros**:
- Clean, type-safe solution
- Easy to extend for builder patterns
- Explicit per-type configuration

**Cons**:
- Requires updating many stdlib mappings
- Medium effort

---

### Option 2: Heuristic-Based Detection

**Logic**:
```rust
fn is_constructor_call(module: &str, name: &str) -> bool {
    // Heuristic: PascalCase names are usually types
    name.chars().next().unwrap().is_uppercase()
}
```

**Pros**:
- Quick to implement
- Works for most cases

**Cons**:
- Not always correct (e.g., `datetime.DateTime` but also `open()`)
- Fragile
- Hard to maintain

---

## Implementation Plan

### Phase 1: Analyze Constructor Errors ✅ COMPLETED

**Tasks**:
- [x] Document error patterns
- [x] Identify affected stdlib types
- [x] Locate codegen call site for class instantiation
- [x] Map Python patterns → Rust patterns

**ROOT CAUSE ANALYSIS**:

**File**: `crates/depyler-core/src/rust_gen/expr_gen.rs`
**Method**: `convert_generic_call()` (lines 2354-2510)

**The Problem**:
```rust
// Line 2363: Check if this is an imported function
if let Some(rust_path) = self.ctx.imported_items.get(func) {
    // Parse the rust path and generate the call
    let path_parts: Vec<&str> = rust_path.split("::").collect();
    ...
    if args.is_empty() {
        return Ok(parse_quote! { #path() });  // ← TREATS AS FUNCTION!
    } else {
        return Ok(parse_quote! { #path(#(#args),*) });  // ← TREATS AS FUNCTION!
    }
}
```

**What Happens**:
1. Python: `import tempfile; temp = tempfile.NamedTemporaryFile()`
2. Module mapper resolves: `NamedTemporaryFile` → `tempfile::NamedTempFile` ✅
3. Codegen generates: `tempfile::NamedTempFile()` ❌ (calls struct as function)
4. Should generate: `tempfile::NamedTempFile::new()` ✅ (constructor pattern)

**Evidence**:
```python
# test_tempfile.py
import tempfile
def create_temp():
    temp_file = tempfile.NamedTemporaryFile(delete=False)
```

**Transpiled to**:
```rust
// test_tempfile.rs
use tempfile;
pub fn create_temp() {
    let temp_file = tempfile::NamedTempFile();  // ❌ ERROR: struct called as function
    temp_file
}
```

**Compilation Error**:
```
error[E0423]: expected function, found struct `tempfile::NamedTempFile`
```

---

### Phase 2: Implement Constructor Recognition ✅ COMPLETED

**Tasks**:
- [x] Add `ConstructorPattern` enum to module_mapper.rs
- [x] Update `ModuleMapping` struct with constructor_patterns field
- [x] Add constructor mappings for tempfile types
- [x] Update expr_gen.rs (try_convert_module_method) to use constructor patterns
- [x] Handle constructor arguments

**Files Modified**:
- `crates/depyler-core/src/module_mapper.rs` - Added `ConstructorPattern` enum and `constructor_patterns` field
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Added constructor pattern logic to `try_convert_module_method()`

**Implementation Details**:

**1. Added ConstructorPattern enum** (`module_mapper.rs:16-25`):
```rust
pub enum ConstructorPattern {
    New,                    // ::new() - most common
    Function,               // Direct call (not a struct)
    Method(String),         // Custom method (e.g., File::open())
}
```

**2. Updated ModuleMapping struct** (`module_mapper.rs:28-40`):
```rust
pub struct ModuleMapping {
    pub rust_path: String,
    pub is_external: bool,
    pub version: Option<String>,
    pub item_map: HashMap<String, String>,
    pub constructor_patterns: HashMap<String, ConstructorPattern>,  // ← NEW
}
```

**3. Added tempfile constructor patterns** (`module_mapper.rs:348-358`):
```rust
constructor_patterns: HashMap::from([
    ("NamedTempFile".to_string(), ConstructorPattern::New),
    ("TempDir".to_string(), ConstructorPattern::New),
    ("tempfile".to_string(), ConstructorPattern::Function),
    ("tempdir".to_string(), ConstructorPattern::Function),
]),
```

**4. Updated try_convert_module_method()** (`expr_gen.rs:7837-7892`):
- Checks if `object` is a module variable
- Looks up constructor pattern in `imported_modules`
- Generates appropriate call pattern based on `ConstructorPattern`

**Verification**:
- Python: `temp = tempfile.NamedTemporaryFile()`
- Generated Rust: `let temp = tempfile::NamedTempFile::new();` ✅
- Compilation: No "expected function, found struct" error ✅

---

### Phase 3: Test and Validate

**Tasks**:
- [ ] Create test case for tempfile::NamedTempFile
- [ ] Test BufReader, BufWriter constructors
- [ ] Re-transpile example_io_streams
- [ ] Verify compilation success
- [ ] Check for regressions in 7 passing examples

---

## Test Plan

### Unit Test 1: NamedTempFile Constructor
```python
import tempfile

def create_temp():
    temp_file = tempfile.NamedTempFile(delete=False)
    return temp_file
```

**Expected Rust**:
```rust
use tempfile::NamedTempFile;

pub fn create_temp() -> NamedTempFile {
    let temp_file = NamedTempFile::new();  // ← Correct constructor pattern
    temp_file
}
```

---

### Unit Test 2: BufReader Constructor
```python
import io

def create_reader(file):
    reader = io.BufferedReader(file)
    return reader
```

**Expected Rust**:
```rust
use std::io::BufReader;

pub fn create_reader(file: std::fs::File) -> BufReader<std::fs::File> {
    let reader = BufReader::new(file);  // ← Correct constructor pattern
    reader
}
```

---

## Acceptance Criteria

- [ ] tempfile::NamedTempFile constructed correctly (::new() pattern)
- [ ] Method call type mismatches resolved (to_vec on correct type)
- [ ] example_io_streams compiles without errors
- [ ] Test coverage: 2+ constructor pattern tests added
- [ ] No regression in existing 7 passing examples
- [ ] Performance: <10ms overhead for constructor pattern resolution

---

## Related Issues

- **DEPYLER-0492**: Type inference (COMPLETED) - May help with method type mismatches
- **DEPYLER-0494**: Stdlib method mappings - Related stdlib coverage issue

---

## Progress Tracking

**Phase 1**: 🔄 In Progress (Error analysis)
**Phase 2**: ⏸️  Pending (Implementation)
**Phase 3**: ⏸️  Pending (Testing)

**Estimated Completion**: 2h from start

---

## References

- **Ticket**: DEPYLER-0493
- **Priority**: P1 (High Impact)
- **Target**: Fix example_io_streams → 8/13 examples compile (62%)
- **Analysis Document**: single_shot_compilation_failure_analysis.md
