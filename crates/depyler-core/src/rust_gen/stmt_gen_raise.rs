pub(crate) fn codegen_raise_stmt(
    exception: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let exc = match exception {
        Some(exc) => exc,
        None => return Ok(quote! { return Err("Exception raised".into()); }),
    };

    let exc_expr = extract_raise_message(exc, ctx)?;
    let exception_type = extract_exception_type(exc);
    set_exception_type_flag(&exception_type, ctx);

    if ctx.is_exception_handled(&exception_type) {
        return Ok(quote! { panic!("{}", #exc_expr); });
    }

    if !ctx.current_function_can_fail {
        return Ok(quote! { panic!("{}", #exc_expr); });
    }

    let needs_boxing =
        matches!(ctx.current_error_type, Some(crate::rust_gen::context::ErrorType::DynBox));
    let is_already_wrapped = matches!(
        exc,
        HirExpr::Call { func, .. } if func == &exception_type
    );
    let should_wrap = !is_already_wrapped && is_known_exception_type(&exception_type);

    emit_return_err(&exception_type, &exc_expr, needs_boxing, should_wrap)
}

fn extract_raise_message(
    exc: &HirExpr,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    match exc {
        HirExpr::MethodCall { object, method, args, .. }
            if matches!(object.as_ref(), HirExpr::Var(v) if v == "argparse")
                && method == "ArgumentTypeError"
                && !args.is_empty() =>
        {
            args[0].to_rust_expr(ctx)
        }
        HirExpr::Call { func, args, .. } if func == "ArgumentTypeError" && !args.is_empty() => {
            args[0].to_rust_expr(ctx)
        }
        _ => exc.to_rust_expr(ctx),
    }
}

fn set_exception_type_flag(exception_type: &str, ctx: &mut CodeGenContext) {
    match exception_type {
        "ValueError" => ctx.needs_valueerror = true,
        "ArgumentTypeError" => ctx.needs_argumenttypeerror = true,
        "ZeroDivisionError" => ctx.needs_zerodivisionerror = true,
        "IndexError" => ctx.needs_indexerror = true,
        "RuntimeError" => ctx.needs_runtimeerror = true,
        "FileNotFoundError" => ctx.needs_filenotfounderror = true,
        "SyntaxError" => ctx.needs_syntaxerror = true,
        "TypeError" => ctx.needs_typeerror = true,
        "KeyError" => ctx.needs_keyerror = true,
        "IOError" => ctx.needs_ioerror = true,
        "AttributeError" => ctx.needs_attributeerror = true,
        "StopIteration" => ctx.needs_stopiteration = true,
        _ => {}
    }
}

fn is_known_exception_type(exception_type: &str) -> bool {
    matches!(
        exception_type,
        "ValueError"
            | "ArgumentTypeError"
            | "TypeError"
            | "KeyError"
            | "IndexError"
            | "RuntimeError"
            | "FileNotFoundError"
            | "ZeroDivisionError"
            | "SyntaxError"
            | "IOError"
            | "AttributeError"
            | "StopIteration"
    )
}

fn emit_return_err(
    exception_type: &str,
    exc_expr: &syn::Expr,
    needs_boxing: bool,
    should_wrap: bool,
) -> Result<proc_macro2::TokenStream> {
    if needs_boxing {
        emit_boxed_return_err(exception_type, exc_expr, should_wrap)
    } else {
        emit_plain_return_err(exception_type, exc_expr, should_wrap)
    }
}

fn emit_boxed_return_err(
    exception_type: &str,
    exc_expr: &syn::Expr,
    should_wrap: bool,
) -> Result<proc_macro2::TokenStream> {
    if should_wrap {
        let exc_type = safe_ident(exception_type);
        Ok(quote! { return Err(Box::new(#exc_type::new(#exc_expr))); })
    } else {
        Ok(quote! { return Err(Box::new(#exc_expr)); })
    }
}

fn emit_plain_return_err(
    exception_type: &str,
    exc_expr: &syn::Expr,
    should_wrap: bool,
) -> Result<proc_macro2::TokenStream> {
    if should_wrap {
        let exc_type = safe_ident(exception_type);
        Ok(quote! { return Err(#exc_type::new(#exc_expr)); })
    } else {
        Ok(quote! { return Err(#exc_expr); })
    }
}

pub(crate) fn codegen_with_stmt(
    context: &HirExpr,
    target: &Option<String>,
    body: &[HirStmt],
    is_async: bool,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Convert context expression
    let context_expr = context.to_rust_expr(ctx)?;

    // DEPYLER-0357: Save and restore is_final_statement flag so return statements
    // in with blocks get the explicit 'return' keyword (not treated as final statement)
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

    // Convert body statements
    let body_stmts: Vec<_> =
        body.iter().map(|stmt| stmt.to_rust_tokens(ctx)).collect::<Result<_>>()?;

    // Restore is_final_statement flag
    ctx.is_final_statement = saved_is_final;

    // DEPYLER-0387: Detect if context is from open() builtin
    // DEPYLER-0533: Also detect tempfile patterns (NamedTemporaryFile, TemporaryDirectory)
    // These return file-like objects that bind directly without __enter__()
    let is_file_context_manager = matches!(
        context,
        HirExpr::Call { func, .. } if func.as_str() == "open"
    ) || matches!(
        context,
        HirExpr::MethodCall { object, method, .. }
        if matches!(object.as_ref(), HirExpr::Var(module) if module == "tempfile")
            && (method == "NamedTemporaryFile" || method == "TemporaryDirectory"
                || method == "TemporaryFile" || method == "SpooledTemporaryFile"
                || method == "NamedTempFile")
    );

    // Generate code that calls __enter__() for custom context managers
    // or binds File directly for open() calls
    // Note: __exit__() is not yet called (Drop trait implementation pending)
    if let Some(var_name) = target {
        let var_ident = safe_ident(var_name); // DEPYLER-0023
        ctx.declare_var(var_name);

        if is_file_context_manager {
            // DEPYLER-0387: For open() calls, bind File directly (no __enter__)
            // DEPYLER-0533: Also for tempfile patterns
            // DEPYLER-0417: No block wrapper - Python allows accessing variables from with blocks
            // DEPYLER-0458: Add mut to file handles for Read/Write trait methods
            Ok(quote! {
                let mut #var_ident = #context_expr;
                #(#body_stmts)*
            })
        } else if is_async {
            // DEPYLER-0188: For async context managers, call __aenter__().await
            Ok(quote! {
                let mut _context = #context_expr;
                let #var_ident = _context.__aenter__().await;
                #(#body_stmts)*
                // Note: __aexit__().await should be called here (pending Drop trait async support)
            })
        } else {
            // For custom context managers, call __enter__()
            // DEPYLER-0417: No block wrapper - Python allows accessing variables from with blocks
            // DEPYLER-0602: Context variable must be mutable since __enter__ takes &mut self
            Ok(quote! {
                let mut _context = #context_expr;
                let #var_ident = _context.__enter__();
                #(#body_stmts)*
            })
        }
    } else {
        // DEPYLER-0417: No block wrapper - Python allows accessing variables from with blocks
        // DEPYLER-0602: Context variable must be mutable for __enter__() if called
        if is_async {
            Ok(quote! {
                let mut _context = #context_expr;
                let _ = _context.__aenter__().await;
                #(#body_stmts)*
            })
        } else {
            Ok(quote! {
                let mut _context = #context_expr;
                #(#body_stmts)*
            })
        }
    }
}
