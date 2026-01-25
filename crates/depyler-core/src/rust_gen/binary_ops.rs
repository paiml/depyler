//! Binary operation code generation
//!
//! This module handles converting Python binary operations to Rust expressions.
//! Extracted from expr_gen.rs for focused testing and maintainability.

use crate::hir::{BinOp, HirExpr, Literal};
use syn::parse_quote;

/// Get operator precedence for Rust binary operators
/// Higher numbers = higher precedence (binds tighter)
pub fn get_rust_op_precedence(op: &syn::BinOp) -> u8 {
    match op {
        syn::BinOp::Or(_) => 1,
        syn::BinOp::And(_) => 2,
        syn::BinOp::Eq(_) | syn::BinOp::Ne(_) => 3,
        syn::BinOp::Lt(_) | syn::BinOp::Le(_) | syn::BinOp::Gt(_) | syn::BinOp::Ge(_) => 4,
        syn::BinOp::BitOr(_) => 5,
        syn::BinOp::BitXor(_) => 6,
        syn::BinOp::BitAnd(_) => 7,
        syn::BinOp::Shl(_) | syn::BinOp::Shr(_) => 8,
        syn::BinOp::Add(_) | syn::BinOp::Sub(_) => 9,
        syn::BinOp::Mul(_) | syn::BinOp::Div(_) | syn::BinOp::Rem(_) => 10,
        _ => 0, // Unknown operators get lowest precedence
    }
}

/// Get operator precedence for Python binary operators
/// Mapped to Rust precedence values for comparison
pub fn get_python_op_precedence(op: BinOp) -> u8 {
    match op {
        BinOp::Or => 1,
        BinOp::And => 2,
        BinOp::In | BinOp::NotIn => 3,
        BinOp::Eq | BinOp::NotEq => 3,
        BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq => 4,
        BinOp::BitOr => 5,
        BinOp::BitXor => 6,
        BinOp::BitAnd => 7,
        BinOp::LShift | BinOp::RShift => 8,
        BinOp::Add | BinOp::Sub => 9,
        BinOp::Mul | BinOp::Div | BinOp::Mod | BinOp::FloorDiv => 10,
        BinOp::Pow => 11, // Power has highest precedence in Python
    }
}

/// Wrap an expression in parentheses if it has lower precedence than the parent
pub fn parenthesize_if_lower_precedence(expr: syn::Expr, parent_op: BinOp) -> syn::Expr {
    let parent_prec = get_python_op_precedence(parent_op);

    // Check if the expression is a binary operation with lower precedence
    if let syn::Expr::Binary(bin) = &expr {
        let child_prec = get_rust_op_precedence(&bin.op);
        if child_prec < parent_prec {
            return parse_quote! { (#expr) };
        }
    }
    expr
}

/// Wrap expression in a block for explicit parenthesization
/// Uses a block expression { expr } which is guaranteed to not be optimized away
pub fn wrap_in_parens(expr: syn::Expr) -> syn::Expr {
    syn::Expr::Block(syn::ExprBlock {
        attrs: vec![],
        label: None,
        block: syn::Block {
            brace_token: syn::token::Brace::default(),
            stmts: vec![syn::Stmt::Expr(expr, None)],
        },
    })
}

/// Check if an expression is an integer literal
pub fn is_int_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::Int(_)))
}

/// Check if an expression is a float literal
pub fn is_float_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::Float(_)))
}

/// Check if an expression is a string literal
pub fn is_string_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::String(_)))
}

/// Check if an expression is a boolean literal
pub fn is_bool_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::Bool(_)))
}

/// Check if binary operation is a comparison
pub fn is_comparison_op(op: BinOp) -> bool {
    matches!(
        op,
        BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq | BinOp::Eq | BinOp::NotEq
    )
}

/// Check if binary operation is an ordering comparison (excludes eq/ne)
pub fn is_ordering_comparison(op: BinOp) -> bool {
    matches!(op, BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq)
}

/// Check if binary operation is arithmetic
pub fn is_arithmetic_op(op: BinOp) -> bool {
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

/// Check if binary operation is logical
pub fn is_logical_op(op: BinOp) -> bool {
    matches!(op, BinOp::And | BinOp::Or)
}

/// Check if binary operation is bitwise
pub fn is_bitwise_op(op: BinOp) -> bool {
    matches!(
        op,
        BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::LShift | BinOp::RShift
    )
}

/// Convert Python binary operator to Rust binary operator
pub fn python_to_rust_binop(op: BinOp) -> syn::BinOp {
    match op {
        BinOp::Add => syn::BinOp::Add(Default::default()),
        BinOp::Sub => syn::BinOp::Sub(Default::default()),
        BinOp::Mul => syn::BinOp::Mul(Default::default()),
        BinOp::Div => syn::BinOp::Div(Default::default()),
        BinOp::Mod => syn::BinOp::Rem(Default::default()),
        BinOp::Eq => syn::BinOp::Eq(Default::default()),
        BinOp::NotEq => syn::BinOp::Ne(Default::default()),
        BinOp::Lt => syn::BinOp::Lt(Default::default()),
        BinOp::LtEq => syn::BinOp::Le(Default::default()),
        BinOp::Gt => syn::BinOp::Gt(Default::default()),
        BinOp::GtEq => syn::BinOp::Ge(Default::default()),
        BinOp::And => syn::BinOp::And(Default::default()),
        BinOp::Or => syn::BinOp::Or(Default::default()),
        BinOp::BitAnd => syn::BinOp::BitAnd(Default::default()),
        BinOp::BitOr => syn::BinOp::BitOr(Default::default()),
        BinOp::BitXor => syn::BinOp::BitXor(Default::default()),
        BinOp::LShift => syn::BinOp::Shl(Default::default()),
        BinOp::RShift => syn::BinOp::Shr(Default::default()),
        // These need special handling, return a placeholder
        BinOp::FloorDiv | BinOp::Pow | BinOp::In | BinOp::NotIn => {
            syn::BinOp::Add(Default::default()) // placeholder
        }
    }
}

/// Generate floor division expression
/// Python: a // b (rounds toward negative infinity)
/// Rust: needs special handling for negative numbers
pub fn generate_floor_div(left: syn::Expr, right: syn::Expr) -> syn::Expr {
    parse_quote! {
        {
            let a = #left;
            let b = #right;
            let q = a / b;
            let r = a % b;
            if r != 0 && (a < 0) != (b < 0) {
                q - 1
            } else {
                q
            }
        }
    }
}

/// Generate power expression with type awareness
/// Handles integer and float exponents differently
pub fn generate_pow_int_int(base: syn::Expr, exp: syn::Expr, exp_val: i64) -> syn::Expr {
    if exp_val < 0 {
        // Negative exponent: convert to float
        parse_quote! {
            (#base as f64).powf(#exp as f64)
        }
    } else {
        // Positive integer exponent: use checked_pow
        parse_quote! {
            (#base as i32).checked_pow(#exp as u32)
                .expect("Power operation overflowed")
        }
    }
}

/// Generate power expression for float base
pub fn generate_pow_float(base: syn::Expr, exp: syn::Expr) -> syn::Expr {
    parse_quote! {
        (#base as f64).powf(#exp as f64)
    }
}

/// Generate modulo expression
pub fn generate_modulo(left: syn::Expr, right: syn::Expr) -> syn::Expr {
    parse_quote! { #left % #right }
}

/// Generate string repetition (string * int)
pub fn generate_string_repeat(string_expr: syn::Expr, count_expr: syn::Expr) -> syn::Expr {
    parse_quote! { #string_expr.repeat(#count_expr as usize) }
}

/// Check if expression could be a string variable based on naming conventions
pub fn is_likely_string_var_by_name(name: &str) -> bool {
    name == "text" || name == "s" || name == "line" || name.ends_with("_str")
}

/// Coerce an integer expression to float if needed
pub fn coerce_to_float(expr: syn::Expr, use_f32: bool) -> syn::Expr {
    if use_f32 {
        parse_quote! { (#expr as f32) }
    } else {
        parse_quote! { (#expr as f64) }
    }
}

/// Generate integer literal as float
pub fn int_to_float_literal(n: i64, use_f32: bool) -> syn::Expr {
    if use_f32 {
        let val = n as f32;
        parse_quote! { #val }
    } else {
        let val = n as f64;
        parse_quote! { #val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    // ============================================================================
    // get_rust_op_precedence tests - 100% branch coverage
    // ============================================================================

    #[test]
    fn test_rust_precedence_or() {
        let op = syn::BinOp::Or(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 1);
    }

    #[test]
    fn test_rust_precedence_and() {
        let op = syn::BinOp::And(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 2);
    }

    #[test]
    fn test_rust_precedence_eq() {
        let op = syn::BinOp::Eq(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 3);
    }

    #[test]
    fn test_rust_precedence_ne() {
        let op = syn::BinOp::Ne(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 3);
    }

    #[test]
    fn test_rust_precedence_lt() {
        let op = syn::BinOp::Lt(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 4);
    }

    #[test]
    fn test_rust_precedence_le() {
        let op = syn::BinOp::Le(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 4);
    }

    #[test]
    fn test_rust_precedence_gt() {
        let op = syn::BinOp::Gt(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 4);
    }

    #[test]
    fn test_rust_precedence_ge() {
        let op = syn::BinOp::Ge(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 4);
    }

    #[test]
    fn test_rust_precedence_bitor() {
        let op = syn::BinOp::BitOr(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 5);
    }

    #[test]
    fn test_rust_precedence_bitxor() {
        let op = syn::BinOp::BitXor(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 6);
    }

    #[test]
    fn test_rust_precedence_bitand() {
        let op = syn::BinOp::BitAnd(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 7);
    }

    #[test]
    fn test_rust_precedence_shl() {
        let op = syn::BinOp::Shl(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 8);
    }

    #[test]
    fn test_rust_precedence_shr() {
        let op = syn::BinOp::Shr(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 8);
    }

    #[test]
    fn test_rust_precedence_add() {
        let op = syn::BinOp::Add(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 9);
    }

    #[test]
    fn test_rust_precedence_sub() {
        let op = syn::BinOp::Sub(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 9);
    }

    #[test]
    fn test_rust_precedence_mul() {
        let op = syn::BinOp::Mul(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 10);
    }

    #[test]
    fn test_rust_precedence_div() {
        let op = syn::BinOp::Div(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 10);
    }

    #[test]
    fn test_rust_precedence_rem() {
        let op = syn::BinOp::Rem(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 10);
    }

    #[test]
    fn test_rust_precedence_unknown() {
        // Test the wildcard case - AddAssign is not a BinOp in the traditional sense
        let op = syn::BinOp::AddAssign(Default::default());
        assert_eq!(get_rust_op_precedence(&op), 0);
    }

    // ============================================================================
    // get_python_op_precedence tests - 100% branch coverage
    // ============================================================================

    #[test]
    fn test_python_precedence_or() {
        assert_eq!(get_python_op_precedence(BinOp::Or), 1);
    }

    #[test]
    fn test_python_precedence_and() {
        assert_eq!(get_python_op_precedence(BinOp::And), 2);
    }

    #[test]
    fn test_python_precedence_in() {
        assert_eq!(get_python_op_precedence(BinOp::In), 3);
    }

    #[test]
    fn test_python_precedence_notin() {
        assert_eq!(get_python_op_precedence(BinOp::NotIn), 3);
    }

    #[test]
    fn test_python_precedence_eq() {
        assert_eq!(get_python_op_precedence(BinOp::Eq), 3);
    }

    #[test]
    fn test_python_precedence_noteq() {
        assert_eq!(get_python_op_precedence(BinOp::NotEq), 3);
    }

    #[test]
    fn test_python_precedence_lt() {
        assert_eq!(get_python_op_precedence(BinOp::Lt), 4);
    }

    #[test]
    fn test_python_precedence_lteq() {
        assert_eq!(get_python_op_precedence(BinOp::LtEq), 4);
    }

    #[test]
    fn test_python_precedence_gt() {
        assert_eq!(get_python_op_precedence(BinOp::Gt), 4);
    }

    #[test]
    fn test_python_precedence_gteq() {
        assert_eq!(get_python_op_precedence(BinOp::GtEq), 4);
    }

    #[test]
    fn test_python_precedence_bitor() {
        assert_eq!(get_python_op_precedence(BinOp::BitOr), 5);
    }

    #[test]
    fn test_python_precedence_bitxor() {
        assert_eq!(get_python_op_precedence(BinOp::BitXor), 6);
    }

    #[test]
    fn test_python_precedence_bitand() {
        assert_eq!(get_python_op_precedence(BinOp::BitAnd), 7);
    }

    #[test]
    fn test_python_precedence_lshift() {
        assert_eq!(get_python_op_precedence(BinOp::LShift), 8);
    }

    #[test]
    fn test_python_precedence_rshift() {
        assert_eq!(get_python_op_precedence(BinOp::RShift), 8);
    }

    #[test]
    fn test_python_precedence_add() {
        assert_eq!(get_python_op_precedence(BinOp::Add), 9);
    }

    #[test]
    fn test_python_precedence_sub() {
        assert_eq!(get_python_op_precedence(BinOp::Sub), 9);
    }

    #[test]
    fn test_python_precedence_mul() {
        assert_eq!(get_python_op_precedence(BinOp::Mul), 10);
    }

    #[test]
    fn test_python_precedence_div() {
        assert_eq!(get_python_op_precedence(BinOp::Div), 10);
    }

    #[test]
    fn test_python_precedence_mod() {
        assert_eq!(get_python_op_precedence(BinOp::Mod), 10);
    }

    #[test]
    fn test_python_precedence_floordiv() {
        assert_eq!(get_python_op_precedence(BinOp::FloorDiv), 10);
    }

    #[test]
    fn test_python_precedence_pow() {
        assert_eq!(get_python_op_precedence(BinOp::Pow), 11);
    }

    // ============================================================================
    // is_*_literal tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_is_int_literal_true() {
        assert!(is_int_literal(&HirExpr::Literal(Literal::Int(42))));
        assert!(is_int_literal(&HirExpr::Literal(Literal::Int(0))));
        assert!(is_int_literal(&HirExpr::Literal(Literal::Int(-1))));
    }

    #[test]
    fn test_is_int_literal_false() {
        assert!(!is_int_literal(&HirExpr::Literal(Literal::Float(3.15))));
        assert!(!is_int_literal(&HirExpr::Literal(Literal::String(
            "hi".to_string()
        ))));
        assert!(!is_int_literal(&HirExpr::Var("x".to_string())));
    }

    #[test]
    fn test_is_float_literal_true() {
        assert!(is_float_literal(&HirExpr::Literal(Literal::Float(3.15))));
        assert!(is_float_literal(&HirExpr::Literal(Literal::Float(0.0))));
        assert!(is_float_literal(&HirExpr::Literal(Literal::Float(-1.5))));
    }

    #[test]
    fn test_is_float_literal_false() {
        assert!(!is_float_literal(&HirExpr::Literal(Literal::Int(42))));
        assert!(!is_float_literal(&HirExpr::Literal(Literal::String(
            "3.14".to_string()
        ))));
        assert!(!is_float_literal(&HirExpr::Var("x".to_string())));
    }

    #[test]
    fn test_is_string_literal_true() {
        assert!(is_string_literal(&HirExpr::Literal(Literal::String(
            "hello".to_string()
        ))));
        assert!(is_string_literal(&HirExpr::Literal(Literal::String(
            "".to_string()
        ))));
    }

    #[test]
    fn test_is_string_literal_false() {
        assert!(!is_string_literal(&HirExpr::Literal(Literal::Int(42))));
        assert!(!is_string_literal(&HirExpr::Literal(Literal::Float(3.15))));
        assert!(!is_string_literal(&HirExpr::Var("s".to_string())));
    }

    #[test]
    fn test_is_bool_literal_true() {
        assert!(is_bool_literal(&HirExpr::Literal(Literal::Bool(true))));
        assert!(is_bool_literal(&HirExpr::Literal(Literal::Bool(false))));
    }

    #[test]
    fn test_is_bool_literal_false() {
        assert!(!is_bool_literal(&HirExpr::Literal(Literal::Int(1))));
        assert!(!is_bool_literal(&HirExpr::Literal(Literal::String(
            "true".to_string()
        ))));
        assert!(!is_bool_literal(&HirExpr::Var("flag".to_string())));
    }

    // ============================================================================
    // is_*_op tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_is_comparison_op_true() {
        assert!(is_comparison_op(BinOp::Lt));
        assert!(is_comparison_op(BinOp::LtEq));
        assert!(is_comparison_op(BinOp::Gt));
        assert!(is_comparison_op(BinOp::GtEq));
        assert!(is_comparison_op(BinOp::Eq));
        assert!(is_comparison_op(BinOp::NotEq));
    }

    #[test]
    fn test_is_comparison_op_false() {
        assert!(!is_comparison_op(BinOp::Add));
        assert!(!is_comparison_op(BinOp::Sub));
        assert!(!is_comparison_op(BinOp::And));
        assert!(!is_comparison_op(BinOp::Or));
        assert!(!is_comparison_op(BinOp::In));
    }

    #[test]
    fn test_is_ordering_comparison_true() {
        assert!(is_ordering_comparison(BinOp::Lt));
        assert!(is_ordering_comparison(BinOp::LtEq));
        assert!(is_ordering_comparison(BinOp::Gt));
        assert!(is_ordering_comparison(BinOp::GtEq));
    }

    #[test]
    fn test_is_ordering_comparison_false() {
        assert!(!is_ordering_comparison(BinOp::Eq));
        assert!(!is_ordering_comparison(BinOp::NotEq));
        assert!(!is_ordering_comparison(BinOp::Add));
    }

    #[test]
    fn test_is_arithmetic_op_true() {
        assert!(is_arithmetic_op(BinOp::Add));
        assert!(is_arithmetic_op(BinOp::Sub));
        assert!(is_arithmetic_op(BinOp::Mul));
        assert!(is_arithmetic_op(BinOp::Div));
        assert!(is_arithmetic_op(BinOp::Mod));
        assert!(is_arithmetic_op(BinOp::FloorDiv));
        assert!(is_arithmetic_op(BinOp::Pow));
    }

    #[test]
    fn test_is_arithmetic_op_false() {
        assert!(!is_arithmetic_op(BinOp::Eq));
        assert!(!is_arithmetic_op(BinOp::And));
        assert!(!is_arithmetic_op(BinOp::BitOr));
    }

    #[test]
    fn test_is_logical_op_true() {
        assert!(is_logical_op(BinOp::And));
        assert!(is_logical_op(BinOp::Or));
    }

    #[test]
    fn test_is_logical_op_false() {
        assert!(!is_logical_op(BinOp::Add));
        assert!(!is_logical_op(BinOp::Eq));
        assert!(!is_logical_op(BinOp::BitAnd));
    }

    #[test]
    fn test_is_bitwise_op_true() {
        assert!(is_bitwise_op(BinOp::BitAnd));
        assert!(is_bitwise_op(BinOp::BitOr));
        assert!(is_bitwise_op(BinOp::BitXor));
        assert!(is_bitwise_op(BinOp::LShift));
        assert!(is_bitwise_op(BinOp::RShift));
    }

    #[test]
    fn test_is_bitwise_op_false() {
        assert!(!is_bitwise_op(BinOp::And));
        assert!(!is_bitwise_op(BinOp::Or));
        assert!(!is_bitwise_op(BinOp::Add));
    }

    // ============================================================================
    // python_to_rust_binop tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_python_to_rust_add() {
        let op = python_to_rust_binop(BinOp::Add);
        assert!(matches!(op, syn::BinOp::Add(_)));
    }

    #[test]
    fn test_python_to_rust_sub() {
        let op = python_to_rust_binop(BinOp::Sub);
        assert!(matches!(op, syn::BinOp::Sub(_)));
    }

    #[test]
    fn test_python_to_rust_mul() {
        let op = python_to_rust_binop(BinOp::Mul);
        assert!(matches!(op, syn::BinOp::Mul(_)));
    }

    #[test]
    fn test_python_to_rust_div() {
        let op = python_to_rust_binop(BinOp::Div);
        assert!(matches!(op, syn::BinOp::Div(_)));
    }

    #[test]
    fn test_python_to_rust_mod() {
        let op = python_to_rust_binop(BinOp::Mod);
        assert!(matches!(op, syn::BinOp::Rem(_)));
    }

    #[test]
    fn test_python_to_rust_eq() {
        let op = python_to_rust_binop(BinOp::Eq);
        assert!(matches!(op, syn::BinOp::Eq(_)));
    }

    #[test]
    fn test_python_to_rust_noteq() {
        let op = python_to_rust_binop(BinOp::NotEq);
        assert!(matches!(op, syn::BinOp::Ne(_)));
    }

    #[test]
    fn test_python_to_rust_lt() {
        let op = python_to_rust_binop(BinOp::Lt);
        assert!(matches!(op, syn::BinOp::Lt(_)));
    }

    #[test]
    fn test_python_to_rust_lteq() {
        let op = python_to_rust_binop(BinOp::LtEq);
        assert!(matches!(op, syn::BinOp::Le(_)));
    }

    #[test]
    fn test_python_to_rust_gt() {
        let op = python_to_rust_binop(BinOp::Gt);
        assert!(matches!(op, syn::BinOp::Gt(_)));
    }

    #[test]
    fn test_python_to_rust_gteq() {
        let op = python_to_rust_binop(BinOp::GtEq);
        assert!(matches!(op, syn::BinOp::Ge(_)));
    }

    #[test]
    fn test_python_to_rust_and() {
        let op = python_to_rust_binop(BinOp::And);
        assert!(matches!(op, syn::BinOp::And(_)));
    }

    #[test]
    fn test_python_to_rust_or() {
        let op = python_to_rust_binop(BinOp::Or);
        assert!(matches!(op, syn::BinOp::Or(_)));
    }

    #[test]
    fn test_python_to_rust_bitand() {
        let op = python_to_rust_binop(BinOp::BitAnd);
        assert!(matches!(op, syn::BinOp::BitAnd(_)));
    }

    #[test]
    fn test_python_to_rust_bitor() {
        let op = python_to_rust_binop(BinOp::BitOr);
        assert!(matches!(op, syn::BinOp::BitOr(_)));
    }

    #[test]
    fn test_python_to_rust_bitxor() {
        let op = python_to_rust_binop(BinOp::BitXor);
        assert!(matches!(op, syn::BinOp::BitXor(_)));
    }

    #[test]
    fn test_python_to_rust_lshift() {
        let op = python_to_rust_binop(BinOp::LShift);
        assert!(matches!(op, syn::BinOp::Shl(_)));
    }

    #[test]
    fn test_python_to_rust_rshift() {
        let op = python_to_rust_binop(BinOp::RShift);
        assert!(matches!(op, syn::BinOp::Shr(_)));
    }

    #[test]
    fn test_python_to_rust_floordiv_placeholder() {
        // FloorDiv needs special handling, returns placeholder
        let op = python_to_rust_binop(BinOp::FloorDiv);
        assert!(matches!(op, syn::BinOp::Add(_)));
    }

    #[test]
    fn test_python_to_rust_pow_placeholder() {
        let op = python_to_rust_binop(BinOp::Pow);
        assert!(matches!(op, syn::BinOp::Add(_)));
    }

    #[test]
    fn test_python_to_rust_in_placeholder() {
        let op = python_to_rust_binop(BinOp::In);
        assert!(matches!(op, syn::BinOp::Add(_)));
    }

    #[test]
    fn test_python_to_rust_notin_placeholder() {
        let op = python_to_rust_binop(BinOp::NotIn);
        assert!(matches!(op, syn::BinOp::Add(_)));
    }

    // ============================================================================
    // wrap_in_parens tests
    // ============================================================================

    #[test]
    fn test_wrap_in_parens_simple() {
        let expr: syn::Expr = parse_quote! { x };
        let wrapped = wrap_in_parens(expr);
        let s = wrapped.to_token_stream().to_string();
        assert!(s.contains("{") && s.contains("}"));
    }

    #[test]
    fn test_wrap_in_parens_binary() {
        let expr: syn::Expr = parse_quote! { a + b };
        let wrapped = wrap_in_parens(expr);
        let s = wrapped.to_token_stream().to_string();
        assert!(s.contains("a + b"));
    }

    // ============================================================================
    // parenthesize_if_lower_precedence tests
    // ============================================================================

    #[test]
    fn test_parenthesize_lower_prec() {
        // Addition has lower precedence than multiplication
        let expr: syn::Expr = parse_quote! { a + b };
        let result = parenthesize_if_lower_precedence(expr.clone(), BinOp::Mul);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("("));
    }

    #[test]
    fn test_parenthesize_same_prec() {
        // Same precedence - no parentheses needed
        let expr: syn::Expr = parse_quote! { a + b };
        let result = parenthesize_if_lower_precedence(expr.clone(), BinOp::Add);
        let s = result.to_token_stream().to_string();
        // Should not add extra parens
        assert_eq!(s.matches("(").count(), 0);
    }

    #[test]
    fn test_parenthesize_non_binary() {
        // Non-binary expression - no change
        let expr: syn::Expr = parse_quote! { x };
        let result = parenthesize_if_lower_precedence(expr.clone(), BinOp::Mul);
        let s = result.to_token_stream().to_string();
        assert_eq!(s.trim(), "x");
    }

    // ============================================================================
    // generate_floor_div tests
    // ============================================================================

    #[test]
    fn test_generate_floor_div() {
        let left: syn::Expr = parse_quote! { a };
        let right: syn::Expr = parse_quote! { b };
        let result = generate_floor_div(left, right);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("a / b"));
        assert!(s.contains("a % b"));
        assert!(s.contains("q - 1"));
    }

    // ============================================================================
    // generate_pow tests
    // ============================================================================

    #[test]
    fn test_generate_pow_int_positive() {
        let base: syn::Expr = parse_quote! { 2 };
        let exp: syn::Expr = parse_quote! { 3 };
        let result = generate_pow_int_int(base, exp, 3);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("checked_pow"));
    }

    #[test]
    fn test_generate_pow_int_negative() {
        let base: syn::Expr = parse_quote! { 2 };
        let exp: syn::Expr = parse_quote! { -1 };
        let result = generate_pow_int_int(base, exp, -1);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("powf"));
    }

    #[test]
    fn test_generate_pow_float() {
        let base: syn::Expr = parse_quote! { 2.0 };
        let exp: syn::Expr = parse_quote! { 0.5 };
        let result = generate_pow_float(base, exp);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("powf"));
    }

    // ============================================================================
    // generate_modulo tests
    // ============================================================================

    #[test]
    fn test_generate_modulo() {
        let left: syn::Expr = parse_quote! { a };
        let right: syn::Expr = parse_quote! { b };
        let result = generate_modulo(left, right);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("%"));
    }

    // ============================================================================
    // generate_string_repeat tests
    // ============================================================================

    #[test]
    fn test_generate_string_repeat() {
        let s: syn::Expr = parse_quote! { "abc" };
        let n: syn::Expr = parse_quote! { 3 };
        let result = generate_string_repeat(s, n);
        let out = result.to_token_stream().to_string();
        assert!(out.contains("repeat"));
    }

    // ============================================================================
    // is_likely_string_var_by_name tests
    // ============================================================================

    #[test]
    fn test_likely_string_var_text() {
        assert!(is_likely_string_var_by_name("text"));
    }

    #[test]
    fn test_likely_string_var_s() {
        assert!(is_likely_string_var_by_name("s"));
    }

    #[test]
    fn test_likely_string_var_line() {
        assert!(is_likely_string_var_by_name("line"));
    }

    #[test]
    fn test_likely_string_var_suffix() {
        assert!(is_likely_string_var_by_name("my_str"));
        assert!(is_likely_string_var_by_name("result_str"));
    }

    #[test]
    fn test_not_likely_string_var() {
        assert!(!is_likely_string_var_by_name("x"));
        assert!(!is_likely_string_var_by_name("count"));
        assert!(!is_likely_string_var_by_name("data"));
    }

    // ============================================================================
    // coerce_to_float tests
    // ============================================================================

    #[test]
    fn test_coerce_to_f64() {
        let expr: syn::Expr = parse_quote! { x };
        let result = coerce_to_float(expr, false);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("f64"));
    }

    #[test]
    fn test_coerce_to_f32() {
        let expr: syn::Expr = parse_quote! { x };
        let result = coerce_to_float(expr, true);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("f32"));
    }

    // ============================================================================
    // int_to_float_literal tests
    // ============================================================================

    #[test]
    fn test_int_to_f64_literal() {
        let result = int_to_float_literal(42, false);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("42"));
    }

    #[test]
    fn test_int_to_f32_literal() {
        let result = int_to_float_literal(42, true);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("42"));
    }

    #[test]
    fn test_int_to_float_zero() {
        let result = int_to_float_literal(0, false);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("0"));
    }

    #[test]
    fn test_int_to_float_negative() {
        let result = int_to_float_literal(-5, false);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("-5") || s.contains("- 5"));
    }
}
