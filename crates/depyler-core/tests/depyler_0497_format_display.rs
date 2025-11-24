//! DEPYLER-0497: Option/Result Types in format! Macro Need Debug Formatting
//!
//! Tests that Result<T>, Option<T>, and Vec<T> types in format! macros use
//! {:?} Debug formatting instead of {} Display formatting.
//!
//! BUG: format!("value: {}", result_fn()) tries to use Display trait
//! Expected: format!("value: {:?}", result_fn()) uses Debug trait

use depyler_core::DepylerPipeline;
use std::io::Write;

#[test]
fn test_format_result_type() {
    // Result-returning function in format! macro
    let python = r#"
def divide(a: int, b: int) -> int:
    if b == 0:
        raise ValueError("Division by zero")
    return a // b

def show_result():
    result = divide(10, 2)
    print(f"Result: {result}")
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should use {:?} for Result type or unwrap it
    // Either: format!("Result: {:?}", divide(...))
    // Or: format!("Result: {}", divide(...)?)
    let has_debug_fmt = rust.contains("{:?}");
    let has_question_unwrap = rust.contains("divide(10, 2)?");

    assert!(
        has_debug_fmt || has_question_unwrap,
        "BUG: format! with Result type must use {{:?}} or unwrap with ?\n\
         Expected: format!(\"Result: {{:?}}\", result) or format!(\"Result: {{}}\", result?)\n\
         Generated:\n{}",
        rust
    );
}

#[test]
fn test_format_option_type() {
    // Option value in format! macro
    let python = r#"
from typing import Optional

def find_value(items: list, target: int) -> Optional[int]:
    for i, item in enumerate(items):
        if item == target:
            return i
    return None

def show_index():
    items = [10, 20, 30]
    index = find_value(items, 20)
    print(f"Found at index: {index}")
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should use {:?} for Option type or unwrap_or
    let has_debug_fmt = rust.contains("{:?}");
    let has_unwrap_or = rust.contains(".unwrap_or");

    assert!(
        has_debug_fmt || has_unwrap_or,
        "BUG: format! with Option type must use {{:?}} or .unwrap_or()\n\
         Expected: format!(\"Found at index: {{:?}}\", index)\n\
         Generated:\n{}",
        rust
    );
}

#[test]
fn test_format_vec_type() {
    // Vec/List value in format! macro
    let python = r#"
def get_numbers() -> list:
    return [1, 2, 3, 4, 5]

def show_list():
    numbers = get_numbers()
    print(f"Numbers: {numbers}")
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should use {:?} for Vec type
    assert!(
        rust.contains("{:?}"),
        "BUG: format! with Vec type must use {{:?}}\n\
         Expected: format!(\"Numbers: {{:?}}\", numbers)\n\
         Generated:\n{}",
        rust
    );
}

#[test]
fn test_print_result_function_call() {
    // Direct print of Result-returning function
    let python = r#"
def compute(x: int) -> int:
    if x < 0:
        raise ValueError("Negative value")
    return x * 2

def main():
    print(compute(5))
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should handle Result in print macro
    let has_debug_fmt = rust.contains("{:?}");
    let has_question = rust.contains("compute(5)?");
    let has_unwrap = rust.contains("compute(5).unwrap");

    assert!(
        has_debug_fmt || has_question || has_unwrap,
        "BUG: print with Result must use {{:?}}, ?, or .unwrap()\n\
         Generated:\n{}",
        rust
    );
}

#[test]
fn test_multiple_format_args_mixed_types() {
    // Multiple arguments with mixed types in format!
    let python = r#"
from typing import Optional

def process_data(x: int) -> int:
    if x < 0:
        raise ValueError("Invalid")
    return x

def find_index(target: int) -> Optional[int]:
    return 42 if target > 0 else None

def show_data():
    result = process_data(10)
    index = find_index(20)
    values = [1, 2, 3]
    print(f"Result: {result}, Index: {index}, Values: {values}")
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should handle multiple non-Display types correctly
    // At minimum, need some {:?} formatting
    assert!(
        rust.contains("{:?}"),
        "BUG: format! with multiple non-Display types must use {{:?}}\n\
         Generated:\n{}",
        rust
    );
}

#[test]
fn test_f_string_with_expressions() {
    // f-string with Result expression
    let python = r#"
def safe_div(a: int, b: int) -> int:
    if b == 0:
        raise ValueError("Div by zero")
    return a // b

def show_calculation():
    a, b = 10, 2
    print(f"{a} / {b} = {safe_div(a, b)}")
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Result in format should be handled
    let has_debug_fmt = rust.contains("{:?}");
    let has_question = rust.contains("safe_div(a, b)?");

    assert!(
        has_debug_fmt || has_question,
        "BUG: f-string with Result expression must use {{:?}} or ?\n\
         Generated:\n{}",
        rust
    );
}

#[test]
fn test_compilation_no_e0277() {
    // Verify generated code compiles without E0277 Display error
    let python = r#"
from typing import Optional

def get_value(x: int) -> int:
    if x < 0:
        raise ValueError("Negative")
    return x

def find_item(items: list, target: int) -> Optional[int]:
    return 0 if len(items) > 0 else None

def main():
    result = get_value(5)
    print(f"Result: {result}")

    items = [1, 2, 3]
    print(f"Items: {items}")

    index = find_item(items, 2)
    print(f"Index: {index}")
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Write to temp file
    let mut file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(rust.as_bytes()).expect("Failed to write");

    // Try to compile with rustc
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--crate-name=test_format")
        .arg("--deny=warnings")
        .arg(file.path())
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("rustc output:\n{}", stderr);

    // Should NOT have E0277 Display trait error
    assert!(
        !stderr.contains("E0277") || !stderr.contains("doesn't implement `std::fmt::Display`"),
        "BUG: Generated code has E0277 Display trait error\n\
         This means Result/Option/Vec in format! are missing {{:?}} or unwrap\n\
         rustc stderr:\n{}",
        stderr
    );
}

#[test]
fn test_nested_format_expressions() {
    // Nested expressions with Result types
    let python = r#"
def compute(x: int) -> int:
    if x < 0:
        raise ValueError()
    return x * 2

def show_nested():
    x = 5
    print(f"Double of {x} is {compute(x)}")
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Result should be handled in format
    let has_debug_fmt = rust.contains("{:?}");
    let has_question = rust.contains("compute(x)?");

    assert!(
        has_debug_fmt || has_question,
        "BUG: Nested Result expression in format! must use {{:?}} or ?\n\
         Generated:\n{}",
        rust
    );
}
