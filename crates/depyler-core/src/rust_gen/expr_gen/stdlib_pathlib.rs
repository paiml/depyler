//! Stdlib pathlib instance method converters
//!
//! DEPYLER-REFACTOR: Extracted from expr_gen/mod.rs
//!
//! Contains converters for pathlib instance method calls:
//! - `convert_pathlib_instance_method` — Maps Path/PathBuf variable method calls
//!   (e.g., `p.write_text(content)`, `p.exists()`)

use super::ExpressionConverter;
use anyhow::{bail, Result};
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// DEPYLER-0829: Convert pathlib methods on Path/PathBuf variable instances
    /// This handles cases like `p.write_text(content)` where p is a Path variable
    /// Unlike try_convert_pathlib_method which handles module calls like pathlib.Path(...).method()
    #[inline]
    pub(crate) fn convert_pathlib_instance_method(
        &mut self,
        path_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        let result = match method {
            // File I/O operations
            "write_text" => {
                if arg_exprs.is_empty() {
                    bail!("write_text() requires at least 1 argument (content)");
                }
                let content = &arg_exprs[0];
                parse_quote! { std::fs::write(&#path_expr, #content).expect("operation failed") }
            }

            "read_text" => {
                parse_quote! { std::fs::read_to_string(&#path_expr).expect("operation failed") }
            }

            "read_bytes" => {
                parse_quote! { std::fs::read(&#path_expr).expect("operation failed") }
            }

            "write_bytes" => {
                if arg_exprs.is_empty() {
                    bail!("write_bytes() requires at least 1 argument (data)");
                }
                let data = &arg_exprs[0];
                parse_quote! { std::fs::write(&#path_expr, #data).expect("operation failed") }
            }

            // Path predicates
            "exists" => {
                parse_quote! { #path_expr.exists() }
            }

            "is_file" => {
                parse_quote! { #path_expr.is_file() }
            }

            "is_dir" => {
                parse_quote! { #path_expr.is_dir() }
            }

            // Directory operations
            "mkdir" => {
                // Check if parents=True was passed
                if !arg_exprs.is_empty() {
                    parse_quote! { std::fs::create_dir_all(&#path_expr).expect("operation failed") }
                } else {
                    parse_quote! { std::fs::create_dir(&#path_expr).expect("operation failed") }
                }
            }

            "rmdir" => {
                parse_quote! { std::fs::remove_dir(&#path_expr).expect("operation failed") }
            }

            "unlink" => {
                parse_quote! { std::fs::remove_file(&#path_expr).expect("operation failed") }
            }

            "iterdir" => {
                parse_quote! {
                    std::fs::read_dir(&#path_expr)
                        .expect("operation failed")
                        .map(|e| e.expect("operation failed").path())
                        .collect::<Vec<_>>()
                }
            }

            // Glob operations - require glob crate
            "glob" => {
                self.ctx.needs_glob = true;
                if arg_exprs.is_empty() {
                    bail!("glob() requires at least 1 argument (pattern)");
                }
                let pattern = &arg_exprs[0];
                parse_quote! {
                    glob::glob(&format!("{}/{}", #path_expr.display(), #pattern))
                        .expect("operation failed")
                        .filter_map(|e| e.ok())
                        .collect::<Vec<_>>()
                }
            }

            "rglob" => {
                self.ctx.needs_glob = true;
                if arg_exprs.is_empty() {
                    bail!("rglob() requires at least 1 argument (pattern)");
                }
                let pattern = &arg_exprs[0];
                parse_quote! {
                    glob::glob(&format!("{}/**/{}", #path_expr.display(), #pattern))
                        .expect("operation failed")
                        .filter_map(|e| e.ok())
                        .collect::<Vec<_>>()
                }
            }

            // Path transformations
            "with_name" => {
                if arg_exprs.is_empty() {
                    bail!("with_name() requires 1 argument (name)");
                }
                let name = &arg_exprs[0];
                parse_quote! { #path_expr.with_file_name(#name) }
            }

            "with_suffix" => {
                if arg_exprs.is_empty() {
                    bail!("with_suffix() requires 1 argument (suffix)");
                }
                let suffix = &arg_exprs[0];
                parse_quote! { #path_expr.with_extension(#suffix.trim_start_matches('.')) }
            }

            "with_stem" => {
                // Python's with_stem - change stem keeping extension
                if arg_exprs.is_empty() {
                    bail!("with_stem() requires 1 argument (stem)");
                }
                let stem = &arg_exprs[0];
                parse_quote! {
                    {
                        let p = &#path_expr;
                        let ext = p.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
                        p.with_file_name(format!("{}{}", #stem, ext))
                    }
                }
            }

            "resolve" | "absolute" => {
                parse_quote! { #path_expr.canonicalize().expect("operation failed") }
            }

            "relative_to" => {
                if arg_exprs.is_empty() {
                    bail!("relative_to() requires 1 argument (base)");
                }
                let base = &arg_exprs[0];
                parse_quote! { #path_expr.strip_prefix(#base).expect("operation failed").to_path_buf() }
            }

            _ => {
                // Fall through to regular method call
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                parse_quote! { #path_expr.#method_ident(#(#arg_exprs),*) }
            }
        };

        Ok(result)
    }
}
