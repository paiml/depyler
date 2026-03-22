pub(crate) fn infer_return_type_from_body_with_params(
    func: &HirFunction,
    ctx: &CodeGenContext,
) -> Option<Type> {
    // Build initial type environment with function parameters
    let mut var_types: std::collections::HashMap<String, Type> = std::collections::HashMap::new();

    // Add parameter types to environment
    // For argparse validators, parameters are typically strings
    // DEPYLER-0455 #7: Validator functions receive &str parameters
    let is_validator = ctx.validator_functions.contains(&func.name);
    for param in &func.params {
        let param_type = if is_validator && matches!(param.ty, Type::Unknown) {
            // Validator function parameters without type annotations are strings
            Type::String
        } else {
            param.ty.clone()
        };
        var_types.insert(param.name.clone(), param_type);
    }

    // Build additional types from variable assignments
    // DEPYLER-1007: Use full class-aware version to recognize:
    // - Constructor calls like `p = Point(3, 4)` → p has type Type::Custom("Point")
    // - Method calls like `dist_sq = p.distance_squared()` → dist_sq has type Int
    build_var_type_env_full(
        &func.body,
        &mut var_types,
        &ctx.function_return_types,
        &ctx.class_method_return_types,
    );

    // DEPYLER-1007: Collect return types with class method return type awareness
    // This enables proper return type inference for expressions like `p.distance_squared()`
    let mut return_types = Vec::new();
    collect_return_types_with_class_methods(
        &func.body,
        &mut return_types,
        &var_types,
        &ctx.class_method_return_types,
    );

    // DEPYLER-1084: Do NOT infer return type from trailing expressions
    // Python does NOT have implicit returns - expression statements just evaluate
    // and discard their value. Only explicit `return x` contributes to return type.
    // See comment in infer_return_type_from_body() for details.

    if return_types.is_empty() {
        return None;
    }

    // DEPYLER-0460: Check for Optional pattern BEFORE homogeneous type check
    // If function returns None in some paths and a consistent type in others,
    // infer return type as Optional<T>
    // This MUST come before the homogeneous type check to avoid returning Type::None
    // when we should return Type::Optional
    let has_none = return_types.iter().any(|t| matches!(t, Type::None));
    if has_none {
        // Find all non-None, non-Unknown types
        let non_none_types: Vec<&Type> =
            return_types.iter().filter(|t| !matches!(t, Type::None | Type::Unknown)).collect();

        if !non_none_types.is_empty() {
            // Check if all non-None types are the same
            let first_non_none = non_none_types[0];
            if non_none_types.iter().all(|t| *t == first_non_none) {
                // Pattern detected: return None | return T → Option<T>
                return Some(Type::Optional(Box::new(first_non_none.clone())));
            }
        }

        // DEPYLER-0460: If we have None + only Unknown types, still infer Optional
        // Example: def get(d, key): if ...: return d[key]  else: return None
        // d[key] type is Unknown, but the pattern is clearly Optional
        let has_only_unknown = return_types.iter().all(|t| matches!(t, Type::None | Type::Unknown));
        if has_only_unknown && return_types.len() > 1 {
            // At least one None and one Unknown -> Optional<Unknown>
            return Some(Type::Optional(Box::new(Type::Unknown)));
        }

        // If all returns are only None (no Unknown), return Type::None
        if return_types.iter().all(|t| matches!(t, Type::None)) {
            return Some(Type::None);
        }
    }

    // DEPYLER-0744: Handle T and Option<Unknown> → Option<T>
    // When a function returns both a typed value and an Option<Unknown> (from a param with default=None),
    // unify to Option<T> where T is the non-Optional type
    // Example: def f(x: int, fallback=None): return x OR return fallback
    //   → return types: [Int, Optional(Unknown)] → Option<Int>
    let has_optional_unknown = return_types
        .iter()
        .any(|t| matches!(t, Type::Optional(inner) if matches!(inner.as_ref(), Type::Unknown)));
    if has_optional_unknown {
        // Find the concrete non-Optional, non-Unknown type
        let concrete_type = return_types
            .iter()
            .find(|t| !matches!(t, Type::Optional(_) | Type::Unknown | Type::None));
        if let Some(t) = concrete_type {
            // Unify: T + Option<Unknown> → Option<T>
            return Some(Type::Optional(Box::new(t.clone())));
        }
    }

    // If all types are Unknown, return None
    if return_types.iter().all(|t| matches!(t, Type::Unknown)) {
        return None;
    }

    // Check for homogeneous type (all return types are the same, ignoring Unknown)
    // This runs AFTER Optional detection to avoid misclassifying Optional patterns
    let first_known = return_types.iter().find(|t| !matches!(t, Type::Unknown));
    if let Some(first) = first_known {
        if return_types.iter().all(|t| matches!(t, Type::Unknown) || t == first) {
            return Some(first.clone());
        }
    }

    // Mixed types - return the first known type
    first_known.cloned()
}

pub(crate) fn build_var_type_env(
    stmts: &[HirStmt],
    var_types: &mut std::collections::HashMap<String, Type>,
) {
    build_var_type_env_with_classes(stmts, var_types, &std::collections::HashMap::new());
}

pub(crate) fn build_var_type_env_with_classes(
    stmts: &[HirStmt],
    var_types: &mut std::collections::HashMap<String, Type>,
    function_return_types: &std::collections::HashMap<String, Type>,
) {
    // Call the full version with empty class_method_return_types for backward compat
    build_var_type_env_full(
        stmts,
        var_types,
        function_return_types,
        &std::collections::HashMap::new(),
    );
}

pub(crate) fn collect_return_types_with_class_methods(
    stmts: &[HirStmt],
    types: &mut Vec<Type>,
    var_types: &std::collections::HashMap<String, Type>,
    class_method_return_types: &std::collections::HashMap<(String, String), Type>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                types.push(infer_expr_type_with_class_methods(
                    expr,
                    var_types,
                    class_method_return_types,
                ));
            }
            HirStmt::Return(None) => {
                types.push(Type::None);
            }
            HirStmt::If { then_body, else_body, .. } => {
                collect_return_types_with_class_methods(
                    then_body,
                    types,
                    var_types,
                    class_method_return_types,
                );
                if let Some(else_stmts) = else_body {
                    collect_return_types_with_class_methods(
                        else_stmts,
                        types,
                        var_types,
                        class_method_return_types,
                    );
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                collect_return_types_with_class_methods(
                    body,
                    types,
                    var_types,
                    class_method_return_types,
                );
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                collect_return_types_with_class_methods(
                    body,
                    types,
                    var_types,
                    class_method_return_types,
                );
                for handler in handlers {
                    collect_return_types_with_class_methods(
                        &handler.body,
                        types,
                        var_types,
                        class_method_return_types,
                    );
                }
                if let Some(orelse_stmts) = orelse {
                    collect_return_types_with_class_methods(
                        orelse_stmts,
                        types,
                        var_types,
                        class_method_return_types,
                    );
                }
                if let Some(finally_stmts) = finalbody {
                    collect_return_types_with_class_methods(
                        finally_stmts,
                        types,
                        var_types,
                        class_method_return_types,
                    );
                }
            }
            HirStmt::With { body, .. } => {
                collect_return_types_with_class_methods(
                    body,
                    types,
                    var_types,
                    class_method_return_types,
                );
            }
            _ => {}
        }
    }
}
