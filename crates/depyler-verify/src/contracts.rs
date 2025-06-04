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
                        name: format!("{}_not_null", param_name),
                        expression: format!("{} is not None", param_name),
                        description: format!("Parameter {} must not be null", param_name),
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
            checks.push_str(&format!("fn check_{}_preconditions(", func_name));
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
            checks.push_str(&format!("fn check_{}_postconditions(", func_name));
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
