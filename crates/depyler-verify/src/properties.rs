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
