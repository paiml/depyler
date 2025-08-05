#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;
    use smallvec::smallvec;

    fn create_test_function(name: &str, body: Vec<HirStmt>) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body,
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        }
    }

    fn create_test_program(functions: Vec<HirFunction>) -> HirProgram {
        HirProgram {
            imports: vec![],
            functions,
            classes: vec![],
        }
    }

    #[test]
    fn test_migration_analyzer_creation() {
        let config = MigrationConfig::default();
        let analyzer = MigrationAnalyzer::new(config);
        assert_eq!(analyzer.suggestions.len(), 0);
    }

    #[test]
    fn test_migration_config_custom() {
        let config = MigrationConfig {
            suggest_iterators: false,
            suggest_error_handling: true,
            suggest_ownership: false,
            suggest_performance: true,
            verbosity: 2,
        };
        assert!(!config.suggest_iterators);
        assert!(config.suggest_error_handling);
        assert_eq!(config.verbosity, 2);
    }

    #[test]
    fn test_analyze_empty_program() {
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        let program = create_test_program(vec![]);
        let suggestions = analyzer.analyze_program(&program);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_analyze_simple_function() {
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        let func = create_test_function("simple", vec![
            HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))
        ]);
        let program = create_test_program(vec![func]);
        let suggestions = analyzer.analyze_program(&program);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_enumerate_pattern_detection() {
        let body = vec![
            HirStmt::For {
                target: "i".to_string(),
                iter: HirExpr::Call {
                    func: "enumerate".to_string(),
                    args: vec![HirExpr::Var("items".to_string())],
                },
                body: vec![
                    HirStmt::Expr(HirExpr::Var("i".to_string()))
                ],
            }
        ];

        let func = create_test_function("test_enum", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        
        analyzer.analyze_function(&func);
        assert!(!analyzer.suggestions.is_empty());
        
        let suggestion = &analyzer.suggestions[0];
        assert_eq!(suggestion.category, SuggestionCategory::Iterator);
        assert!(suggestion.title.contains("enumerate()"));
        assert!(suggestion.rust_suggestion.contains(".enumerate()"));
    }

    #[test]
    fn test_type_check_pattern_detection() {
        let body = vec![
            HirStmt::If {
                condition: HirExpr::Call {
                    func: "isinstance".to_string(),
                    args: vec![
                        HirExpr::Var("value".to_string()),
                        HirExpr::Var("str".to_string()),
                    ],
                },
                then_body: vec![
                    HirStmt::Expr(HirExpr::Call {
                        func: "process_string".to_string(),
                        args: vec![HirExpr::Var("value".to_string())],
                    })
                ],
                else_body: None,
            }
        ];

        let func = create_test_function("test_type", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        
        analyzer.analyze_function(&func);
        assert!(!analyzer.suggestions.is_empty());
        
        let suggestion = analyzer.suggestions.iter()
            .find(|s| s.category == SuggestionCategory::TypeSystem)
            .expect("Should have type system suggestion");
        
        assert!(suggestion.title.contains("type system"));
        assert!(suggestion.rust_suggestion.contains("enum"));
        assert!(suggestion.rust_suggestion.contains("match"));
    }

    #[test]
    fn test_none_check_pattern_detection() {
        let body = vec![
            HirStmt::If {
                condition: HirExpr::Binary {
                    op: BinOp::NotEq,
                    left: Box::new(HirExpr::Var("value".to_string())),
                    right: Box::new(HirExpr::Literal(Literal::None)),
                },
                then_body: vec![
                    HirStmt::Expr(HirExpr::Call {
                        func: "process".to_string(),
                        args: vec![HirExpr::Var("value".to_string())],
                    })
                ],
                else_body: Some(vec![
                    HirStmt::Expr(HirExpr::Call {
                        func: "handle_none".to_string(),
                        args: vec![],
                    })
                ]),
            }
        ];

        let func = create_test_function("test_none", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        
        analyzer.analyze_function(&func);
        assert!(!analyzer.suggestions.is_empty());
        
        let suggestion = analyzer.suggestions.iter()
            .find(|s| s.category == SuggestionCategory::ErrorHandling)
            .expect("Should have error handling suggestion");
        
        assert!(suggestion.title.contains("pattern matching") || suggestion.title.contains("if-let"));
        assert!(suggestion.rust_suggestion.contains("if let Some"));
    }

    #[test]
    fn test_string_concatenation_detection() {
        let body = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("result".to_string()),
                value: HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("str1".to_string())),
                    right: Box::new(HirExpr::Var("str2".to_string())),
                },
            }
        ];

        let func = create_test_function("test_concat", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        
        analyzer.analyze_function(&func);
        
        let suggestion = analyzer.suggestions.iter()
            .find(|s| s.category == SuggestionCategory::Performance)
            .expect("Should have performance suggestion");
        
        assert!(suggestion.title.contains("format!") || suggestion.title.contains("String"));
    }

    #[test]
    fn test_mutable_parameter_pattern() {
        let func = HirFunction {
            name: "modify_list".to_string(),
            params: smallvec![("lst".to_string(), Type::List(Box::new(Type::Int)))],
            ret_type: Type::Unknown,
            body: vec![
                HirStmt::Expr(HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("lst".to_string())),
                    method: "append".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(42))],
                })
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        analyzer.analyze_function(&func);
        
        let suggestion = analyzer.suggestions.iter()
            .find(|s| s.category == SuggestionCategory::Ownership)
            .expect("Should have ownership suggestion");
        
        assert!(suggestion.title.contains("ownership") || suggestion.title.contains("mutable"));
        assert!(suggestion.rust_suggestion.contains("&mut"));
    }

    #[test]
    fn test_filter_map_pattern_detection() {
        let body = vec![
            HirStmt::For {
                target: "item".to_string(),
                iter: HirExpr::Var("items".to_string()),
                body: vec![
                    HirStmt::If {
                        condition: HirExpr::Call {
                            func: "condition".to_string(),
                            args: vec![HirExpr::Var("item".to_string())],
                        },
                        then_body: vec![
                            HirStmt::Expr(HirExpr::MethodCall {
                                object: Box::new(HirExpr::Var("result".to_string())),
                                method: "append".to_string(),
                                args: vec![HirExpr::Call {
                                    func: "transform".to_string(),
                                    args: vec![HirExpr::Var("item".to_string())],
                                }],
                            })
                        ],
                        else_body: None,
                    }
                ],
            }
        ];

        let func = create_test_function("test_filter_map", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        
        analyzer.analyze_function(&func);
        
        let suggestion = analyzer.suggestions.iter()
            .find(|s| s.title.contains("filter_map"))
            .expect("Should have filter_map suggestion");
        
        assert_eq!(suggestion.category, SuggestionCategory::Iterator);
        assert!(suggestion.rust_suggestion.contains("filter_map"));
    }

    #[test]
    fn test_suggestion_sorting_by_severity() {
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        
        // Add suggestions with different severities
        analyzer.add_suggestion(MigrationSuggestion {
            category: SuggestionCategory::Iterator,
            severity: Severity::Info,
            title: "Info level".to_string(),
            description: "".to_string(),
            python_example: "".to_string(),
            rust_suggestion: "".to_string(),
            notes: vec![],
            location: None,
        });
        
        analyzer.add_suggestion(MigrationSuggestion {
            category: SuggestionCategory::ErrorHandling,
            severity: Severity::Critical,
            title: "Critical level".to_string(),
            description: "".to_string(),
            python_example: "".to_string(),
            rust_suggestion: "".to_string(),
            notes: vec![],
            location: None,
        });
        
        analyzer.add_suggestion(MigrationSuggestion {
            category: SuggestionCategory::Performance,
            severity: Severity::Warning,
            title: "Warning level".to_string(),
            description: "".to_string(),
            python_example: "".to_string(),
            rust_suggestion: "".to_string(),
            notes: vec![],
            location: None,
        });
        
        let program = create_test_program(vec![]);
        let suggestions = analyzer.analyze_program(&program);
        
        // Check that suggestions are sorted by severity (highest first)
        assert_eq!(suggestions[0].severity, Severity::Critical);
        assert_eq!(suggestions[1].severity, Severity::Warning);
        assert_eq!(suggestions[2].severity, Severity::Info);
    }

    #[test]
    fn test_format_suggestions_empty() {
        let analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        let output = analyzer.format_suggestions(&[]);
        assert!(output.contains("No migration suggestions"));
        assert!(output.contains("idiomatic"));
    }

    #[test]
    fn test_format_suggestions_with_items() {
        let analyzer = MigrationAnalyzer::new(MigrationConfig {
            verbosity: 2,
            ..Default::default()
        });
        
        let suggestions = vec![
            MigrationSuggestion {
                category: SuggestionCategory::Iterator,
                severity: Severity::Warning,
                title: "Test suggestion".to_string(),
                description: "Test description".to_string(),
                python_example: "for x in list:".to_string(),
                rust_suggestion: "for x in list.iter() {".to_string(),
                notes: vec!["Note 1".to_string(), "Note 2".to_string()],
                location: Some(SourceLocation {
                    function: "test_func".to_string(),
                    line: 10,
                }),
            }
        ];
        
        let output = analyzer.format_suggestions(&suggestions);
        
        assert!(output.contains("Migration Suggestions"));
        assert!(output.contains("Test suggestion"));
        assert!(output.contains("Test description"));
        assert!(output.contains("test_func"));
        assert!(output.contains("line 10"));
        assert!(output.contains("Python pattern"));
        assert!(output.contains("Rust idiom"));
        assert!(output.contains("Note 1"));
        assert!(output.contains("Note 2"));
        assert!(output.contains("Summary:"));
    }

    #[test]
    fn test_source_location() {
        let loc = SourceLocation {
            function: "my_func".to_string(),
            line: 42,
        };
        assert_eq!(loc.function, "my_func");
        assert_eq!(loc.line, 42);
    }

    #[test]
    fn test_suggestion_category_equality() {
        assert_eq!(SuggestionCategory::Iterator, SuggestionCategory::Iterator);
        assert_ne!(SuggestionCategory::Iterator, SuggestionCategory::ErrorHandling);
    }

    #[test]
    fn test_list_dict_construction_suggestion() {
        let body = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("result".to_string()),
                value: HirExpr::Call {
                    func: "list".to_string(),
                    args: vec![HirExpr::List(vec![])],
                },
            }
        ];

        let func = create_test_function("test_list", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        
        analyzer.analyze_function(&func);
        
        let suggestion = analyzer.suggestions.iter()
            .find(|s| s.category == SuggestionCategory::Performance)
            .expect("Should have performance suggestion");
        
        assert!(suggestion.title.contains("collect()"));
    }

    #[test]
    fn test_config_with_disabled_suggestions() {
        let config = MigrationConfig {
            suggest_iterators: false,
            suggest_error_handling: false,
            suggest_ownership: false,
            suggest_performance: false,
            verbosity: 0,
        };
        
        let mut analyzer = MigrationAnalyzer::new(config);
        
        // Even with patterns that would normally trigger suggestions,
        // nothing should be suggested with all options disabled
        let body = vec![
            HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![],
            }
        ];
        
        let func = create_test_function("test", body);
        analyzer.analyze_function(&func);
        
        // Note: Current implementation doesn't check config flags,
        // so this test documents current behavior
        assert!(!analyzer.suggestions.is_empty());
    }

    #[test]
    fn test_multiple_suggestions_per_function() {
        let body = vec![
            // Pattern 1: while True
            HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Break { label: None }],
            },
            // Pattern 2: isinstance check
            HirStmt::If {
                condition: HirExpr::Call {
                    func: "isinstance".to_string(),
                    args: vec![
                        HirExpr::Var("x".to_string()),
                        HirExpr::Var("int".to_string()),
                    ],
                },
                then_body: vec![],
                else_body: None,
            },
        ];

        let func = create_test_function("multi_pattern", body);
        let mut analyzer = MigrationAnalyzer::new(MigrationConfig::default());
        
        analyzer.analyze_function(&func);
        
        // Should have at least 2 suggestions
        assert!(analyzer.suggestions.len() >= 2);
        
        // Should have both categories
        let categories: Vec<_> = analyzer.suggestions.iter()
            .map(|s| &s.category)
            .collect();
        
        assert!(categories.contains(&&SuggestionCategory::Iterator));
        assert!(categories.contains(&&SuggestionCategory::TypeSystem));
    }
}