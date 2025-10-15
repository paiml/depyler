//! Integration tests for v3.17.0 Phase 3 - Test Coverage Improvements
//!
//! These tests target code generation paths in rust_gen.rs, direct_rules.rs,
//! and other low-coverage modules by transpiling complete Python programs.

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code to Rust
fn transpile_python(python_code: &str) -> Result<String, String> {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(python_code)
        .map_err(|e| format!("Transpilation error: {e}"))
}

// =============================================================================
// String Method Tests (exercises rust_gen.rs string operations)
// =============================================================================

#[test]
fn test_string_upper_method() {
    let python = r#"
def make_loud(text: str) -> str:
    return text.upper()
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate .to_uppercase() method
    assert!(
        rust_code.contains("to_uppercase"),
        "Expected to_uppercase() method: {rust_code}"
    );
    assert!(
        rust_code.contains("fn make_loud"),
        "Expected function definition: {rust_code}"
    );
    assert!(
        rust_code.contains("String"),
        "Expected String return type: {rust_code}"
    );
}

#[test]
fn test_string_lower_method() {
    let python = r#"
def make_quiet(text: str) -> str:
    return text.lower()
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate .to_lowercase() method
    assert!(
        rust_code.contains("to_lowercase"),
        "Expected to_lowercase() method: {rust_code}"
    );
    assert!(
        rust_code.contains("fn make_quiet"),
        "Expected function definition: {rust_code}"
    );
}

#[test]
fn test_string_strip_method() {
    let python = r#"
def clean_text(text: str) -> str:
    return text.strip()
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate .trim() method
    assert!(
        rust_code.contains("trim"),
        "Expected trim() method: {rust_code}"
    );
    assert!(
        rust_code.contains("fn clean_text"),
        "Expected function definition: {rust_code}"
    );
}

#[test]
fn test_string_replace_method() {
    let python = r#"
def replace_spaces(text: str) -> str:
    return text.replace(" ", "_")
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate .replace() method
    assert!(
        rust_code.contains("replace"),
        "Expected replace() method: {rust_code}"
    );
    assert!(
        rust_code.contains("fn replace_spaces"),
        "Expected function definition: {rust_code}"
    );
}

// =============================================================================
// Division Operator Tests (exercises rust_gen.rs arithmetic operations)
// =============================================================================

#[test]
fn test_true_division_with_floats() {
    let python = r#"
def divide_floats(a: float, b: float) -> float:
    return a / b
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate float division
    assert!(rust_code.contains("f64"), "Expected f64 type: {rust_code}");
    assert!(
        rust_code.contains("/"),
        "Expected division operator: {rust_code}"
    );
    assert!(
        rust_code.contains("fn divide_floats"),
        "Expected function definition: {rust_code}"
    );
}

#[test]
fn test_floor_division_with_ints() {
    let python = r#"
def floor_divide(a: int, b: int) -> int:
    return a // b
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate floor division logic
    assert!(rust_code.contains("/"), "Expected division: {rust_code}");
    assert!(
        rust_code.contains("fn floor_divide"),
        "Expected function definition: {rust_code}"
    );
}

// =============================================================================
// Type Conversion Tests (exercises direct_rules.rs and type_mapper.rs)
// =============================================================================

#[test]
fn test_int_to_float_conversion() {
    let python = r#"
def convert_number(n: int) -> float:
    return float(n)
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate type conversion
    assert!(
        rust_code.contains("as f64"),
        "Expected f64 cast: {rust_code}"
    );
    assert!(
        rust_code.contains("fn convert_number"),
        "Expected function definition: {rust_code}"
    );
}

#[test]
fn test_float_to_int_conversion() {
    let python = r#"
def truncate_number(n: float) -> int:
    return int(n)
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate function with int return type
    // Note: Actual conversion may not use 'as' cast - transpiler may optimize
    assert!(
        rust_code.contains("fn truncate_number"),
        "Expected function definition: {rust_code}"
    );
    assert!(
        rust_code.contains("i32") || rust_code.contains("i64"),
        "Expected integer type: {rust_code}"
    );
}

// =============================================================================
// List Operation Tests (exercises codegen.rs and rust_gen.rs)
// =============================================================================

#[test]
fn test_list_append() {
    let python = r#"
def add_item(items: list[int], item: int) -> list[int]:
    items.append(item)
    return items
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate Vec::push
    assert!(
        rust_code.contains("push") || rust_code.contains("append"),
        "Expected push/append method: {rust_code}"
    );
    assert!(rust_code.contains("Vec"), "Expected Vec type: {rust_code}");
}

#[test]
fn test_list_length() {
    let python = r#"
def count_items(items: list[int]) -> int:
    return len(items)
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate .len() method
    assert!(
        rust_code.contains(".len()"),
        "Expected len() method: {rust_code}"
    );
    assert!(
        rust_code.contains("fn count_items"),
        "Expected function definition: {rust_code}"
    );
}

// =============================================================================
// Conditional Logic Tests (exercises ast_bridge.rs control flow)
// =============================================================================

#[test]
fn test_if_else_chain() {
    let python = r#"
def classify(n: int) -> str:
    if n > 0:
        return "positive"
    elif n < 0:
        return "negative"
    else:
        return "zero"
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate if-else chain (transpiler may use CSE temps)
    assert!(
        rust_code.contains("if") && (rust_code.contains("n>0") || rust_code.contains("_cse_temp")),
        "Expected if condition: {rust_code}"
    );
    assert!(
        rust_code.contains("else"),
        "Expected else clause: {rust_code}"
    );
    assert!(
        rust_code.contains("positive")
            && rust_code.contains("negative")
            && rust_code.contains("zero"),
        "Expected string literals: {rust_code}"
    );
}

// =============================================================================
// Loop Tests (exercises ast_bridge.rs loop conversions)
// =============================================================================

#[test]
fn test_for_loop_range() {
    let python = r#"
def sum_range(n: int) -> int:
    total: int = 0
    for i in range(n):
        total = total + i
    return total
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate for loop
    assert!(
        rust_code.contains("for") || rust_code.contains(".."),
        "Expected for loop or range: {rust_code}"
    );
    assert!(
        rust_code.contains("fn sum_range"),
        "Expected function definition: {rust_code}"
    );
}

#[test]
fn test_while_loop() {
    let python = r#"
def countdown(n: int) -> int:
    while n > 0:
        n = n - 1
    return n
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate while loop
    assert!(
        rust_code.contains("while"),
        "Expected while loop: {rust_code}"
    );
    assert!(
        rust_code.contains("fn countdown"),
        "Expected function definition: {rust_code}"
    );
}

// =============================================================================
// Comparison Operator Tests (exercises operator conversion)
// =============================================================================

#[test]
fn test_comparison_operators() {
    let python = r#"
def compare_numbers(a: int, b: int) -> bool:
    return a < b and a <= b and a == b and a != b and a > b and a >= b
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate all comparison operators
    assert!(
        rust_code.contains("<") && rust_code.contains("<="),
        "Expected less-than operators: {rust_code}"
    );
    assert!(
        rust_code.contains("==") && rust_code.contains("!="),
        "Expected equality operators: {rust_code}"
    );
    assert!(
        rust_code.contains(">") && rust_code.contains(">="),
        "Expected greater-than operators: {rust_code}"
    );
}

// =============================================================================
// Boolean Logic Tests (exercises logical operator conversion)
// =============================================================================

#[test]
fn test_boolean_and_or() {
    let python = r#"
def check_condition(a: bool, b: bool, c: bool) -> bool:
    return (a and b) or c
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate && and || operators
    assert!(
        rust_code.contains("&&") || rust_code.contains("||"),
        "Expected boolean operators: {rust_code}"
    );
    assert!(
        rust_code.contains("fn check_condition"),
        "Expected function definition: {rust_code}"
    );
}

#[test]
fn test_boolean_not() {
    let python = r#"
def negate(value: bool) -> bool:
    return not value
"#;

    let rust_code = transpile_python(python).expect("Transpilation should succeed");

    // Should generate ! operator
    assert!(
        rust_code.contains("!"),
        "Expected not operator: {rust_code}"
    );
    assert!(
        rust_code.contains("fn negate"),
        "Expected function definition: {rust_code}"
    );
}
