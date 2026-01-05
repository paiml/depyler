//! Operator Precedence Helpers
//!
//! This module contains helpers for managing operator precedence during
//! code generation. Extracted from expr_gen.rs for better testability.
//!
//! DEPYLER-0582: Precedence preservation for correct Rust output

use crate::hir::BinOp;
use syn::parse_quote;

/// Get precedence of Rust binary operator (higher = binds tighter)
pub fn get_rust_op_precedence(op: &syn::BinOp) -> u8 {
    match op {
        syn::BinOp::Mul(_) | syn::BinOp::Div(_) | syn::BinOp::Rem(_) => 13,
        syn::BinOp::Add(_) | syn::BinOp::Sub(_) => 12,
        syn::BinOp::Shl(_) | syn::BinOp::Shr(_) => 11,
        syn::BinOp::BitAnd(_) => 10,
        syn::BinOp::BitXor(_) => 9,
        syn::BinOp::BitOr(_) => 8,
        syn::BinOp::Lt(_)
        | syn::BinOp::Le(_)
        | syn::BinOp::Gt(_)
        | syn::BinOp::Ge(_)
        | syn::BinOp::Eq(_)
        | syn::BinOp::Ne(_) => 7,
        syn::BinOp::And(_) => 6,
        syn::BinOp::Or(_) => 5,
        _ => 0, // Compound assignment operators, etc.
    }
}

/// Get precedence of Python binary operator for our HIR
pub fn get_python_op_precedence(op: BinOp) -> u8 {
    match op {
        BinOp::Pow => 14,
        BinOp::Mul | BinOp::Div | BinOp::Mod | BinOp::FloorDiv => 13,
        BinOp::Add | BinOp::Sub => 12,
        BinOp::LShift | BinOp::RShift => 11,
        BinOp::BitAnd => 10,
        BinOp::BitXor => 9,
        BinOp::BitOr => 8,
        BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq | BinOp::Eq | BinOp::NotEq => 7,
        BinOp::In | BinOp::NotIn => 7,
        BinOp::And => 6,
        BinOp::Or => 5,
    }
}

/// Wrap expression in parentheses if it's a binary operation with lower precedence
/// This preserves Python's parenthesized expressions in Rust output
/// e.g., (1 - beta1) * x should become (1.0 - beta1) * x, not 1.0 - beta1 * x
pub fn parenthesize_if_lower_precedence(expr: syn::Expr, parent_op: BinOp) -> syn::Expr {
    // Check if expression is a binary operation
    if let syn::Expr::Binary(bin_expr) = &expr {
        let child_precedence = get_rust_op_precedence(&bin_expr.op);
        let parent_precedence = get_python_op_precedence(parent_op);

        // If child has lower precedence, wrap in parentheses
        if child_precedence < parent_precedence {
            return parse_quote! { (#expr) };
        }
    }
    expr
}

/// Check if parentheses are needed for this operator combination
pub fn needs_parentheses(child_op: &syn::BinOp, parent_op: BinOp) -> bool {
    let child_prec = get_rust_op_precedence(child_op);
    let parent_prec = get_python_op_precedence(parent_op);
    child_prec < parent_prec
}

/// Get precedence level name for debugging
pub fn precedence_level_name(precedence: u8) -> &'static str {
    match precedence {
        14 => "exponentiation",
        13 => "multiplicative",
        12 => "additive",
        11 => "shift",
        10 => "bitwise_and",
        9 => "bitwise_xor",
        8 => "bitwise_or",
        7 => "comparison",
        6 => "logical_and",
        5 => "logical_or",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    // ============ get_rust_op_precedence tests ============

    #[test]
    fn test_rust_precedence_mul() {
        let op: syn::BinOp = parse_quote!(*);
        assert_eq!(get_rust_op_precedence(&op), 13);
    }

    #[test]
    fn test_rust_precedence_div() {
        let op: syn::BinOp = parse_quote!(/);
        assert_eq!(get_rust_op_precedence(&op), 13);
    }

    #[test]
    fn test_rust_precedence_rem() {
        let op: syn::BinOp = parse_quote!(%);
        assert_eq!(get_rust_op_precedence(&op), 13);
    }

    #[test]
    fn test_rust_precedence_add() {
        let op: syn::BinOp = parse_quote!(+);
        assert_eq!(get_rust_op_precedence(&op), 12);
    }

    #[test]
    fn test_rust_precedence_sub() {
        let op: syn::BinOp = parse_quote!(-);
        assert_eq!(get_rust_op_precedence(&op), 12);
    }

    #[test]
    fn test_rust_precedence_shl() {
        let op: syn::BinOp = parse_quote!(<<);
        assert_eq!(get_rust_op_precedence(&op), 11);
    }

    #[test]
    fn test_rust_precedence_shr() {
        let op: syn::BinOp = parse_quote!(>>);
        assert_eq!(get_rust_op_precedence(&op), 11);
    }

    #[test]
    fn test_rust_precedence_bitand() {
        let op: syn::BinOp = parse_quote!(&);
        assert_eq!(get_rust_op_precedence(&op), 10);
    }

    #[test]
    fn test_rust_precedence_bitxor() {
        let op: syn::BinOp = parse_quote!(^);
        assert_eq!(get_rust_op_precedence(&op), 9);
    }

    #[test]
    fn test_rust_precedence_bitor() {
        let op: syn::BinOp = parse_quote!(|);
        assert_eq!(get_rust_op_precedence(&op), 8);
    }

    #[test]
    fn test_rust_precedence_lt() {
        let op: syn::BinOp = parse_quote!(<);
        assert_eq!(get_rust_op_precedence(&op), 7);
    }

    #[test]
    fn test_rust_precedence_le() {
        let op: syn::BinOp = parse_quote!(<=);
        assert_eq!(get_rust_op_precedence(&op), 7);
    }

    #[test]
    fn test_rust_precedence_gt() {
        let op: syn::BinOp = parse_quote!(>);
        assert_eq!(get_rust_op_precedence(&op), 7);
    }

    #[test]
    fn test_rust_precedence_ge() {
        let op: syn::BinOp = parse_quote!(>=);
        assert_eq!(get_rust_op_precedence(&op), 7);
    }

    #[test]
    fn test_rust_precedence_eq() {
        let op: syn::BinOp = parse_quote!(==);
        assert_eq!(get_rust_op_precedence(&op), 7);
    }

    #[test]
    fn test_rust_precedence_ne() {
        let op: syn::BinOp = parse_quote!(!=);
        assert_eq!(get_rust_op_precedence(&op), 7);
    }

    #[test]
    fn test_rust_precedence_and() {
        let op: syn::BinOp = parse_quote!(&&);
        assert_eq!(get_rust_op_precedence(&op), 6);
    }

    #[test]
    fn test_rust_precedence_or() {
        let op: syn::BinOp = parse_quote!(||);
        assert_eq!(get_rust_op_precedence(&op), 5);
    }

    #[test]
    fn test_rust_precedence_compound_assign() {
        let op: syn::BinOp = parse_quote!(+=);
        assert_eq!(get_rust_op_precedence(&op), 0);
    }

    // ============ get_python_op_precedence tests ============

    #[test]
    fn test_python_precedence_pow() {
        assert_eq!(get_python_op_precedence(BinOp::Pow), 14);
    }

    #[test]
    fn test_python_precedence_mul() {
        assert_eq!(get_python_op_precedence(BinOp::Mul), 13);
    }

    #[test]
    fn test_python_precedence_div() {
        assert_eq!(get_python_op_precedence(BinOp::Div), 13);
    }

    #[test]
    fn test_python_precedence_mod() {
        assert_eq!(get_python_op_precedence(BinOp::Mod), 13);
    }

    #[test]
    fn test_python_precedence_floor_div() {
        assert_eq!(get_python_op_precedence(BinOp::FloorDiv), 13);
    }

    #[test]
    fn test_python_precedence_add() {
        assert_eq!(get_python_op_precedence(BinOp::Add), 12);
    }

    #[test]
    fn test_python_precedence_sub() {
        assert_eq!(get_python_op_precedence(BinOp::Sub), 12);
    }

    #[test]
    fn test_python_precedence_lshift() {
        assert_eq!(get_python_op_precedence(BinOp::LShift), 11);
    }

    #[test]
    fn test_python_precedence_rshift() {
        assert_eq!(get_python_op_precedence(BinOp::RShift), 11);
    }

    #[test]
    fn test_python_precedence_bitand() {
        assert_eq!(get_python_op_precedence(BinOp::BitAnd), 10);
    }

    #[test]
    fn test_python_precedence_bitxor() {
        assert_eq!(get_python_op_precedence(BinOp::BitXor), 9);
    }

    #[test]
    fn test_python_precedence_bitor() {
        assert_eq!(get_python_op_precedence(BinOp::BitOr), 8);
    }

    #[test]
    fn test_python_precedence_lt() {
        assert_eq!(get_python_op_precedence(BinOp::Lt), 7);
    }

    #[test]
    fn test_python_precedence_lteq() {
        assert_eq!(get_python_op_precedence(BinOp::LtEq), 7);
    }

    #[test]
    fn test_python_precedence_gt() {
        assert_eq!(get_python_op_precedence(BinOp::Gt), 7);
    }

    #[test]
    fn test_python_precedence_gteq() {
        assert_eq!(get_python_op_precedence(BinOp::GtEq), 7);
    }

    #[test]
    fn test_python_precedence_eq() {
        assert_eq!(get_python_op_precedence(BinOp::Eq), 7);
    }

    #[test]
    fn test_python_precedence_noteq() {
        assert_eq!(get_python_op_precedence(BinOp::NotEq), 7);
    }

    #[test]
    fn test_python_precedence_in() {
        assert_eq!(get_python_op_precedence(BinOp::In), 7);
    }

    #[test]
    fn test_python_precedence_notin() {
        assert_eq!(get_python_op_precedence(BinOp::NotIn), 7);
    }

    #[test]
    fn test_python_precedence_and() {
        assert_eq!(get_python_op_precedence(BinOp::And), 6);
    }

    #[test]
    fn test_python_precedence_or() {
        assert_eq!(get_python_op_precedence(BinOp::Or), 5);
    }

    // ============ needs_parentheses tests ============

    #[test]
    fn test_needs_parens_add_in_mul() {
        // (a + b) * c - addition inside multiplication needs parens
        let child: syn::BinOp = parse_quote!(+);
        assert!(needs_parentheses(&child, BinOp::Mul));
    }

    #[test]
    fn test_needs_parens_mul_in_add() {
        // a * b + c - multiplication inside addition doesn't need parens
        let child: syn::BinOp = parse_quote!(*);
        assert!(!needs_parentheses(&child, BinOp::Add));
    }

    #[test]
    fn test_needs_parens_or_in_and() {
        // (a || b) && c - or inside and needs parens
        let child: syn::BinOp = parse_quote!(||);
        assert!(needs_parentheses(&child, BinOp::And));
    }

    #[test]
    fn test_needs_parens_and_in_or() {
        // a && b || c - and inside or doesn't need parens
        let child: syn::BinOp = parse_quote!(&&);
        assert!(!needs_parentheses(&child, BinOp::Or));
    }

    #[test]
    fn test_needs_parens_same_precedence() {
        // a + b - c - same precedence doesn't need parens
        let child: syn::BinOp = parse_quote!(+);
        assert!(!needs_parentheses(&child, BinOp::Add));
    }

    #[test]
    fn test_needs_parens_sub_in_mul() {
        let child: syn::BinOp = parse_quote!(-);
        assert!(needs_parentheses(&child, BinOp::Mul));
    }

    #[test]
    fn test_needs_parens_bitor_in_bitand() {
        let child: syn::BinOp = parse_quote!(|);
        assert!(needs_parentheses(&child, BinOp::BitAnd));
    }

    #[test]
    fn test_needs_parens_shift_in_bitand() {
        let child: syn::BinOp = parse_quote!(<<);
        assert!(!needs_parentheses(&child, BinOp::BitAnd));
    }

    // ============ parenthesize_if_lower_precedence tests ============

    #[test]
    fn test_parenthesize_non_binary() {
        let expr: syn::Expr = parse_quote!(x);
        let result = parenthesize_if_lower_precedence(expr.clone(), BinOp::Mul);
        // Non-binary expressions should not be wrapped
        assert!(matches!(result, syn::Expr::Path(_)));
    }

    #[test]
    fn test_parenthesize_add_in_mul() {
        let expr: syn::Expr = parse_quote!(a + b);
        let result = parenthesize_if_lower_precedence(expr, BinOp::Mul);
        // Should be wrapped in parens
        assert!(matches!(result, syn::Expr::Paren(_)));
    }

    #[test]
    fn test_parenthesize_mul_in_add() {
        let expr: syn::Expr = parse_quote!(a * b);
        let result = parenthesize_if_lower_precedence(expr, BinOp::Add);
        // Should NOT be wrapped in parens (mul has higher precedence)
        assert!(matches!(result, syn::Expr::Binary(_)));
    }

    #[test]
    fn test_parenthesize_or_in_and() {
        let expr: syn::Expr = parse_quote!(a || b);
        let result = parenthesize_if_lower_precedence(expr, BinOp::And);
        assert!(matches!(result, syn::Expr::Paren(_)));
    }

    // ============ precedence_level_name tests ============

    #[test]
    fn test_level_name_exponentiation() {
        assert_eq!(precedence_level_name(14), "exponentiation");
    }

    #[test]
    fn test_level_name_multiplicative() {
        assert_eq!(precedence_level_name(13), "multiplicative");
    }

    #[test]
    fn test_level_name_additive() {
        assert_eq!(precedence_level_name(12), "additive");
    }

    #[test]
    fn test_level_name_shift() {
        assert_eq!(precedence_level_name(11), "shift");
    }

    #[test]
    fn test_level_name_bitwise_and() {
        assert_eq!(precedence_level_name(10), "bitwise_and");
    }

    #[test]
    fn test_level_name_bitwise_xor() {
        assert_eq!(precedence_level_name(9), "bitwise_xor");
    }

    #[test]
    fn test_level_name_bitwise_or() {
        assert_eq!(precedence_level_name(8), "bitwise_or");
    }

    #[test]
    fn test_level_name_comparison() {
        assert_eq!(precedence_level_name(7), "comparison");
    }

    #[test]
    fn test_level_name_logical_and() {
        assert_eq!(precedence_level_name(6), "logical_and");
    }

    #[test]
    fn test_level_name_logical_or() {
        assert_eq!(precedence_level_name(5), "logical_or");
    }

    #[test]
    fn test_level_name_other() {
        assert_eq!(precedence_level_name(0), "other");
        assert_eq!(precedence_level_name(100), "other");
    }
}
