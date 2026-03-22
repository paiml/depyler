pub(crate) fn build_var_type_env_full(
    stmts: &[HirStmt],
    var_types: &mut std::collections::HashMap<String, Type>,
    function_return_types: &std::collections::HashMap<String, Type>,
    class_method_return_types: &std::collections::HashMap<(String, String), Type>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol(name),
                value,
                type_annotation,
            } => {
                // DEPYLER-0714: Prefer explicit type annotation over inferred type
                // For `data: Dict[str, int] = {}`, use Dict(String, Int) not Dict(Unknown, Unknown)
                let value_type = if let Some(annot) = type_annotation {
                    annot.clone()
                } else {
                    // DEPYLER-1007: Check if this is a class constructor call
                    // e.g., `p = Point(3, 4)` → p should have type Type::Custom("Point")
                    if let HirExpr::Call { func, .. } = value {
                        if let Some(ctor_type) = function_return_types.get(func) {
                            ctor_type.clone()
                        } else {
                            // Use class-aware type inference for method calls
                            infer_expr_type_with_class_methods(
                                value,
                                var_types,
                                class_method_return_types,
                            )
                        }
                    } else {
                        // DEPYLER-1007: Use class-aware type inference for method calls
                        // e.g., `dist_sq = p.distance_squared()` → dist_sq should have Int type
                        infer_expr_type_with_class_methods(
                            value,
                            var_types,
                            class_method_return_types,
                        )
                    }
                };
                if !matches!(value_type, Type::Unknown) {
                    // DEPYLER-99MODE-S9: Don't overwrite concrete param/annotation types
                    // with structurally different inferred types (e.g., String → List(Int)
                    // from `prefix = prefix[:-1]` where HM mis-infers the slice type)
                    let should_insert = match var_types.get(name) {
                        None | Some(Type::Unknown) => true,
                        Some(existing) => {
                            std::mem::discriminant(existing) == std::mem::discriminant(&value_type)
                        }
                    };
                    if should_insert {
                        var_types.insert(name.clone(), value_type);
                    }
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                build_var_type_env_full(
                    then_body,
                    var_types,
                    function_return_types,
                    class_method_return_types,
                );
                if let Some(else_stmts) = else_body {
                    build_var_type_env_full(
                        else_stmts,
                        var_types,
                        function_return_types,
                        class_method_return_types,
                    );
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                build_var_type_env_full(
                    body,
                    var_types,
                    function_return_types,
                    class_method_return_types,
                );
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                build_var_type_env_full(
                    body,
                    var_types,
                    function_return_types,
                    class_method_return_types,
                );
                for handler in handlers {
                    build_var_type_env_full(
                        &handler.body,
                        var_types,
                        function_return_types,
                        class_method_return_types,
                    );
                }
                if let Some(orelse_stmts) = orelse {
                    build_var_type_env_full(
                        orelse_stmts,
                        var_types,
                        function_return_types,
                        class_method_return_types,
                    );
                }
                if let Some(finally_stmts) = finalbody {
                    build_var_type_env_full(
                        finally_stmts,
                        var_types,
                        function_return_types,
                        class_method_return_types,
                    );
                }
            }
            HirStmt::With { body, .. } => {
                build_var_type_env_full(
                    body,
                    var_types,
                    function_return_types,
                    class_method_return_types,
                );
            }
            _ => {}
        }
    }
}

pub(crate) fn infer_expr_type_with_class_methods(
    expr: &HirExpr,
    var_types: &std::collections::HashMap<String, Type>,
    class_method_return_types: &std::collections::HashMap<(String, String), Type>,
) -> Type {
    match expr {
        // DEPYLER-1007: Handle method calls on typed variables (e.g., p.distance_squared())
        HirExpr::MethodCall { object, method, .. } => {
            // First try to get the object's type from the environment
            let object_type =
                infer_expr_type_with_class_methods(object, var_types, class_method_return_types);

            // If object is a Custom type (user-defined class), look up the method return type
            if let Type::Custom(class_name) = &object_type {
                if let Some(ret_type) =
                    class_method_return_types.get(&(class_name.clone(), method.clone()))
                {
                    return ret_type.clone();
                }
            }

            // Fall back to standard inference
            infer_expr_type_with_env(expr, var_types)
        }
        // For all other expressions, delegate to standard inference
        _ => infer_expr_type_with_env(expr, var_types),
    }
}
