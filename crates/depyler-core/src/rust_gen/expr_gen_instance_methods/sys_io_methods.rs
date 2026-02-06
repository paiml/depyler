//! Sys I/O method handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size. Contains the `convert_sys_io_method`
//! handler covering: stdout/stderr write, flush, stdin read/readline/readlines.

use crate::rust_gen::expr_gen::ExpressionConverter;
use anyhow::{bail, Result};
use quote::quote;
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// DEPYLER-0381: Convert sys I/O stream method calls
    /// sys.stdout.write(msg) → writeln!(std::io::stdout(), "{}", msg).unwrap()
    /// sys.stdin.read() → { let mut s = String::new(); std::io::stdin().read_to_string(&mut s).unwrap(); s }
    /// sys.stdout.flush() → std::io::stdout().flush().unwrap()
    #[inline]
    pub(super) fn convert_sys_io_method(
        &self,
        stream: &str,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        let stream_fn = match stream {
            "stdin" => quote! { std::io::stdin() },
            "stdout" => quote! { std::io::stdout() },
            "stderr" => quote! { std::io::stderr() },
            _ => bail!("Unknown I/O stream: {}", stream),
        };

        let result = match (stream, method) {
            // stdout/stderr write methods
            ("stdout" | "stderr", "write") => {
                if arg_exprs.is_empty() {
                    bail!("{}.write() requires an argument", stream);
                }
                let msg = &arg_exprs[0];
                // Use writeln! macro for cleaner code and automatic newline handling
                // If the message already has \n, use write! instead
                parse_quote! {
                    {
                        use std::io::Write;
                        write!(#stream_fn, "{}", #msg).expect("write failed");
                    }
                }
            }

            // flush method
            (_, "flush") => {
                parse_quote! {
                    {
                        use std::io::Write;
                        #stream_fn.flush().expect("flush failed")
                    }
                }
            }

            // stdin read methods
            ("stdin", "read") => {
                parse_quote! {
                    {
                        use std::io::Read;
                        let mut buffer = String::new();
                        #stream_fn.read_to_string(&mut buffer).expect("read failed");
                        buffer
                    }
                }
            }

            ("stdin", "readline") => {
                parse_quote! {
                    {
                        use std::io::BufRead;
                        let mut line = String::new();
                        #stream_fn.lock().read_line(&mut line).expect("read failed");
                        line
                    }
                }
            }

            // DEPYLER-0638: stdin.readlines() → collect all lines from stdin
            // Python: lines = sys.stdin.readlines()
            // Rust: std::io::stdin().lock().lines().collect::<Result<Vec<_>, _>>().unwrap()
            ("stdin", "readlines") => {
                parse_quote! {
                    {
                        use std::io::BufRead;
                        #stream_fn.lock().lines().collect::<Result<Vec<_>, _>>().expect("read failed")
                    }
                }
            }

            _ => bail!("{}.{}() is not yet supported", stream, method),
        };

        Ok(result)
    }
}
