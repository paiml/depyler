//! Tests for string allocation optimizations

use depyler_core::DepylerPipeline;

#[test]
fn test_read_only_string_no_allocation() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def print_message():
    message = "Hello, World!"
    print(message)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for print_message:\n{}", rust_code);

    // Should not allocate a String for a read-only literal
    assert!(
        !rust_code.contains(".to_string()"),
        "Should not allocate String for read-only literal"
    );
}

#[test]
fn test_returned_string_uses_appropriate_type() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_greeting():
    return "Hello!"
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for get_greeting:\n{}", rust_code);

    // When returning a string literal, it should use appropriate ownership
    // Current implementation uses DynamicType or String
    assert!(
        rust_code.contains("String") || rust_code.contains("Cow") || rust_code.contains("DynamicType"),
        "Should handle string return appropriately"
    );
}

#[test]
fn test_string_concatenation_allocates() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def concat_strings(a: str, b: str) -> str:
    return a + b
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for concat_strings:\n{}", rust_code);

    // String concatenation should work
    assert!(
        rust_code.contains("+"),
        "Should contain concatenation operator"
    );
}

#[test]
fn test_interned_strings_for_repeated_literals() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def use_repeated_string():
    s1 = "repeated"
    s2 = "repeated"
    s3 = "repeated"
    s4 = "repeated"
    s5 = "repeated"
    return [s1, s2, s3, s4, s5]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for use_repeated_string:\n{}", rust_code);

    // Should generate a constant for repeated string literals
    assert!(
        rust_code.contains("const STR_"),
        "Should intern repeated string literal"
    );
}

#[test]
fn test_function_taking_str_reference() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def validate_string(s: str) -> bool:
    return len(s) > 0
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for validate_string:\n{}", rust_code);

    // Should take &str not String for read-only parameter
    assert!(rust_code.contains("&"), "Should borrow string parameter");
    assert!(rust_code.contains("str"), "Should use str type");
}

#[test]
fn test_local_string_variable_optimization() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def format_number(n: int) -> str:
    prefix = "Number: "
    return prefix + str(n)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for format_number:\n{}", rust_code);

    // Local string that's used in concatenation
    // The prefix might be inlined by the optimizer
    assert!(
        rust_code.contains("Number: ") || rust_code.contains("prefix"),
        "Should have prefix string either as variable or inlined"
    );
}
