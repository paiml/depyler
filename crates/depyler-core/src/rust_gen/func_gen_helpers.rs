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
//! - `infer_expr_type_simple` - Basic type inference for expressions

use crate::hir::{HirExpr, Literal, Type};
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

/// Simple type inference for expressions (no context needed)
/// Used for inferring types from assignments
pub fn infer_expr_type_simple(expr: &HirExpr) -> Type {
    match expr {
        HirExpr::Literal(lit) => match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Bool(_) => Type::Bool,
            Literal::None => Type::None,
            Literal::Bytes(_) => Type::Custom("Vec<u8>".to_string()),
        },
        HirExpr::List(elems) => {
            if elems.is_empty() {
                Type::List(Box::new(Type::Unknown))
            } else {
                Type::List(Box::new(infer_expr_type_simple(&elems[0])))
            }
        }
        HirExpr::Dict(items) => {
            if items.is_empty() {
                Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown))
            } else {
                let (key, val) = &items[0];
                Type::Dict(
                    Box::new(infer_expr_type_simple(key)),
                    Box::new(infer_expr_type_simple(val)),
                )
            }
        }
        HirExpr::Set(elems) => {
            if elems.is_empty() {
                Type::Set(Box::new(Type::Unknown))
            } else {
                Type::Set(Box::new(infer_expr_type_simple(&elems[0])))
            }
        }
        HirExpr::Tuple(elems) => Type::Tuple(elems.iter().map(infer_expr_type_simple).collect()),
        HirExpr::FString { .. } => Type::String,
        HirExpr::Binary { op, .. } => {
            use crate::hir::BinOp;
            match op {
                BinOp::Eq
                | BinOp::NotEq
                | BinOp::Lt
                | BinOp::LtEq
                | BinOp::Gt
                | BinOp::GtEq
                | BinOp::And
                | BinOp::Or
                | BinOp::In
                | BinOp::NotIn => Type::Bool,
                BinOp::Div => Type::Float,
                _ => Type::Unknown,
            }
        }
        HirExpr::Unary { op, .. } => {
            use crate::hir::UnaryOp;
            match op {
                UnaryOp::Not => Type::Bool,
                _ => Type::Unknown,
            }
        }
        // Note: HirExpr doesn't have a Compare variant - comparisons use Binary with comparison ops
        _ => Type::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic_inference::TypeParameter;
    use crate::hir::FunctionProperties;

    // ============================================
    // codegen_generic_params tests
    // ============================================

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
    fn test_codegen_generic_params_multiple_lifetimes() {
        let result = codegen_generic_params(&[], &["'a".to_string(), "'b".to_string()]);
        let code = result.to_string();
        assert!(code.contains("'a"));
        assert!(code.contains("'b"));
    }

    #[test]
    fn test_codegen_generic_params_filters_static() {
        let result = codegen_generic_params(&[], &["'static".to_string(), "'a".to_string()]);
        let code = result.to_string();
        assert!(code.contains("'a"));
        // 'static should be filtered out as it's reserved
        assert!(!code.contains("static"));
    }

    #[test]
    fn test_codegen_generic_params_single_type() {
        let type_params = vec![TypeParameter {
            name: "T".to_string(),
            bounds: vec![],
            default: None,
        }];
        let result = codegen_generic_params(&type_params, &[]);
        let code = result.to_string();
        assert!(code.contains("T"));
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
    fn test_codegen_generic_params_type_with_multiple_bounds() {
        let type_params = vec![TypeParameter {
            name: "T".to_string(),
            bounds: vec!["Clone".to_string(), "Debug".to_string()],
            default: None,
        }];
        let result = codegen_generic_params(&type_params, &[]);
        let code = result.to_string();
        assert!(code.contains("T"));
        assert!(code.contains("Clone"));
        assert!(code.contains("Debug"));
    }

    #[test]
    fn test_codegen_generic_params_mixed() {
        let type_params = vec![TypeParameter {
            name: "T".to_string(),
            bounds: vec!["Clone".to_string()],
            default: None,
        }];
        let result = codegen_generic_params(&type_params, &["'a".to_string()]);
        let code = result.to_string();
        assert!(code.contains("'a"));
        assert!(code.contains("T"));
    }

    // ============================================
    // codegen_where_clause tests
    // ============================================

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
        assert!(code.contains("'a"));
        assert!(code.contains("'b"));
    }

    #[test]
    fn test_codegen_where_clause_multiple_bounds() {
        let bounds = vec![
            ("'a".to_string(), "'b".to_string()),
            ("'c".to_string(), "'d".to_string()),
        ];
        let result = codegen_where_clause(&bounds);
        let code = result.to_string();
        assert!(code.contains("where"));
        assert!(code.contains("'a"));
        assert!(code.contains("'c"));
    }

    // ============================================
    // codegen_function_attrs tests
    // ============================================

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
        let code = result[0].to_string();
        assert!(code.contains("doc"));
        assert!(code.contains("Test function"));
    }

    #[test]
    fn test_codegen_function_attrs_panic_free() {
        let props = FunctionProperties {
            panic_free: true,
            ..Default::default()
        };
        let result = codegen_function_attrs(&None, &props, &[]);
        assert_eq!(result.len(), 1);
        let code = result[0].to_string();
        assert!(code.contains("panic-free"));
    }

    #[test]
    fn test_codegen_function_attrs_always_terminates() {
        let props = FunctionProperties {
            always_terminates: true,
            ..Default::default()
        };
        let result = codegen_function_attrs(&None, &props, &[]);
        assert_eq!(result.len(), 1);
        let code = result[0].to_string();
        assert!(code.contains("terminate"));
    }

    #[test]
    fn test_codegen_function_attrs_custom() {
        let props = FunctionProperties::default();
        let result = codegen_function_attrs(&None, &props, &["inline".to_string()]);
        assert_eq!(result.len(), 1);
        let code = result[0].to_string();
        assert!(code.contains("inline"));
    }

    #[test]
    fn test_codegen_function_attrs_multiple_custom() {
        let props = FunctionProperties::default();
        let result = codegen_function_attrs(
            &None,
            &props,
            &["inline".to_string(), "must_use".to_string()],
        );
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_codegen_function_attrs_all() {
        let props = FunctionProperties {
            panic_free: true,
            always_terminates: true,
            ..Default::default()
        };
        let result =
            codegen_function_attrs(&Some("Docs".to_string()), &props, &["inline".to_string()]);
        assert_eq!(result.len(), 4); // doc, panic-free, terminates, inline
    }

    // ============================================
    // infer_expr_type_simple tests
    // ============================================

    #[test]
    fn test_infer_expr_type_simple_int() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert_eq!(infer_expr_type_simple(&expr), Type::Int);
    }

    #[test]
    fn test_infer_expr_type_simple_float() {
        let expr = HirExpr::Literal(Literal::Float(3.15));
        assert_eq!(infer_expr_type_simple(&expr), Type::Float);
    }

    #[test]
    fn test_infer_expr_type_simple_string() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert_eq!(infer_expr_type_simple(&expr), Type::String);
    }

    #[test]
    fn test_infer_expr_type_simple_bool() {
        let expr = HirExpr::Literal(Literal::Bool(true));
        assert_eq!(infer_expr_type_simple(&expr), Type::Bool);
    }

    #[test]
    fn test_infer_expr_type_simple_none() {
        let expr = HirExpr::Literal(Literal::None);
        assert_eq!(infer_expr_type_simple(&expr), Type::None);
    }

    #[test]
    fn test_infer_expr_type_simple_bytes() {
        let expr = HirExpr::Literal(Literal::Bytes(vec![1, 2, 3]));
        assert_eq!(
            infer_expr_type_simple(&expr),
            Type::Custom("Vec<u8>".to_string())
        );
    }

    #[test]
    fn test_infer_expr_type_simple_empty_list() {
        let expr = HirExpr::List(vec![]);
        assert_eq!(
            infer_expr_type_simple(&expr),
            Type::List(Box::new(Type::Unknown))
        );
    }

    #[test]
    fn test_infer_expr_type_simple_int_list() {
        let expr = HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))]);
        assert_eq!(
            infer_expr_type_simple(&expr),
            Type::List(Box::new(Type::Int))
        );
    }

    #[test]
    fn test_infer_expr_type_simple_empty_dict() {
        let expr = HirExpr::Dict(vec![]);
        assert_eq!(
            infer_expr_type_simple(&expr),
            Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown))
        );
    }

    #[test]
    fn test_infer_expr_type_simple_string_int_dict() {
        let expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);
        assert_eq!(
            infer_expr_type_simple(&expr),
            Type::Dict(Box::new(Type::String), Box::new(Type::Int))
        );
    }

    #[test]
    fn test_infer_expr_type_simple_empty_set() {
        let expr = HirExpr::Set(vec![]);
        assert_eq!(
            infer_expr_type_simple(&expr),
            Type::Set(Box::new(Type::Unknown))
        );
    }

    #[test]
    fn test_infer_expr_type_simple_int_set() {
        let expr = HirExpr::Set(vec![HirExpr::Literal(Literal::Int(1))]);
        assert_eq!(
            infer_expr_type_simple(&expr),
            Type::Set(Box::new(Type::Int))
        );
    }

    #[test]
    fn test_infer_expr_type_simple_tuple() {
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("s".to_string())),
        ]);
        assert_eq!(
            infer_expr_type_simple(&expr),
            Type::Tuple(vec![Type::Int, Type::String])
        );
    }

    #[test]
    fn test_infer_expr_type_simple_fstring() {
        let expr = HirExpr::FString { parts: vec![] };
        assert_eq!(infer_expr_type_simple(&expr), Type::String);
    }

    #[test]
    fn test_infer_expr_type_simple_comparison_eq() {
        use crate::hir::BinOp;
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Eq,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert_eq!(infer_expr_type_simple(&expr), Type::Bool);
    }

    #[test]
    fn test_infer_expr_type_simple_comparison_lt() {
        use crate::hir::BinOp;
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Lt,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert_eq!(infer_expr_type_simple(&expr), Type::Bool);
    }

    #[test]
    fn test_infer_expr_type_simple_logical_and() {
        use crate::hir::BinOp;
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Bool(true))),
            op: BinOp::And,
            right: Box::new(HirExpr::Literal(Literal::Bool(false))),
        };
        assert_eq!(infer_expr_type_simple(&expr), Type::Bool);
    }

    #[test]
    fn test_infer_expr_type_simple_logical_or() {
        use crate::hir::BinOp;
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Bool(true))),
            op: BinOp::Or,
            right: Box::new(HirExpr::Literal(Literal::Bool(false))),
        };
        assert_eq!(infer_expr_type_simple(&expr), Type::Bool);
    }

    #[test]
    fn test_infer_expr_type_simple_in() {
        use crate::hir::BinOp;
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::In,
            right: Box::new(HirExpr::List(vec![])),
        };
        assert_eq!(infer_expr_type_simple(&expr), Type::Bool);
    }

    #[test]
    fn test_infer_expr_type_simple_division() {
        use crate::hir::BinOp;
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Div,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert_eq!(infer_expr_type_simple(&expr), Type::Float);
    }

    #[test]
    fn test_infer_expr_type_simple_add_unknown() {
        use crate::hir::BinOp;
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        // Add returns Unknown because it could be int, float, or string concatenation
        assert_eq!(infer_expr_type_simple(&expr), Type::Unknown);
    }

    #[test]
    fn test_infer_expr_type_simple_unary_not() {
        use crate::hir::UnaryOp;
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Literal(Literal::Bool(true))),
        };
        assert_eq!(infer_expr_type_simple(&expr), Type::Bool);
    }

    #[test]
    fn test_infer_expr_type_simple_unary_neg() {
        use crate::hir::UnaryOp;
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        // Neg returns Unknown because it could be int or float
        assert_eq!(infer_expr_type_simple(&expr), Type::Unknown);
    }

    // Note: No HirExpr::Compare test - comparisons use Binary with comparison ops
    // which is already tested above (test_infer_expr_type_simple_comparison_*)

    #[test]
    fn test_infer_expr_type_simple_var() {
        let expr = HirExpr::Var("x".to_string());
        assert_eq!(infer_expr_type_simple(&expr), Type::Unknown);
    }
}
