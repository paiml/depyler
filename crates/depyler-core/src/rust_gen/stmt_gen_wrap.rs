pub(crate) fn codegen_assign_symbol(
    symbol: &str,
    value_expr: syn::Expr,
    type_annotation_tokens: Option<proc_macro2::TokenStream>,
    is_final: bool,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let target_ident = safe_ident(symbol);

    if ctx.in_generator && ctx.generator_state_vars.contains(symbol) {
        return Ok(quote! { self.#target_ident = #value_expr; });
    }

    if is_final {
        return codegen_final_assign(&target_ident, value_expr, type_annotation_tokens);
    }

    if ctx.is_declared(symbol) {
        return codegen_reassign_symbol(symbol, &target_ident, value_expr, ctx);
    }

    codegen_first_declaration(symbol, &target_ident, value_expr, type_annotation_tokens, ctx)
}

fn codegen_final_assign(
    target_ident: &syn::Ident,
    value_expr: syn::Expr,
    type_annotation_tokens: Option<proc_macro2::TokenStream>,
) -> Result<proc_macro2::TokenStream> {
    if let Some(type_ann) = type_annotation_tokens {
        Ok(quote! { const #target_ident #type_ann = #value_expr; })
    } else {
        Ok(quote! { const #target_ident = #value_expr; })
    }
}

fn normalize_reassign_value(
    value_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> syn::Expr {
    let value_str = quote!(#value_expr).to_string();
    let is_string_literal = value_str.starts_with('"') && value_str.ends_with('"');
    let is_str_param = ctx.fn_str_params.contains(&value_str);
    let is_ref_binding = ctx.subcommand_match_fields.contains(&value_str);
    let is_interned_const = value_str.starts_with("STR_");

    if is_string_literal || is_str_param || is_ref_binding || is_interned_const {
        parse_quote! { #value_expr.to_string() }
    } else {
        value_expr
    }
}

fn try_mut_option_assign(
    symbol: &str,
    target_ident: &syn::Ident,
    value_expr: &syn::Expr,
    ctx: &CodeGenContext,
) -> Option<proc_macro2::TokenStream> {
    let is_mut_option = ctx.mut_option_dict_params.contains(symbol)
        || ctx.mut_option_params.contains(symbol);
    if !is_mut_option {
        return None;
    }
    let value_str = quote!(#value_expr).to_string();
    if value_str.starts_with("Some") || value_str == "None" {
        Some(quote! { *#target_ident = #value_expr; })
    } else {
        Some(quote! { *#target_ident = Some(#value_expr); })
    }
}

fn wrap_optional_value(
    symbol: &str,
    value_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> syn::Expr {
    let inner_type = match ctx.var_types.get(symbol) {
        Some(Type::Optional(inner)) => inner.clone(),
        _ => return value_expr,
    };

    let value_str = quote!(#value_expr).to_string();

    let source_var_is_optional = check_source_is_optional(&value_expr, ctx);
    let expr_returns_option = check_expr_returns_option(&value_str);

    if source_var_is_optional || expr_returns_option {
        if source_var_is_optional && !expr_returns_option {
            parse_quote! { #value_expr.clone() }
        } else {
            value_expr
        }
    } else {
        wrap_in_some(value_expr, &inner_type)
    }
}

fn check_source_is_optional(value_expr: &syn::Expr, ctx: &CodeGenContext) -> bool {
    if let syn::Expr::Path(ref path) = value_expr {
        if path.path.segments.len() == 1 {
            let source_var = path.path.segments[0].ident.to_string();
            return ctx
                .var_types
                .get(&source_var)
                .map(|ty| matches!(ty, Type::Optional(_)))
                .unwrap_or(false);
        }
    }
    false
}

fn check_expr_returns_option(value_str: &str) -> bool {
    value_str.starts_with("Some")
        || value_str == "None"
        || value_str.ends_with(".ok()")
        || value_str.ends_with(". ok ()")
        || value_str.contains(".get(")
        || value_str.contains(". get (")
        || value_str.ends_with(".cloned()")
        || value_str.ends_with(". cloned ()")
        || (value_str.contains(".as_ref()") && !value_str.contains(".unwrap()"))
}

fn wrap_in_some(value_expr: syn::Expr, inner_type: &Type) -> syn::Expr {
    if matches!(inner_type, Type::String) {
        let value_str = quote!(#value_expr).to_string();
        if value_str.starts_with('"') && value_str.ends_with('"') {
            return parse_quote! { Some(#value_expr.to_string()) };
        }
    }
    parse_quote! { Some(#value_expr) }
}

fn codegen_reassign_symbol(
    symbol: &str,
    target_ident: &syn::Ident,
    value_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let value_expr = normalize_reassign_value(value_expr, ctx);

    if let Some(tokens) = try_mut_option_assign(symbol, target_ident, &value_expr, ctx) {
        return Ok(tokens);
    }

    let final_value = wrap_optional_value(symbol, value_expr, ctx);
    Ok(quote! { #target_ident = #final_value; })
}
