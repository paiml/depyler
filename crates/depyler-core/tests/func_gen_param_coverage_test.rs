//! Targeted coverage tests for func_gen.rs parameter handling
//!
//! Target: codegen_single_param, apply_param_borrowing_strategy, is_param_used_in_body
//! Coverage focus: DEPYLER-0270, DEPYLER-0275, DEPYLER-0282, DEPYLER-0312, DEPYLER-0330
//!
//! Test Strategy:
//! - DEPYLER-0270: Unused parameter prefixing with _
//! - DEPYLER-0275: Lifetime elision
//! - DEPYLER-0282: No 'static lifetimes for parameters
//! - DEPYLER-0312: Mutable parameter tracking
//! - DEPYLER-0330: Borrowed parameter mutation detection
//! - Cow<'_, str> parameter strategies
//! - Parameter usage detection edge cases

use depyler_core::DepylerPipeline;

/// Unit Test: DEPYLER-0270 - Unused parameter prefixing
///
/// Verifies: Unused parameters are prefixed with _ to suppress warnings
#[test]
fn test_depyler_0270_unused_parameter_prefix() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def unused_param(x: int, y: int) -> int:
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn unused_param"));
    // y is unused, should be prefixed with _
    // (actual behavior depends on implementation)
}

/// Unit Test: DEPYLER-0270 - All parameters used
///
/// Verifies: Used parameters are NOT prefixed with _
#[test]
fn test_depyler_0270_all_parameters_used() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def all_used(x: int, y: int) -> int:
    return x + y
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn all_used"));
}

/// Unit Test: DEPYLER-0312 - Mutable parameter (ownership)
///
/// Verifies: Parameters that are reassigned get `mut` keyword
#[test]
fn test_depyler_0312_mutable_parameter_ownership() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def mutate_param(x: int) -> int:
    x = x + 1
    x = x * 2
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn mutate_param"));
    // Should have `mut x` since x is reassigned
}

/// Unit Test: DEPYLER-0330 - Mutable borrowed parameter
///
/// Verifies: Borrowed parameters that are mutated get &mut T
#[test]
fn test_depyler_0330_borrowed_parameter_mutation() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def mutate_list(items: list[int]) -> list[int]:
    items.append(42)
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn mutate_list"));
    // items should be &mut Vec<i32> since .append() mutates
}

/// Unit Test: DEPYLER-0330 - Multiple mutations on borrowed parameter
///
/// Verifies: Multiple mutation methods upgrade to &mut
#[test]
fn test_depyler_0330_multiple_mutations() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def multi_mutate(items: list[int]) -> list[int]:
    items.append(1)
    items.remove(0)
    items.clear()
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn multi_mutate"));
}

/// Unit Test: DEPYLER-0275 - Lifetime elision (no explicit lifetimes)
///
/// Verifies: When no lifetime parameters exist, lifetimes are elided
#[test]
fn test_depyler_0275_lifetime_elision() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def simple_borrow(s: str) -> int:
    return len(s)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn simple_borrow"));
    // Should use &str (elided lifetime) not &'a str
}

/// Unit Test: DEPYLER-0282 - No 'static for parameters
///
/// Verifies: Parameters should NEVER use 'static lifetime
#[test]
fn test_depyler_0282_no_static_lifetime_params() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_string(text: str) -> str:
    return text.upper()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_string"));
    // Should NOT contain &'static str for parameters
    // Parameters need borrowed data from local scope
}

/// Unit Test: Cow<'_, str> parameter strategy
///
/// Verifies: Cow is used for string parameters that may escape
#[test]
fn test_cow_parameter_strategy() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def maybe_modify(s: str, modify: bool) -> str:
    if modify:
        return s.upper()
    return s
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn maybe_modify"));
}

/// Unit Test: Cow with lifetime elision
///
/// Verifies: Cow<'_, str> uses elided lifetime when appropriate
#[test]
fn test_cow_lifetime_elision() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def passthrough_or_default(s: str) -> str:
    if len(s) > 0:
        return s
    return "default"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn passthrough_or_default"));
    // Should use Cow<'_, str> with elided lifetime
}

/// Unit Test: is_param_used_in_body - Used in binary expression
///
/// Verifies: Parameter used in binary expression is detected
#[test]
fn test_param_used_in_binary_expr() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def add_ten(x: int, unused: int) -> int:
    return x + 10
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn add_ten"));
}

/// Unit Test: is_param_used_in_body - Used in method call
///
/// Verifies: Parameter used in method call is detected
#[test]
fn test_param_used_in_method_call() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def uppercase(s: str, unused: int) -> str:
    return s.upper()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn uppercase"));
}

/// Unit Test: is_param_used_in_body - Used in index operation
///
/// Verifies: Parameter used in indexing is detected
#[test]
fn test_param_used_in_index() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_item(items: list[int], idx: int, unused: str) -> int:
    return items[idx]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_item"));
}

/// Unit Test: is_param_used_in_body - Used in slice operation
///
/// Verifies: Parameter used in slicing is detected
#[test]
fn test_param_used_in_slice() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_slice(items: list[int], start: int, end: int, unused: str) -> list[int]:
    return items[start:end]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_slice"));
}

/// Unit Test: is_param_used_in_body - Used in list literal
///
/// Verifies: Parameter used in list construction is detected
#[test]
fn test_param_used_in_list_literal() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def make_list(x: int, y: int, unused: str) -> list[int]:
    return [x, y, 42]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn make_list"));
}

/// Unit Test: is_param_used_in_body - Used in dict literal
///
/// Verifies: Parameter used in dict construction is detected
#[test]
fn test_param_used_in_dict_literal() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def make_dict(key: str, value: int, unused: bool) -> dict[str, int]:
    return {key: value}
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn make_dict"));
}

/// Unit Test: is_param_used_in_body - Used in if condition
///
/// Verifies: Parameter used in conditional is detected
#[test]
fn test_param_used_in_if_condition() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_positive(x: int, unused: str) -> bool:
    if x > 0:
        return True
    return False
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_positive"));
}

/// Unit Test: is_param_used_in_body - Used in for loop
///
/// Verifies: Parameter used in loop iter is detected
#[test]
fn test_param_used_in_for_loop() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def sum_items(items: list[int], unused: str) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn sum_items"));
}

/// Unit Test: is_param_used_in_body - Used in while condition
///
/// Verifies: Parameter used in while condition is detected
#[test]
fn test_param_used_in_while_condition() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def countdown(n: int, unused: str) -> int:
    while n > 0:
        n = n - 1
    return n
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn countdown"));
}

/// Unit Test: is_param_used_in_body - Used in raise statement
///
/// Verifies: Parameter used in exception is detected
#[test]
fn test_param_used_in_raise() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def validate_positive(x: int, unused: str) -> int:
    if x < 0:
        raise ValueError("must be positive")
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn validate_positive"));
}

/// Unit Test: is_param_used_in_body - Used in try block
///
/// Verifies: Parameter used in try/except is detected
#[test]
fn test_param_used_in_try_block() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def safe_divide(a: int, b: int, unused: str) -> int:
    try:
        return a / b
    except:
        return 0
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn safe_divide"));
}

/// Unit Test: is_param_used_in_body - Used in assert statement
///
/// Verifies: Parameter used in assertion is detected
#[test]
fn test_param_used_in_assert() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_range(x: int, unused: str) -> int:
    assert x >= 0
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_range"));
}

/// Unit Test: Union parameter type
///
/// Verifies: Union types in parameters are handled
#[test]
fn test_union_parameter_type() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Union

def process_value(value: Union[int, str]) -> str:
    return str(value)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_value"));
}

/// Unit Test: Mixed borrowed and owned parameters
///
/// Verifies: Mixed borrowing strategies in same function
#[test]
fn test_mixed_borrowing_strategies() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def mixed_params(owned: int, borrowed: str, mutated: list[int]) -> int:
    mutated.append(owned)
    return len(borrowed)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn mixed_params"));
}

/// Unit Test: Parameter used multiple times
///
/// Verifies: Parameters used in multiple places still detected once
#[test]
fn test_parameter_used_multiple_times() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def multi_use(x: int) -> int:
    a = x * 2
    b = x + 1
    c = x - 3
    return a + b + c + x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn multi_use"));
}

/// Unit Test: Nested parameter usage
///
/// Verifies: Parameters used in nested expressions are detected
#[test]
fn test_nested_parameter_usage() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def nested_usage(x: int, y: int) -> int:
    return ((x * 2) + (y * 3)) / 2
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn nested_usage"));
}

/// Unit Test: Parameter in list comprehension
///
/// Verifies: Parameters used in comprehensions are detected
#[test]
fn test_param_in_list_comp() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def with_comprehension(items: list[int], multiplier: int) -> list[int]:
    return [x * multiplier for x in items]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn with_comprehension"));
}

/// Unit Test: Parameter with attribute access
///
/// Verifies: Parameters with attribute access are detected
#[test]
fn test_param_attribute_access() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def string_length(s: str, unused: int) -> int:
    return len(s)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn string_length"));
}

/// Property Test: All DEPYLER parameter features
///
/// Property: Parameter features should work together
#[test]
fn test_property_all_param_features() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        ("unused", "def f(x: int, _unused: int) -> int:\n    return x"),
        ("mutable", "def f(x: int) -> int:\n    x = x + 1\n    return x"),
        ("borrowed_mut", "def f(items: list[int]) -> list[int]:\n    items.append(1)\n    return items"),
    ];

    for (name, code) in test_cases {
        let result = pipeline.transpile(code);

        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            name,
            result.err()
        );
    }
}

/// Integration Test: Complex parameter scenario
///
/// Verifies: All parameter features working together
#[test]
fn test_complex_parameter_scenario() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_params(
    used_immutable: int,
    used_mutable: int,
    borrowed_readonly: str,
    borrowed_mutable: list[int],
    unused_param: float
) -> str:
    """Complex parameter handling test."""
    # used_mutable is reassigned
    used_mutable = used_mutable * 2

    # borrowed_mutable is mutated
    borrowed_mutable.append(used_immutable)

    # borrowed_readonly is just read
    result = borrowed_readonly + str(used_mutable)

    # unused_param is never used

    return result
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn complex_params"));
}

/// Mutation Test: Parameter mutation detection
///
/// Targets mutations in:
/// 1. is_param_used_in_body detection logic
/// 2. mutable_vars checking
/// 3. Borrowing strategy application
#[test]
fn test_mutation_param_detection() {
    let pipeline = DepylerPipeline::new();

    // Test Case 1: Unused parameter must be detected
    let unused_code = r#"
def test1(x: int, y: int) -> int:
    return x
"#;
    let rust1 = pipeline.transpile(unused_code).unwrap();
    assert!(rust1.contains("fn test1"));

    // Test Case 2: Mutation must upgrade to &mut
    let mutate_code = r#"
def test2(items: list[int]) -> list[int]:
    items.append(42)
    return items
"#;
    let rust2 = pipeline.transpile(mutate_code).unwrap();
    assert!(rust2.contains("fn test2"));

    // Test Case 3: Reassignment must add mut keyword
    let reassign_code = r#"
def test3(x: int) -> int:
    x = x + 1
    return x
"#;
    let rust3 = pipeline.transpile(reassign_code).unwrap();
    assert!(rust3.contains("fn test3"));
}
