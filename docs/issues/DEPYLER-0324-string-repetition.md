# DEPYLER-0324: String Repetition Operator (`s * n`) Not Translated

**Date Created**: 2025-10-31
**Status**: 📋 ANALYSIS - Ready for Implementation
**Priority**: P3 - Low (uncommon operation, easy workaround)
**Estimate**: 30 minutes
**Related**: None

## Problem Statement

Python's string repetition operator (`s * n`) is translated literally as `s * count`, but `Cow<'_, str>` doesn't implement `Mul<i32>`.

## Example

**Python**:
```python
def repeat_string(s: str, count: int) -> str:
    return s * count  # Repeat string 'count' times
```

**Generated Rust (WRONG)**:
```rust
pub fn repeat_string(s: Cow<'_, str>, count: i32) -> Cow<'static, str> {
    s * count  // ❌ ERROR: cannot multiply Cow<str> by i32
}
```

**Error**:
```
error[E0369]: cannot multiply `Cow<'_, str>` by `i32`
  --> src/lib.rs:242:7
   |
242 |     s * count
   |     - ^ ----- i32
   |     |
   |     Cow<'_, str>
```

**Correct Rust**:
```rust
pub fn repeat_string(s: &str, count: i32) -> String {
    s.repeat(count.max(0) as usize)  // ✅ Use .repeat() method
}
```

## Root Cause

Binary operator handler doesn't translate Python's `*` operator for strings to Rust's `.repeat()` method.

## Implementation Strategy

Add special case in BinOp handler:

```rust
// In expr_gen.rs, BinOp::Mul handler
BinOp::Mul => {
    let left_type = self.infer_type(left, ctx);
    let right_type = self.infer_type(right, ctx);

    match (left_type, right_type) {
        (Some(RustType::String | RustType::Str), Some(RustType::Int(_))) => {
            // DEPYLER-0324: String repetition s * n → s.repeat(n)
            let string_expr = self.generate_expr(left, ctx)?;
            let count_expr = self.generate_expr(right, ctx)?;
            Ok(parse_quote! { #string_expr.repeat((#count_expr).max(0) as usize) })
        }
        (Some(RustType::Int(_)), Some(RustType::String | RustType::Str)) => {
            // Reversed: n * s → s.repeat(n)
            let count_expr = self.generate_expr(left, ctx)?;
            let string_expr = self.generate_expr(right, ctx)?;
            Ok(parse_quote! { #string_expr.repeat((#count_expr).max(0) as usize) })
        }
        _ => {
            // Normal numeric multiplication
            let left_expr = self.generate_expr(left, ctx)?;
            let right_expr = self.generate_expr(right, ctx)?;
            Ok(parse_quote! { #left_expr * #right_expr })
        }
    }
}
```

## Testing

```python
assert "ab" * 3 == "ababab"
assert 3 * "ab" == "ababab"  # Reversed order
assert "x" * 0 == ""
assert "x" * -1 == ""  # Negative should be 0
```

## Impact

- **08_string_operations**: 1 error fixed
- **Estimate**: 30 minutes
- **Priority**: P3 (uncommon operation)
- **Line Affected**: 242

---
**Status**: Ready for implementation
