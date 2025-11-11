# DEPYLER-0363: ArgParse Transpilation Incomplete/Broken

**Status**: 🛑 BLOCKING - STOP THE LINE
**Severity**: P0 (Compilation Failure)
**Created**: 2025-11-11
**Component**: codegen/argparse, expr_gen
**Affects**: All Python code using argparse module

---

## Problem Statement

The transpiler fails to properly convert Python `argparse` module usage to Rust, resulting in non-compiling output with multiple critical issues:

1. **Incomplete Module Mapping**: Generated code contains `argparse.ArgumentParser()` (Python syntax) instead of Rust clap equivalent
2. **Malformed Function Signatures**: Functions use invalid `&Path` without proper imports
3. **Broken Error Handling**: try/except blocks produce malformed output
4. **Syntax Errors**: Multiple syntax errors prevent compilation

## Reproduction

### Input File
`examples/argparse_cli/python/wordcount.py` (94 lines)

```python
#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path
from typing import NamedTuple

class Stats(NamedTuple):
    lines: int
    words: int
    chars: int
    filename: str

def count_file(filepath: Path) -> Stats:
    try:
        content = filepath.read_text()
        lines = len(content.splitlines())
        words = len(content.split())
        chars = len(content)
        return Stats(lines, words, chars, str(filepath))
    except IOError as e:
        print(f"Error reading {filepath}: {e}", file=sys.stderr)
        return Stats(0, 0, 0, str(filepath))

def main() -> int:
    parser = argparse.ArgumentParser(
        description="Count lines, words, and characters in files",
        epilog="Similar to wc(1) Unix command"
    )
    parser.add_argument("files", nargs="+", type=Path, help="Files to process")
    parser.add_argument("-l", "--lines", action="store_true", help="Show only line count")
    # ... more arguments
    args = parser.parse_args()
    # ... main logic
    return 0
```

### Command
```bash
cargo run --bin depyler -- transpile examples/argparse_cli/python/wordcount.py
```

### Generated Output (BROKEN)
`examples/argparse_cli/python/wordcount.rs` (65 lines)

```rust
#[doc = "// TODO: Map Python module 'argparse'"]
#[doc = "// Python import: sys"]
use std::path::PathBuf;

pub fn count_file(filepath: & Path) -> Stats {  // ERROR: Path not in scope
    { let content = filepath.read_text();  // ERROR: read_text() doesn't exist
    let lines = content.splitlines().len() as i32;  // ERROR: splitlines() doesn't exist
    // ... incomplete error handling
    println!("{}", format!("Error reading {}: {}", filepath, e));  // ERROR: orphaned println
    return Stats::new(0, 0, 0, filepath.to_string());
}

pub fn main () -> i32 {
    let parser = argparse.ArgumentParser();  // ERROR: argparse doesn't exist in Rust
    parser.add_argument("files".to_string());  // ERROR: method doesn't exist
    // ... more broken code
}
```

### Compilation Attempt
```bash
rustc --crate-type lib wordcount.rs
# FAILS with multiple errors:
# - cannot find type `Path` in this scope
# - cannot find value `argparse` in this scope
# - no method named `read_text` found for reference `&Path`
# - no method named `splitlines` found for struct `String`
# - syntax errors in multiple locations
```

## Root Cause Analysis

### Issue 1: ArgParse Module Not Mapped
**Location**: `crates/depyler-core/src/rust_gen/stmt_gen.rs` (or module mapping)

**Problem**: The transpiler emits a TODO comment but doesn't actually map argparse to clap:
```rust
#[doc = "// TODO: Map Python module 'argparse'"]
```

**Why This Exists**: ArgParse-to-Clap mapping is not implemented. The transpiler recognizes the import but doesn't perform the transformation.

**Impact**: Any Python CLI using argparse produces non-compiling Rust code.

### Issue 2: Path Type Import Missing
**Location**: `crates/depyler-core/src/rust_gen/func_gen.rs` or type inference

**Problem**: Function signature uses `&Path` but only imports `PathBuf`:
```rust
use std::path::PathBuf;  // Only PathBuf imported
pub fn count_file(filepath: & Path) -> Stats {  // Uses Path
```

**Why This Exists**: Type mapper converts `pathlib.Path` to `&Path` but doesn't track that `Path` needs to be imported.

**Impact**: Compilation error on every function using Path parameters.

### Issue 3: Python String Methods Unmapped
**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` - method call generation

**Problem**: Python string methods translated directly without Rust equivalents:
- `filepath.read_text()` → Should be `fs::read_to_string(filepath)`
- `content.splitlines()` → Should be `content.lines().collect()`
- `content.split()` → Should be `content.split_whitespace()`

**Why This Exists**: Method call transpilation doesn't check if Python methods have direct Rust equivalents.

**Impact**: Generated code calls non-existent methods.

### Issue 4: Malformed Try/Except Blocks
**Location**: `crates/depyler-core/src/rust_gen/stmt_gen.rs` - exception handling

**Problem**: Error handling generates orphaned statements outside function body:
```rust
pub fn count_file(filepath: & Path) -> Stats {
    { let content = filepath.read_text();  // try block?
    // ... success code
    println!("{}", format!("Error reading {}: {}", filepath, e));  // ERROR: outside function
    return Stats::new(0, 0, 0, filepath.to_string());  // ERROR: orphaned
}
}  // Extra closing brace
```

**Why This Exists**: Try/except block codegen doesn't properly wrap in match expression or Result handling.

**Impact**: Syntax errors, orphaned statements, extra braces.

## Solution Design

### Fix 1: Implement ArgParse → Clap Mapping
**Files to Modify**:
- `crates/depyler-core/src/module_mapping.rs` (or create if doesn't exist)
- `crates/depyler-core/src/rust_gen/stmt_gen.rs`

**Approach**:
1. Detect `import argparse` in HIR
2. Map to `use clap::Parser;`
3. Transform `ArgumentParser()` → `#[derive(Parser)] struct Args`
4. Map `add_argument()` calls to clap field attributes
5. Transform `parse_args()` → `Args::parse()`

**Example Transformation**:
```python
# Python
parser = argparse.ArgumentParser(description="...")
parser.add_argument("files", nargs="+", type=Path)
parser.add_argument("-l", "--lines", action="store_true")
args = parser.parse_args()
```

```rust
// Rust (target output)
#[derive(Parser)]
#[command(about = "...")]
struct Args {
    /// Files to process
    files: Vec<PathBuf>,

    /// Show only line count
    #[arg(short = 'l', long)]
    lines: bool,
}

let args = Args::parse();
```

### Fix 2: Improve Import Tracking
**Files to Modify**:
- `crates/depyler-core/src/rust_gen/mod.rs` - import tracking
- `crates/depyler-core/src/type_mapper.rs`

**Approach**:
1. When type mapper returns `&Path`, register `std::path::Path` in imports
2. Track all types referenced in signatures
3. Emit `use std::path::{Path, PathBuf};` when both are needed

### Fix 3: Python Method → Rust Function/Method Mapping
**Files to Modify**:
- `crates/depyler-core/src/rust_gen/expr_gen.rs`
- Create `crates/depyler-core/src/python_method_map.rs`

**Approach**:
Create mapping table:
```rust
// python_method_map.rs
pub fn map_method_call(receiver_type: &str, method: &str) -> Option<MappedCall> {
    match (receiver_type, method) {
        ("Path", "read_text") => Some(MappedCall::Function {
            module: "std::fs",
            func: "read_to_string",
            args_transform: |receiver, args| format!("fs::read_to_string({})", receiver)
        }),
        ("str", "splitlines") => Some(MappedCall::Method {
            method: "lines",
            chain: Some(".collect::<Vec<_>>()")
        }),
        ("str", "split") => Some(MappedCall::Method {
            method: "split_whitespace",
            chain: Some(".collect::<Vec<_>>()")
        }),
        _ => None
    }
}
```

### Fix 4: Fix Try/Except Codegen
**Files to Modify**:
- `crates/depyler-core/src/rust_gen/stmt_gen.rs` - `gen_try_except()`

**Approach**:
Transform try/except to match on Result:
```python
# Python
try:
    content = filepath.read_text()
    # ... success code
except IOError as e:
    print(f"Error: {e}", file=sys.stderr)
    return default_value
```

```rust
// Rust (correct output)
match fs::read_to_string(filepath) {
    Ok(content) => {
        // ... success code
    }
    Err(e) => {
        eprintln!("Error: {}", e);
        return default_value;
    }
}
```

## Test Plan

### Regression Test (Add Before Fix)
**File**: `crates/depyler-core/tests/test_argparse_transpilation.rs`

```rust
#[test]
fn test_argparse_basic_transpilation() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="+")
    args = parser.parse_args()
    return 0
"#;

    let rust = transpile(python);

    // Must compile
    assert!(compile_rust(&rust).is_ok());

    // Must contain clap usage
    assert!(rust.contains("use clap"));
    assert!(rust.contains("derive(Parser)"));

    // Must NOT contain Python artifacts
    assert!(!rust.contains("argparse"));
    assert!(!rust.contains("TODO"));
}

#[test]
fn test_path_method_mapping() {
    let python = r#"
from pathlib import Path

def read_file(p: Path) -> str:
    return p.read_text()
"#;

    let rust = transpile(python);

    // Must compile
    assert!(compile_rust(&rust).is_ok());

    // Must use fs::read_to_string
    assert!(rust.contains("fs::read_to_string"));

    // Must import Path
    assert!(rust.contains("use std::path::Path"));
}

#[test]
fn test_string_method_mapping() {
    let python = r#"
def count_lines(text: str) -> int:
    return len(text.splitlines())
"#;

    let rust = transpile(python);

    assert!(compile_rust(&rust).is_ok());
    assert!(rust.contains(".lines()"));
}

#[test]
fn test_try_except_with_path_io() {
    let python = r#"
from pathlib import Path

def safe_read(p: Path) -> str:
    try:
        return p.read_text()
    except IOError:
        return ""
"#;

    let rust = transpile(python);

    assert!(compile_rust(&rust).is_ok());
    assert!(rust.contains("match"));
    assert!(rust.contains("Ok("));
    assert!(rust.contains("Err("));
}
```

### Integration Test
Run full wordcount.py transpilation and verify:
1. Output compiles with `rustc --crate-type bin`
2. Output passes `cargo clippy -- -D warnings`
3. Binary produces identical output to Python version
4. All 26 Python test cases pass with Rust implementation

### Property Test
Generate random argparse configurations and verify all transpile without syntax errors.

## Implementation Checklist

- [ ] Add regression tests (RED phase)
- [ ] Implement ArgParse → Clap mapping
- [ ] Implement Path method mapping
- [ ] Implement string method mapping
- [ ] Fix try/except block generation
- [ ] Improve import tracking
- [ ] Run regression tests (GREEN phase)
- [ ] Transpile wordcount.py successfully
- [ ] Verify compiled binary matches Python output
- [ ] Run clippy and fix warnings (REFACTOR phase)
- [ ] Update examples with working transpilation
- [ ] Document ArgParse mapping in book

## Estimated Effort

- Analysis: 1 hour (DONE - this document)
- Test writing: 2 hours
- ArgParse mapping: 4 hours
- Method mapping: 2 hours
- Try/except fix: 2 hours
- Integration verification: 2 hours
- **Total**: ~13 hours

## Related Issues

- Previous try/except fixes: Context from session (DEPYLER-0360, 0361, 0362 mentioned)
- Module mapping: Likely related to other stdlib module mappings

## References

- Python argparse docs: https://docs.python.org/3/library/argparse.html
- Rust clap docs: https://docs.rs/clap/latest/clap/
- STOP THE LINE protocol: `/docs/processes/stop-the-line.md`

---

**SACRED RULE**: Fix the transpiler, not the generated output. NEVER ship hand-crafted "reference implementations".
