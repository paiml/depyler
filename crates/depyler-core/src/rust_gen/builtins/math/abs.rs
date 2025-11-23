//! Handle Python's abs() builtin function
//!
//! Python: abs(value) → Rust: value.abs()
//! Handles integers, floats, and expressions

use crate::hir::{HirExpr, Literal, UnaryOp};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::Result;
use syn::parse_quote;

pub fn handle_abs(args: &[HirExpr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if args.len() != 1 {
        anyhow::bail!("abs() takes exactly 1 argument ({} given)", args.len());
    }

    let value_expr = args[0].to_rust_expr(ctx)?;

    // For numeric literals, cast to concrete type and call .abs()
    // For variables and expressions, just call .abs() directly
    let abs_expr = match &args[0] {
        HirExpr::Literal(Literal::Int(_)) => {
            parse_quote! { (#value_expr as i32).abs() }
        }
        HirExpr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => {
            // For negative integer literals like -42, cast to i32 before .abs()
            if matches!(**operand, HirExpr::Literal(Literal::Int(_))) {
                parse_quote! { (#value_expr as i32).abs() }
            } else if matches!(**operand, HirExpr::Literal(Literal::Float(_))) {
                parse_quote! { (#value_expr as f64).abs() }
            } else {
                parse_quote! { (#value_expr).abs() }
            }
        }
        HirExpr::Literal(Literal::Float(_)) => {
            parse_quote! { (#value_expr as f64).abs() }
        }
        HirExpr::Var(_) => {
            // For variables, don't wrap in parentheses or cast
            parse_quote! { #value_expr.abs() }
        }
        HirExpr::Binary { .. } => {
            // For binary operations like (a - b), keep parentheses
            parse_quote! { (#value_expr).abs() }
        }
        _ => {
            // For other expressions, wrap in parentheses
            parse_quote! { (#value_expr).abs() }
        }
    };

    Ok(abs_expr)
}
