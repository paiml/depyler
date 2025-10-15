//! TDD Tests for Try/Except/Finally (DEPYLER-0114 Phase 3)
//!
//! Phase 3: Finally clause for cleanup guarantees
//! Python: finally: → Rust: Cleanup code that always executes
//!
//! Test Coverage (10 tests):
//! 1. Simple try/finally (no except)
//! 2. Try/except/finally
//! 3. Finally with return in try
//! 4. Finally with exception in try
//! 5. Finally with side effects (cleanup)
//! 6. Finally with variable assignment
//! 7. Try/except/else/finally (complete pattern)
//! 8. Nested try/finally
//! 9. Finally with resource cleanup
//! 10. Finally with multiple statements

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_try_finally() {
    let python = r#"
def cleanup_operation(x: int) -> int:
    try:
        result = x * 2
        return result
    finally:
        print("cleanup")
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
        rust_code.contains("fn cleanup_operation"),
        "Should have cleanup_operation function.\nGot:\n{}",
        rust_code
    );

    // Should have print statement for cleanup
    let has_print = rust_code.contains("print") || rust_code.contains("println");
    assert!(has_print, "Should have cleanup print.\nGot:\n{}", rust_code);
}

#[test]
fn test_try_except_finally() {
    let python = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
    finally:
        print("operation complete")
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
}

#[test]
fn test_finally_with_return_in_try() {
    let python = r#"
def process_with_cleanup(x: int) -> int:
    count = 0
    try:
        count = x * 2
        return count
    finally:
        count = count + 1
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
        rust_code.contains("fn process_with_cleanup"),
        "Should have process_with_cleanup function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_finally_with_exception() {
    let python = r#"
def handle_with_cleanup(data: str) -> int:
    result = 0
    try:
        result = int(data)
        return result
    except ValueError:
        result = -1
        return result
    finally:
        print("cleanup executed")
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
        rust_code.contains("fn handle_with_cleanup"),
        "Should have handle_with_cleanup function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_finally_with_side_effects() {
    let python = r#"
def log_and_cleanup(x: int) -> int:
    try:
        result = x * 2
        print("processing")
        return result
    finally:
        print("cleanup started")
        print("cleanup finished")
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
        rust_code.contains("fn log_and_cleanup"),
        "Should have log_and_cleanup function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_finally_with_variable_assignment() {
    let python = r#"
def track_execution(x: int) -> int:
    executed = False
    try:
        result = x * 2
        return result
    finally:
        executed = True
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
        rust_code.contains("fn track_execution"),
        "Should have track_execution function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_try_except_else_finally() {
    let python = r#"
def complete_pattern(x: int) -> int:
    try:
        result = x * 2
    except ValueError:
        result = -1
    else:
        result = result + 1
    finally:
        print("done")
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
        rust_code.contains("fn complete_pattern"),
        "Should have complete_pattern function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_nested_try_finally() {
    let python = r#"
def nested_cleanup(x: int) -> int:
    try:
        try:
            result = x * 2
            return result
        finally:
            print("inner cleanup")
    finally:
        print("outer cleanup")
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
        rust_code.contains("fn nested_cleanup"),
        "Should have nested_cleanup function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_finally_with_resource_cleanup() {
    let python = r#"
def open_and_process(filename: str) -> str:
    file = None
    try:
        file = open(filename)
        return file.read()
    except IOError:
        return "error"
    finally:
        if file:
            file.close()
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
        rust_code.contains("fn open_and_process"),
        "Should have open_and_process function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_finally_with_multiple_statements() {
    let python = r#"
def complex_cleanup(x: int, y: int) -> int:
    a = 0
    b = 0
    try:
        a = x * 2
        b = y * 3
        return a + b
    except ValueError:
        return -1
    finally:
        print("cleanup a")
        print("cleanup b")
        a = 0
        b = 0
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
        rust_code.contains("fn complex_cleanup"),
        "Should have complex_cleanup function.\nGot:\n{}",
        rust_code
    );
}
