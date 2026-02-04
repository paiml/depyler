//! OS Module Code Generation - EXTREME TDD
//!
//! Handles Python `os` module method conversions to Rust std::env/std::fs.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python os module method calls to Rust
///
/// # Supported Methods
/// - `os.getenv(key)` → `std::env::var(key)?`
/// - `os.getenv(key, default)` → `std::env::var(key).unwrap_or_else(...)`
/// - `os.unlink(path)` / `os.remove(path)` → `std::fs::remove_file(path)`
/// - `os.mkdir(path)` → `std::fs::create_dir(path)`
/// - `os.makedirs(path)` → `std::fs::create_dir_all(path)`
/// - `os.rmdir(path)` → `std::fs::remove_dir(path)`
/// - `os.rename(src, dst)` → `std::fs::rename(src, dst)`
/// - `os.getcwd()` → `std::env::current_dir()`
/// - `os.chdir(path)` → `std::env::set_current_dir(path)`
/// - `os.listdir(path)` → `std::fs::read_dir(path)`
/// - `os.walk(path)` → `walkdir::WalkDir::new(path)`
/// - `os.urandom(n)` → rand crate for random bytes
///
/// # Complexity: 10 (match with 10+ branches)
pub fn convert_os_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    // Convert arguments first
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "getenv" => convert_getenv(&arg_exprs)?,
        "unlink" | "remove" => convert_unlink(method, &arg_exprs)?,
        "mkdir" => convert_mkdir(&arg_exprs)?,
        "makedirs" => convert_makedirs(&arg_exprs)?,
        "rmdir" => convert_rmdir(&arg_exprs)?,
        "rename" => convert_rename(&arg_exprs)?,
        "getcwd" => convert_getcwd(&arg_exprs, ctx)?,
        "chdir" => convert_chdir(&arg_exprs, ctx)?,
        "listdir" => convert_listdir(&arg_exprs, ctx)?,
        "walk" => convert_walk(&arg_exprs)?,
        "urandom" => convert_urandom(&arg_exprs)?,
        _ => return Ok(None),
    };

    Ok(Some(result))
}

/// os.getenv(key) / os.getenv(key, default)
fn convert_getenv(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() || arg_exprs.len() > 2 {
        bail!("os.getenv() requires 1 or 2 arguments");
    }

    if arg_exprs.len() == 1 {
        let key = &arg_exprs[0];
        Ok(parse_quote! { std::env::var(#key)? })
    } else {
        let key = &arg_exprs[0];
        let default = &arg_exprs[1];
        Ok(parse_quote! {
            std::env::var(#key).unwrap_or_else(|_| #default.to_string())
        })
    }
}

/// os.unlink(path) / os.remove(path) → std::fs::remove_file(path)
fn convert_unlink(method: &str, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("os.{}() requires exactly 1 argument", method);
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { std::fs::remove_file(#path).expect("filesystem operation failed") })
}

/// os.mkdir(path) → std::fs::create_dir(path)
fn convert_mkdir(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("os.mkdir() requires at least 1 argument");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { std::fs::create_dir(#path).expect("filesystem operation failed") })
}

/// os.makedirs(path) → std::fs::create_dir_all(path)
fn convert_makedirs(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("os.makedirs() requires at least 1 argument");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { std::fs::create_dir_all(#path).expect("filesystem operation failed") })
}

/// os.rmdir(path) → std::fs::remove_dir(path)
fn convert_rmdir(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("os.rmdir() requires exactly 1 argument");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! { std::fs::remove_dir(#path).expect("filesystem operation failed") })
}

/// os.rename(src, dst) → std::fs::rename(src, dst)
fn convert_rename(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("os.rename() requires exactly 2 arguments");
    }
    let src = &arg_exprs[0];
    let dst = &arg_exprs[1];
    Ok(parse_quote! { std::fs::rename(#src, #dst).expect("filesystem operation failed") })
}

/// os.getcwd() → std::env::current_dir()
fn convert_getcwd(arg_exprs: &[syn::Expr], ctx: &CodeGenContext) -> Result<syn::Expr> {
    if !arg_exprs.is_empty() {
        bail!("os.getcwd() takes no arguments");
    }
    if ctx.current_function_can_fail {
        Ok(parse_quote! { std::env::current_dir()?.to_string_lossy().to_string() })
    } else {
        Ok(
            parse_quote! { std::env::current_dir().expect("Failed to get current directory").to_string_lossy().to_string() },
        )
    }
}

/// os.chdir(path) → std::env::set_current_dir(path)
fn convert_chdir(arg_exprs: &[syn::Expr], ctx: &CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("os.chdir() requires exactly 1 argument");
    }
    let path = &arg_exprs[0];
    if ctx.current_function_can_fail {
        Ok(parse_quote! { std::env::set_current_dir(#path)? })
    } else {
        Ok(parse_quote! { std::env::set_current_dir(#path).expect("Failed to change directory") })
    }
}

/// os.listdir(path) → std::fs::read_dir(path)
fn convert_listdir(arg_exprs: &[syn::Expr], ctx: &CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        // os.listdir() with no args uses current directory
        if ctx.current_function_can_fail {
            Ok(parse_quote! {
                std::fs::read_dir(".")?
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
            })
        } else {
            Ok(parse_quote! {
                std::fs::read_dir(".").expect("Failed to read directory")
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
            })
        }
    } else {
        let path = &arg_exprs[0];
        if ctx.current_function_can_fail {
            Ok(parse_quote! {
                std::fs::read_dir(#path)?
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
            })
        } else {
            Ok(parse_quote! {
                std::fs::read_dir(#path).expect("Failed to read directory")
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
            })
        }
    }
}

/// os.walk(path) → walkdir::WalkDir::new(path)
fn convert_walk(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("os.walk() requires at least 1 argument");
    }
    let path = &arg_exprs[0];
    Ok(parse_quote! {
        walkdir::WalkDir::new(#path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .map(|dir_entry| {
                let root = dir_entry.path().to_string_lossy().to_string();
                let mut dirs = vec![];
                let mut files = vec![];
                if let Ok(entries) = std::fs::read_dir(dir_entry.path()) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                            dirs.push(name);
                        } else {
                            files.push(name);
                        }
                    }
                }
                (root, dirs, files)
            })
            .collect::<Vec<_>>()
    })
}

/// os.urandom(n) → rand crate for random bytes
fn convert_urandom(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("os.urandom() requires exactly 1 argument");
    }
    let n = &arg_exprs[0];
    Ok(parse_quote! {
        {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let mut bytes = vec![0u8; #n as usize];
            rng.fill(&mut bytes[..]);
            bytes
        }
    })
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    // ============ convert_os_method - getenv() tests ============

    #[test]
    fn test_convert_os_getenv_single_arg() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("HOME".to_string()))];
        let result = convert_os_method("getenv", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("env") && code.contains("var"));
    }

    #[test]
    fn test_convert_os_getenv_with_default() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String("HOME".to_string())),
            HirExpr::Literal(Literal::String("/tmp".to_string())),
        ];
        let result = convert_os_method("getenv", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("unwrap_or_else"));
    }

    #[test]
    fn test_convert_os_getenv_no_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("getenv", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires 1 or 2 arguments"));
    }

    #[test]
    fn test_convert_os_getenv_too_many_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String("A".to_string())),
            HirExpr::Literal(Literal::String("B".to_string())),
            HirExpr::Literal(Literal::String("C".to_string())),
        ];
        let result = convert_os_method("getenv", &args, &mut ctx);
        assert!(result.is_err());
    }

    // ============ convert_os_method - unlink/remove() tests ============

    #[test]
    fn test_convert_os_unlink_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(
            "/tmp/file.txt".to_string(),
        ))];
        let result = convert_os_method("unlink", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("remove_file"));
    }

    #[test]
    fn test_convert_os_remove_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(
            "/tmp/file.txt".to_string(),
        ))];
        let result = convert_os_method("remove", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("remove_file"));
    }

    #[test]
    fn test_convert_os_unlink_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("unlink", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires exactly 1 argument"));
    }

    // ============ convert_os_method - mkdir() tests ============

    #[test]
    fn test_convert_os_mkdir_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/tmp/newdir".to_string()))];
        let result = convert_os_method("mkdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("create_dir"));
    }

    #[test]
    fn test_convert_os_mkdir_no_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("mkdir", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 1 argument"));
    }

    // ============ convert_os_method - makedirs() tests ============

    #[test]
    fn test_convert_os_makedirs_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/tmp/a/b/c".to_string()))];
        let result = convert_os_method("makedirs", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("create_dir_all"));
    }

    #[test]
    fn test_convert_os_makedirs_no_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("makedirs", &args, &mut ctx);
        assert!(result.is_err());
    }

    // ============ convert_os_method - rmdir() tests ============

    #[test]
    fn test_convert_os_rmdir_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/tmp/dir".to_string()))];
        let result = convert_os_method("rmdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("remove_dir"));
    }

    #[test]
    fn test_convert_os_rmdir_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("rmdir", &args, &mut ctx);
        assert!(result.is_err());
    }

    // ============ convert_os_method - rename() tests ============

    #[test]
    fn test_convert_os_rename_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String("/tmp/old".to_string())),
            HirExpr::Literal(Literal::String("/tmp/new".to_string())),
        ];
        let result = convert_os_method("rename", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("rename"));
    }

    #[test]
    fn test_convert_os_rename_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/tmp/old".to_string()))];
        let result = convert_os_method("rename", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires exactly 2 arguments"));
    }

    // ============ convert_os_method - getcwd() tests ============

    #[test]
    fn test_convert_os_getcwd_basic() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("getcwd", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("current_dir"));
    }

    #[test]
    fn test_convert_os_getcwd_can_fail() {
        let mut ctx = CodeGenContext::default();
        ctx.current_function_can_fail = true;
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("getcwd", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("?"));
    }

    // ============ convert_os_method - chdir() tests ============

    #[test]
    fn test_convert_os_chdir_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/tmp".to_string()))];
        let result = convert_os_method("chdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("set_current_dir"));
    }

    #[test]
    fn test_convert_os_chdir_can_fail() {
        let mut ctx = CodeGenContext::default();
        ctx.current_function_can_fail = true;
        let args = vec![HirExpr::Literal(Literal::String("/tmp".to_string()))];
        let result = convert_os_method("chdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("?"));
    }

    #[test]
    fn test_convert_os_chdir_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("chdir", &args, &mut ctx);
        assert!(result.is_err());
    }

    // ============ convert_os_method - listdir() tests ============

    #[test]
    fn test_convert_os_listdir_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/tmp".to_string()))];
        let result = convert_os_method("listdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("read_dir"));
    }

    #[test]
    fn test_convert_os_listdir_no_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("listdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("read_dir") && code.contains("."));
    }

    #[test]
    fn test_convert_os_listdir_can_fail() {
        let mut ctx = CodeGenContext::default();
        ctx.current_function_can_fail = true;
        let args = vec![HirExpr::Literal(Literal::String("/tmp".to_string()))];
        let result = convert_os_method("listdir", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("?"));
    }

    // ============ convert_os_method - walk() tests ============

    #[test]
    fn test_convert_os_walk_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("/tmp".to_string()))];
        let result = convert_os_method("walk", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("WalkDir") && code.contains("new"));
    }

    #[test]
    fn test_convert_os_walk_no_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("walk", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 1 argument"));
    }

    // ============ convert_os_method - urandom() tests ============

    #[test]
    fn test_convert_os_urandom_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Int(16))];
        let result = convert_os_method("urandom", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("rand") && code.contains("Rng"));
    }

    #[test]
    fn test_convert_os_urandom_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("urandom", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires exactly 1 argument"));
    }

    // ============ convert_os_method - unknown method tests ============

    #[test]
    fn test_convert_os_unknown_method() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_os_method("unknown_func", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Returns None for unknown methods
    }
}
