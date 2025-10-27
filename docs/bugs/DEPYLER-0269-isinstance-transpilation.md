# DEPYLER-0269: isinstance() Generates Invalid Rust Code

**Status**: 🛑 STOP THE LINE - P0 Critical
**Discovery**: 2025-10-27 - Matrix Testing Column A → B (01_basic_types)
**Severity**: P0 - Blocks compilation

---

## Bug Description

**Issue**: `isinstance(value, int)` generates invalid Rust code that references undefined `int` type.

**Root Cause**: The transpiler in `crates/depyler-core/src/rust_gen/expr_gen.rs` does not recognize `isinstance()` as a Python builtin that should be removed or transformed for statically-typed Rust.

**Impact**: ANY Python code using `isinstance()` for type checking will fail to compile after transpilation.

---

## Expected Output (Idiomatic Rust)

```rust
#[doc = "Check if value is an integer (always True due to type system)."]
pub fn type_check_int(_value: i32) -> bool {
    // In Rust, type system guarantees this is always true
    true
}
```

**Rationale**: In statically-typed Rust, the type system guarantees that `value: i32` is always an integer. Runtime type checks are unnecessary and should be optimized away.

---

## Actual Output (Generated Code)

```rust
#[doc = "Check if value is an integer(always True due to type system)."]
pub fn type_check_int(value: i32) -> bool {
    return isinstance(value, int);
}
```

**Errors**:
```
error[E0425]: cannot find function `isinstance` in this scope
  --> src/lib.rs:48:12
   |
48 |     return isinstance(value, int);
   |            ^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `int` in this scope
  --> src/lib.rs:48:30
   |
48 |     return isinstance(value, int);
   |                              ^^^ not found in this scope
```

---

## Python Source Input

```python
def type_check_int(value: int) -> bool:
    """Check if value is an integer (always True due to type system)."""
    return isinstance(value, int)

def type_check_str(value: str) -> bool:
    """Check if value is a string (always True due to type system)."""
    return isinstance(value, str)
```

---

## Quality Gate Failures

- ❌ **cargo check**: FAIL - `isinstance` and `int` not found in scope
- ❌ **clippy -D warnings**: Not reached (compilation failure)
- ❌ **cargo test**: Not reached (compilation failure)
- ❌ **cargo-llvm-cov**: Not reached (compilation failure)

---

## Affected Examples

**Confirmed Affected**:
- `examples/01_basic_types/column_b` (type_check_int, type_check_str)

**Estimated Impact**: 10-15% of all examples (any code using isinstance for validation)

**Search Pattern**:
```bash
rg "isinstance" python-to-rust-conversion-examples/examples/*/column_a/*.py
```

---

## Fix Verification Plan

### Step 1: RED - Write Failing Test

```rust
// tests/depyler_0269_isinstance_test.rs
#[test]
fn test_isinstance_int_removed() {
    let python = r#"
def check_int(x: int) -> bool:
    return isinstance(x, int)
"#;
    let rust = depyler_transpile(python);

    // Should NOT contain isinstance
    assert!(!rust.contains("isinstance"));

    // Should return true (type system guarantees)
    assert!(rust.contains("true"));
}

#[test]
fn test_isinstance_str_removed() {
    let python = r#"
def check_str(s: str) -> bool:
    return isinstance(s, str)
"#;
    let rust = depyler_transpile(python);

    assert!(!rust.contains("isinstance"));
    assert!(rust.contains("true"));
}

#[test]
fn test_isinstance_compiles() {
    let python = r#"
def check_type(value: int) -> bool:
    return isinstance(value, int)
"#;
    let rust = depyler_transpile(python);
    write_temp_file(&rust);

    let output = Command::new("rustc")
        .arg("--crate-type").arg("lib")
        .arg("/tmp/test_0269.rs")
        .output()
        .unwrap();

    assert!(output.status.success(), "Generated code must compile");
}
```

### Step 2: GREEN - Fix Transpiler

**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs`

**Strategy**: Detect `isinstance()` calls and replace with type-system guarantees:

```rust
// In expr_gen.rs - add to handle_call_expr()
fn transpile_isinstance(args: &[Expr], func_type: &Type) -> Result<String> {
    // isinstance(value, type) with type annotations is always true
    // Return literal true since Rust's type system guarantees correctness
    Ok("true".to_string())
}
```

### Step 3: REFACTOR - Verify Quality

```bash
# Run regression tests
cargo test depyler_0269

# Verify no regressions
cargo test --workspace

# Check quality gates
cargo clippy --workspace -- -D warnings
pmat tdg . --min-grade A- --fail-on-violation
```

### Step 4: RE-TRANSPILE

```bash
# Find all affected examples
cd python-to-rust-conversion-examples
rg "isinstance" examples/*/column_a/*.py -l > affected.txt

# Re-transpile each
while read -r file; do
    base=$(dirname "$file")
    depyler transpile "$file" --output "${base/column_a/column_b}/src/lib.rs"
done < affected.txt

# Verify all compile
./scripts/validate_all_examples.sh
```

---

## Stop the Line Checklist

- [x] 🛑 Stopped all related work (matrix-testing paused)
- [x] 📋 Documented bug with reproducible test case
- [ ] 🔍 Root cause identified in transpiler code
- [ ] ✅ Fix implemented with regression test
- [ ] 🔄 Re-transpiled ALL affected examples
- [ ] ✅ All quality gates passing
- [ ] 📝 CHANGELOG.md updated
- [ ] 🚀 Ready to resume development

---

## Workaround (Temporary)

**Manual Fix** (for blocked development):

```rust
// Replace generated code:
pub fn type_check_int(value: i32) -> bool {
    return isinstance(value, int);  // ❌ INVALID
}

// With:
pub fn type_check_int(_value: i32) -> bool {
    // In Rust, type system guarantees this is always true
    true  // ✅ VALID
}
```

**Note**: This is NOT a permanent solution. Transpiler MUST be fixed.

---

## Related Issues

- **DEPYLER-0270**: Cow<'static, str> type inference bug
- **DEPYLER-0271**: Unnecessary return statements (clippy warnings)
- **DEPYLER-0272**: Unnecessary type casts

---

## References

- **Stop the Line Protocol**: [docs/processes/stop-the-line.md](../processes/stop-the-line.md)
- **Toyota Way - Jidoka**: Build quality in, stop on defects
- **Matrix Testing Spec**: [docs/specifications/matrix-testing-python-to-rust-projects.md](../specifications/matrix-testing-python-to-rust-projects.md)

---

**Created**: 2025-10-27
**Last Updated**: 2025-10-27
**Status**: OPEN - Awaiting Fix
**Priority**: P0 - Critical
