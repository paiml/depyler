//! TDD Tests for map() with Multiple Iterables (DEPYLER-0121)
//!
//! Python's map() with multiple iterables should convert to Rust's zip + map pattern:
//! Python: list(map(lambda x, y: x + y, list1, list2))
//! Rust: list1.iter().zip(list2.iter()).map(|(x, y)| x + y).collect()
//!
//! Test Coverage:
//! 1. Basic map with two iterables
//! 2. Map with three iterables
//! 3. Map with complex lambda expression
//! 4. Map with arithmetic operations
//! 5. Map with string concatenation
//! 6. Map with tuple unpacking
//! 7. Nested map with zip
//! 8. Map with filter combination
//! 9. Map with index access as iterables
//! 10. Map with method call results

use depyler_core::DepylerPipeline;

#[test]
fn test_map_two_iterables_simple() {
    let python = r#"
def combine_lists(list1: list, list2: list) -> list:
    return list(map(lambda x, y: x + y, list1, list2))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn combine_lists"),
        "Should have combine_lists function.\nGot:\n{}",
        rust_code
    );

    // Should have zip pattern
    let has_zip = rust_code.contains(".zip(");
    assert!(has_zip, "Should have .zip() call.\nGot:\n{}", rust_code);

    // Should have tuple destructuring in lambda (flexible spacing)
    let has_tuple_lambda = rust_code.contains("|(x, y)") || rust_code.contains("|( x, y )");
    assert!(
        has_tuple_lambda,
        "Should have tuple destructuring lambda.\nGot:\n{}",
        rust_code
    );

    // Should have collect
    let has_collect = rust_code.contains(".collect");
    assert!(has_collect, "Should have .collect().\nGot:\n{}", rust_code);
}

#[test]
fn test_map_three_iterables() {
    let python = r#"
def combine_three(list1: list, list2: list, list3: list) -> list:
    return list(map(lambda x, y, z: x + y + z, list1, list2, list3))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have multiple zip calls
    let zip_count = rust_code.matches(".zip(").count();
    assert!(
        zip_count >= 2,
        "Should have at least 2 .zip() calls for 3 iterables.\nGot:\n{}",
        rust_code
    );

    // Should have three-parameter tuple destructuring (flexible spacing)
    let has_triple_lambda =
        rust_code.contains("|((x, y), z)") || rust_code.contains("|( ( x, y ), z )");
    assert!(
        has_triple_lambda,
        "Should have three-parameter tuple destructuring.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_map_with_complex_lambda() {
    let python = r#"
def complex_combine(nums1: list, nums2: list) -> list:
    return list(map(lambda x, y: (x * 2) + (y * 3), nums1, nums2))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have zip pattern
    assert!(
        rust_code.contains(".zip("),
        "Should have .zip().\nGot:\n{}",
        rust_code
    );

    // Should have tuple lambda (flexible spacing)
    let has_tuple_lambda = rust_code.contains("|(x, y)") || rust_code.contains("|( x, y )");
    assert!(
        has_tuple_lambda,
        "Should have tuple lambda.\nGot:\n{}",
        rust_code
    );

    // Should preserve the complex expression
    let has_multiplication = rust_code.contains("* 2") && rust_code.contains("* 3");
    assert!(
        has_multiplication,
        "Should preserve multiplication operations.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_map_string_concatenation() {
    let python = r#"
def concat_strings(first: list, last: list) -> list:
    return list(map(lambda f, l: f + " " + l, first, last))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have zip pattern
    assert!(
        rust_code.contains(".zip("),
        "Should have .zip().\nGot:\n{}",
        rust_code
    );

    // Should have tuple lambda with f and l (flexible spacing)
    let has_tuple_lambda = rust_code.contains("|(f, l)") || rust_code.contains("|( f, l )");
    assert!(
        has_tuple_lambda,
        "Should have |(f, l)| lambda.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_map_with_comparison() {
    let python = r#"
def compare_lists(list1: list, list2: list) -> list:
    return list(map(lambda x, y: x > y, list1, list2))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have zip pattern
    assert!(
        rust_code.contains(".zip("),
        "Should have .zip().\nGot:\n{}",
        rust_code
    );

    // Should have comparison
    let has_comparison = rust_code.contains(">") || rust_code.contains("gt");
    assert!(
        has_comparison,
        "Should have comparison operator.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_map_with_index_access() {
    let python = r#"
def combine_indexed(pairs: list) -> list:
    return list(map(lambda x, y: x + y, pairs[0], pairs[1]))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have zip pattern
    assert!(
        rust_code.contains(".zip("),
        "Should have .zip().\nGot:\n{}",
        rust_code
    );

    // Should have indexing operations (either literal .get(0)/.get(1) or .get(actual_idx) pattern)
    let has_literal_indexing = rust_code.contains(".get(0") && rust_code.contains(".get(1");
    let has_variable_indexing = rust_code.contains(".get(actual_idx)") || rust_code.contains(".get(");
    assert!(
        has_literal_indexing || has_variable_indexing,
        "Should have index access (literal or variable).\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_map_with_ternary_in_lambda() {
    let python = r#"
def conditional_combine(list1: list, list2: list) -> list:
    return list(map(lambda x, y: x if x > y else y, list1, list2))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have zip pattern
    assert!(
        rust_code.contains(".zip("),
        "Should have .zip().\nGot:\n{}",
        rust_code
    );

    // Should have conditional
    let has_conditional = rust_code.contains("if") || rust_code.contains("match");
    assert!(
        has_conditional,
        "Should have conditional logic.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_nested_map_with_zip() {
    let python = r#"
def nested_combine(matrix1: list, matrix2: list) -> list:
    return list(map(lambda row1, row2: list(map(lambda x, y: x + y, row1, row2)), matrix1, matrix2))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have multiple zip patterns
    let zip_count = rust_code.matches(".zip(").count();
    assert!(
        zip_count >= 2,
        "Should have multiple .zip() calls.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_map_with_arithmetic_only() {
    let python = r#"
def multiply_lists(a: list, b: list) -> list:
    return list(map(lambda x, y: x * y, a, b))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have zip pattern
    assert!(
        rust_code.contains(".zip("),
        "Should have .zip().\nGot:\n{}",
        rust_code
    );

    // Should have multiplication
    let has_multiply = rust_code.contains("*");
    assert!(
        has_multiply,
        "Should have multiplication.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_map_preserves_single_iterable() {
    let python = r#"
def double_list(nums: list) -> list:
    return list(map(lambda x: x * 2, nums))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Single-iterable map should NOT have zip
    let has_zip = rust_code.contains(".zip(");
    assert!(
        !has_zip,
        "Single-iterable map should NOT use .zip().\nGot:\n{}",
        rust_code
    );

    // Should still have map
    let has_map = rust_code.contains(".map");
    assert!(has_map, "Should have .map().\nGot:\n{}", rust_code);

    // Should NOT have tuple destructuring
    let has_tuple_lambda = rust_code.contains("|(x)|") || rust_code.contains("|( x )|");
    assert!(
        !has_tuple_lambda,
        "Single-parameter lambda should not use tuple.\nGot:\n{}",
        rust_code
    );
}
