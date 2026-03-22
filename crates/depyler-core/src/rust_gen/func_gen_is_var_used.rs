fn is_var_used_in_remaining_stmts(var_name: &str, stmts: &[HirStmt]) -> bool {
    stmts.iter().any(|stmt| is_var_used_anywhere(var_name, stmt))
}

fn is_var_used_anywhere(var_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            is_var_used_in_target(var_name, target) || is_var_used_in_expr_any(var_name, value)
        }
        HirStmt::If { condition, then_body, else_body } => {
            is_var_used_in_expr_any(var_name, condition)
                || then_body.iter().any(|s| is_var_used_anywhere(var_name, s))
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(|s| is_var_used_anywhere(var_name, s)))
        }
        HirStmt::While { condition, body } => {
            is_var_used_in_expr_any(var_name, condition)
                || body.iter().any(|s| is_var_used_anywhere(var_name, s))
        }
        HirStmt::For { iter, body, .. } => {
            is_var_used_in_expr_any(var_name, iter)
                || body.iter().any(|s| is_var_used_anywhere(var_name, s))
        }
        HirStmt::Return(Some(expr)) => is_var_used_in_expr_any(var_name, expr),
        HirStmt::Expr(expr) => is_var_used_in_expr_any(var_name, expr),
        HirStmt::Raise { exception, .. } => {
            exception.as_ref().is_some_and(|e| is_var_used_in_expr_any(var_name, e))
        }
        HirStmt::Assert { test, msg, .. } => {
            is_var_used_in_expr_any(var_name, test)
                || msg.as_ref().is_some_and(|m| is_var_used_in_expr_any(var_name, m))
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
        HirStmt::With { context, body, .. } => {
            is_var_used_in_expr_any(var_name, context)
                || body.iter().any(|s| is_var_used_anywhere(var_name, s))
        }
        _ => false,
    }
}

fn is_var_used_in_target(var_name: &str, target: &AssignTarget) -> bool {
    match target {
        AssignTarget::Symbol(_) => false, // Target itself doesn't use the var
        AssignTarget::Index { base, index } => {
            is_var_used_in_expr_any(var_name, base) || is_var_used_in_expr_any(var_name, index)
        }
        AssignTarget::Attribute { value, .. } => is_var_used_in_expr_any(var_name, value),
        AssignTarget::Tuple(targets) => targets.iter().any(|t| is_var_used_in_target(var_name, t)),
    }
}

fn is_var_used_in_expr_any(var_name: &str, expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Var(name) => name == var_name,
        HirExpr::Literal(_) => false,
        HirExpr::Binary { left, right, .. } => {
            is_var_used_in_expr_any(var_name, left) || is_var_used_in_expr_any(var_name, right)
        }
        HirExpr::Unary { operand, .. } => is_var_used_in_expr_any(var_name, operand),
        HirExpr::Call { func, args, kwargs } => {
            func == var_name
                || args.iter().any(|a| is_var_used_in_expr_any(var_name, a))
                || kwargs.iter().any(|(_, v)| is_var_used_in_expr_any(var_name, v))
        }
        HirExpr::MethodCall { object, args, kwargs, .. } => {
            is_var_used_in_expr_any(var_name, object)
                || args.iter().any(|a| is_var_used_in_expr_any(var_name, a))
                || kwargs.iter().any(|(_, v)| is_var_used_in_expr_any(var_name, v))
        }
        HirExpr::DynamicCall { callee, args, kwargs } => {
            is_var_used_in_expr_any(var_name, callee)
                || args.iter().any(|a| is_var_used_in_expr_any(var_name, a))
                || kwargs.iter().any(|(_, v)| is_var_used_in_expr_any(var_name, v))
        }
        HirExpr::Attribute { value, .. } => is_var_used_in_expr_any(var_name, value),
        HirExpr::Index { base, index } => {
            is_var_used_in_expr_any(var_name, base) || is_var_used_in_expr_any(var_name, index)
        }
        HirExpr::Slice { base, start, stop, step } => {
            is_var_used_in_expr_any(var_name, base)
                || start.as_ref().is_some_and(|s| is_var_used_in_expr_any(var_name, s))
                || stop.as_ref().is_some_and(|s| is_var_used_in_expr_any(var_name, s))
                || step.as_ref().is_some_and(|s| is_var_used_in_expr_any(var_name, s))
        }
        HirExpr::List(items)
        | HirExpr::Tuple(items)
        | HirExpr::Set(items)
        | HirExpr::FrozenSet(items) => items.iter().any(|i| is_var_used_in_expr_any(var_name, i)),
        HirExpr::Dict(pairs) => pairs.iter().any(|(k, v)| {
            is_var_used_in_expr_any(var_name, k) || is_var_used_in_expr_any(var_name, v)
        }),
        HirExpr::Borrow { expr, .. } => is_var_used_in_expr_any(var_name, expr),
        HirExpr::IfExpr { test, body, orelse } => {
            is_var_used_in_expr_any(var_name, test)
                || is_var_used_in_expr_any(var_name, body)
                || is_var_used_in_expr_any(var_name, orelse)
        }
        HirExpr::ListComp { element, generators }
        | HirExpr::SetComp { element, generators }
        | HirExpr::GeneratorExp { element, generators } => {
            is_var_used_in_expr_any(var_name, element)
                || generators.iter().any(|g| {
                    is_var_used_in_expr_any(var_name, &g.iter)
                        || g.conditions.iter().any(|i| is_var_used_in_expr_any(var_name, i))
                })
        }
        HirExpr::DictComp { key, value, generators } => {
            is_var_used_in_expr_any(var_name, key)
                || is_var_used_in_expr_any(var_name, value)
                || generators.iter().any(|g| {
                    is_var_used_in_expr_any(var_name, &g.iter)
                        || g.conditions.iter().any(|i| is_var_used_in_expr_any(var_name, i))
                })
        }
        HirExpr::Lambda { body, .. } => is_var_used_in_expr_any(var_name, body),
        HirExpr::Await { value } => is_var_used_in_expr_any(var_name, value),
        HirExpr::FString { parts } => parts.iter().any(|p| match p {
            crate::hir::FStringPart::Expr(e) => is_var_used_in_expr_any(var_name, e),
            _ => false,
        }),
        HirExpr::Yield { value } => {
            value.as_ref().is_some_and(|v| is_var_used_in_expr_any(var_name, v))
        }
        HirExpr::SortByKey { iterable, key_body, reverse_expr, .. } => {
            is_var_used_in_expr_any(var_name, iterable)
                || is_var_used_in_expr_any(var_name, key_body)
                || reverse_expr.as_ref().is_some_and(|r| is_var_used_in_expr_any(var_name, r))
        }
        HirExpr::NamedExpr { target, value } => {
            target == var_name || is_var_used_in_expr_any(var_name, value)
        }
    }
}
