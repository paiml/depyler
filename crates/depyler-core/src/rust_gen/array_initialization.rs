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
/// - `range(end)` → `0..(end)`
/// - `range(start, end)` → `(start)..(end)`
/// - `range(start, end, step)` → `(start..end).step_by(step)` or `(end..start).rev().step_by(step)`
///
/// # DEPYLER-0905: Range expressions require parenthesization
/// When the end expression is a complex binary operation (e.g., `floor_div + 1`),
/// the `..` operator has higher precedence than `+`, causing parse errors like:
/// `for i in 0..{block} + 1 {` where the parser thinks `+ 1 {` starts the loop body.
/// Solution: Always wrap range bounds in parentheses for safety.
///
/// # Complexity: 3
pub fn convert_range_call(args: &[syn::Expr]) -> Result<syn::Expr> {
    match args.len() {
        1 => {
            let end = &args[0];
            // DEPYLER-0905: Parenthesize end to handle complex expressions like floor_div + 1
            Ok(parse_quote! { 0..(#end) })
        }
        2 => {
            let start = &args[0];
            let end = &args[1];
            // DEPYLER-0905: Parenthesize both bounds for complex expressions
            Ok(parse_quote! { (#start)..(#end) })
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

/// DEPYLER-0695: Convert array literals to Vec for consistent return types
///
/// Always uses vec![] to ensure Python list semantics (dynamically sized).
/// Previously used fixed arrays for small sizes, but this caused E0308 type
/// mismatches when functions declared Vec<T> return types.
///
/// - `zeros(5)` → `vec![0; 5]`
/// - `ones(5)` → `vec![1; 5]`
/// - `full(5, 42)` → `vec![42; 5]`
///
/// # Complexity: 4
pub fn convert_array_small_literal(
    ctx: &mut CodeGenContext,
    func: &str,
    args: &[HirExpr],
    size: i64,
) -> Result<syn::Expr> {
    let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
    // DEPYLER-E0282-FIX: Use i32 suffix to avoid type inference ambiguity
    // when used with py_add/py_sub/etc. which have multiple impl overloads
    match func {
        "zeros" => Ok(parse_quote! { vec![0i32; #size_lit] }),
        "ones" => Ok(parse_quote! { vec![1i32; #size_lit] }),
        "full" => {
            if args.len() >= 2 {
                let value = args[1].to_rust_expr(ctx)?;
                Ok(parse_quote! { vec![#value; #size_lit] })
            } else {
                bail!("full() requires a value argument");
            }
        }
        _ => unreachable!(),
    }
}

/// Convert large array literals (>32 elements) to Vec
///
/// - `zeros(100)` → `vec![0i32; 100]`
/// - `ones(100)` → `vec![1i32; 100]`
/// - `full(100, 42)` → `vec![42; 100]`
///
/// # Complexity: 4
pub fn convert_array_large_literal(
    ctx: &mut CodeGenContext,
    func: &str,
    args: &[HirExpr],
) -> Result<syn::Expr> {
    let size_expr = args[0].to_rust_expr(ctx)?;
    // DEPYLER-E0282-FIX: Use i32 suffix to avoid type inference ambiguity
    match func {
        "zeros" => Ok(parse_quote! { vec![0i32; #size_expr as usize] }),
        "ones" => Ok(parse_quote! { vec![1i32; #size_expr as usize] }),
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
/// - `zeros(n)` → `vec![0i32; n as usize]`
/// - `ones(n)` → `vec![1i32; n as usize]`
/// - `full(n, val)` → `vec![val; n as usize]`
///
/// # Complexity: 4
pub fn convert_array_dynamic_size(
    ctx: &mut CodeGenContext,
    func: &str,
    args: &[HirExpr],
) -> Result<syn::Expr> {
    let size_expr = args[0].to_rust_expr(ctx)?;
    // DEPYLER-E0282-FIX: Use i32 suffix to avoid type inference ambiguity
    match func {
        "zeros" => Ok(parse_quote! { vec![0i32; #size_expr as usize] }),
        "ones" => Ok(parse_quote! { vec![1i32; #size_expr as usize] }),
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

    // ========================================================================
    // Range Expression Tests
    // ========================================================================

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
        let args: Vec<syn::Expr> =
            vec![parse_quote! { 0 }, parse_quote! { 10 }, parse_quote! { 2 }];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("step_by") || result_str.contains("step"));
    }

    #[test]
    fn test_range_with_negative_step() {
        let args: Vec<syn::Expr> =
            vec![parse_quote! { 10 }, parse_quote! { 0 }, parse_quote! { -1 }];
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

    #[test]
    fn test_range_positive_step_direct() {
        let start: syn::Expr = parse_quote! { 0 };
        let end: syn::Expr = parse_quote! { 10 };
        let step: syn::Expr = parse_quote! { 2 };
        let result = convert_range_positive_step(&start, &end, &step).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("step_by"));
    }

    #[test]
    fn test_range_negative_step_direct() {
        let start: syn::Expr = parse_quote! { 10 };
        let end: syn::Expr = parse_quote! { 0 };
        let step: syn::Expr = parse_quote! { -1 };
        let result = convert_range_negative_step(&start, &end, &step).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("rev"));
        assert!(result_str.contains("abs"));
    }

    #[test]
    fn test_convert_range_with_step_dispatch() {
        // Test positive step dispatch
        let start: syn::Expr = parse_quote! { 0 };
        let end: syn::Expr = parse_quote! { 10 };
        let step_pos: syn::Expr = parse_quote! { 2 };
        let result_pos = convert_range_with_step(&start, &end, &step_pos).unwrap();
        let pos_str = quote::quote!(#result_pos).to_string();
        // Positive step should NOT have rev
        assert!(!pos_str.contains("rev"));

        // Test negative step dispatch
        let step_neg: syn::Expr = parse_quote! { -2 };
        let result_neg = convert_range_with_step(&start, &end, &step_neg).unwrap();
        let neg_str = quote::quote!(#result_neg).to_string();
        // Negative step SHOULD have rev
        assert!(neg_str.contains("rev"));
    }

    // ========================================================================
    // Additional Range Expression Tests
    // ========================================================================

    /// Test range with complex expressions as bounds
    #[test]
    fn test_range_complex_end() {
        let args: Vec<syn::Expr> = vec![parse_quote! { n + 1 }];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        // Should be parenthesized to handle complex expressions
        assert!(result_str.contains("0"));
    }

    /// Test range with variable bounds
    #[test]
    fn test_range_variable_bounds() {
        let args: Vec<syn::Expr> = vec![parse_quote! { start }, parse_quote! { end }];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("start"));
        assert!(result_str.contains("end"));
    }

    /// Test range with function call as bound
    #[test]
    fn test_range_function_call_bound() {
        let args: Vec<syn::Expr> = vec![parse_quote! { len(items) }];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("len"));
    }

    /// Test range with binary operation step
    #[test]
    fn test_range_binary_step() {
        let args: Vec<syn::Expr> = vec![
            parse_quote! { 0 },
            parse_quote! { 100 },
            parse_quote! { n * 2 },
        ];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        // Variable step uses step_by
        assert!(result_str.contains("step_by") || result_str.contains("step"));
    }

    /// Test that zero step generates panic code
    #[test]
    fn test_range_step_zero_protection() {
        let start: syn::Expr = parse_quote! { 0 };
        let end: syn::Expr = parse_quote! { 10 };
        let step: syn::Expr = parse_quote! { step };
        let result = convert_range_positive_step(&start, &end, &step).unwrap();
        let result_str = quote::quote!(#result).to_string();
        // Should have zero check
        assert!(result_str.contains("== 0") || result_str.contains("panic"));
    }

    /// Test negative step with abs() call
    #[test]
    fn test_range_negative_step_abs() {
        let start: syn::Expr = parse_quote! { 10 };
        let end: syn::Expr = parse_quote! { 0 };
        let step: syn::Expr = parse_quote! { -2 };
        let result = convert_range_negative_step(&start, &end, &step).unwrap();
        let result_str = quote::quote!(#result).to_string();
        // Should convert negative step using abs
        assert!(result_str.contains("abs"));
    }

    /// Test range with literal zero end
    #[test]
    fn test_range_literal_zero() {
        let args: Vec<syn::Expr> = vec![parse_quote! { 0 }];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("0"));
    }

    /// Test range with negative start
    #[test]
    fn test_range_negative_start() {
        let args: Vec<syn::Expr> = vec![parse_quote! { -5 }, parse_quote! { 5 }];
        let result = convert_range_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("-"));
        assert!(result_str.contains("5"));
    }
}
