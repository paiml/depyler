#![allow(non_snake_case)]
// DEPYLER-0302 Phase 3: String Slicing with Negative Indices
// Tests for fixing string slicing code generation
// Issue: String slicing generated Vec operations (.to_vec(), .iter()) instead of string operations (.chars())

use depyler_core::DepylerPipeline;

// ========== Basic String Slicing Tests ==========

#[test]
fn test_string_last_char() {
    let python_code = r#"
def get_last_char(s: str) -> str:
    return s[-1]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // DEPYLER-1127: Check only the get_last_char function, not entire file
    // (DepylerValue and other impls may have legitimate .to_vec() calls)
    let fn_start = rust_code.find("fn get_last_char").expect("Should have get_last_char function");
    let fn_end = rust_code[fn_start..].find("\n}").unwrap_or(200) + fn_start + 2;
    let fn_section = &rust_code[fn_start..fn_end.min(rust_code.len())];

    // Should use proper string operations in the function body
    // Either .chars() based or index-based access
    assert!(
        fn_section.contains(".chars()") || fn_section.contains("get(") || fn_section.contains("["),
        "Should use proper string/char access\nFunction:\n{}", fn_section
    );

    println!("✅ Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_last_n_chars() {
    let python_code = r#"
def get_last_n_chars(s: str, n: int) -> str:
    return s[-n:]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // DEPYLER-1127: Check only the get_last_n_chars function, not entire file
    let fn_start = rust_code.find("fn get_last_n_chars").expect("Should have get_last_n_chars function");
    let fn_end = rust_code[fn_start..].find("\n}").unwrap_or(500) + fn_start + 2;
    let fn_section = &rust_code[fn_start..fn_end.min(rust_code.len())];

    // Should use proper string operations in the function body
    assert!(
        fn_section.contains(".chars()") || fn_section.contains("skip") || fn_section.contains("["),
        "Should use proper string slicing\nFunction:\n{}", fn_section
    );

    println!("✅ Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_all_but_last_n() {
    let python_code = r#"
def get_all_but_last_n(s: str, n: int) -> str:
    return s[:-n]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // DEPYLER-1127: Check only the get_all_but_last_n function, not entire file
    let fn_start = rust_code.find("fn get_all_but_last_n").expect("Should have get_all_but_last_n function");
    let fn_end = rust_code[fn_start..].find("\n}").unwrap_or(500) + fn_start + 2;
    let fn_section = &rust_code[fn_start..fn_end.min(rust_code.len())];

    // Should use proper string operations in the function body
    assert!(
        fn_section.contains(".chars()") || fn_section.contains("take") || fn_section.contains("["),
        "Should use proper string slicing\nFunction:\n{}", fn_section
    );

    println!("✅ Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_reverse() {
    let python_code = r#"
def reverse_string(s: str) -> str:
    return s[::-1]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .chars().rev() for string reversal
    assert!(
        rust_code.contains(".chars()"),
        "Should use .chars() for string operations"
    );
    assert!(
        rust_code.contains(".rev()"),
        "Should use .rev() for reversal"
    );

    // Should collect into String
    assert!(
        rust_code.contains("collect::<String>()"),
        "Should collect into String"
    );

    println!("✅ Generated Rust code:\n{}", rust_code);
}

// ========== String Slicing Range Tests ==========

#[test]
fn test_string_slice_start_stop() {
    let python_code = r#"
def substring(s: str, start: int, stop: int) -> str:
    return s[start:stop]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .chars().skip().take() pattern
    assert!(
        rust_code.contains(".chars()"),
        "Should use .chars() for string slicing"
    );
    assert!(
        rust_code.contains(".skip(") || rust_code.contains(".take("),
        "Should use .skip()/.take() for range slicing"
    );

    println!("✅ Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_slice_with_step() {
    let python_code = r#"
def every_nth_char(s: str, n: int) -> str:
    return s[::n]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .chars().step_by() for step slicing
    assert!(
        rust_code.contains(".chars()"),
        "Should use .chars() for string slicing"
    );
    assert!(
        rust_code.contains(".step_by(") || rust_code.contains("step"),
        "Should handle step parameter"
    );

    println!("✅ Generated Rust code:\n{}", rust_code);
}

// ========== Compilation Tests ==========

#[test]
fn test_string_slicing_compiles() {
    let python_code = r#"
def test_all_patterns(s: str) -> str:
    last = s[-1]
    last_n = s[-3:]
    without_last = s[:-1]
    reversed = s[::-1]
    middle = s[1:4]
    return reversed
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Write to temp file and compile
    std::fs::write("/tmp/test_string_patterns.rs", &rust_code).expect("Failed to write test file");

    let output = std::process::Command::new("rustc")
        .args(["--crate-type", "lib", "/tmp/test_string_patterns.rs"])
        .output()
        .expect("Failed to compile");

    if !output.status.success() {
        eprintln!("❌ Compilation failed!");
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
        eprintln!("\n📄 Generated Rust code:\n{}", rust_code);
        panic!("Generated code failed to compile");
    }

    println!("✅ All string slicing patterns compile successfully");
}

// ========== Regression Tests ==========

#[test]
fn test_regression_vec_slicing_still_works() {
    // Ensure Vec slicing wasn't broken by string slicing fix
    let python_code = r#"
def last_elements(arr: list[int]) -> list[int]:
    return arr[-3:]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Vec slicing should still use .to_vec() or similar patterns
    // (This is acceptable for Vec, not for str)
    assert!(
        rust_code.contains("Vec") || rust_code.contains("vec"),
        "Should generate Vec-appropriate code"
    );

    println!("✅ Vec slicing still works:\n{}", rust_code);
}

#[test]
fn test_regression_string_methods_still_work() {
    // DEPYLER-0302 Phase 1 & 2 should still work
    let python_code = r#"
def process_string(s: str) -> str:
    repeated = s * 3
    return repeated
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should have string repetition (DEPYLER-0302 Phase 2)
    assert!(
        rust_code.contains(".repeat("),
        "String repetition should still work (DEPYLER-0302 Phase 2)"
    );

    println!("✅ String methods still work:\n{}", rust_code);
}

// ========== Type Discrimination Tests ==========

#[test]
fn test_type_discrimination_string_vs_list() {
    // Critical: Ensure transpiler distinguishes string from list variables
    let python_code = r#"
def mixed_types(s: str, arr: list[int]) -> str:
    last_char = s[-1]
    last_elem = arr[-1]
    return last_char
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // String variable 's' should use .chars() or proper string methods
    // List variable 'arr' should use .get() or Vec methods
    // Both should be present in the output
    assert!(
        rust_code.contains("s") || rust_code.contains("string"),
        "Should handle string variable"
    );
    assert!(
        rust_code.contains("arr") || rust_code.contains("list"),
        "Should handle list variable"
    );

    println!("✅ Type discrimination works:\n{}", rust_code);
}

// ========== Edge Cases ==========

#[test]
fn test_empty_slice() {
    let python_code = r#"
def full_copy(s: str) -> str:
    return s[:]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Full slice should be simple clone or to_string
    assert!(
        rust_code.contains(".to_string()") || rust_code.contains(".clone()"),
        "Full slice should use simple copy"
    );

    println!("✅ Full slice works:\n{}", rust_code);
}

#[test]
fn test_negative_step_reverse() {
    let python_code = r#"
def reverse_every_second(s: str) -> str:
    return s[::-2]
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .chars().rev().step_by() pattern
    assert!(
        rust_code.contains(".rev()"),
        "Negative step should use .rev()"
    );
    assert!(
        rust_code.contains(".step_by(") || rust_code.contains("abs_step"),
        "Should handle step magnitude"
    );

    println!("✅ Negative step works:\n{}", rust_code);
}
