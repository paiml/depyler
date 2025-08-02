//! Tests for lifetime violations in V1 supported features

use depyler_core::DepylerPipeline;

#[test]
fn test_returning_ref_to_temporary() {
    let pipeline = DepylerPipeline::new();

    // Create a function that would need to return a reference to temporary data
    let python_code = r#"
def get_temp_string() -> str:
    return "temporary" + "data"
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should return String, not &str since it's a temporary
    assert!(
        rust_code.contains("-> String") || rust_code.contains("-> Cow"),
        "Should return owned type for temporary"
    );
}

#[test]
fn test_parameter_lifetime_propagation() {
    let pipeline = DepylerPipeline::new();

    // Function that returns one of its parameters
    let python_code = r#"
def select_longer(s1: str, s2: str) -> str:
    if len(s1) > len(s2):
        return s1
    else:
        return s2
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for select_longer:\n{}", rust_code);

    // Should handle lifetime relationships correctly
    assert!(
        rust_code.contains("Cow") || rust_code.contains("String"),
        "Should handle string selection appropriately"
    );
}

#[test]
fn test_mixed_lifetime_returns() {
    let pipeline = DepylerPipeline::new();

    // Function that might return parameter or literal
    let python_code = r#"
def get_string(use_default: bool, custom: str) -> str:
    if use_default:
        return "default"
    else:
        return custom
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for get_string:\n{}", rust_code);

    // Should use Cow or String to handle mixed lifetimes
    assert!(
        rust_code.contains("Cow") || rust_code.contains("String"),
        "Should handle mixed lifetime returns"
    );
}

#[test]
fn test_nested_scope_lifetimes() {
    let pipeline = DepylerPipeline::new();

    // Function with nested scopes
    let python_code = r#"
def process_in_scope(data: str) -> str:
    if len(data) > 0:
        temp = data + "!"
        return temp
    return ""
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for process_in_scope:\n{}", rust_code);

    // Should handle temporary in nested scope
    assert!(
        rust_code.contains("String") || rust_code.contains("Cow"),
        "Should return owned type"
    );
}

#[test]
fn test_lifetime_elision_single_param() {
    let pipeline = DepylerPipeline::new();

    // Function where lifetime can be elided
    let python_code = r#"
def get_first_char(s: str) -> str:
    if len(s) > 0:
        return s[0]
    return ""
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for get_first_char:\n{}", rust_code);

    // Should properly handle substring lifetime
    // Note: Python s[0] returns a character, not a substring
    assert!(
        rust_code.contains("char") || rust_code.contains("String"),
        "Should handle character extraction"
    );
}

#[test]
fn test_function_composition_lifetimes() {
    let pipeline = DepylerPipeline::new();

    // Functions that call each other
    let python_code = r#"
def outer(s: str) -> str:
    return inner(s)
    
def inner(s: str) -> str:
    return s
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for function composition:\n{}", rust_code);

    // Should propagate lifetimes through function calls
    assert!(rust_code.contains("inner"), "Should call inner function");
}
