//! Unary operation code generation utilities
//!
//! This module provides helper functions for unary operation handling
//! in Python-to-Rust transpilation.

use crate::hir::{HirExpr, Literal, Type, UnaryOp};
use syn::parse_quote;

/// Convert Python unary operator to Rust expression
pub fn python_to_rust_unary(op: UnaryOp, operand: syn::Expr) -> syn::Expr {
    match op {
        UnaryOp::Not => parse_quote! { !#operand },
        UnaryOp::Neg => parse_quote! { -#operand },
        UnaryOp::Pos => operand, // No +x in Rust, just return operand
        UnaryOp::BitNot => parse_quote! { !#operand },
    }
}

/// Check if unary operator is logical NOT
pub fn is_logical_not(op: UnaryOp) -> bool {
    matches!(op, UnaryOp::Not)
}

/// Check if unary operator is arithmetic negation
pub fn is_negation(op: UnaryOp) -> bool {
    matches!(op, UnaryOp::Neg)
}

/// Check if unary operator is positive (no-op in Rust)
pub fn is_positive(op: UnaryOp) -> bool {
    matches!(op, UnaryOp::Pos)
}

/// Check if unary operator is bitwise NOT
pub fn is_bitwise_not(op: UnaryOp) -> bool {
    matches!(op, UnaryOp::BitNot)
}

/// Check if a type requires special NOT handling (collections use .is_empty())
pub fn type_needs_is_empty_for_not(ty: &Type) -> bool {
    matches!(
        ty,
        Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::String
    )
}

/// Check if a type requires .is_none() for NOT (Optional types)
pub fn type_needs_is_none_for_not(ty: &Type) -> bool {
    matches!(ty, Type::Optional(_))
}

/// Check if expression is a double negation (--x)
pub fn is_double_negation(expr: &HirExpr) -> bool {
    matches!(
        expr,
        HirExpr::Unary {
            op: UnaryOp::Neg,
            operand
        } if matches!(operand.as_ref(), HirExpr::Unary { op: UnaryOp::Neg, .. })
    )
}

/// Check if expression is a double NOT (not not x)
pub fn is_double_not(expr: &HirExpr) -> bool {
    matches!(
        expr,
        HirExpr::Unary {
            op: UnaryOp::Not,
            operand
        } if matches!(operand.as_ref(), HirExpr::Unary { op: UnaryOp::Not, .. })
    )
}

/// Simplify double negation to the inner operand
pub fn simplify_double_negation(expr: &HirExpr) -> Option<&HirExpr> {
    if let HirExpr::Unary {
        op: UnaryOp::Neg,
        operand: outer,
    } = expr
    {
        if let HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: inner,
        } = outer.as_ref()
        {
            return Some(inner.as_ref());
        }
    }
    None
}

/// Simplify double NOT to the inner operand
pub fn simplify_double_not(expr: &HirExpr) -> Option<&HirExpr> {
    if let HirExpr::Unary {
        op: UnaryOp::Not,
        operand: outer,
    } = expr
    {
        if let HirExpr::Unary {
            op: UnaryOp::Not,
            operand: inner,
        } = outer.as_ref()
        {
            return Some(inner.as_ref());
        }
    }
    None
}

/// Check if expression is negation of a literal
pub fn is_negated_literal(expr: &HirExpr) -> bool {
    matches!(
        expr,
        HirExpr::Unary {
            op: UnaryOp::Neg,
            operand
        } if matches!(operand.as_ref(), HirExpr::Literal(Literal::Int(_) | Literal::Float(_)))
    )
}

/// Get the value of a negated int literal
pub fn get_negated_int_value(expr: &HirExpr) -> Option<i64> {
    if let HirExpr::Unary {
        op: UnaryOp::Neg,
        operand,
    } = expr
    {
        if let HirExpr::Literal(Literal::Int(n)) = operand.as_ref() {
            return Some(-n);
        }
    }
    None
}

/// Get the value of a negated float literal
pub fn get_negated_float_value(expr: &HirExpr) -> Option<f64> {
    if let HirExpr::Unary {
        op: UnaryOp::Neg,
        operand,
    } = expr
    {
        if let HirExpr::Literal(Literal::Float(f)) = operand.as_ref() {
            return Some(-f);
        }
    }
    None
}

/// Check if method name returns Option (needs .is_none() for NOT)
pub fn is_option_returning_method(method: &str) -> bool {
    matches!(method, "find" | "search" | "match" | "get" | "ok")
}

/// Check if function name returns Option (needs .is_none() for NOT)
pub fn is_option_returning_function(func: &str) -> bool {
    matches!(func, "match" | "search" | "find")
}

/// Generate .is_empty() expression for collection NOT
pub fn generate_is_empty(operand: syn::Expr) -> syn::Expr {
    parse_quote! { #operand.is_empty() }
}

/// Generate .is_none() expression for Option NOT
pub fn generate_is_none(operand: syn::Expr) -> syn::Expr {
    parse_quote! { #operand.is_none() }
}

/// Generate .is_some() expression for Option truthiness
pub fn generate_is_some(operand: syn::Expr) -> syn::Expr {
    parse_quote! { #operand.is_some() }
}

/// Generate !.is_empty() expression for collection truthiness
pub fn generate_not_is_empty(operand: syn::Expr) -> syn::Expr {
    parse_quote! { !#operand.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    // ============================================================================
    // python_to_rust_unary tests
    // ============================================================================

    #[test]
    fn test_unary_not() {
        let operand: syn::Expr = syn::parse_quote! { x };
        let result = python_to_rust_unary(UnaryOp::Not, operand);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("!"));
    }

    #[test]
    fn test_unary_neg() {
        let operand: syn::Expr = syn::parse_quote! { x };
        let result = python_to_rust_unary(UnaryOp::Neg, operand);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("-"));
    }

    #[test]
    fn test_unary_pos() {
        let operand: syn::Expr = syn::parse_quote! { x };
        let result = python_to_rust_unary(UnaryOp::Pos, operand);
        let s = result.to_token_stream().to_string();
        assert_eq!(s.trim(), "x");
    }

    #[test]
    fn test_unary_bitnot() {
        let operand: syn::Expr = syn::parse_quote! { x };
        let result = python_to_rust_unary(UnaryOp::BitNot, operand);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("!"));
    }

    // ============================================================================
    // is_* operator tests
    // ============================================================================

    #[test]
    fn test_is_logical_not() {
        assert!(is_logical_not(UnaryOp::Not));
        assert!(!is_logical_not(UnaryOp::Neg));
        assert!(!is_logical_not(UnaryOp::Pos));
        assert!(!is_logical_not(UnaryOp::BitNot));
    }

    #[test]
    fn test_is_negation() {
        assert!(is_negation(UnaryOp::Neg));
        assert!(!is_negation(UnaryOp::Not));
        assert!(!is_negation(UnaryOp::Pos));
        assert!(!is_negation(UnaryOp::BitNot));
    }

    #[test]
    fn test_is_positive() {
        assert!(is_positive(UnaryOp::Pos));
        assert!(!is_positive(UnaryOp::Neg));
        assert!(!is_positive(UnaryOp::Not));
        assert!(!is_positive(UnaryOp::BitNot));
    }

    #[test]
    fn test_is_bitwise_not() {
        assert!(is_bitwise_not(UnaryOp::BitNot));
        assert!(!is_bitwise_not(UnaryOp::Not));
        assert!(!is_bitwise_not(UnaryOp::Neg));
        assert!(!is_bitwise_not(UnaryOp::Pos));
    }

    // ============================================================================
    // type_needs_* tests
    // ============================================================================

    #[test]
    fn test_type_needs_is_empty_list() {
        assert!(type_needs_is_empty_for_not(&Type::List(Box::new(Type::Int))));
    }

    #[test]
    fn test_type_needs_is_empty_dict() {
        assert!(type_needs_is_empty_for_not(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
    }

    #[test]
    fn test_type_needs_is_empty_set() {
        assert!(type_needs_is_empty_for_not(&Type::Set(Box::new(Type::Int))));
    }

    #[test]
    fn test_type_needs_is_empty_string() {
        assert!(type_needs_is_empty_for_not(&Type::String));
    }

    #[test]
    fn test_type_not_needs_is_empty() {
        assert!(!type_needs_is_empty_for_not(&Type::Int));
        assert!(!type_needs_is_empty_for_not(&Type::Float));
        assert!(!type_needs_is_empty_for_not(&Type::Bool));
    }

    #[test]
    fn test_type_needs_is_none() {
        assert!(type_needs_is_none_for_not(&Type::Optional(Box::new(
            Type::Int
        ))));
        assert!(type_needs_is_none_for_not(&Type::Optional(Box::new(
            Type::String
        ))));
    }

    #[test]
    fn test_type_not_needs_is_none() {
        assert!(!type_needs_is_none_for_not(&Type::Int));
        assert!(!type_needs_is_none_for_not(&Type::String));
        assert!(!type_needs_is_none_for_not(&Type::List(Box::new(Type::Int))));
    }

    // ============================================================================
    // is_double_* tests
    // ============================================================================

    #[test]
    fn test_is_double_negation_true() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(HirExpr::Var("x".to_string())),
            }),
        };
        assert!(is_double_negation(&expr));
    }

    #[test]
    fn test_is_double_negation_false() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(!is_double_negation(&expr));
    }

    #[test]
    fn test_is_double_negation_wrong_op() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(HirExpr::Var("x".to_string())),
            }),
        };
        assert!(!is_double_negation(&expr));
    }

    #[test]
    fn test_is_double_not_true() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(HirExpr::Var("x".to_string())),
            }),
        };
        assert!(is_double_not(&expr));
    }

    #[test]
    fn test_is_double_not_false() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(!is_double_not(&expr));
    }

    // ============================================================================
    // simplify_double_* tests
    // ============================================================================

    #[test]
    fn test_simplify_double_negation() {
        let inner = HirExpr::Var("x".to_string());
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(inner.clone()),
            }),
        };
        let result = simplify_double_negation(&expr);
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), HirExpr::Var(v) if v == "x"));
    }

    #[test]
    fn test_simplify_double_negation_not_double() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(simplify_double_negation(&expr).is_none());
    }

    #[test]
    fn test_simplify_double_not() {
        let inner = HirExpr::Var("x".to_string());
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(inner.clone()),
            }),
        };
        let result = simplify_double_not(&expr);
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), HirExpr::Var(v) if v == "x"));
    }

    #[test]
    fn test_simplify_double_not_not_double() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(simplify_double_not(&expr).is_none());
    }

    // ============================================================================
    // is_negated_literal tests
    // ============================================================================

    #[test]
    fn test_is_negated_int_literal() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(42))),
        };
        assert!(is_negated_literal(&expr));
    }

    #[test]
    fn test_is_negated_float_literal() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Float(3.15))),
        };
        assert!(is_negated_literal(&expr));
    }

    #[test]
    fn test_is_negated_var_not_literal() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(!is_negated_literal(&expr));
    }

    #[test]
    fn test_is_negated_wrong_op() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Literal(Literal::Int(42))),
        };
        assert!(!is_negated_literal(&expr));
    }

    // ============================================================================
    // get_negated_*_value tests
    // ============================================================================

    #[test]
    fn test_get_negated_int_value() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(42))),
        };
        assert_eq!(get_negated_int_value(&expr), Some(-42));
    }

    #[test]
    fn test_get_negated_int_value_zero() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert_eq!(get_negated_int_value(&expr), Some(0));
    }

    #[test]
    fn test_get_negated_int_value_not_int() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert_eq!(get_negated_int_value(&expr), None);
    }

    #[test]
    fn test_get_negated_float_value() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Float(3.15))),
        };
        assert_eq!(get_negated_float_value(&expr), Some(-3.15));
    }

    #[test]
    fn test_get_negated_float_value_not_float() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(42))),
        };
        assert_eq!(get_negated_float_value(&expr), None);
    }

    // ============================================================================
    // is_option_returning_* tests
    // ============================================================================

    #[test]
    fn test_is_option_returning_method() {
        assert!(is_option_returning_method("find"));
        assert!(is_option_returning_method("search"));
        assert!(is_option_returning_method("match"));
        assert!(is_option_returning_method("get"));
        assert!(is_option_returning_method("ok"));
        assert!(!is_option_returning_method("append"));
        assert!(!is_option_returning_method("push"));
    }

    #[test]
    fn test_is_option_returning_function() {
        assert!(is_option_returning_function("match"));
        assert!(is_option_returning_function("search"));
        assert!(is_option_returning_function("find"));
        assert!(!is_option_returning_function("len"));
        assert!(!is_option_returning_function("print"));
    }

    // ============================================================================
    // generate_* expression tests
    // ============================================================================

    #[test]
    fn test_generate_is_empty() {
        let operand: syn::Expr = syn::parse_quote! { list };
        let result = generate_is_empty(operand);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("is_empty"));
    }

    #[test]
    fn test_generate_is_none() {
        let operand: syn::Expr = syn::parse_quote! { opt };
        let result = generate_is_none(operand);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("is_none"));
    }

    #[test]
    fn test_generate_is_some() {
        let operand: syn::Expr = syn::parse_quote! { opt };
        let result = generate_is_some(operand);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("is_some"));
    }

    #[test]
    fn test_generate_not_is_empty() {
        let operand: syn::Expr = syn::parse_quote! { list };
        let result = generate_not_is_empty(operand);
        let s = result.to_token_stream().to_string();
        assert!(s.contains("!"));
        assert!(s.contains("is_empty"));
    }
}
