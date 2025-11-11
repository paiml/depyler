//! DEPYLER-0354: simplified_hir.rs Coverage Tests
//!
//! **EXTREME TDD Protocol - Coverage Boost**
//!
//! Target: simplified_hir.rs 0% → 85%+ coverage
//! TDG Score: Excellent (A+) - Clean data structures with derive macros
//!
//! This test suite validates simplified HIR data structures:
//! - Hir struct construction and access
//! - HirMetadata with Default trait
//! - All 16 HirExpr variants
//! - HirStatement variants
//! - HirParam construction
//! - All 5 HirLiteral types
//! - All 15 HirBinaryOp variants
//! - All 3 HirUnaryOp variants
//! - All 8 HirType variants
//! - Clone, PartialEq, Debug traits
//! - Serde serialization/deserialization
//! - Nested and recursive structures

use depyler_core::simplified_hir::*;

// ============================================================================
// HIR STRUCT TESTS
// ============================================================================

#[test]
fn test_depyler_0354_hir_struct_basic_construction() {
    let metadata = HirMetadata {
        source_file: Some("test.py".to_string()),
        module_name: Some("test_module".to_string()),
    };

    let hir = Hir {
        root: HirExpr::Literal(HirLiteral::Integer(42)),
        metadata,
    };

    assert_eq!(hir.metadata.source_file, Some("test.py".to_string()));
    assert_eq!(hir.metadata.module_name, Some("test_module".to_string()));
}

#[test]
fn test_depyler_0354_hir_struct_clone() {
    let hir1 = Hir {
        root: HirExpr::Literal(HirLiteral::Bool(true)),
        metadata: HirMetadata::default(),
    };

    let hir2 = hir1.clone();
    assert_eq!(hir1, hir2);
}

#[test]
fn test_depyler_0354_hir_struct_debug_format() {
    let hir = Hir {
        root: HirExpr::Identifier("x".to_string()),
        metadata: HirMetadata::default(),
    };

    let debug = format!("{:?}", hir);
    assert!(debug.contains("Hir"));
    assert!(debug.contains("Identifier"));
}

#[test]
fn test_depyler_0354_hir_struct_serde_roundtrip() {
    let hir = Hir {
        root: HirExpr::Literal(HirLiteral::String("hello".to_string())),
        metadata: HirMetadata {
            source_file: Some("main.py".to_string()),
            module_name: None,
        },
    };

    let json = serde_json::to_string(&hir).unwrap();
    let deserialized: Hir = serde_json::from_str(&json).unwrap();
    assert_eq!(hir, deserialized);
}

// ============================================================================
// HIR METADATA TESTS
// ============================================================================

#[test]
fn test_depyler_0354_metadata_default() {
    let metadata = HirMetadata::default();
    assert_eq!(metadata.source_file, None);
    assert_eq!(metadata.module_name, None);
}

#[test]
fn test_depyler_0354_metadata_with_source_file_only() {
    let metadata = HirMetadata {
        source_file: Some("script.py".to_string()),
        module_name: None,
    };

    assert_eq!(metadata.source_file, Some("script.py".to_string()));
    assert_eq!(metadata.module_name, None);
}

#[test]
fn test_depyler_0354_metadata_with_module_name_only() {
    let metadata = HirMetadata {
        source_file: None,
        module_name: Some("my_module".to_string()),
    };

    assert_eq!(metadata.source_file, None);
    assert_eq!(metadata.module_name, Some("my_module".to_string()));
}

#[test]
fn test_depyler_0354_metadata_clone_and_equality() {
    let meta1 = HirMetadata {
        source_file: Some("a.py".to_string()),
        module_name: Some("a".to_string()),
    };

    let meta2 = meta1.clone();
    assert_eq!(meta1, meta2);
}

// ============================================================================
// HIR LITERAL TESTS
// ============================================================================

#[test]
fn test_depyler_0354_literal_integer() {
    let lit = HirLiteral::Integer(100);
    assert_eq!(lit, HirLiteral::Integer(100));
    assert_ne!(lit, HirLiteral::Integer(200));
}

#[test]
fn test_depyler_0354_literal_float() {
    let lit = HirLiteral::Float(std::f64::consts::PI);
    let lit2 = lit.clone();
    assert_eq!(lit, lit2);
}

#[test]
fn test_depyler_0354_literal_string() {
    let lit = HirLiteral::String("test".to_string());
    assert_eq!(lit, HirLiteral::String("test".to_string()));
}

#[test]
fn test_depyler_0354_literal_bool() {
    let lit_true = HirLiteral::Bool(true);
    let lit_false = HirLiteral::Bool(false);
    assert_ne!(lit_true, lit_false);
}

#[test]
fn test_depyler_0354_literal_none() {
    let lit = HirLiteral::None;
    assert_eq!(lit, HirLiteral::None);
}

#[test]
fn test_depyler_0354_literal_debug_format() {
    let lit = HirLiteral::Integer(42);
    let debug = format!("{:?}", lit);
    assert!(debug.contains("Integer"));
    assert!(debug.contains("42"));
}

// ============================================================================
// HIR BINARY OP TESTS
// ============================================================================

#[test]
fn test_depyler_0354_binary_op_arithmetic() {
    let ops = vec![
        HirBinaryOp::Add,
        HirBinaryOp::Subtract,
        HirBinaryOp::Multiply,
        HirBinaryOp::Divide,
        HirBinaryOp::Modulo,
        HirBinaryOp::Power,
    ];

    for op in ops {
        let cloned = op;
        assert_eq!(op, cloned);
    }
}

#[test]
fn test_depyler_0354_binary_op_comparison() {
    let ops = vec![
        HirBinaryOp::Equal,
        HirBinaryOp::NotEqual,
        HirBinaryOp::Less,
        HirBinaryOp::LessEqual,
        HirBinaryOp::Greater,
        HirBinaryOp::GreaterEqual,
    ];

    for op in ops {
        let debug = format!("{:?}", op);
        assert!(!debug.is_empty());
    }
}

#[test]
fn test_depyler_0354_binary_op_logical() {
    let and_op = HirBinaryOp::And;
    let or_op = HirBinaryOp::Or;

    assert_ne!(and_op, or_op);
    assert_eq!(and_op, HirBinaryOp::And);
}

#[test]
fn test_depyler_0354_binary_op_bitwise() {
    let ops = vec![
        HirBinaryOp::BitwiseAnd,
        HirBinaryOp::BitwiseOr,
        HirBinaryOp::BitwiseXor,
        HirBinaryOp::LeftShift,
        HirBinaryOp::RightShift,
    ];

    for op in ops {
        let _serialized = serde_json::to_string(&op).unwrap();
    }
}

// ============================================================================
// HIR UNARY OP TESTS
// ============================================================================

#[test]
fn test_depyler_0354_unary_op_not() {
    let op = HirUnaryOp::Not;
    assert_eq!(op, HirUnaryOp::Not);
}

#[test]
fn test_depyler_0354_unary_op_negate() {
    let op = HirUnaryOp::Negate;
    assert_eq!(op, HirUnaryOp::Negate);
}

#[test]
fn test_depyler_0354_unary_op_bitwise_not() {
    let op = HirUnaryOp::BitwiseNot;
    let cloned = op;
    assert_eq!(op, cloned);
}

#[test]
fn test_depyler_0354_unary_op_all_variants_distinct() {
    let ops = vec![
        HirUnaryOp::Not,
        HirUnaryOp::Negate,
        HirUnaryOp::BitwiseNot,
    ];

    for (i, op1) in ops.iter().enumerate() {
        for (j, op2) in ops.iter().enumerate() {
            if i == j {
                assert_eq!(op1, op2);
            } else {
                assert_ne!(op1, op2);
            }
        }
    }
}

// ============================================================================
// HIR TYPE TESTS
// ============================================================================

#[test]
fn test_depyler_0354_type_primitives() {
    let types = vec![
        HirType::Int,
        HirType::Float,
        HirType::String,
        HirType::Bool,
        HirType::Any,
    ];

    for typ in types {
        let cloned = typ.clone();
        assert_eq!(typ, cloned);
    }
}

#[test]
fn test_depyler_0354_type_list() {
    let list_type = HirType::List(Box::new(HirType::Int));
    assert_eq!(list_type, HirType::List(Box::new(HirType::Int)));
}

#[test]
fn test_depyler_0354_type_optional() {
    let opt_type = HirType::Optional(Box::new(HirType::String));
    let cloned = opt_type.clone();
    assert_eq!(opt_type, cloned);
}

#[test]
fn test_depyler_0354_type_named() {
    let named_type = HirType::Named("CustomType".to_string());
    assert_eq!(named_type, HirType::Named("CustomType".to_string()));
}

#[test]
fn test_depyler_0354_type_nested_list() {
    let nested = HirType::List(Box::new(HirType::List(Box::new(HirType::Int))));
    let debug = format!("{:?}", nested);
    assert!(debug.contains("List"));
}

#[test]
fn test_depyler_0354_type_optional_list() {
    let typ = HirType::Optional(Box::new(HirType::List(Box::new(HirType::Bool))));
    let _serialized = serde_json::to_string(&typ).unwrap();
}

// ============================================================================
// HIR PARAM TESTS
// ============================================================================

#[test]
fn test_depyler_0354_param_name_only() {
    let param = HirParam {
        name: "x".to_string(),
        typ: None,
        default: None,
    };

    assert_eq!(param.name, "x");
    assert_eq!(param.typ, None);
    assert_eq!(param.default, None);
}

#[test]
fn test_depyler_0354_param_with_type() {
    let param = HirParam {
        name: "y".to_string(),
        typ: Some(HirType::Int),
        default: None,
    };

    assert_eq!(param.typ, Some(HirType::Int));
}

#[test]
fn test_depyler_0354_param_with_default() {
    let param = HirParam {
        name: "z".to_string(),
        typ: Some(HirType::Int),
        default: Some(Box::new(HirExpr::Literal(HirLiteral::Integer(42)))),
    };

    assert!(param.default.is_some());
}

#[test]
fn test_depyler_0354_param_clone_equality() {
    let param1 = HirParam {
        name: "param".to_string(),
        typ: Some(HirType::String),
        default: Some(Box::new(HirExpr::Literal(HirLiteral::String("default".to_string())))),
    };

    let param2 = param1.clone();
    assert_eq!(param1, param2);
}

// ============================================================================
// HIR EXPRESSION TESTS - SIMPLE VARIANTS
// ============================================================================

#[test]
fn test_depyler_0354_expr_literal() {
    let expr = HirExpr::Literal(HirLiteral::Integer(10));
    assert_eq!(expr, HirExpr::Literal(HirLiteral::Integer(10)));
}

#[test]
fn test_depyler_0354_expr_identifier() {
    let expr = HirExpr::Identifier("variable".to_string());
    let cloned = expr.clone();
    assert_eq!(expr, cloned);
}

#[test]
fn test_depyler_0354_expr_list_empty() {
    let expr = HirExpr::List(vec![]);
    assert_eq!(expr, HirExpr::List(vec![]));
}

#[test]
fn test_depyler_0354_expr_list_with_elements() {
    let expr = HirExpr::List(vec![
        HirExpr::Literal(HirLiteral::Integer(1)),
        HirExpr::Literal(HirLiteral::Integer(2)),
        HirExpr::Literal(HirLiteral::Integer(3)),
    ]);

    let debug = format!("{:?}", expr);
    assert!(debug.contains("List"));
}

#[test]
fn test_depyler_0354_expr_return_some() {
    let expr = HirExpr::Return(Some(Box::new(HirExpr::Literal(HirLiteral::Integer(0)))));
    let cloned = expr.clone();
    assert_eq!(expr, cloned);
}

#[test]
fn test_depyler_0354_expr_return_none() {
    let expr = HirExpr::Return(None);
    assert_eq!(expr, HirExpr::Return(None));
}

#[test]
fn test_depyler_0354_expr_break_with_label() {
    let expr = HirExpr::Break(Some("outer".to_string()));
    assert_eq!(expr, HirExpr::Break(Some("outer".to_string())));
}

#[test]
fn test_depyler_0354_expr_break_without_label() {
    let expr = HirExpr::Break(None);
    assert_eq!(expr, HirExpr::Break(None));
}

#[test]
fn test_depyler_0354_expr_continue_with_label() {
    let expr = HirExpr::Continue(Some("loop".to_string()));
    let cloned = expr.clone();
    assert_eq!(expr, cloned);
}

#[test]
fn test_depyler_0354_expr_continue_without_label() {
    let expr = HirExpr::Continue(None);
    assert_eq!(expr, HirExpr::Continue(None));
}

#[test]
fn test_depyler_0354_expr_await() {
    let expr = HirExpr::Await(Box::new(HirExpr::Identifier("future".to_string())));
    let debug = format!("{:?}", expr);
    assert!(debug.contains("Await"));
}

// ============================================================================
// HIR EXPRESSION TESTS - BINARY
// ============================================================================

#[test]
fn test_depyler_0354_expr_binary_add() {
    let expr = HirExpr::Binary {
        left: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
        op: HirBinaryOp::Add,
        right: Box::new(HirExpr::Literal(HirLiteral::Integer(2))),
    };

    let cloned = expr.clone();
    assert_eq!(expr, cloned);
}

#[test]
fn test_depyler_0354_expr_binary_comparison() {
    let expr = HirExpr::Binary {
        left: Box::new(HirExpr::Identifier("x".to_string())),
        op: HirBinaryOp::Less,
        right: Box::new(HirExpr::Literal(HirLiteral::Integer(10))),
    };

    let debug = format!("{:?}", expr);
    assert!(debug.contains("Binary"));
}

#[test]
fn test_depyler_0354_expr_binary_nested() {
    let inner = HirExpr::Binary {
        left: Box::new(HirExpr::Literal(HirLiteral::Integer(2))),
        op: HirBinaryOp::Multiply,
        right: Box::new(HirExpr::Literal(HirLiteral::Integer(3))),
    };

    let outer = HirExpr::Binary {
        left: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
        op: HirBinaryOp::Add,
        right: Box::new(inner),
    };

    let _serialized = serde_json::to_string(&outer).unwrap();
}

// ============================================================================
// HIR EXPRESSION TESTS - UNARY
// ============================================================================

#[test]
fn test_depyler_0354_expr_unary_not() {
    let expr = HirExpr::Unary {
        op: HirUnaryOp::Not,
        operand: Box::new(HirExpr::Literal(HirLiteral::Bool(true))),
    };

    assert_eq!(expr.clone(), expr);
}

#[test]
fn test_depyler_0354_expr_unary_negate() {
    let expr = HirExpr::Unary {
        op: HirUnaryOp::Negate,
        operand: Box::new(HirExpr::Literal(HirLiteral::Integer(5))),
    };

    let debug = format!("{:?}", expr);
    assert!(debug.contains("Unary"));
}

#[test]
fn test_depyler_0354_expr_unary_nested() {
    let inner = HirExpr::Unary {
        op: HirUnaryOp::Negate,
        operand: Box::new(HirExpr::Identifier("x".to_string())),
    };

    let outer = HirExpr::Unary {
        op: HirUnaryOp::Negate,
        operand: Box::new(inner),
    };

    let _serialized = serde_json::to_string(&outer).unwrap();
}

// ============================================================================
// HIR EXPRESSION TESTS - CALL
// ============================================================================

#[test]
fn test_depyler_0354_expr_call_no_args() {
    let expr = HirExpr::Call { func: Box::new(HirExpr::Identifier("foo".to_string())), args: vec![] };

    assert_eq!(expr.clone(), expr);
}

#[test]
fn test_depyler_0354_expr_call_with_args() {
    let expr = HirExpr::Call { func: Box::new(HirExpr::Identifier("print".to_string())), args: vec![
            HirExpr::Literal(HirLiteral::String("hello".to_string())),
            HirExpr::Literal(HirLiteral::Integer(42)),
        ] };

    let debug = format!("{:?}", expr);
    assert!(debug.contains("Call"));
}

#[test]
fn test_depyler_0354_expr_call_nested() {
    let inner_call = HirExpr::Call { func: Box::new(HirExpr::Identifier("inner".to_string())), args: vec![HirExpr::Literal(HirLiteral::Integer(1))] };

    let outer_call = HirExpr::Call { func: Box::new(HirExpr::Identifier("outer".to_string())), args: vec![inner_call] };

    let _serialized = serde_json::to_string(&outer_call).unwrap();
}

// ============================================================================
// HIR EXPRESSION TESTS - IF
// ============================================================================

#[test]
fn test_depyler_0354_expr_if_without_else() {
    let expr = HirExpr::If {
        condition: Box::new(HirExpr::Literal(HirLiteral::Bool(true))),
        then_branch: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
        else_branch: None,
    };

    assert_eq!(expr.clone(), expr);
}

#[test]
fn test_depyler_0354_expr_if_with_else() {
    let expr = HirExpr::If {
        condition: Box::new(HirExpr::Identifier("condition".to_string())),
        then_branch: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
        else_branch: Some(Box::new(HirExpr::Literal(HirLiteral::Integer(0)))),
    };

    let debug = format!("{:?}", expr);
    assert!(debug.contains("If"));
}

#[test]
fn test_depyler_0354_expr_if_nested() {
    let inner_if = HirExpr::If {
        condition: Box::new(HirExpr::Literal(HirLiteral::Bool(false))),
        then_branch: Box::new(HirExpr::Literal(HirLiteral::Integer(2))),
        else_branch: None,
    };

    let outer_if = HirExpr::If {
        condition: Box::new(HirExpr::Literal(HirLiteral::Bool(true))),
        then_branch: Box::new(inner_if),
        else_branch: Some(Box::new(HirExpr::Literal(HirLiteral::Integer(0)))),
    };

    let _serialized = serde_json::to_string(&outer_if).unwrap();
}

// ============================================================================
// HIR EXPRESSION TESTS - BLOCK
// ============================================================================

#[test]
fn test_depyler_0354_expr_block_empty() {
    let expr = HirExpr::Block(vec![]);
    assert_eq!(expr, HirExpr::Block(vec![]));
}

#[test]
fn test_depyler_0354_expr_block_single_statement() {
    let stmt = HirStatement::Expression(Box::new(HirExpr::Literal(HirLiteral::Integer(42))));
    let expr = HirExpr::Block(vec![stmt]);

    let cloned = expr.clone();
    assert_eq!(expr, cloned);
}

#[test]
fn test_depyler_0354_expr_block_multiple_statements() {
    let stmts = vec![
        HirStatement::Let {
            name: "x".to_string(),
            value: Box::new(HirExpr::Literal(HirLiteral::Integer(10))),
            is_mutable: false,
        },
        HirStatement::Expression(Box::new(HirExpr::Identifier("x".to_string()))),
    ];

    let expr = HirExpr::Block(stmts);
    let debug = format!("{:?}", expr);
    assert!(debug.contains("Block"));
}

// ============================================================================
// HIR EXPRESSION TESTS - FUNCTION
// ============================================================================

#[test]
fn test_depyler_0354_expr_function_basic() {
    let expr = HirExpr::Function {
        name: "add".to_string(),
        params: vec![],
        body: Box::new(HirExpr::Literal(HirLiteral::Integer(0))),
        is_async: false,
        return_type: None,
    };

    assert_eq!(expr.clone(), expr);
}

#[test]
fn test_depyler_0354_expr_function_with_params() {
    let expr = HirExpr::Function {
        name: "multiply".to_string(),
        params: vec![
            HirParam {
                name: "a".to_string(),
                typ: Some(HirType::Int),
                default: None,
            },
            HirParam {
                name: "b".to_string(),
                typ: Some(HirType::Int),
                default: None,
            },
        ],
        body: Box::new(HirExpr::Binary {
            left: Box::new(HirExpr::Identifier("a".to_string())),
            op: HirBinaryOp::Multiply,
            right: Box::new(HirExpr::Identifier("b".to_string())),
        }),
        is_async: false,
        return_type: Some(HirType::Int),
    };

    let debug = format!("{:?}", expr);
    assert!(debug.contains("Function"));
}

#[test]
fn test_depyler_0354_expr_function_async() {
    let expr = HirExpr::Function {
        name: "async_func".to_string(),
        params: vec![],
        body: Box::new(HirExpr::Await(Box::new(HirExpr::Identifier("future".to_string())))),
        is_async: true,
        return_type: None,
    };

    let _serialized = serde_json::to_string(&expr).unwrap();
}

// ============================================================================
// HIR EXPRESSION TESTS - LAMBDA
// ============================================================================

#[test]
fn test_depyler_0354_expr_lambda_no_params() {
    let expr = HirExpr::Lambda {
        params: vec![],
        body: Box::new(HirExpr::Literal(HirLiteral::Integer(42))),
    };

    assert_eq!(expr.clone(), expr);
}

#[test]
fn test_depyler_0354_expr_lambda_with_params() {
    let expr = HirExpr::Lambda {
        params: vec![
            HirParam {
                name: "x".to_string(),
                typ: None,
                default: None,
            },
        ],
        body: Box::new(HirExpr::Binary {
            left: Box::new(HirExpr::Identifier("x".to_string())),
            op: HirBinaryOp::Add,
            right: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
        }),
    };

    let debug = format!("{:?}", expr);
    assert!(debug.contains("Lambda"));
}

// ============================================================================
// HIR EXPRESSION TESTS - FOR
// ============================================================================

#[test]
fn test_depyler_0354_expr_for_loop() {
    let expr = HirExpr::For {
        var: "i".to_string(),
        iter: Box::new(HirExpr::List(vec![
            HirExpr::Literal(HirLiteral::Integer(1)),
            HirExpr::Literal(HirLiteral::Integer(2)),
            HirExpr::Literal(HirLiteral::Integer(3)),
        ])),
        body: Box::new(HirExpr::Block(vec![
            HirStatement::Expression(Box::new(HirExpr::Identifier("i".to_string()))),
        ])),
    };

    let cloned = expr.clone();
    assert_eq!(expr, cloned);
}

#[test]
fn test_depyler_0354_expr_for_loop_nested() {
    let inner_loop = HirExpr::For {
        var: "j".to_string(),
        iter: Box::new(HirExpr::List(vec![])),
        body: Box::new(HirExpr::Block(vec![])),
    };

    let outer_loop = HirExpr::For {
        var: "i".to_string(),
        iter: Box::new(HirExpr::List(vec![])),
        body: Box::new(inner_loop),
    };

    let _serialized = serde_json::to_string(&outer_loop).unwrap();
}

// ============================================================================
// HIR EXPRESSION TESTS - WHILE
// ============================================================================

#[test]
fn test_depyler_0354_expr_while_loop() {
    let expr = HirExpr::While {
        condition: Box::new(HirExpr::Literal(HirLiteral::Bool(true))),
        body: Box::new(HirExpr::Block(vec![
            HirStatement::Expression(Box::new(HirExpr::Break(None))),
        ])),
    };

    assert_eq!(expr.clone(), expr);
}

#[test]
fn test_depyler_0354_expr_while_nested() {
    let inner_while = HirExpr::While {
        condition: Box::new(HirExpr::Literal(HirLiteral::Bool(false))),
        body: Box::new(HirExpr::Block(vec![])),
    };

    let outer_while = HirExpr::While {
        condition: Box::new(HirExpr::Literal(HirLiteral::Bool(true))),
        body: Box::new(inner_while),
    };

    let debug = format!("{:?}", outer_while);
    assert!(debug.contains("While"));
}

// ============================================================================
// HIR STATEMENT TESTS
// ============================================================================

#[test]
fn test_depyler_0354_statement_let_immutable() {
    let stmt = HirStatement::Let {
        name: "x".to_string(),
        value: Box::new(HirExpr::Literal(HirLiteral::Integer(10))),
        is_mutable: false,
    };

    let cloned = stmt.clone();
    assert_eq!(stmt, cloned);
}

#[test]
fn test_depyler_0354_statement_let_mutable() {
    let stmt = HirStatement::Let {
        name: "counter".to_string(),
        value: Box::new(HirExpr::Literal(HirLiteral::Integer(0))),
        is_mutable: true,
    };

    let debug = format!("{:?}", stmt);
    assert!(debug.contains("Let"));
}

#[test]
fn test_depyler_0354_statement_expression() {
    let stmt = HirStatement::Expression(Box::new(HirExpr::Call { func: Box::new(HirExpr::Identifier("print".to_string())), args: vec![HirExpr::Literal(HirLiteral::String("hello".to_string()))] }));

    let _serialized = serde_json::to_string(&stmt).unwrap();
}

// ============================================================================
// COMPLEX INTEGRATION TESTS
// ============================================================================

#[test]
fn test_depyler_0354_complex_nested_structure() {
    let hir = Hir {
        root: HirExpr::Block(vec![
            HirStatement::Let {
                name: "result".to_string(),
                value: Box::new(HirExpr::Binary {
                    left: Box::new(HirExpr::Literal(HirLiteral::Integer(10))),
                    op: HirBinaryOp::Multiply,
                    right: Box::new(HirExpr::Binary {
                        left: Box::new(HirExpr::Literal(HirLiteral::Integer(2))),
                        op: HirBinaryOp::Add,
                        right: Box::new(HirExpr::Literal(HirLiteral::Integer(3))),
                    }),
                }),
                is_mutable: false,
            },
            HirStatement::Expression(Box::new(HirExpr::Return(Some(Box::new(
                HirExpr::Identifier("result".to_string()),
            ))))),
        ]),
        metadata: HirMetadata {
            source_file: Some("complex.py".to_string()),
            module_name: Some("complex".to_string()),
        },
    };

    let json = serde_json::to_string(&hir).unwrap();
    let deserialized: Hir = serde_json::from_str(&json).unwrap();
    assert_eq!(hir, deserialized);
}

#[test]
fn test_depyler_0354_function_with_control_flow() {
    let expr = HirExpr::Function {
        name: "factorial".to_string(),
        params: vec![HirParam {
            name: "n".to_string(),
            typ: Some(HirType::Int),
            default: None,
        }],
        body: Box::new(HirExpr::If {
            condition: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Identifier("n".to_string())),
                op: HirBinaryOp::LessEqual,
                right: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
            }),
            then_branch: Box::new(HirExpr::Return(Some(Box::new(HirExpr::Literal(
                HirLiteral::Integer(1),
            ))))),
            else_branch: Some(Box::new(HirExpr::Return(Some(Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Identifier("n".to_string())),
                op: HirBinaryOp::Multiply,
                right: Box::new(HirExpr::Call { func: Box::new(HirExpr::Identifier("factorial".to_string())), args: vec![HirExpr::Binary {
                        left: Box::new(HirExpr::Identifier("n".to_string())),
                        op: HirBinaryOp::Subtract,
                        right: Box::new(HirExpr::Literal(HirLiteral::Integer(1))), }],
                }),
            }))))),
        }),
        is_async: false,
        return_type: Some(HirType::Int),
    };

    let debug = format!("{:?}", expr);
    assert!(debug.contains("Function"));
    assert!(debug.contains("factorial"));
}

#[test]
fn test_depyler_0354_async_function_with_await() {
    let expr = HirExpr::Function {
        name: "fetch_data".to_string(),
        params: vec![HirParam {
            name: "url".to_string(),
            typ: Some(HirType::String),
            default: None,
        }],
        body: Box::new(HirExpr::Block(vec![
            HirStatement::Let {
                name: "response".to_string(),
                value: Box::new(HirExpr::Await(Box::new(HirExpr::Call { func: Box::new(HirExpr::Identifier("fetch".to_string())), args: vec![HirExpr::Identifier("url".to_string())] }))),
                is_mutable: false,
            },
            HirStatement::Expression(Box::new(HirExpr::Return(Some(Box::new(
                HirExpr::Identifier("response".to_string()),
            ))))),
        ])),
        is_async: true,
        return_type: Some(HirType::Named("Response".to_string())),
    };

    let _serialized = serde_json::to_string(&expr).unwrap();
}

// ============================================================================
// PROPERTY TESTS - HIR Structure Robustness
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_literal_integer_roundtrip(value in -10000i64..10000i64) {
            let lit = HirLiteral::Integer(value);
            let json = serde_json::to_string(&lit).unwrap();
            let deserialized: HirLiteral = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(lit, deserialized);
        }

        #[test]
        fn prop_literal_string_roundtrip(s in "\\PC{0,100}") {
            let lit = HirLiteral::String(s);
            let json = serde_json::to_string(&lit).unwrap();
            let deserialized: HirLiteral = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(lit, deserialized);
        }

        #[test]
        fn prop_identifier_never_panics(name in "\\PC{1,50}") {
            let expr = HirExpr::Identifier(name.clone());
            let cloned = expr.clone();
            prop_assert_eq!(expr, cloned);
        }

        #[test]
        fn prop_metadata_source_file_roundtrip(file in proptest::option::of("\\PC{1,100}")) {
            let metadata = HirMetadata {
                source_file: file,
                module_name: None,
            };
            let json = serde_json::to_string(&metadata).unwrap();
            let deserialized: HirMetadata = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(metadata, deserialized);
        }

        #[test]
        fn prop_binary_expr_clone_equals_self(
            left_val in -100i64..100i64,
            right_val in -100i64..100i64,
        ) {
            let expr = HirExpr::Binary {
                left: Box::new(HirExpr::Literal(HirLiteral::Integer(left_val))),
                op: HirBinaryOp::Add,
                right: Box::new(HirExpr::Literal(HirLiteral::Integer(right_val))),
            };

            let cloned = expr.clone();
            prop_assert_eq!(expr, cloned);
        }

        #[test]
        fn prop_list_expr_any_length(count in 0usize..50) {
            let elements: Vec<HirExpr> = (0..count)
                .map(|i| HirExpr::Literal(HirLiteral::Integer(i as i64)))
                .collect();

            let expr = HirExpr::List(elements);
            let _debug = format!("{:?}", expr);
        }

        #[test]
        fn prop_param_name_never_panics(name in "\\PC{1,50}") {
            let param = HirParam {
                name,
                typ: None,
                default: None,
            };

            let cloned = param.clone();
            prop_assert_eq!(param, cloned);
        }
    }
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_depyler_0354_deeply_nested_binary_expressions() {
    let mut expr = HirExpr::Literal(HirLiteral::Integer(0));

    for i in 1..20 {
        expr = HirExpr::Binary {
            left: Box::new(expr),
            op: HirBinaryOp::Add,
            right: Box::new(HirExpr::Literal(HirLiteral::Integer(i))),
        };
    }

    let _debug = format!("{:?}", expr);
}

#[test]
fn test_depyler_0354_empty_string_identifiers() {
    let expr = HirExpr::Identifier("".to_string());
    assert_eq!(expr, HirExpr::Identifier("".to_string()));
}

#[test]
fn test_depyler_0354_unicode_in_strings() {
    let lit = HirLiteral::String("Hello, 世界! 🌍".to_string());
    let json = serde_json::to_string(&lit).unwrap();
    let deserialized: HirLiteral = serde_json::from_str(&json).unwrap();
    assert_eq!(lit, deserialized);
}

#[test]
fn test_depyler_0354_float_special_values() {
    let infinity = HirLiteral::Float(f64::INFINITY);
    let neg_infinity = HirLiteral::Float(f64::NEG_INFINITY);
    let zero = HirLiteral::Float(0.0);

    let _d1 = format!("{:?}", infinity);
    let _d2 = format!("{:?}", neg_infinity);
    let _d3 = format!("{:?}", zero);
}

#[test]
fn test_depyler_0354_all_binary_ops_distinct() {
    let ops = vec![
        HirBinaryOp::Add,
        HirBinaryOp::Subtract,
        HirBinaryOp::Multiply,
        HirBinaryOp::Divide,
        HirBinaryOp::Modulo,
        HirBinaryOp::Power,
        HirBinaryOp::Equal,
        HirBinaryOp::NotEqual,
        HirBinaryOp::Less,
        HirBinaryOp::LessEqual,
        HirBinaryOp::Greater,
        HirBinaryOp::GreaterEqual,
        HirBinaryOp::And,
        HirBinaryOp::Or,
        HirBinaryOp::BitwiseAnd,
        HirBinaryOp::BitwiseOr,
        HirBinaryOp::BitwiseXor,
        HirBinaryOp::LeftShift,
        HirBinaryOp::RightShift,
    ];

    // All ops should be distinct
    for (i, op1) in ops.iter().enumerate() {
        for (j, op2) in ops.iter().enumerate() {
            if i == j {
                assert_eq!(op1, op2);
            } else {
                assert_ne!(op1, op2);
            }
        }
    }
}

#[test]
fn test_depyler_0354_recursive_type_list_of_lists() {
    let typ = HirType::List(Box::new(HirType::List(Box::new(HirType::List(Box::new(
        HirType::Int,
    ))))));

    let json = serde_json::to_string(&typ).unwrap();
    let deserialized: HirType = serde_json::from_str(&json).unwrap();
    assert_eq!(typ, deserialized);
}

#[test]
fn test_depyler_0354_optional_of_optional() {
    let typ = HirType::Optional(Box::new(HirType::Optional(Box::new(HirType::String))));
    let cloned = typ.clone();
    assert_eq!(typ, cloned);
}
