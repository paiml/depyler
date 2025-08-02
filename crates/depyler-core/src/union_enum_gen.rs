use crate::hir::Type;
use crate::type_mapper::RustType;
use std::collections::HashMap;
use quote::quote;

/// Generates Rust enum types for Python Union types
#[derive(Debug, Default)]
pub struct UnionEnumGenerator {
    /// Counter for generating unique enum names
    enum_counter: usize,
    /// Cache of generated enums by their variant types
    enum_cache: HashMap<Vec<Type>, String>,
}

impl UnionEnumGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate an enum for a Union type, returning the enum name
    pub fn generate_union_enum(&mut self, types: &[Type]) -> (String, proc_macro2::TokenStream) {
        // Check if we've already generated an enum for these types
        let mut sorted_types = types.to_vec();
        sorted_types.sort_by_key(|t| format!("{:?}", t));
        
        if let Some(cached_name) = self.enum_cache.get(&sorted_types) {
            return (cached_name.clone(), quote! {});
        }

        // Generate a descriptive enum name
        let enum_name = self.generate_enum_name(&sorted_types);
        
        // Generate variant names and types
        let variants: Vec<_> = sorted_types
            .iter()
            .enumerate()
            .map(|(i, ty)| {
                let variant_name = self.type_to_variant_name(ty, i);
                (variant_name, ty)
            })
            .collect();

        // Generate the enum definition
        let enum_ident = syn::Ident::new(&enum_name, proc_macro2::Span::call_site());
        let variant_defs: Vec<_> = variants
            .iter()
            .map(|(name, ty)| {
                let variant_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                let rust_type = self.type_to_rust_type(ty);
                match rust_type {
                    RustType::Unit => quote! { #variant_ident },
                    _ => {
                        let ty_tokens = match crate::rust_gen::rust_type_to_syn(&rust_type) {
                            Ok(tokens) => tokens,
                            Err(_) => syn::parse_quote! { () },
                        };
                        quote! { #variant_ident(#ty_tokens) }
                    }
                }
            })
            .collect();

        let enum_def = quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub enum #enum_ident {
                #(#variant_defs),*
            }
        };

        // Generate conversion implementations
        let from_impls = self.generate_from_impls(&enum_ident, &variants);
        
        // Generate match helper methods
        let match_methods = self.generate_match_methods(&enum_ident, &variants);

        let full_definition = quote! {
            #enum_def
            
            #from_impls
            
            impl #enum_ident {
                #match_methods
            }
        };

        // Cache the enum name
        self.enum_cache.insert(sorted_types, enum_name.clone());
        
        (enum_name, full_definition)
    }

    fn generate_enum_name(&mut self, types: &[Type]) -> String {
        // Try to generate a meaningful name based on types
        let type_names: Vec<String> = types
            .iter()
            .map(|t| match t {
                Type::Int => "Int".to_string(),
                Type::Float => "Float".to_string(),
                Type::String => "String".to_string(),
                Type::Bool => "Bool".to_string(),
                Type::None => "None".to_string(),
                Type::List(_) => "List".to_string(),
                Type::Dict(_, _) => "Dict".to_string(),
                Type::Custom(name) => name.clone(),
                _ => "Type".to_string(),
            })
            .collect();

        if type_names.len() <= 3 {
            format!("{}Union", type_names.join("Or"))
        } else {
            self.enum_counter += 1;
            format!("UnionType{}", self.enum_counter)
        }
    }

    fn type_to_variant_name(&self, ty: &Type, index: usize) -> String {
        match ty {
            Type::Int => "Integer".to_string(),
            Type::Float => "Float".to_string(),
            Type::String => "Text".to_string(),
            Type::Bool => "Boolean".to_string(),
            Type::None => "None".to_string(),
            Type::List(_) => "List".to_string(),
            Type::Dict(_, _) => "Dict".to_string(),
            Type::Custom(name) => name.clone(),
            Type::TypeVar(name) => format!("Type{}", name),
            _ => format!("Variant{}", index),
        }
    }

    fn type_to_rust_type(&self, ty: &Type) -> RustType {
        // Convert HIR Type to RustType
        let mapper = crate::type_mapper::TypeMapper::new();
        mapper.map_type(ty)
    }

    fn generate_from_impls(
        &self,
        enum_ident: &syn::Ident,
        variants: &[(String, &Type)],
    ) -> proc_macro2::TokenStream {
        let from_impls: Vec<_> = variants
            .iter()
            .filter(|(_, ty)| !matches!(ty, Type::None))
            .map(|(variant_name, ty)| {
                let variant_ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                let rust_type = self.type_to_rust_type(ty);
                let ty_tokens = match crate::rust_gen::rust_type_to_syn(&rust_type) {
                    Ok(tokens) => tokens,
                    Err(_) => syn::parse_quote! { () },
                };
                
                quote! {
                    impl From<#ty_tokens> for #enum_ident {
                        fn from(value: #ty_tokens) -> Self {
                            #enum_ident::#variant_ident(value)
                        }
                    }
                }
            })
            .collect();

        quote! {
            #(#from_impls)*
        }
    }

    fn generate_match_methods(
        &self,
        enum_ident: &syn::Ident,
        variants: &[(String, &Type)],
    ) -> proc_macro2::TokenStream {
        let is_methods: Vec<_> = variants
            .iter()
            .map(|(variant_name, ty)| {
                let method_name = syn::Ident::new(
                    &format!("is_{}", variant_name.to_lowercase()),
                    proc_macro2::Span::call_site(),
                );
                let variant_ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                
                let pattern = if matches!(ty, Type::None) {
                    quote! { #enum_ident::#variant_ident }
                } else {
                    quote! { #enum_ident::#variant_ident(_) }
                };
                
                quote! {
                    pub fn #method_name(&self) -> bool {
                        matches!(self, #pattern)
                    }
                }
            })
            .collect();

        let as_methods: Vec<_> = variants
            .iter()
            .filter(|(_, ty)| !matches!(ty, Type::None))
            .map(|(variant_name, ty)| {
                let method_name = syn::Ident::new(
                    &format!("as_{}", variant_name.to_lowercase()),
                    proc_macro2::Span::call_site(),
                );
                let variant_ident = syn::Ident::new(variant_name, proc_macro2::Span::call_site());
                let rust_type = self.type_to_rust_type(ty);
                let ty_tokens = match crate::rust_gen::rust_type_to_syn(&rust_type) {
                    Ok(tokens) => tokens,
                    Err(_) => syn::parse_quote! { () },
                };
                
                quote! {
                    pub fn #method_name(&self) -> Option<&#ty_tokens> {
                        match self {
                            #enum_ident::#variant_ident(value) => Some(value),
                            _ => None,
                        }
                    }
                }
            })
            .collect();

        quote! {
            #(#is_methods)*
            #(#as_methods)*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_union_enum() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Int, Type::String];
        
        let (name, _tokens) = generator.generate_union_enum(&types);
        assert_eq!(name, "IntOrStringUnion");
    }

    #[test]
    fn test_union_with_none() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::String, Type::None];
        
        let (name, tokens) = generator.generate_union_enum(&types);
        assert_eq!(name, "NoneOrStringUnion");
        
        // Should generate is_none and is_text methods
        let code = tokens.to_string();
        assert!(code.contains("is_none"));
        assert!(code.contains("is_text"));
    }

    #[test]
    fn test_complex_union() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![
            Type::Int,
            Type::String,
            Type::List(Box::new(Type::Float)),
            Type::Bool,
        ];
        
        let (name, _) = generator.generate_union_enum(&types);
        assert_eq!(name, "UnionType1");
    }

    #[test]
    fn test_enum_caching() {
        let mut generator = UnionEnumGenerator::new();
        let types1 = vec![Type::Int, Type::String];
        let types2 = vec![Type::String, Type::Int]; // Different order
        
        let (name1, _) = generator.generate_union_enum(&types1);
        let (name2, tokens2) = generator.generate_union_enum(&types2);
        
        // Should return the same enum name (cached)
        assert_eq!(name1, name2);
        // Second call should return empty tokens (no redefinition)
        assert_eq!(tokens2.to_string(), "");
    }
}