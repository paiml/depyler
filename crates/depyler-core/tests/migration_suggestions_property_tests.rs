use depyler_core::hir::*;
use depyler_core::migration_suggestions::*;
use proptest::prelude::*;
use smallvec::smallvec;

// Generate arbitrary migration config
prop_compose! {
    fn arb_migration_config()(
        suggest_iterators in any::<bool>(),
        suggest_error_handling in any::<bool>(),
        suggest_ownership in any::<bool>(),
        suggest_performance in any::<bool>(),
        verbosity in 0..3u8
    ) -> MigrationConfig {
        MigrationConfig {
            suggest_iterators,
            suggest_error_handling,
            suggest_ownership,
            suggest_performance,
            verbosity,
        }
    }
}

// Generate arbitrary function names
prop_compose! {
    fn arb_func_name()(
        name in "[a-z][a-z0-9_]{0,20}"
    ) -> String {
        name
    }
}

// Generate arbitrary variable names
prop_compose! {
    fn arb_var_name()(
        name in "[a-z][a-z0-9_]{0,10}"
    ) -> String {
        name
    }
}

// Generate simple HIR expressions
fn arb_simple_expr() -> impl Strategy<Value = HirExpr> {
    prop_oneof![
        Just(HirExpr::Literal(Literal::None)),
        Just(HirExpr::Literal(Literal::Bool(true))),
        Just(HirExpr::Literal(Literal::Bool(false))),
        arb_var_name().prop_map(HirExpr::Var),
        (0..100i64).prop_map(|n| HirExpr::Literal(Literal::Int(n))),
    ]
}

// Generate simple HIR statements
fn arb_simple_stmt() -> impl Strategy<Value = HirStmt> {
    prop_oneof![
        Just(HirStmt::Break { label: None }),
        Just(HirStmt::Continue { label: None }),
        arb_simple_expr().prop_map(HirStmt::Expr),
        Just(HirStmt::Return(None)),
        Just(HirStmt::Return(Some(HirExpr::Literal(Literal::None)))),
    ]
}

// Generate simple functions
prop_compose! {
    fn arb_simple_function()(
        func_name in arb_func_name(),
        stmt_count in 0..5usize
    )(
        body in prop::collection::vec(arb_simple_stmt(), stmt_count),
        name in Just(func_name)
    ) -> HirFunction {
        HirFunction {
            name,
            params: smallvec![],
            ret_type: Type::Unknown,
            body,
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        }
    }
}

// Generate simple programs
prop_compose! {
    fn arb_simple_program()(
        func_count in 0..5usize
    )(
        functions in prop::collection::vec(arb_simple_function(), func_count)
    ) -> HirProgram {
        HirProgram {
            imports: vec![],
            functions,
            classes: vec![],
        }
    }
}

proptest! {
    #[test]
    fn test_analyzer_never_panics(
        config in arb_migration_config(),
        program in arb_simple_program()
    ) {
        let mut analyzer = MigrationAnalyzer::new(config);
        // Should never panic
        let _ = analyzer.analyze_program(&program);
    }

    #[test]
    fn test_analyze_program_deterministic(
        config in arb_migration_config(),
        program in arb_simple_program()
    ) {
        let mut analyzer1 = MigrationAnalyzer::new(config.clone());
        let mut analyzer2 = MigrationAnalyzer::new(config);

        let suggestions1 = analyzer1.analyze_program(&program);
        let suggestions2 = analyzer2.analyze_program(&program);

        // Same input should produce same number of suggestions
        prop_assert_eq!(suggestions1.len(), suggestions2.len());
    }

    #[test]
    fn test_format_suggestions_never_panics(
        config in arb_migration_config(),
        program in arb_simple_program()
    ) {
        let mut analyzer = MigrationAnalyzer::new(config);
        let suggestions = analyzer.analyze_program(&program);

        // Formatting should never panic
        let _ = analyzer.format_suggestions(&suggestions);
    }

    #[test]
    fn test_empty_program_no_suggestions(
        config in arb_migration_config()
    ) {
        let mut analyzer = MigrationAnalyzer::new(config);
        let program = HirProgram {
            imports: vec![],
            functions: vec![],
            classes: vec![],
        };

        let suggestions = analyzer.analyze_program(&program);
        prop_assert!(suggestions.is_empty());
    }

    #[test]
    fn test_suggestions_sorted_by_severity(
        config in arb_migration_config(),
        program in arb_simple_program()
    ) {
        let mut analyzer = MigrationAnalyzer::new(config);
        let suggestions = analyzer.analyze_program(&program);

        // Verify suggestions are sorted by severity
        for window in suggestions.windows(2) {
            prop_assert!(window[0].severity >= window[1].severity);
        }
    }

    #[test]
    fn test_while_true_always_detected(
        config in arb_migration_config(),
        func_name in arb_func_name()
    ) {
        let mut analyzer = MigrationAnalyzer::new(config);

        let func = HirFunction {
            name: func_name,
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![
                HirStmt::While {
                    condition: HirExpr::Literal(Literal::Bool(true)),
                    body: vec![HirStmt::Break { label: None }],
                }
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let program = HirProgram {
            imports: vec![],
            functions: vec![func],
            classes: vec![],
        };

        let suggestions = analyzer.analyze_program(&program);

        // Should always detect while True pattern
        prop_assert!(suggestions.iter().any(|s| s.title.contains("loop")));
    }

    #[test]
    fn test_isinstance_always_detected(
        config in arb_migration_config(),
        func_name in arb_func_name(),
        var_name in arb_var_name(),
        type_name in arb_var_name()
    ) {
        let mut analyzer = MigrationAnalyzer::new(config);

        let func = HirFunction {
            name: func_name,
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![
                HirStmt::If {
                    condition: HirExpr::Call {
                        func: "isinstance".to_string(),
                        args: vec![
                            HirExpr::Var(var_name),
                            HirExpr::Var(type_name),
                        ],
                    },
                    then_body: vec![HirStmt::Expr(HirExpr::Literal(Literal::None))],
                    else_body: None,
                }
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let program = HirProgram {
            imports: vec![],
            functions: vec![func],
            classes: vec![],
        };

        let suggestions = analyzer.analyze_program(&program);

        // Should always detect isinstance pattern
        prop_assert!(suggestions.iter().any(|s|
            s.category == SuggestionCategory::TypeSystem &&
            s.title.contains("type system")
        ));
    }

    #[test]
    fn test_enumerate_always_detected(
        config in arb_migration_config(),
        func_name in arb_func_name(),
        target in arb_var_name(),
        items in arb_var_name()
    ) {
        let mut analyzer = MigrationAnalyzer::new(config);

        let func = HirFunction {
            name: func_name,
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![
                HirStmt::For {
                    target: AssignTarget::Symbol(target),
                    iter: HirExpr::Call {
                        func: "enumerate".to_string(),
                        args: vec![HirExpr::Var(items)],
                    },
                    body: vec![HirStmt::Expr(HirExpr::Literal(Literal::None))],
                }
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let program = HirProgram {
            imports: vec![],
            functions: vec![func],
            classes: vec![],
        };

        let suggestions = analyzer.analyze_program(&program);

        // Should always detect enumerate pattern
        prop_assert!(suggestions.iter().any(|s|
            s.category == SuggestionCategory::Iterator &&
            s.title.contains("enumerate()")
        ));
    }

    #[test]
    fn test_format_output_contains_key_elements(
        config in arb_migration_config(),
        program in arb_simple_program()
    ) {
        let mut analyzer = MigrationAnalyzer::new(config);
        let suggestions = analyzer.analyze_program(&program);
        let output = analyzer.format_suggestions(&suggestions);

        if suggestions.is_empty() {
            prop_assert!(output.contains("No migration suggestions"));
        } else {
            prop_assert!(output.contains("Migration Suggestions"));
            prop_assert!(output.contains("Summary:"));
        }
    }
}

#[test]
fn test_severity_comparison() {
    assert!(Severity::Critical > Severity::Important);
    assert!(Severity::Important > Severity::Warning);
    assert!(Severity::Warning > Severity::Info);
    assert!(Severity::Critical > Severity::Info);
}

#[test]
fn test_config_default_values() {
    let config = MigrationConfig::default();
    assert!(config.suggest_iterators);
    assert!(config.suggest_error_handling);
    assert!(config.suggest_ownership);
    assert!(config.suggest_performance);
    assert_eq!(config.verbosity, 1);
}
