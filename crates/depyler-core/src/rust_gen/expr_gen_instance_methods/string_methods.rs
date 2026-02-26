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
        // DEPYLER-0564: Convert serde_json::Value to &str for string method calls
        // Check both HIR pattern and Rust expression pattern
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

        let obj = if needs_json_conversion {
            parse_quote! { #object_expr.as_str().unwrap_or_default() }
        } else if is_depyler_var {
            // Extract string from DepylerValue using to_string()
            parse_quote! { #object_expr.to_string() }
        } else {
            object_expr.clone()
        };

        match method {
            "upper" => {
                if !arg_exprs.is_empty() {
                    bail!("upper() takes no arguments");
                }
                // DEPYLER-99MODE-S9: char.to_uppercase() returns ToUppercase iterator,
                // needs .to_string() to produce String. String.to_uppercase() returns String.
                let is_char = if let HirExpr::Var(name) = hir_object {
                    self.ctx.char_iter_vars.contains(name.as_str())
                } else {
                    false
                };
                if is_char {
                    Ok(parse_quote! { #obj.to_uppercase().to_string() })
                } else {
                    Ok(parse_quote! { #obj.to_uppercase() })
                }
            }
            "lower" => {
                if !arg_exprs.is_empty() {
                    bail!("lower() takes no arguments");
                }
                // DEPYLER-99MODE-S9: char.to_lowercase() returns ToLowercase iterator
                let is_char = if let HirExpr::Var(name) = hir_object {
                    self.ctx.char_iter_vars.contains(name.as_str())
                } else {
                    false
                };
                if is_char {
                    Ok(parse_quote! { #obj.to_lowercase().to_string() })
                } else {
                    Ok(parse_quote! { #obj.to_lowercase() })
                }
            }
            "strip" => {
                // DEPYLER-0595: str.strip([chars]) → trim_matches
                // DEPYLER-0821: If receiver is a char from Counter iteration, use is_whitespace()
                // Python's char.strip() on a single char returns "" if whitespace, the char otherwise
                // In boolean context: if char.strip(): means "if not whitespace"
                if arg_exprs.is_empty() {
                    // Check if receiver is a char variable from string/Counter iteration
                    // Use both explicit tracking and heuristics for variable names
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
                        // For char type, strip() in boolean context = "is not whitespace"
                        return Ok(parse_quote! { !#obj.is_whitespace() });
                    }
                    Ok(parse_quote! { #obj.trim().to_string() })
                } else {
                    let chars = &arg_exprs[0];
                    Ok(parse_quote! { #obj.trim_matches(|c: char| #chars.contains(c)).to_string() })
                }
            }
            "startswith" => {
                if hir_args.len() != 1 {
                    bail!("startswith() requires exactly one argument");
                }
                // DEPYLER-0945: Extract bare string literal for Pattern trait compatibility
                // String doesn't implement Pattern, but &str does
                // Only borrow if the arg is a String variable (not if already &str from fn_str_params)
                let prefix: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[0].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[0];
                        parse_quote! { &#arg }
                    }
                };
                Ok(parse_quote! { #obj.starts_with(#prefix) })
            }
            "endswith" => {
                if hir_args.len() != 1 {
                    bail!("endswith() requires exactly one argument");
                }
                // DEPYLER-0945: Extract bare string literal for Pattern trait compatibility
                // String doesn't implement Pattern, but &str does
                // Only borrow if the arg is a String variable (not if already &str from fn_str_params)
                let suffix: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[0].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[0];
                        parse_quote! { &#arg }
                    }
                };
                Ok(parse_quote! { #obj.ends_with(#suffix) })
            }
            "split" => {
                if arg_exprs.is_empty() {
                    Ok(
                        parse_quote! { #obj.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 1 {
                    // DEPYLER-0225/0945: Extract bare string literal for Pattern trait compatibility
                    // Only borrow if the arg is a String variable (not if already &str)
                    let sep: syn::Expr = match &hir_args[0] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                            arg_exprs[0].clone()
                        }
                        _ => {
                            let arg = &arg_exprs[0];
                            parse_quote! { &#arg }
                        }
                    };
                    Ok(
                        parse_quote! { #obj.split(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 2 {
                    // DEPYLER-0590: str.split(sep, maxsplit) → splitn(maxsplit+1, sep)
                    // Python's maxsplit is the max number of splits; Rust's splitn takes n parts
                    let sep: syn::Expr = match &hir_args[0] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                            arg_exprs[0].clone()
                        }
                        _ => {
                            let arg = &arg_exprs[0];
                            parse_quote! { &#arg }
                        }
                    };
                    let maxsplit = &arg_exprs[1];
                    Ok(
                        parse_quote! { #obj.splitn((#maxsplit + 1) as usize, #sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("split() accepts at most 2 arguments (separator, maxsplit)");
                }
            }
            // DEPYLER-0202: str.rsplit(sep[, maxsplit]) - reverse split with Pattern trait fix
            // Must extract bare string literals for Pattern trait compatibility
            "rsplit" => {
                if arg_exprs.is_empty() {
                    // Python's rsplit() without args splits on whitespace
                    Ok(
                        parse_quote! { #obj.split_whitespace().rev().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 1 {
                    // DEPYLER-0202/0945: Extract bare string literal for Pattern trait compatibility
                    // Only borrow if the arg is a String variable (not if already &str)
                    let sep: syn::Expr = match &hir_args[0] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                            arg_exprs[0].clone()
                        }
                        _ => {
                            let arg = &arg_exprs[0];
                            parse_quote! { &#arg }
                        }
                    };
                    Ok(
                        parse_quote! { #obj.rsplit(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 2 {
                    // DEPYLER-0202: str.rsplit(sep, maxsplit) → rsplitn(maxsplit+1, sep)
                    // Python's maxsplit is the max number of splits; Rust's rsplitn takes n parts
                    let sep: syn::Expr = match &hir_args[0] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                            arg_exprs[0].clone()
                        }
                        _ => {
                            let arg = &arg_exprs[0];
                            parse_quote! { &#arg }
                        }
                    };
                    let maxsplit = &arg_exprs[1];
                    Ok(
                        parse_quote! { #obj.rsplitn((#maxsplit + 1) as usize, #sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("rsplit() accepts at most 2 arguments (separator, maxsplit)");
                }
            }
            "join" => {
                // DEPYLER-0196: sep.join(iterable) → iterable.join(sep) or iterable.collect::<Vec<_>>().join(sep)
                // DEPYLER-0575: Generator expressions yield iterators, need collect() before join()
                // DEPYLER-0597: Only use collect() for iterators, not for Vec/slice types
                if hir_args.len() != 1 {
                    bail!("join() requires exactly one argument");
                }
                let iterable = &arg_exprs[0];
                // Extract bare string literal for separator
                let separator = match hir_object {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => object_expr.clone(),
                };
                // Check if the iterable is already a collection (Var, List, etc.) vs an iterator
                // DEPYLER-0597: Vecs don't have .collect(), only iterators do
                let needs_collect = match &hir_args[0] {
                    HirExpr::GeneratorExp { .. } => true,
                    HirExpr::Call { func, .. }
                        if func == "map"
                            || func == "filter"
                            || func == "iter"
                            || func == "enumerate" =>
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
            "replace" => {
                // DEPYLER-0195: str.replace(old, new) → .replace(old, new)
                // DEPYLER-0301: str.replace(old, new, count) → .replacen(old, new, count)
                // DEPYLER-0595: datetime.replace() uses kwargs, handled separately via convert_instance_method
                // Use bare string literals without .to_string() for correct types
                if hir_args.len() < 2 {
                    // DEPYLER-99MODE: Not enough args for str.replace(), bail with clear error
                    // datetime.replace() with kwargs is handled by convert_instance_method fallback
                    bail!(
                        "str.replace() requires at least 2 arguments (old, new), got {}",
                        hir_args.len()
                    );
                }
                if hir_args.len() > 3 {
                    bail!("replace() requires 2 or 3 arguments");
                }
                // DEPYLER-0945: Extract bare string literals for Pattern trait compatibility
                // When argument is a variable, borrow it since String doesn't implement Pattern
                // But skip borrowing if the variable is already &str from function parameter
                let old: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[0].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[0];
                        parse_quote! { &#arg }
                    }
                };
                let new: syn::Expr = match &hir_args[1] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[1].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[1];
                        parse_quote! { &#arg }
                    }
                };

                if hir_args.len() == 3 {
                    // Python: str.replace(old, new, count)
                    // Rust: str.replacen(old, new, count as usize)
                    let count = &arg_exprs[2];
                    Ok(parse_quote! { #object_expr.replacen(#old, #new, #count as usize) })
                } else {
                    // Python: str.replace(old, new)
                    // Rust: str.replace(old, new) - replaces all
                    Ok(parse_quote! { #object_expr.replace(#old, #new) })
                }
            }
            "find" => {
                // DEPYLER-0197/0338: str.find(sub[, start]) → .find(sub).map(|i| i as i32).unwrap_or(-1)
                // Python's find() returns -1 if not found, Rust's returns Option<usize>
                // Python supports optional start parameter: str.find(sub, start)
                if hir_args.is_empty() || hir_args.len() > 2 {
                    bail!("find() requires 1 or 2 arguments, got {}", hir_args.len());
                }

                // DEPYLER-0945: Extract bare string literal for Pattern trait compatibility
                // When argument is a variable, borrow it since String doesn't implement Pattern
                // But skip borrowing if the variable is already &str from function parameter
                let substring: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[0].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[0];
                        parse_quote! { &#arg }
                    }
                };

                if hir_args.len() == 2 {
                    // Python: str.find(sub, start)
                    // Rust: str[start..].find(sub).map(|i| (i + start) as i32).unwrap_or(-1)
                    let start = &arg_exprs[1];
                    Ok(parse_quote! {
                        #object_expr[#start as usize..].find(#substring)
                            .map(|i| (i + #start as usize) as i32)
                            .unwrap_or(-1)
                    })
                } else {
                    // Python: str.find(sub)
                    // Rust: str.find(sub).map(|i| i as i32).unwrap_or(-1)
                    Ok(parse_quote! {
                        #object_expr.find(#substring)
                            .map(|i| i as i32)
                            .unwrap_or(-1)
                    })
                }
            }
            "count" => {
                // DEPYLER-0198/0226: str.count(sub) → .matches(sub).count() as i32
                // Extract bare string literal for Pattern trait compatibility
                if hir_args.len() != 1 {
                    bail!("count() requires exactly one argument");
                }
                let substring: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => {
                        // DEPYLER-0200: Use &* to deref-reborrow for Pattern trait compliance
                        // Works for both String (&*String -> &str) and &str (&*&str -> &str)
                        let arg = &arg_exprs[0];
                        parse_quote! { &*#arg }
                    }
                };
                Ok(parse_quote! { #object_expr.matches(#substring).count() as i32 })
            }
            "isdigit" => {
                // DEPYLER-0199: str.isdigit() → .chars().all(|c| c.is_numeric())
                if !arg_exprs.is_empty() {
                    bail!("isdigit() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_numeric() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_numeric()) })
            }
            "isalpha" => {
                // DEPYLER-0200: str.isalpha() → .chars().all(|c| c.is_alphabetic())
                if !arg_exprs.is_empty() {
                    bail!("isalpha() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_alphabetic() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_alphabetic()) })
            }
            "isspace" => {
                // DEPYLER-0650: str.isspace() → .chars().all(|c| c.is_whitespace())
                if !arg_exprs.is_empty() {
                    bail!("isspace() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_whitespace() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_whitespace()) })
            }
            "lstrip" => {
                // DEPYLER-0302/0595: str.lstrip([chars]) → .trim_start_matches
                if arg_exprs.is_empty() {
                    Ok(parse_quote! { #object_expr.trim_start().to_string() })
                } else {
                    let chars = &arg_exprs[0];
                    Ok(
                        parse_quote! { #object_expr.trim_start_matches(|c: char| #chars.contains(c)).to_string() },
                    )
                }
            }
            "rstrip" => {
                // DEPYLER-0302/0595: str.rstrip([chars]) → .trim_end_matches
                if arg_exprs.is_empty() {
                    Ok(parse_quote! { #object_expr.trim_end().to_string() })
                } else {
                    let chars = &arg_exprs[0];
                    Ok(
                        parse_quote! { #object_expr.trim_end_matches(|c: char| #chars.contains(c)).to_string() },
                    )
                }
            }
            "encode" => {
                // DEPYLER-0594: str.encode([encoding]) → .as_bytes().to_vec()
                // Python: s.encode() or s.encode('utf-8')
                // Rust: s.as_bytes().to_vec() (returns Vec<u8>)
                // Note: Only UTF-8 encoding is supported
                Ok(parse_quote! { #object_expr.as_bytes().to_vec() })
            }
            "decode" => {
                // DEPYLER-0594: bytes.decode([encoding]) → String::from_utf8_lossy()
                // Python: b.decode() or b.decode('utf-8')
                // Rust: String::from_utf8_lossy(bytes).to_string()
                // DEPYLER-1003: base64.b64encode now returns Vec<u8> so this works uniformly
                Ok(parse_quote! { String::from_utf8_lossy(&#obj).to_string() })
            }
            "isalnum" => {
                // DEPYLER-0302: str.isalnum() → .chars().all(|c| c.is_alphanumeric())
                if !arg_exprs.is_empty() {
                    bail!("isalnum() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_alphanumeric() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_alphanumeric()) })
            }
            "title" => {
                // DEPYLER-0302 Phase 2: str.title() → custom title case implementation
                // Python's title() capitalizes the first letter of each word
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

            // DEPYLER-STDLIB-STR: index() - find with panic if not found
            "index" => {
                if hir_args.len() != 1 {
                    bail!("index() requires exactly one argument");
                }
                let substring = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! {
                    #object_expr.find(#substring)
                        .map(|i| i as i32)
                        .expect("substring not found")
                })
            }

            // DEPYLER-STDLIB-STR: rfind() - find from right (last occurrence)
            "rfind" => {
                if hir_args.len() != 1 {
                    bail!("rfind() requires exactly one argument");
                }
                let substring = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! {
                    #object_expr.rfind(#substring)
                        .map(|i| i as i32)
                        .unwrap_or(-1)
                })
            }

            // DEPYLER-STDLIB-STR: rindex() - rfind with panic if not found
            "rindex" => {
                if hir_args.len() != 1 {
                    bail!("rindex() requires exactly one argument");
                }
                let substring = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! {
                    #object_expr.rfind(#substring)
                        .map(|i| i as i32)
                        .expect("substring not found")
                })
            }

            // DEPYLER-STDLIB-STR: center() - center string in field
            "center" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("center() requires 1 or 2 arguments");
                }
                let width = &arg_exprs[0];
                let fillchar =
                    if arg_exprs.len() == 2 { &arg_exprs[1] } else { &parse_quote!(" ") };

                Ok(parse_quote! {
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
                })
            }

            // DEPYLER-STDLIB-STR: ljust() - left justify string
            "ljust" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("ljust() requires 1 or 2 arguments");
                }
                let width = &arg_exprs[0];
                let fillchar =
                    if arg_exprs.len() == 2 { &arg_exprs[1] } else { &parse_quote!(" ") };

                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        let width = #width as usize;
                        let fillchar = #fillchar;
                        if s.len() >= width {
                            s.to_string()
                        } else {
                            format!("{}{}", s, fillchar.repeat(width - s.len()))
                        }
                    }
                })
            }

            // DEPYLER-STDLIB-STR: rjust() - right justify string
            "rjust" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("rjust() requires 1 or 2 arguments");
                }
                let width = &arg_exprs[0];
                let fillchar =
                    if arg_exprs.len() == 2 { &arg_exprs[1] } else { &parse_quote!(" ") };

                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        let width = #width as usize;
                        let fillchar = #fillchar;
                        if s.len() >= width {
                            s.to_string()
                        } else {
                            format!("{}{}", fillchar.repeat(width - s.len()), s)
                        }
                    }
                })
            }

            // DEPYLER-STDLIB-STR: zfill() - zero-fill numeric string
            "zfill" => {
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

            // DEPYLER-STDLIB-50: capitalize() - capitalize first character
            "capitalize" => {
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

            // DEPYLER-STDLIB-50: swapcase() - swap upper/lower case
            "swapcase" => {
                if !arg_exprs.is_empty() {
                    bail!("swapcase() takes no arguments");
                }
                Ok(parse_quote! {
                    #object_expr.chars().map(|c| {
                        if c.is_uppercase() {
                            c.to_lowercase().to_string()
                        } else {
                            c.to_uppercase().to_string()
                        }
                    }).collect::<String>()
                })
            }

            // DEPYLER-STDLIB-50: expandtabs() - expand tab characters
            "expandtabs" => {
                if arg_exprs.is_empty() {
                    Ok(parse_quote! {
                        #object_expr.replace("\t", &" ".repeat(8))
                    })
                } else if arg_exprs.len() == 1 {
                    // tabsize argument will be used at runtime
                    let tabsize_expr = &arg_exprs[0];
                    Ok(parse_quote! {
                        #object_expr.replace("\t", &" ".repeat(#tabsize_expr as usize))
                    })
                } else {
                    bail!("expandtabs() takes 0 or 1 arguments")
                }
            }

            // DEPYLER-STDLIB-50: splitlines() - split by line breaks
            "splitlines" => {
                if !arg_exprs.is_empty() {
                    bail!("splitlines() takes no arguments");
                }
                Ok(parse_quote! {
                    #object_expr.lines().map(|s| s.to_string()).collect::<Vec<String>>()
                })
            }

            // DEPYLER-STDLIB-50: partition() - partition by separator
            "partition" => {
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

            // DEPYLER-STDLIB-50: casefold() - aggressive lowercase for caseless matching
            "casefold" => {
                if !arg_exprs.is_empty() {
                    bail!("casefold() takes no arguments");
                }
                // casefold() is like lower() but more aggressive for Unicode
                Ok(parse_quote! { #object_expr.to_lowercase() })
            }

            // DEPYLER-STDLIB-50: isprintable() - check if all characters are printable
            "isprintable" => {
                if !arg_exprs.is_empty() {
                    bail!("isprintable() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(
                            parse_quote! { !#object_expr.is_control() || #object_expr == '\t' || #object_expr == '\n' || #object_expr == '\r' },
                        );
                    }
                }
                Ok(parse_quote! {
                    #object_expr.chars().all(|c| !c.is_control() || c == '\t' || c == '\n' || c == '\r')
                })
            }
            // DEPYLER-0652: Additional is* string methods
            "isupper" => {
                if !arg_exprs.is_empty() {
                    bail!("isupper() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(
                            parse_quote! { !#object_expr.is_alphabetic() || #object_expr.is_uppercase() },
                        );
                    }
                }
                Ok(
                    parse_quote! { #object_expr.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) },
                )
            }
            "islower" => {
                if !arg_exprs.is_empty() {
                    bail!("islower() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(
                            parse_quote! { !#object_expr.is_alphabetic() || #object_expr.is_lowercase() },
                        );
                    }
                }
                Ok(
                    parse_quote! { #object_expr.chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) },
                )
            }
            "istitle" => {
                if !arg_exprs.is_empty() {
                    bail!("istitle() takes no arguments");
                }
                // Title case: first char of each word is uppercase, rest lowercase
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
            "isnumeric" => {
                if !arg_exprs.is_empty() {
                    bail!("isnumeric() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_numeric() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_numeric()) })
            }
            "isascii" => {
                if !arg_exprs.is_empty() {
                    bail!("isascii() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_ascii() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_ascii()) })
            }
            "isdecimal" => {
                if !arg_exprs.is_empty() {
                    bail!("isdecimal() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_ascii_digit() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_ascii_digit()) })
            }
            "isidentifier" => {
                if !arg_exprs.is_empty() {
                    bail!("isidentifier() takes no arguments");
                }
                // Python identifier: starts with letter/underscore, followed by alphanumeric/underscore
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

            // DEPYLER-0538: str/bytes.hex() - convert bytes to hexadecimal string
            "hex" => {
                if !arg_exprs.is_empty() {
                    bail!("hex() takes no arguments");
                }
                // Python: b"hello".hex() → "68656c6c6f"
                // Rust: convert each byte to 2-char hex string
                Ok(parse_quote! {
                    #object_expr.bytes().map(|b| format!("{:02x}", b)).collect::<String>()
                })
            }

            // DEPYLER-0770: str.format() - runtime string formatting
            "format" => {
                // Python: "Hello, {}!".format(name) -> "Hello, World!"
                // Rust: Use sequential replacen for positional formatting
                if arg_exprs.is_empty() {
                    // No args - return template unchanged
                    Ok(object_expr.clone())
                } else if arg_exprs.len() == 1 {
                    // Single arg - replace first {}
                    let arg = &arg_exprs[0];
                    Ok(parse_quote! {
                        #object_expr.replacen("{}", &format!("{}", #arg), 1)
                    })
                } else {
                    // Multiple args - chain replacen calls
                    // Build: template.replacen("{}", &format!("{}", a0), 1)
                    //                .replacen("{}", &format!("{}", a1), 1)...
                    let mut result: syn::Expr = parse_quote! { #object_expr.to_string() };
                    for arg in arg_exprs {
                        result = parse_quote! {
                            #result.replacen("{}", &format!("{}", #arg), 1)
                        };
                    }
                    Ok(result)
                }
            }

            _ => bail!("Unknown string method: {}", method),
        }
    }
}
