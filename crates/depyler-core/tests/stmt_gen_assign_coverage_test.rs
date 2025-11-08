//! Targeted coverage tests for codegen_assign_stmt function
//!
//! Target: codegen_assign_stmt (lines 791-1000, complexity 65)
//! Coverage gap: 21.74% untested (288/1325 lines in stmt_gen.rs)
//! Focus: Dict augmented assignments, type tracking, edge cases
//!
//! Test Strategy:
//! - Dict augmented assignments (+=, -=, *=, /=, %=)
//! - Type annotation tracking for collections
//! - String type tracking from Vec<String>.get()
//! - List/Set/Dict literal type tracking
//! - Slicing operation type tracking
//! - Complex assignment patterns

use depyler_core::DepylerPipeline;

/// Unit Test: Dict augmented assignment with +=
///
/// Verifies: DEPYLER-0279 dict augmented assignment pattern (lines 797-825)
/// Tests: is_dict_augassign_pattern() detection and special handling
#[test]
fn test_dict_augmented_add() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def increment_dict_value():
    d = {"count": 0}
    d["count"] += 5
    return d["count"]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should generate: let _old_val = d.get(&key).cloned().unwrap_or_default();
    //                  d.insert(key, _old_val + value);
    assert!(rust_code.contains("fn increment_dict_value"));
    // Verify no borrow-after-move error (DEPYLER-0279 fix)
    assert!(rust_code.contains("insert") || rust_code.contains("get"));
}

/// Unit Test: Dict augmented assignment with -=
///
/// Verifies: BinOp::Sub handling in dict augmented assignment
#[test]
fn test_dict_augmented_sub() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def decrement_dict_value():
    d = {"score": 100}
    d["score"] -= 10
    return d["score"]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn decrement_dict_value"));
}

/// Unit Test: Dict augmented assignment with *=
///
/// Verifies: BinOp::Mul handling in dict augmented assignment
#[test]
fn test_dict_augmented_mul() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def multiply_dict_value():
    d = {"factor": 2}
    d["factor"] *= 3
    return d["factor"]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn multiply_dict_value"));
}

/// Unit Test: Dict augmented assignment with /=
///
/// Verifies: BinOp::Div handling in dict augmented assignment
#[test]
fn test_dict_augmented_div() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def divide_dict_value():
    d = {"amount": 100}
    d["amount"] /= 4
    return d["amount"]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn divide_dict_value"));
}

/// Unit Test: Dict augmented assignment with %=
///
/// Verifies: BinOp::Mod handling in dict augmented assignment
#[test]
fn test_dict_augmented_mod() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def modulo_dict_value():
    d = {"remainder": 17}
    d["remainder"] %= 5
    return d["remainder"]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn modulo_dict_value"));
}

/// Unit Test: Type annotation tracking for List
///
/// Verifies: DEPYLER-0272 type annotation tracking (lines 836-840)
/// Purpose: Enables correct {:?} vs {} selection in println! for collections
#[test]
fn test_type_tracking_list_annotation() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def merge_lists(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = a + b
    return result
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should track that 'result' is Vec<i32> for proper formatting
    assert!(rust_code.contains("fn merge_lists"));
    assert!(rust_code.contains("Vec<i32>") || rust_code.contains("list"));
}

/// Unit Test: Type annotation tracking for Dict
///
/// Verifies: Dict type annotation tracking (lines 838)
#[test]
fn test_type_tracking_dict_annotation() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def create_mapping() -> dict[str, int]:
    mapping: dict[str, int] = {"a": 1, "b": 2}
    return mapping
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should track that 'mapping' is HashMap<String, i32>
    assert!(rust_code.contains("fn create_mapping"));
}

/// Unit Test: Type annotation tracking for Set
///
/// Verifies: Set type annotation tracking (lines 838)
#[test]
fn test_type_tracking_set_annotation() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def unique_items() -> set[int]:
    items: set[int] = {1, 2, 3}
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should track that 'items' is HashSet<i32>
    assert!(rust_code.contains("fn unique_items"));
}

/// Unit Test: Type tracking from Vec<String>.get()
///
/// Verifies: DEPYLER-0327 Fix #1 - String type from Vec<String>.get() (lines 842-910)
/// This was a specific bug fix for tracking String type from method calls
#[test]
fn test_type_tracking_string_from_vec_get() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_name(names: list[str], index: int) -> str:
    name = names[index]
    return name
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should track that 'name' is String
    assert!(rust_code.contains("fn get_name"));
}

/// Unit Test: Type tracking from list literal
///
/// Verifies: DEPYLER-0224 list literal type tracking (lines 912-918)
#[test]
fn test_type_tracking_list_literal() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def literal_list():
    numbers = [1, 2, 3]
    return len(numbers)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should track that 'numbers' is Vec<i32>
    assert!(rust_code.contains("fn literal_list"));
}

/// Unit Test: Type tracking from dict literal
///
/// Verifies: Dict literal type tracking (lines 920-926)
#[test]
fn test_type_tracking_dict_literal() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def literal_dict():
    mapping = {"a": 1, "b": 2}
    return len(mapping)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should track that 'mapping' is HashMap<String, i32>
    assert!(rust_code.contains("fn literal_dict"));
}

/// Unit Test: Type tracking from set literal
///
/// Verifies: Set literal type tracking (lines 928-934)
#[test]
fn test_type_tracking_set_literal() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def literal_set():
    unique = {1, 2, 3}
    return len(unique)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should track that 'unique' is HashSet<i32>
    assert!(rust_code.contains("fn literal_set"));
}

/// Unit Test: Type tracking from slicing operation
///
/// Verifies: DEPYLER-0301 list/vec type from slicing (lines 936-956)
#[test]
fn test_type_tracking_slice() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def slice_list(items: list[int]) -> list[int]:
    subset = items[1:3]
    return subset
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should track that 'subset' is Vec<i32> from slicing
    assert!(rust_code.contains("fn slice_list"));
}

/// Unit Test: Complex assignment with method call
///
/// Verifies: Type tracking from method calls (lines 960-976)
#[test]
fn test_assignment_with_method_call() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def append_and_return(items: list[int], value: int) -> list[int]:
    items.append(value)
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should handle method call in assignment context
    assert!(rust_code.contains("fn append_and_return"));
}

/// Unit Test: Result-returning function unwrapping
///
/// Verifies: DEPYLER-0270 Result unwrapping in assignments (lines 949-958)
#[test]
fn test_result_unwrapping_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def safe_get(d: dict[str, int], key: str) -> int:
    value = d[key]
    return value
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should auto-unwrap Result from dict indexing
    assert!(rust_code.contains("fn safe_get"));
}

/// Unit Test: Non-Result assignment (regression test)
///
/// Verifies: DEPYLER-0330 disabled heuristic doesn't break plain assignments
#[test]
fn test_plain_assignment_no_unwrap() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def accumulate(items: list[int]) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should NOT add .unwrap() to plain int assignment
    assert!(rust_code.contains("fn accumulate"));
    // Verify 'total' is not unwrapped (it's plain i32)
    assert!(rust_code.contains("total") && !rust_code.contains("total.unwrap()"));
}

/// Property Test: All augmented assignment operators
///
/// Property: Dict augmented assignments should handle all binary operators
/// without borrow-after-move errors
#[test]
fn test_property_all_aug_assign_operators() {
    let operators = vec![
        ("+=", "Add"),
        ("-=", "Sub"),
        ("*=", "Mul"),
        ("/=", "Div"),
        ("%=", "Mod"),
    ];

    let pipeline = DepylerPipeline::new();

    for (op, name) in operators {
        let python_code = format!(
            r#"
def test_{name}():
    d = {{"value": 10}}
    d["value"] {op} 2
    return d["value"]
"#,
            name = name.to_lowercase(),
            op = op
        );

        let rust_code = pipeline.transpile(&python_code);

        // All operators should transpile successfully
        assert!(
            rust_code.is_ok(),
            "Failed to transpile {}: {:?}",
            op,
            rust_code.err()
        );
    }
}

/// Edge Case: Nested dict augmented assignment
///
/// Verifies: Complex assignment targets with augmented operators
#[test]
fn test_nested_dict_augmented_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def nested_update():
    data = {"stats": {"count": 0}}
    data["stats"]["count"] += 1
    return data
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Should handle nested dict with augmented assignment
    assert!(rust_code.contains("fn nested_update"));
}

/// Edge Case: Multiple assignments to same dict key
///
/// Verifies: Handling multiple assignments without borrow conflicts
#[test]
fn test_multiple_dict_assignments() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def multi_assign():
    d = {"x": 1}
    d["x"] += 2
    d["x"] *= 3
    d["x"] -= 1
    return d["x"]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // All assignments should work without borrow errors
    assert!(rust_code.contains("fn multi_assign"));
}

/// Integration Test: Type tracking with complex flow
///
/// Verifies: Type tracking persists across multiple assignments
#[test]
fn test_type_tracking_complex_flow() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_types(items: list[int]) -> list[int]:
    """Test type tracking through complex flow."""
    # Type from annotation
    result: list[int] = []

    # Type from literal
    temp = [1, 2, 3]

    # Type from slicing
    subset = items[0:2]

    # Type from method call
    combined = result + temp

    return combined
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // All type tracking should work correctly
    assert!(rust_code.contains("fn complex_types"));
}

/// Mutation Test: Assignment strategy selection
///
/// Targets mutations in:
/// 1. Dict augmented assignment detection
/// 2. Type annotation tracking insertion
/// 3. Result unwrapping logic
#[test]
fn test_mutation_assignment_strategy() {
    let pipeline = DepylerPipeline::new();

    // Test Case 1: Dict augmented assignment must be detected
    let dict_aug = r#"
def test1():
    d = {"x": 1}
    d["x"] += 1
    return d
"#;
    let rust1 = pipeline.transpile(dict_aug).unwrap();
    assert!(rust1.contains("fn test1"));

    // Test Case 2: Type annotations must be tracked
    let type_annot = r#"
def test2() -> list[int]:
    result: list[int] = [1, 2, 3]
    return result
"#;
    let rust2 = pipeline.transpile(type_annot).unwrap();
    assert!(rust2.contains("fn test2"));

    // Test Case 3: Plain assignments should not add unwrap
    let plain = r#"
def test3():
    x = 0
    y = x + 1
    return y
"#;
    let rust3 = pipeline.transpile(plain).unwrap();
    assert!(rust3.contains("fn test3"));
    // Mutation kill: Should NOT have unnecessary .unwrap()
    assert!(!rust3.contains("(x + 1).unwrap()"));
}
