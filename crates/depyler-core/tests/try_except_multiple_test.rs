//! TDD Tests for Try/Except - Multiple Exception Handlers (DEPYLER-0114 Phase 2)
//!
//! Phase 2: Multiple except clauses with different exception types
//! Python: Multiple except blocks → Rust: Match on error types
//!
//! Test Coverage (20 tests):
//! 1. Two different exception types
//! 2. Three different exception types
//! 3. Specific exception followed by bare except
//! 4. Multiple exceptions in one handler (ValueError | KeyError)
//! 5. Exception with different actions
//! 6. Nested exception handling with multiple handlers
//! 7. Exception handlers with return values
//! 8. Exception order matters (specific before general)
//! 9. Different exception variables in handlers
//! 10. Exception handlers with side effects
//! 11. Multiple handlers with pass
//! 12. Multiple handlers accessing exception message
//! 13. Exception handlers with computations
//! 14. Re-raise in specific handler
//! 15. Multiple handlers with variable assignment
//! 16. Exception handlers calling functions
//! 17. Multiple handlers with conditionals
//! 18. Exception handlers modifying state
//! 19. Chained exception handlers
//! 20. Complex exception handling pattern

use depyler_core::DepylerPipeline;

#[test]
fn test_two_exception_types() {
    let python = r#"
def parse_data(data: str) -> int:
    try:
        return int(data)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn parse_data"),
        "Should have parse_data function.\nGot:\n{}",
        rust_code
    );

    // Should have error handling for multiple types
    let has_error_handling = rust_code.contains("match") || rust_code.contains("if let");
    assert!(has_error_handling, "Should have error handling.\nGot:\n{}", rust_code);
}

#[test]
fn test_three_exception_types() {
    let python = r#"
def safe_operation(x: int) -> int:
    try:
        result = x * 2
        return result
    except ValueError:
        return -1
    except KeyError:
        return -2
    except IndexError:
        return -3
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn safe_operation"),
        "Should have safe_operation function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_specific_then_bare_except() {
    let python = r#"
def handle_errors(data: str) -> str:
    try:
        return data.upper()
    except ValueError:
        return "value_error"
    except:
        return "unknown_error"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn handle_errors"),
        "Should have handle_errors function.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Multiple exception types in one handler needs tuple support
fn test_multiple_exceptions_one_handler() {
    let python = r#"
def process(data: str) -> int:
    try:
        return int(data)
    except (ValueError, TypeError):
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn process"),
        "Should have process function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_exceptions_with_different_actions() {
    let python = r#"
def calculate(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        print("Division by zero")
        return 0
    except TypeError:
        print("Type error")
        return -1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn calculate"),
        "Should have calculate function.\nGot:\n{}",
        rust_code
    );

    // Should have print statements
    let has_print = rust_code.contains("print") || rust_code.contains("println");
    assert!(has_print, "Should have print statements.\nGot:\n{}", rust_code);
}

#[test]
fn test_nested_with_multiple_handlers() {
    let python = r#"
def nested_operation(x: int, y: int) -> int:
    try:
        try:
            return x // y
        except ValueError:
            return x
    except ZeroDivisionError:
        return 0
    except:
        return -1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn nested_operation"),
        "Should have nested_operation function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_handlers_with_return_values() {
    let python = r#"
def get_value(data: dict, key: str) -> str:
    try:
        return data[key]
    except KeyError:
        return "default_key"
    except TypeError:
        return "default_type"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn get_value"),
        "Should have get_value function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_exception_order_matters() {
    let python = r#"
def specific_first(x: int) -> str:
    try:
        result = x * 2
        return str(result)
    except ValueError:
        return "value_error"
    except Exception:
        return "general_error"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn specific_first"),
        "Should have specific_first function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_different_exception_variables() {
    let python = r#"
def handle_with_vars(data: str) -> str:
    try:
        return data.upper()
    except ValueError as ve:
        return str(ve)
    except TypeError as te:
        return str(te)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn handle_with_vars"),
        "Should have handle_with_vars function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_handlers_with_side_effects() {
    let python = r#"
def log_errors(x: int) -> int:
    try:
        result = x * 2
        return result
    except ValueError:
        print("ValueError occurred")
        return 0
    except TypeError:
        print("TypeError occurred")
        return -1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn log_errors"),
        "Should have log_errors function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_multiple_handlers_with_pass() {
    let python = r#"
def ignore_specific(x: int) -> int:
    try:
        return x * 2
    except ValueError:
        pass
    except TypeError:
        pass
    return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn ignore_specific"),
        "Should have ignore_specific function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_handlers_accessing_exception_message() {
    let python = r#"
def get_error_message(data: str) -> str:
    try:
        return int(data)
    except ValueError as e:
        return "ValueError: " + str(e)
    except TypeError as e:
        return "TypeError: " + str(e)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn get_error_message"),
        "Should have get_error_message function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_handlers_with_computations() {
    let python = r#"
def compute_fallback(x: int, y: int) -> int:
    try:
        return x // y
    except ZeroDivisionError:
        return x * 2
    except TypeError:
        return x + y
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn compute_fallback"),
        "Should have compute_fallback function.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore] // FUTURE: Re-raise needs explicit support
fn test_reraise_in_handler() {
    let python = r#"
def reraise_specific(x: int) -> int:
    try:
        return x * 2
    except ValueError:
        print("ValueError caught")
        raise
    except TypeError:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn reraise_specific"),
        "Should have reraise_specific function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_handlers_with_variable_assignment() {
    let python = r#"
def assign_in_handlers(x: int) -> int:
    result = 0
    try:
        result = x * 2
    except ValueError:
        result = -1
    except TypeError:
        result = -2
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn assign_in_handlers"),
        "Should have assign_in_handlers function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_handlers_calling_functions() {
    let python = r#"
def handle_with_calls(x: int) -> int:
    try:
        return x * 2
    except ValueError:
        return abs(x)
    except TypeError:
        return len(str(x))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn handle_with_calls"),
        "Should have handle_with_calls function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_handlers_with_conditionals() {
    let python = r#"
def conditional_handlers(x: int, flag: bool) -> int:
    try:
        return x * 2
    except ValueError:
        if flag:
            return 0
        else:
            return -1
    except TypeError:
        return -2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn conditional_handlers"),
        "Should have conditional_handlers function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_handlers_modifying_state() {
    let python = r#"
def modify_state(x: int) -> int:
    count = 0
    try:
        count = x * 2
        return count
    except ValueError:
        count = count + 1
        return count
    except TypeError:
        count = count + 2
        return count
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn modify_state"),
        "Should have modify_state function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_chained_exception_handlers() {
    let python = r#"
def chained_handling(data: list) -> int:
    try:
        return data[0]
    except IndexError:
        try:
            return len(data)
        except TypeError:
            return -1
    except ValueError:
        return -2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn chained_handling"),
        "Should have chained_handling function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_complex_exception_pattern() {
    let python = r#"
def complex_handling(a: int, b: int, c: int) -> int:
    result = 0
    try:
        temp = a // b
        result = temp * c
        return result
    except ZeroDivisionError:
        result = a * c
        return result
    except ValueError:
        result = b * c
        return result
    except TypeError:
        result = -1
        return result
    except:
        result = -2
        return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn complex_handling"),
        "Should have complex_handling function.\nGot:\n{}",
        rust_code
    );
}
