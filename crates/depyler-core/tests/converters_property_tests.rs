use depyler_core::ast_bridge::ExprConverter;
use depyler_core::hir::*;
use proptest::prelude::*;
use rustpython_ast::{self as ast, bigint::BigInt, Constant, Expr, ExprConstant, ExprName};

// Strategy for generating valid Python identifiers
prop_compose! {
    fn arb_identifier()(s in "[a-zA-Z_][a-zA-Z0-9_]{0,15}") -> String {
        s
    }
}

// Strategy for generating small integers (to avoid overflow)
prop_compose! {
    fn arb_small_int()(n in -1000i64..1000i64) -> i64 {
        n
    }
}

// Strategy for generating simple literal expressions
prop_compose! {
    fn arb_literal_expr()(
        choice in 0..5u8,
        int_val in arb_small_int(),
        float_val in -1000.0f64..1000.0f64,
        str_val in "[a-zA-Z0-9 ]{0,20}",
        bool_val in any::<bool>()
    ) -> ast::Expr {
        match choice {
            0 => Expr::Constant(ExprConstant {
                value: Constant::Int(BigInt::from(int_val)),
                kind: None,
                range: Default::default(),
            }),
            1 => Expr::Constant(ExprConstant {
                value: Constant::Float(float_val),
                kind: None,
                range: Default::default(),
            }),
            2 => Expr::Constant(ExprConstant {
                value: Constant::Str(str_val.into()),
                kind: None,
                range: Default::default(),
            }),
            3 => Expr::Constant(ExprConstant {
                value: Constant::Bool(bool_val),
                kind: None,
                range: Default::default(),
            }),
            _ => Expr::Constant(ExprConstant {
                value: Constant::None,
                kind: None,
                range: Default::default(),
            }),
        }
    }
}

// Strategy for generating variable expressions
prop_compose! {
    fn arb_var_expr()(name in arb_identifier()) -> ast::Expr {
        Expr::Name(ExprName {
            id: name.into(),
            ctx: ast::ExprContext::Load,
            range: Default::default(),
        })
    }
}

// Strategy for generating simple expressions
prop_compose! {
    fn arb_simple_expr()(
        is_literal in any::<bool>(),
        literal in arb_literal_expr(),
        var in arb_var_expr()
    ) -> ast::Expr {
        if is_literal {
            literal
        } else {
            var
        }
    }
}

proptest! {
    #[test]
    fn test_literal_conversion_never_panics(expr in arb_literal_expr()) {
        // Converting a literal should never panic
        let _ = ExprConverter::convert(expr);
    }

    #[test]
    fn test_var_conversion_never_panics(expr in arb_var_expr()) {
        // Converting a variable should never panic
        let _ = ExprConverter::convert(expr);
    }

    #[test]
    fn test_converted_literals_preserve_values(
        int_val in arb_small_int(),
        float_val in -1000.0f64..1000.0f64,
        str_val in "[a-zA-Z0-9 ]{0,20}",
        bool_val in any::<bool>()
    ) {
        // Test integer
        let int_expr = Expr::Constant(ExprConstant {
            value: Constant::Int(BigInt::from(int_val)),
            kind: None,
            range: Default::default(),
        });
        match ExprConverter::convert(int_expr).unwrap() {
            HirExpr::Literal(Literal::Int(n)) => prop_assert_eq!(n, int_val),
            _ => panic!("Expected int literal"),
        }

        // Test float
        let float_expr = Expr::Constant(ExprConstant {
            value: Constant::Float(float_val),
            kind: None,
            range: Default::default(),
        });
        match ExprConverter::convert(float_expr).unwrap() {
            HirExpr::Literal(Literal::Float(f)) => {
                prop_assert!((f - float_val).abs() < 0.0001);
            }
            _ => panic!("Expected float literal"),
        }

        // Test string
        let str_expr = Expr::Constant(ExprConstant {
            value: Constant::Str(str_val.clone().into()),
            kind: None,
            range: Default::default(),
        });
        match ExprConverter::convert(str_expr).unwrap() {
            HirExpr::Literal(Literal::String(s)) => prop_assert_eq!(s, str_val),
            _ => panic!("Expected string literal"),
        }

        // Test bool
        let bool_expr = Expr::Constant(ExprConstant {
            value: Constant::Bool(bool_val),
            kind: None,
            range: Default::default(),
        });
        match ExprConverter::convert(bool_expr).unwrap() {
            HirExpr::Literal(Literal::Bool(b)) => prop_assert_eq!(b, bool_val),
            _ => panic!("Expected bool literal"),
        }
    }

    #[test]
    fn test_variable_names_preserved(name in arb_identifier()) {
        let expr = Expr::Name(ExprName {
            id: name.clone().into(),
            ctx: ast::ExprContext::Load,
            range: Default::default(),
        });
        
        match ExprConverter::convert(expr).unwrap() {
            HirExpr::Var(var_name) => prop_assert_eq!(var_name, name),
            _ => panic!("Expected variable"),
        }
    }

    #[test]
    fn test_list_conversion_preserves_length(
        elements in prop::collection::vec(arb_literal_expr(), 0..10)
    ) {
        let list_expr = ast::Expr::List(ast::ExprList {
            elts: elements.clone(),
            ctx: ast::ExprContext::Load,
            range: Default::default(),
        });
        
        match ExprConverter::convert(list_expr) {
            Ok(HirExpr::List(hir_elements)) => {
                prop_assert_eq!(hir_elements.len(), elements.len());
            }
            _ => prop_assert!(false, "Expected list conversion to succeed"),
        }
    }

    #[test]
    fn test_tuple_conversion_preserves_length(
        elements in prop::collection::vec(arb_literal_expr(), 0..10)
    ) {
        let tuple_expr = ast::Expr::Tuple(ast::ExprTuple {
            elts: elements.clone(),
            ctx: ast::ExprContext::Load,
            range: Default::default(),
        });
        
        match ExprConverter::convert(tuple_expr) {
            Ok(HirExpr::Tuple(hir_elements)) => {
                prop_assert_eq!(hir_elements.len(), elements.len());
            }
            _ => prop_assert!(false, "Expected tuple conversion to succeed"),
        }
    }

    #[test]
    fn test_binary_op_structure(
        left in arb_literal_expr(),
        right in arb_literal_expr(),
        op_choice in 0..5u8
    ) {
        let op = match op_choice {
            0 => ast::Operator::Add,
            1 => ast::Operator::Sub,
            2 => ast::Operator::Mult,
            3 => ast::Operator::Div,
            _ => ast::Operator::Mod,
        };
        
        let binop_expr = ast::Expr::BinOp(ast::ExprBinOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
            range: Default::default(),
        });
        
        match ExprConverter::convert(binop_expr) {
            Ok(HirExpr::Binary { op: _, left: _, right: _ }) => {
                // Successfully converted to binary operation
                prop_assert!(true);
            }
            _ => prop_assert!(false, "Expected binary operation"),
        }
    }

    #[test]
    fn test_unary_op_structure(
        operand in arb_literal_expr(),
        op_choice in 0..3u8
    ) {
        let op = match op_choice {
            0 => ast::UnaryOp::UAdd,
            1 => ast::UnaryOp::USub,
            _ => ast::UnaryOp::Not,
        };
        
        let unary_expr = ast::Expr::UnaryOp(ast::ExprUnaryOp {
            op,
            operand: Box::new(operand),
            range: Default::default(),
        });
        
        match ExprConverter::convert(unary_expr) {
            Ok(HirExpr::Unary { op: _, operand: _ }) => {
                prop_assert!(true);
            }
            _ => prop_assert!(false, "Expected unary operation"),
        }
    }

    #[test]
    fn test_function_call_structure(
        func_name in arb_identifier(),
        args in prop::collection::vec(arb_literal_expr(), 0..5)
    ) {
        let call_expr = ast::Expr::Call(ast::ExprCall {
            func: Box::new(Expr::Name(ExprName {
                id: func_name.clone().into(),
                ctx: ast::ExprContext::Load,
                range: Default::default(),
            })),
            args: args.clone(),
            keywords: vec![],
            range: Default::default(),
        });
        
        match ExprConverter::convert(call_expr) {
            Ok(HirExpr::Call { func, args: hir_args }) => {
                prop_assert_eq!(func, func_name);
                prop_assert_eq!(hir_args.len(), args.len());
            }
            _ => prop_assert!(false, "Expected function call"),
        }
    }
}

// Additional deterministic tests
#[test]
fn test_conversion_determinism() {
    // Test that converting the same expression multiple times gives the same result
    let expr = Expr::Constant(ExprConstant {
        value: Constant::Int(BigInt::from(42)),
        kind: None,
        range: Default::default(),
    });
    
    let result1 = ExprConverter::convert(expr.clone()).unwrap();
    let result2 = ExprConverter::convert(expr).unwrap();
    
    // Can't directly compare HirExpr with Eq, but we can pattern match
    match (&result1, &result2) {
        (HirExpr::Literal(Literal::Int(n1)), HirExpr::Literal(Literal::Int(n2))) => {
            assert_eq!(n1, n2);
        }
        _ => panic!("Results don't match"),
    }
}

#[test]
fn test_nested_structure_preservation() {
    // Test that nested structures are preserved correctly
    let inner_list = ast::Expr::List(ast::ExprList {
        elts: vec![
            Expr::Constant(ExprConstant {
                value: Constant::Int(BigInt::from(1)),
                kind: None,
                range: Default::default(),
            }),
            Expr::Constant(ExprConstant {
                value: Constant::Int(BigInt::from(2)),
                kind: None,
                range: Default::default(),
            }),
        ],
        ctx: ast::ExprContext::Load,
        range: Default::default(),
    });
    
    let outer_list = ast::Expr::List(ast::ExprList {
        elts: vec![inner_list],
        ctx: ast::ExprContext::Load,
        range: Default::default(),
    });
    
    match ExprConverter::convert(outer_list).unwrap() {
        HirExpr::List(outer_elems) => {
            assert_eq!(outer_elems.len(), 1);
            match &outer_elems[0] {
                HirExpr::List(inner_elems) => {
                    assert_eq!(inner_elems.len(), 2);
                }
                _ => panic!("Expected inner list"),
            }
        }
        _ => panic!("Expected outer list"),
    }
}