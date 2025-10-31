# DEPYLER-0320: Python String Method Translation Missing

**Date Created**: 2025-10-31
**Status**: 📋 ANALYSIS - Ready for Implementation
**Priority**: P2 - High (blocks common string operations)
**Estimate**: 2-3 hours
**Related**: DEPYLER-0321, DEPYLER-0323

## Problem Statement

Python string methods (`title()`, `lstrip()`, `rstrip()`, `isalnum()`) are emitted literally without translation to Rust equivalents, causing compilation errors.

## Examples

**Python → Generated Rust (WRONG)**:
```python
s.title()      # → s.title()      ❌ no method `title`
s.lstrip()     # → s.lstrip()     ❌ no method `lstrip`
s.rstrip()     # → s.rstrip()     ❌ no method `rstrip`
s.isalnum()    # → s.isalnum()    ❌ no method `isalnum`
```

**Correct Rust**:
```rust
// title(): titlecase each word
s.split_whitespace()
    .map(|word| {
        let mut c = word.chars();
        match c.next() {
            Some(f) => f.to_uppercase().chain(c.as_str().to_lowercase()).collect(),
            None => String::new(),
        }
    })
    .collect::<Vec<_>>()
    .join(" ")

s.trim_start().to_string()      // lstrip()
s.trim_end().to_string()        // rstrip()
s.chars().all(|c| c.is_alphanumeric())  // isalnum()
```

## Implementation Strategy

Create Python→Rust string method mapping table:

```rust
// In expr_gen.rs, method call handler
match method_name {
    "title" => {
        // Complex: titlecase transformation
        Ok(parse_quote! {
            #receiver.split_whitespace()
                .map(|word| {
                    let mut c = word.chars();
                    match c.next() {
                        Some(f) => f.to_uppercase().chain(c.as_str().to_lowercase()).collect::<String>(),
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
    }
    "lstrip" => Ok(parse_quote! { #receiver.trim_start().to_string() }),
    "rstrip" => Ok(parse_quote! { #receiver.trim_end().to_string() }),
    "isalnum" => Ok(parse_quote! { #receiver.chars().all(|c| c.is_alphanumeric()) }),
    // ... other methods
}
```

## Testing

```python
assert "hello world".title() == "Hello World"
assert "  hello".lstrip() == "hello"
assert "hello  ".rstrip() == "hello"
assert "abc123".isalnum() == True
assert "abc-123".isalnum() == False
```

## Impact

- **08_string_operations**: 4 errors fixed
- **Estimate**: 2-3 hours (method mapping + tests)
- **Priority**: P2 (common operations)

---
**Status**: Ready for implementation
**Lines Affected**: 35, 47, 53, 117
