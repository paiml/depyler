//! Comprehensive tests for convert_stmt function (via apply_rules)
//! Following EXTREME TDD: Tests written BEFORE refactoring
//!
//! Note: convert_stmt is private, so we test through apply_rules
//! which calls convert_body which calls convert_stmt

use depyler_core::direct_rules::apply_rules;
use depyler_core::hir::*;
use depyler_core::type_mapper::TypeMapper;
use smallvec::smallvec;

/// Helper to create an empty HIR module
fn create_empty_module() -> HirModule {
    HirModule {
        imports: vec![],
        functions: vec![],
        classes: vec![],
        type_aliases: vec![],
        protocols: vec![],
        constants: vec![],
    }
}

/// Helper to create a simple function with given statements
fn create_function_with_body(name: &str, body: Vec<HirStmt>) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body,
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    }
}

// ============================================================================
// ASSIGNMENT TESTS - Symbol (Simple Variable Assignment)
// ============================================================================

#[test]
fn test_assign_symbol_simple() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_assign_symbol_complex_expr() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Assign {
            target: AssignTarget::Symbol("result".to_string()),
            value: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(1))),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            },
            type_annotation: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_assign_symbol_string() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Assign {
            target: AssignTarget::Symbol("name".to_string()),
            value: HirExpr::Literal(Literal::String("hello".to_string())),
            type_annotation: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// ASSIGNMENT TESTS - Index (Subscript Assignment)
// ============================================================================

#[test]
fn test_assign_index_simple() {
    let mut module = create_empty_module();

    // d[k] = v
    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
            },
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_assign_index_nested() {
    let mut module = create_empty_module();

    // d[k1][k2] = v (represented as nested Index)
    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Index {
                    base: Box::new(HirExpr::Var("d".to_string())),
                    index: Box::new(HirExpr::Literal(Literal::String("k1".to_string()))),
                }),
                index: Box::new(HirExpr::Literal(Literal::String("k2".to_string()))),
            },
            value: HirExpr::Literal(Literal::Int(99)),
            type_annotation: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_assign_index_complex_value() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            value: HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(HirExpr::Literal(Literal::Int(10))),
                right: Box::new(HirExpr::Literal(Literal::Int(20))),
            },
            type_annotation: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// ASSIGNMENT TESTS - Attribute (Object Attribute Assignment)
// ============================================================================

#[test]
fn test_assign_attribute_simple() {
    let mut module = create_empty_module();

    // obj.field = value
    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Assign {
            target: AssignTarget::Attribute {
                value: Box::new(HirExpr::Var("obj".to_string())),
                attr: "field".to_string(),
            },
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_assign_attribute_nested() {
    let mut module = create_empty_module();

    // obj.nested.field = value
    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Assign {
            target: AssignTarget::Attribute {
                value: Box::new(HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("obj".to_string())),
                    attr: "nested".to_string(),
                }),
                attr: "field".to_string(),
            },
            value: HirExpr::Literal(Literal::String("data".to_string())),
            type_annotation: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// RETURN STATEMENT TESTS
// ============================================================================

#[test]
fn test_return_with_value() {
    let mut module = create_empty_module();

    module.functions.push(HirFunction {
        name: "test".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    });

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_return_without_value() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Return(None)],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_return_complex_expr() {
    let mut module = create_empty_module();

    module.functions.push(HirFunction {
        name: "test".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        }))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    });

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// IF STATEMENT TESTS
// ============================================================================

#[test]
fn test_if_without_else() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_if_with_else() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                2,
            ))))]),
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_if_complex_condition() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(10))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Bool(true))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(
                Literal::Bool(false),
            )))]),
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// WHILE LOOP TESTS
// ============================================================================

#[test]
fn test_while_simple() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Break { label: None }],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_while_complex_condition() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::While {
            condition: HirExpr::Binary {
                op: BinOp::Lt,
                left: Box::new(HirExpr::Var("i".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(10))),
            },
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("i".to_string()),
                value: HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("i".to_string())),
                    right: Box::new(HirExpr::Literal(Literal::Int(1))),
                },
                type_annotation: None,
            }],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// FOR LOOP TESTS
// ============================================================================

#[test]
fn test_for_simple() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Var("i".to_string()))],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_for_with_assignment() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::For {
            target: AssignTarget::Symbol("x".to_string()),
            iter: HirExpr::Var("numbers".to_string()),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("sum".to_string()),
                value: HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("sum".to_string())),
                    right: Box::new(HirExpr::Var("x".to_string())),
                },
                type_annotation: None,
            }],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// EXPRESSION STATEMENT TESTS
// ============================================================================

#[test]
fn test_expr_stmt_simple() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(42)))],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_expr_stmt_function_call() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
            kwargs: vec![],
        })],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// RAISE (EXCEPTION) TESTS
// ============================================================================

#[test]
fn test_raise_with_exception() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Raise {
            exception: Some(HirExpr::Literal(Literal::String(
                "Error occurred".to_string(),
            ))),
            cause: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_raise_without_exception() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::Raise {
            exception: None,
            cause: None,
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// BREAK STATEMENT TESTS
// ============================================================================

#[test]
fn test_break_without_label() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Break { label: None }],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_break_with_label() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Break {
                label: Some("outer".to_string()),
            }],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// CONTINUE STATEMENT TESTS
// ============================================================================

#[test]
fn test_continue_without_label() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Continue { label: None }],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_continue_with_label() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Continue {
                label: Some("outer".to_string()),
            }],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// WITH (CONTEXT MANAGER) TESTS
// ============================================================================

#[test]
fn test_with_no_target() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::With {
            context: HirExpr::Var("file".to_string()),
            target: None,
            body: vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(1)))],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_with_target() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::With {
            context: HirExpr::Var("file".to_string()),
            target: Some("f".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Var("f".to_string()))],
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// ============================================================================
// INTEGRATION TESTS (Multiple Statement Types)
// ============================================================================

#[test]
fn test_multiple_statements() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(10)),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Literal(Literal::Int(20)),
                type_annotation: None,
            },
            HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Var("y".to_string())),
            })),
        ],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_nested_control_flow() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(false)),
                body: vec![HirStmt::Break { label: None }],
            }],
            else_body: Some(vec![HirStmt::Return(None)]),
        }],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_complex_assignment_sequence() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![
            // Symbol assignment
            HirStmt::Assign {
                target: AssignTarget::Symbol("data".to_string()),
                value: HirExpr::Literal(Literal::Int(100)),
                type_annotation: None,
            },
            // Index assignment
            HirStmt::Assign {
                target: AssignTarget::Index {
                    base: Box::new(HirExpr::Var("d".to_string())),
                    index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
                },
                value: HirExpr::Var("data".to_string()),
                type_annotation: None,
            },
            // Attribute assignment
            HirStmt::Assign {
                target: AssignTarget::Attribute {
                    value: Box::new(HirExpr::Var("obj".to_string())),
                    attr: "value".to_string(),
                },
                value: HirExpr::Var("data".to_string()),
                type_annotation: None,
            },
        ],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_all_statement_types() {
    let mut module = create_empty_module();

    module.functions.push(create_function_with_body(
        "test",
        vec![
            // Assignment
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            // If
            HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![
                    // While
                    HirStmt::While {
                        condition: HirExpr::Literal(Literal::Bool(true)),
                        body: vec![HirStmt::Break { label: None }],
                    },
                ],
                else_body: None,
            },
            // For
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Var("items".to_string()),
                body: vec![HirStmt::Continue { label: None }],
            },
            // Expression statement
            HirStmt::Expr(HirExpr::Literal(Literal::Int(42))),
            // Return
            HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
        ],
    ));

    let type_mapper = TypeMapper::new();
    let result = apply_rules(&module, &type_mapper);
    assert!(result.is_ok());
}

// Test count: 30 comprehensive tests covering all statement types ✅
