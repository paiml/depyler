//! Method call routing for ExpressionConverter
//!
//! Contains convert_dynamic_call and convert_method_call - routing method calls
//! to appropriate handlers based on receiver type and method name.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::trace_decision;
use anyhow::Result;
use syn::{self, parse_quote};

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_dynamic_call(
        &mut self,
        callee: &HirExpr,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        let callee_expr = callee.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        if arg_exprs.is_empty() {
            Ok(parse_quote! { (#callee_expr)() })
        } else {
            Ok(parse_quote! { (#callee_expr)(#(#arg_exprs),*) })
        }
    }

    pub(crate) fn convert_method_call(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
        // CITL: Trace method dispatch decision
        trace_decision!(
            category = DecisionCategory::MethodDispatch,
            name = "method_call",
            chosen = method,
            alternatives = ["trait_method", "inherent_method", "extension", "ufcs"],
            confidence = 0.88
        );

        // DEPYLER-1205: Usage-Based Type Inference
        self.infer_type_from_method_usage(object, method, args);

        // CB-200 Batch 13: Option/Dict method dispatch
        if let Some(result) = self.try_convert_mut_option_method(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.try_convert_mut_option_dict_method(object, method, args)? {
            return Ok(result);
        }

        // CB-200 Batch 13: is_some/is_none on precomputed fields and Result-returning calls
        if let Some(result) = self.try_convert_option_check(object, method, args)? {
            return Ok(result);
        }

        // CB-200 Batch 14: Subprocess child, serde_json, asyncio, colorsys, class method dispatch
        if let Some(result) = self.try_convert_subprocess_wait(object, method, args)? {
            return Ok(result);
        }

        if self.is_serde_json_value_expr(object) || self.is_serde_json_value(object) {
            if let Some(result) = self.try_convert_serde_json_method(object, method, args)? {
                return Ok(result);
            }
        }

        if let Some(result) = self.try_convert_asyncio_method(object, method, args)? {
            return Ok(result);
        }

        if let Some(result) = self.try_convert_colorsys_method(object, method, args)? {
            return Ok(result);
        }

        if let Some(result) = self.try_convert_class_method_call(object, method, args)? {
            return Ok(result);
        }

        // CB-200 Final: Hasher/Counter method dispatch
        if let Some(result) = self.try_convert_hasher_method(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.try_convert_most_common(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.try_convert_hasher_update(object, method, args)? {
            return Ok(result);
        }

        // CB-200 Batch 15: String/pathlib/datetime method dispatch extracted to helpers
        if let Some(result) = self.try_dispatch_string_method(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.try_dispatch_pathlib_method(object, method, args)? {
            return Ok(result);
        }
        if let Some(result) = self.try_dispatch_datetime_method(object, method, args)? {
            return Ok(result);
        }

        // CB-200 Final: Static method call dispatch
        if let Some(result) = self.try_convert_static_method(object, method, args)? {
            return Ok(result);
        }

        // Try classmethod handling first
        if let Some(result) = self.try_convert_classmethod(object, method, args)? {
            return Ok(result);
        }

        // Try module method handling
        // DEPYLER-0426: Pass kwargs to module method converter
        if let Some(result) = self.try_convert_module_method(object, method, args, kwargs)? {
            return Ok(result);
        }

        // CB-200 Final: Handle .decode() on base64 encode calls
        if let Some(result) = self.try_convert_base64_decode(object, method)? {
            return Ok(result);
        }

        // CB-200 Final: External module method dispatch
        if let Some(result) = self.try_convert_external_module_method(object, method, args)? {
            return Ok(result);
        }

        // DEPYLER-1064: Handle method calls on DepylerValue variables
        // When calling string methods on a DepylerValue, extract the string first
        let is_depyler_value_var = if let HirExpr::Var(var_name) = object {
            self.ctx.type_mapper.nasa_mode
                && self.ctx.var_types.get(var_name).is_some_and(|t| {
                    matches!(t, Type::Unknown)
                        || matches!(t, Type::Custom(name) if name == "Any" || name == "object")
                })
        } else {
            false
        };

        let is_string_method = matches!(
            method,
            "upper"
                | "lower"
                | "strip"
                | "lstrip"
                | "rstrip"
                | "startswith"
                | "endswith"
                | "split"
                | "rsplit"
                | "splitlines"
                | "join"
                | "replace"
                | "find"
                | "rfind"
                | "rindex"
                | "isdigit"
                | "isalpha"
                | "isalnum"
                | "isspace"
                | "isupper"
                | "islower"
                | "istitle"
                | "title"
                | "capitalize"
                | "swapcase"
                | "casefold"
                | "center"
                | "ljust"
                | "rjust"
                | "zfill"
                | "format"
                | "encode"
                | "decode"
        );

        let object_expr = if is_depyler_value_var && is_string_method {
            // Extract string from DepylerValue before calling string method
            let base_expr = object.to_rust_expr(self.ctx)?;
            parse_quote! { #base_expr.to_string() }
        } else {
            object.to_rust_expr(self.ctx)?
        };

        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // DEPYLER-0445: Pass original args and kwargs separately to convert_instance_method
        // Some methods like sort(key=func) need to preserve keyword argument names
        // For other methods, they can merge kwargs as positional if needed
        self.convert_instance_method(object, &object_expr, method, &arg_exprs, args, kwargs)
    }

    // ========================================================================
    // CB-200 Final: Helpers extracted from convert_method_call
    // ========================================================================

    /// CB-200 Final: Handle hexdigest() on hasher objects
    fn try_convert_hasher_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        _args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if method != "hexdigest" {
            return Ok(None);
        }
        self.ctx.needs_hex = true;
        self.ctx.needs_digest = true;
        let object_expr = object.to_rust_expr(self.ctx)?;
        Ok(Some(parse_quote! {
            { use digest::DynDigest; hex::encode(#object_expr.finalize_reset()) }
        }))
    }

    /// CB-200 Final: Handle Counter.most_common(n)
    fn try_convert_most_common(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if method != "most_common" {
            return Ok(None);
        }
        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        if let Some(n_arg) = arg_exprs.first() {
            Ok(Some(parse_quote! {
                {
                    let mut entries: Vec<_> = #object_expr.iter().map(|(k, v)| (k.clone(), *v)).collect();
                    entries.sort_by(|a, b| b.1.cmp(&a.1));
                    entries.into_iter().take(#n_arg as usize).collect::<Vec<_>>()
                }
            }))
        } else {
            Ok(Some(parse_quote! {
                {
                    let mut entries: Vec<_> = #object_expr.iter().map(|(k, v)| (k.clone(), *v)).collect();
                    entries.sort_by(|a, b| b.1.cmp(&a.1));
                    entries
                }
            }))
        }
    }

    /// CB-200 Final: Handle hasher.update() (not dict/set.update())
    fn try_convert_hasher_update(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if method != "update" || args.is_empty() || self.is_dict_expr(object) || self.is_set_expr(object) {
            return Ok(None);
        }
        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
        let data = &arg_exprs[0];
        Ok(Some(parse_quote! { #object_expr.update(&#data) }))
    }

    /// CB-200 Final: Handle static method calls (ClassName.method())
    fn try_convert_static_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let class_name = if let HirExpr::Var(name) = object { name } else { return Ok(None) };
        let is_const = class_name.chars().all(|c| c.is_uppercase() || c == '_');
        let starts_uppercase = class_name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);

        if !starts_uppercase || is_const {
            return Ok(None);
        }

        let safe_name = crate::direct_rules::safe_class_name(class_name);
        let class_ident = syn::Ident::new(&safe_name, proc_macro2::Span::call_site());
        let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;
        Ok(Some(parse_quote! { #class_ident::#method_ident(#(#arg_exprs),*) }))
    }

    /// CB-200 Final: Handle .decode() on base64 encode calls
    fn try_convert_base64_decode(
        &mut self,
        object: &HirExpr,
        method: &str,
    ) -> Result<Option<syn::Expr>> {
        if method != "decode" {
            return Ok(None);
        }
        if let HirExpr::MethodCall { object: inner_obj, method: inner_method, .. } = object {
            if let HirExpr::Var(module) = inner_obj.as_ref() {
                if module == "base64"
                    && (inner_method.contains("b64encode")
                        || inner_method.contains("urlsafe_b64encode"))
                {
                    return Ok(Some(object.to_rust_expr(self.ctx)?));
                }
            }
        }
        Ok(None)
    }

    /// CB-200 Final: Handle external module method calls
    fn try_convert_external_module_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let module_name = if let HirExpr::Var(name) = object { name } else { return Ok(None) };

        // Collections module constructors
        if let Some(result) = self.try_collections_constructor(module_name, method, args)? {
            return Ok(Some(result));
        }

        // Imported module method dispatch
        if self.ctx.all_imported_modules.contains(module_name) {
            return self.try_imported_module_method(module_name, method, args);
        }

        Ok(None)
    }

    /// CB-200 Final: Handle collections.Counter/deque/defaultdict constructors
    fn try_collections_constructor(
        &mut self,
        module_name: &str,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if module_name != "collections" {
            return Ok(None);
        }
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        match method {
            "Counter" => Ok(Some(crate::rust_gen::collection_constructors::convert_counter_builtin(self.ctx, &arg_exprs)?)),
            "deque" => Ok(Some(crate::rust_gen::collection_constructors::convert_deque_builtin(self.ctx, &arg_exprs)?)),
            "defaultdict" => Ok(Some(crate::rust_gen::collection_constructors::convert_defaultdict_builtin(self.ctx, &arg_exprs)?)),
            _ => Ok(None),
        }
    }

    /// CB-200 Final: Handle imported module method calls with Rust path mapping
    fn try_imported_module_method(
        &mut self,
        module_name: &str,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if let Some(mapping) = self.ctx.imported_modules.get(module_name).cloned() {
            if let Some(result) = self.try_mapped_module_method(&mapping, method, args)? {
                return Ok(Some(result));
            }
        }

        // Fallback: module::function() syntax
        let module_ident = crate::rust_gen::keywords::safe_ident(module_name);
        let method_ident = crate::rust_gen::keywords::safe_ident(method);
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;
        Ok(Some(parse_quote! { #module_ident::#method_ident(#(#arg_exprs),*) }))
    }

    /// CB-200 Final: Handle mapped module method (macro or function call)
    fn try_mapped_module_method(
        &mut self,
        mapping: &crate::module_mapper::ModuleMapping,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let rust_path = &mapping.rust_path;
        let rust_name = match mapping.item_map.get(method) {
            Some(name) => name,
            None => return Ok(None),
        };

        if rust_name.ends_with('!') {
            let macro_name_str = rust_name.trim_end_matches('!');
            let macro_ident = syn::Ident::new(macro_name_str, proc_macro2::Span::call_site());
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return Ok(Some(parse_quote! { #macro_ident!(#(#arg_exprs),*) }));
        }

        if !rust_path.is_empty() {
            let full_path: syn::Path = syn::parse_str(&format!("{}::{}", rust_path, rust_name))?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return Ok(Some(parse_quote! { #full_path(#(#arg_exprs),*) }));
        }

        Ok(None)
    }

    /// CB-200 Batch 15: Check if object is a known stdlib module
    fn is_stdlib_module_var(&self, object: &HirExpr) -> bool {
        if let HirExpr::Var(name) = object {
            !self.ctx.is_declared(name)
                && matches!(
                    name.as_str(),
                    "re" | "json" | "math" | "random" | "os" | "sys" | "time"
                        | "datetime" | "pathlib" | "struct" | "statistics" | "fractions"
                        | "decimal" | "collections" | "itertools" | "functools" | "shutil"
                        | "csv" | "base64" | "hashlib" | "subprocess" | "string" | "tempfile"
                )
        } else {
            false
        }
    }

    /// CB-200 Batch 15: Try dispatching as a string method
    fn try_dispatch_string_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if self.is_stdlib_module_var(object) {
            return Ok(None);
        }
        if !matches!(
            method,
            "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "startswith"
                | "endswith" | "split" | "splitlines" | "join" | "find" | "rfind"
                | "rindex" | "isdigit" | "isalpha" | "isalnum" | "title" | "center"
                | "ljust" | "rjust" | "zfill" | "hex" | "format"
        ) {
            return Ok(None);
        }
        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
        Ok(Some(self.convert_string_method(object, &object_expr, method, &arg_exprs, args)?))
    }

    /// CB-200 Batch 15: Try dispatching as a pathlib instance method
    fn try_dispatch_pathlib_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let is_os_module = matches!(object, HirExpr::Var(name) if name == "os");
        if is_os_module {
            return Ok(None);
        }
        if !matches!(
            method,
            "write_text" | "read_text" | "read_bytes" | "write_bytes" | "exists"
                | "is_file" | "is_dir" | "mkdir" | "rmdir" | "unlink" | "iterdir"
                | "glob" | "rglob" | "with_name" | "with_suffix" | "with_stem"
                | "resolve" | "absolute" | "relative_to"
        ) {
            return Ok(None);
        }
        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
        Ok(Some(self.convert_pathlib_instance_method(&object_expr, method, &arg_exprs)?))
    }

    /// CB-200 Batch 15: Try dispatching as a datetime/timedelta instance method
    fn try_dispatch_datetime_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if !matches!(
            method,
            "total_seconds" | "fromisoformat" | "isoformat" | "strftime" | "timestamp"
                | "timetuple" | "weekday" | "isoweekday" | "isocalendar" | "replace"
        ) {
            return Ok(None);
        }
        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
        Ok(Some(self.convert_datetime_instance_method(&object_expr, method, args, &arg_exprs)?))
    }

    /// CB-200 Batch 12: Handle method calls on &mut Option<T> parameters
    fn try_convert_mut_option_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Var(var_name) = object else {
            return Ok(None);
        };
        if !self.ctx.mut_option_params.contains(var_name) {
            return Ok(None);
        }

        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
        match method {
            "is_none" if args.is_empty() => Ok(Some(parse_quote! { #var_ident.is_none() })),
            "is_some" if args.is_empty() => Ok(Some(parse_quote! { #var_ident.is_some() })),
            _ => {
                let needs_unwrap = matches!(
                    method,
                    "year"
                        | "month"
                        | "day"
                        | "hour"
                        | "minute"
                        | "second"
                        | "weekday"
                        | "isoweekday"
                        | "timestamp"
                        | "date"
                        | "time"
                        | "replace"
                        | "strftime"
                        | "isoformat"
                );
                if needs_unwrap {
                    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                    let arg_exprs: Vec<syn::Expr> = args
                        .iter()
                        .map(|arg| arg.to_rust_expr(self.ctx))
                        .collect::<Result<Vec<_>>>()?;
                    if arg_exprs.is_empty() {
                        Ok(Some(
                            parse_quote! { #var_ident.as_ref().expect("value is None").#method_ident() },
                        ))
                    } else {
                        Ok(Some(
                            parse_quote! { #var_ident.as_ref().expect("value is None").#method_ident(#(#arg_exprs),*) },
                        ))
                    }
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// CB-200 Batch 12: Handle method calls on &mut Option<HashMap<K, V>> parameters
    fn try_convert_mut_option_dict_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Var(var_name) = object else {
            return Ok(None);
        };
        if !self.ctx.mut_option_dict_params.contains(var_name) {
            return Ok(None);
        }

        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
        match method {
            "get" => {
                if args.is_empty() {
                    return Ok(Some(
                        parse_quote! { #var_ident.as_ref().expect("value is None").get() },
                    ));
                }
                let key_expr = args[0].to_rust_expr(self.ctx)?;
                if args.len() > 1 {
                    let default_expr = args[1].to_rust_expr(self.ctx)?;
                    Ok(Some(parse_quote! {
                        #var_ident.as_ref().expect("value is None").get(&#key_expr).cloned().unwrap_or(#default_expr)
                    }))
                } else {
                    Ok(Some(parse_quote! {
                        #var_ident.as_ref().expect("value is None").get(&#key_expr).cloned()
                    }))
                }
            }
            "contains_key" | "__contains__" if !args.is_empty() => {
                let key_expr = args[0].to_rust_expr(self.ctx)?;
                Ok(Some(parse_quote! {
                    #var_ident.as_ref().expect("value is None").contains_key(&#key_expr)
                }))
            }
            "keys" if args.is_empty() => Ok(Some(
                parse_quote! { #var_ident.as_ref().expect("value is None").keys() },
            )),
            "values" if args.is_empty() => Ok(Some(
                parse_quote! { #var_ident.as_ref().expect("value is None").values() },
            )),
            "items" if args.is_empty() => Ok(Some(
                parse_quote! { #var_ident.as_ref().expect("value is None").iter() },
            )),
            "len" if args.is_empty() => Ok(Some(
                parse_quote! { #var_ident.as_ref().expect("value is None").len() as i32 },
            )),
            _ => Ok(None),
        }
    }

    /// CB-200 Batch 12/14: Handle serde_json::Value method calls
    fn try_convert_serde_json_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // CB-200 Batch 14: Try read-only methods first, then mutation methods
        if let Some(result) = Self::try_serde_json_query(&object_expr, method, &arg_exprs, args)? {
            return Ok(Some(result));
        }
        Self::try_serde_json_mutation(&object_expr, method, &arg_exprs, args)
    }

    /// CB-200 Batch 14: serde_json query/read methods (len, iter, is_none, get, keys, etc.)
    fn try_serde_json_query(
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        match method {
            "len" if args.is_empty() => Ok(Some(parse_quote! {
                #object_expr.as_array().map(|a| a.len()).unwrap_or_else(||
                    #object_expr.as_object().map(|o| o.len()).unwrap_or(0)
                ) as i32
            })),
            "iter" if args.is_empty() => Ok(Some(parse_quote! {
                #object_expr.as_array().into_iter().flatten()
            })),
            "is_none" if args.is_empty() => Ok(Some(parse_quote! { #object_expr.is_null() })),
            "is_some" if args.is_empty() => Ok(Some(parse_quote! { !#object_expr.is_null() })),
            "is_empty" if args.is_empty() => Ok(Some(parse_quote! {
                #object_expr.as_array().map(|a| a.is_empty()).unwrap_or_else(||
                    #object_expr.as_object().map(|o| o.is_empty()).unwrap_or(true)
                )
            })),
            "get" if !args.is_empty() => {
                let key = &arg_exprs[0];
                if args.len() > 1 {
                    let default = &arg_exprs[1];
                    Ok(Some(parse_quote! {
                        #object_expr.get(#key).cloned().unwrap_or(serde_json::json!(#default))
                    }))
                } else {
                    Ok(Some(parse_quote! { #object_expr.get(#key).cloned() }))
                }
            }
            "keys" if args.is_empty() => Ok(Some(parse_quote! {
                #object_expr.as_object().into_iter().flat_map(|o| o.keys().cloned()).collect::<Vec<_>>()
            })),
            "values" if args.is_empty() => Ok(Some(parse_quote! {
                #object_expr.as_object().into_iter().flat_map(|o| o.values().cloned()).collect::<Vec<_>>()
            })),
            "items" if args.is_empty() => Ok(Some(parse_quote! {
                #object_expr.as_object().into_iter().flat_map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone()))).collect::<Vec<_>>()
            })),
            "contains" if args.len() == 1 => {
                let arg = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    #object_expr.as_array().map(|a| a.iter().any(|v| v == &serde_json::json!(#arg))).unwrap_or(false)
                }))
            }
            "contains_key" | "__contains__" if args.len() == 1 => {
                let key = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    #object_expr.as_object().map(|o| o.contains_key(#key)).unwrap_or(false)
                }))
            }
            "copy" | "clone" if args.is_empty() => {
                Ok(Some(parse_quote! { #object_expr.clone() }))
            }
            _ => Ok(None),
        }
    }

    /// CB-200 Batch 14: serde_json mutation methods (append, push, pop, insert, remove, etc.)
    fn try_serde_json_mutation(
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        match method {
            "append" | "push" if args.len() == 1 => {
                let arg = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    #object_expr.as_array_mut().map(|a| a.push(serde_json::json!(#arg)))
                }))
            }
            "pop" if args.is_empty() => Ok(Some(parse_quote! {
                #object_expr.as_array_mut().and_then(|a| a.pop()).unwrap_or(serde_json::Value::Null)
            })),
            "pop_front" | "popleft" if args.is_empty() => Ok(Some(parse_quote! {
                #object_expr.as_array_mut().and_then(|a| if a.is_empty() { None } else { Some(a.remove(0)) }).unwrap_or(serde_json::Value::Null)
            })),
            "push_back" if args.len() == 1 => {
                let arg = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    #object_expr.as_array_mut().map(|a| a.push(serde_json::json!(#arg)))
                }))
            }
            "push_front" | "appendleft" if args.len() == 1 => {
                let arg = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    #object_expr.as_array_mut().map(|a| a.insert(0, serde_json::json!(#arg)))
                }))
            }
            "insert" if args.len() == 2 => {
                let key = &arg_exprs[0];
                let val = &arg_exprs[1];
                Ok(Some(parse_quote! {
                    #object_expr.as_object_mut().map(|o| o.insert(#key.to_string(), serde_json::json!(#val)))
                }))
            }
            "remove" if args.len() == 1 => {
                let key = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    #object_expr.as_object_mut().and_then(|o| o.remove(#key))
                }))
            }
            "clear" if args.is_empty() => Ok(Some(parse_quote! {
                { if let Some(a) = #object_expr.as_array_mut() { a.clear() }
                  else if let Some(o) = #object_expr.as_object_mut() { o.clear() } }
            })),
            "extend" if args.len() == 1 => {
                let other = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    { if let (Some(a1), Some(a2)) = (#object_expr.as_array_mut(), #other.as_array()) {
                        a1.extend(a2.iter().cloned());
                    } else if let (Some(o1), Some(o2)) = (#object_expr.as_object_mut(), #other.as_object()) {
                        for (k, v) in o2 { o1.insert(k.clone(), v.clone()); }
                    } }
                }))
            }
            "add" if args.len() == 1 => {
                let arg = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    #object_expr.as_array_mut().map(|a| if !a.iter().any(|v| v == &serde_json::json!(#arg)) { a.push(serde_json::json!(#arg)) })
                }))
            }
            "discard" if args.len() == 1 => {
                let arg = &arg_exprs[0];
                Ok(Some(parse_quote! {
                    #object_expr.as_array_mut().map(|a| a.retain(|v| v != &serde_json::json!(#arg)))
                }))
            }
            _ => Ok(None),
        }
    }

    /// CB-200 Batch 12: Handle asyncio module method calls
    fn try_convert_asyncio_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Var(module) = object else {
            return Ok(None);
        };
        if module != "asyncio" {
            return Ok(None);
        }

        let nasa_mode = self.ctx.type_mapper.nasa_mode;
        if !nasa_mode {
            self.ctx.needs_tokio = true;
        }
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        match method {
            "sleep" => {
                if nasa_mode {
                    if let Some(arg) = arg_exprs.first() {
                        return Ok(Some(parse_quote! {
                            std::thread::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                        }));
                    }
                    return Ok(Some(parse_quote! {
                        std::thread::sleep(std::time::Duration::from_secs(0))
                    }));
                }
                if let Some(arg) = arg_exprs.first() {
                    return Ok(Some(parse_quote! {
                        tokio::time::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                    }));
                }
                Ok(Some(parse_quote! {
                    tokio::time::sleep(std::time::Duration::from_secs(0))
                }))
            }
            "run" => {
                if nasa_mode {
                    if let Some(arg) = arg_exprs.first() {
                        return Ok(Some(parse_quote! { #arg }));
                    }
                } else if let Some(arg) = arg_exprs.first() {
                    return Ok(Some(parse_quote! {
                        tokio::runtime::Runtime::new().expect("operation failed").block_on(#arg)
                    }));
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    /// CB-200 Batch 12: Handle colorsys module method calls
    fn try_convert_colorsys_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Var(module) = object else {
            return Ok(None);
        };
        if module != "colorsys" {
            return Ok(None);
        }

        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        match method {
            "rgb_to_hsv" if arg_exprs.len() == 3 => {
                let (r, g, b) = (&arg_exprs[0], &arg_exprs[1], &arg_exprs[2]);
                Ok(Some(parse_quote! {
                    {
                        let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                        let max_c = r.max(g).max(b);
                        let min_c = r.min(g).min(b);
                        let v = max_c;
                        if min_c == max_c {
                            (0.0, 0.0, v)
                        } else {
                            let s = (max_c - min_c) / max_c;
                            let rc = (max_c - r) / (max_c - min_c);
                            let gc = (max_c - g) / (max_c - min_c);
                            let bc = (max_c - b) / (max_c - min_c);
                            let h = if r == max_c { bc - gc }
                                    else if g == max_c { 2.0 + rc - bc }
                                    else { 4.0 + gc - rc };
                            let h = (h / 6.0) % 1.0;
                            let h = if h < 0.0 { h + 1.0 } else { h };
                            (h, s, v)
                        }
                    }
                }))
            }
            "hsv_to_rgb" if arg_exprs.len() == 3 => {
                let (h, s, v) = (&arg_exprs[0], &arg_exprs[1], &arg_exprs[2]);
                Ok(Some(parse_quote! {
                    {
                        let (h, s, v) = (#h as f64, #s as f64, #v as f64);
                        if s == 0.0 {
                            (v, v, v)
                        } else {
                            let i = (h * 6.0).floor();
                            let f = (h * 6.0) - i;
                            let p = v * (1.0 - s);
                            let q = v * (1.0 - s * f);
                            let t = v * (1.0 - s * (1.0 - f));
                            let i = i as i32 % 6;
                            match i {
                                0 => (v, t, p), 1 => (q, v, p), 2 => (p, v, t),
                                3 => (p, q, v), 4 => (t, p, v), _ => (v, p, q),
                            }
                        }
                    }
                }))
            }
            "rgb_to_hls" if arg_exprs.len() == 3 => {
                let (r, g, b) = (&arg_exprs[0], &arg_exprs[1], &arg_exprs[2]);
                Ok(Some(parse_quote! {
                    {
                        let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                        let max_c = r.max(g).max(b);
                        let min_c = r.min(g).min(b);
                        let l = (min_c + max_c) / 2.0;
                        if min_c == max_c {
                            (0.0, l, 0.0)
                        } else {
                            let s = if l <= 0.5 { (max_c - min_c) / (max_c + min_c) }
                                    else { (max_c - min_c) / (2.0 - max_c - min_c) };
                            let rc = (max_c - r) / (max_c - min_c);
                            let gc = (max_c - g) / (max_c - min_c);
                            let bc = (max_c - b) / (max_c - min_c);
                            let h = if r == max_c { bc - gc }
                                    else if g == max_c { 2.0 + rc - bc }
                                    else { 4.0 + gc - rc };
                            let h = (h / 6.0) % 1.0;
                            let h = if h < 0.0 { h + 1.0 } else { h };
                            (h, l, s)
                        }
                    }
                }))
            }
            "hls_to_rgb" if arg_exprs.len() == 3 => {
                let (h, l, s) = (&arg_exprs[0], &arg_exprs[1], &arg_exprs[2]);
                Ok(Some(parse_quote! {
                    {
                        let (h, l, s) = (#h as f64, #l as f64, #s as f64);
                        if s == 0.0 {
                            (l, l, l)
                        } else {
                            let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - (l * s) };
                            let m1 = 2.0 * l - m2;
                            let _v = |hue: f64| {
                                let hue = hue % 1.0;
                                let hue = if hue < 0.0 { hue + 1.0 } else { hue };
                                if hue < 1.0/6.0 { m1 + (m2 - m1) * hue * 6.0 }
                                else if hue < 0.5 { m2 }
                                else if hue < 2.0/3.0 { m1 + (m2 - m1) * (2.0/3.0 - hue) * 6.0 }
                                else { m1 }
                            };
                            (_v(h + 1.0/3.0), _v(h), _v(h - 1.0/3.0))
                        }
                    }
                }))
            }
            _ => Ok(None),
        }
    }

    // CB-200 Batch 13: Usage-based type inference from method calls
    fn infer_type_from_method_usage(&mut self, object: &HirExpr, method: &str, args: &[HirExpr]) {
        let HirExpr::Var(var_name) = object else { return; };
        let current_type = self.ctx.var_types.get(var_name).cloned();
        if !matches!(current_type, None | Some(Type::Unknown)) {
            return;
        }
        match method {
            "append" => {
                let element_type = if !args.is_empty() {
                    self.infer_type_from_hir_expr(&args[0])
                } else {
                    Type::Unknown
                };
                self.ctx.var_types.insert(var_name.clone(), Type::List(Box::new(element_type.clone())));
                tracing::debug!("DEPYLER-1211: Inferred {} as List<{:?}> (via append())", var_name, element_type);
            }
            "insert" => {
                let element_type = if args.len() >= 2 {
                    self.infer_type_from_hir_expr(&args[1])
                } else {
                    Type::Unknown
                };
                self.ctx.var_types.insert(var_name.clone(), Type::List(Box::new(element_type.clone())));
                tracing::debug!("DEPYLER-1211: Inferred {} as List<{:?}> (via insert())", var_name, element_type);
            }
            "extend" | "pop" | "remove" | "sort" | "reverse" | "clear" | "copy" | "index" | "count" => {
                self.ctx.var_types.insert(var_name.clone(), Type::List(Box::new(Type::Unknown)));
                tracing::debug!("DEPYLER-1205: Inferred {} as List (via {}())", var_name, method);
            }
            "lower" | "upper" | "strip" | "lstrip" | "rstrip" | "split" | "join"
            | "replace" | "startswith" | "endswith" | "find" | "rfind" | "isdigit"
            | "isalpha" | "isalnum" | "isupper" | "islower" | "title" | "capitalize"
            | "swapcase" | "center" | "ljust" | "rjust" | "zfill" | "encode" => {
                self.ctx.var_types.insert(var_name.clone(), Type::String);
                tracing::debug!("DEPYLER-1205: Inferred {} as String (via {}())", var_name, method);
            }
            "keys" | "values" | "items" | "get" | "setdefault" | "update" | "popitem" => {
                self.ctx.var_types.insert(var_name.clone(), Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)));
                tracing::debug!("DEPYLER-1205: Inferred {} as Dict (via {}())", var_name, method);
            }
            "iter" => {
                self.ctx.var_types.insert(var_name.clone(), Type::List(Box::new(Type::Unknown)));
                tracing::debug!("DEPYLER-1205: Inferred {} as List (via iter())", var_name);
            }
            "add" | "discard" | "difference" | "intersection" | "union"
            | "symmetric_difference" | "issubset" | "issuperset" | "isdisjoint" => {
                self.ctx.var_types.insert(var_name.clone(), Type::Set(Box::new(Type::Unknown)));
                tracing::debug!("DEPYLER-1205: Inferred {} as Set (via {}())", var_name, method);
            }
            _ => {}
        }
    }

    // CB-200 Batch 13: Handle is_some/is_none on precomputed fields and Result-returning calls
    fn try_convert_option_check(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if !matches!(method, "is_some" | "is_none") || !args.is_empty() {
            return Ok(None);
        }
        // Precomputed argparse Option fields
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(_) = value.as_ref() {
                if self.ctx.precomputed_option_fields.contains(attr) {
                    let has_var_name = format!("has_{}", attr);
                    let has_ident = syn::Ident::new(&has_var_name, proc_macro2::Span::call_site());
                    return if method == "is_some" {
                        Ok(Some(parse_quote! { #has_ident }))
                    } else {
                        Ok(Some(parse_quote! { !#has_ident }))
                    };
                }
            }
        }
        // Result-returning function calls
        if let HirExpr::Call { func, .. } = object {
            if self.ctx.type_mapper.nasa_mode
                && self.ctx.result_returning_functions.contains(func)
                && self.ctx.current_function_can_fail
            {
                let object_expr = object.to_rust_expr(self.ctx)?;
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                return Ok(Some(parse_quote! { #object_expr?.#method_ident() }));
            }
        }
        Ok(None)
    }

    /// CB-200 Batch 14: Handle subprocess.Child .wait() method
    fn try_convert_subprocess_wait(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if method != "wait" || !args.is_empty() {
            return Ok(None);
        }
        let HirExpr::Var(var_name) = object else {
            return Ok(None);
        };
        let Some(var_type) = self.ctx.var_types.get(var_name) else {
            return Ok(None);
        };
        let is_subprocess_child = matches!(
            var_type,
            Type::Custom(s) if s == "std::process::Child" || s == "Child"
        ) || matches!(
            var_type,
            Type::Optional(inner) if matches!(
                inner.as_ref(),
                Type::Custom(s) if s == "std::process::Child" || s == "Child"
            )
        );
        if !is_subprocess_child {
            return Ok(None);
        }
        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
        if matches!(var_type, Type::Optional(_)) {
            Ok(Some(parse_quote! {
                #var_ident.as_mut().expect("value is None").wait().ok().and_then(|s| s.code()).unwrap_or(-1)
            }))
        } else {
            Ok(Some(parse_quote! {
                #var_ident.wait().ok().and_then(|s| s.code()).unwrap_or(-1)
            }))
        }
    }

    /// CB-200 Batch 14: Handle dict.fromkeys and int.from_bytes class methods
    fn try_convert_class_method_call(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Var(var_name) = object else {
            return Ok(None);
        };
        if var_name == "dict" && method == "fromkeys" {
            return self.try_convert_dict_fromkeys(args);
        }
        if var_name == "int" && method == "from_bytes" {
            return self.try_convert_int_from_bytes(args);
        }
        Ok(None)
    }

    /// CB-200 Batch 14: dict.fromkeys(keys, default) class method
    fn try_convert_dict_fromkeys(&mut self, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;
        if arg_exprs.len() >= 2 {
            let keys_expr = &arg_exprs[0];
            let default_expr = &arg_exprs[1];
            Ok(Some(parse_quote! {
                #keys_expr.iter().map(|k| (k.clone(), #default_expr)).collect()
            }))
        } else if arg_exprs.len() == 1 {
            let keys_expr = &arg_exprs[0];
            Ok(Some(parse_quote! {
                #keys_expr.iter().map(|k| (k.clone(), ())).collect()
            }))
        } else {
            Ok(None)
        }
    }

    /// CB-200 Batch 14: int.from_bytes(bytes, byteorder) class method
    fn try_convert_int_from_bytes(&mut self, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;
        if arg_exprs.len() < 2 {
            return Ok(None);
        }
        let bytes_expr = &arg_exprs[0];
        let is_big_endian = if let HirExpr::Literal(Literal::String(s)) = &args[1] {
            s == "big"
        } else {
            true
        };
        if is_big_endian {
            Ok(Some(parse_quote! {
                i64::from_be_bytes({
                    let mut arr = [0u8; 8];
                    let bytes: &[u8] = #bytes_expr.as_ref();
                    let start = 8usize.saturating_sub(bytes.len());
                    arr[start..].copy_from_slice(bytes);
                    arr
                })
            }))
        } else {
            Ok(Some(parse_quote! {
                i64::from_le_bytes({
                    let mut arr = [0u8; 8];
                    let bytes: &[u8] = #bytes_expr.as_ref();
                    arr[..bytes.len().min(8)].copy_from_slice(&bytes[..bytes.len().min(8)]);
                    arr
                })
            }))
        }
    }
}
