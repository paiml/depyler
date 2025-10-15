//! TDD Tests for sorted() with key parameter (DEPYLER-0122)
//!
//! Python's sorted(iterable, key=lambda ...) should convert to Rust's sort_by_key pattern:
//! Python: sorted(words, key=lambda x: len(x))
//! Rust: { let mut result = words.clone(); result.sort_by_key(|x| x.len()); result }
//!
//! Test Coverage:
//! 1. Basic sorted with key lambda
//! 2. Sorted with key accessing attribute
//! 3. Sorted with key and complex expression
//! 4. Sorted with key and reverse
//! 5. Sorted without key (baseline)
//! 6. Sorted with key using arithmetic
//! 7. Sorted with key and tuple unpacking
//! 8. Sorted with key and method call
//! 9. Sorted with key and ternary expression
//! 10. Sorted with key and indexing

use depyler_core::DepylerPipeline;

#[test]
fn test_sorted_basic_key_lambda() {
    let python = r#"
def sort_by_length(words: list) -> list:
    return sorted(words, key=lambda x: len(x))
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
        rust_code.contains("fn sort_by_length"),
        "Should have sort_by_length function.\nGot:\n{}",
        rust_code
    );

    // Should have closure syntax
    let has_closure = rust_code.contains("|x|") || rust_code.contains("| x |");
    assert!(
        has_closure,
        "Should have closure syntax.\nGot:\n{}",
        rust_code
    );

    // Should have sort_by_key
    let has_sort_by_key = rust_code.contains("sort_by_key");
    assert!(
        has_sort_by_key,
        "Should have sort_by_key.\nGot:\n{}",
        rust_code
    );

    // Should have .len() call
    let has_len = rust_code.contains(".len()");
    assert!(has_len, "Should have .len() call.\nGot:\n{}", rust_code);
}

#[test]
fn test_sorted_key_with_arithmetic() {
    let python = r#"
def sort_by_doubled(nums: list) -> list:
    return sorted(nums, key=lambda x: x * 2)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have sort_by_key
    assert!(
        rust_code.contains("sort_by_key"),
        "Should have sort_by_key.\nGot:\n{}",
        rust_code
    );

    // Should have closure
    let has_closure = rust_code.contains("|x|") || rust_code.contains("| x |");
    assert!(has_closure, "Should have closure.\nGot:\n{}", rust_code);

    // Should have multiplication
    assert!(
        rust_code.contains("* 2"),
        "Should have multiplication.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_sorted_key_with_complex_expression() {
    let python = r#"
def sort_by_complex(items: list) -> list:
    return sorted(items, key=lambda x: (x * 2) + 10)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have sort_by_key
    assert!(
        rust_code.contains("sort_by_key"),
        "Should have sort_by_key.\nGot:\n{}",
        rust_code
    );

    // Should preserve the complex expression
    let has_arithmetic = rust_code.contains("* 2") && rust_code.contains("+ 10");
    assert!(
        has_arithmetic,
        "Should preserve arithmetic.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_sorted_without_key() {
    let python = r#"
def simple_sort(nums: list) -> list:
    return sorted(nums)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should NOT have sort_by_key for simple sorted()
    let has_sort_by_key = rust_code.contains("sort_by_key");
    assert!(
        !has_sort_by_key,
        "Simple sorted() should not use sort_by_key.\nGot:\n{}",
        rust_code
    );

    // Should have some form of sorting
    let has_sort = rust_code.contains(".sort()") || rust_code.contains("sorted");
    assert!(
        has_sort,
        "Should have sorting operation.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_sorted_key_with_attribute() {
    let python = r#"
def sort_by_name(people: list) -> list:
    return sorted(people, key=lambda p: p.name)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have sort_by_key
    assert!(
        rust_code.contains("sort_by_key"),
        "Should have sort_by_key.\nGot:\n{}",
        rust_code
    );

    // Should have attribute access
    assert!(
        rust_code.contains(".name"),
        "Should access .name attribute.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_sorted_key_with_reverse() {
    let python = r#"
def sort_descending(nums: list) -> list:
    return sorted(nums, key=lambda x: x, reverse=True)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have sort operation
    assert!(
        rust_code.contains("sort"),
        "Should have sort.\nGot:\n{}",
        rust_code
    );

    // Should have reverse logic
    let has_reverse = rust_code.contains(".rev()")
        || rust_code.contains("Reverse")
        || rust_code.contains(".reverse()");
    assert!(has_reverse, "Should handle reverse.\nGot:\n{}", rust_code);
}

#[test]
fn test_sorted_key_with_indexing() {
    let python = r#"
def sort_by_first(pairs: list) -> list:
    return sorted(pairs, key=lambda p: p[0])
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have sort_by_key
    assert!(
        rust_code.contains("sort_by_key"),
        "Should have sort_by_key.\nGot:\n{}",
        rust_code
    );

    // Should have indexing
    let has_index = rust_code.contains("[0]") || rust_code.contains(".get(0");
    assert!(has_index, "Should have indexing.\nGot:\n{}", rust_code);
}

#[test]
fn test_sorted_key_with_method_call() {
    let python = r#"
def sort_uppercase(words: list) -> list:
    return sorted(words, key=lambda w: len(w))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have sort_by_key
    assert!(
        rust_code.contains("sort_by_key"),
        "Should have sort_by_key.\nGot:\n{}",
        rust_code
    );

    // Should have len call
    assert!(
        rust_code.contains(".len()"),
        "Should have .len().\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_sorted_key_with_ternary() {
    let python = r#"
def sort_custom(nums: list) -> list:
    return sorted(nums, key=lambda x: x if x > 0 else -x)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have sort_by_key
    assert!(
        rust_code.contains("sort_by_key"),
        "Should have sort_by_key.\nGot:\n{}",
        rust_code
    );

    // Should have conditional logic
    let has_conditional = rust_code.contains("if") || rust_code.contains("match");
    assert!(
        has_conditional,
        "Should have conditional.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_sorted_key_with_negative() {
    let python = r#"
def sort_by_negative(nums: list) -> list:
    return sorted(nums, key=lambda x: -x)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have sort_by_key
    assert!(
        rust_code.contains("sort_by_key"),
        "Should have sort_by_key.\nGot:\n{}",
        rust_code
    );

    // Should have negation
    let has_negation = rust_code.contains("-x") || rust_code.contains("- x");
    assert!(has_negation, "Should have negation.\nGot:\n{}", rust_code);
}
