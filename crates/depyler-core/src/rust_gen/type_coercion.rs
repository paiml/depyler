//! Type coercion utilities for code generation
//!
//! This module provides helper functions for type coercion and conversion
//! in Python-to-Rust transpilation.

use crate::hir::{BinOp, HirExpr, Literal, Type};
use std::collections::HashMap;
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
    matches!(ty, Type::Int | Type::Float | Type::String | Type::Bool)
}

/// Check if a type supports the 'in' operator
pub fn supports_containment(ty: &Type) -> bool {
    matches!(
        ty,
        Type::List(_) | Type::Set(_) | Type::Dict(_, _) | Type::String | Type::Tuple(_)
    )
}

// ============================================================================
// DEPYLER-COVERAGE-95: Extended type coercion helpers from expr_gen.rs
// ============================================================================

/// Check if Type represents f32 specifically (for trueno/numpy compatibility)
pub fn is_f32_type(ty: &Type) -> bool {
    matches!(ty, Type::Custom(s) if s == "f32")
}

/// Check if Type represents any integer variant including custom Rust types
pub fn is_int_type_extended(ty: &Type) -> bool {
    match ty {
        Type::Int => true,
        Type::Custom(s) => matches!(
            s.as_str(),
            "i32" | "i64" | "i128" | "isize" | "u32" | "u64" | "u128" | "usize"
        ),
        _ => false,
    }
}

/// Check if Type represents any float variant including custom Rust types
pub fn is_float_type_extended(ty: &Type) -> bool {
    match ty {
        Type::Float => true,
        Type::Custom(s) => matches!(s.as_str(), "f64" | "f32"),
        _ => false,
    }
}

/// Heuristic: Check if variable name suggests float type
/// Matches common ML parameter names and color channel variables
pub fn is_float_var_name(name: &str) -> bool {
    let name_lower = name.to_lowercase();

    // ML hyperparameter names
    if name_lower.contains("beta")
        || name_lower.contains("alpha")
        || name_lower.contains("lr")
        || name_lower.contains("eps")
        || name_lower.contains("rate")
        || name_lower.contains("momentum")
    {
        return true;
    }

    // DEPYLER-0950: Color channel variables (r, g, h, s, v, l, c, m, k)
    // Single-letter names from colorsys.hsv_to_rgb(), rgb_to_hsv(), etc.
    // DEPYLER-0954: Exclude a, b, x, y (too generic, causes false positives)
    matches!(name, "r" | "g" | "h" | "s" | "v" | "l" | "c" | "m" | "k")
}

/// Check if an HirExpr is a pure integer expression (recursive)
/// Handles variables, literals, and binary operations on integers
pub fn is_int_expr_recursive(expr: &HirExpr, var_types: &HashMap<String, Type>) -> bool {
    match expr {
        HirExpr::Var(name) => var_types
            .get(name)
            .map(is_int_type_extended)
            .unwrap_or(false),
        HirExpr::Literal(Literal::Int(_)) => true,
        // Binary operations on integers produce integers (except division)
        HirExpr::Binary { left, right, op } => {
            // Add, Sub, Mul, Mod, FloorDiv produce Int if both operands are Int
            // Division always produces Float in Python
            if matches!(
                op,
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Mod | BinOp::FloorDiv
            ) {
                is_int_expr_recursive(left, var_types) && is_int_expr_recursive(right, var_types)
            } else {
                false
            }
        }
        // Unary minus on integer is still integer
        HirExpr::Unary { operand, .. } => is_int_expr_recursive(operand, var_types),
        _ => false,
    }
}

/// Coerce an integer literal to a float literal expression (f64)
pub fn coerce_int_literal_to_f64(val: i64) -> syn::Expr {
    let float_val = val as f64;
    parse_quote! { #float_val }
}

/// Coerce an integer literal to an f32 literal expression
pub fn coerce_int_literal_to_f32(val: i64) -> syn::Expr {
    let float_val = val as f32;
    parse_quote! { #float_val }
}

/// Check if expression is a comparison operator
pub fn is_comparison_op(op: &BinOp) -> bool {
    matches!(
        op,
        BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq | BinOp::Eq | BinOp::NotEq
    )
}

/// Check if expression is an ordering comparison (excludes equality)
pub fn is_ordering_comparison(op: &BinOp) -> bool {
    matches!(op, BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq)
}

/// Check if operator is a logical operator (and/or)
pub fn is_logical_op(op: &BinOp) -> bool {
    matches!(op, BinOp::And | BinOp::Or)
}

/// Check if operator is a bitwise operator
pub fn is_bitwise_op(op: &BinOp) -> bool {
    matches!(
        op,
        BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::LShift | BinOp::RShift
    )
}

/// Check if operator is an arithmetic operator
pub fn is_arithmetic_op(op: &BinOp) -> bool {
    matches!(
        op,
        BinOp::Add
            | BinOp::Sub
            | BinOp::Mul
            | BinOp::Div
            | BinOp::Mod
            | BinOp::FloorDiv
            | BinOp::Pow
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
        assert!(!is_int_literal(&HirExpr::Literal(Literal::Float(3.15))));
        assert!(!is_int_literal(&HirExpr::Literal(Literal::String(
            "42".to_string()
        ))));
        assert!(!is_int_literal(&HirExpr::Var("x".to_string())));
    }

    // ============================================================================
    // is_float_literal tests
    // ============================================================================

    #[test]
    fn test_is_float_literal_true() {
        assert!(is_float_literal(&HirExpr::Literal(Literal::Float(3.15))));
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
        assert!(is_numeric_literal(&HirExpr::Literal(Literal::Float(3.15))));
    }

    #[test]
    fn test_is_numeric_literal_false() {
        assert!(!is_numeric_literal(&HirExpr::Literal(Literal::String(
            "42".to_string()
        ))));
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
        assert!(is_container_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
        assert!(is_container_type(&Type::Set(Box::new(Type::Int))));
        assert!(is_container_type(&Type::Tuple(vec![
            Type::Int,
            Type::String
        ])));
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
        assert!(is_negative_int_literal(&HirExpr::Literal(Literal::Int(
            -100
        ))));
        assert!(!is_negative_int_literal(&HirExpr::Literal(Literal::Int(0))));
        assert!(!is_negative_int_literal(&HirExpr::Literal(Literal::Int(1))));
    }

    #[test]
    fn test_is_positive_int_literal() {
        assert!(is_positive_int_literal(&HirExpr::Literal(Literal::Int(1))));
        assert!(is_positive_int_literal(&HirExpr::Literal(Literal::Int(
            100
        ))));
        assert!(!is_positive_int_literal(&HirExpr::Literal(Literal::Int(0))));
        assert!(!is_positive_int_literal(&HirExpr::Literal(Literal::Int(
            -1
        ))));
    }

    // ============================================================================
    // get_* value tests
    // ============================================================================

    #[test]
    fn test_get_int_value() {
        assert_eq!(get_int_value(&HirExpr::Literal(Literal::Int(42))), Some(42));
        assert_eq!(
            get_int_value(&HirExpr::Literal(Literal::Int(-10))),
            Some(-10)
        );
        assert_eq!(get_int_value(&HirExpr::Var("x".to_string())), None);
    }

    #[test]
    fn test_get_float_value() {
        assert_eq!(
            get_float_value(&HirExpr::Literal(Literal::Float(3.15))),
            Some(3.15)
        );
        assert_eq!(get_float_value(&HirExpr::Var("x".to_string())), None);
    }

    #[test]
    fn test_get_string_value() {
        assert_eq!(
            get_string_value(&HirExpr::Literal(Literal::String("hello".to_string()))),
            Some("hello")
        );
        assert_eq!(get_string_value(&HirExpr::Var("x".to_string())), None);
    }

    #[test]
    fn test_get_bool_value() {
        assert_eq!(
            get_bool_value(&HirExpr::Literal(Literal::Bool(true))),
            Some(true)
        );
        assert_eq!(
            get_bool_value(&HirExpr::Literal(Literal::Bool(false))),
            Some(false)
        );
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
        assert!(supports_containment(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
        assert!(supports_containment(&Type::String));
        assert!(supports_containment(&Type::Tuple(vec![Type::Int])));
        assert!(!supports_containment(&Type::Int));
        assert!(!supports_containment(&Type::Float));
        assert!(!supports_containment(&Type::Bool));
    }

    // ============================================================================
    // DEPYLER-COVERAGE-95: Extended type coercion helper tests
    // ============================================================================

    #[test]
    fn test_is_f32_type_f32() {
        assert!(is_f32_type(&Type::Custom("f32".to_string())));
    }

    #[test]
    fn test_is_f32_type_f64() {
        assert!(!is_f32_type(&Type::Custom("f64".to_string())));
    }

    #[test]
    fn test_is_f32_type_float() {
        assert!(!is_f32_type(&Type::Float));
    }

    #[test]
    fn test_is_f32_type_int() {
        assert!(!is_f32_type(&Type::Int));
    }

    #[test]
    fn test_is_int_type_extended_int() {
        assert!(is_int_type_extended(&Type::Int));
    }

    #[test]
    fn test_is_int_type_extended_i32() {
        assert!(is_int_type_extended(&Type::Custom("i32".to_string())));
    }

    #[test]
    fn test_is_int_type_extended_i64() {
        assert!(is_int_type_extended(&Type::Custom("i64".to_string())));
    }

    #[test]
    fn test_is_int_type_extended_i128() {
        assert!(is_int_type_extended(&Type::Custom("i128".to_string())));
    }

    #[test]
    fn test_is_int_type_extended_isize() {
        assert!(is_int_type_extended(&Type::Custom("isize".to_string())));
    }

    #[test]
    fn test_is_int_type_extended_u32() {
        assert!(is_int_type_extended(&Type::Custom("u32".to_string())));
    }

    #[test]
    fn test_is_int_type_extended_u64() {
        assert!(is_int_type_extended(&Type::Custom("u64".to_string())));
    }

    #[test]
    fn test_is_int_type_extended_u128() {
        assert!(is_int_type_extended(&Type::Custom("u128".to_string())));
    }

    #[test]
    fn test_is_int_type_extended_usize() {
        assert!(is_int_type_extended(&Type::Custom("usize".to_string())));
    }

    #[test]
    fn test_is_int_type_extended_float() {
        assert!(!is_int_type_extended(&Type::Float));
    }

    #[test]
    fn test_is_int_type_extended_string() {
        assert!(!is_int_type_extended(&Type::String));
    }

    #[test]
    fn test_is_int_type_extended_custom_other() {
        assert!(!is_int_type_extended(&Type::Custom("MyType".to_string())));
    }

    #[test]
    fn test_is_float_type_extended_float() {
        assert!(is_float_type_extended(&Type::Float));
    }

    #[test]
    fn test_is_float_type_extended_f64() {
        assert!(is_float_type_extended(&Type::Custom("f64".to_string())));
    }

    #[test]
    fn test_is_float_type_extended_f32() {
        assert!(is_float_type_extended(&Type::Custom("f32".to_string())));
    }

    #[test]
    fn test_is_float_type_extended_int() {
        assert!(!is_float_type_extended(&Type::Int));
    }

    #[test]
    fn test_is_float_type_extended_string() {
        assert!(!is_float_type_extended(&Type::String));
    }

    #[test]
    fn test_is_float_var_name_beta() {
        assert!(is_float_var_name("beta"));
        assert!(is_float_var_name("beta1"));
        assert!(is_float_var_name("BETA"));
    }

    #[test]
    fn test_is_float_var_name_alpha() {
        assert!(is_float_var_name("alpha"));
        assert!(is_float_var_name("alpha_decay"));
    }

    #[test]
    fn test_is_float_var_name_lr() {
        assert!(is_float_var_name("lr"));
        assert!(is_float_var_name("learning_lr"));
    }

    #[test]
    fn test_is_float_var_name_eps() {
        assert!(is_float_var_name("eps"));
        assert!(is_float_var_name("epsilon"));
    }

    #[test]
    fn test_is_float_var_name_rate() {
        assert!(is_float_var_name("rate"));
        assert!(is_float_var_name("learning_rate"));
    }

    #[test]
    fn test_is_float_var_name_momentum() {
        assert!(is_float_var_name("momentum"));
        assert!(is_float_var_name("MOMENTUM"));
    }

    #[test]
    fn test_is_float_var_name_color_channels() {
        assert!(is_float_var_name("r"));
        assert!(is_float_var_name("g"));
        assert!(is_float_var_name("h"));
        assert!(is_float_var_name("s"));
        assert!(is_float_var_name("v"));
        assert!(is_float_var_name("l"));
        assert!(is_float_var_name("c"));
        assert!(is_float_var_name("m"));
        assert!(is_float_var_name("k"));
    }

    #[test]
    fn test_is_float_var_name_excluded_generic() {
        // These are too generic and excluded per DEPYLER-0954
        assert!(!is_float_var_name("a"));
        assert!(!is_float_var_name("b"));
        assert!(!is_float_var_name("x"));
        assert!(!is_float_var_name("y"));
    }

    #[test]
    fn test_is_float_var_name_regular_vars() {
        assert!(!is_float_var_name("count"));
        assert!(!is_float_var_name("index"));
        assert!(!is_float_var_name("name"));
        assert!(!is_float_var_name("items"));
    }

    #[test]
    fn test_is_int_expr_recursive_int_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        let var_types = HashMap::new();
        assert!(is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_float_literal() {
        let expr = HirExpr::Literal(Literal::Float(3.15));
        let var_types = HashMap::new();
        assert!(!is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_int_var() {
        let expr = HirExpr::Var("count".to_string());
        let mut var_types = HashMap::new();
        var_types.insert("count".to_string(), Type::Int);
        assert!(is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_float_var() {
        let expr = HirExpr::Var("rate".to_string());
        let mut var_types = HashMap::new();
        var_types.insert("rate".to_string(), Type::Float);
        assert!(!is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_unknown_var() {
        let expr = HirExpr::Var("unknown".to_string());
        let var_types = HashMap::new();
        assert!(!is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_binary_add_ints() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let var_types = HashMap::new();
        assert!(is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_binary_sub_ints() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(5))),
            op: BinOp::Sub,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let var_types = HashMap::new();
        assert!(is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_binary_mul_ints() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(2))),
            op: BinOp::Mul,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let var_types = HashMap::new();
        assert!(is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_binary_mod_ints() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            op: BinOp::Mod,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let var_types = HashMap::new();
        assert!(is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_binary_floordiv_ints() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            op: BinOp::FloorDiv,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let var_types = HashMap::new();
        assert!(is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_binary_div_ints_not_int() {
        // Division always returns float in Python
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            op: BinOp::Div,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let var_types = HashMap::new();
        assert!(!is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_binary_add_mixed() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Float(2.0))),
        };
        let var_types = HashMap::new();
        assert!(!is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_unary_minus_int() {
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let var_types = HashMap::new();
        assert!(is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_unary_minus_float() {
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Float(5.0))),
        };
        let var_types = HashMap::new();
        assert!(!is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_nested_binary() {
        // (1 + 2) * 3
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Literal(Literal::Int(1))),
                op: BinOp::Add,
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
            op: BinOp::Mul,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let var_types = HashMap::new();
        assert!(is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_string_literal() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        let var_types = HashMap::new();
        assert!(!is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_recursive_bool_literal() {
        let expr = HirExpr::Literal(Literal::Bool(true));
        let var_types = HashMap::new();
        assert!(!is_int_expr_recursive(&expr, &var_types));
    }

    #[test]
    fn test_coerce_int_literal_to_f64_zero() {
        let expr = coerce_int_literal_to_f64(0);
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("0"));
    }

    #[test]
    fn test_coerce_int_literal_to_f64_positive() {
        let expr = coerce_int_literal_to_f64(42);
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("42"));
    }

    #[test]
    fn test_coerce_int_literal_to_f64_negative() {
        let expr = coerce_int_literal_to_f64(-10);
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("10"));
    }

    #[test]
    fn test_coerce_int_literal_to_f32_zero() {
        let expr = coerce_int_literal_to_f32(0);
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("0"));
    }

    #[test]
    fn test_coerce_int_literal_to_f32_positive() {
        let expr = coerce_int_literal_to_f32(100);
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("100"));
    }

    #[test]
    fn test_is_comparison_op_lt() {
        assert!(is_comparison_op(&BinOp::Lt));
    }

    #[test]
    fn test_is_comparison_op_lteq() {
        assert!(is_comparison_op(&BinOp::LtEq));
    }

    #[test]
    fn test_is_comparison_op_gt() {
        assert!(is_comparison_op(&BinOp::Gt));
    }

    #[test]
    fn test_is_comparison_op_gteq() {
        assert!(is_comparison_op(&BinOp::GtEq));
    }

    #[test]
    fn test_is_comparison_op_eq() {
        assert!(is_comparison_op(&BinOp::Eq));
    }

    #[test]
    fn test_is_comparison_op_noteq() {
        assert!(is_comparison_op(&BinOp::NotEq));
    }

    #[test]
    fn test_is_comparison_op_add() {
        assert!(!is_comparison_op(&BinOp::Add));
    }

    #[test]
    fn test_is_comparison_op_sub() {
        assert!(!is_comparison_op(&BinOp::Sub));
    }

    #[test]
    fn test_is_comparison_op_mul() {
        assert!(!is_comparison_op(&BinOp::Mul));
    }

    #[test]
    fn test_is_comparison_op_and() {
        assert!(!is_comparison_op(&BinOp::And));
    }

    #[test]
    fn test_is_ordering_comparison_lt() {
        assert!(is_ordering_comparison(&BinOp::Lt));
    }

    #[test]
    fn test_is_ordering_comparison_lteq() {
        assert!(is_ordering_comparison(&BinOp::LtEq));
    }

    #[test]
    fn test_is_ordering_comparison_gt() {
        assert!(is_ordering_comparison(&BinOp::Gt));
    }

    #[test]
    fn test_is_ordering_comparison_gteq() {
        assert!(is_ordering_comparison(&BinOp::GtEq));
    }

    #[test]
    fn test_is_ordering_comparison_eq() {
        // Equality is NOT an ordering comparison
        assert!(!is_ordering_comparison(&BinOp::Eq));
    }

    #[test]
    fn test_is_ordering_comparison_noteq() {
        // Not-equal is NOT an ordering comparison
        assert!(!is_ordering_comparison(&BinOp::NotEq));
    }

    #[test]
    fn test_is_logical_op_and() {
        assert!(is_logical_op(&BinOp::And));
    }

    #[test]
    fn test_is_logical_op_or() {
        assert!(is_logical_op(&BinOp::Or));
    }

    #[test]
    fn test_is_logical_op_add() {
        assert!(!is_logical_op(&BinOp::Add));
    }

    #[test]
    fn test_is_logical_op_lt() {
        assert!(!is_logical_op(&BinOp::Lt));
    }

    #[test]
    fn test_is_bitwise_op_bitand() {
        assert!(is_bitwise_op(&BinOp::BitAnd));
    }

    #[test]
    fn test_is_bitwise_op_bitor() {
        assert!(is_bitwise_op(&BinOp::BitOr));
    }

    #[test]
    fn test_is_bitwise_op_bitxor() {
        assert!(is_bitwise_op(&BinOp::BitXor));
    }

    #[test]
    fn test_is_bitwise_op_lshift() {
        assert!(is_bitwise_op(&BinOp::LShift));
    }

    #[test]
    fn test_is_bitwise_op_rshift() {
        assert!(is_bitwise_op(&BinOp::RShift));
    }

    #[test]
    fn test_is_bitwise_op_add() {
        assert!(!is_bitwise_op(&BinOp::Add));
    }

    #[test]
    fn test_is_arithmetic_op_add() {
        assert!(is_arithmetic_op(&BinOp::Add));
    }

    #[test]
    fn test_is_arithmetic_op_sub() {
        assert!(is_arithmetic_op(&BinOp::Sub));
    }

    #[test]
    fn test_is_arithmetic_op_mul() {
        assert!(is_arithmetic_op(&BinOp::Mul));
    }

    #[test]
    fn test_is_arithmetic_op_div() {
        assert!(is_arithmetic_op(&BinOp::Div));
    }

    #[test]
    fn test_is_arithmetic_op_mod() {
        assert!(is_arithmetic_op(&BinOp::Mod));
    }

    #[test]
    fn test_is_arithmetic_op_floordiv() {
        assert!(is_arithmetic_op(&BinOp::FloorDiv));
    }

    #[test]
    fn test_is_arithmetic_op_pow() {
        assert!(is_arithmetic_op(&BinOp::Pow));
    }

    #[test]
    fn test_is_arithmetic_op_and() {
        assert!(!is_arithmetic_op(&BinOp::And));
    }

    #[test]
    fn test_is_arithmetic_op_eq() {
        assert!(!is_arithmetic_op(&BinOp::Eq));
    }
}
