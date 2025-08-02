use depyler_annotations::TranspilationAnnotations;
use depyler_core::direct_rules::apply_rules;
use depyler_core::hir::{BinOp, HirExpr, HirFunction, HirModule, HirStmt, Literal, Type};
use depyler_core::rust_gen::generate_rust_file;
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
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
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
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
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
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
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
        // FloorDiv is not yet implemented
        // (BinOp::FloorDiv, "/"), // TODO: Should be different from Div
    ];

    for (op, expected) in test_cases {
        let module = HirModule {
            functions: vec![HirFunction {
                name: format!("test_{op:?}").to_lowercase(),
                params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: Default::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
        };

        let type_mapper = TypeMapper::default();
        let result = apply_rules(&module, &type_mapper).unwrap();
        let code = quote::quote! { #result }.to_string();

        assert!(
            code.contains(&format!("a {expected} b")),
            "Expected operator {expected} in code: {code}"
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
                name: format!("test_{op:?}").to_lowercase(),
                params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
                ret_type: Type::Bool,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: Default::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
        };

        let type_mapper = TypeMapper::default();
        let result = apply_rules(&module, &type_mapper).unwrap();
        let code = quote::quote! { #result }.to_string();

        assert!(
            code.contains(&format!("a {expected} b")),
            "Expected operator {expected} in code: {code}"
        );
    }
}

#[test]
fn test_logical_operators() {
    let test_cases = vec![(BinOp::And, "&&"), (BinOp::Or, "||")];

    for (op, expected) in test_cases {
        let module = HirModule {
            functions: vec![HirFunction {
                name: format!("test_{op:?}").to_lowercase(),
                params: vec![("a".to_string(), Type::Bool), ("b".to_string(), Type::Bool)].into(),
                ret_type: Type::Bool,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: Default::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
        };

        let type_mapper = TypeMapper::default();
        let result = apply_rules(&module, &type_mapper).unwrap();
        let code = quote::quote! { #result }.to_string();

        assert!(
            code.contains(&format!("a {expected} b")),
            "Expected operator {expected} in code: {code}"
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
                name: format!("test_{op:?}").to_lowercase(),
                params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: Default::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
        };

        let type_mapper = TypeMapper::default();
        let result = apply_rules(&module, &type_mapper).unwrap();
        let code = quote::quote! { #result }.to_string();

        assert!(
            code.contains(&format!("a {expected} b")),
            "Expected operator {expected} in code: {code}"
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
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
    };

    let type_mapper = TypeMapper::default();
    apply_rules(&module, &type_mapper).unwrap();
}

#[test]
fn test_array_length_subtraction_safety() {
    // Test that len(arr) - 1 uses saturating_sub
    let module = HirModule {
        functions: vec![HirFunction {
            name: "safe_last_index".to_string(),
            params: vec![("arr".to_string(), Type::List(Box::new(Type::Int)))].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Sub,
                left: Box::new(HirExpr::Call {
                    func: "len".to_string(),
                    args: vec![HirExpr::Var("arr".to_string())],
                }),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            }))],
            properties: Default::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: Some("Get the last index of an array safely".to_string()),
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
    };

    let type_mapper = TypeMapper::default();
    let result = generate_rust_file(&module, &type_mapper).unwrap();

    // Verify that saturating_sub is used
    assert!(
        result.contains("saturating_sub"),
        "Expected saturating_sub for array length subtraction, got: {}",
        result
    );
}

#[test]
fn test_regular_subtraction_unchanged() {
    // Test that regular x - y doesn't use saturating_sub
    let module = HirModule {
        functions: vec![HirFunction {
            name: "regular_sub".to_string(),
            params: vec![("x".to_string(), Type::Int), ("y".to_string(), Type::Int)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Sub,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Var("y".to_string())),
            }))],
            properties: Default::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
    };

    let type_mapper = TypeMapper::default();
    let result = generate_rust_file(&module, &type_mapper).unwrap();

    // Verify regular subtraction doesn't use saturating_sub
    assert!(
        !result.contains("saturating_sub"),
        "Regular subtraction should not use saturating_sub, got: {}",
        result
    );
}

#[test]
fn test_len_variable_subtraction_safety() {
    // Test that len(items) - offset uses saturating_sub
    let module = HirModule {
        functions: vec![HirFunction {
            name: "complex_len_sub".to_string(),
            params: vec![
                ("items".to_string(), Type::List(Box::new(Type::String))),
                ("offset".to_string(), Type::Int),
            ]
            .into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Sub,
                left: Box::new(HirExpr::Call {
                    func: "len".to_string(),
                    args: vec![HirExpr::Var("items".to_string())],
                }),
                right: Box::new(HirExpr::Var("offset".to_string())),
            }))],
            properties: Default::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: Some("Subtract offset from list length".to_string()),
        }],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        classes: vec![],
    };

    let type_mapper = TypeMapper::default();
    let result = generate_rust_file(&module, &type_mapper).unwrap();

    // Should use saturating_sub for len() - variable
    assert!(
        result.contains("saturating_sub"),
        "Expected saturating_sub for len() - variable, got: {}",
        result
    );
}
