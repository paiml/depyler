fn is_string_key_name_heuristic(name_str: &str) -> bool {
    name_str == "key"
        || name_str == "k"
        || name_str == "name"
        || name_str == "id"
        || name_str == "word"
        || name_str == "text"
        || name_str == "char"
        || name_str == "character"
        || name_str == "c"
        || name_str.ends_with("_key")
        || name_str.ends_with("_name")
}

fn infer_numeric_from_index_heuristic(index: &HirExpr, ctx: &CodeGenContext) -> bool {
    match index {
        HirExpr::Var(name) => {
            if let Some(idx_type) = ctx.var_types.get(name) {
                matches!(idx_type, Type::Int | Type::Float | Type::Bool)
            } else {
                !is_string_key_name_heuristic(name.as_str())
            }
        }
        HirExpr::Binary { .. } | HirExpr::Literal(crate::hir::Literal::Int(_)) => true,
        _ => false,
    }
}

fn resolve_numeric_index_for_typed_base(
    base_type: &Type,
    index: &HirExpr,
    ctx: &CodeGenContext,
) -> bool {
    match base_type {
        Type::List(_) => true,
        Type::Dict(_, _) => false,
        _ => infer_numeric_from_index_heuristic(index, ctx),
    }
}

fn resolve_numeric_index_for_nested_base(
    base: &HirExpr,
    index: &HirExpr,
    ctx: &CodeGenContext,
) -> bool {
    let root_name = find_root_var_in_chain(base);
    if let Some(ref name) = root_name {
        if let Some(root_type) = ctx.var_types.get(name) {
            let mut current = root_type.clone();
            let depth = {
                let mut d = 0;
                let mut tmp = base;
                while let HirExpr::Index { base: inner, .. } = tmp {
                    d += 1;
                    tmp = inner;
                }
                d
            };
            for _ in 0..depth {
                match current {
                    Type::List(elem) => current = *elem,
                    Type::Dict(_, val) => current = *val,
                    _ => break,
                }
            }
            matches!(current, Type::List(_))
        } else {
            matches!(
                index,
                HirExpr::Binary { .. } | HirExpr::Literal(crate::hir::Literal::Int(_))
            )
        }
    } else {
        match index {
            HirExpr::Var(name) => !is_string_key_name_heuristic(name.as_str()),
            HirExpr::Binary { .. } | HirExpr::Literal(crate::hir::Literal::Int(_)) => true,
            _ => false,
        }
    }
}

fn resolve_is_numeric_index(
    base: &HirExpr,
    index: &HirExpr,
    ctx: &CodeGenContext,
) -> bool {
    if let HirExpr::Var(base_name) = base {
        if let Some(base_type) = ctx.var_types.get(base_name) {
            resolve_numeric_index_for_typed_base(base_type, index, ctx)
        } else {
            infer_numeric_from_index_heuristic(index, ctx)
        }
    } else {
        resolve_numeric_index_for_nested_base(base, index, ctx)
    }
}

fn check_json_name_heuristic(name_str: &str) -> (bool, bool) {
    let is_value_name = name_str == "config"
        || name_str == "data"
        || name_str == "value"
        || name_str == "current"
        || name_str == "obj"
        || name_str == "json";
    let is_dict_value_name = name_str == "info"
        || name_str == "result"
        || name_str == "stats"
        || name_str == "metadata"
        || name_str == "output"
        || name_str == "response";
    (is_value_name, is_dict_value_name)
}

fn detect_json_assign_context(
    base: &HirExpr,
    is_numeric_index: bool,
    ctx: &CodeGenContext,
) -> (bool, bool) {
    if ctx.type_mapper.nasa_mode {
        return (false, false);
    }
    if let HirExpr::Var(base_name) = base {
        if !is_numeric_index {
            if let Some(base_type) = ctx.var_types.get(base_name) {
                match base_type {
                    Type::Custom(s) if s == "serde_json::Value" || s == "Value" => (true, false),
                    Type::Dict(_, val_type) => {
                        let val_is_json = match val_type.as_ref() {
                            Type::Unknown => true,
                            Type::Custom(s) => s == "serde_json::Value" || s == "Value",
                            _ => false,
                        };
                        (val_is_json, val_is_json)
                    }
                    Type::Unknown => check_json_name_heuristic(base_name.as_str()),
                    _ => check_json_name_heuristic(base_name.as_str()),
                }
            } else {
                check_json_name_heuristic(base_name.as_str())
            }
        } else {
            (false, false)
        }
    } else {
        (false, false)
    }
}
