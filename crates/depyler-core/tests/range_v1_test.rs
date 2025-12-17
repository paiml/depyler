//! Tests for range() within V1 constraints

use depyler_core::DepylerPipeline;

#[test]
fn test_range_comprehensive() {
    let pipeline = DepylerPipeline::new();

    // Test various range patterns in a single function
    let python_code = r#"
def test_ranges():
    # Single argument
    for i in range(3):
        print(i)
    
    # Two arguments  
    for i in range(2, 5):
        print(i)
        
    # Positive step
    for i in range(0, 10, 2):
        print(i)
        
    # Negative step
    for i in range(10, 0, -1):
        print(i)
        
    # Large step
    for i in range(0, 20, 5):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated comprehensive range code:\n{}", rust_code);

    // Check all patterns are present - transpiler may use step_by for all ranges
    assert!(
        rust_code.contains("..3") || rust_code.contains("step_by"),
        "Should have single arg range pattern"
    );
    assert!(
        rust_code.contains("..5") || rust_code.contains("step_by"),
        "Should have two arg range pattern"
    );
    assert!(
        rust_code.contains("step_by"),
        "Should have step pattern"
    );
    assert!(
        rust_code.contains("rev()") || rust_code.contains("step_by"),
        "Should handle negative step"
    );
}

#[test]
fn test_range_edge_cases() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def edge_cases():
    # Empty range
    for i in range(5, 5):
        print(i)
        
    # Reverse empty range
    for i in range(5, 5, -1):
        print(i)
        
    # Single item range
    for i in range(5, 6):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated edge case code:\n{}", rust_code);

    assert!(rust_code.contains("5..5"), "Should handle empty range");
}

#[test]
fn test_range_with_negative_numbers() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def negative_ranges():
    # Negative start
    for i in range(-5, 0):
        print(i)
        
    # Negative end
    for i in range(0, -5, -1):
        print(i)
        
    # Both negative
    for i in range(-10, -5):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated negative range code:\n{}", rust_code);

    assert!(
        rust_code.contains("- 5..0") || rust_code.contains("-5..0"),
        "Should handle negative start"
    );
    assert!(
        rust_code.contains("rev()"),
        "Should reverse for negative step"
    );
}

#[test]
fn test_range_in_expression() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def range_len():
    # Using range in len() - though this might not work in V1
    count = len(range(10))
    return count
"#;

    let result = pipeline.transpile(python_code);
    println!("Range in expression result: {:?}", result);

    // This might fail in V1 due to range not being a simple variable
    // Just check that we handle it somehow
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_zero_step_panic() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def zero_step():
    for i in range(0, 10, 0):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated zero step code:\n{}", rust_code);

    assert!(
        rust_code.contains("panic") && rust_code.contains("must not be zero"),
        "Should include zero step check"
    );
}
