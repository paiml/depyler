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

    // DEPYLER-1109: PyMul trait handles list multiplication semantically correctly
    // Python [x] * n creates a list, which maps to Vec via PyMul trait
    assert!(rust_code.contains("py_mul") || rust_code.contains("[0; 10]"));
    assert!(rust_code.contains("vec![0]") || rust_code.contains("[0; 10]"));
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
    // DEPYLER-REFACTOR-001: Extracted module generates cleaner [0; N] without unnecessary cast
    // for small literal sizes (≤32), since the size is a compile-time constant
    assert!(rust_code.contains("[0; 10]"));
    assert!(rust_code.contains("[1; 5]"));
    assert!(rust_code.contains("[42; 8]"));
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

    // DEPYLER-1109: PyMul trait handles list multiplication
    // Either array syntax [0; 50] or PyMul trait call is acceptable
    assert!(rust_code.contains("[0; 50]") || rust_code.contains("py_mul"));
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

    // DEPYLER-1109: PyMul trait handles list multiplication with variables
    // Either array syntax [x; 10] or PyMul trait call is acceptable
    assert!(rust_code.contains("[x; 10]") || rust_code.contains("py_mul"));
    // Mixed arrays should use vec!
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
    // Extract just the function body to check (prelude contains contains_key in DepylerValue)
    let fn_start = rust_code.find("fn test_membership").unwrap();
    let fn_body = &rust_code[fn_start..fn_start + 200.min(rust_code.len() - fn_start)];
    // Membership check should NOT use contains_key (that's for dicts)
    assert!(!fn_body.contains("contains_key"), "Function should not use contains_key for list membership");
    // Membership check can use either .contains() or .any(|x| x == value)
    assert!(fn_body.contains("contains") || fn_body.contains("any(|"),
        "Expected membership check using contains or any()");
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
