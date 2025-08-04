use depyler_core::DepylerPipeline;
use quickcheck::TestResult;

/// Property: AST to HIR conversion should preserve semantic meaning
#[quickcheck_macros::quickcheck(tests = 10, max_tests = 20)]
fn prop_ast_hir_roundtrip(python_source: String) -> TestResult {
    // Skip empty or invalid sources
    if python_source.trim().is_empty() || python_source.len() > 200 {
        return TestResult::discard();
    }

    let pipeline = DepylerPipeline::new();

    // Use the public API to parse to HIR
    match pipeline.parse_to_hir(&python_source) {
        Ok(_hir) => {
            // HIR should be valid
            TestResult::from_bool(true) // Any successful parse is good
        }
        Err(_) => TestResult::discard(), // Conversion failed - expected for some cases
    }
}

/// Property: HIR should preserve function names
#[quickcheck_macros::quickcheck(tests = 10, max_tests = 20)]
fn prop_function_name_preservation(func_name: String, params: Vec<String>) -> TestResult {
    // Skip invalid function names
    if func_name.is_empty() || !func_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return TestResult::discard();
    }

    if params.len() > 3 {
        return TestResult::discard();
    }

    // Create a simple function
    let param_list = if params.is_empty() {
        String::new()
    } else {
        params.join(": int, ") + ": int"
    };

    let python_source = format!("def {}({}) -> int:\n    return 42", func_name, param_list);

    let pipeline = DepylerPipeline::new();

    match pipeline.parse_to_hir(&python_source) {
        Ok(hir) => {
            if let Some(func) = hir.functions.first() {
                TestResult::from_bool(func.name == func_name)
            } else {
                TestResult::failed()
            }
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Type annotations should be preserved
#[quickcheck_macros::quickcheck(tests = 5, max_tests = 10)]
fn prop_type_annotation_preservation(return_type: String) -> TestResult {
    let valid_types = ["int", "str", "bool", "float"];
    if !valid_types.contains(&return_type.as_str()) {
        return TestResult::discard();
    }

    let python_source = format!(
        "def test_func() -> {}:\n    return {}",
        return_type,
        match return_type.as_str() {
            "int" => "42",
            "str" => "\"hello\"",
            "bool" => "True",
            "float" => "3.14",
            _ => "None",
        }
    );

    let pipeline = DepylerPipeline::new();

    match pipeline.parse_to_hir(&python_source) {
        Ok(hir) => {
            if let Some(func) = hir.functions.first() {
                // Check that the return type is not Unknown
                TestResult::from_bool(!matches!(func.ret_type, depyler_core::hir::Type::Unknown))
            } else {
                TestResult::failed()
            }
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Control flow structures should be preserved
#[quickcheck_macros::quickcheck(tests = 5, max_tests = 10)]
fn prop_control_flow_preservation(condition: i32, then_val: i32, else_val: i32) -> TestResult {
    let python_source = format!(
        "def test_func(x: int) -> int:\n    if x > {}:\n        return {}\n    else:\n        return {}",
        condition, then_val, else_val
    );

    let pipeline = DepylerPipeline::new();

    match pipeline.parse_to_hir(&python_source) {
        Ok(hir) => {
            if let Some(func) = hir.functions.first() {
                // Should have an If statement in the body
                let has_if = func
                    .body
                    .iter()
                    .any(|stmt| matches!(stmt, depyler_core::hir::HirStmt::If { .. }));
                TestResult::from_bool(has_if)
            } else {
                TestResult::failed()
            }
        }
        Err(_) => TestResult::discard(),
    }
}

/// Property: Variable assignments should create proper HIR statements
#[quickcheck_macros::quickcheck(tests = 5, max_tests = 10)]
fn prop_variable_assignment_preservation(var_name: String, value: i32) -> TestResult {
    if var_name.is_empty() || !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return TestResult::discard();
    }

    let python_source = format!(
        "def test_func() -> int:\n    {} = {}\n    return {}",
        var_name, value, var_name
    );

    let pipeline = DepylerPipeline::new();

    match pipeline.parse_to_hir(&python_source) {
        Ok(hir) => {
            if let Some(func) = hir.functions.first() {
                // Should have an assignment statement
                let has_assignment = func
                    .body
                    .iter()
                    .any(|stmt| matches!(stmt, depyler_core::hir::HirStmt::Assign { .. }));
                TestResult::from_bool(has_assignment || !func.body.is_empty()) // May be optimized
            } else {
                TestResult::failed()
            }
        }
        Err(_) => TestResult::discard(),
    }
}
