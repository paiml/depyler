pub(crate) fn codegen_while_stmt(
    condition: &HirExpr,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0791: Save and restore is_final_statement for loop body
    // Return statements inside loops are always early exits, never final expressions
    // Without this, `return count` inside `if` inside `loop` gets generated as just `count`
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

    // DEPYLER-0698: Convert `while True:` to `loop {}` for idiomatic Rust
    // Rust warns: "denote infinite loops with `loop { ... }`"
    if matches!(condition, HirExpr::Literal(Literal::Bool(true))) {
        ctx.enter_scope();
        let body_stmts: Vec<_> =
            body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();
        ctx.is_final_statement = saved_is_final;
        return Ok(quote! {
            loop {
                #(#body_stmts)*
            }
        });
    }

    let mut cond = condition.to_rust_expr(ctx)?;

    // DEPYLER-0421: Apply Python truthiness conversion for while loops
    // Convert non-boolean expressions to boolean (e.g., `while queue` where queue: VecDeque)
    cond = apply_truthiness_conversion(condition, cond, ctx);

    ctx.enter_scope();
    let body_stmts: Vec<_> =
        body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();
    ctx.is_final_statement = saved_is_final;
    Ok(quote! {
        while #cond {
            #(#body_stmts)*
        }
    })
}

fn apply_negated_truthiness(
    operand: &HirExpr,
    cond_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> syn::Expr {
    // DEPYLER-1027: Check if cond_expr is already a truthiness-converted method call
    // to_rust_expr for UnaryOp::Not may have already converted `not x` to `x.is_empty()`
    // or `x.is_none()`. In that case, return it directly to avoid double conversion.
    if let syn::Expr::MethodCall(method_call) = &cond_expr {
        let method_name = method_call.method.to_string();
        if method_name == "is_empty" || method_name == "is_none" {
            // Already converted by to_rust_expr - return as-is
            return cond_expr;
        }
    }

    // Extract the inner expression (strip the `!` from the already-converted cond_expr)
    // The cond_expr is already `!inner_expr` from to_rust_expr, so we need the inner part
    let inner_expr =
        if let syn::Expr::Unary(syn::ExprUnary { op: syn::UnOp::Not(_), expr, .. }) = &cond_expr {
            expr.as_ref().clone()
        } else {
            // Fallback: use the whole expression
            cond_expr.clone()
        };

    // Helper to get the type for type-based truthiness
    let get_type_for_operand = |op: &HirExpr| -> Option<Type> {
        match op {
            HirExpr::Var(var_name) => ctx.var_types.get(var_name).cloned(),
            HirExpr::Attribute { value, attr } => {
                if let HirExpr::Var(obj_name) = value.as_ref() {
                    if obj_name == "self" {
                        return ctx.class_field_types.get(attr).cloned();
                    }
                }
                None
            }
            _ => None,
        }
    };

    // Get the type of the inner operand
    if let Some(operand_type) = get_type_for_operand(operand) {
        return match operand_type {
            // Bool: keep the negation as-is
            Type::Bool => cond_expr,

            // String/List/Dict/Set: `not x` → `x.is_empty()` (inverted truthiness)
            Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                parse_quote! { #inner_expr.is_empty() }
            }

            // Optional: `not x` → `x.is_none()` (inverted truthiness)
            Type::Optional(_) => {
                parse_quote! { #inner_expr.is_none() }
            }

            // Numeric: `not x` → `x == 0` (inverted truthiness)
            Type::Int => {
                parse_quote! { #inner_expr == 0 }
            }
            Type::Float => {
                parse_quote! { #inner_expr == 0.0 }
            }

            // Unknown or other types: keep the negation
            _ => cond_expr,
        };
    }

    // DEPYLER-0966: Heuristic for self.* fields with common list/collection names
    // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
    if let HirExpr::Attribute { value, attr } = operand {
        if let HirExpr::Var(obj_name) = value.as_ref() {
            if obj_name == "self" && is_collection_attr_name(attr) {
                return parse_quote! { #inner_expr.is_empty() };
            }
        }
    }

    // Fallback: keep the original expression (including the negation)
    cond_expr
}

fn apply_truthiness_conversion(
    condition: &HirExpr,
    cond_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> syn::Expr {
    // DEPYLER-0966: Handle negated truthiness: `if not x:` where x is non-boolean
    // For `not collection`, we should generate `collection.is_empty()` (no double negation)
    // For `not optional`, we should generate `optional.is_none()`
    if let HirExpr::Unary { op: UnaryOp::Not, operand } = condition {
        // Get the type of the inner operand and generate inverted truthiness
        return apply_negated_truthiness(operand, cond_expr, ctx);
    }

    // Check if this is a variable reference that needs truthiness conversion
    if let HirExpr::Var(var_name) = condition {
        // DEPYLER-0969: Check if type is tracked and handle known types
        if let Some(var_type) = ctx.var_types.get(var_name) {
            match var_type {
                // Already boolean - no conversion needed
                Type::Bool => return cond_expr,

                // String/List/Dict/Set - check if empty
                Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                    return parse_quote! { !#cond_expr.is_empty() };
                }

                // Optional - check if Some
                Type::Optional(_) => {
                    return parse_quote! { #cond_expr.is_some() };
                }

                // Numeric types - check if non-zero
                Type::Int => {
                    return parse_quote! { #cond_expr != 0 };
                }
                Type::Float => {
                    return parse_quote! { #cond_expr != 0.0 };
                }

                // DEPYLER-0969: Custom types that are collections (VecDeque, BinaryHeap, etc.)
                // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
                Type::Custom(type_name) => {
                    if is_collection_type_name(type_name) {
                        return parse_quote! { !#cond_expr.is_empty() };
                    }
                    // Fall through to heuristics for non-collection custom types
                }

                // DEPYLER-0969: Generic types that are collections
                // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
                Type::Generic { base, .. } => {
                    if is_collection_generic_base(base) {
                        return parse_quote! { !#cond_expr.is_empty() };
                    }
                    // Fall through to heuristics for non-collection generic types
                }

                // Unknown - fall through to heuristics below
                Type::Unknown => {
                    // Don't return - let heuristics handle it
                }

                // Other concrete types - fall through to heuristics
                _ => {}
            }
        }

        // DEPYLER-0517: Heuristic fallback for common string variable names
        // This handles variables from tuple unpacking that aren't tracked in var_types
        // e.g., `let (returncode, stdout, stderr) = run_command(...)`
        // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
        if is_string_var_name(var_name) {
            return parse_quote! { !#cond_expr.is_empty() };
        }

        // DEPYLER-0969: Heuristic fallback for common collection variable names
        // This is the ARCHITECTURAL FIX for truthiness - handles untracked collection types
        // Pattern: `while queue:` where queue is VecDeque/Vec/etc not in var_types
        // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
        if is_collection_var_name(var_name) {
            return parse_quote! { !#cond_expr.is_empty() };
        }

        // DEPYLER-1071: Heuristic fallback for common Option variable names
        // This handles regex match results and other optional values
        // Pattern: `if m:` where m is a regex match result (Option<Match>)
        if is_option_var_name(var_name) {
            return parse_quote! { #cond_expr.is_some() };
        }
    }

    // DEPYLER-0570: Handle dict index access in conditions
    // Python: `if info["extension"]:` checks if the value is truthy (non-empty string)
    // Rust: info.get("extension")... returns serde_json::Value, need to check truthiness
    // Convert to: `.as_str().is_some_and(|s| !s.is_empty())` for string values
    if let HirExpr::Index { base, index } = condition {
        // Check if using string key (dict-like access)
        let has_string_key = matches!(index.as_ref(), HirExpr::Literal(Literal::String(_)));

        // Check if base is a dict (HashMap) or common dict variable name
        // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
        let is_dict_access = if let HirExpr::Var(var_name) = base.as_ref() {
            // Known dict type
            if let Some(var_type) = ctx.var_types.get(var_name) {
                matches!(var_type, Type::Dict(_, _))
            } else {
                // Unknown type - use string key OR common dict variable names as heuristics
                has_string_key || is_dict_var_name(var_name)
            }
        } else {
            // Nested access or other expression - use string key as heuristic
            has_string_key
        };

        if is_dict_access {
            // Dict value access - check if the Value is truthy
            // serde_json::Value truthiness: string must be non-empty
            return parse_quote! { #cond_expr.as_str().is_some_and(|s| !s.is_empty()) };
        }
    }

    // CB-200 Batch 9: Attribute access truthiness extracted to helper
    if let HirExpr::Attribute { value, attr } = condition {
        if let HirExpr::Var(obj_name) = value.as_ref() {
            if let Some(result) =
                apply_truthiness_attribute_access(obj_name, attr, cond_expr.clone(), ctx)
            {
                return result;
            }
        }
    }

    // CB-200 Batch 9: Fallback checks extracted to helper
    apply_truthiness_fallbacks(condition, cond_expr, ctx)
}

fn apply_truthiness_fallbacks(
    condition: &HirExpr,
    cond_expr: syn::Expr,
    _ctx: &CodeGenContext,
) -> syn::Expr {
    // DEPYLER-0455: Fallback - detect Option types by method call patterns
    if let HirExpr::MethodCall { method, .. } = condition {
        let vec_returning_methods =
            ["groups", "split", "split_whitespace", "splitlines", "findall"];
        if vec_returning_methods.contains(&method.as_str()) {
            return parse_quote! { !#cond_expr.is_empty() };
        }
    }

    if looks_like_option_expr(condition) {
        return parse_quote! { #cond_expr.is_some() };
    }

    // DEPYLER-0570: Fallback - check if the generated expression looks like dict access
    let cond_str = quote::quote!(#cond_expr).to_string();
    if cond_str.contains(".get(") && cond_str.contains("unwrap_or_default") {
        return parse_quote! { #cond_expr.as_str().is_some_and(|s| !s.is_empty()) };
    }

    cond_expr
}
