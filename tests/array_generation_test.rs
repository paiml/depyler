use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code snippets to Rust
fn transpile_snippet(python_code: &str) -> Result<String, String> {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(python_code)
        .map_err(|e| format!("Transpilation error: {e}"))
}

#[test]
fn test_literal_array_generation() {
    let py_code = r#"
def test_arrays():
    arr1 = [1, 2, 3, 4, 5]
    arr2 = [True, False, True]
    return arr1, arr2
"#;

    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");

    // Check that literal arrays are generated as arrays
    assert!(rust_code.contains("[1, 2, 3, 4, 5]"));
    assert!(rust_code.contains("[true, false, true]"));
    assert!(rust_code.contains("vec!"));
}

#[test]
fn test_array_multiplication_pattern() {
    let py_code = r#"
def test_multiplication():
    zeros = [0] * 10
    ones = [1] * 5
    pattern = [42] * 8
    reverse = 10 * [0]
    return zeros, ones, pattern, reverse
"#;

    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");

    // Check array syntax with size
    assert!(rust_code.contains("[0; 10]") || rust_code.contains("[0;\n    10]"));
    assert!(rust_code.contains("[1; 5]") || rust_code.contains("[1;\n    5]"));
    assert!(rust_code.contains("[42; 8]") || rust_code.contains("[42;\n    8]"));
}

#[test]
fn test_array_init_functions() {
    let py_code = r#"
def test_init():
    z = zeros(10)
    o = ones(5)
    f = full(8, 42)
    return z, o, f
"#;

    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");

    // Check array initialization functions
    assert!(rust_code.contains("[0; 10 as usize]"));
    assert!(rust_code.contains("[1; 5 as usize]"));
    assert!(rust_code.contains("[42; 8 as usize]"));
}

#[test]
fn test_large_array_uses_vec() {
    let py_code = r#"
def test_large():
    # Arrays larger than 32 should use vec
    large = [0] * 50
    return large
"#;

    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");

    // Large arrays will continue to use normal syntax
    assert!(rust_code.contains("[0; 50]"));
    assert!(!rust_code.contains("* 50"));
}

#[test]
fn test_non_literal_arrays_use_vec() {
    let py_code = r#"
def test_dynamic():
    x = 5
    # Non-literal arrays should use vec
    dynamic = [x] * 10
    mixed = [x, 1, 2]
    return dynamic, mixed
"#;

    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");

    // Non-literal elements should still use array syntax for multiplication
    assert!(rust_code.contains("[x; 10]") || rust_code.contains("[x;\n    10]"));
    // But mixed arrays should use vec!
    assert!(rust_code.contains("vec!"));
}

#[test]
fn test_nested_arrays() {
    let py_code = r#"
def test_nested():
    matrix = [[1, 2], [3, 4], [5, 6]]
    return matrix
"#;

    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");

    // Nested arrays should use vec! for the outer array
    assert!(rust_code.contains("vec!"));
    // But inner arrays can be arrays
    assert!(rust_code.contains("[1, 2]"));
    assert!(rust_code.contains("[3, 4]"));
}

#[test]
fn test_not_in_string_list() {
    let py_code = r#"
def test_membership(variable: str) -> bool:
    return variable not in ['A', 'B']
"#;

    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");
    println!("{}", rust_code);

    assert!(rust_code.contains("fn test_membership"));
    assert!(!rust_code.contains("contains_key"));
    assert!(rust_code.contains("contains"));
}

#[test]
fn test_array_indexing() {
    let py_code = r#"
def test_ok() -> None:
    array = [1, 2, 3, 4, 5]
    array[0]
    return None
"#;

    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");
    assert!(!rust_code.contains("() Ok(())"));
}

#[test]
fn test_array_iteration_with_fstring() {
    let py_code = r#"
def test_array_iteration_with_fstring():
    array = [10, 20, 30, 40, 50]
    temp = "10"
    for item in array:
        temp = f"{item}"
    
    return temp
"#;

    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");
    println!("{}", rust_code);
    assert!(rust_code.contains("fn test_array_iteration_with_fstring"));
    assert!(rust_code.contains("[10, 20, 30, 40, 50]"));
    assert!(rust_code.contains("for item in array"));
    assert!(rust_code.contains("format!"));
}
