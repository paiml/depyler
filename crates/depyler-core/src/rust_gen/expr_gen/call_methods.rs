//! Class/struct/CSV/OS/module method conversion for ExpressionConverter
//!
//! Contains try_convert_classmethod, try_convert_struct_method,
//! try_convert_csv_method, try_convert_os_environ_method, try_convert_module_method.

use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::stdlib_method_gen;
use anyhow::{bail, Result};
use quote::quote;
use syn::{self, parse_quote};

use super::ExpressionConverter;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn try_convert_classmethod(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if let HirExpr::Var(var_name) = object {
            if var_name == "cls" && self.ctx.is_classmethod {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(Some(parse_quote! { Self::#method_ident(#(#arg_exprs),*) }));
            }
        }
        Ok(None)
    }

    /// DEPYLER-0021: Handle struct module methods (pack, unpack, calcsize)
    /// Only supports format codes 'i' (signed 32-bit int) and 'ii' (two ints)
    pub(crate) fn try_convert_struct_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        match method {
            "pack" => {
                if args.is_empty() {
                    bail!("struct.pack() requires at least a format argument");
                }

                // First arg is format string
                if let HirExpr::Literal(Literal::String(format)) = &args[0] {
                    let count = format.chars().filter(|&c| c == 'i').count();

                    if count == 0 {
                        bail!("struct.pack() format '{}' not supported (only 'i' and 'ii' implemented)", format);
                    }

                    if count != args.len() - 1 {
                        bail!(
                            "struct.pack() format '{}' expects {} values, got {}",
                            format,
                            count,
                            args.len() - 1
                        );
                    }

                    // Convert value arguments
                    let value_exprs: Vec<syn::Expr> = args[1..]
                        .iter()
                        .map(|arg| arg.to_rust_expr(self.ctx))
                        .collect::<Result<Vec<_>>>()?;

                    if count == 1 {
                        // struct.pack('i', value) → (value as i32).to_le_bytes().to_vec()
                        let val = &value_exprs[0];
                        Ok(Some(parse_quote! {
                            (#val as i32).to_le_bytes().to_vec()
                        }))
                    } else {
                        // struct.pack('ii', a, b) → { let mut v = Vec::new(); v.extend_from_slice(&(a as i32).to_le_bytes()); ... }
                        Ok(Some(parse_quote! {
                            {
                                let mut __struct_pack_result = Vec::new();
                                #(__struct_pack_result.extend_from_slice(&(#value_exprs as i32).to_le_bytes());)*
                                __struct_pack_result
                            }
                        }))
                    }
                } else {
                    bail!("struct.pack() requires string literal format (dynamic formats not supported)");
                }
            }
            "unpack" => {
                if args.len() != 2 {
                    bail!("struct.unpack() requires exactly 2 arguments (format, bytes)");
                }

                // First arg is format string
                if let HirExpr::Literal(Literal::String(format)) = &args[0] {
                    let count = format.chars().filter(|&c| c == 'i').count();

                    if count == 0 {
                        bail!("struct.unpack() format '{}' not supported (only 'i' and 'ii' implemented)", format);
                    }

                    let bytes_expr = args[1].to_rust_expr(self.ctx)?;

                    if count == 1 {
                        // struct.unpack('i', bytes) → (i32::from_le_bytes(bytes[0..4].try_into().expect("...")),)
                        Ok(Some(parse_quote! {
                            (i32::from_le_bytes(#bytes_expr[0..4].try_into().expect("operation failed")),)
                        }))
                    } else if count == 2 {
                        // struct.unpack('ii', bytes) → (i32::from_le_bytes(...), i32::from_le_bytes(...))
                        Ok(Some(parse_quote! {
                            (
                                i32::from_le_bytes(#bytes_expr[0..4].try_into().expect("operation failed")),
                                i32::from_le_bytes(#bytes_expr[4..8].try_into().expect("operation failed")),
                            )
                        }))
                    } else {
                        bail!(
                            "struct.unpack() only supports 'i' and 'ii' formats (got {} ints)",
                            count
                        );
                    }
                } else {
                    bail!("struct.unpack() requires string literal format (dynamic formats not supported)");
                }
            }
            "calcsize" => {
                if args.len() != 1 {
                    bail!("struct.calcsize() requires exactly 1 argument");
                }

                // Arg is format string
                if let HirExpr::Literal(Literal::String(format)) = &args[0] {
                    let count = format.chars().filter(|&c| c == 'i').count();

                    if count == 0 {
                        bail!("struct.calcsize() format '{}' not supported (only 'i' and 'ii' implemented)", format);
                    }

                    let size = (count * 4) as i32;
                    Ok(Some(parse_quote! { #size }))
                } else {
                    bail!("struct.calcsize() requires string literal format (dynamic formats not supported)");
                }
            }
            _ => {
                bail!("struct.{} not implemented", method);
            }
        }
    }

    // DEPYLER-COVERAGE-95: try_convert_json_method moved to stdlib_method_gen::json

    // DEPYLER-COVERAGE-95: try_convert_re_method moved to stdlib_method_gen::regex_mod

    // DEPYLER-COVERAGE-95: try_convert_string_method moved to stdlib_method_gen::string

    // DEPYLER-COVERAGE-95: try_convert_time_method moved to stdlib_method_gen::time

    // DEPYLER-COVERAGE-95: try_convert_shutil_method moved to stdlib_method_gen::shutil

    /// Try to convert csv module method calls
    /// DEPYLER-STDLIB-CSV: CSV file reading and writing
    ///
    /// Maps Python csv module to Rust csv crate:
    /// - csv.reader() → csv::Reader::from_reader()
    /// - csv.writer() → csv::Writer::from_writer()
    /// - csv.DictReader → csv with headers
    /// - csv.DictWriter → csv with headers
    ///
    /// # Complexity
    /// 4 (match with 4 branches - simplified for core operations)
    #[inline]
    pub(crate) fn try_convert_csv_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // Mark that we need csv crate
        self.ctx.needs_csv = true;

        let result = match method {
            // CSV Reader
            "reader" => {
                if arg_exprs.is_empty() {
                    bail!("csv.reader() requires at least 1 argument (file)");
                }
                let file = &arg_exprs[0];

                // csv.reader(file) → csv::Reader::from_reader(file)
                // Note: Real implementation needs more context for delimiter, etc.
                parse_quote! { csv::Reader::from_reader(#file) }
            }

            // CSV Writer
            "writer" => {
                if arg_exprs.is_empty() {
                    bail!("csv.writer() requires at least 1 argument (file)");
                }
                let file = &arg_exprs[0];

                // csv.writer(file) → csv::Writer::from_writer(file)
                parse_quote! { csv::Writer::from_writer(#file) }
            }

            // DictReader (simplified - actual implementation more complex)
            "DictReader" => {
                if arg_exprs.is_empty() {
                    bail!("csv.DictReader() requires at least 1 argument (file)");
                }
                let file = &arg_exprs[0];

                // csv.DictReader(file) → csv::ReaderBuilder::new().has_headers(true).from_reader(file)
                parse_quote! {
                    csv::ReaderBuilder::new()
                        .has_headers(true)
                        .from_reader(#file)
                }
            }

            // DictWriter (simplified)
            // DEPYLER-0426: Handle both positional and keyword arguments
            // csv.DictWriter(file, fieldnames=[...]) or csv.DictWriter(file, fieldnames=...)
            "DictWriter" => {
                // Get file argument (first positional arg required)
                if arg_exprs.is_empty() {
                    bail!("csv.DictWriter() requires at least 1 argument (file)");
                }
                let file = &arg_exprs[0];

                // Get fieldnames from either positional arg or kwargs
                let _fieldnames = if arg_exprs.len() >= 2 {
                    // Positional: csv.DictWriter(file, ['col1', 'col2'])
                    Some(&arg_exprs[1])
                } else {
                    // Keyword: csv.DictWriter(file, fieldnames=['col1', 'col2'])
                    kwargs
                        .iter()
                        .find(|(key, _)| key == "fieldnames")
                        .map(|(_, value)| value.to_rust_expr(self.ctx))
                        .transpose()?
                        .as_ref()
                        .map(|_| &arg_exprs[0]) // Placeholder, we don't use fieldnames yet
                };

                if _fieldnames.is_none() {
                    bail!("csv.DictWriter() requires fieldnames argument (positional or keyword)");
                }

                // csv.DictWriter(file, fieldnames) → csv::Writer::from_writer(file)
                // Note: fieldnames handling requires more context
                parse_quote! { csv::Writer::from_writer(#file) }
            }

            _ => {
                bail!("csv.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    // DEPYLER-COVERAGE-95: try_convert_os_method moved to stdlib_method_gen::os

    /// Try to convert os.environ method calls
    /// DEPYLER-0386: os.environ dictionary-like interface for environment variables
    ///
    /// Maps Python os.environ methods to Rust std::env:
    /// - os.environ.get(key) → std::env::var(key).ok()
    /// - os.environ.get(key, default) → std::env::var(key).unwrap_or_else(|_| default.to_string())
    ///
    /// # Complexity
    /// ≤10 (match with few branches)
    #[inline]
    pub(crate) fn try_convert_os_environ_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        let result = match method {
            "get" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("os.environ.get() requires 1 or 2 arguments");
                }

                if arg_exprs.len() == 1 {
                    // os.environ.get("KEY") → std::env::var("KEY").ok()
                    // Returns Option<String>: Some(value) if exists, None otherwise
                    // DEPYLER-0486: Handle Option-typed keys (e.g., from argparse nargs="?")
                    // If key is an &Option<String> or Option<String>, unwrap it first
                    let key = &arg_exprs[0];
                    let key_with_unwrap = if let HirExpr::Var(var_name) = &args[0] {
                        // DEPYLER-0644: Check if variable is already unwrapped (inside if-let body)
                        // If so, the key is already a concrete String, not Option<String>
                        // DEPYLER-0666: Also check if var_name is an UNWRAPPED name (value in map)
                        let is_unwrapped = self.ctx.option_unwrap_map.contains_key(var_name)
                            || self.ctx.option_unwrap_map.values().any(|v| v == var_name);
                        if is_unwrapped {
                            // Variable was already unwrapped, don't add .as_ref().unwrap()
                            key.clone()
                        } else if let Some(var_type) = self.ctx.var_types.get(var_name) {
                            if matches!(var_type, Type::Optional(_)) {
                                // Key is an Option type - unwrap it
                                parse_quote! { #key.as_ref().expect("value is None") }
                            } else {
                                key.clone()
                            }
                        } else {
                            key.clone()
                        }
                    } else {
                        key.clone()
                    };
                    parse_quote! { std::env::var(#key_with_unwrap).ok() }
                } else {
                    // os.environ.get("KEY", "default") → std::env::var("KEY").unwrap_or_else(|_| "default".to_string())
                    // Returns String: value if exists, default otherwise
                    // DEPYLER-0486: Auto-borrow variables (not string literals) to avoid move errors
                    let key = &arg_exprs[0];
                    let key_with_borrow = if matches!(&args[0], HirExpr::Var(_)) {
                        // Variable: borrow it to avoid moving in loops
                        parse_quote! { &#key }
                    } else {
                        // String literal or other expression: use as-is
                        key.clone()
                    };
                    let default = &arg_exprs[1];
                    parse_quote! {
                        std::env::var(#key_with_borrow).unwrap_or_else(|_| #default.to_string())
                    }
                }
            }
            _ => {
                return Ok(None);
            }
        };

        Ok(Some(result))
    }

    // DEPYLER-REFACTOR: try_convert_numpy_call, try_convert_numpy_call_nasa_mode moved to stdlib_numpy

    // DEPYLER-REFACTOR: try_convert_os_path_method moved to stdlib_os

    // DEPYLER-REFACTOR: bisect, heapq, copy methods moved to stdlib_misc
    // DEPYLER-COVERAGE-95: try_convert_itertools_method moved to stdlib_method_gen::itertools
    // DEPYLER-COVERAGE-95: try_convert_functools_method moved to stdlib_method_gen::functools
    // DEPYLER-COVERAGE-95: try_convert_warnings_method moved to stdlib_method_gen::warnings

    // DEPYLER-REFACTOR: sys, pickle, pprint, fractions methods moved to stdlib_misc
    // DEPYLER-COVERAGE-95: try_convert_pathlib_method moved to stdlib_method_gen::pathlib

    // DEPYLER-REFACTOR: convert_pathlib_instance_method moved to stdlib_pathlib

    // DEPYLER-REFACTOR: decimal, statistics methods moved to stdlib_misc
    // DEPYLER-COVERAGE-95: try_convert_random_method moved to stdlib_method_gen::random

    // DEPYLER-COVERAGE-95: try_convert_math_method moved to stdlib_method_gen::math

    /// Try to convert module method call (e.g., os.getcwd())
    #[inline]
    pub(crate) fn try_convert_module_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<Option<syn::Expr>> {
        // DEPYLER-0493: Handle constructor patterns for imported types
        // tempfile.NamedTempFile() → tempfile::NamedTempFile::new()
        if let HirExpr::Var(module_name) = object {
            // Check if this module is imported and has constructor pattern metadata
            if let Some(module_mapping) = self.ctx.imported_modules.get(module_name) {
                // Look up the Python name → Rust name mapping
                if let Some(rust_name) = module_mapping.item_map.get(method) {
                    // Check if this has a constructor pattern defined
                    if let Some(constructor_pattern) =
                        module_mapping.constructor_patterns.get(rust_name)
                    {
                        use crate::module_mapper::ConstructorPattern;

                        // Clone what we need to avoid borrow checker issues
                        let rust_path_str = format!("{}::{}", module_mapping.rust_path, rust_name);
                        let constructor_pattern_owned = constructor_pattern.clone();
                        let rust_name_owned = rust_name.clone(); // DEPYLER-0534: Clone for later use

                        // Build the full Rust path
                        let path_parts: Vec<&str> = rust_path_str.split("::").collect();
                        let mut path = quote! {};
                        for (i, part) in path_parts.iter().enumerate() {
                            let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                            if i == 0 {
                                path = quote! { #part_ident };
                            } else {
                                path = quote! { #path::#part_ident };
                            }
                        }

                        // Convert arguments
                        let arg_exprs: Vec<syn::Expr> = args
                            .iter()
                            .map(|arg| arg.to_rust_expr(self.ctx))
                            .collect::<Result<Vec<_>>>()?;

                        // GH-204: Handle collections module constructors specially
                        // Counter, deque, and defaultdict need custom conversion, not generic new()
                        if module_name == "collections" {
                            match method {
                                "Counter" => {
                                    return Ok(Some(
                                        crate::rust_gen::collection_constructors::convert_counter_builtin(
                                            self.ctx,
                                            &arg_exprs,
                                        )?,
                                    ));
                                }
                                "deque" => {
                                    return Ok(Some(
                                        crate::rust_gen::collection_constructors::convert_deque_builtin(
                                            self.ctx,
                                            &arg_exprs,
                                        )?,
                                    ));
                                }
                                "defaultdict" => {
                                    return Ok(Some(
                                        crate::rust_gen::collection_constructors::convert_defaultdict_builtin(
                                            self.ctx,
                                            &arg_exprs,
                                        )?,
                                    ));
                                }
                                _ => {} // Fall through to generic pattern handling
                            }
                        }

                        // Generate call based on constructor pattern
                        let result = match constructor_pattern_owned {
                            ConstructorPattern::New => {
                                // Struct type → use ::new() pattern
                                if arg_exprs.is_empty() {
                                    parse_quote! { #path::new() }
                                } else {
                                    parse_quote! { #path::new(#(#arg_exprs),*) }
                                }
                            }
                            ConstructorPattern::Method(method_name) => {
                                // Custom method (e.g., File::open())
                                let method_ident =
                                    syn::Ident::new(&method_name, proc_macro2::Span::call_site());
                                if arg_exprs.is_empty() {
                                    parse_quote! { #path::#method_ident() }
                                } else {
                                    parse_quote! { #path::#method_ident(#(#arg_exprs),*) }
                                }
                            }
                            ConstructorPattern::Function => {
                                // Regular function call
                                if arg_exprs.is_empty() {
                                    parse_quote! { #path() }
                                } else {
                                    parse_quote! { #path(#(#arg_exprs),*) }
                                }
                            }
                        };

                        // DEPYLER-0534: Unwrap fallible constructors
                        // tempfile functions return Result<T, io::Error>
                        // Use .unwrap() for simplicity (matches Python's exception-on-failure behavior)
                        let is_fallible_constructor = module_name == "tempfile"
                            && (rust_name_owned == "NamedTempFile"
                                || rust_name_owned == "TempFile"
                                || rust_name_owned == "TempDir");

                        // DEPYLER-1002: Set needs_tempfile when using tempfile constructors
                        if module_name == "tempfile" {
                            self.ctx.needs_tempfile = true;
                        }

                        let result = if is_fallible_constructor {
                            parse_quote! { #result.expect("operation failed") }
                        } else {
                            result
                        };

                        return Ok(Some(result));
                    }
                }
            }
        }

        // DEPYLER-0386: Handle os.environ.get() and other os.environ methods
        // os.environ.get('VAR') → std::env::var('VAR').ok()
        // os.environ.get('VAR', 'default') → std::env::var('VAR').unwrap_or_else(|_| 'default'.to_string())
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(module_name) = &**value {
                if module_name == "os" && attr == "environ" {
                    return self.try_convert_os_environ_method(method, args);
                }
                // DEPYLER-0430: Handle os.path.exists(), os.path.join(), etc.
                // os.path.exists(path) → Path::new(path).exists()
                // os.path.join(a, b) → PathBuf::from(a).join(b)
                if module_name == "os" && attr == "path" {
                    return self.try_convert_os_path_method(method, args);
                }
                // DEPYLER-0553: Handle datetime.datetime.method() calls
                // datetime.datetime.fromtimestamp(ts) → chrono::DateTime::from_timestamp(ts, 0)
                // datetime.datetime.now() → chrono::Local::now()
                if module_name == "datetime" && attr == "datetime" {
                    return self.try_convert_datetime_method(method, args);
                }
            }
        }

        if let HirExpr::Var(module_name) = object {
            // DEPYLER-99MODE-S9: Skip module routing if this variable is known to be a
            // local/declared variable (not a module). This prevents local variables named
            // 'copy', 'calendar', etc. from being mistakenly routed to module method handlers.
            let is_local_var = self.ctx.is_declared(module_name);
            let is_actually_imported = !is_local_var;

            // DEPYLER-0021: Handle struct module (pack, unpack, calcsize)
            if is_actually_imported && module_name == "struct" {
                return self.try_convert_struct_method(method, args);
            }

            // DEPYLER-STDLIB-MATH: Handle math module functions
            // math.sqrt(x) → x.sqrt()
            // math.sin(x) → x.sin()
            // math.pow(x, y) → x.powf(y)
            if is_actually_imported && module_name == "math" {
                return stdlib_method_gen::convert_math_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-RANDOM: Handle random module functions
            // random.random() → thread_rng().gen()
            // random.randint(a, b) → thread_rng().gen_range(a..=b)
            if is_actually_imported && module_name == "random" {
                return stdlib_method_gen::convert_random_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-STATISTICS: Handle statistics module functions
            // statistics.mean(data) → inline calculation
            // statistics.median(data) → sorted median calculation
            if is_actually_imported && module_name == "statistics" {
                return self.try_convert_statistics_method(method, args);
            }

            // DEPYLER-STDLIB-FRACTIONS: Handle fractions module functions
            // Fraction(1, 2) → Ratio::new(1, 2)
            // f.limit_denominator(100) → approximate with max denominator
            if is_actually_imported && module_name == "fractions" {
                return self.try_convert_fractions_method(method, args);
            }

            // DEPYLER-STDLIB-PATHLIB: Handle pathlib module functions
            // Path("/foo/bar").exists() → PathBuf::from("/foo/bar").exists()
            // Path("/foo").join("bar") → PathBuf::from("/foo").join("bar")
            if is_actually_imported && module_name == "pathlib" {
                return stdlib_method_gen::convert_pathlib_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-DATETIME: Handle datetime module functions
            // datetime.datetime.now() → Local::now().naive_local()
            // datetime.datetime.utcnow() → Utc::now().naive_utc()
            // datetime.date.today() → Local::now().date_naive()
            // DEPYLER-0594: Also handle "date" and "time" when imported directly
            // (from datetime import date; date.today())
            // DEPYLER-0188: Don't match module_name == "time" here - that's the time module!
            // Only match "date" for `from datetime import date` pattern.
            // The time module (import time; time.time()) is handled separately below.

            // DEPYLER-1069: Handle date.min/max vs datetime.min/max specially
            // date.min → DepylerDate(1, 1, 1), datetime.min → DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0)
            if is_actually_imported
                && (module_name == "datetime" || module_name == "date")
                && (method == "min" || method == "max")
            {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if module_name == "date" {
                    // date.min / date.max
                    if nasa_mode {
                        self.ctx.needs_depyler_date = true;
                        return Ok(Some(if method == "min" {
                            parse_quote! { DepylerDate::new(1, 1, 1) }
                        } else {
                            parse_quote! { DepylerDate::new(9999, 12, 31) }
                        }));
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(Some(if method == "min" {
                            parse_quote! { chrono::NaiveDate::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDate::MAX }
                        }));
                    }
                } else {
                    // datetime.min / datetime.max
                    if nasa_mode {
                        self.ctx.needs_depyler_datetime = true;
                        return Ok(Some(if method == "min" {
                            parse_quote! { DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0) }
                        } else {
                            parse_quote! { DepylerDateTime::new(9999, 12, 31, 23, 59, 59, 999999) }
                        }));
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(Some(if method == "min" {
                            parse_quote! { chrono::NaiveDateTime::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDateTime::MAX }
                        }));
                    }
                }
            }

            // DEPYLER-1069: Handle date.today() vs datetime.today() separately
            // date.today() → DepylerDate::today(), datetime.today() → DepylerDateTime::today()
            if is_actually_imported
                && (module_name == "datetime" || module_name == "date")
                && method == "today"
                && args.is_empty()
            {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if module_name == "date" {
                    if nasa_mode {
                        self.ctx.needs_depyler_date = true;
                        return Ok(Some(parse_quote! { DepylerDate::today() }));
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(Some(parse_quote! { chrono::Local::now().date_naive() }));
                    }
                } else {
                    // datetime.today()
                    if nasa_mode {
                        self.ctx.needs_depyler_datetime = true;
                        return Ok(Some(parse_quote! { DepylerDateTime::today() }));
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(Some(parse_quote! { chrono::Local::now().naive_local() }));
                    }
                }
            }

            if is_actually_imported && (module_name == "datetime" || module_name == "date") {
                return self.try_convert_datetime_method(method, args);
            }

            // DEPYLER-1069: Handle datetime.time class attributes (min, max)
            // These are only valid for datetime.time class, not the time module
            // time.min → (0, 0, 0, 0), time.max → (23, 59, 59, 999999)
            if is_actually_imported && module_name == "time" && (method == "min" || method == "max")
            {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if nasa_mode {
                    // Return tuple (hour, minute, second, microsecond)
                    return Ok(Some(if method == "min" {
                        parse_quote! { (0u32, 0u32, 0u32, 0u32) }
                    } else {
                        parse_quote! { (23u32, 59u32, 59u32, 999999u32) }
                    }));
                } else {
                    self.ctx.needs_chrono = true;
                    return Ok(Some(if method == "min" {
                        parse_quote! { chrono::NaiveTime::MIN }
                    } else {
                        parse_quote! { chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999999).expect("invalid time") }
                    }));
                }
            }

            // DEPYLER-0595: Handle bytes class methods
            // bytes.fromhex("aabbcc") → hex string to byte array
            if is_actually_imported
                && module_name == "bytes"
                && method == "fromhex"
                && args.len() == 1
            {
                let hex_str = args[0].to_rust_expr(self.ctx)?;
                // Convert hex string to Vec<u8> using inline parsing
                return Ok(Some(parse_quote! {
                    (#hex_str).as_bytes()
                        .chunks(2)
                        .map(|c| u8::from_str_radix(std::str::from_utf8(c).expect("parse failed"), 16).expect("parse failed"))
                        .collect::<Vec<u8>>()
                }));
            }

            // DEPYLER-STDLIB-DECIMAL: Handle decimal module functions
            // decimal.Decimal("123.45") → Decimal::from_str("123.45")
            // Note: Decimal() constructor is handled separately in convert_call
            if is_actually_imported && module_name == "decimal" {
                return self.try_convert_decimal_method(method, args);
            }

            // DEPYLER-STDLIB-JSON: Handle json module functions
            // json.dumps(obj) → serde_json::to_string(&obj)
            // json.loads(s) → serde_json::from_str(&s)
            if is_actually_imported && module_name == "json" {
                return stdlib_method_gen::convert_json_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-RE: Regular expressions module
            if is_actually_imported && module_name == "re" {
                return stdlib_method_gen::convert_re_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-STRING: String module utilities
            if is_actually_imported && module_name == "string" {
                return stdlib_method_gen::convert_string_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-TIME: Time module
            if is_actually_imported && module_name == "time" {
                return stdlib_method_gen::convert_time_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-SHUTIL: Shell utilities for file operations
            // shutil.copy(src, dst) → std::fs::copy(src, dst)
            // shutil.copy2(src, dst) → std::fs::copy(src, dst)
            // shutil.move(src, dst) → std::fs::rename(src, dst)
            if is_actually_imported && module_name == "shutil" {
                return stdlib_method_gen::convert_shutil_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-CSV: CSV file operations
            // DEPYLER-0426: Pass kwargs for DictWriter(file, fieldnames=...)
            if is_actually_imported && module_name == "csv" {
                return self.try_convert_csv_method(method, args, kwargs);
            }

            // DEPYLER-0380: os module operations (getenv, etc.)
            // Must be checked before os.path to handle non-path os functions
            if is_actually_imported && module_name == "os" {
                if let Some(result) = stdlib_method_gen::convert_os_method(method, args, self.ctx)?
                {
                    return Ok(Some(result));
                }
                // Fall through to os.path handler if method not recognized
            }

            // DEPYLER-STDLIB-OSPATH: os.path file system operations
            // Only match the actual module "os.path", not variables named "path"
            // Variables named "path" are typically PathBuf instances from Path() constructor
            if is_actually_imported && module_name == "os.path" {
                return self.try_convert_os_path_method(method, args);
            }

            // DEPYLER-STDLIB-BASE64: Base64 encoding/decoding operations
            if is_actually_imported && module_name == "base64" {
                return self.try_convert_base64_method(method, args);
            }

            // DEPYLER-STDLIB-SECRETS: Cryptographically strong random operations
            if is_actually_imported && module_name == "secrets" {
                return self.try_convert_secrets_method(method, args);
            }

            // DEPYLER-STDLIB-HASHLIB: Cryptographic hash functions
            if is_actually_imported && module_name == "hashlib" {
                return self.try_convert_hashlib_method(method, args);
            }

            // DEPYLER-STDLIB-UUID: UUID generation (RFC 4122)
            if is_actually_imported && module_name == "uuid" {
                return self.try_convert_uuid_method(method, args);
            }

            // DEPYLER-STDLIB-HMAC: HMAC authentication
            if is_actually_imported && module_name == "hmac" {
                return self.try_convert_hmac_method(method, args);
            }

            // DEPYLER-0430: platform module - system information
            if is_actually_imported && module_name == "platform" {
                return self.try_convert_platform_method(method, args);
            }

            // DEPYLER-STDLIB-BINASCII: Binary/ASCII conversions
            if is_actually_imported && module_name == "binascii" {
                return self.try_convert_binascii_method(method, args);
            }

            // DEPYLER-STDLIB-URLLIB-PARSE: URL parsing and encoding
            if is_actually_imported && (module_name == "urllib.parse" || module_name == "parse") {
                return self.try_convert_urllib_parse_method(method, args);
            }

            // DEPYLER-STDLIB-FNMATCH: Unix shell-style pattern matching
            if is_actually_imported && module_name == "fnmatch" {
                return self.try_convert_fnmatch_method(method, args);
            }

            // DEPYLER-STDLIB-SHLEX: Shell command line lexing
            if is_actually_imported && module_name == "shlex" {
                return self.try_convert_shlex_method(method, args);
            }

            // DEPYLER-STDLIB-TEXTWRAP: Text wrapping and formatting
            if is_actually_imported && module_name == "textwrap" {
                return self.try_convert_textwrap_method(method, args);
            }

            // DEPYLER-STDLIB-BISECT: Binary search for sorted sequences
            if is_actually_imported && module_name == "bisect" {
                return self.try_convert_bisect_method(method, args);
            }

            // DEPYLER-STDLIB-HEAPQ: Heap queue algorithm (priority queue)
            if is_actually_imported && module_name == "heapq" {
                return self.try_convert_heapq_method(method, args);
            }

            // DEPYLER-STDLIB-COPY: Shallow and deep copy operations
            if is_actually_imported && module_name == "copy" {
                return self.try_convert_copy_method(method, args);
            }

            // DEPYLER-STDLIB-ITERTOOLS: Iterator combinatorics and lazy evaluation
            if is_actually_imported && module_name == "itertools" {
                return stdlib_method_gen::convert_itertools_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-FUNCTOOLS: Higher-order functions
            if is_actually_imported && module_name == "functools" {
                return stdlib_method_gen::convert_functools_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-WARNINGS: Warning control
            if is_actually_imported && module_name == "warnings" {
                return stdlib_method_gen::convert_warnings_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-SYS: System-specific parameters and functions
            if is_actually_imported && module_name == "sys" {
                return self.try_convert_sys_method(method, args);
            }

            // DEPYLER-STDLIB-PICKLE: Object serialization
            if is_actually_imported && module_name == "pickle" {
                return self.try_convert_pickle_method(method, args);
            }

            // DEPYLER-STDLIB-PPRINT: Pretty printing
            if is_actually_imported && module_name == "pprint" {
                return self.try_convert_pprint_method(method, args);
            }

            // DEPYLER-0424: Calendar module - date/time calculations
            if is_actually_imported && module_name == "calendar" {
                return self.try_convert_calendar_method(method, args);
            }

            // DEPYLER-0335 FIX #2: Get rust_path and rust_name before converting args (avoid borrow conflict)
            let module_info = self.ctx.imported_modules.get(module_name).and_then(|mapping| {
                mapping
                    .item_map
                    .get(method)
                    .map(|rust_name| (mapping.rust_path.clone(), rust_name.clone()))
            });

            if let Some((rust_path, rust_name)) = module_info {
                // Convert args
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                // DEPYLER-0335 FIX #2: Special handling for math module functions (use method syntax)
                // Python: math.sqrt(x) → Rust: x.sqrt() or f64::sqrt(x)
                if module_name == "math" && !arg_exprs.is_empty() {
                    let receiver = &arg_exprs[0];
                    let method_ident = syn::Ident::new(&rust_name, proc_macro2::Span::call_site());
                    return Ok(Some(parse_quote! { (#receiver).#method_ident() }));
                }

                // DEPYLER-0335 FIX #2: Use rust_path from mapping instead of hardcoding "std"
                // Build the Rust function path using the module's rust_path

                // DEPYLER-0840: Handle macro names (ending with !) specially
                // Macros like "join!" cannot be split and used as identifiers
                if rust_name.ends_with('!') {
                    // This is a macro - handle it specially
                    // For now, skip macro-based mappings as they need special handling
                    // Note: Implement proper macro invocation support
                    return Ok(None);
                }

                let path_parts: Vec<&str> = rust_name.split("::").collect();

                // Start with the module's rust_path instead of hardcoded "std"
                let base_path: syn::Path =
                    syn::parse_str(&rust_path).unwrap_or_else(|_| parse_quote! { std });
                let mut path = quote! { #base_path };

                for part in path_parts {
                    let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                    path = quote! { #path::#part_ident };
                }

                // Special handling for certain functions
                let result = match rust_name.as_str() {
                    "env::current_dir" => {
                        // current_dir returns Result<PathBuf>, we need to convert to String
                        parse_quote! {
                            #path().expect("operation failed").to_string_lossy().to_string()
                        }
                    }
                    "Regex::new" => {
                        // re.compile(pattern) -> Regex::new(pattern)
                        if arg_exprs.is_empty() {
                            bail!("re.compile() requires a pattern argument");
                        }
                        let pattern = &arg_exprs[0];
                        parse_quote! {
                            regex::Regex::new(#pattern).expect("parse failed")
                        }
                    }
                    _ => {
                        if arg_exprs.is_empty() {
                            parse_quote! { #path() }
                        } else {
                            parse_quote! { #path(#(#arg_exprs),*) }
                        }
                    }
                };
                return Ok(Some(result));
            }
        }
        Ok(None)
    }
}
