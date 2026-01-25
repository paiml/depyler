use crate::hir::Type;
use crate::type_mapper::RustType;
use quote::quote;
use std::collections::HashMap;

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

        // DEPYLER-0962: Add serde derives for JSON serialization/deserialization
        let enum_def = quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

    // ============ generate_enum_name tests ============

    #[test]
    fn test_generate_enum_name_two_types() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Int, Type::String];
        let name = generator.generate_enum_name(&types);
        assert_eq!(name, "IntOrStringUnion");
    }

    #[test]
    fn test_generate_enum_name_three_types() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Int, Type::Float, Type::String];
        let name = generator.generate_enum_name(&types);
        assert_eq!(name, "IntOrFloatOrStringUnion");
    }

    #[test]
    fn test_generate_enum_name_more_than_three() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Int, Type::Float, Type::String, Type::Bool];
        let name = generator.generate_enum_name(&types);
        assert_eq!(name, "UnionType1");
    }

    #[test]
    fn test_generate_enum_name_increments_counter() {
        let mut generator = UnionEnumGenerator::new();
        let types1 = vec![Type::Int, Type::Float, Type::String, Type::Bool];
        let types2 = vec![Type::Int, Type::Float, Type::String, Type::None];
        let name1 = generator.generate_enum_name(&types1);
        let name2 = generator.generate_enum_name(&types2);
        assert_eq!(name1, "UnionType1");
        assert_eq!(name2, "UnionType2");
    }

    #[test]
    fn test_generate_enum_name_list_type() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::List(Box::new(Type::Int)), Type::None];
        let name = generator.generate_enum_name(&types);
        assert_eq!(name, "ListOrNoneUnion");
    }

    #[test]
    fn test_generate_enum_name_dict_type() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
            Type::None,
        ];
        let name = generator.generate_enum_name(&types);
        assert_eq!(name, "DictOrNoneUnion");
    }

    #[test]
    fn test_generate_enum_name_custom_type() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Custom("MyClass".to_string()), Type::None];
        let name = generator.generate_enum_name(&types);
        assert_eq!(name, "MyClassOrNoneUnion");
    }

    // ============ type_to_variant_name tests ============

    #[test]
    fn test_variant_name_int() {
        let generator = UnionEnumGenerator::new();
        assert_eq!(generator.type_to_variant_name(&Type::Int, 0), "Integer");
    }

    #[test]
    fn test_variant_name_float() {
        let generator = UnionEnumGenerator::new();
        assert_eq!(generator.type_to_variant_name(&Type::Float, 0), "Float");
    }

    #[test]
    fn test_variant_name_string() {
        let generator = UnionEnumGenerator::new();
        assert_eq!(generator.type_to_variant_name(&Type::String, 0), "Text");
    }

    #[test]
    fn test_variant_name_bool() {
        let generator = UnionEnumGenerator::new();
        assert_eq!(generator.type_to_variant_name(&Type::Bool, 0), "Boolean");
    }

    #[test]
    fn test_variant_name_none() {
        let generator = UnionEnumGenerator::new();
        assert_eq!(generator.type_to_variant_name(&Type::None, 0), "None");
    }

    #[test]
    fn test_variant_name_list() {
        let generator = UnionEnumGenerator::new();
        assert_eq!(
            generator.type_to_variant_name(&Type::List(Box::new(Type::Int)), 0),
            "List"
        );
    }

    #[test]
    fn test_variant_name_dict() {
        let generator = UnionEnumGenerator::new();
        let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        assert_eq!(generator.type_to_variant_name(&dict_type, 0), "Dict");
    }

    #[test]
    fn test_variant_name_custom() {
        let generator = UnionEnumGenerator::new();
        assert_eq!(
            generator.type_to_variant_name(&Type::Custom("MyClass".to_string()), 0),
            "MyClass"
        );
    }

    #[test]
    fn test_variant_name_typevar() {
        let generator = UnionEnumGenerator::new();
        assert_eq!(
            generator.type_to_variant_name(&Type::TypeVar("T".to_string()), 0),
            "TypeT"
        );
    }

    #[test]
    fn test_variant_name_unknown() {
        let generator = UnionEnumGenerator::new();
        assert_eq!(
            generator.type_to_variant_name(&Type::Unknown, 5),
            "Variant5"
        );
    }

    // ============ type_to_rust_type tests ============

    #[test]
    fn test_type_to_rust_type_int() {
        let generator = UnionEnumGenerator::new();
        let rust_type = generator.type_to_rust_type(&Type::Int);
        assert!(matches!(
            rust_type,
            RustType::Primitive(crate::type_mapper::PrimitiveType::I32)
        ));
    }

    #[test]
    fn test_type_to_rust_type_float() {
        let generator = UnionEnumGenerator::new();
        let rust_type = generator.type_to_rust_type(&Type::Float);
        assert!(matches!(
            rust_type,
            RustType::Primitive(crate::type_mapper::PrimitiveType::F64)
        ));
    }

    #[test]
    fn test_type_to_rust_type_string() {
        let generator = UnionEnumGenerator::new();
        let rust_type = generator.type_to_rust_type(&Type::String);
        assert!(matches!(rust_type, RustType::String));
    }

    #[test]
    fn test_type_to_rust_type_bool() {
        let generator = UnionEnumGenerator::new();
        let rust_type = generator.type_to_rust_type(&Type::Bool);
        assert!(matches!(
            rust_type,
            RustType::Primitive(crate::type_mapper::PrimitiveType::Bool)
        ));
    }

    // ============ generate_union_enum integration tests ============

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

    #[test]
    fn test_union_enum_generates_from_impls() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Int, Type::String];
        let (_, tokens) = generator.generate_union_enum(&types);
        let code = tokens.to_string();
        // Should generate From implementations for each non-None type
        assert!(code.contains("impl From"));
    }

    #[test]
    fn test_union_enum_generates_as_methods() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Int, Type::String];
        let (_, tokens) = generator.generate_union_enum(&types);
        let code = tokens.to_string();
        // Should generate as_* methods
        assert!(code.contains("as_integer") || code.contains("as_text"));
    }

    #[test]
    fn test_union_enum_derives() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Int, Type::String];
        let (_, tokens) = generator.generate_union_enum(&types);
        let code = tokens.to_string();
        // Should include serde derives for JSON support
        assert!(code.contains("Serialize"));
        assert!(code.contains("Deserialize"));
    }

    #[test]
    fn test_union_enum_with_float() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Float, Type::None];
        let (name, tokens) = generator.generate_union_enum(&types);
        assert_eq!(name, "FloatOrNoneUnion");
        let code = tokens.to_string();
        assert!(code.contains("is_float"));
    }

    #[test]
    fn test_union_enum_only_primitives() {
        let mut generator = UnionEnumGenerator::new();
        let types = vec![Type::Int, Type::Float, Type::Bool];
        let (name, tokens) = generator.generate_union_enum(&types);
        assert_eq!(name, "BoolOrFloatOrIntUnion"); // Sorted alphabetically by type debug string
        let code = tokens.to_string();
        assert!(code.contains("is_integer"));
        assert!(code.contains("is_float"));
        assert!(code.contains("is_boolean"));
    }
}
