//! Cross-function type propagation for HIR
//!
//! DEPYLER-0575: Cross-function type propagation from call sites
//! DEPYLER-0587: Loop variable type inference from iterators
//! DEPYLER-0950: Inter-procedural type unification
//!
//! This module handles propagating type information across function boundaries
//! by analyzing call sites and inferring types from usage patterns.

use crate::hir::{AssignTarget, HirExpr, HirModule, HirStmt, Literal, Type};
use std::collections::HashMap;

/// DEPYLER-0575: Cross-function type propagation from call sites
/// When a variable with known type is passed to a function, propagate that type
/// to the function's parameter if the parameter type is still Unknown.
pub fn propagate_call_site_types(hir: &mut HirModule) {
    // Phase 1: Build map of function names to their parameter counts and return types
    let func_param_counts: HashMap<String, usize> = hir
        .functions
        .iter()
        .map(|f| (f.name.clone(), f.params.len()))
        .collect();

    let func_return_types: HashMap<String, Type> = hir
        .functions
        .iter()
        .filter(|f| !matches!(f.ret_type, Type::Unknown))
        .map(|f| (f.name.clone(), f.ret_type.clone()))
        .collect();

    // Phase 2: Build map of variable types from all functions
    // Collect from: parameters with types, return values, locals
    let mut var_types: HashMap<(String, String), Type> = HashMap::new(); // (func_name, var_name) -> Type

    for func in &hir.functions {
        // Collect parameter types
        for param in &func.params {
            if !matches!(param.ty, Type::Unknown) {
                var_types.insert((func.name.clone(), param.name.clone()), param.ty.clone());
            }
        }

        // Collect variable types from assignments (including from function calls)
        collect_var_types_from_stmts(&func.body, &func.name, &func_return_types, &mut var_types);
    }

    // Phase 3: Collect call site argument types
    // Maps (called_func_name, param_index) -> inferred_type
    let mut call_site_types: HashMap<(String, usize), Type> = HashMap::new();

    for func in &hir.functions {
        collect_call_site_types(
            &func.body,
            &func.name,
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );
    }

    // Phase 4: Apply call site types to function parameters
    for func in &mut hir.functions {
        for (idx, param) in func.params.iter_mut().enumerate() {
            if let Some(inferred_type) = call_site_types.get(&(func.name.clone(), idx)) {
                // DEPYLER-0950: Call-site evidence from literals is authoritative
                // Override Unknown types AND heuristic String inferences
                // when there's concrete literal evidence (Int/Float/Bool)
                let should_apply = matches!(param.ty, Type::Unknown)
                    || (matches!(param.ty, Type::String)
                        && matches!(inferred_type, Type::Int | Type::Float | Type::Bool));

                if should_apply {
                    param.ty = inferred_type.clone();
                    eprintln!(
                        "DEPYLER-0575: Applied call-site type: {}::{} -> {:?}",
                        func.name, param.name, param.ty
                    );
                }
            }
        }
    }
}

/// Collect variable types from statements (assignments, etc.)
pub fn collect_var_types_from_stmts(
    stmts: &[HirStmt],
    func_name: &str,
    func_return_types: &HashMap<String, Type>,
    var_types: &mut HashMap<(String, String), Type>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(var_name),
                value,
                type_annotation,
            } => {
                // If there's a type annotation, use that
                if let Some(ty) = type_annotation {
                    if !matches!(ty, Type::Unknown) {
                        var_types.insert((func_name.to_string(), var_name.clone()), ty.clone());
                        continue;
                    }
                }
                // Otherwise infer from value (including function call return types)
                if let Some(ty) = infer_expr_type_with_returns(value, func_return_types) {
                    var_types.insert((func_name.to_string(), var_name.clone()), ty);
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_var_types_from_stmts(then_body, func_name, func_return_types, var_types);
                if let Some(else_stmts) = else_body {
                    collect_var_types_from_stmts(
                        else_stmts,
                        func_name,
                        func_return_types,
                        var_types,
                    );
                }
            }
            HirStmt::While { body, .. } => {
                collect_var_types_from_stmts(body, func_name, func_return_types, var_types);
            }
            HirStmt::For { target, iter, body } => {
                // DEPYLER-0587: Track loop variable types from iterator
                if let Some(iter_type) = infer_expr_type_with_returns(iter, func_return_types) {
                    let elem_type = extract_element_type(&iter_type);
                    add_target_to_var_types(target, &elem_type, func_name, var_types);
                }
                collect_var_types_from_stmts(body, func_name, func_return_types, var_types);
            }
            HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => {
                collect_var_types_from_stmts(body, func_name, func_return_types, var_types);
                for handler in handlers {
                    collect_var_types_from_stmts(
                        &handler.body,
                        func_name,
                        func_return_types,
                        var_types,
                    );
                }
                if let Some(finally) = finalbody {
                    collect_var_types_from_stmts(finally, func_name, func_return_types, var_types);
                }
            }
            HirStmt::With { body, .. } => {
                collect_var_types_from_stmts(body, func_name, func_return_types, var_types);
            }
            _ => {}
        }
    }
}

/// DEPYLER-0587: Extract element type from iterator type (Vec<T> → T)
pub fn extract_element_type(iter_type: &Type) -> Type {
    match iter_type {
        Type::List(elem) => (**elem).clone(),
        Type::Dict(k, _) => (**k).clone(), // dict iteration yields keys
        Type::Tuple(elems) => {
            // Tuple iteration - return first element type as approximation
            elems.first().cloned().unwrap_or(Type::Unknown)
        }
        Type::String => Type::String, // str iteration yields str (chars)
        _ => Type::Unknown,
    }
}

/// DEPYLER-0587: Add target variable(s) to var_types map
pub fn add_target_to_var_types(
    target: &AssignTarget,
    elem_type: &Type,
    func_name: &str,
    var_types: &mut HashMap<(String, String), Type>,
) {
    match target {
        AssignTarget::Symbol(var_name) => {
            var_types.insert((func_name.to_string(), var_name.clone()), elem_type.clone());
        }
        AssignTarget::Tuple(targets) => {
            // Handle tuple unpacking: for (i, x) in enumerate(items)
            // elem_type should be Tuple type for enumerate
            if let Type::Tuple(elem_types) = elem_type {
                for (target, ty) in targets.iter().zip(elem_types.iter()) {
                    add_target_to_var_types(target, ty, func_name, var_types);
                }
            } else {
                // Fallback: all targets get Unknown type
                for t in targets {
                    add_target_to_var_types(t, &Type::Unknown, func_name, var_types);
                }
            }
        }
        _ => {} // Index and Attribute targets not typical in for loops
    }
}

/// Infer type from an expression (simple cases only)
#[allow(dead_code)]
pub fn infer_expr_type(expr: &HirExpr) -> Option<Type> {
    infer_expr_type_with_returns(expr, &HashMap::new())
}

/// Infer type from an expression, including function return types
pub fn infer_expr_type_with_returns(
    expr: &HirExpr,
    func_return_types: &HashMap<String, Type>,
) -> Option<Type> {
    match expr {
        HirExpr::Literal(lit) => Some(match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Bool(_) => Type::Bool,
            Literal::None => Type::None,
            _ => return None,
        }),
        HirExpr::List(elems) => {
            let elem_type = elems
                .first()
                .and_then(|e| infer_expr_type_with_returns(e, func_return_types))
                .unwrap_or(Type::Unknown);
            Some(Type::List(Box::new(elem_type)))
        }
        HirExpr::Dict(_) => Some(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Custom("serde_json::Value".to_string())),
        )),
        // DEPYLER-0575: Infer type from function call return type
        HirExpr::Call { func, .. } => func_return_types.get(func).cloned(),
        _ => None,
    }
}

/// Collect call site types: when func(var) is called and var has a known type,
/// record that type for the function's parameter at that position
pub fn collect_call_site_types(
    stmts: &[HirStmt],
    caller_func_name: &str,
    var_types: &HashMap<(String, String), Type>,
    func_param_counts: &HashMap<String, usize>,
    call_site_types: &mut HashMap<(String, usize), Type>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Assign { value, .. } => {
                collect_call_site_types_from_expr(
                    value,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            HirStmt::Expr(expr) | HirStmt::Return(Some(expr)) => {
                collect_call_site_types_from_expr(
                    expr,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_call_site_types_from_expr(
                    condition,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types(
                    then_body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                if let Some(else_stmts) = else_body {
                    collect_call_site_types(
                        else_stmts,
                        caller_func_name,
                        var_types,
                        func_param_counts,
                        call_site_types,
                    );
                }
            }
            HirStmt::While { condition, body } => {
                collect_call_site_types_from_expr(
                    condition,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types(
                    body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            HirStmt::For { iter, body, .. } => {
                collect_call_site_types_from_expr(
                    iter,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types(
                    body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => {
                collect_call_site_types(
                    body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                for handler in handlers {
                    collect_call_site_types(
                        &handler.body,
                        caller_func_name,
                        var_types,
                        func_param_counts,
                        call_site_types,
                    );
                }
                if let Some(finally) = finalbody {
                    collect_call_site_types(
                        finally,
                        caller_func_name,
                        var_types,
                        func_param_counts,
                        call_site_types,
                    );
                }
            }
            HirStmt::With { context, body, .. } => {
                collect_call_site_types_from_expr(
                    context,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types(
                    body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            _ => {}
        }
    }
}

/// Collect call site types from an expression
pub fn collect_call_site_types_from_expr(
    expr: &HirExpr,
    caller_func_name: &str,
    var_types: &HashMap<(String, String), Type>,
    func_param_counts: &HashMap<String, usize>,
    call_site_types: &mut HashMap<(String, usize), Type>,
) {
    match expr {
        HirExpr::Call { func, args, .. } => {
            // Check if this is a call to a user-defined function
            if func_param_counts.contains_key(func) {
                // For each argument, if it's a variable with known type, record it
                for (idx, arg) in args.iter().enumerate() {
                    // DEPYLER-0950: Handle both variable and literal arguments
                    let arg_type = match arg {
                        HirExpr::Var(var_name) => {
                            // Look up the variable's type in the caller's scope
                            var_types
                                .get(&(caller_func_name.to_string(), var_name.clone()))
                                .cloned()
                        }
                        // Handle literal arguments (e.g., process(10), add(1, 2.5))
                        HirExpr::Literal(lit) => Some(match lit {
                            Literal::Int(_) => Type::Int,
                            Literal::Float(_) => Type::Float,
                            Literal::String(_) => Type::String,
                            Literal::Bool(_) => Type::Bool,
                            _ => Type::Unknown,
                        }),
                        _ => None,
                    };

                    if let Some(ty) = arg_type {
                        // DEPYLER-0575: Skip Unknown and Optional types
                        // Optional types often get unwrapped before use, so don't propagate them
                        let should_propagate = !matches!(ty, Type::Unknown | Type::Optional(_));
                        if should_propagate {
                            call_site_types.insert((func.clone(), idx), ty.clone());
                        }
                    }
                }
            }
            // Recurse into arguments
            for arg in args {
                collect_call_site_types_from_expr(
                    arg,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
        }
        HirExpr::MethodCall { object, args, .. } => {
            collect_call_site_types_from_expr(
                object,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            for arg in args {
                collect_call_site_types_from_expr(
                    arg,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
        }
        HirExpr::Binary { left, right, .. } => {
            collect_call_site_types_from_expr(
                left,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            collect_call_site_types_from_expr(
                right,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
        }
        HirExpr::Unary { operand, .. } => {
            collect_call_site_types_from_expr(
                operand,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
        }
        HirExpr::Index { base, index } => {
            collect_call_site_types_from_expr(
                base,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            collect_call_site_types_from_expr(
                index,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
        }
        HirExpr::List(elems) => {
            for elem in elems {
                collect_call_site_types_from_expr(
                    elem,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
        }
        HirExpr::Dict(items) => {
            for (k, v) in items {
                collect_call_site_types_from_expr(
                    k,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types_from_expr(
                    v,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
        }
        HirExpr::IfExpr { test, body, orelse } => {
            collect_call_site_types_from_expr(
                test,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            collect_call_site_types_from_expr(
                body,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            collect_call_site_types_from_expr(
                orelse,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ extract_element_type tests ============

    #[test]
    fn test_extract_element_type_list_int() {
        let list_type = Type::List(Box::new(Type::Int));
        assert_eq!(extract_element_type(&list_type), Type::Int);
    }

    #[test]
    fn test_extract_element_type_list_string() {
        let list_type = Type::List(Box::new(Type::String));
        assert_eq!(extract_element_type(&list_type), Type::String);
    }

    #[test]
    fn test_extract_element_type_list_nested() {
        let nested = Type::List(Box::new(Type::List(Box::new(Type::Float))));
        assert_eq!(
            extract_element_type(&nested),
            Type::List(Box::new(Type::Float))
        );
    }

    #[test]
    fn test_extract_element_type_dict_yields_keys() {
        let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        assert_eq!(extract_element_type(&dict_type), Type::String);
    }

    #[test]
    fn test_extract_element_type_tuple() {
        let tuple_type = Type::Tuple(vec![Type::Int, Type::String]);
        assert_eq!(extract_element_type(&tuple_type), Type::Int);
    }

    #[test]
    fn test_extract_element_type_empty_tuple() {
        let tuple_type = Type::Tuple(vec![]);
        assert_eq!(extract_element_type(&tuple_type), Type::Unknown);
    }

    #[test]
    fn test_extract_element_type_string() {
        assert_eq!(extract_element_type(&Type::String), Type::String);
    }

    #[test]
    fn test_extract_element_type_unknown() {
        assert_eq!(extract_element_type(&Type::Int), Type::Unknown);
        assert_eq!(extract_element_type(&Type::Float), Type::Unknown);
        assert_eq!(extract_element_type(&Type::Bool), Type::Unknown);
    }

    // ============ add_target_to_var_types tests ============

    #[test]
    fn test_add_target_symbol() {
        let mut var_types = HashMap::new();
        let target = AssignTarget::Symbol("x".to_string());
        add_target_to_var_types(&target, &Type::Int, "func", &mut var_types);
        assert_eq!(
            var_types.get(&("func".to_string(), "x".to_string())),
            Some(&Type::Int)
        );
    }

    #[test]
    fn test_add_target_tuple_with_tuple_type() {
        let mut var_types = HashMap::new();
        let target = AssignTarget::Tuple(vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ]);
        let tuple_type = Type::Tuple(vec![Type::Int, Type::String]);
        add_target_to_var_types(&target, &tuple_type, "func", &mut var_types);
        assert_eq!(
            var_types.get(&("func".to_string(), "a".to_string())),
            Some(&Type::Int)
        );
        assert_eq!(
            var_types.get(&("func".to_string(), "b".to_string())),
            Some(&Type::String)
        );
    }

    #[test]
    fn test_add_target_tuple_with_non_tuple_type() {
        let mut var_types = HashMap::new();
        let target = AssignTarget::Tuple(vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ]);
        add_target_to_var_types(&target, &Type::Int, "func", &mut var_types);
        // When tuple target meets non-tuple type, all get Unknown
        assert_eq!(
            var_types.get(&("func".to_string(), "a".to_string())),
            Some(&Type::Unknown)
        );
        assert_eq!(
            var_types.get(&("func".to_string(), "b".to_string())),
            Some(&Type::Unknown)
        );
    }

    #[test]
    fn test_add_target_nested_tuple() {
        let mut var_types = HashMap::new();
        let target = AssignTarget::Tuple(vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Tuple(vec![
                AssignTarget::Symbol("b".to_string()),
                AssignTarget::Symbol("c".to_string()),
            ]),
        ]);
        let tuple_type = Type::Tuple(vec![
            Type::Int,
            Type::Tuple(vec![Type::String, Type::Float]),
        ]);
        add_target_to_var_types(&target, &tuple_type, "func", &mut var_types);
        assert_eq!(
            var_types.get(&("func".to_string(), "a".to_string())),
            Some(&Type::Int)
        );
        assert_eq!(
            var_types.get(&("func".to_string(), "b".to_string())),
            Some(&Type::String)
        );
        assert_eq!(
            var_types.get(&("func".to_string(), "c".to_string())),
            Some(&Type::Float)
        );
    }

    // ============ infer_expr_type tests ============

    #[test]
    fn test_infer_expr_type_int_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert_eq!(infer_expr_type(&expr), Some(Type::Int));
    }

    #[test]
    fn test_infer_expr_type_float_literal() {
        let expr = HirExpr::Literal(Literal::Float(3.15));
        assert_eq!(infer_expr_type(&expr), Some(Type::Float));
    }

    #[test]
    fn test_infer_expr_type_string_literal() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert_eq!(infer_expr_type(&expr), Some(Type::String));
    }

    #[test]
    fn test_infer_expr_type_bool_literal() {
        let expr = HirExpr::Literal(Literal::Bool(true));
        assert_eq!(infer_expr_type(&expr), Some(Type::Bool));
    }

    #[test]
    fn test_infer_expr_type_none_literal() {
        let expr = HirExpr::Literal(Literal::None);
        assert_eq!(infer_expr_type(&expr), Some(Type::None));
    }

    #[test]
    fn test_infer_expr_type_list_int() {
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        assert_eq!(
            infer_expr_type(&expr),
            Some(Type::List(Box::new(Type::Int)))
        );
    }

    #[test]
    fn test_infer_expr_type_empty_list() {
        let expr = HirExpr::List(vec![]);
        assert_eq!(
            infer_expr_type(&expr),
            Some(Type::List(Box::new(Type::Unknown)))
        );
    }

    #[test]
    fn test_infer_expr_type_dict() {
        let expr = HirExpr::Dict(vec![]);
        let result = infer_expr_type(&expr);
        assert!(matches!(result, Some(Type::Dict(_, _))));
    }

    #[test]
    fn test_infer_expr_type_var_returns_none() {
        let expr = HirExpr::Var("x".to_string());
        assert_eq!(infer_expr_type(&expr), None);
    }

    // ============ infer_expr_type_with_returns tests ============

    #[test]
    fn test_infer_expr_type_with_returns_call() {
        let mut func_return_types = HashMap::new();
        func_return_types.insert("get_count".to_string(), Type::Int);

        let expr = HirExpr::Call {
            func: "get_count".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert_eq!(
            infer_expr_type_with_returns(&expr, &func_return_types),
            Some(Type::Int)
        );
    }

    #[test]
    fn test_infer_expr_type_with_returns_unknown_call() {
        let func_return_types = HashMap::new();
        let expr = HirExpr::Call {
            func: "unknown_func".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert_eq!(
            infer_expr_type_with_returns(&expr, &func_return_types),
            None
        );
    }

    #[test]
    fn test_infer_expr_type_with_returns_literal() {
        let func_return_types = HashMap::new();
        let expr = HirExpr::Literal(Literal::Int(42));
        assert_eq!(
            infer_expr_type_with_returns(&expr, &func_return_types),
            Some(Type::Int)
        );
    }

    // ============ collect_call_site_types_from_expr tests ============

    #[test]
    fn test_collect_call_site_types_from_literal_arg() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("process".to_string(), 1);
        let var_types = HashMap::new();

        let expr = HirExpr::Call {
            func: "process".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(42))],
            kwargs: vec![],
        };

        collect_call_site_types_from_expr(
            &expr,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("process".to_string(), 0)),
            Some(&Type::Int)
        );
    }

    #[test]
    fn test_collect_call_site_types_from_float_literal() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("compute".to_string(), 1);
        let var_types = HashMap::new();

        let expr = HirExpr::Call {
            func: "compute".to_string(),
            args: vec![HirExpr::Literal(Literal::Float(3.15))],
            kwargs: vec![],
        };

        collect_call_site_types_from_expr(
            &expr,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("compute".to_string(), 0)),
            Some(&Type::Float)
        );
    }

    #[test]
    fn test_collect_call_site_types_from_var_arg() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("process".to_string(), 1);

        let mut var_types = HashMap::new();
        var_types.insert(("caller".to_string(), "x".to_string()), Type::String);

        let expr = HirExpr::Call {
            func: "process".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };

        collect_call_site_types_from_expr(
            &expr,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("process".to_string(), 0)),
            Some(&Type::String)
        );
    }

    #[test]
    fn test_collect_call_site_types_skips_unknown() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("process".to_string(), 1);

        let mut var_types = HashMap::new();
        var_types.insert(("caller".to_string(), "x".to_string()), Type::Unknown);

        let expr = HirExpr::Call {
            func: "process".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };

        collect_call_site_types_from_expr(
            &expr,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        // Unknown types should not be propagated
        assert!(!call_site_types.contains_key(&("process".to_string(), 0)));
    }

    #[test]
    fn test_collect_call_site_types_skips_optional() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("process".to_string(), 1);

        let mut var_types = HashMap::new();
        var_types.insert(
            ("caller".to_string(), "x".to_string()),
            Type::Optional(Box::new(Type::Int)),
        );

        let expr = HirExpr::Call {
            func: "process".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };

        collect_call_site_types_from_expr(
            &expr,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        // Optional types should not be propagated
        assert!(!call_site_types.contains_key(&("process".to_string(), 0)));
    }

    #[test]
    fn test_collect_call_site_types_multiple_args() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("add".to_string(), 2);
        let var_types = HashMap::new();

        let expr = HirExpr::Call {
            func: "add".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Float(2.5)),
            ],
            kwargs: vec![],
        };

        collect_call_site_types_from_expr(
            &expr,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("add".to_string(), 0)),
            Some(&Type::Int)
        );
        assert_eq!(
            call_site_types.get(&("add".to_string(), 1)),
            Some(&Type::Float)
        );
    }

    #[test]
    fn test_collect_call_site_types_ignores_non_user_func() {
        let mut call_site_types = HashMap::new();
        let func_param_counts = HashMap::new(); // No user functions
        let var_types = HashMap::new();

        let expr = HirExpr::Call {
            func: "builtin_func".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(42))],
            kwargs: vec![],
        };

        collect_call_site_types_from_expr(
            &expr,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        // Should not record types for non-user functions
        assert!(call_site_types.is_empty());
    }

    #[test]
    fn test_collect_call_site_types_binary_expr() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("process".to_string(), 1);
        let var_types = HashMap::new();

        // process(10) nested in binary expr: result + process(10)
        let expr = HirExpr::Binary {
            op: crate::hir::BinOp::Add,
            left: Box::new(HirExpr::Var("result".to_string())),
            right: Box::new(HirExpr::Call {
                func: "process".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            }),
        };

        collect_call_site_types_from_expr(
            &expr,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("process".to_string(), 0)),
            Some(&Type::Int)
        );
    }

    #[test]
    fn test_collect_call_site_types_if_expr() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("f".to_string(), 1);
        let var_types = HashMap::new();

        // f(1) if cond else f(2)
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(HirExpr::Call {
                func: "f".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            }),
            orelse: Box::new(HirExpr::Call {
                func: "f".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(2))],
                kwargs: vec![],
            }),
        };

        collect_call_site_types_from_expr(
            &expr,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        // Should find call site from both branches
        assert_eq!(call_site_types.get(&("f".to_string(), 0)), Some(&Type::Int));
    }

    // ============ collect_call_site_types (stmts) tests ============

    #[test]
    fn test_collect_call_site_types_from_assign() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("get_value".to_string(), 1);
        let var_types = HashMap::new();

        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Call {
                func: "get_value".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(42))],
                kwargs: vec![],
            },
            type_annotation: None,
        }];

        collect_call_site_types(
            &stmts,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("get_value".to_string(), 0)),
            Some(&Type::Int)
        );
    }

    #[test]
    fn test_collect_call_site_types_from_return() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("compute".to_string(), 1);
        let var_types = HashMap::new();

        let stmts = vec![HirStmt::Return(Some(HirExpr::Call {
            func: "compute".to_string(),
            args: vec![HirExpr::Literal(Literal::String("test".to_string()))],
            kwargs: vec![],
        }))];

        collect_call_site_types(
            &stmts,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("compute".to_string(), 0)),
            Some(&Type::String)
        );
    }

    #[test]
    fn test_collect_call_site_types_from_if_stmt() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("process".to_string(), 1);
        let var_types = HashMap::new();

        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Expr(HirExpr::Call {
                func: "process".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            })],
            else_body: Some(vec![HirStmt::Expr(HirExpr::Call {
                func: "process".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(2))],
                kwargs: vec![],
            })]),
        }];

        collect_call_site_types(
            &stmts,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("process".to_string(), 0)),
            Some(&Type::Int)
        );
    }

    #[test]
    fn test_collect_call_site_types_from_while_stmt() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("update".to_string(), 1);
        let var_types = HashMap::new();

        let stmts = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "update".to_string(),
                args: vec![HirExpr::Literal(Literal::Float(1.5))],
                kwargs: vec![],
            })],
        }];

        collect_call_site_types(
            &stmts,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("update".to_string(), 0)),
            Some(&Type::Float)
        );
    }

    #[test]
    fn test_collect_call_site_types_from_for_stmt() {
        let mut call_site_types = HashMap::new();
        let mut func_param_counts = HashMap::new();
        func_param_counts.insert("handle".to_string(), 1);
        let var_types = HashMap::new();

        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "handle".to_string(),
                args: vec![HirExpr::Literal(Literal::Bool(false))],
                kwargs: vec![],
            })],
        }];

        collect_call_site_types(
            &stmts,
            "caller",
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );

        assert_eq!(
            call_site_types.get(&("handle".to_string(), 0)),
            Some(&Type::Bool)
        );
    }

    // ============ collect_var_types_from_stmts tests ============

    #[test]
    fn test_collect_var_types_from_assign_with_annotation() {
        let mut var_types = HashMap::new();
        let func_return_types = HashMap::new();

        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Float), // Annotation overrides
        }];

        collect_var_types_from_stmts(&stmts, "func", &func_return_types, &mut var_types);

        assert_eq!(
            var_types.get(&("func".to_string(), "x".to_string())),
            Some(&Type::Float)
        );
    }

    #[test]
    fn test_collect_var_types_from_assign_without_annotation() {
        let mut var_types = HashMap::new();
        let func_return_types = HashMap::new();

        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }];

        collect_var_types_from_stmts(&stmts, "func", &func_return_types, &mut var_types);

        assert_eq!(
            var_types.get(&("func".to_string(), "x".to_string())),
            Some(&Type::Int)
        );
    }

    #[test]
    fn test_collect_var_types_from_assign_call_return() {
        let mut var_types = HashMap::new();
        let mut func_return_types = HashMap::new();
        func_return_types.insert("get_count".to_string(), Type::Int);

        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("count".to_string()),
            value: HirExpr::Call {
                func: "get_count".to_string(),
                args: vec![],
                kwargs: vec![],
            },
            type_annotation: None,
        }];

        collect_var_types_from_stmts(&stmts, "func", &func_return_types, &mut var_types);

        assert_eq!(
            var_types.get(&("func".to_string(), "count".to_string())),
            Some(&Type::Int)
        );
    }

    #[test]
    fn test_collect_var_types_from_for_loop() {
        let mut var_types = HashMap::new();
        let func_return_types = HashMap::new();

        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("item".to_string()),
            iter: HirExpr::List(vec![HirExpr::Literal(Literal::String("a".to_string()))]),
            body: vec![],
        }];

        collect_var_types_from_stmts(&stmts, "func", &func_return_types, &mut var_types);

        assert_eq!(
            var_types.get(&("func".to_string(), "item".to_string())),
            Some(&Type::String)
        );
    }

    #[test]
    fn test_collect_var_types_from_if_stmt() {
        let mut var_types = HashMap::new();
        let func_return_types = HashMap::new();

        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            }],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Literal(Literal::Float(2.0)),
                type_annotation: None,
            }]),
        }];

        collect_var_types_from_stmts(&stmts, "func", &func_return_types, &mut var_types);

        assert_eq!(
            var_types.get(&("func".to_string(), "x".to_string())),
            Some(&Type::Int)
        );
        assert_eq!(
            var_types.get(&("func".to_string(), "y".to_string())),
            Some(&Type::Float)
        );
    }
}
