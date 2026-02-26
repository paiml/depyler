//! Tests for direct_rules_convert module
//!
//! DEPYLER-COVERAGE-95: Comprehensive test coverage for statement and expression conversion.

use super::*;

// =========================================================================
// convert_literal tests
// =========================================================================

#[test]
fn test_convert_literal_int() {
    let lit = Literal::Int(42);
    let result = convert_literal(&lit);
    let result_str = quote::quote!(#result).to_string();
    assert!(result_str.contains("42"));
}

#[test]
fn test_convert_literal_int_negative() {
    let lit = Literal::Int(-100);
    let result = convert_literal(&lit);
    let result_str = quote::quote!(#result).to_string();
    assert!(result_str.contains("100"));
}

#[test]
fn test_convert_literal_float() {
    let lit = Literal::Float(3.15);
    let result = convert_literal(&lit);
    let result_str = quote::quote!(#result).to_string();
    assert!(result_str.contains("3.15") || result_str.contains("3.1"));
}

#[test]
fn test_convert_literal_string() {
    let lit = Literal::String("hello".to_string());
    let result = convert_literal(&lit);
    let result_str = quote::quote!(#result).to_string();
    assert!(result_str.contains("hello"));
}

#[test]
fn test_convert_literal_string_empty() {
    let lit = Literal::String("".to_string());
    let result = convert_literal(&lit);
    let result_str = quote::quote!(#result).to_string();
    assert!(result_str.contains("\"\"") || result_str.contains("String"));
}

#[test]
fn test_convert_literal_bool_true() {
    let lit = Literal::Bool(true);
    let result = convert_literal(&lit);
    let result_str = quote::quote!(#result).to_string();
    assert!(result_str.contains("true"));
}

#[test]
fn test_convert_literal_bool_false() {
    let lit = Literal::Bool(false);
    let result = convert_literal(&lit);
    let result_str = quote::quote!(#result).to_string();
    assert!(result_str.contains("false"));
}

#[test]
fn test_convert_literal_none() {
    let lit = Literal::None;
    let result = convert_literal(&lit);
    let result_str = quote::quote!(#result).to_string();
    assert!(result_str.contains("None") || result_str.contains("()"));
}

// =========================================================================
// convert_binop tests
// =========================================================================

#[test]
fn test_convert_binop_add() {
    let result = convert_binop(BinOp::Add);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_sub() {
    let result = convert_binop(BinOp::Sub);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_mul() {
    let result = convert_binop(BinOp::Mul);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_div() {
    let result = convert_binop(BinOp::Div);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_mod() {
    let result = convert_binop(BinOp::Mod);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_eq() {
    let result = convert_binop(BinOp::Eq);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_not_eq() {
    let result = convert_binop(BinOp::NotEq);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_lt() {
    let result = convert_binop(BinOp::Lt);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_gt() {
    let result = convert_binop(BinOp::Gt);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_lt_eq() {
    let result = convert_binop(BinOp::LtEq);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_gt_eq() {
    let result = convert_binop(BinOp::GtEq);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_and() {
    let result = convert_binop(BinOp::And);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_or() {
    let result = convert_binop(BinOp::Or);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_bit_and() {
    let result = convert_binop(BinOp::BitAnd);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_bit_or() {
    let result = convert_binop(BinOp::BitOr);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_bit_xor() {
    let result = convert_binop(BinOp::BitXor);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_lshift() {
    let result = convert_binop(BinOp::LShift);
    assert!(result.is_ok());
}

#[test]
fn test_convert_binop_rshift() {
    let result = convert_binop(BinOp::RShift);
    assert!(result.is_ok());
}

// =========================================================================
// convert_arithmetic_op tests
// =========================================================================

#[test]
fn test_convert_arithmetic_op_add() {
    let result = convert_arithmetic_op(BinOp::Add);
    assert!(result.is_ok());
}

#[test]
fn test_convert_arithmetic_op_sub() {
    let result = convert_arithmetic_op(BinOp::Sub);
    assert!(result.is_ok());
}

#[test]
fn test_convert_arithmetic_op_mul() {
    let result = convert_arithmetic_op(BinOp::Mul);
    assert!(result.is_ok());
}

#[test]
fn test_convert_arithmetic_op_div() {
    let result = convert_arithmetic_op(BinOp::Div);
    assert!(result.is_ok());
}

#[test]
fn test_convert_arithmetic_op_mod() {
    let result = convert_arithmetic_op(BinOp::Mod);
    assert!(result.is_ok());
}

#[test]
fn test_convert_arithmetic_op_floor_div_special() {
    let result = convert_arithmetic_op(BinOp::FloorDiv);
    assert!(result.is_err());
}

#[test]
fn test_convert_arithmetic_op_pow_special() {
    let result = convert_arithmetic_op(BinOp::Pow);
    assert!(result.is_err());
}

#[test]
fn test_convert_arithmetic_op_invalid() {
    let result = convert_arithmetic_op(BinOp::And);
    assert!(result.is_err());
}

// =========================================================================
// convert_comparison_op tests
// =========================================================================

#[test]
fn test_convert_comparison_op_eq() {
    let result = convert_comparison_op(BinOp::Eq);
    assert!(result.is_ok());
}

#[test]
fn test_convert_comparison_op_not_eq() {
    let result = convert_comparison_op(BinOp::NotEq);
    assert!(result.is_ok());
}

#[test]
fn test_convert_comparison_op_lt() {
    let result = convert_comparison_op(BinOp::Lt);
    assert!(result.is_ok());
}

#[test]
fn test_convert_comparison_op_lt_eq() {
    let result = convert_comparison_op(BinOp::LtEq);
    assert!(result.is_ok());
}

#[test]
fn test_convert_comparison_op_gt() {
    let result = convert_comparison_op(BinOp::Gt);
    assert!(result.is_ok());
}

#[test]
fn test_convert_comparison_op_gt_eq() {
    let result = convert_comparison_op(BinOp::GtEq);
    assert!(result.is_ok());
}

#[test]
fn test_convert_comparison_op_invalid() {
    let result = convert_comparison_op(BinOp::Add);
    assert!(result.is_err());
}

// =========================================================================
// convert_logical_op tests
// =========================================================================

#[test]
fn test_convert_logical_op_and() {
    let result = convert_logical_op(BinOp::And);
    assert!(result.is_ok());
}

#[test]
fn test_convert_logical_op_or() {
    let result = convert_logical_op(BinOp::Or);
    assert!(result.is_ok());
}

#[test]
fn test_convert_logical_op_invalid() {
    let result = convert_logical_op(BinOp::Add);
    assert!(result.is_err());
}

// =========================================================================
// convert_bitwise_op tests
// =========================================================================

#[test]
fn test_convert_bitwise_op_and() {
    let result = convert_bitwise_op(BinOp::BitAnd);
    assert!(result.is_ok());
}

#[test]
fn test_convert_bitwise_op_or() {
    let result = convert_bitwise_op(BinOp::BitOr);
    assert!(result.is_ok());
}

#[test]
fn test_convert_bitwise_op_xor() {
    let result = convert_bitwise_op(BinOp::BitXor);
    assert!(result.is_ok());
}

#[test]
fn test_convert_bitwise_op_lshift() {
    let result = convert_bitwise_op(BinOp::LShift);
    assert!(result.is_ok());
}

#[test]
fn test_convert_bitwise_op_rshift() {
    let result = convert_bitwise_op(BinOp::RShift);
    assert!(result.is_ok());
}

#[test]
fn test_convert_bitwise_op_invalid() {
    let result = convert_bitwise_op(BinOp::Add);
    assert!(result.is_err());
}

// =========================================================================
// is_len_call tests
// =========================================================================

#[test]
fn test_is_len_call_true() {
    let expr = HirExpr::Call {
        func: "len".to_string(),
        args: vec![HirExpr::Var("x".to_string())],
        kwargs: vec![],
    };
    assert!(is_len_call(&expr));
}

#[test]
fn test_is_len_call_false_other_func() {
    let expr = HirExpr::Call { func: "print".to_string(), args: vec![], kwargs: vec![] };
    assert!(!is_len_call(&expr));
}

#[test]
fn test_is_len_call_false_not_call() {
    let expr = HirExpr::Var("len".to_string());
    assert!(!is_len_call(&expr));
}

// =========================================================================
// is_pure_expression_direct tests
// =========================================================================

#[test]
fn test_is_pure_expression_literal_int() {
    let expr = HirExpr::Literal(Literal::Int(42));
    assert!(is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_literal_string() {
    let expr = HirExpr::Literal(Literal::String("hello".to_string()));
    assert!(is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_var() {
    let expr = HirExpr::Var("x".to_string());
    assert!(is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_binary() {
    let expr = HirExpr::Binary {
        op: BinOp::Add,
        left: Box::new(HirExpr::Literal(Literal::Int(1))),
        right: Box::new(HirExpr::Literal(Literal::Int(2))),
    };
    assert!(is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_call_not_pure() {
    // All function calls are considered impure in this implementation
    let expr = HirExpr::Call {
        func: "len".to_string(),
        args: vec![HirExpr::Var("x".to_string())],
        kwargs: vec![],
    };
    assert!(!is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_call_print() {
    let expr = HirExpr::Call { func: "print".to_string(), args: vec![], kwargs: vec![] };
    assert!(!is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_method_call_not_pure() {
    // All method calls are considered impure in this implementation
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("x".to_string())),
        method: "len".to_string(),
        args: vec![],
        kwargs: vec![],
    };
    assert!(!is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_method_call_append() {
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("x".to_string())),
        method: "append".to_string(),
        args: vec![HirExpr::Literal(Literal::Int(1))],
        kwargs: vec![],
    };
    assert!(!is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_attribute() {
    let expr = HirExpr::Attribute {
        value: Box::new(HirExpr::Var("self".to_string())),
        attr: "x".to_string(),
    };
    assert!(is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_index() {
    let expr = HirExpr::Index {
        base: Box::new(HirExpr::Var("arr".to_string())),
        index: Box::new(HirExpr::Literal(Literal::Int(0))),
    };
    assert!(is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_tuple() {
    let expr =
        HirExpr::Tuple(vec![HirExpr::Literal(Literal::Int(1)), HirExpr::Var("x".to_string())]);
    assert!(is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_unary() {
    let expr =
        HirExpr::Unary { op: UnaryOp::Neg, operand: Box::new(HirExpr::Literal(Literal::Int(42))) };
    assert!(is_pure_expression_direct(&expr));
}

// =========================================================================
// find_mutable_vars_in_body tests
// =========================================================================

#[test]
fn test_find_mutable_vars_empty() {
    let stmts: Vec<HirStmt> = vec![];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.is_empty());
}

#[test]
fn test_find_mutable_vars_single_assign() {
    let stmts = vec![HirStmt::Assign {
        target: AssignTarget::Symbol("x".to_string()),
        value: HirExpr::Literal(Literal::Int(1)),
        type_annotation: None,
    }];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.is_empty()); // First assignment is not mutable
}

#[test]
fn test_find_mutable_vars_reassignment() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        },
        HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(2)),
            type_annotation: None,
        },
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("x"));
}

#[test]
fn test_find_mutable_vars_attribute_assign() {
    let stmts = vec![HirStmt::Assign {
        target: AssignTarget::Attribute {
            value: Box::new(HirExpr::Var("self".to_string())),
            attr: "x".to_string(),
        },
        value: HirExpr::Literal(Literal::Int(1)),
        type_annotation: None,
    }];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("self"));
}

#[test]
fn test_find_mutable_vars_index_assign() {
    let stmts = vec![HirStmt::Assign {
        target: AssignTarget::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        },
        value: HirExpr::Literal(Literal::Int(1)),
        type_annotation: None,
    }];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("arr"));
}

#[test]
fn test_find_mutable_vars_tuple_assign() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Symbol("b".to_string()),
            ]),
            value: HirExpr::Tuple(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ]),
            type_annotation: None,
        },
        HirStmt::Assign {
            target: AssignTarget::Symbol("a".to_string()),
            value: HirExpr::Literal(Literal::Int(3)),
            type_annotation: None,
        },
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("a"));
    assert!(!result.contains("b"));
}

#[test]
fn test_find_mutable_vars_append_method() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("lst".to_string()),
            value: HirExpr::List(vec![]),
            type_annotation: None,
        },
        HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        }),
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("lst"));
}

#[test]
fn test_find_mutable_vars_in_if_body() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(0)),
            type_annotation: None,
        },
        HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            }],
            else_body: None,
        },
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("x"));
}

#[test]
fn test_find_mutable_vars_in_else_body() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(0)),
            type_annotation: None,
        },
        HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            }]),
        },
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("x"));
}

#[test]
fn test_find_mutable_vars_in_while_body() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("i".to_string()),
            value: HirExpr::Literal(Literal::Int(0)),
            type_annotation: None,
        },
        HirStmt::While {
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
        },
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("i"));
}

#[test]
fn test_find_mutable_vars_in_for_body() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("total".to_string()),
            value: HirExpr::Literal(Literal::Int(0)),
            type_annotation: None,
        },
        HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("total".to_string()),
                value: HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("total".to_string())),
                    right: Box::new(HirExpr::Var("i".to_string())),
                },
                type_annotation: None,
            }],
        },
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("total"));
}

// =========================================================================
// convert_body tests
// =========================================================================

#[test]
fn test_convert_body_empty() {
    let type_mapper = TypeMapper::default();
    let stmts: Vec<HirStmt> = vec![];
    let result = convert_body(&stmts, &type_mapper);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_convert_body_single_pass() {
    let type_mapper = TypeMapper::default();
    let stmts = vec![HirStmt::Pass];
    let result = convert_body(&stmts, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_body_single_expr() {
    let type_mapper = TypeMapper::default();
    let stmts = vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(42)))];
    let result = convert_body(&stmts, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_body_simple_assign() {
    let type_mapper = TypeMapper::default();
    let stmts = vec![HirStmt::Assign {
        target: AssignTarget::Symbol("x".to_string()),
        value: HirExpr::Literal(Literal::Int(42)),
        type_annotation: None,
    }];
    let result = convert_body(&stmts, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_body_return_int() {
    let type_mapper = TypeMapper::default();
    let stmts = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
    let result = convert_body(&stmts, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_body_return_none() {
    let type_mapper = TypeMapper::default();
    let stmts = vec![HirStmt::Return(None)];
    let result = convert_body(&stmts, &type_mapper);
    assert!(result.is_ok());
}

// =========================================================================
// convert_expr tests
// =========================================================================

#[test]
fn test_convert_expr_literal_int() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Literal(Literal::Int(42));
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_literal_string() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Literal(Literal::String("hello".to_string()));
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_var() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Var("x".to_string());
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_binary_add() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Binary {
        op: BinOp::Add,
        left: Box::new(HirExpr::Literal(Literal::Int(1))),
        right: Box::new(HirExpr::Literal(Literal::Int(2))),
    };
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_mul_with_add_preserves_precedence() {
    // DEPYLER-CLASS-001: Test that 2 * (a + b) preserves parentheses
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Binary {
        op: BinOp::Mul,
        left: Box::new(HirExpr::Literal(Literal::Int(2))),
        right: Box::new(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("width".to_string())),
            right: Box::new(HirExpr::Var("height".to_string())),
        }),
    };
    let result = convert_expr(&expr, &type_mapper).unwrap();
    let result_str = quote::quote!(#result).to_string();
    eprintln!("[DEBUG] Generated code: {}", result_str);
    // The addition should be wrapped in parentheses
    assert!(
        result_str.contains("(width + height)"),
        "Expected parentheses around addition: {}",
        result_str
    );
}

#[test]
fn test_convert_mul_with_self_field_add_preserves_precedence() {
    // DEPYLER-CLASS-001: Test with self.field pattern (method body)
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Binary {
        op: BinOp::Mul,
        left: Box::new(HirExpr::Literal(Literal::Int(2))),
        right: Box::new(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("self".to_string())),
                attr: "width".to_string(),
            }),
            right: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("self".to_string())),
                attr: "height".to_string(),
            }),
        }),
    };
    let result = convert_expr(&expr, &type_mapper).unwrap();
    let result_str = quote::quote!(#result).to_string();
    eprintln!("[DEBUG] Generated code with self.field: {}", result_str);
    // The addition should be wrapped in parentheses even with self.field
    // Note: self.width becomes self.width.clone() in the converter
    assert!(
        result_str.contains("(self . width . clone () + self . height . clone ())")
            || result_str.contains("(self.width.clone() + self.height.clone())"),
        "Expected parentheses around addition: {}",
        result_str
    );
}

#[test]
fn test_convert_expr_list_empty() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::List(vec![]);
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_list_with_elements() {
    let type_mapper = TypeMapper::default();
    let expr =
        HirExpr::List(vec![HirExpr::Literal(Literal::Int(1)), HirExpr::Literal(Literal::Int(2))]);
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_dict_empty() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Dict(vec![]);
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_tuple() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Tuple(vec![
        HirExpr::Literal(Literal::Int(1)),
        HirExpr::Literal(Literal::String("a".to_string())),
    ]);
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_index() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Index {
        base: Box::new(HirExpr::Var("arr".to_string())),
        index: Box::new(HirExpr::Literal(Literal::Int(0))),
    };
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_attribute() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Attribute {
        value: Box::new(HirExpr::Var("obj".to_string())),
        attr: "field".to_string(),
    };
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_call_simple() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Call { func: "func".to_string(), args: vec![], kwargs: vec![] };
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_call_with_args() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Call {
        func: "func".to_string(),
        args: vec![HirExpr::Literal(Literal::Int(1))],
        kwargs: vec![],
    };
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_unary_not() {
    let type_mapper = TypeMapper::default();
    let expr = HirExpr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(HirExpr::Literal(Literal::Bool(true))),
    };
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_expr_unary_neg() {
    let type_mapper = TypeMapper::default();
    let expr =
        HirExpr::Unary { op: UnaryOp::Neg, operand: Box::new(HirExpr::Literal(Literal::Int(42))) };
    let result = convert_expr(&expr, &type_mapper);
    assert!(result.is_ok());
}

// =========================================================================
// convert_stmt tests
// =========================================================================

#[test]
fn test_convert_stmt_pass() {
    let type_mapper = TypeMapper::default();
    let stmt = HirStmt::Pass;
    let result = convert_stmt(&stmt, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_stmt_break() {
    let type_mapper = TypeMapper::default();
    let stmt = HirStmt::Break { label: None };
    let result = convert_stmt(&stmt, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_stmt_continue() {
    let type_mapper = TypeMapper::default();
    let stmt = HirStmt::Continue { label: None };
    let result = convert_stmt(&stmt, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_stmt_expr() {
    let type_mapper = TypeMapper::default();
    let stmt = HirStmt::Expr(HirExpr::Literal(Literal::Int(42)));
    let result = convert_stmt(&stmt, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_stmt_assign() {
    let type_mapper = TypeMapper::default();
    let stmt = HirStmt::Assign {
        target: AssignTarget::Symbol("x".to_string()),
        value: HirExpr::Literal(Literal::Int(42)),
        type_annotation: None,
    };
    let result = convert_stmt(&stmt, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_stmt_return_some() {
    let type_mapper = TypeMapper::default();
    let stmt = HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))));
    let result = convert_stmt(&stmt, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_stmt_return_none() {
    let type_mapper = TypeMapper::default();
    let stmt = HirStmt::Return(None);
    let result = convert_stmt(&stmt, &type_mapper);
    assert!(result.is_ok());
}

// =========================================================================
// convert_block tests
// =========================================================================

#[test]
fn test_convert_block_empty() {
    let type_mapper = TypeMapper::default();
    let stmts: Vec<HirStmt> = vec![];
    let result = convert_block(&stmts, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_block_single_stmt() {
    let type_mapper = TypeMapper::default();
    let stmts = vec![HirStmt::Pass];
    let result = convert_block(&stmts, &type_mapper);
    assert!(result.is_ok());
}

#[test]
fn test_convert_block_multiple_stmts() {
    let type_mapper = TypeMapper::default();
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        },
        HirStmt::Assign {
            target: AssignTarget::Symbol("y".to_string()),
            value: HirExpr::Literal(Literal::Int(2)),
            type_annotation: None,
        },
    ];
    let result = convert_block(&stmts, &type_mapper);
    assert!(result.is_ok());
}

// =========================================================================
// DEPYLER-1049: time module method calls in class methods
// =========================================================================

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_1049_time_time_in_class_method() {
    // Test that time.time() is converted to std::time in class methods
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);

    // time.time() method call
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("time".to_string())),
        method: "time".to_string(),
        args: vec![],
        kwargs: vec![],
    };

    let result = converter.convert(&expr).unwrap();
    let result_str = quote::quote!(#result).to_string();

    // Should contain SystemTime and UNIX_EPOCH
    assert!(result_str.contains("SystemTime"), "Should use std::time::SystemTime");
    assert!(result_str.contains("UNIX_EPOCH"), "Should use UNIX_EPOCH");
    assert!(result_str.contains("as_secs_f64"), "Should return f64 seconds");
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_1049_time_sleep_in_class_method() {
    // Test that time.sleep(n) is converted to std::thread::sleep
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);

    // time.sleep(1.5) method call
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("time".to_string())),
        method: "sleep".to_string(),
        args: vec![HirExpr::Literal(Literal::Float(1.5))],
        kwargs: vec![],
    };

    let result = converter.convert(&expr).unwrap();
    let result_str = quote::quote!(#result).to_string();

    // Should contain thread::sleep and Duration
    assert!(
        result_str.contains("thread") && result_str.contains("sleep"),
        "Should use std::thread::sleep"
    );
    assert!(result_str.contains("Duration"), "Should use Duration");
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_1049_time_monotonic_in_class_method() {
    // Test that time.monotonic() is converted to std::time::Instant
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);

    // time.monotonic() method call
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("time".to_string())),
        method: "monotonic".to_string(),
        args: vec![],
        kwargs: vec![],
    };

    let result = converter.convert(&expr).unwrap();
    let result_str = quote::quote!(#result).to_string();

    // Should contain Instant::now
    assert!(result_str.contains("Instant"), "Should use std::time::Instant");
    assert!(result_str.contains("now"), "Should call now()");
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_1200_re_search_in_class_method() {
    // Test that re.search() is converted to regex::Regex in class methods
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);

    // re.search(r"world", "hello world") method call
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("re".to_string())),
        method: "search".to_string(),
        args: vec![
            HirExpr::Literal(Literal::String("world".to_string())),
            HirExpr::Literal(Literal::String("hello world".to_string())),
        ],
        kwargs: vec![],
    };

    let result = converter.convert(&expr).unwrap();
    let result_str = quote::quote!(#result).to_string();

    // Should contain regex::Regex
    assert!(
        result_str.contains("regex") || result_str.contains("Regex"),
        "Should use regex crate. Got: {}",
        result_str
    );
    assert!(!result_str.contains("None"), "Should NOT generate None. Got: {}", result_str);
}

// =========================================================================
// convert_symbol_assignment tests
// =========================================================================

#[test]
fn test_convert_symbol_assignment_immutable() {
    let value_expr: syn::Expr = parse_quote! { 42 };
    let mutable_vars = std::collections::HashSet::new();
    let result = convert_symbol_assignment("x", value_expr, &mutable_vars).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("let x"), "Should create let binding: {}", code);
    assert!(!code.contains("mut"), "Should not be mutable: {}", code);
}

#[test]
fn test_convert_symbol_assignment_mutable() {
    let value_expr: syn::Expr = parse_quote! { 0 };
    let mut mutable_vars = std::collections::HashSet::new();
    mutable_vars.insert("counter".to_string());
    let result = convert_symbol_assignment("counter", value_expr, &mutable_vars).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("mut"), "Should be mutable: {}", code);
}

#[test]
fn test_convert_symbol_assignment_keyword_name() {
    let value_expr: syn::Expr = parse_quote! { "hello" };
    let mutable_vars = std::collections::HashSet::new();
    let result = convert_symbol_assignment("r#type", value_expr, &mutable_vars).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("let"), "Should create let binding: {}", code);
}

// =========================================================================
// convert_attribute_assignment tests
// =========================================================================

#[test]
fn test_convert_attribute_assignment_self_field() {
    let type_mapper = TypeMapper::default();
    let base = HirExpr::Var("self".to_string());
    let value_expr: syn::Expr = parse_quote! { 42 };
    let result = convert_attribute_assignment(&base, "x", value_expr, &type_mapper).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("self") && code.contains("x"), "Should set self.x: {}", code);
}

#[test]
fn test_convert_attribute_assignment_other_object() {
    let type_mapper = TypeMapper::default();
    let base = HirExpr::Var("obj".to_string());
    let value_expr: syn::Expr = parse_quote! { "hello" };
    let result = convert_attribute_assignment(&base, "name", value_expr, &type_mapper).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("name"), "Should set obj.name: {}", code);
}

// =========================================================================
// convert_index_assignment tests
// =========================================================================

#[test]
fn test_convert_index_assignment_simple() {
    let type_mapper = TypeMapper::default();
    let base = HirExpr::Var("data".to_string());
    let index = HirExpr::Literal(Literal::String("key".to_string()));
    let value_expr: syn::Expr = parse_quote! { 42 };
    let result = convert_index_assignment(&base, &index, value_expr, &type_mapper).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("insert"), "Should use insert for dict assignment: {}", code);
}

#[test]
fn test_convert_index_assignment_int_index() {
    let type_mapper = TypeMapper::default();
    let base = HirExpr::Var("arr".to_string());
    let index = HirExpr::Literal(Literal::Int(0));
    let value_expr: syn::Expr = parse_quote! { 99 };
    let result = convert_index_assignment(&base, &index, value_expr, &type_mapper).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("insert"), "Should use insert: {}", code);
}

// =========================================================================
// convert_assign_stmt_with_mutable_vars tests
// =========================================================================

#[test]
fn test_assign_stmt_with_mutable_vars_symbol() {
    let type_mapper = TypeMapper::default();
    let value_expr: syn::Expr = parse_quote! { 10 };
    let mut mutable_vars = std::collections::HashSet::new();
    mutable_vars.insert("x".to_string());
    let result = convert_assign_stmt_with_mutable_vars(
        &AssignTarget::Symbol("x".to_string()),
        value_expr,
        &type_mapper,
        &mutable_vars,
    )
    .unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("mut"), "Should be mutable: {}", code);
}

#[test]
fn test_assign_stmt_with_mutable_vars_attribute() {
    let type_mapper = TypeMapper::default();
    let value_expr: syn::Expr = parse_quote! { 42 };
    let mutable_vars = std::collections::HashSet::new();
    let target = AssignTarget::Attribute {
        value: Box::new(HirExpr::Var("self".to_string())),
        attr: "value".to_string(),
    };
    let result =
        convert_assign_stmt_with_mutable_vars(&target, value_expr, &type_mapper, &mutable_vars)
            .unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("value"), "Should assign to attribute: {}", code);
}

#[test]
fn test_assign_stmt_with_mutable_vars_index() {
    let type_mapper = TypeMapper::default();
    let value_expr: syn::Expr = parse_quote! { 42 };
    let mutable_vars = std::collections::HashSet::new();
    let target = AssignTarget::Index {
        base: Box::new(HirExpr::Var("data".to_string())),
        index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
    };
    let result =
        convert_assign_stmt_with_mutable_vars(&target, value_expr, &type_mapper, &mutable_vars)
            .unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("insert"), "Should use insert: {}", code);
}

#[test]
fn test_assign_stmt_with_mutable_vars_tuple() {
    let type_mapper = TypeMapper::default();
    let value_expr: syn::Expr = parse_quote! { (1, 2) };
    let mutable_vars = std::collections::HashSet::new();
    let target = AssignTarget::Tuple(vec![
        AssignTarget::Symbol("a".to_string()),
        AssignTarget::Symbol("b".to_string()),
    ]);
    let result =
        convert_assign_stmt_with_mutable_vars(&target, value_expr, &type_mapper, &mutable_vars)
            .unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("a") && code.contains("b"), "Should unpack tuple: {}", code);
}

// =========================================================================
// convert_stmt_with_context tests
// =========================================================================

#[test]
fn test_convert_stmt_with_context_if() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![HirStmt::Pass],
        else_body: None,
    };
    let result = convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map);
    assert!(result.is_ok());
}

#[test]
fn test_convert_stmt_with_context_if_else() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
        else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0))))]),
    };
    let result =
        convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("if") && code.contains("else"), "Should have if/else: {}", code);
}

#[test]
fn test_convert_stmt_with_context_while() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::While {
        condition: HirExpr::Literal(Literal::Bool(true)),
        body: vec![HirStmt::Break { label: None }],
    };
    let result =
        convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("while"), "Should have while: {}", code);
}

#[test]
fn test_convert_stmt_with_context_for_simple() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(10))],
            kwargs: vec![],
        },
        body: vec![HirStmt::Pass],
    };
    let result =
        convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("for"), "Should have for loop: {}", code);
}

#[test]
fn test_convert_stmt_with_context_for_tuple_target() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::For {
        target: AssignTarget::Tuple(vec![
            AssignTarget::Symbol("k".to_string()),
            AssignTarget::Symbol("v".to_string()),
        ]),
        iter: HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "items".to_string(),
            args: vec![],
            kwargs: vec![],
        },
        body: vec![HirStmt::Pass],
    };
    let result =
        convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("k") && code.contains("v"), "Should unpack tuple: {}", code);
    assert!(code.contains("iter"), "Should use .iter() for .items(): {}", code);
}

#[test]
fn test_convert_stmt_with_context_for_dict_keys() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::For {
        target: AssignTarget::Symbol("k".to_string()),
        iter: HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "keys".to_string(),
            args: vec![],
            kwargs: vec![],
        },
        body: vec![HirStmt::Pass],
    };
    let result =
        convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("keys"), "Should use .keys(): {}", code);
}

#[test]
fn test_convert_stmt_with_context_for_dict_values() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::For {
        target: AssignTarget::Symbol("v".to_string()),
        iter: HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "values".to_string(),
            args: vec![],
            kwargs: vec![],
        },
        body: vec![HirStmt::Pass],
    };
    let result =
        convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("values"), "Should use .values(): {}", code);
}

#[test]
fn test_convert_stmt_with_context_return_some() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::Return(Some(HirExpr::Literal(Literal::String("done".to_string()))));
    let result =
        convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("return"), "Should have return: {}", code);
}

#[test]
fn test_convert_stmt_with_context_return_none() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::Return(None);
    let result =
        convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("return"), "Should have return: {}", code);
}

#[test]
fn test_convert_stmt_with_context_pure_expr() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmt = HirStmt::Expr(HirExpr::Var("x".to_string()));
    let result =
        convert_stmt_with_context(&stmt, &type_mapper, false, &empty_set, &empty_map).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("let _"), "Pure expr should use let _: {}", code);
}

// =========================================================================
// convert_stmt_with_mutable_vars tests
// =========================================================================

#[test]
fn test_convert_stmt_with_mutable_vars_assign() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let mut mutable_vars = std::collections::HashSet::new();
    mutable_vars.insert("x".to_string());
    let stmt = HirStmt::Assign {
        target: AssignTarget::Symbol("x".to_string()),
        value: HirExpr::Literal(Literal::Int(42)),
        type_annotation: None,
    };
    let result = convert_stmt_with_mutable_vars(
        &stmt,
        &type_mapper,
        false,
        &empty_set,
        &empty_map,
        &mutable_vars,
    )
    .unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("mut"), "Should be mutable: {}", code);
}

#[test]
fn test_convert_stmt_with_mutable_vars_non_assign_delegates() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let mutable_vars = std::collections::HashSet::new();
    let stmt = HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0))));
    let result = convert_stmt_with_mutable_vars(
        &stmt,
        &type_mapper,
        false,
        &empty_set,
        &empty_map,
        &mutable_vars,
    );
    assert!(result.is_ok());
}

// =========================================================================
// convert_condition_expr tests
// =========================================================================

#[test]
fn test_convert_condition_expr_bool() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let expr = HirExpr::Literal(Literal::Bool(true));
    let result = convert_condition_expr(&expr, &type_mapper, false, &empty_set, &empty_map);
    assert!(result.is_ok());
}

#[test]
fn test_convert_condition_expr_comparison() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let expr = HirExpr::Binary {
        op: BinOp::Gt,
        left: Box::new(HirExpr::Var("x".to_string())),
        right: Box::new(HirExpr::Literal(Literal::Int(0))),
    };
    let result = convert_condition_expr(&expr, &type_mapper, false, &empty_set, &empty_map);
    assert!(result.is_ok());
}

#[test]
fn test_convert_condition_expr_var() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let expr = HirExpr::Var("flag".to_string());
    let result = convert_condition_expr(&expr, &type_mapper, false, &empty_set, &empty_map);
    assert!(result.is_ok());
}

// =========================================================================
// ExprConverter constructor tests
// =========================================================================

#[test]
fn test_expr_converter_new() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    assert!(!converter.is_classmethod);
}

#[test]
fn test_expr_converter_with_classmethod() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::with_classmethod(&type_mapper, true);
    assert!(converter.is_classmethod);
}

#[test]
fn test_expr_converter_with_classmethod_false() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::with_classmethod(&type_mapper, false);
    assert!(!converter.is_classmethod);
}

#[test]
fn test_expr_converter_with_varargs() {
    let type_mapper = TypeMapper::default();
    let mut vararg_functions = std::collections::HashSet::new();
    vararg_functions.insert("my_func".to_string());
    let converter = ExprConverter::with_varargs(&type_mapper, false, &vararg_functions);
    assert!(!converter.is_classmethod);
}

// =========================================================================
// ExprConverter.convert - additional expression types
// =========================================================================

#[test]
fn test_expr_converter_convert_list_empty() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::List(vec![]);
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_list_with_elements() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr =
        HirExpr::List(vec![HirExpr::Literal(Literal::Int(1)), HirExpr::Literal(Literal::Int(2))]);
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_dict_empty() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Dict(vec![]);
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_dict_with_items() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Dict(vec![(
        HirExpr::Literal(Literal::String("a".to_string())),
        HirExpr::Literal(Literal::Int(1)),
    )]);
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_tuple() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Tuple(vec![
        HirExpr::Literal(Literal::Int(1)),
        HirExpr::Literal(Literal::String("a".to_string())),
    ]);
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_set() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr =
        HirExpr::Set(vec![HirExpr::Literal(Literal::Int(1)), HirExpr::Literal(Literal::Int(2))]);
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_frozenset() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::FrozenSet(vec![HirExpr::Literal(Literal::Int(1))]);
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_index() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Index {
        base: Box::new(HirExpr::Var("arr".to_string())),
        index: Box::new(HirExpr::Literal(Literal::Int(0))),
    };
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_attribute() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Attribute {
        value: Box::new(HirExpr::Var("obj".to_string())),
        attr: "field".to_string(),
    };
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_self_attribute_classmethod() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::with_classmethod(&type_mapper, true);
    let expr = HirExpr::Attribute {
        value: Box::new(HirExpr::Var("self".to_string())),
        attr: "name".to_string(),
    };
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_if_expr() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::IfExpr {
        test: Box::new(HirExpr::Literal(Literal::Bool(true))),
        body: Box::new(HirExpr::Literal(Literal::Int(1))),
        orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("if") && code.contains("else"), "Should have ternary: {}", code);
}

#[test]
fn test_expr_converter_convert_fstring() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::FString {
        parts: vec![
            FStringPart::Literal("hello ".to_string()),
            FStringPart::Expr(Box::new(HirExpr::Var("name".to_string()))),
        ],
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("format"), "Should use format!: {}", code);
}

#[test]
fn test_expr_converter_convert_lambda() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Lambda {
        params: vec!["x".to_string()],
        body: Box::new(HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        }),
    };
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_method_call() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("items".to_string())),
        method: "append".to_string(),
        args: vec![HirExpr::Literal(Literal::Int(1))],
        kwargs: vec![],
    };
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_unary_neg() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr =
        HirExpr::Unary { op: UnaryOp::Neg, operand: Box::new(HirExpr::Var("x".to_string())) };
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_unary_not() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(HirExpr::Literal(Literal::Bool(false))),
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("!"), "Should negate: {}", code);
}

#[test]
fn test_expr_converter_convert_binary_floor_div() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Binary {
        op: BinOp::FloorDiv,
        left: Box::new(HirExpr::Literal(Literal::Int(10))),
        right: Box::new(HirExpr::Literal(Literal::Int(3))),
    };
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_binary_pow() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Binary {
        op: BinOp::Pow,
        left: Box::new(HirExpr::Literal(Literal::Int(2))),
        right: Box::new(HirExpr::Literal(Literal::Int(3))),
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("pow"), "Should use pow: {}", code);
}

#[test]
fn test_expr_converter_convert_binary_in() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Binary {
        op: BinOp::In,
        left: Box::new(HirExpr::Literal(Literal::String("a".to_string()))),
        right: Box::new(HirExpr::Var("items".to_string())),
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    // DEPYLER-99MODE-S9: For unknown container types (like "items"), the fallback
    // uses .get().is_some() which works for both HashMap and HashSet
    assert!(
        code.contains("contains") || code.contains("is_some"),
        "Should use contains or get().is_some() for 'in': {}",
        code
    );
}

#[test]
fn test_expr_converter_convert_binary_not_in() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Binary {
        op: BinOp::NotIn,
        left: Box::new(HirExpr::Literal(Literal::String("a".to_string()))),
        right: Box::new(HirExpr::Var("items".to_string())),
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    // DEPYLER-99MODE-S9: For unknown container types, the fallback
    // uses .get().is_none() which works for both HashMap and HashSet
    assert!(
        code.contains("contains") || code.contains("is_none"),
        "Should use contains or get().is_none() for 'not in': {}",
        code
    );
}

#[test]
fn test_expr_converter_convert_call_len() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Call {
        func: "len".to_string(),
        args: vec![HirExpr::Var("items".to_string())],
        kwargs: vec![],
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("len"), "Should convert len(): {}", code);
}

#[test]
fn test_expr_converter_convert_call_print() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Call {
        func: "print".to_string(),
        args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
        kwargs: vec![],
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("println"), "Should convert to println!: {}", code);
}

#[test]
fn test_expr_converter_convert_call_range() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Call {
        func: "range".to_string(),
        args: vec![HirExpr::Literal(Literal::Int(10))],
        kwargs: vec![],
    };
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_call_int_cast() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Call {
        func: "int".to_string(),
        args: vec![HirExpr::Literal(Literal::Float(3.15))],
        kwargs: vec![],
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(
        code.contains("as") || code.contains("i64") || code.contains("i32"),
        "Should cast to int: {}",
        code
    );
}

#[test]
fn test_expr_converter_convert_call_str() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Call {
        func: "str".to_string(),
        args: vec![HirExpr::Literal(Literal::Int(42))],
        kwargs: vec![],
    };
    let result = converter.convert(&expr).unwrap();
    let code = quote::quote!(#result).to_string();
    assert!(code.contains("to_string"), "Should convert to to_string(): {}", code);
}

#[test]
fn test_expr_converter_convert_variable_self() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::new(&type_mapper);
    let expr = HirExpr::Var("self".to_string());
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_expr_converter_convert_variable_classmethod() {
    let type_mapper = TypeMapper::default();
    let converter = ExprConverter::with_classmethod(&type_mapper, true);
    let expr = HirExpr::Var("self".to_string());
    let result = converter.convert(&expr);
    assert!(result.is_ok());
}

// =========================================================================
// convert_body_with_context tests
// =========================================================================

#[test]
fn test_convert_body_with_context_empty() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmts: Vec<HirStmt> = vec![];
    let result = convert_body_with_context(&stmts, &type_mapper, false, &empty_set, &empty_map);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_convert_body_with_context_multiple_stmts() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        },
        HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
    ];
    let result = convert_body_with_context(&stmts, &type_mapper, false, &empty_set, &empty_map);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_convert_body_with_context_classmethod() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmts = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0))))];
    let result = convert_body_with_context(&stmts, &type_mapper, true, &empty_set, &empty_map);
    assert!(result.is_ok());
}

// =========================================================================
// convert_block_with_context tests
// =========================================================================

#[test]
fn test_convert_block_with_context_empty() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let result = convert_block_with_context(&[], &type_mapper, false, &empty_set, &empty_map);
    assert!(result.is_ok());
}

#[test]
fn test_convert_block_with_context_single() {
    let type_mapper = TypeMapper::default();
    let empty_set = std::collections::HashSet::new();
    let empty_map = std::collections::HashMap::new();
    let stmts = vec![HirStmt::Pass];
    let result = convert_block_with_context(&stmts, &type_mapper, false, &empty_set, &empty_map);
    assert!(result.is_ok());
}

// =========================================================================
// find_mutable_vars edge cases
// =========================================================================

#[test]
fn test_find_mutable_vars_extend_method() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("lst".to_string()),
            value: HirExpr::List(vec![]),
            type_annotation: None,
        },
        HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".to_string())),
            method: "extend".to_string(),
            args: vec![HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))])],
            kwargs: vec![],
        }),
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("lst"), "extend should mark as mutable");
}

#[test]
fn test_find_mutable_vars_insert_method() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("data".to_string()),
            value: HirExpr::Dict(vec![]),
            type_annotation: None,
        },
        HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "insert".to_string(),
            args: vec![
                HirExpr::Literal(Literal::String("key".to_string())),
                HirExpr::Literal(Literal::Int(1)),
            ],
            kwargs: vec![],
        }),
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("data"), "insert should mark as mutable");
}

#[test]
fn test_find_mutable_vars_pop_method() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("items".to_string()),
            value: HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))]),
            type_annotation: None,
        },
        HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "pop".to_string(),
            args: vec![],
            kwargs: vec![],
        }),
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("items"), "pop should mark as mutable");
}

#[test]
fn test_find_mutable_vars_sort_method() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("items".to_string()),
            value: HirExpr::List(vec![]),
            type_annotation: None,
        },
        HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "sort".to_string(),
            args: vec![],
            kwargs: vec![],
        }),
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(result.contains("items"), "sort should mark as mutable");
}

#[test]
fn test_find_mutable_vars_non_mutating_method() {
    let stmts = vec![
        HirStmt::Assign {
            target: AssignTarget::Symbol("text".to_string()),
            value: HirExpr::Literal(Literal::String("hello".to_string())),
            type_annotation: None,
        },
        HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("text".to_string())),
            method: "upper".to_string(),
            args: vec![],
            kwargs: vec![],
        }),
    ];
    let result = find_mutable_vars_in_body(&stmts);
    assert!(!result.contains("text"), "upper() should NOT mark as mutable");
}

// =========================================================================
// is_pure_expression additional tests
// =========================================================================

#[test]
fn test_is_pure_expression_nested_binary() {
    let expr = HirExpr::Binary {
        op: BinOp::Add,
        left: Box::new(HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Var("b".to_string())),
        }),
        right: Box::new(HirExpr::Literal(Literal::Int(1))),
    };
    assert!(is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_binary_with_call_impure() {
    let expr = HirExpr::Binary {
        op: BinOp::Add,
        left: Box::new(HirExpr::Call { func: "foo".to_string(), args: vec![], kwargs: vec![] }),
        right: Box::new(HirExpr::Literal(Literal::Int(1))),
    };
    assert!(!is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_nested_index() {
    let expr = HirExpr::Index {
        base: Box::new(HirExpr::Index {
            base: Box::new(HirExpr::Var("matrix".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        }),
        index: Box::new(HirExpr::Literal(Literal::Int(1))),
    };
    assert!(is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_set_not_pure() {
    let expr = HirExpr::Set(vec![HirExpr::Literal(Literal::Int(1))]);
    assert!(!is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_list_not_pure() {
    let expr = HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))]);
    assert!(!is_pure_expression_direct(&expr));
}

#[test]
fn test_is_pure_expression_dict_not_pure() {
    let expr = HirExpr::Dict(vec![]);
    assert!(!is_pure_expression_direct(&expr));
}
