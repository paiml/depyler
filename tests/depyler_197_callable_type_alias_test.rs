//! DEPYLER-197: Test Callable type alias generation
//!
//! Python type aliases like `EventHandler = Callable[[str, dict], None]`
//! must be transpiled to Rust type aliases like:
//! `type EventHandler = Box<dyn Fn(String, HashMap<String, serde_json::Value>)>;`
//!
//! This test verifies:
//! 1. Callable type aliases are extracted from Python AST
//! 2. Callable type aliases are generated as Rust `type` declarations
//! 3. The generated code compiles successfully

use depyler_core::transpile::TranspilationPipeline;

/// Test that Callable type aliases are extracted and generated
#[test]
fn test_depyler_197_callable_type_alias_extraction() {
    let python_code = r#"
from typing import Callable

EventHandler = Callable[[str], None]

def register_handler(handler: EventHandler) -> None:
    pass
"#;

    let pipeline = TranspilationPipeline::default();
    let rust_code = pipeline.transpile(python_code).unwrap();

    // The generated Rust code MUST contain a type alias definition
    assert!(
        rust_code.contains("type EventHandler"),
        "Generated code must contain 'type EventHandler' type alias.\nActual output:\n{}",
        rust_code
    );
}

/// Test that Callable with parameters generates correct function pointer type
#[test]
fn test_depyler_197_callable_with_params() {
    let python_code = r#"
from typing import Callable

DataProcessor = Callable[[int, str], bool]

def process(processor: DataProcessor) -> bool:
    return processor(42, "test")
"#;

    let pipeline = TranspilationPipeline::default();
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should generate type alias for DataProcessor
    assert!(
        rust_code.contains("type DataProcessor"),
        "Generated code must contain 'type DataProcessor' type alias.\nActual output:\n{}",
        rust_code
    );

    // Should be a function type (Box<dyn Fn> or fn pointer)
    assert!(
        rust_code.contains("Fn(") || rust_code.contains("fn("),
        "DataProcessor type alias should contain function type.\nActual output:\n{}",
        rust_code
    );
}

/// Test that multiple Callable type aliases are all generated
#[test]
fn test_depyler_197_multiple_callable_aliases() {
    let python_code = r#"
from typing import Callable

Handler = Callable[[str], None]
Validator = Callable[[int], bool]
Processor = Callable[[], str]

def use_all(h: Handler, v: Validator, p: Processor) -> None:
    pass
"#;

    let pipeline = TranspilationPipeline::default();
    let rust_code = pipeline.transpile(python_code).unwrap();

    // All three type aliases must be generated
    assert!(
        rust_code.contains("type Handler"),
        "Missing type alias 'Handler'.\nActual output:\n{}",
        rust_code
    );
    assert!(
        rust_code.contains("type Validator"),
        "Missing type alias 'Validator'.\nActual output:\n{}",
        rust_code
    );
    assert!(
        rust_code.contains("type Processor"),
        "Missing type alias 'Processor'.\nActual output:\n{}",
        rust_code
    );
}

/// Test that the generated code compiles (integration test)
#[test]
fn test_depyler_197_callable_type_alias_compiles() {
    let python_code = r#"
from typing import Callable

EventHandler = Callable[[str], None]

class EventEmitter:
    def __init__(self) -> None:
        self._handlers: list[EventHandler] = []

    def register(self, handler: EventHandler) -> None:
        self._handlers.append(handler)
"#;

    let pipeline = TranspilationPipeline::default();
    let (rust_code, _deps) = pipeline.transpile_with_dependencies(python_code).unwrap();

    // Write to temp file and try to compile
    let temp_dir = std::env::temp_dir();
    let rs_file = temp_dir.join("depyler_197_test.rs");
    std::fs::write(&rs_file, &rust_code).unwrap();

    // Try to compile with rustc
    let output = std::process::Command::new("rustc")
        .arg("--edition")
        .arg("2021")
        .arg("--crate-type")
        .arg("lib")
        .arg(&rs_file)
        .arg("-o")
        .arg("/dev/null")
        .output()
        .expect("Failed to run rustc");

    // Clean up
    let _ = std::fs::remove_file(&rs_file);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "Generated code failed to compile:\n{}\n\nGenerated code:\n{}",
            stderr, rust_code
        );
    }
}
