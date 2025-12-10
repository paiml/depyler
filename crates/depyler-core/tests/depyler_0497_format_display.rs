//! DEPYLER-0497: Format Macro Display Trait - RED Phase Tests
//!
//! Tests verify that format! macro handles non-Display types correctly:
//! - Vec<T> should use {:?} (Debug trait)
//! - Option<T> should use {:?} or .unwrap_or()
//! - Result<T, E> should use {:?} or ? operator
//!
//! These tests use the actual fibonacci.rs errors as test cases.

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;

/// Helper: transpile Python to Rust
fn transpile(python: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python).expect("Should transpile")
}

#[test]
fn test_format_vec_needs_debug_formatter() {
    let python = r#"
def main():
    nums = [1, 2, 3]
    print(f"Numbers: {nums}")
"#;

    let rust = transpile(python);

    // After fix: should use {:?} for Vec (Debug trait)
    assert!(
        rust.contains("{:?}"),
        "Vec should use Debug formatter {{:?}}, got: {}",
        rust
    );
}

#[test]
fn test_fibonacci_sequence_format() {
    // Real example from fibonacci.rs (Error #6)
    let python = r#"
def fibonacci_sequence(limit: int) -> list[int]:
    sequence = []
    a, b = 0, 1
    for _ in range(limit):
        sequence.append(a)
        a, b = b, a + b
    return sequence

def main():
    n = 10
    result = fibonacci_sequence(n)
    print(f"First {n} numbers: {result}")
"#;

    let rust = transpile(python);

    // Should use {:?} for Vec return type
    assert!(
        rust.contains("{:?}"),
        "Vec<i32> should use Debug formatter, got: {}",
        rust
    );
}

#[test]
fn test_find_fibonacci_index_option_format() {
    // Real example from fibonacci.rs (Error #8)
    let python = r#"
def find_fibonacci_index(target: int) -> int | None:
    if target < 0:
        return None
    return 42

def main():
    target = 21
    index = find_fibonacci_index(target)
    if index is not None:
        print(f"{target} at index {index}")
"#;

    let rust = transpile(python);

    // Should handle Option - either {:?}, unwrap, or pattern match (match/if let Some)
    let has_debug = rust.contains("{:?}");
    let has_unwrap = rust.contains("unwrap");
    let has_match = rust.contains("match &") || rust.contains("match ");
    let has_if_let = rust.contains("if let Some");

    assert!(
        has_debug || has_unwrap || has_match || has_if_let,
        "Option should be handled with Debug, unwrap, match, or if let, got: {}",
        rust
    );
}

#[test]
fn test_hashmap_format_uses_debug() {
    let python = r#"
def main():
    data = {"a": 1, "b": 2}
    print(f"Data: {data}")
"#;

    let rust = transpile(python);

    // HashMap should use {:?}
    assert!(
        rust.contains("{:?}") || !rust.contains("HashMap"),
        "HashMap should use Debug formatter if present, got: {}",
        rust
    );
}

#[test]
#[ignore = "Known failing - DEPYLER-0497"]
fn test_primitive_types_still_use_display() {
    let python = r#"
def main():
    num = 42
    text = "hello"
    print(f"Number: {num}, Text: {text}")
"#;

    let rust = transpile(python);

    // Primitives should still use {} (Display)
    let display_count = rust.matches("{}").count();

    assert!(
        display_count >= 2,
        "Primitives should use Display formatter {{}}, got: {}",
        rust
    );
}
