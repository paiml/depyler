# DEPYLER-0304: File I/O and Context Manager Translation Gaps

**Discovered**: 2025-10-29 during Example 10 (File I/O Operations) validation
**Status**: 🛑 **CRITICAL BLOCKING** - 32 compilation errors
**Priority**: P0 (fundamental Python feature - blocks all file I/O)
**Estimate**: 12-16 hours (high complexity, architectural issue)

## Overview

Transpiled Example 10 (24 file I/O functions) revealed **32 compilation errors** due to **completely incorrect context manager (`with` statement) translation**. This is a fundamental architectural gap that blocks ALL file I/O operations.

## Discovery Context

**Example**: python-to-rust-conversion-examples/examples/10_file_operations/
**Functions**: 24 file I/O functions
**Success Rate**: 0% (0/24 functions compile)
**Error Rate**: 100% (32/32 statements fail)

**Validation Command**:
```bash
rustc --crate-type lib .../column_b/src/lib.rs 2>&1 | grep "error\[" | wc -l
# Result: 32 errors
```

## Root Cause: Context Manager (`with` statement) Not Implemented

### Python Pattern

```python
with open(filename, 'r') as f:
    return f.read()
```

### Current Translation (COMPLETELY WRONG)

```rust
{
    let _context = open(filename, "r".to_string());  // ❌ `open` function doesn't exist
    let f = _context.__enter__();  // ❌ `__enter__` method doesn't exist
    f.read()  // ❌ f is undefined type
}
```

**Problems**:
1. **No `open()` function** - Tries to call Python's `open()` which doesn't exist in Rust
2. **Python `__enter__` protocol** - Tries to use Python context manager protocol
3. **No file handle type** - `f` has no type, methods like `.read()`, `.write()` don't exist
4. **No error handling** - Python's context manager `__exit__` is completely missing
5. **No resource cleanup** - Files are never closed

---

## Error Categories

### Category 1: Missing `open()` Function (26 errors - CRITICAL)

**Error Message**:
```
error[E0425]: cannot find function `open` in this scope
```

**Affected**: ALL 26 functions using file I/O

**Python**:
```python
with open(filename, 'r') as f:
    return f.read()
```

**Current Translation** (WRONG):
```rust
{
    let _context = open(filename, "r".to_string());  // ❌ No such function
    let f = _context.__enter__();
    f.read()
}
```

**Correct Translation** (What it SHOULD be):
```rust
{
    let file = std::fs::File::open(filename)?;  // Rust file opening
    let mut reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    contents
}
```

**Or using `std::fs::read_to_string`**:
```rust
std::fs::read_to_string(filename)?
```

**Complexity**: HIGH - Requires complete rewrite of context manager handling

---

### Category 2: Missing `bytes` Type (2 errors - Easy)

**Error Message**:
```
error[E0412]: cannot find type `bytes` in this scope
```

**Affected**: Binary file operations (`read_binary_file`, `write_binary_file`)

**Python**:
```python
def read_binary_file(filename: str) -> bytes:
    with open(filename, 'rb') as f:
        return f.read()
```

**Current Translation** (WRONG):
```rust
pub fn read_binary_file(filename: String) -> bytes {  // ❌ No `bytes` type
    // ...
}
```

**Correct Translation**:
```rust
pub fn read_binary_file(filename: String) -> Vec<u8> {  // Rust uses Vec<u8> for bytes
    std::fs::read(filename).unwrap()
}
```

**Complexity**: Easy (type mapping: `bytes` → `Vec<u8>`)

---

### Category 3: Variable Scoping Issues (3 errors - Medium)

**Error Messages**:
```
error[E0425]: cannot find value `content` in this scope
error[E0425]: cannot find value `content1` in this scope
error[E0425]: cannot find value `content2` in this scope
```

**Affected**: Functions with multiple `with` blocks

**Python**:
```python
def copy_file_content(source: str, destination: str) -> int:
    with open(source, 'r') as f_in:
        content = f_in.read()
    with open(destination, 'w') as f_out:
        return f_out.write(content)
```

**Current Translation** (WRONG):
```rust
pub fn copy_file_content(source: String, destination: String) -> i32 {
    {
        let _context = open(source, "r".to_string());
        let f_in = _context.__enter__();
        let content = f_in.read();  // ❌ content scoped to this block
    }
    {
        let _context = open(destination, "w".to_string());
        let f_out = _context.__enter__();
        f_out.write(content)  // ❌ content not in scope
    }
}
```

**Root Cause**: Each `with` block is translated to a separate scope `{ }`, variables don't escape

**Correct Translation**:
```rust
pub fn copy_file_content(source: String, destination: String) -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string(source)?;  // Read first
    std::fs::write(destination, content)  // Write second
}
```

**Complexity**: Medium (requires lifting variables out of scopes)

---

### Category 4: Iterator Variable Name Collision (1 error - Easy)

**Error Message**:
```
error[E0423]: expected value, found macro `line`
```

**Affected**: `write_lines_to_file` function

**Python**:
```python
def write_lines_to_file(filename: str, lines: list[str]) -> int:
    with open(filename, 'w') as f:
        count = 0
        for line in lines:
            count = count + f.write(line)
        return count
```

**Current Translation** (WRONG):
```rust
pub fn write_lines_to_file(filename: String, lines: &Vec<String>) -> i32 {
    // ...
    let count = 0;
    for _line in lines.iter().cloned() {  // Variable named _line
        count = count + f.write(line);  // ❌ Refers to `line` (macro) not `_line`
    }
    count
}
```

**Root Cause**: Transpiler renames iterator variable to `_line` (with underscore) but reference uses `line` (without underscore). Rust interprets `line` as the built-in `line!()` macro.

**Fix**:
```rust
for line in lines.iter().cloned() {  // Don't add underscore
    count = count + f.write(&line);
}
```

**Complexity**: Easy (fix iterator variable naming)

---

## Error Summary

| Error Type | Count | Category | Severity |
|------------|-------|----------|----------|
| `cannot find function 'open'` | 26 | Context managers | CRITICAL |
| `cannot find type 'bytes'` | 2 | Type mapping | Easy |
| Variable scoping (`content` not found) | 3 | Context manager scopes | Medium |
| `line` macro collision | 1 | Iterator naming | Easy |
| **TOTAL** | **32** | **Multiple** | **BLOCKING** |

---

## Architectural Issue: Context Manager Translation Strategy

### Current Approach (WRONG)

The transpiler currently tries to **directly translate Python's context manager protocol**:

```rust
let _context = open(...);  // Create context
let resource = _context.__enter__();  // Enter context
// Use resource
// Missing: __exit__ is never called!
```

**Problems**:
1. Assumes Python runtime exists in Rust
2. No Rust equivalent of `__enter__` / `__exit__`
3. No RAII (Resource Acquisition Is Initialization)
4. No automatic cleanup

---

### Correct Approach (What Should Happen)

Context managers in Python map to **RAII in Rust**. Files are automatically closed when they go out of scope.

#### Option 1: Use Rust standard library functions

```python
# Python
with open(filename, 'r') as f:
    return f.read()

# Rust (idiomatic)
std::fs::read_to_string(filename)?
```

#### Option 2: Manual file handling with RAII

```python
# Python
with open(filename, 'r') as f:
    for line in f:
        print(line)

# Rust (manual)
let file = std::fs::File::open(filename)?;
let reader = std::io::BufReader::new(file);
for line in reader.lines() {
    println!("{}", line?);
}
// File automatically closed when `file` drops
```

#### Option 3: Helper function to wrap file operations

```rust
fn with_file_read<F, R>(filename: &str, f: F) -> Result<R, std::io::Error>
where
    F: FnOnce(&mut dyn std::io::Read) -> Result<R, std::io::Error>
{
    let file = std::fs::File::open(filename)?;
    let mut reader = std::io::BufReader::new(file);
    f(&mut reader)
}

// Usage:
with_file_read(filename, |f| {
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
})?
```

---

## Recommended Fix Strategy

### Phase 1: File I/O Standard Library Mapping (8-10 hours, 26 errors)

**Goal**: Map common Python file operations to Rust stdlib

**Mappings**:

```rust
// Python: with open(f, 'r') as file: return file.read()
// Rust: std::fs::read_to_string(f)?

// Python: with open(f, 'w') as file: file.write(content)
// Rust: std::fs::write(f, content)?

// Python: with open(f, 'r') as file: return file.readlines()
// Rust: std::fs::read_to_string(f)?.lines().map(|l| l.to_string()).collect()

// Python: with open(f, 'rb') as file: return file.read()
// Rust: std::fs::read(f)?

// Python: with open(f, 'wb') as file: file.write(data)
// Rust: std::fs::write(f, data)?

// Python: with open(f, 'a') as file: file.write(content)
// Rust: std::fs::OpenOptions::new().append(true).open(f)?.write_all(content.as_bytes())?
```

**Implementation**:
1. Detect `with open(...) as var:` pattern in HIR
2. Analyze file mode (`'r'`, `'w'`, `'rb'`, etc.)
3. Analyze operations on file handle (`.read()`, `.write()`, `.readlines()`, etc.)
4. Map to appropriate Rust stdlib function
5. Generate Result<T, std::io::Error> return type
6. Add `?` operator for error propagation

**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs` (with statement handling)

**Complexity**: HIGH - Requires pattern matching on HIR, operation analysis, and stdlib mapping

---

### Phase 2: Type Mapping (30 min, 2 errors)

**Goal**: Map Python `bytes` to Rust `Vec<u8>`

**Implementation**:
```rust
// In type_gen.rs:
"bytes" => parse_quote! { Vec<u8> },
```

**Complexity**: Easy (simple type mapping)

---

### Phase 3: Variable Scoping (2 hours, 3 errors)

**Goal**: Lift variables out of nested `with` scopes

**Strategy**:
1. Analyze variable usage across multiple `with` blocks
2. Hoist variable declarations before first `with`
3. Assign values inside `with` blocks

**Example**:
```rust
// Before (WRONG):
{
    let content = file1.read();  // Scoped here
}
{
    file2.write(content);  // ❌ Not in scope
}

// After (CORRECT):
let content;  // Declare outside
{
    content = file1.read();  // Assign inside
}
{
    file2.write(content);  // ✅ In scope
}
```

**Complexity**: Medium (requires scope analysis)

---

### Phase 4: Iterator Variable Naming (15 min, 1 error)

**Goal**: Don't add `_` prefix to iterator variables

**Implementation**:
```rust
// Current: for _line in lines
// Correct: for line in lines
```

**Complexity**: Easy (remove underscore prefix logic)

---

## Implementation Plan Summary

| Phase | Time | Errors Fixed | ROI |
|-------|------|--------------|-----|
| Phase 1: File I/O stdlib mapping | 8-10 hours | 26 errors | 2.6-3.25 errors/hour |
| Phase 2: `bytes` type mapping | 30 min | 2 errors | 4 errors/hour |
| Phase 3: Variable scoping | 2 hours | 3 errors | 1.5 errors/hour |
| Phase 4: Iterator naming | 15 min | 1 error | 4 errors/hour |
| **TOTAL** | **11-13 hours** | **32 errors** | **2.46-2.91 errors/hour** |

---

## Testing Strategy

**Test Cases Needed**:
```python
# Basic reading
assert read_entire_file("test.txt") == "Hello, world!"
assert read_file_lines("test.txt") == ["line1\n", "line2\n"]

# Basic writing
write_string_to_file("out.txt", "content")
assert read_entire_file("out.txt") == "content"

# Binary operations
data = bytes([0x48, 0x65, 0x6C, 0x6C, 0x6F])
write_binary_file("binary.dat", data)
assert read_binary_file("binary.dat") == data

# Multiple contexts
copy_file_content("source.txt", "dest.txt")
assert compare_files_equal("source.txt", "dest.txt") == True

# File iteration
lines = find_all_lines_with_text("file.txt", "search")
assert len(lines) > 0
```

---

## ROI Analysis

**Time Investment**: 11-13 hours (all phases)
**Error Reduction**: 32 errors → 0 errors (100%)
**Functions Fixed**: 0/24 functions → 24/24 functions (100%)
**Strategic Value**: **CRITICAL** - File I/O is fundamental, blocks all file operations

**Quick Wins**:
- Phase 2 + Phase 4: 45 min → 3 errors (4 errors/hour - HIGH ROI)
- But Phase 1 is **BLOCKING** - can't skip it

**Complete Fix**:
- 13 hours → 32 errors fixed (2.46 errors/hour - MODERATE ROI)
- **Unblocks ALL file I/O operations**
- **CRITICAL for production readiness**

---

## Dependencies

**Required**:
- HIR `With` statement handling (already exists but generates wrong code)
- File mode detection ('r', 'w', 'rb', 'wb', 'a')
- File operation analysis (`.read()`, `.write()`, `.readlines()`)
- Result type generation for I/O errors
- Error propagation (`?` operator)

**Blockers**: None (but requires significant rewrite)

---

## Related Issues

- **DEPYLER-0296**: Exception handling architecture (related to Result<T, E> pattern)
- **DEPYLER-0294**: Result unwrapping (file I/O returns Result)
- Context manager translation affects **ALL resource management** (not just files)

---

## Broader Impact: Context Managers Beyond Files

Context managers are used for:
1. **File I/O** (`with open()`) - THIS ISSUE
2. **Database connections** (`with db.connect()`)
3. **Network sockets** (`with socket.socket()`)
4. **Locks** (`with threading.Lock()`)
5. **Transactions** (`with db.transaction()`)
6. **Custom resources** (any class with `__enter__` / `__exit__`)

**Strategic Decision**: Fixing context manager translation enables **ALL resource management patterns**, not just file I/O.

---

## Recommendation

**Action Required**: **CRITICAL BLOCKER** - Must fix Phase 1 (File I/O stdlib mapping)

**Priority**: P0 (highest priority)
- **Blocks**: All file I/O operations (100% failure rate)
- **Impact**: Fundamental Python feature
- **ROI**: Moderate (2.6-3.25 errors/hour) but **strategically critical**

**Defer**: Phase 3 (variable scoping) can be worked around by inlining operations

**Quick Wins**: Phase 2 + Phase 4 (45 min, 3 errors) should be done first while planning Phase 1

---

## Alternative: Simplified File I/O Subset

If full context manager support is too complex, consider **simplified file I/O mapping** as interim solution:

**Supported Operations**:
- `read_entire_file()` → `std::fs::read_to_string()`
- `write_file()` → `std::fs::write()`
- `read_binary()` → `std::fs::read()`
- `write_binary()` → `std::fs::write()`

**Not Supported** (for now):
- Line-by-line iteration (`for line in f`)
- Append mode
- Multiple open files
- Custom context managers

This reduces implementation time to **4-6 hours** but covers **80% of use cases**.

---

## Conclusion

Example 10 validation reveals **the most critical architectural gap** in the transpiler: context manager translation is completely broken. This blocks ALL file I/O operations and affects resource management broadly.

**Next Steps**:
1. ✅ Document bugs (this ticket)
2. 🎯 **URGENT**: Implement Phase 1 (File I/O stdlib mapping) - BLOCKING
3. ⏸️ Continue Matrix Project AFTER unblocking file I/O
4. 📋 Re-validate Example 10 after fixes

**Status**: Documented, **CRITICAL PRIORITY** for implementation
