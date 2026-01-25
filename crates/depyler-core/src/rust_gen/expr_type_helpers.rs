//! Expression Type Helpers
//!
//! DEPYLER-COVERAGE-95: Extracted pure type-checking helpers from expr_gen.rs
//! for improved testability and coverage.
//!
//! These functions analyze HIR expressions and types to determine:
//! - Integer vs float type checking
//! - Borrowing requirements
//! - Type coercion needs

use crate::hir::{BinOp, HirExpr, Literal, Type};
use std::collections::HashMap;
use syn::parse_quote;

/// Check if expression evaluates to an integer type.
/// Handles variables, literals, and binary operations on integers.
///
/// DEPYLER-0805: Extracted from ExpressionConverter::is_int_expr
pub fn is_int_expr(expr: &HirExpr, var_types: &HashMap<String, Type>) -> bool {
    match expr {
        HirExpr::Var(name) => {
            if let Some(var_type) = var_types.get(name) {
                matches!(var_type, Type::Int)
            } else {
                false
            }
        }
        HirExpr::Literal(Literal::Int(_)) => true,
        // Binary operations on integers produce integers
        // (Add, Sub, Mul produce Int if both operands are Int)
        // Division in Python returns Float, so we don't include Div
        HirExpr::Binary { left, right, op } => {
            if matches!(
                op,
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Mod | BinOp::FloorDiv
            ) {
                is_int_expr(left, var_types) && is_int_expr(right, var_types)
            } else {
                false
            }
        }
        // Unary minus on integer is still integer
        HirExpr::Unary { operand, .. } => is_int_expr(operand, var_types),
        _ => false,
    }
}

/// Check if expression is a variable with integer type.
pub fn is_int_var(expr: &HirExpr, var_types: &HashMap<String, Type>) -> bool {
    if let HirExpr::Var(name) = expr {
        if let Some(var_type) = var_types.get(name) {
            if matches!(var_type, Type::Int) {
                return true;
            }
            if let Type::Custom(s) = var_type {
                if s == "i32" || s == "i64" || s == "usize" || s == "isize" {
                    return true;
                }
            }
        }
    }
    false
}

/// Check if expression is a variable with float type.
/// Also uses heuristics for common float parameter names.
///
/// DEPYLER-0950: Heuristic for colorsys color channel variables
pub fn is_float_var(expr: &HirExpr, var_types: &HashMap<String, Type>) -> bool {
    if let HirExpr::Var(name) = expr {
        if let Some(var_type) = var_types.get(name) {
            if matches!(var_type, Type::Float) {
                return true;
            }
            if let Type::Custom(s) = var_type {
                if s == "f64" || s == "f32" {
                    return true;
                }
            }
        }
        // Heuristic: common float parameter names
        if is_float_var_name(name) {
            return true;
        }
    }
    false
}

/// Check if variable name is likely a float parameter.
/// Uses heuristics for common float parameter names.
pub fn is_float_var_name(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    if name_lower.contains("beta")
        || name_lower.contains("alpha")
        || name_lower.contains("lr")
        || name_lower.contains("eps")
        || name_lower.contains("rate")
        || name_lower.contains("momentum")
    {
        return true;
    }
    // DEPYLER-0950: Heuristic for colorsys color channel variables
    // Single-letter color channel names like r, g, h, s, v, l are typically f64
    // DEPYLER-0954: Note: a, b, x, y are too generic and cause false positives
    matches!(name, "r" | "g" | "h" | "s" | "v" | "l" | "c" | "m" | "k")
}

/// Add & to borrow a path expression if it's a simple variable.
/// This prevents moving String parameters in PathBuf::from() and File::open()
///
/// DEPYLER-0465: Extracted from ExpressionConverter::borrow_if_needed
#[inline]
pub fn borrow_if_needed(expr: &syn::Expr) -> syn::Expr {
    match expr {
        // If it's a simple path (variable), add &
        syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
            parse_quote! { &#expr }
        }
        // Otherwise, use as-is (literals, method calls, etc.)
        _ => expr.clone(),
    }
}

/// Check if variable name indicates a string type.
/// Used for type inference heuristics.
pub fn is_string_var_name(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    name_lower.contains("name")
        || name_lower.contains("text")
        || name_lower.contains("msg")
        || name_lower.contains("message")
        || name_lower.contains("path")
        || name_lower.contains("file")
        || name_lower.contains("line")
        || name_lower.contains("word")
        || name_lower.contains("str")
        || name_lower.contains("string")
        || name_lower.ends_with("_s")
}

/// Check if type is an integer type.
pub fn is_int_type(ty: &Type) -> bool {
    match ty {
        Type::Int => true,
        Type::Custom(s) => matches!(
            s.as_str(),
            "i32" | "i64" | "isize" | "usize" | "u32" | "u64"
        ),
        _ => false,
    }
}

/// Check if type is a float type.
pub fn is_float_type(ty: &Type) -> bool {
    match ty {
        Type::Float => true,
        Type::Custom(s) => matches!(s.as_str(), "f64" | "f32"),
        _ => false,
    }
}

/// Check if type is a boolean type.
pub fn is_bool_type(ty: &Type) -> bool {
    matches!(ty, Type::Bool)
}

/// Check if type is a string type.
pub fn is_string_type(ty: &Type) -> bool {
    match ty {
        Type::String => true,
        Type::Custom(s) => s == "String" || s == "&str",
        _ => false,
    }
}

/// Check if binary operator is a comparison operator.
pub fn is_comparison_op(op: &BinOp) -> bool {
    matches!(
        op,
        BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq
    )
}

/// Check if binary operator is an ordering comparison (not equality).
pub fn is_ordering_comparison(op: &BinOp) -> bool {
    matches!(op, BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq)
}

/// Check if binary operator is a logical operator.
pub fn is_logical_op(op: &BinOp) -> bool {
    matches!(op, BinOp::And | BinOp::Or)
}

/// Check if binary operator is a bitwise operator.
pub fn is_bitwise_op(op: &BinOp) -> bool {
    matches!(
        op,
        BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::LShift | BinOp::RShift
    )
}

/// Check if binary operator is an arithmetic operator.
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

/// Check if binary operator produces an integer result (when both operands are int).
pub fn op_preserves_int(op: &BinOp) -> bool {
    matches!(
        op,
        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Mod | BinOp::FloorDiv
    )
}

/// Check if the expression is a literal of the specified type.
pub fn is_literal_type(expr: &HirExpr, expected: &str) -> bool {
    match expr {
        HirExpr::Literal(lit) => matches!(
            (lit, expected),
            (Literal::Int(_), "int")
                | (Literal::Float(_), "float")
                | (Literal::String(_), "string")
                | (Literal::Bool(_), "bool")
                | (Literal::None, "none")
                | (Literal::Bytes(_), "bytes")
        ),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    // =========================================================================
    // Tests for is_int_expr
    // =========================================================================

    #[test]
    fn test_is_int_expr_literal() {
        let var_types = HashMap::new();
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_float_literal() {
        let var_types = HashMap::new();
        let expr = HirExpr::Literal(Literal::Float(3.15));
        assert!(!is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_var_int_type() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Int);
        let expr = HirExpr::Var("x".to_string());
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_var_float_type() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Float);
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_var_unknown() {
        let var_types = HashMap::new();
        let expr = HirExpr::Var("unknown".to_string());
        assert!(!is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_add_ints() {
        let mut var_types = HashMap::new();
        var_types.insert("a".to_string(), Type::Int);
        var_types.insert("b".to_string(), Type::Int);
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_add_int_float() {
        let mut var_types = HashMap::new();
        var_types.insert("a".to_string(), Type::Int);
        var_types.insert("b".to_string(), Type::Float);
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        assert!(!is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_sub_ints() {
        let var_types = HashMap::new();
        let expr = HirExpr::Binary {
            op: BinOp::Sub,
            left: Box::new(HirExpr::Literal(Literal::Int(5))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_mul_ints() {
        let var_types = HashMap::new();
        let expr = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Literal(Literal::Int(2))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_mod() {
        let var_types = HashMap::new();
        let expr = HirExpr::Binary {
            op: BinOp::Mod,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_floordiv() {
        let var_types = HashMap::new();
        let expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_div_not_int() {
        let var_types = HashMap::new();
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(!is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_unary_neg() {
        let var_types = HashMap::new();
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_string_literal() {
        let var_types = HashMap::new();
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert!(!is_int_expr(&expr, &var_types));
    }

    // =========================================================================
    // Tests for is_int_var
    // =========================================================================

    #[test]
    fn test_is_int_var_int_type() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Int);
        let expr = HirExpr::Var("x".to_string());
        assert!(is_int_var(&expr, &var_types));
    }

    #[test]
    fn test_is_int_var_i32_custom() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Custom("i32".to_string()));
        let expr = HirExpr::Var("x".to_string());
        assert!(is_int_var(&expr, &var_types));
    }

    #[test]
    fn test_is_int_var_i64_custom() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Custom("i64".to_string()));
        let expr = HirExpr::Var("x".to_string());
        assert!(is_int_var(&expr, &var_types));
    }

    #[test]
    fn test_is_int_var_usize_custom() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Custom("usize".to_string()));
        let expr = HirExpr::Var("x".to_string());
        assert!(is_int_var(&expr, &var_types));
    }

    #[test]
    fn test_is_int_var_isize_custom() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Custom("isize".to_string()));
        let expr = HirExpr::Var("x".to_string());
        assert!(is_int_var(&expr, &var_types));
    }

    #[test]
    fn test_is_int_var_float_type() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Float);
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_int_var(&expr, &var_types));
    }

    #[test]
    fn test_is_int_var_not_var() {
        let var_types = HashMap::new();
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_int_var(&expr, &var_types));
    }

    // =========================================================================
    // Tests for is_float_var
    // =========================================================================

    #[test]
    fn test_is_float_var_float_type() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Float);
        let expr = HirExpr::Var("x".to_string());
        assert!(is_float_var(&expr, &var_types));
    }

    #[test]
    fn test_is_float_var_f64_custom() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Custom("f64".to_string()));
        let expr = HirExpr::Var("x".to_string());
        assert!(is_float_var(&expr, &var_types));
    }

    #[test]
    fn test_is_float_var_f32_custom() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Custom("f32".to_string()));
        let expr = HirExpr::Var("x".to_string());
        assert!(is_float_var(&expr, &var_types));
    }

    #[test]
    fn test_is_float_var_heuristic_beta() {
        let var_types = HashMap::new();
        let expr = HirExpr::Var("beta1".to_string());
        assert!(is_float_var(&expr, &var_types));
    }

    #[test]
    fn test_is_float_var_heuristic_alpha() {
        let var_types = HashMap::new();
        let expr = HirExpr::Var("alpha".to_string());
        assert!(is_float_var(&expr, &var_types));
    }

    #[test]
    fn test_is_float_var_heuristic_lr() {
        let var_types = HashMap::new();
        let expr = HirExpr::Var("lr".to_string());
        assert!(is_float_var(&expr, &var_types));
    }

    #[test]
    fn test_is_float_var_heuristic_momentum() {
        let var_types = HashMap::new();
        let expr = HirExpr::Var("momentum".to_string());
        assert!(is_float_var(&expr, &var_types));
    }

    #[test]
    fn test_is_float_var_heuristic_r_color() {
        let var_types = HashMap::new();
        let expr = HirExpr::Var("r".to_string());
        assert!(is_float_var(&expr, &var_types));
    }

    #[test]
    fn test_is_float_var_heuristic_h_color() {
        let var_types = HashMap::new();
        let expr = HirExpr::Var("h".to_string());
        assert!(is_float_var(&expr, &var_types));
    }

    #[test]
    fn test_is_float_var_not_float() {
        let mut var_types = HashMap::new();
        var_types.insert("x".to_string(), Type::Int);
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_float_var(&expr, &var_types));
    }

    // =========================================================================
    // Tests for is_float_var_name
    // =========================================================================

    #[test]
    fn test_is_float_var_name_beta() {
        assert!(is_float_var_name("beta1"));
        assert!(is_float_var_name("BETA"));
        assert!(is_float_var_name("beta_value"));
    }

    #[test]
    fn test_is_float_var_name_alpha() {
        assert!(is_float_var_name("alpha"));
        assert!(is_float_var_name("ALPHA"));
        assert!(is_float_var_name("alpha_decay"));
    }

    #[test]
    fn test_is_float_var_name_learning_rate() {
        assert!(is_float_var_name("lr"));
        assert!(is_float_var_name("learning_rate"));
        assert!(is_float_var_name("LR"));
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
    }

    #[test]
    fn test_is_float_var_name_not_float() {
        assert!(!is_float_var_name("x"));
        assert!(!is_float_var_name("count"));
        assert!(!is_float_var_name("index"));
        assert!(!is_float_var_name("a")); // DEPYLER-0954: too generic
        assert!(!is_float_var_name("b")); // DEPYLER-0954: too generic
    }

    // =========================================================================
    // Tests for borrow_if_needed
    // =========================================================================

    #[test]
    fn test_borrow_if_needed_simple_path() {
        let expr: syn::Expr = parse_quote! { x };
        let borrowed = borrow_if_needed(&expr);
        let tokens = borrowed.to_token_stream().to_string();
        assert!(tokens.contains("&"));
    }

    #[test]
    fn test_borrow_if_needed_literal() {
        let expr: syn::Expr = parse_quote! { "hello" };
        let result = borrow_if_needed(&expr);
        let tokens = result.to_token_stream().to_string();
        assert_eq!(tokens, "\"hello\"");
    }

    #[test]
    fn test_borrow_if_needed_method_call() {
        let expr: syn::Expr = parse_quote! { x.clone() };
        let result = borrow_if_needed(&expr);
        let tokens = result.to_token_stream().to_string();
        assert!(!tokens.starts_with("&"));
    }

    #[test]
    fn test_borrow_if_needed_qualified_path() {
        let expr: syn::Expr = parse_quote! { std::path::Path };
        let result = borrow_if_needed(&expr);
        // Qualified path should not be borrowed
        let tokens = result.to_token_stream().to_string();
        assert!(!tokens.starts_with("& "));
    }

    // =========================================================================
    // Tests for is_string_var_name
    // =========================================================================

    #[test]
    fn test_is_string_var_name_common_patterns() {
        assert!(is_string_var_name("name"));
        assert!(is_string_var_name("username"));
        assert!(is_string_var_name("text"));
        assert!(is_string_var_name("message"));
        assert!(is_string_var_name("path"));
        assert!(is_string_var_name("file_path"));
        assert!(is_string_var_name("filename"));
        assert!(is_string_var_name("line"));
        assert!(is_string_var_name("word"));
    }

    #[test]
    fn test_is_string_var_name_suffix() {
        assert!(is_string_var_name("result_s"));
        assert!(is_string_var_name("output_s"));
    }

    #[test]
    fn test_is_string_var_name_not_string() {
        assert!(!is_string_var_name("count"));
        assert!(!is_string_var_name("index"));
        assert!(!is_string_var_name("x"));
        assert!(!is_string_var_name("value"));
    }

    // =========================================================================
    // Tests for type checking helpers
    // =========================================================================

    #[test]
    fn test_is_int_type() {
        assert!(is_int_type(&Type::Int));
        assert!(is_int_type(&Type::Custom("i32".to_string())));
        assert!(is_int_type(&Type::Custom("i64".to_string())));
        assert!(is_int_type(&Type::Custom("usize".to_string())));
        assert!(!is_int_type(&Type::Float));
        assert!(!is_int_type(&Type::String));
    }

    #[test]
    fn test_is_float_type() {
        assert!(is_float_type(&Type::Float));
        assert!(is_float_type(&Type::Custom("f64".to_string())));
        assert!(is_float_type(&Type::Custom("f32".to_string())));
        assert!(!is_float_type(&Type::Int));
        assert!(!is_float_type(&Type::String));
    }

    #[test]
    fn test_is_bool_type() {
        assert!(is_bool_type(&Type::Bool));
        assert!(!is_bool_type(&Type::Int));
        assert!(!is_bool_type(&Type::String));
    }

    #[test]
    fn test_is_string_type() {
        assert!(is_string_type(&Type::String));
        assert!(is_string_type(&Type::Custom("String".to_string())));
        assert!(is_string_type(&Type::Custom("&str".to_string())));
        assert!(!is_string_type(&Type::Int));
        assert!(!is_string_type(&Type::Bool));
    }

    // =========================================================================
    // Tests for operator classification
    // =========================================================================

    #[test]
    fn test_is_comparison_op() {
        assert!(is_comparison_op(&BinOp::Eq));
        assert!(is_comparison_op(&BinOp::NotEq));
        assert!(is_comparison_op(&BinOp::Lt));
        assert!(is_comparison_op(&BinOp::LtEq));
        assert!(is_comparison_op(&BinOp::Gt));
        assert!(is_comparison_op(&BinOp::GtEq));
        assert!(!is_comparison_op(&BinOp::Add));
        assert!(!is_comparison_op(&BinOp::And));
    }

    #[test]
    fn test_is_ordering_comparison() {
        assert!(is_ordering_comparison(&BinOp::Lt));
        assert!(is_ordering_comparison(&BinOp::LtEq));
        assert!(is_ordering_comparison(&BinOp::Gt));
        assert!(is_ordering_comparison(&BinOp::GtEq));
        assert!(!is_ordering_comparison(&BinOp::Eq));
        assert!(!is_ordering_comparison(&BinOp::NotEq));
    }

    #[test]
    fn test_is_logical_op() {
        assert!(is_logical_op(&BinOp::And));
        assert!(is_logical_op(&BinOp::Or));
        assert!(!is_logical_op(&BinOp::BitAnd));
        assert!(!is_logical_op(&BinOp::Add));
    }

    #[test]
    fn test_is_bitwise_op() {
        assert!(is_bitwise_op(&BinOp::BitAnd));
        assert!(is_bitwise_op(&BinOp::BitOr));
        assert!(is_bitwise_op(&BinOp::BitXor));
        assert!(is_bitwise_op(&BinOp::LShift));
        assert!(is_bitwise_op(&BinOp::RShift));
        assert!(!is_bitwise_op(&BinOp::And));
        assert!(!is_bitwise_op(&BinOp::Add));
    }

    #[test]
    fn test_is_arithmetic_op() {
        assert!(is_arithmetic_op(&BinOp::Add));
        assert!(is_arithmetic_op(&BinOp::Sub));
        assert!(is_arithmetic_op(&BinOp::Mul));
        assert!(is_arithmetic_op(&BinOp::Div));
        assert!(is_arithmetic_op(&BinOp::Mod));
        assert!(is_arithmetic_op(&BinOp::FloorDiv));
        assert!(is_arithmetic_op(&BinOp::Pow));
        assert!(!is_arithmetic_op(&BinOp::Eq));
        assert!(!is_arithmetic_op(&BinOp::And));
    }

    #[test]
    fn test_op_preserves_int() {
        assert!(op_preserves_int(&BinOp::Add));
        assert!(op_preserves_int(&BinOp::Sub));
        assert!(op_preserves_int(&BinOp::Mul));
        assert!(op_preserves_int(&BinOp::Mod));
        assert!(op_preserves_int(&BinOp::FloorDiv));
        assert!(!op_preserves_int(&BinOp::Div));
        assert!(!op_preserves_int(&BinOp::Pow));
    }

    // =========================================================================
    // Tests for is_literal_type
    // =========================================================================

    #[test]
    fn test_is_literal_type_int() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(is_literal_type(&expr, "int"));
        assert!(!is_literal_type(&expr, "float"));
    }

    #[test]
    fn test_is_literal_type_float() {
        let expr = HirExpr::Literal(Literal::Float(3.15));
        assert!(is_literal_type(&expr, "float"));
        assert!(!is_literal_type(&expr, "int"));
    }

    #[test]
    fn test_is_literal_type_string() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert!(is_literal_type(&expr, "string"));
        assert!(!is_literal_type(&expr, "int"));
    }

    #[test]
    fn test_is_literal_type_bool() {
        let expr = HirExpr::Literal(Literal::Bool(true));
        assert!(is_literal_type(&expr, "bool"));
        assert!(!is_literal_type(&expr, "int"));
    }

    #[test]
    fn test_is_literal_type_none() {
        let expr = HirExpr::Literal(Literal::None);
        assert!(is_literal_type(&expr, "none"));
        assert!(!is_literal_type(&expr, "int"));
    }

    #[test]
    fn test_is_literal_type_bytes() {
        let expr = HirExpr::Literal(Literal::Bytes(vec![1, 2, 3]));
        assert!(is_literal_type(&expr, "bytes"));
        assert!(!is_literal_type(&expr, "string"));
    }

    #[test]
    fn test_is_literal_type_not_literal() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_literal_type(&expr, "int"));
        assert!(!is_literal_type(&expr, "float"));
    }
}
