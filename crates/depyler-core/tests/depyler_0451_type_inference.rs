// DEPYLER-0451: Type Inference Improvements
//
// This test suite validates systematic type inference from usage context,
// reducing reliance on serde_json::Value for unannotated parameters.
//
// Created: 2025-11-21
// Ticket: https://github.com/paiml/depyler/issues/DEPYLER-0451

#![allow(non_snake_case)] // Test naming convention

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

/// Helper function to check if generated Rust code contains a pattern
fn assert_contains(rust_code: &str, pattern: &str) {
    assert!(
        rust_code.contains(pattern),
        "Expected pattern not found:\n  Pattern: {}\n  Code:\n{}",
        pattern,
        rust_code
    );
}

/// Helper function to check if generated Rust code does NOT contain a pattern
#[allow(dead_code)]
fn assert_not_contains(rust_code: &str, pattern: &str) {
    assert!(
        !rust_code.contains(pattern),
        "Unexpected pattern found:\n  Pattern: {}\n  Code:\n{}",
        pattern,
        rust_code
    );
}

// ====================================================================================
// Test 1: File Path Parameter Inference (from open() usage)
// ====================================================================================

#[test]
fn test_DEPYLER_0451_01_file_path_inference() {
    let python = r#"
def read_file(filepath):
    with open(filepath) as f:
        return f.read()
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should infer filepath as &str (not serde_json::Value)
    let has_str_param =
        rust_code.contains("filepath: &str") || rust_code.contains("filepath: String");

    // Should NOT use serde_json::Value
    let has_value_param = rust_code.contains("filepath: serde_json::Value");

    assert!(
        has_str_param && !has_value_param,
        "Expected filepath: &str or String, not Value. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 2: Integer Parameter Inference (from arithmetic operations)
// ====================================================================================

#[test]
fn test_DEPYLER_0451_02_integer_inference() {
    let python = r#"
def increment(x):
    return x + 1
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should infer x as i32/i64 (from + with integer literal)
    let has_int_param = rust_code.contains("x: i32")
        || rust_code.contains("x: i64")
        || rust_code.contains("x: isize");

    // Should NOT use serde_json::Value
    let has_value_param = rust_code.contains("x: serde_json::Value");

    assert!(
        has_int_param && !has_value_param,
        "Expected x: i32/i64, not Value. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 3: String Parameter Inference (from string methods)
// ====================================================================================

#[test]
fn test_DEPYLER_0451_03_string_method_inference() {
    let python = r#"
def greet(name):
    return f"Hello, {name}"
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should infer name as &str (from f-string usage)
    let has_str_param = rust_code.contains("name: &str") || rust_code.contains("name: String");

    // Should NOT use serde_json::Value
    let has_value_param = rust_code.contains("name: serde_json::Value");

    assert!(
        has_str_param && !has_value_param,
        "Expected name: &str or String, not Value. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 4: String Method Inference (from .upper() call)
// ====================================================================================

#[test]
fn test_DEPYLER_0451_04_string_upper_inference() {
    let python = r#"
def uppercase(text):
    return text.upper()
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should infer text as &str (from .upper() method)
    let has_str_param = rust_code.contains("text: &str") || rust_code.contains("text: String");

    // Should NOT use serde_json::Value
    let has_value_param = rust_code.contains("text: serde_json::Value");

    assert!(
        has_str_param && !has_value_param,
        "Expected text: &str or String, not Value. Got:\n{}",
        rust_code
    );

    // Should use .to_uppercase() (Rust equivalent)
    assert_contains(&rust_code, "to_uppercase");
}

// ====================================================================================
// Test 5: List Parameter Inference (from iteration)
// ====================================================================================

#[test]
fn test_DEPYLER_0451_05_list_iteration_inference() {
    let python = r#"
def sum_list(items):
    total = 0
    for item in items:
        total += item
    return total
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should infer items as slice/Vec (from iteration + arithmetic)
    let has_slice_param = rust_code.contains("items: &[i32]")
        || rust_code.contains("items: &[i64]")
        || rust_code.contains("items: Vec<i32>")
        || rust_code.contains("items: Vec<i64>")
        || rust_code.contains("items: &Vec<i32>")
        || rust_code.contains("items: &Vec<i64>");

    // Should NOT use serde_json::Value
    let has_value_param = rust_code.contains("items: serde_json::Value")
        || rust_code.contains("items: &serde_json::Value");

    assert!(
        has_slice_param && !has_value_param,
        "Expected items: &[i32] or Vec<i32> or &Vec<i32>, not Value. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 6: Boolean Parameter Inference (from conditional usage)
// ====================================================================================

#[test]
fn test_DEPYLER_0451_06_boolean_inference() {
    let python = r#"
def conditional(flag):
    if flag:
        return "yes"
    return "no"
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should infer flag as bool (from if condition)
    let has_bool_param = rust_code.contains("flag: bool");

    // Should NOT use serde_json::Value
    let has_value_param = rust_code.contains("flag: serde_json::Value")
        || rust_code.contains("flag: &serde_json::Value");

    assert!(
        has_bool_param && !has_value_param,
        "Expected flag: bool, not Value. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 7: Mixed Type Inference (multiple constraints)
// ====================================================================================

#[test]
fn test_DEPYLER_0451_07_mixed_inference() {
    let python = r#"
def process(data, flag):
    if flag:
        return data.upper()
    return data
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should infer data as &str (from .upper() method)
    let has_str_data = rust_code.contains("data: &str") || rust_code.contains("data: String");

    // Should infer flag as bool (from if condition)
    let has_bool_flag = rust_code.contains("flag: bool");

    // Should NOT use serde_json::Value for either
    let has_value_data = rust_code.contains("data: serde_json::Value")
        || rust_code.contains("data: &serde_json::Value");
    let has_value_flag = rust_code.contains("flag: serde_json::Value");

    assert!(
        has_str_data && !has_value_data,
        "Expected data: &str or String, not Value. Got:\n{}",
        rust_code
    );

    assert!(
        has_bool_flag && !has_value_flag,
        "Expected flag: bool, not Value. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 8: CSV Reader Type Inference (stdlib API context)
// ====================================================================================

#[test]
#[ignore] // DEPYLER-0451: Phase 3 - Context-aware stdlib API inference
fn test_DEPYLER_0451_08_csv_reader_inference() {
    let python = r#"
import csv

def process_csv(path):
    with open(path) as f:
        reader = csv.DictReader(f)
        for row in reader:
            print(row['name'])
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should infer path as &str
    let has_str_param = rust_code.contains("path: &str") || rust_code.contains("path: String");

    // Should NOT use serde_json::Value
    let has_value_param = rust_code.contains("path: serde_json::Value");

    assert!(
        has_str_param && !has_value_param,
        "Expected path: &str or String, not Value. Got:\n{}",
        rust_code
    );

    // Should use correct csv iteration pattern
    assert_contains(&rust_code, "csv::ReaderBuilder");
}

// ====================================================================================
// Test 9: Backward Propagation from Return Type
// ====================================================================================

#[test]
fn test_DEPYLER_0451_09_return_type_propagation() {
    let python = r#"
def get_length(text: str) -> int:
    return len(text)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should respect type annotations
    let has_str_param = rust_code.contains("text: &str") || rust_code.contains("text: String");

    let has_int_return = rust_code.contains("-> i32")
        || rust_code.contains("-> i64")
        || rust_code.contains("-> usize");

    assert!(
        has_str_param,
        "Expected text: &str or String. Got:\n{}",
        rust_code
    );

    assert!(
        has_int_return,
        "Expected -> i32/i64/usize. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 10: Multiple Parameter Inference (complex function)
// ====================================================================================

#[test]
fn test_DEPYLER_0451_10_multiple_parameters() {
    let python = r#"
def search(items, target):
    for item in items:
        if item == target:
            return True
    return False
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should infer items as slice (from iteration)
    let has_slice_param = rust_code.contains("items: &[") || rust_code.contains("items: Vec<");

    // Should NOT use serde_json::Value for items
    let has_value_items = rust_code.contains("items: serde_json::Value")
        || rust_code.contains("items: &serde_json::Value");

    // target type depends on items element type, may be generic or inferred
    // For now, just check it's not needlessly Value
    let has_value_target = rust_code.contains("target: serde_json::Value");

    assert!(
        has_slice_param && !has_value_items,
        "Expected items: &[T] or Vec<T>, not Value. Got:\n{}",
        rust_code
    );

    // Lenient on target type (could be generic T or specific type)
    // Main point: avoid Value when possible
    if has_value_target {
        eprintln!("Warning: target still using Value, but may be acceptable for generic case");
    }
}
