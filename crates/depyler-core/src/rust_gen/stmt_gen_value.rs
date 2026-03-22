fn is_json_value_method_chain_or_fallback(expr: &HirExpr, ctx: &CodeGenContext) -> bool {
    match expr {
        // Reached the base: check if it's a dict.get() on Value-containing HashMap
        HirExpr::MethodCall { object, method, .. } if method == "get" => {
            if let HirExpr::Var(var_name) = object.as_ref() {
                if let Some(t) = ctx.var_types.get(var_name) {
                    // DEPYLER-0607: A dict with Unknown value type maps to HashMap<_, serde_json::Value>
                    // So we should treat Unknown as potentially Value
                    matches!(t, Type::Dict(_, v) if matches!(v.as_ref(),
                        Type::Custom(n) if n.contains("Value") || n.contains("json"))
                        || matches!(v.as_ref(), Type::Unknown))
                } else {
                    // DEPYLER-0607: Fallback for untracked local dicts
                    ctx.needs_serde_json
                }
            } else {
                false
            }
        }
        // Continue traversing the chain
        HirExpr::MethodCall { object, method, .. } => {
            let is_chain_method = method == "cloned"
                || method == "unwrap_or_default"
                || method == "unwrap_or"
                || method == "unwrap";
            if is_chain_method {
                is_json_value_method_chain_or_fallback(object.as_ref(), ctx)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn detect_json_value_iteration(iter: &HirExpr, ctx: &CodeGenContext) -> bool {
    match iter {
        HirExpr::Index { base, .. } => match base.as_ref() {
            HirExpr::Var(var_name) => {
                if let Some(t) = ctx.var_types.get(var_name) {
                    is_dict_with_value_type(t)
                } else {
                    ctx.needs_serde_json
                }
            }
            HirExpr::Dict { .. } => true,
            _ => false,
        },
        HirExpr::MethodCall { object, method, .. } => {
            let is_value_chain = method == "cloned"
                || method == "unwrap_or_default"
                || method == "unwrap_or"
                || method == "unwrap";
            if is_value_chain {
                is_json_value_method_chain_or_fallback(object.as_ref(), ctx)
            } else if method == "get" {
                if let HirExpr::Var(var_name) = object.as_ref() {
                    if let Some(t) = ctx.var_types.get(var_name) {
                        is_dict_with_value_type(t)
                    } else {
                        ctx.needs_serde_json
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
