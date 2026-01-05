//! String Module Code Generation - EXTREME TDD
//!
//! Handles Python `string` module method conversions to Rust.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python string module method calls to Rust
///
/// # Supported Methods
/// - capwords: Capitalize words in a string
///
/// # Complexity: 2 (match with 1 branch + default)
pub fn convert_string_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "capwords" => convert_capwords(&arg_exprs)?,
        _ => bail!("string.{} not implemented yet", method),
    };

    Ok(Some(result))
}

/// string.capwords(text) - Capitalize words in a string
fn convert_capwords(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("string.capwords() requires at least 1 argument (text)");
    }
    let text = &arg_exprs[0];

    // string.capwords(text) → text.split_whitespace().map(|w| {
    //     let mut c = w.chars();
    //     match c.next() {
    //         None => String::new(),
    //         Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
    //     }
    // }).collect::<Vec<_>>().join(" ")
    Ok(parse_quote! {
        #text.split_whitespace()
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        let mut result = first.to_uppercase().collect::<String>();
                        result.push_str(&chars.as_str().to_lowercase());
                        result
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    // ============================================
    // capwords() tests - 6 tests
    // ============================================

    #[test]
    fn test_convert_string_capwords() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("hello world".to_string()))];
        let result = convert_string_method("capwords", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("split_whitespace"));
        assert!(code.contains("to_uppercase"));
    }

    #[test]
    fn test_convert_string_capwords_variable() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("text".to_string())];
        let result = convert_string_method("capwords", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("text"));
    }

    #[test]
    fn test_convert_string_capwords_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_string_method("capwords", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 1 argument"));
    }

    #[test]
    fn test_convert_string_capwords_contains_join() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("s".to_string())];
        let result = convert_string_method("capwords", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("join"));
    }

    #[test]
    fn test_convert_string_capwords_contains_collect() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("input".to_string())];
        let result = convert_string_method("capwords", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("collect"));
    }

    #[test]
    fn test_convert_capwords_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![];
        let result = convert_capwords(&arg_exprs);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_capwords_direct_valid() {
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(my_string)];
        let result = convert_capwords(&arg_exprs);
        assert!(result.is_ok());
    }

    // ============================================
    // Unknown method tests - 3 tests
    // ============================================

    #[test]
    fn test_convert_string_unknown() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_string_method("unknown", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not implemented"));
    }

    #[test]
    fn test_convert_string_unknown_with_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("x".to_string())];
        let result = convert_string_method("template", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_string_digits() {
        // string.digits is a constant, not a function - should fail
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_string_method("digits", &args, &mut ctx);
        assert!(result.is_err());
    }
}
