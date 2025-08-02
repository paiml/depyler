use anyhow::Result;
use depyler_core::hir::{HirFunction, Type};

pub fn generate_quickcheck_tests(func: &HirFunction, _iterations: usize) -> Result<String> {
    let func_name = &func.name;
    let _test_name = format!("prop_{func_name}_properties");

    let mut test_code = String::new();

    // Add quickcheck imports
    test_code.push_str("#[cfg(test)]\n");
    test_code.push_str("mod tests {\n");
    test_code.push_str("    use super::*;\n");
    test_code.push_str("    use quickcheck::{quickcheck, TestResult, Arbitrary};\n\n");

    // Generate property test for type preservation
    if has_numeric_types(&func.params) {
        test_code.push_str(&generate_numeric_property_test(func)?);
    }

    // Generate property test for bounds checking
    if has_container_params(&func.params) {
        test_code.push_str(&generate_bounds_property_test(func)?);
    }

    // Generate property test for termination
    if func.properties.always_terminates {
        test_code.push_str(&generate_termination_test(func)?);
    }

    test_code.push_str("}\n");

    Ok(test_code)
}

fn has_numeric_types(params: &[(String, Type)]) -> bool {
    params
        .iter()
        .any(|(_, ty)| matches!(ty, Type::Int | Type::Float))
}

fn has_container_params(params: &[(String, Type)]) -> bool {
    params.iter().any(|(_, ty)| ty.is_container())
}

fn generate_numeric_property_test(func: &HirFunction) -> Result<String> {
    let func_name = &func.name;
    let mut test = String::new();

    test.push_str("    quickcheck! {\n");
    test.push_str(&format!("        fn prop_{func_name}_numeric_overflow("));

    // Generate parameters
    let param_list: Vec<String> = func
        .params
        .iter()
        .map(|(name, ty)| {
            match ty {
                Type::Int => format!("{name}: i32"),
                Type::Float => format!("{name}: f64"),
                Type::List(inner) if matches!(**inner, Type::Int) => {
                    format!("{name}: Vec<i32>")
                }
                _ => format!("{name}: i32"), // Default
            }
        })
        .collect();

    test.push_str(&param_list.join(", "));
    test.push_str(") -> TestResult {\n");

    // Add overflow checks
    test.push_str("            // Check for potential overflows\n");
    for (name, ty) in &func.params {
        if matches!(ty, Type::Int) {
            test.push_str(&format!(
                "            if {name}.checked_add(1).is_none() {{ return TestResult::discard(); }}\n"
            ));
        }
    }

    // Call the function
    test.push_str(&format!("            let result = {func_name}("));
    let args: Vec<String> = func
        .params
        .iter()
        .map(|(name, ty)| {
            if ty.is_container() {
                format!("&{name}")
            } else {
                name.clone()
            }
        })
        .collect();
    test.push_str(&args.join(", "));
    test.push_str(");\n");

    // Add basic property check
    test.push_str("            // Verify result is within expected bounds\n");
    test.push_str("            TestResult::from_bool(true) // Add specific checks\n");
    test.push_str("        }\n");
    test.push_str("    }\n\n");

    Ok(test)
}

fn generate_bounds_property_test(func: &HirFunction) -> Result<String> {
    let func_name = &func.name;
    let mut test = String::new();

    test.push_str("    quickcheck! {\n");
    test.push_str(&format!("        fn prop_{func_name}_bounds_checking("));

    // Generate parameters
    let param_list: Vec<String> = func
        .params
        .iter()
        .map(|(name, ty)| match ty {
            Type::List(inner) => {
                let inner_type = type_to_rust_string(inner);
                format!("{name}: Vec<{inner_type}>")
            }
            _ => format!("{name}: i32"),
        })
        .collect();

    test.push_str(&param_list.join(", "));
    test.push_str(") -> TestResult {\n");

    // Add empty container checks
    for (name, ty) in &func.params {
        if matches!(ty, Type::List(_)) {
            test.push_str(&format!(
                "            if {name}.is_empty() {{ return TestResult::discard(); }}\n"
            ));
        }
    }

    // Call the function
    test.push_str(&format!("            let result = {func_name}("));
    let args: Vec<String> = func
        .params
        .iter()
        .map(|(name, ty)| {
            if ty.is_container() {
                format!("&{name}")
            } else {
                name.clone()
            }
        })
        .collect();
    test.push_str(&args.join(", "));
    test.push_str(");\n");

    // Add bounds verification
    test.push_str("            // No panic means bounds were properly checked\n");
    test.push_str("            TestResult::passed()\n");
    test.push_str("        }\n");
    test.push_str("    }\n\n");

    Ok(test)
}

fn generate_termination_test(func: &HirFunction) -> Result<String> {
    let func_name = &func.name;
    let mut test = String::new();

    test.push_str("    #[test]\n");
    test.push_str(&format!("    fn test_{func_name}_terminates() {{\n"));
    test.push_str(
        "        // For functions proven to terminate, we generate specific test cases\n",
    );

    // Generate simple test cases
    test.push_str(&format!("        let result = {func_name}("));
    let args: Vec<String> = func
        .params
        .iter()
        .map(|(_, ty)| match ty {
            Type::Int => "42",
            Type::Float => "3.14",
            Type::String => "\"test\".to_string()",
            Type::Bool => "true",
            Type::List(_) => "&vec![1, 2, 3]",
            _ => "Default::default()",
        })
        .map(|s| s.to_string())
        .collect();
    test.push_str(&args.join(", "));
    test.push_str(");\n");

    test.push_str("        // Function completed without hanging\n");
    test.push_str("    }\n\n");

    Ok(test)
}

fn type_to_rust_string(ty: &Type) -> String {
    match ty {
        Type::Int => "i32".to_string(),
        Type::Float => "f64".to_string(),
        Type::String => "String".to_string(),
        Type::Bool => "bool".to_string(),
        Type::None => "()".to_string(),
        Type::List(inner) => {
            let inner_type = type_to_rust_string(inner);
            format!("Vec<{inner_type}>")
        }
        Type::Dict(k, v) => {
            let k_type = type_to_rust_string(k);
            let v_type = type_to_rust_string(v);
            format!("HashMap<{k_type}, {v_type}>")
        }
        Type::Optional(inner) => {
            let inner_type = type_to_rust_string(inner);
            format!("Option<{inner_type}>")
        }
        _ => "i32".to_string(), // Default fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_annotations::TranspilationAnnotations;
    use depyler_core::hir::FunctionProperties;

    fn create_test_function(
        name: &str,
        params: Vec<(String, Type)>,
        ret_type: Type,
        properties: FunctionProperties,
    ) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: params.into(),
            ret_type,
            body: vec![],
            properties,
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }
    }

    #[test]
    fn test_generate_quickcheck_tests_basic() {
        let func = create_test_function(
            "add",
            vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)],
            Type::Int,
            FunctionProperties::default(),
        );

        let result = generate_quickcheck_tests(&func, 100).unwrap();

        assert!(result.contains("#[cfg(test)]"));
        assert!(result.contains("mod tests"));
        assert!(result.contains("use quickcheck"));
        assert!(result.contains("prop_add_numeric_overflow"));
    }

    #[test]
    fn test_generate_quickcheck_tests_with_containers() {
        let func = create_test_function(
            "process_list",
            vec![("items".to_string(), Type::List(Box::new(Type::Int)))],
            Type::Int,
            FunctionProperties::default(),
        );

        let result = generate_quickcheck_tests(&func, 100).unwrap();

        assert!(result.contains("prop_process_list_bounds_checking"));
        assert!(result.contains("Vec<i32>"));
        assert!(result.contains("is_empty()"));
    }

    #[test]
    fn test_generate_quickcheck_tests_with_termination() {
        let properties = FunctionProperties {
            is_pure: false,
            always_terminates: true,
            panic_free: false,
            max_stack_depth: None,
            can_fail: false,
            error_types: vec![],
        };

        let func = create_test_function(
            "loop_func",
            vec![("n".to_string(), Type::Int)],
            Type::Int,
            properties,
        );

        let result = generate_quickcheck_tests(&func, 100).unwrap();

        assert!(result.contains("test_loop_func_terminates"));
        assert!(result.contains("Function completed without hanging"));
    }

    #[test]
    fn test_has_numeric_types() {
        let params_with_int = vec![("x".to_string(), Type::Int)];
        assert!(has_numeric_types(&params_with_int));

        let params_with_float = vec![("x".to_string(), Type::Float)];
        assert!(has_numeric_types(&params_with_float));

        let params_without_numeric = vec![("x".to_string(), Type::String)];
        assert!(!has_numeric_types(&params_without_numeric));
    }

    #[test]
    fn test_has_container_params() {
        let params_with_list = vec![("x".to_string(), Type::List(Box::new(Type::Int)))];
        assert!(has_container_params(&params_with_list));

        let params_with_dict = vec![(
            "x".to_string(),
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
        )];
        assert!(has_container_params(&params_with_dict));

        let params_without_containers = vec![("x".to_string(), Type::Int)];
        assert!(!has_container_params(&params_without_containers));
    }

    #[test]
    fn test_generate_numeric_property_test() {
        let func = create_test_function(
            "multiply",
            vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Float)],
            Type::Float,
            FunctionProperties::default(),
        );

        let result = generate_numeric_property_test(&func).unwrap();

        assert!(result.contains("prop_multiply_numeric_overflow"));
        assert!(result.contains("a: i32"));
        assert!(result.contains("b: f64"));
        assert!(result.contains("checked_add(1).is_none()"));
        assert!(result.contains("multiply(a, b)"));
    }

    #[test]
    fn test_generate_bounds_property_test() {
        let func = create_test_function(
            "sum_list",
            vec![("numbers".to_string(), Type::List(Box::new(Type::Int)))],
            Type::Int,
            FunctionProperties::default(),
        );

        let result = generate_bounds_property_test(&func).unwrap();

        assert!(result.contains("prop_sum_list_bounds_checking"));
        assert!(result.contains("numbers: Vec<i32>"));
        assert!(result.contains("if numbers.is_empty()"));
        assert!(result.contains("sum_list(&numbers)"));
        assert!(result.contains("TestResult::passed()"));
    }

    #[test]
    fn test_generate_termination_test() {
        let func = create_test_function(
            "factorial",
            vec![("n".to_string(), Type::Int)],
            Type::Int,
            FunctionProperties::default(),
        );

        let result = generate_termination_test(&func).unwrap();

        assert!(result.contains("test_factorial_terminates"));
        assert!(result.contains("factorial(42)"));
        assert!(result.contains("Function completed without hanging"));
    }

    #[test]
    fn test_type_to_rust_string() {
        assert_eq!(type_to_rust_string(&Type::Int), "i32");
        assert_eq!(type_to_rust_string(&Type::Float), "f64");
        assert_eq!(type_to_rust_string(&Type::String), "String");
        assert_eq!(type_to_rust_string(&Type::Bool), "bool");
        assert_eq!(type_to_rust_string(&Type::None), "()");

        assert_eq!(
            type_to_rust_string(&Type::List(Box::new(Type::Int))),
            "Vec<i32>"
        );

        assert_eq!(
            type_to_rust_string(&Type::Dict(Box::new(Type::String), Box::new(Type::Int))),
            "HashMap<String, i32>"
        );

        assert_eq!(
            type_to_rust_string(&Type::Optional(Box::new(Type::String))),
            "Option<String>"
        );

        // Test unknown type fallback
        assert_eq!(type_to_rust_string(&Type::Unknown), "i32");
    }

    #[test]
    fn test_generate_quickcheck_tests_no_properties() {
        let func = create_test_function(
            "simple",
            vec![("x".to_string(), Type::String)],
            Type::String,
            FunctionProperties::default(),
        );

        let result = generate_quickcheck_tests(&func, 100).unwrap();

        // Should still have basic structure but no specific tests
        assert!(result.contains("#[cfg(test)]"));
        assert!(result.contains("mod tests"));
        assert!(result.contains("use quickcheck"));

        // Should not contain specific property tests
        assert!(!result.contains("prop_simple_numeric_overflow"));
        assert!(!result.contains("prop_simple_bounds_checking"));
        assert!(!result.contains("test_simple_terminates"));
    }

    #[test]
    fn test_generate_quickcheck_complex_function() {
        let properties = FunctionProperties {
            is_pure: true,
            always_terminates: true,
            panic_free: true,
            max_stack_depth: Some(10),
            can_fail: false,
            error_types: vec![],
        };

        let func = create_test_function(
            "complex_func",
            vec![
                ("nums".to_string(), Type::List(Box::new(Type::Int))),
                ("threshold".to_string(), Type::Float),
                ("count".to_string(), Type::Int),
            ],
            Type::Bool,
            properties,
        );

        let result = generate_quickcheck_tests(&func, 100).unwrap();

        // Should contain tests for both numeric and container properties
        assert!(result.contains("prop_complex_func_numeric_overflow"));
        assert!(result.contains("prop_complex_func_bounds_checking"));
        assert!(result.contains("test_complex_func_terminates"));

        // Check parameter types
        assert!(result.contains("Vec<i32>"));
        assert!(result.contains("f64"));
        assert!(result.contains("i32"));
    }
}
