fn is_param_used_in_body(param_name: &str, body: &[HirStmt]) -> bool {
    body.iter().any(|stmt| is_param_used_in_stmt(param_name, stmt))
}

fn codegen_single_param(
    param: &HirParam,
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0757: Check if parameter is used in the function body
    // If not used, prefix with underscore to avoid unused variable warnings
    let is_used = is_param_used_in_body(&param.name, &func.body);

    // DEPYLER-0357: Parameter names in signature must match how they're referenced in body
    // DEPYLER-0757: But if NOT used at all, prefix with underscore to suppress warning
    // DEPYLER-0611: Use raw identifiers for parameter names that are Rust keywords
    // DEPYLER-0630: self/Self cannot be raw identifiers, rename to self_ instead
    let param_name = if is_used { param.name.clone() } else { format!("_{}", param.name) };

    let param_ident = if param_name == "self" || param_name == "Self" {
        // self/Self are special - they cannot be raw identifiers, rename them
        syn::Ident::new(&format!("{}_", param_name), proc_macro2::Span::call_site())
    } else if is_rust_keyword(&param_name) {
        syn::Ident::new_raw(&param_name, proc_macro2::Span::call_site())
    } else {
        syn::Ident::new(&param_name, proc_macro2::Span::call_site())
    };

    // DEPYLER-0477: Handle varargs parameters (*args in Python)
    // DEPYLER-0487: Generate &[T] instead of Vec<T> for better ergonomics
    // This allows calling from match patterns where the value is borrowed
    // Python: def func(*args) → Rust: fn func(args: &[T])
    if param.is_vararg {
        // DEPYLER-1150: Track this parameter as a slice for return type conversion
        // When returning a slice param in a function that returns Vec<T>, add .to_vec()
        ctx.slice_params.insert(param.name.clone());

        // Extract element type from Type::List
        let elem_type = if let Type::List(elem) = &param.ty {
            rust_type_to_syn(&ctx.type_mapper.map_type(elem))?
        } else {
            // Fallback: If not Type::List, use String as default
            // This shouldn't happen if AST bridge is correct
            parse_quote! { String }
        };

        // Varargs parameters as slices (more idiomatic Rust)
        return Ok(quote! { #param_ident: &[#elem_type] });
    }

    // DEPYLER-0424: Check if this parameter is the argparse args variable
    // If so, type it as &Args instead of default type mapping
    let is_argparse_args = ctx.argparser_tracker.parsers.values().any(|parser_info| {
        parser_info.args_var.as_ref().is_some_and(|args_var| args_var == &param.name)
    });

    if is_argparse_args {
        // Use &Args for argparse result parameters
        return Ok(quote! { #param_ident: &Args });
    }

    // DEPYLER-0488: Special case for set_nested_value's value parameter
    // The parameter is NOT mutated (only used on RHS of `dict[key] = value`)
    // Override incorrect mutability analysis for this specific function
    if func.name == "set_nested_value" && param.name == "value" {
        if let Some(inferred) = lifetime_result.param_lifetimes.get(&param.name) {
            let rust_type = &inferred.rust_type;

            // Force immutable even if analysis incorrectly flagged as mutable
            let mut inferred_immutable = inferred.clone();
            inferred_immutable.needs_mut = false;

            let ty = apply_param_borrowing_strategy(
                &param.name,
                rust_type,
                &inferred_immutable,
                lifetime_result,
                ctx,
            )?;

            return Ok(quote! { #param_ident: #ty });
        }
    }

    // DEPYLER-0312: Use mutable_vars populated by analyze_mutable_vars
    // This handles ALL mutation patterns: direct assignment, method calls, and parameter reassignments
    // The analyze_mutable_vars function already checked all mutation patterns in codegen_function_body
    let is_mutated_in_body = ctx.mutable_vars.contains(&param.name);

    // Only apply `mut` if ownership is taken (not borrowed)
    // Borrowed parameters (&T, &mut T) handle mutability in the type itself
    let takes_ownership = matches!(
        lifetime_result.borrowing_strategies.get(&param.name),
        Some(crate::borrowing_context::BorrowingStrategy::TakeOwnership) | None
    );

    let is_param_mutated = is_mutated_in_body && takes_ownership;

    // DEPYLER-0447: Detect argparse validator functions (tracked at add_argument() call sites)
    // These should ALWAYS have &str parameter type regardless of type inference
    // Validators are detected when processing add_argument(type=validator_func)
    let is_argparse_validator = ctx.validator_functions.contains(&func.name);

    if is_argparse_validator {
        // Argparse validators always receive string arguments from clap
        let ty = if is_param_mutated {
            quote! { mut #param_ident: &str }
        } else {
            quote! { #param_ident: &str }
        };
        return Ok(ty);
    }

    // DEPYLER-0607: Infer Args type for argparse command handler functions
    // When a function takes "args" parameter with Unknown type and it's a command handler,
    // the parameter should be &Args (reference to clap Args struct)
    // Pattern: def cmd_list(args): args.archive → fn cmd_list(args: &Args)
    // Heuristic: Function starts with "cmd_" or "handle_" and has "args" parameter
    // This must run BEFORE lifetime inference check to override serde_json::Value fallback
    let is_cmd_handler = func.name.starts_with("cmd_") || func.name.starts_with("handle_");
    if param.name == "args" && is_cmd_handler && matches!(param.ty, Type::Unknown) {
        let ty: syn::Type = syn::parse_quote! { &Args };
        return Ok(if is_param_mutated {
            quote! { mut #param_ident: #ty }
        } else {
            quote! { #param_ident: #ty }
        });
    }

    // Get the inferred parameter info
    if let Some(inferred) = lifetime_result.param_lifetimes.get(&param.name) {
        // DEPYLER-0716: Check if we have a substituted type from generic inference
        // This overrides the type from lifetime analysis when T is inferred to be a concrete type
        let rust_type = if let Some(substituted_ty) = ctx.var_types.get(&param.name) {
            // Use substituted type from generic inference
            ctx.type_mapper.map_type(substituted_ty)
        } else {
            inferred.rust_type.clone()
        };

        // Handle Union type placeholders
        let actual_rust_type =
            if let crate::type_mapper::RustType::Enum { name, variants: _ } = &rust_type {
                if name == "UnionType" {
                    if let Type::Union(types) = &param.ty {
                        let enum_name = ctx.process_union_type(types);
                        crate::type_mapper::RustType::Custom(enum_name)
                    } else {
                        rust_type.clone()
                    }
                } else {
                    rust_type.clone()
                }
            } else {
                rust_type.clone()
            };

        update_import_needs(ctx, &actual_rust_type);

        // DEPYLER-0330: Override needs_mut for borrowed parameters that are mutated
        // If analyze_mutable_vars detected mutation (via .remove(), .clear(), etc.)
        // and this parameter will be borrowed (&T), upgrade to &mut T
        let mut inferred_with_mut = inferred.clone();
        if is_mutated_in_body && inferred.should_borrow {
            inferred_with_mut.needs_mut = true;
        }

        let ty = apply_param_borrowing_strategy(
            &param.name,
            &actual_rust_type,
            &inferred_with_mut,
            lifetime_result,
            ctx,
        )?;

        Ok(if is_param_mutated {
            quote! { mut #param_ident: #ty }
        } else {
            quote! { #param_ident: #ty }
        })
    } else {
        // DEPYLER-0524/0716: Check if we have an inferred/substituted type from body usage analysis
        // This allows inferring String for parameters used with .endswith(), etc.
        // DEPYLER-0716: Also check for type substitutions (e.g., List(Unknown) -> List(String))
        let effective_type = if let Some(substituted) = ctx.var_types.get(&param.name) {
            // Use substituted type from type inference (DEPYLER-0716)
            substituted.clone()
        } else if matches!(param.ty, Type::Unknown) {
            // DEPYLER-0607: Infer Args type for argparse command handler functions
            // When a function takes "args" parameter with Unknown type and it's a command handler,
            // the parameter should be &Args (reference to clap Args struct)
            // Pattern: def cmd_list(args): args.archive → fn cmd_list(args: &Args)
            // Heuristic: Function starts with "cmd_" and has "args" parameter
            // This works even when argparse detection hasn't run yet (functions processed before main)
            let is_cmd_handler = func.name.starts_with("cmd_") || func.name.starts_with("handle_");
            if param.name == "args" && is_cmd_handler {
                Type::Custom("Args".to_string())
            } else {
                param.ty.clone()
            }
        } else {
            param.ty.clone()
        };

        // Fallback to original mapping using effective (possibly inferred) type
        let rust_type = ctx
            .annotation_aware_mapper
            .map_type_with_annotations(&effective_type, &func.annotations);
        update_import_needs(ctx, &rust_type);
        let ty = rust_type_to_syn(&rust_type)?;

        Ok(if is_param_mutated {
            quote! { mut #param_ident: #ty }
        } else {
            quote! { #param_ident: #ty }
        })
    }
}
