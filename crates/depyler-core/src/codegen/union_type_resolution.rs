//! Union type resolution for Rust code generation
//!
//! This module handles Python union types (e.g., `int | str`, `Optional[T]`)
//! and resolves them to appropriate Rust type representations.
//!
//! # Strategies
//! - `int | float` → `f64` (widest numeric)
//! - `T | None` → `Option<T>`
//! - All same type → that type
//! - Complex unions → `serde_json::Value`

use crate::hir::Type;
use quote::quote;

/// DEPYLER-0765: Resolve Python union types to valid Rust types
///
/// Strategy:
/// 1. `int | float` → `f64` (widest numeric type)
/// 2. `T | None` → `Option<T>` (optional type)
/// 3. All same type → that type
/// 4. Complex unions → `serde_json::Value` (catch-all)
pub fn resolve_union_type(
    types: &[Type],
    type_converter: impl Fn(&Type) -> proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    // Filter out duplicates and check for None
    let has_none = types.iter().any(is_none_type);
    let non_none_types: Vec<&Type> = types.iter().filter(|t| !is_none_type(t)).collect();

    // Case 1: T | None → Option<T>
    if has_none && non_none_types.len() == 1 {
        let inner = type_converter(non_none_types[0]);
        return quote! { Option<#inner> };
    }

    // Case 2: Only None → ()
    if non_none_types.is_empty() {
        return quote! { () };
    }

    // Case 3: All numeric types → use widest (f64)
    let all_numeric = non_none_types.iter().all(|&t| is_numeric_type(t));
    if all_numeric {
        // If float present, use f64; if only int, use i64
        let has_float = non_none_types.iter().any(|&t| is_float_type(t));
        if has_float {
            return quote! { f64 };
        } else {
            return quote! { i64 };
        }
    }

    // Case 4: All string types → String
    let all_string = non_none_types.iter().all(|&t| is_string_type(t));
    if all_string {
        return quote! { String };
    }

    // Case 5: All same type → that type
    if non_none_types.len() == 1 {
        return type_converter(non_none_types[0]);
    }

    // Case 6: Check for int | str or other common patterns → use serde_json::Value
    // This is the safest catch-all for heterogeneous unions
    quote! { serde_json::Value }
}

/// Check if a type is None (including Custom("None"))
pub fn is_none_type(t: &Type) -> bool {
    matches!(t, Type::None) || matches!(t, Type::Custom(n) if n == "None" || n == "NoneType")
}

/// Check if a type is numeric (including Custom variants)
pub fn is_numeric_type(t: &Type) -> bool {
    matches!(t, Type::Int | Type::Float)
        || matches!(t, Type::Custom(n) if n == "int" || n == "float" || n == "i64" || n == "f64")
}

/// Check if a type is float-like
pub fn is_float_type(t: &Type) -> bool {
    matches!(t, Type::Float) || matches!(t, Type::Custom(n) if n == "float" || n == "f64")
}

/// Check if a type is string-like
pub fn is_string_type(t: &Type) -> bool {
    matches!(t, Type::String) || matches!(t, Type::Custom(n) if n == "str" || n == "String")
}

/// Check if a type is boolean
pub fn is_bool_type(t: &Type) -> bool {
    matches!(t, Type::Bool) || matches!(t, Type::Custom(n) if n == "bool")
}

/// Check if a type is a collection (list, dict, set)
pub fn is_collection_type(t: &Type) -> bool {
    matches!(
        t,
        Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::Tuple(_)
    )
}

/// Classify a union type for error messages and debugging
pub fn classify_union(types: &[Type]) -> UnionClassification {
    let has_none = types.iter().any(is_none_type);
    let non_none: Vec<&Type> = types.iter().filter(|t| !is_none_type(t)).collect();

    if non_none.is_empty() {
        return UnionClassification::OnlyNone;
    }

    if has_none && non_none.len() == 1 {
        return UnionClassification::Optional;
    }

    if non_none.iter().all(|t| is_numeric_type(t)) {
        return UnionClassification::NumericUnion;
    }

    if non_none.iter().all(|t| is_string_type(t)) {
        return UnionClassification::StringUnion;
    }

    UnionClassification::Heterogeneous
}

/// Classification of a union type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnionClassification {
    /// Only None type
    OnlyNone,
    /// T | None pattern
    Optional,
    /// All numeric types (int, float)
    NumericUnion,
    /// All string types
    StringUnion,
    /// Mixed types requiring serde_json::Value
    Heterogeneous,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ is_none_type tests ============

    #[test]
    fn test_is_none_type_none() {
        assert!(is_none_type(&Type::None));
    }

    #[test]
    fn test_is_none_type_custom_none() {
        assert!(is_none_type(&Type::Custom("None".to_string())));
    }

    #[test]
    fn test_is_none_type_custom_nonetype() {
        assert!(is_none_type(&Type::Custom("NoneType".to_string())));
    }

    #[test]
    fn test_is_none_type_int() {
        assert!(!is_none_type(&Type::Int));
    }

    #[test]
    fn test_is_none_type_string() {
        assert!(!is_none_type(&Type::String));
    }

    #[test]
    fn test_is_none_type_custom_other() {
        assert!(!is_none_type(&Type::Custom("MyClass".to_string())));
    }

    // ============ is_numeric_type tests ============

    #[test]
    fn test_is_numeric_type_int() {
        assert!(is_numeric_type(&Type::Int));
    }

    #[test]
    fn test_is_numeric_type_float() {
        assert!(is_numeric_type(&Type::Float));
    }

    #[test]
    fn test_is_numeric_type_custom_int() {
        assert!(is_numeric_type(&Type::Custom("int".to_string())));
    }

    #[test]
    fn test_is_numeric_type_custom_float() {
        assert!(is_numeric_type(&Type::Custom("float".to_string())));
    }

    #[test]
    fn test_is_numeric_type_custom_i64() {
        assert!(is_numeric_type(&Type::Custom("i64".to_string())));
    }

    #[test]
    fn test_is_numeric_type_custom_f64() {
        assert!(is_numeric_type(&Type::Custom("f64".to_string())));
    }

    #[test]
    fn test_is_numeric_type_string() {
        assert!(!is_numeric_type(&Type::String));
    }

    #[test]
    fn test_is_numeric_type_bool() {
        assert!(!is_numeric_type(&Type::Bool));
    }

    // ============ is_float_type tests ============

    #[test]
    fn test_is_float_type_float() {
        assert!(is_float_type(&Type::Float));
    }

    #[test]
    fn test_is_float_type_custom_float() {
        assert!(is_float_type(&Type::Custom("float".to_string())));
    }

    #[test]
    fn test_is_float_type_custom_f64() {
        assert!(is_float_type(&Type::Custom("f64".to_string())));
    }

    #[test]
    fn test_is_float_type_int() {
        assert!(!is_float_type(&Type::Int));
    }

    #[test]
    fn test_is_float_type_custom_int() {
        assert!(!is_float_type(&Type::Custom("int".to_string())));
    }

    // ============ is_string_type tests ============

    #[test]
    fn test_is_string_type_string() {
        assert!(is_string_type(&Type::String));
    }

    #[test]
    fn test_is_string_type_custom_str() {
        assert!(is_string_type(&Type::Custom("str".to_string())));
    }

    #[test]
    fn test_is_string_type_custom_string() {
        assert!(is_string_type(&Type::Custom("String".to_string())));
    }

    #[test]
    fn test_is_string_type_int() {
        assert!(!is_string_type(&Type::Int));
    }

    #[test]
    fn test_is_string_type_custom_other() {
        assert!(!is_string_type(&Type::Custom("bytes".to_string())));
    }

    // ============ is_bool_type tests ============

    #[test]
    fn test_is_bool_type_bool() {
        assert!(is_bool_type(&Type::Bool));
    }

    #[test]
    fn test_is_bool_type_custom_bool() {
        assert!(is_bool_type(&Type::Custom("bool".to_string())));
    }

    #[test]
    fn test_is_bool_type_int() {
        assert!(!is_bool_type(&Type::Int));
    }

    // ============ is_collection_type tests ============

    #[test]
    fn test_is_collection_type_list() {
        assert!(is_collection_type(&Type::List(Box::new(Type::Int))));
    }

    #[test]
    fn test_is_collection_type_dict() {
        assert!(is_collection_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
    }

    #[test]
    fn test_is_collection_type_set() {
        assert!(is_collection_type(&Type::Set(Box::new(Type::Int))));
    }

    #[test]
    fn test_is_collection_type_tuple() {
        assert!(is_collection_type(&Type::Tuple(vec![
            Type::Int,
            Type::String
        ])));
    }

    #[test]
    fn test_is_collection_type_int() {
        assert!(!is_collection_type(&Type::Int));
    }

    #[test]
    fn test_is_collection_type_string() {
        assert!(!is_collection_type(&Type::String));
    }

    // ============ classify_union tests ============

    #[test]
    fn test_classify_union_only_none() {
        let types = vec![Type::None];
        assert_eq!(classify_union(&types), UnionClassification::OnlyNone);
    }

    #[test]
    fn test_classify_union_optional_int() {
        let types = vec![Type::Int, Type::None];
        assert_eq!(classify_union(&types), UnionClassification::Optional);
    }

    #[test]
    fn test_classify_union_optional_string() {
        let types = vec![Type::None, Type::String];
        assert_eq!(classify_union(&types), UnionClassification::Optional);
    }

    #[test]
    fn test_classify_union_numeric_int_float() {
        let types = vec![Type::Int, Type::Float];
        assert_eq!(classify_union(&types), UnionClassification::NumericUnion);
    }

    #[test]
    fn test_classify_union_string_union() {
        let types = vec![Type::String, Type::Custom("str".to_string())];
        assert_eq!(classify_union(&types), UnionClassification::StringUnion);
    }

    #[test]
    fn test_classify_union_heterogeneous() {
        let types = vec![Type::Int, Type::String];
        assert_eq!(classify_union(&types), UnionClassification::Heterogeneous);
    }

    #[test]
    fn test_classify_union_heterogeneous_with_bool() {
        let types = vec![Type::Int, Type::Bool];
        assert_eq!(classify_union(&types), UnionClassification::Heterogeneous);
    }

    // ============ resolve_union_type tests ============

    fn simple_type_converter(t: &Type) -> proc_macro2::TokenStream {
        match t {
            Type::Int => quote! { i32 },
            Type::Float => quote! { f64 },
            Type::String => quote! { String },
            Type::Bool => quote! { bool },
            _ => quote! { () },
        }
    }

    #[test]
    fn test_resolve_union_type_optional_int() {
        let types = vec![Type::Int, Type::None];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("Option"));
        assert!(code.contains("i32"));
    }

    #[test]
    fn test_resolve_union_type_optional_string() {
        let types = vec![Type::String, Type::None];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("Option"));
        assert!(code.contains("String"));
    }

    #[test]
    fn test_resolve_union_type_only_none() {
        let types = vec![Type::None];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("()"));
    }

    #[test]
    fn test_resolve_union_type_int_float_to_f64() {
        let types = vec![Type::Int, Type::Float];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("f64"));
    }

    #[test]
    fn test_resolve_union_type_int_only_to_i64() {
        let types = vec![Type::Int, Type::Custom("int".to_string())];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("i64"));
    }

    #[test]
    fn test_resolve_union_type_all_strings() {
        let types = vec![Type::String, Type::Custom("str".to_string())];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("String"));
    }

    #[test]
    fn test_resolve_union_type_single_non_none() {
        // Single numeric type goes through the "all numeric" path → i64
        let types = vec![Type::Int];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("i64"));
    }

    #[test]
    fn test_resolve_union_type_heterogeneous_int_string() {
        let types = vec![Type::Int, Type::String];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("serde_json") || code.contains("Value"));
    }

    #[test]
    fn test_resolve_union_type_heterogeneous_int_bool() {
        let types = vec![Type::Int, Type::Bool];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("serde_json") || code.contains("Value"));
    }

    #[test]
    fn test_resolve_union_type_none_custom_none() {
        let types = vec![Type::None, Type::Custom("None".to_string())];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("()"));
    }

    #[test]
    fn test_resolve_union_type_optional_with_custom_none() {
        let types = vec![Type::Int, Type::Custom("NoneType".to_string())];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("Option"));
    }

    #[test]
    fn test_resolve_union_type_triple_numeric() {
        let types = vec![Type::Int, Type::Float, Type::Custom("float".to_string())];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        assert!(code.contains("f64"));
    }

    #[test]
    fn test_resolve_union_type_complex_with_none() {
        let types = vec![Type::Int, Type::String, Type::None];
        let tokens = resolve_union_type(&types, simple_type_converter);
        let code = tokens.to_string();
        // int | str | None is still heterogeneous (serde_json::Value)
        assert!(code.contains("serde_json") || code.contains("Value"));
    }
}
