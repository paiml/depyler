//! Handle Python's min() and max() builtin functions
//!
//! Python: max(a, b) → Rust: std::cmp::max(a, b) or a.max(b) for floats
//! Python: max(a, b, c) → Rust: std::cmp::max(std::cmp::max(a, b), c)
//! Python: max([1, 2, 3]) → Rust: *list.iter().max().unwrap()
//! Python: min() follows the same patterns

use crate::hir::{HirExpr, Literal, Type};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::Result;
use syn::parse_quote;

pub fn handle_max(args: &[HirExpr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if args.is_empty() {
        anyhow::bail!("max expected at least 1 argument, got 0");
    }

    if args.len() == 1 {
        // max(iterable) → iterable.iter().max().unwrap() or use fold for floats
        return handle_max_iterable(&args[0], ctx);
    }

    if args.len() == 2 {
        // max(a, b) → std::cmp::max(a, b) or a.max(b) for floats
        return handle_max_two(args, ctx);
    }

    // max(a, b, c, ...) → chain std::cmp::max or .max() calls
    handle_max_multiple(args, ctx)
}

pub fn handle_min(args: &[HirExpr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if args.is_empty() {
        anyhow::bail!("min expected at least 1 argument, got 0");
    }

    if args.len() == 1 {
        // min(iterable) → iterable.iter().min().unwrap() or use fold for floats
        return handle_min_iterable(&args[0], ctx);
    }

    if args.len() == 2 {
        // min(a, b) → std::cmp::min(a, b) or a.min(b) for floats
        return handle_min_two(args, ctx);
    }

    // min(a, b, c, ...) → chain std::cmp::min or .min() calls
    handle_min_multiple(args, ctx)
}

fn handle_max_two(args: &[HirExpr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    let mut arg1 = args[0].to_rust_expr(ctx)?;
    let mut arg2 = args[1].to_rust_expr(ctx)?;

    // Check if arguments are floats (which don't implement Ord)
    let is_float = ctx.is_expr_float_type(&args[0]) || ctx.is_expr_float_type(&args[1]);

    if is_float {
        // Cast float literals to f64 to avoid ambiguous type errors
        // Wrap in parens to allow method call after cast
        if matches!(args[0], HirExpr::Literal(Literal::Float(_))) {
            arg1 = parse_quote! { (#arg1 as f64) };
        }
        if matches!(args[1], HirExpr::Literal(Literal::Float(_))) {
            arg2 = parse_quote! { (#arg2 as f64) };
        }
        // Use f64::max method for floats
        Ok(parse_quote! { #arg1.max(#arg2) })
    } else {
        // Use std::cmp::max for types implementing Ord
        Ok(parse_quote! { std::cmp::max(#arg1, #arg2) })
    }
}

fn handle_min_two(args: &[HirExpr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    let mut arg1 = args[0].to_rust_expr(ctx)?;
    let mut arg2 = args[1].to_rust_expr(ctx)?;

    // Check if arguments are floats (which don't implement Ord)
    let is_float = ctx.is_expr_float_type(&args[0]) || ctx.is_expr_float_type(&args[1]);

    if is_float {
        // Cast float literals to f64 to avoid ambiguous type errors
        // Wrap in parens to allow method call after cast
        if matches!(args[0], HirExpr::Literal(Literal::Float(_))) {
            arg1 = parse_quote! { (#arg1 as f64) };
        }
        if matches!(args[1], HirExpr::Literal(Literal::Float(_))) {
            arg2 = parse_quote! { (#arg2 as f64) };
        }
        // Use f64::min method for floats
        Ok(parse_quote! { #arg1.min(#arg2) })
    } else {
        // Use std::cmp::min for types implementing Ord
        Ok(parse_quote! { std::cmp::min(#arg1, #arg2) })
    }
}

fn handle_max_multiple(args: &[HirExpr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    // Check if any argument is a float
    let is_float = args.iter().any(|arg| ctx.is_expr_float_type(arg));

    if is_float {
        // Chain .max() calls: a.max(b).max(c)
        // Cast the first literal to f64 to avoid ambiguous type, wrap in parens for method call
        let mut result = args[0].to_rust_expr(ctx)?;
        if matches!(args[0], HirExpr::Literal(Literal::Float(_))) {
            result = parse_quote! { (#result as f64) };
        }

        for arg in &args[1..] {
            // Don't wrap method arguments in extra parens - they're already in the right context
            let arg_expr = arg.to_rust_expr(ctx)?;
            result = parse_quote! { #result.max(#arg_expr) };
        }
        Ok(result)
    } else {
        // Chain std::cmp::max calls: std::cmp::max(std::cmp::max(a, b), c)
        let mut result = args[0].to_rust_expr(ctx)?;
        for arg in &args[1..] {
            let arg_expr = arg.to_rust_expr(ctx)?;
            result = parse_quote! { std::cmp::max(#result, #arg_expr) };
        }
        Ok(result)
    }
}

fn handle_min_multiple(args: &[HirExpr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    // Check if any argument is a float
    let is_float = args.iter().any(|arg| ctx.is_expr_float_type(arg));

    if is_float {
        // Chain .min() calls: a.min(b).min(c)
        // Cast the first literal to f64 to avoid ambiguous type, wrap in parens for method call
        let mut result = args[0].to_rust_expr(ctx)?;
        if matches!(args[0], HirExpr::Literal(Literal::Float(_))) {
            result = parse_quote! { (#result as f64) };
        }

        for arg in &args[1..] {
            // Don't wrap method arguments in extra parens - they're already in the right context
            let arg_expr = arg.to_rust_expr(ctx)?;
            result = parse_quote! { #result.min(#arg_expr) };
        }
        Ok(result)
    } else {
        // Chain std::cmp::min calls: std::cmp::min(std::cmp::min(a, b), c)
        let mut result = args[0].to_rust_expr(ctx)?;
        for arg in &args[1..] {
            let arg_expr = arg.to_rust_expr(ctx)?;
            result = parse_quote! { std::cmp::min(#result, #arg_expr) };
        }
        Ok(result)
    }
}

fn handle_max_iterable(arg: &HirExpr, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    let iter_expr = arg.to_rust_expr(ctx)?;

    // Check if we're dealing with floats (which don't implement Ord)
    let is_float = match arg {
        HirExpr::Var(var_name) => {
            if let Some(Type::List(element_type)) = ctx.var_types.get(var_name) {
                matches!(element_type.as_ref(), Type::Float)
            } else {
                false
            }
        }
        _ => false,
    };

    if is_float {
        // For float lists, use fold with f64::NEG_INFINITY
        Ok(parse_quote! {
            #iter_expr.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
        })
    } else {
        // For other types, use standard .max()
        Ok(parse_quote! { *#iter_expr.iter().max().unwrap() })
    }
}

fn handle_min_iterable(arg: &HirExpr, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    let iter_expr = arg.to_rust_expr(ctx)?;

    // Check if we're dealing with floats (which don't implement Ord)
    let is_float = match arg {
        HirExpr::Var(var_name) => {
            if let Some(Type::List(element_type)) = ctx.var_types.get(var_name) {
                matches!(element_type.as_ref(), Type::Float)
            } else {
                false
            }
        }
        _ => false,
    };

    if is_float {
        // For float lists, use fold with f64::INFINITY
        Ok(parse_quote! {
            #iter_expr.iter().fold(f64::INFINITY, |a, &b| a.min(b))
        })
    } else {
        // For other types, use standard .min()
        Ok(parse_quote! { *#iter_expr.iter().min().unwrap() })
    }
}
