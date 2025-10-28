# DEPYLER-0276: CSE Must Preserve Type Context

**Status**: FIXED ✅
**Priority**: P1 (Blocking - breaks compilation)
**Discovered**: 2025-10-28
**Fixed**: 2025-10-28
**Root Cause**: CSE optimization doesn't preserve type information when extracting expressions

## Issue

After DEPYLER-0275 fix (removing premature cast from `len()`), CSE optimization now generates type mismatches.

### Example

**Python**:
```python
right: int = len(arr) - 1
```

**Generated Rust (BROKEN)**:
```rust
let _cse_temp_0 = arr.len();  // Returns usize
let mut right: i32 = _cse_temp_0 - 1;  // ERROR: expected i32, found usize
```

**Should Generate**:
```rust
let _cse_temp_0 = arr.len() as i32;  // Cast to i32 for context
let mut right: i32 = _cse_temp_0 - 1;  // OK
```

## Root Cause

In `optimizer.rs`, the CSE optimization extracts `len(arr)` without checking:
1. The target type annotation (`right: int`)
2. The usage context (subtraction with `i32`)

DEPYLER-0275 removed the cast from `convert_len_call()`, expecting the assignment/return logic to add it. But CSE runs BEFORE that logic, so it stores `usize` without cast.

## Solution ✅

**Implemented Fix**: Reverted DEPYLER-0275 change to `convert_len_call()` in `expr_gen.rs:690-701`

**Rationale**:
- Initially removed cast to fix double-cast issue (DEPYLER-0275)
- However, CSE optimization runs BEFORE return statement processing
- CSE compatibility is more important than avoiding occasional double casts
- Simpler fix than making CSE aware of type contexts

**Implementation**:
```rust
// expr_gen.rs:690-701
fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("len() requires exactly one argument");
    }
    let arg = &args[0];

    // DEPYLER-0276: Keep cast for CSE compatibility
    // Python's len() returns int (maps to i32)
    // Rust's .len() returns usize, so we cast to i32
    // CSE optimization runs before return statement processing, so we need the cast here
    // to avoid type mismatches when CSE extracts len() into a temporary variable
    Ok(parse_quote! { #arg.len() as i32 })
}
```

**Alternative Considered**: Make CSE optimization type-aware (more complex, deferred)

## Test Case

```python
def binary_search(arr: List[int], target: int) -> int:
    right: int = len(arr) - 1  # CSE should preserve i32 type
```

Should generate:
```rust
let _cse_temp_0 = arr.len() as i32;  // With cast
let mut right: i32 = _cse_temp_0 - 1;  // OK
```

## Impact

- **Severity**: P1 - Breaks compilation
- **Scope**: Any code using `len()` with CSE optimization
- **Examples Affected**: `binary_search.py`, `contracts_example.py`

## Verification ✅

**Test Results**:
- `binary_search.py` → transpiles and compiles with zero errors/warnings
- `contracts_example.py` → transpiles and compiles with zero errors/warnings
- Generated code correctly shows: `let _cse_temp_0 = arr.len() as i32;`

**Validation Command**:
```bash
cargo run --release --bin depyler -- transpile examples/showcase/binary_search.py --output /tmp/test.rs
rustc --crate-type lib /tmp/test.rs --deny warnings  # ✅ PASSES
```

## Related

- DEPYLER-0275: Removed premature cast from `len()` (exposed this bug)
- CSE optimization in `optimizer.rs`

## Extreme TDD Cycle ✅

- **RED**: binary_search.rs failed to compile (type mismatch: expected i32, found usize)
- **GREEN**: Reverted cast removal in expr_gen.rs:690-701 → compiles successfully
- **REFACTOR**: Verified on multiple showcase examples, all pass
