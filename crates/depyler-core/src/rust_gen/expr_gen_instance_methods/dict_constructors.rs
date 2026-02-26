//! Dict constructor handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size and improve maintainability.
//! Contains handlers for: dict literal conversion and related helpers.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::direct_rules::type_to_rust_type;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::quote;
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_dict(&mut self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
        // CITL: Trace dict construction decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "dict_construction",
            chosen = "hashmap_or_json",
            alternatives = ["HashMap", "BTreeMap", "serde_json", "IndexMap"],
            confidence = 0.85
        );

        // DEPYLER-1015: In NASA mode, use std-only types (no serde_json)
        // Convert all values to String for single-shot compile compatibility
        let nasa_mode = self.ctx.type_mapper.nasa_mode;

        // DEPYLER-0376: Detect heterogeneous dicts (mixed value types)
        // DEPYLER-0461: Also check if we're in json!() context (nested dicts must use json!())
        // DEPYLER-0560: Check if function returns Dict with Any/Unknown value type
        // For mixed types or json context, use serde_json::json! instead of HashMap
        // DEPYLER-1023: In NASA mode, still detect mixed types but convert to String instead of json
        let has_mixed_types = self.dict_has_mixed_types(items)?;
        let in_json_context = if nasa_mode { false } else { self.ctx.in_json_context };

        // DEPYLER-0560: Check if return type requires serde_json::Value
        // If function returns Dict[str, Any] → HashMap<String, serde_json::Value>
        // DEPYLER-1015: Skip in NASA mode
        let return_needs_json = if nasa_mode { false } else { self.return_type_needs_json_dict() };

        // DEPYLER-1045: Check if target type annotation requires DepylerValue
        // When `values: dict = {...}`, the type annotation maps to DepylerValue values
        // even if all literal values are strings (homogeneous)
        // DEPYLER-1166: Also check for Type::Custom("DepylerValue") which is what Dict[str, Any] maps to in NASA mode
        let target_needs_depyler_value = if let Some(Type::Dict(_, val_type)) =
            &self.ctx.current_assign_type
        {
            // Unknown or DepylerValue value type → use DepylerValue wrapping
            matches!(val_type.as_ref(), Type::Unknown)
                || matches!(val_type.as_ref(), Type::Custom(name) if name == "DepylerValue" || name == "Any" || name == "serde_json::Value")
        } else {
            false
        };

        // DEPYLER-1141: Check if target type annotation specifies a CONCRETE value type
        // When `stats: Dict[str, float] = {...}`, even if values are mixed (int/float),
        // we should coerce to the target type instead of using DepylerValue.
        // This enables Python's implicit int→float coercion.
        // DEPYLER-1153: ALSO check for nested Dict/List with concrete element types
        // e.g., Dict[str, Dict[str, int]] should NOT use DepylerValue wrapping
        let target_has_concrete_value_type =
            if let Some(Type::Dict(_, val_type)) = &self.ctx.current_assign_type {
                self.is_concrete_type(val_type.as_ref())
            } else {
                false
            };

        // DEPYLER-1050: Check if function return type requires DepylerValue
        // When function returns HashMap<String, DepylerValue>, ALL dict literals in the
        // function body must use DepylerValue wrapping (even in nested return statements)
        // DEPYLER-1166: Also check for Type::Custom("DepylerValue") which is what Dict[str, Any] maps to in NASA mode
        let return_needs_depyler_value = if let Some(Type::Dict(_, val_type)) =
            &self.ctx.current_return_type
        {
            // Unknown or DepylerValue value type → use DepylerValue wrapping
            matches!(val_type.as_ref(), Type::Unknown)
                || matches!(val_type.as_ref(), Type::Custom(name) if name == "DepylerValue" || name == "Any" || name == "serde_json::Value")
        } else {
            false
        };

        // DEPYLER-1060: Check if dict has non-string keys
        // Point 14: {1: "a"} requires DepylerValue keys, not String keys
        let has_non_string_keys =
            items.iter().any(|(key, _)| !matches!(key, HirExpr::Literal(Literal::String(_))));

        // DEPYLER-1023: In NASA mode with mixed types, use DepylerValue enum
        // This ensures proper type fidelity for heterogeneous Python dicts
        // DEPYLER-1045: Also use DepylerValue when target type annotation requires it
        // DEPYLER-1050: Also use DepylerValue when function return type requires it
        // DEPYLER-1060: Also use DepylerValue when dict has non-string keys
        // DEPYLER-1141: BUT skip DepylerValue when target has CONCRETE value type
        // This allows `Dict[str, float]` to coerce int→float instead of wrapping in DepylerValue
        if nasa_mode
            && !target_has_concrete_value_type
            && (has_mixed_types
                || target_needs_depyler_value
                || return_needs_depyler_value
                || has_non_string_keys)
        {
            self.ctx.needs_hashmap = true;
            self.ctx.needs_depyler_value_enum = true;

            // DEPYLER-1047: Check if return/target type expects String keys
            // Pattern: `fn f() -> HashMap<String, DepylerValue>` should use String keys
            // NOT DepylerValue keys, even when values are DepylerValue
            // NOTE: Bare `dict` return type parses as Dict(Unknown, Unknown) but generates
            // HashMap<String, DepylerValue>, so Unknown key type also means String keys
            // DEPYLER-1213: Also handle Type::Custom("dict") which is how bare dict annotations are stored
            let return_expects_string_keys = match &self.ctx.current_return_type {
                Some(Type::Dict(key_type, _)) => {
                    matches!(key_type.as_ref(), Type::String | Type::Unknown)
                }
                // Bare `dict` annotation stored as Custom("dict") - treat as Dict[str, Any]
                Some(Type::Custom(name)) if name == "dict" || name == "Dict" => true,
                _ => false,
            };
            let target_expects_string_keys = match &self.ctx.current_assign_type {
                Some(Type::Dict(key_type, _)) => {
                    matches!(key_type.as_ref(), Type::String | Type::Unknown)
                }
                Some(Type::Custom(name)) if name == "dict" || name == "Dict" => true,
                _ => false,
            };
            let use_string_keys =
                (return_expects_string_keys || target_expects_string_keys) && !has_non_string_keys;

            let mut insert_stmts = Vec::new();
            for (key, value) in items {
                let key_expr_raw = key.to_rust_expr(self.ctx)?;

                // DEPYLER-1166: Propagate DepylerValue context for nested Dict/List values
                // When converting a nested dict that will become a DepylerValue, set the context
                // so the inner dict also uses DepylerValue wrapping for its values
                let prev_assign_type = self.ctx.current_assign_type.clone();
                if matches!(value, HirExpr::Dict(_) | HirExpr::List(_)) {
                    // Set context to Dict with Unknown value type → triggers DepylerValue
                    self.ctx.current_assign_type =
                        Some(Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)));
                }
                let val_expr = value.to_rust_expr(self.ctx)?;
                // Restore context
                self.ctx.current_assign_type = prev_assign_type;

                // DEPYLER-1047: Use String keys when return/target type expects String keys
                // DEPYLER-1060: Wrap keys in DepylerValue to support non-string keys
                // Point 14: {1: "a"} must use DepylerValue::Int(1), not String
                let key_expr: syn::Expr = if use_string_keys {
                    // Return type expects HashMap<String, _>, use String keys
                    match key {
                        HirExpr::Literal(Literal::String(_)) => {
                            parse_quote! { #key_expr_raw.to_string() }
                        }
                        _ => {
                            // For non-string keys in string-key context, convert to string
                            parse_quote! { format!("{}", #key_expr_raw) }
                        }
                    }
                } else {
                    // Return type expects HashMap<DepylerValue, _>, use DepylerValue keys
                    match key {
                        HirExpr::Literal(Literal::Int(_)) => {
                            parse_quote! { DepylerValue::Int(#key_expr_raw as i64) }
                        }
                        HirExpr::Literal(Literal::Float(_)) => {
                            parse_quote! { DepylerValue::Float(#key_expr_raw as f64) }
                        }
                        HirExpr::Literal(Literal::String(_)) => {
                            parse_quote! { DepylerValue::Str(#key_expr_raw.to_string()) }
                        }
                        HirExpr::Literal(Literal::Bool(_)) => {
                            parse_quote! { DepylerValue::Bool(#key_expr_raw) }
                        }
                        HirExpr::Var(_) => {
                            // For variables, use .into() to convert to DepylerValue
                            parse_quote! { DepylerValue::from(#key_expr_raw) }
                        }
                        _ => {
                            // For complex expressions, try .into()
                            parse_quote! { DepylerValue::from(#key_expr_raw) }
                        }
                    }
                };

                // Wrap values in DepylerValue enum variants
                let wrapped_val: syn::Expr = match value {
                    HirExpr::Literal(Literal::Int(_)) => {
                        parse_quote! { DepylerValue::Int(#val_expr as i64) }
                    }
                    HirExpr::Literal(Literal::Float(_)) => {
                        parse_quote! { DepylerValue::Float(#val_expr as f64) }
                    }
                    HirExpr::Literal(Literal::String(_)) => {
                        parse_quote! { DepylerValue::Str(#val_expr.to_string()) }
                    }
                    HirExpr::Literal(Literal::Bool(_)) => {
                        parse_quote! { DepylerValue::Bool(#val_expr) }
                    }
                    HirExpr::Literal(Literal::None) => {
                        parse_quote! { DepylerValue::None }
                    }
                    HirExpr::Var(name) => {
                        let var_type = self.ctx.var_types.get(name);
                        match var_type {
                            Some(Type::Int) => parse_quote! { DepylerValue::Int(#val_expr as i64) },
                            Some(Type::Float) => {
                                parse_quote! { DepylerValue::Float(#val_expr as f64) }
                            }
                            Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#val_expr) },
                            Some(Type::String) => {
                                parse_quote! { DepylerValue::Str(#val_expr.to_string()) }
                            }
                            _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) },
                        }
                    }
                    // DEPYLER-1040: Handle struct field access (e.g., args.debug, args.count)
                    // DEPYLER-1143: Also check argparse field types for proper wrapping
                    HirExpr::Attribute { value: attr_value, attr, .. } => {
                        // First try class_field_types
                        let found_type: Option<&Type> = self.ctx.class_field_types.get(attr);
                        let mut inferred_type_str: Option<String> = None;

                        // DEPYLER-1143: If not found, check argparse args.field access
                        // IMPORTANT: Always use rust_type() for argparse fields, not arg_type
                        // Because rust_type() includes Option<T> wrapping for optional arguments
                        // while arg_type only has the inner Python type (e.g., Type::Int instead of Option<i32>)
                        if found_type.is_none() {
                            if let HirExpr::Var(obj_name) = attr_value.as_ref() {
                                for parser_info in self.ctx.argparser_tracker.parsers.values() {
                                    if let Some(ref args_var) = parser_info.args_var {
                                        if args_var == obj_name {
                                            for arg in &parser_info.arguments {
                                                if arg.rust_field_name() == *attr {
                                                    // Always use rust_type() which accounts for Option wrapping
                                                    inferred_type_str = Some(arg.rust_type());
                                                    break;
                                                }
                                            }
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(field_type) = found_type {
                            match field_type {
                                Type::Int => parse_quote! { DepylerValue::Int(#val_expr as i64) },
                                Type::Float => {
                                    parse_quote! { DepylerValue::Float(#val_expr as f64) }
                                }
                                Type::Bool => parse_quote! { DepylerValue::Bool(#val_expr) },
                                Type::String => {
                                    parse_quote! { DepylerValue::Str(#val_expr.to_string()) }
                                }
                                Type::Optional(inner) => match inner.as_ref() {
                                    Type::Int => parse_quote! {
                                        match #val_expr {
                                            Some(v) => DepylerValue::Int(v as i64),
                                            None => DepylerValue::None,
                                        }
                                    },
                                    Type::String => parse_quote! {
                                        match #val_expr {
                                            Some(v) => DepylerValue::Str(v.to_string()),
                                            None => DepylerValue::None,
                                        }
                                    },
                                    _ => {
                                        parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) }
                                    }
                                },
                                Type::List(_) => parse_quote! {
                                    DepylerValue::List(#val_expr.iter().map(|v| DepylerValue::Str(v.to_string())).collect())
                                },
                                _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) },
                            }
                        } else if let Some(type_str) = inferred_type_str {
                            // DEPYLER-1143: Handle inferred types from argparse rust_type()
                            match type_str.as_str() {
                                "bool" => parse_quote! { DepylerValue::Bool(#val_expr) },
                                "i32" | "i64" | "isize" => {
                                    parse_quote! { DepylerValue::Int(#val_expr as i64) }
                                }
                                "u8" | "u32" | "u64" => {
                                    parse_quote! { DepylerValue::Int(#val_expr as i64) }
                                }
                                "f32" | "f64" => {
                                    parse_quote! { DepylerValue::Float(#val_expr as f64) }
                                }
                                "String" => {
                                    parse_quote! { DepylerValue::Str(#val_expr.to_string()) }
                                }
                                s if s.starts_with("Vec<") => parse_quote! {
                                    DepylerValue::List(#val_expr.iter().map(|v| DepylerValue::Str(v.to_string())).collect())
                                },
                                s if s.starts_with("Option<") && s.contains("String") => {
                                    parse_quote! {
                                        match #val_expr {
                                            Some(v) => DepylerValue::Str(v.to_string()),
                                            None => DepylerValue::None,
                                        }
                                    }
                                }
                                // DEPYLER-1143: Handle Option<i32/i64/f32/f64> with proper numeric wrapping
                                s if s.starts_with("Option<i") || s.starts_with("Option<u") => {
                                    parse_quote! {
                                        match #val_expr {
                                            Some(v) => DepylerValue::Int(v as i64),
                                            None => DepylerValue::None,
                                        }
                                    }
                                }
                                s if s.starts_with("Option<f") => parse_quote! {
                                    match #val_expr {
                                        Some(v) => DepylerValue::Float(v as f64),
                                        None => DepylerValue::None,
                                    }
                                },
                                s if s.starts_with("Option<") => parse_quote! {
                                    match #val_expr {
                                        Some(v) => DepylerValue::Str(format!("{:?}", v)),
                                        None => DepylerValue::None,
                                    }
                                },
                                _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) },
                            }
                        } else {
                            // DEPYLER-1040: Without explicit type info, use safe stringify
                            // Name-based heuristics are unreliable because fields might be Option<T>
                            // (e.g., args.count could be Option<i32>, not i32)
                            // Using format!("{:?}") is safe for any type
                            parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) }
                        }
                    }
                    // DEPYLER-1166: Handle nested Dict values
                    // Wrap inner HashMap<String, DepylerValue> as DepylerValue::Dict(HashMap<DepylerValue, DepylerValue>)
                    // by converting String keys to DepylerValue::Str keys
                    HirExpr::Dict(_) => {
                        parse_quote! {
                            DepylerValue::Dict(#val_expr.into_iter()
                                .map(|(k, v)| (DepylerValue::Str(k), v))
                                .collect())
                        }
                    }
                    // DEPYLER-1166: Handle nested List values
                    // Elements must be wrapped in DepylerValue variants
                    HirExpr::List(elts) => {
                        // Convert each element to DepylerValue
                        let wrapped_elements: Vec<syn::Expr> = elts
                            .iter()
                            .map(|elem| {
                                let elem_expr = elem.to_rust_expr(self.ctx)?;
                                Ok(match elem {
                                    HirExpr::Literal(Literal::Int(_)) => {
                                        parse_quote! { DepylerValue::Int(#elem_expr as i64) }
                                    }
                                    HirExpr::Literal(Literal::Float(_)) => {
                                        parse_quote! { DepylerValue::Float(#elem_expr as f64) }
                                    }
                                    HirExpr::Literal(Literal::String(_)) => {
                                        parse_quote! { DepylerValue::Str(#elem_expr.to_string()) }
                                    }
                                    HirExpr::Literal(Literal::Bool(_)) => {
                                        parse_quote! { DepylerValue::Bool(#elem_expr) }
                                    }
                                    HirExpr::Literal(Literal::None) => {
                                        parse_quote! { DepylerValue::None }
                                    }
                                    HirExpr::Dict(_) => {
                                        // Nested dict - wrap as DepylerValue::Dict
                                        parse_quote! {
                                            DepylerValue::Dict(#elem_expr.into_iter()
                                                .map(|(k, v)| (DepylerValue::Str(k), v))
                                                .collect())
                                        }
                                    }
                                    HirExpr::List(_) => {
                                        // Nested list - already converted recursively
                                        parse_quote! { DepylerValue::List(#elem_expr) }
                                    }
                                    _ => {
                                        // For variables/complex expressions, use from()
                                        parse_quote! { DepylerValue::from(#elem_expr) }
                                    }
                                })
                            })
                            .collect::<Result<Vec<_>>>()?;

                        parse_quote! { DepylerValue::List(vec![#(#wrapped_elements),*]) }
                    }
                    // Fallback for other complex types
                    _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) },
                };

                insert_stmts.push(quote! { map.insert(#key_expr, #wrapped_val); });
            }

            // DEPYLER-1159: Add explicit type annotations to HashMap to help type inference
            // Without type annotations, empty or nested dicts can cause E0282 errors
            // when the type can't be inferred from context (e.g., inside format! macro)
            return if use_string_keys {
                Ok(parse_quote! {
                    {
                        let mut map: HashMap<String, DepylerValue> = HashMap::new();
                        #(#insert_stmts)*
                        map
                    }
                })
            } else {
                Ok(parse_quote! {
                    {
                        let mut map: HashMap<DepylerValue, DepylerValue> = HashMap::new();
                        #(#insert_stmts)*
                        map
                    }
                })
            };
        }

        // DEPYLER-0560: When inside json!() context (nested dict), use json!() macro
        // This produces serde_json::Value which is what nested contexts expect
        if in_json_context {
            self.ctx.needs_serde_json = true;
            let mut entries = Vec::new();
            for (key, value) in items {
                let key_str = match key {
                    HirExpr::Literal(Literal::String(s)) => s.clone(),
                    _ => bail!("Dict keys for JSON output must be string literals"),
                };
                let val_expr = value.to_rust_expr(self.ctx)?;
                entries.push(quote! { #key_str: #val_expr });
            }
            return Ok(parse_quote! {
                serde_json::json!({
                    #(#entries),*
                })
            });
        }

        // DEPYLER-0560: When return type is HashMap<String, serde_json::Value>,
        // build HashMap with json!() wrapped values (NOT a raw json!() object)
        // DEPYLER-1023: Skip in NASA mode (handled above with String conversion)
        if !nasa_mode && (has_mixed_types || return_needs_json) {
            self.ctx.needs_serde_json = true;
            self.ctx.needs_hashmap = true;

            let mut insert_stmts = Vec::new();
            for (key, value) in items {
                let key_str = match key {
                    HirExpr::Literal(Literal::String(s)) => s.clone(),
                    _ => bail!("Dict keys for JSON output must be string literals"),
                };

                // Set json context for value conversion (nested dicts become json!())
                let prev_json_context = self.ctx.in_json_context;
                self.ctx.in_json_context = true;
                let val_expr = value.to_rust_expr(self.ctx)?;
                self.ctx.in_json_context = prev_json_context;

                // DEPYLER-0669: Check if val_expr is a HashMap block (can't go in json!())
                let val_str = quote! { #val_expr }.to_string();
                let wrapped_val = if val_str.contains("HashMap") || val_str.contains("let mut map")
                {
                    // Use serde_json::to_value() for HashMap block expressions
                    quote! { serde_json::to_value(#val_expr).expect("operation failed") }
                } else {
                    // Wrap each value in json!() to convert to serde_json::Value
                    quote! { serde_json::json!(#val_expr) }
                };

                insert_stmts.push(quote! {
                    map.insert(#key_str.to_string(), #wrapped_val);
                });
            }

            return Ok(parse_quote! {
                {
                    let mut map = std::collections::HashMap::new();
                    #(#insert_stmts)*
                    map
                }
            });
        }

        // Homogeneous dict: use HashMap
        self.ctx.needs_hashmap = true;

        // DEPYLER-0740: Detect if any dict value is None
        // If so, wrap non-None values in Some() to create HashMap<K, Option<V>>
        // DEPYLER-0741: Also check context flag - set when list of dicts has ANY dict with None
        let has_none_value =
            items.iter().any(|(_, v)| matches!(v, HirExpr::Literal(Literal::None)));

        // Use Option wrapping if this dict has None OR if we're in a list context
        // where another dict has None (for type consistency)
        if has_none_value || self.ctx.force_dict_value_option_wrap {
            let mut insert_stmts = Vec::new();
            for (key, value) in items {
                let mut key_expr = key.to_rust_expr(self.ctx)?;

                // Convert string literal keys to owned Strings
                if matches!(key, HirExpr::Literal(Literal::String(_))) {
                    key_expr = parse_quote! { #key_expr.to_string() };
                }

                let val_expr: syn::Expr = if matches!(value, HirExpr::Literal(Literal::None)) {
                    // None stays as None
                    parse_quote! { None }
                } else {
                    // Non-None values get wrapped in Some()
                    let mut inner = value.to_rust_expr(self.ctx)?;
                    // Convert string literals to owned Strings
                    if matches!(value, HirExpr::Literal(Literal::String(_))) {
                        inner = parse_quote! { #inner.to_string() };
                    }
                    parse_quote! { Some(#inner) }
                };

                insert_stmts.push(quote! { map.insert(#key_expr, #val_expr); });
            }

            return Ok(parse_quote! {
                {
                    let mut map = HashMap::new();
                    #(#insert_stmts)*
                    map
                }
            });
        }

        // DEPYLER-0953: String literal values are now always converted to String
        // (Previously DEPYLER-0729 only converted when target type required it)
        // DEPYLER-1141: Get target value type for coercion
        let target_value_type = if let Some(Type::Dict(_, val_type)) = &self.ctx.current_assign_type
        {
            Some(val_type.as_ref().clone())
        } else {
            None
        };

        let mut insert_stmts = Vec::new();
        for (key, value) in items {
            let mut key_expr = key.to_rust_expr(self.ctx)?;

            // DEPYLER-1153: Propagate value type to nested dict/list values
            // For Dict[str, Dict[str, int]], inner dict needs current_assign_type = Dict[str, int]
            let prev_assign_type = self.ctx.current_assign_type.clone();
            if matches!(value, HirExpr::Dict(_) | HirExpr::List(_)) {
                if let Some(ref val_type) = target_value_type {
                    self.ctx.current_assign_type = Some(val_type.clone());
                }
            }
            let mut val_expr = value.to_rust_expr(self.ctx)?;
            self.ctx.current_assign_type = prev_assign_type;

            // DEPYLER-0810: Unwrap Result-returning function calls in dict value context
            // HashMap<K, V> expects V, not Result<V, E>, so we need to unwrap
            if let HirExpr::Call { func, .. } = value {
                if self.ctx.result_returning_functions.contains(func) {
                    let error_msg = format!("{} failed", func);
                    val_expr = parse_quote! { #val_expr.expect(#error_msg) };
                }
            }

            // DEPYLER-0270 FIX: ALWAYS convert string literal keys to owned Strings
            // Dict literals should use HashMap<String, V> not HashMap<&str, V>
            // This ensures they can be passed to functions expecting HashMap<String, V>
            if matches!(key, HirExpr::Literal(Literal::String(_))) {
                key_expr = parse_quote! { #key_expr.to_string() };
            }

            // DEPYLER-0729/0953: Convert string literal values to owned String
            // HashMap<K, String> is the standard pattern, not HashMap<K, &str>
            // This ensures consistent types across dict literal, access, and assignment
            // DEPYLER-1021: In NASA mode, DON'T convert primitive types (int, float, bool)
            // to String - this breaks type inference. Only convert string literals.
            // The original DEPYLER-1015 logic was too aggressive.
            if matches!(value, HirExpr::Literal(Literal::String(_))) {
                val_expr = parse_quote! { #val_expr.to_string() };
            }

            // DEPYLER-1141: Coerce values to target type when annotation specifies concrete type
            // This handles Python's implicit int→float coercion in Dict[str, float] = {"key": int_var}
            // IMPORTANT: Only coerce NUMERIC values - strings/bools must NOT be cast
            if let Some(ref target_type) = target_value_type {
                // Check if value is a numeric type that can be coerced
                let value_is_numeric = matches!(
                    value,
                    HirExpr::Literal(Literal::Int(_)) | HirExpr::Literal(Literal::Float(_))
                ) || {
                    if let HirExpr::Var(name) = value {
                        matches!(self.ctx.var_types.get(name), Some(Type::Int | Type::Float))
                    } else {
                        false
                    }
                };

                if value_is_numeric {
                    match target_type {
                        Type::Float => {
                            // Coerce to f64 - handles int→float coercion
                            val_expr = parse_quote! { (#val_expr) as f64 };
                        }
                        Type::Int => {
                            // Coerce to i32
                            val_expr = parse_quote! { (#val_expr) as i32 };
                        }
                        // String and Bool don't need coercion
                        _ => {}
                    }
                }
            }

            insert_stmts.push(quote! { map.insert(#key_expr, #val_expr); });
        }

        // DEPYLER-0279: Only add `mut` if there are items to insert
        // Empty dicts don't need mutable bindings
        // DEPYLER-0472: When in json context, use json!({}) instead of HashMap::new()
        // This happens when dict is assigned to serde_json::Value (e.g., current[k] = {})
        // DEPYLER-1015: Skip json!({}) in NASA mode
        if items.is_empty() {
            if self.ctx.in_json_context && !nasa_mode {
                // Use json!({}) for serde_json::Value compatibility (non-NASA mode only)
                self.ctx.needs_serde_json = true;
                Ok(parse_quote! { serde_json::json!({}) })
            } else if nasa_mode {
                // DEPYLER-1029: Check for type annotation to get proper HashMap types
                // If we have `d: Dict[int, str] = {}`, use HashMap<i32, String>
                // Otherwise default to HashMap<String, String>
                if let Some(Type::Dict(ref key_type, ref val_type)) = self.ctx.current_assign_type {
                    // DEPYLER-1073: Use DepylerValue for float keys (f64 doesn't implement Hash/Eq)
                    let key_tokens = if matches!(key_type.as_ref(), Type::Float) {
                        quote! { DepylerValue }
                    } else {
                        type_to_rust_type(key_type, self.ctx.type_mapper)
                    };
                    let val_tokens = type_to_rust_type(val_type, self.ctx.type_mapper);

                    return Ok(parse_quote! {
                        {
                            let map: HashMap<#key_tokens, #val_tokens> = HashMap::new();
                            map
                        }
                    });
                }

                // Default: HashMap<String, String> when no type annotation
                Ok(parse_quote! {
                    {
                        let map: HashMap<String, String> = HashMap::new();
                        map
                    }
                })
            } else {
                // Regular HashMap for normal dicts
                Ok(parse_quote! {
                    {
                        let map = HashMap::new();
                        map
                    }
                })
            }
        } else {
            // DEPYLER-1159: Add type annotations when target/return type is known
            // to help Rust's type inference in complex expressions
            let key_type = self
                .ctx
                .current_assign_type
                .as_ref()
                .or(self.ctx.current_return_type.as_ref())
                .and_then(|t| if let Type::Dict(k, _) = t { Some(k.as_ref()) } else { None });
            let val_type = self
                .ctx
                .current_assign_type
                .as_ref()
                .or(self.ctx.current_return_type.as_ref())
                .and_then(|t| if let Type::Dict(_, v) = t { Some(v.as_ref()) } else { None });

            if let (Some(k), Some(v)) = (key_type, val_type) {
                let key_tokens = type_to_rust_type(k, self.ctx.type_mapper);
                let val_tokens = type_to_rust_type(v, self.ctx.type_mapper);
                Ok(parse_quote! {
                    {
                        let mut map: HashMap<#key_tokens, #val_tokens> = HashMap::new();
                        #(#insert_stmts)*
                        map
                    }
                })
            } else {
                // No type information available, let Rust infer
                Ok(parse_quote! {
                    {
                        let mut map = HashMap::new();
                        #(#insert_stmts)*
                        map
                    }
                })
            }
        }
    }

    // DEPYLER-COVERAGE-95: return_type_is_dict_list_union moved to stdlib_method_gen::json

    /// DEPYLER-0560: Check if function return type requires serde_json::Value for dicts
    /// DEPYLER-0727: Also check assignment target type for inline dict literals
    ///
    /// Returns true if current function returns Dict[str, Any] or Dict[str, Unknown],
    /// OR if assigning to a variable with Dict[str, Any] type annotation,
    /// which maps to HashMap<String, serde_json::Value>. In these cases, dict literals
    /// should use json!() to ensure type compatibility.
    pub(crate) fn return_type_needs_json_dict(&self) -> bool {
        // DEPYLER-0727: Check assignment target type first (e.g., d: Dict[str, Any] = {...})
        if let Some(ref assign_type) = self.ctx.current_assign_type {
            match assign_type {
                Type::Dict(_, value_type) => {
                    if Self::is_json_value_type(value_type.as_ref()) {
                        return true;
                    }
                }
                Type::Custom(s) if s.contains("HashMap") && s.contains("Value") => return true,
                _ => {}
            }
        }

        // Check function return type
        if let Some(ref ret_type) = self.ctx.current_return_type {
            // Check if return type is Dict with Any/Unknown value type
            match ret_type {
                Type::Dict(_, value_type) => Self::is_json_value_type(value_type.as_ref()),
                // Custom type might be Result<Dict<K, V>, E> - check inner type
                Type::Custom(s) if s.contains("HashMap") && s.contains("Value") => true,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Helper: Check if a type should use serde_json::Value
    /// DEPYLER-0726: Also check for Type::Custom("Any") after DEPYLER-0725 fix
    /// DEPYLER-0773: Also check for "object" which is Python's top-level type
    pub(crate) fn is_json_value_type(ty: &Type) -> bool {
        matches!(ty, Type::Unknown)
            || matches!(ty, Type::Custom(s) if s == "serde_json::Value" || s == "Value" || s == "Any" || s == "object")
    }

    /// DEPYLER-0376: Check if dict has heterogeneous value types
    /// DEPYLER-0270 FIX: Only flag as heterogeneous when we have strong evidence
    /// DEPYLER-0461: Also detect nested dicts which require serde_json::Value
    pub(crate) fn dict_has_mixed_types(&self, items: &[(HirExpr, HirExpr)]) -> Result<bool> {
        // DEPYLER-1166: Check for nested dicts FIRST (before item count check)
        // A single-item dict like {"key": {...}} should still trigger DepylerValue
        // wrapping because the nested dict needs to be wrapped as DepylerValue::Dict
        if self.dict_contains_nested_dict(items) {
            return Ok(true);
        }

        if items.len() <= 1 {
            return Ok(false); // Single type or empty
        }

        // STRATEGY 1: Check for obvious mixing of literal types
        let mut has_bool_literal = false;
        let mut has_int_literal = false;
        let mut has_float_literal = false;
        let mut has_string_literal = false;
        // DEPYLER-1031: Track complex expressions (function calls, method calls, etc.)
        // These are treated as "unknown type" and trigger mixed type handling
        let mut has_complex_expr = false;
        // DEPYLER-1040: Track unknown-type attribute accesses
        // If we have multiple, assume they're potentially different types
        let mut unknown_attribute_count = 0;
        // DEPYLER-0601: Also track list element types for heterogeneous detection
        let mut has_list_of_int = false;
        let mut has_list_of_string = false;
        let mut has_list_of_bool = false;
        let mut has_list_of_float = false;

        for (_key, value) in items {
            match value {
                HirExpr::Literal(Literal::Bool(_)) => has_bool_literal = true,
                HirExpr::Literal(Literal::Int(_)) => has_int_literal = true,
                HirExpr::Literal(Literal::Float(_)) => has_float_literal = true,
                HirExpr::Literal(Literal::String(_)) => has_string_literal = true,
                // DEPYLER-1031: Function calls and method calls are complex expressions
                HirExpr::Call { .. } | HirExpr::MethodCall { .. } => has_complex_expr = true,
                // DEPYLER-1023: Check variable types from ctx.var_types for heterogeneous detection
                HirExpr::Var(name) => {
                    if let Some(var_type) = self.ctx.var_types.get(name) {
                        match var_type {
                            Type::Bool => has_bool_literal = true,
                            Type::Int => has_int_literal = true,
                            Type::Float => has_float_literal = true,
                            Type::String => has_string_literal = true,
                            _ => {}
                        }
                    }
                }
                // DEPYLER-1040: Handle struct field access (e.g., args.debug, args.count)
                // Look up field type from class_field_types or argparse_tracker
                // DEPYLER-1143: Also check argparse field types for heterogeneous dict detection
                HirExpr::Attribute { value, attr, .. } => {
                    // First try class_field_types
                    let mut found_type: Option<&Type> = self.ctx.class_field_types.get(attr);
                    // Track inferred type string from argparse rust_type()
                    let mut inferred_type_str: Option<String> = None;

                    // DEPYLER-1143: If not found, check if this is an argparse args.field access
                    if found_type.is_none() {
                        if let HirExpr::Var(obj_name) = value.as_ref() {
                            // Check if obj_name is an argparse args_var
                            for parser_info in self.ctx.argparser_tracker.parsers.values() {
                                if let Some(ref args_var) = parser_info.args_var {
                                    if args_var == obj_name {
                                        // Found matching parser, look up field type
                                        for arg in &parser_info.arguments {
                                            if arg.rust_field_name() == *attr {
                                                // Try explicit arg_type first
                                                found_type = arg.arg_type.as_ref();
                                                // If not set, use rust_type() which handles
                                                // action="store_true" → bool, nargs="+" → Vec, etc.
                                                if found_type.is_none() {
                                                    inferred_type_str = Some(arg.rust_type());
                                                }
                                                break;
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    // Process found explicit Type
                    if let Some(field_type) = found_type {
                        match field_type {
                            Type::Bool => has_bool_literal = true,
                            Type::Int => has_int_literal = true,
                            Type::Float => has_float_literal = true,
                            Type::String => has_string_literal = true,
                            Type::List(_) => has_complex_expr = true,
                            Type::Optional(_) => has_complex_expr = true,
                            _ => has_complex_expr = true,
                        }
                    } else if let Some(type_str) = inferred_type_str {
                        // DEPYLER-1143: Handle inferred types from argparse rust_type()
                        match type_str.as_str() {
                            "bool" => has_bool_literal = true,
                            "i32" | "i64" | "isize" | "u8" | "u32" | "u64" => {
                                has_int_literal = true
                            }
                            "f32" | "f64" => has_float_literal = true,
                            "String" => has_string_literal = true,
                            s if s.starts_with("Vec<") => has_complex_expr = true,
                            s if s.starts_with("Option<") => has_complex_expr = true,
                            _ => has_complex_expr = true,
                        }
                    } else {
                        // DEPYLER-1040: Without explicit type info, count as unknown
                        // Multiple unknown attributes likely have different types
                        unknown_attribute_count += 1;
                    }
                }
                // DEPYLER-0601: Check list element types for heterogeneous detection
                HirExpr::List(elems) if !elems.is_empty() => {
                    // Determine list element type from first element
                    match &elems[0] {
                        HirExpr::Literal(Literal::Int(_)) => has_list_of_int = true,
                        HirExpr::Literal(Literal::String(_)) => has_list_of_string = true,
                        HirExpr::Literal(Literal::Bool(_)) => has_list_of_bool = true,
                        HirExpr::Literal(Literal::Float(_)) => has_list_of_float = true,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Count how many distinct literal types we have
        // DEPYLER-1031: Include complex expressions in the count
        let distinct_literal_types = [
            has_bool_literal,
            has_int_literal,
            has_float_literal,
            has_string_literal,
            has_complex_expr, // Treat complex expressions as a distinct type
        ]
        .iter()
        .filter(|&&b| b)
        .count();

        // DEPYLER-0601: Count how many distinct list element types we have
        let distinct_list_types =
            [has_list_of_int, has_list_of_string, has_list_of_bool, has_list_of_float]
                .iter()
                .filter(|&&b| b)
                .count();

        // Use DepylerValue/json! if we have 2+ distinct literal types OR 2+ distinct list types
        // This handles both {"a": 1, "b": "str"} and {"items": [1,2], "tags": ["a"]}
        // DEPYLER-1040: Also trigger for multiple unknown-type attributes (likely different types)
        Ok(distinct_literal_types >= 2 || distinct_list_types >= 2 || unknown_attribute_count >= 2)
    }

    /// DEPYLER-0461: Recursively check if dict contains any nested dicts
    /// Returns true if any value is a Dict. When this is true, ALL nested dicts
    /// in the tree must use json!() for consistency (json!() doesn't accept HashMap blocks)
    pub(crate) fn dict_contains_nested_dict(&self, items: &[(HirExpr, HirExpr)]) -> bool {
        for (_key, value) in items {
            if self.expr_is_or_contains_dict(value) {
                return true;
            }
        }
        false
    }

    /// DEPYLER-0461: Check if expression is a dict or recursively contains a dict
    pub(crate) fn expr_is_or_contains_dict(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Dict(_) => true,
            HirExpr::List(items) => items.iter().any(|e| self.expr_is_or_contains_dict(e)),
            HirExpr::Tuple(items) => items.iter().any(|e| self.expr_is_or_contains_dict(e)),
            _ => false,
        }
    }
}
