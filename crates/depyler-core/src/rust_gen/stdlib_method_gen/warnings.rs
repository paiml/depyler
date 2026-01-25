//! Warnings Module Code Generation - EXTREME TDD
//!
//! Handles Python `warnings` module method conversions to Rust.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python warnings module method calls to Rust
///
/// # Supported Methods
/// - warn: Print warning to stderr using eprintln!
///
/// # Complexity: 2 (match with 1 branch + default)
pub fn convert_warnings_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "warn" => convert_warn(&arg_exprs)?,
        _ => bail!("warnings.{} not implemented yet (available: warn)", method),
    };

    Ok(Some(result))
}

/// warnings.warn(message) - Print warning to stderr
fn convert_warn(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("warnings.warn() requires at least 1 argument");
    }
    let message = &arg_exprs[0];

    Ok(parse_quote! {
        eprintln!("Warning: {}", #message)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    // ============================================
    // warn() tests - 6 tests
    // ============================================

    #[test]
    fn test_convert_warnings_warn() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(
            "Test warning".to_string(),
        ))];
        let result = convert_warnings_method("warn", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("eprintln"));
        assert!(code.contains("Warning"));
    }

    #[test]
    fn test_convert_warnings_warn_variable() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("msg".to_string())];
        let result = convert_warnings_method("warn", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_warnings_warn_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_warnings_method("warn", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 1 argument"));
    }

    #[test]
    fn test_convert_warn_direct() {
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!("test message")];
        let result = convert_warn(&arg_exprs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_warn_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![];
        let result = convert_warn(&arg_exprs);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_warnings_warn_contains_stderr() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("warning_text".to_string())];
        let result = convert_warnings_method("warn", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        // eprintln! writes to stderr
        assert!(code.contains("eprintln"));
    }

    // ============================================
    // Unknown method tests - 3 tests
    // ============================================

    #[test]
    fn test_convert_warnings_unknown() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_warnings_method("unknown", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not implemented"));
    }

    #[test]
    fn test_convert_warnings_filterwarnings() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("action".to_string())];
        let result = convert_warnings_method("filterwarnings", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_warnings_simplefilter() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_warnings_method("simplefilter", &args, &mut ctx);
        assert!(result.is_err());
    }
}
