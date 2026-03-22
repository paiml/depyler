fn codegen_native_depyler_tuple_unpack(
    symbols: &[&str],
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let temp_var = syn::Ident::new("_tuple_tmp", proc_macro2::Span::call_site());

    let target_info: Vec<_> = symbols
        .iter()
        .map(|s| {
            let target_type = ctx.var_types.get(*s).cloned();
            let is_mutable = ctx.mutable_vars.contains(*s);
            (*s, target_type, is_mutable)
        })
        .collect();

    for (s, target_type, _) in &target_info {
        if !ctx.is_declared(s) {
            ctx.declare_var(s);
        }
        if target_type.is_none() {
            ctx.var_types.insert(s.to_string(), Type::Unknown);
        }
    }

    let assignments: Vec<proc_macro2::TokenStream> = target_info
        .iter()
        .enumerate()
        .map(|(idx, (s, target_type, is_mutable))| {
            let ident = safe_ident(s);
            let index = syn::Index::from(idx);
            let mut_token = mut_token_for(*is_mutable);
            emit_positional_coercion(&temp_var, &ident, &index, &mut_token, target_type)
        })
        .collect();

    Ok(quote! {
        let #temp_var = #value_expr;
        #(#assignments)*
    })
}

fn mut_token_for(is_mutable: bool) -> proc_macro2::TokenStream {
    if is_mutable {
        quote! { mut }
    } else {
        quote! {}
    }
}

fn emit_positional_coercion(
    temp_var: &syn::Ident,
    ident: &syn::Ident,
    index: &syn::Index,
    mut_token: &proc_macro2::TokenStream,
    target_type: &Option<Type>,
) -> proc_macro2::TokenStream {
    match target_type {
        Some(Type::Int) => {
            quote! { let #mut_token #ident: i32 = #temp_var.#index.to_i64() as i32; }
        }
        Some(Type::Float) => {
            quote! { let #mut_token #ident: f64 = #temp_var.#index.to_f64(); }
        }
        Some(Type::String) => {
            quote! { let #mut_token #ident: String = #temp_var.#index.to_string(); }
        }
        Some(Type::Bool) => {
            quote! { let #mut_token #ident: bool = #temp_var.#index.to_bool(); }
        }
        _ => {
            quote! { let #mut_token #ident = #temp_var.#index.clone(); }
        }
    }
}
