pub fn method_mutates_self(method: &HirMethod) -> bool {
    for stmt in &method.body {
        if stmt_mutates_self(stmt) {
            return true;
        }
    }
    false
}

fn stmt_mutates_self(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, .. } => {
            // Check if target is self.field assignment
            matches!(target, AssignTarget::Attribute { value, .. }
                if matches!(value.as_ref(), HirExpr::Var(sym) if sym.as_str() == "self"))
        }
        // DEPYLER-1008: Check for method calls that mutate self.field
        // e.g., self.messages.append(msg) -> self.messages is mutated
        HirStmt::Expr(expr) => expr_mutates_self(expr),
        // DEPYLER-1152: Check for mutations in return statements
        // e.g., return self._items.pop() - the pop() mutates self._items
        HirStmt::Return(Some(expr)) => expr_mutates_self(expr),
        HirStmt::If { then_body, else_body, .. } => {
            then_body.iter().any(stmt_mutates_self)
                || else_body.as_ref().is_some_and(|body| body.iter().any(stmt_mutates_self))
        }
        HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
            body.iter().any(stmt_mutates_self)
        }
        _ => false,
    }
}

fn expr_mutates_self(expr: &HirExpr) -> bool {
    if let HirExpr::MethodCall { object, method, .. } = expr {
        // Check if method is a mutating method
        let is_mutating = matches!(
            method.as_str(),
            "append"
                | "push"
                | "push_back"
                | "push_front"
                | "appendleft"
                | "popleft"
                | "pop"
                | "insert"
                | "remove"
                | "clear"
                | "extend"
                | "add"
                | "update"
                | "discard"
        );
        if is_mutating {
            // Check if object is self.field
            if let HirExpr::Attribute { value, .. } = object.as_ref() {
                if let HirExpr::Var(name) = value.as_ref() {
                    return name == "self";
                }
            }
        }
    }
    false
}
