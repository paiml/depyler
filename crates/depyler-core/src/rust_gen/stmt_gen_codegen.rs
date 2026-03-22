fn codegen_assign_tuple_symbols(
    symbols: Vec<&str>,
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0494 FIX: Check if we're in a generator with state variables
    let generator_state_vars: Vec<_> = if ctx.in_generator {
        symbols
            .iter()
            .filter(|s| ctx.generator_state_vars.contains(&s.to_string()))
            .collect()
    } else {
        vec![]
    };

    if !generator_state_vars.is_empty() {
        return codegen_generator_tuple_assign(&symbols, value_expr, ctx);
    }

    // No generator state variables - original logic
    let all_declared = symbols.iter().all(|s| ctx.is_declared(s));

    // DEPYLER-0671: Check if value_expr is a Vec from split/collect
    let value_str = quote! { #value_expr }.to_string();
    let is_vec_from_split = value_str.contains("collect ::<Vec")
        || value_str.contains("collect::< Vec")
        || value_str.contains(".collect ()")
        || (value_str.contains("splitn") && value_str.contains("collect"));

    if is_vec_from_split {
        return codegen_vec_split_unpack(&symbols, value_expr, ctx);
    }

    if all_declared {
        let idents: Vec<_> = symbols
            .iter()
            .map(|s| safe_ident(s)) // DEPYLER-0023
            .collect();
        Ok(quote! { (#(#idents),*) = #value_expr; })
    } else {
        symbols.iter().for_each(|s| ctx.declare_var(s));
        let idents_with_mut: Vec<_> = symbols
            .iter()
            .map(|s| {
                let ident = safe_ident(s); // DEPYLER-0023
                if ctx.mutable_vars.contains(*s) {
                    quote! { mut #ident }
                } else {
                    quote! { #ident }
                }
            })
            .collect();
        Ok(quote! { let (#(#idents_with_mut),*) = #value_expr; })
    }
}

fn codegen_generator_tuple_assign(
    symbols: &[&str],
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let temp_var = syn::Ident::new("_tuple_temp", proc_macro2::Span::call_site());
    let assignments: Vec<_> = symbols
        .iter()
        .enumerate()
        .map(|(idx, s)| {
            let ident = safe_ident(s);
            let index = syn::Index::from(idx);

            if ctx.generator_state_vars.contains(&s.to_string()) {
                quote! { self.#ident = #temp_var.#index; }
            } else if ctx.is_declared(s) {
                quote! { #ident = #temp_var.#index; }
            } else {
                ctx.declare_var(s);
                let mut_token = mut_token_for(ctx.mutable_vars.contains(*s));
                quote! { let #mut_token #ident = #temp_var.#index; }
            }
        })
        .collect();

    Ok(quote! {
        let #temp_var = #value_expr;
        #(#assignments)*
    })
}

fn codegen_vec_split_unpack(
    symbols: &[&str],
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let parts_name =
        syn::Ident::new("_split_parts", proc_macro2::Span::call_site());

    let assignments: Vec<_> = symbols
        .iter()
        .enumerate()
        .map(|(idx, s)| {
            let ident = safe_ident(s);
            ctx.declare_var(s);
            let mut_token = mut_token_for(ctx.mutable_vars.contains(*s));
            let idx_lit = syn::Index::from(idx);
            quote! { let #mut_token #ident = #parts_name.get(#idx_lit).cloned().unwrap_or_default(); }
        })
        .collect();

    Ok(quote! {
        let #parts_name = #value_expr;
        #(#assignments)*
    })
}
