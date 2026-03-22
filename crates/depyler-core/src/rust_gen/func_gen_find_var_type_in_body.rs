fn find_var_type_in_body(var_name: &str, stmts: &[HirStmt]) -> Option<Type> {
    find_var_type_in_body_with_params(var_name, stmts, &std::collections::HashMap::new())
}

fn find_var_type_in_body_with_params(
    var_name: &str,
    stmts: &[HirStmt],
    param_types: &std::collections::HashMap<String, Type>,
) -> Option<Type> {
    // DEPYLER-0965: Collect all types from assignments to this variable
    // Return the first non-Unknown type found
    for stmt in stmts {
        match stmt {
            // Simple assignment: x = expr
            HirStmt::Assign { target: AssignTarget::Symbol(name), type_annotation, value }
                if name == var_name =>
            {
                // First try explicit type annotation
                if type_annotation.is_some() {
                    return type_annotation.clone();
                }
                // If assigned from a variable, check if it's a known parameter
                if let HirExpr::Var(source_var) = value {
                    if let Some(ty) = param_types.get(source_var) {
                        return Some(ty.clone());
                    }
                }
                // DEPYLER-0265: If assigned from subscript (e.g., longest = words[0]),
                // infer element type from the base variable's type
                if let HirExpr::Index { base, .. } = value {
                    if let HirExpr::Var(base_var) = base.as_ref() {
                        if let Some(base_type) = param_types.get(base_var) {
                            let elem_type = match base_type {
                                Type::List(elem) => Some(*elem.clone()),
                                Type::Dict(_, val) => Some(*val.clone()),
                                Type::String => Some(Type::String),
                                Type::Tuple(elems) => elems.first().cloned(),
                                _ => None,
                            };
                            if let Some(ty) = elem_type {
                                if !matches!(ty, Type::Unknown) {
                                    return Some(ty);
                                }
                            }
                        }
                    }
                }
                // If no annotation, infer from the assigned value
                let inferred = infer_expr_type_simple(value);
                // DEPYLER-0965: Skip Unknown and None types - continue looking for concrete types
                // When variable is first assigned None (Type::None) but later assigned a string,
                // we want to infer String, not None (which maps to () in Rust)
                if !matches!(inferred, Type::Unknown | Type::None) {
                    return Some(inferred);
                }
                // DEPYLER-0965: Don't return None here - continue looking for more assignments
                // The first assignment might be `x = None` but later ones might have concrete types
            }
            // Tuple unpacking: (a, b) = (1, 2)
            HirStmt::Assign { target: AssignTarget::Tuple(targets), value, .. } => {
                // Find var_name position in the targets
                if let Some(pos) = targets
                    .iter()
                    .position(|t| matches!(t, AssignTarget::Symbol(name) if name == var_name))
                {
                    // Check if RHS is a tuple expression
                    if let HirExpr::Tuple(elems) = value {
                        if pos < elems.len() {
                            let elem_type = infer_expr_type_simple(&elems[pos]);
                            if !matches!(elem_type, Type::Unknown) {
                                return Some(elem_type);
                            }
                        }
                    }
                    // Infer type from the RHS tuple type
                    let rhs_type = infer_expr_type_simple(value);
                    if let Type::Tuple(elem_types) = rhs_type {
                        if pos < elem_types.len() {
                            let elem_type = elem_types[pos].clone();
                            if !matches!(elem_type, Type::Unknown) {
                                return Some(elem_type);
                            }
                        }
                    }
                }
            }
            // Recurse into for loops
            HirStmt::For { body, .. } => {
                if let Some(ty) = find_var_type_in_body_with_params(var_name, body, param_types) {
                    return Some(ty);
                }
            }
            // Recurse into while loops
            HirStmt::While { body, .. } => {
                if let Some(ty) = find_var_type_in_body_with_params(var_name, body, param_types) {
                    return Some(ty);
                }
            }
            // Recurse into if/else blocks
            HirStmt::If { then_body, else_body, .. } => {
                if let Some(ty) =
                    find_var_type_in_body_with_params(var_name, then_body, param_types)
                {
                    return Some(ty);
                }
                if let Some(else_stmts) = else_body {
                    if let Some(ty) =
                        find_var_type_in_body_with_params(var_name, else_stmts, param_types)
                    {
                        return Some(ty);
                    }
                }
            }
            // Recurse into try/except blocks
            HirStmt::Try { body, handlers, finalbody, .. } => {
                if let Some(ty) = find_var_type_in_body_with_params(var_name, body, param_types) {
                    return Some(ty);
                }
                for handler in handlers {
                    if let Some(ty) =
                        find_var_type_in_body_with_params(var_name, &handler.body, param_types)
                    {
                        return Some(ty);
                    }
                }
                if let Some(finally) = finalbody {
                    if let Some(ty) =
                        find_var_type_in_body_with_params(var_name, finally, param_types)
                    {
                        return Some(ty);
                    }
                }
            }
            _ => {}
        }
    }
    None
}
