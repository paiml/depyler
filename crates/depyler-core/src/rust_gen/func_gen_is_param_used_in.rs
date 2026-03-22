fn is_param_used_in_stmt(param_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            is_param_used_in_assign_target(param_name, target)
                || is_param_used_in_expr(param_name, value)
        }
        HirStmt::If { condition, then_body, else_body } => {
            is_param_used_in_expr(param_name, condition)
                || then_body.iter().any(|s| is_param_used_in_stmt(param_name, s))
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(|s| is_param_used_in_stmt(param_name, s)))
        }
        HirStmt::While { condition, body } => {
            is_param_used_in_expr(param_name, condition)
                || body.iter().any(|s| is_param_used_in_stmt(param_name, s))
        }
        HirStmt::For { iter, body, .. } => {
            is_param_used_in_expr(param_name, iter)
                || body.iter().any(|s| is_param_used_in_stmt(param_name, s))
        }
        HirStmt::Return(Some(expr)) => is_param_used_in_expr(param_name, expr),
        HirStmt::Expr(expr) => is_param_used_in_expr(param_name, expr),
        HirStmt::Try { body, handlers, orelse, finalbody, .. } => {
            body.iter().any(|s| is_param_used_in_stmt(param_name, s))
                || handlers
                    .iter()
                    .any(|h| h.body.iter().any(|s| is_param_used_in_stmt(param_name, s)))
                || orelse
                    .as_ref()
                    .is_some_and(|stmts| stmts.iter().any(|s| is_param_used_in_stmt(param_name, s)))
                || finalbody
                    .as_ref()
                    .is_some_and(|stmts| stmts.iter().any(|s| is_param_used_in_stmt(param_name, s)))
        }
        // DEPYLER-0833: Also check the context expression in With statements
        // Example: `with open(path) as f:` - path is used in context, not body
        HirStmt::With { context, body, .. } => {
            is_param_used_in_expr(param_name, context)
                || body.iter().any(|s| is_param_used_in_stmt(param_name, s))
        }
        // DEPYLER-0758: Check nested function bodies for closure captures
        // If outer param is used in nested function, it's a closure capture and must not be renamed
        HirStmt::FunctionDef { body, .. } => {
            body.iter().any(|s| is_param_used_in_stmt(param_name, s))
        }
        // DEPYLER-0950: Check assert statements for parameter usage
        HirStmt::Assert { test, msg } => {
            is_param_used_in_expr(param_name, test)
                || msg.as_ref().is_some_and(|e| is_param_used_in_expr(param_name, e))
        }
        _ => false,
    }
}

fn is_param_used_in_assign_target(param_name: &str, target: &AssignTarget) -> bool {
    match target {
        AssignTarget::Symbol(name) => name == param_name,
        AssignTarget::Index { base, index } => {
            is_param_used_in_expr(param_name, base) || is_param_used_in_expr(param_name, index)
        }
        AssignTarget::Attribute { value, .. } => is_param_used_in_expr(param_name, value),
        AssignTarget::Tuple(targets) => {
            targets.iter().any(|t| is_param_used_in_assign_target(param_name, t))
        }
    }
}
