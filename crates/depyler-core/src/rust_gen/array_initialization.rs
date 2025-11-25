//! Array initialization and range expression code generation
//!
//! This module handles Python array initialization functions and range expressions:
//! - zeros(n), ones(n), full(n, value) - numpy-style array creation
//! - range(end), range(start, end), range(start, end, step)
//!
//! Extracted from expr_gen.rs as part of DEPYLER-REFACTOR-001 (God File split)
//!
//! # DEPYLER-REFACTOR-001 Traceability
//! - Original location: expr_gen.rs lines 1663-1815
//! - Extraction date: 2025-11-25
//! - Tests: tests/refactor_array_initialization_test.rs

use crate::hir::{HirExpr, Literal};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

// ============================================================================
// Range Expression Functions
// ============================================================================

/// Convert Python range() call to Rust range expression
///
/// Python range() has three forms:
/// - `range(end)` → `0..end`
/// - `range(start, end)` → `start..end`
/// - `range(start, end, step)` → `(start..end).step_by(step)` or `(end..start).rev().step_by(step)`
///
/// # Complexity: 3
pub fn convert_range_call(args: &[syn::Expr]) -> Result<syn::Expr> {
    match args.len() {
        1 => {
            let end = &args[0];
            Ok(parse_quote! { 0..#end })
        }
        2 => {
            let start = &args[0];
            let end = &args[1];
            Ok(parse_quote! { #start..#end })
        }
        3 => convert_range_with_step(&args[0], &args[1], &args[2]),
        _ => bail!("Invalid number of arguments for range()"),
    }
}

/// Convert range with step argument
///
/// Detects if step is negative by checking for unary minus operator.
/// Routes to appropriate helper based on step direction.
///
/// # Complexity: 3
pub fn convert_range_with_step(
    start: &syn::Expr,
    end: &syn::Expr,
    step: &syn::Expr,
) -> Result<syn::Expr> {
    // Check if step is negative by looking at the expression
    let is_negative_step =
        matches!(step, syn::Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)));

    if is_negative_step {
        convert_range_negative_step(start, end, step)
    } else {
        convert_range_positive_step(start, end, step)
    }
}

/// Convert range with negative step
///
/// Python: range(10, 0, -1) → Rust: (0..10).rev().step_by(1)
///
/// DEPYLER-0313: Cast to i32 before abs() to avoid ambiguous numeric type
/// DEPYLER-0316: Always use .step_by() for consistent iterator type
///
/// # Complexity: 4
pub fn convert_range_negative_step(
    start: &syn::Expr,
    end: &syn::Expr,
    step: &syn::Expr,
) -> Result<syn::Expr> {
    // For negative steps, we need to reverse the range
    // Python: range(10, 0, -1) → Rust: (0..10).rev()
    Ok(parse_quote! {
        {
            // DEPYLER-0313: Cast to i32 before abs() to avoid ambiguous numeric type
            let step = (#step as i32).abs() as usize;
            if step == 0 {
                panic!("range() arg 3 must not be zero");
            }
            // DEPYLER-0316: Always use .step_by() for consistent iterator type
            // This avoids if/else branches returning different types:
            // - Rev<Range<i32>> vs StepBy<Rev<Range<i32>>>
            // Using step.max(1) ensures step is never 0 (already checked above)
            (#end..#start).rev().step_by(step.max(1))
        }
    })
}

/// Convert range with positive step
///
/// Python: range(0, 10, 2) → Rust: (0..10).step_by(2)
///
/// Includes zero-step protection at runtime.
///
/// # Complexity: 3
pub fn convert_range_positive_step(
    start: &syn::Expr,
    end: &syn::Expr,
    step: &syn::Expr,
) -> Result<syn::Expr> {
    // Positive step - check for zero
    Ok(parse_quote! {
        {
            let step = #step as usize;
            if step == 0 {
                panic!("range() arg 3 must not be zero");
            }
            (#start..#end).step_by(step)
        }
    })
}

// ============================================================================
// Array Initialization Functions
// ============================================================================

/// Convert numpy-style array initialization call
///
/// Handles zeros(n), ones(n), full(n, value) patterns.
/// Routes to specialized handlers based on size:
/// - Small literals (≤32): Fixed-size arrays `[value; N]`
/// - Large literals (>32): Vec `vec![value; n]`
/// - Dynamic size: Vec `vec![value; n]`
///
/// # Complexity: 5
pub fn convert_array_init_call(
    ctx: &mut CodeGenContext,
    func: &str,
    args: &[HirExpr],
    _arg_exprs: &[syn::Expr],
) -> Result<syn::Expr> {
    // Handle zeros(n), ones(n), full(n, value) patterns
    if args.is_empty() {
        bail!("{} requires at least one argument", func);
    }

    // Extract size from first argument if it's a literal
    if let HirExpr::Literal(Literal::Int(size)) = &args[0] {
        if *size > 0 && *size <= 32 {
            convert_array_small_literal(ctx, func, args, *size)
        } else {
            convert_array_large_literal(ctx, func, args)
        }
    } else {
        convert_array_dynamic_size(ctx, func, args)
    }
}

/// Convert small array literals (≤32 elements) to fixed-size arrays
///
/// - `zeros(5)` → `[0; 5]`
/// - `ones(5)` → `[1; 5]`
/// - `full(5, 42)` → `[42; 5]`
///
/// # Complexity: 4
pub fn convert_array_small_literal(
    ctx: &mut CodeGenContext,
    func: &str,
    args: &[HirExpr],
    size: i64,
) -> Result<syn::Expr> {
    let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
    match func {
        "zeros" => Ok(parse_quote! { [0; #size_lit] }),
        "ones" => Ok(parse_quote! { [1; #size_lit] }),
        "full" => {
            if args.len() >= 2 {
                let value = args[1].to_rust_expr(ctx)?;
                Ok(parse_quote! { [#value; #size_lit] })
            } else {
                bail!("full() requires a value argument");
            }
        }
        _ => unreachable!(),
    }
}

/// Convert large array literals (>32 elements) to Vec
///
/// - `zeros(100)` → `vec![0; 100]`
/// - `ones(100)` → `vec![1; 100]`
/// - `full(100, 42)` → `vec![42; 100]`
///
/// # Complexity: 4
pub fn convert_array_large_literal(
    ctx: &mut CodeGenContext,
    func: &str,
    args: &[HirExpr],
) -> Result<syn::Expr> {
    let size_expr = args[0].to_rust_expr(ctx)?;
    match func {
        "zeros" => Ok(parse_quote! { vec![0; #size_expr as usize] }),
        "ones" => Ok(parse_quote! { vec![1; #size_expr as usize] }),
        "full" => {
            if args.len() >= 2 {
                let value = args[1].to_rust_expr(ctx)?;
                Ok(parse_quote! { vec![#value; #size_expr as usize] })
            } else {
                bail!("full() requires a value argument");
            }
        }
        _ => unreachable!(),
    }
}

/// Convert dynamic-sized array initialization to Vec
///
/// When size is a variable/expression, always use Vec.
///
/// - `zeros(n)` → `vec![0; n as usize]`
/// - `ones(n)` → `vec![1; n as usize]`
/// - `full(n, val)` → `vec![val; n as usize]`
///
/// # Complexity: 4
pub fn convert_array_dynamic_size(
    ctx: &mut CodeGenContext,
    func: &str,
    args: &[HirExpr],
) -> Result<syn::Expr> {
    let size_expr = args[0].to_rust_expr(ctx)?;
    match func {
        "zeros" => Ok(parse_quote! { vec![0; #size_expr as usize] }),
        "ones" => Ok(parse_quote! { vec![1; #size_expr as usize] }),
        "full" => {
            if args.len() >= 2 {
                let value = args[1].to_rust_expr(ctx)?;
                Ok(parse_quote! { vec![#value; #size_expr as usize] })
            } else {
                bail!("full() requires a value argument");
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_single_arg() {
        let args: Vec<syn::Expr> = vec![parse_quote! { 5 }];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("0"));
        assert!(result_str.contains("5"));
    }

    #[test]
    fn test_range_two_args() {
        let args: Vec<syn::Expr> = vec![parse_quote! { 2 }, parse_quote! { 7 }];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("2"));
        assert!(result_str.contains("7"));
    }

    #[test]
    fn test_range_with_positive_step() {
        let args: Vec<syn::Expr> = vec![parse_quote! { 0 }, parse_quote! { 10 }, parse_quote! { 2 }];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("step_by") || result_str.contains("step"));
    }

    #[test]
    fn test_range_with_negative_step() {
        let args: Vec<syn::Expr> = vec![
            parse_quote! { 10 },
            parse_quote! { 0 },
            parse_quote! { -1 },
        ];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("rev"));
    }

    #[test]
    fn test_range_invalid_args() {
        let args: Vec<syn::Expr> = vec![];
        assert!(convert_range_call(&args).is_err());

        let too_many: Vec<syn::Expr> = vec![
            parse_quote! { 0 },
            parse_quote! { 10 },
            parse_quote! { 2 },
            parse_quote! { 3 },
        ];
        assert!(convert_range_call(&too_many).is_err());
    }
}
