fn infer_return_type_from_body(body: &[HirStmt]) -> Option<Type> {
    // DEPYLER-0415: Build type environment from variable assignments
    let mut var_types: std::collections::HashMap<String, Type> = std::collections::HashMap::new();
    build_var_type_env(body, &mut var_types);

    let mut return_types = Vec::new();
    collect_return_types_with_env(body, &mut return_types, &var_types);

    // DEPYLER-1084: Do NOT infer return type from trailing expressions
    // Python does NOT have implicit returns like Rust - expression statements
    // just evaluate and discard their value. Only explicit `return x` statements
    // contribute to the return type.
    //
    // Previous DEPYLER-0412 incorrectly treated `x + y` as an implicit return,
    // causing functions like `def compute(): x = 10; y = 20; x + y` to have
    // inferred return type i32 instead of () (None).
    //
    // Explicit returns are already collected by collect_return_types_with_env() above.

    if return_types.is_empty() {
        return None;
    }

    // If all return types are the same (ignoring Unknown), use that type
    let first_known = return_types.iter().find(|t| !matches!(t, Type::Unknown));
    if let Some(first) = first_known {
        if return_types.iter().all(|t| matches!(t, Type::Unknown) || t == first) {
            return Some(first.clone());
        }
    }

    // DEPYLER-0448: Do NOT default Unknown to Int - this causes dict/list/Value returns
    // to be incorrectly typed as i32. Instead, return None and let the type mapper
    // handle the fallback (which will use serde_json::Value for complex types).
    //
    // Previous behavior (DEPYLER-0422): Defaulted Unknown → Int for lambda returns
    // Problem: This also affected dict/list returns, causing E0308 errors
    // New behavior: Return None for Unknown types, allowing proper Value fallback
    if return_types.iter().all(|t| matches!(t, Type::Unknown)) {
        // We have return statements but all returned Unknown types
        // Don't assume Int - let type mapper decide the appropriate fallback
        return None;
    }

    // Mixed types - return the first known type
    first_known.cloned()
}

pub(crate) fn collect_return_types_with_env(
    stmts: &[HirStmt],
    types: &mut Vec<Type>,
    var_types: &std::collections::HashMap<String, Type>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                types.push(infer_expr_type_with_env(expr, var_types));
            }
            HirStmt::Return(None) => {
                types.push(Type::None);
            }
            HirStmt::If { then_body, else_body, .. } => {
                collect_return_types_with_env(then_body, types, var_types);
                if let Some(else_stmts) = else_body {
                    collect_return_types_with_env(else_stmts, types, var_types);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                collect_return_types_with_env(body, types, var_types);
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                collect_return_types_with_env(body, types, var_types);
                for handler in handlers {
                    collect_return_types_with_env(&handler.body, types, var_types);
                }
                if let Some(orelse_stmts) = orelse {
                    collect_return_types_with_env(orelse_stmts, types, var_types);
                }
                if let Some(finally_stmts) = finalbody {
                    collect_return_types_with_env(finally_stmts, types, var_types);
                }
            }
            HirStmt::With { body, .. } => {
                collect_return_types_with_env(body, types, var_types);
            }
            _ => {}
        }
    }
}

fn collect_returned_var_names(stmts: &[HirStmt]) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    collect_returned_var_names_impl(stmts, &mut names);
    names
}

fn collect_returned_var_names_impl(
    stmts: &[HirStmt],
    names: &mut std::collections::HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(HirExpr::Var(name))) => {
                names.insert(name.clone());
            }
            HirStmt::If { then_body, else_body, .. } => {
                collect_returned_var_names_impl(then_body, names);
                if let Some(else_stmts) = else_body {
                    collect_returned_var_names_impl(else_stmts, names);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                collect_returned_var_names_impl(body, names);
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                collect_returned_var_names_impl(body, names);
                for handler in handlers {
                    collect_returned_var_names_impl(&handler.body, names);
                }
                if let Some(orelse_stmts) = orelse {
                    collect_returned_var_names_impl(orelse_stmts, names);
                }
                if let Some(finally_stmts) = finalbody {
                    collect_returned_var_names_impl(finally_stmts, names);
                }
            }
            HirStmt::With { body, .. } => {
                collect_returned_var_names_impl(body, names);
            }
            _ => {}
        }
    }
}
