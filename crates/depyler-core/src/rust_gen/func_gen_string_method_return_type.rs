enum StringMethodReturnType {
    /// Returns owned String (e.g., upper, lower, strip, replace)
    Owned,
    /// Returns borrowed &str or bool (e.g., starts_with, is_digit)
    Borrowed,
}

fn classify_string_method(method_name: &str) -> StringMethodReturnType {
    match method_name {
        // Transformation methods that return owned String
        "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "replace" | "format" | "title"
        | "capitalize" | "swapcase" | "expandtabs" | "center" | "ljust" | "rjust" | "zfill" => {
            StringMethodReturnType::Owned
        }

        // Query/test methods that return bool or &str (borrowed)
        "startswith" | "endswith" | "isalpha" | "isdigit" | "isalnum" | "isspace" | "islower"
        | "isupper" | "istitle" | "isascii" | "isprintable" | "find" | "rfind" | "index"
        | "rindex" | "count" => StringMethodReturnType::Borrowed,

        // Default: assume owned to be safe
        _ => StringMethodReturnType::Owned,
    }
}

fn contains_owned_string_method(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::MethodCall { method, .. } => {
            // Check if this method returns owned String
            classify_string_method(method) == StringMethodReturnType::Owned
        }
        HirExpr::Binary { left, right, .. } => {
            // Check both sides of binary operations
            contains_owned_string_method(left) || contains_owned_string_method(right)
        }
        HirExpr::Unary { operand, .. } => contains_owned_string_method(operand),
        HirExpr::IfExpr { body, orelse, .. } => {
            // Check both branches of conditional
            contains_owned_string_method(body) || contains_owned_string_method(orelse)
        }
        // DEPYLER-0598: String literals get .to_string() in codegen, so they're owned
        HirExpr::Literal(crate::hir::Literal::String(_)) => true,
        // F-strings generate format!() which returns owned String
        HirExpr::FString { .. } => true,
        HirExpr::Call { .. }
        | HirExpr::Var(_)
        | HirExpr::Literal(_) // Non-string literals
        | HirExpr::List(_)
        | HirExpr::Dict(_)
        | HirExpr::Tuple(_)
        | HirExpr::Set(_)
        | HirExpr::FrozenSet(_)
        | HirExpr::Index { .. }
        | HirExpr::Slice { .. }
        | HirExpr::Attribute { .. }
        | HirExpr::Borrow { .. }
        | HirExpr::ListComp { .. }
        | HirExpr::SetComp { .. }
        | HirExpr::DictComp { .. }
        | HirExpr::Lambda { .. }
        | HirExpr::Await { .. }
        | HirExpr::Yield { .. }
        | HirExpr::SortByKey { .. }
        | HirExpr::GeneratorExp { .. }
        | HirExpr::NamedExpr { .. }
        | HirExpr::DynamicCall { .. } => false,
    }
}

pub(crate) fn function_returns_owned_string(func: &HirFunction) -> bool {
    // Recursively check all return statements in the function body
    stmt_block_returns_owned_string(&func.body)
}

fn stmt_block_returns_owned_string(stmts: &[HirStmt]) -> bool {
    for stmt in stmts {
        if stmt_returns_owned_string(stmt) {
            return true;
        }
    }
    false
}

fn stmt_returns_owned_string(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Return(Some(expr)) => contains_owned_string_method(expr),
        HirStmt::If { then_body, else_body, .. } => {
            stmt_block_returns_owned_string(then_body)
                || else_body.as_ref().is_some_and(|body| stmt_block_returns_owned_string(body))
        }
        HirStmt::While { body, .. } => stmt_block_returns_owned_string(body),
        HirStmt::For { body, .. } => stmt_block_returns_owned_string(body),
        HirStmt::With { body, .. } => stmt_block_returns_owned_string(body),
        HirStmt::Try { body, handlers, orelse, finalbody } => {
            stmt_block_returns_owned_string(body)
                || handlers.iter().any(|h| stmt_block_returns_owned_string(&h.body))
                || orelse.as_ref().is_some_and(|body| stmt_block_returns_owned_string(body))
                || finalbody.as_ref().is_some_and(|body| stmt_block_returns_owned_string(body))
        }
        _ => false,
    }
}
