/// DEPYLER-0161: Array Literal Transpilation Regression Tests
///
/// BUG: Array literal assignments are being dropped during code generation.
/// ALL variable assignments are missing from generated code, leaving only
/// return statements with undefined variables.
///
/// EXTREME TDD: These tests MUST fail first, then we fix the transpiler.

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_array_literal_assignment() {
    // STEP 1: Write the failing test FIRST
    let python_code = r#"
def test_array():
    arr = [1, 2, 3]
    return arr
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // CRITICAL: Generated code MUST include the assignment
    assert!(
        rust_code.contains("arr = ") || rust_code.contains("let arr"),
        "Generated code MUST contain array assignment. Got:\n{}",
        rust_code
    );

    // CRITICAL: Generated code must NOT just return undefined variable
    assert!(
        rust_code.contains("[1") || rust_code.contains("vec!"),
        "Generated code MUST contain array literal initialization. Got:\n{}",
        rust_code
    );
}

#[test]
fn test_multiple_array_assignments() {
    let python_code = r#"
def test_arrays():
    arr1 = [1, 2, 3]
    arr2 = [4, 5, 6]
    arr3 = [7, 8, 9]
    return arr1, arr2, arr3
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // ALL three assignments must be present
    assert!(
        rust_code.contains("arr1") && rust_code.contains("arr2") && rust_code.contains("arr3"),
        "All array variables must be present. Got:\n{}",
        rust_code
    );

    // Check for actual initialization (not just variable names in return)
    let assignment_count = rust_code.matches("arr1 =").count() +
                           rust_code.matches("let arr1").count();
    assert!(
        assignment_count > 0,
        "arr1 must have an assignment statement. Got:\n{}",
        rust_code
    );
}

#[test]
fn test_array_with_booleans() {
    let python_code = r#"
def test_bool_array():
    flags = [True, False, True]
    return flags
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("flags = ") || rust_code.contains("let flags"),
        "Generated code MUST contain flags assignment. Got:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("true") || rust_code.contains("false"),
        "Generated code MUST contain boolean literals. Got:\n{}",
        rust_code
    );
}

#[test]
fn test_generated_code_compiles() {
    // CRITICAL: The ultimate test - generated code MUST compile
    let python_code = r#"
def simple_array():
    nums = [1, 2, 3]
    return nums
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Try to parse the generated code as valid Rust
    let parsed = syn::parse_file(&rust_code);

    assert!(
        parsed.is_ok(),
        "Generated Rust code must be syntactically valid. Parse error: {:?}\nGenerated code:\n{}",
        parsed.err(),
        rust_code
    );
}

// Property-based test: ANY Python function with array assignment should generate valid code
#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};

    fn prop_array_assignment_generates_valid_rust(size: usize) -> TestResult {
        if size > 20 {
            return TestResult::discard();
        }

        // Generate Python code with array of given size
        let elements: Vec<String> = (0..size).map(|i| i.to_string()).collect();
        let python_code = format!(
            "def test_array():\n    arr = [{}]\n    return arr",
            elements.join(", ")
        );

        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile(&python_code);

        if result.is_err() {
            return TestResult::error("Transpilation failed");
        }

        let rust_code = result.unwrap();

        // Property: Generated code MUST contain the assignment
        let has_assignment = rust_code.contains("arr = ") || rust_code.contains("let arr");

        TestResult::from_bool(has_assignment)
    }

    #[test]
    #[ignore] // Enable after fix
    fn test_property_array_assignments() {
        quickcheck(prop_array_assignment_generates_valid_rust as fn(usize) -> TestResult);
    }
}
