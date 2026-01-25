//! DEPYLER-0977: Field type inference from method usage
//!
//! Tests for bidirectional type inference where field types are inferred
//! from their usage in methods (e.g., self.messages.append(msg: str) → List[str])

use depyler_core::ast_bridge::python_to_hir;
use depyler_core::hir::Type;
use rustpython_parser::{parse, Mode};

/// Test: Field initialized as [] should infer type from append(typed_param)
/// TODO: Implement bidirectional field type inference from method usage
#[test]
#[ignore = "needs bidirectional field type inference"]
fn test_depyler_0977_list_field_infer_from_append() {
    let python = r#"
class Logger:
    def __init__(self):
        self.messages = []

    def log(self, msg: str) -> int:
        self.messages.append(msg)
        return len(self.messages)
"#;

    let ast = parse(python, Mode::Module, "<test>").unwrap();
    let (hir, _) = python_to_hir(ast).unwrap();

    assert_eq!(hir.classes.len(), 1);
    let class = &hir.classes[0];
    assert_eq!(class.name, "Logger");

    // Find the messages field
    let messages_field = class.fields.iter().find(|f| f.name == "messages");
    assert!(messages_field.is_some(), "Should have messages field");

    let field = messages_field.unwrap();
    // Should be List[String], not List[Unknown]
    match &field.field_type {
        Type::List(inner) => {
            assert_eq!(
                **inner,
                Type::String,
                "messages should be List[String] from append(msg: str)"
            );
        }
        _ => panic!("messages should be List type, got {:?}", field.field_type),
    }
}

/// Test: Field type inference from direct assignment in method
#[test]
fn test_depyler_0977_field_infer_from_assignment() {
    let python = r#"
class Config:
    def __init__(self):
        self.mode = ""
        self.timeout = 0
        self.retry = False

    def configure(self, mode: str, timeout: int, retry: bool):
        self.mode = mode
        self.timeout = timeout
        self.retry = retry
"#;

    let ast = parse(python, Mode::Module, "<test>").unwrap();
    let (hir, _) = python_to_hir(ast).unwrap();

    let class = &hir.classes[0];

    // Find fields
    let mode_field = class.fields.iter().find(|f| f.name == "mode");
    let timeout_field = class.fields.iter().find(|f| f.name == "timeout");
    let retry_field = class.fields.iter().find(|f| f.name == "retry");

    // These should already have correct types from literal inference
    assert!(matches!(
        mode_field.map(|f| &f.field_type),
        Some(Type::String)
    ));
    assert!(matches!(
        timeout_field.map(|f| &f.field_type),
        Some(Type::Int)
    ));
    assert!(matches!(
        retry_field.map(|f| &f.field_type),
        Some(Type::Bool)
    ));
}

/// Test: Dict field type inference from update/assignment
/// TODO: Implement dict field type inference from subscript assignment
#[test]
#[ignore = "needs dict field type inference from subscript"]
fn test_depyler_0977_dict_field_infer_from_subscript_assign() {
    let python = r#"
class Cache:
    def __init__(self):
        self.data = {}

    def set(self, key: str, value: int):
        self.data[key] = value
"#;

    let ast = parse(python, Mode::Module, "<test>").unwrap();
    let (hir, _) = python_to_hir(ast).unwrap();

    let class = &hir.classes[0];
    let data_field = class.fields.iter().find(|f| f.name == "data");
    assert!(data_field.is_some(), "Should have data field");

    let field = data_field.unwrap();
    // Should be Dict[String, Int] from self.data[key: str] = value: int
    match &field.field_type {
        Type::Dict(key_type, value_type) => {
            assert_eq!(**key_type, Type::String, "dict key should be String");
            assert_eq!(**value_type, Type::Int, "dict value should be Int");
        }
        _ => panic!("data should be Dict type, got {:?}", field.field_type),
    }
}
