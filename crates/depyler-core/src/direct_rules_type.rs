fn convert_function(func: &HirFunction, type_mapper: &TypeMapper) -> Result<syn::ItemFn> {
    let name = make_ident(&func.name);

    // Convert parameters
    let mut inputs = Vec::new();
    for param in &func.params {
        let rust_type = type_mapper.map_type(&param.ty);
        let ty = rust_type_to_syn(&rust_type)?;
        let pat = syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: make_ident(&param.name),
            subpat: None,
        });

        // Use references for non-copy types
        let ty = if type_mapper.needs_reference(&rust_type) {
            parse_quote! { &#ty }
        } else {
            ty
        };

        inputs.push(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(pat),
            colon_token: Default::default(),
            ty: Box::new(ty),
        }));
    }

    // Convert return type
    let rust_ret_type = type_mapper.map_return_type(&func.ret_type);

    // DEPYLER-0612: Fix main() return type - Rust main can only return () or Result<(), E>
    // Cannot return Result<i32, E> - convert to Result<(), E>
    let rust_ret_type = if func.name == "main" {
        match &rust_ret_type {
            RustType::Result(inner, err) if matches!(**inner, RustType::Primitive(_)) => {
                // Result<i32, E> -> Result<(), E> for main
                RustType::Result(Box::new(RustType::Unit), err.clone())
            }
            _ => rust_ret_type,
        }
    } else {
        rust_ret_type
    };

    let output = if matches!(rust_ret_type, RustType::Unit) {
        syn::ReturnType::Default
    } else {
        let ty = rust_type_to_syn(&rust_ret_type)?;
        syn::ReturnType::Type(Default::default(), Box::new(ty))
    };

    // Convert body
    let body_stmts = convert_body(&func.body, type_mapper)?;
    let block = syn::Block { brace_token: Default::default(), stmts: body_stmts };

    // Add documentation
    let mut attrs = vec![];

    // Add docstring as documentation if present
    if let Some(docstring) = &func.docstring {
        attrs.push(parse_quote! {
            #[doc = #docstring]
        });
    }

    if func.properties.panic_free {
        attrs.push(parse_quote! {
            #[doc = " Depyler: verified panic-free"]
        });
    }
    if func.properties.always_terminates {
        attrs.push(parse_quote! {
            #[doc = " Depyler: proven to terminate"]
        });
    }

    Ok(syn::ItemFn {
        attrs,
        vis: syn::Visibility::Public(Default::default()),
        sig: syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: name,
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: inputs.into_iter().collect(),
            variadic: None,
            output,
        },
        block: Box::new(block),
    })
}

fn rust_type_to_syn(rust_type: &RustType) -> Result<syn::Type> {
    Ok(match rust_type {
        RustType::Primitive(p) => {
            let ident = syn::Ident::new(p.to_rust_string(), proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::String => parse_quote! { String },
        RustType::Vec(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Vec<#inner_ty> }
        }
        RustType::HashMap(k, v) => {
            let key_ty = rust_type_to_syn(k)?;
            let val_ty = rust_type_to_syn(v)?;
            parse_quote! { HashMap<#key_ty, #val_ty> }
        }
        RustType::Option(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Option<#inner_ty> }
        }
        RustType::Unit => parse_quote! { () },
        RustType::Array { element_type, size } => {
            let element = rust_type_to_syn(element_type)?;
            match size {
                crate::type_mapper::RustConstGeneric::Literal(n) => {
                    let size_lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
                    parse_quote! { [#element; #size_lit] }
                }
                crate::type_mapper::RustConstGeneric::Parameter(name) => {
                    let param_ident = make_ident(name);
                    parse_quote! { [#element; #param_ident] }
                }
                crate::type_mapper::RustConstGeneric::Expression(expr) => {
                    let expr_tokens: proc_macro2::TokenStream = expr
                        .parse()
                        .unwrap_or_else(|_| "/* invalid const expression */".parse().unwrap());
                    parse_quote! { [#element; #expr_tokens] }
                }
            }
        }
        _ => bail!("Unsupported Rust type: {:?}", rust_type),
    })
}

    fn create_test_type_mapper() -> TypeMapper {
        TypeMapper::default()
    }
