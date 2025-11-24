//! Mutation Testing: Type Inference Validation
//!
//! This test file targets MISSED mutations in ast_bridge.rs:infer_type_from_expr
//! Kill target: Lines 969-985 (type inference match arms)
//!
//! MISSED Mutations Being Targeted:
//! - Line 971: delete match arm ast::Constant::Int(_)
//! - Line 972: delete match arm ast::Constant::Float(_)
//! - Line 973: delete match arm ast::Constant::Str(_)
//! - Line 974: delete match arm ast::Constant::Bool(_)
//! - Line 975: delete match arm ast::Constant::None
//! - Line 978: delete match arm ast::Expr::List(_)
//! - Line 979: delete match arm ast::Expr::Dict(_)
//! - Line 970: delete match arm ast::Expr::Constant(c)
//! - Line 982: delete match arm ast::Expr::Set(_)

use depyler_core::ast_bridge::AstBridge;
use depyler_core::hir::Type;
use rustpython_parser::{parse, Mode};

/// Helper to test that a Python variable assignment correctly infers the type
/// from the assigned expression. This exercises infer_type_from_expr through
/// the field inference path in class __init__ methods.
fn assert_field_type_inference(python_code: &str, expected_field_name: &str, expected_type: Type) {
    let ast = parse(python_code, Mode::Module, "<test>").expect("Failed to parse");
    let bridge = AstBridge::new();
    let (hir, _type_env) = bridge.python_to_hir(ast).expect("Failed to convert to HIR");

    // Find the class and check its fields
    assert!(!hir.classes.is_empty(), "Expected at least one class");
    let class = &hir.classes[0];

    // Find the field with the expected name
    let field = class
        .fields
        .iter()
        .find(|f| f.name == expected_field_name)
        .unwrap_or_else(|| panic!("Field '{}' not found in class", expected_field_name));

    assert_eq!(
        field.field_type, expected_type,
        "Field '{}' has incorrect type. Expected {:?}, got {:?}",
        expected_field_name, expected_type, field.field_type
    );
}

// ============================================================================
// MUTATION KILL TESTS: ast::Constant::Int(_) - Line 971
// ============================================================================

#[test]
fn test_infer_type_from_int_literal() {
    let python = r#"
class Config:
    def __init__(self):
        self.count = 42
"#;
    assert_field_type_inference(python, "count", Type::Int);
}

// Note: Negative numbers in Python are UnaryOp expressions, not constants
// So they don't go through the infer_type_from_expr Constant match arm
// Skipping this test as it would test a different code path

#[test]
fn test_infer_type_from_zero() {
    let python = r#"
class Config:
    def __init__(self):
        self.zero = 0
"#;
    assert_field_type_inference(python, "zero", Type::Int);
}

// ============================================================================
// MUTATION KILL TESTS: ast::Constant::Float(_) - Line 972
// ============================================================================

#[test]
fn test_infer_type_from_float_literal() {
    let python = r#"
class Config:
    def __init__(self):
        self.pi = 3.14159
"#;
    assert_field_type_inference(python, "pi", Type::Float);
}

#[test]
fn test_infer_type_from_scientific_notation() {
    let python = r#"
class Config:
    def __init__(self):
        self.speed_of_light = 3.0e8
"#;
    assert_field_type_inference(python, "speed_of_light", Type::Float);
}

// Note: Negative floats in Python are UnaryOp expressions, not constants
// Skipping this test as it would test a different code path

// ============================================================================
// MUTATION KILL TESTS: ast::Constant::Str(_) - Line 973
// ============================================================================

#[test]
fn test_infer_type_from_string_literal() {
    let python = r#"
class Config:
    def __init__(self):
        self.name = "depyler"
"#;
    assert_field_type_inference(python, "name", Type::String);
}

#[test]
fn test_infer_type_from_empty_string() {
    let python = r#"
class Config:
    def __init__(self):
        self.empty = ""
"#;
    assert_field_type_inference(python, "empty", Type::String);
}

#[test]
fn test_infer_type_from_multiline_string() {
    let python = r#"
class Config:
    def __init__(self):
        self.description = """
        This is a multiline
        description string
        """
"#;
    assert_field_type_inference(python, "description", Type::String);
}

// ============================================================================
// MUTATION KILL TESTS: ast::Constant::Bool(_) - Line 974
// ============================================================================

#[test]
fn test_infer_type_from_true_literal() {
    let python = r#"
class Config:
    def __init__(self):
        self.enabled = True
"#;
    assert_field_type_inference(python, "enabled", Type::Bool);
}

#[test]
fn test_infer_type_from_false_literal() {
    let python = r#"
class Config:
    def __init__(self):
        self.disabled = False
"#;
    assert_field_type_inference(python, "disabled", Type::Bool);
}

// ============================================================================
// MUTATION KILL TESTS: ast::Constant::None - Line 975
// ============================================================================

#[test]
fn test_infer_type_from_none_literal() {
    let python = r#"
class Config:
    def __init__(self):
        self.optional = None
"#;
    assert_field_type_inference(python, "optional", Type::None);
}

// ============================================================================
// MUTATION KILL TESTS: ast::Expr::List(_) - Line 978
// ============================================================================

#[test]
fn test_infer_type_from_empty_list() {
    let python = r#"
class Config:
    def __init__(self):
        self.items = []
"#;
    assert_field_type_inference(python, "items", Type::List(Box::new(Type::Unknown)));
}

#[test]
fn test_infer_type_from_list_with_elements() {
    let python = r#"
class Config:
    def __init__(self):
        self.numbers = [1, 2, 3]
"#;
    assert_field_type_inference(python, "numbers", Type::List(Box::new(Type::Unknown)));
}

// ============================================================================
// MUTATION KILL TESTS: ast::Expr::Dict(_) - Line 979
// ============================================================================

#[test]
fn test_infer_type_from_empty_dict() {
    let python = r#"
class Config:
    def __init__(self):
        self.mapping = {}
"#;
    assert_field_type_inference(
        python,
        "mapping",
        Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)),
    );
}

#[test]
fn test_infer_type_from_dict_with_entries() {
    let python = r#"
class Config:
    def __init__(self):
        self.config = {"key": "value", "count": 42}
"#;
    assert_field_type_inference(
        python,
        "config",
        Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)),
    );
}

// ============================================================================
// MUTATION KILL TESTS: ast::Expr::Set(_) - Line 982
// ============================================================================

#[test]
fn test_infer_type_from_set_literal() {
    let python = r#"
class Config:
    def __init__(self):
        self.unique_items = {1, 2, 3}
"#;
    assert_field_type_inference(python, "unique_items", Type::Set(Box::new(Type::Unknown)));
}

#[test]
fn test_infer_type_from_set_with_strings() {
    let python = r#"
class Config:
    def __init__(self):
        self.tags = {"python", "rust", "transpiler"}
"#;
    assert_field_type_inference(python, "tags", Type::Set(Box::new(Type::Unknown)));
}

// ============================================================================
// EDGE CASE TESTS: Ensure all type paths are exercised
// ============================================================================

#[test]
fn test_multiple_fields_with_different_types() {
    let python = r#"
class Config:
    def __init__(self):
        self.count = 10
        self.ratio = 3.14
        self.name = "test"
        self.active = True
        self.optional = None
        self.items = []
        self.mapping = {}
        self.unique = {1, 2}
"#;

    let ast = parse(python, Mode::Module, "<test>").expect("Failed to parse");
    let bridge = AstBridge::new();
    let (hir, _type_env) = bridge.python_to_hir(ast).expect("Failed to convert to HIR");

    assert!(!hir.classes.is_empty());
    let class = &hir.classes[0];

    // Verify all 8 fields have correct types
    assert_eq!(class.fields.len(), 8, "Expected 8 fields");

    let field_types: Vec<(&str, Type)> = vec![
        ("count", Type::Int),
        ("ratio", Type::Float),
        ("name", Type::String),
        ("active", Type::Bool),
        ("optional", Type::None),
        ("items", Type::List(Box::new(Type::Unknown))),
        (
            "mapping",
            Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)),
        ),
        ("unique", Type::Set(Box::new(Type::Unknown))),
    ];

    for (expected_name, expected_type) in field_types {
        let field = class
            .fields
            .iter()
            .find(|f| f.name == expected_name)
            .unwrap_or_else(|| panic!("Field '{}' not found", expected_name));

        assert_eq!(
            field.field_type, expected_type,
            "Field '{}' has wrong type: expected {:?}, got {:?}",
            expected_name, expected_type, field.field_type
        );
    }
}

// ============================================================================
// COVERAGE: Test that unknown expressions return None/Unknown
// ============================================================================

#[test]
fn test_complex_expression_falls_back_to_unknown() {
    let python = r#"
class Config:
    def __init__(self):
        self.computed = len([1, 2, 3])
"#;

    let ast = parse(python, Mode::Module, "<test>").expect("Failed to parse");
    let bridge = AstBridge::new();
    let (hir, _type_env) = bridge.python_to_hir(ast).expect("Failed to convert to HIR");

    // Complex expressions should either infer as Unknown or not be added as fields
    assert!(!hir.classes.is_empty());
    let class = &hir.classes[0];

    // The field may exist with Unknown type, or may not exist at all
    // Both are acceptable behaviors for complex expressions
    if let Some(field) = class.fields.iter().find(|f| f.name == "computed") {
        assert_eq!(
            field.field_type,
            Type::Unknown,
            "Complex expression should be Unknown type"
        );
    }
}
