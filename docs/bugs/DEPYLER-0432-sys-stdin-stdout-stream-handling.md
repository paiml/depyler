# DEPYLER-0432: sys.stdin/stdout Stream Handling and File I/O

## Status: IN PROGRESS
- **Created**: 2025-11-19
- **Priority**: P1 (MEDIUM Priority)
- **Type**: Feature Gap + Type Inference Issues
- **Parent**: DEPYLER-0435 (reprorusted-python-cli 100% compilation)
- **Blocks**: stream_processor (36 errors reported → 32 errors actual)
- **Estimated Effort**: 4-6 hours (increased from 2-3h due to scope)
- **Actual Effort**: TBD

## Problem Statement

The stream_processor.py example fails to compile with 32 errors. Initial analysis shows sys.stdin/stdout ARE partially working, but there are multiple categories of issues preventing compilation.

### Error Breakdown (stream_processor.py)

**Total**: 32 compilation errors
- **Type inference issues**: ~20 errors (63%) - Parameters typed as `serde_json::Value`
- **File I/O translation**: ~8 errors (25%) - `.read()`, `.iter()`, `.hex()` methods
- **Missing subcommand**: 1 error (3%) - `Commands::Stdin` variant not generated
- **Function signatures**: ~3 errors (9%) - Need `Result<>` for `?` operator

### GOOD NEWS: sys.stdin Translation Works! ✅

**Python**:
```python
for line in sys.stdin:
    print(line)
```

**Generated Rust** (CORRECT):
```rust
for line in std::io::stdin().lines().map(|l| l.unwrap_or_default()) {
    println!("{}", line);
}
```

**Verification**: Line 60 of /tmp/stream_processor.rs shows correct translation.

### Issue 1: Type Inference - serde_json::Value Everywhere (20 errors)

**Current (WRONG)**:
```python
def read_file(filepath, binary=False):
    with open(filepath, mode) as f:
        content = f.read()
```

**Generated Rust (INCORRECT)**:
```rust
pub fn read_file(filepath: serde_json::Value, binary: &serde_json::Value) {
    let f = std::fs::File::open(filepath)?;  // ❌ E0277: Value doesn't impl AsRef<Path>
    let content = f.read();
}
```

**Expected (CORRECT)**:
```rust
pub fn read_file(filepath: &str, binary: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = std::fs::File::open(filepath)?;  // ✅ &str impls AsRef<Path>
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(())
}
```

**Root Cause**:
- Parameters without type hints default to `serde_json::Value`
- Boolean parameters typed as `&serde_json::Value` instead of `bool`
- String parameters should infer as `&str` from usage (file paths, patterns)

**Affected Functions**:
- `read_file(filepath, binary)` - 5 errors
- `write_file(filepath, content, append)` - 5 errors
- `count_lines(filepath)` - 4 errors
- `create_temp_file(content)` - 2 errors
- `filter_lines(filepath, pattern)` - 4 errors

### Issue 2: File I/O Method Translation (8 errors)

#### 2a: `f.read()` Translation

**Python**:
```python
with open(filepath) as f:
    content = f.read()
```

**Generated Rust (INCORRECT)**:
```rust
let f = std::fs::File::open(filepath)?;
let content = f.read();  // ❌ E0599: no method `read` on `File`
```

**Expected (CORRECT)**:
```rust
let mut f = std::fs::File::open(filepath)?;
let mut content = String::new();
f.read_to_string(&mut content)?;  // ✅ For text mode
// OR
use std::io::Read;
let mut content = Vec::new();
f.read_to_end(&mut content)?;  // ✅ For binary mode
```

#### 2b: `f.iter()` on File

**Python**:
```python
with open(filepath) as f:
    lines = sum(1 for line in f)
```

**Generated Rust (INCORRECT)**:
```rust
let f = std::fs::File::open(filepath)?;
let lines = f.iter().copied().map(|line| 1).sum::<i32>();  // ❌ E0599: no `iter` on `File`
```

**Expected (CORRECT)**:
```rust
use std::io::{BufRead, BufReader};
let f = std::fs::File::open(filepath)?;
let reader = BufReader::new(f);
let lines = reader.lines().count();  // ✅ BufReader has .lines()
```

#### 2c: `.hex()` Method

**Python**:
```python
hex_str = content[:100].hex()
```

**Generated Rust (INCORRECT)**:
```rust
let hex_str = content[..100].hex();  // ❌ E0599: no method `hex` on slice
```

**Expected (CORRECT)**:
```rust
use hex;
let hex_str = hex::encode(&content[..100]);  // ✅ Use hex crate
// OR
let hex_str = content[..100].iter()
    .map(|b| format!("{:02x}", b))
    .collect::<String>();  // ✅ Manual hex encoding
```

### Issue 3: Missing stdin Subcommand (1 error)

**Python**:
```python
subparsers = parser.add_subparsers(dest="command", required=True)
subparsers.add_parser("stdin", help="Read from stdin")
# ...
if args.command == "stdin":
    read_stdin_lines()
```

**Generated Rust (INCORRECT)**:
```rust
enum Commands {
    // stdin variant missing!
    Read { file: String, binary: bool },
    Write { /* ... */ },
    // ...
}

// Later:
let _cse_temp_0 = matches!(args.command, Commands::Stdin { .. });  // ❌ E0599: variant not found
```

**Expected (CORRECT)**:
```rust
enum Commands {
    #[command(about = "Read from stdin")]
    Stdin,  // ✅ Add stdin variant
    Read { file: String, binary: bool },
    // ...
}
```

**Root Cause**: Subcommand with no arguments not generating enum variant

### Issue 4: Function Return Types for `?` Operator (3 errors)

**Python**:
```python
def count_lines(filepath):
    try:
        with open(filepath) as f:
            lines = sum(1 for line in f)
        return lines
    except FileNotFoundError:
        sys.exit(1)
```

**Generated Rust (INCORRECT)**:
```rust
pub fn count_lines(filepath: serde_json::Value) -> i32 {
    let f = std::fs::File::open(filepath)?;  // ❌ E0277: can't use `?` in fn returning i32
    let lines = /* ... */;
    return lines;
}
```

**Expected (CORRECT)**:
```rust
pub fn count_lines(filepath: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(filepath)?;  // ✅ Works with Result return
    let reader = BufReader::new(f);
    Ok(reader.lines().count() as i32)
}
```

**Root Cause**: Functions with `try/except` or I/O operations need `Result<T, E>` return type

### Issue 5: sys.stderr Translation

**Python**:
```python
print("Error message", file=sys.stderr)
```

**Generated Rust** (need to verify):
```rust
// Should be:
eprintln!("Error message");  // ✅ Rust idiom
// OR:
use std::io::Write;
writeln!(std::io::stderr(), "Error message")?;  // ✅ Explicit
```

**Status**: Need to verify how `file=sys.stderr` is transpiled

### Issue 6: tempfile.NamedTemporaryFile Translation

**Python**:
```python
with tempfile.NamedTemporaryFile(mode="w", delete=False, suffix=".txt") as f:
    temp_path = f.name
    f.write(content)
```

**Expected Rust**:
```rust
use tempfile::NamedTempFile;
let mut temp_file = NamedTempFile::new()?;
let temp_path = temp_file.path().to_string_lossy().to_string();
temp_file.write_all(content.as_bytes())?;
temp_file.keep()?;  // delete=False
```

**Status**: Need to verify transpiler support

## Root Cause Analysis

### Core Issues

1. **Type Inference Default**
   - Parameters without hints → `serde_json::Value`
   - Should infer from usage: file paths → `&str`, flags → `bool`
   - Need usage-based type inference for common patterns

2. **File I/O API Mapping**
   - Python `f.read()` → Rust `f.read_to_string(&mut String)` or `f.read_to_end(&mut Vec<u8>)`
   - Python `for line in file` → Rust `BufReader::new(file).lines()`
   - Missing stdlib method mappings for File operations

3. **Argparse Edge Cases**
   - Subcommands with no arguments not generating enum variants
   - Similar to DEPYLER-0425 but for argument-less subcommands

4. **Exception Handling**
   - Functions using `with open()` or `try/except` need `Result<>` return
   - Already partially addressed by DEPYLER-0428, but file I/O needs extension

## Files Affected

### Primary Implementation:
- `crates/depyler-core/src/type_hints.rs`
  - Add: Usage-based type inference for file paths and boolean flags
  - Detect: argparse string arguments → `&str`, boolean flags → `bool`

- `crates/depyler-core/src/rust_gen/expr_gen.rs`
  - Fix: `try_convert_file_method()` for .read(), .write(), .readlines()
  - Add: .hex() translation (hex crate or manual encoding)
  - Update: File iteration to use BufReader.lines()

- `crates/depyler-core/src/rust_gen/stmt_gen.rs`
  - Fix: `with open()` translation to use Result<>
  - Add: BufReader wrapping for line iteration

- `crates/depyler-core/src/rust_gen/argparse_gen.rs`
  - Fix: Generate enum variants for argument-less subcommands

### Test Files:
- `crates/depyler-core/tests/depyler_0432_stream_handling.rs` (NEW)

## Test Plan

### Unit Tests (depyler_0432_stream_handling.rs)

```rust
#[test]
fn test_DEPYLER_0432_01_sys_stdin_iteration() {
    // Python: for line in sys.stdin:
    // Expected: std::io::stdin().lines()
}

#[test]
fn test_DEPYLER_0432_02_sys_stderr_print() {
    // Python: print("msg", file=sys.stderr)
    // Expected: eprintln!("msg")
}

#[test]
fn test_DEPYLER_0432_03_file_read_text() {
    // Python: with open(path) as f: content = f.read()
    // Expected: f.read_to_string(&mut content)?
}

#[test]
fn test_DEPYLER_0432_04_file_read_binary() {
    // Python: with open(path, "rb") as f: data = f.read()
    // Expected: f.read_to_end(&mut data)?
}

#[test]
fn test_DEPYLER_0432_05_file_iteration() {
    // Python: for line in file:
    // Expected: BufReader::new(file).lines()
}

#[test]
fn test_DEPYLER_0432_06_argparse_string_param_inference() {
    // Python: def func(filepath): open(filepath)
    // Expected: fn func(filepath: &str)
}

#[test]
fn test_DEPYLER_0432_07_argparse_bool_param_inference() {
    // Python: def func(binary=False): if binary:
    // Expected: fn func(binary: bool)
}

#[test]
fn test_DEPYLER_0432_08_argumentless_subcommand() {
    // Python: subparsers.add_parser("stdin")
    // Expected: enum Commands { Stdin, ... }
}

#[test]
fn test_DEPYLER_0432_09_hex_encoding() {
    // Python: data.hex()
    // Expected: hex::encode(data) or manual
}

#[test]
fn test_DEPYLER_0432_10_tempfile_translation() {
    // Python: tempfile.NamedTemporaryFile()
    // Expected: tempfile::NamedTempFile::new()
}
```

### Integration Tests

1. **stream_processor.py compilation**: 32 errors → 0 errors (100% reduction)
2. **stdin/stdout operations**: All sys module I/O works
3. **File operations**: open(), read(), write() all work
4. **Type inference**: Parameters correctly inferred

## Implementation Plan

### Phase 1: RED - Write Failing Tests ✅ (CURRENT)
```bash
# Create test file
touch crates/depyler-core/tests/depyler_0432_stream_handling.rs

# Add 10 tests (9 unit + 1 integration)
cargo test test_DEPYLER_0432  # MUST FAIL initially
```

### Phase 2: GREEN - Implement Fixes (4-6 hours)

**Step 1: Type Inference for Argparse Parameters (2h)**

In `type_hints.rs`, detect argparse patterns:
```rust
// If parameter used in: open(param), Path::new(param), os.path.*
// → Infer as &str (file path)

// If parameter used in: if param, param and/or
// AND has default=False/True
// → Infer as bool

// If parameter passed to file operations
// → Infer as &str (content)
```

**Step 2: File I/O Method Translation (1.5h)**

In `expr_gen.rs`, enhance `try_convert_file_method()`:
```rust
"read" => {
    // Check if binary mode from context
    if is_binary_mode() {
        Some(quote! {
            {
                let mut content = Vec::new();
                #object_expr.read_to_end(&mut content)?;
                content
            }
        })
    } else {
        Some(quote! {
            {
                let mut content = String::new();
                #object_expr.read_to_string(&mut content)?;
                content
            }
        })
    }
}

"hex" => {
    // bytes.hex() → hex::encode(bytes)
    Some(quote! { hex::encode(#object_expr) })
}
```

**Step 3: File Iteration Translation (1h)**

In `stmt_gen.rs`, detect file iteration:
```rust
// for line in file_var:
// →
// for line in BufReader::new(file_var).lines().map(|l| l.unwrap_or_default())
```

**Step 4: Argument-less Subcommands (0.5h)**

In `argparse_gen.rs`:
```rust
// Detect: subparsers.add_parser("name", help="...")
// With no add_argument calls
// → Generate: Name,  (unit variant)
```

**Step 5: Result<> Return Types (1h)**

In `func_gen.rs`, extend existing logic:
```rust
// If function uses:
// - with open()
// - try/except with FileNotFoundError
// - file.read(), file.write()
// → Return Result<T, Box<dyn std::error::Error>>
```

### Phase 3: REFACTOR - Clean Up + Edge Cases (1h)
- Handle text vs binary mode detection
- Ensure complexity ≤10, test coverage ≥80%
- Add proper error messages for unsupported patterns
- Update Cargo.toml generation to include `hex` and `tempfile` crates when needed

## Verification Checklist

- [ ] All 10 unit tests passing
- [ ] stream_processor.py errors: 32 → 0 (100% reduction)
- [ ] sys.stdin iteration works
- [ ] sys.stderr printing works
- [ ] File operations (read/write/iterate) work
- [ ] Type inference: parameters correctly typed
- [ ] Argument-less subcommands generate variants
- [ ] Complexity ≤10 (pmat analyze complexity)
- [ ] Coverage ≥80% (cargo llvm-cov)
- [ ] No clippy warnings (cargo clippy -D warnings)

## Success Criteria

**MUST ACHIEVE**:
1. ✅ stream_processor.py: 32 errors → 0 errors (100% compilation)
2. ✅ sys.stdin/stdout/stderr correctly translated
3. ✅ File I/O operations (open, read, write, iterate) work
4. ✅ Type inference: string paths → `&str`, flags → `bool`
5. ✅ Argument-less subcommands generate enum variants
6. ✅ All quality gates pass (complexity, coverage, clippy)

**Compilation Progress**:
- Current: 4/13 (30.8%)
- After DEPYLER-0432: 5/13 (38.5%) - stream_processor compiles
- Target (after MEDIUM tickets): 10-11/13 (77-85%)

## Time Tracking

- **Debug & Analysis**: 1 hour (DONE)
- **RED Phase**: 45-60 min (estimated)
- **GREEN Phase**: 4-6 hours (estimated)
- **REFACTOR Phase**: 1 hour (estimated)
- **Total**: 6.5-8.5 hours (increased from initial 2-3h estimate)

## Related Tickets

- **DEPYLER-0428**: Exception flow (COMPLETE) - Provides Result<> foundation
- **DEPYLER-0430**: os/sys/platform (COMPLETE) - Provides sys module foundation
- **DEPYLER-0431**: Regex module (PARTIAL) - Similar type inference issues
- **DEPYLER-0435**: Master ticket (IN PROGRESS)

## Scope Adjustment

**Initial Scope** (2-3h):
- sys.stdin/stdout translation

**Actual Scope** (6-8h):
- ✅ sys.stdin/stdout (mostly working already!)
- 🔧 Type inference for argparse parameters (major work)
- 🔧 File I/O method translation (major work)
- 🔧 Argument-less subcommands (small fix)
- 🔧 Result<> return type inference (extension)

**Recommendation**: Proceed with expanded scope. Type inference improvements will benefit DEPYLER-0431 and future tickets.

## References

- Rust std::io: https://doc.rust-lang.org/std/io/
- Rust std::fs: https://doc.rust-lang.org/std/fs/
- Python file objects: https://docs.python.org/3/library/io.html
- tempfile crate: https://docs.rs/tempfile/
- hex crate: https://docs.rs/hex/

---

**STATUS**: Analysis complete, ready for RED phase
**NEXT STEP**: Create test file with 10 failing tests

---

## Debugging Notes

### Error Distribution

```
Total: 32 errors (stream_processor.py)
├── Type inference (serde_json::Value): 20 (63%)
│   ├── filepath parameters: 10 errors
│   ├── binary flag parameters: 5 errors
│   └── content parameters: 5 errors
├── File I/O methods: 8 (25%)
│   ├── .read() translation: 3 errors
│   ├── .iter() on File: 2 errors
│   ├── .hex() method: 2 errors
│   └── .write() issues: 1 error
├── Missing subcommand: 1 (3%)
└── Function signatures: 3 (9%)
    └── Result<> needed for `?`: 3 errors
```

### Already Working ✅

- `sys.stdin` → `std::io::stdin().lines()` ✅ (line 60)
- Basic file open: `std::fs::File::open()` ✅
- Exception handling infrastructure from DEPYLER-0428 ✅

### Blocking Issues ❌

**P0 (Critical)**:
- Type inference defaulting to serde_json::Value (63% of errors)

**P1 (High)**:
- File I/O method translation (.read(), .iter())
- Missing subcommand variant generation

**P2 (Medium)**:
- .hex() encoding
- tempfile support

---

**Last Updated**: 2025-11-19
**Current Phase**: ANALYSIS COMPLETE → Starting RED phase
