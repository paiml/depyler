use depyler_core::DepylerPipeline;

#[test]
fn test_floor_division_with_zero_division_handler_compiles() {
    let python = r#"
def calculate(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        print("Division by zero")
        return 0
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

    // Write to temp file and compile
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_depyler_0360.rs");
    std::fs::write(&test_file, &rust_code).expect("Should write file");

    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_depyler_0360.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "❌ DEPYLER-0360: Generated code should compile!\n\nGenerated code:\n{}\n\nCompilation errors:\n{}",
            rust_code, stderr
        );
    }

    println!("✅ Generated code compiles successfully");
}
