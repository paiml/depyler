//! DEPYLER-0932: Dataclass Parameter Ordering Tests
//!
//! Tests for ensuring generated `new()` method parameters match the
//! field order in the struct definition, and that call sites construct
//! arguments in the correct order.

use depyler_core::DepylerPipeline;
use std::process::Command;

/// Helper to check if generated Rust code compiles
/// Uses unique temp directory per test to avoid parallel test conflicts
fn compiles_with_cargo(code: &str, test_id: &str) -> bool {
    let temp_dir = format!("/tmp/depyler_0932_{}", test_id);
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(format!("{}/src", temp_dir)).unwrap();
    std::fs::write(
        format!("{}/Cargo.toml", temp_dir),
        format!(
            r#"[package]
name = "test_0932_{}"
version = "0.1.0"
edition = "2021"
"#,
            test_id
        ),
    )
    .unwrap();
    std::fs::write(format!("{}/src/lib.rs", temp_dir), code).unwrap();

    let output = Command::new("cargo")
        .args(["build"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run cargo");

    output.status.success()
}

/// Get compilation errors
fn compile_errors_cargo(code: &str, test_id: &str) -> String {
    let temp_dir = format!("/tmp/depyler_0932_{}", test_id);
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(format!("{}/src", temp_dir)).unwrap();
    std::fs::write(
        format!("{}/Cargo.toml", temp_dir),
        format!(
            r#"[package]
name = "test_0932_{}"
version = "0.1.0"
edition = "2021"
"#,
            test_id
        ),
    )
    .unwrap();
    std::fs::write(format!("{}/src/lib.rs", temp_dir), code).unwrap();

    let output = Command::new("cargo")
        .args(["build"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run cargo");

    String::from_utf8_lossy(&output.stderr).to_string()
}

/// Test basic dataclass with multiple fields - new() should preserve order
#[test]
fn test_depyler_0932_basic_dataclass_field_order() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Person:
    name: str
    age: int
    email: str

def create_person() -> Person:
    return Person("Alice", 30, "alice@example.com")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Verify struct has fields in correct order
    assert!(
        code.contains("pub struct Person"),
        "Should generate Person struct: {}",
        code
    );

    // Verify new() has parameters in correct order (name, age, email)
    // The signature should be: new(name: String, age: i32, email: String)
    let new_pos = code.find("fn new(");
    assert!(new_pos.is_some(), "Should have new() method: {}", code);

    // Extract the new() signature
    let new_start = new_pos.unwrap();
    let new_section = &code[new_start..];
    let paren_end = new_section.find(')').unwrap_or(200);
    let new_sig = &new_section[..paren_end];

    // Parameters should appear in order: name before age before email
    let name_pos = new_sig.find("name");
    let age_pos = new_sig.find("age");
    let email_pos = new_sig.find("email");

    assert!(
        name_pos.is_some() && age_pos.is_some() && email_pos.is_some(),
        "new() should have all parameters: {}",
        new_sig
    );
    assert!(
        name_pos.unwrap() < age_pos.unwrap() && age_pos.unwrap() < email_pos.unwrap(),
        "Parameters should be in order (name, age, email): {}",
        new_sig
    );

    // CRITICAL: Generated code must compile
    if !compiles_with_cargo(&code, "basic") {
        let errors = compile_errors_cargo(&code, "basic");
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test dataclass with default values - parameters should still be in order
#[test]
fn test_depyler_0932_dataclass_with_defaults() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Config:
    host: str
    port: int = 8080
    debug: bool = False

def create_config() -> Config:
    return Config("localhost")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Verify struct exists
    assert!(
        code.contains("pub struct Config"),
        "Should generate Config struct: {}",
        code
    );

    // CRITICAL: Generated code must compile
    if !compiles_with_cargo(&code, "defaults") {
        let errors = compile_errors_cargo(&code, "defaults");
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test dataclass instantiation with positional arguments
#[test]
fn test_depyler_0932_dataclass_call_site_order() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int
    z: int

def create_point() -> Point:
    return Point(1, 2, 3)

def use_point() -> int:
    p = Point(10, 20, 30)
    return p.x + p.y + p.z
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Verify Point::new() calls have arguments in correct order
    // Should generate: Point::new(1, 2, 3) not Point::new(3, 2, 1)
    assert!(
        code.contains("Point::new(1"),
        "Should call Point::new with first arg 1: {}",
        code
    );

    // CRITICAL: Generated code must compile
    if !compiles_with_cargo(&code, "callsite") {
        let errors = compile_errors_cargo(&code, "callsite");
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test dataclass with keyword arguments at call site
#[test]
fn test_depyler_0932_dataclass_keyword_args() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Rectangle:
    width: int
    height: int

def create_rect() -> Rectangle:
    return Rectangle(width=100, height=50)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Verify struct exists
    assert!(
        code.contains("pub struct Rectangle"),
        "Should generate Rectangle struct: {}",
        code
    );

    // Even with keyword args, the call should work correctly
    // Generated: Rectangle::new(100, 50) - width=100 first, height=50 second

    // CRITICAL: Generated code must compile
    if !compiles_with_cargo(&code, "kwargs") {
        let errors = compile_errors_cargo(&code, "kwargs");
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test dataclass with mixed positional and keyword arguments
#[test]
fn test_depyler_0932_dataclass_mixed_args() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class User:
    id: int
    name: str
    active: bool

def create_user() -> User:
    return User(1, name="Bob", active=True)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // CRITICAL: Generated code must compile
    if !compiles_with_cargo(&code, "mixed") {
        let errors = compile_errors_cargo(&code, "mixed");
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test dataclass field order matches struct field order
#[test]
fn test_depyler_0932_struct_field_order_matches_python() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Employee:
    department: str
    employee_id: int
    salary: float
    is_manager: bool

def hire_employee() -> Employee:
    return Employee("Engineering", 12345, 75000.0, False)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Verify struct field order matches Python definition
    // Fields should appear in order: department, employee_id, salary, is_manager
    let struct_pos = code.find("pub struct Employee");
    assert!(struct_pos.is_some(), "Should generate Employee struct: {}", code);

    let struct_start = struct_pos.unwrap();
    let struct_end = code[struct_start..].find('}').map(|p| struct_start + p + 1).unwrap_or(code.len());
    let struct_def = &code[struct_start..struct_end];

    let dept_pos = struct_def.find("department");
    let id_pos = struct_def.find("employee_id");
    let salary_pos = struct_def.find("salary");
    let manager_pos = struct_def.find("is_manager");

    assert!(
        dept_pos.is_some() && id_pos.is_some() && salary_pos.is_some() && manager_pos.is_some(),
        "Struct should have all fields: {}",
        struct_def
    );

    // Verify order
    assert!(
        dept_pos.unwrap() < id_pos.unwrap()
            && id_pos.unwrap() < salary_pos.unwrap()
            && salary_pos.unwrap() < manager_pos.unwrap(),
        "Struct fields should be in Python definition order: {}",
        struct_def
    );

    // CRITICAL: Generated code must compile
    if !compiles_with_cargo(&code, "employee") {
        let errors = compile_errors_cargo(&code, "employee");
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}
