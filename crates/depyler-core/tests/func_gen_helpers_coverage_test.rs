//! Extended coverage tests for func_gen.rs helper functions
//!
//! Target: func_gen.rs gaps (216 uncovered lines at 78.66%)
//! Coverage focus: Helper functions, string method classification, return type analysis
//!
//! Test Strategy:
//! - is_rust_keyword: All keywords and edge cases
//! - classify_string_method: Owned vs borrowed methods
//! - contains_owned_string_method: Expression tree traversal
//! - function_returns_owned_string: Return statement analysis
//! - contains_string_concatenation: Concatenation detection
//! - return_type_expects_float: Type expectation analysis

use depyler_core::DepylerPipeline;

// ============================================================================
// is_rust_keyword Coverage Tests
// ============================================================================

/// Unit Test: Rust keywords - control flow keywords
#[test]
fn test_rust_keywords_control_flow() {
    let pipeline = DepylerPipeline::new();

    // Test control flow keywords: if, else, for, while, loop, break, continue, return
    let python_code = r#"
def use_if(x: int) -> int:
    if x > 0:
        return x
    else:
        return 0

def use_for(items: list[int]) -> int:
    total = 0
    for item in items:
        total = total + item
    return total

def use_while(n: int) -> int:
    count = 0
    while count < n:
        count = count + 1
    return count
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn use_if"));
    assert!(rust_code.contains("fn use_for"));
    assert!(rust_code.contains("fn use_while"));
}

/// Unit Test: Rust keywords - type/visibility keywords
#[test]
fn test_rust_keywords_types_visibility() {
    let pipeline = DepylerPipeline::new();

    // Python code that might generate code near type/visibility keywords
    let python_code = r#"
def process_type(value: int) -> str:
    return str(value)

def use_self(obj) -> int:
    return 42

def create_struct() -> dict[str, int]:
    return {"value": 123}
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_type"));
    assert!(rust_code.contains("fn use_self"));
    assert!(rust_code.contains("fn create_struct"));
}

/// Unit Test: Rust keywords - async/await keywords
#[test]
fn test_rust_keywords_async_await() {
    let pipeline = DepylerPipeline::new();

    // Test code that might use async/await-related names
    let python_code = r#"
def process_async() -> int:
    return 42

def process_await() -> str:
    return "done"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_async"));
    assert!(rust_code.contains("fn process_await"));
}

/// Unit Test: Rust keywords - reserved/future keywords
#[test]
fn test_rust_keywords_reserved() {
    let pipeline = DepylerPipeline::new();

    // Test code with names matching reserved keywords: abstract, become, box, do, final, etc.
    let python_code = r#"
def use_abstract() -> int:
    return 1

def use_final() -> int:
    return 2

def use_override() -> int:
    return 3
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn use_abstract"));
    assert!(rust_code.contains("fn use_final"));
    assert!(rust_code.contains("fn use_override"));
}

// ============================================================================
// classify_string_method Coverage Tests
// ============================================================================

/// Unit Test: String methods that return owned String
#[test]
fn test_string_methods_owned_upper_lower() {
    let pipeline = DepylerPipeline::new();

    // Methods: upper, lower
    let python_code = r#"
def make_upper(text: str) -> str:
    return text.upper()

def make_lower(text: str) -> str:
    return text.lower()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn make_upper"));
    assert!(rust_code.contains("fn make_lower"));
}

/// Unit Test: String methods that return owned String - strip variants
#[test]
fn test_string_methods_owned_strip() {
    let pipeline = DepylerPipeline::new();

    // Methods: strip, lstrip, rstrip
    let python_code = r#"
def clean_text(text: str) -> str:
    return text.strip()

def clean_left(text: str) -> str:
    return text.lstrip()

def clean_right(text: str) -> str:
    return text.rstrip()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn clean_text"));
    assert!(rust_code.contains("fn clean_left"));
    assert!(rust_code.contains("fn clean_right"));
}

/// Unit Test: String methods that return owned String - transformation
#[test]
fn test_string_methods_owned_transform() {
    let pipeline = DepylerPipeline::new();

    // Methods: replace, title, capitalize, swapcase
    let python_code = r#"
def replace_text(text: str) -> str:
    return text.replace("old", "new")

def title_case(text: str) -> str:
    return text.title()

def cap_first(text: str) -> str:
    return text.capitalize()

def swap_case(text: str) -> str:
    return text.swapcase()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn replace_text"));
    assert!(rust_code.contains("fn title_case"));
    assert!(rust_code.contains("fn cap_first"));
    assert!(rust_code.contains("fn swap_case"));
}

/// Unit Test: String methods that return borrowed/bool
#[test]
fn test_string_methods_borrowed() {
    let pipeline = DepylerPipeline::new();

    // Methods: startswith, endswith, isalpha, isdigit, find
    let python_code = r#"
def starts_with_hello(text: str) -> bool:
    return text.startswith("hello")

def ends_with_world(text: str) -> bool:
    return text.endswith("world")

def is_alpha(text: str) -> bool:
    return text.isalpha()

def is_digit(text: str) -> bool:
    return text.isdigit()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn starts_with_hello"));
    assert!(rust_code.contains("fn ends_with_world"));
    assert!(rust_code.contains("fn is_alpha"));
    assert!(rust_code.contains("fn is_digit"));
}

// ============================================================================
// contains_owned_string_method Coverage Tests
// ============================================================================

/// Unit Test: Detect owned string method in MethodCall
#[test]
fn test_contains_owned_method_methodcall() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def transform_string(text: str) -> str:
    return text.upper()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn transform_string"));
    // Should generate code that handles owned String return
}

/// Unit Test: Detect owned string method in Binary expression
#[test]
fn test_contains_owned_method_binary() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def combine_transforms(a: str, b: str) -> str:
    x = a.upper()
    y = b.lower()
    return x + y
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn combine_transforms"));
}

/// Unit Test: Detect owned string method in IfExpr
#[test]
fn test_contains_owned_method_ifexpr() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def conditional_transform(text: str, upper: bool) -> str:
    return text.upper() if upper else text.lower()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn conditional_transform"));
}

/// Unit Test: No owned method in literal/var/other expressions
#[test]
fn test_contains_owned_method_no_method() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def return_literal() -> str:
    return "hello"

def return_var(text: str) -> str:
    return text

def return_list() -> list[str]:
    return ["a", "b"]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn return_literal"));
    assert!(rust_code.contains("fn return_var"));
    assert!(rust_code.contains("fn return_list"));
}

// ============================================================================
// function_returns_owned_string Coverage Tests
// ============================================================================

/// Unit Test: Function returns owned string method result
#[test]
fn test_function_returns_owned_string_direct() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def get_upper(text: str) -> str:
    return text.upper()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_upper"));
}

/// Unit Test: Function returns owned string via multiple returns
#[test]
fn test_function_returns_owned_string_multiple() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def transform_conditional(text: str, mode: int) -> str:
    if mode == 1:
        return text.upper()
    elif mode == 2:
        return text.lower()
    else:
        return text.title()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn transform_conditional"));
}

/// Unit Test: Function does not return owned string
#[test]
fn test_function_not_returns_owned_string() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def check_starts(text: str) -> bool:
    return text.startswith("hello")

def return_plain(text: str) -> str:
    return text
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_starts"));
    assert!(rust_code.contains("fn return_plain"));
}

// ============================================================================
// contains_string_concatenation Coverage Tests (DEPYLER-0270)
// ============================================================================

/// Unit Test: Detect string concatenation with Add operator
#[test]
fn test_contains_string_concat_add() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def concat_strings(a: str, b: str) -> str:
    return a + b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn concat_strings"));
    // Should handle as owned String (format! macro)
}

/// Unit Test: Detect string concatenation in f-strings
#[test]
fn test_contains_string_concat_fstring() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def format_message(name: str, age: int) -> str:
    return f"Hello {name}, you are {age} years old"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn format_message"));
    assert!(rust_code.contains("format!") || rust_code.contains("String"));
}

/// Unit Test: Detect concat in nested Binary expressions
#[test]
fn test_contains_string_concat_nested() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def triple_concat(a: str, b: str, c: str) -> str:
    return a + b + c
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn triple_concat"));
}

/// Unit Test: Detect concat in IfExpr branches
#[test]
fn test_contains_string_concat_ifexpr() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def conditional_concat(a: str, b: str, flag: bool) -> str:
    return a + b if flag else a + "default"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn conditional_concat"));
}

/// Unit Test: No concat in non-Add expressions
#[test]
fn test_no_string_concat_other_ops() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def compare_strings(a: str, b: str) -> bool:
    return a == b

def multiply_number(a: int, b: int) -> int:
    return a * b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn compare_strings"));
    assert!(rust_code.contains("fn multiply_number"));
}

// ============================================================================
// function_returns_string_concatenation Coverage Tests
// ============================================================================

/// Unit Test: Function returns string concatenation
#[test]
fn test_function_returns_concat_direct() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def join_names(first: str, last: str) -> str:
    return first + " " + last
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn join_names"));
}

/// Unit Test: Function returns f-string
#[test]
fn test_function_returns_fstring() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn greet"));
}

/// Unit Test: Function returns concat in multiple branches
#[test]
fn test_function_returns_concat_branches() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def format_greeting(name: str, formal: bool) -> str:
    if formal:
        return "Dear " + name
    else:
        return "Hi " + name
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn format_greeting"));
}

// ============================================================================
// return_type_expects_float Coverage Tests
// ============================================================================

/// Unit Test: Return type expects float - direct float
#[test]
fn test_return_expects_float_direct() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def get_pi() -> float:
    return 3.14159
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_pi"));
    assert!(rust_code.contains("f64") || rust_code.contains("float"));
}

/// Unit Test: Return type expects float - Optional[float]
#[test]
fn test_return_expects_float_optional() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
from typing import Optional

def maybe_float(flag: bool) -> Optional[float]:
    if flag:
        return 3.14
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn maybe_float"));
    assert!(rust_code.contains("Option"));
}

/// Unit Test: Return type expects float - list[float]
#[test]
fn test_return_expects_float_list() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def get_floats() -> list[float]:
    return [1.0, 2.0, 3.0]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_floats"));
    assert!(rust_code.contains("Vec") || rust_code.contains("vec"));
}

/// Unit Test: Return type expects float - tuple with float
#[test]
fn test_return_expects_float_tuple() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def get_coordinates() -> tuple[float, float]:
    return (1.5, 2.5)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_coordinates"));
}

/// Unit Test: Return type does not expect float - int
#[test]
fn test_return_not_expects_float_int() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def get_count() -> int:
    return 42
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_count"));
    assert!(rust_code.contains("i32") || rust_code.contains("i64") || rust_code.contains("int"));
}

/// Unit Test: Return type does not expect float - str
#[test]
fn test_return_not_expects_float_str() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def get_name() -> str:
    return "Alice"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_name"));
    assert!(rust_code.contains("String") || rust_code.contains("str"));
}

// ============================================================================
// Property Tests: Combination scenarios
// ============================================================================

/// Property Test: All string transformation methods work
#[test]
fn test_property_all_string_transforms() {
    let pipeline = DepylerPipeline::new();

    let methods = vec![
        ("upper", "HELLO"),
        ("lower", "hello"),
        ("title", "Hello"),
        ("capitalize", "Hello"),
    ];

    for (method, _expected) in methods {
        let python_code = format!(
            r#"
def test_{}_method(text: str) -> str:
    return text.{}()
"#,
            method, method
        );
        let result = pipeline.transpile(&python_code);

        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            method,
            result.err()
        );
    }
}

/// Property Test: All string query methods work
#[test]
fn test_property_all_string_queries() {
    let pipeline = DepylerPipeline::new();

    let methods = vec![
        "startswith",
        "endswith",
        "isalpha",
        "isdigit",
        "isalnum",
        "isspace",
    ];

    for method in methods {
        let python_code = if method == "startswith" || method == "endswith" {
            format!(
                r#"
def test_{}_method(text: str) -> bool:
    return text.{}("test")
"#,
                method, method
            )
        } else {
            format!(
                r#"
def test_{}_method(text: str) -> bool:
    return text.{}()
"#,
                method, method
            )
        };

        let result = pipeline.transpile(&python_code);

        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            method,
            result.err()
        );
    }
}

/// Integration Test: Complex string operations
#[test]
fn test_integration_complex_string_ops() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def process_text(text: str, mode: str) -> str:
    if text.startswith("hello"):
        return text.upper()
    elif text.endswith("world"):
        return text.lower()
    elif mode == "title":
        return text.title()
    else:
        return text.strip()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_text"));
}

/// Integration Test: Mixed concat and method calls
#[test]
fn test_integration_concat_and_methods() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def format_and_transform(first: str, last: str) -> str:
    full_name = first + " " + last
    return full_name.upper()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn format_and_transform"));
}
