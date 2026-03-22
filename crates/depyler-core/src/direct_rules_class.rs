pub fn convert_class_to_struct(
    class: &HirClass,
    type_mapper: &TypeMapper,
    vararg_functions: &std::collections::HashSet<String>,
) -> Result<Vec<syn::Item>> {
    let safe_name = safe_class_name(&class.name);
    let struct_name = make_ident(&safe_name);

    if class_is_enum(class) {
        return convert_enum_class(class, type_mapper, vararg_functions);
    }

    let is_exception_class = class_is_exception(class);

    let (instance_fields, class_fields): (Vec<_>, Vec<_>) =
        class.fields.iter().partition(|f| !f.is_class_var);

    let (fields, has_non_clone_field) =
        build_struct_fields(&instance_fields, is_exception_class, type_mapper)?;

    let generics = build_class_generics(&class.type_params, !has_non_clone_field);

    let final_fields = add_phantom_data_if_needed(fields, &class.type_params);

    let struct_item = build_struct_item(
        &struct_name,
        &generics,
        final_fields,
        has_non_clone_field,
        class.is_dataclass,
    );

    let impl_items = build_class_impl_items(
        class,
        &struct_name,
        type_mapper,
        vararg_functions,
        &class_fields,
    )?;

    let mut items = vec![struct_item];
    if !impl_items.is_empty() {
        items.push(build_impl_block(
            &struct_name,
            &class.type_params,
            &generics,
            impl_items,
        ));
    }

    Ok(items)
}

fn class_is_enum(class: &HirClass) -> bool {
    class.base_classes.iter().any(|base| {
        matches!(
            base.as_str(),
            "Enum" | "IntEnum" | "enum.Enum" | "enum.IntEnum" | "StrEnum" | "enum.StrEnum"
        )
    })
}

fn class_is_exception(class: &HirClass) -> bool {
    class.base_classes.iter().any(|base| {
        matches!(
            base.as_str(),
            "Exception"
                | "BaseException"
                | "ValueError"
                | "TypeError"
                | "KeyError"
                | "RuntimeError"
                | "IOError"
                | "OSError"
                | "AttributeError"
                | "IndexError"
                | "StopIteration"
                | "SyntaxError"
                | "FileNotFoundError"
                | "ZeroDivisionError"
        )
    })
}

fn build_class_generics(
    type_params: &[String],
    needs_clone_bound: bool,
) -> syn::Generics {
    if type_params.is_empty() {
        return syn::Generics::default();
    }
    let params: syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]> = type_params
        .iter()
        .map(|name| {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            let bounds = if needs_clone_bound {
                let clone_bound: syn::TypeParamBound = parse_quote!(Clone);
                let mut bounds = syn::punctuated::Punctuated::new();
                bounds.push(clone_bound);
                bounds
            } else {
                syn::punctuated::Punctuated::new()
            };
            syn::GenericParam::Type(syn::TypeParam {
                attrs: vec![],
                ident,
                colon_token: if needs_clone_bound {
                    Some(syn::Token![:](proc_macro2::Span::call_site()))
                } else {
                    None
                },
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

fn add_phantom_data_if_needed(
    mut fields: Vec<syn::Field>,
    type_params: &[String],
) -> Vec<syn::Field> {
    if type_params.is_empty() {
        return fields;
    }
    let unused_type_params: Vec<&String> = type_params
        .iter()
        .filter(|tp| {
            !fields.iter().any(|f| {
                let type_str = quote::quote!(#f.ty).to_string();
                type_str.contains(*tp)
            })
        })
        .collect();

    if unused_type_params.is_empty() {
        return fields;
    }

    let phantom_types: Vec<syn::Type> = unused_type_params
        .iter()
        .map(|tp| {
            let ident = syn::Ident::new(tp, proc_macro2::Span::call_site());
            parse_quote!(#ident)
        })
        .collect();

    let phantom_type: syn::Type = if phantom_types.len() == 1 {
        let t = &phantom_types[0];
        parse_quote!(std::marker::PhantomData<#t>)
    } else {
        parse_quote!(std::marker::PhantomData<(#(#phantom_types),*)>)
    };

    fields.push(syn::Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(syn::Ident::new("_phantom", proc_macro2::Span::call_site())),
        colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
        ty: phantom_type,
    });

    fields
}

fn build_struct_item(
    struct_name: &syn::Ident,
    generics: &syn::Generics,
    final_fields: Vec<syn::Field>,
    has_non_clone_field: bool,
    is_dataclass: bool,
) -> syn::Item {
    syn::Item::Struct(syn::ItemStruct {
        attrs: if has_non_clone_field {
            vec![parse_quote! { #[derive(Debug)] }]
        } else if is_dataclass {
            vec![parse_quote! { #[derive(Debug, Clone, PartialEq)] }]
        } else {
            vec![parse_quote! { #[derive(Debug, Clone)] }]
        },
        vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
        struct_token: syn::Token![struct](proc_macro2::Span::call_site()),
        ident: struct_name.clone(),
        generics: generics.clone(),
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: final_fields.into_iter().collect(),
        }),
        semi_token: None,
    })
}

fn build_impl_block(
    struct_name: &syn::Ident,
    type_params: &[String],
    generics: &syn::Generics,
    impl_items: Vec<syn::ImplItem>,
) -> syn::Item {
    let self_ty: syn::Type = if type_params.is_empty() {
        parse_quote! { #struct_name }
    } else {
        let type_args: syn::punctuated::Punctuated<syn::Type, syn::Token![,]> = type_params
            .iter()
            .map(|name| {
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                syn::Type::Path(syn::TypePath { qself: None, path: syn::Path::from(ident) })
            })
            .collect();
        parse_quote! { #struct_name<#type_args> }
    };

    syn::Item::Impl(syn::ItemImpl {
        attrs: vec![],
        defaultness: None,
        unsafety: None,
        impl_token: syn::Token![impl](proc_macro2::Span::call_site()),
        generics: generics.clone(),
        trait_: None,
        self_ty: Box::new(self_ty),
        brace_token: syn::token::Brace::default(),
        items: impl_items,
    })
}
