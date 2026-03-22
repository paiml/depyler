fn walk_expr_for_args(expr: &HirExpr, args_name: &str, fields: &mut std::collections::HashSet<String>) {
    match expr {
        HirExpr::Attribute { value, attr } => {
            if let HirExpr::Var(name) = value.as_ref() {
                if name == args_name {
                    fields.insert(attr.clone());
                }
            }
            walk_expr_for_args(value, args_name, fields);
        }
        HirExpr::Binary { left, right, .. } => {
            walk_expr_for_args(left, args_name, fields);
            walk_expr_for_args(right, args_name, fields);
        }
        HirExpr::Unary { operand, .. } => {
            walk_expr_for_args(operand, args_name, fields);
        }
        HirExpr::Call { args: call_args, kwargs, .. } => {
            for arg in call_args {
                walk_expr_for_args(arg, args_name, fields);
            }
            for (_, kwarg_val) in kwargs {
                walk_expr_for_args(kwarg_val, args_name, fields);
            }
        }
        HirExpr::MethodCall { object, args: method_args, kwargs, .. } => {
            walk_expr_for_args(object, args_name, fields);
            for arg in method_args {
                walk_expr_for_args(arg, args_name, fields);
            }
            for (_, kwarg_val) in kwargs {
                walk_expr_for_args(kwarg_val, args_name, fields);
            }
        }
        _ => walk_expr_for_args_collections(expr, args_name, fields),
    }
}

fn walk_expr_for_args_collections(
    expr: &HirExpr,
    args_name: &str,
    fields: &mut std::collections::HashSet<String>,
) {
    match expr {
        HirExpr::List(elems) | HirExpr::Tuple(elems) | HirExpr::Set(elems) => {
            for elem in elems {
                walk_expr_for_args(elem, args_name, fields);
            }
        }
        HirExpr::Dict(items) => {
            for (key, value) in items {
                walk_expr_for_args(key, args_name, fields);
                walk_expr_for_args(value, args_name, fields);
            }
        }
        HirExpr::Index { base, index } => {
            walk_expr_for_args(base, args_name, fields);
            walk_expr_for_args(index, args_name, fields);
        }
        HirExpr::IfExpr { test, body, orelse } => {
            walk_expr_for_args(test, args_name, fields);
            walk_expr_for_args(body, args_name, fields);
            walk_expr_for_args(orelse, args_name, fields);
        }
        HirExpr::FString { parts } => {
            for part in parts {
                if let crate::hir::FStringPart::Expr(fstring_expr) = part {
                    walk_expr_for_args(fstring_expr, args_name, fields);
                }
            }
        }
        HirExpr::Slice { base, start, stop, step } => {
            walk_expr_for_args(base, args_name, fields);
            if let Some(s) = start {
                walk_expr_for_args(s, args_name, fields);
            }
            if let Some(s) = stop {
                walk_expr_for_args(s, args_name, fields);
            }
            if let Some(s) = step {
                walk_expr_for_args(s, args_name, fields);
            }
        }
        HirExpr::ListComp { element, generators }
        | HirExpr::SetComp { element, generators } => {
            walk_expr_for_args(element, args_name, fields);
            for gen in generators {
                walk_expr_for_args(&gen.iter, args_name, fields);
                for cond in &gen.conditions {
                    walk_expr_for_args(cond, args_name, fields);
                }
            }
        }
        HirExpr::DictComp { key, value, generators } => {
            walk_expr_for_args(key, args_name, fields);
            walk_expr_for_args(value, args_name, fields);
            for gen in generators {
                walk_expr_for_args(&gen.iter, args_name, fields);
                for cond in &gen.conditions {
                    walk_expr_for_args(cond, args_name, fields);
                }
            }
        }
        HirExpr::Lambda { body, .. } => {
            walk_expr_for_args(body, args_name, fields);
        }
        HirExpr::Borrow { expr: borrow_expr, .. } => {
            walk_expr_for_args(borrow_expr, args_name, fields);
        }
        HirExpr::Yield { value: Some(v) } => {
            walk_expr_for_args(v, args_name, fields);
        }
        HirExpr::Yield { value: None } => {}
        HirExpr::Await { value } => {
            walk_expr_for_args(value, args_name, fields);
        }
        _ => {}
    }
}

fn walk_stmt_for_args(stmt: &HirStmt, args_name: &str, fields: &mut std::collections::HashSet<String>) {
    match stmt {
        HirStmt::Expr(expr) => walk_expr_for_args(expr, args_name, fields),
        HirStmt::Assign { value, .. } => walk_expr_for_args(value, args_name, fields),
        HirStmt::Return(Some(expr)) => walk_expr_for_args(expr, args_name, fields),
        HirStmt::If { condition, then_body, else_body } => {
            walk_expr_for_args(condition, args_name, fields);
            for s in then_body {
                walk_stmt_for_args(s, args_name, fields);
            }
            if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    walk_stmt_for_args(s, args_name, fields);
                }
            }
        }
        HirStmt::While { condition, body } => {
            walk_expr_for_args(condition, args_name, fields);
            for s in body {
                walk_stmt_for_args(s, args_name, fields);
            }
        }
        HirStmt::For { iter, body, .. } => {
            walk_expr_for_args(iter, args_name, fields);
            for s in body {
                walk_stmt_for_args(s, args_name, fields);
            }
        }
        HirStmt::With { context, body, .. } => {
            walk_expr_for_args(context, args_name, fields);
            for s in body {
                walk_stmt_for_args(s, args_name, fields);
            }
        }
        HirStmt::Try { body, handlers, orelse, finalbody } => {
            for s in body {
                walk_stmt_for_args(s, args_name, fields);
            }
            for handler in handlers {
                for s in &handler.body {
                    walk_stmt_for_args(s, args_name, fields);
                }
            }
            if let Some(else_stmts) = orelse {
                for s in else_stmts {
                    walk_stmt_for_args(s, args_name, fields);
                }
            }
            if let Some(final_stmts) = finalbody {
                for s in final_stmts {
                    walk_stmt_for_args(s, args_name, fields);
                }
            }
        }
        HirStmt::FunctionDef { body, .. } => {
            for s in body {
                walk_stmt_for_args(s, args_name, fields);
            }
        }
        _ => {}
    }
}
