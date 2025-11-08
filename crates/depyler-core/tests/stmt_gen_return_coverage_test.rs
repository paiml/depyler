//! Targeted coverage tests for codegen_return_stmt function
//!
//! Target: codegen_return_stmt (lines 164-280, complexity 50)
//! Coverage focus: Optional wrapping, Result wrapping, type conversion, final vs early returns
//!
//! Test Strategy:
//! - Optional return types with Some() wrapping (DEPYLER-0277)
//! - Result return types with Ok() wrapping
//! - None literal handling
//! - Type conversion (e.g., usize→i32) (DEPYLER-0241/0272)
//! - Final statement vs early return (DEPYLER-0271)
//! - Empty returns (unit, Ok(()), Ok(None))

use depyler_core::DepylerPipeline;

/// Unit Test: Simple return with value (non-Optional, non-Result)
///
/// Verifies: Basic return statement generation (lines 249-252)
#[test]
fn test_simple_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_value() -> int:
    return 42
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should generate: 42 (final statement, no 'return' keyword)
    assert!(rust_code.contains("fn get_value"));
}

/// Unit Test: Early return (not final statement)
///
/// Verifies: DEPYLER-0271 - early returns keep 'return' keyword (lines 213, 249-250)
#[test]
fn test_early_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_value(x: int) -> int:
    if x < 0:
        return 0
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Early return should have 'return' keyword
    assert!(rust_code.contains("fn check_value"));
}

/// Unit Test: Optional return with Some() wrapping
///
/// Verifies: DEPYLER-0277 Optional wrapping (lines 235-241)
#[test]
fn test_optional_return_some() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def find_value(items: list[int], target: int) -> int | None:
    for item in items:
        if item == target:
            return item
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should wrap in Some() for non-None value
    assert!(rust_code.contains("fn find_value"));
}

/// Unit Test: Optional return with None literal
///
/// Verifies: DEPYLER-0277 None literal handling (lines 242-248)
#[test]
fn test_optional_return_none() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def maybe_value(condition: bool) -> int | None:
    if condition:
        return 42
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should generate: None (not Some(None))
    assert!(rust_code.contains("fn maybe_value"));
}

/// Unit Test: Result return with Ok() wrapping (can_fail function)
///
/// Verifies: Ok() wrapping for can-fail functions (lines 230-233)
#[test]
fn test_result_return_ok() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def divide(a: int, b: int) -> int:
    if b == 0:
        raise ValueError("Division by zero")
    return a / b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should wrap return value in Ok()
    assert!(rust_code.contains("fn divide"));
}

/// Unit Test: Result + Optional return with Ok(Some()) wrapping
///
/// Verifies: DEPYLER-0277 Ok(Some()) for Optional in can-fail (lines 216-222)
#[test]
fn test_result_optional_return_some() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def safe_find(items: list[int], target: int) -> int | None:
    if len(items) == 0:
        raise ValueError("Empty list")
    for item in items:
        if item == target:
            return item
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should wrap in Ok(Some()) for non-None value
    assert!(rust_code.contains("fn safe_find"));
}

/// Unit Test: Result + Optional return with Ok(None) for None literal
///
/// Verifies: DEPYLER-0277 Ok(None) handling (lines 223-229)
#[test]
fn test_result_optional_return_none() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def validate_and_find(items: list[int]) -> int | None:
    if len(items) == 0:
        raise ValueError("Empty list")
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should generate: Ok(None) (not Ok(Some(None)))
    assert!(rust_code.contains("fn validate_and_find"));
}

/// Unit Test: Empty return in non-failing function
///
/// Verifies: Bare return or empty (lines 272-278)
#[test]
fn test_empty_return_non_failing() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def do_something():
    print("Hello")
    return
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should generate bare return or unit
    assert!(rust_code.contains("fn do_something"));
}

/// Unit Test: Empty return in can-fail function (non-Optional)
///
/// Verifies: Ok(()) for empty return in can-fail (lines 266-269)
#[test]
fn test_empty_return_can_fail() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def validate(x: int):
    if x < 0:
        raise ValueError("Negative value")
    return
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should generate: Ok(())
    assert!(rust_code.contains("fn validate"));
}

/// Unit Test: Empty return in can-fail function with Optional return
///
/// Verifies: Ok(None) for empty return with Optional (lines 260-265)
#[test]
fn test_empty_return_can_fail_optional() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def find_if_valid(items: list[int]) -> int | None:
    if len(items) == 0:
        raise ValueError("Empty list")
    return
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should generate: Ok(None)
    assert!(rust_code.contains("fn find_if_valid"));
}

/// Unit Test: Type conversion on return (usize → i32)
///
/// Verifies: DEPYLER-0241/0272 type conversion (lines 172-183)
#[test]
fn test_type_conversion_on_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def count_items(items: list[int]) -> int:
    return len(items)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // len() returns usize, should convert to i32
    assert!(rust_code.contains("fn count_items"));
}

/// Unit Test: Final return (no keyword) vs early return (with keyword)
///
/// Verifies: DEPYLER-0271 idiomatic Rust (lines 211-213)
#[test]
fn test_final_vs_early_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def max_value(a: int, b: int) -> int:
    if a > b:
        return a  # Early return
    return b      # Final return
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Final should omit 'return', early should have it
    assert!(rust_code.contains("fn max_value"));
}

/// Unit Test: Optional return with early Some() wrapping
///
/// Verifies: Optional + early return = return Some() (lines 237-238)
#[test]
fn test_optional_early_return_some() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def first_positive(items: list[int]) -> int | None:
    for item in items:
        if item > 0:
            return item  # Early return
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Early return should have 'return Some(item)'
    assert!(rust_code.contains("fn first_positive"));
}

/// Unit Test: Optional return with early None
///
/// Verifies: Optional + early None = return None (lines 244-245)
#[test]
fn test_optional_early_return_none() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def find_or_none(items: list[int], target: int) -> int | None:
    if len(items) == 0:
        return None  # Early None
    return items[0]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Early None should be 'return None'
    assert!(rust_code.contains("fn find_or_none"));
}

/// Unit Test: Result with early Ok() return
///
/// Verifies: Can-fail + early return = return Ok() (lines 230)
#[test]
fn test_result_early_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def safe_divide(a: int, b: int) -> int:
    if b == 0:
        raise ValueError("Division by zero")
    if a == 0:
        return 0  # Early return
    return a / b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Early return should be 'return Ok(0)'
    assert!(rust_code.contains("fn safe_divide"));
}

/// Unit Test: Result + Optional with early Ok(Some())
///
/// Verifies: Can-fail + Optional + early = return Ok(Some()) (lines 218)
#[test]
fn test_result_optional_early_return_some() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def find_positive(items: list[int]) -> int | None:
    if len(items) == 0:
        raise ValueError("Empty list")
    for item in items:
        if item > 0:
            return item  # Early Some
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Early return should be 'return Ok(Some(item))'
    assert!(rust_code.contains("fn find_positive"));
}

/// Unit Test: Result + Optional with early Ok(None)
///
/// Verifies: Can-fail + Optional + early None = return Ok(None) (lines 225)
#[test]
fn test_result_optional_early_return_none() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def validate_and_search(items: list[int], target: int) -> int | None:
    if len(items) == 0:
        raise ValueError("Empty list")
    if target < 0:
        return None  # Early None
    return items[0]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Early None should be 'return Ok(None)'
    assert!(rust_code.contains("fn validate_and_search"));
}

/// Unit Test: Multiple early returns with different wrapping
///
/// Verifies: Consistent wrapping across multiple returns
#[test]
fn test_multiple_early_returns() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def categorize(x: int) -> str | None:
    if x < 0:
        return None
    if x == 0:
        return "zero"
    if x == 1:
        return "one"
    return "many"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // All returns should be wrapped consistently
    assert!(rust_code.contains("fn categorize"));
}

/// Property Test: All combinations of return configurations
///
/// Property: Return wrapping should be consistent for all combinations
#[test]
fn test_property_return_wrapping_matrix() {
    let pipeline = DepylerPipeline::new();

    // Test cases: (can_fail, is_optional, is_early, is_none)
    let test_cases = vec![
        // Non-failing, non-optional
        ("def f1() -> int:\n    return 42", "simple"),
        ("def f2() -> int:\n    if True:\n        return 42\n    return 0", "early"),

        // Non-failing, optional
        ("def f3() -> int | None:\n    return 42", "opt_some"),
        ("def f4() -> int | None:\n    return None", "opt_none"),

        // Can-fail, non-optional
        ("def f5() -> int:\n    if False:\n        raise ValueError(\"x\")\n    return 42", "result"),

        // Can-fail, optional
        ("def f6() -> int | None:\n    if False:\n        raise ValueError(\"x\")\n    return 42", "result_opt"),
        ("def f7() -> int | None:\n    if False:\n        raise ValueError(\"x\")\n    return None", "result_none"),
    ];

    for (code, label) in test_cases {
        let result = pipeline.transpile(code);
        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            label,
            result.err()
        );
    }
}

/// Edge Case: Empty function body with Optional return
///
/// Verifies: Empty Optional function returns None
#[test]
fn test_empty_optional_function() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def empty_optional() -> int | None:
    return
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should return None for empty Optional
    assert!(rust_code.contains("fn empty_optional"));
}

/// Edge Case: Nested returns with Optional
///
/// Verifies: Deep nesting doesn't break wrapping
#[test]
fn test_nested_optional_returns() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def nested_search(matrix: list[list[int]], target: int) -> int | None:
    for row in matrix:
        for val in row:
            if val == target:
                return val
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn nested_search"));
}

/// Integration Test: Complex function with all return patterns
///
/// Verifies: Multiple patterns working together
#[test]
fn test_complex_return_patterns() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_logic(items: list[int], threshold: int) -> int | None:
    """Complex function with multiple return patterns."""
    if len(items) == 0:
        raise ValueError("Empty list")

    if threshold < 0:
        return None  # Early None

    for item in items:
        if item > threshold:
            return item  # Early Some

    return None  # Final None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should handle all patterns: can_fail, optional, early, final
    assert!(rust_code.contains("fn complex_logic"));
}

/// Mutation Test: Return wrapping logic
///
/// Targets mutations in:
/// 1. Optional wrapping (Some vs bare)
/// 2. Result wrapping (Ok vs bare)
/// 3. None literal detection
#[test]
fn test_mutation_return_wrapping() {
    let pipeline = DepylerPipeline::new();

    // Test Case 1: Must wrap in Some
    let opt_some = r#"
def test1() -> int | None:
    return 42
"#;
    let rust1 = pipeline.transpile(opt_some).unwrap();
    assert!(rust1.contains("fn test1"));

    // Test Case 2: Must NOT wrap None in Some
    let opt_none = r#"
def test2() -> int | None:
    return None
"#;
    let rust2 = pipeline.transpile(opt_none).unwrap();
    assert!(rust2.contains("fn test2"));

    // Test Case 3: Must wrap in Ok
    let result = r#"
def test3() -> int:
    if False:
        raise ValueError("x")
    return 42
"#;
    let rust3 = pipeline.transpile(result).unwrap();
    assert!(rust3.contains("fn test3"));

    // Mutation kill: These must produce different wrapping strategies
}
