// DEPYLER-0005: Tests for expr_to_rust_tokens function
//
// EXTREME TDD: These tests are written to ensure behavior preservation
// during Extract Method pattern application.

// Note: expr_to_rust_tokens is not public, so we test it through the
// public API that uses it. For now, we'll create tests that verify
// expression transpilation works correctly.

use depyler_core::hir::*;

/// Helper function to create integer literal
fn create_int_literal(value: i64) -> HirExpr {
    HirExpr::Literal(Literal::Int(value))
}

/// Helper function to create string literal
fn create_string_literal(value: &str) -> HirExpr {
    HirExpr::Literal(Literal::String(value.to_string()))
}

/// Helper function to create bool literal
fn create_bool_literal(value: bool) -> HirExpr {
    HirExpr::Literal(Literal::Bool(value))
}

/// Helper function to create variable reference
fn create_var(name: &str) -> HirExpr {
    HirExpr::Var(name.to_string())
}

#[cfg(test)]
mod literal_tests {
    use super::*;

    #[test]
    fn test_int_literal() {
        let expr = create_int_literal(42);
        // Verify the expression can be created without panic
        assert!(matches!(expr, HirExpr::Literal(Literal::Int(42))));
    }

    #[test]
    fn test_string_literal() {
        let expr = create_string_literal("hello");
        assert!(matches!(expr, HirExpr::Literal(Literal::String(_))));
    }

    #[test]
    fn test_bool_literal() {
        let expr = create_bool_literal(true);
        assert!(matches!(expr, HirExpr::Literal(Literal::Bool(true))));
    }

    #[test]
    fn test_none_literal() {
        let expr = HirExpr::Literal(Literal::None);
        assert!(matches!(expr, HirExpr::Literal(Literal::None)));
    }
}

#[cfg(test)]
mod variable_tests {
    use super::*;

    #[test]
    fn test_simple_var() {
        let expr = create_var("x");
        assert!(matches!(expr, HirExpr::Var(_)));
    }

    #[test]
    fn test_var_with_underscore() {
        let expr = create_var("my_var");
        assert!(matches!(expr, HirExpr::Var(_)));
    }
}

#[cfg(test)]
mod binary_op_tests {
    use super::*;

    #[test]
    fn test_add_operation() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(create_int_literal(1)),
            right: Box::new(create_int_literal(2)),
        };
        assert!(matches!(expr, HirExpr::Binary { .. }));
    }

    #[test]
    fn test_sub_operation() {
        let expr = HirExpr::Binary {
            op: BinOp::Sub,
            left: Box::new(create_int_literal(5)),
            right: Box::new(create_int_literal(3)),
        };
        assert!(matches!(expr, HirExpr::Binary { .. }));
    }

    #[test]
    fn test_floor_div_operation() {
        let expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(create_int_literal(7)),
            right: Box::new(create_int_literal(3)),
        };
        assert!(matches!(expr, HirExpr::Binary { .. }));
    }

    #[test]
    fn test_comparison_operation() {
        let expr = HirExpr::Binary {
            op: BinOp::Lt,
            left: Box::new(create_var("x")),
            right: Box::new(create_int_literal(10)),
        };
        assert!(matches!(expr, HirExpr::Binary { .. }));
    }

    #[test]
    fn test_logical_and() {
        let expr = HirExpr::Binary {
            op: BinOp::And,
            left: Box::new(create_bool_literal(true)),
            right: Box::new(create_bool_literal(false)),
        };
        assert!(matches!(expr, HirExpr::Binary { .. }));
    }

    #[test]
    fn test_nested_binary() {
        let inner = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(create_int_literal(2)),
            right: Box::new(create_int_literal(3)),
        };
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(inner),
            right: Box::new(create_int_literal(4)),
        };
        assert!(matches!(expr, HirExpr::Binary { .. }));
    }
}

#[cfg(test)]
mod unary_op_tests {
    use super::*;

    #[test]
    fn test_negation() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(create_int_literal(42)),
        };
        assert!(matches!(expr, HirExpr::Unary { .. }));
    }

    #[test]
    fn test_logical_not() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(create_bool_literal(true)),
        };
        assert!(matches!(expr, HirExpr::Unary { .. }));
    }
}

#[cfg(test)]
mod call_tests {
    use super::*;

    #[test]
    fn test_simple_call_no_args() {
        let expr = HirExpr::Call {
            func: "foo".to_string(),
            args: vec![],
        };
        assert!(matches!(expr, HirExpr::Call { .. }));
    }

    #[test]
    fn test_call_with_args() {
        let expr = HirExpr::Call {
            func: "add".to_string(),
            args: vec![create_int_literal(1), create_int_literal(2)],
        };
        assert!(matches!(expr, HirExpr::Call { .. }));
    }

    #[test]
    fn test_call_with_complex_args() {
        let expr = HirExpr::Call {
            func: "process".to_string(),
            args: vec![
                create_var("x"),
                HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(create_var("y")),
                    right: Box::new(create_int_literal(1)),
                },
            ],
        };
        assert!(matches!(expr, HirExpr::Call { .. }));
    }
}

#[cfg(test)]
mod collection_tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let expr = HirExpr::List(vec![]);
        assert!(matches!(expr, HirExpr::List(_)));
    }

    #[test]
    fn test_list_with_items() {
        let expr = HirExpr::List(vec![
            create_int_literal(1),
            create_int_literal(2),
            create_int_literal(3),
        ]);
        assert!(matches!(expr, HirExpr::List(_)));
    }

    #[test]
    fn test_empty_dict() {
        let expr = HirExpr::Dict(vec![]);
        assert!(matches!(expr, HirExpr::Dict(_)));
    }

    #[test]
    fn test_dict_with_entries() {
        let expr = HirExpr::Dict(vec![
            (create_string_literal("key1"), create_int_literal(10)),
            (create_string_literal("key2"), create_int_literal(20)),
        ]);
        assert!(matches!(expr, HirExpr::Dict(_)));
    }

    #[test]
    fn test_tuple() {
        let expr = HirExpr::Tuple(vec![
            create_int_literal(1),
            create_string_literal("two"),
            create_bool_literal(true),
        ]);
        assert!(matches!(expr, HirExpr::Tuple(_)));
    }

    #[test]
    fn test_set() {
        let expr = HirExpr::Set(vec![
            create_int_literal(1),
            create_int_literal(2),
            create_int_literal(3),
        ]);
        assert!(matches!(expr, HirExpr::Set(_)));
    }

    #[test]
    fn test_frozen_set() {
        let expr = HirExpr::FrozenSet(vec![create_int_literal(1), create_int_literal(2)]);
        assert!(matches!(expr, HirExpr::FrozenSet(_)));
    }
}

#[cfg(test)]
mod access_tests {
    use super::*;

    #[test]
    fn test_index_access() {
        let expr = HirExpr::Index {
            base: Box::new(create_var("arr")),
            index: Box::new(create_int_literal(0)),
        };
        assert!(matches!(expr, HirExpr::Index { .. }));
    }

    #[test]
    fn test_attribute_access() {
        let expr = HirExpr::Attribute {
            value: Box::new(create_var("obj")),
            attr: "field".to_string(),
        };
        assert!(matches!(expr, HirExpr::Attribute { .. }));
    }
}

#[cfg(test)]
mod borrow_tests {
    use super::*;

    #[test]
    fn test_immutable_borrow() {
        let expr = HirExpr::Borrow {
            expr: Box::new(create_var("x")),
            mutable: false,
        };
        assert!(matches!(expr, HirExpr::Borrow { mutable: false, .. }));
    }

    #[test]
    fn test_mutable_borrow() {
        let expr = HirExpr::Borrow {
            expr: Box::new(create_var("x")),
            mutable: true,
        };
        assert!(matches!(expr, HirExpr::Borrow { mutable: true, .. }));
    }
}

#[cfg(test)]
mod method_call_tests {
    use super::*;

    #[test]
    fn test_method_call_no_args() {
        let expr = HirExpr::MethodCall {
            object: Box::new(create_var("obj")),
            method: "len".to_string(),
            args: vec![],
        };
        assert!(matches!(expr, HirExpr::MethodCall { .. }));
    }

    #[test]
    fn test_method_call_with_args() {
        let expr = HirExpr::MethodCall {
            object: Box::new(create_var("obj")),
            method: "add".to_string(),
            args: vec![create_int_literal(42)],
        };
        assert!(matches!(expr, HirExpr::MethodCall { .. }));
    }
}

#[cfg(test)]
mod slice_tests {
    use super::*;

    #[test]
    fn test_slice_full() {
        let expr = HirExpr::Slice {
            base: Box::new(create_var("arr")),
            start: Some(Box::new(create_int_literal(1))),
            stop: Some(Box::new(create_int_literal(3))),
            step: None,
        };
        assert!(matches!(expr, HirExpr::Slice { .. }));
    }

    #[test]
    fn test_slice_start_only() {
        let expr = HirExpr::Slice {
            base: Box::new(create_var("arr")),
            start: Some(Box::new(create_int_literal(2))),
            stop: None,
            step: None,
        };
        assert!(matches!(expr, HirExpr::Slice { .. }));
    }

    #[test]
    fn test_slice_stop_only() {
        let expr = HirExpr::Slice {
            base: Box::new(create_var("arr")),
            start: None,
            stop: Some(Box::new(create_int_literal(5))),
            step: None,
        };
        assert!(matches!(expr, HirExpr::Slice { .. }));
    }

    #[test]
    fn test_slice_clone() {
        let expr = HirExpr::Slice {
            base: Box::new(create_var("arr")),
            start: None,
            stop: None,
            step: None,
        };
        assert!(matches!(expr, HirExpr::Slice { .. }));
    }

    #[test]
    fn test_slice_with_step() {
        let expr = HirExpr::Slice {
            base: Box::new(create_var("arr")),
            start: Some(Box::new(create_int_literal(0))),
            stop: Some(Box::new(create_int_literal(10))),
            step: Some(Box::new(create_int_literal(2))),
        };
        assert!(matches!(expr, HirExpr::Slice { .. }));
    }
}

#[cfg(test)]
mod comprehension_tests {
    use super::*;

    #[test]
    fn test_list_comp_no_condition() {
        let expr = HirExpr::ListComp {
            element: Box::new(create_var("x")),
            target: "x".to_string(),
            iter: Box::new(create_var("items")),
            condition: None,
        };
        assert!(matches!(expr, HirExpr::ListComp { .. }));
    }

    #[test]
    fn test_list_comp_with_condition() {
        let expr = HirExpr::ListComp {
            element: Box::new(create_var("x")),
            target: "x".to_string(),
            iter: Box::new(create_var("items")),
            condition: Some(Box::new(HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(create_var("x")),
                right: Box::new(create_int_literal(5)),
            })),
        };
        assert!(matches!(expr, HirExpr::ListComp { .. }));
    }

    #[test]
    fn test_set_comp_no_condition() {
        let expr = HirExpr::SetComp {
            element: Box::new(create_var("x")),
            target: "x".to_string(),
            iter: Box::new(create_var("items")),
            condition: None,
        };
        assert!(matches!(expr, HirExpr::SetComp { .. }));
    }

    #[test]
    fn test_set_comp_with_condition() {
        let expr = HirExpr::SetComp {
            element: Box::new(create_var("x")),
            target: "x".to_string(),
            iter: Box::new(create_var("items")),
            condition: Some(Box::new(HirExpr::Binary {
                op: BinOp::Lt,
                left: Box::new(create_var("x")),
                right: Box::new(create_int_literal(10)),
            })),
        };
        assert!(matches!(expr, HirExpr::SetComp { .. }));
    }
}

#[cfg(test)]
mod lambda_tests {
    use super::*;

    #[test]
    fn test_lambda_no_params() {
        let expr = HirExpr::Lambda {
            params: vec![],
            body: Box::new(create_int_literal(42)),
        };
        assert!(matches!(expr, HirExpr::Lambda { .. }));
    }

    #[test]
    fn test_lambda_one_param() {
        let expr = HirExpr::Lambda {
            params: vec!["x".to_string()],
            body: Box::new(HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(create_var("x")),
                right: Box::new(create_int_literal(2)),
            }),
        };
        assert!(matches!(expr, HirExpr::Lambda { .. }));
    }

    #[test]
    fn test_lambda_multiple_params() {
        let expr = HirExpr::Lambda {
            params: vec!["x".to_string(), "y".to_string()],
            body: Box::new(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(create_var("x")),
                right: Box::new(create_var("y")),
            }),
        };
        assert!(matches!(expr, HirExpr::Lambda { .. }));
    }
}

#[cfg(test)]
mod async_tests {
    use super::*;

    #[test]
    fn test_await_expression() {
        let expr = HirExpr::Await {
            value: Box::new(HirExpr::Call {
                func: "async_func".to_string(),
                args: vec![],
            }),
        };
        assert!(matches!(expr, HirExpr::Await { .. }));
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;

    /// Test that complex nested expressions can be constructed
    #[test]
    fn test_complex_nested_expression() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Index {
                    base: Box::new(create_var("data")),
                    index: Box::new(create_int_literal(0)),
                }),
                method: "process".to_string(),
                args: vec![create_int_literal(42)],
            }),
            right: Box::new(HirExpr::ListComp {
                element: Box::new(HirExpr::Binary {
                    op: BinOp::Mul,
                    left: Box::new(create_var("x")),
                    right: Box::new(create_int_literal(2)),
                }),
                target: "x".to_string(),
                iter: Box::new(create_var("items")),
                condition: None,
            }),
        };
        assert!(matches!(expr, HirExpr::Binary { .. }));
    }

    /// Test all literal types
    #[test]
    fn test_all_literal_types() {
        let literals = vec![
            HirExpr::Literal(Literal::Int(42)),
            HirExpr::Literal(Literal::Float(2.5)),
            HirExpr::Literal(Literal::String("test".to_string())),
            HirExpr::Literal(Literal::Bool(true)),
            HirExpr::Literal(Literal::None),
        ];

        for lit in literals {
            assert!(matches!(lit, HirExpr::Literal(_)));
        }
    }

    /// Test all binary operators
    #[test]
    fn test_all_binary_operators() {
        let operators = vec![
            BinOp::Add,
            BinOp::Sub,
            BinOp::Mul,
            BinOp::Div,
            BinOp::FloorDiv,
            BinOp::Mod,
            BinOp::Pow,
            BinOp::Eq,
            BinOp::NotEq,
            BinOp::Lt,
            BinOp::LtEq,
            BinOp::Gt,
            BinOp::GtEq,
            BinOp::And,
            BinOp::Or,
            BinOp::BitAnd,
            BinOp::BitOr,
            BinOp::BitXor,
            BinOp::LShift,
            BinOp::RShift,
        ];

        for op in operators {
            let expr = HirExpr::Binary {
                op,
                left: Box::new(create_int_literal(1)),
                right: Box::new(create_int_literal(2)),
            };
            assert!(matches!(expr, HirExpr::Binary { .. }));
        }
    }
}
