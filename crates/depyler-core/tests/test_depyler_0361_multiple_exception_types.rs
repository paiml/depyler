use depyler_core::DepylerPipeline;

/// SLOW: Requires rustc compilation validation
#[test]
#[ignore = "slow: requires rustc compilation"]
fn test_multiple_exception_handlers_compiles() {
    // DEPYLER-0361: Multiple exception handlers should ALL be generated
    let python = r#"
def parse_value(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        print("Value error")
        return -1
    except TypeError:
        print("Type error")
        return -2
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

    // Should contain both handlers
    assert!(
        rust_code.contains("Value error"),
        "Should contain ValueError handler"
    );
    assert!(
        rust_code.contains("Type error"),
        "Should contain TypeError handler"
    );

    // Write to temp file and compile
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_depyler_0361.rs");
    std::fs::write(&test_file, &rust_code).expect("Should write file");

    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_depyler_0361.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "❌ DEPYLER-0361: Generated code should compile!\n\nGenerated code:\n{}\n\nCompilation errors:\n{}",
            rust_code, stderr
        );
    }

    println!("✅ Generated code compiles successfully");
}
