fn check_expr_for_capture(
    expr: &crate::hir::HirExpr,
    local_vars: &std::collections::HashSet<&str>,
    outer_vars: &std::collections::HashSet<String>,
) -> bool {
    use crate::hir::HirExpr;
    match expr {
        HirExpr::Var(name) => {
            !local_vars.contains(name.as_str()) && outer_vars.contains(name)
        }
        HirExpr::Binary { left, right, .. } => {
            check_expr_for_capture(left, local_vars, outer_vars)
                || check_expr_for_capture(right, local_vars, outer_vars)
        }
        HirExpr::Unary { operand, .. } => {
            check_expr_for_capture(operand, local_vars, outer_vars)
        }
        HirExpr::Call { func, args, kwargs, .. } => {
            let captures_func =
                !local_vars.contains(func.as_str()) && outer_vars.contains(func);
            captures_func
                || args.iter().any(|a| check_expr_for_capture(a, local_vars, outer_vars))
                || kwargs.iter().any(|(_, v)| check_expr_for_capture(v, local_vars, outer_vars))
        }
        HirExpr::DynamicCall { callee, args, kwargs } => {
            check_expr_for_capture(callee, local_vars, outer_vars)
                || args.iter().any(|a| check_expr_for_capture(a, local_vars, outer_vars))
                || kwargs.iter().any(|(_, v)| check_expr_for_capture(v, local_vars, outer_vars))
        }
        HirExpr::MethodCall { object, args, kwargs, .. } => {
            check_expr_for_capture(object, local_vars, outer_vars)
                || args.iter().any(|a| check_expr_for_capture(a, local_vars, outer_vars))
                || kwargs.iter().any(|(_, v)| check_expr_for_capture(v, local_vars, outer_vars))
        }
        HirExpr::Attribute { value, .. } => {
            check_expr_for_capture(value, local_vars, outer_vars)
        }
        HirExpr::Index { base, index } => {
            check_expr_for_capture(base, local_vars, outer_vars)
                || check_expr_for_capture(index, local_vars, outer_vars)
        }
        HirExpr::IfExpr { test, body, orelse } => {
            check_expr_for_capture(test, local_vars, outer_vars)
                || check_expr_for_capture(body, local_vars, outer_vars)
                || check_expr_for_capture(orelse, local_vars, outer_vars)
        }
        HirExpr::List(items)
        | HirExpr::Tuple(items)
        | HirExpr::Set(items)
        | HirExpr::FrozenSet(items) => {
            items.iter().any(|i| check_expr_for_capture(i, local_vars, outer_vars))
        }
        HirExpr::Dict(pairs) => pairs.iter().any(|(k, v)| {
            check_expr_for_capture(k, local_vars, outer_vars)
                || check_expr_for_capture(v, local_vars, outer_vars)
        }),
        HirExpr::ListComp { element, generators }
        | HirExpr::SetComp { element, generators }
        | HirExpr::GeneratorExp { element, generators } => {
            check_expr_capture_in_comprehension(element, generators, local_vars, outer_vars)
        }
        HirExpr::DictComp { key, value, generators } => {
            check_expr_for_capture(key, local_vars, outer_vars)
                || check_expr_for_capture(value, local_vars, outer_vars)
                || generators.iter().any(|g| {
                    check_expr_for_capture(&g.iter, local_vars, outer_vars)
                        || g.conditions
                            .iter()
                            .any(|c| check_expr_for_capture(c, local_vars, outer_vars))
                })
        }
        HirExpr::Lambda { body, .. } => check_expr_for_capture(body, local_vars, outer_vars),
        HirExpr::Await { value } => check_expr_for_capture(value, local_vars, outer_vars),
        HirExpr::Slice { base, start, stop, step } => {
            check_expr_capture_in_slice(base, start, stop, step, local_vars, outer_vars)
        }
        HirExpr::Borrow { expr, .. } => check_expr_for_capture(expr, local_vars, outer_vars),
        HirExpr::FString { parts } => parts.iter().any(|p| {
            if let crate::hir::FStringPart::Expr(e) = p {
                check_expr_for_capture(e, local_vars, outer_vars)
            } else {
                false
            }
        }),
        HirExpr::Yield { value } => {
            value.as_ref().is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
        }
        HirExpr::SortByKey { iterable, key_body, reverse_expr, .. } => {
            check_expr_for_capture(iterable, local_vars, outer_vars)
                || check_expr_for_capture(key_body, local_vars, outer_vars)
                || reverse_expr
                    .as_ref()
                    .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
        }
        HirExpr::NamedExpr { value, .. } => {
            check_expr_for_capture(value, local_vars, outer_vars)
        }
        _ => false,
    }
}

fn check_expr_capture_in_comprehension(
    element: &crate::hir::HirExpr,
    generators: &[crate::hir::Comprehension],
    local_vars: &std::collections::HashSet<&str>,
    outer_vars: &std::collections::HashSet<String>,
) -> bool {
    check_expr_for_capture(element, local_vars, outer_vars)
        || generators.iter().any(|g| {
            check_expr_for_capture(&g.iter, local_vars, outer_vars)
                || g.conditions
                    .iter()
                    .any(|c| check_expr_for_capture(c, local_vars, outer_vars))
        })
}

fn check_expr_capture_in_slice(
    base: &crate::hir::HirExpr,
    start: &Option<Box<crate::hir::HirExpr>>,
    stop: &Option<Box<crate::hir::HirExpr>>,
    step: &Option<Box<crate::hir::HirExpr>>,
    local_vars: &std::collections::HashSet<&str>,
    outer_vars: &std::collections::HashSet<String>,
) -> bool {
    check_expr_for_capture(base, local_vars, outer_vars)
        || start
            .as_ref()
            .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
        || stop
            .as_ref()
            .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
        || step
            .as_ref()
            .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
}
