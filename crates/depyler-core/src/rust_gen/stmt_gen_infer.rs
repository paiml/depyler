pub(crate) fn infer_try_body_return_type(body: &[HirStmt], ctx: &CodeGenContext) -> Option<Type> {
    for stmt in body {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                // Found a return with a value - infer its type
                // DEPYLER-1157: If inference returns Unknown, fall back to function's return type
                // This handles cases like `return json.loads(...)` where the method isn't recognized
                let inferred = infer_expr_return_type(expr, ctx);
                if matches!(inferred, Type::Unknown) {
                    // Use function's annotated return type if available
                    if let Some(ret_type) = ctx.current_return_type.as_ref() {
                        return Some(ret_type.clone());
                    }
                }
                return Some(inferred);
            }
            HirStmt::While { body: inner, .. } | HirStmt::For { body: inner, .. } => {
                // Check inside loops
                if let Some(ty) = infer_try_body_return_type(inner, ctx) {
                    return Some(ty);
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                // Check inside if/else
                if let Some(ty) = infer_try_body_return_type(then_body, ctx) {
                    return Some(ty);
                }
                if let Some(else_stmts) = else_body {
                    if let Some(ty) = infer_try_body_return_type(else_stmts, ctx) {
                        return Some(ty);
                    }
                }
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                if let Some(ty) = infer_try_body_return_type(body, ctx) {
                    return Some(ty);
                }
                for h in handlers {
                    if let Some(ty) = infer_try_body_return_type(&h.body, ctx) {
                        return Some(ty);
                    }
                }
                if let Some(else_stmts) = orelse {
                    if let Some(ty) = infer_try_body_return_type(else_stmts, ctx) {
                        return Some(ty);
                    }
                }
                if let Some(final_stmts) = finalbody {
                    if let Some(ty) = infer_try_body_return_type(final_stmts, ctx) {
                        return Some(ty);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn infer_expr_return_type(expr: &HirExpr, ctx: &CodeGenContext) -> Type {
    match expr {
        HirExpr::Var(name) => {
            // Look up variable type in context
            ctx.var_types.get(name).cloned().unwrap_or(Type::Unknown)
        }
        HirExpr::Literal(lit) => match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Bool(_) => Type::Bool,
            Literal::None => Type::None,
            Literal::Bytes(_) => Type::Custom("bytes".to_string()),
        },
        HirExpr::MethodCall { method, .. } => {
            // Common method return types
            match method.as_str() {
                "hexdigest" | "encode" | "decode" | "strip" | "upper" | "lower" | "to_string" => {
                    Type::String
                }
                "len" | "count" | "wait" | "poll" | "returncode" => Type::Int,
                "is_empty" | "startswith" | "endswith" | "exists" | "is_file" | "is_dir" => {
                    Type::Bool
                }
                _ => Type::Unknown,
            }
        }
        HirExpr::Attribute { attr, .. } => match attr.as_str() {
            "returncode" => Type::Int,
            "stdout" | "stderr" => Type::String,
            _ => Type::Unknown,
        },
        HirExpr::Call { func, .. } => {
            // Common function return types
            match func.as_str() {
                "str" | "format" | "hex::encode" => Type::String,
                "int" | "len" => Type::Int,
                "float" => Type::Float,
                "bool" => Type::Bool,
                _ => Type::Unknown,
            }
        }
        HirExpr::Binary { left, right, .. } => {
            let left_type = infer_expr_return_type(left, ctx);
            if !matches!(left_type, Type::Unknown) {
                left_type
            } else {
                infer_expr_return_type(right, ctx)
            }
        }
        _ => Type::Unknown,
    }
}
