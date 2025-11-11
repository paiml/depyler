// DEPYLER-0282: Test that String parameters don't use Cow<'static, str>
// This test demonstrates the bug and will initially FAIL

use depyler_core::DepylerPipeline;

#[test]
fn test_string_param_no_static_lifetime() {
    // GIVEN: Python function with String parameters
    let python_code = r#"
def concatenate(a: str, b: str) -> str:
    """Test function with String parameters."""
    return a + b
"#;

    // WHEN: We transpile the code
    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // THEN: Generated code should NOT contain Cow<'static, str> for parameters
    // The bug: current code generates `a: Cow<'static, str>, b: &str`
    // We want to assert this is NOT present

    println!("Generated code:\n{}", generated_code);

    // Check if the function signature uses Cow<'static, str>
    let has_static_cow_param = generated_code.contains("Cow<'static, str>");

    assert!(
        !has_static_cow_param,
        "❌ BUG CONFIRMED: Parameters use Cow<'static, str>\n\
         Expected: Generic lifetime 'a or just &str\n\
         Found: Cow<'static, str>\n\n\
         Generated code:\n{}",
        generated_code
    );
}

#[test]
fn test_string_param_compiles_with_local_strings() {
    // GIVEN: Python function with String parameters
    let python_code = r#"
def concatenate(a: str, b: str) -> str:
    """Test function with String parameters."""
    return a + b
"#;

    // WHEN: We transpile with test generation (disabled for now due to DEPYLER-0281 workaround)
    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // THEN: Generated code should compile
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_cow_compile.rs");

    // Add necessary test code that uses local Strings
    let test_code = format!(
        r#"{}

#[cfg(test)]
mod test_local_strings {{
    use super::*;

    #[test]
    fn test_with_local_strings() {{
        // DEPYLER-0357: Parameters use &str (not Cow), so we pass borrowed references
        // This works because &str accepts both &String and &'static str
        let a = String::from("hello");
        let b = String::from("world");
        let _result = concatenate(&a, &b);
    }}
}}
"#,
        generated_code
    );

    std::fs::write(&test_file, &test_code).expect("Failed to write test file");

    // Try to compile - this will fail with the current bug
    let output = std::process::Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--test")
        .arg(&test_file)
        .arg("--out-dir")
        .arg(&temp_dir)
        .arg("--edition")
        .arg("2021")
        .output()
        .expect("Failed to run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Check if it's the lifetime error we expect
        if stderr.contains("Cow<'static, str>")
            || stderr.contains("cannot infer")
            || stderr.contains("lifetime")
        {
            panic!(
                "❌ BUG CONFIRMED: Generated code requires 'static lifetime for parameters!\n\
                 This prevents using local Strings in tests.\n\n\
                 Compilation error:\n{}\n\n\
                 Generated code:\n{}",
                stderr, generated_code
            );
        } else {
            // Different compilation error - still fail but with different message
            panic!(
                "Compilation failed (may be unrelated to DEPYLER-0282):\n{}\n\n\
                 Generated code:\n{}",
                stderr, generated_code
            );
        }
    }

    println!("✅ Generated code compiles successfully with local Strings");
}
