/// DEPYLER-0290 & DEPYLER-0292: Collection Operation Tests
///
/// These tests verify that the transpiler correctly handles:
/// 1. Vector concatenation (list1 + list2)
/// 2. Iterator conversion for extend()
///
/// Expected behavior: All tests should PASS after transpiler fixes
use depyler_core::DepylerPipeline;

#[test]
fn test_vector_concatenation_compiles() {
    let python_code = r#"
def concat_lists(list1: list[int], list2: list[int]) -> list[int]:
    """Concatenate two lists."""
    return list1 + list2
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // DEPYLER-0290: The generated code should handle Vec concatenation
    // Should NOT generate: list1 + list2 (invalid for &Vec)
    // Should generate: iterator chain or extend pattern

    println!("Generated code:\n{}", generated_code);

    // Verify the generated code compiles
    let syntax_check = syn::parse_file(&generated_code);
    assert!(
        syntax_check.is_ok(),
        "Generated code should be valid Rust syntax: {:?}",
        syntax_check.err()
    );
}

#[test]
fn test_vector_concatenation_executes() {
    let python_code = r#"
def concat_lists(list1: list[int], list2: list[int]) -> list[int]:
    """Concatenate two lists."""
    return list1 + list2
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // Write to temp file and compile with rustc
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_vector_concat.rs");

    std::fs::write(&test_file, &generated_code).expect("Should write temp file");

    // Try to compile it
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_vector_concat.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "DEPYLER-0290: Generated code should compile!\n\nGenerated code:\n{}\n\nRustc errors:\n{}",
            generated_code, stderr
        );
    }

    // Cleanup
    let _ = std::fs::remove_file(&test_file);
    let _ = std::fs::remove_file(temp_dir.join("test_vector_concat.rlib"));
}

#[test]
fn test_list_extend_compiles() {
    let python_code = r#"
def extend_list(list1: list[int], list2: list[int]) -> list[int]:
    """Extend list1 with list2."""
    result = list1.copy()
    result.extend(list2)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // DEPYLER-0292: The generated code should handle extend() properly
    // Should NOT generate: result.extend(list2) where list2 is &Vec
    // Should generate: result.extend(list2.iter().cloned())

    println!("Generated code:\n{}", generated_code);

    // Verify the generated code compiles
    let syntax_check = syn::parse_file(&generated_code);
    assert!(
        syntax_check.is_ok(),
        "Generated code should be valid Rust syntax: {:?}",
        syntax_check.err()
    );
}

#[test]
fn test_list_extend_executes() {
    let python_code = r#"
def extend_list(list1: list[int], list2: list[int]) -> list[int]:
    """Extend list1 with list2."""
    result = list1.copy()
    result.extend(list2)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // Write to temp file and compile with rustc
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_list_extend.rs");

    std::fs::write(&test_file, &generated_code).expect("Should write temp file");

    // Try to compile it
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_list_extend.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "DEPYLER-0292: Generated code should compile!\n\nGenerated code:\n{}\n\nRustc errors:\n{}",
            generated_code, stderr
        );
    }

    // Cleanup
    let _ = std::fs::remove_file(&test_file);
    let _ = std::fs::remove_file(temp_dir.join("test_list_extend.rlib"));
}

#[test]
fn test_combined_vector_operations() {
    let python_code = r#"
def combine_and_extend(list1: list[int], list2: list[int], list3: list[int]) -> list[int]:
    """Combine lists using both concatenation and extend."""
    # First concatenate list1 and list2
    combined = list1 + list2
    # Then extend with list3
    result = combined.copy()
    result.extend(list3)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    println!("Generated code:\n{}", generated_code);

    // Verify the generated code compiles
    let syntax_check = syn::parse_file(&generated_code);
    assert!(
        syntax_check.is_ok(),
        "Generated code should be valid Rust syntax: {:?}",
        syntax_check.err()
    );

    // Write to temp file and compile with rustc
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_combined_ops.rs");

    std::fs::write(&test_file, &generated_code).expect("Should write temp file");

    // Try to compile it
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_combined_ops.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "Combined operations should compile!\n\nGenerated code:\n{}\n\nRustc errors:\n{}",
            generated_code, stderr
        );
    }

    // Cleanup
    let _ = std::fs::remove_file(&test_file);
    let _ = std::fs::remove_file(temp_dir.join("test_combined_ops.rlib"));
}
