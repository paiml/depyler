//! Attribute access and borrow conversion for ExpressionConverter
//!
//! Contains convert_attribute, convert_borrow, wrap_range_in_parens.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::rust_gen::keywords;
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_attribute(&mut self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        // DEPYLER-1402: Handle type(x).__name__ pattern
        if attr == "__name__" {
            if let HirExpr::Call { func, args, .. } = value {
                if func == "type" && args.len() == 1 {
                    let arg_expr = args[0].to_rust_expr(self.ctx)?;
                    return Ok(parse_quote! { std::any::type_name_of_val(&#arg_expr) });
                }
            }
        }

        // DEPYLER-0608: In cmd_* handlers, args.X → X (field is now a direct parameter)
        if self.ctx.in_cmd_handler {
            if let HirExpr::Var(var_name) = value {
                if var_name == "args"
                    && self.ctx.cmd_handler_args_fields.contains(&attr.to_string())
                {
                    let attr_ident = if keywords::is_rust_keyword(attr) {
                        syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                    } else {
                        syn::Ident::new(attr, proc_macro2::Span::call_site())
                    };
                    return Ok(parse_quote! { #attr_ident });
                }
            }
        }

        // DEPYLER-0200: Handle os.environ direct access
        if let HirExpr::Var(var_name) = value {
            if var_name == "os" && attr == "environ" {
                return Ok(parse_quote! {
                    std::env::vars().collect::<std::collections::HashMap<String, String>>()
                });
            }
        }

        // CB-200 Batch 12: Datetime class constants dispatcher
        if let Some(result) = self.try_convert_datetime_class_constant(value, attr)? {
            return Ok(result);
        }

        // CB-200 Batch 12: Variable heuristic-based attribute dispatch
        if let Some(result) = self.try_convert_var_heuristic_attribute(value, attr)? {
            return Ok(result);
        }

        // DEPYLER-0425: Handle subcommand field access (args.url → url)
        if let HirExpr::Var(var_name) = value {
            if (var_name == "args" || var_name.ends_with("args"))
                && self.ctx.argparser_tracker.has_subcommands()
            {
                let mut is_subcommand_field = false;
                for subcommand in self.ctx.argparser_tracker.subcommands.values() {
                    for arg in &subcommand.arguments {
                        if arg.rust_field_name() == attr {
                            is_subcommand_field = true;
                            break;
                        }
                    }
                    if is_subcommand_field {
                        break;
                    }
                }

                if is_subcommand_field {
                    let attr_ident = if keywords::is_rust_keyword(attr) {
                        syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                    } else {
                        syn::Ident::new(attr, proc_macro2::Span::call_site())
                    };
                    return Ok(parse_quote! { #attr_ident });
                }
            }
        }

        // Handle classmethod cls.ATTR → Self::ATTR and enum constants TypeName::CONSTANT
        if let HirExpr::Var(var_name) = value {
            if var_name == "cls" && self.ctx.is_classmethod {
                let attr_ident = if keywords::is_rust_keyword(attr) {
                    syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                } else {
                    syn::Ident::new(attr, proc_macro2::Span::call_site())
                };
                return Ok(parse_quote! { Self::#attr_ident });
            }

            let first_char = var_name.chars().next().unwrap_or('a');
            let is_type_name = first_char.is_uppercase();
            let is_constant =
                attr.chars().all(|c| c.is_uppercase() || c == '_' || c.is_ascii_digit());

            if is_type_name && is_constant {
                let type_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #type_ident::#attr_ident });
            }
        }

        // CB-200 Batch 12: Module constant dispatcher (math, string, re, sys, imported modules)
        if let Some(result) = self.try_convert_module_constant(value, attr)? {
            return Ok(result);
        }

        // CB-200 Batch 12: Instance property dispatcher (fractions, pathlib, datetime, timedelta)
        let value_expr = value.to_rust_expr(self.ctx)?;
        if let Some(result) = self.try_convert_instance_property(value, attr, &value_expr)? {
            return Ok(result);
        }

        // DEPYLER-0452: Check stdlib API mappings before default fallback
        if let Some(mapping) = self.ctx.stdlib_mappings.lookup("csv", "DictReader", attr) {
            let rust_code =
                mapping.generate_rust_code(&value_expr.to_token_stream().to_string(), &[]);
            if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
                return Ok(expr);
            }
        }
        if let Some(mapping) = self.ctx.stdlib_mappings.lookup("csv", "Reader", attr) {
            let rust_code =
                mapping.generate_rust_code(&value_expr.to_token_stream().to_string(), &[]);
            if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
                return Ok(expr);
            }
        }

        // Default behavior for non-module attributes
        let attr_ident = if keywords::is_rust_keyword(attr) {
            syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(attr, proc_macro2::Span::call_site())
        };

        // DEPYLER-1138: DepylerValue proxy methods require () for property-like access
        let depyler_value_properties = ["tag", "text", "find", "findall", "set"];
        if depyler_value_properties.contains(&attr) && !self.ctx.module_aliases.is_empty() {
            return Ok(parse_quote! { #value_expr.#attr_ident() });
        }

        // DEPYLER-0737: Check if this is a @property method access
        if self.ctx.property_methods.contains(attr) {
            Ok(parse_quote! { #value_expr.#attr_ident() })
        } else {
            Ok(parse_quote! { #value_expr.#attr_ident })
        }
    }

    /// CB-200 Batch 12: Handle datetime class constants (date.min, datetime.max, etc.)
    fn try_convert_datetime_class_constant(
        &mut self,
        value: &HirExpr,
        attr: &str,
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Var(var_name) = value else {
            return Ok(None);
        };
        let nasa_mode = self.ctx.type_mapper.nasa_mode;
        if !matches!(var_name.as_str(), "date" | "datetime" | "time" | "timedelta")
            || !matches!(attr, "min" | "max" | "resolution")
        {
            return Ok(None);
        }

        match var_name.as_str() {
            "date" => {
                self.ctx.needs_depyler_date = true;
                if nasa_mode {
                    Ok(Some(if attr == "min" {
                        parse_quote! { DepylerDate::new(1, 1, 1) }
                    } else {
                        parse_quote! { DepylerDate::new(9999, 12, 31) }
                    }))
                } else {
                    self.ctx.needs_chrono = true;
                    Ok(Some(if attr == "min" {
                        parse_quote! { chrono::NaiveDate::MIN }
                    } else {
                        parse_quote! { chrono::NaiveDate::MAX }
                    }))
                }
            }
            "datetime" => {
                self.ctx.needs_depyler_datetime = true;
                if nasa_mode {
                    Ok(Some(if attr == "min" {
                        parse_quote! { DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0) }
                    } else {
                        parse_quote! { DepylerDateTime::new(9999, 12, 31, 23, 59, 59, 999999) }
                    }))
                } else {
                    self.ctx.needs_chrono = true;
                    Ok(Some(if attr == "min" {
                        parse_quote! { chrono::NaiveDateTime::MIN }
                    } else {
                        parse_quote! { chrono::NaiveDateTime::MAX }
                    }))
                }
            }
            "time" => {
                if nasa_mode {
                    Ok(Some(if attr == "min" {
                        parse_quote! { (0u32, 0u32, 0u32, 0u32) }
                    } else if attr == "max" {
                        parse_quote! { (23u32, 59u32, 59u32, 999999u32) }
                    } else {
                        parse_quote! { (0u32, 0u32, 0u32, 1u32) }
                    }))
                } else {
                    self.ctx.needs_chrono = true;
                    Ok(Some(if attr == "min" {
                        parse_quote! { chrono::NaiveTime::MIN }
                    } else if attr == "max" {
                        parse_quote! { chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999999).expect("operation failed") }
                    } else {
                        parse_quote! { chrono::Duration::microseconds(1) }
                    }))
                }
            }
            "timedelta" => {
                self.ctx.needs_depyler_timedelta = true;
                if nasa_mode {
                    Ok(Some(if attr == "min" {
                        parse_quote! { DepylerTimeDelta::new(-999999999, 0, 0) }
                    } else if attr == "max" {
                        parse_quote! { DepylerTimeDelta::new(999999999, 86399, 999999) }
                    } else {
                        parse_quote! { DepylerTimeDelta::new(0, 0, 1) }
                    }))
                } else {
                    self.ctx.needs_chrono = true;
                    Ok(Some(if attr == "min" {
                        parse_quote! { chrono::Duration::min_value() }
                    } else if attr == "max" {
                        parse_quote! { chrono::Duration::max_value() }
                    } else {
                        parse_quote! { chrono::Duration::microseconds(1) }
                    }))
                }
            }
            _ => Ok(None),
        }
    }

    /// CB-200 Batch 12: Handle variable heuristic-based attributes (exception, tempfile, stats, path)
    fn try_convert_var_heuristic_attribute(
        &mut self,
        value: &HirExpr,
        attr: &str,
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Var(var_name) = value else {
            return Ok(None);
        };

        // DEPYLER-0517: Handle exception variable attributes
        let is_likely_exception = var_name == "e"
            || var_name == "err"
            || var_name == "error"
            || var_name == "exc"
            || var_name == "exception";

        if is_likely_exception && attr == "returncode" {
            return Ok(Some(parse_quote! { 1 }));
        }

        // DEPYLER-0535: Handle tempfile file handle attributes
        let is_likely_tempfile = var_name == "f"
            || var_name == "temp"
            || var_name == "tmp"
            || var_name.contains("temp")
            || var_name.contains("tmp");

        if is_likely_tempfile && attr == "name" {
            let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
            return Ok(Some(
                parse_quote! { #var_ident.path().to_string_lossy().to_string() },
            ));
        }

        // DEPYLER-0551: Handle os.stat_result attributes
        if let Some(result) = self.try_convert_stat_attribute(var_name, attr)? {
            return Ok(Some(result));
        }

        // DEPYLER-0551: Handle pathlib.Path attributes on variables
        if let Some(result) = self.try_convert_path_var_attribute(var_name, attr)? {
            return Ok(Some(result));
        }

        Ok(None)
    }

    /// CB-200 Batch 12: Handle os.stat_result attributes (st_size, st_mtime, etc.)
    fn try_convert_stat_attribute(
        &self,
        var_name: &str,
        attr: &str,
    ) -> Result<Option<syn::Expr>> {
        let is_likely_stats =
            var_name == "stats" || var_name == "stat" || var_name.ends_with("_stats");
        if !is_likely_stats {
            return Ok(None);
        }

        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
        match attr {
            "st_size" => Ok(Some(parse_quote! { #var_ident.len() as i64 })),
            "st_mtime" => Ok(Some(parse_quote! {
                #var_ident.modified().expect("operation failed").duration_since(std::time::UNIX_EPOCH).expect("operation failed").as_secs_f64()
            })),
            "st_ctime" => Ok(Some(parse_quote! {
                #var_ident.created().unwrap_or_else(|_| #var_ident.modified().expect("operation failed"))
                    .duration_since(std::time::UNIX_EPOCH).expect("operation failed").as_secs_f64()
            })),
            "st_atime" => Ok(Some(parse_quote! {
                #var_ident.accessed().expect("operation failed").duration_since(std::time::UNIX_EPOCH).expect("operation failed").as_secs_f64()
            })),
            "st_mode" => Ok(Some(parse_quote! { #var_ident.permissions().mode() })),
            _ => Ok(None),
        }
    }

    /// CB-200 Batch 12: Handle pathlib.Path variable attributes (name, suffix, stem, parent)
    fn try_convert_path_var_attribute(
        &self,
        var_name: &str,
        attr: &str,
    ) -> Result<Option<syn::Expr>> {
        let is_named_path = var_name == "path" || var_name.ends_with("_path");
        let is_typed_path = self
            .ctx
            .var_types
            .get(var_name)
            .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
            .unwrap_or(false);
        if !is_named_path && !is_typed_path {
            return Ok(None);
        }

        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
        match attr {
            "name" => Ok(Some(parse_quote! {
                #var_ident.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string()
            })),
            "suffix" => Ok(Some(parse_quote! {
                #var_ident.extension().map(|e| format!(".{}", e.to_str().expect("operation failed"))).unwrap_or_default()
            })),
            "stem" => Ok(Some(parse_quote! {
                #var_ident.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string()
            })),
            "parent" => Ok(Some(parse_quote! {
                #var_ident.parent().map(|p| p.to_path_buf()).unwrap_or_default()
            })),
            _ => Ok(None),
        }
    }

    /// CB-200 Batch 12: Handle module constant access (math.pi, string.digits, re.IGNORECASE, sys.argv, imported modules)
    fn try_convert_module_constant(
        &mut self,
        value: &HirExpr,
        attr: &str,
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Var(module_name) = value else {
            return Ok(None);
        };

        if module_name == "math" {
            return self.convert_math_constant(attr).map(Some);
        }
        if module_name == "string" {
            return self.convert_string_module_constant(attr).map(Some);
        }
        if module_name == "re" {
            return self.convert_re_constant(module_name, attr).map(Some);
        }
        if module_name == "sys" {
            return self.convert_sys_attribute(attr).map(Some);
        }

        // DEPYLER-0335 FIX #2: Imported module attribute mapping
        let module_info = self.ctx.imported_modules.get(module_name).and_then(|mapping| {
            mapping
                .item_map
                .get(attr)
                .map(|rust_name| (mapping.rust_path.clone(), rust_name.clone()))
        });

        if let Some((rust_path, rust_name)) = module_info {
            let path_parts: Vec<&str> = rust_name.split("::").collect();
            if path_parts.len() > 1 {
                let base_path: syn::Path =
                    syn::parse_str(&rust_path).unwrap_or_else(|_| parse_quote! { std });
                let mut path = quote! { #base_path };
                for part in path_parts {
                    let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                    path = quote! { #path::#part_ident };
                }
                return Ok(Some(parse_quote! { #path }));
            } else {
                let ident = syn::Ident::new(&rust_name, proc_macro2::Span::call_site());
                return Ok(Some(parse_quote! { #ident }));
            }
        }

        Ok(None)
    }

    /// CB-200 Batch 12: math module constants and first-class function values
    fn convert_math_constant(&self, attr: &str) -> Result<syn::Expr> {
        match attr {
            "pi" => Ok(parse_quote! { std::f64::consts::PI }),
            "e" => Ok(parse_quote! { std::f64::consts::E }),
            "tau" => Ok(parse_quote! { std::f64::consts::TAU }),
            "inf" => Ok(parse_quote! { f64::INFINITY }),
            "nan" => Ok(parse_quote! { f64::NAN }),
            "sin" => Ok(parse_quote! { f64::sin }),
            "cos" => Ok(parse_quote! { f64::cos }),
            "tan" => Ok(parse_quote! { f64::tan }),
            "asin" => Ok(parse_quote! { f64::asin }),
            "acos" => Ok(parse_quote! { f64::acos }),
            "atan" => Ok(parse_quote! { f64::atan }),
            "sqrt" => Ok(parse_quote! { f64::sqrt }),
            "exp" => Ok(parse_quote! { f64::exp }),
            "log" => Ok(parse_quote! { f64::ln }),
            "log10" => Ok(parse_quote! { f64::log10 }),
            "floor" => Ok(parse_quote! { f64::floor }),
            "ceil" => Ok(parse_quote! { f64::ceil }),
            "abs" => Ok(parse_quote! { f64::abs }),
            _ => bail!("math.{} is not a recognized constant or method", attr),
        }
    }

    /// CB-200 Batch 12: string module constants
    fn convert_string_module_constant(&self, attr: &str) -> Result<syn::Expr> {
        match attr {
            "ascii_lowercase" => Ok(parse_quote! { "abcdefghijklmnopqrstuvwxyz" }),
            "ascii_uppercase" => Ok(parse_quote! { "ABCDEFGHIJKLMNOPQRSTUVWXYZ" }),
            "ascii_letters" => {
                Ok(parse_quote! { "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" })
            }
            "digits" => Ok(parse_quote! { "0123456789" }),
            "hexdigits" => Ok(parse_quote! { "0123456789abcdefABCDEF" }),
            "octdigits" => Ok(parse_quote! { "01234567" }),
            "punctuation" => Ok(parse_quote! { "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~" }),
            "whitespace" => Ok(parse_quote! { " \t\n\r\x0b\x0c" }),
            "printable" => {
                Ok(parse_quote! { "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c" })
            }
            _ => bail!("string.{} is not a recognized constant", attr),
        }
    }

    /// CB-200 Batch 12: re module constants (regex flags)
    fn convert_re_constant(&self, module_name: &str, attr: &str) -> Result<syn::Expr> {
        match attr {
            "IGNORECASE" | "I" => Ok(parse_quote! { 2i32 }),
            "MULTILINE" | "M" => Ok(parse_quote! { 8i32 }),
            "DOTALL" | "S" => Ok(parse_quote! { 16i32 }),
            "VERBOSE" | "X" => Ok(parse_quote! { 64i32 }),
            "ASCII" | "A" => Ok(parse_quote! { 256i32 }),
            "LOCALE" | "L" => Ok(parse_quote! { 4i32 }),
            "UNICODE" | "U" => Ok(parse_quote! { 32i32 }),
            _ => {
                let module_ident = syn::Ident::new(module_name, proc_macro2::Span::call_site());
                let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                Ok(parse_quote! { #module_ident.#attr_ident })
            }
        }
    }

    /// CB-200 Batch 12: sys module attributes
    fn convert_sys_attribute(&self, attr: &str) -> Result<syn::Expr> {
        match attr {
            "argv" => Ok(parse_quote! { std::env::args().collect::<Vec<String>>() }),
            "platform" => {
                #[cfg(target_os = "linux")]
                let platform = "linux";
                #[cfg(target_os = "macos")]
                let platform = "darwin";
                #[cfg(target_os = "windows")]
                let platform = "win32";
                #[cfg(not(any(
                    target_os = "linux",
                    target_os = "macos",
                    target_os = "windows"
                )))]
                let platform = "unknown";
                Ok(parse_quote! { #platform.to_string() })
            }
            "stdin" => Ok(parse_quote! { std::io::stdin() }),
            "stdout" => Ok(parse_quote! { std::io::stdout() }),
            "stderr" => Ok(parse_quote! { std::io::stderr() }),
            "version_info" => Ok(parse_quote! { (3, 11) }),
            _ => bail!("sys.{} is not a recognized attribute", attr),
        }
    }

    /// CB-200 Batch 12: Handle instance property access (fractions, pathlib, datetime, timedelta)
    fn try_convert_instance_property(
        &mut self,
        value: &HirExpr,
        attr: &str,
        value_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        match attr {
            "numerator" => return Ok(Some(parse_quote! { *#value_expr.numer() })),
            "denominator" => return Ok(Some(parse_quote! { *#value_expr.denom() })),
            "stem" => {
                return Ok(Some(parse_quote! {
                    #value_expr.file_stem().expect("operation failed").to_str().expect("operation failed").to_string()
                }));
            }
            "suffix" => {
                return Ok(Some(parse_quote! {
                    #value_expr.extension()
                        .map(|e| format!(".{}", e.to_str().expect("operation failed")))
                        .unwrap_or_default()
                }));
            }
            "parent" => {
                return Ok(Some(parse_quote! {
                    #value_expr.parent().expect("operation failed").to_path_buf()
                }));
            }
            "parts" => {
                return Ok(Some(parse_quote! {
                    #value_expr.components()
                        .map(|c| c.as_os_str().to_str().expect("operation failed").to_string())
                        .collect::<Vec<_>>()
                }));
            }
            "year" | "month" | "day" | "hour" | "minute" | "second" | "microsecond" => {
                if let HirExpr::Var(var_name) = value {
                    if self.ctx.mut_option_params.contains(var_name) {
                        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                        let method_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                        return Ok(Some(
                            parse_quote! { #var_ident.as_ref().expect("value is None").#method_ident() as i32 },
                        ));
                    }
                }
                let method_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(Some(parse_quote! { #value_expr.#method_ident() as i32 }));
            }
            "days" => return Ok(Some(parse_quote! { #value_expr.days() as i32 })),
            "seconds" => return Ok(Some(parse_quote! { #value_expr.seconds() as i32 })),
            "microseconds" => return Ok(Some(parse_quote! { #value_expr.microseconds() as i32 })),
            _ => Ok(None),
        }
    }

    pub(crate) fn convert_borrow(&mut self, expr: &HirExpr, mutable: bool) -> Result<syn::Expr> {
        // CITL: Trace borrowing strategy decision
        #[cfg(feature = "decision-tracing")]
        let borrow_type = if mutable { "&mut" } else { "&" };
        trace_decision!(
            category = DecisionCategory::BorrowStrategy,
            name = "explicit_borrow",
            chosen = borrow_type,
            alternatives = ["&ref", "&mut_ref", "move", "clone"],
            confidence = 0.92
        );

        let expr_tokens = expr.to_rust_expr(self.ctx)?;
        if mutable {
            Ok(parse_quote! { &mut #expr_tokens })
        } else {
            Ok(parse_quote! { &#expr_tokens })
        }
    }

    /// DEPYLER-0511: Wrap range expressions in parentheses before method calls
    ///
    /// Ranges need parentheses when followed by method calls due to operator precedence.
    /// Without parens: `0..5.into_iter()` parses as `0..(5.into_iter())` ❌
    /// With parens: `(0..5).into_iter()` parses correctly ✅
    ///
    /// Detects syn::Expr::Range and wraps in syn::Expr::Paren.
    pub(crate) fn wrap_range_in_parens(&self, expr: syn::Expr) -> syn::Expr {
        match &expr {
            syn::Expr::Range(_) => {
                // Wrap range in parentheses
                parse_quote! { (#expr) }
            }
            _ => expr, // No wrapping needed for other expressions
        }
    }
}
