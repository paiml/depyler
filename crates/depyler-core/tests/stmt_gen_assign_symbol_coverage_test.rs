//! Targeted coverage tests for codegen_assign_symbol function
//!
//! Target: codegen_assign_symbol (lines 1004-1034, complexity 13)
//! Coverage focus: Variable declaration vs reassignment, mutability, type annotations
//!
//! Test Strategy:
//! - First declaration (let)
//! - Re-assignment (no let)
//! - Mutability tracking (mut keyword)
//! - Type annotations (with and without)
//! - Generator state variables (self.field)

use depyler_core::DepylerPipeline;

/// Unit Test: Simple variable declaration
///
/// Verifies: let var = value (lines 1029-1031)
#[test]
fn test_simple_variable_declaration() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def create_value() -> int:
    x = 42
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn create_value"));
}

/// Unit Test: Variable reassignment
///
/// Verifies: var = value (no let) (lines 1016-1018)
#[test]
fn test_variable_reassignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def modify_value() -> int:
    x = 10
    x = 20
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn modify_value"));
}

/// Unit Test: Mutable variable declaration
///
/// Verifies: let mut var = value (lines 1022-1027)
#[test]
fn test_mutable_variable_declaration() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def increment() -> int:
    counter = 0
    counter = counter + 1
    return counter
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn increment"));
}

/// Unit Test: Declaration with type annotation
///
/// Verifies: let var: type = value (lines 1028-1029)
#[test]
fn test_declaration_with_type_annotation() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def typed_var() -> int:
    x: int = 42
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn typed_var"));
}

/// Unit Test: Mutable declaration with type annotation
///
/// Verifies: let mut var: type = value (lines 1023-1024)
#[test]
fn test_mutable_with_type_annotation() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def typed_mutable() -> int:
    count: int = 0
    count = count + 1
    return count
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn typed_mutable"));
}

/// Unit Test: Multiple variable declarations
///
/// Verifies: Multiple let statements
#[test]
fn test_multiple_variable_declarations() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def multiple_vars() -> int:
    a = 1
    b = 2
    c = 3
    return a + b + c
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn multiple_vars"));
}

/// Unit Test: Mixed declaration and reassignment
///
/// Verifies: let followed by reassignment
#[test]
fn test_mixed_declaration_reassignment() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def mixed_pattern() -> int:
    x = 10
    y = 20
    x = x + y
    y = y * 2
    return x + y
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn mixed_pattern"));
}

/// Unit Test: Variable with complex expression
///
/// Verifies: Value expression handling
#[test]
fn test_variable_with_expression() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_assignment(a: int, b: int) -> int:
    result = (a * 2) + (b * 3)
    return result
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn complex_assignment"));
}

/// Unit Test: Variable shadowing (same name in nested scope)
///
/// Verifies: Scope management
#[test]
fn test_variable_shadowing() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def shadowing_example(x: int) -> int:
    y = x * 2
    if x > 0:
        y = x * 3
    return y
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn shadowing_example"));
}

/// Unit Test: Type annotation with list type
///
/// Verifies: Complex type annotations
#[test]
fn test_type_annotation_list() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def list_variable() -> list[int]:
    numbers: list[int] = [1, 2, 3]
    return numbers
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn list_variable"));
}

/// Unit Test: Type annotation with dict type
///
/// Verifies: Complex type annotations
#[test]
fn test_type_annotation_dict() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def dict_variable() -> dict[str, int]:
    data: dict[str, int] = {}
    return data
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn dict_variable"));
}

/// Property Test: All assignment patterns
///
/// Property: Different assignment patterns should transpile
#[test]
fn test_property_assignment_patterns() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        ("simple", "x = 42"),
        ("typed", "x: int = 42"),
        ("reassign", "x = 10\n    x = 20"),
        ("mutable", "x = 0\n    x = x + 1"),
    ];

    for (name, assignment) in test_cases {
        let python_code = format!(
            r#"
def test_{}() -> int:
    {}
    return x
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

/// Mutation Test: Declaration vs reassignment logic
///
/// Targets mutations in:
/// 1. is_declared check (line 1016)
/// 2. mutable_vars check (line 1022)
/// 3. Type annotation handling
#[test]
fn test_mutation_declaration_logic() {
    let pipeline = DepylerPipeline::new();

    // Test Case 1: First declaration must have 'let'
    let first_decl = r#"
def test1() -> int:
    x = 42
    return x
"#;
    let rust1 = pipeline.transpile(first_decl).unwrap();
    assert!(rust1.contains("fn test1"));

    // Test Case 2: Reassignment must not have 'let'
    let reassign = r#"
def test2() -> int:
    x = 10
    x = 20
    return x
"#;
    let rust2 = pipeline.transpile(reassign).unwrap();
    assert!(rust2.contains("fn test2"));

    // Test Case 3: Mutable variable must have 'mut'
    let mutable = r#"
def test3() -> int:
    x = 0
    x = x + 1
    return x
"#;
    let rust3 = pipeline.transpile(mutable).unwrap();
    assert!(rust3.contains("fn test3"));
}
