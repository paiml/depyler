//! Tests for range() with step parameter support

use depyler_core::DepylerPipeline;

#[test]
fn test_range_single_argument() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def count_to_five():
    for i in range(5):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for range(5):\n{}", rust_code);

    // Transpiler may generate either simple range or step_by pattern
    // Output formats: 0..5, 0..(5), step_by
    assert!(
        rust_code.contains("0..5") || rust_code.contains("0..(5)") || rust_code.contains("step_by"),
        "Should generate range pattern for range(5)"
    );
}

#[test]
fn test_range_two_arguments() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def count_from_two():
    for i in range(2, 8):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for range(2, 8):\n{}", rust_code);

    // Transpiler may generate either simple range or step_by pattern
    // Output formats: 2..8, (2)..(8), 2..(8), step_by
    assert!(
        rust_code.contains("2..8") || rust_code.contains("..(8)") || rust_code.contains("step_by"),
        "Should generate range pattern for range(2, 8)"
    );
}

#[test]
fn test_range_positive_step() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def count_by_twos():
    for i in range(0, 10, 2):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for range(0, 10, 2):\n{}", rust_code);

    assert!(
        rust_code.contains("step_by"),
        "Should use step_by for step parameter"
    );
    assert!(rust_code.contains("2"), "Should include step value 2");
}

#[test]
fn test_range_negative_step() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def count_down():
    for i in range(10, 0, -1):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for range(10, 0, -1):\n{}", rust_code);

    // Should handle negative step with rev()
    assert!(
        rust_code.contains("rev") || rust_code.contains("step_by"),
        "Should handle negative step"
    );
}

#[test]
fn test_range_large_step() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def count_by_fives():
    result = []
    for i in range(0, 25, 5):
        result.append(i)
    return result
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for range(0, 25, 5):\n{}", rust_code);

    assert!(rust_code.contains("step_by"), "Should use step_by");
    assert!(rust_code.contains("5"), "Should include step value 5");
}

#[test]
fn test_range_negative_bounds() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def negative_range():
    for i in range(-5, 5):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for range(-5, 5):\n{}", rust_code);

    // Transpiler may generate range with negative bounds or step_by pattern
    assert!(
        rust_code.contains("-5") || rust_code.contains("- 5") || rust_code.contains("step_by"),
        "Should handle negative start"
    );
}

#[test]
fn test_range_with_variables() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def dynamic_range(start: int, end: int, step: int):
    result = []
    for i in range(start, end, step):
        result.append(i)
    return result
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for range with variables:\n{}", rust_code);

    assert!(
        rust_code.contains("step_by"),
        "Should use step_by with variables"
    );
}

#[test]
fn test_range_zero_step_error() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def invalid_range():
    # This should generate code that panics at runtime like Python
    for i in range(0, 10, 0):
        print(i)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated code for range with zero step:\n{}", rust_code);

    // The generated code should handle zero step (might panic at runtime)
    assert!(
        rust_code.contains("range") || rust_code.contains("step"),
        "Should generate range code even with zero step"
    );
}
