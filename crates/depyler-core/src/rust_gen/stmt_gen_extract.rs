fn extract_var_element_type(var_name: &str, ctx: &CodeGenContext) -> Option<Type> {
    ctx.var_types.get(var_name).and_then(|t| match t {
        Type::List(elem_t) => Some(*elem_t.clone()),
        Type::Set(elem_t) => Some(*elem_t.clone()),
        Type::Dict(key_t, _) => Some(*key_t.clone()),
        _ => None,
    })
}

fn extract_self_field_element_type(attr: &str, ctx: &CodeGenContext) -> Option<Type> {
    ctx.class_field_types.get(attr).and_then(|t| match t {
        Type::List(elem_t) => Some(*elem_t.clone()),
        Type::Set(elem_t) => Some(*elem_t.clone()),
        Type::Dict(key_t, _) => Some(*key_t.clone()),
        _ => None,
    })
}

fn extract_enumerate_element_type(arg: &HirExpr, ctx: &CodeGenContext) -> Option<Type> {
    match arg {
        HirExpr::Var(var_name) => ctx.var_types.get(var_name).and_then(|t| match t {
            Type::List(elem_t) => Some(Type::Tuple(vec![Type::Int, *elem_t.clone()])),
            Type::Set(elem_t) => Some(Type::Tuple(vec![Type::Int, *elem_t.clone()])),
            _ => None,
        }),
        HirExpr::Attribute { value, attr, .. } => {
            if matches!(value.as_ref(), HirExpr::Var(name) if name == "self") {
                ctx.class_field_types.get(attr).and_then(|t| match t {
                    Type::List(elem_t) => Some(Type::Tuple(vec![Type::Int, *elem_t.clone()])),
                    Type::Set(elem_t) => Some(Type::Tuple(vec![Type::Int, *elem_t.clone()])),
                    _ => None,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn extract_glob_element_type(object: &HirExpr, ctx: &CodeGenContext) -> Option<Type> {
    if let HirExpr::Var(var_name) = object {
        let is_path = ctx
            .var_types
            .get(var_name)
            .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
            .unwrap_or(false);
        if is_path {
            return Some(Type::Custom("PathBuf".to_string()));
        }
    }
    None
}

fn extract_iterator_element_type(iter: &HirExpr, ctx: &CodeGenContext) -> Option<Type> {
    match iter {
        HirExpr::Var(var_name) => extract_var_element_type(var_name, ctx),
        HirExpr::Attribute { value, attr, .. } => {
            if matches!(value.as_ref(), HirExpr::Var(name) if name == "self") {
                extract_self_field_element_type(attr, ctx)
            } else {
                None
            }
        }
        HirExpr::Call { func, args, .. } if func == "enumerate" => {
            args.first().and_then(|arg| extract_enumerate_element_type(arg, ctx))
        }
        HirExpr::MethodCall { object, method, .. } if method == "glob" => {
            extract_glob_element_type(object.as_ref(), ctx)
        }
        // DEPYLER-99MODE-S9: Handle subscript iteration (e.g., `for ik in trie[k]:`)
        // Walk the Index chain to root variable, resolve through Dict/List value types,
        // then extract element type from the result type.
        HirExpr::Index { base, .. } => {
            // Walk to root variable
            let mut current: &HirExpr = base;
            while let HirExpr::Index { base: inner, .. } = current {
                current = inner;
            }
            if let HirExpr::Var(root_name) = current {
                let mut cur_type = ctx.var_types.get(root_name)?.clone();
                // Peel one level for the subscript
                match cur_type {
                    Type::Dict(_, val) => cur_type = *val,
                    Type::List(elem) => cur_type = *elem,
                    _ => return None,
                }
                // Now extract element type from the resolved type
                match cur_type {
                    Type::List(elem) => Some(*elem),
                    Type::Set(elem) => Some(*elem),
                    Type::Dict(key, _) => Some(*key),
                    Type::String => Some(Type::String),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}
