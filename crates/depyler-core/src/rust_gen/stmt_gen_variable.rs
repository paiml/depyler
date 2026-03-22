pub(crate) fn codegen_if_stmt(
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    use std::collections::HashSet;

    // DEPYLER-1400: Handle `if TYPE_CHECKING:` blocks
    if let Some(elided) = try_elide_type_checking(condition) {
        return Ok(elided);
    }

    // CITL: Trace if statement pattern decision
    trace_decision!(
        category = DecisionCategory::TypeMapping,
        name = "if_statement",
        chosen = "if_else",
        alternatives = ["match_pattern", "if_let", "guard", "ternary"],
        confidence = 0.85
    );

    // DEPYLER-0399: Detect subcommand dispatch pattern and convert to match
    if ctx.argparser_tracker.has_subcommands() {
        if let Some(match_stmt) =
            try_generate_subcommand_match(condition, then_body, else_body, ctx)?
        {
            return Ok(match_stmt);
        }
    }

    // DEPYLER-0627: Detect Option variable truthiness check and generate if-let pattern
    if let HirExpr::Var(var_name) = condition {
        if let Some(var_type) = ctx.var_types.get(var_name) {
            if matches!(var_type, Type::Optional(_)) {
                return codegen_option_if_let(var_name, then_body, else_body, ctx);
            }
        }
    }

    // DEPYLER-0188: Extract walrus operator assignments from condition
    let (walrus_assigns, simplified_condition) = extract_walrus_from_condition(condition);
    let walrus_lets = codegen_walrus_lets(&walrus_assigns, ctx)?;

    let mut cond = prepare_if_condition(&simplified_condition, condition, ctx)?;

    // DEPYLER-0339: Apply Python truthiness conversion
    cond = apply_truthiness_conversion(condition, cond, ctx);

    // DEPYLER-0379 + DEPYLER-0823: Collect hoisted variables
    let hoisted_vars = collect_hoisted_vars(then_body, else_body, ctx);

    // DEPYLER-0379: Generate hoisted variable declarations
    let hoisted_decls = generate_hoisted_decls(&hoisted_vars, then_body, else_body, ctx)?;

    // DEPYLER-0935: Save and restore is_final_statement for if body
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

    ctx.enter_scope();
    let then_stmts: Vec<_> =
        then_body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    let result = if let Some(else_stmts) = else_body {
        ctx.enter_scope();
        let else_tokens: Vec<_> =
            else_stmts.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();
        Ok(quote! {
            #(#walrus_lets)*
            #(#hoisted_decls)*
            if #cond {
                #(#then_stmts)*
            } else {
                #(#else_tokens)*
            }
        })
    } else {
        Ok(quote! {
            #(#walrus_lets)*
            #(#hoisted_decls)*
            if #cond {
                #(#then_stmts)*
            }
        })
    };

    // DEPYLER-0935: Restore is_final_statement flag
    ctx.is_final_statement = saved_is_final;

    // DEPYLER-0455 #2: Clean up hoisted inference vars after if-statement
    for var_name in &hoisted_vars {
        ctx.hoisted_inference_vars.remove(var_name);
    }

    // DEPYLER-1151: Detect None-check-early-exit pattern to narrow Option variables
    if else_body.is_none() {
        if let Some(var_name) = detect_none_check_variable(condition) {
            if is_early_exit_body(then_body) {
                ctx.narrowed_option_vars.insert(var_name);
            }
        }
    }

    result
}

fn try_elide_type_checking(condition: &HirExpr) -> Option<proc_macro2::TokenStream> {
    if let HirExpr::Var(var_name) = condition {
        if var_name == "TYPE_CHECKING" {
            trace_decision!(
                category = DecisionCategory::TypeMapping,
                name = "type_checking_elision",
                chosen = "skip_block",
                alternatives = ["emit_false_const", "keep_as_is"],
                confidence = 0.95
            );
            return Some(proc_macro2::TokenStream::new());
        }
    }
    None
}

fn codegen_walrus_lets(
    walrus_assigns: &[(String, HirExpr)],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut walrus_lets = Vec::new();
    for (name, value_expr) in walrus_assigns {
        let var_ident = safe_ident(name);
        let value_tokens = value_expr.to_rust_expr(ctx)?;
        walrus_lets.push(quote! { let #var_ident = #value_tokens; });
        ctx.declare_var(name);
    }
    Ok(walrus_lets)
}

fn prepare_if_condition(
    simplified_condition: &HirExpr,
    original_condition: &HirExpr,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    // DEPYLER-0844: isinstance(x, T) → true
    let mut cond = if let HirExpr::Call { func, .. } = simplified_condition {
        if func == "isinstance" {
            parse_quote! { true }
        } else {
            simplified_condition.to_rust_expr(ctx)?
        }
    } else {
        simplified_condition.to_rust_expr(ctx)?
    };

    // DEPYLER-0308: Auto-unwrap Result<bool> in if conditions
    if let HirExpr::Call { func, .. } = original_condition {
        if ctx.result_bool_functions.contains(func) {
            cond = parse_quote! { #cond.unwrap_or(false) };
        }
    }

    // DEPYLER-99MODE-S9: Auto-unwrap Result-returning calls in truthiness contexts
    if let HirExpr::Call { func, .. } = original_condition {
        if ctx.type_mapper.nasa_mode
            && ctx.result_returning_functions.contains(func)
            && ctx.current_function_can_fail
            && !ctx.result_bool_functions.contains(func)
        {
            cond = parse_quote! { #cond? };
        }
    }

    Ok(cond)
}

fn collect_hoisted_vars(
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &CodeGenContext,
) -> std::collections::HashSet<String> {
    use std::collections::HashSet;

    let then_vars = extract_toplevel_assigned_symbols(then_body);
    let mut hoisted_vars: HashSet<String> = if let Some(else_stmts) = else_body {
        let else_vars = extract_toplevel_assigned_symbols(else_stmts);
        then_vars.intersection(&else_vars).cloned().collect()
    } else {
        HashSet::new()
    };

    // DEPYLER-0823: Add None-placeholder variables
    for var_name in &then_vars {
        if ctx.none_placeholder_vars.contains(var_name) {
            hoisted_vars.insert(var_name.clone());
        }
    }
    if let Some(else_stmts) = else_body {
        let else_vars = extract_toplevel_assigned_symbols(else_stmts);
        for var_name in &else_vars {
            if ctx.none_placeholder_vars.contains(var_name) {
                hoisted_vars.insert(var_name.clone());
            }
        }
    }

    hoisted_vars
}

fn detect_none_check_variable(condition: &HirExpr) -> Option<String> {
    match condition {
        // Pattern: x.is_none() -> MethodCall { object: Var(x), method: "is_none" }
        // This also handles `x is None` which is converted to x.is_none() by AST bridge
        HirExpr::MethodCall { object, method, .. } if method == "is_none" => {
            if let HirExpr::Var(var_name) = object.as_ref() {
                return Some(var_name.clone());
            }
            None
        }
        // Pattern: `not x` where x is Optional -> Unary { op: Not, operand: Var(x) }
        HirExpr::Unary { op: UnaryOp::Not, operand } => {
            if let HirExpr::Var(var_name) = operand.as_ref() {
                // This could be a None check if var is Optional
                // We'll verify it's Optional when we use it
                return Some(var_name.clone());
            }
            None
        }
        _ => None,
    }
}

fn is_early_exit_body(body: &[HirStmt]) -> bool {
    if body.len() != 1 {
        return false;
    }
    matches!(
        &body[0],
        HirStmt::Return(_)
            | HirStmt::Break { .. }
            | HirStmt::Continue { .. }
            | HirStmt::Raise { .. }
    )
}

fn codegen_option_if_let(
    var_name: &str,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Generate the variable identifier (handle Rust keywords)
    let var_ident = safe_ident(var_name);
    // Generate the unwrapped variable name
    let unwrapped_name = format!("{}_val", var_name);
    let unwrapped_ident = safe_ident(&unwrapped_name);

    // DEPYLER-0645: Inside generators, state variables need self. prefix
    let var_expr: proc_macro2::TokenStream =
        if ctx.in_generator && ctx.generator_state_vars.contains(var_name) {
            quote! { self.#var_ident }
        } else {
            quote! { #var_ident }
        };

    // Add mapping so variable references inside body use unwrapped name
    ctx.option_unwrap_map.insert(var_name.to_string(), unwrapped_name.clone());

    // Process then body with the mapping active
    ctx.enter_scope();
    let then_stmts: Vec<_> =
        then_body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    // Remove the mapping
    ctx.option_unwrap_map.remove(var_name);

    // Generate if-let pattern
    let result = if let Some(else_stmts) = else_body {
        ctx.enter_scope();
        let else_tokens: Vec<_> =
            else_stmts.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();

        quote! {
            if let Some(ref #unwrapped_ident) = #var_expr {
                #(#then_stmts)*
            } else {
                #(#else_tokens)*
            }
        }
    } else {
        quote! {
            if let Some(ref #unwrapped_ident) = #var_expr {
                #(#then_stmts)*
            }
        }
    };

    Ok(result)
}
