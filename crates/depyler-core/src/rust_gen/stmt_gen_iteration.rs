fn is_loop_var_used(var_name: &str, body: &[HirStmt]) -> bool {
    body.iter().any(|stmt| is_var_used_in_stmt(var_name, stmt))
}

fn is_loop_var_reassigned(var_name: &str, body: &[HirStmt]) -> bool {
    body.iter().any(|stmt| is_var_reassigned_in_stmt(var_name, stmt))
}

fn track_range_loop_var(target: &AssignTarget, iter: &HirExpr, ctx: &mut CodeGenContext) {
    if let AssignTarget::Symbol(name) = target {
        if matches!(iter, HirExpr::Call { func, .. } if func == "range") {
            ctx.var_types.insert(name.clone(), Type::Int);
        }
    }
}

fn track_collection_loop_var(
    target: &AssignTarget,
    iter: &HirExpr,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) {
    if let AssignTarget::Symbol(name) = target {
        let elem_type = match iter {
            HirExpr::Var(collection_name) => match ctx.var_types.get(collection_name) {
                Some(Type::List(elem)) => Some(elem.as_ref().clone()),
                Some(Type::Set(elem)) => Some(elem.as_ref().clone()),
                _ => None,
            },
            // for x in dict[key] where dict value type is List(T) → x: T
            HirExpr::Index { base, .. } => {
                if let HirExpr::Var(base_name) = base.as_ref() {
                    match ctx.var_types.get(base_name) {
                        Some(Type::Dict(_, val_type)) => {
                            if let Type::List(elem) = val_type.as_ref() {
                                Some(elem.as_ref().clone())
                            } else {
                                None
                            }
                        }
                        Some(Type::List(elem)) => {
                            // Nested list: for row in matrix[i]
                            Some(elem.as_ref().clone())
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(ty) = elem_type {
            if !matches!(ty, Type::Unknown | Type::UnificationVar(_)) {
                ctx.var_types.insert(name.clone(), ty);
            } else {
                // Fallback: infer element type from loop body usage
                if let Some(inferred) =
                    crate::param_type_inference::infer_param_type_from_body(name, body)
                {
                    ctx.var_types.insert(name.clone(), inferred);
                }
            }
        }
    }
}

fn track_char_counter_iter(target: &AssignTarget, iter: &HirExpr, ctx: &mut CodeGenContext) {
    if let AssignTarget::Tuple(targets) = target {
        if let HirExpr::MethodCall { object, method, .. } = iter {
            if method == "items" || method == "most_common" {
                if let HirExpr::Var(counter_name) = object.as_ref() {
                    if ctx.char_counter_vars.contains(counter_name) {
                        if let Some(AssignTarget::Symbol(first_var)) = targets.first() {
                            ctx.char_iter_vars.insert(first_var.clone());
                        }
                    }
                }
            }
        }
    }
}

fn generate_symbol_pattern(name: &str, body: &[HirStmt]) -> syn::Pat {
    let var_name =
        if is_loop_var_used(name, body) { name.to_string() } else { format!("_{}", name) };
    let ident = safe_ident(&var_name);
    if is_loop_var_reassigned(name, body) {
        parse_quote! { mut #ident }
    } else {
        parse_quote! { #ident }
    }
}

fn generate_tuple_pattern(targets: &[AssignTarget], body: &[HirStmt]) -> syn::Pat {
    let patterns: Vec<syn::Pat> = targets
        .iter()
        .map(|t| match t {
            AssignTarget::Symbol(s) => generate_symbol_pattern(s, body),
            _ => parse_quote! { _nested },
        })
        .collect();
    parse_quote! { (#(#patterns),*) }
}

fn generate_for_target_pattern(target: &AssignTarget, body: &[HirStmt]) -> Result<syn::Pat> {
    match target {
        AssignTarget::Symbol(name) => Ok(generate_symbol_pattern(name, body)),
        AssignTarget::Tuple(targets) => Ok(generate_tuple_pattern(targets, body)),
        _ => bail!("Unsupported for loop target type"),
    }
}

fn is_stdin_iteration(iter: &HirExpr) -> bool {
    matches!(iter, HirExpr::Attribute { value, attr }
        if matches!(&**value, HirExpr::Var(m) if m == "sys") && attr == "stdin")
}

fn is_file_iteration(iter: &HirExpr) -> bool {
    if let HirExpr::Var(var_name) = iter {
        var_name == "f"
            || var_name == "file"
            || var_name == "input"
            || var_name == "output"
            || var_name.ends_with("_file")
            || var_name.starts_with("file_")
    } else {
        false
    }
}

fn is_csv_reader_iteration(iter: &HirExpr) -> bool {
    if let HirExpr::Var(var_name) = iter {
        var_name == "reader"
            || var_name.contains("csv")
            || var_name.ends_with("_reader")
            || var_name.starts_with("reader_")
    } else {
        false
    }
}

fn apply_stdin_iteration(iter_expr: syn::Expr) -> syn::Expr {
    parse_quote! { #iter_expr.lines().map(|l| l.unwrap_or_default()) }
}

fn apply_file_iteration(iter_expr: syn::Expr, ctx: &mut CodeGenContext) -> syn::Expr {
    ctx.needs_bufread = true;
    parse_quote! {
        std::io::BufReader::new(#iter_expr).lines()
            .map(|l| l.unwrap_or_default())
    }
}

fn declare_loop_vars_with_types(
    target: &AssignTarget,
    element_type: Option<Type>,
    ctx: &mut CodeGenContext,
) {
    match (target, element_type) {
        (AssignTarget::Symbol(name), Some(elem_type)) => {
            ctx.declare_var(name);
            ctx.var_types.insert(name.clone(), elem_type);
        }
        (AssignTarget::Symbol(name), None) => {
            ctx.declare_var(name);
        }
        (AssignTarget::Tuple(targets), Some(Type::Tuple(elem_types)))
            if targets.len() == elem_types.len() =>
        {
            for (t, typ) in targets.iter().zip(elem_types.iter()) {
                if let AssignTarget::Symbol(s) = t {
                    ctx.declare_var(s);
                    ctx.var_types.insert(s.clone(), typ.clone());
                }
            }
        }
        (AssignTarget::Tuple(targets), _) => {
            for t in targets {
                if let AssignTarget::Symbol(s) = t {
                    ctx.declare_var(s);
                }
            }
        }
        _ => {}
    }
}

fn needs_enumerate_index_cast(iter: &HirExpr, target: &AssignTarget) -> bool {
    matches!(iter, HirExpr::Call { func, .. } if func == "enumerate")
        && matches!(target, AssignTarget::Tuple(targets) if !targets.is_empty())
}

fn needs_char_to_string_conversion(
    target: &AssignTarget,
    iter: &HirExpr,
    body: &[HirStmt],
    ctx: &CodeGenContext,
) -> bool {
    let is_range_iteration = matches!(iter, HirExpr::Call { func, .. } if func == "range");
    if is_range_iteration {
        return false;
    }

    let is_string_iteration = if let AssignTarget::Symbol(loop_var_name) = target {
        ctx.char_iter_vars.contains(loop_var_name)
    } else {
        false
    };

    if !is_string_iteration {
        return false;
    }

    if let AssignTarget::Symbol(loop_var_name) = target {
        body.iter().any(|stmt| is_var_used_as_dict_key_in_stmt(loop_var_name, stmt))
    } else {
        false
    }
}

fn generate_enumerate_cast_loop(
    target: &AssignTarget,
    target_pattern: &syn::Pat,
    iter_expr: &syn::Expr,
    body: &[HirStmt],
    body_stmts: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    if let AssignTarget::Tuple(targets) = target {
        if let Some(AssignTarget::Symbol(index_var)) = targets.first() {
            let is_index_used = body.iter().any(|stmt| is_var_used_in_stmt(index_var, stmt));
            if is_index_used {
                let index_ident = safe_ident(index_var);
                return quote! {
                    for #target_pattern in #iter_expr {
                        let #index_ident = #index_ident as i32;
                        #(#body_stmts)*
                    }
                };
            }
        }
    }
    quote! {
        for #target_pattern in #iter_expr {
            #(#body_stmts)*
        }
    }
}

fn generate_char_to_string_loop(
    target: &AssignTarget,
    target_pattern: &syn::Pat,
    iter_expr: &syn::Expr,
    body_stmts: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    if let AssignTarget::Symbol(var_name) = target {
        let var_ident = safe_ident(var_name);
        let temp_ident = safe_ident(&format!("_{}", var_name));
        quote! {
            for #temp_ident in #iter_expr {
                let #var_ident = #temp_ident.to_string();
                #(#body_stmts)*
            }
        }
    } else {
        quote! {
            for #target_pattern in #iter_expr {
                #(#body_stmts)*
            }
        }
    }
}

fn generate_csv_result_loop(
    target: &AssignTarget,
    target_pattern: &syn::Pat,
    iter_expr: &syn::Expr,
    body_stmts: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    if let AssignTarget::Symbol(var_name) = target {
        let var_ident = safe_ident(var_name);
        let result_ident = safe_ident("result");
        quote! {
            for #result_ident in #iter_expr {
                let #var_ident = #result_ident?;
                #(#body_stmts)*
            }
        }
    } else {
        quote! {
            for #target_pattern in #iter_expr {
                #(#body_stmts)*
            }
        }
    }
}

fn apply_var_iteration_strategy(
    var_name: &str,
    target: &AssignTarget,
    iter_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> syn::Expr {
    let is_string_type = ctx.var_types.get(var_name).is_some_and(|t| matches!(t, Type::String));
    let has_known_non_string_type = ctx.var_types.get(var_name).is_some_and(|t| {
        matches!(
            t,
            Type::List(_)
                | Type::Dict(_, _)
                | Type::Set(_)
                | Type::Int
                | Type::Float
                | Type::Bool
                | Type::Tuple(_)
        )
    });
    let is_string_name = if has_known_non_string_type {
        false
    } else {
        let n = var_name;
        (n == "s"
            || n == "string"
            || n == "text"
            || n == "word"
            || n == "line"
            || n == "char"
            || n == "character")
            || (n.starts_with("str") && !n.starts_with("strings"))
            || (n.starts_with("word") && !n.starts_with("words"))
            || (n.starts_with("text") && !n.starts_with("texts"))
            || (n.ends_with("_str") && !n.ends_with("_strs"))
            || (n.ends_with("_string") && !n.ends_with("_strings"))
            || (n.ends_with("_word") && !n.ends_with("_words"))
            || (n.ends_with("_text") && !n.ends_with("_texts"))
    };
    let is_json_value = ctx.var_types.get(var_name).is_some_and(|t| {
        matches!(t, Type::Custom(name) if name == "Value" || name == "serde_json::Value" || name.contains("json"))
    });

    if is_string_type || is_string_name {
        if let AssignTarget::Symbol(loop_var_name) = target {
            ctx.char_iter_vars.insert(loop_var_name.clone());
        }
        parse_quote! { #iter_expr.chars() }
    } else if is_json_value {
        parse_quote! { #iter_expr.as_array().expect("JSON value is not an array").iter().cloned() }
    } else if ctx.iterator_vars.contains(var_name) {
        iter_expr
    } else {
        let is_dict_type =
            ctx.var_types.get(var_name).is_some_and(|t| matches!(t, Type::Dict(_, _)));
        if is_dict_type {
            parse_quote! { #iter_expr.keys().cloned() }
        } else {
            let is_vector_type = ctx.var_types.get(var_name).is_some_and(|t| match t {
                Type::Custom(name) => name.starts_with("Vector<") || name == "Vector",
                _ => false,
            });
            if is_vector_type {
                parse_quote! { #iter_expr.as_slice().iter().cloned() }
            } else {
                parse_quote! { #iter_expr.iter().cloned() }
            }
        }
    }
}

fn apply_csv_iteration(
    iter_expr: &mut syn::Expr,
    ctx: &mut CodeGenContext,
) -> bool {
    let mut csv_yields_results = false;
    if let Some(pattern) = ctx.stdlib_mappings.get_iteration_pattern("csv", "DictReader") {
        if let crate::stdlib_mappings::RustPattern::IterationPattern {
            yields_results, ..
        } = pattern
        {
            csv_yields_results = *yields_results;
        }
        let rust_code =
            pattern.generate_rust_code(&iter_expr.to_token_stream().to_string(), &[]);
        if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
            ctx.needs_csv = true;
            *iter_expr = expr;
        }
    }
    csv_yields_results
}

pub(crate) fn codegen_for_stmt(
    target: &AssignTarget,
    iter: &HirExpr,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

    trace_decision!(
        category = DecisionCategory::BorrowStrategy,
        name = "for_loop_iter",
        chosen = "for_in_iter",
        alternatives = ["iter", "into_iter", "iter_mut", "drain", "range"],
        confidence = 0.88
    );

    track_range_loop_var(target, iter, ctx);
    track_collection_loop_var(target, iter, body, ctx);
    track_char_counter_iter(target, iter, ctx);

    let target_pattern: syn::Pat = generate_for_target_pattern(target, body)?;

    // GH-207: Handle dict method calls (items/keys/values) specially for iteration
    let mut iter_expr = if let HirExpr::MethodCall { object, method, .. } = iter {
        if method == "items" {
            let obj_expr = object.to_rust_expr(ctx)?;
            parse_quote! { #obj_expr.iter().map(|(k, v)| (k.clone(), v.clone())) }
        } else if method == "keys" {
            if let AssignTarget::Symbol(name) = target {
                ctx.fn_str_params.insert(name.clone());
            }
            let obj_expr = object.to_rust_expr(ctx)?;
            parse_quote! { #obj_expr.keys() }
        } else if method == "values" {
            let obj_expr = object.to_rust_expr(ctx)?;
            parse_quote! { #obj_expr.values() }
        } else {
            iter.to_rust_expr(ctx)?
        }
    } else {
        iter.to_rust_expr(ctx)?
    };

    // DEPYLER-1082: Handle generator iterator state vars
    if ctx.in_generator {
        if let HirExpr::Var(var_name) = iter {
            if ctx.generator_iterator_state_vars.contains(var_name) {
                let var_ident = safe_ident(var_name);
                ctx.enter_scope();
                if let AssignTarget::Symbol(name) = target {
                    ctx.declare_var(name);
                }
                let body_stmts: Vec<_> =
                    body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
                ctx.exit_scope();
                ctx.is_final_statement = saved_is_final;
                return Ok(quote! {
                    while let Some(#target_pattern) = self.#var_ident.next() {
                        #(#body_stmts)*
                    }
                });
            }
        }
    }

    let is_stdin_iter = is_stdin_iteration(iter);
    let is_file_iter = is_file_iteration(iter);
    let is_csv_reader = is_csv_reader_iteration(iter);

    if is_stdin_iter {
        iter_expr = apply_stdin_iteration(iter_expr);
    } else if is_file_iter {
        iter_expr = apply_file_iteration(iter_expr, ctx);
    }

    let mut csv_yields_results = false;
    if !is_stdin_iter && !is_file_iter && is_csv_reader {
        csv_yields_results = apply_csv_iteration(&mut iter_expr, ctx);
    }

    // Apply collection/string/json iteration strategy for variable iterators
    if !is_stdin_iter && !is_file_iter && !is_csv_reader {
        if let HirExpr::Var(var_name) = iter {
            iter_expr = apply_var_iteration_strategy(var_name, target, iter_expr, ctx);
        }

        // DEPYLER-1189: Handle dict index expression iteration
        if let HirExpr::Index { base, .. } = iter {
            if let HirExpr::Var(base_name) = base.as_ref() {
                if let Some(Type::Dict(_, value_type)) = ctx.var_types.get(base_name) {
                    if matches!(value_type.as_ref(), Type::Dict(_, _)) {
                        iter_expr = parse_quote! { #iter_expr.keys().cloned() };
                    }
                }
            }
        }

        // DEPYLER-1045: Handle string method calls returning String → need .chars()
        if let HirExpr::MethodCall { method, .. } = iter {
            let is_string_returning_method = matches!(
                method.as_str(),
                "lower" | "upper" | "strip" | "lstrip" | "rstrip"
                    | "capitalize" | "title" | "swapcase" | "casefold" | "replace"
            );
            if is_string_returning_method {
                iter_expr = parse_quote! { #iter_expr.chars() };
                if let AssignTarget::Symbol(loop_var_name) = target {
                    ctx.char_iter_vars.insert(loop_var_name.clone());
                }
            }
        }
    }

    // DEPYLER-0607: Handle JSON Value iteration
    if !is_stdin_iter && !is_file_iter && !is_csv_reader {
        let is_json_value_iteration = detect_json_value_iteration(iter, ctx);
        let iter_expr_str = quote!(#iter_expr).to_string();
        let has_value_pattern = iter_expr_str.contains("unwrap_or_default")
            || iter_expr_str.contains("unwrap_or (")
            || (iter_expr_str.contains(".get") && iter_expr_str.contains(".cloned"));
        if is_json_value_iteration || (has_value_pattern && ctx.needs_serde_json) {
            iter_expr = parse_quote! {
                #iter_expr.as_array().unwrap_or(&vec![]).iter().cloned()
            };
        }
    }

    ctx.enter_scope();

    let element_type = extract_iterator_element_type(iter, ctx);
    declare_loop_vars_with_types(target, element_type, ctx);
    let body_stmts: Vec<_> =
        body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();
    ctx.is_final_statement = saved_is_final;

    let needs_enumerate_cast = needs_enumerate_index_cast(iter, target);
    let needs_char_to_string = needs_char_to_string_conversion(target, iter, body, ctx);

    if needs_enumerate_cast {
        Ok(generate_enumerate_cast_loop(target, &target_pattern, &iter_expr, body, &body_stmts))
    } else if needs_char_to_string {
        Ok(generate_char_to_string_loop(target, &target_pattern, &iter_expr, &body_stmts))
    } else if csv_yields_results {
        Ok(generate_csv_result_loop(target, &target_pattern, &iter_expr, &body_stmts))
    } else {
        Ok(quote! {
            for #target_pattern in #iter_expr {
                #(#body_stmts)*
            }
        })
    }
}
