//! String method handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size. Contains the `convert_string_method`
//! handler covering: upper, lower, strip, startswith, endswith, split, join,
//! replace, find, count, isdigit, isalpha, and related string operations.

use crate::hir::*;
use crate::rust_gen::expr_gen::ExpressionConverter;
use anyhow::{bail, Result};
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Handle string methods (upper, lower, strip, startswith, endswith, split, join, replace, find, count, isdigit, isalpha)
    #[inline]
    pub(super) fn convert_string_method(
        &mut self,
        hir_object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // CB-200 Batch 14: Compute the effective object expression with type-aware conversion
        let obj = self.compute_string_method_receiver(hir_object, object_expr);

        match method {
            "upper" => self.str_upper(hir_object, &obj, arg_exprs),
            "lower" => self.str_lower(hir_object, &obj, arg_exprs),
            "strip" => self.str_strip(hir_object, &obj, arg_exprs),
            "startswith" => self.str_startswith(&obj, arg_exprs, hir_args),
            "endswith" => self.str_endswith(&obj, arg_exprs, hir_args),
            "split" => self.str_split(&obj, arg_exprs, hir_args),
            "rsplit" => self.str_rsplit(&obj, arg_exprs, hir_args),
            "join" => self.str_join(hir_object, object_expr, arg_exprs, hir_args),
            "replace" => self.str_replace(object_expr, arg_exprs, hir_args),
            "find" => self.str_find(object_expr, arg_exprs, hir_args),
            "count" => self.str_count(object_expr, arg_exprs, hir_args),
            "isdigit" => {
                self.str_char_predicate(hir_object, object_expr, arg_exprs, "isdigit", "is_numeric")
            }
            "isalpha" => self.str_char_predicate(
                hir_object,
                object_expr,
                arg_exprs,
                "isalpha",
                "is_alphabetic",
            ),
            "isspace" => self.str_char_predicate(
                hir_object,
                object_expr,
                arg_exprs,
                "isspace",
                "is_whitespace",
            ),
            "isalnum" => self.str_char_predicate(
                hir_object,
                object_expr,
                arg_exprs,
                "isalnum",
                "is_alphanumeric",
            ),
            "isnumeric" => self.str_char_predicate(
                hir_object,
                object_expr,
                arg_exprs,
                "isnumeric",
                "is_numeric",
            ),
            "isascii" => {
                self.str_char_predicate(hir_object, object_expr, arg_exprs, "isascii", "is_ascii")
            }
            "isdecimal" => self.str_char_predicate(
                hir_object,
                object_expr,
                arg_exprs,
                "isdecimal",
                "is_ascii_digit",
            ),
            "lstrip" => self.str_lstrip(object_expr, arg_exprs),
            "rstrip" => self.str_rstrip(object_expr, arg_exprs),
            "encode" => Ok(parse_quote! { #object_expr.as_bytes().to_vec() }),
            "decode" => Ok(parse_quote! { String::from_utf8_lossy(&#obj).to_string() }),
            "title" => self.str_title(object_expr, arg_exprs),
            "index" => self.str_index(object_expr, arg_exprs, hir_args),
            "rfind" => self.str_rfind(object_expr, arg_exprs, hir_args),
            "rindex" => self.str_rindex(object_expr, arg_exprs, hir_args),
            "center" => self.str_justify(object_expr, arg_exprs, "center"),
            "ljust" => self.str_justify(object_expr, arg_exprs, "ljust"),
            "rjust" => self.str_justify(object_expr, arg_exprs, "rjust"),
            "zfill" => self.str_zfill(object_expr, arg_exprs),
            "capitalize" => self.str_capitalize(object_expr, arg_exprs),
            "swapcase" => self.str_swapcase(object_expr, arg_exprs),
            "expandtabs" => self.str_expandtabs(object_expr, arg_exprs),
            "splitlines" => self.str_splitlines(object_expr, arg_exprs),
            "partition" => self.str_partition(object_expr, arg_exprs),
            "casefold" => self.str_casefold(object_expr, arg_exprs),
            "isprintable" => self.str_isprintable(hir_object, object_expr, arg_exprs),
            "isupper" => self.str_isupper(hir_object, object_expr, arg_exprs),
            "islower" => self.str_islower(hir_object, object_expr, arg_exprs),
            "istitle" => self.str_istitle(object_expr, arg_exprs),
            "isidentifier" => self.str_isidentifier(object_expr, arg_exprs),
            "hex" => self.str_hex(object_expr, arg_exprs),
            "format" => self.str_format(object_expr, arg_exprs),
            _ => bail!("Unknown string method: {}", method),
        }
    }

    // CB-200 Batch 14: Compute the effective receiver for string method calls
    // Handles serde_json::Value and DepylerValue type conversions
    fn compute_string_method_receiver(
        &self,
        hir_object: &HirExpr,
        object_expr: &syn::Expr,
    ) -> syn::Expr {
        // DEPYLER-0564: Convert serde_json::Value to &str for string method calls
        let needs_json_conversion = self.needs_value_to_string_conversion(hir_object)
            || self.rust_expr_needs_value_conversion(object_expr);

        // DEPYLER-1064: Extract string from DepylerValue before calling string methods
        let is_depyler_var = if let HirExpr::Var(var_name) = hir_object {
            self.ctx.type_mapper.nasa_mode
                && self.ctx.var_types.get(var_name).is_some_and(|t| {
                    matches!(t, Type::Unknown)
                        || matches!(t, Type::Custom(n) if n == "Any" || n == "object")
                })
        } else {
            false
        };

        if needs_json_conversion {
            parse_quote! { #object_expr.as_str().unwrap_or_default() }
        } else if is_depyler_var {
            parse_quote! { #object_expr.to_string() }
        } else {
            object_expr.clone()
        }
    }

    // CB-200 Batch 13: Helper to extract pattern-compatible expression from HIR arg
    fn extract_pattern_expr(&self, hir_arg: &HirExpr, arg_expr: &syn::Expr) -> syn::Expr {
        match hir_arg {
            HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
            HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => arg_expr.clone(),
            _ => {
                let arg = arg_expr;
                parse_quote! { &#arg }
            }
        }
    }

    // CB-200 Batch 13: Check if hir_object is a char iteration variable
    fn is_char_iter_var(&self, hir_object: &HirExpr) -> bool {
        if let HirExpr::Var(name) = hir_object {
            self.ctx.char_iter_vars.contains(name.as_str())
        } else {
            false
        }
    }

    fn str_upper(
        &self,
        hir_object: &HirExpr,
        obj: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("upper() takes no arguments");
        }
        if self.is_char_iter_var(hir_object) {
            Ok(parse_quote! { #obj.to_uppercase().to_string() })
        } else {
            Ok(parse_quote! { #obj.to_uppercase() })
        }
    }

    fn str_lower(
        &self,
        hir_object: &HirExpr,
        obj: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("lower() takes no arguments");
        }
        if self.is_char_iter_var(hir_object) {
            Ok(parse_quote! { #obj.to_lowercase().to_string() })
        } else {
            Ok(parse_quote! { #obj.to_lowercase() })
        }
    }

    fn str_strip(
        &self,
        hir_object: &HirExpr,
        obj: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            let is_likely_char = if let HirExpr::Var(var_name) = hir_object {
                self.ctx.char_iter_vars.contains(var_name)
                    || var_name == "char"
                    || var_name == "ch"
                    || var_name == "c"
                    || var_name == "character"
            } else {
                false
            };
            if is_likely_char {
                return Ok(parse_quote! { !#obj.is_whitespace() });
            }
            Ok(parse_quote! { #obj.trim().to_string() })
        } else {
            let chars = &arg_exprs[0];
            Ok(parse_quote! { #obj.trim_matches(|c: char| #chars.contains(c)).to_string() })
        }
    }

    fn str_startswith(
        &self,
        obj: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if hir_args.len() != 1 {
            bail!("startswith() requires exactly one argument");
        }
        let prefix = self.extract_pattern_expr(&hir_args[0], &arg_exprs[0]);
        Ok(parse_quote! { #obj.starts_with(#prefix) })
    }

    fn str_endswith(
        &self,
        obj: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if hir_args.len() != 1 {
            bail!("endswith() requires exactly one argument");
        }
        let suffix = self.extract_pattern_expr(&hir_args[0], &arg_exprs[0]);
        Ok(parse_quote! { #obj.ends_with(#suffix) })
    }

    fn str_split(
        &self,
        obj: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            Ok(
                parse_quote! { #obj.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() },
            )
        } else if arg_exprs.len() == 1 {
            let sep = self.extract_pattern_expr(&hir_args[0], &arg_exprs[0]);
            Ok(parse_quote! { #obj.split(#sep).map(|s| s.to_string()).collect::<Vec<String>>() })
        } else if arg_exprs.len() == 2 {
            let sep = self.extract_pattern_expr(&hir_args[0], &arg_exprs[0]);
            let maxsplit = &arg_exprs[1];
            Ok(
                parse_quote! { #obj.splitn((#maxsplit + 1) as usize, #sep).map(|s| s.to_string()).collect::<Vec<String>>() },
            )
        } else {
            bail!("split() accepts at most 2 arguments (separator, maxsplit)");
        }
    }

    fn str_rsplit(
        &self,
        obj: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            Ok(
                parse_quote! { #obj.split_whitespace().rev().map(|s| s.to_string()).collect::<Vec<String>>() },
            )
        } else if arg_exprs.len() == 1 {
            let sep = self.extract_pattern_expr(&hir_args[0], &arg_exprs[0]);
            Ok(parse_quote! { #obj.rsplit(#sep).map(|s| s.to_string()).collect::<Vec<String>>() })
        } else if arg_exprs.len() == 2 {
            let sep = self.extract_pattern_expr(&hir_args[0], &arg_exprs[0]);
            let maxsplit = &arg_exprs[1];
            Ok(
                parse_quote! { #obj.rsplitn((#maxsplit + 1) as usize, #sep).map(|s| s.to_string()).collect::<Vec<String>>() },
            )
        } else {
            bail!("rsplit() accepts at most 2 arguments (separator, maxsplit)");
        }
    }

    fn str_join(
        &self,
        hir_object: &HirExpr,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if hir_args.len() != 1 {
            bail!("join() requires exactly one argument");
        }
        let iterable = &arg_exprs[0];
        let separator = match hir_object {
            HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
            _ => object_expr.clone(),
        };
        let needs_collect = match &hir_args[0] {
            HirExpr::GeneratorExp { .. } => true,
            HirExpr::Call { func, .. }
                if func == "map" || func == "filter" || func == "iter" || func == "enumerate" =>
            {
                true
            }
            _ => false,
        };
        if needs_collect {
            Ok(parse_quote! { #iterable.collect::<Vec<_>>().join(#separator) })
        } else {
            Ok(parse_quote! { #iterable.join(#separator) })
        }
    }

    fn str_replace(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if hir_args.len() < 2 {
            bail!("str.replace() requires at least 2 arguments (old, new), got {}", hir_args.len());
        }
        if hir_args.len() > 3 {
            bail!("replace() requires 2 or 3 arguments");
        }
        let old = self.extract_pattern_expr(&hir_args[0], &arg_exprs[0]);
        let new: syn::Expr = match &hir_args[1] {
            HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
            HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => arg_exprs[1].clone(),
            _ => {
                let arg = &arg_exprs[1];
                parse_quote! { &#arg }
            }
        };
        if hir_args.len() == 3 {
            let count = &arg_exprs[2];
            Ok(parse_quote! { #object_expr.replacen(#old, #new, #count as usize) })
        } else {
            Ok(parse_quote! { #object_expr.replace(#old, #new) })
        }
    }

    fn str_find(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if hir_args.is_empty() || hir_args.len() > 2 {
            bail!("find() requires 1 or 2 arguments, got {}", hir_args.len());
        }
        let substring = self.extract_pattern_expr(&hir_args[0], &arg_exprs[0]);
        if hir_args.len() == 2 {
            let start = &arg_exprs[1];
            Ok(parse_quote! {
                #object_expr[#start as usize..].find(#substring)
                    .map(|i| (i + #start as usize) as i32)
                    .unwrap_or(-1)
            })
        } else {
            Ok(parse_quote! {
                #object_expr.find(#substring)
                    .map(|i| i as i32)
                    .unwrap_or(-1)
            })
        }
    }

    fn str_count(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if hir_args.len() != 1 {
            bail!("count() requires exactly one argument");
        }
        let substring: syn::Expr = match &hir_args[0] {
            HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
            _ => {
                let arg = &arg_exprs[0];
                parse_quote! { &*#arg }
            }
        };
        Ok(parse_quote! { #object_expr.matches(#substring).count() as i32 })
    }

    // CB-200 Batch 13: Unified handler for char predicate methods (isdigit, isalpha, etc.)
    fn str_char_predicate(
        &self,
        hir_object: &HirExpr,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        method_name: &str,
        char_method: &str,
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("{}() takes no arguments", method_name);
        }
        if let HirExpr::Var(var_name) = hir_object {
            if self.ctx.char_iter_vars.contains(var_name) {
                let char_ident = syn::Ident::new(char_method, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #object_expr.#char_ident() });
            }
        }
        let char_ident = syn::Ident::new(char_method, proc_macro2::Span::call_site());
        Ok(parse_quote! { #object_expr.chars().all(|c| c.#char_ident()) })
    }

    fn str_lstrip(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            Ok(parse_quote! { #object_expr.trim_start().to_string() })
        } else {
            let chars = &arg_exprs[0];
            Ok(
                parse_quote! { #object_expr.trim_start_matches(|c: char| #chars.contains(c)).to_string() },
            )
        }
    }

    fn str_rstrip(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            Ok(parse_quote! { #object_expr.trim_end().to_string() })
        } else {
            let chars = &arg_exprs[0];
            Ok(
                parse_quote! { #object_expr.trim_end_matches(|c: char| #chars.contains(c)).to_string() },
            )
        }
    }

    fn str_title(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("title() takes no arguments");
        }
        Ok(parse_quote! {
            #object_expr
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
    }

    fn str_index(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if hir_args.len() != 1 {
            bail!("index() requires exactly one argument");
        }
        let substring = match &hir_args[0] {
            HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
            _ => arg_exprs[0].clone(),
        };
        Ok(
            parse_quote! { #object_expr.find(#substring).map(|i| i as i32).expect("substring not found") },
        )
    }

    fn str_rfind(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if hir_args.len() != 1 {
            bail!("rfind() requires exactly one argument");
        }
        let substring = match &hir_args[0] {
            HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
            _ => arg_exprs[0].clone(),
        };
        Ok(parse_quote! { #object_expr.rfind(#substring).map(|i| i as i32).unwrap_or(-1) })
    }

    fn str_rindex(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if hir_args.len() != 1 {
            bail!("rindex() requires exactly one argument");
        }
        let substring = match &hir_args[0] {
            HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
            _ => arg_exprs[0].clone(),
        };
        Ok(
            parse_quote! { #object_expr.rfind(#substring).map(|i| i as i32).expect("substring not found") },
        )
    }

    // CB-200 Batch 13: Unified handler for center/ljust/rjust
    fn str_justify(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        kind: &str,
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() || arg_exprs.len() > 2 {
            bail!("{}() requires 1 or 2 arguments", kind);
        }
        let width = &arg_exprs[0];
        let fillchar = if arg_exprs.len() == 2 { &arg_exprs[1] } else { &parse_quote!(" ") };
        match kind {
            "center" => Ok(parse_quote! {
                {
                    let s = #object_expr;
                    let width = #width as usize;
                    let fillchar = #fillchar;
                    if s.len() >= width {
                        s.to_string()
                    } else {
                        let total_pad = width - s.len();
                        let left_pad = total_pad / 2;
                        let right_pad = total_pad - left_pad;
                        format!("{}{}{}", fillchar.repeat(left_pad), s, fillchar.repeat(right_pad))
                    }
                }
            }),
            "ljust" => Ok(parse_quote! {
                {
                    let s = #object_expr;
                    let width = #width as usize;
                    let fillchar = #fillchar;
                    if s.len() >= width { s.to_string() }
                    else { format!("{}{}", s, fillchar.repeat(width - s.len())) }
                }
            }),
            "rjust" => Ok(parse_quote! {
                {
                    let s = #object_expr;
                    let width = #width as usize;
                    let fillchar = #fillchar;
                    if s.len() >= width { s.to_string() }
                    else { format!("{}{}", fillchar.repeat(width - s.len()), s) }
                }
            }),
            _ => bail!("Unknown justify method: {}", kind),
        }
    }

    fn str_zfill(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("zfill() requires exactly 1 argument");
        }
        let width = &arg_exprs[0];
        Ok(parse_quote! {
            {
                let s = #object_expr;
                let width = #width as usize;
                if s.len() >= width {
                    s.to_string()
                } else {
                    let sign = if s.starts_with('-') || s.starts_with('+') { &s[0..1] } else { "" };
                    let num = if !sign.is_empty() { &s[1..] } else { &s[..] };
                    format!("{}{}{}", sign, "0".repeat(width - s.len()), num)
                }
            }
        })
    }

    fn str_capitalize(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("capitalize() takes no arguments");
        }
        Ok(parse_quote! {
            {
                let s = #object_expr;
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                }
            }
        })
    }

    fn str_swapcase(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("swapcase() takes no arguments");
        }
        Ok(parse_quote! {
            #object_expr.chars().map(|c| {
                if c.is_uppercase() { c.to_lowercase().to_string() }
                else { c.to_uppercase().to_string() }
            }).collect::<String>()
        })
    }

    fn str_expandtabs(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            Ok(parse_quote! { #object_expr.replace("\t", &" ".repeat(8)) })
        } else if arg_exprs.len() == 1 {
            let tabsize_expr = &arg_exprs[0];
            Ok(parse_quote! { #object_expr.replace("\t", &" ".repeat(#tabsize_expr as usize)) })
        } else {
            bail!("expandtabs() takes 0 or 1 arguments")
        }
    }

    fn str_splitlines(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("splitlines() takes no arguments");
        }
        Ok(parse_quote! { #object_expr.lines().map(|s| s.to_string()).collect::<Vec<String>>() })
    }

    fn str_partition(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("partition() requires exactly 1 argument (separator)");
        }
        let sep = &arg_exprs[0];
        Ok(parse_quote! {
            {
                let s = #object_expr;
                let sep_str = #sep;
                if let Some(pos) = s.find(sep_str) {
                    let before = &s[..pos];
                    let after = &s[pos + sep_str.len()..];
                    (before.to_string(), sep_str.to_string(), after.to_string())
                } else {
                    (s.to_string(), String::new(), String::new())
                }
            }
        })
    }

    fn str_casefold(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("casefold() takes no arguments");
        }
        Ok(parse_quote! { #object_expr.to_lowercase() })
    }

    fn str_isprintable(
        &self,
        hir_object: &HirExpr,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("isprintable() takes no arguments");
        }
        if let HirExpr::Var(var_name) = hir_object {
            if self.ctx.char_iter_vars.contains(var_name) {
                return Ok(
                    parse_quote! { !#object_expr.is_control() || #object_expr == '\t' || #object_expr == '\n' || #object_expr == '\r' },
                );
            }
        }
        Ok(
            parse_quote! { #object_expr.chars().all(|c| !c.is_control() || c == '\t' || c == '\n' || c == '\r') },
        )
    }

    fn str_isupper(
        &self,
        hir_object: &HirExpr,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("isupper() takes no arguments");
        }
        if let HirExpr::Var(var_name) = hir_object {
            if self.ctx.char_iter_vars.contains(var_name) {
                return Ok(
                    parse_quote! { !#object_expr.is_alphabetic() || #object_expr.is_uppercase() },
                );
            }
        }
        Ok(parse_quote! { #object_expr.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) })
    }

    fn str_islower(
        &self,
        hir_object: &HirExpr,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("islower() takes no arguments");
        }
        if let HirExpr::Var(var_name) = hir_object {
            if self.ctx.char_iter_vars.contains(var_name) {
                return Ok(
                    parse_quote! { !#object_expr.is_alphabetic() || #object_expr.is_lowercase() },
                );
            }
        }
        Ok(parse_quote! { #object_expr.chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) })
    }

    fn str_istitle(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("istitle() takes no arguments");
        }
        Ok(parse_quote! {
            {
                let s = #object_expr;
                let mut prev_is_cased = false;
                s.chars().all(|c| {
                    let is_upper = c.is_uppercase();
                    let is_lower = c.is_lowercase();
                    let result = if c.is_alphabetic() {
                        if prev_is_cased { is_lower } else { is_upper }
                    } else { true };
                    prev_is_cased = c.is_alphabetic();
                    result
                })
            }
        })
    }

    fn str_isidentifier(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("isidentifier() takes no arguments");
        }
        Ok(parse_quote! {
            {
                let s = #object_expr;
                !s.is_empty() && s.chars().enumerate().all(|(i, c)| {
                    if i == 0 { c.is_alphabetic() || c == '_' }
                    else { c.is_alphanumeric() || c == '_' }
                })
            }
        })
    }

    fn str_hex(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("hex() takes no arguments");
        }
        Ok(parse_quote! { #object_expr.bytes().map(|b| format!("{:02x}", b)).collect::<String>() })
    }

    fn str_format(&self, object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            Ok(object_expr.clone())
        } else if arg_exprs.len() == 1 {
            let arg = &arg_exprs[0];
            Ok(parse_quote! { #object_expr.replacen("{}", &format!("{}", #arg), 1) })
        } else {
            let mut result: syn::Expr = parse_quote! { #object_expr.to_string() };
            for arg in arg_exprs {
                result = parse_quote! { #result.replacen("{}", &format!("{}", #arg), 1) };
            }
            Ok(result)
        }
    }
}
