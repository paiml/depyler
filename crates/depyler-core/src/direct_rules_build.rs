fn build_class_impl_items(
    class: &HirClass,
    struct_name: &syn::Ident,
    type_mapper: &TypeMapper,
    vararg_functions: &std::collections::HashSet<String>,
    class_fields: &[&HirField],
) -> Result<Vec<syn::ImplItem>> {
    let mut impl_items = Vec::new();

    for class_field in class_fields {
        if let Some(default_value) = &class_field.default_value {
            let const_name = make_ident(&class_field.name);
            let rust_type = type_mapper.map_type(&class_field.field_type);
            let const_type = rust_type_to_syn_type(&rust_type)?;
            let value_expr = convert_expr(default_value, type_mapper)?;

            impl_items.push(parse_quote! {
                pub const #const_name: #const_type = #value_expr;
            });
        }
    }

    let has_init = class.methods.iter().any(|m| m.name == "__init__");

    if has_init {
        for method in &class.methods {
            if method.name == "__init__" {
                let new_method = convert_init_to_new(
                    method,
                    class,
                    struct_name,
                    type_mapper,
                    vararg_functions,
                )?;
                impl_items.push(syn::ImplItem::Fn(new_method));
            } else {
                let rust_method = convert_method_to_impl_item(
                    method,
                    type_mapper,
                    vararg_functions,
                    &class.fields,
                    &class.type_params,
                )?;
                impl_items.push(syn::ImplItem::Fn(rust_method));
            }
        }
    } else {
        if class.is_dataclass
            || class.fields.iter().all(|f| f.default_value.is_some() || f.field_type == Type::Int)
        {
            let new_method = generate_dataclass_new(class, struct_name, type_mapper)?;
            impl_items.push(syn::ImplItem::Fn(new_method));
        }

        for method in &class.methods {
            let rust_method = convert_method_to_impl_item(
                method,
                type_mapper,
                vararg_functions,
                &class.fields,
                &class.type_params,
            )?;
            impl_items.push(syn::ImplItem::Fn(rust_method));
        }
    }

    Ok(impl_items)
}

fn generate_dataclass_new(
    class: &HirClass,
    _struct_name: &syn::Ident,
    type_mapper: &TypeMapper,
) -> Result<syn::ImplItemFn> {
    // Generate parameters from fields (include all instance fields)
    // DEPYLER-0939: Include fields with defaults in new() signature to match Python semantics
    // Defaults should be handled at call site or via builder pattern
    let mut inputs = syn::punctuated::Punctuated::new();
    let instance_fields: Vec<_> = class.fields.iter().filter(|f| !f.is_class_var).collect();

    for field in &instance_fields {
        let param_ident =
            syn::Ident::new(&sanitize_identifier(&field.name), proc_macro2::Span::call_site());
        let rust_type = type_mapper.map_type(&field.field_type);
        let param_syn_type = rust_type_to_syn_type(&rust_type)?;

        inputs.push(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: param_ident.clone(),
                subpat: None,
            })),
            colon_token: syn::Token![:](proc_macro2::Span::call_site()),
            ty: Box::new(param_syn_type),
        }));
    }

    // Generate body that initializes struct fields (skip class variables)
    let mut field_inits: Vec<proc_macro2::TokenStream> = class
        .fields
        .iter()
        .filter(|f| !f.is_class_var) // Skip class constants
        .map(|field| {
            let field_ident =
                syn::Ident::new(&sanitize_identifier(&field.name), proc_macro2::Span::call_site());
            // DEPYLER-0939: Always use parameter since we now accept all fields in new()
            quote! { #field_ident }
        })
        .collect();

    // DEPYLER-0837: Add PhantomData initialization if ANY type params aren't used in fields
    if !class.type_params.is_empty() {
        let instance_fields: Vec<_> = class.fields.iter().filter(|f| !f.is_class_var).collect();
        // Check if ANY type param is unused (we need PhantomData for those)
        let has_unused_params = class.type_params.iter().any(|tp| {
            !instance_fields.iter().any(|f| {
                let type_str = format!("{:?}", f.field_type);
                type_str.contains(tp)
            })
        });
        if has_unused_params {
            field_inits.push(quote! { _phantom: std::marker::PhantomData });
        }
    }

    let body = parse_quote! {
        {
            Self {
                #(#field_inits),*
            }
        }
    };

    Ok(syn::ImplItemFn {
        attrs: vec![],
        vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
        defaultness: None,
        sig: syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: syn::Token![fn](proc_macro2::Span::call_site()),
            ident: syn::Ident::new("new", proc_macro2::Span::call_site()),
            generics: syn::Generics::default(),
            paren_token: syn::token::Paren::default(),
            inputs,
            variadic: None,
            output: syn::ReturnType::Type(
                syn::Token![->](proc_macro2::Span::call_site()),
                Box::new(parse_quote! { Self }),
            ),
        },
        block: body,
    })
}
