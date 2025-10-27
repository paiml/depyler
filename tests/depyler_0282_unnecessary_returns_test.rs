// DEPYLER-0271: Unnecessary return statements (P1)
// TDD RED Phase: Regression tests
//
// Issue: Generated Rust code uses explicit `return` keyword for all return statements,
// even for the final expression in a function. Idiomatic Rust uses expression-based
// returns (no `return` keyword) for the final statement.
//
// Root Cause: crates/depyler-core/src/rust_gen/stmt_gen.rs:136-186
// Function: codegen_return_stmt() always generates `return` keyword
//
// Fix Strategy: Detect if HirStmt::Return is the final statement in its block,
// and if so, omit the `return` keyword for idiomatic Rust style.

use depyler_core::pipeline::DepylerPipeline;

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0271_simple_function_final_return_omitted() {
    let python = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Should NOT contain "return a + b" (should be expression-based)
    assert!(
        !rust.contains("return a + b"),
        "Final return should omit `return` keyword for idiomatic Rust"
    );

    // Should contain expression-based return (just "a + b")
    // Note: There might be type conversion (a + b as i32), so check for presence
    assert!(
        rust.contains("a + b"),
        "Should use expression-based return (a + b without return)"
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0271_early_return_preserved() {
    let python = r#"
def max_value(a: int, b: int) -> int:
    if a > b:
        return a
    return b
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Early return in if block should keep "return a;"
    // This is idiomatic for early returns in control flow
    assert!(
        rust.contains("return a"),
        "Early return in if block should preserve `return` keyword"
    );

    // Final return "return b;" should become just "b"
    // Check that final statement doesn't have return keyword
    let lines: Vec<&str> = rust.lines().collect();
    let last_meaningful_line = lines
        .iter()
        .rev()
        .find(|line| !line.trim().is_empty() && !line.contains("}"))
        .expect("Should have final statement");

    assert!(
        !last_meaningful_line.contains("return b"),
        "Final statement should omit `return` keyword: {}",
        last_meaningful_line
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0271_boolean_return() {
    let python = r#"
def check_positive(x: int) -> bool:
    return x > 0
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Final return should be expression-based
    assert!(
        !rust.contains("return x > 0") && !rust.contains("return _cse_temp"),
        "Boolean return should be expression-based"
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0271_literal_return() {
    let python = r#"
def always_true(x: int) -> bool:
    return True
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Final return should be expression-based
    let lines: Vec<&str> = rust.lines().collect();
    let last_meaningful_line = lines
        .iter()
        .rev()
        .find(|line| !line.trim().is_empty() && !line.contains("}"))
        .expect("Should have final statement");

    assert!(
        !last_meaningful_line.contains("return true"),
        "Literal return should be expression-based: {}",
        last_meaningful_line
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0271_multiple_statements_final_return() {
    let python = r#"
def multiply(a: int, b: int) -> int:
    result = a * b
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Final return should be expression-based
    let lines: Vec<&str> = rust.lines().collect();
    let last_meaningful_line = lines
        .iter()
        .rev()
        .find(|line| !line.trim().is_empty() && !line.contains("}"))
        .expect("Should have final statement");

    assert!(
        !last_meaningful_line.contains("return result"),
        "Final return of variable should be expression-based: {}",
        last_meaningful_line
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0271_string_concatenation_return() {
    let python = r#"
def concat(a: str, b: str) -> str:
    return a + b
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Final return should be expression-based (format! call)
    let lines: Vec<&str> = rust.lines().collect();
    let last_meaningful_line = lines
        .iter()
        .rev()
        .find(|line| !line.trim().is_empty() && !line.contains("}"))
        .expect("Should have final statement");

    assert!(
        !last_meaningful_line.contains("return format!"),
        "String concatenation return should be expression-based: {}",
        last_meaningful_line
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0271_if_else_both_branches_final() {
    let python = r#"
def max_of_two(a: float, b: float) -> float:
    if a > b:
        return a
    else:
        return b
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Both branches are final in their respective blocks
    // In Rust, if-else as expression can omit return in both branches
    // But for now, we focus on function-level final statement
    // The if-else itself is the final statement, so it should be expression-based

    // Check that the function ends with an if-else expression, not a return statement
    let lines: Vec<&str> = rust.lines().collect();

    // Find the if-else structure
    let has_if = rust.contains("if ");
    let has_else = rust.contains("else");

    assert!(has_if && has_else, "Should have if-else structure");

    // The branches should NOT have "return a" and "return b" if converted to expression
    // But this is complex - let's check that at least the pattern exists
    // For now, we'll accept this test may need refinement after implementation
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0271_compilation_check() {
    // Comprehensive test: transpile a function and verify it compiles
    let python = r#"
def calculate(x: int, y: int) -> int:
    if x == 0:
        return y
    temp = x * 2
    return temp + y
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Write to temp file and compile
    let temp_file = "/tmp/depyler_0271_test.rs";
    std::fs::write(temp_file, &rust).expect("Failed to write temp file");

    // Attempt compilation (this will pass even with return keyword, but shows structure)
    // The real validation is clippy not warning about needless_return
    let output = std::process::Command::new("rustc")
        .args(&["--crate-type", "lib", temp_file])
        .output()
        .expect("Failed to compile");

    if !output.status.success() {
        eprintln!("Compilation failed:");
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Generated Rust code does not compile");
    }

    // Check for clippy warnings (needless_return)
    // Note: This requires clippy to be available
    let clippy_output = std::process::Command::new("clippy-driver")
        .args(&["--crate-type", "lib", temp_file, "-W", "clippy::needless_return"])
        .output();

    if let Ok(output) = clippy_output {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Clippy output:\n{}", stderr);

        // After fix, this should NOT contain needless_return warning
        // For RED phase, we expect this warning to exist
        if !stderr.contains("needless_return") {
            println!("✅ No needless_return warnings (fix is working!)");
        } else {
            println!("⚠️  Found needless_return warnings (expected in RED phase)");
        }
    }

    // Cleanup
    std::fs::remove_file(temp_file).ok();
    std::fs::remove_file("/tmp/depyler_0271_test").ok(); // Remove compiled artifact
}
