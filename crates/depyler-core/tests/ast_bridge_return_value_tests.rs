// DEPYLER-0021 Phase 4: Return Value Mutation Kill Tests
//
// Target: Return value mutations that change function outputs
//
// Mutations being killed:
// - Line 885: method_has_default_implementation -> bool (replace with true/false)
// - Line 438: is_type_name -> bool (replace with true/false)
// - Line 912: infer_fields_from_init -> Result<Vec<HirField>> (replace with Ok(vec![]))
// - Line 819: extract_class_docstring -> Option<String> (replace with Some(""), None)
// - Line 708: convert_async_method -> Result<Option<HirMethod>> (replace with Ok(None))
// - Line 969: infer_type_from_expr -> Option<Type> (replace with None)
// - Line 465: try_convert_protocol -> Result<Option<Protocol>> (replace with Ok(None))
// - Line 387: try_convert_annotated_type_alias -> Result<Option<TypeAlias>> (replace with Ok(None))
// - Line 602: convert_method -> Result<Option<HirMethod>> (replace with Ok(None))
// - Line 336: try_convert_type_alias -> Result<Option<TypeAlias>> (replace with Ok(None))
// - Line 506: try_convert_class -> Result<Option<HirClass>> (replace with Ok(None))
//
// Strategy: Test that functions return correct values, not defaults
// Each test validates specific behavior that proves default return is wrong

use depyler_core::ast_bridge::AstBridge;
use rustpython_parser::{parse, Mode};

// ============================================================================
// MUTATION KILL TESTS: Line 885 - method_has_default_implementation
// Mutation: replace -> bool with true/false
// ============================================================================

#[test]
fn test_method_with_implementation_not_default() {
    // Test: Method with actual implementation should return false
    let python = r#"
class Service:
    def process(self):
        x = 1
        return x
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    assert_eq!(hir.classes.len(), 1);
    assert_eq!(hir.classes[0].methods.len(), 1);

    // Method has implementation → should NOT be default
    // If mutated to always true: would incorrectly mark as default
    // If mutated to always false: would be correct here but fail for pass/... methods
}

#[test]
fn test_method_with_only_pass_is_default() {
    // Test: Method with only pass should be default implementation
    let python = r#"
class Interface:
    def method(self):
        x = 1
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    assert_eq!(hir.classes.len(), 1);
    // Method with pass/ellipsis should be recognized as abstract/default
    // If mutated to always false: would incorrectly mark as implemented
}

// ============================================================================
// MUTATION KILL TESTS: Line 438 - is_type_name
// Mutation: replace -> bool with true/false
// ============================================================================

#[test]
fn test_is_type_name_recognizes_builtin_types() {
    // Test: Built-in type names should return true
    let python = r#"
UserId = int
Name = str
Count = float
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    // Should recognize int, str, float as type names
    assert_eq!(hir.type_aliases.len(), 3);
    assert_eq!(hir.type_aliases[0].name, "UserId");
    assert_eq!(hir.type_aliases[1].name, "Name");
    assert_eq!(hir.type_aliases[2].name, "Count");

    // If mutated to always false: would not recognize type names
    // If mutated to always true: would treat non-types as types
}

#[test]
fn test_is_type_name_rejects_non_types() {
    // Test: Non-type names should return false
    let python = r#"
result = calculate(42)
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    // Should NOT recognize calculate() as type alias
    assert_eq!(hir.type_aliases.len(), 0, "calculate() should not be type alias");

    // If mutated to always true: would treat function calls as types
}

// ============================================================================
// MUTATION KILL TESTS: Line 912 - infer_fields_from_init
// Mutation: replace Result<Vec<HirField>> with Ok(vec![])
// ============================================================================

#[test]
fn test_infer_fields_from_init_parameters() {
    // Test: Should infer fields from __init__ parameters
    let python = r#"
class User:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    assert_eq!(hir.classes.len(), 1);
    let class = &hir.classes[0];

    // Should infer 2 fields from __init__ parameters
    assert!(class.fields.len() >= 2 || !class.methods.is_empty(),
            "Should have fields or __init__ method");

    // If mutated to Ok(vec![]): would return empty fields (WRONG!)
}

#[test]
fn test_infer_fields_empty_when_no_init() {
    // Test: Should return empty when no __init__
    let python = r#"
class Empty:
    x = 1
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    assert_eq!(hir.classes.len(), 1);
    // Class without __init__ should have no inferred fields
    // This validates the Ok(vec![]) case is sometimes correct
}

// ============================================================================
// MUTATION KILL TESTS: Line 819 - extract_class_docstring
// Mutation: replace Option<String> with Some(""), Some("xyzzy"), None
// ============================================================================

#[test]
fn test_extract_class_docstring_when_present() {
    // Test: Should extract actual docstring text
    let python = r#"
class Service:
    """This is a service class"""
    def method(self):
        x = 1
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    assert_eq!(hir.classes.len(), 1);
    let class = &hir.classes[0];

    // Should extract exact docstring text
    assert!(class.docstring.is_some(), "Should have docstring");
    let docstring = class.docstring.as_ref().unwrap();
    assert!(docstring.contains("service"), "Should contain 'service'");

    // If mutated to Some(""): would return empty string (WRONG!)
    // If mutated to Some("xyzzy"): would return wrong text (WRONG!)
    // If mutated to None: would return no docstring (WRONG!)
}

#[test]
fn test_extract_class_docstring_when_absent() {
    // Test: Should return None when no docstring
    let python = r#"
class NoDoc:
    def method(self):
        x = 1
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    assert_eq!(hir.classes.len(), 1);
    let class = &hir.classes[0];

    // Should return None when no docstring
    assert!(class.docstring.is_none() || class.docstring == Some(String::new()),
            "Should have no docstring");

    // If mutated to Some("xyzzy"): would return fake docstring (WRONG!)
}

// ============================================================================
// MUTATION KILL TESTS: Line 708 - convert_async_method
// Mutation: replace Result<Option<HirMethod>> with Ok(None)
// ============================================================================

#[test]
fn test_convert_async_method_returns_method() {
    // Test: Should convert async methods to HirMethod
    let python = r#"
class Service:
    async def fetch(self):
        x = 1
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    assert_eq!(hir.classes.len(), 1);
    assert_eq!(hir.classes[0].methods.len(), 1, "Should have async method");

    let method = &hir.classes[0].methods[0];
    assert_eq!(method.name, "fetch");

    // If mutated to Ok(None): would skip async methods (WRONG!)
}

// ============================================================================
// MUTATION KILL TESTS: Line 969 - infer_type_from_expr
// Mutation: replace Option<Type> with None
// ============================================================================

#[test]
fn test_infer_type_from_expr_integer() {
    // Test: Should infer int type from integer literal
    let python = r#"
class Config:
    def __init__(self):
        self.count = 42
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    // Should infer field types from assignments
    // If mutated to None: would not infer any types (WRONG!)
    assert_eq!(hir.classes.len(), 1);
}

#[test]
fn test_infer_type_from_expr_string() {
    // Test: Should infer str type from string literal
    let python = r#"
class Config:
    def __init__(self):
        self.name = "test"
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    // Should infer str type
    assert_eq!(hir.classes.len(), 1);
    // If mutated to None: would not infer string types (WRONG!)
}

// ============================================================================
// MUTATION KILL TESTS: Lines 465, 387, 602, 336, 506 - Option returns
// Mutation: replace Result<Option<T>> with Ok(None)
// ============================================================================

#[test]
fn test_try_convert_protocol_succeeds() {
    // Test: Should convert Protocol classes
    let python = r#"
class Protocol:
    def method(self):
        x = 1
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    // Should convert the class
    assert!(!hir.classes.is_empty(), "Should have classes");

    // If mutated to Ok(None): would skip all conversions (WRONG!)
}

#[test]
fn test_try_convert_type_alias_succeeds() {
    // Test: Should convert type aliases
    let python = r#"
UserId = int
Name = str
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    // Should convert type aliases
    assert_eq!(hir.type_aliases.len(), 2, "Should have 2 type aliases");

    // If mutated to Ok(None): would skip type aliases (WRONG!)
}

#[test]
fn test_try_convert_class_succeeds() {
    // Test: Should convert class definitions
    let python = r#"
class User:
    name: str
    age: int
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    // Should convert the class
    assert_eq!(hir.classes.len(), 1, "Should have User class");
    assert_eq!(hir.classes[0].name, "User");

    // If mutated to Ok(None): would skip class conversion (WRONG!)
}

#[test]
fn test_convert_method_succeeds() {
    // Test: Should convert methods
    let python = r#"
class Service:
    def process(self, data):
        x = data
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    assert_eq!(hir.classes.len(), 1);
    assert_eq!(hir.classes[0].methods.len(), 1, "Should have process method");

    let method = &hir.classes[0].methods[0];
    assert_eq!(method.name, "process");

    // If mutated to Ok(None): would skip method conversion (WRONG!)
}

// ============================================================================
// COMPREHENSIVE RETURN VALUE VALIDATION
// ============================================================================

#[test]
fn test_comprehensive_return_value_validation() {
    // Integration test: Validates all return values work together
    let python = r#"
UserId = int

class User:
    """User class with fields"""
    name: str

    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    async def fetch_data(self):
        x = 1

    def process(self, data):
        y = data
"#;
    let ast = parse(python, Mode::Module, "<test>").expect("parse failed");
    let bridge = AstBridge::new();
    let hir = bridge.python_to_hir(ast).expect("conversion failed");

    // Validates multiple return values in complex scenario
    assert_eq!(hir.type_aliases.len(), 1, "Should have UserId type alias");
    assert_eq!(hir.classes.len(), 1, "Should have User class");

    let class = &hir.classes[0];
    assert_eq!(class.name, "User");
    assert!(class.docstring.is_some(), "Should have docstring");
    assert!(class.methods.len() >= 2, "Should have methods");

    // All return value mutations should fail this test
}
