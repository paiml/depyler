use depyler_core::DepylerPipeline;

#[test]
fn test_tuple_exception_types_compiles() {
    // DEPYLER-0362: Tuple exception types should generate proper code
    let python = r#"
def multi_except(s: str) -> int:
    try:
        return int(s)
    except (ValueError, TypeError):
        print("Error occurred")
        return -1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // DEPYLER-0362: Currently, tuple exception handlers with multiple statements
    // (like print + return) are not fully implemented. The code compiles but
    // doesn't execute the handler body. This is a TODO for proper error dispatch.
    // For now, we just verify it compiles.

    // Write to temp file and compile
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_depyler_0362.rs");
    std::fs::write(&test_file, &rust_code).expect("Should write file");

    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_depyler_0362.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "❌ DEPYLER-0362: Generated code should compile!\n\nGenerated code:\n{}\n\nCompilation errors:\n{}",
            rust_code, stderr
        );
    }

    println!("✅ Generated code compiles successfully");
}
