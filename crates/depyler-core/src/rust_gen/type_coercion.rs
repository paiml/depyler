//! Type coercion utilities for code generation
//!
//! This module provides helper functions for type coercion and conversion
//! in Python-to-Rust transpilation.

use crate::hir::{HirExpr, Literal, Type};
use syn::parse_quote;

/// Check if an expression is an integer literal
pub fn is_int_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::Int(_)))
}

/// Check if an expression is a float literal
pub fn is_float_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::Float(_)))
}

/// Check if an expression is a numeric literal (int or float)
pub fn is_numeric_literal(expr: &HirExpr) -> bool {
    matches!(
        expr,
        HirExpr::Literal(Literal::Int(_)) | HirExpr::Literal(Literal::Float(_))
    )
}

/// Check if a type is numeric (Int or Float)
pub fn is_numeric_type(ty: &Type) -> bool {
    matches!(ty, Type::Int | Type::Float)
}

/// Check if a type is an integer type
pub fn is_int_type(ty: &Type) -> bool {
    matches!(ty, Type::Int)
}

/// Check if a type is a float type
pub fn is_float_type(ty: &Type) -> bool {
    matches!(ty, Type::Float)
}

/// Check if a type is a string type
pub fn is_string_type(ty: &Type) -> bool {
    matches!(ty, Type::String)
}

/// Check if a type is a boolean type
pub fn is_bool_type(ty: &Type) -> bool {
    matches!(ty, Type::Bool)
}

/// Check if a type is a container type (list, dict, set, tuple)
pub fn is_container_type(ty: &Type) -> bool {
    matches!(
        ty,
        Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::Tuple(_)
    )
}

/// Check if a type is an optional type
pub fn is_optional_type(ty: &Type) -> bool {
    matches!(ty, Type::Optional(_))
}

/// Get the inner type of an optional type
pub fn unwrap_optional(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Optional(inner) => Some(inner),
        _ => None,
    }
}

/// Get the element type of a list
pub fn unwrap_list(ty: &Type) -> Option<&Type> {
    match ty {
        Type::List(inner) => Some(inner),
        _ => None,
    }
}

/// Get the key and value types of a dict
pub fn unwrap_dict(ty: &Type) -> Option<(&Type, &Type)> {
    match ty {
        Type::Dict(k, v) => Some((k, v)),
        _ => None,
    }
}

/// Get the element type of a set
pub fn unwrap_set(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Set(inner) => Some(inner),
        _ => None,
    }
}

/// Check if expression represents zero
pub fn is_zero_literal(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Literal(Literal::Int(0)) => true,
        HirExpr::Literal(Literal::Float(f)) => *f == 0.0,
        _ => false,
    }
}

/// Check if expression represents one
pub fn is_one_literal(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Literal(Literal::Int(1)) => true,
        HirExpr::Literal(Literal::Float(f)) => *f == 1.0,
        _ => false,
    }
}

/// Check if expression is a negative integer literal
pub fn is_negative_int_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::Int(n)) if *n < 0)
}

/// Check if expression is a positive integer literal
pub fn is_positive_int_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::Int(n)) if *n > 0)
}

/// Get the integer value from an int literal
pub fn get_int_value(expr: &HirExpr) -> Option<i64> {
    match expr {
        HirExpr::Literal(Literal::Int(n)) => Some(*n),
        _ => None,
    }
}

/// Get the float value from a float literal
pub fn get_float_value(expr: &HirExpr) -> Option<f64> {
    match expr {
        HirExpr::Literal(Literal::Float(f)) => Some(*f),
        _ => None,
    }
}

/// Get the string value from a string literal
pub fn get_string_value(expr: &HirExpr) -> Option<&str> {
    match expr {
        HirExpr::Literal(Literal::String(s)) => Some(s),
        _ => None,
    }
}

/// Get the boolean value from a bool literal
pub fn get_bool_value(expr: &HirExpr) -> Option<bool> {
    match expr {
        HirExpr::Literal(Literal::Bool(b)) => Some(*b),
        _ => None,
    }
}

/// Generate a cast expression to i32
pub fn cast_to_i32(expr: syn::Expr) -> syn::Expr {
    parse_quote! { (#expr as i32) }
}

/// Generate a cast expression to i64
pub fn cast_to_i64(expr: syn::Expr) -> syn::Expr {
    parse_quote! { (#expr as i64) }
}

/// Generate a cast expression to f32
pub fn cast_to_f32(expr: syn::Expr) -> syn::Expr {
    parse_quote! { (#expr as f32) }
}

/// Generate a cast expression to f64
pub fn cast_to_f64(expr: syn::Expr) -> syn::Expr {
    parse_quote! { (#expr as f64) }
}

/// Generate a cast expression to usize
pub fn cast_to_usize(expr: syn::Expr) -> syn::Expr {
    parse_quote! { (#expr as usize) }
}

/// Generate a cast expression to isize
pub fn cast_to_isize(expr: syn::Expr) -> syn::Expr {
    parse_quote! { (#expr as isize) }
}

/// Check if conversion between types requires explicit cast
pub fn needs_cast(from: &Type, to: &Type) -> bool {
    match (from, to) {
        // Same types don't need cast
        (Type::Int, Type::Int) => false,
        (Type::Float, Type::Float) => false,
        (Type::String, Type::String) => false,
        (Type::Bool, Type::Bool) => false,
        // Int to Float needs cast
        (Type::Int, Type::Float) => true,
        // Float to Int needs cast
        (Type::Float, Type::Int) => true,
        // Bool can be used as int in Python
        (Type::Bool, Type::Int) => true,
        // Int can be used as bool (truthiness)
        (Type::Int, Type::Bool) => true,
        // String conversions
        (Type::String, Type::Int) => true,
        (Type::String, Type::Float) => true,
        (Type::Int, Type::String) => true,
        (Type::Float, Type::String) => true,
        // Default: need cast for different types
        _ => from != to,
    }
}

/// Check if a type can be used in arithmetic operations
pub fn is_arithmetic_type(ty: &Type) -> bool {
    matches!(ty, Type::Int | Type::Float)
}

/// Check if a type can be compared
pub fn is_comparable_type(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Int | Type::Float | Type::String | Type::Bool
    )
}

/// Check if a type supports the 'in' operator
pub fn supports_containment(ty: &Type) -> bool {
    matches!(
        ty,
        Type::List(_) | Type::Set(_) | Type::Dict(_, _) | Type::String | Type::Tuple(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // is_int_literal tests
    // ============================================================================

    #[test]
    fn test_is_int_literal_true() {
        assert!(is_int_literal(&HirExpr::Literal(Literal::Int(42))));
        assert!(is_int_literal(&HirExpr::Literal(Literal::Int(0))));
        assert!(is_int_literal(&HirExpr::Literal(Literal::Int(-100))));
    }

    #[test]
    fn test_is_int_literal_false() {
        assert!(!is_int_literal(&HirExpr::Literal(Literal::Float(3.14))));
        assert!(!is_int_literal(&HirExpr::Literal(Literal::String("42".to_string()))));
        assert!(!is_int_literal(&HirExpr::Var("x".to_string())));
    }

    // ============================================================================
    // is_float_literal tests
    // ============================================================================

    #[test]
    fn test_is_float_literal_true() {
        assert!(is_float_literal(&HirExpr::Literal(Literal::Float(3.14))));
        assert!(is_float_literal(&HirExpr::Literal(Literal::Float(0.0))));
        assert!(is_float_literal(&HirExpr::Literal(Literal::Float(-1.5))));
    }

    #[test]
    fn test_is_float_literal_false() {
        assert!(!is_float_literal(&HirExpr::Literal(Literal::Int(42))));
        assert!(!is_float_literal(&HirExpr::Var("x".to_string())));
    }

    // ============================================================================
    // is_numeric_literal tests
    // ============================================================================

    #[test]
    fn test_is_numeric_literal_int() {
        assert!(is_numeric_literal(&HirExpr::Literal(Literal::Int(42))));
    }

    #[test]
    fn test_is_numeric_literal_float() {
        assert!(is_numeric_literal(&HirExpr::Literal(Literal::Float(3.14))));
    }

    #[test]
    fn test_is_numeric_literal_false() {
        assert!(!is_numeric_literal(&HirExpr::Literal(Literal::String("42".to_string()))));
        assert!(!is_numeric_literal(&HirExpr::Var("x".to_string())));
    }

    // ============================================================================
    // Type checking tests
    // ============================================================================

    #[test]
    fn test_is_numeric_type() {
        assert!(is_numeric_type(&Type::Int));
        assert!(is_numeric_type(&Type::Float));
        assert!(!is_numeric_type(&Type::String));
        assert!(!is_numeric_type(&Type::Bool));
    }

    #[test]
    fn test_is_int_type() {
        assert!(is_int_type(&Type::Int));
        assert!(!is_int_type(&Type::Float));
        assert!(!is_int_type(&Type::String));
    }

    #[test]
    fn test_is_float_type() {
        assert!(is_float_type(&Type::Float));
        assert!(!is_float_type(&Type::Int));
        assert!(!is_float_type(&Type::String));
    }

    #[test]
    fn test_is_string_type() {
        assert!(is_string_type(&Type::String));
        assert!(!is_string_type(&Type::Int));
    }

    #[test]
    fn test_is_bool_type() {
        assert!(is_bool_type(&Type::Bool));
        assert!(!is_bool_type(&Type::Int));
    }

    #[test]
    fn test_is_container_type() {
        assert!(is_container_type(&Type::List(Box::new(Type::Int))));
        assert!(is_container_type(&Type::Dict(Box::new(Type::String), Box::new(Type::Int))));
        assert!(is_container_type(&Type::Set(Box::new(Type::Int))));
        assert!(is_container_type(&Type::Tuple(vec![Type::Int, Type::String])));
        assert!(!is_container_type(&Type::Int));
        assert!(!is_container_type(&Type::String));
    }

    #[test]
    fn test_is_optional_type() {
        assert!(is_optional_type(&Type::Optional(Box::new(Type::Int))));
        assert!(!is_optional_type(&Type::Int));
    }

    // ============================================================================
    // unwrap_* tests
    // ============================================================================

    #[test]
    fn test_unwrap_optional() {
        let opt = Type::Optional(Box::new(Type::Int));
        assert_eq!(unwrap_optional(&opt), Some(&Type::Int));
        assert_eq!(unwrap_optional(&Type::Int), None);
    }

    #[test]
    fn test_unwrap_list() {
        let list = Type::List(Box::new(Type::String));
        assert_eq!(unwrap_list(&list), Some(&Type::String));
        assert_eq!(unwrap_list(&Type::Int), None);
    }

    #[test]
    fn test_unwrap_dict() {
        let dict = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        let (k, v) = unwrap_dict(&dict).unwrap();
        assert_eq!(k, &Type::String);
        assert_eq!(v, &Type::Int);
        assert_eq!(unwrap_dict(&Type::Int), None);
    }

    #[test]
    fn test_unwrap_set() {
        let set = Type::Set(Box::new(Type::Float));
        assert_eq!(unwrap_set(&set), Some(&Type::Float));
        assert_eq!(unwrap_set(&Type::Int), None);
    }

    // ============================================================================
    // Special literal tests
    // ============================================================================

    #[test]
    fn test_is_zero_literal_int() {
        assert!(is_zero_literal(&HirExpr::Literal(Literal::Int(0))));
        assert!(!is_zero_literal(&HirExpr::Literal(Literal::Int(1))));
    }

    #[test]
    fn test_is_zero_literal_float() {
        assert!(is_zero_literal(&HirExpr::Literal(Literal::Float(0.0))));
        assert!(!is_zero_literal(&HirExpr::Literal(Literal::Float(0.1))));
    }

    #[test]
    fn test_is_one_literal_int() {
        assert!(is_one_literal(&HirExpr::Literal(Literal::Int(1))));
        assert!(!is_one_literal(&HirExpr::Literal(Literal::Int(0))));
    }

    #[test]
    fn test_is_one_literal_float() {
        assert!(is_one_literal(&HirExpr::Literal(Literal::Float(1.0))));
        assert!(!is_one_literal(&HirExpr::Literal(Literal::Float(0.5))));
    }

    #[test]
    fn test_is_negative_int_literal() {
        assert!(is_negative_int_literal(&HirExpr::Literal(Literal::Int(-1))));
        assert!(is_negative_int_literal(&HirExpr::Literal(Literal::Int(-100))));
        assert!(!is_negative_int_literal(&HirExpr::Literal(Literal::Int(0))));
        assert!(!is_negative_int_literal(&HirExpr::Literal(Literal::Int(1))));
    }

    #[test]
    fn test_is_positive_int_literal() {
        assert!(is_positive_int_literal(&HirExpr::Literal(Literal::Int(1))));
        assert!(is_positive_int_literal(&HirExpr::Literal(Literal::Int(100))));
        assert!(!is_positive_int_literal(&HirExpr::Literal(Literal::Int(0))));
        assert!(!is_positive_int_literal(&HirExpr::Literal(Literal::Int(-1))));
    }

    // ============================================================================
    // get_* value tests
    // ============================================================================

    #[test]
    fn test_get_int_value() {
        assert_eq!(get_int_value(&HirExpr::Literal(Literal::Int(42))), Some(42));
        assert_eq!(get_int_value(&HirExpr::Literal(Literal::Int(-10))), Some(-10));
        assert_eq!(get_int_value(&HirExpr::Var("x".to_string())), None);
    }

    #[test]
    fn test_get_float_value() {
        assert_eq!(get_float_value(&HirExpr::Literal(Literal::Float(3.14))), Some(3.14));
        assert_eq!(get_float_value(&HirExpr::Var("x".to_string())), None);
    }

    #[test]
    fn test_get_string_value() {
        assert_eq!(get_string_value(&HirExpr::Literal(Literal::String("hello".to_string()))), Some("hello"));
        assert_eq!(get_string_value(&HirExpr::Var("x".to_string())), None);
    }

    #[test]
    fn test_get_bool_value() {
        assert_eq!(get_bool_value(&HirExpr::Literal(Literal::Bool(true))), Some(true));
        assert_eq!(get_bool_value(&HirExpr::Literal(Literal::Bool(false))), Some(false));
        assert_eq!(get_bool_value(&HirExpr::Var("x".to_string())), None);
    }

    // ============================================================================
    // Cast expression tests
    // ============================================================================

    #[test]
    fn test_cast_to_i32() {
        use quote::ToTokens;
        let expr: syn::Expr = syn::parse_quote! { x };
        let result = cast_to_i32(expr);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("i32"));
    }

    #[test]
    fn test_cast_to_i64() {
        use quote::ToTokens;
        let expr: syn::Expr = syn::parse_quote! { x };
        let result = cast_to_i64(expr);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("i64"));
    }

    #[test]
    fn test_cast_to_f32() {
        use quote::ToTokens;
        let expr: syn::Expr = syn::parse_quote! { x };
        let result = cast_to_f32(expr);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("f32"));
    }

    #[test]
    fn test_cast_to_f64() {
        use quote::ToTokens;
        let expr: syn::Expr = syn::parse_quote! { x };
        let result = cast_to_f64(expr);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("f64"));
    }

    #[test]
    fn test_cast_to_usize() {
        use quote::ToTokens;
        let expr: syn::Expr = syn::parse_quote! { x };
        let result = cast_to_usize(expr);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("usize"));
    }

    #[test]
    fn test_cast_to_isize() {
        use quote::ToTokens;
        let expr: syn::Expr = syn::parse_quote! { x };
        let result = cast_to_isize(expr);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("isize"));
    }

    // ============================================================================
    // needs_cast tests
    // ============================================================================

    #[test]
    fn test_needs_cast_same_types() {
        assert!(!needs_cast(&Type::Int, &Type::Int));
        assert!(!needs_cast(&Type::Float, &Type::Float));
        assert!(!needs_cast(&Type::String, &Type::String));
        assert!(!needs_cast(&Type::Bool, &Type::Bool));
    }

    #[test]
    fn test_needs_cast_numeric() {
        assert!(needs_cast(&Type::Int, &Type::Float));
        assert!(needs_cast(&Type::Float, &Type::Int));
    }

    #[test]
    fn test_needs_cast_bool_int() {
        assert!(needs_cast(&Type::Bool, &Type::Int));
        assert!(needs_cast(&Type::Int, &Type::Bool));
    }

    #[test]
    fn test_needs_cast_string() {
        assert!(needs_cast(&Type::String, &Type::Int));
        assert!(needs_cast(&Type::String, &Type::Float));
        assert!(needs_cast(&Type::Int, &Type::String));
        assert!(needs_cast(&Type::Float, &Type::String));
    }

    #[test]
    fn test_needs_cast_different_types() {
        assert!(needs_cast(&Type::Int, &Type::String));
        assert!(needs_cast(&Type::List(Box::new(Type::Int)), &Type::Int));
    }

    // ============================================================================
    // is_arithmetic_type tests
    // ============================================================================

    #[test]
    fn test_is_arithmetic_type() {
        assert!(is_arithmetic_type(&Type::Int));
        assert!(is_arithmetic_type(&Type::Float));
        assert!(!is_arithmetic_type(&Type::String));
        assert!(!is_arithmetic_type(&Type::Bool));
        assert!(!is_arithmetic_type(&Type::List(Box::new(Type::Int))));
    }

    // ============================================================================
    // is_comparable_type tests
    // ============================================================================

    #[test]
    fn test_is_comparable_type() {
        assert!(is_comparable_type(&Type::Int));
        assert!(is_comparable_type(&Type::Float));
        assert!(is_comparable_type(&Type::String));
        assert!(is_comparable_type(&Type::Bool));
        assert!(!is_comparable_type(&Type::List(Box::new(Type::Int))));
    }

    // ============================================================================
    // supports_containment tests
    // ============================================================================

    #[test]
    fn test_supports_containment() {
        assert!(supports_containment(&Type::List(Box::new(Type::Int))));
        assert!(supports_containment(&Type::Set(Box::new(Type::Int))));
        assert!(supports_containment(&Type::Dict(Box::new(Type::String), Box::new(Type::Int))));
        assert!(supports_containment(&Type::String));
        assert!(supports_containment(&Type::Tuple(vec![Type::Int])));
        assert!(!supports_containment(&Type::Int));
        assert!(!supports_containment(&Type::Float));
        assert!(!supports_containment(&Type::Bool));
    }
}
