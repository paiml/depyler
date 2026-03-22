fn convert_method_to_impl_item(
    method: &HirMethod,
    type_mapper: &TypeMapper,
    vararg_functions: &std::collections::HashSet<String>,
    fields: &[HirField],
    class_type_params: &[String],
) -> Result<syn::ImplItemFn> {
    let (method_name, method_level_type_params) =
        resolve_method_name_and_generics(method, class_type_params);

    let inputs = build_method_inputs(method, type_mapper, fields)?;

    let effective_ret_type = infer_effective_return_type(method, fields);
    let rust_ret_type = type_mapper.map_type(&effective_ret_type);
    let ret_type = rust_type_to_syn_type(&rust_ret_type)?;

    let param_types: std::collections::HashMap<String, Type> =
        method.params.iter().map(|p| (p.name.clone(), p.ty.clone())).collect();
    let class_field_types: std::collections::HashMap<String, Type> =
        fields.iter().map(|f| (f.name.clone(), f.field_type.clone())).collect();

    let body = build_method_body(
        method,
        type_mapper,
        vararg_functions,
        &param_types,
        &class_field_types,
        &effective_ret_type,
    )?;

    let generics = build_method_generics(&method_level_type_params);

    Ok(syn::ImplItemFn {
        attrs: vec![],
        vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
        defaultness: None,
        sig: syn::Signature {
            constness: None,
            asyncness: if method.is_async {
                Some(syn::Token![async](proc_macro2::Span::call_site()))
            } else {
                None
            },
            unsafety: None,
            abi: None,
            fn_token: syn::Token![fn](proc_macro2::Span::call_site()),
            ident: method_name,
            generics,
            paren_token: syn::token::Paren::default(),
            inputs,
            variadic: None,
            output: if matches!(effective_ret_type, Type::None) {
                syn::ReturnType::Default
            } else {
                syn::ReturnType::Type(
                    syn::Token![->](proc_macro2::Span::call_site()),
                    Box::new(ret_type),
                )
            },
        },
        block: body,
    })
}

fn infer_effective_return_type(method: &HirMethod, fields: &[HirField]) -> Type {
    if matches!(method.ret_type, Type::Unknown | Type::None) {
        infer_method_return_type(&method.body, fields).unwrap_or_else(|| method.ret_type.clone())
    } else {
        method.ret_type.clone()
    }
}

fn build_method_body(
    method: &HirMethod,
    type_mapper: &TypeMapper,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
    effective_ret_type: &Type,
) -> Result<syn::Block> {
    let body_is_only_pass = method.body.iter().all(|stmt| matches!(stmt, HirStmt::Pass));
    let is_non_unit_return = !matches!(effective_ret_type, Type::None | Type::Unknown);

    if method.body.is_empty() || (body_is_only_pass && is_non_unit_return) {
        if is_non_unit_return {
            Ok(parse_quote! { { unimplemented!() } })
        } else {
            Ok(parse_quote! { {} })
        }
    } else {
        convert_method_body_block(
            &method.body,
            type_mapper,
            method.is_classmethod,
            vararg_functions,
            param_types,
            class_field_types,
            effective_ret_type,
        )
    }
}

fn build_method_generics(method_level_type_params: &[String]) -> syn::Generics {
    if method_level_type_params.is_empty() {
        return syn::Generics::default();
    }
    let params: syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]> =
        method_level_type_params
            .iter()
            .map(|name| {
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                let clone_bound: syn::TypeParamBound = parse_quote!(Clone);
                let mut bounds = syn::punctuated::Punctuated::new();
                bounds.push(clone_bound);
                syn::GenericParam::Type(syn::TypeParam {
                    attrs: vec![],
                    ident,
                    colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
                    bounds,
                    eq_token: None,
                    default: None,
                })
            })
            .collect();
    syn::Generics {
        lt_token: Some(syn::Token![<](proc_macro2::Span::call_site())),
        params,
        gt_token: Some(syn::Token![>](proc_macro2::Span::call_site())),
        where_clause: None,
    }
}
