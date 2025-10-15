//! TDD Tests for F-String Support (DEPYLER-0110 Phase 1)
//!
//! This test MUST FAIL initially (Red phase), then pass after implementation (Green phase)
//!
//! F-String mapping:
//! Python: f"Hello {name}"
//! Rust:   format!("Hello {}", name)

use depyler_core::DepylerPipeline;

#[test]
fn test_fstring_simple_variable() {
    let python = r#"
def greet(name: str) -> str:
    return f"Hello {name}"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should contain format! macro (flexible whitespace)
    let has_format = rust_code.contains("format!") || rust_code.contains("format !");
    assert!(
        has_format,
        "F-string should generate format!() macro.\nGot:\n{}",
        rust_code
    );

    // Should have the format string template
    assert!(
        rust_code.contains("\"Hello {}\""),
        "F-string template should be \"Hello {{}}\".\nGot:\n{}",
        rust_code
    );

    // Should have the variable as argument
    assert!(
        rust_code.contains("name"),
        "F-string should pass 'name' to format!().\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fstring_multiple_variables() {
    let python = r#"
def describe(name: str, age: int) -> str:
    return f"{name} is {age} years old"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    let has_format = rust_code.contains("format!") || rust_code.contains("format !");
    assert!(
        has_format,
        "Should generate format!().\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("{} is {} years old"),
        "Template should have two {{}}.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fstring_in_assignment() {
    let python = r#"
def test() -> str:
    name = "Alice"
    message = f"Welcome {name}"
    return message
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    let has_format = rust_code.contains("format!") || rust_code.contains("format !");
    assert!(
        has_format,
        "F-string in assignment should work.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fstring_empty_string() {
    let python = r#"
def test() -> str:
    return f""
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Empty f-string should just be empty string
    assert!(
        rust_code.contains("\"\"") || rust_code.contains("String::new()"),
        "Empty f-string should be empty string.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fstring_no_interpolation() {
    let python = r#"
def test() -> str:
    return f"Hello World"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // F-string with no variables should be plain string
    assert!(
        rust_code.contains("\"Hello World\""),
        "F-string with no vars should be plain string.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fstring_with_numbers() {
    let python = r#"
def test(x: int, y: float) -> str:
    return f"x={x}, y={y}"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    let has_format = rust_code.contains("format!") || rust_code.contains("format !");
    assert!(
        has_format,
        "F-string with numbers should work.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("x={}, y={}"),
        "Template should preserve literal text.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fstring_escaped_braces() {
    let python = r#"
def test() -> str:
    return f"{{literal braces}}"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // {{}} in Python f-string → {{}} in Rust format string (still escaped)
    assert!(
        rust_code.contains("{{literal braces}}") || rust_code.contains("{literal braces}"),
        "Escaped braces should be preserved.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fstring_concatenation() {
    let python = r#"
def test(first: str, last: str) -> str:
    full_name = f"{first}" + f" {last}"
    return full_name
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should contain string concatenation with format!
    let has_format = rust_code.contains("format!") || rust_code.contains("format !");
    assert!(
        has_format,
        "Concatenated f-strings should work.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fstring_in_function_call() {
    let python = r#"
def helper(s: str) -> str:
    return s

def test(name: str) -> str:
    return helper(f"Hello {name}")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    let has_format = rust_code.contains("format!") || rust_code.contains("format !");
    assert!(
        has_format,
        "F-string as function argument should work.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_fstring_multiline() {
    let python = r#"
def test(name: str, age: int) -> str:
    message = f"""Hello {name}
You are {age} years old"""
    return message
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    let has_format = rust_code.contains("format!") || rust_code.contains("format !");
    assert!(
        has_format,
        "Multiline f-string should work.\nGot:\n{}",
        rust_code
    );
}
