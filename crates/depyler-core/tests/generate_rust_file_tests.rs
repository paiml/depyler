// DEPYLER-0004: Tests for generate_rust_file function
//
// EXTREME TDD: These tests are written BEFORE refactoring to ensure
// behavior preservation during Extract Method pattern application.

use depyler_annotations::TranspilationAnnotations;
use depyler_core::hir::*;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use smallvec::SmallVec;

/// Helper function to create a minimal valid HirFunction
fn create_simple_function(name: &str) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: SmallVec::new(),
        ret_type: Type::None,
        body: vec![],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    }
}

/// Helper function to create an empty HirModule
fn create_empty_module() -> HirModule {
    HirModule {
        functions: vec![],
        classes: vec![],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
    }
}

#[cfg(test)]
mod baseline_tests {
    use super::*;

    #[test]
    fn test_empty_module_never_panics() {
        let module = create_empty_module();
        let type_mapper = TypeMapper::new();
        let result = generate_rust_file(&module, &type_mapper);

        // Should not panic - either Ok or Err is acceptable
        let _ = result;
    }

    #[test]
    fn test_empty_module_produces_valid_rust() {
        let module = create_empty_module();
        let type_mapper = TypeMapper::new();
        let result = generate_rust_file(&module, &type_mapper);

        assert!(result.is_ok(), "Empty module should generate valid code");

        if let Ok((code, _dependencies)) = result {
            let parse_result = syn::parse_file(&code);
            assert!(
                parse_result.is_ok(),
                "Generated code must be parseable Rust. Error: {:?}",
                parse_result.err()
            );
        }
    }

    #[test]
    fn test_empty_module_deterministic() {
        let module = create_empty_module();
        let type_mapper = TypeMapper::new();

        let output1 = generate_rust_file(&module, &type_mapper);
        let output2 = generate_rust_file(&module, &type_mapper);

        assert_eq!(output1.is_ok(), output2.is_ok());
        if let (Ok((code1, _deps1)), Ok((code2, _deps2))) = (output1, output2) {
            assert_eq!(code1, code2, "Output must be deterministic");
        }
    }
}

#[cfg(test)]
mod simple_function_tests {
    use super::*;

    #[test]
    fn test_simple_function_never_panics() {
        let mut module = create_empty_module();
        module.functions = vec![create_simple_function("test_func")];

        let type_mapper = TypeMapper::new();
        let result = generate_rust_file(&module, &type_mapper);

        // Should not panic - either Ok or Err is acceptable
        let _ = result;
    }

    #[test]
    fn test_simple_function_produces_valid_rust() {
        let mut module = create_empty_module();
        module.functions = vec![create_simple_function("simple_function")];

        let type_mapper = TypeMapper::new();
        let result = generate_rust_file(&module, &type_mapper);

        assert!(
            result.is_ok(),
            "Simple function should transpile successfully"
        );

        if let Ok((code, _dependencies)) = result {
            assert!(syn::parse_file(&code).is_ok(), "Code must be valid Rust");
            assert!(code.contains("fn "), "Should contain function declaration");
        }
    }

    #[test]
    fn test_function_name_preserved() {
        let func_name = "my_custom_function";
        let mut module = create_empty_module();
        module.functions = vec![create_simple_function(func_name)];

        let type_mapper = TypeMapper::new();
        let result = generate_rust_file(&module, &type_mapper);

        assert!(result.is_ok());

        if let Ok((code, _dependencies)) = result {
            assert!(
                code.contains(func_name),
                "Function name '{}' should appear in output",
                func_name
            );
        }
    }

    #[test]
    fn test_simple_function_deterministic() {
        let mut module = create_empty_module();
        module.functions = vec![create_simple_function("test_func")];

        let type_mapper = TypeMapper::new();

        let output1 = generate_rust_file(&module, &type_mapper);
        let output2 = generate_rust_file(&module, &type_mapper);

        assert_eq!(output1.is_ok(), output2.is_ok());
        if let (Ok((code1, _deps1)), Ok((code2, _deps2))) = (output1, output2) {
            assert_eq!(code1, code2, "Output must be deterministic");
        }
    }
}

#[cfg(test)]
mod multiple_functions_tests {
    use super::*;

    #[test]
    fn test_multiple_functions() {
        let mut module = create_empty_module();
        module.functions = vec![
            create_simple_function("func1"),
            create_simple_function("func2"),
            create_simple_function("func3"),
        ];

        let type_mapper = TypeMapper::new();
        let result = generate_rust_file(&module, &type_mapper);

        assert!(result.is_ok());

        if let Ok((code, _dependencies)) = result {
            assert!(code.contains("fn func1"));
            assert!(code.contains("fn func2"));
            assert!(code.contains("fn func3"));
            assert!(syn::parse_file(&code).is_ok());
        }
    }

    #[test]
    fn test_multiple_functions_deterministic() {
        let mut module = create_empty_module();
        module.functions = vec![
            create_simple_function("func1"),
            create_simple_function("func2"),
        ];

        let type_mapper = TypeMapper::new();

        let output1 = generate_rust_file(&module, &type_mapper);
        let output2 = generate_rust_file(&module, &type_mapper);

        assert_eq!(output1.is_ok(), output2.is_ok());
        if let (Ok((code1, _deps1)), Ok((code2, _deps2))) = (output1, output2) {
            assert_eq!(code1, code2);
        }
    }
}

#[cfg(test)]
mod function_with_params_tests {
    use super::*;

    #[test]
    fn test_function_with_one_param() {
        let mut module = create_empty_module();

        let mut func = create_simple_function("add_one");
        func.params = SmallVec::from_vec(vec![HirParam {
            name: Symbol::from("x"),
            ty: Type::Int,
            default: None,
        }]);
        func.ret_type = Type::Int;

        module.functions = vec![func];

        let type_mapper = TypeMapper::new();
        let result = generate_rust_file(&module, &type_mapper);

        assert!(result.is_ok(), "Function with param should transpile");

        if let Ok((code, _dependencies)) = result {
            assert!(code.contains("fn add_one"));
            assert!(syn::parse_file(&code).is_ok());
        }
    }

    #[test]
    fn test_function_with_multiple_params() {
        let mut module = create_empty_module();

        let mut func = create_simple_function("add");
        func.params = SmallVec::from_vec(vec![
            HirParam {
                name: Symbol::from("a"),
                ty: Type::Int,
                default: None,
            },
            HirParam {
                name: Symbol::from("b"),
                ty: Type::Int,
                default: None,
            },
        ]);
        func.ret_type = Type::Int;

        module.functions = vec![func];

        let type_mapper = TypeMapper::new();
        let result = generate_rust_file(&module, &type_mapper);

        assert!(
            result.is_ok(),
            "Function with multiple params should transpile"
        );

        if let Ok((code, _dependencies)) = result {
            assert!(code.contains("fn add"));
            assert!(syn::parse_file(&code).is_ok());
        }
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;

    /// Test that generated code is always parseable
    #[test]
    fn test_output_always_parseable() {
        let test_cases = vec![
            create_empty_module(),
            {
                let mut m = create_empty_module();
                m.functions = vec![create_simple_function("test")];
                m
            },
            {
                let mut m = create_empty_module();
                m.functions = vec![
                    create_simple_function("func1"),
                    create_simple_function("func2"),
                ];
                m
            },
        ];

        let type_mapper = TypeMapper::new();

        for module in test_cases {
            let result = generate_rust_file(&module, &type_mapper);
            if let Ok((code, _dependencies)) = result {
                assert!(
                    syn::parse_file(&code).is_ok(),
                    "All generated code must parse as valid Rust"
                );
            }
        }
    }

    /// Test that function never panics (safety property)
    #[test]
    fn test_never_panics_on_various_inputs() {
        let test_cases = vec![
            create_empty_module(),
            {
                let mut m = create_empty_module();
                m.functions = vec![create_simple_function("x")];
                m
            },
            {
                let mut m = create_empty_module();
                let mut func = create_simple_function("f");
                func.params = SmallVec::from_vec(vec![HirParam {
                    name: Symbol::from("p"),
                    ty: Type::Int,
                    default: None,
                }]);
                m.functions = vec![func];
                m
            },
        ];

        let type_mapper = TypeMapper::new();

        for module in test_cases {
            let _result = generate_rust_file(&module, &type_mapper);
            // If we reach here without panic, test passes
        }
    }
}
