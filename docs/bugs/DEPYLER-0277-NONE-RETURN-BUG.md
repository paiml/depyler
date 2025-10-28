# DEPYLER-0277: Incorrect Return Value for None in Optional Return Types

**Status**: FIXED ✅
**Priority**: P1 (Blocking - breaks compilation)
**Discovered**: 2025-10-28
**Fixed**: 2025-10-28
**Root Cause**: Return statement codegen generates `Ok(())` instead of `Ok(None)` for functions returning `Option<T>`

## Issue

Functions with `Optional[T]` return type that return `None` generate incorrect Rust code.

### Example

**Python**:
```python
def process_config(config: Dict[str, str]) -> Optional[str]:
    """Process configuration dictionary and return debug value if present."""
    if "debug" in config:
        return config["debug"]
    return None  # Python None
```

**Generated Rust (BROKEN)**:
```rust
pub fn process_config(config: &HashMap<String, String>) -> Result<Option<String>, IndexError> {
    let _cse_temp_0 = config.contains_key("debug");
    if _cse_temp_0 {
        return Ok(Some(config.get("debug").cloned().unwrap_or_default()));
    }
    Ok(())  // ERROR: expected Option<String>, found ()
}
```

**Compilation Error**:
```
error[E0308]: mismatched types
  --> examples/showcase/process_config.rs:26:8
   |
26 |     Ok(())
   |     -- ^^ expected `Option<String>`, found `()`
   |     |
   |     arguments to this enum variant are incorrect
```

**Should Generate**:
```rust
pub fn process_config(config: &HashMap<String, String>) -> Result<Option<String>, IndexError> {
    let _cse_temp_0 = config.contains_key("debug");
    if _cse_temp_0 {
        return Ok(Some(config.get("debug").cloned().unwrap_or_default()));
    }
    Ok(None)  // Correct: None for Optional return type
}
```

## Root Cause

In statement generation (`stmt_gen.rs`), when processing return statements:
1. Python `None` literal gets converted to Rust `()`
2. But when function return type is `Result<Option<T>, E>`, the inner value should be `None`, not `()`
3. The transpiler doesn't check if return type is Optional and map `None` → `None` instead of `None` → `()`

## Solution ✅

**Implemented Fix**: Added explicit handling for `is_none_literal` when `is_optional_return` is true in `stmt_gen.rs:204-210, 223-229`

### Implementation

Modified `codegen_return_stmt()` in `/home/noah/src/depyler/crates/depyler-core/src/rust_gen/stmt_gen.rs`:

**Case 1: Result<Option<T>, E> return type** (lines 204-210):
```rust
} else if is_optional_return && is_none_literal {
    // DEPYLER-0277: Return None for Optional types (not ())
    if use_return_keyword {
        Ok(quote! { return Ok(None); })
    } else {
        Ok(quote! { Ok(None) })
    }
```

**Case 2: Option<T> return type** (lines 223-229):
```rust
} else if is_optional_return && is_none_literal {
    // DEPYLER-0277: Return None for Optional types (not ()) - non-Result case
    if use_return_keyword {
        Ok(quote! { return None; })
    } else {
        Ok(quote! { None })
    }
```

**Key Insight**: The existing code had logic for `is_optional_return && !is_none_literal` (wrap in `Some()`), but no explicit branch for `is_optional_return && is_none_literal`. This caused None literals to fall through to the general case, which generated `Ok(())` instead of `Ok(None)`.

## Test Case

```python
def optional_function(x: int) -> Optional[str]:
    if x > 0:
        return "positive"
    return None  # Should generate Ok(None), not Ok(())
```

Should generate:
```rust
pub fn optional_function(x: i32) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if x > 0 {
        return Ok(Some("positive".to_string()));
    }
    Ok(None)  // Correct
}
```

## Impact

- **Severity**: P1 - Breaks compilation for any function returning Optional types
- **Scope**: All functions with `Optional[T]` return type annotations
- **Examples Affected**: `process_config.py`, `annotated_example.py`

## Verification ✅

**Test Results**:
- `process_config.py` → transpiles and compiles with zero errors/warnings ✅
- `annotated_example.py` → transpiles correctly, generates `Ok(None)` on line 79 ✅
  - Still fails compilation due to DEPYLER-0278 (fnv dependency), but None bug is fixed

**Validation Command**:
```bash
cargo run --release --bin depyler -- transpile examples/showcase/process_config.py --output /tmp/test.rs
rustc --crate-type lib /tmp/test.rs --deny warnings  # ✅ PASSES
```

## Related

- Statement generation in `stmt_gen.rs`
- Type mapping for Optional types in `type_mapper.rs`
- DEPYLER-0278: FNV dependency issue (separate bug)

## Extreme TDD Cycle ✅

- **RED**: process_config.rs failed to compile (`expected Option<String>, found ()`)
- **GREEN**: Added explicit `is_none_literal` branches for both Result and non-Result cases
- **REFACTOR**: Verified on process_config and annotated_example, both generate correct `Ok(None)`
