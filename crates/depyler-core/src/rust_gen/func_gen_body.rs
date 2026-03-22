fn collect_typed_locals_from_body(stmts: &[HirStmt]) -> std::collections::HashMap<String, Type> {
    let mut locals = std::collections::HashMap::new();
    for stmt in stmts {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                type_annotation: Some(ty),
                ..
            } => {
                locals.insert(name.clone(), ty.clone());
            }
            HirStmt::If { then_body, else_body, .. } => {
                for (k, v) in collect_typed_locals_from_body(then_body) {
                    locals.entry(k).or_insert(v);
                }
                if let Some(else_stmts) = else_body {
                    for (k, v) in collect_typed_locals_from_body(else_stmts) {
                        locals.entry(k).or_insert(v);
                    }
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                for (k, v) in collect_typed_locals_from_body(body) {
                    locals.entry(k).or_insert(v);
                }
            }
            _ => {}
        }
    }
    locals
}

pub(crate) fn codegen_function_body(
    func: &HirFunction,
    can_fail: bool,
    error_type: Option<crate::rust_gen::context::ErrorType>,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    // Enter function scope and declare parameters
    ctx.enter_scope();
    ctx.current_function_can_fail = can_fail;

    // GH-70: Pre-populate nested function parameter types with inference
    // This must happen before processing body statements so that nested function
    // code generation can use the inferred types from ctx.nested_function_params
    let _ = detect_returns_nested_function(func, ctx);

    // DEPYLER-0460: Infer return type from body if not explicitly annotated
    // This must happen before setting ctx.current_return_type so that return
    // statement generation uses the correct type (e.g., wrapping in Some() for Optional)
    // Use the SAME inference logic as signature generation for consistency
    // DEPYLER-0460: Also infer when ret_type is None (could be Optional pattern)
    // DEPYLER-0662: Also infer when ret_type is empty tuple (from `-> tuple` annotation)
    // DEPYLER-0662: Python `-> tuple` parses to Type::Custom("tuple"), not Type::Tuple
    let should_infer = matches!(func.ret_type, Type::Unknown | Type::None)
        || matches!(&func.ret_type, Type::Tuple(elems) if elems.is_empty() || elems.iter().any(|t| matches!(t, Type::Unknown)))
        || matches!(&func.ret_type, Type::List(elem) if matches!(**elem, Type::Unknown))
        || matches!(&func.ret_type, Type::Custom(name) if name == "tuple");

    let effective_return_type = if should_infer {
        // No explicit annotation - try to infer from function body
        if let Some(inferred) = infer_return_type_from_body_with_params(func, ctx) {
            inferred
        } else {
            func.ret_type.clone()
        }
    } else {
        func.ret_type.clone()
    };
    ctx.current_return_type = Some(effective_return_type.clone());

    // DEPYLER-1076: Set flag when function returns impl Iterator/IntoIterator
    // This triggers `move` keyword on closures that capture local variables
    ctx.returns_impl_iterator = match &effective_return_type {
        Type::Custom(name) => {
            name.starts_with("Generator")
                || name.starts_with("Iterator")
                || name.contains("Iterator")
                || name.contains("Iterable")
        }
        Type::Generic { base, .. } => {
            base == "Generator" || base == "Iterator" || base == "Iterable"
        }
        _ => false,
    };

    // DEPYLER-0310: Set error type for raise statement wrapping
    ctx.current_error_type = error_type;

    // DEPYLER-1045: Clear fn_str_params from previous function
    // Without this, &str params from previous functions pollute later functions,
    // causing auto-borrow logic to skip local String variables with same names
    ctx.fn_str_params.clear();

    // DEPYLER-1044: Clear numpy_vars from previous function
    // Without this, numpy tracking from one function pollutes subsequent functions
    // especially CSE temps like _cse_temp_0 which are reused across functions
    ctx.numpy_vars.clear();

    for param in &func.params {
        ctx.declare_var(&param.name);
        // Store parameter type information for set/dict disambiguation
        ctx.var_types.insert(param.name.clone(), param.ty.clone());

        // DEPYLER-0543: Track function params with str type (become &str in Rust)
        // These should NOT have & added when used as dict keys
        if matches!(param.ty, Type::String) {
            ctx.fn_str_params.insert(param.name.clone());
        }
    }

    // DEPYLER-0690: Build var_types from local variable assignments BEFORE codegen
    // This enables type-aware string concatenation detection (format! vs +)
    // and other type-based code generation decisions
    // DEPYLER-99MODE-S9: Use full version with function_return_types so class
    // constructor calls (e.g., c = Config("test")) get the correct Custom type
    // instead of falling back to Unknown → HashMap<DepylerValue, DepylerValue>
    build_var_type_env_full(
        &func.body,
        &mut ctx.var_types,
        &ctx.function_return_types,
        &ctx.class_method_return_types,
    );

    // DEPYLER-1134: Propagate return type annotation to returned variables
    // This enables Constraint-Aware Coercion - if function returns List[List[str]],
    // the variable being returned (e.g., `rows`) gets that concrete type
    if let Some(ref ret_type) = ctx.current_return_type {
        propagate_return_type_to_vars(&func.body, &mut ctx.var_types, ret_type);
    }

    // Refine container element types for local vars like `result = []`
    // where subsequent `result.append(42)` reveals the element type
    crate::container_element_inference::refine_container_types_from_usage(
        &func.body,
        &mut ctx.var_types,
    );

    // DEPYLER-0312 NOTE: analyze_mutable_vars is now called in impl RustCodeGen BEFORE
    // codegen_function_params, so ctx.mutable_vars is already populated here

    // DEPYLER-0784: Don't hoist nested functions
    // Previous DEPYLER-0613 hoisted ALL nested functions to fix E0425 (forward references),
    // but this causes E0282 (type annotations needed) because Rust can't infer closure
    // types without an initializer (`let x;` for closures doesn't work).
    // Since most Python code defines functions before calling them, we remove hoisting.
    // If forward references are needed, the Rust compiler will give a clear E0425 error.
    let mut all_nested_fns = Vec::new();
    collect_nested_function_names(&func.body, &mut all_nested_fns);

    // Start with an empty body
    let mut body_stmts: Vec<proc_macro2::TokenStream> = Vec::new();

    // DEPYLER-0784: Don't pre-declare nested functions
    // The closure will be declared inline when processing the FunctionDef statement
    // Note: We collect names but don't call ctx.declare_var() so that
    // codegen_nested_function_def will emit `let name = move |...|` instead of `name = ...`
    let _ = all_nested_fns; // Silence unused warning

    // DEPYLER-0963: Build parameter types map for variable type inference
    // This allows us to infer types for variables assigned from parameters (e.g., result = n)
    let mut param_types: std::collections::HashMap<String, Type> =
        func.params.iter().map(|p| (p.name.clone(), p.ty.clone())).collect();

    // DEPYLER-99MODE-S9: Extend param_types with typed local variable annotations.
    // This allows codegen to know types of locals like `x: Tuple[int, int] = create_pair()`
    // so that tuple field access (x.0, x.1) is used instead of dict-like .get()
    for (name, ty) in collect_typed_locals_from_body(&func.body) {
        param_types.entry(name).or_insert(ty);
    }

    // DEPYLER-0762: Hoist loop-escaping variables
    // Python has function-level scoping, so variables assigned in for/while loops
    // are visible after the loop. Rust has block-level scoping, so we must hoist
    // these variable declarations to function scope.
    // We initialize with Default::default() to avoid E0381 "possibly-uninitialized"
    // errors, since Rust can't prove the loop will always execute.
    // DEPYLER-0763: Use safe_ident to escape Rust keywords (e.g. match -> r#match)
    // DEPYLER-0963: Add type annotations to avoid E0790 (can't call Default::default() on trait)
    let loop_escaping_vars = collect_loop_escaping_variables(&func.body);
    for var_name in &loop_escaping_vars {
        if !ctx.is_declared(var_name) {
            let ident = safe_ident(var_name);
            // Try to infer the variable's type from its assignments (with param context)
            if let Some(ty) = find_var_type_in_body_with_params(var_name, &func.body, &param_types)
            {
                let rust_type = ctx.type_mapper.map_type(&ty);
                if let Ok(syn_type) = rust_type_to_syn(&rust_type) {
                    body_stmts.push(quote! { let mut #ident: #syn_type = Default::default(); });
                } else {
                    // DEPYLER-1065: Hybrid Fallback - use DepylerValue instead of ()
                    let fallback_type = if ctx.type_mapper.nasa_mode {
                        quote! { DepylerValue }
                    } else {
                        quote! { serde_json::Value }
                    };
                    body_stmts
                        .push(quote! { let mut #ident: #fallback_type = Default::default(); });
                }
            } else {
                // DEPYLER-1065: Hybrid Fallback - use DepylerValue instead of ()
                let fallback_type = if ctx.type_mapper.nasa_mode {
                    quote! { DepylerValue }
                } else {
                    quote! { serde_json::Value }
                };
                body_stmts.push(quote! { let mut #ident: #fallback_type = Default::default(); });
            }
            ctx.declare_var(var_name);
        }
    }

    // DEPYLER-0834: Hoist if-escaping variables
    // Python has function-level scoping, so variables assigned in if/else blocks
    // are visible after the if statement. Rust has block-level scoping, so we must
    // hoist these variable declarations to function scope.
    // We initialize with Default::default() to avoid E0381 "possibly-uninitialized"
    // errors, since the if branch may not be taken.
    // DEPYLER-0963: Add type annotations to avoid E0790 (can't call Default::default() on trait)
    let if_escaping_vars = collect_if_escaping_variables(&func.body);
    for var_name in &if_escaping_vars {
        if !ctx.is_declared(var_name) {
            let ident = safe_ident(var_name);
            // Try to infer the variable's type from its assignments (with param context)
            if let Some(ty) = find_var_type_in_body_with_params(var_name, &func.body, &param_types)
            {
                let rust_type = ctx.type_mapper.map_type(&ty);
                if let Ok(syn_type) = rust_type_to_syn(&rust_type) {
                    body_stmts.push(quote! { let mut #ident: #syn_type = Default::default(); });
                } else {
                    // DEPYLER-1065: Hybrid Fallback - use DepylerValue instead of ()
                    let fallback_type = if ctx.type_mapper.nasa_mode {
                        quote! { DepylerValue }
                    } else {
                        quote! { serde_json::Value }
                    };
                    body_stmts
                        .push(quote! { let mut #ident: #fallback_type = Default::default(); });
                }
            } else {
                // DEPYLER-1065: Hybrid Fallback - use DepylerValue instead of ()
                let fallback_type = if ctx.type_mapper.nasa_mode {
                    quote! { DepylerValue }
                } else {
                    quote! { serde_json::Value }
                };
                body_stmts.push(quote! { let mut #ident: #fallback_type = Default::default(); });
            }
            ctx.declare_var(var_name);
        }
    }

    // DEPYLER-0688: Emit statements in original order, preserving Python semantics
    // Nested functions that capture outer variables must be emitted AFTER those variables
    // are declared. Forward declarations (let mut fib;) are already emitted above.
    let body_len = func.body.len();
    for (i, stmt) in func.body.iter().enumerate() {
        // Mark final statement for idiomatic expression-based return
        // (only if it's not a FunctionDef, as those are assignments not returns)
        ctx.is_final_statement = i == body_len - 1 && !matches!(stmt, HirStmt::FunctionDef { .. });

        // DEPYLER-1168: Populate vars_used_later for call-site clone detection
        // Before processing each statement, compute which variables are used in remaining statements
        ctx.vars_used_later.clear();
        let remaining_stmts = &func.body[i + 1..];
        for var_name in ctx.var_types.keys() {
            if is_var_used_in_remaining_stmts(var_name, remaining_stmts) {
                ctx.vars_used_later.insert(var_name.clone());
            }
        }

        let tokens = stmt.to_rust_tokens(ctx)?;
        body_stmts.push(tokens);
    }

    ctx.exit_scope();
    ctx.current_function_can_fail = false;
    ctx.current_return_type = None;
    ctx.returns_impl_iterator = false;

    Ok(body_stmts)
}
