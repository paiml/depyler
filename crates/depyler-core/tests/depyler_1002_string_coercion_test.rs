//! DEPYLER-1002: String to &str coercion in function arguments
//!
//! When a function is defined with a `str` type parameter (which becomes `&str` in Rust),
//! string literals passed to that function should NOT get `.to_string()` added.
//!
//! Python:
//!   def parse_int_safe(s: str) -> int:
//!       ...
//!   result = parse_int_safe("42")
//!
//! Correct Rust:
//!   pub fn parse_int_safe(s: &str) -> i64 { ... }
//!   let result = parse_int_safe("42");
//!
//! Wrong Rust:
//!   pub fn parse_int_safe(s: &str) -> i64 { ... }
//!   let result = parse_int_safe("42".to_string());  // ERROR: expected &str, found String

use depyler_core::DepylerPipeline;

fn transpile(python: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python).unwrap()
}

#[test]
fn test_depyler_1002_str_param_no_to_string() {
    let python = r#"
def parse_int_safe(s: str) -> int:
    try:
        return int(s)
    except:
        return 0

def main():
    result = parse_int_safe("42")
    print(f"Result: {result}")
"#;

    let rust = transpile(python);

    // The function call should NOT have .to_string()
    // Correct: parse_int_safe("42")
    // Wrong: parse_int_safe("42".to_string())
    assert!(
        rust.contains(r#"parse_int_safe("42")"#),
        "String literal passed to &str param should not have .to_string(): {}",
        rust
    );

    // Make sure the wrong pattern is NOT present
    assert!(
        !rust.contains(r#"parse_int_safe("42".to_string())"#),
        "Should not add .to_string() to string literal for &str param: {}",
        rust
    );
}

#[test]
fn test_depyler_1002_multiple_str_params() {
    let python = r#"
def concat(a: str, b: str) -> str:
    return a + b

def main():
    result = concat("hello", "world")
    print(result)
"#;

    let rust = transpile(python);

    // Both string literals should not have .to_string()
    assert!(
        rust.contains(r#"concat("hello""#) || rust.contains(r#"concat ( "hello""#),
        "First string literal should not have .to_string(): {}",
        rust
    );
    assert!(
        rust.contains(r#""world")"#) || rust.contains(r#""world" )"#),
        "Second string literal should not have .to_string(): {}",
        rust
    );
}

#[test]
fn test_depyler_1002_mixed_params() {
    let python = r#"
def format_number(s: str, n: int) -> str:
    return f"{s}: {n}"

def main():
    result = format_number("count", 42)
    print(result)
"#;

    let rust = transpile(python);

    // String literal should not have .to_string()
    assert!(
        rust.contains(r#"format_number("count""#) || rust.contains(r#"format_number ( "count""#),
        "String literal should not have .to_string(): {}",
        rust
    );
}
