fn check_needs_clone(
    value_expr: &syn::Expr,
    symbol: &str,
    ctx: &CodeGenContext,
) -> bool {
    if let syn::Expr::Path(ref path) = value_expr {
        if path.path.segments.len() == 1 {
            let var_name = path.path.segments[0].ident.to_string();
            return ctx.subcommand_match_fields.contains(&var_name)
                || (ctx.is_declared(&var_name) && var_name != symbol);
        }
    } else if let syn::Expr::Field(ref field) = value_expr {
        if let syn::Expr::Path(ref base_path) = *field.base {
            if base_path.path.segments.len() == 1 {
                let base_name = base_path.path.segments[0].ident.to_string();
                return ctx.is_declared(&base_name);
            }
        }
    }
    false
}

fn check_is_str_param(value_expr: &syn::Expr, ctx: &CodeGenContext) -> bool {
    if let syn::Expr::Path(ref path) = value_expr {
        if path.path.segments.len() == 1 {
            let var_name = path.path.segments[0].ident.to_string();
            return ctx.fn_str_params.contains(&var_name);
        }
    }
    false
}

fn codegen_first_declaration(
    symbol: &str,
    target_ident: &syn::Ident,
    value_expr: syn::Expr,
    type_annotation_tokens: Option<proc_macro2::TokenStream>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    ctx.declare_var(symbol);
    if ctx.mutable_vars.contains(symbol) {
        let is_str_param = check_is_str_param(&value_expr, ctx);
        let needs_clone = check_needs_clone(&value_expr, symbol, ctx);

        let init_expr = if is_str_param {
            parse_quote! { #value_expr.to_string() }
        } else if needs_clone {
            parse_quote! { #value_expr.clone() }
        } else {
            value_expr
        };

        if let Some(type_ann) = type_annotation_tokens {
            Ok(quote! { let mut #target_ident #type_ann = #init_expr; })
        } else {
            Ok(quote! { let mut #target_ident = #init_expr; })
        }
    } else if let Some(type_ann) = type_annotation_tokens {
        Ok(quote! { let #target_ident #type_ann = #value_expr; })
    } else {
        Ok(quote! { let #target_ident = #value_expr; })
    }
}
