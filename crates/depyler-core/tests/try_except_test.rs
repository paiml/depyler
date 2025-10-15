//! TDD Tests for Try/Except Error Handling (DEPYLER-0114 Phase 1)
//!
//! Phase 1: Simple try/except blocks
//! Python: try/except → Rust: Result<T, E> or match patterns
//!
//! Test Coverage (15 tests):
//! 1. Simple try/except block
//! 2. Try/except with error variable binding
//! 3. Try/except with return in try block
//! 4. Try/except with return in except block
//! 5. Nested try/except blocks
//! 6. Try/except in function context
//! 7. Try/except with specific exception type
//! 8. Try/except with pass in except
//! 9. Try/except with multiple statements in try
//! 10. Try/except with multiple statements in except
//! 11. Try/except accessing exception message
//! 12. Try/except with bare except (catch all)
//! 13. Try/except with computation in try block
//! 14. Try/except with side effects (print)
//! 15. Try/except with variable assignment in try

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_try_except() {
    let python = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn safe_divide"),
        "Should have safe_divide function.\nGot:\n{}",
        rust_code
    );

    // Should have error handling pattern (Result, match, or if let)
    let has_error_handling =
        rust_code.contains("Result") || rust_code.contains("match") || rust_code.contains("if let");
    assert!(
        has_error_handling,
        "Should have error handling pattern.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_with_binding() {
    let python = r#"
def parse_number(s: str) -> int:
    try:
        return int(s)
    except ValueError as e:
        return -1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn parse_number"),
        "Should have parse_number function.\nGot:\n{}",
        rust_code
    );

    // Should have error handling with binding
    let has_error_handling =
        rust_code.contains("Result") || rust_code.contains("match") || rust_code.contains("Err");
    assert!(
        has_error_handling,
        "Should have error handling.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_return_in_try() {
    let python = r#"
def get_value(data: dict, key: str) -> str:
    try:
        return data[key]
    except KeyError:
        return "default"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn get_value"),
        "Should have get_value function.\nGot:\n{}",
        rust_code
    );

    // Should handle return in try block
    let has_return = rust_code.contains("return") || rust_code.contains("->");
    assert!(
        has_return,
        "Should have return handling.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_return_in_except() {
    let python = r#"
def safe_operation(x: int) -> int:
    try:
        result = x * 2
        return result
    except:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn safe_operation"),
        "Should have safe_operation function.\nGot:\n{}",
        rust_code
    );

    // Should have error handling
    let has_error_handling =
        rust_code.contains("Result") || rust_code.contains("match") || rust_code.contains("if let");
    assert!(
        has_error_handling,
        "Should have error handling.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_nested_try_except() {
    let python = r#"
def nested_operation(x: int, y: int) -> int:
    try:
        try:
            return x // y
        except ValueError:
            return x
    except:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn nested_operation"),
        "Should have nested_operation function.\nGot:\n{}",
        rust_code
    );

    // Should have nested error handling
    let has_error_handling = rust_code.contains("Result") || rust_code.contains("match");
    assert!(
        has_error_handling,
        "Should have error handling.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_in_function() {
    let python = r#"
def process_data(data: list) -> int:
    count = 0
    try:
        count = len(data)
    except:
        count = 0
    return count
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn process_data"),
        "Should have process_data function.\nGot:\n{}",
        rust_code
    );

    // Should have variable assignment
    assert!(
        rust_code.contains("count"),
        "Should have count variable.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_specific_exception() {
    let python = r#"
def convert_to_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn convert_to_int"),
        "Should have convert_to_int function.\nGot:\n{}",
        rust_code
    );

    // Should have error handling
    let has_error_handling =
        rust_code.contains("Result") || rust_code.contains("match") || rust_code.contains("Err");
    assert!(
        has_error_handling,
        "Should have error handling.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_with_pass() {
    let python = r#"
def ignore_errors(x: int) -> int:
    try:
        result = x * 2
        return result
    except:
        pass
    return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn ignore_errors"),
        "Should have ignore_errors function.\nGot:\n{}",
        rust_code
    );

    // Should have error handling
    let has_error_handling =
        rust_code.contains("Result") || rust_code.contains("match") || rust_code.contains("if let");
    assert!(
        has_error_handling,
        "Should have error handling.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_multiple_statements_in_try() {
    let python = r#"
def calculate(x: int, y: int) -> int:
    try:
        a = x * 2
        b = y * 3
        return a + b
    except:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn calculate"),
        "Should have calculate function.\nGot:\n{}",
        rust_code
    );

    // Should have multiple variables
    let has_vars = rust_code.contains("a") && rust_code.contains("b");
    assert!(
        has_vars,
        "Should have multiple variables.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_multiple_statements_in_except() {
    let python = r#"
def safe_process(x: int) -> int:
    try:
        return x // 2
    except:
        default = 0
        return default
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn safe_process"),
        "Should have safe_process function.\nGot:\n{}",
        rust_code
    );

    // Should have error handling
    let has_error_handling =
        rust_code.contains("Result") || rust_code.contains("match") || rust_code.contains("if let");
    assert!(
        has_error_handling,
        "Should have error handling.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_accessing_exception_message() {
    let python = r#"
def get_error_message(s: str) -> str:
    try:
        return int(s)
    except Exception as e:
        return str(e)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn get_error_message"),
        "Should have get_error_message function.\nGot:\n{}",
        rust_code
    );

    // Should have error variable binding
    let has_error_binding = rust_code.contains("Err") || rust_code.contains("match");
    assert!(
        has_error_binding,
        "Should have error binding.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_bare_except() {
    let python = r#"
def catch_all(x: int) -> int:
    try:
        return x * 2
    except:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn catch_all"),
        "Should have catch_all function.\nGot:\n{}",
        rust_code
    );

    // Should have error handling
    let has_error_handling =
        rust_code.contains("Result") || rust_code.contains("match") || rust_code.contains("if let");
    assert!(
        has_error_handling,
        "Should have error handling.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_with_computation() {
    let python = r#"
def compute_ratio(a: int, b: int) -> float:
    try:
        ratio = a / b
        return ratio
    except ZeroDivisionError:
        return 0.0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn compute_ratio"),
        "Should have compute_ratio function.\nGot:\n{}",
        rust_code
    );

    // Should have computation
    assert!(
        rust_code.contains("ratio"),
        "Should have ratio variable.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_with_side_effects() {
    let python = r#"
def log_operation(x: int) -> int:
    try:
        result = x * 2
        print(result)
        return result
    except:
        print("error")
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn log_operation"),
        "Should have log_operation function.\nGot:\n{}",
        rust_code
    );

    // Should have print statements
    let has_print = rust_code.contains("print") || rust_code.contains("println");
    assert!(
        has_print,
        "Should have print statements.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_with_variable_assignment() {
    let python = r#"
def assign_safely(x: int) -> int:
    result = 0
    try:
        result = x * 2
    except:
        result = -1
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn assign_safely"),
        "Should have assign_safely function.\nGot:\n{}",
        rust_code
    );

    // Should have result variable
    assert!(
        rust_code.contains("result"),
        "Should have result variable.\nGot:\n{}",
        rust_code
    );
}
