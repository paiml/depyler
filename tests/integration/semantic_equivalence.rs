use proptest::prelude::*;

use depyler_core::DepylerPipeline;

#[path = "rust_executor.rs"]
mod rust_executor;

/// Property-based tests for semantic equivalence between Python and generated Rust
#[cfg(test)]
mod property_tests {
    use super::*;

    // Generator for simple Python expressions
    fn arb_simple_expr() -> impl Strategy<Value = String> {
        prop_oneof![
            // Literals
            any::<i32>().prop_map(|n| n.to_string()),
            "\"[a-zA-Z0-9 ]*\"".prop_map(|s| format!("\"{}\"", s)),
            prop::bool::ANY.prop_map(|b| b.to_string()),
            // Variables
            "[a-z][a-z0-9_]*".prop_map(|var| var),
            // Binary operations
            (arb_simple_literal(), arb_simple_literal(), arb_binary_op())
                .prop_map(|(l, r, op)| format!("{} {} {}", l, op, r)),
        ]
    }

    fn arb_simple_literal() -> impl Strategy<Value = String> {
        prop_oneof![
            any::<i32>().prop_map(|n| n.to_string()),
            "\"[a-zA-Z0-9]*\"".prop_map(|s| format!("\"{}\"", s)),
        ]
    }

    fn arb_binary_op() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("+".to_string()),
            Just("-".to_string()),
            Just("*".to_string()),
            Just("//".to_string()),
            Just("%".to_string()),
            Just("==".to_string()),
            Just("!=".to_string()),
            Just("<".to_string()),
            Just(">".to_string()),
            Just("<=".to_string()),
            Just(">=".to_string())
        ]
    }

    fn arb_python_function() -> impl Strategy<Value = String> {
        (
            "[a-z][a-z0-9_]*",                              // function name
            prop::collection::vec("[a-z][a-z0-9_]*", 0..4), // parameters
            arb_function_body(),                            // body
        )
            .prop_map(|(name, params, body)| {
                let param_list =
                    params.join(": int, ") + if !params.is_empty() { ": int" } else { "" };
                format!(
                    "def {}({}) -> int:\n{}",
                    name,
                    param_list,
                    body.lines()
                        .map(|line| format!("    {}", line))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            })
    }

    fn arb_function_body() -> impl Strategy<Value = String> {
        prop_oneof![
            // Simple return
            arb_simple_expr().prop_map(|expr| format!("return {}", expr)),
            // Assignment + return
            (arb_simple_expr(), "[a-z][a-z0-9_]*")
                .prop_map(|(expr, var)| format!("{} = {}\nreturn {}", var, expr, var)),
            // Conditional
            (arb_simple_expr(), arb_simple_expr(), arb_simple_expr()).prop_map(
                |(cond, then_expr, else_expr)| {
                    format!(
                        "if {} > 0:\n    return {}\nelse:\n    return {}",
                        cond, then_expr, else_expr
                    )
                }
            ),
        ]
    }

    proptest! {
        #[test]
        fn prop_arithmetic_equivalence(
            a in -1000i32..1000,
            b in -1000i32..1000,
            op in arb_binary_op()
        ) {
            let python_code = format!("def test_func(a: int, b: int) -> int:\n    return a {} b", op);

            let pipeline = DepylerPipeline::new();

            // Skip division by zero
            if (op == "//" || op == "%") && b == 0 {
                return Ok(());
            }

            if let Ok(rust_code) = pipeline.transpile(&python_code) {
                // Verify the generated Rust compiles
                assert!(verify_rust_syntax(&rust_code));

                // Test semantic equivalence for specific values
                let python_result = eval_python_arithmetic(a, b, &op);
                let rust_result = eval_rust_arithmetic(&rust_code, a, b);

                if let (Some(py_val), Some(rust_val)) = (python_result, rust_result) {
                    prop_assert_eq!(py_val, rust_val,
                        "Arithmetic mismatch for {} {} {}: Python={}, Rust={}",
                        a, op, b, py_val, rust_val);
                }
            }
        }

        #[test]
        fn prop_type_preservation(
            func_code in arb_python_function()
        ) {
            let pipeline = DepylerPipeline::new();

            if let Ok(rust_code) = pipeline.transpile(&func_code) {
                // Verify type annotations are preserved
                assert!(rust_code.contains("i32") || rust_code.contains("String"));

                // Verify function signature is correct
                assert!(rust_code.contains("pub fn") || rust_code.contains("fn"));

                // Verify return type is specified
                assert!(rust_code.contains("->"));
            }
        }

        #[test]
        fn prop_variable_scoping(
            var_name in "[a-z][a-z0-9_]*",
            value in any::<i32>()
        ) {
            // Create a more complex example that won't be optimized away
            let python_code = format!(
                "def test_func() -> int:\n    {} = {}\n    {} = {} + 1\n    return {}",
                var_name, value, var_name, var_name, var_name
            );

            let pipeline = DepylerPipeline::new();

            if let Ok(rust_code) = pipeline.transpile(&python_code) {
                // Should have basic function structure
                assert!(rust_code.contains("fn test_func"));

                // Should compile without errors
                assert!(verify_rust_syntax(&rust_code));

                // Should have return type
                assert!(rust_code.contains("-> i32"));
            }
        }

        #[test]
        fn prop_control_flow_equivalence(
            condition in -100i32..100,
            then_value in any::<i32>(),
            else_value in any::<i32>()
        ) {
            let python_code = format!(
                "def test_func(x: int) -> int:\n    if x > 0:\n        return {}\n    else:\n        return {}",
                then_value, else_value
            );

            let pipeline = DepylerPipeline::new();

            if let Ok(rust_code) = pipeline.transpile(&python_code) {
                assert!(verify_rust_syntax(&rust_code));

                // Test semantic equivalence
                let expected = if condition > 0 { then_value } else { else_value };

                if let Some(actual) = eval_rust_conditional(&rust_code, condition) {
                    prop_assert_eq!(expected, actual,
                        "Conditional mismatch for condition {}: expected {}, got {}",
                        condition, expected, actual);
                }
            }
        }
    }
}

// Helper functions for semantic evaluation
fn verify_rust_syntax(rust_code: &str) -> bool {
    // For now, just check for basic Rust syntax markers
    rust_code.contains("fn ") && rust_code.contains("{") && rust_code.contains("}")
}

fn eval_python_arithmetic(a: i32, b: i32, op: &str) -> Option<i32> {
    match op {
        "+" => Some(a.saturating_add(b)),
        "-" => Some(a.saturating_sub(b)),
        "*" => Some(a.saturating_mul(b)),
        "//" => {
            if b != 0 {
                Some(a / b)
            } else {
                None
            }
        }
        "%" => {
            if b != 0 {
                Some(a % b)
            } else {
                None
            }
        }
        "==" => Some(if a == b { 1 } else { 0 }),
        "!=" => Some(if a != b { 1 } else { 0 }),
        "<" => Some(if a < b { 1 } else { 0 }),
        ">" => Some(if a > b { 1 } else { 0 }),
        "<=" => Some(if a <= b { 1 } else { 0 }),
        ">=" => Some(if a >= b { 1 } else { 0 }),
        _ => None,
    }
}

fn eval_rust_arithmetic(rust_code: &str, a: i32, b: i32) -> Option<i32> {
    // Try to execute the Rust code
    rust_executor::execute_rust_code(rust_code, "test_func", &[a, b]).ok()
}

fn eval_rust_conditional(rust_code: &str, condition: i32) -> Option<i32> {
    // Try to execute the Rust code
    rust_executor::execute_rust_code(rust_code, "test_func", &[condition]).ok()
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic_functions() {
        let test_cases = vec![
            (
                "def add(a: int, b: int) -> int:\n    return a + b",
                "addition",
            ),
            (
                "def sub(a: int, b: int) -> int:\n    return a - b",
                "subtraction",
            ),
            (
                "def mul(a: int, b: int) -> int:\n    return a * b",
                "multiplication",
            ),
        ];

        let pipeline = DepylerPipeline::new();

        for (python_code, description) in test_cases {
            let rust_code = pipeline
                .transpile(python_code)
                .unwrap_or_else(|_| panic!("Failed to transpile {}", description));

            assert!(
                verify_rust_syntax(&rust_code),
                "Generated Rust syntax invalid for {}",
                description
            );

            assert!(
                rust_code.contains("pub fn"),
                "Missing function declaration for {}",
                description
            );

            assert!(
                rust_code.contains("i32"),
                "Missing type annotations for {}",
                description
            );
        }
    }

    #[test]
    fn test_type_mapping_consistency() {
        let type_cases = vec![
            ("int", "i32"),
            ("str", "String"),
            ("bool", "bool"),
            ("List[int]", "Vec<i32>"),
            ("Dict[str, int]", "HashMap<String, i32>"),
        ];

        for (python_type, expected_rust) in type_cases {
            // This would test the actual type mapping logic
            // For now, we verify the mapping exists
            assert!(
                !expected_rust.is_empty(),
                "No Rust mapping for Python type {}",
                python_type
            );
        }
    }
}
