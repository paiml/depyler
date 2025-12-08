//! DEPYLER-0824: Test for cast followed by binary operator
//!
//! Problem: When `len(self.field)` returns `self.field.len() as i32` and a
//! comparison operator follows, the code generator produces invalid Rust:
//! `self.field.len() as i32 < x` which fails parse_quote! with "expected `,`"
//!
//! Solution: Wrap cast expressions in parentheses: `(self.field.len() as i32) < x`

use depyler_core::DepylerPipeline;

/// Test that len() of class field followed by comparison generates valid Rust
#[test]
fn test_DEPYLER_0824_len_class_field_comparison() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
class Decoder:
    def __init__(self) -> None:
        self.buffer: list[int] = []

    def check(self) -> bool:
        x = 5
        if len(self.buffer) < x:
            return True
        return False
"#;

    // This should NOT panic - it should generate valid Rust
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());
}

/// Test with bytes type
#[test]
fn test_DEPYLER_0824_len_bytes_field_comparison() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
class Decoder:
    def __init__(self) -> None:
        self.buffer = b""

    def decode(self) -> bool:
        total_length = 5
        if len(self.buffer) < total_length:
            return False
        return True
"#;

    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());
}

/// Test arithmetic expression with cast in comparison
#[test]
fn test_DEPYLER_0824_arithmetic_cast_comparison() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
class Decoder:
    def __init__(self) -> None:
        self.buffer = b""

    def decode(self) -> bool:
        length_bytes = 2
        remaining_length = 10
        total_length = 1 + length_bytes + remaining_length
        if len(self.buffer) < total_length:
            return False
        return True
"#;

    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());
}
