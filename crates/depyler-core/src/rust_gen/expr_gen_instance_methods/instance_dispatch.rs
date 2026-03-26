//! Instance method dispatch for ExpressionConverter
//!
//! Contains convert_instance_method - the main router for Python instance method calls.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::rust_gen::keywords;
use anyhow::{bail, Result};
use syn::{self, parse_quote};

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_instance_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
        // CB-200 Batch 13: Argparse stub methods
        if let Some(result) = self.try_convert_argparse_stub(method)? {
            return Ok(result);
        }

        // CB-200 Batch 13: sys I/O stream methods
        if let Some(result) = self.try_convert_sys_io(object, method, arg_exprs)? {
            return Ok(result);
        }

        // CB-200 Batch 13: File I/O methods
        if let Some(result) = self.try_convert_file_io(object_expr, method, arg_exprs, hir_args)? {
            return Ok(result);
        }

        // CB-200 Batch 13: Path methods
        if let Some(result) = self.try_convert_path_method(object, object_expr, method, arg_exprs)? {
            return Ok(result);
        }

        // CB-200 Batch 13: Datetime methods
        if let Some(result) = self.try_convert_datetime_instance_method(object, object_expr, method, arg_exprs, hir_args)? {
            return Ok(result);
        }

        // CB-200 Batch 13: CSV methods
        if let Some(result) = Self::try_convert_csv_instance_method(object_expr, method, arg_exprs)? {
            return Ok(result);
        }

        // CB-200 Batch 13: Regex group method
        if method == "group" {
            return self.convert_group_method(object_expr, arg_exprs, hir_args);
        }

        // CB-200 Batch 13: String method routing
        if let Some(result) = self.try_route_string_method(object, object_expr, method, arg_exprs, hir_args)? {
            return Ok(result);
        }

        // CB-200 Batch 13: Dict methods on self.field
        if let Some(result) = self.try_convert_self_field_dict_method(object, object_expr, method, arg_exprs, hir_args)? {
            return Ok(result);
        }

        // CB-200 Batch 13: User-defined class instance methods
        if let Some(result) = self.try_convert_class_instance_method(object, object_expr, method, arg_exprs, hir_args)? {
            return Ok(result);
        }

        // CB-200 Batch 13: Set/Dict type-based dispatch
        if let Some(result) = self.try_dispatch_set_or_dict(object, object_expr, method, arg_exprs, hir_args)? {
            return Ok(result);
        }

        // DEPYLER-DUNDER-CALL-FIX: Translate Python dunder methods to Rust equivalents
        let method = Self::translate_dunder(method);

        // CB-200 Batch 14: Deque-specific methods (must come before list methods)
        if let Some(result) = self.try_convert_deque_method(object, object_expr, method, arg_exprs, hir_args, kwargs)? {
            return Ok(result);
        }

        // Fallback to method name dispatch
        match method {

            // List methods (remaining)
            "extend" | "insert" | "remove" | "index" | "copy" | "clear" | "reverse" | "sort" => {
                self.convert_list_method(object_expr, object, method, arg_exprs, hir_args, kwargs)
            }

            // DEPYLER-0226: Disambiguate count() for list vs string
            // DEPYLER-0302: Improved heuristic using is_string_base()
            "count" => {
                // Heuristic: Check if object is string-typed using is_string_base()
                // This covers string literals, variables with str type annotations, and string method results
                if self.is_string_base(object) {
                    // String: use str.count() → .matches().count()
                    self.convert_string_method(object, object_expr, method, arg_exprs, hir_args)
                } else {
                    // List: use list.count() → .iter().filter().count()
                    self.convert_list_method(
                        object_expr,
                        object,
                        method,
                        arg_exprs,
                        hir_args,
                        kwargs,
                    )
                }
            }

            // DEPYLER-0223: Disambiguate update() for dict vs set
            "update" => {
                // Check if argument is a set or dict literal
                if !hir_args.is_empty() && self.is_set_expr(&hir_args[0]) {
                    // numbers.update({3, 4}) - set update
                    self.convert_set_method(object_expr, object, method, arg_exprs, hir_args)
                } else {
                    // data.update({"b": 2}) - dict update (default for variables)
                    // DEPYLER-0540: Pass object for serde_json::Value detection
                    self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)
                }
            }

            // DEPYLER-0422: Disambiguate .get() for list vs dict
            // List/Vec .get() takes usize by value, Dict .get() takes &K by reference
            "get" => {
                // Only use list handler when we're CERTAIN it's a list (not dict)
                // Default to dict handler for uncertain types (dict.get() supports 1 or 2 args)
                if self.is_list_expr(object) && !self.is_dict_expr(object) {
                    // List/Vec .get() - cast index to usize (must be exactly 1 arg)
                    if arg_exprs.len() != 1 {
                        bail!("list.get() requires exactly one argument");
                    }
                    let index = &arg_exprs[0];
                    // Cast integer index to usize (Vec/slice .get() requires usize, not &i32)
                    Ok(parse_quote! { #object_expr.get(#index as usize).cloned() })
                } else {
                    // Dict .get() - use existing dict handler (supports 1 or 2 args)
                    // DEPYLER-0540: Pass object for serde_json::Value detection
                    self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)
                }
            }

            // Dict methods (for variables without type info)
            "keys" | "values" | "items" | "setdefault" | "popitem" => {
                // DEPYLER-0540: Pass object for serde_json::Value detection
                self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)
            }

            // String methods
            // Note: "count" handled separately above with disambiguation logic
            // Note: "index" handled in list methods above (lists take precedence)
            "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "startswith" | "endswith"
            | "split" | "rsplit" | "splitlines" | "join" | "replace" | "find" | "rfind"
            | "rindex" | "isdigit" | "isalpha" | "isalnum" | "isspace" | "isupper" | "islower"
            | "istitle" | "isnumeric" | "isascii" | "isdecimal" | "isidentifier"
            | "isprintable" | "title" | "capitalize" | "swapcase" | "casefold" | "center"
            | "ljust" | "rjust" | "zfill" | "hex" | "encode" | "decode" => {
                self.convert_string_method(object, object_expr, method, arg_exprs, hir_args)
            }

            // Set methods (for variables without type info)
            // Note: "update" handled separately above with disambiguation logic
            // Note: "remove" is ambiguous (list vs set) - keep in list fallback for now
            "add"
            | "discard"
            | "intersection_update"
            | "difference_update"
            | "symmetric_difference_update"
            | "union"
            | "intersection"
            | "difference"
            | "symmetric_difference"
            | "issubset"
            | "issuperset"
            | "isdisjoint" => {
                self.convert_set_method(object_expr, object, method, arg_exprs, hir_args)
            }

            // DEPYLER-0431: Regex methods (compiled Regex + Match object)
            // Compiled Regex: findall, match, search (note: "find" conflicts with string.find())
            // Match object: group, groups, start, end, span, as_str
            "findall" | "match" | "search" | "group" | "groups" | "start" | "end" | "span"
            | "as_str" => self.convert_regex_method(object_expr, method, arg_exprs),

            // Path instance methods (DEPYLER-0363)
            "read_text" => {
                // filepath.read_text() → std::fs::read_to_string(filepath).unwrap()
                if !arg_exprs.is_empty() {
                    bail!("Path.read_text() takes no arguments");
                }
                Ok(parse_quote! { std::fs::read_to_string(#object_expr).expect("read failed") })
            }

            // DEPYLER-0960: contains/__contains__ method - dict uses contains_key
            "contains" | "__contains__" => {
                if arg_exprs.len() != 1 {
                    bail!("contains() requires exactly one argument");
                }
                let key = &arg_exprs[0];
                // Check if object is a dict/HashMap - use contains_key
                if self.is_dict_expr(object) {
                    Ok(parse_quote! { #object_expr.contains_key(&#key) })
                } else {
                    // String/Set/List uses .contains()
                    Ok(parse_quote! { #object_expr.contains(&#key) })
                }
            }

            // Default: generic method call
            _ => {
                // DEPYLER-1202: Detect Python-specific methods that need trait bridge
                // These methods don't exist on Rust types, so we inject the traits
                match method {
                    // Python string methods that might not be translated
                    "lower" | "upper" | "strip" | "lstrip" | "rstrip" | "split_py"
                    | "startswith" | "endswith" | "find" | "isalpha" | "isdigit" | "isalnum"
                    | "isspace" | "islower" | "isupper" | "capitalize" | "title" | "swapcase"
                    | "center" | "ljust" | "rjust" | "zfill" => {
                        self.ctx.needs_python_string_ops = true;
                    }
                    // Python int methods
                    "bit_length" | "bit_count" => {
                        self.ctx.needs_python_int_ops = true;
                    }
                    _ => {}
                }

                // DEPYLER-0306 FIX: Use raw identifiers for method names that are Rust keywords
                let method_ident = if keywords::is_rust_keyword(method) {
                    syn::Ident::new_raw(method, proc_macro2::Span::call_site())
                } else {
                    syn::Ident::new(method, proc_macro2::Span::call_site())
                };

                // DEPYLER-0712: Auto-borrow class instance arguments when calling user-defined methods
                // When calling obj.method(other) where both obj and other are class instances,
                // the method signature likely expects &Self, so we borrow the argument.
                // Use is_class_instance helper which checks both var_types and class_names.
                let receiver_is_class = self.is_class_instance(object);

                // Process arguments, adding & when receiver and argument are both class instances
                let processed_args: Vec<syn::Expr> = hir_args
                    .iter()
                    .zip(arg_exprs.iter())
                    .map(|(hir_arg, arg_expr)| {
                        // If receiver is a class instance and argument is also a class instance,
                        // the method likely expects &Self for the argument
                        if receiver_is_class && self.is_class_instance(hir_arg) {
                            return parse_quote! { &#arg_expr };
                        }
                        arg_expr.clone()
                    })
                    .collect();

                // DEPYLER-0823: Wrap cast expressions in parentheses before method calls
                // Rust parses `x as i32.method()` as `x as (i32.method())` which is invalid
                // Must be: `(x as i32).method()`
                let safe_object_expr: syn::Expr = if matches!(object_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#object_expr) }
                } else {
                    object_expr.clone()
                };

                Ok(parse_quote! { #safe_object_expr.#method_ident(#(#processed_args),*) })
            }
        }
    }

    // CB-200 Batch 13: Translate Python dunder methods to Rust equivalents
    fn translate_dunder(method: &str) -> &str {
        match method {
            "__next__" => "next",
            "__iter__" => "iter",
            "__len__" => "len",
            "__str__" => "to_string",
            "__repr__" => "fmt",
            "__contains__" => "contains",
            "__hash__" => "hash",
            "__eq__" => "eq",
            "__ne__" => "ne",
            "__lt__" => "lt",
            "__le__" => "le",
            "__gt__" => "gt",
            "__ge__" => "ge",
            _ => method,
        }
    }

    // CB-200 Batch 13: Handle argparse stub methods (parse_args, add_argument, print_help)
    fn try_convert_argparse_stub(&self, method: &str) -> Result<Option<syn::Expr>> {
        match method {
            "parse_args" | "add_argument" => Ok(Some(parse_quote! { () })),
            "print_help" => Ok(Some(parse_quote! {
                {
                    use clap::CommandFactory;
                    Args::command().print_help().expect("print help failed")
                }
            })),
            _ => Ok(None),
        }
    }

    // CB-200 Batch 13: Handle sys I/O stream method calls
    fn try_convert_sys_io(
        &mut self,
        object: &HirExpr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<Option<syn::Expr>> {
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(module) = &**value {
                if module == "sys" && matches!(attr.as_str(), "stdin" | "stdout" | "stderr") {
                    return Ok(Some(self.convert_sys_io_method(attr, method, arg_exprs)?));
                }
            }
        }
        Ok(None)
    }

    // CB-200 Batch 13: Handle file I/O methods (read, readlines, readline, write, close)
    fn try_convert_file_io(
        &mut self,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        match method {
            "read" if arg_exprs.is_empty() => Ok(Some(parse_quote! {
                {
                    let mut content = String::new();
                    #object_expr.read_to_string(&mut content)?;
                    content
                }
            })),
            "read" if arg_exprs.len() == 1 => {
                let size = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    {
                        let mut _read_buf = vec![0u8; #size];
                        let _n = #object_expr.read(&mut _read_buf).unwrap_or(0);
                        _read_buf.truncate(_n);
                        _read_buf
                    }
                }))
            }
            "readlines" if arg_exprs.is_empty() => {
                self.ctx.needs_bufread = true;
                Ok(Some(parse_quote! {
                    std::io::BufReader::new(#object_expr)
                        .lines()
                        .map(|l| l.unwrap_or_default())
                        .collect::<Vec<_>>()
                }))
            }
            "readline" if arg_exprs.is_empty() => {
                self.ctx.needs_bufread = true;
                Ok(Some(parse_quote! {
                    {
                        let mut _line = String::new();
                        std::io::BufReader::new(&mut #object_expr).read_line(&mut _line).unwrap_or(0);
                        _line
                    }
                }))
            }
            "write" if arg_exprs.len() == 1 => {
                self.ctx.needs_io_write = true;
                let content = &arg_exprs[0];
                let is_option_content = self.is_write_content_option(hir_args);
                if is_option_content {
                    Ok(Some(parse_quote! {
                        #object_expr.write_all(#content.as_ref().expect("value is None").as_bytes()).expect("write failed")
                    }))
                } else {
                    Ok(Some(parse_quote! {
                        #object_expr.write_all(#content.as_bytes()).expect("write failed")
                    }))
                }
            }
            "close" if arg_exprs.is_empty() => Ok(Some(parse_quote! { () })),
            _ => Ok(None),
        }
    }

    // CB-200 Batch 13: Check if write() content arg is an Option type
    fn is_write_content_option(&self, hir_args: &[HirExpr]) -> bool {
        if let Some(HirExpr::Var(var_name)) = hir_args.first() {
            let is_unwrapped = self.ctx.option_unwrap_map.contains_key(var_name)
                || self.ctx.option_unwrap_map.values().any(|v| v == var_name);
            if is_unwrapped {
                return false;
            }
            match self.ctx.var_types.get(var_name) {
                Some(Type::Optional(_)) => true,
                Some(_) => false,
                None => {
                    var_name == "content"
                        || var_name.ends_with("_content")
                        || var_name.ends_with("_text")
                }
            }
        } else {
            false
        }
    }

    // CB-200 Batch 13: Handle path methods
    fn try_convert_path_method(
        &self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<Option<syn::Expr>> {
        let is_path_object = if let HirExpr::Var(var_name) = object {
            var_name == "path" || var_name.ends_with("_path") || var_name == "p"
        } else {
            false
        };
        if !is_path_object {
            return Ok(None);
        }
        match method {
            "stat" if arg_exprs.is_empty() => {
                Ok(Some(parse_quote! { std::fs::metadata(&#object_expr).expect("operation failed") }))
            }
            "absolute" | "resolve" if arg_exprs.is_empty() => {
                Ok(Some(parse_quote! { #object_expr.canonicalize().expect("operation failed").to_string_lossy().to_string() }))
            }
            _ => Ok(None),
        }
    }

    // CB-200 Batch 13: Handle datetime instance methods
    fn try_convert_datetime_instance_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let is_datetime_object = if let HirExpr::Var(var_name) = object {
            var_name == "dt"
                || var_name == "d"
                || var_name == "t"
                || var_name == "datetime"
                || var_name == "date"
                || var_name == "time"
                || var_name.ends_with("_dt")
                || var_name.ends_with("_datetime")
                || var_name.ends_with("_date")
                || var_name.ends_with("_time")
                || var_name.starts_with("date_")
                || var_name.starts_with("time_")
        } else {
            matches!(method, "strftime" | "isoformat" | "timestamp" | "weekday" | "isoweekday")
        };
        if !is_datetime_object {
            return Ok(None);
        }
        let nasa_mode = self.ctx.type_mapper.nasa_mode;
        if !nasa_mode {
            self.ctx.needs_chrono = true;
        }
        match method {
            "isoformat" if arg_exprs.is_empty() => {
                if nasa_mode {
                    Ok(Some(parse_quote! { format!("{:?}", #object_expr) }))
                } else {
                    Ok(Some(parse_quote! { #object_expr.to_string() }))
                }
            }
            "strftime" if arg_exprs.len() == 1 => {
                if nasa_mode {
                    Ok(Some(parse_quote! { format!("{:?}", #object_expr) }))
                } else {
                    let fmt = match hir_args.first() {
                        Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
                        _ => arg_exprs[0].clone(),
                    };
                    Ok(Some(parse_quote! { #object_expr.format(#fmt).to_string() }))
                }
            }
            "timestamp" if arg_exprs.is_empty() => {
                if nasa_mode {
                    Ok(Some(parse_quote! { #object_expr.duration_since(std::time::UNIX_EPOCH).expect("operation failed").as_secs() as f64 }))
                } else {
                    Ok(Some(parse_quote! { #object_expr.and_utc().timestamp() as f64 }))
                }
            }
            "date" if arg_exprs.is_empty() => {
                if nasa_mode { Ok(Some(parse_quote! { #object_expr })) }
                else { Ok(Some(parse_quote! { #object_expr.date() })) }
            }
            "time" if arg_exprs.is_empty() => {
                if nasa_mode { Ok(Some(parse_quote! { #object_expr })) }
                else { Ok(Some(parse_quote! { #object_expr.time() })) }
            }
            _ => Ok(None),
        }
    }

    // CB-200 Batch 13: Handle CSV instance methods
    fn try_convert_csv_instance_method(
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<Option<syn::Expr>> {
        match method {
            "writeheader" if arg_exprs.is_empty() => Ok(Some(parse_quote! { () })),
            "writerow" if arg_exprs.len() == 1 => {
                let row = &arg_exprs[0];
                Ok(Some(parse_quote! { #object_expr.serialize(&#row).expect("operation failed") }))
            }
            _ => Ok(None),
        }
    }

    // CB-200 Batch 13: Handle regex Match.group() method
    fn convert_group_method(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        let nasa_mode = self.ctx.type_mapper.nasa_mode;
        if nasa_mode {
            if arg_exprs.is_empty() || hir_args.is_empty() {
                return Ok(parse_quote! { #object_expr.group(0) });
            }
            let idx = &arg_exprs[0];
            return Ok(parse_quote! { #object_expr.group(#idx as usize) });
        }
        if arg_exprs.is_empty() || hir_args.is_empty() {
            return Ok(parse_quote! { #object_expr.as_str().to_string() });
        }
        if let HirExpr::Literal(Literal::Int(n)) = &hir_args[0] {
            if *n == 0 {
                return Ok(parse_quote! { #object_expr.as_str().to_string() });
            }
            let idx = &arg_exprs[0];
            return Ok(parse_quote! { #object_expr.get(#idx).map(|m| m.as_str().to_string()).unwrap_or_default() });
        }
        let idx = &arg_exprs[0];
        Ok(parse_quote! {
            if #idx == 0 { #object_expr.as_str().to_string() }
            else { #object_expr.get(#idx).map(|m| m.as_str().to_string()).unwrap_or_default() }
        })
    }

    // CB-200 Batch 13: Route to string method handler when method is a known string method
    fn try_route_string_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let is_string_replace = method == "replace" && hir_args.len() >= 2;
        let is_known_string = is_string_replace
            || matches!(
                method,
                "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "startswith" | "endswith"
                    | "split" | "splitlines" | "join" | "find" | "rfind" | "rindex"
                    | "isdigit" | "isalpha" | "isalnum" | "title" | "center" | "ljust"
                    | "rjust" | "zfill" | "hex" | "format" | "encode" | "decode"
            );
        if !is_known_string {
            return Ok(None);
        }
        let is_depyler_var = if let HirExpr::Var(var_name) = object {
            self.ctx.type_mapper.nasa_mode
                && self.ctx.var_types.get(var_name).is_some_and(|t| {
                    matches!(t, Type::Unknown)
                        || matches!(t, Type::Custom(n) if n == "Any" || n == "object")
                })
        } else {
            false
        };
        let adjusted_object_expr = if is_depyler_var {
            parse_quote! { #object_expr.to_string() }
        } else {
            object_expr.clone()
        };
        Ok(Some(self.convert_string_method(object, &adjusted_object_expr, method, arg_exprs, hir_args)?))
    }

    // CB-200 Batch 13: Handle dict methods on self.field attributes
    fn try_convert_self_field_dict_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Attribute { value, attr } = object else { return Ok(None); };
        let HirExpr::Var(var_name) = value.as_ref() else { return Ok(None); };
        if var_name != "self" { return Ok(None); }
        let is_dict_method = matches!(method, "items" | "keys" | "values" | "get" | "update" | "setdefault" | "popitem");
        let field_type_opt = self.ctx.class_field_types.get(attr);
        let is_dict_field = field_type_opt.map_or_else(
            || {
                matches!(attr.as_str(), "config" | "settings" | "options" | "data" | "metadata" | "headers" | "params" | "kwargs")
                    || attr.ends_with("_dict") || attr.ends_with("_map")
            },
            |field_type| matches!(field_type, Type::Dict(_, _)) || matches!(field_type, Type::Custom(s) if s == "Dict"),
        );
        if is_dict_method && is_dict_field {
            Ok(Some(self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)?))
        } else {
            Ok(None)
        }
    }

    // CB-200 Batch 13: Handle user-defined class instance method calls
    fn try_convert_class_instance_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if !self.is_class_instance(object) {
            return Ok(None);
        }
        let method = Self::translate_dunder(method);
        let method_ident = if keywords::is_rust_keyword(method) {
            syn::Ident::new_raw(method, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(method, proc_macro2::Span::call_site())
        };
        let processed_args: Vec<syn::Expr> = hir_args
            .iter()
            .zip(arg_exprs.iter())
            .map(|(hir_arg, arg_expr)| {
                if self.is_class_instance(hir_arg) {
                    parse_quote! { &#arg_expr }
                } else if let HirExpr::Literal(Literal::String(s)) = hir_arg {
                    let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                    parse_quote! { #lit.to_string() }
                } else {
                    arg_expr.clone()
                }
            })
            .collect();
        Ok(Some(parse_quote! { #object_expr.#method_ident(#(#processed_args),*) }))
    }

    // CB-200 Batch 13: Dispatch to set or dict method handlers based on type
    fn try_dispatch_set_or_dict(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if self.is_set_expr(object) {
            if matches!(method, "add" | "remove" | "discard" | "update" | "intersection_update"
                | "difference_update" | "union" | "intersection" | "difference"
                | "symmetric_difference" | "issubset" | "issuperset" | "isdisjoint") {
                return Ok(Some(self.convert_set_method(object_expr, object, method, arg_exprs, hir_args)?));
            }
        }
        if self.is_dict_expr(object) {
            if matches!(method, "get" | "keys" | "values" | "items" | "update") {
                return Ok(Some(self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)?));
            }
        }
        Ok(None)
    }

    // CB-200 Batch 14: Wrap a single argument in DepylerValue based on HIR type information
    fn wrap_arg_in_depyler_value(
        &self,
        arg: &syn::Expr,
        hir_args: &[HirExpr],
    ) -> syn::Expr {
        if !hir_args.is_empty() {
            match &hir_args[0] {
                HirExpr::Literal(Literal::Int(_)) => {
                    parse_quote! { DepylerValue::Int(#arg as i64) }
                }
                HirExpr::Literal(Literal::Float(_)) => {
                    parse_quote! { DepylerValue::Float(#arg as f64) }
                }
                HirExpr::Literal(Literal::String(_)) => {
                    parse_quote! { DepylerValue::Str(#arg.to_string()) }
                }
                HirExpr::Literal(Literal::Bool(_)) => {
                    parse_quote! { DepylerValue::Bool(#arg) }
                }
                HirExpr::Var(name) => match self.ctx.var_types.get(name) {
                    Some(Type::Int) => parse_quote! { DepylerValue::Int(#arg as i64) },
                    Some(Type::Float) => parse_quote! { DepylerValue::Float(#arg as f64) },
                    Some(Type::String) => parse_quote! { DepylerValue::Str(#arg.to_string()) },
                    Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#arg) },
                    _ => parse_quote! { DepylerValue::from(#arg) },
                },
                _ => parse_quote! { DepylerValue::from(#arg) },
            }
        } else {
            parse_quote! { DepylerValue::from(#arg) }
        }
    }

    // CB-200 Batch 14: Check if deque has DepylerValue element type
    fn is_deque_depyler_value(&self, object: &HirExpr) -> bool {
        if let HirExpr::Var(var_name) = object {
            if let Some(Type::Custom(type_str)) = self.ctx.var_types.get(var_name) {
                return type_str.contains("VecDeque") && type_str.contains("DepylerValue");
            }
        }
        false
    }

    // CB-200 Batch 14: Handle deque-specific methods (appendleft, popleft, extendleft, append, pop)
    fn try_convert_deque_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<Option<syn::Expr>> {
        match method {
            "appendleft" => {
                if arg_exprs.len() != 1 {
                    bail!("appendleft() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                if self.is_deque_depyler_value(object) && self.ctx.type_mapper.nasa_mode {
                    let wrapped_arg = self.wrap_arg_in_depyler_value(arg, hir_args);
                    self.ctx.needs_depyler_value_enum = true;
                    Ok(Some(parse_quote! { #object_expr.push_front(#wrapped_arg) }))
                } else {
                    Ok(Some(parse_quote! { #object_expr.push_front(#arg) }))
                }
            }
            "popleft" => {
                if !arg_exprs.is_empty() {
                    bail!("popleft() takes no arguments");
                }
                Ok(Some(parse_quote! { #object_expr.pop_front().expect("popleft from empty deque") }))
            }
            "extendleft" => {
                if arg_exprs.len() != 1 {
                    bail!("extendleft() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    for __item in #arg.into_iter().rev() {
                        #object_expr.push_front(__item);
                    }
                }))
            }
            "append" if self.is_deque_expr(object) => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                if self.is_deque_depyler_value(object) && self.ctx.type_mapper.nasa_mode {
                    let wrapped_arg = self.wrap_arg_in_depyler_value(arg, hir_args);
                    self.ctx.needs_depyler_value_enum = true;
                    Ok(Some(parse_quote! { #object_expr.push_back(#wrapped_arg) }))
                } else {
                    Ok(Some(parse_quote! { #object_expr.push_back(#arg) }))
                }
            }
            "pop" if self.is_deque_expr(object) => {
                if !arg_exprs.is_empty() {
                    bail!("deque.pop() does not accept an index argument");
                }
                Ok(Some(parse_quote! { #object_expr.pop_back().unwrap_or_default() }))
            }
            _ => Ok(None),
        }
    }
}
