use depyler_core::hir::{HirExpr, HirFunction, HirStmt, Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub preconditions: Vec<Condition>,
    pub postconditions: Vec<Condition>,
    pub invariants: Vec<Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub expression: String,
    pub description: String,
}

pub struct ContractChecker;

impl ContractChecker {
    pub fn extract_contracts(func: &HirFunction) -> Contract {
        let mut contract = Contract {
            preconditions: vec![],
            postconditions: vec![],
            invariants: vec![],
        };

        // Extract implicit preconditions from parameter types
        for (param_name, param_type) in &func.params {
            match param_type {
                Type::List(_) => {
                    contract.preconditions.push(Condition {
                        name: format!("{param_name}_not_null"),
                        expression: format!("{param_name} is not None"),
                        description: format!("Parameter {param_name} must not be null"),
                    });
                }
                Type::Int => {
                    // Could add range constraints if needed
                }
                _ => {}
            }
        }

        // Extract implicit postconditions from return type
        match &func.ret_type {
            Type::Optional(_) => {
                contract.postconditions.push(Condition {
                    name: "result_valid".to_string(),
                    expression: "result is None or result meets type constraints".to_string(),
                    description: "Result must be None or valid value".to_string(),
                });
            }
            Type::List(_) => {
                contract.postconditions.push(Condition {
                    name: "result_not_null".to_string(),
                    expression: "result is not None".to_string(),
                    description: "Result list must not be null".to_string(),
                });
            }
            _ => {}
        }

        // Extract invariants from function properties
        if func.properties.panic_free {
            contract.invariants.push(Condition {
                name: "no_panics".to_string(),
                expression: "all array accesses are bounds-checked".to_string(),
                description: "Function must not panic on any input".to_string(),
            });
        }

        if func.properties.always_terminates {
            contract.invariants.push(Condition {
                name: "termination".to_string(),
                expression: "loop variants decrease monotonically".to_string(),
                description: "Function must terminate for all inputs".to_string(),
            });
        }

        contract
    }

    pub fn generate_contract_checks(contract: &Contract, func_name: &str) -> String {
        let mut checks = String::new();

        // Generate precondition checks
        if !contract.preconditions.is_empty() {
            checks.push_str(&format!("fn check_{func_name}_preconditions("));
            checks.push_str("/* params */) -> Result<(), &'static str> {\n");

            for pre in &contract.preconditions {
                checks.push_str(&format!("    // {}\n", pre.description));
                checks.push_str(&format!("    // TODO: Check {}\n", pre.expression));
            }

            checks.push_str("    Ok(())\n");
            checks.push_str("}\n\n");
        }

        // Generate postcondition checks
        if !contract.postconditions.is_empty() {
            checks.push_str(&format!("fn check_{func_name}_postconditions("));
            checks.push_str("/* result */) -> Result<(), &'static str> {\n");

            for post in &contract.postconditions {
                checks.push_str(&format!("    // {}\n", post.description));
                checks.push_str(&format!("    // TODO: Check {}\n", post.expression));
            }

            checks.push_str("    Ok(())\n");
            checks.push_str("}\n\n");
        }

        checks
    }

    pub fn check_contract_violations(func: &HirFunction) -> Vec<String> {
        let mut violations = Vec::new();

        // Check for potential contract violations in the function body
        for stmt in &func.body {
            violations.extend(check_stmt_contracts(stmt));
        }

        violations
    }
}

fn check_stmt_contracts(stmt: &HirStmt) -> Vec<String> {
    let mut violations = Vec::new();

    match stmt {
        HirStmt::Assign { value, .. } => {
            violations.extend(check_expr_contracts(value));
        }
        HirStmt::Return(Some(expr)) => {
            violations.extend(check_expr_contracts(expr));
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            violations.extend(check_expr_contracts(condition));
            for s in then_body {
                violations.extend(check_stmt_contracts(s));
            }
            if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    violations.extend(check_stmt_contracts(s));
                }
            }
        }
        HirStmt::While { condition, body } => {
            violations.extend(check_expr_contracts(condition));
            for s in body {
                violations.extend(check_stmt_contracts(s));
            }
        }
        HirStmt::For { iter, body, .. } => {
            violations.extend(check_expr_contracts(iter));
            for s in body {
                violations.extend(check_stmt_contracts(s));
            }
        }
        HirStmt::Expr(expr) => {
            violations.extend(check_expr_contracts(expr));
        }
        _ => {}
    }

    violations
}

fn check_expr_contracts(expr: &HirExpr) -> Vec<String> {
    let mut violations = Vec::new();

    match expr {
        HirExpr::Index { base, .. } => {
            // Unchecked array access could violate panic-free contract
            violations.push("Potential array bounds violation".to_string());
            violations.extend(check_expr_contracts(base));
        }
        HirExpr::Binary { left, right, .. } => {
            violations.extend(check_expr_contracts(left));
            violations.extend(check_expr_contracts(right));
        }
        HirExpr::Call { args, .. } => {
            for arg in args {
                violations.extend(check_expr_contracts(arg));
            }
        }
        _ => {}
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_annotations::TranspilationAnnotations;
    use depyler_core::hir::{FunctionProperties, Literal};

    fn create_test_function(
        name: &str,
        params: Vec<(String, Type)>,
        ret_type: Type,
        body: Vec<HirStmt>,
        properties: FunctionProperties,
    ) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: params.into(),
            ret_type,
            body,
            properties,
            annotations: TranspilationAnnotations::default(),
        }
    }

    #[test]
    fn test_contract_creation() {
        let contract = Contract {
            preconditions: vec![Condition {
                name: "param_valid".to_string(),
                expression: "param > 0".to_string(),
                description: "Parameter must be positive".to_string(),
            }],
            postconditions: vec![Condition {
                name: "result_valid".to_string(),
                expression: "result >= 0".to_string(),
                description: "Result must be non-negative".to_string(),
            }],
            invariants: vec![Condition {
                name: "no_overflow".to_string(),
                expression: "no arithmetic overflow".to_string(),
                description: "Arithmetic operations must not overflow".to_string(),
            }],
        };

        assert_eq!(contract.preconditions.len(), 1);
        assert_eq!(contract.postconditions.len(), 1);
        assert_eq!(contract.invariants.len(), 1);
        assert_eq!(contract.preconditions[0].name, "param_valid");
    }

    #[test]
    fn test_condition_creation() {
        let condition = Condition {
            name: "test_condition".to_string(),
            expression: "x > 0".to_string(),
            description: "x must be positive".to_string(),
        };

        assert_eq!(condition.name, "test_condition");
        assert_eq!(condition.expression, "x > 0");
        assert_eq!(condition.description, "x must be positive");
    }

    #[test]
    fn test_extract_contracts_with_list_param() {
        let func = create_test_function(
            "process_list",
            vec![("items".to_string(), Type::List(Box::new(Type::Int)))],
            Type::Int,
            vec![],
            FunctionProperties::default(),
        );

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.preconditions.len(), 1);
        assert_eq!(contract.preconditions[0].name, "items_not_null");
        assert!(contract.preconditions[0]
            .expression
            .contains("items is not None"));
    }

    #[test]
    fn test_extract_contracts_with_int_param() {
        let func = create_test_function(
            "calculate",
            vec![("num".to_string(), Type::Int)],
            Type::Int,
            vec![],
            FunctionProperties::default(),
        );

        let contract = ContractChecker::extract_contracts(&func);

        // Int parameters don't generate preconditions in the current implementation
        assert_eq!(contract.preconditions.len(), 0);
    }

    #[test]
    fn test_extract_contracts_with_optional_return() {
        let func = create_test_function(
            "find_item",
            vec![],
            Type::Optional(Box::new(Type::String)),
            vec![],
            FunctionProperties::default(),
        );

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.postconditions.len(), 1);
        assert_eq!(contract.postconditions[0].name, "result_valid");
        assert!(contract.postconditions[0]
            .expression
            .contains("result is None or result meets type constraints"));
    }

    #[test]
    fn test_extract_contracts_with_list_return() {
        let func = create_test_function(
            "get_items",
            vec![],
            Type::List(Box::new(Type::String)),
            vec![],
            FunctionProperties::default(),
        );

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.postconditions.len(), 1);
        assert_eq!(contract.postconditions[0].name, "result_not_null");
        assert_eq!(contract.postconditions[0].expression, "result is not None");
    }

    #[test]
    fn test_extract_contracts_with_panic_free_property() {
        let properties = FunctionProperties {
            is_pure: false,
            always_terminates: false,
            panic_free: true,
            max_stack_depth: Some(100),
        };

        let func = create_test_function("safe_function", vec![], Type::Int, vec![], properties);

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.invariants.len(), 1);
        assert_eq!(contract.invariants[0].name, "no_panics");
        assert!(contract.invariants[0]
            .expression
            .contains("array accesses are bounds-checked"));
    }

    #[test]
    fn test_extract_contracts_with_termination_property() {
        let properties = FunctionProperties {
            is_pure: false,
            always_terminates: true,
            panic_free: false,
            max_stack_depth: Some(100),
        };

        let func = create_test_function(
            "terminating_function",
            vec![],
            Type::Int,
            vec![],
            properties,
        );

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.invariants.len(), 1);
        assert_eq!(contract.invariants[0].name, "termination");
        assert!(contract.invariants[0]
            .expression
            .contains("loop variants decrease monotonically"));
    }

    #[test]
    fn test_extract_contracts_with_all_properties() {
        let properties = FunctionProperties {
            is_pure: true,
            always_terminates: true,
            panic_free: true,
            max_stack_depth: Some(100),
        };

        let func = create_test_function(
            "perfect_function",
            vec![("data".to_string(), Type::List(Box::new(Type::Int)))],
            Type::List(Box::new(Type::Int)),
            vec![],
            properties,
        );

        let contract = ContractChecker::extract_contracts(&func);

        // Should have preconditions for list param
        assert_eq!(contract.preconditions.len(), 1);
        // Should have postconditions for list return
        assert_eq!(contract.postconditions.len(), 1);
        // Should have invariants for both properties
        assert_eq!(contract.invariants.len(), 2);
    }

    #[test]
    fn test_generate_contract_checks() {
        let contract = Contract {
            preconditions: vec![Condition {
                name: "param_positive".to_string(),
                expression: "param > 0".to_string(),
                description: "Parameter must be positive".to_string(),
            }],
            postconditions: vec![Condition {
                name: "result_valid".to_string(),
                expression: "result >= 0".to_string(),
                description: "Result must be non-negative".to_string(),
            }],
            invariants: vec![],
        };

        let checks = ContractChecker::generate_contract_checks(&contract, "test_func");

        assert!(checks.contains("check_test_func_preconditions"));
        assert!(checks.contains("check_test_func_postconditions"));
        assert!(checks.contains("Parameter must be positive"));
        assert!(checks.contains("Result must be non-negative"));
        assert!(checks.contains("TODO: Check param > 0"));
        assert!(checks.contains("TODO: Check result >= 0"));
    }

    #[test]
    fn test_generate_contract_checks_empty() {
        let contract = Contract {
            preconditions: vec![],
            postconditions: vec![],
            invariants: vec![],
        };

        let checks = ContractChecker::generate_contract_checks(&contract, "empty_func");

        // Should be empty for contract with no conditions
        assert!(checks.is_empty());
    }

    #[test]
    fn test_check_contract_violations_with_index() {
        let body = vec![HirStmt::Assign {
            target: "result".to_string(),
            value: HirExpr::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
        }];

        let func = create_test_function(
            "access_array",
            vec![],
            Type::Int,
            body,
            FunctionProperties::default(),
        );

        let violations = ContractChecker::check_contract_violations(&func);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0], "Potential array bounds violation");
    }

    #[test]
    fn test_check_contract_violations_with_nested_expressions() {
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: depyler_core::hir::BinOp::Add,
            left: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr1".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            }),
            right: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr2".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(1))),
            }),
        }))];

        let func = create_test_function(
            "add_array_elements",
            vec![],
            Type::Int,
            body,
            FunctionProperties::default(),
        );

        let violations = ContractChecker::check_contract_violations(&func);

        // Should detect two potential array bounds violations
        assert_eq!(violations.len(), 2);
        assert!(violations
            .iter()
            .all(|v| v == "Potential array bounds violation"));
    }

    #[test]
    fn test_check_contract_violations_with_control_flow() {
        let then_body = vec![HirStmt::Assign {
            target: "x".to_string(),
            value: HirExpr::Index {
                base: Box::new(HirExpr::Var("data".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
        }];

        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body,
            else_body: None,
        }];

        let func = create_test_function(
            "conditional_access",
            vec![],
            Type::Int,
            body,
            FunctionProperties::default(),
        );

        let violations = ContractChecker::check_contract_violations(&func);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0], "Potential array bounds violation");
    }

    #[test]
    fn test_check_contract_violations_no_violations() {
        let body = vec![
            HirStmt::Assign {
                target: "result".to_string(),
                value: HirExpr::Literal(Literal::Int(42)),
            },
            HirStmt::Return(Some(HirExpr::Var("result".to_string()))),
        ];

        let func = create_test_function(
            "safe_function",
            vec![],
            Type::Int,
            body,
            FunctionProperties::default(),
        );

        let violations = ContractChecker::check_contract_violations(&func);

        assert!(violations.is_empty());
    }

    #[test]
    fn test_contract_serialization() {
        let contract = Contract {
            preconditions: vec![Condition {
                name: "test".to_string(),
                expression: "x > 0".to_string(),
                description: "Test condition".to_string(),
            }],
            postconditions: vec![],
            invariants: vec![],
        };

        // Test that contract can be serialized to JSON
        let json = serde_json::to_string(&contract).unwrap();
        assert!(json.contains("\"name\":\"test\""));
        assert!(json.contains("\"expression\":\"x > 0\""));

        // Test that it can be deserialized back
        let deserialized: Contract = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.preconditions.len(), 1);
        assert_eq!(deserialized.preconditions[0].name, "test");
    }
}
