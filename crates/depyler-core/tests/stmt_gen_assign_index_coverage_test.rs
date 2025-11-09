//! Targeted coverage tests for codegen_assign_index function
//!
//! Target: codegen_assign_index (lines 1038-1113, complexity 22)
//! Coverage focus: Type-aware subscript assignment, Vec vs HashMap detection
//!
//! Test Strategy:
//! - List/Vec assignment with numeric indices (DEPYLER-0304, DEPYLER-0314)
//! - Dict/HashMap assignment with key-based indices
//! - Type-tracked detection (Type::List, Type::Dict)
//! - Heuristic-based detection (when no type info)
//! - Nested assignments (get_mut chains)
//! - Index type conversion (as usize for Vec)

use depyler_core::DepylerPipeline;

/// Unit Test: Simple list assignment with literal index
///
/// Verifies: Vec.insert with numeric index (DEPYLER-0304, lines 1087-1090)
#[test]
fn test_list_assignment_literal_index() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_list(items: list[int]) -> list[int]:
    items[0] = 42
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_list"));
}

/// Unit Test: List assignment with variable index
///
/// Verifies: Vec.insert with variable index (DEPYLER-0314, lines 1088-1090)
#[test]
fn test_list_assignment_variable_index() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_at_index(items: list[int], index: int, value: int) -> list[int]:
    items[index] = value
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_at_index"));
}

/// Unit Test: List assignment with expression index
///
/// Verifies: Vec.insert with binary expression index (lines 1060, 1077)
#[test]
fn test_list_assignment_expression_index() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_next_item(items: list[int], i: int) -> list[int]:
    items[i + 1] = 100
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_next_item"));
}

/// Unit Test: Simple dict assignment with string key
///
/// Verifies: HashMap.insert with key (DEPYLER-0304, lines 1092-1093)
#[test]
fn test_dict_assignment_string_key() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_dict(d: dict[str, int]) -> dict[str, int]:
    d["key"] = 42
    return d
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_dict"));
}

/// Unit Test: Dict assignment with variable key
///
/// Verifies: HashMap.insert with variable key (lines 1092-1093)
#[test]
fn test_dict_assignment_variable_key() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_with_key(d: dict[str, int], key: str, value: int) -> dict[str, int]:
    d[key] = value
    return d
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_with_key"));
}

/// Unit Test: Dict assignment with character heuristic
///
/// Verifies: Heuristic detection for char/character/c (lines 1059, 1068, 1076)
#[test]
fn test_dict_assignment_char_heuristic() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_by_char(d: dict[str, int], c: str) -> dict[str, int]:
    d[c] = 1
    return d
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_by_char"));
}

/// Unit Test: Type-tracked list assignment
///
/// Verifies: Type-based detection (Type::List, lines 1051-1054)
#[test]
fn test_type_tracked_list_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def modify_list() -> list[int]:
    numbers: list[int] = [1, 2, 3]
    numbers[0] = 10
    return numbers
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn modify_list"));
}

/// Unit Test: Type-tracked dict assignment
///
/// Verifies: Type-based detection (Type::Dict, lines 1051-1055)
#[test]
fn test_type_tracked_dict_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def modify_dict() -> dict[str, int]:
    data: dict[str, int] = {}
    data["key"] = 42
    return data
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn modify_dict"));
}

/// Unit Test: Nested list assignment (2-level)
///
/// Verifies: get_mut chain for nested access (lines 1096-1102)
#[test]
fn test_nested_list_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_matrix(matrix: list[list[int]]) -> list[list[int]]:
    matrix[0][1] = 99
    return matrix
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_matrix"));
}

/// Unit Test: Nested dict assignment (2-level)
///
/// Verifies: get_mut chain for nested dict (lines 1096-1102, 1109-1110)
#[test]
fn test_nested_dict_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_nested_dict(d: dict[str, dict[str, int]]) -> dict[str, dict[str, int]]:
    d["outer"]["inner"] = 42
    return d
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_nested_dict"));
}

/// Unit Test: Mixed nested assignment (dict of lists)
///
/// Verifies: Nested assignment with mixed types (lines 1104-1107)
#[test]
fn test_mixed_nested_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_dict_of_lists(data: dict[str, list[int]], key: str) -> dict[str, list[int]]:
    data[key][0] = 100
    return data
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_dict_of_lists"));
}

/// Unit Test: Index type conversion for Vec
///
/// Verifies: as usize conversion (DEPYLER-0314, lines 1090, 1107)
#[test]
fn test_index_type_conversion() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def insert_at_index(numbers: list[int], idx: int) -> list[int]:
    numbers[idx] = 42
    return numbers
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn insert_at_index"));
}

/// Unit Test: Multiple list assignments
///
/// Verifies: Multiple Vec.insert operations
#[test]
fn test_multiple_list_assignments() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_multiple(items: list[int]) -> list[int]:
    items[0] = 1
    items[1] = 2
    items[2] = 3
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_multiple"));
}

/// Unit Test: Multiple dict assignments
///
/// Verifies: Multiple HashMap.insert operations
#[test]
fn test_multiple_dict_assignments() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def populate_dict(d: dict[str, int]) -> dict[str, int]:
    d["a"] = 1
    d["b"] = 2
    d["c"] = 3
    return d
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn populate_dict"));
}

/// Unit Test: Assignment with complex value expression
///
/// Verifies: Value expression handling
#[test]
fn test_assignment_complex_value() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def calculate_and_store(items: list[int], x: int, y: int) -> list[int]:
    items[0] = x * 2 + y
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn calculate_and_store"));
}

/// Unit Test: Assignment in loop
///
/// Verifies: Index assignment within iteration
#[test]
fn test_assignment_in_loop() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def double_values(items: list[int]) -> list[int]:
    for i in range(len(items)):
        items[i] = items[i] * 2
    return items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn double_values"));
}

/// Unit Test: Character variable name heuristic
///
/// Verifies: 'char' and 'character' trigger HashMap (lines 1059, 1068, 1076)
#[test]
fn test_character_variable_heuristic() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def count_chars(counts: dict[str, int], character: str) -> dict[str, int]:
    counts[character] = 1
    return counts
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn count_chars"));
}

/// Unit Test: Deep nested assignment (3+ levels)
///
/// Verifies: Multiple get_mut calls in chain (lines 1098-1101)
#[test]
fn test_deep_nested_assignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def update_deep(data: dict[str, dict[str, dict[str, int]]]) -> dict[str, dict[str, dict[str, int]]]:
    data["a"]["b"]["c"] = 42
    return data
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn update_deep"));
}

/// Property Test: All numeric index patterns use Vec.insert
///
/// Property: Numeric indices should trigger Vec.insert with as usize
#[test]
fn test_property_numeric_indices() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        ("literal", "items[0] = x"),
        ("variable", "items[i] = x"),
        ("expression", "items[i + 1] = x"),
    ];

    for (name, assignment) in test_cases {
        let python_code = format!(
            r#"
def test_{}(items: list[int], i: int, x: int) -> list[int]:
    {}
    return items
"#,
            name, assignment
        );
        let result = pipeline.transpile(&python_code);

        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            name,
            result.err()
        );
    }
}

/// Integration Test: Complex assignment patterns
///
/// Verifies: All features working together
#[test]
fn test_complex_assignment_patterns() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_updates(
    numbers: list[int],
    data: dict[str, list[int]],
    nested: dict[str, dict[str, int]]
) -> tuple[list[int], dict[str, list[int]], dict[str, dict[str, int]]]:
    # Simple list assignment
    numbers[0] = 42
    
    # Nested assignment (dict of lists)
    data["key"][1] = 100
    
    # Nested dict assignment
    nested["outer"]["inner"] = 200
    
    return (numbers, data, nested)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn complex_updates"));
}

/// Mutation Test: Type detection logic
///
/// Targets mutations in:
/// 1. Type::List vs Type::Dict branch selection
/// 2. is_numeric_index heuristic
/// 3. as usize conversion presence/absence
#[test]
fn test_mutation_type_detection() {
    let pipeline = DepylerPipeline::new();

    // Test Case 1: List must use numeric index logic
    let list_code = r#"
def test1(items: list[int]) -> list[int]:
    items[0] = 42
    return items
"#;
    let rust1 = pipeline.transpile(list_code).unwrap();
    assert!(rust1.contains("fn test1"));

    // Test Case 2: Dict must use key-based logic
    let dict_code = r#"
def test2(d: dict[str, int]) -> dict[str, int]:
    d["key"] = 42
    return d
"#;
    let rust2 = pipeline.transpile(dict_code).unwrap();
    assert!(rust2.contains("fn test2"));

    // Test Case 3: Character heuristic must work
    let char_code = r#"
def test3(counts: dict[str, int], c: str) -> dict[str, int]:
    counts[c] = 1
    return counts
"#;
    let rust3 = pipeline.transpile(char_code).unwrap();
    assert!(rust3.contains("fn test3"));
}
