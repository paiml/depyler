use depyler_core::direct_rules::apply_rules;
use depyler_core::hir::{BinOp, HirExpr, HirFunction, HirModule, HirStmt, Literal, Type};
use depyler_core::type_mapper::TypeMapper;

#[test]
fn test_augmented_assignment() {
    let module = HirModule {
        functions: vec![HirFunction {
            name: "test_aug_assign".to_string(),
            params: vec![("x".to_string(), Type::Int)].into(),
            ret_type: Type::Int,
            body: vec![
                HirStmt::Assign {
                    target: "total".to_string(),
                    value: HirExpr::Literal(Literal::Int(0)),
                },
                // total += x (converted to total = total + x)
                HirStmt::Assign {
                    target: "total".to_string(),
                    value: HirExpr::Binary {
                        op: BinOp::Add,
                        left: Box::new(HirExpr::Var("total".to_string())),
                        right: Box::new(HirExpr::Var("x".to_string())),
                    },
                },
                HirStmt::Return(Some(HirExpr::Var("total".to_string()))),
            ],
            properties: Default::default(),
        }],
        imports: vec![],
    };

    let type_mapper = TypeMapper::default();
    let result = apply_rules(&module, &type_mapper).unwrap();
    let code = quote::quote! { #result }.to_string();

    assert!(code.contains("let mut total = 0"));
    assert!(code.contains("total = total + x"));
}

#[test]
fn test_in_operator() {
    let module = HirModule {
        functions: vec![HirFunction {
            name: "test_in".to_string(),
            params: vec![].into(),
            ret_type: Type::Bool,
            body: vec![
                HirStmt::Assign {
                    target: "dict".to_string(),
                    value: HirExpr::Dict(vec![(
                        HirExpr::Literal(Literal::String("key".to_string())),
                        HirExpr::Literal(Literal::String("value".to_string())),
                    )]),
                },
                HirStmt::Return(Some(HirExpr::Binary {
                    op: BinOp::In,
                    left: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
                    right: Box::new(HirExpr::Var("dict".to_string())),
                })),
            ],
            properties: Default::default(),
        }],
        imports: vec![],
    };

    let type_mapper = TypeMapper::default();
    let result = apply_rules(&module, &type_mapper).unwrap();
    let code = quote::quote! { #result }.to_string();

    assert!(code.contains("contains_key"));
    assert!(code.contains("HashMap"));
}

#[test]
fn test_not_in_operator() {
    let module = HirModule {
        functions: vec![HirFunction {
            name: "test_not_in".to_string(),
            params: vec![].into(),
            ret_type: Type::Bool,
            body: vec![
                HirStmt::Assign {
                    target: "dict".to_string(),
                    value: HirExpr::Dict(vec![]),
                },
                HirStmt::Return(Some(HirExpr::Binary {
                    op: BinOp::NotIn,
                    left: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
                    right: Box::new(HirExpr::Var("dict".to_string())),
                })),
            ],
            properties: Default::default(),
        }],
        imports: vec![],
    };

    let type_mapper = TypeMapper::default();
    let result = apply_rules(&module, &type_mapper).unwrap();
    let code = quote::quote! { #result }.to_string();

    assert!(code.contains("! dict . contains_key"));
}

#[test]
fn test_all_arithmetic_operators() {
    let test_cases = vec![
        (BinOp::Add, "+"),
        (BinOp::Sub, "-"),
        (BinOp::Mul, "*"),
        (BinOp::Div, "/"),
        (BinOp::Mod, "%"),
        (BinOp::FloorDiv, "/"), // TODO: Should be different from Div
    ];

    for (op, expected) in test_cases {
        let module = HirModule {
            functions: vec![HirFunction {
                name: format!("test_{:?}", op).to_lowercase(),
                params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: Default::default(),
            }],
            imports: vec![],
        };

        let type_mapper = TypeMapper::default();
        let result = apply_rules(&module, &type_mapper).unwrap();
        let code = quote::quote! { #result }.to_string();

        assert!(
            code.contains(&format!("a {} b", expected)),
            "Expected operator {} in code: {}",
            expected,
            code
        );
    }
}

#[test]
fn test_comparison_operators() {
    let test_cases = vec![
        (BinOp::Eq, "=="),
        (BinOp::NotEq, "!="),
        (BinOp::Lt, "<"),
        (BinOp::LtEq, "<="),
        (BinOp::Gt, ">"),
        (BinOp::GtEq, ">="),
    ];

    for (op, expected) in test_cases {
        let module = HirModule {
            functions: vec![HirFunction {
                name: format!("test_{:?}", op).to_lowercase(),
                params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
                ret_type: Type::Bool,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: Default::default(),
            }],
            imports: vec![],
        };

        let type_mapper = TypeMapper::default();
        let result = apply_rules(&module, &type_mapper).unwrap();
        let code = quote::quote! { #result }.to_string();

        assert!(
            code.contains(&format!("a {} b", expected)),
            "Expected operator {} in code: {}",
            expected,
            code
        );
    }
}

#[test]
fn test_logical_operators() {
    let test_cases = vec![(BinOp::And, "&&"), (BinOp::Or, "||")];

    for (op, expected) in test_cases {
        let module = HirModule {
            functions: vec![HirFunction {
                name: format!("test_{:?}", op).to_lowercase(),
                params: vec![("a".to_string(), Type::Bool), ("b".to_string(), Type::Bool)].into(),
                ret_type: Type::Bool,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: Default::default(),
            }],
            imports: vec![],
        };

        let type_mapper = TypeMapper::default();
        let result = apply_rules(&module, &type_mapper).unwrap();
        let code = quote::quote! { #result }.to_string();

        assert!(
            code.contains(&format!("a {} b", expected)),
            "Expected operator {} in code: {}",
            expected,
            code
        );
    }
}

#[test]
fn test_bitwise_operators() {
    let test_cases = vec![
        (BinOp::BitAnd, "&"),
        (BinOp::BitOr, "|"),
        (BinOp::BitXor, "^"),
        (BinOp::LShift, "<<"),
        (BinOp::RShift, ">>"),
    ];

    for (op, expected) in test_cases {
        let module = HirModule {
            functions: vec![HirFunction {
                name: format!("test_{:?}", op).to_lowercase(),
                params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: Default::default(),
            }],
            imports: vec![],
        };

        let type_mapper = TypeMapper::default();
        let result = apply_rules(&module, &type_mapper).unwrap();
        let code = quote::quote! { #result }.to_string();

        assert!(
            code.contains(&format!("a {} b", expected)),
            "Expected operator {} in code: {}",
            expected,
            code
        );
    }
}

#[test]
#[should_panic(expected = "Power operator not directly supported")]
fn test_power_operator_not_supported() {
    let module = HirModule {
        functions: vec![HirFunction {
            name: "test_pow".to_string(),
            params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Pow,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: Default::default(),
        }],
        imports: vec![],
    };

    let type_mapper = TypeMapper::default();
    apply_rules(&module, &type_mapper).unwrap();
}
