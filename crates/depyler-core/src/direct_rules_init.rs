pub(crate) fn sanitize_identifier(name: &str) -> String {
    if name.is_empty() {
        return "_empty".to_string();
    }

    let mut sanitized = String::with_capacity(name.len());

    for (i, c) in name.chars().enumerate() {
        if i == 0 {
            // First character must be letter or underscore
            if c.is_ascii_alphabetic() || c == '_' {
                sanitized.push(c);
            } else if c.is_ascii_digit() {
                // Prefix with underscore if starts with digit
                sanitized.push('_');
                sanitized.push(c);
            } else {
                // Replace invalid char with underscore
                sanitized.push('_');
            }
        } else {
            // Subsequent characters can be alphanumeric or underscore
            if c.is_ascii_alphanumeric() || c == '_' {
                sanitized.push(c);
            } else {
                sanitized.push('_');
            }
        }
    }

    // Ensure we have at least one character
    if sanitized.is_empty() {
        return "_unnamed".to_string();
    }

    // Handle Rust keywords by prefixing with underscore
    // We can't use r# raw identifiers in syn::Ident::new easily,
    // so we append underscore suffix instead
    if is_rust_keyword(&sanitized) {
        sanitized.push('_');
    }

    sanitized
}

pub(crate) fn extract_nested_indices(
    expr: &HirExpr,
    type_mapper: &TypeMapper,
) -> Result<(syn::Expr, Vec<syn::Expr>)> {
    let mut indices = Vec::new();
    let mut current = expr;

    // Walk up the chain collecting indices
    loop {
        match current {
            HirExpr::Index { base, index } => {
                indices.push(convert_expr(index, type_mapper)?);
                current = base;
            }
            _ => {
                // We've reached the base
                let base_expr = convert_expr(current, type_mapper)?;
                indices.reverse(); // We collected from inner to outer, need outer to inner
                return Ok((base_expr, indices));
            }
        }
    }
}

fn build_struct_fields(
    instance_fields: &[&HirField],
    is_exception_class: bool,
    type_mapper: &TypeMapper,
) -> Result<(Vec<syn::Field>, bool)> {
    let mut fields = Vec::new();
    let mut has_non_clone_field = false;

    for field in instance_fields {
        let field_name =
            syn::Ident::new(&sanitize_identifier(&field.name), proc_macro2::Span::call_site());
        let effective_field_type = if is_exception_class && field.field_type == Type::Unknown {
            Type::String
        } else {
            field.field_type.clone()
        };
        let rust_type = type_mapper.map_type(&effective_field_type);
        let field_type = rust_type_to_syn_type(&rust_type)?;

        let type_str = quote::quote!(#field_type).to_string();
        if type_str.contains("Mutex")
            || type_str.contains("RefCell")
            || type_str.contains("Condvar")
            || type_str.contains("RwLock")
            || type_str.contains("mpsc::")
            || type_str.contains("Receiver")
            || type_str.contains("Sender")
            || type_str.contains("JoinHandle")
        {
            has_non_clone_field = true;
        }

        fields.push(syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
            mutability: syn::FieldMutability::None,
            ident: Some(field_name),
            colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
            ty: field_type,
        });
    }

    Ok((fields, has_non_clone_field))
}

fn convert_init_to_new(
    init_method: &HirMethod,
    class: &HirClass,
    _struct_name: &syn::Ident,
    type_mapper: &TypeMapper,
    _vararg_functions: &std::collections::HashSet<String>,
) -> Result<syn::ImplItemFn> {
    // DEPYLER-0957: Check if class inherits from Exception
    // Exception classes should default Unknown types to String (not serde_json::Value)
    let is_exception_class = class.base_classes.iter().any(|base| {
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
    });

    // DEPYLER-0697: Collect field names to determine which params are used
    let field_names: std::collections::HashSet<&str> =
        class.fields.iter().filter(|f| !f.is_class_var).map(|f| f.name.as_str()).collect();

    // Convert parameters
    // DEPYLER-1100: Track String params for impl Into<String> pattern
    let mut string_param_names: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    let mut inputs = syn::punctuated::Punctuated::new();

    for param in &init_method.params {
        // DEPYLER-0697: Prefix unused constructor parameters with _ to avoid warnings
        let param_name = if field_names.contains(param.name.as_str()) {
            param.name.clone()
        } else {
            format!("_{}", param.name)
        };
        let param_ident = make_ident(&param_name);
        // DEPYLER-0957: For Exception classes, default Unknown param types to String
        let effective_param_type = if is_exception_class && param.ty == Type::Unknown {
            Type::String
        } else {
            param.ty.clone()
        };

        // DEPYLER-1100: Use impl Into<String> for String parameters to allow both String and &str
        let is_string_param = effective_param_type == Type::String;
        if is_string_param {
            string_param_names.insert(param.name.clone());
        }

        let param_syn_type: syn::Type = if is_string_param {
            // Use impl Into<String> for string parameters
            parse_quote!(impl Into<String>)
        } else {
            let rust_type = type_mapper.map_type(&effective_param_type);
            rust_type_to_syn_type(&rust_type)?
        };

        inputs.push(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: param_ident,
                subpat: None,
            })),
            colon_token: syn::Token![:](proc_macro2::Span::call_site()),
            ty: Box::new(param_syn_type),
        }));
    }

    // Generate field initializers based on class fields and parameters
    // Skip class variables (constants) - only initialize instance fields
    let mut field_inits = Vec::new();

    for field in &class.fields {
        // Skip class variables (constants/statics)
        if field.is_class_var {
            continue;
        }

        let field_ident =
            syn::Ident::new(&sanitize_identifier(&field.name), proc_macro2::Span::call_site());

        // Check if this field matches a parameter name
        if init_method.params.iter().any(|param| param.name == field.name) {
            // DEPYLER-1100: For string parameters using impl Into<String>, call .into()
            if string_param_names.contains(&field.name) {
                field_inits.push(quote! { #field_ident: #field_ident.into() });
            } else {
                // Initialize from parameter (shorthand field init)
                field_inits.push(quote! { #field_ident });
            }
        } else {
            // Initialize with default value based on type
            let default_value = match &field.field_type {
                Type::Int => quote! { 0 },
                Type::Float => quote! { 0.0 },
                Type::String => quote! { String::new() },
                Type::Bool => quote! { false },
                Type::List(_) => quote! { Vec::new() },
                Type::Dict(_, _) => quote! { std::collections::HashMap::new() },
                Type::Set(_) => quote! { std::collections::HashSet::new() },
                _ => quote! { Default::default() },
            };
            field_inits.push(quote! { #field_ident: #default_value });
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
