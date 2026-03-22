pub(crate) fn propagate_return_type_to_vars(
    stmts: &[HirStmt],
    var_types: &mut std::collections::HashMap<String, Type>,
    return_type: &Type,
) {
    // Only propagate if we have a concrete return type
    if matches!(return_type, Type::Unknown | Type::None) {
        return;
    }

    // DEPYLER-1160: First, collect all returned variable names
    // This enables the "Short Circuit" heuristic: when we see `result = []`
    // and result is returned, we infer its type from the return type
    let returned_vars = collect_returned_var_names(stmts);

    propagate_return_type_impl(stmts, var_types, return_type, &returned_vars);
}

fn propagate_return_type_impl(
    stmts: &[HirStmt],
    var_types: &mut std::collections::HashMap<String, Type>,
    return_type: &Type,
    returned_vars: &std::collections::HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            // DEPYLER-1160: Target-typed inference for empty list assignments
            // When `result = []` is assigned and `result` is returned from a function
            // with return type List[T], infer result's type as List[T]
            // DEPYLER-1164: Extended to empty dict assignments
            HirStmt::Assign { target: AssignTarget::Symbol(name), value, .. } => {
                if let HirExpr::List(elements) = value {
                    if elements.is_empty() && returned_vars.contains(name) {
                        // Only propagate if return type is a List with concrete element type
                        if let Type::List(_elem_type) = return_type {
                            // Don't override if we already have a concrete type
                            let should_update = match var_types.get(name) {
                                None => true,
                                Some(Type::Unknown) => true,
                                Some(Type::List(inner))
                                    if matches!(inner.as_ref(), Type::Unknown) =>
                                {
                                    true
                                }
                                _ => false,
                            };
                            if should_update {
                                var_types.insert(name.clone(), return_type.clone());
                            }
                        }
                    }
                }
                // DEPYLER-1164: Target-typed inference for empty dict assignments
                // When `result = {}` is assigned and `result` is returned from a function
                // with return type Dict[K, V], infer result's type as Dict[K, V]
                if let HirExpr::Dict(items) = value {
                    if items.is_empty() && returned_vars.contains(name) {
                        // Only propagate if return type is a Dict with concrete key/value types
                        if let Type::Dict(_key_type, _val_type) = return_type {
                            // Don't override if we already have a concrete type
                            let should_update = match var_types.get(name) {
                                None => true,
                                Some(Type::Unknown) => true,
                                Some(Type::Dict(k, v))
                                    if matches!(k.as_ref(), Type::Unknown)
                                        || matches!(v.as_ref(), Type::Unknown) =>
                                {
                                    true
                                }
                                _ => false,
                            };
                            if should_update {
                                var_types.insert(name.clone(), return_type.clone());
                            }
                        }
                    }
                }
            }
            HirStmt::Return(Some(HirExpr::Var(var_name))) => {
                // If returning a simple variable, propagate return type to it
                // Check if the variable has an Unknown or weaker type
                let should_update = match var_types.get(var_name) {
                    None => true,
                    Some(Type::Unknown) => true,
                    Some(Type::List(elem)) if matches!(elem.as_ref(), Type::Unknown) => true,
                    Some(Type::Dict(k, v))
                        if matches!(k.as_ref(), Type::Unknown)
                            || matches!(v.as_ref(), Type::Unknown) =>
                    {
                        true
                    }
                    _ => false,
                };
                if should_update {
                    var_types.insert(var_name.clone(), return_type.clone());
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                propagate_return_type_impl(then_body, var_types, return_type, returned_vars);
                if let Some(else_stmts) = else_body {
                    propagate_return_type_impl(else_stmts, var_types, return_type, returned_vars);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                propagate_return_type_impl(body, var_types, return_type, returned_vars);
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                propagate_return_type_impl(body, var_types, return_type, returned_vars);
                for handler in handlers {
                    propagate_return_type_impl(
                        &handler.body,
                        var_types,
                        return_type,
                        returned_vars,
                    );
                }
                if let Some(orelse_stmts) = orelse {
                    propagate_return_type_impl(orelse_stmts, var_types, return_type, returned_vars);
                }
                if let Some(finally_stmts) = finalbody {
                    propagate_return_type_impl(
                        finally_stmts,
                        var_types,
                        return_type,
                        returned_vars,
                    );
                }
            }
            HirStmt::With { body, .. } => {
                propagate_return_type_impl(body, var_types, return_type, returned_vars);
            }
            _ => {}
        }
    }
}
