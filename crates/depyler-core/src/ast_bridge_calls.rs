fn stmt_calls_failing_function(
    stmt: &HirStmt,
    can_fail_map: &std::collections::HashMap<String, bool>,
) -> bool {
    match stmt {
        HirStmt::Return(Some(expr)) | HirStmt::Expr(expr) | HirStmt::Assign { value: expr, .. } => {
            expr_calls_failing_function(expr, can_fail_map)
        }
        HirStmt::If { condition, then_body, else_body } => {
            expr_calls_failing_function(condition, can_fail_map)
                || calls_failing_function(then_body, can_fail_map)
                || else_body
                    .as_ref()
                    .map(|body| calls_failing_function(body, can_fail_map))
                    .unwrap_or(false)
        }
        HirStmt::While { condition, body } => {
            expr_calls_failing_function(condition, can_fail_map)
                || calls_failing_function(body, can_fail_map)
        }
        HirStmt::For { iter, body, .. } => {
            expr_calls_failing_function(iter, can_fail_map)
                || calls_failing_function(body, can_fail_map)
        }
        HirStmt::Try { body, handlers, finalbody, .. } => {
            calls_failing_function(body, can_fail_map)
                || handlers.iter().any(|h| calls_failing_function(&h.body, can_fail_map))
                || finalbody
                    .as_ref()
                    .map(|fb| calls_failing_function(fb, can_fail_map))
                    .unwrap_or(false)
        }
        _ => false,
    }
}

fn expr_calls_failing_function(
    expr: &HirExpr,
    can_fail_map: &std::collections::HashMap<String, bool>,
) -> bool {
    match expr {
        HirExpr::Call { func, args, .. } => {
            // Check if the called function is known to fail
            if can_fail_map.get(func).copied().unwrap_or(false) {
                return true;
            }
            // Also check arguments recursively
            args.iter().any(|arg| expr_calls_failing_function(arg, can_fail_map))
        }
        HirExpr::Binary { left, right, .. } => {
            expr_calls_failing_function(left, can_fail_map)
                || expr_calls_failing_function(right, can_fail_map)
        }
        HirExpr::Unary { operand, .. } => expr_calls_failing_function(operand, can_fail_map),
        HirExpr::List(elements) | HirExpr::Tuple(elements) | HirExpr::Set(elements) => {
            elements.iter().any(|e| expr_calls_failing_function(e, can_fail_map))
        }
        HirExpr::MethodCall { object, args, .. } => {
            expr_calls_failing_function(object, can_fail_map)
                || args.iter().any(|arg| expr_calls_failing_function(arg, can_fail_map))
        }
        HirExpr::Index { base, index } => {
            expr_calls_failing_function(base, can_fail_map)
                || expr_calls_failing_function(index, can_fail_map)
        }
        HirExpr::Slice { base, .. } => expr_calls_failing_function(base, can_fail_map),
        _ => false,
    }
}
