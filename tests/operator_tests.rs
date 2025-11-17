use depyler_annotations::TranspilationAnnotations;
use depyler_core::direct_rules::apply_rules;
use depyler_core::hir::{
    AssignTarget, BinOp, HirExpr, HirFunction, HirModule, HirParam, HirStmt, Literal, Type,
};
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use quote::ToTokens;
use smallvec::smallvec;

#[test]
fn test_augmented_assignment() {
    let module = HirModule {
        functions: vec![HirFunction {
            name: "test_aug_assign".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("total".to_string()),
                    value: HirExpr::Literal(Literal::Int(0)),
                    type_annotation: None,
                },
                // total += x (converted to total = total + x)
                HirStmt::Assign {
                    target: AssignTarget::Symbol("total".to_string()),
                    value: HirExpr::Binary {
                        op: BinOp::Add,
                        left: Box::new(HirExpr::Var("total".to_string())),
                        right: Box::new(HirExpr::Var("x".to_string())),
                    },
                    type_annotation: None,
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
            params: smallvec![],
            ret_type: Type::Bool,
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("dict".to_string()),
                    value: HirExpr::Dict(vec![(
                        HirExpr::Literal(Literal::String("key".to_string())),
                        HirExpr::Literal(Literal::String("value".to_string())),
                    )]),
                    type_annotation: None,
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
            params: smallvec![],
            ret_type: Type::Bool,
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("dict".to_string()),
                    value: HirExpr::Dict(vec![]),
                    type_annotation: None,
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
#[allow(clippy::cognitive_complexity)]
fn test_all_arithmetic_operators() {
    let test_cases = vec![
        (BinOp::Add, "+"),
        (BinOp::Sub, "-"),
        (BinOp::Mul, "*"),
        (BinOp::Div, "/"),
        (BinOp::Mod, "%"),
        (BinOp::FloorDiv, "//"),
        (BinOp::Pow, "**"),
    ];

    for (op, expected) in test_cases {
        let module = HirModule {
            functions: vec![HirFunction {
                name: format!("test_{op:?}").to_lowercase(),
                params: smallvec![
                    HirParam::new("a".to_string(), Type::Int),
                    HirParam::new("b".to_string(), Type::Int)
                ],
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

        // Special handling for complex operators
        match op {
            BinOp::FloorDiv => {
                // Floor division generates complex Python-compatible logic
                assert!(
                    code.contains("let q = a / b"),
                    "Expected floor division logic in code: {code}"
                );
                assert!(
                    code.contains("let r = a % b"),
                    "Expected modulo in floor division logic: {code}"
                );
                // DEPYLER-0236: Floor division now uses intermediate boolean variables
                // for better readability and to avoid rustfmt edge cases
                assert!(
                    code.contains("let r_negative = r < 0")
                        && code.contains("let b_negative = b < 0")
                        && code.contains("let r_nonzero = r != 0")
                        && code.contains("let signs_differ = r_negative != b_negative")
                        && code.contains("let needs_adjustment = r_nonzero && signs_differ"),
                    "Expected floor division condition with intermediate booleans: {code}"
                );
            }
            BinOp::Pow => {
                // Power operation generates type-specific handling
                assert!(
                    code.contains("checked_pow") || code.contains("powf"),
                    "Expected power operation in code: {code}"
                );
            }
            _ => {
                // Simple operators should have the expected pattern
                assert!(
                    code.contains(&format!("a {expected} b")),
                    "Expected operator {expected} in code: {code}"
                );
            }
        }
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
                params: smallvec![
                    HirParam::new("a".to_string(), Type::Int),
                    HirParam::new("b".to_string(), Type::Int)
                ],
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
                params: smallvec![
                    HirParam::new("a".to_string(), Type::Bool),
                    HirParam::new("b".to_string(), Type::Bool)
                ],
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
                params: smallvec![
                    HirParam::new("a".to_string(), Type::Int),
                    HirParam::new("b".to_string(), Type::Int)
                ],
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
fn test_power_operator() {
    let module = HirModule {
        functions: vec![HirFunction {
            name: "test_pow".to_string(),
            params: smallvec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int)
            ],
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

    // Power operator should now be supported
    let type_mapper = TypeMapper::default();
    let rust_module = apply_rules(&module, &type_mapper).unwrap();
    let rust_code = rust_module.to_token_stream().to_string();

    // Should generate runtime check for negative exponents
    assert!(rust_code.contains("if b >= 0"));
    assert!(rust_code.contains("checked_pow"));
    assert!(rust_code.contains("powf"));
}

#[test]
fn test_floor_division_operator() {
    let module = HirModule {
        functions: vec![HirFunction {
            name: "test_floor_div".to_string(),
            params: smallvec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int)
            ],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::FloorDiv,
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

    // Floor division should now be supported
    let type_mapper = TypeMapper::default();
    let rust_module = apply_rules(&module, &type_mapper).unwrap();
    let rust_code = rust_module.to_token_stream().to_string();

    // Should generate Python floor division semantics
    assert!(rust_code.contains("let a ="));
    assert!(rust_code.contains("let b ="));
    assert!(rust_code.contains("let q = a / b"));
    assert!(rust_code.contains("let r = a % b"));
    // DEPYLER-0236: Floor division now uses intermediate boolean variables
    assert!(rust_code.contains("let r_negative = r < 0"));
    assert!(rust_code.contains("let b_negative = b < 0"));
    assert!(rust_code.contains("let r_nonzero = r != 0"));
    assert!(rust_code.contains("let signs_differ = r_negative != b_negative"));
    assert!(rust_code.contains("let needs_adjustment = r_nonzero && signs_differ"));
    assert!(rust_code.contains("if needs_adjustment"));
    assert!(rust_code.contains("{ q - 1 } else { q }"));
}

#[test]
fn test_array_length_subtraction_safety() {
    // Test that len(arr) - 1 uses saturating_sub
    let module = HirModule {
        functions: vec![HirFunction {
            name: "safe_last_index".to_string(),
            params: smallvec![HirParam::new(
                "arr".to_string(),
                Type::List(Box::new(Type::Int))
            )],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Sub,
                left: Box::new(HirExpr::Call { func: "len".to_string(), args: vec![HirExpr::Var("arr".to_string())], kwargs: vec![] }),
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
    let (result, _dependencies) = generate_rust_file(&module, &type_mapper).unwrap();

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
            params: smallvec![
                HirParam::new("x".to_string(), Type::Int),
                HirParam::new("y".to_string(), Type::Int)
            ],
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
    let (result, _dependencies) = generate_rust_file(&module, &type_mapper).unwrap();

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
            params: smallvec![
                HirParam::new("items".to_string(), Type::List(Box::new(Type::String))),
                HirParam::new("offset".to_string(), Type::Int),
            ],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Sub,
                left: Box::new(HirExpr::Call {
                    func: "len".to_string(),
                    args: vec![HirExpr::Var("items".to_string())],
                    kwargs: vec![],
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
    let (result, _dependencies) = generate_rust_file(&module, &type_mapper).unwrap();

    // Should use saturating_sub for len() - variable
    assert!(
        result.contains("saturating_sub"),
        "Expected saturating_sub for len() - variable, got: {}",
        result
    );
}
