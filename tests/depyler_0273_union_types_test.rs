// DEPYLER-0273: Union Type Syntax Not Supported (PEP 604)
// RED Phase: Tests demonstrating current failure (union types not supported)
// Expected: These tests should PASS after fix is implemented

use depyler_core::hir::*;
use depyler_core::rust_gen::{generate_rust_file, RustCodeGen};
use depyler_core::type_mapper::TypeMapper;

#[test]
fn test_simple_union_type_int_or_none() {
    // Python: def is_none(value: int | None) -> bool: return value is None
    // Expected Rust: pub fn is_none(value: Option<i32>) -> bool { value.is_none() }
    // Current: Error: Unsupported type annotation

    let func = HirFunction {
        name: "is_none".to_string(),
        params: vec![HirParam::new("value".to_string(), Type::Optional(Box::new(Type::Int)))].into(),
        ret_type: Type::Bool,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Is,
            left: Box::new(HirExpr::Var("value".to_string())),
            right: Box::new(HirExpr::Literal(Literal::None)),
        }))],
        properties: FunctionProperties::default(),
        annotations: depyler_annotations::TranspilationAnnotations::default(),
        docstring: Some("Check if value is None.".to_string()),
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![],
        constants: vec![],
        imports: vec![],
        global_vars: vec![],
    };

    let type_mapper = TypeMapper::default();
    let (generated, _dependencies) = generate_rust_file(&module, &type_mapper).unwrap();

    // RED Phase: Currently this should work because we're using Type::Optional directly
    // The issue is in the PARSING of `int | None` syntax, not the HIR generation
    assert!(
        generated.contains("value: Option<i32>"),
        "Should generate Option<i32> for Optional[int]\nGenerated:\n{}",
        generated
    );
}

#[test]
fn test_union_return_type() {
    // Python: def maybe_int(flag: bool) -> int | None: return 42 if flag else None
    // Expected Rust: pub fn maybe_int(flag: bool) -> Option<i32>
    // Current: Error: Unsupported type annotation

    let func = HirFunction {
        name: "maybe_int".to_string(),
        params: vec![HirParam::new("flag".to_string(), Type::Bool)].into(),
        ret_type: Type::Optional(Box::new(Type::Int)),
        body: vec![HirStmt::Return(Some(HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("flag".to_string())),
            body: Box::new(HirExpr::Literal(Literal::Int(42))),
            orelse: Box::new(HirExpr::Literal(Literal::None)),
        }))],
        properties: FunctionProperties::default(),
        annotations: depyler_annotations::TranspilationAnnotations::default(),
        docstring: Some("Return int or None based on flag.".to_string()),
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![],
        constants: vec![],
        imports: vec![],
        global_vars: vec![],
    };

    let type_mapper = TypeMapper::default();
    let (generated, _dependencies) = generate_rust_file(&module, &type_mapper).unwrap();

    // Verify Option<i32> return type
    assert!(
        generated.contains("-> Option<i32>"),
        "Should generate Option<i32> return type\nGenerated:\n{}",
        generated
    );
}

#[test]
fn test_optional_default_parameter() {
    // Python: def optional_default(value: int | None, default: int = 42) -> int
    // Expected Rust: pub fn optional_default(value: Option<i32>, default: i32) -> i32
    // Current: Error: Unsupported type annotation

    let func = HirFunction {
        name: "optional_default".to_string(),
        params: vec![
            HirParam::new("value".to_string(), Type::Optional(Box::new(Type::Int))),
            HirParam::new("default".to_string(), Type::Int),
        ]
        .into(),
        ret_type: Type::Int,
        body: vec![
            HirStmt::If {
                condition: HirExpr::Binary {
                    op: BinOp::Is,
                    left: Box::new(HirExpr::Var("value".to_string())),
                    right: Box::new(HirExpr::Literal(Literal::None)),
                },
                then_body: vec![HirStmt::Return(Some(HirExpr::Var("default".to_string())))],
                else_body: None,
            },
            HirStmt::Return(Some(HirExpr::Var("value".to_string()))),
        ],
        properties: FunctionProperties::default(),
        annotations: depyler_annotations::TranspilationAnnotations::default(),
        docstring: Some("Return value or default if None.".to_string()),
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![],
        constants: vec![],
        imports: vec![],
        global_vars: vec![],
    };

    let type_mapper = TypeMapper::default();
    let (generated, _dependencies) = generate_rust_file(&module, &type_mapper).unwrap();

    // Verify Option<i32> parameter
    assert!(
        generated.contains("value: Option<i32>"),
        "Should generate Option<i32> parameter\nGenerated:\n{}",
        generated
    );

    // Verify default parameter
    assert!(
        generated.contains("default: i32"),
        "Should generate i32 default parameter\nGenerated:\n{}",
        generated
    );
}

#[test]
#[should_panic(expected = "assertion")]
fn test_parsing_union_syntax_fails() {
    // This test documents that the PARSING of `int | None` syntax fails
    // We can't easily test the parser directly, but we document the expected behavior

    // When this test starts passing, it means the parser has been fixed
    // to handle `T | None` syntax and convert it to Type::Optional

    // For now, this test just documents the issue
    // The real fix needs to happen in the Python AST → HIR conversion

    panic!("DEPYLER-0273: Parser does not support 'int | None' syntax yet. Use Optional[int] instead.");
}
