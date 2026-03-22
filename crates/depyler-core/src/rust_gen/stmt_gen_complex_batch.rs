pub(crate) fn codegen_try_stmt(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // CITL: Trace error handling strategy
    trace_decision!(
        category = DecisionCategory::ErrorHandling,
        name = "try_except",
        chosen = "match_result",
        alternatives = ["unwrap_or", "question_mark", "anyhow_context", "custom_error"],
        confidence = 0.80
    );

    // CB-200 Batch 12: Hoist variable declarations for try/except blocks
    let hoisted_decls = generate_hoisted_try_decls(body, handlers, ctx)?;

    // DEPYLER-0578: Detect json.load(sys.stdin) pattern with exit handler
    if let Some(result) = try_generate_json_stdin_match(body, handlers, finalbody, ctx)? {
        return Ok(result);
    }

    // CB-200 Batch 12: Detect simple try-except pattern for optimization
    let simple_pattern_info = detect_simple_try_except_pattern(body, handlers);

    // DEPYLER-0333: Extract handled exception types for scope tracking
    let handled_types: Vec<String> =
        handlers.iter().filter_map(|h| h.exception_type.clone()).collect();

    // DEPYLER-0333: Enter try block scope with handled exception types
    // Empty list means bare except (catches all exceptions)
    ctx.enter_try_scope(handled_types.clone());

    // CB-200 Batch 14: Check for floor division with ZeroDivisionError handler
    if let Some(result) = try_codegen_zero_div_pattern(body, handlers, finalbody, ctx)? {
        ctx.exit_exception_scope();
        return Ok(result);
    }

    // Convert try body to statements
    // DEPYLER-0395: Try block statements should include 'return' keyword
    // Save and temporarily disable is_final_statement so return statements
    // in try blocks get the explicit 'return' keyword (needed for proper exception handling)
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

    ctx.enter_scope();
    let try_stmts: Vec<_> =
        body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    // Restore is_final_statement flag
    ctx.is_final_statement = saved_is_final;

    // DEPYLER-0333: Exit try block scope
    ctx.exit_exception_scope();

    // Generate except handler code
    let mut handler_tokens = Vec::new();
    for handler in handlers {
        // DEPYLER-0333: Enter handler scope for each except clause
        ctx.enter_handler_scope();
        ctx.enter_scope();

        // If there's a name binding, declare it in scope
        if let Some(var_name) = &handler.name {
            ctx.declare_var(var_name);
        }

        // DEPYLER-0357: Handler statements should include 'return' keyword
        // Save and temporarily disable is_final_statement so return statements
        // in handlers get the explicit 'return' keyword (needed for proper exception handling)
        let saved_is_final = ctx.is_final_statement;
        ctx.is_final_statement = false;

        let handler_stmts: Vec<_> =
            handler.body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;

        // Restore is_final_statement flag
        ctx.is_final_statement = saved_is_final;
        ctx.exit_scope();
        // DEPYLER-0333: Exit handler scope
        ctx.exit_exception_scope();

        // DEPYLER-0931: Transform handler returns to wrap in Ok() when inside nested exception scope
        // Handler code returns from the outer try/except closure which expects Result<T, E>
        let is_nested = ctx.exception_nesting_depth() > 0;
        let handler_stmts_transformed: Vec<_> = if is_nested {
            handler_stmts
                .iter()
                .map(|stmt| {
                    let stmt_str = stmt.to_string();
                    if stmt_str.starts_with("return ") && !stmt_str.starts_with("return Ok (") {
                        if let Some(expr_part) = stmt_str.strip_prefix("return ") {
                            if let Some(expr) = expr_part.strip_suffix(" ;") {
                                let wrapped = format!("return Ok({}) ;", expr);
                                return wrapped.parse().unwrap_or_else(|_| stmt.clone());
                            }
                        }
                    }
                    stmt.clone()
                })
                .collect()
        } else {
            handler_stmts
        };

        handler_tokens.push(quote! { #(#handler_stmts_transformed)* });
    }

    // Generate finally clause if present
    let finally_stmts = if let Some(finally_body) = finalbody {
        let stmts: Vec<_> =
            finally_body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
        Some(quote! { #(#stmts)* })
    } else {
        None
    };

    // Generate try/except/finally pattern
    if handlers.is_empty() {
        // Try/finally without except
        if let Some(finally_code) = finally_stmts {
            Ok(quote! {
                #(#hoisted_decls)*
                {
                    #(#try_stmts)*
                    #finally_code
                }
            })
        } else {
            // DEPYLER-0681: Include hoisted declarations for try block variables
            Ok(quote! {
                #(#hoisted_decls)*
                #(#try_stmts)*
            })
        }
    } else {
        // DEPYLER-0437/0429: Generate proper match expressions for parse() patterns
        // Check if try_stmts contains a .parse() call that we can convert to match
        if handlers.len() == 1 {
            if let Some((var_name, parse_expr_str, remaining_stmts)) =
                extract_parse_from_tokens(&try_stmts)
            {
                // Parse the expression string back to token stream
                let parse_expr: proc_macro2::TokenStream = match parse_expr_str.parse() {
                    Ok(ts) => ts,
                    Err(_) => return Ok(quote! { #(#try_stmts)* }), // Fallback on parse error
                };
                let ok_var = safe_ident(&var_name);

                // Generate Ok branch (remaining statements after parse)
                let ok_body = quote! { #(#remaining_stmts)* };

                // Generate Err branch (handler body)
                let err_body = &handler_tokens[0];

                // DEPYLER-0429: Check if exception variable should be bound
                let err_pattern = if let Some(exc_var) = &handlers[0].name {
                    // Bind exception variable: Err(e) => { ... }
                    let exc_ident = safe_ident(exc_var);
                    quote! { Err(#exc_ident) }
                } else {
                    // No exception variable: Err(_) => { ... }
                    quote! { Err(_) }
                };

                // Build match expression
                let match_expr = quote! {
                    match #parse_expr {
                        Ok(#ok_var) => { #ok_body },
                        #err_pattern => { #err_body }
                    }
                };

                // Wrap with finally if present
                if let Some(finally_code) = finally_stmts {
                    return Ok(quote! {
                        {
                            #match_expr
                            #finally_code
                        }
                    });
                } else {
                    return Ok(match_expr);
                }
            }
        }

        // Fall through to existing simple_pattern_info logic
        if let Some((exception_value_str, _exception_type)) = simple_pattern_info {
            // Fall through to existing unwrap_or logic if not a match pattern
            // Convert try_stmts to string to post-process
            let try_code = quote! { #(#try_stmts)* };
            let try_str = try_code.to_string();

            // DEPYLER-0358: Replace unwrap_or_default() with unwrap_or(exception_value)
            // This handles the case where int(str) generates .parse().unwrap_or_default()
            // but we want .parse().unwrap_or(-1) based on the except clause
            if try_str.contains("unwrap_or_default") {
                // Parse the try code and replace unwrap_or_default with unwrap_or(value)
                // Handle both "unwrap_or_default ()" and "unwrap_or_default()"
                let fixed_code = try_str
                    .replace(
                        "unwrap_or_default ()",
                        &format!("unwrap_or ({})", exception_value_str),
                    )
                    .replace("unwrap_or_default()", &format!("unwrap_or({})", exception_value_str));

                // Parse back to token stream
                let fixed_tokens: proc_macro2::TokenStream = fixed_code.parse().unwrap_or(try_code);

                // DEPYLER-0437: Include hoisted variable declarations
                if let Some(finally_code) = finally_stmts {
                    Ok(quote! {
                        {
                            #(#hoisted_decls)*
                            #fixed_tokens
                            #finally_code
                        }
                    })
                } else if hoisted_decls.is_empty() {
                    Ok(fixed_tokens)
                } else {
                    Ok(quote! {
                        #(#hoisted_decls)*
                        #fixed_tokens
                    })
                }
            } else {
                // Pattern matched but no unwrap_or_default found
                // This means it's not a parse operation, so fall through to normal concatenation
                // to include the exception handler code
                // DEPYLER-0437: Include hoisted variable declarations
                let handler_code = &handler_tokens[0];
                if let Some(finally_code) = finally_stmts {
                    Ok(quote! {
                        {
                            #(#hoisted_decls)*
                            #(#try_stmts)*
                            #handler_code
                            #finally_code
                        }
                    })
                } else {
                    Ok(quote! {
                        {
                            #(#hoisted_decls)*
                            #(#try_stmts)*
                            #handler_code
                        }
                    })
                }
            }
        } else {
            // DEPYLER-0931: Always use closure pattern for robust control flow & scoping
            // This guarantees that:
            // 1. Variables are hoisted and accessible after the block (declared outside)
            // 2. Control flow (return/raise) inside try block correctly jumps to handler or exits
            // 3. Variables assigned in try block are correctly captured by mutable reference

            // Infer return type from try body
            let try_return_type = infer_try_body_return_type(body, ctx);
            let return_type_tokens = try_return_type
                .as_ref()
                .map(try_return_type_to_tokens)
                .unwrap_or_else(|| quote! { () });
            let ok_value = try_return_type
                .as_ref()
                .map(|_| quote! { _result })
                .unwrap_or_else(|| quote! { () });

            // The Ok arm extracts _result from Result
            // DEPYLER-0819: When handlers contain raise, the function returns Result<T, E>
            // and we must wrap the success value in Ok()
            // DEPYLER-0931: Always use Ok(_result) when returning from try/except closure
            // because we're inside a Result-returning closure (even for nested try/except)
            let any_handler_raises = handlers.iter().any(|h| handler_contains_raise(&h.body));
            let ok_arm_body = if try_return_type.is_some() {
                // Always wrap in Ok() - we're returning from a Result<T, E> closure
                // If any_handler_raises OR outer function returns Result, we must wrap in Ok()
                if any_handler_raises
                    || ctx.exception_nesting_depth() > 0
                    || ctx.current_function_can_fail
                {
                    quote! { return Ok(_result); }
                } else {
                    quote! { return _result; }
                }
            } else {
                quote! {}
            };

            // Transform try body return statements to wrap values in Ok()
            // The closure returns Result<T, E>, so `return expr;` must become `return Ok(expr);`
            let try_stmts_transformed: Vec<_> = try_stmts
                .iter()
                .map(|stmt| {
                    let stmt_str = stmt.to_string();
                    // Transform `return expr ;` to `return Ok ( expr ) ;`
                    // Simple text-based transformation for now (robust enough for generated code)
                    if stmt_str.starts_with("return ") && !stmt_str.starts_with("return Ok (") {
                        if let Some(expr_part) = stmt_str.strip_prefix("return ") {
                            if let Some(expr) = expr_part.strip_suffix(" ;") {
                                let wrapped = format!("return Ok({}) ;", expr);
                                return wrapped.parse().unwrap_or_else(|_| stmt.clone());
                            }
                        }
                    }
                    stmt.clone()
                })
                .collect();

            // Check if try body always returns (to avoid unreachable code warning)
            let always_returns = body.iter().any(|s| matches!(s, HirStmt::Return(_)));

            // Only add fallback Ok(Default::default()) when try body has no return
            // If try body has returns, they're already wrapped in Ok() and there's no need for fallback
            let closure_fallback = if always_returns {
                quote! {}
            } else if try_return_type.is_none() {
                quote! { Ok(()) } // Return unit for fallthrough
            } else {
                // If try body returns a value, we need a fallback for fallthrough path
                // (e.g., if try block finishes without returning)
                // Use Default::default() for the return type
                quote! { Ok(Default::default()) }
            };

            // Generate handler matching logic
            let match_expr = if handlers.len() == 1 {
                // Single handler - use match pattern
                let err_pattern = if let Some(exc_var) = &handlers[0].name {
                    let exc_ident = safe_ident(exc_var);
                    quote! { Err(#exc_ident) }
                } else {
                    quote! { Err(_) }
                };

                let handler_code = &handler_tokens[0];

                quote! {
                    match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                        #(#try_stmts_transformed)*
                        #closure_fallback
                    })() {
                        Ok(#ok_value) => { #ok_arm_body },
                        #err_pattern => { #handler_code }
                    }
                }
            } else {
                // Multiple handlers - find one with binding or fallback to catch-all
                // Note: Implement proper type-based dispatch for multiple handlers
                let exc_var_opt = handlers.iter().find_map(|h| h.name.as_ref());
                let handler_code = if let Some(idx) = handlers.iter().position(|h| h.name.is_some())
                {
                    &handler_tokens[idx]
                } else {
                    &handler_tokens[0]
                };

                if let Some(exc_var) = exc_var_opt {
                    let exc_ident = safe_ident(exc_var);
                    quote! {
                        match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                            #(#try_stmts_transformed)*
                            #closure_fallback
                        })() {
                            Ok(#ok_value) => { #ok_arm_body },
                            Err(#exc_ident) => { #handler_code }
                        }
                    }
                } else {
                    quote! {
                        match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                            #(#try_stmts_transformed)*
                            #closure_fallback
                        })() {
                            Ok(#ok_value) => { #ok_arm_body },
                            Err(_) => { #handler_code }
                        }
                    }
                }
            };

            // DEPYLER-0931: Emit hoisted declarations OUTSIDE the match/closure
            // This ensures variables are captured by mutable reference and retain values
            // after the try/except block.
            if let Some(finally_code) = finally_stmts {
                Ok(quote! {
                    #(#hoisted_decls)*
                    {
                        #match_expr
                        #finally_code
                    }
                })
            } else {
                Ok(quote! {
                    #(#hoisted_decls)*
                    #match_expr
                })
            }
        }
    }
}

fn generate_hoisted_try_decls(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let try_vars = extract_toplevel_assigned_symbols(body);
    let handler_vars: std::collections::HashSet<String> =
        handlers.iter().flat_map(|h| extract_toplevel_assigned_symbols(&h.body)).collect();

    let hoisted_try_vars: Vec<String> =
        try_vars.union(&handler_vars).filter(|v| !ctx.is_declared(v)).cloned().collect();

    let mut hoisted_decls = Vec::new();
    for var_name in &hoisted_try_vars {
        let var_ident = safe_ident(var_name);

        let var_type = find_variable_type(var_name, body)
            .or_else(|| handlers.iter().find_map(|h| find_variable_type(var_name, &h.body)));

        if let Some(ty) = var_type {
            let needs_option_wrap = matches!(
                &ty,
                Type::Custom(s) if s == "std::process::Child" || s == "Child"
            );

            if needs_option_wrap {
                let opt_type = Type::Optional(Box::new(ty.clone()));
                let rust_type = ctx.type_mapper.map_type(&opt_type);
                let syn_type = rust_type_to_syn(&rust_type)?;
                hoisted_decls.push(quote! { let mut #var_ident: #syn_type = None; });
                ctx.var_types.insert(var_name.clone(), opt_type);
            } else {
                let rust_type = ctx.type_mapper.map_type(&ty);
                let syn_type = rust_type_to_syn(&rust_type)?;
                hoisted_decls.push(quote! { let mut #var_ident: #syn_type = Default::default(); });
                ctx.var_types.insert(var_name.clone(), ty);
            }
        } else {
            let value_type = crate::hir::Type::Custom("serde_json::Value".to_string());
            let opt_type = crate::hir::Type::Optional(Box::new(value_type));
            ctx.var_types.insert(var_name.clone(), opt_type);
            hoisted_decls.push(quote! { let mut #var_ident: Option<serde_json::Value> = None; });
            ctx.hoisted_inference_vars.insert(var_name.clone());
        }

        ctx.declare_var(var_name);
    }
    Ok(hoisted_decls)
}

fn detect_simple_try_except_pattern(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
) -> Option<(String, Option<String>)> {
    if body.len() != 1 || handlers.len() != 1 || handlers[0].body.len() != 1 || handlers[0].name.is_some() {
        return None;
    }

    match &handlers[0].body[0] {
        HirStmt::Return(Some(HirExpr::Literal(lit))) => Some((
            (match lit {
                Literal::Int(n) => n.to_string(),
                Literal::Float(f) => f.to_string(),
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Bool(b) => b.to_string(),
                _ => "Default::default()".to_string(),
            })
            .to_string(),
            handlers[0].exception_type.clone(),
        )),
        HirStmt::Return(Some(HirExpr::Unary { op, operand })) => {
            if let HirExpr::Literal(lit) = &**operand {
                match (op, lit) {
                    (crate::hir::UnaryOp::Neg, Literal::Int(n)) => {
                        Some((format!("-{}", n), handlers[0].exception_type.clone()))
                    }
                    (crate::hir::UnaryOp::Neg, Literal::Float(f)) => {
                        Some((format!("-{}", f), handlers[0].exception_type.clone()))
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}
