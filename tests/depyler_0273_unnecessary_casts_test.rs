// DEPYLER-0272: Unnecessary Type Casts in Generated Rust Code
// RED Phase: Tests demonstrating current behavior (unnecessary casts)
// Expected: These tests should PASS after fix is implemented

use depyler_core::hir::*;
use depyler_core::rust_gen::{generate_rust_file, RustCodeGen};
use depyler_core::type_mapper::TypeMapper;

#[test]
fn test_simple_variable_return_should_not_cast() {
    // Python: def identity(x: int) -> int: return x
    // Expected Rust: pub fn identity(x: i32) -> i32 { x }
    // Current (WRONG): pub fn identity(x: i32) -> i32 { x as i32 }

    let func = HirFunction {
        name: "identity".to_string(),
        params: vec![HirParam::new("x".to_string(), Type::Int)].into(),
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
        properties: FunctionProperties::default(),
        annotations: depyler_annotations::TranspilationAnnotations::default(),
        docstring: None,
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
        global_vars: vec![],
    };

    let type_mapper = TypeMapper::default();
    let generated = generate_rust_file(&module, &type_mapper).unwrap();

    // RED Phase: Currently this FAILS because code contains "x as i32"
    // GREEN Phase: After fix, should PASS (no cast)
    assert!(
        !generated.contains("x as i32"),
        "Should NOT cast i32 variable to i32\nGenerated:\n{}",
        generated
    );
}

#[test]
fn test_binary_expression_return_should_not_cast() {
    // Python: def add(a: int, b: int) -> int: return a + b
    // Expected Rust: pub fn add(a: i32, b: i32) -> i32 { a + b }
    // Current (WRONG): pub fn add(a: i32, b: i32) -> i32 { a + b as i32 }

    let func = HirFunction {
        name: "add".to_string(),
        params: vec![
            HirParam::new("a".to_string(), Type::Int),
            HirParam::new("b".to_string(), Type::Int),
        ]
        .into(),
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Var("b".to_string())),
        }))],
        properties: FunctionProperties::default(),
        annotations: depyler_annotations::TranspilationAnnotations::default(),
        docstring: None,
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
        global_vars: vec![],
    };

    let type_mapper = TypeMapper::default();
    let generated = generate_rust_file(&module, &type_mapper).unwrap();

    // RED Phase: Currently this FAILS because code contains cast
    // GREEN Phase: After fix, should PASS (no cast on result)
    // Note: Individual operands might have casts in binary expressions
    // but the final result shouldn't need one
    assert!(
        !generated.contains("+ b as i32") && !generated.contains("+ (b as i32)"),
        "Should NOT cast i32 operands in binary expression\nGenerated:\n{}",
        generated
    );
}

#[test]
fn test_array_length_should_cast() {
    // Python: def array_len(arr: list[int]) -> int: return len(arr)
    // Expected Rust: pub fn array_len(arr: &[i32]) -> i32 { arr.len() as i32 }
    // Current: pub fn array_len(arr: &[i32]) -> i32 { arr.len() as i32 } ✅

    let func = HirFunction {
        name: "array_len".to_string(),
        params: vec![HirParam::new("arr".to_string(), Type::List(Box::new(Type::Int)))].into(),
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::MethodCall { object: Box::new(HirExpr::Var("arr".to_string())), method: "len".to_string(), args: vec![], kwargs: vec![] }))],
        properties: FunctionProperties::default(),
        annotations: depyler_annotations::TranspilationAnnotations::default(),
        docstring: None,
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
        global_vars: vec![],
    };

    let type_mapper = TypeMapper::default();
    let generated = generate_rust_file(&module, &type_mapper).unwrap();

    // This should ALWAYS pass - len() returns usize, needs cast
    assert!(
        generated.contains("len () as i32") || generated.contains("len() as i32"),
        "SHOULD cast usize from len() to i32\nGenerated:\n{}",
        generated
    );
}

#[test]
fn test_conditional_return_should_not_cast() {
    // Python:
    // def max_value(a: int, b: int) -> int:
    //     if a > b:
    //         return a
    //     return b
    //
    // Expected Rust:
    // pub fn max_value(a: i32, b: i32) -> i32 {
    //     if a > b {
    //         return a;
    //     }
    //     b
    // }
    //
    // Current (WRONG):
    // pub fn max_value(a: i32, b: i32) -> i32 {
    //     if a > b {
    //         return a as i32;  // ❌ Unnecessary
    //     }
    //     b as i32  // ❌ Unnecessary
    // }

    let func = HirFunction {
        name: "max_value".to_string(),
        params: vec![
            HirParam::new("a".to_string(), Type::Int),
            HirParam::new("b".to_string(), Type::Int),
        ]
        .into(),
        ret_type: Type::Int,
        body: vec![
            HirStmt::If {
                condition: HirExpr::Binary {
                    op: BinOp::Gt,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                },
                then_body: vec![HirStmt::Return(Some(HirExpr::Var("a".to_string())))],
                else_body: None,
            },
            HirStmt::Return(Some(HirExpr::Var("b".to_string()))),
        ],
        properties: FunctionProperties::default(),
        annotations: depyler_annotations::TranspilationAnnotations::default(),
        docstring: None,
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
        global_vars: vec![],
    };

    let type_mapper = TypeMapper::default();
    let generated = generate_rust_file(&module, &type_mapper).unwrap();

    // RED Phase: Currently FAILS because both returns have casts
    // GREEN Phase: After fix, should PASS (no casts)
    assert!(
        !generated.contains("a as i32") && !generated.contains("b as i32"),
        "Should NOT cast i32 variables in conditional returns\nGenerated:\n{}",
        generated
    );
}

#[test]
fn test_literal_return_should_not_cast() {
    // Python: def get_constant() -> int: return 42
    // Expected Rust: pub fn get_constant() -> i32 { 42 }
    // Current (WRONG): pub fn get_constant() -> i32 { 42 as i32 }

    let func = HirFunction {
        name: "get_constant".to_string(),
        params: vec![].into(),
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        properties: FunctionProperties::default(),
        annotations: depyler_annotations::TranspilationAnnotations::default(),
        docstring: None,
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
        global_vars: vec![],
    };

    let type_mapper = TypeMapper::default();
    let generated = generate_rust_file(&module, &type_mapper).unwrap();

    // RED Phase: Currently FAILS because literal has cast
    // GREEN Phase: After fix, should PASS (no cast)
    assert!(
        !generated.contains("42 as i32"),
        "Should NOT cast integer literal to i32\nGenerated:\n{}",
        generated
    );
}

#[test]
fn test_count_method_should_cast() {
    // Python: def count_items(items: list[int], value: int) -> int: return items.count(value)
    // Expected Rust: pub fn count_items(...) -> i32 { items.iter().filter(...).count() as i32 }
    // Current: Should have cast ✅

    let func = HirFunction {
        name: "count_items".to_string(),
        params: vec![
            HirParam::new("items".to_string(), Type::List(Box::new(Type::Int))),
            HirParam::new("value".to_string(), Type::Int),
        ]
        .into(),
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::MethodCall { object: Box::new(HirExpr::Var("items".to_string())), method: "count".to_string(), args: vec![HirExpr::Var("value".to_string())], kwargs: vec![] }))],
        properties: FunctionProperties::default(),
        annotations: depyler_annotations::TranspilationAnnotations::default(),
        docstring: None,
    };

    let module = HirModule {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
        global_vars: vec![],
    };

    let type_mapper = TypeMapper::default();
    let generated = generate_rust_file(&module, &type_mapper).unwrap();

    // This should ALWAYS pass - count() returns usize, needs cast
    assert!(
        generated.contains("count () as i32") || generated.contains("count() as i32"),
        "SHOULD cast usize from count() to i32\nGenerated:\n{}",
        generated
    );
}
