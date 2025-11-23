//! Handle Python's pow() builtin function and ** operator
//!
//! Python: pow(base, exp) → Rust: base.pow(exp) or base.powf(exp)
//! Python: base ** exp → Rust: base.pow(exp) or base.powf(exp)
//! Uses checked_pow for runtime overflow detection with variables

use crate::hir::{HirExpr, Literal, UnaryOp};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::Result;
use syn::parse_quote;

pub fn handle_pow(args: &[HirExpr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if args.len() != 2 {
        anyhow::bail!("pow() takes exactly 2 arguments ({} given)", args.len());
    }

    let base_expr = args[0].to_rust_expr(ctx)?;
    let exp_expr = args[1].to_rust_expr(ctx)?;

    // Determine if base is int or float type
    let base_is_float = ctx.is_expr_float_type(&args[0]);
    let exp_is_float = ctx.is_expr_float_type(&args[1]);

    // Check if base and exponent are literals
    let base_is_literal =
        matches!(&args[0], HirExpr::Literal(_)) || matches!(&args[0], HirExpr::Unary { op: UnaryOp::Neg, .. });
    let exp_is_literal =
        matches!(&args[1], HirExpr::Literal(_)) || matches!(&args[1], HirExpr::Unary { op: UnaryOp::Neg, .. });

    if base_is_float || exp_is_float {
        // Use powf for floats - need explicit type for literals
        let base_final = if base_is_literal {
            parse_quote! { (#base_expr as f64) }
        } else {
            base_expr
        };
        let exp_final = if exp_is_literal {
            parse_quote! { #exp_expr as f64 }
        } else {
            exp_expr
        };
        Ok(parse_quote! { #base_final.powf(#exp_final) })
    } else if base_is_literal && exp_is_literal {
        // Both literals and int types - add type suffix to base, cast exp to u32
        // Extract the base value and create a typed literal
        let base_with_type: syn::Expr = match &args[0] {
            HirExpr::Literal(Literal::Int(val)) => {
                let val_str = val.to_string();
                let ident = syn::LitInt::new(&format!("{}_i32", val_str), proc_macro2::Span::call_site());
                parse_quote! { #ident }
            }
            HirExpr::Unary {
                op: UnaryOp::Neg,
                operand,
            } => {
                if let HirExpr::Literal(Literal::Int(val)) = &**operand {
                    let val_str = val.to_string();
                    let ident = syn::LitInt::new(&format!("{}_i32", val_str), proc_macro2::Span::call_site());
                    parse_quote! { -#ident }
                } else {
                    parse_quote! { (#base_expr as i32) }
                }
            }
            _ => parse_quote! { (#base_expr as i32) },
        };

        Ok(parse_quote! { #base_with_type.pow(#exp_expr as u32) })
    } else if exp_is_literal {
        // Base is variable, exp is literal
        // If base is integer type, use .pow(), otherwise use checked_pow
        // We already handled float case above, so this is integer
        Ok(parse_quote! { #base_expr.pow(#exp_expr as u32) })
    } else {
        // Both are variables - use checked_pow for safety
        Ok(parse_quote! { #base_expr.checked_pow(#exp_expr as u32).expect("Power operation overflowed") })
    }
}
