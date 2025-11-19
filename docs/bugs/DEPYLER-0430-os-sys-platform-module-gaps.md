# DEPYLER-0430: os/sys/platform Module Gaps

## Status: IN PROGRESS (Analysis Complete, Ready for RED)
- **Created**: 2025-11-19
- **Priority**: P1 (HIGH - MEDIUM Priority)
- **Type**: Feature Gap
- **Parent**: DEPYLER-0435 (reprorusted-python-cli 100% compilation)
- **Blocks**: env_info (27 errors), config_manager (43 errors), stdlib_integration (41 errors)
- **Estimated Effort**: 4-6 hours
- **Actual Effort**: TBD

## Problem Statement

The env_info.py example fails to compile with 27 errors, of which 11 (41%) are due to missing or incorrect implementations of os/sys/platform module operations.

### Error Breakdown (env_info.py)

**Total**: 27 errors
- **DEPYLER-0430 scope**: 11/27 (41%)
  - Platform module: 3 errors (platform.system(), machine(), python_version())
  - Path operations: 8 errors (os.path.* methods called statically instead of as instance methods)
- **Out of scope**: 16/27 (59%) - argparse subcommands, type inference, string slicing

### Issue 1: Platform Module - Not Implemented (3 errors)

**Current (WRONG)**:
```python
platform.system()          # → platform.system() ❌ E0425: cannot find value `platform`
platform.machine()         # → platform.machine() ❌ E0425
platform.python_version()  # → platform.python_version() ❌ E0425
```

**Generated Rust (INCORRECT)**:
```rust
println!("{}", format!("  OS: {:?}", platform.system()));  // ❌ `platform` not defined
println!("{}", format!("  Architecture: {:?}", platform.machine()));
println!("{}", format!("  Python: {:?}", platform.python_version()));
```

**Expected (CORRECT)**:
```rust
println!("{}", format!("  OS: {:?}", std::env::consts::OS));
println!("{}", format!("  Architecture: {:?}", std::env::consts::ARCH));
println!("{}", format!("  Python: {:?}", env!("CARGO_PKG_VERSION")));  // Or constant
```

**Root Cause**: platform module operations not mapped to Rust equivalents

### Issue 2: Path Operations - Static vs Instance Methods (8 errors)

**Current (WRONG)**:
```python
os.path.exists(path)       # → std::path::Path.exists() ❌ E0423: expected value, found struct
os.path.isfile(path)       # → std::path::Path.isfile() ❌ E0423
os.path.isdir(path)        # → std::path::Path.isdir() ❌ E0423
os.path.expanduser(path)   # → std::path::Path.expanduser() ❌ E0423
os.path.dirname(path)      # → std::path::Path.dirname() ❌ E0423
os.path.basename(path)     # → std::path::Path.basename() ❌ E0423
```

**Generated Rust (INCORRECT)**:
```rust
let expanded = std::path::Path.expanduser(path);  // ❌ Static call on struct type
if std::path::Path.exists(expanded) {             // ❌ Static call
    println!("{}", format!("Is file: {:?}", std::path::Path.isfile(expanded)));  // ❌
    println!("{}", format!("Is directory: {:?}", std::path::Path.isdir(expanded)));  // ❌
    println!("{}", format!("Dirname: {:?}", std::path::Path.dirname(expanded)));  // ❌
    println!("{}", format!("Basename: {:?}", std::path::Path.basename(expanded)));  // ❌
}
```

**Expected (CORRECT)**:
```rust
use std::path::{Path, PathBuf};
use shellexpand;  // For expanduser

let expanded = shellexpand::tilde(path).to_string();  // expanduser
let path_obj = Path::new(&expanded);
if path_obj.exists() {                                // Instance method
    println!("{}", format!("Is file: {:?}", path_obj.is_file()));
    println!("{}", format!("Is directory: {:?}", path_obj.is_dir()));
    println!("{}", format!("Dirname: {:?}", path_obj.parent().unwrap_or(Path::new(""))));
    println!("{}", format!("Basename: {:?}", path_obj.file_name().unwrap_or_default()));
}
```

**Root Cause**: os.path.* methods incorrectly transpiled as static methods on `std::path::Path` struct instead of instance methods on Path objects

### What's Already Working ✅

**os.environ.get()**:
```python
value = os.environ.get(var_name)
```
**Generated (CORRECT)**:
```rust
value = std::env::var(var_name).ok();  // ✅ Works!
```

**sys.platform**:
```python
sys.platform
```
**Generated (CORRECT)**:
```rust
"linux".to_string()  // ✅ Hardcoded but works
```

## Root Cause Analysis

### Platform Module

**Location**: Not implemented in expr_gen.rs

**Current Behavior**: platform.* calls are not recognized, transpiled literally

**What's Needed**: Add `try_convert_platform_method()` to map:
- `platform.system()` → `std::env::consts::OS`
- `platform.machine()` → `std::env::consts::ARCH`
- `platform.python_version()` → `env!("CARGO_PKG_VERSION")` or constant
- `platform.release()` → OS-specific release detection

### Path Operations

**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs` (try_convert_os_method)

**Current Behavior**: Generates `std::path::Path.method()` static calls

**Bug**: Rust Path methods are instance methods, not static methods:
```rust
// Wrong:
std::path::Path.exists(path)

// Correct:
Path::new(path).exists()
```

**What's Needed**: Fix os.path.* transpilation to:
1. Create Path instance: `Path::new(path)`
2. Call instance method: `.exists()`, `.is_file()`, etc.
3. Handle expanduser with shellexpand crate
4. Map dirname → `.parent()`, basename → `.file_name()`

## Files Affected

### Primary Implementation:
- `crates/depyler-core/src/rust_gen/expr_gen.rs`
  - Add: `try_convert_platform_method()` (NEW function)
  - Fix: `try_convert_os_method()` for path operations
  - Update: Module mapping to include platform

### Dependencies:
- Add `shellexpand` crate for `os.path.expanduser()`

### Test Files:
- `crates/depyler-core/tests/depyler_0430_os_sys_platform.rs` (NEW)

## Test Plan

### Unit Tests (depyler_0430_os_sys_platform.rs)

```rust
#[test]
fn test_DEPYLER_0430_01_platform_system() {
    // Python: platform.system()
    // Expected: std::env::consts::OS
}

#[test]
fn test_DEPYLER_0430_02_platform_machine() {
    // Python: platform.machine()
    // Expected: std::env::consts::ARCH
}

#[test]
fn test_DEPYLER_0430_03_path_exists() {
    // Python: os.path.exists(path)
    // Expected: Path::new(path).exists()
}

#[test]
fn test_DEPYLER_0430_04_path_isfile() {
    // Python: os.path.isfile(path)
    // Expected: Path::new(path).is_file()
}

#[test]
fn test_DEPYLER_0430_05_path_expanduser() {
    // Python: os.path.expanduser("~/file")
    // Expected: shellexpand::tilde("~/file").to_string()
}

#[test]
fn test_DEPYLER_0430_06_path_dirname_basename() {
    // Python: os.path.dirname(path), os.path.basename(path)
    // Expected: .parent(), .file_name()
}

#[test]
fn test_DEPYLER_0430_07_env_info_integration() {
    // Full env_info.py transpilation
    // Verify 11/27 errors fixed
}
```

### Integration Tests

1. **env_info.py compilation**: 27 errors → 16 errors (11 fixed)
2. **Path operations**: All os.path.* methods work
3. **Platform module**: All platform.* methods work

## Implementation Plan

### Phase 1: RED - Write Failing Tests ✅
```bash
# Create test file
touch crates/depyler-core/tests/depyler_0430_os_sys_platform.rs

# Add 7 tests (6 unit + 1 integration)
cargo test test_DEPYLER_0430  # MUST FAIL initially
```

### Phase 2: GREEN - Implement Fixes

**Step 1: Add platform module support**
```rust
// In expr_gen.rs

fn try_convert_platform_method(
    method: &str,
    args: &[HirExpr],
    ctx: &CodeGenContext,
) -> Option<proc_macro2::TokenStream> {
    match method {
        "system" => {
            // platform.system() → std::env::consts::OS
            Some(quote! { std::env::consts::OS.to_string() })
        }
        "machine" => {
            // platform.machine() → std::env::consts::ARCH
            Some(quote! { std::env::consts::ARCH.to_string() })
        }
        "python_version" => {
            // platform.python_version() → version constant
            Some(quote! { "3.11.0".to_string() })  // Or env!("CARGO_PKG_VERSION")
        }
        _ => None,
    }
}
```

**Step 2: Fix os.path.* methods**
```rust
// In try_convert_os_method()

match method {
    "exists" => {
        // os.path.exists(path) → Path::new(path).exists()
        let path_arg = &args[0].to_rust_tokens(ctx)?;
        Some(quote! { std::path::Path::new(#path_arg).exists() })
    }
    "isfile" => {
        let path_arg = &args[0].to_rust_tokens(ctx)?;
        Some(quote! { std::path::Path::new(#path_arg).is_file() })
    }
    "isdir" => {
        let path_arg = &args[0].to_rust_tokens(ctx)?;
        Some(quote! { std::path::Path::new(#path_arg).is_dir() })
    }
    "expanduser" => {
        let path_arg = &args[0].to_rust_tokens(ctx)?;
        Some(quote! { shellexpand::tilde(#path_arg).to_string() })
    }
    "dirname" => {
        let path_arg = &args[0].to_rust_tokens(ctx)?;
        Some(quote! {
            std::path::Path::new(#path_arg)
                .parent()
                .unwrap_or(std::path::Path::new(""))
                .to_str()
                .unwrap_or("")
                .to_string()
        })
    }
    "basename" => {
        let path_arg = &args[0].to_rust_tokens(ctx)?;
        Some(quote! {
            std::path::Path::new(#path_arg)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string()
        })
    }
    _ => None,
}
```

**Step 3: Update module detection**
```rust
// Detect platform module usage
if module == "platform" {
    return try_convert_platform_method(method, args, ctx);
}
```

### Phase 3: REFACTOR - Clean Up + Edge Cases
- Handle path.join() with multiple arguments
- Add os.path.abspath() → std::fs::canonicalize()
- Ensure complexity ≤10, test coverage ≥80%
- Add shellexpand to Cargo.toml dependencies

## Verification Checklist

- [ ] All 7 unit tests passing
- [ ] env_info.py errors: 27 → 16 (11 fixed)
- [ ] Platform module methods work correctly
- [ ] Path operations use instance methods
- [ ] expanduser works with shellexpand
- [ ] Complexity ≤10 (pmat analyze complexity)
- [ ] Coverage ≥80% (cargo llvm-cov)
- [ ] No clippy warnings (cargo clippy -D warnings)

## Success Criteria

**MUST ACHIEVE**:
1. ✅ env_info.py: 27 errors → 16 errors (11 fixed, 41% reduction)
2. ✅ platform.system(), machine(), python_version() work
3. ✅ All os.path.* operations use instance methods
4. ✅ All quality gates pass (complexity, coverage, clippy)
5. ✅ Progress toward compilation target

**Compilation Progress**:
- Current: 4/13 (30.8%)
- After DEPYLER-0430: 4-5/13 (30.8-38.5%) - env_info may still need more work
- Target (after all MEDIUM tickets): 10-11/13 (77-85%)

## Time Tracking

- **Debug & Analysis**: 1 hour (DONE)
- **RED Phase**: 1 hour (estimated)
- **GREEN Phase**: 2-3 hours (estimated)
- **REFACTOR Phase**: 1 hour (estimated)
- **Total**: 4-6 hours

## Related Tickets

- **DEPYLER-0428**: Exception flow (COMPLETE)
- **DEPYLER-0429**: Exception binding (PARTIAL)
- **DEPYLER-0435**: Master ticket (IN PROGRESS)
- **DEPYLER-0431**: regex module (NOT STARTED)
- **DEPYLER-0432**: sys.stdin/stdout (NOT STARTED)

## References

- Rust std::env::consts: https://doc.rust-lang.org/std/env/consts/
- Rust std::path: https://doc.rust-lang.org/std/path/
- shellexpand crate: https://docs.rs/shellexpand/
- Python platform: https://docs.python.org/3/library/platform.html
- Python os.path: https://docs.python.org/3/library/os.path.html

---

## Debugging Notes

### Error Count by Category

```
Total: 27 errors (env_info.py)
├── DEPYLER-0430 scope: 11 (41%)
│   ├── Platform module: 3 (platform.system/machine/python_version)
│   └── Path operations: 8 (os.path.exists/isfile/isdir/etc)
└── Out of scope: 16 (59%)
    ├── Argparse subcommands: 7
    ├── Type inference: 6
    └── Other: 3
```

### Already Implemented ✅

- `os.environ.get()` → `std::env::var().ok()`
- `sys.platform` → `"linux".to_string()`

### Missing Implementations ❌

**Platform Module**:
- `platform.system()` → std::env::consts::OS
- `platform.machine()` → std::env::consts::ARCH
- `platform.python_version()` → version constant
- `platform.release()` → OS-specific

**Path Operations** (fix static → instance):
- `os.path.exists()` → Path::new().exists()
- `os.path.isfile()` → Path::new().is_file()
- `os.path.isdir()` → Path::new().is_dir()
- `os.path.expanduser()` → shellexpand::tilde()
- `os.path.dirname()` → Path::new().parent()
- `os.path.basename()` → Path::new().file_name()
- `os.path.abspath()` → std::fs::canonicalize()
- `os.path.join()` → PathBuf::from().join()

---

**STATUS**: Analysis complete, ready for RED phase
**NEXT STEP**: `pmat prompt show continue DEPYLER-0430` to begin RED phase
