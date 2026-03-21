//! Stdlib os.path method converters
//!
//! DEPYLER-REFACTOR: Extracted from expr_gen/mod.rs
//!
//! Contains converters for Python os.path module:
//! - `try_convert_os_path_method` — Maps os.path calls to std::path + std::fs

use super::ExpressionConverter;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use anyhow::{bail, Result};
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Try to convert os.path module method calls
    /// DEPYLER-STDLIB-OSPATH: Path manipulation and file system operations
    ///
    /// Maps Python os.path module to Rust std::path + std::fs:
    /// - os.path.join() → PathBuf::new().join()
    /// - os.path.basename() → Path::file_name()
    /// - os.path.exists() → Path::exists()
    #[inline]
    pub(crate) fn try_convert_os_path_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        let result = match method {
            "join" => self.convert_ospath_join(&arg_exprs)?,
            "basename" => self.convert_ospath_single_path(&arg_exprs, "basename", Self::gen_basename)?,
            "dirname" => self.convert_ospath_single_path(&arg_exprs, "dirname", Self::gen_dirname)?,
            "split" => self.convert_ospath_single_path(&arg_exprs, "split", Self::gen_split)?,
            "splitext" => self.convert_ospath_single_path(&arg_exprs, "splitext", Self::gen_splitext)?,
            "exists" => self.convert_ospath_single_path(&arg_exprs, "exists", Self::gen_exists)?,
            "isfile" => self.convert_ospath_single_path(&arg_exprs, "isfile", Self::gen_isfile)?,
            "isdir" => self.convert_ospath_single_path(&arg_exprs, "isdir", Self::gen_isdir)?,
            "isabs" => self.convert_ospath_single_path(&arg_exprs, "isabs", Self::gen_isabs)?,
            "abspath" => self.convert_ospath_single_path(&arg_exprs, "abspath", Self::gen_abspath)?,
            "normpath" => self.convert_ospath_single_path(&arg_exprs, "normpath", Self::gen_normpath)?,
            "realpath" => self.convert_ospath_single_path(&arg_exprs, "realpath", Self::gen_realpath)?,
            "getsize" => self.convert_ospath_single_path(&arg_exprs, "getsize", Self::gen_getsize)?,
            "getmtime" => self.convert_ospath_single_path(&arg_exprs, "getmtime", Self::gen_getmtime)?,
            "getctime" => self.convert_ospath_single_path(&arg_exprs, "getctime", Self::gen_getctime)?,
            "expanduser" => self.convert_ospath_single_path(&arg_exprs, "expanduser", Self::gen_expanduser)?,
            "expandvars" => self.convert_ospath_single_path(&arg_exprs, "expandvars", Self::gen_expandvars)?,
            "relpath" => self.convert_ospath_relpath(&arg_exprs)?,
            _ => return Ok(None),
        };

        Ok(Some(result))
    }

    fn convert_ospath_join(
        &self,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            bail!("os.path.join() requires at least 1 argument");
        }
        let first = &arg_exprs[0];
        if arg_exprs.len() == 1 {
            return Ok(parse_quote! { std::path::PathBuf::from(#first) });
        }
        let mut result: syn::Expr = parse_quote! { std::path::PathBuf::from(#first) };
        for part in &arg_exprs[1..] {
            result = parse_quote! { #result.join(#part) };
        }
        Ok(parse_quote! { #result.to_string_lossy().to_string() })
    }

    fn convert_ospath_single_path(
        &self,
        arg_exprs: &[syn::Expr],
        name: &str,
        gen_fn: fn(&syn::Expr) -> syn::Expr,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("os.path.{}() requires exactly 1 argument", name);
        }
        Ok(gen_fn(&arg_exprs[0]))
    }

    fn gen_basename(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            std::path::Path::new(&#path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string()
        }
    }

    fn gen_dirname(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            std::path::Path::new(&#path)
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("")
                .to_string()
        }
    }

    fn gen_split(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            {
                let p = std::path::Path::new(&#path);
                let dirname = p.parent().and_then(|p| p.to_str()).unwrap_or("").to_string();
                let basename = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                (dirname, basename)
            }
        }
    }

    fn gen_splitext(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            {
                let p = std::path::Path::new(&#path);
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                let ext = p.extension().and_then(|e| e.to_str()).map(|e| format!(".{}", e)).unwrap_or_default();
                (stem, ext)
            }
        }
    }

    fn gen_exists(path: &syn::Expr) -> syn::Expr {
        parse_quote! { std::path::Path::new(&#path).exists() }
    }

    fn gen_isfile(path: &syn::Expr) -> syn::Expr {
        parse_quote! { std::path::Path::new(&#path).is_file() }
    }

    fn gen_isdir(path: &syn::Expr) -> syn::Expr {
        parse_quote! { std::path::Path::new(&#path).is_dir() }
    }

    fn gen_isabs(path: &syn::Expr) -> syn::Expr {
        parse_quote! { std::path::Path::new(&#path).is_absolute() }
    }

    fn gen_abspath(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            std::fs::canonicalize(&#path)
                .unwrap_or_else(|_| std::path::PathBuf::from(&#path))
                .to_string_lossy()
                .to_string()
        }
    }

    fn gen_normpath(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            {
                let p = std::path::Path::new(&#path);
                let mut components = Vec::new();
                for component in p.components() {
                    match component {
                        std::path::Component::CurDir => {},
                        std::path::Component::ParentDir => {
                            components.pop();
                        }
                        _ => components.push(component),
                    }
                }
                components.iter()
                    .map(|c| c.as_os_str().to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(std::path::MAIN_SEPARATOR_STR)
            }
        }
    }

    fn gen_realpath(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            std::fs::canonicalize(#path)
                .unwrap_or_else(|_| std::path::PathBuf::from(#path))
                .to_string_lossy()
                .to_string()
        }
    }

    fn gen_getsize(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            std::fs::metadata(&#path).expect("operation failed").len() as i64
        }
    }

    fn gen_getmtime(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            std::fs::metadata(&#path)
                .expect("operation failed")
                .modified()
                .expect("operation failed")
                .duration_since(std::time::UNIX_EPOCH)
                .expect("operation failed")
                .as_secs_f64()
        }
    }

    fn gen_getctime(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            std::fs::metadata(&#path)
                .expect("operation failed")
                .created()
                .expect("operation failed")
                .duration_since(std::time::UNIX_EPOCH)
                .expect("operation failed")
                .as_secs_f64()
        }
    }

    fn gen_expanduser(path: &syn::Expr) -> syn::Expr {
        parse_quote! {
            {
                let p = #path;
                if p.starts_with("~") {
                    if let Some(home) = std::env::var_os("HOME") {
                        format!("{}{}", home.to_string_lossy(), &p[1..])
                    } else {
                        p.to_string()
                    }
                } else {
                    p.to_string()
                }
            }
        }
    }

    fn gen_expandvars(path: &syn::Expr) -> syn::Expr {
        parse_quote! { #path.to_string() }
    }

    fn convert_ospath_relpath(
        &self,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 2 {
            bail!("os.path.relpath() requires exactly 2 arguments");
        }
        let path = &arg_exprs[0];
        let start = &arg_exprs[1];
        Ok(parse_quote! {
            {
                let path_obj = std::path::Path::new(#path);
                let start_obj = std::path::Path::new(#start);
                path_obj
                    .strip_prefix(start_obj)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| #path.to_string())
            }
        })
    }
}
