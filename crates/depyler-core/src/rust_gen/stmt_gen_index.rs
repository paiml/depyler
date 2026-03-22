fn find_root_var_in_chain(expr: &HirExpr) -> Option<String> {
    let mut current = expr;
    loop {
        match current {
            HirExpr::Index { base, .. } => current = base,
            HirExpr::Var(name) => return Some(name.clone()),
            _ => return None,
        }
    }
}

fn build_nested_type_chain(ctx: &CodeGenContext, root_var: &str, depth: usize) -> Vec<bool> {
    let mut chain = Vec::with_capacity(depth);
    let root_type = match ctx.var_types.get(root_var) {
        Some(t) => t.clone(),
        None => return chain,
    };
    let mut current = root_type;
    for _ in 0..depth {
        match current {
            Type::List(elem) => {
                chain.push(true);
                current = *elem;
            }
            Type::Dict(_, val) => {
                chain.push(false);
                current = *val;
            }
            _ => {
                // Unknown type at this level, stop
                break;
            }
        }
    }
    chain
}

fn generate_nested_index_assign(
    base: &HirExpr,
    base_expr: &syn::Expr,
    indices: &[syn::Expr],
    final_index: &syn::Expr,
    final_value_expr: &syn::Expr,
    is_numeric_index: bool,
    needs_as_object_mut: bool,
    index: &HirExpr,
    ctx: &CodeGenContext,
) -> proc_macro2::TokenStream {
    let root_var_name = find_root_var_in_chain(base);
    let type_chain = if let Some(ref name) = root_var_name {
        build_nested_type_chain(ctx, name, indices.len() + 1)
    } else {
        vec![]
    };

    let mut chain = quote! { #base_expr };
    for (i, idx) in indices.iter().enumerate() {
        let level_is_list = type_chain.get(i).copied().unwrap_or(is_numeric_index);
        if level_is_list {
            chain = quote! { #chain[#idx as usize] };
        } else {
            let idx_str = quote! { #idx }.to_string();
            let first_char = idx_str.trim_start().chars().next();
            let is_str_lit = first_char == Some('"');
            let is_already_ref = is_str_lit || ctx.fn_str_params.contains(idx_str.trim());
            chain = if is_already_ref {
                quote! { #chain.get_mut(#idx).expect("key not found in dict") }
            } else {
                quote! { #chain.get_mut(&#idx).expect("key not found in dict") }
            };
        }
    }

    let final_is_list = type_chain.get(indices.len()).copied().unwrap_or(is_numeric_index);
    if final_is_list {
        quote! { #chain[(#final_index) as usize] = #final_value_expr; }
    } else if needs_as_object_mut {
        quote! { #chain.as_object_mut().expect("JSON value is not an object").insert((#final_index).clone(), #final_value_expr); }
    } else {
        let needs_clone = matches!(index, HirExpr::Var(_));
        if needs_clone {
            quote! { #chain.insert(#final_index.clone(), #final_value_expr); }
        } else {
            quote! { #chain.insert(#final_index, #final_value_expr); }
        }
    }
}
