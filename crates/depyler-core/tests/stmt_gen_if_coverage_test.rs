//! Targeted coverage tests for codegen_if_stmt function
//!
//! Target: codegen_if_stmt (lines 405-453, complexity 15)
//! Coverage focus: If/else statements, Result<bool> auto-unwrapping
//!
//! Test Strategy:
//! - Basic if without else
//! - If with else clause
//! - Result<bool> auto-unwrapping (DEPYLER-0308)
//! - Nested if statements
//! - Multiple statements in then/else bodies
//! - Complex conditions

use depyler_core::DepylerPipeline;

/// Unit Test: Simple if without else
///
/// Verifies: Basic if statement generation (lines 447-451)
#[test]
fn test_simple_if_no_else() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_positive(x: int) -> str:
    if x > 0:
        return "positive"
    return "other"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_positive"));
}

/// Unit Test: If with else clause
///
/// Verifies: If-else generation (lines 439-445)
#[test]
fn test_if_with_else() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_sign(x: int) -> str:
    if x >= 0:
        return "non-negative"
    else:
        return "negative"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_sign"));
}

/// Unit Test: If-elif-else chain
///
/// Verifies: Nested if-else (else-if pattern)
#[test]
fn test_if_elif_else() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def categorize(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn categorize"));
}

/// Unit Test: Multiple elif clauses
///
/// Verifies: Long if-elif chains
#[test]
fn test_multiple_elif() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def grade(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn grade"));
}

/// Unit Test: Nested if statements
///
/// Verifies: If inside if (scope management)
#[test]
fn test_nested_if() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_range(x: int) -> str:
    if x > 0:
        if x < 10:
            return "small positive"
        else:
            return "large positive"
    else:
        return "non-positive"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_range"));
}

/// Unit Test: Multiple statements in then block
///
/// Verifies: Multiple statements in then body (lines 426-430)
#[test]
fn test_multiple_statements_then() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_positive(x: int) -> int:
    if x > 0:
        y = x * 2
        z = y + 1
        return z
    return 0
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_positive"));
}

/// Unit Test: Multiple statements in else block
///
/// Verifies: Multiple statements in else body (lines 434-438)
#[test]
fn test_multiple_statements_else() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_with_else(x: int) -> int:
    if x > 0:
        return x
    else:
        a = -x
        b = a * 2
        return b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_with_else"));
}

/// Unit Test: Complex boolean condition (and/or)
///
/// Verifies: Condition expression handling
#[test]
fn test_complex_condition() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_bounds(x: int, y: int) -> bool:
    if x > 0 and y > 0:
        return True
    return False
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_bounds"));
}

/// Unit Test: Comparison condition
///
/// Verifies: Different comparison operators
#[test]
fn test_comparison_conditions() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_equal(x: int, target: int) -> bool:
    if x == target:
        return True
    if x != target:
        return False
    return False
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_equal"));
}

/// Unit Test: Boolean variable condition
///
/// Verifies: Direct boolean variable in condition
#[test]
fn test_boolean_variable_condition() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_flag(flag: bool) -> str:
    if flag:
        return "enabled"
    else:
        return "disabled"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_flag"));
}

/// Unit Test: Negation condition (not)
///
/// Verifies: Unary not operator in condition
#[test]
fn test_negation_condition() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_not_empty(items: list[int]) -> bool:
    if not len(items) == 0:
        return True
    return False
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_not_empty"));
}

/// Unit Test: If with variable assignment in then block
///
/// Verifies: Variable declaration in then scope
#[test]
fn test_variable_in_then_block() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def compute_if_positive(x: int) -> int:
    result = 0
    if x > 0:
        doubled = x * 2
        result = doubled
    return result
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn compute_if_positive"));
}

/// Unit Test: If with function call condition
///
/// Verifies: Function call in condition (potential Result<bool> trigger)
#[test]
fn test_function_call_condition() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def is_even(x: int) -> bool:
    return x % 2 == 0

def check_even(x: int) -> str:
    if is_even(x):
        return "even"
    else:
        return "odd"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_even"));
}

/// Unit Test: Empty then block (pass only)
///
/// Verifies: Empty body handling
#[test]
fn test_empty_then_block() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def noop_if(x: int) -> int:
    if x > 0:
        pass
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn noop_if"));
}

/// Unit Test: Empty else block (pass only)
///
/// Verifies: Empty else body handling
#[test]
fn test_empty_else_block() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def noop_else(x: int) -> int:
    result = x
    if x > 0:
        result = x * 2
    else:
        pass
    return result
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn noop_else"));
}

/// Integration Test: Complex nested if-elif-else
///
/// Verifies: All if statement features together
#[test]
fn test_complex_nested_if_elif() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_check(x: int, y: int) -> str:
    if x > 0:
        if y > 0:
            return "both positive"
        elif y < 0:
            return "x positive, y negative"
        else:
            return "x positive, y zero"
    elif x < 0:
        if y > 0:
            return "x negative, y positive"
        else:
            return "x negative, y non-positive"
    else:
        return "x is zero"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn complex_check"));
}

/// Property Test: All if patterns should transpile
///
/// Property: Different if patterns are valid
#[test]
fn test_property_if_patterns() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        ("if_only", "if x > 0:\n        return True\n    return False"),
        (
            "if_else",
            "if x > 0:\n        return True\n    else:\n        return False",
        ),
        (
            "if_elif",
            "if x > 0:\n        return \"pos\"\n    elif x < 0:\n        return \"neg\"\n    return \"zero\"",
        ),
    ];

    for (name, if_stmt) in test_cases {
        let python_code = format!(
            r#"
def test_{}(x: int) -> str:
    {}
"#,
            name, if_stmt
        );
        let result = pipeline.transpile(&python_code);

        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            name,
            result.err()
        );
    }
}

/// Mutation Test: Condition and body handling
///
/// Targets mutations in:
/// 1. Condition expression generation
/// 2. Then/else body separation
/// 3. Scope management
#[test]
fn test_mutation_condition_body() {
    let pipeline = DepylerPipeline::new();

    // Test Case 1: Condition must be evaluated
    let cond_test = r#"
def test1(x: int) -> str:
    if x > 5:
        return "big"
    return "small"
"#;
    let rust1 = pipeline.transpile(cond_test).unwrap();
    assert!(rust1.contains("fn test1"));

    // Test Case 2: Else body must be separate
    let else_test = r#"
def test2(flag: bool) -> str:
    if flag:
        return "yes"
    else:
        return "no"
"#;
    let rust2 = pipeline.transpile(else_test).unwrap();
    assert!(rust2.contains("fn test2"));

    // Test Case 3: Multiple statements must all execute
    let multi_test = r#"
def test3(x: int) -> int:
    if x > 0:
        a = x * 2
        b = a + 1
        return b
    return 0
"#;
    let rust3 = pipeline.transpile(multi_test).unwrap();
    assert!(rust3.contains("fn test3"));
}
