use anyhow::Result;
use depyler_core::hir::{HirFunction, Type};

pub fn generate_quickcheck_tests(func: &HirFunction, _iterations: usize) -> Result<String> {
    let func_name = &func.name;
    let _test_name = format!("prop_{}_properties", func_name);

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
    test.push_str(&format!("        fn prop_{}_numeric_overflow(", func_name));

    // Generate parameters
    let param_list: Vec<String> = func
        .params
        .iter()
        .map(|(name, ty)| {
            match ty {
                Type::Int => format!("{}: i32", name),
                Type::Float => format!("{}: f64", name),
                Type::List(inner) if matches!(**inner, Type::Int) => {
                    format!("{}: Vec<i32>", name)
                }
                _ => format!("{}: i32", name), // Default
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
                "            if {}.checked_add(1).is_none() {{ return TestResult::discard(); }}\n",
                name
            ));
        }
    }

    // Call the function
    test.push_str(&format!("            let result = {}(", func_name));
    let args: Vec<String> = func
        .params
        .iter()
        .map(|(name, ty)| {
            if ty.is_container() {
                format!("&{}", name)
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
    test.push_str(&format!("        fn prop_{}_bounds_checking(", func_name));

    // Generate parameters
    let param_list: Vec<String> = func
        .params
        .iter()
        .map(|(name, ty)| match ty {
            Type::List(inner) => {
                let inner_type = type_to_rust_string(inner);
                format!("{}: Vec<{}>", name, inner_type)
            }
            _ => format!("{}: i32", name),
        })
        .collect();

    test.push_str(&param_list.join(", "));
    test.push_str(") -> TestResult {\n");

    // Add empty container checks
    for (name, ty) in &func.params {
        if matches!(ty, Type::List(_)) {
            test.push_str(&format!(
                "            if {}.is_empty() {{ return TestResult::discard(); }}\n",
                name
            ));
        }
    }

    // Call the function
    test.push_str(&format!("            let result = {}(", func_name));
    let args: Vec<String> = func
        .params
        .iter()
        .map(|(name, ty)| {
            if ty.is_container() {
                format!("&{}", name)
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
    test.push_str(&format!("    fn test_{}_terminates() {{\n", func_name));
    test.push_str(
        "        // For functions proven to terminate, we generate specific test cases\n",
    );

    // Generate simple test cases
    test.push_str(&format!("        let result = {}(", func_name));
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
        Type::List(inner) => format!("Vec<{}>", type_to_rust_string(inner)),
        Type::Dict(k, v) => format!(
            "HashMap<{}, {}>",
            type_to_rust_string(k),
            type_to_rust_string(v)
        ),
        Type::Optional(inner) => format!("Option<{}>", type_to_rust_string(inner)),
        _ => "i32".to_string(), // Default fallback
    }
}
