use crate::hir::{BinOp, FunctionProperties, HirExpr, HirStmt};

pub struct FunctionAnalyzer;

impl FunctionAnalyzer {
    pub fn analyze(body: &[HirStmt]) -> FunctionProperties {
        let (can_fail, error_types) = Self::check_can_fail(body);
        FunctionProperties {
            is_pure: Self::check_pure(body),
            always_terminates: Self::check_termination(body),
            panic_free: Self::check_panic_free(body),
            max_stack_depth: Self::calculate_max_stack_depth(body),
            can_fail,
            error_types,
            is_async: false, // Set by AST bridge when needed
        }
    }

    fn check_pure(body: &[HirStmt]) -> bool {
        // V1: Conservative - only if no calls to unknown functions
        for stmt in body {
            if Self::has_side_effects(stmt) {
                return false;
            }
        }
        true
    }

    fn has_side_effects(stmt: &HirStmt) -> bool {
        match stmt {
            HirStmt::Expr(HirExpr::Call { func, .. }) => {
                // Whitelist of pure functions
                !matches!(func.as_str(), "len" | "max" | "min" | "sum" | "abs")
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                then_body.iter().any(Self::has_side_effects)
                    || else_body
                        .as_ref()
                        .is_some_and(|b| b.iter().any(Self::has_side_effects))
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                body.iter().any(Self::has_side_effects)
            }
            _ => false,
        }
    }

    fn check_termination(body: &[HirStmt]) -> bool {
        // V1: Only guarantee for simple cases
        for stmt in body {
            if let HirStmt::While { .. } = stmt {
                return false; // Can't guarantee termination with while loops
            }
            if let HirStmt::For { iter, .. } = stmt {
                // Only guarantee for finite iterators
                if !Self::is_finite_iterator(iter) {
                    return false;
                }
            }
        }
        true
    }

    fn is_finite_iterator(expr: &HirExpr) -> bool {
        match expr {
            HirExpr::List(_) | HirExpr::Tuple(_) | HirExpr::Dict(_) => true,
            HirExpr::Call { func, .. } => {
                matches!(func.as_str(), "range" | "enumerate" | "zip")
            }
            _ => false,
        }
    }

    fn check_panic_free(body: &[HirStmt]) -> bool {
        // V1: Check for obvious panic cases
        for stmt in body {
            if Self::has_panic_risk(stmt) {
                return false;
            }
        }
        true
    }

    fn has_panic_risk(stmt: &HirStmt) -> bool {
        match stmt {
            HirStmt::Expr(expr) | HirStmt::Assign { value: expr, .. } => {
                Self::expr_has_panic_risk(expr)
            }
            HirStmt::Return(Some(expr)) => Self::expr_has_panic_risk(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                Self::expr_has_panic_risk(condition)
                    || then_body.iter().any(Self::has_panic_risk)
                    || else_body
                        .as_ref()
                        .is_some_and(|b| b.iter().any(Self::has_panic_risk))
            }
            HirStmt::While { condition, body } => {
                Self::expr_has_panic_risk(condition) || body.iter().any(Self::has_panic_risk)
            }
            HirStmt::For { iter, body, .. } => {
                Self::expr_has_panic_risk(iter) || body.iter().any(Self::has_panic_risk)
            }
            HirStmt::Raise { .. } => true, // Raise statements can fail
            _ => false,
        }
    }

    fn expr_has_panic_risk(expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Index { .. } => true, // Array bounds
            HirExpr::Binary {
                op: BinOp::Div | BinOp::FloorDiv | BinOp::Mod,
                ..
            } => true, // Division by zero
            HirExpr::Binary { left, right, .. } => {
                Self::expr_has_panic_risk(left) || Self::expr_has_panic_risk(right)
            }
            HirExpr::Call { args, .. } => args.iter().any(Self::expr_has_panic_risk),
            _ => false,
        }
    }

    fn check_can_fail(body: &[HirStmt]) -> (bool, Vec<String>) {
        let mut error_types = Vec::new();
        let mut can_fail = false;

        for stmt in body {
            let (stmt_can_fail, mut stmt_errors) = Self::stmt_can_fail(stmt);
            if stmt_can_fail {
                can_fail = true;
                error_types.append(&mut stmt_errors);
            }
        }

        // Remove duplicates
        error_types.sort();
        error_types.dedup();

        (can_fail, error_types)
    }

    fn stmt_can_fail(stmt: &HirStmt) -> (bool, Vec<String>) {
        match stmt {
            HirStmt::Raise { exception, .. } => {
                let error_type = Self::extract_exception_type(exception);
                (true, vec![error_type])
            }
            HirStmt::Expr(expr) | HirStmt::Assign { value: expr, .. } => Self::expr_can_fail(expr),
            HirStmt::Return(Some(expr)) => Self::expr_can_fail(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let (cond_fail, cond_errors) = Self::expr_can_fail(condition);
                let (then_fail, mut then_errors) = Self::check_can_fail(then_body);
                let (else_fail, mut else_errors) = else_body
                    .as_ref()
                    .map(|b| Self::check_can_fail(b))
                    .unwrap_or((false, Vec::new()));

                let mut all_errors = cond_errors;
                all_errors.append(&mut then_errors);
                all_errors.append(&mut else_errors);

                (cond_fail || then_fail || else_fail, all_errors)
            }
            HirStmt::While { condition, body } => {
                let (cond_fail, cond_errors) = Self::expr_can_fail(condition);
                let (body_fail, mut body_errors) = Self::check_can_fail(body);

                let mut all_errors = cond_errors;
                all_errors.append(&mut body_errors);

                (cond_fail || body_fail, all_errors)
            }
            HirStmt::For { iter, body, .. } => {
                let (iter_fail, iter_errors) = Self::expr_can_fail(iter);
                let (body_fail, mut body_errors) = Self::check_can_fail(body);

                let mut all_errors = iter_errors;
                all_errors.append(&mut body_errors);

                (iter_fail || body_fail, all_errors)
            }
            _ => (false, Vec::new()),
        }
    }

    fn expr_can_fail(expr: &HirExpr) -> (bool, Vec<String>) {
        match expr {
            HirExpr::Index { .. } => (true, vec!["IndexError".to_string()]),
            HirExpr::Binary {
                op: BinOp::Div | BinOp::FloorDiv | BinOp::Mod,
                ..
            } => (true, vec!["ZeroDivisionError".to_string()]),
            HirExpr::Call { func, args } => {
                // Check if function can fail and combine with argument errors
                let func_errors = match func.as_str() {
                    "int" => vec!["ValueError".to_string()],
                    _ => Vec::new(),
                };

                let (args_fail, mut args_errors) = Self::check_exprs_can_fail(args);
                let mut all_errors = func_errors.clone();
                all_errors.append(&mut args_errors);

                (!func_errors.is_empty() || args_fail, all_errors)
            }
            HirExpr::Binary { left, right, .. } => {
                let (left_fail, left_errors) = Self::expr_can_fail(left);
                let (right_fail, mut right_errors) = Self::expr_can_fail(right);

                let mut all_errors = left_errors;
                all_errors.append(&mut right_errors);

                (left_fail || right_fail, all_errors)
            }
            _ => (false, Vec::new()),
        }
    }

    fn check_exprs_can_fail(exprs: &[HirExpr]) -> (bool, Vec<String>) {
        let mut can_fail = false;
        let mut all_errors = Vec::new();

        for expr in exprs {
            let (expr_fail, mut expr_errors) = Self::expr_can_fail(expr);
            if expr_fail {
                can_fail = true;
                all_errors.append(&mut expr_errors);
            }
        }

        (can_fail, all_errors)
    }

    fn extract_exception_type(exception: &Option<HirExpr>) -> String {
        match exception {
            Some(HirExpr::Call { func, .. }) => func.clone(),
            Some(HirExpr::Var(name)) => name.clone(),
            _ => "Exception".to_string(),
        }
    }

    fn calculate_max_stack_depth(body: &[HirStmt]) -> Option<usize> {
        // Simple estimation for V1
        Some(Self::estimate_stack_depth(body, 0))
    }

    fn estimate_stack_depth(body: &[HirStmt], current: usize) -> usize {
        body.iter().fold(current, |max_depth, stmt| {
            let stmt_depth = match stmt {
                HirStmt::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    let then_depth = Self::estimate_stack_depth(then_body, current + 1);
                    let else_depth = else_body
                        .as_ref()
                        .map(|b| Self::estimate_stack_depth(b, current + 1))
                        .unwrap_or(current);
                    then_depth.max(else_depth)
                }
                HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                    Self::estimate_stack_depth(body, current + 1)
                }
                HirStmt::Raise { .. } => current, // Raise doesn't add stack depth
                _ => current,
            };
            max_depth.max(stmt_depth)
        })
    }
}
