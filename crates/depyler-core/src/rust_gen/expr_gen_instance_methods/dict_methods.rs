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
            "get" => {
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    // DEPYLER-0330: Keep dict.get() as Option to support .is_none() checks
                    // Python: result = d.get(key); if result is None: ...
                    // Rust: let result = d.get(key).cloned(); if result.is_none() { ... }

                    // DEPYLER-1316: For DepylerValue, use get_str() for string keys
                    if object_is_depyler_value {
                        // Check if key is a string literal or string variable
                        if let Some(HirExpr::Literal(Literal::String(s))) = hir_args.first() {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            return Ok(parse_quote! { #object_expr.get_str(#lit).cloned() });
                        } else if let Some(HirExpr::Var(var_name)) = hir_args.first() {
                            // String variable - borrow if needed
                            if self.is_borrowed_str_param(var_name) {
                                return Ok(parse_quote! { #object_expr.get_str(#key).cloned() });
                            } else {
                                return Ok(parse_quote! { #object_expr.get_str(&#key).cloned() });
                            }
                        }
                        // For non-string keys on DepylerValue, fall through to standard handling
                    }

                    // DEPYLER-0542: Always borrow the key to prevent move semantics issues
                    // HashMap::get() expects &Q where Q: Borrow<K>. Using & prevents:
                    // 1. Moving owned String keys (error E0382: use of moved value)
                    // 2. Type mismatches when key is &str vs String
                    // For &str params, &key becomes &&str but HashMap::get handles this fine
                    let key_expr: syn::Expr = if let Some(HirExpr::Var(var_name)) = hir_args.first()
                    {
                        // DEPYLER-0539: Check if var is known &str param - don't double borrow
                        if self.is_borrowed_str_param(var_name) {
                            parse_quote! { #key }
                        } else {
                            // Owned String or unknown - borrow to prevent move
                            parse_quote! { &#key }
                        }
                    } else if let Some(HirExpr::Literal(Literal::String(s))) = hir_args.first() {
                        // DEPYLER-0634: String literal key - use bare literal, not .to_string()
                        // HashMap.get() expects &Q where Q: Borrow<K>. A &str literal works
                        // directly with Borrow<String> because String implements Borrow<str>.
                        // Using "key".to_string() creates owned String which doesn't match &Q.
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    } else {
                        // Other expression - borrow to prevent move
                        parse_quote! { &#key }
                    };

                    // Return Option - downstream code will handle unwrapping if needed
                    Ok(parse_quote! { #object_expr.get(#key_expr).cloned() })
                } else if arg_exprs.len() == 2 {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];

                    // DEPYLER-1316: For DepylerValue, use get_str() for string keys
                    if object_is_depyler_value {
                        // Check if key is a string literal or string variable
                        if let Some(HirExpr::Literal(Literal::String(s))) = hir_args.first() {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            // For DepylerValue, wrap the default in DepylerValue if needed
                            let default_expr: syn::Expr = match hir_args.get(1) {
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
                            };
                            return Ok(
                                parse_quote! { #object_expr.get_str(#lit).cloned().unwrap_or(#default_expr) },
                            );
                        } else if let Some(HirExpr::Var(var_name)) = hir_args.first() {
                            let default_expr: syn::Expr = match hir_args.get(1) {
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
                            };
                            if self.is_borrowed_str_param(var_name) {
                                return Ok(
                                    parse_quote! { #object_expr.get_str(#key).cloned().unwrap_or(#default_expr) },
                                );
                            } else {
                                return Ok(
                                    parse_quote! { #object_expr.get_str(&#key).cloned().unwrap_or(#default_expr) },
                                );
                            }
                        }
                    }

                    // DEPYLER-0542: Borrow keys for dict.get() (but not string literals)
                    let key_expr: syn::Expr = if let Some(HirExpr::Var(var_name)) = hir_args.first()
                    {
                        if self.is_borrowed_str_param(var_name) {
                            parse_quote! { #key }
                        } else {
                            parse_quote! { &#key }
                        }
                    } else if let Some(HirExpr::Literal(Literal::String(s))) = hir_args.first() {
                        // DEPYLER-0634: String literal key - use bare literal, not .to_string()
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    } else {
                        parse_quote! { &#key }
                    };

                    // DEPYLER-0700: Check if dict has serde_json::Value values (heterogeneous dict)
                    // If so, we need to wrap the default with serde_json::json!() for type compatibility
                    let dict_has_json_values = self.dict_has_json_value_values(hir_object);

                    // DEPYLER-0631: For string literal defaults, use directly without .to_string()
                    // HashMap<String, &str>.get() returns Option<&&str>, .cloned() gives Option<&str>
                    // unwrap_or expects &str, not String
                    let result = if dict_has_json_values {
                        // DEPYLER-0700: Dict has serde_json::Value values
                        // For dict.get(key, default), we need to:
                        // 1. Get the Value from dict
                        // 2. Convert to the expected type (usually String)
                        // Pattern: dict.get(key).and_then(|v| v.as_str()).unwrap_or(default).to_string()
                        self.ctx.needs_serde_json = true;
                        if matches!(hir_args.get(1), Some(HirExpr::Literal(Literal::String(s))) if !s.is_empty())
                        {
                            // String default - extract as string with fallback
                            if let Some(HirExpr::Literal(Literal::String(s))) = hir_args.get(1) {
                                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                                parse_quote! { #object_expr.get(#key_expr).and_then(|v| v.as_str()).unwrap_or(#lit).to_string() }
                            } else {
                                parse_quote! { #object_expr.get(#key_expr).and_then(|v| v.as_str()).unwrap_or(#default).to_string() }
                            }
                        } else {
                            // Non-string default - use json!() and keep as Value
                            parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(serde_json::json!(#default)) }
                        }
                    } else if matches!(hir_args.get(1), Some(HirExpr::Literal(Literal::String(_))))
                    {
                        // DEPYLER-0729: String literal default
                        // Check if dict value type is String (needs .to_string()) or &str (bare literal ok)
                        let dict_value_is_string = self.dict_value_type_is_string(hir_object);
                        if let HirExpr::Literal(Literal::String(s)) = &hir_args[1] {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            if dict_value_is_string {
                                // HashMap<K, String>.get().cloned() returns Option<String>
                                // unwrap_or needs String, so convert literal
                                parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#lit.to_string()) }
                            } else {
                                // HashMap<K, &str> or unknown - use bare literal
                                parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#lit) }
                            }
                        } else {
                            parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#default) }
                        }
                    } else {
                        // Non-literal default - use as-is
                        parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#default) }
                    };
                    Ok(result)
                } else if arg_exprs.is_empty() {
                    // DEPYLER-0188: 0-arg get() is NOT dict.get() - fall through to generic handler
                    // This supports asyncio.Queue.get(), multiprocessing.Queue.get(), etc.
                    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                    Ok(parse_quote! { #object_expr.#method_ident() })
                } else {
                    bail!("get() requires 1 or 2 arguments (or 0 for Queue.get())");
                }
            }
            "keys" => {
                // DEPYLER-0596: If keys() has arguments, it's a user-defined method, not dict.keys()
                // Fall through to generic handler for user-defined keys(section) methods
                if arg_exprs.is_empty() {
                    // DEPYLER-0303 Phase 3 Fix #8: Return Vec for compatibility
                    // .keys() returns an iterator, but Python's dict.keys() returns a list-like view
                    // We collect to Vec for better ergonomics (indexing, len(), etc.)
                    // DEPYLER-0540: serde_json::Value needs .as_object().unwrap() before .keys()
                    if is_json_value {
                        Ok(
                            parse_quote! { #object_expr.as_object().expect("expected JSON object").keys().cloned().collect::<Vec<_>>() },
                        )
                    } else {
                        Ok(parse_quote! { #object_expr.keys().cloned().collect::<Vec<_>>() })
                    }
                } else {
                    // User-defined keys() method with arguments - use generic call
                    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                    Ok(parse_quote! { #object_expr.#method_ident(#(#arg_exprs),*) })
                }
            }
            "values" => {
                if !arg_exprs.is_empty() {
                    bail!("values() takes no arguments");
                }
                // DEPYLER-0303 Phase 3 Fix #8: Return Vec for compatibility
                // However, this causes redundant .collect().iter() in sum(d.values())
                // NOTE: Consider context-aware return type (Vec vs Iterator) for optimization (tracked in DEPYLER-0303)
                // DEPYLER-0540: serde_json::Value needs .as_object().unwrap() before .values()
                if is_json_value {
                    Ok(
                        parse_quote! { #object_expr.as_object().expect("expected JSON object").values().cloned().collect::<Vec<_>>() },
                    )
                } else {
                    Ok(parse_quote! { #object_expr.values().cloned().collect::<Vec<_>>() })
                }
            }
            "items" => {
                if !arg_exprs.is_empty() {
                    bail!("items() takes no arguments");
                }
                // DEPYLER-0540: serde_json::Value needs .as_object().unwrap() before .iter()
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
            "update" => {
                if arg_exprs.len() != 1 {
                    bail!("update() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // DEPYLER-0728: When iterating over borrowed HashMap<K, V>, iterator yields (&K, &V)
                // insert() expects (K, V), so we need to clone the references
                // Using .iter() explicitly handles both owned and borrowed dicts correctly
                Ok(parse_quote! {
                    for (k, v) in (#arg).iter() {
                        #object_expr.insert(k.clone(), v.clone());
                    }
                })
            }
            "setdefault" => {
                // dict.setdefault(key, default) - get or insert with default
                // Python: dict.setdefault(key, default) returns value at key, or inserts default and returns it
                // Rust: entry().or_insert(default).clone()
                if arg_exprs.len() != 2 {
                    bail!("setdefault() requires exactly 2 arguments (key, default)");
                }
                let key = &arg_exprs[0];
                let default = &arg_exprs[1];
                Ok(parse_quote! {
                    #object_expr.entry(#key).or_insert(#default).clone()
                })
            }
            "popitem" => {
                // dict.popitem() - remove and return arbitrary (key, value) pair
                // Python: dict.popitem() removes and returns arbitrary item, or raises KeyError
                // Rust: iter().next() to get first item, then remove it
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
            "pop" => {
                // dict.pop(key, default=None) - remove and return value for key
                // Python: dict.pop(key[, default]) removes key and returns value, or returns default
                // Rust: remove() returns Option, use unwrap_or() for default
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
            // DEPYLER-STDLIB-50: clear() - remove all items
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            // DEPYLER-STDLIB-50: copy() - shallow copy
            "copy" => {
                if !arg_exprs.is_empty() {
                    bail!("copy() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clone() })
            }
            _ => bail!("Unknown dict method: {}", method),
        }
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
