// GH-70: Nested function return type inference
//
// BUG: Nested functions without type annotations generate closures with
// incorrect return type `()` instead of inferring from body.
//
// ROOT CAUSE: Type::Unknown defaults to `()` in hir_type_to_tokens,
// causing explicit `-> ()` in closure definition which conflicts with
// actual return value from body.
//
// FIX: Omit `-> ReturnType` annotation for closures with Type::Unknown,
// allowing Rust's type inference to determine correct return type.

use depyler_core::DepylerPipeline;
use std::process::Command;

/// Helper: Compile generated Rust code and return success/failure
fn compile_rust_code(rust_code: &str, test_name: &str) -> Result<(), String> {
    // Write to temporary file
    let temp_file = format!("/tmp/gh70_{}.rs", test_name);
    std::fs::write(&temp_file, rust_code).map_err(|e| format!("Write failed: {}", e))?;

    // Try to compile
    let output = Command::new("rustc")
        .args(["--crate-type", "lib", &temp_file])
        .output()
        .map_err(|e| format!("Rustc execution failed: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Compilation failed:\n{}", stderr))
    }
}

#[test]
fn test_gh70_nested_function_string_return() {
    // Minimal case: nested function returns string slice
    let python_code = r#"
def outer():
    def inner(entry):
        timestamp, level, message = entry
        return timestamp[11:13]  # Returns str

    return inner
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // ASSERTION 1: Generated code should compile without errors
    compile_rust_code(&rust_code, "string_return")
        .expect("Generated Rust code MUST compile without errors");

    // ASSERTION 2: Closure should not have explicit `-> ()` return type
    // when returning non-unit value
    assert!(
        !rust_code.contains("| -> () {") || !rust_code.contains("return"),
        "Closure with return statement should not have explicit `-> ()` type.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_gh70_nested_function_int_return() {
    // Nested function returns int (from list index)
    let python_code = r#"
def get_first_element():
    def extract_key(item):
        return item[0]  # Returns int

    return extract_key
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // Must compile
    compile_rust_code(&rust_code, "int_return")
        .expect("Generated Rust code MUST compile without errors");
}

#[test]
fn test_gh70_nested_function_with_annotation() {
    // With explicit annotation, should use that type
    let python_code = r#"
def outer():
    def inner(x: int) -> int:
        return x * 2

    return inner
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // Should have explicit return type when annotated
    assert!(
        rust_code.contains("-> i64") || rust_code.contains("-> i32"),
        "Annotated function should have explicit return type.\nGenerated:\n{}",
        rust_code
    );

    compile_rust_code(&rust_code, "with_annotation").expect("Generated Rust code MUST compile");
}

#[test]
fn test_gh70_itertools_groupby_pattern() {
    // Real-world case from issue: groupby key extractor
    let python_code = r#"
def group_by_hour(entries):
    def extract_hour(entry):
        timestamp, level, message = entry
        return timestamp[11:13]

    # Would use with itertools.groupby(entries, key=extract_hour)
    return extract_hour
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    compile_rust_code(&rust_code, "groupby_pattern")
        .expect("groupby key extractor pattern MUST compile");
}

#[test]
fn test_gh70_void_nested_function() {
    // Nested function with no return (truly void)
    let python_code = r#"
def outer():
    def printer(msg):
        print(msg)  # No return, truly void

    return printer
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // Void function can have `-> ()` or omit it
    compile_rust_code(&rust_code, "void_function").expect("Void nested function MUST compile");
}
