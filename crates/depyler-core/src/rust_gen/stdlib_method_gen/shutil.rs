//! Shutil Module Code Generation - EXTREME TDD
//!
//! Handles Python `shutil` module method conversions to Rust std::fs.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python shutil module method calls to Rust std::fs
///
/// # Supported Methods
/// - copy, copy2: File copying
/// - move: File/directory moving
/// - rmtree: Recursive directory removal
/// - copytree: Recursive directory copy
/// - which: Command lookup
///
/// # Complexity: 6 (match with 6 branches)
pub fn convert_shutil_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "copy" | "copy2" => convert_copy(method, &arg_exprs)?,
        "move" | "r#move" => convert_move(&arg_exprs)?,
        "rmtree" => convert_rmtree(&arg_exprs)?,
        "copytree" => convert_copytree(&arg_exprs)?,
        "which" => convert_which(&arg_exprs)?,
        _ => bail!("shutil.{} not implemented yet", method),
    };

    Ok(Some(result))
}

/// shutil.copy/copy2(src, dst)
fn convert_copy(method: &str, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("shutil.{}() requires 2 arguments (src, dst)", method);
    }
    let src = &arg_exprs[0];
    let dst = &arg_exprs[1];
    Ok(parse_quote! {
        {
            std::fs::copy(&#src, &#dst).unwrap();
            #dst.clone()
        }
    })
}

/// shutil.move(src, dst)
fn convert_move(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("shutil.move() requires 2 arguments (src, dst)");
    }
    let src = &arg_exprs[0];
    let dst = &arg_exprs[1];
    Ok(parse_quote! {
        {
            std::fs::rename(&#src, &#dst).unwrap();
            #dst.clone()
        }
    })
}

/// shutil.rmtree(path)
fn convert_rmtree(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("shutil.rmtree() requires 1 argument (path)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { std::fs::remove_dir_all(&#path).unwrap() })
}

/// shutil.copytree(src, dst)
fn convert_copytree(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("shutil.copytree() requires 2 arguments (src, dst)");
    }
    let src = &arg_exprs[0];
    let dst = &arg_exprs[1];
    Ok(parse_quote! {
        {
            fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
                std::fs::create_dir_all(dst)?;
                for entry in std::fs::read_dir(src)? {
                    let entry = entry?;
                    let file_type = entry.file_type()?;
                    if file_type.is_dir() {
                        copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
                    } else {
                        std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
                    }
                }
                Ok(())
            }
            copy_dir_all(std::path::Path::new(&#src), std::path::Path::new(&#dst)).unwrap();
            #dst.clone()
        }
    })
}

/// shutil.which(cmd)
fn convert_which(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("shutil.which() requires 1 argument (cmd)");
    }
    let cmd = &arg_exprs[0];
    Ok(parse_quote! {
        std::process::Command::new("which")
            .arg(&#cmd)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    #[test]
    fn test_convert_shutil_copy() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String("/src/file".to_string())),
            HirExpr::Literal(Literal::String("/dst/file".to_string())),
        ];
        let result = convert_shutil_method("copy", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("fs") && code.contains("copy"));
    }

    #[test]
    fn test_convert_shutil_copy2() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String("/src".to_string())),
            HirExpr::Literal(Literal::String("/dst".to_string())),
        ];
        let result = convert_shutil_method("copy2", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_shutil_copy_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/src".to_string()))];
        let result = convert_shutil_method("copy", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_shutil_move() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String("/src".to_string())),
            HirExpr::Literal(Literal::String("/dst".to_string())),
        ];
        let result = convert_shutil_method("move", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("rename"));
    }

    #[test]
    fn test_convert_shutil_move_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/src".to_string()))];
        let result = convert_shutil_method("move", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_shutil_rmtree() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(
            "/path/to/dir".to_string(),
        ))];
        let result = convert_shutil_method("rmtree", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("remove_dir_all"));
    }

    #[test]
    fn test_convert_shutil_rmtree_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_shutil_method("rmtree", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_shutil_copytree() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String("/src/dir".to_string())),
            HirExpr::Literal(Literal::String("/dst/dir".to_string())),
        ];
        let result = convert_shutil_method("copytree", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("copy_dir_all"));
    }

    #[test]
    fn test_convert_shutil_copytree_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/src".to_string()))];
        let result = convert_shutil_method("copytree", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_shutil_which() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("python".to_string()))];
        let result = convert_shutil_method("which", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("Command") && code.contains("which"));
    }

    #[test]
    fn test_convert_shutil_which_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_shutil_method("which", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_shutil_unknown() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_shutil_method("unknown", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not implemented"));
    }
}
