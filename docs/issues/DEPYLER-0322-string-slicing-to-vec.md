# DEPYLER-0322: String Slicing Generates `.to_vec()` Instead of `.to_string()`

**Date Created**: 2025-10-31
**Status**: 📋 ANALYSIS - Ready for Implementation
**Priority**: P2 - High (blocks string slicing operations)
**Estimate**: 45 minutes
**Related**: DEPYLER-0323 (string iteration)

## Problem Statement

String slicing operations (`s[start:end]`) generate `.to_vec()` which doesn't exist for `&str` (that's for `&[T]` slices). Should generate `.to_string()` for string types.

## Examples

**Python**:
```python
def get_substring(s: str, start: int, end: int) -> str:
    return s[start:end]

def get_first_n(s: str, n: int) -> str:
    return s[:n]
```

**Generated Rust (WRONG)**:
```rust
pub fn get_substring(s: &str, start: i32, end: i32) -> String {
    let start = (start).max(0) as usize;
    let stop = (end).max(0) as usize;
    if start < s.len() {
        s[start..stop.min(s.len())].to_vec()  // ❌ ERROR: no .to_vec() for str
    } else {
        Vec::new()  // ❌ ERROR: returns Vec<u8>, expects String
    }
}
```

**Errors**:
```
error[E0599]: no method named `to_vec` found for type `str`
  --> src/lib.rs:196:47
   |
196 |             base[start..stop.min(base.len())].to_vec()
   |                                                ^^^^^^ method not found

error[E0308]: mismatched types
   --> src/lib.rs:198:13
    |
198 |         Vec::new()
    |         ^^^^^^^^^^ expected `String`, found `Vec<_>`
```

**Correct Rust**:
```rust
pub fn get_substring(s: &str, start: i32, end: i32) -> String {
    let start = (start).max(0) as usize;
    let stop = (end).max(0) as usize;
    if start < s.len() {
        s[start..stop.min(s.len())].to_string()  // ✅ Use .to_string()
    } else {
        String::new()  // ✅ Return empty String
    }
}
```

## Root Cause

Transpiler treats ALL slicing uniformly and emits `.to_vec()`:
- `list[1:3]` → `list[1..3].to_vec()` ✅ Correct for &[T]
- `string[1:3]` → `string[1..3].to_vec()` ❌ Wrong for &str

## Implementation Strategy

Detect base type when generating slice operations:

```rust
// In slice expression generation
fn generate_slice(&mut self, base: &HirExpr, slice: &Slice, ctx: &mut Context) -> Result<TokenStream> {
    let base_expr = self.generate_expr(base, ctx)?;
    let base_type = self.infer_type(base, ctx);

    // Generate slice range...
    let slice_expr = parse_quote! { #base_expr[#range] };

    match base_type {
        Some(RustType::String | RustType::Str) => {
            // DEPYLER-0322: Strings need .to_string() not .to_vec()
            Ok(parse_quote! { #slice_expr.to_string() })
        }
        Some(RustType::Vec(_)) | Some(RustType::List(_)) => {
            // Collections use .to_vec()
            Ok(parse_quote! { #slice_expr.to_vec() })
        }
        _ => {
            // Default: assume collection
            Ok(parse_quote! { #slice_expr.to_vec() })
        }
    }
}
```

## Testing

```python
def test_slice(s: str) -> str:
    return s[1:3]  # Should emit .to_string()

def test_list_slice(items: list[int]) -> list[int]:
    return items[1:3]  # Should emit .to_vec() (unchanged)
```

## Impact

- **08_string_operations**: 3 errors fixed directly + 3 type mismatch errors resolved
- **Estimate**: 45 minutes
- **Priority**: P2 (common string operation)
- **Lines Affected**: 196, 209, 220 (+ mismatches at 198, 222, 252)

---
**Status**: Ready for implementation
