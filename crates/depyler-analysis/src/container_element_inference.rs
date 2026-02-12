//! Container element type inference from usage patterns
//!
//! When Python code uses bare `list`/`dict`/`set` without type parameters
//! (e.g., `def f(numbers: list)`), the transpiler maps the inner type to
//! `Type::Unknown`, producing `Vec<DepylerValue>`. This module infers
//! concrete element types from usage patterns so `list` → `Vec<i32>`.

use crate::param_type_inference::infer_param_type_from_body;
use depyler_hir::hir::{HirExpr, HirStmt, Literal, Type};
use std::collections::HashMap;

/// Check whether a type contains an `Unknown` inner element that could be refined.
///
/// Returns true for `List(Unknown)`, `Dict(_, Unknown)`, `Set(Unknown)`,
/// `Optional(List(Unknown))`, etc.
pub fn has_unknown_inner_type(ty: &Type) -> bool {
    match ty {
        Type::List(elem) | Type::Set(elem) => matches!(elem.as_ref(), Type::Unknown),
        Type::Dict(_, val) => matches!(val.as_ref(), Type::Unknown),
        Type::Optional(inner) => has_unknown_inner_type(inner),
        _ => false,
    }
}

/// Infer the concrete container type by analyzing usage of `container_name` in `body`.
///
/// Returns a refined type (e.g., `List(Int)` instead of `List(Unknown)`) or `None`
/// if no usage pattern provides a clear signal.
pub fn infer_container_element_type(
    container_name: &str,
    container_type: &Type,
    body: &[HirStmt],
) -> Option<Type> {
    match container_type {
        Type::List(elem) if matches!(elem.as_ref(), Type::Unknown) => {
            infer_list_element_type(container_name, body)
                .map(|elem_ty| Type::List(Box::new(elem_ty)))
        }
        Type::Set(elem) if matches!(elem.as_ref(), Type::Unknown) => {
            infer_list_element_type(container_name, body)
                .map(|elem_ty| Type::Set(Box::new(elem_ty)))
        }
        Type::Dict(key, val) if matches!(val.as_ref(), Type::Unknown) => {
            let key_ty = if matches!(key.as_ref(), Type::Unknown) {
                Type::String
            } else {
                key.as_ref().clone()
            };
            infer_dict_value_type(container_name, body)
                .map(|val_ty| Type::Dict(Box::new(key_ty), Box::new(val_ty)))
        }
        Type::Optional(inner) => {
            infer_container_element_type(container_name, inner, body)
                .map(|refined| Type::Optional(Box::new(refined)))
        }
        _ => None,
    }
}

/// Infer the element type of a list/set container from usage patterns.
fn infer_list_element_type(container_name: &str, body: &[HirStmt]) -> Option<Type> {
    // Try each strategy; first match wins
    infer_element_from_for_loop(container_name, body)
        .or_else(|| infer_element_from_append(container_name, body))
        .or_else(|| infer_element_from_builtin(container_name, body))
}

/// Find for-loops iterating over the container and infer element type from loop body usage.
///
/// Pattern: `for x in container: x > 0` → element is Int
fn infer_element_from_for_loop(container_name: &str, body: &[HirStmt]) -> Option<Type> {
    for stmt in body {
        if let Some(ty) = try_infer_from_for_stmt(container_name, stmt) {
            return Some(ty);
        }
        if let Some(ty) = recurse_into_stmt(stmt, |b| infer_element_from_for_loop(container_name, b)) {
            return Some(ty);
        }
    }
    None
}

/// Try to extract element type from a single for-loop statement.
fn try_infer_from_for_stmt(container_name: &str, stmt: &HirStmt) -> Option<Type> {
    let HirStmt::For { target, iter, body } = stmt else {
        return None;
    };
    if !iter_references_container(iter, container_name) {
        return None;
    }
    let var_name = extract_loop_var_name(target)?;
    infer_param_type_from_body(var_name, body)
        .or_else(|| infer_type_from_comparisons(var_name, body))
}

/// Check if a for-loop iterator references the given container.
fn iter_references_container(iter: &HirExpr, container_name: &str) -> bool {
    match iter {
        HirExpr::Var(name) => name == container_name,
        HirExpr::Call { func, args, .. } if func == "enumerate" => {
            args.first().is_some_and(|a| matches!(a, HirExpr::Var(n) if n == container_name))
        }
        _ => false,
    }
}

/// Extract the element variable name from a for-loop target.
fn extract_loop_var_name(target: &depyler_hir::hir::AssignTarget) -> Option<&str> {
    use depyler_hir::hir::AssignTarget;
    match target {
        AssignTarget::Symbol(name) => Some(name.as_str()),
        AssignTarget::Tuple(targets) if targets.len() == 2 => {
            // enumerate: for i, x in enumerate(lst) → second element
            if let AssignTarget::Symbol(name) = &targets[1] {
                Some(name.as_str())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Find `.append()` calls on the container and infer element type from the argument.
///
/// Pattern: `lst.append(42)` → element is Int
fn infer_element_from_append(container_name: &str, body: &[HirStmt]) -> Option<Type> {
    for stmt in body {
        if let Some(ty) = check_append_stmt(container_name, stmt) {
            return Some(ty);
        }
        if let Some(ty) = recurse_into_stmt(stmt, |b| infer_element_from_append(container_name, b)) {
            return Some(ty);
        }
    }
    None
}

/// Check if a statement is an append/add call on `container_name` and infer arg type.
fn check_append_stmt(container_name: &str, stmt: &HirStmt) -> Option<Type> {
    let HirStmt::Expr(HirExpr::MethodCall { object, method, args, .. }) = stmt else {
        return None;
    };
    if method != "append" && method != "add" {
        return None;
    }
    let HirExpr::Var(name) = object.as_ref() else {
        return None;
    };
    if name != container_name || args.is_empty() {
        return None;
    }
    infer_type_from_literal_or_expr(&args[0])
}

/// Find builtin calls that imply element type.
///
/// Pattern: `sum(lst)` → `List(Int)`, `",".join(lst)` → `List(String)`
fn infer_element_from_builtin(container_name: &str, body: &[HirStmt]) -> Option<Type> {
    for stmt in body {
        let expr = match stmt {
            HirStmt::Expr(e) => Some(e),
            HirStmt::Assign { value, .. } => Some(value),
            HirStmt::Return(Some(e)) => Some(e),
            _ => None,
        };

        if let Some(expr) = expr {
            if let Some(ty) = check_builtin_expr(container_name, expr) {
                return Some(ty);
            }
        }

        // Recurse into nested structures
        if let Some(ty) = recurse_into_stmt(stmt, |inner_body| {
            infer_element_from_builtin(container_name, inner_body)
        }) {
            return Some(ty);
        }
    }
    None
}

/// Check a single expression for builtin calls that reveal container element type.
fn check_builtin_expr(container_name: &str, expr: &HirExpr) -> Option<Type> {
    check_numeric_builtin(container_name, expr)
        .or_else(|| check_join_builtin(container_name, expr))
}

/// Check if expr is `sum(lst)`, `max(lst)`, etc. → Int.
fn check_numeric_builtin(container_name: &str, expr: &HirExpr) -> Option<Type> {
    let HirExpr::Call { func, args, .. } = expr else { return None };
    let numeric_builtins = ["sum", "max", "min", "sorted"];
    if !numeric_builtins.contains(&func.as_str()) {
        return None;
    }
    match args.first() {
        Some(HirExpr::Var(name)) if name == container_name => Some(Type::Int),
        _ => None,
    }
}

/// Check if expr is `",".join(lst)` → String.
fn check_join_builtin(container_name: &str, expr: &HirExpr) -> Option<Type> {
    let HirExpr::MethodCall { method, args, .. } = expr else { return None };
    if method != "join" {
        return None;
    }
    match args.first() {
        Some(HirExpr::Var(name)) if name == container_name => Some(Type::String),
        _ => None,
    }
}

/// Infer dict value type from assignment and method call patterns.
fn infer_dict_value_type(dict_name: &str, body: &[HirStmt]) -> Option<Type> {
    infer_dict_value_from_assignment(dict_name, body)
        .or_else(|| infer_dict_value_from_get(dict_name, body))
}

/// Find `d[k] = v` patterns and infer value type from `v`.
fn infer_dict_value_from_assignment(dict_name: &str, body: &[HirStmt]) -> Option<Type> {
    for stmt in body {
        if let Some(ty) = check_dict_index_assign(dict_name, stmt) {
            return Some(ty);
        }
        if let Some(ty) = recurse_into_stmt(stmt, |b| infer_dict_value_from_assignment(dict_name, b)) {
            return Some(ty);
        }
    }
    None
}

/// Check if statement is `dict_name[key] = value` and infer value type.
fn check_dict_index_assign(dict_name: &str, stmt: &HirStmt) -> Option<Type> {
    let HirStmt::Assign { target, value, .. } = stmt else {
        return None;
    };
    let depyler_hir::hir::AssignTarget::Index { base, .. } = target else {
        return None;
    };
    let HirExpr::Var(name) = base.as_ref() else {
        return None;
    };
    if name != dict_name {
        return None;
    }
    infer_type_from_literal_or_expr(value)
}

/// Find `d.get(k, default)` patterns and infer value type from the default.
fn infer_dict_value_from_get(dict_name: &str, body: &[HirStmt]) -> Option<Type> {
    for stmt in body {
        let expr = match stmt {
            HirStmt::Expr(e) => Some(e),
            HirStmt::Assign { value, .. } => Some(value),
            HirStmt::Return(Some(e)) => Some(e),
            _ => None,
        };

        if let Some(expr) = expr {
            if let Some(ty) = check_dict_get_expr(dict_name, expr) {
                return Some(ty);
            }
        }

        if let Some(ty) = recurse_into_stmt(stmt, |inner_body| {
            infer_dict_value_from_get(dict_name, inner_body)
        }) {
            return Some(ty);
        }
    }
    None
}

/// Check for `dict.get(key, default)` and infer value type from the default argument.
fn check_dict_get_expr(dict_name: &str, expr: &HirExpr) -> Option<Type> {
    if let Some(ty) = try_match_dict_get(dict_name, expr) {
        return Some(ty);
    }
    // Recurse into sub-expressions
    match expr {
        HirExpr::MethodCall { args, .. } => {
            args.iter().find_map(|a| check_dict_get_expr(dict_name, a))
        }
        HirExpr::Binary { left, right, .. } => {
            check_dict_get_expr(dict_name, left)
                .or_else(|| check_dict_get_expr(dict_name, right))
        }
        _ => None,
    }
}

/// Match `dict_name.get(key, default)` and return the default's type.
fn try_match_dict_get(dict_name: &str, expr: &HirExpr) -> Option<Type> {
    let HirExpr::MethodCall { object, method, args, .. } = expr else {
        return None;
    };
    if method != "get" || args.len() < 2 {
        return None;
    }
    let HirExpr::Var(name) = object.as_ref() else {
        return None;
    };
    if name != dict_name {
        return None;
    }
    infer_type_from_literal_or_expr(&args[1])
}

/// Infer type from comparison usage patterns (n > 0, n < 10, n == "hello").
///
/// Covers comparison operators that `infer_param_type_from_body` doesn't handle.
fn infer_type_from_comparisons(var_name: &str, body: &[HirStmt]) -> Option<Type> {
    for stmt in body {
        if let Some(ty) = check_comparison_in_stmt(var_name, stmt) {
            return Some(ty);
        }
        if let Some(ty) = recurse_into_stmt(stmt, |b| infer_type_from_comparisons(var_name, b)) {
            return Some(ty);
        }
    }
    None
}

/// Extract comparison expressions from a single statement and check them.
fn check_comparison_in_stmt(var_name: &str, stmt: &HirStmt) -> Option<Type> {
    // If-statement conditions are prime comparison sites
    if let HirStmt::If { condition, .. } = stmt {
        if let Some(ty) = check_comparison_expr(var_name, condition) {
            return Some(ty);
        }
    }
    let expr = extract_stmt_expr(stmt)?;
    check_comparison_expr(var_name, expr)
}

/// Extract the primary expression from a statement, if any.
fn extract_stmt_expr(stmt: &HirStmt) -> Option<&HirExpr> {
    match stmt {
        HirStmt::Expr(e) | HirStmt::Assign { value: e, .. } => Some(e),
        HirStmt::Return(Some(e)) => Some(e),
        _ => None,
    }
}

/// Check a single expression for comparison patterns involving `var_name`.
fn check_comparison_expr(var_name: &str, expr: &HirExpr) -> Option<Type> {
    use depyler_hir::hir::BinOp;

    let HirExpr::Binary { op, left, right, .. } = expr else {
        return None;
    };

    // Recurse into logical combinations (n > 0 and n < 10)
    if matches!(op, BinOp::And | BinOp::Or) {
        return check_comparison_expr(var_name, left)
            .or_else(|| check_comparison_expr(var_name, right));
    }

    if !matches!(op, BinOp::Gt | BinOp::Lt | BinOp::GtEq | BinOp::LtEq | BinOp::Eq | BinOp::NotEq) {
        return None;
    }

    // var > literal → infer from literal
    check_var_against_literal(var_name, left, right)
        .or_else(|| check_var_against_literal(var_name, right, left))
}

/// If `candidate` is `Var(var_name)`, infer type from `other` side of comparison.
fn check_var_against_literal(var_name: &str, candidate: &HirExpr, other: &HirExpr) -> Option<Type> {
    if let HirExpr::Var(name) = candidate {
        if name == var_name {
            return infer_type_from_literal_or_expr(other);
        }
    }
    None
}

/// Infer type from a literal expression or simple patterns.
fn infer_type_from_literal_or_expr(expr: &HirExpr) -> Option<Type> {
    match expr {
        HirExpr::Literal(Literal::Int(_)) => Some(Type::Int),
        HirExpr::Literal(Literal::Float(_)) => Some(Type::Float),
        HirExpr::Literal(Literal::String(_)) => Some(Type::String),
        HirExpr::Literal(Literal::Bool(_)) => Some(Type::Bool),
        HirExpr::Unary { operand, .. } => infer_type_from_literal_or_expr(operand),
        HirExpr::Binary { left, right, op, .. } => {
            // Arithmetic ops on int literals → Int
            if matches!(op, depyler_hir::hir::BinOp::Add
                | depyler_hir::hir::BinOp::Sub
                | depyler_hir::hir::BinOp::Mul
                | depyler_hir::hir::BinOp::Div
                | depyler_hir::hir::BinOp::Mod
                | depyler_hir::hir::BinOp::FloorDiv
            ) {
                let left_ty = infer_type_from_literal_or_expr(left);
                let right_ty = infer_type_from_literal_or_expr(right);
                match (left_ty, right_ty) {
                    (Some(Type::Float), _) | (_, Some(Type::Float)) => return Some(Type::Float),
                    (Some(Type::Int), _) | (_, Some(Type::Int)) => return Some(Type::Int),
                    _ => {}
                }
            }
            None
        }
        HirExpr::List(_) => Some(Type::List(Box::new(Type::Unknown))),
        _ => None,
    }
}

/// Iterate all variable types entries with unknown inner types and refine them.
///
/// This is the main entry point for local variable refinement.
pub fn refine_container_types_from_usage(
    body: &[HirStmt],
    var_types: &mut HashMap<String, Type>,
) {
    let refinements: Vec<(String, Type)> = var_types
        .iter()
        .filter(|(_, ty)| has_unknown_inner_type(ty))
        .filter_map(|(name, ty)| {
            infer_container_element_type(name, ty, body).map(|refined| (name.clone(), refined))
        })
        .collect();

    for (name, refined_ty) in refinements {
        var_types.insert(name, refined_ty);
    }
}

/// Helper to recurse into compound statement bodies.
fn recurse_into_stmt<F>(stmt: &HirStmt, f: F) -> Option<Type>
where
    F: Fn(&[HirStmt]) -> Option<Type>,
{
    match stmt {
        HirStmt::If { then_body, else_body, .. } => {
            f(then_body).or_else(|| else_body.as_ref().and_then(|b| f(b)))
        }
        HirStmt::While { body, .. } => f(body),
        HirStmt::With { body, .. } => f(body),
        HirStmt::Try { body, handlers, orelse, finalbody } => {
            f(body)
                .or_else(|| handlers.iter().find_map(|h| f(&h.body)))
                .or_else(|| orelse.as_ref().and_then(|b| f(b)))
                .or_else(|| finalbody.as_ref().and_then(|b| f(b)))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_hir::hir::{AssignTarget, BinOp};

    #[test]
    fn test_has_unknown_inner_type_list() {
        assert!(has_unknown_inner_type(&Type::List(Box::new(Type::Unknown))));
        assert!(!has_unknown_inner_type(&Type::List(Box::new(Type::Int))));
    }

    #[test]
    fn test_has_unknown_inner_type_dict() {
        assert!(has_unknown_inner_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Unknown)
        )));
        assert!(!has_unknown_inner_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
    }

    #[test]
    fn test_has_unknown_inner_type_set() {
        assert!(has_unknown_inner_type(&Type::Set(Box::new(Type::Unknown))));
        assert!(!has_unknown_inner_type(&Type::Set(Box::new(Type::Int))));
    }

    #[test]
    fn test_has_unknown_inner_type_optional_list() {
        assert!(has_unknown_inner_type(&Type::Optional(Box::new(
            Type::List(Box::new(Type::Unknown))
        ))));
    }

    #[test]
    fn test_has_unknown_inner_type_non_container() {
        assert!(!has_unknown_inner_type(&Type::Int));
        assert!(!has_unknown_inner_type(&Type::String));
        assert!(!has_unknown_inner_type(&Type::Unknown));
    }

    #[test]
    fn test_infer_from_for_loop_arithmetic() {
        // for n in numbers: n > 0 → numbers is List(Int)
        let body = vec![HirStmt::For {
            target: AssignTarget::Symbol("n".into()),
            iter: HirExpr::Var("numbers".into()),
            body: vec![HirStmt::Expr(HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(HirExpr::Var("n".into())),
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            })],
        }];

        let result = infer_container_element_type(
            "numbers",
            &Type::List(Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(result, Some(Type::List(Box::new(Type::Int))));
    }

    #[test]
    fn test_infer_from_for_loop_string_method() {
        // for s in items: s.upper() → items is List(String)
        let body = vec![HirStmt::For {
            target: AssignTarget::Symbol("s".into()),
            iter: HirExpr::Var("items".into()),
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("s".into())),
                method: "upper".into(),
                args: vec![],
                kwargs: vec![],
            })],
        }];

        let result = infer_container_element_type(
            "items",
            &Type::List(Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(result, Some(Type::List(Box::new(Type::String))));
    }

    #[test]
    fn test_infer_from_append_int() {
        // lst.append(42)
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".into())),
            method: "append".into(),
            args: vec![HirExpr::Literal(Literal::Int(42))],
            kwargs: vec![],
        })];

        let result = infer_container_element_type(
            "lst",
            &Type::List(Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(result, Some(Type::List(Box::new(Type::Int))));
    }

    #[test]
    fn test_infer_from_append_string() {
        // lst.append("hello")
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".into())),
            method: "append".into(),
            args: vec![HirExpr::Literal(Literal::String("hello".into()))],
            kwargs: vec![],
        })];

        let result = infer_container_element_type(
            "lst",
            &Type::List(Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(result, Some(Type::List(Box::new(Type::String))));
    }

    #[test]
    fn test_infer_from_sum_builtin() {
        // total = sum(numbers)
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("total".into()),
            value: HirExpr::Call {
                func: "sum".into(),
                args: vec![HirExpr::Var("numbers".into())],
                kwargs: vec![],
            },
            type_annotation: None,
        }];

        let result = infer_container_element_type(
            "numbers",
            &Type::List(Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(result, Some(Type::List(Box::new(Type::Int))));
    }

    #[test]
    fn test_infer_from_join_builtin() {
        // result = ",".join(items)
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("result".into()),
            value: HirExpr::MethodCall {
                object: Box::new(HirExpr::Literal(Literal::String(",".into()))),
                method: "join".into(),
                args: vec![HirExpr::Var("items".into())],
                kwargs: vec![],
            },
            type_annotation: None,
        }];

        let result = infer_container_element_type(
            "items",
            &Type::List(Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(result, Some(Type::List(Box::new(Type::String))));
    }

    #[test]
    fn test_infer_dict_value_from_assignment() {
        // d["key"] = 42
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("d".into())),
                index: Box::new(HirExpr::Literal(Literal::String("key".into()))),
            },
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }];

        let result = infer_container_element_type(
            "d",
            &Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(
            result,
            Some(Type::Dict(Box::new(Type::String), Box::new(Type::Int)))
        );
    }

    #[test]
    fn test_infer_dict_value_from_get() {
        // val = d.get("key", 0)
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("val".into()),
            value: HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("d".into())),
                method: "get".into(),
                args: vec![
                    HirExpr::Literal(Literal::String("key".into())),
                    HirExpr::Literal(Literal::Int(0)),
                ],
                kwargs: vec![],
            },
            type_annotation: None,
        }];

        let result = infer_container_element_type(
            "d",
            &Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(
            result,
            Some(Type::Dict(Box::new(Type::String), Box::new(Type::Int)))
        );
    }

    #[test]
    fn test_infer_set_element_from_add() {
        // s.add(42)
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".into())),
            method: "add".into(),
            args: vec![HirExpr::Literal(Literal::Int(42))],
            kwargs: vec![],
        })];

        let result = infer_container_element_type(
            "s",
            &Type::Set(Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(result, Some(Type::Set(Box::new(Type::Int))));
    }

    #[test]
    fn test_no_inference_without_usage() {
        let body = vec![HirStmt::Pass];
        let result = infer_container_element_type(
            "numbers",
            &Type::List(Box::new(Type::Unknown)),
            &body,
        );
        assert_eq!(result, None);
    }

    #[test]
    fn test_refine_container_types_from_usage() {
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".into())),
            method: "append".into(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];

        let mut var_types = HashMap::new();
        var_types.insert("data".into(), Type::List(Box::new(Type::Unknown)));
        var_types.insert("x".into(), Type::Int); // Should be untouched

        refine_container_types_from_usage(&body, &mut var_types);

        assert_eq!(
            var_types.get("data"),
            Some(&Type::List(Box::new(Type::Int)))
        );
        assert_eq!(var_types.get("x"), Some(&Type::Int));
    }
}
