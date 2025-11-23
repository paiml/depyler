//! Handle Python's round() builtin function
//!

use crate::hir::{HirExpr, Literal, Type, UnaryOp};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::Result;
use syn::parse_quote;

pub fn handle_round(args: &[HirExpr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if args.len() != 1 {
        anyhow::bail!("round() takes exactly 1 argument ({} given)", args.len());
    }

    let value_expr = args[0].to_rust_expr(ctx)?;

    // Check if the return type is float or int
    let returns_float = matches!(&ctx.current_return_type, Some(Type::Float));

    // For float literals, add type suffix to avoid ambiguity
    let needs_float_cast = match &args[0] {
        HirExpr::Literal(Literal::Float(_)) => true,
        HirExpr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => {
            matches!(**operand, HirExpr::Literal(Literal::Float(_)))
        }
        _ => false,
    };

    if returns_float {
        // Return float without casting
        if needs_float_cast {
            Ok(parse_quote! { (#value_expr as f64).round() })
        } else {
            // For variables, don't add unnecessary parens
            Ok(parse_quote! { #value_expr.round() })
        }
    } else {
        // Return int with cast to i32 (Python's round() returns int)
        // Cast to i32 to match the actual return type
        if needs_float_cast {
            Ok(parse_quote! { (#value_expr as f64).round() as i32 })
        } else {
            Ok(parse_quote! { #value_expr.round() as i32 })
        }
    }
}
