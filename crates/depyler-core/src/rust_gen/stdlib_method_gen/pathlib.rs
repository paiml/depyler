//! Pathlib Module Code Generation - EXTREME TDD
//!
//! Handles Python `pathlib` module method conversions to Rust std::path.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python pathlib.Path method calls to Rust std::path
///
/// # Supported Methods
/// - Path queries: exists, is_file, is_dir, is_absolute
/// - Transformations: absolute, resolve, with_name, with_suffix
/// - Directory ops: mkdir, rmdir, iterdir
/// - File ops: read_text, read_bytes, write_text, write_bytes, unlink, rename
/// - Conversions: as_posix
///
/// # Complexity: 10 (delegated to match branches)
pub fn convert_pathlib_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        // Path queries
        "exists" => convert_exists(&arg_exprs)?,
        "is_file" => convert_is_file(&arg_exprs)?,
        "is_dir" => convert_is_dir(&arg_exprs)?,
        "is_absolute" => convert_is_absolute(&arg_exprs)?,
        // Transformations
        "absolute" | "resolve" => convert_resolve(method, &arg_exprs)?,
        "with_name" => convert_with_name(&arg_exprs)?,
        "with_suffix" => convert_with_suffix(&arg_exprs)?,
        // Directory operations
        "mkdir" => convert_mkdir(&arg_exprs)?,
        "rmdir" => convert_rmdir(&arg_exprs)?,
        "iterdir" => convert_iterdir(&arg_exprs)?,
        // File operations
        "read_text" => convert_read_text(&arg_exprs)?,
        "read_bytes" => convert_read_bytes(&arg_exprs)?,
        "write_text" => convert_write_text(&arg_exprs)?,
        "write_bytes" => convert_write_bytes(&arg_exprs)?,
        "unlink" => convert_unlink(&arg_exprs)?,
        "rename" => convert_rename(&arg_exprs)?,
        // Conversions
        "as_posix" => convert_as_posix(&arg_exprs)?,
        _ => return Ok(None),
    };

    Ok(Some(result))
}

fn convert_exists(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.exists() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { #path.exists() })
}

fn convert_is_file(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.is_file() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { #path.is_file() })
}

fn convert_is_dir(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.is_dir() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { #path.is_dir() })
}

fn convert_is_absolute(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.is_absolute() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { #path.is_absolute() })
}

fn convert_resolve(method: &str, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.{}() requires exactly 1 argument (self)", method);
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { #path.canonicalize().unwrap() })
}

fn convert_with_name(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("Path.with_name() requires exactly 2 arguments (self, name)");
    }
    let path = &arg_exprs[0];
    let name = &arg_exprs[1];
    Ok(parse_quote! { #path.with_file_name(#name) })
}

fn convert_with_suffix(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("Path.with_suffix() requires exactly 2 arguments (self, suffix)");
    }
    let path = &arg_exprs[0];
    let suffix = &arg_exprs[1];
    Ok(parse_quote! { #path.with_extension(#suffix.trim_start_matches('.')) })
}

fn convert_mkdir(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() || arg_exprs.len() > 2 {
        bail!("Path.mkdir() requires 1-2 arguments");
    }
    let path = &arg_exprs[0];
    if arg_exprs.len() == 2 {
        Ok(parse_quote! { std::fs::create_dir_all(#path).unwrap() })
    } else {
        Ok(parse_quote! { std::fs::create_dir(#path).unwrap() })
    }
}

fn convert_rmdir(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.rmdir() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { std::fs::remove_dir(#path).unwrap() })
}

fn convert_iterdir(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.iterdir() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! {
        std::fs::read_dir(#path)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect::<Vec<_>>()
    })
}

fn convert_read_text(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.read_text() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { std::fs::read_to_string(#path).unwrap() })
}

fn convert_read_bytes(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.read_bytes() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { std::fs::read(#path).unwrap() })
}

fn convert_write_text(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("Path.write_text() requires exactly 2 arguments (self, content)");
    }
    let path = &arg_exprs[0];
    let content = &arg_exprs[1];
    Ok(parse_quote! { std::fs::write(#path, #content).unwrap() })
}

fn convert_write_bytes(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("Path.write_bytes() requires exactly 2 arguments (self, content)");
    }
    let path = &arg_exprs[0];
    let content = &arg_exprs[1];
    Ok(parse_quote! { std::fs::write(#path, #content).unwrap() })
}

fn convert_unlink(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.unlink() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { std::fs::remove_file(#path).unwrap() })
}

fn convert_rename(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("Path.rename() requires exactly 2 arguments (self, target)");
    }
    let path = &arg_exprs[0];
    let target = &arg_exprs[1];
    Ok(
        parse_quote! { { std::fs::rename(&#path, #target).unwrap(); std::path::PathBuf::from(#target) } },
    )
}

fn convert_as_posix(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("Path.as_posix() requires exactly 1 argument (self)");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { #path.to_str().unwrap().to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    #[test]
    fn test_convert_pathlib_exists() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("exists", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("exists"));
    }

    #[test]
    fn test_convert_pathlib_is_file() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("is_file", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_is_dir() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("is_dir", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_is_absolute() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("is_absolute", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_resolve() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("resolve", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("canonicalize"));
    }

    #[test]
    fn test_convert_pathlib_absolute() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("absolute", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_with_name() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("p".to_string()),
            HirExpr::Literal(Literal::String("new_name.txt".to_string())),
        ];
        let result = convert_pathlib_method("with_name", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("with_file_name"));
    }

    #[test]
    fn test_convert_pathlib_with_suffix() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("p".to_string()),
            HirExpr::Literal(Literal::String(".txt".to_string())),
        ];
        let result = convert_pathlib_method("with_suffix", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_mkdir() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("mkdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("create_dir"));
    }

    #[test]
    fn test_convert_pathlib_mkdir_parents() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("p".to_string()),
            HirExpr::Literal(Literal::Bool(true)),
        ];
        let result = convert_pathlib_method("mkdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("create_dir_all"));
    }

    #[test]
    fn test_convert_pathlib_rmdir() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("rmdir", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_iterdir() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("iterdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("read_dir"));
    }

    #[test]
    fn test_convert_pathlib_read_text() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("read_text", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("read_to_string"));
    }

    #[test]
    fn test_convert_pathlib_read_bytes() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("read_bytes", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_write_text() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("p".to_string()),
            HirExpr::Literal(Literal::String("content".to_string())),
        ];
        let result = convert_pathlib_method("write_text", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_write_bytes() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("p".to_string()),
            HirExpr::Var("data".to_string()),
        ];
        let result = convert_pathlib_method("write_bytes", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_unlink() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("unlink", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("remove_file"));
    }

    #[test]
    fn test_convert_pathlib_rename() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("p".to_string()),
            HirExpr::Literal(Literal::String("/new/path".to_string())),
        ];
        let result = convert_pathlib_method("rename", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_as_posix() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("as_posix", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_pathlib_unknown() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("unknown", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    // Error cases
    #[test]
    fn test_convert_pathlib_exists_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_pathlib_method("exists", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_pathlib_with_name_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("p".to_string())];
        let result = convert_pathlib_method("with_name", &args, &mut ctx);
        assert!(result.is_err());
    }
}
