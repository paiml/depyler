use depyler::transpile_snippet;

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
    assert!(!rust_code.contains("vec!"));
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
    assert!(rust_code.contains("[0; 10]") || rust_code.contains("[0;\n    10]"));
    assert!(rust_code.contains("[1; 5]") || rust_code.contains("[1;\n    5]"));
    assert!(rust_code.contains("[42; 8]") || rust_code.contains("[42;\n    8]"));
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
    
    // Large arrays should not use array syntax
    assert!(!rust_code.contains("[0; 50]"));
    assert!(rust_code.contains("* 50"));
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