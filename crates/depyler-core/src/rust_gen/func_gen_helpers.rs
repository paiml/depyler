//! Function Code Generation Helpers - EXTREME TDD
//!
//! This module contains extracted pure helper functions from func_gen.rs
//! for better testability and coverage.
//!
//! ## Functions
//!
//! - `codegen_generic_params` - Generate <'a, T: Bound> style generics
//! - `codegen_where_clause` - Generate where clauses for lifetimes
//! - `codegen_function_attrs` - Generate function attributes (docs, #[inline], etc.)

use quote::quote;

/// Generate combined generic parameters (<'a, 'b, T, U: Bound>)
pub fn codegen_generic_params(
    type_params: &[crate::generic_inference::TypeParameter],
    lifetime_params: &[String],
) -> proc_macro2::TokenStream {
    if type_params.is_empty() && lifetime_params.is_empty() {
        return quote! {};
    }

    let mut all_params = Vec::new();

    // Add lifetime parameters first (filter out 'static)
    for lt in lifetime_params {
        if lt != "'static" {
            let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
            all_params.push(quote! { #lt_ident });
        }
    }

    // Add type parameters with their bounds
    for type_param in type_params {
        let param_name = syn::Ident::new(&type_param.name, proc_macro2::Span::call_site());
        if type_param.bounds.is_empty() {
            all_params.push(quote! { #param_name });
        } else {
            let bounds: Vec<_> = type_param
                .bounds
                .iter()
                .map(|b| {
                    syn::parse_str::<syn::TypeParamBound>(b)
                        .map(|bound| quote! { #bound })
                        .or_else(|_| syn::parse_str::<syn::Path>(b).map(|path| quote! { #path }))
                        .unwrap_or_else(|_| quote! { Clone })
                })
                .collect();
            all_params.push(quote! { #param_name: #(#bounds)+* });
        }
    }

    quote! { <#(#all_params),*> }
}

/// Generate where clause for lifetime bounds (where 'a: 'b, 'c: 'd)
pub fn codegen_where_clause(lifetime_bounds: &[(String, String)]) -> proc_macro2::TokenStream {
    if lifetime_bounds.is_empty() {
        return quote! {};
    }

    let bounds: Vec<_> = lifetime_bounds
        .iter()
        .map(|(from, to)| {
            let from_lt = syn::Lifetime::new(from, proc_macro2::Span::call_site());
            let to_lt = syn::Lifetime::new(to, proc_macro2::Span::call_site());
            quote! { #from_lt: #to_lt }
        })
        .collect();

    quote! { where #(#bounds),* }
}

/// Generate function attributes (doc comments, panic-free, termination proofs, custom attributes)
pub fn codegen_function_attrs(
    docstring: &Option<String>,
    properties: &crate::hir::FunctionProperties,
    custom_attributes: &[String],
) -> Vec<proc_macro2::TokenStream> {
    let mut attrs = vec![];

    // Add docstring as documentation if present
    if let Some(docstring) = docstring {
        attrs.push(quote! {
            #[doc = #docstring]
        });
    }

    if properties.panic_free {
        attrs.push(quote! {
            #[doc = " Depyler: verified panic-free"]
        });
    }

    if properties.always_terminates {
        attrs.push(quote! {
            #[doc = " Depyler: proven to terminate"]
        });
    }

    // Add custom Rust attributes
    for attr in custom_attributes {
        if let Ok(tokens) = attr.parse::<proc_macro2::TokenStream>() {
            attrs.push(quote! {
                #[#tokens]
            });
        }
    }

    attrs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic_inference::TypeParameter;
    use crate::hir::FunctionProperties;

    #[test]
    fn test_codegen_generic_params_empty() {
        let result = codegen_generic_params(&[], &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_codegen_generic_params_single_lifetime() {
        let result = codegen_generic_params(&[], &["'a".to_string()]);
        let code = result.to_string();
        assert!(code.contains("'a"));
    }

    #[test]
    fn test_codegen_generic_params_filters_static() {
        let result = codegen_generic_params(&[], &["'static".to_string(), "'a".to_string()]);
        let code = result.to_string();
        assert!(code.contains("'a"));
        assert!(!code.contains("static"));
    }

    #[test]
    fn test_codegen_generic_params_type_with_bound() {
        let type_params = vec![TypeParameter {
            name: "T".to_string(),
            bounds: vec!["Clone".to_string()],
            default: None,
        }];
        let result = codegen_generic_params(&type_params, &[]);
        let code = result.to_string();
        assert!(code.contains("T"));
        assert!(code.contains("Clone"));
    }

    #[test]
    fn test_codegen_where_clause_empty() {
        let result = codegen_where_clause(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_codegen_where_clause_single_bound() {
        let bounds = vec![("'a".to_string(), "'b".to_string())];
        let result = codegen_where_clause(&bounds);
        let code = result.to_string();
        assert!(code.contains("where"));
    }

    #[test]
    fn test_codegen_function_attrs_empty() {
        let props = FunctionProperties::default();
        let result = codegen_function_attrs(&None, &props, &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_codegen_function_attrs_with_docstring() {
        let props = FunctionProperties::default();
        let result = codegen_function_attrs(&Some("Test function".to_string()), &props, &[]);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_codegen_function_attrs_panic_free() {
        let props = FunctionProperties { panic_free: true, ..Default::default() };
        let result = codegen_function_attrs(&None, &props, &[]);
        assert_eq!(result.len(), 1);
    }
}
