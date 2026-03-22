fn check_stmt_for_capture(
    stmt: &HirStmt,
    local_vars: &std::collections::HashSet<&str>,
    outer_vars: &std::collections::HashSet<String>,
) -> bool {
    match stmt {
        HirStmt::Expr(expr) | HirStmt::Return(Some(expr)) => {
            check_expr_for_capture(expr, local_vars, outer_vars)
        }
        HirStmt::Assign { value, .. } => check_expr_for_capture(value, local_vars, outer_vars),
        HirStmt::If { condition, then_body, else_body } => {
            check_expr_for_capture(condition, local_vars, outer_vars)
                || then_body.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
                || else_body.as_ref().is_some_and(|b| {
                    b.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
                })
        }
        HirStmt::While { condition, body } => {
            check_expr_for_capture(condition, local_vars, outer_vars)
                || body.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
        }
        HirStmt::For { iter, body, .. } => {
            check_expr_for_capture(iter, local_vars, outer_vars)
                || body.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
        }
        HirStmt::With { context, body, .. } => {
            check_expr_for_capture(context, local_vars, outer_vars)
                || body.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
        }
        HirStmt::Try { body, handlers, orelse, finalbody } => {
            check_stmt_capture_in_try(body, handlers, orelse, finalbody, local_vars, outer_vars)
        }
        HirStmt::FunctionDef { body, .. } => {
            body.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
        }
        HirStmt::Block(stmts) => {
            stmts.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
        }
        HirStmt::Assert { test, msg } => {
            check_expr_for_capture(test, local_vars, outer_vars)
                || msg
                    .as_ref()
                    .is_some_and(|m| check_expr_for_capture(m, local_vars, outer_vars))
        }
        HirStmt::Raise { exception, cause } => {
            exception
                .as_ref()
                .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
                || cause
                    .as_ref()
                    .is_some_and(|c| check_expr_for_capture(c, local_vars, outer_vars))
        }
        _ => false,
    }
}

fn check_stmt_capture_in_try(
    body: &[HirStmt],
    handlers: &[crate::hir::ExceptHandler],
    orelse: &Option<Vec<HirStmt>>,
    finalbody: &Option<Vec<HirStmt>>,
    local_vars: &std::collections::HashSet<&str>,
    outer_vars: &std::collections::HashSet<String>,
) -> bool {
    body.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
        || handlers.iter().any(|h| {
            h.body.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
        })
        || orelse.as_ref().is_some_and(|b| {
            b.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
        })
        || finalbody.as_ref().is_some_and(|b| {
            b.iter().any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
        })
}
