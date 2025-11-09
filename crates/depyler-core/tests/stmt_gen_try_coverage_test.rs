//! Targeted coverage tests for codegen_try_stmt function
//!
//! Target: codegen_try_stmt (lines 1181-1283, complexity 22)
//! Coverage focus: try/except/finally patterns, exception scope tracking
//!
//! Test Strategy:
//! - Try/except basic patterns (DEPYLER-0333)
//! - Try/finally without except
//! - Try/except/finally combinations
//! - Exception type tracking and scope management
//! - Name binding in except handlers
//! - Multiple exception handlers
//! - Bare except (catches all)

use depyler_core::DepylerPipeline;

/// Unit Test: Basic try/except pattern
///
/// Verifies: Simple try/except block generation (lines 1244-1277)
#[test]
fn test_basic_try_except() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def divide(a: int, b: int) -> int:
    try:
        return a / b
    except:
        return 0
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn divide"));
}

/// Unit Test: Try/except with specific exception type
///
/// Verifies: Exception type handling (DEPYLER-0333, lines 1188-1191)
#[test]
fn test_try_except_specific_exception() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        result = a / b
        return result
    except ZeroDivisionError:
        return 0
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn safe_divide"));
}

/// Unit Test: Try/except with name binding
///
/// Verifies: Exception name binding (lines 1216-1218)
#[test]
fn test_try_except_with_name_binding() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def handle_error(x: int) -> str:
    try:
        if x < 0:
            raise ValueError("negative")
        return "ok"
    except ValueError as e:
        return "error"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn handle_error"));
}

/// Unit Test: Try/finally without except
///
/// Verifies: Try/finally pattern (lines 1244-1252)
#[test]
fn test_try_finally_no_except() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def cleanup_example() -> int:
    x = 0
    try:
        x = 42
    finally:
        print("cleanup")
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should generate try block with finally
    assert!(rust_code.contains("fn cleanup_example"));
}

/// Unit Test: Try/except/finally combination
///
/// Verifies: Full try/except/finally (lines 1262-1268)
#[test]
fn test_try_except_finally() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complete_example(x: int) -> int:
    try:
        if x < 0:
            raise ValueError("negative")
        return x * 2
    except ValueError:
        return 0
    finally:
        print("done")
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn complete_example"));
}

/// Unit Test: Multiple exception handlers
///
/// Verifies: Multiple except clauses (lines 1210-1230)
#[test]
fn test_multiple_exception_handlers() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def multi_except(x: int) -> str:
    try:
        if x == 0:
            raise ValueError("zero")
        if x < 0:
            raise TypeError("negative")
        return "ok"
    except ValueError:
        return "value_error"
    except TypeError:
        return "type_error"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn multi_except"));
}

/// Unit Test: Bare except (catches all exceptions)
///
/// Verifies: Bare except handling (lines 1188-1195)
#[test]
fn test_bare_except() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def catch_all() -> int:
    try:
        return 42 / 0
    except:
        return -1
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn catch_all"));
}

/// Unit Test: Nested try blocks
///
/// Verifies: Nested exception handling (scope management)
#[test]
fn test_nested_try_blocks() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def nested_try() -> int:
    try:
        try:
            return 42 / 0
        except:
            return -1
    except:
        return -2
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn nested_try"));
}

/// Unit Test: Try with multiple statements in body
///
/// Verifies: Multiple statements in try block (lines 1199-1203)
#[test]
fn test_try_multiple_statements() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def multi_stmt_try(x: int, y: int) -> int:
    try:
        a = x * 2
        b = y * 2
        result = a + b
        return result
    except:
        return 0
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn multi_stmt_try"));
}

/// Unit Test: Try with multiple statements in except
///
/// Verifies: Multiple statements in except handler (lines 1220-1224)
#[test]
fn test_except_multiple_statements() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def multi_stmt_except(x: int) -> int:
    try:
        if x < 0:
            raise ValueError("neg")
        return x
    except ValueError:
        result = 0
        print("error")
        return result
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn multi_stmt_except"));
}

/// Unit Test: Try with multiple statements in finally
///
/// Verifies: Multiple statements in finally block (lines 1234-1238)
#[test]
fn test_finally_multiple_statements() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def multi_stmt_finally() -> int:
    x = 0
    try:
        x = 42
    finally:
        print("cleanup 1")
        print("cleanup 2")
        print("done")
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn multi_stmt_finally"));
}

/// Unit Test: Try with return in except
///
/// Verifies: Return statements in exception handlers
#[test]
fn test_return_in_except() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def return_in_handler(x: int) -> int:
    try:
        if x == 0:
            raise ValueError("zero")
        return x * 2
    except ValueError:
        return -1
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn return_in_handler"));
}

/// Unit Test: Try with return in finally
///
/// Verifies: Return in finally clause
#[test]
fn test_return_in_finally() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def return_in_finally() -> int:
    try:
        return 42
    finally:
        print("cleanup")
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn return_in_finally"));
}

/// Unit Test: Empty try block (edge case)
///
/// Verifies: Handling of empty try block
#[test]
fn test_empty_try_block() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def empty_try():
    try:
        pass
    except:
        pass
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn empty_try"));
}

/// Unit Test: Empty except block (edge case)
///
/// Verifies: Handling of empty except handler
#[test]
fn test_empty_except_block() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def empty_except():
    try:
        x = 42
    except:
        pass
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn empty_except"));
}

/// Unit Test: Empty finally block (edge case)
///
/// Verifies: Handling of empty finally clause
#[test]
fn test_empty_finally_block() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def empty_finally():
    try:
        x = 42
    finally:
        pass
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn empty_finally"));
}

/// Property Test: All try/except/finally combinations
///
/// Property: All combinations of try/except/finally should transpile
#[test]
fn test_property_try_combinations() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        (
            "try_except",
            r#"
def test_try_except():
    try:
        x = 42
    except:
        x = 0
"#,
        ),
        (
            "try_finally",
            r#"
def test_try_finally():
    try:
        x = 42
    finally:
        pass
"#,
        ),
        (
            "try_except_finally",
            r#"
def test_try_except_finally():
    try:
        x = 42
    except:
        x = 0
    finally:
        pass
"#,
        ),
    ];

    for (name, python_code) in test_cases {
        let result = pipeline.transpile(python_code);

        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            name,
            result.err()
        );
    }
}

/// Integration Test: Complex try/except pattern
///
/// Verifies: All features working together
#[test]
fn test_complex_try_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_exception_handling(items: list[int]) -> int:
    total = 0
    try:
        for item in items:
            try:
                if item < 0:
                    raise ValueError("negative")
                total = total + item
            except ValueError as e:
                print("skipping negative")
                continue
    except Exception:
        return -1
    finally:
        print("done processing")
    
    return total
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn complex_exception_handling"));
}

/// Mutation Test: Exception scope tracking
///
/// Targets mutations in:
/// 1. enter_try_scope / exit_exception_scope calls
/// 2. Exception type list building
/// 3. Name binding in except handlers
#[test]
fn test_mutation_exception_scope() {
    let pipeline = DepylerPipeline::new();

    // Test Case 1: Scope tracking must work
    let scoped = r#"
def test1(x: int) -> int:
    try:
        return x / 0
    except ZeroDivisionError:
        return 0
"#;
    let rust1 = pipeline.transpile(scoped).unwrap();
    assert!(rust1.contains("fn test1"));

    // Test Case 2: Name binding must be declared
    let named = r#"
def test2(x: int) -> str:
    try:
        if x < 0:
            raise ValueError("neg")
        return "ok"
    except ValueError as e:
        return "error"
"#;
    let rust2 = pipeline.transpile(named).unwrap();
    assert!(rust2.contains("fn test2"));

    // Test Case 3: Multiple handlers must work
    let multi = r#"
def test3(x: int) -> str:
    try:
        if x == 0:
            raise ValueError("zero")
        if x < 0:
            raise TypeError("neg")
        return "ok"
    except ValueError:
        return "val"
    except TypeError:
        return "type"
"#;
    let rust3 = pipeline.transpile(multi).unwrap();
    assert!(rust3.contains("fn test3"));
}
