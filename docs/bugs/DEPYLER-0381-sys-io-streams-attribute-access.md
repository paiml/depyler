# DEPYLER-0381: sys I/O Streams Attribute Access Not Supported

**Ticket**: DEPYLER-0381
**Severity**: P0 (CRITICAL - Blocks transpilation)
**Status**: 🛑 STOP THE LINE
**Created**: 2025-11-17
**Component**: Module Mapper / Attribute Resolution
**Affects**: sys.stdin, sys.stdout, sys.stderr attribute access

## Problem Statement

The transpiler fails with "sys.stderr is not a recognized attribute" when trying to access `sys.stdin`, `sys.stdout`, or `sys.stderr` and call methods on them (`.write()`, `.read()`, `.flush()`).

While the module_mapper.rs correctly maps these attributes to Rust equivalents:
- `sys.stdin` → `std::io::stdin`
- `sys.stdout` → `std::io::stdout`
- `sys.stderr` → `std::io::stderr`

The attribute resolution fails before the mapping can be applied.

## Reproduction

**Input** (`test_sys_import.py`):
```python
import sys

def exit_with_error(message: str, code: int = 1) -> None:
    """Print error to stderr and exit"""
    sys.stderr.write(f"Error: {message}\n")
    sys.exit(code)

def print_to_stdout(message: str) -> None:
    """Print message to stdout"""
    sys.stdout.write(message + "\n")
    sys.stdout.flush()

def read_from_stdin() -> str:
    """Read all input from stdin"""
    return sys.stdin.read()
```

**Current Error**:
```
Error: sys.stderr is not a recognized attribute
```

**Expected Behavior**: Should transpile successfully with proper Rust I/O stream handling.

## Root Cause Analysis

### Location
Likely in attribute resolution before module mapping is applied.

The error occurs in the HIR generation or type inference phase, where `sys.stderr` is not recognized as a valid attribute access pattern.

### Why It's Wrong
1. `module_mapper.rs` has correct mappings (lines 83-85)
2. Attribute access to module-level I/O streams should be supported
3. Method calls on I/O streams (`.write()`, `.read()`, `.flush()`) need proper transpilation

### Semantic Differences: Python vs Rust

**Python**:
- `sys.stdin`, `sys.stdout`, `sys.stderr` are file-like objects
- Global, always available after `import sys`
- Methods: `.read()`, `.write()`, `.flush()`, `.readline()`, etc.

**Rust**:
- `std::io::stdin()`, `std::io::stdout()`, `std::io::stderr()` are **functions** that return handles
- Must call the function to get the handle
- Handles implement `Read`, `Write`, `BufRead` traits

**Key Difference**: Python uses **attributes** (`sys.stdin`), Rust uses **function calls** (`std::io::stdin()`).

## Impact Assessment

**Affected Code Patterns**:
1. `sys.stdin.read()` - Reading from standard input
2. `sys.stdout.write(msg)` - Writing to standard output
3. `sys.stderr.write(msg)` - Writing to standard error
4. `sys.stdout.flush()` - Flushing output buffer
5. Any method call on sys I/O streams

**Severity**: **P0 CRITICAL**
- Blocks transpilation of any code using standard I/O
- Common pattern in CLI applications
- Required for `example_io_streams` validation

## Solution Design

### Approach
Add special handling for `sys.stdin/stdout/stderr` attribute access that:
1. Recognizes these as special module attributes
2. Transpiles to function calls in Rust
3. Handles method calls on the returned handles

### Algorithm

**For attribute access** (`sys.stdin`):
```rust
// Python: sys.stdin
// Rust: std::io::stdin()
```

**For method calls** (`sys.stdin.read()`):
```rust
// Python: sys.stdin.read()
// Rust: std::io::stdin().lock().read_to_string(&mut String::new()).unwrap()

// Or for repeated use:
// let stdin = std::io::stdin();
// let mut handle = stdin.lock();
// handle.read_to_string(&mut buf)
```

### Implementation Strategy

**Option 1: Special Case in Attribute Handler**
Add checks in `convert_attribute()` in `expr_gen.rs`:
```rust
// Pseudocode
fn convert_attribute(module: &str, attr: &str) -> Result<syn::Expr> {
    if module == "sys" {
        match attr {
            "stdin" => return Ok(parse_quote! { std::io::stdin() }),
            "stdout" => return Ok(parse_quote! { std::io::stdout() }),
            "stderr" => return Ok(parse_quote! { std::io::stderr() }),
            _ => {}
        }
    }
    // ... rest of attribute handling
}
```

**Option 2: Method Call Transform**
When method is called on sys I/O stream:
```rust
// Python: sys.stdout.write("Hello\n")
// Rust: writeln!(std::io::stdout(), "Hello").unwrap()

// Or:
// std::io::stdout().write_all(b"Hello\n").unwrap()
```

### Complexity Considerations
- Attribute handler: Add 3-5 lines for sys stream check
- Method call handler: Add pattern matching for common I/O operations
- Target: Keep cyclomatic complexity ≤ 10

## Test Plan

### Test Case 1: sys.stderr.write()
**Input**:
```python
import sys

def error(msg: str) -> None:
    sys.stderr.write(f"Error: {msg}\n")
```

**Expected Rust**:
```rust
pub fn error(msg: String) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(std::io::stderr(), "Error: {}", msg)?;
    Ok(())
}
```

### Test Case 2: sys.stdout.flush()
**Input**:
```python
import sys

def flush_output() -> None:
    sys.stdout.flush()
```

**Expected Rust**:
```rust
use std::io::Write;

pub fn flush_output() -> Result<(), Box<dyn std::error::Error>> {
    std::io::stdout().flush()?;
    Ok(())
}
```

### Test Case 3: sys.stdin.read()
**Input**:
```python
import sys

def read_all() -> str:
    return sys.stdin.read()
```

**Expected Rust**:
```rust
use std::io::Read;

pub fn read_all() -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
```

### Test Case 4: Multiple I/O Operations
**Input**:
```python
import sys

def process() -> None:
    sys.stdout.write("Enter name: ")
    sys.stdout.flush()
    name = sys.stdin.readline()
    sys.stderr.write(f"Processing {name}\n")
```

**Expected Rust**:
```rust
use std::io::{self, Write, BufRead};

pub fn process() -> Result<(), Box<dyn std::error::Error>> {
    write!(io::stdout(), "Enter name: ")?;
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut name = String::new();
    stdin.lock().read_line(&mut name)?;

    writeln!(io::stderr(), "Processing {}", name.trim())?;
    Ok(())
}
```

## Verification Steps

1. **Unit Tests**: Add tests for each I/O stream attribute access
2. **Integration Tests**: Test full file with multiple I/O operations
3. **Property Tests**: QuickCheck with random strings to write/read
4. **Compilation Test**: Verify generated Rust compiles with `rustc --deny warnings`
5. **Runtime Test**: Execute transpiled binary and verify I/O works correctly

## Related Issues

- **DEPYLER-0380**: String literals and os module (completed) - similar attribute access issue
- **example_io_streams**: GitHub validation issue - blocked by this bug

## Implementation Checklist

- [ ] Create failing tests for sys I/O streams
- [ ] Add attribute recognition for sys.stdin/stdout/stderr
- [ ] Implement method call transpilation (.write(), .read(), .flush())
- [ ] Handle readline(), read_to_end() and other common methods
- [ ] Add proper error handling (Result types)
- [ ] Add necessary use statements (std::io::Write, Read, BufRead)
- [ ] Test with all I/O stream combinations
- [ ] Verify transpiled code compiles
- [ ] Run full test suite for regressions
- [ ] Update documentation

## Estimated Complexity

**TDG Target**: ≤ 2.0 (A- or better)
**Cyclomatic Complexity**: ≤ 10 per function
**Test Coverage**: ≥ 85%
**Implementation Time**: 2-4 hours (including EXTREME TDD)

## Notes

**Key Insight**: Python's file-like objects vs Rust's function-based I/O is a fundamental semantic difference that requires careful transpilation.

**Performance**: Rust's `lock()` pattern for repeated I/O operations is more efficient than calling `stdin()` each time.

**Error Handling**: Rust I/O operations return `Result`, requiring `?` operator or explicit unwrap.
