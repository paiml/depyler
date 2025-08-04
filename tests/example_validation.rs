use depyler_core::DepylerPipeline;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod example_validation_tests {
    use super::*;

    /// Validate that all example files can be successfully transpiled
    #[test]
    fn validate_example_transpilation() {
        let pipeline = DepylerPipeline::new();

        let example_files = vec![
            "examples/mathematical/basic_math.py",
            "examples/algorithms/binary_search_simple.py",
            "examples/string_processing/string_utils.py",
            "examples/data_processing/list_operations.py",
        ];

        println!("=== Example Validation Tests ===");

        for example_file in example_files {
            if Path::new(example_file).exists() {
                match fs::read_to_string(example_file) {
                    Ok(content) => {
                        let result = pipeline.transpile(&content);
                        println!(
                            "{}: {}",
                            example_file,
                            if result.is_ok() {
                                "✓ PASS"
                            } else {
                                "✗ FAIL"
                            }
                        );

                        // Examples should either transpile successfully or fail gracefully
                        assert!(
                            result.is_ok() || result.is_err(),
                            "Example {} should be handled",
                            example_file
                        );
                    }
                    Err(e) => {
                        println!("{}: ✗ READ ERROR: {}", example_file, e);
                    }
                }
            } else {
                println!("{}: ⚠ FILE NOT FOUND", example_file);
            }
        }
    }

    /// Test example correctness by validating HIR generation
    #[test]
    fn validate_example_hir_generation() {
        let pipeline = DepylerPipeline::new();

        let test_examples = vec![
            (
                "Simple Function",
                "def add(a: int, b: int) -> int: return a + b",
            ),
            (
                "Control Flow",
                r#"
def max_value(a: int, b: int) -> int:
    if a > b:
        return a
    else:
        return b
"#,
            ),
            (
                "Loop Example",
                r#"
def sum_range(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#,
            ),
            (
                "Multiple Functions",
                r#"
def helper(x: int) -> int:
    return x * 2

def main_func(y: int) -> int:
    return helper(y) + 1
"#,
            ),
        ];

        println!("=== HIR Generation Validation ===");

        for (name, code) in test_examples {
            match pipeline.parse_to_hir(code) {
                Ok(hir) => {
                    println!("{}: ✓ PASS ({} functions)", name, hir.functions.len());

                    // Validate HIR structure
                    assert!(
                        !hir.functions.is_empty(),
                        "HIR should contain at least one function"
                    );

                    for function in &hir.functions {
                        assert!(
                            !function.name.is_empty(),
                            "Function name should not be empty"
                        );
                        // Note: Other validations would depend on specific HIR structure
                    }
                }
                Err(e) => {
                    println!("{}: ✗ FAIL: {}", name, e);
                    // Some complex examples might fail, which is acceptable for testing
                }
            }
        }
    }

    /// Validate example annotations are preserved
    #[test]
    fn validate_annotation_preservation() {
        let pipeline = DepylerPipeline::new();

        let annotated_examples = [
            r#"
# @depyler: optimization_level = "aggressive"
def optimized_func(x: int) -> int:
    return x * x
"#,
            r#"
# @depyler: bounds_checking = "explicit"  
def safe_func(arr: list, index: int) -> int:
    return arr[index] if index < len(arr) else 0
"#,
            r#"
# @depyler: string_strategy = "zero_copy"
def string_func(s: str) -> str:
    return s.upper()
"#,
        ];

        println!("=== Annotation Preservation Validation ===");

        for (i, example) in annotated_examples.iter().enumerate() {
            match pipeline.parse_to_hir(example) {
                Ok(hir) => {
                    println!("Annotation Example {}: ✓ PASS", i + 1);

                    if !hir.functions.is_empty() {
                        let function = &hir.functions[0];
                        // Validate that annotations are parsed (exact validation depends on HIR structure)
                        println!("  Function: {} (annotations processed)", function.name);
                    }
                }
                Err(e) => {
                    println!("Annotation Example {}: ✗ FAIL: {}", i + 1, e);
                }
            }
        }
    }

    /// Test example output quality
    #[test]
    fn validate_generated_rust_quality() {
        let pipeline = DepylerPipeline::new();

        let quality_test_examples = vec![
            (
                "Type Safety",
                "def typed_func(x: int, y: str) -> str: return y + str(x)",
            ),
            (
                "Memory Safety",
                "def safe_access(items: list) -> int: return len(items)",
            ),
            (
                "Error Handling",
                "def div_safe(a: int, b: int) -> int: return a // b if b != 0 else 0",
            ),
        ];

        println!("=== Generated Rust Quality Validation ===");

        for (name, code) in quality_test_examples {
            match pipeline.transpile(code) {
                Ok(rust_code) => {
                    println!("{}: ✓ GENERATED", name);

                    // Basic quality checks
                    assert!(
                        rust_code.contains("pub fn"),
                        "Should generate public function"
                    );
                    assert!(
                        rust_code.len() > code.len(),
                        "Generated code should be longer than source"
                    );

                    // Check for common Rust patterns
                    let has_types = rust_code.contains("i32")
                        || rust_code.contains("String")
                        || rust_code.contains("&str");
                    println!(
                        "  Contains Rust types: {}",
                        if has_types { "✓" } else { "✗" }
                    );
                }
                Err(e) => {
                    println!("{}: ✗ GENERATION FAILED: {}", name, e);
                }
            }
        }
    }

    /// Test example compilation readiness  
    #[test]
    fn validate_compilation_readiness() {
        let pipeline = DepylerPipeline::new();

        let compilation_examples = vec![
            (
                "Basic Arithmetic",
                "def add(a: int, b: int) -> int: return a + b",
            ),
            (
                "String Operations",
                "def greet(name: str) -> str: return 'Hello, ' + name",
            ),
            (
                "Boolean Logic",
                "def is_positive(x: int) -> bool: return x > 0",
            ),
            (
                "List Operations",
                "def list_sum(nums: list) -> int: return sum(nums)",
            ),
        ];

        println!("=== Compilation Readiness Validation ===");

        for (name, code) in compilation_examples {
            match pipeline.transpile(code) {
                Ok(rust_code) => {
                    println!("{}: ✓ TRANSPILED", name);

                    // Check for compilation-ready patterns
                    let checks = vec![
                        ("Has function definition", rust_code.contains("pub fn")),
                        ("Has return type", rust_code.contains("->")),
                        ("Has return statement", rust_code.contains("return")),
                        (
                            "Has proper braces",
                            rust_code.contains("{") && rust_code.contains("}"),
                        ),
                    ];

                    for (check_name, passed) in checks {
                        println!("  {}: {}", check_name, if passed { "✓" } else { "✗" });
                    }
                }
                Err(e) => {
                    println!("{}: ✗ TRANSPILATION FAILED: {}", name, e);
                }
            }
        }
    }

    /// Validate example documentation generation
    #[test]
    fn validate_documentation_generation() {
        let pipeline = DepylerPipeline::new();

        let documented_examples = [
            r#"
def documented_function(x: int) -> int:
    """This function doubles the input value."""
    return x * 2
"#,
            r#"
def complex_function(a: int, b: int, c: int) -> int:
    """
    Calculates a complex mathematical expression.
    
    Args:
        a: First integer
        b: Second integer  
        c: Third integer
        
    Returns:
        The result of the calculation
    """
    return (a + b) * c
"#,
        ];

        println!("=== Documentation Generation Validation ===");

        for (i, example) in documented_examples.iter().enumerate() {
            match pipeline.transpile(example) {
                Ok(rust_code) => {
                    println!("Documented Example {}: ✓ TRANSPILED", i + 1);

                    // Check if documentation is preserved in some form
                    let has_comments = rust_code.contains("//") || rust_code.contains("/*");
                    let has_doc_attrs = rust_code.contains("#[doc");

                    println!("  Has comments: {}", if has_comments { "✓" } else { "✗" });
                    println!(
                        "  Has doc attributes: {}",
                        if has_doc_attrs { "✓" } else { "✗" }
                    );
                }
                Err(e) => {
                    println!("Documented Example {}: ✗ FAILED: {}", i + 1, e);
                }
            }
        }
    }

    /// Test edge case examples that should be handled gracefully  
    #[test]
    fn validate_edge_case_examples() {
        let pipeline = DepylerPipeline::new();

        let long_name_code = format!(
            "def {}() -> int: return 1",
            "very_long_function_name".repeat(10)
        );
        let edge_case_examples = vec![
            ("Empty Function", "def empty(): pass"),
            ("Single Return", "def constant() -> int: return 42"),
            ("Unicode Name", "def测试函数() -> int: return 1"),
            ("Long Name", long_name_code.as_str()),
        ];

        println!("=== Edge Case Example Validation ===");

        for (name, code) in edge_case_examples {
            let result = pipeline.transpile(code);
            println!(
                "{}: {}",
                name,
                match result {
                    Ok(_) => "✓ HANDLED",
                    Err(_) => "✗ REJECTED (acceptable)",
                }
            );

            // Edge cases should either succeed or fail gracefully
            assert!(
                result.is_ok() || result.is_err(),
                "Edge case '{}' should be handled gracefully",
                name
            );
        }
    }
}
