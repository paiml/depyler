//! Functools Module Code Generation - EXTREME TDD
//!
//! Handles Python `functools` module method conversions to Rust.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python functools module method calls to Rust
///
/// # Supported Methods
/// - reduce: Fold/reduce operation using Iterator::fold()
///
/// # Complexity: 2 (match with 1 branch + default)
pub fn convert_functools_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "reduce" => convert_reduce(&arg_exprs)?,
        _ => bail!(
            "functools.{} not implemented yet (available: reduce)",
            method
        ),
    };

    Ok(Some(result))
}

/// functools.reduce(function, iterable, [initial]) - Reduce operation
fn convert_reduce(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("functools.reduce() requires at least 2 arguments");
    }
    let function = &arg_exprs[0];
    let iterable = &arg_exprs[1];

    if arg_exprs.len() >= 3 {
        // With initial value
        let initial = &arg_exprs[2];
        Ok(parse_quote! {
            {
                let func = #function;
                let items = #iterable;
                let init = #initial;
                items.into_iter().fold(init, func)
            }
        })
    } else {
        // Without initial value - use first element
        Ok(parse_quote! {
            {
                let func = #function;
                let mut items = (#iterable).into_iter();
                let init = items.next().expect("reduce() of empty sequence with no initial value");
                items.fold(init, func)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    // ============================================
    // reduce() tests - 8 tests
    // ============================================

    #[test]
    fn test_convert_functools_reduce_no_initial() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("add".to_string()),
            HirExpr::Var("numbers".to_string()),
        ];
        let result = convert_functools_method("reduce", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("fold"));
        assert!(code.contains("next"));
    }

    #[test]
    fn test_convert_functools_reduce_with_initial() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("add".to_string()),
            HirExpr::Var("numbers".to_string()),
            HirExpr::Literal(Literal::Int(0)),
        ];
        let result = convert_functools_method("reduce", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("fold"));
        assert!(!code.contains("next")); // no next() when initial value provided
    }

    #[test]
    fn test_convert_functools_reduce_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("func".to_string())];
        let result = convert_functools_method("reduce", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_functools_reduce_lambda() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("lambda_fn".to_string()),
            HirExpr::Var("items".to_string()),
        ];
        let result = convert_functools_method("reduce", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_functools_reduce_with_string_initial() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("concat".to_string()),
            HirExpr::Var("strings".to_string()),
            HirExpr::Literal(Literal::String("".to_string())),
        ];
        let result = convert_functools_method("reduce", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_reduce_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(f)];
        let result = convert_reduce(&arg_exprs);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_reduce_direct_two_args() {
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(add), parse_quote!(nums)];
        let result = convert_reduce(&arg_exprs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_reduce_direct_three_args() {
        let arg_exprs: Vec<syn::Expr> =
            vec![parse_quote!(add), parse_quote!(nums), parse_quote!(0)];
        let result = convert_reduce(&arg_exprs);
        assert!(result.is_ok());
    }

    // ============================================
    // Unknown method tests - 3 tests
    // ============================================

    #[test]
    fn test_convert_functools_unknown() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_functools_method("unknown", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not implemented"));
    }

    #[test]
    fn test_convert_functools_partial() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("f".to_string())];
        let result = convert_functools_method("partial", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_functools_lru_cache() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_functools_method("lru_cache", &args, &mut ctx);
        assert!(result.is_err());
    }
}
