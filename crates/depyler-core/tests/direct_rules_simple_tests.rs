#[cfg(test)]
mod tests {
    use depyler_core::direct_rules::apply_rules;
    use depyler_core::hir::*;
    use depyler_core::type_mapper::TypeMapper;
    use smallvec::smallvec;

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

    #[test]
    fn test_empty_module() {
        let module = create_empty_module();
        let type_mapper = TypeMapper::new();

        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_simple_function() {
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
    fn test_function_with_params() {
        let mut module = create_empty_module();

        module.functions.push(HirFunction {
            name: "add".to_string(),
            params: smallvec![
                HirParam {
                    name: Symbol::from("a"),
                    ty: Type::Int,
                    default: None,
                    is_vararg: false,
                },
                HirParam {
                    name: Symbol::from("b"),
                    ty: Type::Int,
                    default: None,
                    is_vararg: false,
                }
            ],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: Some("Add two numbers".to_string()),
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_simple_class() {
        let mut module = create_empty_module();

        module.classes.push(HirClass {
            name: "Test".to_string(),
            base_classes: vec![],
            methods: vec![],
            fields: vec![],
            is_dataclass: false,
            docstring: None,
            type_params: vec![],
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_class_with_field() {
        let mut module = create_empty_module();

        module.classes.push(HirClass {
            name: "Point".to_string(),
            base_classes: vec![],
            methods: vec![],
            fields: vec![HirField {
                name: "x".to_string(),
                field_type: Type::Float,
                default_value: None,
                is_class_var: false,
            }],
            is_dataclass: true,
            docstring: None,
            type_params: vec![],
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_alias() {
        let mut module = create_empty_module();

        module.type_aliases.push(TypeAlias {
            name: "MyInt".to_string(),
            target_type: Type::Int,
            is_newtype: false,
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_import() {
        let mut module = create_empty_module();

        module.imports.push(Import {
            module: "math".to_string(),
            alias: None,
            items: vec![ImportItem::Named("sqrt".to_string())],
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_protocol() {
        let mut module = create_empty_module();

        module.protocols.push(Protocol {
            name: "Comparable".to_string(),
            type_params: vec![],
            methods: vec![],
            is_runtime_checkable: false,
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_control_flow() {
        let mut module = create_empty_module();

        module.functions.push(HirFunction {
            name: "control_flow".to_string(),
            params: smallvec![HirParam {
                name: Symbol::from("x"),
                ty: Type::Int,
                default: None,
                is_vararg: false,
            }],
            ret_type: Type::Int,
            body: vec![HirStmt::If {
                condition: HirExpr::Binary {
                    op: BinOp::Gt,
                    left: Box::new(HirExpr::Var("x".to_string())),
                    right: Box::new(HirExpr::Literal(Literal::Int(0))),
                },
                then_body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
                else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                    0,
                ))))]),
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_loop_structures() {
        let mut module = create_empty_module();

        module.functions.push(HirFunction {
            name: "loops".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![
                // For loop
                HirStmt::For {
                    target: AssignTarget::Symbol("i".to_string()),
                    iter: HirExpr::Call {
                        func: "range".to_string(),
                        args: vec![HirExpr::Literal(Literal::Int(10))],
                        kwargs: vec![],
                    },
                    body: vec![HirStmt::Expr(HirExpr::Var("i".to_string()))],
                },
                // While loop
                HirStmt::While {
                    condition: HirExpr::Literal(Literal::Bool(true)),
                    body: vec![HirStmt::Break { label: None }],
                },
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_collections() {
        let mut module = create_empty_module();

        module.functions.push(HirFunction {
            name: "collections".to_string(),
            params: smallvec![],
            ret_type: Type::List(Box::new(Type::Int)),
            body: vec![HirStmt::Return(Some(HirExpr::List(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
                HirExpr::Literal(Literal::Int(3)),
            ])))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_method() {
        let mut module = create_empty_module();

        module.classes.push(HirClass {
            name: "Counter".to_string(),
            base_classes: vec![],
            methods: vec![HirMethod {
                name: "increment".to_string(),
                params: smallvec![HirParam {
                    name: Symbol::from("self"),
                    ty: Type::Unknown,
                    default: None,
                    is_vararg: false,
                }],
                ret_type: Type::None,
                body: vec![],
                is_static: false,
                is_classmethod: false,
                is_property: false,
                is_async: false,
                docstring: None,
            }],
            fields: vec![],
            is_dataclass: false,
            docstring: None,
            type_params: vec![],
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_complex_expression() {
        let mut module = create_empty_module();

        module.functions.push(HirFunction {
            name: "complex".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Binary {
                    op: BinOp::Mul,
                    left: Box::new(HirExpr::Literal(Literal::Int(2))),
                    right: Box::new(HirExpr::Literal(Literal::Int(3))),
                }),
                right: Box::new(HirExpr::Literal(Literal::Int(4))),
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lambda() {
        let mut module = create_empty_module();

        module.functions.push(HirFunction {
            name: "use_lambda".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("f".to_string()),
                    value: HirExpr::Lambda {
                        params: vec!["x".to_string()],
                        body: Box::new(HirExpr::Binary {
                            op: BinOp::Mul,
                            left: Box::new(HirExpr::Var("x".to_string())),
                            right: Box::new(HirExpr::Literal(Literal::Int(2))),
                        }),
                    },
                    type_annotation: None,
                },
                HirStmt::Return(Some(HirExpr::Call {
                    func: "f".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(5))],
                    kwargs: vec![],
                })),
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_components() {
        let mut module = create_empty_module();

        // Add various components
        module.imports.push(Import {
            module: "typing".to_string(),
            alias: None,
            items: vec![
                ImportItem::Named("List".to_string()),
                ImportItem::Aliased {
                    name: "Dict".to_string(),
                    alias: "Dictionary".to_string(),
                },
            ],
        });

        module.type_aliases.push(TypeAlias {
            name: "IntList".to_string(),
            target_type: Type::List(Box::new(Type::Int)),
            is_newtype: false,
        });

        module.functions.push(HirFunction {
            name: "process".to_string(),
            params: smallvec![HirParam {
                name: Symbol::from("data"),
                ty: Type::List(Box::new(Type::Int)),
                default: None,
                is_vararg: false,
            }],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0))))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        });

        let type_mapper = TypeMapper::new();
        let result = apply_rules(&module, &type_mapper);
        assert!(result.is_ok());
    }
}

/// Property tests for direct_rules
#[cfg(test)]
mod property_tests {
    use depyler_core::direct_rules::apply_rules;
    use depyler_core::hir::*;
    use depyler_core::type_mapper::TypeMapper;
    use proptest::prelude::*;
    use smallvec::smallvec;

    prop_compose! {
        fn arb_simple_module()(
            num_functions in 0..5usize
        ) -> HirModule {
            let mut module = HirModule {
                imports: vec![],
                functions: vec![],
                classes: vec![],
                type_aliases: vec![],
                protocols: vec![],
                constants: vec![],
            };

            for i in 0..num_functions {
                module.functions.push(HirFunction {
                    name: format!("func{}", i),
                    params: smallvec![],
                    ret_type: Type::Unknown,
                    body: vec![],
                    properties: FunctionProperties::default(),
                    annotations: Default::default(),
                    docstring: None,
                });
            }

            module
        }
    }

    proptest! {
        #[test]
        fn test_apply_rules_doesnt_panic(module in arb_simple_module()) {
            let type_mapper = TypeMapper::new();
            let _ = apply_rules(&module, &type_mapper);
        }
    }
}
