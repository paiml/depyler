//! Dict method handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size and improve maintainability.
//! Contains handlers for: get, keys, values, items, update, setdefault,
//! popitem, pop, clear, copy.

use crate::hir::*;
use crate::rust_gen::expr_gen::ExpressionConverter;
use anyhow::{bail, Result};
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Handle dict methods (get, keys, values, items, update)
    /// DEPYLER-0540: Added hir_object param to detect serde_json::Value types
    #[inline]
    pub(super) fn convert_dict_method(
        &mut self,
        object_expr: &syn::Expr,
        hir_object: &HirExpr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0540: Check if this is a serde_json::Value that needs special handling
        let is_json_value = self.is_serde_json_value(hir_object);

        // DEPYLER-1316: Check if object is a DepylerValue (heterogeneous dict wrapper)
        // DepylerValue.get(&DepylerValue) vs DepylerValue.get_str(&str)
        let object_is_depyler_value =
            self.expr_returns_depyler_value(hir_object) && self.ctx.type_mapper.nasa_mode;

        match method {
            "get" => self.convert_dict_get(
                object_expr,
                hir_object,
                arg_exprs,
                hir_args,
                object_is_depyler_value,
            ),
            "keys" => Self::convert_dict_keys(object_expr, arg_exprs, method, is_json_value),
            "values" => Self::convert_dict_values(object_expr, arg_exprs, is_json_value),
            "items" => Self::convert_dict_items(object_expr, arg_exprs, is_json_value),
            "update" => Self::convert_dict_update(object_expr, arg_exprs),
            "setdefault" => Self::convert_dict_setdefault(object_expr, arg_exprs),
            "popitem" => Self::convert_dict_popitem(object_expr, arg_exprs),
            "pop" => Self::convert_dict_pop(object_expr, arg_exprs),
            "clear" => Self::convert_dict_clear(object_expr, arg_exprs),
            "copy" => Self::convert_dict_copy(object_expr, arg_exprs),
            _ => bail!("Unknown dict method: {}", method),
        }
    }

    fn convert_dict_get(
        &mut self,
        object_expr: &syn::Expr,
        hir_object: &HirExpr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        object_is_depyler_value: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() == 1 {
            self.convert_dict_get_single(object_expr, arg_exprs, hir_args, object_is_depyler_value)
        } else if arg_exprs.len() == 2 {
            self.convert_dict_get_with_default(
                object_expr,
                hir_object,
                arg_exprs,
                hir_args,
                object_is_depyler_value,
            )
        } else if arg_exprs.is_empty() {
            // DEPYLER-0188: 0-arg get() is NOT dict.get() - fall through to generic handler
            // This supports asyncio.Queue.get(), multiprocessing.Queue.get(), etc.
            let method_ident = syn::Ident::new("get", proc_macro2::Span::call_site());
            Ok(parse_quote! { #object_expr.#method_ident() })
        } else {
            bail!("get() requires 1 or 2 arguments (or 0 for Queue.get())");
        }
    }

    fn convert_dict_get_single(
        &self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        object_is_depyler_value: bool,
    ) -> Result<syn::Expr> {
        let key = &arg_exprs[0];

        // DEPYLER-1316: For DepylerValue, use get_str() for string keys
        if object_is_depyler_value {
            if let Some(result) = self.try_depyler_value_get_str(object_expr, key, hir_args.first())
            {
                return Ok(result);
            }
        }

        let key_expr = self.build_dict_key_expr(key, hir_args.first());
        Ok(parse_quote! { #object_expr.get(#key_expr).cloned() })
    }

    fn try_depyler_value_get_str(
        &self,
        object_expr: &syn::Expr,
        key: &syn::Expr,
        hir_key: Option<&HirExpr>,
    ) -> Option<syn::Expr> {
        match hir_key {
            Some(HirExpr::Literal(Literal::String(s))) => {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                Some(parse_quote! { #object_expr.get_str(#lit).cloned() })
            }
            Some(HirExpr::Var(var_name)) => {
                if self.is_borrowed_str_param(var_name) {
                    Some(parse_quote! { #object_expr.get_str(#key).cloned() })
                } else {
                    Some(parse_quote! { #object_expr.get_str(&#key).cloned() })
                }
            }
            _ => None,
        }
    }

    fn build_dict_key_expr(&self, key: &syn::Expr, hir_key: Option<&HirExpr>) -> syn::Expr {
        if let Some(HirExpr::Var(var_name)) = hir_key {
            // DEPYLER-0539: Check if var is known &str param - don't double borrow
            if self.is_borrowed_str_param(var_name) {
                parse_quote! { #key }
            } else {
                parse_quote! { &#key }
            }
        } else if let Some(HirExpr::Literal(Literal::String(s))) = hir_key {
            // DEPYLER-0634: String literal key - use bare literal, not .to_string()
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        } else {
            parse_quote! { &#key }
        }
    }

    fn wrap_depyler_value_default(default: &syn::Expr, hir_default: Option<&HirExpr>) -> syn::Expr {
        match hir_default {
            Some(HirExpr::Literal(Literal::Int(i))) => {
                parse_quote! { DepylerValue::Int(#i) }
            }
            Some(HirExpr::Literal(Literal::Float(f))) => {
                parse_quote! { DepylerValue::Float(#f) }
            }
            Some(HirExpr::Literal(Literal::String(s))) => {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                parse_quote! { DepylerValue::Str(#lit.to_string()) }
            }
            _ => parse_quote! { #default },
        }
    }

    fn convert_dict_get_with_default(
        &mut self,
        object_expr: &syn::Expr,
        hir_object: &HirExpr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        object_is_depyler_value: bool,
    ) -> Result<syn::Expr> {
        let key = &arg_exprs[0];
        let default = &arg_exprs[1];

        // DEPYLER-1316: For DepylerValue, use get_str() for string keys
        if object_is_depyler_value {
            if let Some(result) =
                self.try_depyler_value_get_with_default(object_expr, key, default, hir_args)
            {
                return Ok(result);
            }
        }

        let key_expr = self.build_dict_key_expr(key, hir_args.first());
        let result =
            self.build_dict_get_default_expr(object_expr, hir_object, &key_expr, default, hir_args);
        Ok(result)
    }

    fn try_depyler_value_get_with_default(
        &self,
        object_expr: &syn::Expr,
        key: &syn::Expr,
        default: &syn::Expr,
        hir_args: &[HirExpr],
    ) -> Option<syn::Expr> {
        let default_expr = Self::wrap_depyler_value_default(default, hir_args.get(1));
        match hir_args.first() {
            Some(HirExpr::Literal(Literal::String(s))) => {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                Some(parse_quote! { #object_expr.get_str(#lit).cloned().unwrap_or(#default_expr) })
            }
            Some(HirExpr::Var(var_name)) => {
                if self.is_borrowed_str_param(var_name) {
                    Some(
                        parse_quote! { #object_expr.get_str(#key).cloned().unwrap_or(#default_expr) },
                    )
                } else {
                    Some(
                        parse_quote! { #object_expr.get_str(&#key).cloned().unwrap_or(#default_expr) },
                    )
                }
            }
            _ => None,
        }
    }

    fn build_dict_get_default_expr(
        &mut self,
        object_expr: &syn::Expr,
        hir_object: &HirExpr,
        key_expr: &syn::Expr,
        default: &syn::Expr,
        hir_args: &[HirExpr],
    ) -> syn::Expr {
        // DEPYLER-0700: Check if dict has serde_json::Value values (heterogeneous dict)
        let dict_has_json_values = self.dict_has_json_value_values(hir_object);

        if dict_has_json_values {
            self.build_json_value_default_expr(object_expr, key_expr, default, hir_args)
        } else if matches!(hir_args.get(1), Some(HirExpr::Literal(Literal::String(_)))) {
            self.build_string_literal_default_expr(
                object_expr,
                hir_object,
                key_expr,
                default,
                hir_args,
            )
        } else {
            parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#default) }
        }
    }

    fn build_json_value_default_expr(
        &mut self,
        object_expr: &syn::Expr,
        key_expr: &syn::Expr,
        default: &syn::Expr,
        hir_args: &[HirExpr],
    ) -> syn::Expr {
        self.ctx.needs_serde_json = true;
        if matches!(hir_args.get(1), Some(HirExpr::Literal(Literal::String(s))) if !s.is_empty()) {
            if let Some(HirExpr::Literal(Literal::String(s))) = hir_args.get(1) {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                return parse_quote! { #object_expr.get(#key_expr).and_then(|v| v.as_str()).unwrap_or(#lit).to_string() };
            }
            parse_quote! { #object_expr.get(#key_expr).and_then(|v| v.as_str()).unwrap_or(#default).to_string() }
        } else {
            parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(serde_json::json!(#default)) }
        }
    }

    fn build_string_literal_default_expr(
        &self,
        object_expr: &syn::Expr,
        hir_object: &HirExpr,
        key_expr: &syn::Expr,
        default: &syn::Expr,
        hir_args: &[HirExpr],
    ) -> syn::Expr {
        // DEPYLER-0729: String literal default
        let dict_value_is_string = self.dict_value_type_is_string(hir_object);
        if let HirExpr::Literal(Literal::String(s)) = &hir_args[1] {
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            if dict_value_is_string {
                parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#lit.to_string()) }
            } else {
                parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#lit) }
            }
        } else {
            parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#default) }
        }
    }

    fn convert_dict_keys(
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        method: &str,
        is_json_value: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            if is_json_value {
                Ok(
                    parse_quote! { #object_expr.as_object().expect("expected JSON object").keys().cloned().collect::<Vec<_>>() },
                )
            } else {
                Ok(parse_quote! { #object_expr.keys().cloned().collect::<Vec<_>>() })
            }
        } else {
            let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
            Ok(parse_quote! { #object_expr.#method_ident(#(#arg_exprs),*) })
        }
    }

    fn convert_dict_values(
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        is_json_value: bool,
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("values() takes no arguments");
        }
        if is_json_value {
            Ok(
                parse_quote! { #object_expr.as_object().expect("expected JSON object").values().cloned().collect::<Vec<_>>() },
            )
        } else {
            Ok(parse_quote! { #object_expr.values().cloned().collect::<Vec<_>>() })
        }
    }

    fn convert_dict_items(
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        is_json_value: bool,
    ) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("items() takes no arguments");
        }
        if is_json_value {
            Ok(
                parse_quote! { #object_expr.as_object().expect("expected JSON object").iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() },
            )
        } else {
            Ok(
                parse_quote! { #object_expr.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() },
            )
        }
    }

    fn convert_dict_update(object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("update() requires exactly one argument");
        }
        let arg = &arg_exprs[0];
        Ok(parse_quote! {
            for (k, v) in (#arg).iter() {
                #object_expr.insert(k.clone(), v.clone());
            }
        })
    }

    fn convert_dict_setdefault(
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 2 {
            bail!("setdefault() requires exactly 2 arguments (key, default)");
        }
        let key = &arg_exprs[0];
        let default = &arg_exprs[1];
        Ok(parse_quote! {
            #object_expr.entry(#key).or_insert(#default).clone()
        })
    }

    fn convert_dict_popitem(object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("popitem() takes no arguments");
        }
        Ok(parse_quote! {
            {
                let key = #object_expr.keys().next().cloned()
                    .expect("KeyError: popitem(): dictionary is empty");
                let value = #object_expr.remove(&key)
                    .expect("KeyError: key disappeared");
                (key, value)
            }
        })
    }

    fn convert_dict_pop(object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.is_empty() || arg_exprs.len() > 2 {
            bail!("pop() requires 1 or 2 arguments (key, optional default)");
        }
        let key = &arg_exprs[0];
        if arg_exprs.len() == 2 {
            let default = &arg_exprs[1];
            Ok(parse_quote! {
                #object_expr.remove(#key).unwrap_or(#default)
            })
        } else {
            Ok(parse_quote! {
                #object_expr.remove(#key).expect("KeyError: key not found")
            })
        }
    }

    fn convert_dict_clear(object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("clear() takes no arguments");
        }
        Ok(parse_quote! { #object_expr.clear() })
    }

    fn convert_dict_copy(object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("copy() takes no arguments");
        }
        Ok(parse_quote! { #object_expr.clone() })
    }

    /// DEPYLER-0564: Check if object is dict value access that returns serde_json::Value
    /// When calling string methods on dict values, we need to convert Value to &str first
    #[inline]
    pub(super) fn needs_value_to_string_conversion(&self, hir_object: &HirExpr) -> bool {
        // Pattern: dict["key"] where dict is HashMap<String, serde_json::Value>
        if let HirExpr::Index { base, .. } = hir_object {
            if let HirExpr::Var(var_name) = base.as_ref() {
                // Check if the variable is tracked as a Dict with Unknown value type
                if let Some(Type::Dict(_, val_type)) = self.ctx.var_types.get(var_name) {
                    return matches!(val_type.as_ref(), Type::Unknown);
                }
                // Heuristic: common dict variable names
                let name = var_name.as_str();
                return name == "info" || name == "data" || name == "config" || name == "result";
            }
        }
        // Pattern: dict.get("key") - check nested method chains
        self.check_dict_value_chain(hir_object)
    }

    /// DEPYLER-0564: Recursively check if expression is a dict value access chain
    pub(super) fn check_dict_value_chain(&self, expr: &HirExpr) -> bool {
        match expr {
            // Direct dict.get("key") call
            HirExpr::MethodCall { object, method, .. } if method == "get" => {
                if let HirExpr::Var(var_name) = object.as_ref() {
                    let name = var_name.as_str();
                    return name == "info"
                        || name == "data"
                        || name == "config"
                        || name == "result";
                }
                false
            }
            // Chained method calls like dict.get("key").cloned().unwrap_or_default()
            HirExpr::MethodCall { object, method, .. }
                if method == "cloned" || method == "unwrap_or_default" || method == "unwrap" =>
            {
                // Check if base object is a dict access
                self.check_dict_value_chain(object)
            }
            _ => false,
        }
    }

    /// DEPYLER-0564: Check if Rust expression is likely a serde_json::Value
    /// by looking for patterns like .unwrap_or_default() which indicate dict value access
    pub(super) fn rust_expr_needs_value_conversion(&self, expr: &syn::Expr) -> bool {
        // Convert to string and check for patterns
        let expr_str = quote::quote!(#expr).to_string();
        // Remove spaces for easier pattern matching
        let normalized = expr_str.replace(' ', "");
        // Pattern: .unwrap_or_default() on a .get() call suggests serde_json::Value
        if normalized.contains("unwrap_or_default") && normalized.contains(".get(") {
            // Check for common dict variable names
            return normalized.contains("info.")
                || normalized.contains("data.")
                || normalized.contains("config.")
                || normalized.contains("result.")
                || normalized.contains("stats.");
        }
        false
    }
}
