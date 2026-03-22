pub(crate) fn parse_target_pattern(target: &str) -> syn::Pat {
    if target.starts_with('(') {
        // Manually construct tuple pattern
        let inner = target.trim_start_matches('(').trim_end_matches(')');
        let parts: Vec<syn::Pat> = inner
            .split(',')
            .map(|s| {
                let ident = make_ident(s.trim());
                syn::Pat::Ident(syn::PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident,
                    subpat: None,
                })
            })
            .collect();
        syn::Pat::Tuple(syn::PatTuple {
            attrs: vec![],
            paren_token: syn::token::Paren::default(),
            elems: parts.into_iter().collect(),
        })
    } else {
        let target_ident = make_ident(target);
        syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: target_ident,
            subpat: None,
        })
    }
}

pub(crate) fn make_ident(name: &str) -> syn::Ident {
    if name.is_empty() {
        return syn::Ident::new("_empty", proc_macro2::Span::call_site());
    }
    // Special case: "self", "super", "crate" cannot be raw identifiers as variable names
    // Convert them to name with underscore suffix
    // DEPYLER-0741: "Self" is valid as a type name in impl blocks, so return it directly
    match name {
        "Self" => {
            // Self is valid as a type name, return as-is
            return syn::Ident::new(name, proc_macro2::Span::call_site());
        }
        "self" | "super" | "crate" => {
            let suffixed = format!("{}_", name);
            return syn::Ident::new(&suffixed, proc_macro2::Span::call_site());
        }
        _ => {}
    }
    // Check if it's a valid identifier that's also a keyword
    if is_rust_keyword(name) {
        // Use raw identifier r#keyword
        return syn::Ident::new_raw(name, proc_macro2::Span::call_site());
    }
    // Check if name is a valid identifier
    let is_valid = name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if is_valid {
        syn::Ident::new(name, proc_macro2::Span::call_site())
    } else {
        // Sanitize and create
        let sanitized = sanitize_identifier(name);
        syn::Ident::new(&sanitized, proc_macro2::Span::call_site())
    }
}

fn convert_array_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    if let Array { element_type, size } = rust_type {
        let element = rust_type_to_syn_type(element_type)?;
        Ok(match size {
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
        })
    } else {
        unreachable!("convert_array_type called with non-array type")
    }
}
