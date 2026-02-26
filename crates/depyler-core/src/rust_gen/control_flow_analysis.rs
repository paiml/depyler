//! Control Flow Analysis Module - EXTREME TDD (PMAT v3.21.0)
//!
//! Pure functions for analyzing control flow in HIR statements.
//! These functions detect:
//! - Guaranteed return paths
//! - Nested function definitions
//! - Block-escaping variables (if/loop scopes)
//! - Variable assignment tracking across blocks

use crate::hir::{AssignTarget, HirExpr, HirStmt};
use std::collections::HashSet;

/// Check if a statement always returns (guaranteed termination path)
/// DEPYLER-0622: Used to determine if code after this statement is unreachable
pub fn stmt_always_returns(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Return(_) => true,
        HirStmt::Raise { .. } => true,
        HirStmt::Try { body, handlers, orelse, finalbody: _ } => {
            // Try always returns if:
            // 1. Body always returns AND
            // 2. All exception handlers always return AND
            // 3. Orelse (if present) always returns
            let body_returns = body.iter().any(stmt_always_returns);
            let handlers_return = !handlers.is_empty()
                && handlers.iter().all(|h| h.body.iter().any(stmt_always_returns));
            let orelse_returns =
                orelse.as_ref().map(|stmts| stmts.iter().any(stmt_always_returns)).unwrap_or(true);

            body_returns && handlers_return && orelse_returns
        }
        HirStmt::With { body, .. } => body.iter().any(stmt_always_returns),
        HirStmt::If { then_body, else_body, .. } => {
            let then_returns = then_body.iter().any(stmt_always_returns);
            let else_returns = else_body
                .as_ref()
                .map(|stmts| stmts.iter().any(stmt_always_returns))
                .unwrap_or(false); // No else = might fall through
            then_returns && else_returns
        }
        // For/While loops don't guarantee return (loop might not execute)
        _ => false,
    }
}

/// Recursively collect all nested function names from a block of statements
/// DEPYLER-0613: Used to hoist function declarations to the top level
pub fn collect_nested_function_names(stmts: &[HirStmt], names: &mut Vec<String>) {
    for stmt in stmts {
        match stmt {
            HirStmt::FunctionDef { name, body, .. } => {
                if !names.contains(name) {
                    names.push(name.clone());
                }
                collect_nested_function_names(body, names);
            }
            HirStmt::If { then_body, else_body, .. } => {
                collect_nested_function_names(then_body, names);
                if let Some(else_stmts) = else_body {
                    collect_nested_function_names(else_stmts, names);
                }
            }
            HirStmt::While { body, .. }
            | HirStmt::For { body, .. }
            | HirStmt::With { body, .. } => {
                collect_nested_function_names(body, names);
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                collect_nested_function_names(body, names);
                for handler in handlers {
                    collect_nested_function_names(&handler.body, names);
                }
                if let Some(stmts) = orelse {
                    collect_nested_function_names(stmts, names);
                }
                if let Some(stmts) = finalbody {
                    collect_nested_function_names(stmts, names);
                }
            }
            _ => {}
        }
    }
}

/// DEPYLER-0834: Collect variables assigned in if blocks that escape to outer scope
pub fn collect_if_escaping_variables(stmts: &[HirStmt]) -> HashSet<String> {
    let mut escaping_vars = HashSet::new();

    for (i, stmt) in stmts.iter().enumerate() {
        let if_assigned_vars = match stmt {
            HirStmt::If { then_body, else_body, .. } => {
                let then_vars = extract_toplevel_assigned_symbols(then_body);
                let else_vars = if let Some(else_stmts) = else_body {
                    extract_toplevel_assigned_symbols(else_stmts)
                } else {
                    HashSet::new()
                };
                let mut all_vars = then_vars;
                all_vars.extend(else_vars);

                let mut nested_escaping = collect_if_escaping_variables(then_body);
                if let Some(else_stmts) = else_body {
                    nested_escaping.extend(collect_if_escaping_variables(else_stmts));
                }
                escaping_vars.extend(nested_escaping);

                all_vars
            }
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
                escaping_vars.extend(collect_if_escaping_variables(body));
                continue;
            }
            HirStmt::Try { body, handlers, finalbody, .. } => {
                let mut vars = collect_if_escaping_variables(body);
                for handler in handlers {
                    vars.extend(collect_if_escaping_variables(&handler.body));
                }
                if let Some(finally) = finalbody {
                    vars.extend(collect_if_escaping_variables(finally));
                }
                escaping_vars.extend(vars);
                continue;
            }
            _ => continue,
        };

        if !if_assigned_vars.is_empty() {
            let remaining_stmts = &stmts[i + 1..];
            for var in if_assigned_vars {
                if is_var_used_in_remaining_stmts(&var, remaining_stmts) {
                    escaping_vars.insert(var);
                }
            }
        }
    }

    escaping_vars
}

/// DEPYLER-0762: Collect variables assigned in loops that escape to outer scope
pub fn collect_loop_escaping_variables(stmts: &[HirStmt]) -> HashSet<String> {
    let mut escaping_vars = HashSet::new();

    for (i, stmt) in stmts.iter().enumerate() {
        let loop_assigned_vars = match stmt {
            HirStmt::For { body, .. } => collect_all_assigned_variables(body),
            HirStmt::While { body, .. } => collect_all_assigned_variables(body),
            HirStmt::If { then_body, else_body, .. } => {
                let mut vars = collect_loop_escaping_variables(then_body);
                if let Some(else_stmts) = else_body {
                    vars.extend(collect_loop_escaping_variables(else_stmts));
                }
                let remaining_stmts = &stmts[i + 1..];
                for var in vars {
                    if is_var_used_in_remaining_stmts(&var, remaining_stmts) {
                        escaping_vars.insert(var);
                    }
                }
                continue;
            }
            HirStmt::Try { body, handlers, finalbody, .. } => {
                let mut vars = collect_loop_escaping_variables(body);
                for handler in handlers {
                    vars.extend(collect_loop_escaping_variables(&handler.body));
                }
                if let Some(finally) = finalbody {
                    vars.extend(collect_loop_escaping_variables(finally));
                }
                let remaining_stmts = &stmts[i + 1..];
                for var in vars {
                    if is_var_used_in_remaining_stmts(&var, remaining_stmts) {
                        escaping_vars.insert(var);
                    }
                }
                continue;
            }
            _ => continue,
        };

        if !loop_assigned_vars.is_empty() {
            let remaining_stmts = &stmts[i + 1..];
            for var in loop_assigned_vars {
                if is_var_used_in_remaining_stmts(&var, remaining_stmts) {
                    escaping_vars.insert(var);
                }
            }
        }
    }

    escaping_vars
}

/// Collect ALL variables assigned inside a statement list (recursively)
pub fn collect_all_assigned_variables(stmts: &[HirStmt]) -> HashSet<String> {
    let mut vars = HashSet::new();

    for stmt in stmts {
        match stmt {
            HirStmt::Assign { target: AssignTarget::Symbol(name), .. } => {
                vars.insert(name.clone());
            }
            HirStmt::Assign { target: AssignTarget::Tuple(targets), .. } => {
                for t in targets {
                    if let AssignTarget::Symbol(name) = t {
                        vars.insert(name.clone());
                    }
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                vars.extend(collect_all_assigned_variables(then_body));
                if let Some(else_stmts) = else_body {
                    vars.extend(collect_all_assigned_variables(else_stmts));
                }
            }
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
                vars.extend(collect_all_assigned_variables(body));
            }
            HirStmt::Try { body, handlers, finalbody, .. } => {
                vars.extend(collect_all_assigned_variables(body));
                for handler in handlers {
                    vars.extend(collect_all_assigned_variables(&handler.body));
                }
                if let Some(finally) = finalbody {
                    vars.extend(collect_all_assigned_variables(finally));
                }
            }
            _ => {}
        }
    }

    vars
}

/// Extract variables assigned at the TOP LEVEL only (not in nested loops)
/// DEPYLER-0476: Used for if-block variable hoisting
pub fn extract_toplevel_assigned_symbols(stmts: &[HirStmt]) -> HashSet<String> {
    let mut vars = HashSet::new();

    for stmt in stmts {
        match stmt {
            HirStmt::Assign { target: AssignTarget::Symbol(name), .. } => {
                vars.insert(name.clone());
            }
            HirStmt::Assign { target: AssignTarget::Tuple(targets), .. } => {
                for t in targets {
                    if let AssignTarget::Symbol(name) = t {
                        vars.insert(name.clone());
                    }
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                vars.extend(extract_toplevel_assigned_symbols(then_body));
                if let Some(else_stmts) = else_body {
                    vars.extend(extract_toplevel_assigned_symbols(else_stmts));
                }
            }
            HirStmt::Try { body, handlers, finalbody, .. } => {
                vars.extend(extract_toplevel_assigned_symbols(body));
                for handler in handlers {
                    vars.extend(extract_toplevel_assigned_symbols(&handler.body));
                }
                if let Some(finally) = finalbody {
                    vars.extend(extract_toplevel_assigned_symbols(finally));
                }
            }
            // DO NOT recurse into for/while loops
            HirStmt::While { .. } | HirStmt::For { .. } => {}
            _ => {}
        }
    }

    vars
}

/// Check if a variable is used in any of the remaining statements
pub fn is_var_used_in_remaining_stmts(var_name: &str, stmts: &[HirStmt]) -> bool {
    stmts.iter().any(|stmt| is_var_used_anywhere(var_name, stmt))
}

/// Check if a variable is used anywhere in a statement (recursive)
pub fn is_var_used_anywhere(var_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            is_var_used_in_target(var_name, target) || is_var_used_in_expr(var_name, value)
        }
        HirStmt::If { condition, then_body, else_body } => {
            is_var_used_in_expr(var_name, condition)
                || then_body.iter().any(|s| is_var_used_anywhere(var_name, s))
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(|s| is_var_used_anywhere(var_name, s)))
        }
        HirStmt::While { condition, body } => {
            is_var_used_in_expr(var_name, condition)
                || body.iter().any(|s| is_var_used_anywhere(var_name, s))
        }
        HirStmt::For { iter, body, .. } => {
            is_var_used_in_expr(var_name, iter)
                || body.iter().any(|s| is_var_used_anywhere(var_name, s))
        }
        HirStmt::Return(Some(expr)) => is_var_used_in_expr(var_name, expr),
        HirStmt::Expr(expr) => is_var_used_in_expr(var_name, expr),
        HirStmt::Raise { exception, .. } => {
            exception.as_ref().is_some_and(|e| is_var_used_in_expr(var_name, e))
        }
        HirStmt::Assert { test, msg, .. } => {
            is_var_used_in_expr(var_name, test)
                || msg.as_ref().is_some_and(|m| is_var_used_in_expr(var_name, m))
        }
        HirStmt::Try { body, handlers, orelse, finalbody } => {
            body.iter().any(|s| is_var_used_anywhere(var_name, s))
                || handlers.iter().any(|h| h.body.iter().any(|s| is_var_used_anywhere(var_name, s)))
                || orelse
                    .as_ref()
                    .is_some_and(|stmts| stmts.iter().any(|s| is_var_used_anywhere(var_name, s)))
                || finalbody
                    .as_ref()
                    .is_some_and(|stmts| stmts.iter().any(|s| is_var_used_anywhere(var_name, s)))
        }
        HirStmt::With { body, .. } => body.iter().any(|s| is_var_used_anywhere(var_name, s)),
        _ => false,
    }
}

/// Check if a variable is used in an assignment target
fn is_var_used_in_target(var_name: &str, target: &AssignTarget) -> bool {
    match target {
        AssignTarget::Symbol(s) => s == var_name,
        AssignTarget::Index { base, index } => {
            is_var_used_in_expr(var_name, base) || is_var_used_in_expr(var_name, index)
        }
        AssignTarget::Attribute { value, .. } => is_var_used_in_expr(var_name, value),
        AssignTarget::Tuple(targets) => targets.iter().any(|t| is_var_used_in_target(var_name, t)),
    }
}

/// Check if a variable is used in an expression
fn is_var_used_in_expr(var_name: &str, expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Var(name) => name == var_name,
        HirExpr::Binary { left, right, .. } => {
            is_var_used_in_expr(var_name, left) || is_var_used_in_expr(var_name, right)
        }
        HirExpr::Unary { operand, .. } => is_var_used_in_expr(var_name, operand),
        HirExpr::Call { args, .. } => args.iter().any(|a| is_var_used_in_expr(var_name, a)),
        HirExpr::MethodCall { object, args, .. } => {
            is_var_used_in_expr(var_name, object)
                || args.iter().any(|a| is_var_used_in_expr(var_name, a))
        }
        HirExpr::Index { base, index } => {
            is_var_used_in_expr(var_name, base) || is_var_used_in_expr(var_name, index)
        }
        HirExpr::Attribute { value, .. } => is_var_used_in_expr(var_name, value),
        HirExpr::List(elements) | HirExpr::Tuple(elements) | HirExpr::Set(elements) => {
            elements.iter().any(|e| is_var_used_in_expr(var_name, e))
        }
        HirExpr::Dict(pairs) => pairs
            .iter()
            .any(|(k, v)| is_var_used_in_expr(var_name, k) || is_var_used_in_expr(var_name, v)),
        HirExpr::IfExpr { test, body, orelse } => {
            is_var_used_in_expr(var_name, test)
                || is_var_used_in_expr(var_name, body)
                || is_var_used_in_expr(var_name, orelse)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, ExceptHandler, Literal};

    fn lit_int(n: i64) -> HirExpr {
        HirExpr::Literal(Literal::Int(n))
    }

    fn lit_bool(b: bool) -> HirExpr {
        HirExpr::Literal(Literal::Bool(b))
    }

    // ============================================================================
    // stmt_always_returns tests
    // ============================================================================

    #[test]
    fn test_return_always_returns() {
        let stmt = HirStmt::Return(Some(lit_int(42)));
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_return_none_always_returns() {
        let stmt = HirStmt::Return(None);
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_raise_always_returns() {
        let stmt =
            HirStmt::Raise { exception: Some(HirExpr::Var("ValueError".to_string())), cause: None };
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_assign_does_not_always_return() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: lit_int(1),
            type_annotation: None,
        };
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_expr_does_not_always_return() {
        let stmt = HirStmt::Expr(lit_int(42));
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_if_both_branches_return() {
        let stmt = HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Return(Some(lit_int(1)))],
            else_body: Some(vec![HirStmt::Return(Some(lit_int(2)))]),
        };
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_if_only_then_returns() {
        let stmt = HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Return(Some(lit_int(1)))],
            else_body: Some(vec![HirStmt::Expr(lit_int(2))]),
        };
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_if_no_else_does_not_always_return() {
        let stmt = HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Return(Some(lit_int(1)))],
            else_body: None,
        };
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_with_body_returns() {
        let stmt = HirStmt::With {
            context: HirExpr::Var("file".to_string()),
            target: Some("f".to_string()),
            body: vec![HirStmt::Return(Some(lit_int(1)))],
            is_async: false,
        };
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_with_body_does_not_return() {
        let stmt = HirStmt::With {
            context: HirExpr::Var("file".to_string()),
            target: Some("f".to_string()),
            body: vec![HirStmt::Expr(lit_int(1))],
            is_async: false,
        };
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_for_loop_does_not_always_return() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Return(Some(lit_int(1)))],
        };
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_while_does_not_always_return() {
        let stmt = HirStmt::While {
            condition: lit_bool(true),
            body: vec![HirStmt::Return(Some(lit_int(1)))],
        };
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_try_all_branches_return() {
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Return(Some(lit_int(1)))],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Return(Some(lit_int(2)))],
            }],
            orelse: Some(vec![HirStmt::Return(Some(lit_int(3)))]),
            finalbody: None,
        };
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_try_no_handlers() {
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Return(Some(lit_int(1)))],
            handlers: vec![],
            orelse: None,
            finalbody: None,
        };
        assert!(!stmt_always_returns(&stmt));
    }

    // ============================================================================
    // collect_nested_function_names tests
    // ============================================================================

    #[test]
    fn test_collect_nested_function_simple() {
        let stmts = vec![HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec::smallvec![]),
            ret_type: crate::hir::Type::None,
            body: vec![],
            docstring: None,
        }];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert_eq!(names, vec!["inner"]);
    }

    #[test]
    fn test_collect_nested_function_in_if() {
        let stmts = vec![HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::FunctionDef {
                name: "then_func".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            }],
            else_body: Some(vec![HirStmt::FunctionDef {
                name: "else_func".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            }]),
        }];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert!(names.contains(&"then_func".to_string()));
        assert!(names.contains(&"else_func".to_string()));
    }

    #[test]
    fn test_collect_nested_function_in_loop() {
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::FunctionDef {
                name: "loop_func".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            }],
        }];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert_eq!(names, vec!["loop_func"]);
    }

    #[test]
    fn test_collect_nested_no_duplicates() {
        let stmts = vec![
            HirStmt::FunctionDef {
                name: "dup".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            },
            HirStmt::FunctionDef {
                name: "dup".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            },
        ];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert_eq!(names.len(), 1);
    }

    // ============================================================================
    // collect_all_assigned_variables tests
    // ============================================================================

    #[test]
    fn test_collect_simple_assign() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: lit_int(1),
            type_annotation: None,
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("x"));
        assert_eq!(vars.len(), 1);
    }

    #[test]
    fn test_collect_tuple_assign() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Symbol("b".to_string()),
            ]),
            value: HirExpr::Tuple(vec![lit_int(1), lit_int(2)]),
            type_annotation: None,
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("a"));
        assert!(vars.contains("b"));
    }

    #[test]
    fn test_collect_from_if_branches() {
        let stmts = vec![HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: lit_int(2),
                type_annotation: None,
            }]),
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
    }

    #[test]
    fn test_collect_from_loop_body() {
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("sum".to_string()),
                value: lit_int(0),
                type_annotation: None,
            }],
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("sum"));
    }

    #[test]
    fn test_collect_from_try_body() {
        let stmts = vec![HirStmt::Try {
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("result".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("error".to_string()),
                    value: lit_int(0),
                    type_annotation: None,
                }],
            }],
            orelse: None,
            finalbody: None,
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("result"));
        assert!(vars.contains("error"));
    }

    // ============================================================================
    // extract_toplevel_assigned_symbols tests
    // ============================================================================

    #[test]
    fn test_toplevel_simple() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: lit_int(1),
            type_annotation: None,
        }];
        let vars = extract_toplevel_assigned_symbols(&stmts);
        assert!(vars.contains("x"));
    }

    #[test]
    fn test_toplevel_includes_if() {
        let stmts = vec![HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            else_body: None,
        }];
        let vars = extract_toplevel_assigned_symbols(&stmts);
        assert!(vars.contains("x"));
    }

    #[test]
    fn test_toplevel_excludes_for() {
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("loop_var".to_string()),
                value: lit_int(0),
                type_annotation: None,
            }],
        }];
        let vars = extract_toplevel_assigned_symbols(&stmts);
        assert!(!vars.contains("loop_var"));
    }

    #[test]
    fn test_toplevel_excludes_while() {
        let stmts = vec![HirStmt::While {
            condition: lit_bool(true),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("loop_var".to_string()),
                value: lit_int(0),
                type_annotation: None,
            }],
        }];
        let vars = extract_toplevel_assigned_symbols(&stmts);
        assert!(!vars.contains("loop_var"));
    }

    // ============================================================================
    // is_var_used_anywhere tests
    // ============================================================================

    #[test]
    fn test_var_used_in_return() {
        let stmt = HirStmt::Return(Some(HirExpr::Var("x".to_string())));
        assert!(is_var_used_anywhere("x", &stmt));
        assert!(!is_var_used_anywhere("y", &stmt));
    }

    #[test]
    fn test_var_used_in_assign_value() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("y".to_string()),
            value: HirExpr::Var("x".to_string()),
            type_annotation: None,
        };
        assert!(is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_used_in_condition() {
        let stmt = HirStmt::If {
            condition: HirExpr::Var("cond".to_string()),
            then_body: vec![],
            else_body: None,
        };
        assert!(is_var_used_anywhere("cond", &stmt));
    }

    #[test]
    fn test_var_used_in_nested_if() {
        let stmt = HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
            else_body: None,
        };
        assert!(is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_used_in_loop_iter() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![],
        };
        assert!(is_var_used_anywhere("items", &stmt));
    }

    // ============================================================================
    // collect_if_escaping_variables tests
    // ============================================================================

    #[test]
    fn test_if_escaping_var_used_after() {
        let stmts = vec![
            HirStmt::If {
                condition: lit_bool(true),
                then_body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: lit_int(1),
                    type_annotation: None,
                }],
                else_body: None,
            },
            HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
        ];
        let escaping = collect_if_escaping_variables(&stmts);
        assert!(escaping.contains("x"));
    }

    #[test]
    fn test_if_escaping_var_not_used_after() {
        let stmts = vec![
            HirStmt::If {
                condition: lit_bool(true),
                then_body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: lit_int(1),
                    type_annotation: None,
                }],
                else_body: None,
            },
            HirStmt::Return(Some(lit_int(42))),
        ];
        let escaping = collect_if_escaping_variables(&stmts);
        assert!(!escaping.contains("x"));
    }

    // ============================================================================
    // collect_loop_escaping_variables tests
    // ============================================================================

    #[test]
    fn test_loop_escaping_var_used_after() {
        let stmts = vec![
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::List(vec![]),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("last".to_string()),
                    value: HirExpr::Var("i".to_string()),
                    type_annotation: None,
                }],
            },
            HirStmt::Return(Some(HirExpr::Var("last".to_string()))),
        ];
        let escaping = collect_loop_escaping_variables(&stmts);
        assert!(escaping.contains("last"));
    }

    #[test]
    fn test_loop_escaping_var_not_used_after() {
        let stmts = vec![
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::List(vec![]),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("temp".to_string()),
                    value: lit_int(1),
                    type_annotation: None,
                }],
            },
            HirStmt::Return(Some(lit_int(0))),
        ];
        let escaping = collect_loop_escaping_variables(&stmts);
        assert!(!escaping.contains("temp"));
    }

    // ============================================================================
    // is_var_used_in_expr tests
    // ============================================================================

    #[test]
    fn test_var_in_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(lit_int(1)),
        };
        assert!(is_var_used_in_expr("x", &expr));
        assert!(!is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_in_method_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "do".to_string(),
            args: vec![HirExpr::Var("arg".to_string())],
            kwargs: vec![],
        };
        assert!(is_var_used_in_expr("obj", &expr));
        assert!(is_var_used_in_expr("arg", &expr));
    }

    #[test]
    fn test_var_in_if_expr() {
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(HirExpr::Var("a".to_string())),
            orelse: Box::new(HirExpr::Var("b".to_string())),
        };
        assert!(is_var_used_in_expr("cond", &expr));
        assert!(is_var_used_in_expr("a", &expr));
        assert!(is_var_used_in_expr("b", &expr));
    }

    #[test]
    fn test_var_in_unary() {
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(is_var_used_in_expr("x", &expr));
        assert!(!is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_in_call_args() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Var("msg".to_string())],
            kwargs: vec![],
        };
        assert!(is_var_used_in_expr("msg", &expr));
        assert!(!is_var_used_in_expr("other", &expr));
    }

    #[test]
    fn test_var_in_index_expr() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("idx".to_string())),
        };
        assert!(is_var_used_in_expr("arr", &expr));
        assert!(is_var_used_in_expr("idx", &expr));
    }

    #[test]
    fn test_var_in_attribute_expr() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        assert!(is_var_used_in_expr("obj", &expr));
        assert!(!is_var_used_in_expr("field", &expr));
    }

    #[test]
    fn test_var_in_list_expr() {
        let expr = HirExpr::List(vec![HirExpr::Var("a".to_string()), lit_int(1)]);
        assert!(is_var_used_in_expr("a", &expr));
        assert!(!is_var_used_in_expr("b", &expr));
    }

    #[test]
    fn test_var_in_tuple_expr() {
        let expr =
            HirExpr::Tuple(vec![HirExpr::Var("x".to_string()), HirExpr::Var("y".to_string())]);
        assert!(is_var_used_in_expr("x", &expr));
        assert!(is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_in_set_expr() {
        let expr = HirExpr::Set(vec![HirExpr::Var("item".to_string())]);
        assert!(is_var_used_in_expr("item", &expr));
    }

    #[test]
    fn test_var_in_dict_expr() {
        let expr =
            HirExpr::Dict(vec![(HirExpr::Var("key".to_string()), HirExpr::Var("val".to_string()))]);
        assert!(is_var_used_in_expr("key", &expr));
        assert!(is_var_used_in_expr("val", &expr));
        assert!(!is_var_used_in_expr("other", &expr));
    }

    #[test]
    fn test_var_not_in_literal() {
        assert!(!is_var_used_in_expr("x", &lit_int(42)));
        assert!(!is_var_used_in_expr("x", &HirExpr::Literal(Literal::String("hello".to_string()))));
    }

    #[test]
    fn test_var_used_in_while_condition() {
        let stmt = HirStmt::While { condition: HirExpr::Var("flag".to_string()), body: vec![] };
        assert!(is_var_used_anywhere("flag", &stmt));
    }

    #[test]
    fn test_var_used_in_while_body() {
        let stmt = HirStmt::While {
            condition: lit_bool(true),
            body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
        };
        assert!(is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_used_in_for_body() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Expr(HirExpr::Var("total".to_string()))],
        };
        assert!(is_var_used_anywhere("total", &stmt));
    }

    #[test]
    fn test_var_used_in_assert_test() {
        let stmt = HirStmt::Assert { test: HirExpr::Var("check".to_string()), msg: None };
        assert!(is_var_used_anywhere("check", &stmt));
        assert!(!is_var_used_anywhere("other", &stmt));
    }

    #[test]
    fn test_var_used_in_assert_msg() {
        let stmt =
            HirStmt::Assert { test: lit_bool(true), msg: Some(HirExpr::Var("msg".to_string())) };
        assert!(is_var_used_anywhere("msg", &stmt));
    }

    #[test]
    fn test_var_used_in_raise() {
        let stmt = HirStmt::Raise { exception: Some(HirExpr::Var("err".to_string())), cause: None };
        assert!(is_var_used_anywhere("err", &stmt));
    }

    #[test]
    fn test_var_used_in_raise_none() {
        let stmt = HirStmt::Raise { exception: None, cause: None };
        assert!(!is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_used_in_try_body() {
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
            handlers: vec![],
            orelse: None,
            finalbody: None,
        };
        assert!(is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_used_in_try_handler() {
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
            }],
            orelse: None,
            finalbody: None,
        };
        assert!(is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_used_in_try_orelse() {
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: Some(vec![HirStmt::Expr(HirExpr::Var("y".to_string()))]),
            finalbody: None,
        };
        assert!(is_var_used_anywhere("y", &stmt));
    }

    #[test]
    fn test_var_used_in_try_finalbody() {
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Expr(HirExpr::Var("z".to_string()))]),
        };
        assert!(is_var_used_anywhere("z", &stmt));
    }

    #[test]
    fn test_var_used_in_with_body() {
        let stmt = HirStmt::With {
            context: HirExpr::Var("ctx".to_string()),
            target: Some("f".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Var("data".to_string()))],
            is_async: false,
        };
        assert!(is_var_used_anywhere("data", &stmt));
        // Note: is_var_used_anywhere for With only checks body, not context
        assert!(!is_var_used_anywhere("not_there", &stmt));
    }

    #[test]
    fn test_var_used_in_if_else() {
        let stmt = HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![],
            else_body: Some(vec![HirStmt::Expr(HirExpr::Var("x".to_string()))]),
        };
        assert!(is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_not_used_in_return_none() {
        let stmt = HirStmt::Return(None);
        assert!(!is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_not_used_in_break() {
        let stmt = HirStmt::Break { label: None };
        assert!(!is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_not_used_in_continue() {
        let stmt = HirStmt::Continue { label: None };
        assert!(!is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_not_used_in_pass() {
        let stmt = HirStmt::Pass;
        assert!(!is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_var_used_in_index_target() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Var("idx".to_string())),
            },
            value: lit_int(42),
            type_annotation: None,
        };
        assert!(is_var_used_anywhere("arr", &stmt));
        assert!(is_var_used_anywhere("idx", &stmt));
    }

    #[test]
    fn test_var_used_in_attribute_target() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Attribute {
                value: Box::new(HirExpr::Var("self_obj".to_string())),
                attr: "field".to_string(),
            },
            value: lit_int(1),
            type_annotation: None,
        };
        assert!(is_var_used_anywhere("self_obj", &stmt));
    }

    #[test]
    fn test_var_used_in_tuple_target() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Symbol("b".to_string()),
            ]),
            value: HirExpr::Tuple(vec![lit_int(1), lit_int(2)]),
            type_annotation: None,
        };
        assert!(is_var_used_anywhere("a", &stmt));
        assert!(is_var_used_anywhere("b", &stmt));
    }

    #[test]
    fn test_collect_nested_in_while() {
        let stmts = vec![HirStmt::While {
            condition: lit_bool(true),
            body: vec![HirStmt::FunctionDef {
                name: "while_func".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            }],
        }];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert_eq!(names, vec!["while_func"]);
    }

    #[test]
    fn test_collect_nested_in_with() {
        let stmts = vec![HirStmt::With {
            context: HirExpr::Var("ctx".to_string()),
            target: Some("f".to_string()),
            body: vec![HirStmt::FunctionDef {
                name: "with_func".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            }],
            is_async: false,
        }];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert_eq!(names, vec!["with_func"]);
    }

    #[test]
    fn test_collect_nested_in_try_all_branches() {
        let stmts = vec![HirStmt::Try {
            body: vec![HirStmt::FunctionDef {
                name: "try_func".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            }],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::FunctionDef {
                    name: "handler_func".to_string(),
                    params: Box::new(smallvec::smallvec![]),
                    ret_type: crate::hir::Type::None,
                    body: vec![],
                    docstring: None,
                }],
            }],
            orelse: Some(vec![HirStmt::FunctionDef {
                name: "orelse_func".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            }]),
            finalbody: Some(vec![HirStmt::FunctionDef {
                name: "finally_func".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            }]),
        }];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert!(names.contains(&"try_func".to_string()));
        assert!(names.contains(&"handler_func".to_string()));
        assert!(names.contains(&"orelse_func".to_string()));
        assert!(names.contains(&"finally_func".to_string()));
    }

    #[test]
    fn test_collect_nested_deeply_nested() {
        let stmts = vec![HirStmt::FunctionDef {
            name: "outer".to_string(),
            params: Box::new(smallvec::smallvec![]),
            ret_type: crate::hir::Type::None,
            body: vec![HirStmt::FunctionDef {
                name: "inner".to_string(),
                params: Box::new(smallvec::smallvec![]),
                ret_type: crate::hir::Type::None,
                body: vec![],
                docstring: None,
            }],
            docstring: None,
        }];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert!(names.contains(&"outer".to_string()));
        assert!(names.contains(&"inner".to_string()));
    }

    #[test]
    fn test_if_escaping_from_nested_for() {
        // Variable assigned in if inside for, used after the if within the for body
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![
                HirStmt::If {
                    condition: lit_bool(true),
                    then_body: vec![HirStmt::Assign {
                        target: AssignTarget::Symbol("found".to_string()),
                        value: lit_bool(true),
                        type_annotation: None,
                    }],
                    else_body: None,
                },
                HirStmt::Expr(HirExpr::Var("found".to_string())),
            ],
        }];
        let escaping = collect_if_escaping_variables(&stmts);
        assert!(escaping.contains("found"));
    }

    #[test]
    fn test_if_escaping_from_try() {
        // Variable assigned in if inside try, used after the if within the try body
        let stmts = vec![HirStmt::Try {
            body: vec![
                HirStmt::If {
                    condition: lit_bool(true),
                    then_body: vec![HirStmt::Assign {
                        target: AssignTarget::Symbol("result".to_string()),
                        value: lit_int(1),
                        type_annotation: None,
                    }],
                    else_body: None,
                },
                HirStmt::Expr(HirExpr::Var("result".to_string())),
            ],
            handlers: vec![],
            orelse: None,
            finalbody: None,
        }];
        let escaping = collect_if_escaping_variables(&stmts);
        assert!(escaping.contains("result"));
    }

    #[test]
    fn test_loop_escaping_from_while() {
        let stmts = vec![
            HirStmt::While {
                condition: lit_bool(true),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("count".to_string()),
                    value: lit_int(0),
                    type_annotation: None,
                }],
            },
            HirStmt::Return(Some(HirExpr::Var("count".to_string()))),
        ];
        let escaping = collect_loop_escaping_variables(&stmts);
        assert!(escaping.contains("count"));
    }

    #[test]
    fn test_loop_escaping_nested_in_if() {
        // For loop inside if body, var assigned in loop used after the If
        let stmts = vec![
            HirStmt::If {
                condition: lit_bool(true),
                then_body: vec![
                    HirStmt::For {
                        target: AssignTarget::Symbol("i".to_string()),
                        iter: HirExpr::List(vec![]),
                        body: vec![HirStmt::Assign {
                            target: AssignTarget::Symbol("acc".to_string()),
                            value: lit_int(0),
                            type_annotation: None,
                        }],
                    },
                    HirStmt::Expr(HirExpr::Var("acc".to_string())),
                ],
                else_body: None,
            },
            HirStmt::Return(Some(HirExpr::Var("acc".to_string()))),
        ];
        let escaping = collect_loop_escaping_variables(&stmts);
        assert!(escaping.contains("acc"));
    }

    #[test]
    fn test_loop_escaping_nested_in_try() {
        // For loop inside try, var assigned in loop used after the Try
        let stmts = vec![
            HirStmt::Try {
                body: vec![
                    HirStmt::For {
                        target: AssignTarget::Symbol("i".to_string()),
                        iter: HirExpr::List(vec![]),
                        body: vec![HirStmt::Assign {
                            target: AssignTarget::Symbol("val".to_string()),
                            value: lit_int(1),
                            type_annotation: None,
                        }],
                    },
                    HirStmt::Expr(HirExpr::Var("val".to_string())),
                ],
                handlers: vec![],
                orelse: None,
                finalbody: None,
            },
            HirStmt::Return(Some(HirExpr::Var("val".to_string()))),
        ];
        let escaping = collect_loop_escaping_variables(&stmts);
        assert!(escaping.contains("val"));
    }

    #[test]
    fn test_collect_from_while_body() {
        let stmts = vec![HirStmt::While {
            condition: lit_bool(true),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("counter".to_string()),
                value: lit_int(0),
                type_annotation: None,
            }],
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("counter"));
    }

    #[test]
    fn test_collect_from_try_finalbody() {
        let stmts = vec![HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("cleaned".to_string()),
                value: lit_bool(true),
                type_annotation: None,
            }]),
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("cleaned"));
    }

    #[test]
    fn test_collect_non_assign_stmts() {
        let stmts = vec![
            HirStmt::Pass,
            HirStmt::Break { label: None },
            HirStmt::Continue { label: None },
            HirStmt::Return(None),
        ];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.is_empty());
    }

    #[test]
    fn test_toplevel_tuple_assign() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Symbol("b".to_string()),
            ]),
            value: HirExpr::Tuple(vec![lit_int(1), lit_int(2)]),
            type_annotation: None,
        }];
        let vars = extract_toplevel_assigned_symbols(&stmts);
        assert!(vars.contains("a"));
        assert!(vars.contains("b"));
    }

    #[test]
    fn test_toplevel_from_try() {
        let stmts = vec![HirStmt::Try {
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("y".to_string()),
                    value: lit_int(2),
                    type_annotation: None,
                }],
            }],
            orelse: None,
            finalbody: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("z".to_string()),
                value: lit_int(3),
                type_annotation: None,
            }]),
        }];
        let vars = extract_toplevel_assigned_symbols(&stmts);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
        assert!(vars.contains("z"));
    }

    #[test]
    fn test_var_used_in_remaining_multiple() {
        let stmts = vec![HirStmt::Expr(lit_int(1)), HirStmt::Expr(HirExpr::Var("x".to_string()))];
        assert!(is_var_used_in_remaining_stmts("x", &stmts));
    }

    #[test]
    fn test_var_not_used_in_remaining_empty() {
        let stmts: Vec<HirStmt> = vec![];
        assert!(!is_var_used_in_remaining_stmts("x", &stmts));
    }

    #[test]
    fn test_try_no_orelse_still_returns() {
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Return(Some(lit_int(1)))],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Return(Some(lit_int(2)))],
            }],
            orelse: None,
            finalbody: None,
        };
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_try_handler_no_return() {
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Return(Some(lit_int(1)))],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Expr(lit_int(0))],
            }],
            orelse: None,
            finalbody: None,
        };
        assert!(!stmt_always_returns(&stmt));
    }
}
