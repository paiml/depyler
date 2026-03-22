fn codegen_depyler_value_tuple_unpack(
    symbols: &[&str],
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let temp_var = syn::Ident::new("_tuple_tmp", proc_macro2::Span::call_site());
    let num_targets = symbols.len();

    let target_info: Vec<_> = symbols
        .iter()
        .map(|s| {
            let target_type = ctx.var_types.get(*s).cloned();
            let is_mutable = ctx.mutable_vars.contains(*s);
            (*s, target_type, is_mutable)
        })
        .collect();

    for s in symbols.iter() {
        if !ctx.is_declared(s) {
            ctx.declare_var(s);
        }
    }

    let assignments: Vec<proc_macro2::TokenStream> = target_info
        .iter()
        .enumerate()
        .map(|(idx, (s, target_type, is_mutable))| {
            let ident = safe_ident(s);
            let idx_lit = syn::LitInt::new(&idx.to_string(), proc_macro2::Span::call_site());
            let mut_token = mut_token_for(*is_mutable);
            emit_get_tuple_elem_coercion(&temp_var, &ident, &idx_lit, &mut_token, target_type)
        })
        .collect();

    let num_lit =
        syn::LitInt::new(&num_targets.to_string(), proc_macro2::Span::call_site());

    Ok(quote! {
        let #temp_var = #value_expr;
        // Validate tuple has expected number of elements
        let _ = #temp_var.extract_tuple(#num_lit);
        #(#assignments)*
    })
}

fn emit_get_tuple_elem_coercion(
    temp_var: &syn::Ident,
    ident: &syn::Ident,
    idx_lit: &syn::LitInt,
    mut_token: &proc_macro2::TokenStream,
    target_type: &Option<Type>,
) -> proc_macro2::TokenStream {
    match target_type {
        Some(Type::Int) => {
            quote! { let #mut_token #ident: i32 = #temp_var.get_tuple_elem(#idx_lit).to_i64() as i32; }
        }
        Some(Type::Float) => {
            quote! { let #mut_token #ident: f64 = #temp_var.get_tuple_elem(#idx_lit).to_f64(); }
        }
        Some(Type::String) => {
            quote! { let #mut_token #ident: String = #temp_var.get_tuple_elem(#idx_lit).to_string(); }
        }
        Some(Type::Bool) => {
            quote! { let #mut_token #ident: bool = #temp_var.get_tuple_elem(#idx_lit).to_bool(); }
        }
        _ => {
            quote! { let #mut_token #ident = #temp_var.get_tuple_elem(#idx_lit); }
        }
    }
}
