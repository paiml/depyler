//! Class-to-Rust conversion: ADT pattern detection and enum generation.
//!
//! Extracted from `rust_gen.rs` as part of the Phase 2-7 decomposition.
//! This module handles converting Python class hierarchies to Rust structs/enums,
//! including detection of Abstract Data Type (ADT) patterns where an ABC parent
//! with `Generic[T,U,...]` and dataclass children maps to a Rust enum.

use crate::hir::*;
use anyhow::Result;
use quote::{quote, ToTokens};
use std::collections::{HashMap, HashSet};

/// Information about ADT patterns detected in the class hierarchy.
/// Complexity: 1
pub(super) struct AdtPatternInfo {
    /// Maps ABC class names to their child class names
    pub abc_to_children: HashMap<String, Vec<String>>,
    /// DEPYLER-0936: Reverse mapping from child class names to parent enum names.
    /// Used to rewrite return types like `ListIter<T>` -> `Iter<T>`.
    pub child_to_parent: HashMap<String, String>,
}

/// Convert Python classes to Rust structs.
///
/// Processes all classes and generates token streams.
/// DEPYLER-0839: Detects ADT patterns (ABC with Generic[T,U] + child dataclasses)
/// and generates Rust enums instead of separate structs.
/// DEPYLER-0936: Also returns child->parent mapping for type rewriting.
/// Complexity: 6 (within <=10 target)
pub(super) fn convert_classes_to_rust(
    classes: &[HirClass],
    type_mapper: &crate::type_mapper::TypeMapper,
    vararg_functions: &std::collections::HashSet<String>, // DEPYLER-0648: Track vararg functions
) -> Result<(Vec<proc_macro2::TokenStream>, HashMap<String, String>)> {
    // DEPYLER-0839: Phase 1 - Detect ADT patterns
    let adt_info = detect_adt_patterns(classes);

    let mut class_items = Vec::new();
    let mut processed_classes: HashSet<String> = HashSet::new();

    for class in classes {
        // Skip if already processed as part of an ADT
        if processed_classes.contains(&class.name) {
            continue;
        }

        // DEPYLER-0839: Check if this is an ABC that forms an ADT
        if let Some(children) = adt_info.abc_to_children.get(&class.name) {
            if !children.is_empty() && !class.type_params.is_empty() {
                // Generate enum for ADT pattern
                let tokens = generate_adt_enum(class, children, classes, type_mapper)?;
                class_items.push(tokens);

                // Mark all children as processed
                for child_name in children {
                    processed_classes.insert(child_name.clone());
                }
                processed_classes.insert(class.name.clone());
                continue;
            }
        }

        // DEPYLER-0648: Pass vararg_functions for proper call site generation in methods
        let items =
            crate::direct_rules::convert_class_to_struct(class, type_mapper, vararg_functions)?;
        for item in items {
            let tokens = item.to_token_stream();
            class_items.push(tokens);
        }
    }
    // DEPYLER-0936: Return both class items and child->parent mapping
    Ok((class_items, adt_info.child_to_parent))
}

/// Detect ADT patterns in the class hierarchy.
/// An ADT pattern is: ABC parent with Generic[T,U,...] + dataclass children.
/// Complexity: 4
pub(super) fn detect_adt_patterns(classes: &[HirClass]) -> AdtPatternInfo {
    let mut abc_to_children: HashMap<String, Vec<String>> = HashMap::new();

    // Build a map of class names for quick lookup
    let class_names: HashSet<&str> = classes.iter().map(|c| c.name.as_str()).collect();

    for class in classes {
        // Look for base classes that exist in our module
        for base in &class.base_classes {
            // Extract base class name (handle Generic[T] syntax like "Either[L, R]")
            let base_name = base.split('[').next().unwrap_or(base);

            if class_names.contains(base_name) {
                abc_to_children
                    .entry(base_name.to_string())
                    .or_default()
                    .push(class.name.clone());
            }
        }
    }

    // Filter to only keep ABC parents with type params (Generic[T,U,...])
    abc_to_children.retain(|parent_name, _| {
        classes
            .iter()
            .find(|c| c.name == *parent_name)
            .map(|c| {
                !c.type_params.is_empty()
                    && c.base_classes
                        .iter()
                        .any(|b| b.contains("ABC") || b.contains("Generic"))
            })
            .unwrap_or(false)
    });

    // DEPYLER-0936: Build reverse mapping from children to parents
    let mut child_to_parent = HashMap::new();
    for (parent, children) in &abc_to_children {
        for child in children {
            child_to_parent.insert(child.clone(), parent.clone());
        }
    }

    AdtPatternInfo {
        abc_to_children,
        child_to_parent,
    }
}

/// Generate a Rust enum for an ADT pattern.
/// Complexity: 7
pub(super) fn generate_adt_enum(
    parent: &HirClass,
    children: &[String],
    all_classes: &[HirClass],
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0900: Rename enum if it shadows stdlib type (e.g., Option -> PyOption)
    let safe_name = crate::direct_rules::safe_class_name(&parent.name);
    let enum_name = syn::Ident::new(&safe_name, proc_macro2::Span::call_site());

    // Build generic params with Clone bound
    let type_params: Vec<syn::Ident> = parent
        .type_params
        .iter()
        .map(|tp| syn::Ident::new(tp, proc_macro2::Span::call_site()))
        .collect();

    let generics = if type_params.is_empty() {
        quote! {}
    } else {
        quote! { <#(#type_params: Clone),*> }
    };

    let generics_no_bounds = if type_params.is_empty() {
        quote! {}
    } else {
        quote! { <#(#type_params),*> }
    };

    // Generate variants for each child
    let mut variants = Vec::new();

    for child_name in children {
        let child = all_classes.iter().find(|c| &c.name == child_name);
        if let Some(child_class) = child {
            // DEPYLER-0900: Rename variant if it shadows stdlib type (e.g., Some -> PySome)
            let safe_variant = crate::direct_rules::safe_class_name(&child_class.name);
            let variant_name = syn::Ident::new(&safe_variant, proc_macro2::Span::call_site());

            // Collect field types for this variant
            let field_types: Vec<proc_macro2::TokenStream> = child_class
                .fields
                .iter()
                .filter(|f| !f.is_class_var && f.name != "_phantom")
                .map(|f| {
                    let rust_type = type_mapper.map_type(&f.field_type);
                    crate::direct_rules::rust_type_to_syn_type(&rust_type)
                        .map(|t| quote! { #t })
                        .unwrap_or_else(|_| quote! { () })
                })
                .collect();

            if field_types.len() == 1 {
                let ft = &field_types[0];
                variants.push(quote! { #variant_name(#ft) });
            } else if field_types.is_empty() {
                variants.push(quote! { #variant_name });
            } else {
                variants.push(quote! { #variant_name(#(#field_types),*) });
            }
        }
    }

    // Generate methods from the parent class
    let methods = generate_adt_methods(parent, children, all_classes, type_mapper)?;

    let result = quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum #enum_name #generics {
            #(#variants),*
        }

        impl #generics #enum_name #generics_no_bounds {
            #methods
        }
    };

    Ok(result)
}

/// Generate impl methods for an ADT enum based on child implementations.
/// Complexity: 6
pub(super) fn generate_adt_methods(
    parent: &HirClass,
    children: &[String],
    all_classes: &[HirClass],
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<proc_macro2::TokenStream> {
    // For now, generate basic accessor methods
    // Full method translation requires deeper integration with stmt_gen/expr_gen

    let _type_params: Vec<syn::Ident> = parent
        .type_params
        .iter()
        .map(|tp| syn::Ident::new(tp, proc_macro2::Span::call_site()))
        .collect();

    // Generate is_left/is_right style methods for each variant
    let mut methods = Vec::new();

    for child_name in children {
        // DEPYLER-0900: Rename variant if it shadows stdlib type (e.g., Some -> PySome)
        let safe_variant = crate::direct_rules::safe_class_name(child_name);
        let variant_name = syn::Ident::new(&safe_variant, proc_macro2::Span::call_site());
        let method_name_str = format!("is_{}", safe_variant.to_lowercase());
        let method_name = syn::Ident::new(&method_name_str, proc_macro2::Span::call_site());

        methods.push(quote! {
            pub fn #method_name(&self) -> bool {
                matches!(self, Self::#variant_name(..))
            }
        });
    }

    // Generate new constructors for each variant
    for child_name in children {
        let child = all_classes.iter().find(|c| &c.name == child_name);
        if let Some(child_class) = child {
            // DEPYLER-0900: Rename variant if it shadows stdlib type (e.g., Some -> PySome)
            let safe_variant = crate::direct_rules::safe_class_name(&child_class.name);
            let variant_name = syn::Ident::new(&safe_variant, proc_macro2::Span::call_site());
            let method_name_str = format!("new_{}", safe_variant.to_lowercase());
            let method_name = syn::Ident::new(&method_name_str, proc_macro2::Span::call_site());

            let fields: Vec<_> = child_class
                .fields
                .iter()
                .filter(|f| !f.is_class_var && f.name != "_phantom")
                .collect();

            if fields.len() == 1 {
                let field = &fields[0];
                let field_name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
                let rust_type = type_mapper.map_type(&field.field_type);
                let field_type = crate::direct_rules::rust_type_to_syn_type(&rust_type)?;

                methods.push(quote! {
                    pub fn #method_name(#field_name: #field_type) -> Self {
                        Self::#variant_name(#field_name)
                    }
                });
            }
        }
    }

    Ok(quote! { #(#methods)* })
}
