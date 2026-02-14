//! Standard library call conversion for ExprConverter
//!
//! Handles os.path, date/datetime, open(), and generic function calls.

use crate::direct_rules::{make_ident, safe_class_name};
use crate::hir::*;
use anyhow::{bail, Result};
use syn::parse_quote;

use super::ExprConverter;

impl<'a> ExprConverter<'a> {
    pub(super) fn convert_splitext_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("splitext() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            {
                let p = std::path::Path::new(&#path);
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                let ext = p.extension().and_then(|e| e.to_str()).map(|e| format!(".{}", e)).unwrap_or_default();
                (stem, ext)
            }
        })
    }

    /// DEPYLER-0721: os.path.basename(path) → Path::file_name
    pub(super) fn convert_basename_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("basename() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            std::path::Path::new(&#path).file_name().and_then(|n| n.to_str()).unwrap_or("").to_string()
        })
    }

    /// DEPYLER-0721: os.path.dirname(path) → Path::parent
    pub(super) fn convert_dirname_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("dirname() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            std::path::Path::new(&#path).parent().and_then(|p| p.to_str()).unwrap_or("").to_string()
        })
    }

    /// DEPYLER-0721: os.path.split(path) → (dirname, basename)
    pub(super) fn convert_path_split_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("split() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            {
                let p = std::path::Path::new(&#path);
                let dirname = p.parent().and_then(|p| p.to_str()).unwrap_or("").to_string();
                let basename = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                (dirname, basename)
            }
        })
    }

    /// DEPYLER-0721: os.path.exists(path) → Path::exists
    pub(super) fn convert_path_exists_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("exists() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! { std::path::Path::new(&#path).exists() })
    }

    /// DEPYLER-0721: os.path.isfile(path) → Path::is_file
    pub(super) fn convert_path_isfile_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("isfile() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! { std::path::Path::new(&#path).is_file() })
    }

    /// DEPYLER-0721: os.path.isdir(path) → Path::is_dir
    pub(super) fn convert_path_isdir_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("isdir() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! { std::path::Path::new(&#path).is_dir() })
    }

    /// DEPYLER-0200: Convert Python open() to Rust file operations
    /// open(path) → std::fs::File::open(path) (read mode)
    /// open(path, "w") → std::fs::File::create(path) (write mode)
    pub(super) fn convert_open_call(&self, hir_args: &[HirExpr], args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("open() requires 1 or 2 arguments");
        }

        let path = &args[0];

        // Determine mode from second argument (default is 'r')
        let mode = if hir_args.len() >= 2 {
            if let HirExpr::Literal(Literal::String(mode_str)) = &hir_args[1] {
                mode_str.as_str()
            } else {
                "r" // Default to read mode
            }
        } else {
            "r" // Default mode
        };

        match mode {
            "w" | "w+" | "wb" => {
                // Write mode: std::fs::File::create()
                Ok(parse_quote! { std::fs::File::create(&#path).expect("file create failed") })
            }
            "a" | "a+" | "ab" => {
                // Append mode: OpenOptions with append
                Ok(parse_quote! {
                    std::fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&#path)
                        .expect("file open failed")
                })
            }
            _ => {
                // Read mode (default): std::fs::File::open()
                Ok(parse_quote! { std::fs::File::open(&#path).expect("file open failed") })
            }
        }
    }

    /// DEPYLER-0200: Convert Python date(year, month, day) to chrono::NaiveDate
    pub(super) fn convert_date_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 3 {
            bail!("date() requires exactly 3 arguments (year, month, day)");
        }
        let year = &args[0];
        let month = &args[1];
        let day = &args[2];
        Ok(parse_quote! {
            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32).expect("invalid date")
        })
    }

    /// DEPYLER-0200: Convert Python datetime(year, month, day, ...) to chrono::NaiveDateTime
    pub(super) fn convert_datetime_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 3 {
            bail!("datetime() requires at least 3 arguments (year, month, day)");
        }
        let year = &args[0];
        let month = &args[1];
        let day = &args[2];

        // Handle optional time components
        let zero: syn::Expr = parse_quote! { 0 };
        let hour = args.get(3).unwrap_or(&zero);
        let minute = args.get(4).unwrap_or(&zero);
        let second = args.get(5).unwrap_or(&zero);

        Ok(parse_quote! {
            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                .expect("invalid date")
                .and_hms_opt(#hour as u32, #minute as u32, #second as u32)
                .expect("invalid time")
        })
    }

    pub(super) fn convert_generic_call(
        &self,
        func: &str,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // Special case: Python print() → Rust println!()
        if func == "print" {
            return if args.is_empty() {
                // print() with no arguments → println!()
                Ok(parse_quote! { println!() })
            } else if args.len() == 1 {
                // print(x) → println!("{}", x)
                let arg = &args[0];
                Ok(parse_quote! { println!("{}", #arg) })
            } else {
                // print(a, b, c) → println!("{} {} {}", a, b, c)
                let format_str = vec!["{}"; args.len()].join(" ");
                Ok(parse_quote! { println!(#format_str, #(#args),*) })
            };
        }

        // DEPYLER-0600: Handle Python built-in type conversion functions
        // These are used in class methods and need proper Rust equivalents
        match func {
            "int" if args.len() == 1 => {
                // int(x) → x.parse::<i32>().unwrap() for strings, x as i32 for numbers
                let arg = &args[0];
                return Ok(parse_quote! { #arg.parse::<i32>().unwrap_or(0) });
            }
            "float" if args.len() == 1 => {
                // float(x) → x.parse::<f64>().unwrap() for strings, x as f64 for integers
                let arg = &args[0];
                return Ok(parse_quote! { #arg.parse::<f64>().unwrap_or(0.0) });
            }
            "str" if args.len() == 1 => {
                // str(x) → x.to_string()
                let arg = &args[0];
                return Ok(parse_quote! { #arg.to_string() });
            }
            "bool" if args.len() == 1 => {
                // bool(x) → general truthiness conversion
                let arg = &args[0];
                return Ok(parse_quote! { !#arg.is_empty() });
            }
            "len" if args.len() == 1 => {
                // len(x) → x.len() as i32
                // DEPYLER-0693: Cast len() to i32 for Python compatibility
                let arg = &args[0];
                return Ok(parse_quote! { #arg.len() as i32 });
            }
            "abs" if args.len() == 1 => {
                // DEPYLER-0815: abs(x) → (x).abs() - parens needed for precedence
                let arg = &args[0];
                return Ok(parse_quote! { (#arg).abs() });
            }
            "min" if args.len() == 1 => {
                // DEPYLER-1094: min(iterable) → iterable.iter().cloned().min().unwrap()
                // Single-arg min finds minimum element in iterable
                let arg = &args[0];
                return Ok(parse_quote! { #arg.iter().cloned().min().expect("empty collection") });
            }
            "min" if args.len() >= 2 => {
                // DEPYLER-1094: min(a, b, ...) → (a as f64).min(b as f64)
                // Cast both to f64 to handle mixed i32/f64 types (Python promotes to float)
                // Use f64::min which is well-defined for all finite floats
                let first = &args[0];
                let rest = &args[1..];
                let mut result = parse_quote! { (#first as f64) };
                for arg in rest {
                    result = parse_quote! { (#result).min(#arg as f64) };
                }
                return Ok(result);
            }
            "max" if args.len() == 1 => {
                // DEPYLER-1094: max(iterable) → iterable.iter().cloned().max().unwrap()
                // Single-arg max finds maximum element in iterable
                let arg = &args[0];
                return Ok(parse_quote! { #arg.iter().cloned().max().expect("empty collection") });
            }
            "max" if args.len() >= 2 => {
                // DEPYLER-1094: max(a, b, ...) → (a as f64).max(b as f64)
                // Cast both to f64 to handle mixed i32/f64 types (Python promotes to float)
                // Use f64::max which is well-defined for all finite floats
                let first = &args[0];
                let rest = &args[1..];
                let mut result = parse_quote! { (#first as f64) };
                for arg in rest {
                    result = parse_quote! { (#result).max(#arg as f64) };
                }
                return Ok(result);
            }
            _ => {}
        }

        // Check if this might be a constructor call (capitalized name)
        if func
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            // DEPYLER-0900: Rename constructor if it shadows stdlib type (e.g., Box -> PyBox)
            // Treat as constructor call - ClassName::new(args)
            let safe_name = safe_class_name(func);
            let class_ident = make_ident(&safe_name);
            if args.is_empty() {
                // Note: Constructor default parameter handling uses simple heuristics.
                // Ideally this would be context-aware and know the actual default values
                // for each class constructor, but currently uses hardcoded patterns.
                // This is a known limitation - constructors may require explicit arguments.
                match func {
                    "Counter" => Ok(parse_quote! { #class_ident::new(0) }),
                    _ => Ok(parse_quote! { #class_ident::new() }),
                }
            } else {
                Ok(parse_quote! { #class_ident::new(#(#args),*) })
            }
        } else {
            // Regular function call
            let func_ident = make_ident(func);

            // DEPYLER-0648: Check if this is a vararg function
            // If so, wrap arguments in a slice: func(a, b) → func(&[a, b])
            if self.vararg_functions.contains(func) && !args.is_empty() {
                Ok(parse_quote! { #func_ident(&[#(#args),*]) })
            } else {
                // DEPYLER-0780: Auto-borrow list/dict/set literals when calling functions
                // Most user-defined functions taking list params expect &Vec<T>
                let borrowed_args: Vec<syn::Expr> = hir_args
                    .iter()
                    .zip(args.iter())
                    .map(|(hir_arg, arg_expr)| {
                        match hir_arg {
                            // List/Dict/Set literals should be borrowed
                            HirExpr::List(_) | HirExpr::Dict(_) | HirExpr::Set(_) => {
                                parse_quote! { &#arg_expr }
                            }
                            _ => arg_expr.clone(),
                        }
                    })
                    .collect();
                Ok(parse_quote! { #func_ident(#(#borrowed_args),*) })
            }
        }
    }


}
