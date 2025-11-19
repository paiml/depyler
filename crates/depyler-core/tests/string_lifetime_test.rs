//! Tests for proper string lifetime annotation generation

use depyler_core::DepylerPipeline;

#[test]
fn test_string_parameter_generates_lifetime() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_string(s: str) -> int:
    return len(s)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code:\n{}", rust_code);

    // DEPYLER-0357: Uses lifetime elision (&str) instead of explicit lifetimes
    // String doesn't escape, so we can borrow immutably
    assert!(
        rust_code.contains("s: &str"),
        "Should use &str with lifetime elision"
    );
    assert!(rust_code.contains("-> i32"), "Should return i32");
}

#[test]
fn test_string_escape_uses_cow() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def identity(s: str) -> str:
    return s
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for identity:\n{}", rust_code);

    // DEPYLER-0357: Uses String instead of Cow for escaping strings
    // Previous Cow<'static> behavior caused lifetime mismatch compilation errors
    assert!(
        rust_code.contains("s: String") && rust_code.contains("-> String"),
        "Should use String for string that escapes through return"
    );
}

#[test]
fn test_multiple_string_params_different_lifetimes() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def select_string(s1: str, s2: str, use_first: bool) -> str:
    if use_first:
        return s1
    else:
        return s2
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for select_string:\n{}", rust_code);

    // DEPYLER-0357: Parameters that escape take ownership
    // Multiple string params that escape both use String
    assert!(
        rust_code.contains("s1: String") && rust_code.contains("s2: String"),
        "String parameters that escape should take ownership"
    );
    assert!(rust_code.contains("-> String"), "Should return String");
}

#[test]
fn test_string_concatenation_ownership() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def concat_strings(s1: str, s2: str) -> str:
    return s1 + s2
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for concat_strings:\n{}", rust_code);

    // Parameters should be borrowed
    assert!(rust_code.contains("&"), "Parameters should be borrowed");
}

#[test]
fn test_string_mutation_takes_ownership() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def append_exclamation(s: str) -> str:
    s = s + "!"
    return s
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for append_exclamation:\n{}", rust_code);

    // Should take ownership since string is reassigned
    assert!(
        !rust_code.contains("&"),
        "Should not borrow when reassigning"
    );
    assert!(
        rust_code.contains("mut s: String"),
        "Should take ownership as mutable String"
    );
}
