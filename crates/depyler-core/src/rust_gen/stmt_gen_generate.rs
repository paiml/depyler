pub(crate) fn codegen_assert_stmt(
    test: &HirExpr,
    msg: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-1005: For binary comparison expressions, generate assert_eq!/assert_ne! macros
    // to avoid the = = tokenization issue when syn::ExprBinary is interpolated into quote!
    if let HirExpr::Binary { op, left, right } = test {
        match op {
            BinOp::Eq => {
                let left_expr = unwrap_result_in_assert(left, ctx)?;
                let right_expr = unwrap_result_in_assert(right, ctx)?;
                if let Some(message_expr) = msg {
                    let msg_tokens = message_expr.to_rust_expr(ctx)?;
                    return Ok(quote! { assert_eq!(#left_expr, #right_expr, "{}", #msg_tokens); });
                } else {
                    return Ok(quote! { assert_eq!(#left_expr, #right_expr); });
                }
            }
            BinOp::NotEq => {
                let left_expr = unwrap_result_in_assert(left, ctx)?;
                let right_expr = unwrap_result_in_assert(right, ctx)?;
                if let Some(message_expr) = msg {
                    let msg_tokens = message_expr.to_rust_expr(ctx)?;
                    return Ok(quote! { assert_ne!(#left_expr, #right_expr, "{}", #msg_tokens); });
                } else {
                    return Ok(quote! { assert_ne!(#left_expr, #right_expr); });
                }
            }
            _ => {} // Fall through to default handling
        }
    }

    let test_expr = test.to_rust_expr(ctx)?;

    if let Some(message_expr) = msg {
        let msg_tokens = message_expr.to_rust_expr(ctx)?;
        Ok(quote! { assert!(#test_expr, "{}", #msg_tokens); })
    } else {
        Ok(quote! { assert!(#test_expr); })
    }
}

fn unwrap_result_in_assert(expr: &HirExpr, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    let rust_expr = expr.to_rust_expr(ctx)?;
    if is_call_to_result_returning_fn(expr, ctx) {
        Ok(syn::parse_quote! { #rust_expr.unwrap() })
    } else {
        Ok(rust_expr)
    }
}
