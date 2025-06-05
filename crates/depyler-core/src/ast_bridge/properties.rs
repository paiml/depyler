use crate::hir::{BinOp, FunctionProperties, HirExpr, HirStmt};

pub struct FunctionAnalyzer;

impl FunctionAnalyzer {
    pub fn analyze(body: &[HirStmt]) -> FunctionProperties {
        FunctionProperties {
            is_pure: Self::check_pure(body),
            always_terminates: Self::check_termination(body),
            panic_free: Self::check_panic_free(body),
            max_stack_depth: Self::calculate_max_stack_depth(body),
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
                _ => current,
            };
            max_depth.max(stmt_depth)
        })
    }
}