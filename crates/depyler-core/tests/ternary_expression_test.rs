//! TDD Tests for Ternary/Conditional Expressions (DEPYLER-0120)
//!
//! Python ternary expressions: `value_if_true if condition else value_if_false`
//! Rust if expressions: `if condition { value_if_true } else { value_if_false }`
//!
//! Test Coverage:
//! 1. Simple ternary with literals
//! 2. Ternary with variables
//! 3. Ternary in assignments
//! 4. Ternary in returns
//! 5. Ternary in function calls
//! 6. Nested ternary expressions
//! 7. Ternary with complex conditions
//! 8. Ternary with method calls
//! 9. Ternary in list comprehensions
//! 10. Ternary in dict values
//! 11. Ternary with boolean operators
//! 12. Ternary in arithmetic
//! 13. Multiple ternary in same statement
//! 14. Ternary with None/null
//! 15. Ternary in lambda bodies

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_ternary_with_literals() {
    let python = r#"
def check_sign(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn check_sign"),
        "Should have check_sign function.\nGot:\n{}",
        rust_code
    );

    // Should have if expression
    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );

    // Should have both string literals
    assert!(
        rust_code.contains("positive") && rust_code.contains("non-positive"),
        "Should have both string literals.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_with_variables() {
    let python = r#"
def max_value(a: int, b: int) -> int:
    return a if a > b else b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn max_value"),
        "Should have max_value function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_in_assignment() {
    let python = r#"
def classify(score: int) -> str:
    grade = "pass" if score >= 60 else "fail"
    return grade
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("let") && rust_code.contains("grade"),
        "Should have grade assignment.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_in_function_call() {
    let python = r#"
def process(x: int) -> int:
    return abs("positive" if x > 0 else "negative")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("abs"),
        "Should have abs function call.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_nested_ternary() {
    let python = r#"
def classify_number(x: int) -> str:
    return "positive" if x > 0 else ("zero" if x == 0 else "negative")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn classify_number"),
        "Should have classify_number function.\nGot:\n{}",
        rust_code
    );

    // Should have nested if expressions
    let if_count = rust_code.matches("if").count();
    assert!(
        if_count >= 2,
        "Should have multiple if expressions for nesting.\nGot:\n{}",
        rust_code
    );

    // Should have all three literals
    assert!(
        rust_code.contains("positive") && rust_code.contains("zero") && rust_code.contains("negative"),
        "Should have all three string literals.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_with_complex_condition() {
    let python = r#"
def check_range(x: int) -> bool:
    return True if x >= 0 and x <= 100 else False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );

    // Should have boolean operators
    assert!(
        rust_code.contains("&&") || rust_code.contains("and"),
        "Should have AND operator.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_with_arithmetic() {
    let python = r#"
def compute(x: int, y: int) -> int:
    return x + y if x > 0 else x - y
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );

    // Should have arithmetic operations
    assert!(
        rust_code.contains("+") && rust_code.contains("-"),
        "Should have arithmetic operators.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_with_none() {
    let python = r#"
def get_value(x: int) -> int:
    return x if x > 0 else None
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );

    // Should handle None
    assert!(
        rust_code.contains("None") || rust_code.contains("null"),
        "Should have None literal.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_multiple_ternary_same_statement() {
    let python = r#"
def compare(a: int, b: int) -> str:
    x = "big" if a > 10 else "small"
    y = "high" if b > 10 else "low"
    return x + y
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have multiple if expressions
    let if_count = rust_code.matches("if").count();
    assert!(
        if_count >= 2,
        "Should have at least 2 if expressions.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_with_boolean_result() {
    let python = r#"
def is_valid(x: int) -> bool:
    valid = True if x > 0 else False
    return valid
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );

    // Should have boolean literals
    assert!(
        rust_code.contains("true") || rust_code.contains("True"),
        "Should have true literal.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("false") || rust_code.contains("False"),
        "Should have false literal.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_in_list_comprehension() {
    let python = r#"
def classify_numbers(numbers: list) -> list:
    return ["positive" if x > 0 else "negative" for x in numbers]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn classify_numbers"),
        "Should have classify_numbers function.\nGot:\n{}",
        rust_code
    );

    // Should have if/else in iteration context
    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );

    // Should have iteration (map or for)
    assert!(
        rust_code.contains("map") || rust_code.contains("for") || rust_code.contains("iter"),
        "Should have iteration.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_with_comparison_chain() {
    let python = r#"
def check_bounds(x: int) -> str:
    return "in range" if 0 <= x <= 100 else "out of range"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_in_lambda() {
    let python = r#"
def classify_list(numbers: list) -> list:
    return list(map(lambda x: "positive" if x > 0 else "negative", numbers))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn classify_list"),
        "Should have classify_list function.\nGot:\n{}",
        rust_code
    );

    // Should have lambda/closure
    let has_closure = rust_code.contains("|x|") || rust_code.contains("| x |");
    assert!(has_closure, "Should have closure syntax.\nGot:\n{}", rust_code);

    // Should have if/else in lambda body
    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_ternary_with_string_methods() {
    let python = r#"
def format_string(s: str, upper: bool) -> str:
    return s.upper() if upper else s.lower()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Should have if/else expression.\nGot:\n{}",
        rust_code
    );

    // Should have method calls
    assert!(
        rust_code.contains("upper") || rust_code.contains("to_uppercase"),
        "Should have upper method.\nGot:\n{}",
        rust_code
    );
}
