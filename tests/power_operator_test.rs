use depyler::transpile_snippet;

#[test]
fn test_basic_power_operator() {
    let py_code = r#"
def test_power():
    a = 2 ** 3
    b = 10 ** 2
    return a, b
"#;
    
    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");
    
    // Check for checked_pow for overflow safety
    assert!(rust_code.contains("checked_pow"));
    assert!(rust_code.contains("2.checked_pow(3 as u32)"));
    assert!(rust_code.contains("10.checked_pow(2 as u32)"));
}

#[test]
fn test_float_power() {
    let py_code = r#"
def test_float_power():
    a = 2.5 ** 2
    b = 4 ** 0.5
    return a, b
"#;
    
    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");
    
    // Float base uses .powf()
    assert!(rust_code.contains("2.5.powf(2 as f64)"));
    // Float exponent uses .powf()
    assert!(rust_code.contains("(4 as f64).powf(0.5)"));
}

#[test]
fn test_negative_exponent() {
    let py_code = r#"
def test_negative():
    a = 2 ** -1
    b = 10 ** -2
    return a, b
"#;
    
    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");
    
    // Negative exponents should use float power
    assert!(rust_code.contains("(2 as f64).powf(-1 as f64)") || 
           rust_code.contains("(2 as f64).powf(- 1 as f64)"));
    assert!(rust_code.contains("(10 as f64).powf(-2 as f64)") || 
           rust_code.contains("(10 as f64).powf(- 2 as f64)"));
}

#[test]
fn test_power_with_variables() {
    let py_code = r#"
def compute_power(base: int, exp: int) -> int:
    return base ** exp
"#;
    
    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");
    
    // Should have runtime check for negative exponents
    assert!(rust_code.contains("if exp >= 0"));
    assert!(rust_code.contains("checked_pow"));
    assert!(rust_code.contains("powf"));
}

#[test]
fn test_power_precedence() {
    let py_code = r#"
def test_precedence():
    a = 2 + 3 ** 2      # Should be 2 + 9 = 11
    b = (2 + 3) ** 2    # Should be 5 ** 2 = 25
    c = 2 ** 3 * 4      # Should be 8 * 4 = 32
    return a, b, c
"#;
    
    let rust_code = transpile_snippet(py_code).expect("Failed to transpile");
    
    // Check proper parenthesization
    assert!(rust_code.contains("2 + 3.checked_pow") || rust_code.contains("(2 + 3.checked_pow"));
    assert!(rust_code.contains("5.checked_pow") || rust_code.contains("(2 + 3).checked_pow"));
}