//! Operator conversion functions for direct rules
//!
//! Converts HIR operators (binary, arithmetic, comparison, logical, bitwise)
//! to their Rust `syn` equivalents. Also contains literal conversion.

use crate::hir::*;
use anyhow::{bail, Result};
use syn::parse_quote;

/// Check if an expression is a len() call
pub(crate) fn is_len_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, args , ..} if func == "len" && args.len() == 1)
}

pub(crate) fn convert_literal(lit: &Literal) -> syn::Expr {
    match lit {
        Literal::Int(n) => {
            let lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::Float(f) => {
            // DEPYLER-0738: Ensure float literals always have a decimal point
            // f64::to_string() outputs "0" for 0.0, which parses as integer
            let s = f.to_string();
            let float_str = if s.contains('.') || s.contains('e') || s.contains('E') {
                s
            } else {
                format!("{}.0", s)
            };
            let lit = syn::LitFloat::new(&float_str, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::String(s) => {
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            parse_quote! { #lit.to_string() }
        }
        Literal::Bytes(b) => {
            let byte_str = syn::LitByteStr::new(b, proc_macro2::Span::call_site());
            parse_quote! { #byte_str }
        }
        Literal::Bool(b) => {
            let lit = syn::LitBool::new(*b, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        // DEPYLER-1037: Literal::None should map to Rust's None (for Option types)
        // Python: return None -> Rust: return None (when return type is Option<T>)
        Literal::None => parse_quote! { None },
    }
}

/// Convert HIR binary operators to Rust binary operators
pub(crate) fn convert_binop(op: BinOp) -> Result<syn::BinOp> {
    match op {
        // Arithmetic operators
        BinOp::Add
        | BinOp::Sub
        | BinOp::Mul
        | BinOp::Div
        | BinOp::Mod
        | BinOp::FloorDiv
        | BinOp::Pow => convert_arithmetic_op(op),

        // Comparison operators
        BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq => {
            convert_comparison_op(op)
        }

        // Logical operators
        BinOp::And | BinOp::Or => convert_logical_op(op),

        // Bitwise operators
        BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::LShift | BinOp::RShift => {
            convert_bitwise_op(op)
        }

        // Special membership operators
        BinOp::In | BinOp::NotIn => {
            bail!("in/not in operators should be handled by convert_binary")
        }
    }
}

pub(crate) fn convert_arithmetic_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        Add => Ok(parse_quote! { + }),
        Sub => Ok(parse_quote! { - }),
        Mul => Ok(parse_quote! { * }),
        Div => Ok(parse_quote! { / }),
        Mod => Ok(parse_quote! { % }),
        FloorDiv => {
            // Floor division requires special handling - it's not implemented as an operator
            // but handled in convert_binary for proper Python semantics
            bail!("Floor division handled in convert_binary with Python semantics")
        }
        Pow => bail!("Power operator handled in convert_binary with type-specific logic"),
        _ => bail!("Invalid operator {:?} for arithmetic conversion", op),
    }
}

pub(crate) fn convert_comparison_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        Eq => Ok(parse_quote! { == }),
        NotEq => Ok(parse_quote! { != }),
        Lt => Ok(parse_quote! { < }),
        LtEq => Ok(parse_quote! { <= }),
        Gt => Ok(parse_quote! { > }),
        GtEq => Ok(parse_quote! { >= }),
        _ => bail!("Invalid operator {:?} for comparison conversion", op),
    }
}

pub(crate) fn convert_logical_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        And => Ok(parse_quote! { && }),
        Or => Ok(parse_quote! { || }),
        _ => bail!("Invalid operator {:?} for logical conversion", op),
    }
}

pub(crate) fn convert_bitwise_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        BitAnd => Ok(parse_quote! { & }),
        BitOr => Ok(parse_quote! { | }),
        BitXor => Ok(parse_quote! { ^ }),
        LShift => Ok(parse_quote! { << }),
        RShift => Ok(parse_quote! { >> }),
        _ => bail!("Invalid operator {:?} for bitwise conversion", op),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    #[test]
    fn test_is_len_call_true() {
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_len_call(&expr));
    }

    #[test]
    fn test_is_len_call_false_other_func() {
        let expr = HirExpr::Call {
            func: "str".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(!is_len_call(&expr));
    }

    #[test]
    fn test_convert_literal_int() {
        let result = convert_literal(&Literal::Int(42));
        assert!(!result.to_token_stream().is_empty());
    }

    #[test]
    fn test_convert_literal_float() {
        let result = convert_literal(&Literal::Float(3.14));
        assert!(!result.to_token_stream().is_empty());
    }

    #[test]
    fn test_convert_literal_string() {
        let result = convert_literal(&Literal::String("hello".to_string()));
        assert!(!result.to_token_stream().is_empty());
    }

    #[test]
    fn test_convert_literal_bool() {
        let result = convert_literal(&Literal::Bool(true));
        assert!(!result.to_token_stream().is_empty());
    }

    #[test]
    fn test_convert_literal_none() {
        let result = convert_literal(&Literal::None);
        assert!(!result.to_token_stream().is_empty());
    }

    #[test]
    fn test_convert_binop_all_arithmetic() {
        assert!(convert_binop(BinOp::Add).is_ok());
        assert!(convert_binop(BinOp::Sub).is_ok());
        assert!(convert_binop(BinOp::Mul).is_ok());
        assert!(convert_binop(BinOp::Div).is_ok());
        assert!(convert_binop(BinOp::Mod).is_ok());
    }

    #[test]
    fn test_convert_binop_floor_div_errors() {
        assert!(convert_binop(BinOp::FloorDiv).is_err());
    }

    #[test]
    fn test_convert_binop_pow_errors() {
        assert!(convert_binop(BinOp::Pow).is_err());
    }

    #[test]
    fn test_convert_binop_comparison() {
        assert!(convert_binop(BinOp::Eq).is_ok());
        assert!(convert_binop(BinOp::NotEq).is_ok());
        assert!(convert_binop(BinOp::Lt).is_ok());
        assert!(convert_binop(BinOp::LtEq).is_ok());
        assert!(convert_binop(BinOp::Gt).is_ok());
        assert!(convert_binop(BinOp::GtEq).is_ok());
    }

    #[test]
    fn test_convert_binop_logical() {
        assert!(convert_binop(BinOp::And).is_ok());
        assert!(convert_binop(BinOp::Or).is_ok());
    }

    #[test]
    fn test_convert_binop_bitwise() {
        assert!(convert_binop(BinOp::BitAnd).is_ok());
        assert!(convert_binop(BinOp::BitOr).is_ok());
        assert!(convert_binop(BinOp::BitXor).is_ok());
        assert!(convert_binop(BinOp::LShift).is_ok());
        assert!(convert_binop(BinOp::RShift).is_ok());
    }

    #[test]
    fn test_convert_binop_in_errors() {
        assert!(convert_binop(BinOp::In).is_err());
        assert!(convert_binop(BinOp::NotIn).is_err());
    }

    #[test]
    fn test_convert_arithmetic_op_invalid() {
        assert!(convert_arithmetic_op(BinOp::Eq).is_err());
    }

    #[test]
    fn test_convert_comparison_op_invalid() {
        assert!(convert_comparison_op(BinOp::Add).is_err());
    }

    #[test]
    fn test_convert_logical_op_invalid() {
        assert!(convert_logical_op(BinOp::Add).is_err());
    }

    #[test]
    fn test_convert_bitwise_op_invalid() {
        assert!(convert_bitwise_op(BinOp::Add).is_err());
    }
}
