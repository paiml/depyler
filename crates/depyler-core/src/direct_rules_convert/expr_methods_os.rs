//! OS module method conversion for ExprConverter
//!
//! Handles os.*, os.path.*, and os.environ.* method calls.

use crate::hir::*;
use anyhow::{bail, Result};
use syn::parse_quote;

use super::ExprConverter;

impl<'a> ExprConverter<'a> {
    /// DEPYLER-0200: Convert os module method calls to Rust std::fs and std::env equivalents
    /// This was missing from class method context, causing 57+ compile errors
    pub(super) fn try_convert_os_method(
        &self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| self.convert(arg)).collect::<Result<Vec<_>>>()?;

        let result = match method {
            "getenv" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("os.getenv() requires 1 or 2 arguments");
                }
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Some(parse_quote! { std::env::var(#key)? })
                } else {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    Some(
                        parse_quote! { std::env::var(#key).unwrap_or_else(|_| #default.to_string()) },
                    )
                }
            }
            "unlink" | "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("os.{}() requires exactly 1 argument", method);
                }
                let path = &arg_exprs[0];
                // DEPYLER-0956: Use .expect() to not require Result return type
                Some(parse_quote! { std::fs::remove_file(#path).expect("operation failed") })
            }
            "mkdir" => {
                if arg_exprs.is_empty() {
                    bail!("os.mkdir() requires at least 1 argument");
                }
                let path = &arg_exprs[0];
                // DEPYLER-0956: Use .expect() to not require Result return type
                Some(parse_quote! { std::fs::create_dir(#path).expect("operation failed") })
            }
            "makedirs" => {
                if arg_exprs.is_empty() {
                    bail!("os.makedirs() requires at least 1 argument");
                }
                let path = &arg_exprs[0];
                // DEPYLER-0956: Use .expect() to not require Result return type
                Some(parse_quote! { std::fs::create_dir_all(#path).expect("operation failed") })
            }
            "rmdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.rmdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                // DEPYLER-0956: Use .expect() to not require Result return type
                Some(parse_quote! { std::fs::remove_dir(#path).expect("operation failed") })
            }
            "rename" => {
                if arg_exprs.len() != 2 {
                    bail!("os.rename() requires exactly 2 arguments");
                }
                let src = &arg_exprs[0];
                let dst = &arg_exprs[1];
                // DEPYLER-0956: Use .expect() to not require Result return type
                Some(parse_quote! { std::fs::rename(#src, #dst).expect("operation failed") })
            }
            "getcwd" => {
                if !arg_exprs.is_empty() {
                    bail!("os.getcwd() takes no arguments");
                }
                Some(parse_quote! { std::env::current_dir()?.to_string_lossy().to_string() })
            }
            "chdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.chdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::env::set_current_dir(#path)? })
            }
            "listdir" => {
                if arg_exprs.is_empty() {
                    Some(parse_quote! {
                        std::fs::read_dir(".")?
                            .filter_map(|e| e.ok())
                            .map(|e| e.file_name().to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                    })
                } else {
                    let path = &arg_exprs[0];
                    Some(parse_quote! {
                        std::fs::read_dir(#path)?
                            .filter_map(|e| e.ok())
                            .map(|e| e.file_name().to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                    })
                }
            }
            "path" => {
                // os.path is a submodule, handled elsewhere
                None
            }
            _ => None,
        };

        Ok(result)
    }

    /// DEPYLER-0200: Convert os.path module method calls to Rust std::path equivalents
    pub(super) fn try_convert_os_path_method(
        &self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| self.convert(arg)).collect::<Result<Vec<_>>>()?;

        let result = match method {
            "join" => {
                if arg_exprs.is_empty() {
                    bail!("os.path.join() requires at least 1 argument");
                }
                let first = &arg_exprs[0];
                if arg_exprs.len() == 1 {
                    Some(parse_quote! { std::path::PathBuf::from(#first) })
                } else {
                    let mut result: syn::Expr = parse_quote! { std::path::PathBuf::from(#first) };
                    for part in &arg_exprs[1..] {
                        result = parse_quote! { #result.join(#part) };
                    }
                    Some(parse_quote! { #result.to_string_lossy().to_string() })
                }
            }
            "basename" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.basename() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! {
                    std::path::Path::new(&#path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string()
                })
            }
            "dirname" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.dirname() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! {
                    std::path::Path::new(&#path)
                        .parent()
                        .and_then(|p| p.to_str())
                        .unwrap_or("")
                        .to_string()
                })
            }
            "exists" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.exists() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::path::Path::new(&#path).exists() })
            }
            "isfile" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.isfile() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::path::Path::new(&#path).is_file() })
            }
            "isdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.isdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::path::Path::new(&#path).is_dir() })
            }
            "expanduser" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.expanduser() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! {
                    if (#path).starts_with("~") {
                        std::env::var("HOME")
                            .map(|home| (#path).replacen("~", &home, 1))
                            .unwrap_or_else(|_| (#path).to_string())
                    } else {
                        (#path).to_string()
                    }
                })
            }
            _ => None,
        };

        Ok(result)
    }

    /// DEPYLER-0200: Convert os.environ method calls to Rust std::env equivalents
    pub(super) fn try_convert_os_environ_method(
        &self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| self.convert(arg)).collect::<Result<Vec<_>>>()?;

        let result = match method {
            "get" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("os.environ.get() requires 1 or 2 arguments");
                }
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Some(parse_quote! { std::env::var(#key).ok() })
                } else {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    Some(
                        parse_quote! { std::env::var(#key).unwrap_or_else(|_| #default.to_string()) },
                    )
                }
            }
            "keys" => Some(parse_quote! { std::env::vars().map(|(k, _)| k).collect::<Vec<_>>() }),
            "values" => Some(parse_quote! { std::env::vars().map(|(_, v)| v).collect::<Vec<_>>() }),
            "items" => Some(parse_quote! { std::env::vars().collect::<Vec<_>>() }),
            "clear" => Some(parse_quote! { { /* env clear not implemented */ } }),
            "update" => Some(parse_quote! { { /* env update not implemented */ } }),
            "insert" | "setdefault" => {
                if arg_exprs.len() >= 2 {
                    let key = &arg_exprs[0];
                    let val = &arg_exprs[1];
                    Some(parse_quote! { std::env::set_var(#key, #val) })
                } else {
                    None
                }
            }
            "contains_key" => {
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Some(parse_quote! { std::env::var(#key).is_ok() })
                } else {
                    None
                }
            }
            _ => None,
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_mapper::TypeMapper;

    fn make_converter() -> ExprConverter<'static> {
        static TM: std::sync::OnceLock<TypeMapper> = std::sync::OnceLock::new();
        ExprConverter::new(TM.get_or_init(TypeMapper::default))
    }

    #[test]
    fn test_os_getenv_single_arg() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_method("getenv", &[HirExpr::Literal(Literal::String("HOME".into()))])
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_getenv_with_default() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_method(
                "getenv",
                &[
                    HirExpr::Literal(Literal::String("HOME".into())),
                    HirExpr::Literal(Literal::String("/tmp".into())),
                ],
            )
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_getcwd() {
        let conv = make_converter();
        let result = conv.try_convert_os_method("getcwd", &[]).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_path_join() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_path_method(
                "join",
                &[
                    HirExpr::Literal(Literal::String("/usr".into())),
                    HirExpr::Literal(Literal::String("bin".into())),
                ],
            )
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_path_basename() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_path_method(
                "basename",
                &[HirExpr::Literal(Literal::String("/usr/bin/ls".into()))],
            )
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_path_dirname() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_path_method(
                "dirname",
                &[HirExpr::Literal(Literal::String("/usr/bin/ls".into()))],
            )
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_path_exists() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_path_method(
                "exists",
                &[HirExpr::Literal(Literal::String("/tmp".into()))],
            )
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_path_isfile() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_path_method(
                "isfile",
                &[HirExpr::Literal(Literal::String("/tmp".into()))],
            )
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_path_isdir() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_path_method(
                "isdir",
                &[HirExpr::Literal(Literal::String("/tmp".into()))],
            )
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_environ_get() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_environ_method(
                "get",
                &[HirExpr::Literal(Literal::String("HOME".into()))],
            )
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_environ_keys() {
        let conv = make_converter();
        let result = conv.try_convert_os_environ_method("keys", &[]).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_environ_values() {
        let conv = make_converter();
        let result = conv.try_convert_os_environ_method("values", &[]).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_environ_items() {
        let conv = make_converter();
        let result = conv.try_convert_os_environ_method("items", &[]).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_environ_contains_key() {
        let conv = make_converter();
        let result = conv
            .try_convert_os_environ_method(
                "contains_key",
                &[HirExpr::Literal(Literal::String("PATH".into()))],
            )
            .unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_os_unknown_method() {
        let conv = make_converter();
        let result = conv.try_convert_os_method("unknown_method", &[]).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_os_path_unknown_method() {
        let conv = make_converter();
        let result = conv.try_convert_os_path_method("unknown_method", &[]).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_os_environ_unknown_method() {
        let conv = make_converter();
        let result = conv.try_convert_os_environ_method("unknown_method", &[]).unwrap();
        assert!(result.is_none());
    }
}
