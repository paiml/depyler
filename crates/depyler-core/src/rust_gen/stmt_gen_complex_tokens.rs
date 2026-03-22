fn try_codegen_zero_div_pattern(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<Option<proc_macro2::TokenStream>> {
    let has_zero_div_handler =
        handlers.iter().any(|h| h.exception_type.as_deref() == Some("ZeroDivisionError"));

    if !has_zero_div_handler || body.len() != 1 {
        return Ok(None);
    }

    let HirStmt::Return(Some(expr)) = &body[0] else {
        return Ok(None);
    };

    if !contains_floor_div(expr) {
        return Ok(None);
    }

    let divisor_expr = extract_divisor_from_floor_div(expr)?;
    let divisor_tokens = divisor_expr.to_rust_expr(ctx)?;

    let zero_div_handler_idx = handlers
        .iter()
        .position(|h| h.exception_type.as_deref() == Some("ZeroDivisionError"))
        .unwrap();

    ctx.enter_scope();
    let old_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;
    let handler_stmts: Vec<_> = handlers[zero_div_handler_idx]
        .body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.is_final_statement = old_is_final;
    ctx.exit_scope();

    let floor_div_result = expr.to_rust_expr(ctx)?;

    let return_expr = if ctx.current_function_can_fail {
        quote! { return Ok(#floor_div_result); }
    } else {
        quote! { return #floor_div_result; }
    };

    if let Some(finalbody) = finalbody {
        ctx.enter_scope();
        let finally_stmts: Vec<_> = finalbody
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();
        Ok(Some(quote! {
            {
                if #divisor_tokens == 0 {
                    #(#handler_stmts)*
                } else {
                    #return_expr
                }
                #(#finally_stmts)*
            }
        }))
    } else {
        Ok(Some(quote! {
            if #divisor_tokens == 0 {
                #(#handler_stmts)*
            } else {
                #return_expr
            }
        }))
    }
}

    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        match self {
            HirStmt::Assign { target, value, type_annotation } => {
                codegen_assign_stmt(target, value, type_annotation, ctx)
            }
            HirStmt::Return(expr) => codegen_return_stmt(expr, ctx),
            HirStmt::If { condition, then_body, else_body } => {
                codegen_if_stmt(condition, then_body, else_body, ctx)
            }
            HirStmt::While { condition, body } => codegen_while_stmt(condition, body, ctx),
            HirStmt::For { target, iter, body } => codegen_for_stmt(target, iter, body, ctx),
            HirStmt::Expr(expr) => codegen_expr_stmt(expr, ctx),
            HirStmt::Raise { exception, cause: _ } => codegen_raise_stmt(exception, ctx),
            HirStmt::Break { label } => codegen_break_stmt(label),
            HirStmt::Continue { label } => codegen_continue_stmt(label),
            HirStmt::With { context, target, body, is_async } => {
                codegen_with_stmt(context, target, body, *is_async, ctx)
            }
            HirStmt::Try { body, handlers, orelse: _, finalbody } => {
                codegen_try_stmt(body, handlers, finalbody, ctx)
            }
            HirStmt::Assert { test, msg } => codegen_assert_stmt(test, msg, ctx),
            HirStmt::Pass => codegen_pass_stmt(),
            // DEPYLER-0614: Handle Block of statements (for multi-target assignment: i = j = 0)
            HirStmt::Block(stmts) => {
                let mut tokens = proc_macro2::TokenStream::new();
                for stmt in stmts {
                    tokens.extend(stmt.to_rust_tokens(ctx)?);
                }
                Ok(tokens)
            }
            HirStmt::FunctionDef { name, params, ret_type, body, docstring: _ } => {
                codegen_nested_function_def(name, params, ret_type, body, ctx)
            }
        }
    }
