//! Simplified HIR for backend usage

use serde::{Deserialize, Serialize};

/// Simplified HIR structure for backend transpilation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hir {
    pub root: HirExpr,
    pub metadata: HirMetadata,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HirMetadata {
    pub source_file: Option<String>,
    pub module_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirExpr {
    Literal(HirLiteral),
    Identifier(String),
    Binary {
        left: Box<HirExpr>,
        op: HirBinaryOp,
        right: Box<HirExpr>,
    },
    Unary {
        op: HirUnaryOp,
        operand: Box<HirExpr>,
    },
    Call {
        func: Box<HirExpr>,
        args: Vec<HirExpr>,
    },
    If {
        condition: Box<HirExpr>,
        then_branch: Box<HirExpr>,
        else_branch: Option<Box<HirExpr>>,
    },
    Block(Vec<HirStatement>),
    List(Vec<HirExpr>),
    Function {
        name: String,
        params: Vec<HirParam>,
        body: Box<HirExpr>,
        is_async: bool,
        return_type: Option<HirType>,
    },
    Lambda {
        params: Vec<HirParam>,
        body: Box<HirExpr>,
    },
    For {
        var: String,
        iter: Box<HirExpr>,
        body: Box<HirExpr>,
    },
    While {
        condition: Box<HirExpr>,
        body: Box<HirExpr>,
    },
    Return(Option<Box<HirExpr>>),
    Break(Option<String>),
    Continue(Option<String>),
    Await(Box<HirExpr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirStatement {
    Let {
        name: String,
        value: Box<HirExpr>,
        is_mutable: bool,
    },
    Expression(Box<HirExpr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirParam {
    pub name: String,
    pub typ: Option<HirType>,
    pub default: Option<Box<HirExpr>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirLiteral {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirUnaryOp {
    Not,
    Negate,
    BitwiseNot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirType {
    Int,
    Float,
    String,
    Bool,
    List(Box<HirType>),
    Optional(Box<HirType>),
    Named(String),
    Any,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Hir struct tests
    // ========================================================================

    #[test]
    fn test_hir_new() {
        let hir = Hir {
            root: HirExpr::Literal(HirLiteral::Integer(42)),
            metadata: HirMetadata::default(),
        };
        assert!(matches!(
            hir.root,
            HirExpr::Literal(HirLiteral::Integer(42))
        ));
    }

    #[test]
    fn test_hir_clone() {
        let hir = Hir {
            root: HirExpr::Identifier("x".to_string()),
            metadata: HirMetadata {
                source_file: Some("test.py".to_string()),
                module_name: Some("test".to_string()),
            },
        };
        let cloned = hir.clone();
        assert_eq!(hir, cloned);
    }

    #[test]
    fn test_hir_debug() {
        let hir = Hir {
            root: HirExpr::Literal(HirLiteral::Bool(true)),
            metadata: HirMetadata::default(),
        };
        let debug_str = format!("{:?}", hir);
        assert!(debug_str.contains("Hir"));
        assert!(debug_str.contains("Bool"));
    }

    #[test]
    fn test_hir_serialize_deserialize() {
        let hir = Hir {
            root: HirExpr::Literal(HirLiteral::String("hello".to_string())),
            metadata: HirMetadata {
                source_file: Some("test.py".to_string()),
                module_name: None,
            },
        };
        let json = serde_json::to_string(&hir).unwrap();
        let deserialized: Hir = serde_json::from_str(&json).unwrap();
        assert_eq!(hir, deserialized);
    }

    // ========================================================================
    // HirMetadata tests
    // ========================================================================

    #[test]
    fn test_hir_metadata_default() {
        let meta = HirMetadata::default();
        assert!(meta.source_file.is_none());
        assert!(meta.module_name.is_none());
    }

    #[test]
    fn test_hir_metadata_with_values() {
        let meta = HirMetadata {
            source_file: Some("main.py".to_string()),
            module_name: Some("main".to_string()),
        };
        assert_eq!(meta.source_file.as_deref(), Some("main.py"));
        assert_eq!(meta.module_name.as_deref(), Some("main"));
    }

    #[test]
    fn test_hir_metadata_clone() {
        let meta = HirMetadata {
            source_file: Some("test.py".to_string()),
            module_name: Some("test_mod".to_string()),
        };
        let cloned = meta.clone();
        assert_eq!(meta, cloned);
    }

    // ========================================================================
    // HirExpr variants tests
    // ========================================================================

    #[test]
    fn test_hir_expr_literal() {
        let expr = HirExpr::Literal(HirLiteral::Integer(100));
        assert!(matches!(expr, HirExpr::Literal(HirLiteral::Integer(100))));
    }

    #[test]
    fn test_hir_expr_identifier() {
        let expr = HirExpr::Identifier("my_var".to_string());
        assert!(matches!(expr, HirExpr::Identifier(ref s) if s == "my_var"));
    }

    #[test]
    fn test_hir_expr_binary() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
            op: HirBinaryOp::Add,
            right: Box::new(HirExpr::Literal(HirLiteral::Integer(2))),
        };
        assert!(matches!(
            expr,
            HirExpr::Binary {
                op: HirBinaryOp::Add,
                ..
            }
        ));
    }

    #[test]
    fn test_hir_expr_unary() {
        let expr = HirExpr::Unary {
            op: HirUnaryOp::Negate,
            operand: Box::new(HirExpr::Literal(HirLiteral::Integer(5))),
        };
        assert!(matches!(
            expr,
            HirExpr::Unary {
                op: HirUnaryOp::Negate,
                ..
            }
        ));
    }

    #[test]
    fn test_hir_expr_call() {
        let expr = HirExpr::Call {
            func: Box::new(HirExpr::Identifier("print".to_string())),
            args: vec![HirExpr::Literal(HirLiteral::String("hello".to_string()))],
        };
        assert!(matches!(expr, HirExpr::Call { .. }));
    }

    #[test]
    fn test_hir_expr_if() {
        let expr = HirExpr::If {
            condition: Box::new(HirExpr::Literal(HirLiteral::Bool(true))),
            then_branch: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
            else_branch: Some(Box::new(HirExpr::Literal(HirLiteral::Integer(0)))),
        };
        assert!(matches!(expr, HirExpr::If { .. }));
    }

    #[test]
    fn test_hir_expr_if_no_else() {
        let expr = HirExpr::If {
            condition: Box::new(HirExpr::Literal(HirLiteral::Bool(true))),
            then_branch: Box::new(HirExpr::Literal(HirLiteral::None)),
            else_branch: None,
        };
        if let HirExpr::If { else_branch, .. } = expr {
            assert!(else_branch.is_none());
        }
    }

    #[test]
    fn test_hir_expr_block() {
        let expr = HirExpr::Block(vec![
            HirStatement::Let {
                name: "x".to_string(),
                value: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
                is_mutable: false,
            },
            HirStatement::Expression(Box::new(HirExpr::Identifier("x".to_string()))),
        ]);
        if let HirExpr::Block(stmts) = expr {
            assert_eq!(stmts.len(), 2);
        }
    }

    #[test]
    fn test_hir_expr_list() {
        let expr = HirExpr::List(vec![
            HirExpr::Literal(HirLiteral::Integer(1)),
            HirExpr::Literal(HirLiteral::Integer(2)),
            HirExpr::Literal(HirLiteral::Integer(3)),
        ]);
        if let HirExpr::List(items) = expr {
            assert_eq!(items.len(), 3);
        }
    }

    #[test]
    fn test_hir_expr_function() {
        let expr = HirExpr::Function {
            name: "add".to_string(),
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
                op: HirBinaryOp::Add,
                right: Box::new(HirExpr::Identifier("b".to_string())),
            }),
            is_async: false,
            return_type: Some(HirType::Int),
        };
        if let HirExpr::Function { name, is_async, .. } = expr {
            assert_eq!(name, "add");
            assert!(!is_async);
        }
    }

    #[test]
    fn test_hir_expr_async_function() {
        let expr = HirExpr::Function {
            name: "fetch".to_string(),
            params: vec![],
            body: Box::new(HirExpr::Literal(HirLiteral::None)),
            is_async: true,
            return_type: None,
        };
        if let HirExpr::Function { is_async, .. } = expr {
            assert!(is_async);
        }
    }

    #[test]
    fn test_hir_expr_lambda() {
        let expr = HirExpr::Lambda {
            params: vec![HirParam {
                name: "x".to_string(),
                typ: None,
                default: None,
            }],
            body: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Identifier("x".to_string())),
                op: HirBinaryOp::Multiply,
                right: Box::new(HirExpr::Literal(HirLiteral::Integer(2))),
            }),
        };
        assert!(matches!(expr, HirExpr::Lambda { .. }));
    }

    #[test]
    fn test_hir_expr_for() {
        let expr = HirExpr::For {
            var: "i".to_string(),
            iter: Box::new(HirExpr::List(vec![])),
            body: Box::new(HirExpr::Literal(HirLiteral::None)),
        };
        if let HirExpr::For { var, .. } = expr {
            assert_eq!(var, "i");
        }
    }

    #[test]
    fn test_hir_expr_while() {
        let expr = HirExpr::While {
            condition: Box::new(HirExpr::Literal(HirLiteral::Bool(true))),
            body: Box::new(HirExpr::Break(None)),
        };
        assert!(matches!(expr, HirExpr::While { .. }));
    }

    #[test]
    fn test_hir_expr_return() {
        let expr = HirExpr::Return(Some(Box::new(HirExpr::Literal(HirLiteral::Integer(42)))));
        assert!(matches!(expr, HirExpr::Return(Some(_))));

        let expr_none = HirExpr::Return(None);
        assert!(matches!(expr_none, HirExpr::Return(None)));
    }

    #[test]
    fn test_hir_expr_break() {
        let expr = HirExpr::Break(None);
        assert!(matches!(expr, HirExpr::Break(None)));

        let expr_labeled = HirExpr::Break(Some("outer".to_string()));
        assert!(matches!(expr_labeled, HirExpr::Break(Some(_))));
    }

    #[test]
    fn test_hir_expr_continue() {
        let expr = HirExpr::Continue(None);
        assert!(matches!(expr, HirExpr::Continue(None)));

        let expr_labeled = HirExpr::Continue(Some("loop1".to_string()));
        assert!(matches!(expr_labeled, HirExpr::Continue(Some(_))));
    }

    #[test]
    fn test_hir_expr_await() {
        let expr = HirExpr::Await(Box::new(HirExpr::Call {
            func: Box::new(HirExpr::Identifier("fetch".to_string())),
            args: vec![],
        }));
        assert!(matches!(expr, HirExpr::Await(_)));
    }

    // ========================================================================
    // HirStatement tests
    // ========================================================================

    #[test]
    fn test_hir_statement_let() {
        let stmt = HirStatement::Let {
            name: "x".to_string(),
            value: Box::new(HirExpr::Literal(HirLiteral::Integer(10))),
            is_mutable: false,
        };
        if let HirStatement::Let { is_mutable, .. } = stmt {
            assert!(!is_mutable);
        }
    }

    #[test]
    fn test_hir_statement_let_mutable() {
        let stmt = HirStatement::Let {
            name: "counter".to_string(),
            value: Box::new(HirExpr::Literal(HirLiteral::Integer(0))),
            is_mutable: true,
        };
        if let HirStatement::Let { is_mutable, .. } = stmt {
            assert!(is_mutable);
        }
    }

    #[test]
    fn test_hir_statement_expression() {
        let stmt = HirStatement::Expression(Box::new(HirExpr::Call {
            func: Box::new(HirExpr::Identifier("print".to_string())),
            args: vec![],
        }));
        assert!(matches!(stmt, HirStatement::Expression(_)));
    }

    // ========================================================================
    // HirParam tests
    // ========================================================================

    #[test]
    fn test_hir_param_simple() {
        let param = HirParam {
            name: "x".to_string(),
            typ: None,
            default: None,
        };
        assert_eq!(param.name, "x");
        assert!(param.typ.is_none());
        assert!(param.default.is_none());
    }

    #[test]
    fn test_hir_param_with_type() {
        let param = HirParam {
            name: "count".to_string(),
            typ: Some(HirType::Int),
            default: None,
        };
        assert!(matches!(param.typ, Some(HirType::Int)));
    }

    #[test]
    fn test_hir_param_with_default() {
        let param = HirParam {
            name: "value".to_string(),
            typ: Some(HirType::Int),
            default: Some(Box::new(HirExpr::Literal(HirLiteral::Integer(0)))),
        };
        assert!(param.default.is_some());
    }

    // ========================================================================
    // HirLiteral tests
    // ========================================================================

    #[test]
    fn test_hir_literal_integer() {
        let lit = HirLiteral::Integer(42);
        assert!(matches!(lit, HirLiteral::Integer(42)));
    }

    #[test]
    fn test_hir_literal_float() {
        let lit = HirLiteral::Float(3.15);
        if let HirLiteral::Float(v) = lit {
            assert!((v - 3.15).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_hir_literal_string() {
        let lit = HirLiteral::String("hello world".to_string());
        assert!(matches!(lit, HirLiteral::String(ref s) if s == "hello world"));
    }

    #[test]
    fn test_hir_literal_bool_true() {
        let lit = HirLiteral::Bool(true);
        assert!(matches!(lit, HirLiteral::Bool(true)));
    }

    #[test]
    fn test_hir_literal_bool_false() {
        let lit = HirLiteral::Bool(false);
        assert!(matches!(lit, HirLiteral::Bool(false)));
    }

    #[test]
    fn test_hir_literal_none() {
        let lit = HirLiteral::None;
        assert!(matches!(lit, HirLiteral::None));
    }

    // ========================================================================
    // HirBinaryOp tests
    // ========================================================================

    #[test]
    fn test_hir_binary_op_arithmetic() {
        assert!(matches!(HirBinaryOp::Add, HirBinaryOp::Add));
        assert!(matches!(HirBinaryOp::Subtract, HirBinaryOp::Subtract));
        assert!(matches!(HirBinaryOp::Multiply, HirBinaryOp::Multiply));
        assert!(matches!(HirBinaryOp::Divide, HirBinaryOp::Divide));
        assert!(matches!(HirBinaryOp::Modulo, HirBinaryOp::Modulo));
        assert!(matches!(HirBinaryOp::Power, HirBinaryOp::Power));
    }

    #[test]
    fn test_hir_binary_op_comparison() {
        assert!(matches!(HirBinaryOp::Equal, HirBinaryOp::Equal));
        assert!(matches!(HirBinaryOp::NotEqual, HirBinaryOp::NotEqual));
        assert!(matches!(HirBinaryOp::Less, HirBinaryOp::Less));
        assert!(matches!(HirBinaryOp::LessEqual, HirBinaryOp::LessEqual));
        assert!(matches!(HirBinaryOp::Greater, HirBinaryOp::Greater));
        assert!(matches!(
            HirBinaryOp::GreaterEqual,
            HirBinaryOp::GreaterEqual
        ));
    }

    #[test]
    fn test_hir_binary_op_logical() {
        assert!(matches!(HirBinaryOp::And, HirBinaryOp::And));
        assert!(matches!(HirBinaryOp::Or, HirBinaryOp::Or));
    }

    #[test]
    fn test_hir_binary_op_bitwise() {
        assert!(matches!(HirBinaryOp::BitwiseAnd, HirBinaryOp::BitwiseAnd));
        assert!(matches!(HirBinaryOp::BitwiseOr, HirBinaryOp::BitwiseOr));
        assert!(matches!(HirBinaryOp::BitwiseXor, HirBinaryOp::BitwiseXor));
        assert!(matches!(HirBinaryOp::LeftShift, HirBinaryOp::LeftShift));
        assert!(matches!(HirBinaryOp::RightShift, HirBinaryOp::RightShift));
    }

    #[test]
    fn test_hir_binary_op_clone() {
        let op = HirBinaryOp::Add;
        let cloned = op;
        assert_eq!(op, cloned);
    }

    #[test]
    fn test_hir_binary_op_debug() {
        let debug_str = format!("{:?}", HirBinaryOp::Add);
        assert_eq!(debug_str, "Add");
    }

    // ========================================================================
    // HirUnaryOp tests
    // ========================================================================

    #[test]
    fn test_hir_unary_op_not() {
        assert!(matches!(HirUnaryOp::Not, HirUnaryOp::Not));
    }

    #[test]
    fn test_hir_unary_op_negate() {
        assert!(matches!(HirUnaryOp::Negate, HirUnaryOp::Negate));
    }

    #[test]
    fn test_hir_unary_op_bitwise_not() {
        assert!(matches!(HirUnaryOp::BitwiseNot, HirUnaryOp::BitwiseNot));
    }

    #[test]
    fn test_hir_unary_op_clone() {
        let op = HirUnaryOp::Not;
        let cloned = op;
        assert_eq!(op, cloned);
    }

    #[test]
    fn test_hir_unary_op_debug() {
        let debug_str = format!("{:?}", HirUnaryOp::Negate);
        assert_eq!(debug_str, "Negate");
    }

    // ========================================================================
    // HirType tests
    // ========================================================================

    #[test]
    fn test_hir_type_primitives() {
        assert!(matches!(HirType::Int, HirType::Int));
        assert!(matches!(HirType::Float, HirType::Float));
        assert!(matches!(HirType::String, HirType::String));
        assert!(matches!(HirType::Bool, HirType::Bool));
        assert!(matches!(HirType::Any, HirType::Any));
    }

    #[test]
    fn test_hir_type_list() {
        let list_type = HirType::List(Box::new(HirType::Int));
        assert!(matches!(list_type, HirType::List(_)));
    }

    #[test]
    fn test_hir_type_optional() {
        let opt_type = HirType::Optional(Box::new(HirType::String));
        assert!(matches!(opt_type, HirType::Optional(_)));
    }

    #[test]
    fn test_hir_type_named() {
        let named_type = HirType::Named("MyClass".to_string());
        assert!(matches!(named_type, HirType::Named(ref s) if s == "MyClass"));
    }

    #[test]
    fn test_hir_type_nested() {
        let nested = HirType::List(Box::new(HirType::Optional(Box::new(HirType::Int))));
        if let HirType::List(inner) = nested {
            assert!(matches!(*inner, HirType::Optional(_)));
        }
    }

    #[test]
    fn test_hir_type_clone() {
        let t = HirType::List(Box::new(HirType::Int));
        let cloned = t.clone();
        assert_eq!(t, cloned);
    }

    #[test]
    fn test_hir_type_debug() {
        let debug_str = format!("{:?}", HirType::Int);
        assert_eq!(debug_str, "Int");
    }

    // ========================================================================
    // Serialization roundtrip tests
    // ========================================================================

    #[test]
    fn test_complex_hir_serialize_roundtrip() {
        let hir = Hir {
            root: HirExpr::Function {
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
                        right: Box::new(HirExpr::Call {
                            func: Box::new(HirExpr::Identifier("factorial".to_string())),
                            args: vec![HirExpr::Binary {
                                left: Box::new(HirExpr::Identifier("n".to_string())),
                                op: HirBinaryOp::Subtract,
                                right: Box::new(HirExpr::Literal(HirLiteral::Integer(1))),
                            }],
                        }),
                    }))))),
                }),
                is_async: false,
                return_type: Some(HirType::Int),
            },
            metadata: HirMetadata {
                source_file: Some("factorial.py".to_string()),
                module_name: Some("math".to_string()),
            },
        };

        let json = serde_json::to_string(&hir).unwrap();
        let deserialized: Hir = serde_json::from_str(&json).unwrap();
        assert_eq!(hir, deserialized);
    }
}
