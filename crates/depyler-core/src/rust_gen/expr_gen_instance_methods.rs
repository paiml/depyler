//! Instance method handlers for ExpressionConverter
//!
//! DEPYLER-COVERAGE-95: Extracted from expr_gen.rs to reduce file size
//! and improve testability. Contains collection and instance method handlers.

use crate::direct_rules::type_to_rust_type;
use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::rust_gen::keywords;
use crate::rust_gen::truthiness_helpers::{
    is_collection_generic_base, is_collection_type_name, is_collection_var_name,
    is_option_var_name, is_string_var_name,
};
use crate::rust_gen::walrus_helpers;
use crate::trace_decision;
#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    // ========================================================================
    // DEPYLER-0142 Phase 2: Category Handlers
    // ========================================================================

    /// Handle list methods (append, extend, pop, insert, remove, sort)
    #[inline]
    pub(crate) fn convert_list_method(
        &mut self,
        object_expr: &syn::Expr,
        object: &HirExpr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
        match method {
            "append" => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];

                // DEPYLER-1134: Constraint-Aware Coercion
                // Check for CONCRETE element type FIRST (from Oracle or type annotations)
                // Only fall back to DepylerValue wrapping if type is truly Unknown
                let concrete_element_type = if let HirExpr::Attribute { value: _, attr } = object {
                    self.ctx.class_field_types.get(attr).and_then(|t| {
                        if let Type::List(elem) = t {
                            if !matches!(elem.as_ref(), Type::Unknown) {
                                Some(elem.as_ref().clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                } else if let HirExpr::Var(var_name) = object {
                    self.ctx.var_types.get(var_name).and_then(|t| {
                        if let Type::List(elem) = t {
                            if !matches!(elem.as_ref(), Type::Unknown) {
                                Some(elem.as_ref().clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                } else {
                    None
                };

                // DEPYLER-1134: If we have a concrete element type, generate type-aware push
                // This is the "bridge" that respects Oracle/annotation types
                if let Some(elem_type) = concrete_element_type {
                    // Generate the appropriate push based on element type
                    let push_expr = self.generate_typed_push(object_expr, arg, &elem_type, hir_args)?;
                    return Ok(push_expr);
                }

                // DEPYLER-1051: Check if target is Vec<DepylerValue> (e.g., untyped class field)
                // If so, wrap the argument in appropriate DepylerValue variant
                let is_vec_depyler_value = if let HirExpr::Attribute { value: _, attr } = object {
                    // Check class field type
                    self.ctx
                        .class_field_types
                        .get(attr)
                        .map(|t| matches!(t, Type::List(elem) if matches!(elem.as_ref(), Type::Unknown)))
                        .unwrap_or(false)
                } else if let HirExpr::Var(var_name) = object {
                    matches!(
                        self.ctx.var_types.get(var_name),
                        Some(Type::List(elem)) if matches!(elem.as_ref(), Type::Unknown)
                    )
                } else {
                    false
                };

                if is_vec_depyler_value {
                    // DEPYLER-1051: Wrap argument in DepylerValue based on argument type
                    let wrapped_arg: syn::Expr = if !hir_args.is_empty() {
                        match &hir_args[0] {
                            HirExpr::Literal(Literal::Int(_)) => parse_quote! { DepylerValue::Int(#arg as i64) },
                            HirExpr::Literal(Literal::Float(_)) => parse_quote! { DepylerValue::Float(#arg as f64) },
                            HirExpr::Literal(Literal::String(_)) => parse_quote! { DepylerValue::Str(#arg.to_string()) },
                            HirExpr::Literal(Literal::Bool(_)) => parse_quote! { DepylerValue::Bool(#arg) },
                            HirExpr::Var(name) => {
                                // Check variable type
                                match self.ctx.var_types.get(name) {
                                    Some(Type::Int) => parse_quote! { DepylerValue::Int(#arg as i64) },
                                    Some(Type::Float) => parse_quote! { DepylerValue::Float(#arg as f64) },
                                    Some(Type::String) => parse_quote! { DepylerValue::Str(#arg.to_string()) },
                                    Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#arg) },
                                    _ => parse_quote! { DepylerValue::Str(format!("{:?}", #arg)) },
                                }
                            }
                            _ => parse_quote! { DepylerValue::Str(format!("{:?}", #arg)) },
                        }
                    } else {
                        parse_quote! { DepylerValue::Str(format!("{:?}", #arg)) }
                    };
                    self.ctx.needs_depyler_value_enum = true;
                    return Ok(parse_quote! { #object_expr.push(#wrapped_arg) });
                }

                // DEPYLER-0422 Fix #7: Convert &str literals to String when pushing to Vec<String>
                // Five-Whys Root Cause:
                // 1. Why: expected String, found &str
                // 2. Why: String literal "X" is &str, but Vec<String>.push() needs String
                // 3. Why: Transpiler generates "X" without .to_string()
                // 4. Why: append method doesn't check element type
                // 5. ROOT CAUSE: Missing .to_string() for literals in Vec<String>
                let needs_to_string = if !hir_args.is_empty() {
                    // Check if argument is a string literal
                    let is_str_literal =
                        matches!(&hir_args[0], HirExpr::Literal(Literal::String(_)));

                    // Check if object is a Vec<String> by examining variable type
                    let is_vec_string = if let HirExpr::Var(var_name) = object {
                        matches!(
                            self.ctx.var_types.get(var_name),
                            Some(Type::List(element_type)) if matches!(**element_type, Type::String)
                        )
                    } else {
                        false
                    };

                    is_str_literal && is_vec_string
                } else {
                    false
                };

                if needs_to_string {
                    Ok(parse_quote! { #object_expr.push(#arg.to_string()) })
                } else {
                    Ok(parse_quote! { #object_expr.push(#arg) })
                }
            }
            "extend" => {
                // DEPYLER-0292: Handle iterator conversion for extend()
                if arg_exprs.len() != 1 {
                    bail!("extend() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // extend() expects IntoIterator<Item = T>, but we often pass &Vec<T>
                // which gives IntoIterator<Item = &T>. Add .iter().cloned() to fix this.
                // Check if arg is a reference (most common case for function parameters)
                let arg_string = quote! { #arg }.to_string();
                if arg_string.contains("&") || !arg_string.contains(".into_iter()") {
                    // Likely a reference or direct variable - add iterator conversion
                    Ok(parse_quote! { #object_expr.extend(#arg.iter().cloned()) })
                } else {
                    // Already an iterator or owned value
                    Ok(parse_quote! { #object_expr.extend(#arg) })
                }
            }
            "pop" => {
                // DEPYLER-0210 FIX: Handle pop() for sets, dicts, and lists
                // Disambiguate based on argument count FIRST, then object type

                if arg_exprs.len() == 2 {
                    // Only dict.pop(key, default) takes 2 arguments
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    // DEPYLER-0303: Don't add & for string literals or variables
                    let needs_ref = !hir_args.is_empty()
                        && !matches!(
                            hir_args[0],
                            HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
                        );
                    if needs_ref {
                        Ok(parse_quote! { #object_expr.remove(&#key).unwrap_or(#default) })
                    } else {
                        Ok(parse_quote! { #object_expr.remove(#key).unwrap_or(#default) })
                    }
                } else if arg_exprs.len() > 2 {
                    bail!("pop() takes at most 2 arguments");
                } else if self.is_set_expr(object) {
                    // Set.pop() - must have 0 arguments
                    if !arg_exprs.is_empty() {
                        bail!("pop() takes no arguments for sets");
                    }
                    Ok(parse_quote! {
                        #object_expr.iter().next().cloned().map(|x| {
                            #object_expr.remove(&x);
                            x
                        }).expect("pop from empty set")
                    })
                } else if self.is_dict_expr(object) {
                    // Dict literal - pop(key) with 1 argument
                    if arg_exprs.len() != 1 {
                        bail!("dict literal pop() requires exactly 1 argument (key)");
                    }
                    let key = &arg_exprs[0];
                    // DEPYLER-0303: Don't add & for string literals or variables
                    let needs_ref = !hir_args.is_empty()
                        && !matches!(
                            hir_args[0],
                            HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
                        );
                    if needs_ref {
                        Ok(
                            parse_quote! { #object_expr.remove(&#key).expect("KeyError: key not found") },
                        )
                    } else {
                        Ok(
                            parse_quote! { #object_expr.remove(#key).expect("KeyError: key not found") },
                        )
                    }
                } else if arg_exprs.is_empty() {
                    // List.pop() with no arguments - remove last element
                    Ok(parse_quote! { #object_expr.pop().unwrap_or_default() })
                } else {
                    // 1 argument: could be list.pop(index) OR dict.pop(key)
                    // Use multiple heuristics to disambiguate:
                    let arg = &arg_exprs[0];

                    // Heuristic 1: Explicit list literal
                    let is_list = self.is_list_expr(object);

                    // Heuristic 2: Explicit dict literal
                    let is_dict = self.is_dict_expr(object);

                    // Heuristic 3: Integer argument suggests list index
                    let arg_is_int = !hir_args.is_empty()
                        && matches!(hir_args[0], HirExpr::Literal(crate::hir::Literal::Int(_)));

                    if is_list || (!is_dict && arg_is_int) {
                        // List.pop(index) - use Vec::remove() which takes usize by value
                        Ok(parse_quote! { #object_expr.remove(#arg as usize) })
                    } else {
                        // dict.pop(key) - HashMap::remove() takes &K by reference
                        // DEPYLER-0303: Don't add & for string literals or variables
                        let needs_ref = !hir_args.is_empty()
                            && !matches!(
                                hir_args[0],
                                HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
                            );
                        if needs_ref {
                            Ok(
                                parse_quote! { #object_expr.remove(&#arg).expect("KeyError: key not found") },
                            )
                        } else {
                            Ok(
                                parse_quote! { #object_expr.remove(#arg).expect("KeyError: key not found") },
                            )
                        }
                    }
                }
            }
            "insert" => {
                if arg_exprs.len() != 2 {
                    bail!("insert() requires exactly two arguments");
                }
                let index = &arg_exprs[0];
                let value = &arg_exprs[1];
                Ok(parse_quote! { #object_expr.insert(#index as usize, #value) })
            }
            "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                if self.is_set_expr(object) {
                    Ok(parse_quote! {
                        if !#object_expr.remove(&#value) {
                            panic!("KeyError: element not in set");
                        }
                    })
                } else {
                    Ok(parse_quote! {
                        if let Some(pos) = #object_expr.iter().position(|x| x == &#value) {
                            #object_expr.remove(pos)
                        } else {
                            panic!("ValueError: list.remove(x): x not in list")
                        }
                    })
                }
            }
            "index" => {
                // Python: list.index(value) -> returns index of first occurrence
                // Rust: list.iter().position(|x| x == &value).ok_or(...)
                if arg_exprs.len() != 1 {
                    bail!("index() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.iter()
                        .position(|x| x == &#value)
                        .map(|i| i as i32)
                        .expect("ValueError: value is not in list")
                })
            }
            "count" => {
                // Python: list.count(value) -> counts occurrences
                // Rust: list.iter().filter(|x| **x == value).count()
                if arg_exprs.len() != 1 {
                    bail!("count() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.iter().filter(|x| **x == #value).count() as i32
                })
            }
            "copy" => {
                // Python: list.copy() -> shallow copy OR copy.copy(x) -> shallow copy
                // Rust: list.clone() OR x.clone()
                // DEPYLER-0024 FIX: Handle copy.copy(x) from copy module
                if arg_exprs.len() == 1 {
                    // This is copy.copy(x) from the copy module being misparsed as method call
                    // Just clone the argument directly
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! { #arg.clone() });
                }
                if !arg_exprs.is_empty() {
                    bail!("copy() takes no arguments");
                }
                // This is list.copy() method - clone the list
                Ok(parse_quote! { #object_expr.clone() })
            }
            "clear" => {
                // Python: list.clear() -> removes all elements
                // Rust: list.clear()
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "reverse" => {
                // Python: list.reverse() -> reverses in place
                // Rust: list.reverse()
                if !arg_exprs.is_empty() {
                    bail!("reverse() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.reverse() })
            }
            "sort" => {
                // DEPYLER-0445: Python: list.sort(key=func, reverse=False)
                // Rust: list.sort_by_key(|x| func(x)) or list.sort()

                // Check for `key` kwarg
                let key_func = kwargs.iter().find(|(k, _)| k == "key").map(|(_, v)| v);
                let reverse = kwargs
                    .iter()
                    .find(|(k, _)| k == "reverse")
                    .and_then(|(_, v)| {
                        if let HirExpr::Literal(crate::hir::Literal::Bool(b)) = v {
                            Some(*b)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false);

                if !arg_exprs.is_empty() {
                    bail!("sort() does not accept positional arguments");
                }

                match (key_func, reverse) {
                    (Some(key_expr), false) => {
                        // list.sort(key=func) → list.sort_by_key(|x| func(x))
                        // Convert key_expr to Rust callable
                        let key_rust = key_expr.to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { #object_expr.sort_by_key(|x| #key_rust(x)) })
                    }
                    (Some(key_expr), true) => {
                        // list.sort(key=func, reverse=True) → list.sort_by_key(|x| std::cmp::Reverse(func(x)))
                        let key_rust = key_expr.to_rust_expr(self.ctx)?;
                        Ok(
                            parse_quote! { #object_expr.sort_by_key(|x| std::cmp::Reverse(#key_rust(x))) },
                        )
                    }
                    (None, false) => {
                        // list.sort() → list.sort()
                        Ok(parse_quote! { #object_expr.sort() })
                    }
                    (None, true) => {
                        // list.sort(reverse=True) → list.sort_by(|a, b| b.cmp(a))
                        Ok(parse_quote! { #object_expr.sort_by(|a, b| b.cmp(a)) })
                    }
                }
            }
            _ => bail!("Unknown list method: {}", method),
        }
    }

    /// DEPYLER-1134: Generate type-aware push for concrete element types
    /// This is the "bridge" that ensures the generator respects Oracle/annotation constraints
    fn generate_typed_push(
        &mut self,
        object_expr: &syn::Expr,
        arg: &syn::Expr,
        elem_type: &Type,
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        match elem_type {
            // For Vec<String>, ensure string conversion
            Type::String => {
                // Check if arg is already a String or needs conversion
                let is_str_literal = !hir_args.is_empty()
                    && matches!(&hir_args[0], HirExpr::Literal(Literal::String(_)));
                if is_str_literal {
                    Ok(parse_quote! { #object_expr.push(#arg.to_string()) })
                } else {
                    // Assume arg is already String type
                    Ok(parse_quote! { #object_expr.push(#arg) })
                }
            }
            // DEPYLER-1135: For Vec<int>, push directly without cast
            // The argument should already be typed correctly (i32 by default, or whatever
            // width was inferred). Casting to i64 causes E0308 when the Vec is Vec<i32>.
            // Trust the type system - if there's a mismatch, it's a type inference issue
            // that should be fixed at the source, not papered over with casts.
            Type::Int => {
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            // DEPYLER-1135: For Vec<f64>, push directly without cast
            // Same reasoning - trust the type system
            Type::Float => {
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            // For Vec<bool>, direct push
            Type::Bool => {
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            // For nested Vec<Vec<T>>, push directly (arg should already be Vec<T>)
            Type::List(_inner) => {
                // The arg should be the inner list type already
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            // For Vec<HashMap<K, V>>, push directly
            Type::Dict(_k, _v) => {
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            // For custom types, use try_into for safe conversion
            Type::Custom(type_name) => {
                let type_ident: syn::Type = syn::parse_str(type_name)?;
                Ok(parse_quote! { #object_expr.push(<#type_ident>::try_from(#arg).expect("Type conversion failed")) })
            }
            // For Optional types, push directly
            Type::Optional(_inner) => {
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            // For Tuple types, push directly
            Type::Tuple(_) => {
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            // Fallback: direct push (trust the type system)
            _ => {
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
        }
    }

    /// Handle dict methods (get, keys, values, items, update)
    /// DEPYLER-0540: Added hir_object param to detect serde_json::Value types
    #[inline]
    pub(crate) fn convert_dict_method(
        &mut self,
        object_expr: &syn::Expr,
        hir_object: &HirExpr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0540: Check if this is a serde_json::Value that needs special handling
        let is_json_value = self.is_serde_json_value(hir_object);

        match method {
            "get" => {
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    // DEPYLER-0330: Keep dict.get() as Option to support .is_none() checks
                    // Python: result = d.get(key); if result is None: ...
                    // Rust: let result = d.get(key).cloned(); if result.is_none() { ... }

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
                        if matches!(hir_args.get(1), Some(HirExpr::Literal(Literal::String(s))) if !s.is_empty()) {
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
                    } else if matches!(hir_args.get(1), Some(HirExpr::Literal(Literal::String(_)))) {
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
                            parse_quote! { #object_expr.as_object().unwrap().keys().cloned().collect::<Vec<_>>() },
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
                        parse_quote! { #object_expr.as_object().unwrap().values().cloned().collect::<Vec<_>>() },
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
                        parse_quote! { #object_expr.as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() },
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
    pub(crate) fn needs_value_to_string_conversion(&self, hir_object: &HirExpr) -> bool {
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
    pub(crate) fn check_dict_value_chain(&self, expr: &HirExpr) -> bool {
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
    pub(crate) fn rust_expr_needs_value_conversion(&self, expr: &syn::Expr) -> bool {
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

    /// Handle string methods (upper, lower, strip, startswith, endswith, split, join, replace, find, count, isdigit, isalpha)
    #[inline]
    pub(crate) fn convert_string_method(
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
                Ok(parse_quote! { #obj.to_uppercase() })
            }
            "lower" => {
                if !arg_exprs.is_empty() {
                    bail!("lower() takes no arguments");
                }
                Ok(parse_quote! { #obj.to_lowercase() })
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
                    HirExpr::Call { func, .. } if func == "map" || func == "filter" || func == "iter" || func == "enumerate" => true,
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
                // DEPYLER-0595: datetime.replace() uses kwargs, has 0-1 positional args
                // Use bare string literals without .to_string() for correct types
                if hir_args.len() < 2 {
                    // Not str.replace - could be datetime.replace() with kwargs
                    // Fall through to generic method call
                    return Ok(parse_quote! { #object_expr.replace() });
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
                    Ok(parse_quote! { #object_expr.trim_start_matches(|c: char| #chars.contains(c)).to_string() })
                }
            }
            "rstrip" => {
                // DEPYLER-0302/0595: str.rstrip([chars]) → .trim_end_matches
                if arg_exprs.is_empty() {
                    Ok(parse_quote! { #object_expr.trim_end().to_string() })
                } else {
                    let chars = &arg_exprs[0];
                    Ok(parse_quote! { #object_expr.trim_end_matches(|c: char| #chars.contains(c)).to_string() })
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
                let fillchar = if arg_exprs.len() == 2 {
                    &arg_exprs[1]
                } else {
                    &parse_quote!(" ")
                };

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
                let fillchar = if arg_exprs.len() == 2 {
                    &arg_exprs[1]
                } else {
                    &parse_quote!(" ")
                };

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
                let fillchar = if arg_exprs.len() == 2 {
                    &arg_exprs[1]
                } else {
                    &parse_quote!(" ")
                };

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
                        return Ok(parse_quote! { !#object_expr.is_control() || #object_expr == '\t' || #object_expr == '\n' || #object_expr == '\r' });
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
                        return Ok(parse_quote! { !#object_expr.is_alphabetic() || #object_expr.is_uppercase() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) })
            }
            "islower" => {
                if !arg_exprs.is_empty() {
                    bail!("islower() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { !#object_expr.is_alphabetic() || #object_expr.is_lowercase() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) })
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

    /// Handle set methods (add, discard, clear)
    #[inline]
    pub(crate) fn convert_set_method(
        &mut self,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        match method {
            "add" => {
                if arg_exprs.len() != 1 {
                    bail!("add() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.insert(#arg) })
            }
            "remove" => {
                // DEPYLER-0224: Set.remove(value) - remove value or panic if not found
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! {
                    if !#object_expr.remove(&#arg) {
                        panic!("KeyError: element not in set")
                    }
                })
            }
            "discard" => {
                if arg_exprs.len() != 1 {
                    bail!("discard() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.remove(&#arg) })
            }
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "update" => {
                // DEPYLER-0211 FIX: Set.update(other) - add all elements from other set
                if arg_exprs.len() != 1 {
                    bail!("update() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    for item in #other {
                        #object_expr.insert(item);
                    }
                })
            }
            "intersection_update" => {
                // DEPYLER-0212 FIX: Set.intersection_update(other) - keep only common elements
                // Note: This generates an expression that returns (), suitable for ExprStmt
                if arg_exprs.len() != 1 {
                    bail!("intersection_update() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    {
                        let temp: std::collections::HashSet<_> = #object_expr.intersection(&#other).cloned().collect();
                        #object_expr.clear();
                        #object_expr.extend(temp);
                    }
                })
            }
            "difference_update" => {
                // DEPYLER-0213 FIX: Set.difference_update(other) - remove elements in other
                // Note: This generates an expression that returns (), suitable for ExprStmt
                if arg_exprs.len() != 1 {
                    bail!("difference_update() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    {
                        let temp: std::collections::HashSet<_> = #object_expr.difference(&#other).cloned().collect();
                        #object_expr.clear();
                        #object_expr.extend(temp);
                    }
                })
            }
            "union" => {
                // Set.union(other) - return new set with elements from both sets
                if arg_exprs.len() != 1 {
                    bail!("union() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.union(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "intersection" => {
                // Set.intersection(other) - return new set with common elements
                if arg_exprs.len() != 1 {
                    bail!("intersection() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.intersection(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "difference" => {
                // Set.difference(other) - return new set with elements not in other
                if arg_exprs.len() != 1 {
                    bail!("difference() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.difference(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "symmetric_difference" => {
                // Set.symmetric_difference(other) - return new set with elements in either but not both
                if arg_exprs.len() != 1 {
                    bail!("symmetric_difference() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.symmetric_difference(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "issubset" => {
                // Set.issubset(other) - check if all elements are in other
                if arg_exprs.len() != 1 {
                    bail!("issubset() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.is_subset(&#other)
                })
            }
            "issuperset" => {
                // Set.issuperset(other) - check if contains all elements of other
                if arg_exprs.len() != 1 {
                    bail!("issuperset() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.is_superset(&#other)
                })
            }
            "isdisjoint" => {
                // Set.isdisjoint(other) - check if no common elements
                if arg_exprs.len() != 1 {
                    bail!("isdisjoint() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.is_disjoint(&#other)
                })
            }
            _ => bail!("Unknown set method: {}", method),
        }
    }

    /// Handle regex methods (findall)
    #[inline]
    /// DEPYLER-0431: Convert regex instance method calls
    /// Handles both compiled Regex methods and Match object methods
    pub(crate) fn convert_regex_method(
        &mut self,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        match method {
            // Compiled Regex methods
            "findall" => {
                if arg_exprs.is_empty() {
                    bail!("findall() requires at least one argument");
                }
                let text = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.find_iter(#text)
                        .map(|m| m.as_str().to_string())
                        .collect::<Vec<String>>()
                })
            }

            // DEPYLER-0431: compiled.match(text) → compiled.find(text)
            // Python re.match() only matches at start, but Rust .find() searches anywhere
            // NOTE: Full .groups() support requires proper regex type tracking (DEPYLER-0563)
            "match" => {
                if arg_exprs.is_empty() {
                    bail!("match() requires at least one argument");
                }
                let text = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.find(#text) })
            }

            // compiled.search(text) → compiled.find(text)
            "search" => {
                if arg_exprs.is_empty() {
                    bail!("search() requires at least one argument");
                }
                let text = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.find(#text) })
            }

            // DEPYLER-0519: Match object methods - handle Option<Match> from .find() results
            // Python's re.match/find returns None or Match, Rust's .find() returns Option<Match>
            // We need to unwrap before calling Match methods like .start(), .as_str()

            // match.group(0) → match.as_str() (for group 0)
            // match.group(n) → match.get(n).map(|m| m.as_str()) (for other groups)
            "group" => {
                if arg_exprs.is_empty() {
                    // No args: default to group 0
                    // DEPYLER-0519: Use map for Option safety
                    Ok(parse_quote! { #object_expr.as_ref().map(|m| m.as_str()).unwrap_or("") })
                } else {
                    // Check if group_num is literal 0
                    if matches!(arg_exprs[0], syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref lit), .. }) if lit.base10_parse::<i32>().ok() == Some(0))
                    {
                        Ok(parse_quote! { #object_expr.as_ref().map(|m| m.as_str()).unwrap_or("") })
                    } else {
                        // Non-zero group: needs captures API
                        bail!(
                            "match.group(n) for n>0 requires .captures() API (not yet implemented)"
                        )
                    }
                }
            }

            // match.groups() → extract all capture groups
            // DEPYLER-0442: Implement match.groups() using captured group extraction
            // Python: match.groups() returns tuple of all captured groups (excluding group 0)
            // NOTE: Full implementation requires regex type tracking (DEPYLER-0563)
            // For now, return empty vec - generator type system uses serde_json::Value as fallback
            "groups" => {
                // TODO: Implement proper capture group extraction when regex types are tracked
                Ok(parse_quote! {
                    Vec::<String>::new()
                })
            }

            // match.start() → match.start() (passthrough, with Option handling)
            "start" => {
                if arg_exprs.is_empty() {
                    // DEPYLER-0519: Handle Option<Match>
                    Ok(parse_quote! { #object_expr.as_ref().map(|m| m.start()).unwrap_or(0) })
                } else {
                    bail!("match.start(group) with group number not yet implemented")
                }
            }

            // match.end() → match.end() (passthrough, with Option handling)
            "end" => {
                if arg_exprs.is_empty() {
                    // DEPYLER-0519: Handle Option<Match>
                    Ok(parse_quote! { #object_expr.as_ref().map(|m| m.end()).unwrap_or(0) })
                } else {
                    bail!("match.end(group) with group number not yet implemented")
                }
            }

            // match.span() → (match.start(), match.end())
            "span" => {
                if arg_exprs.is_empty() {
                    // DEPYLER-0519: Handle Option<Match>
                    Ok(
                        parse_quote! { #object_expr.as_ref().map(|m| (m.start(), m.end())).unwrap_or((0, 0)) },
                    )
                } else {
                    bail!("match.span(group) with group number not yet implemented")
                }
            }

            // match.as_str() → match.as_str() (passthrough, with Option handling)
            "as_str" => {
                if !arg_exprs.is_empty() {
                    bail!("as_str() takes no arguments");
                }
                // DEPYLER-0519: Handle Option<Match>
                Ok(parse_quote! { #object_expr.as_ref().map(|m| m.as_str()).unwrap_or("") })
            }

            _ => bail!("Unknown regex method: {}", method),
        }
    }

    /// DEPYLER-0381: Convert sys I/O stream method calls
    /// sys.stdout.write(msg) → writeln!(std::io::stdout(), "{}", msg).unwrap()
    /// sys.stdin.read() → { let mut s = String::new(); std::io::stdin().read_to_string(&mut s).unwrap(); s }
    /// sys.stdout.flush() → std::io::stdout().flush().unwrap()
    #[inline]
    pub(crate) fn convert_sys_io_method(
        &self,
        stream: &str,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        let stream_fn = match stream {
            "stdin" => quote! { std::io::stdin() },
            "stdout" => quote! { std::io::stdout() },
            "stderr" => quote! { std::io::stderr() },
            _ => bail!("Unknown I/O stream: {}", stream),
        };

        let result = match (stream, method) {
            // stdout/stderr write methods
            ("stdout" | "stderr", "write") => {
                if arg_exprs.is_empty() {
                    bail!("{}.write() requires an argument", stream);
                }
                let msg = &arg_exprs[0];
                // Use writeln! macro for cleaner code and automatic newline handling
                // If the message already has \n, use write! instead
                parse_quote! {
                    {
                        use std::io::Write;
                        write!(#stream_fn, "{}", #msg).unwrap();
                    }
                }
            }

            // flush method
            (_, "flush") => {
                parse_quote! {
                    {
                        use std::io::Write;
                        #stream_fn.flush().unwrap()
                    }
                }
            }

            // stdin read methods
            ("stdin", "read") => {
                parse_quote! {
                    {
                        use std::io::Read;
                        let mut buffer = String::new();
                        #stream_fn.read_to_string(&mut buffer).unwrap();
                        buffer
                    }
                }
            }

            ("stdin", "readline") => {
                parse_quote! {
                    {
                        use std::io::BufRead;
                        let mut line = String::new();
                        #stream_fn.lock().read_line(&mut line).unwrap();
                        line
                    }
                }
            }

            // DEPYLER-0638: stdin.readlines() → collect all lines from stdin
            // Python: lines = sys.stdin.readlines()
            // Rust: std::io::stdin().lock().lines().collect::<Result<Vec<_>, _>>().unwrap()
            ("stdin", "readlines") => {
                parse_quote! {
                    {
                        use std::io::BufRead;
                        #stream_fn.lock().lines().collect::<Result<Vec<_>, _>>().unwrap()
                    }
                }
            }

            _ => bail!("{}.{}() is not yet supported", stream, method),
        };

        Ok(result)
    }

    /// Convert instance method calls (main dispatcher)
    #[inline]
    pub(crate) fn convert_instance_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {

        // DEPYLER-0363: Handle parse_args() → Skip for now, will be replaced with Args::parse()
        // ArgumentParser.parse_args() requires full struct transformation
        // For now, return unit to allow compilation
        if method == "parse_args" {
            // NOTE: Full argparse implementation requires Args::parse() call (tracked in DEPYLER-0363)
            return Ok(parse_quote! { () });
        }

        // DEPYLER-0363: Handle add_argument() → Skip for now, will be accumulated for struct generation
        if method == "add_argument" {
            // NOTE: Accumulate add_argument calls to generate struct fields (tracked in DEPYLER-0363)
            return Ok(parse_quote! { () });
        }

        // DEPYLER-0109: Handle parser.print_help() → Args::command().print_help()
        // Python: parser.print_help() prints help and continues
        // Rust/clap: Args::command().print_help()? with CommandFactory trait
        if method == "print_help" {
            // Generate clap help printing using CommandFactory
            return Ok(parse_quote! {
                {
                    use clap::CommandFactory;
                    Args::command().print_help().unwrap()
                }
            });
        }

        // DEPYLER-0381: Handle sys I/O stream method calls
        // Check if object is a sys I/O stream (sys.stdin(), sys.stdout(), sys.stderr())
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(module) = &**value {
                if module == "sys" && matches!(attr.as_str(), "stdin" | "stdout" | "stderr") {
                    return self.convert_sys_io_method(attr, method, arg_exprs);
                }
            }
        }

        // DEPYLER-0432: Handle file I/O .read() method
        // Python: f.read() → Rust: read_to_string() or read_to_end()
        if method == "read" && arg_exprs.is_empty() {
            // f.read() with no arguments → read entire file
            // Need to determine if text or binary mode
            // For now, default to text mode (read_to_string)
            // TODO: Track file open mode to distinguish text vs binary
            return Ok(parse_quote! {
                {
                    let mut content = String::new();
                    #object_expr.read_to_string(&mut content)?;
                    content
                }
            });
        }

        // DEPYLER-0558: Handle file I/O .read(size) method for chunked reading
        // Python: chunk = f.read(8192) → reads up to 8192 bytes, returns bytes (empty = EOF)
        // Rust: f.read(&mut buf) → reads into buffer, returns count (0 = EOF)
        if method == "read" && arg_exprs.len() == 1 {
            let size = &arg_exprs[0];
            return Ok(parse_quote! {
                {
                    let mut _read_buf = vec![0u8; #size];
                    let _n = #object_expr.read(&mut _read_buf).unwrap_or(0);
                    _read_buf.truncate(_n);
                    _read_buf
                }
            });
        }

        // DEPYLER-0305: Handle file I/O .readlines() method
        // Python: lines = f.readlines() → Rust: BufReader::new(f).lines().collect()
        if method == "readlines" && arg_exprs.is_empty() {
            self.ctx.needs_bufread = true;
            return Ok(parse_quote! {
                std::io::BufReader::new(#object_expr)
                    .lines()
                    .map(|l| l.unwrap_or_default())
                    .collect::<Vec<_>>()
            });
        }

        // DEPYLER-0305: Handle file I/O .readline() method
        // Python: line = f.readline() → Rust: read one line
        if method == "readline" && arg_exprs.is_empty() {
            self.ctx.needs_bufread = true;
            return Ok(parse_quote! {
                {
                    let mut _line = String::new();
                    std::io::BufReader::new(&mut #object_expr).read_line(&mut _line).unwrap_or(0);
                    _line
                }
            });
        }

        // DEPYLER-0458: Handle file I/O .write() method
        // DEPYLER-0537: Use .unwrap() instead of ? for functions without explicit error handling
        // DEPYLER-0536: Handle Option<String> arguments by unwrapping
        // Python: f.write(string) → Rust: f.write_all(bytes).unwrap()
        if method == "write" && arg_exprs.len() == 1 {
            // DEPYLER-0605: Set needs_io_write flag for Write trait import
            self.ctx.needs_io_write = true;
            let content = &arg_exprs[0];
            // Check if content might be an Option type based on HIR expression
            // If it's a variable that's known to be Option, unwrap it first
            // DEPYLER-0536: Detect Option type for write() content argument
            // Priority: type system > name heuristics (only use heuristics when no type info)
            // DEPYLER-0647: Check option_unwrap_map first - if already unwrapped, not Option
            // DEPYLER-0666: Also check if var_name is an UNWRAPPED name (value in map)
            let is_option_content = if let HirExpr::Var(var_name) = &hir_args[0] {
                // Check if variable is already unwrapped (inside if-let body)
                let is_unwrapped =
                    self.ctx.option_unwrap_map.contains_key(var_name)
                        || self.ctx.option_unwrap_map.values().any(|v| v == var_name);
                if is_unwrapped {
                    false // Already unwrapped, not Option
                } else {
                    match self.ctx.var_types.get(var_name) {
                        Some(Type::Optional(_)) => true,
                        Some(_) => false, // Known non-Option type - don't use name heuristic
                        None => {
                            // No type info - fall back to name heuristic
                            var_name == "content"
                                || var_name.ends_with("_content")
                                || var_name.ends_with("_text")
                        }
                    }
                }
            } else {
                false
            };

            // Convert string to bytes and use write_all()
            // Python's write() returns bytes written, but we simplify to just the operation
            // Use unwrap() since Python would raise exception on failure (matches behavior)
            if is_option_content {
                return Ok(parse_quote! {
                    #object_expr.write_all(#content.as_ref().unwrap().as_bytes()).unwrap()
                });
            } else {
                return Ok(parse_quote! {
                    #object_expr.write_all(#content.as_bytes()).unwrap()
                });
            }
        }

        // DEPYLER-0529: Handle file .close() method
        // Python: f.close() → Rust: no-op (files auto-close on drop via RAII)
        // DEPYLER-0550: Generate () instead of drop() because the file may have been
        // moved into a writer (e.g., csv::Writer::from_writer(output)), and we can't
        // drop a moved value. Rust's RAII handles cleanup automatically.
        if method == "close" && arg_exprs.is_empty() {
            // In Rust, files are automatically closed when dropped
            // No explicit close needed - RAII handles it
            return Ok(parse_quote! { () });
        }

        // DEPYLER-0551: Handle pathlib.Path instance methods
        // Python Path methods that need mapping to Rust std::path/std::fs equivalents
        // Check if object is a path variable (named "path" or known PathBuf type)
        let is_path_object = if let HirExpr::Var(var_name) = object {
            var_name == "path" || var_name.ends_with("_path") || var_name == "p"
        } else {
            false
        };

        if is_path_object {
            match method {
                // path.stat() → std::fs::metadata(&path).unwrap()
                "stat" if arg_exprs.is_empty() => {
                    return Ok(parse_quote! { std::fs::metadata(&#object_expr).unwrap() });
                }
                // path.absolute() or path.resolve() → path.canonicalize().unwrap()
                "absolute" | "resolve" if arg_exprs.is_empty() => {
                    return Ok(
                        parse_quote! { #object_expr.canonicalize().unwrap().to_string_lossy().to_string() },
                    );
                }
                _ => {} // Fall through to default handling
            }
        }

        // DEPYLER-0553: Handle datetime instance methods
        // Python datetime methods that need mapping to chrono equivalents
        // Check if object is likely a datetime variable
        // DEPYLER-0620: Expanded heuristics to catch common date variable names
        let is_datetime_object = if let HirExpr::Var(var_name) = object {
            var_name == "dt"
                || var_name == "d"  // DEPYLER-0620: Common date variable name
                || var_name == "t"  // DEPYLER-0620: Common time variable name
                || var_name == "datetime"
                || var_name == "date"  // DEPYLER-0620: Common date variable name
                || var_name == "time"  // DEPYLER-0620: Common time variable name
                || var_name.ends_with("_dt")
                || var_name.ends_with("_datetime")
                || var_name.ends_with("_date")
                || var_name.ends_with("_time")
                || var_name.starts_with("date_")  // DEPYLER-0620: date_xyz pattern
                || var_name.starts_with("time_")  // DEPYLER-0620: time_xyz pattern
        } else {
            // DEPYLER-0620: Also detect datetime methods being called regardless of variable name
            // If the method is datetime-specific (strftime, isoformat), assume datetime object
            matches!(method, "strftime" | "isoformat" | "timestamp" | "weekday" | "isoweekday")
        };

        if is_datetime_object {
            // DEPYLER-1025: In NASA mode, use std::time stubs instead of chrono
            let nasa_mode = self.ctx.type_mapper.nasa_mode;
            if !nasa_mode {
                self.ctx.needs_chrono = true;
            }
            match method {
                // dt.isoformat() → format for ISO string representation
                "isoformat" if arg_exprs.is_empty() => {
                    if nasa_mode {
                        return Ok(parse_quote! { format!("{:?}", #object_expr) });
                    } else {
                        return Ok(parse_quote! { #object_expr.to_string() });
                    }
                }
                // dt.strftime(fmt) → format string
                "strftime" if arg_exprs.len() == 1 => {
                    if nasa_mode {
                        return Ok(parse_quote! { format!("{:?}", #object_expr) });
                    } else {
                        // DEPYLER-0555: chrono's format() takes &str, not String
                        let fmt = match hir_args.first() {
                            Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
                            _ => arg_exprs[0].clone(),
                        };
                        return Ok(parse_quote! { #object_expr.format(#fmt).to_string() });
                    }
                }
                // dt.timestamp() → Unix timestamp
                "timestamp" if arg_exprs.is_empty() => {
                    if nasa_mode {
                        return Ok(parse_quote! { #object_expr.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as f64 });
                    } else {
                        return Ok(parse_quote! { #object_expr.and_utc().timestamp() as f64 });
                    }
                }
                // dt.date() → date component
                "date" if arg_exprs.is_empty() => {
                    if nasa_mode {
                        return Ok(parse_quote! { #object_expr });
                    } else {
                        return Ok(parse_quote! { #object_expr.date() });
                    }
                }
                // dt.time() → time component
                "time" if arg_exprs.is_empty() => {
                    if nasa_mode {
                        return Ok(parse_quote! { #object_expr });
                    } else {
                        return Ok(parse_quote! { #object_expr.time() });
                    }
                }
                _ => {} // Fall through to default handling
            }
        }

        // DEPYLER-0548: Handle csv.DictWriter methods
        // Python csv module methods need mapping to Rust csv crate equivalents
        if method == "writeheader" && arg_exprs.is_empty() {
            // writeheader() → no-op in Rust csv crate
            // Headers are typically written automatically or need explicit handling
            // TODO: Track fieldnames from DictWriter constructor to write proper header
            return Ok(parse_quote! { () });
        }

        if method == "writerow" && arg_exprs.len() == 1 {
            // writerow(row) → writer.serialize(&row).unwrap()
            // Python's DictWriter.writerow expects a dict
            // Rust's csv::Writer.serialize can handle HashMap
            let row = &arg_exprs[0];
            return Ok(parse_quote! {
                #object_expr.serialize(&#row).unwrap()
            });
        }

        // DEPYLER-0519: Handle regex Match.group() method
        // DEPYLER-0961: Return String instead of &str for type compatibility
        // DEPYLER-1070: Support DepylerRegexMatch in NASA mode
        // Python: match.group(0) or match.group(n)
        // NASA mode: DepylerRegexMatch.group(n) → String
        // Regex crate: match.as_str().to_string() for group(0), or handle numbered groups
        if method == "group" {
            let nasa_mode = self.ctx.type_mapper.nasa_mode;

            if nasa_mode {
                // DEPYLER-1070: DepylerRegexMatch has direct .group(n) method
                if arg_exprs.is_empty() || hir_args.is_empty() {
                    // match.group() → match.group(0)
                    return Ok(parse_quote! { #object_expr.group(0) });
                }
                let idx = &arg_exprs[0];
                return Ok(parse_quote! { #object_expr.group(#idx as usize) });
            }

            // Regex crate mode
            if arg_exprs.is_empty() || hir_args.is_empty() {
                // match.group() with no args defaults to group(0) in Python
                return Ok(parse_quote! { #object_expr.as_str().to_string() });
            }

            // Check if argument is literal 0
            if let HirExpr::Literal(Literal::Int(n)) = &hir_args[0] {
                if *n == 0 {
                    // match.group(0) → match.as_str().to_string()
                    return Ok(parse_quote! { #object_expr.as_str().to_string() });
                } else {
                    // match.group(n) → match.get(n).map(|m| m.as_str().to_string()).unwrap_or_default()
                    let idx = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.get(#idx).map(|m| m.as_str().to_string()).unwrap_or_default()
                    });
                }
            }

            // Non-literal argument - use runtime check
            let idx = &arg_exprs[0];
            return Ok(parse_quote! {
                if #idx == 0 {
                    #object_expr.as_str().to_string()
                } else {
                    #object_expr.get(#idx).map(|m| m.as_str().to_string()).unwrap_or_default()
                }
            });
        }

        // DEPYLER-0413: Handle string methods FIRST before class instance check
        // String methods like upper/lower should be converted even for method parameters
        // that might be typed as class instances (due to how we track types)
        // DEPYLER-0621: Added encode/decode to ensure bytes conversion works on any string
        if matches!(
            method,
            "upper"
                | "lower"
                | "strip"
                | "lstrip"
                | "rstrip"
                | "startswith"
                | "endswith"
                | "split"
                | "splitlines"
                | "join"
                | "replace"
                | "find"
                | "rfind"
                | "rindex"
                | "isdigit"
                | "isalpha"
                | "isalnum"
                | "title"
                | "center"
                | "ljust"
                | "rjust"
                | "zfill"
                | "hex"
                | "format"
                | "encode"  // DEPYLER-0621: str.encode() → .as_bytes().to_vec()
                | "decode"  // DEPYLER-0621: bytes.decode() → String::from_utf8_lossy()
        ) {
            // DEPYLER-1064: Check if object is a DepylerValue variable
            // If so, extract string before calling string method
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

            return self.convert_string_method(object, &adjusted_object_expr, method, arg_exprs, hir_args);
        }

        // DEPYLER-0232 FIX: Check for user-defined class instances
        // User-defined classes can have methods with names like "add" that conflict with
        // built-in collection methods. We must prioritize user-defined methods.
        if self.is_class_instance(object) {
            // This is a user-defined class instance - use generic method call
            // DEPYLER-0306 FIX: Use raw identifiers for method names that are Rust keywords
            let method_ident = if keywords::is_rust_keyword(method) {
                syn::Ident::new_raw(method, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(method, proc_macro2::Span::call_site())
            };

            // DEPYLER-0712: Auto-borrow class instance arguments when calling user-defined methods
            // When calling obj.method(other) where both are class instances,
            // the method signature likely expects &Self, so we borrow the argument.
            let processed_args: Vec<syn::Expr> = hir_args
                .iter()
                .zip(arg_exprs.iter())
                .map(|(hir_arg, arg_expr)| {
                    // If argument is also a class instance, borrow it
                    if self.is_class_instance(hir_arg) {
                        parse_quote! { &#arg_expr }
                    } else {
                        arg_expr.clone()
                    }
                })
                .collect();

            return Ok(parse_quote! { #object_expr.#method_ident(#(#processed_args),*) });
        }

        // DEPYLER-0211 FIX: Check object type first for ambiguous methods like update()
        // Both sets and dicts have update(), so we need to disambiguate

        // Check for set-specific context first
        if self.is_set_expr(object) {
            match method {
                "add"
                | "remove"
                | "discard"
                | "update"
                | "intersection_update"
                | "difference_update"
                | "union"
                | "intersection"
                | "difference"
                | "symmetric_difference"
                | "issubset"
                | "issuperset"
                | "isdisjoint" => {
                    return self.convert_set_method(object_expr, method, arg_exprs);
                }
                _ => {}
            }
        }

        // Check for dict-specific context
        if self.is_dict_expr(object) {
            match method {
                "get" | "keys" | "values" | "items" | "update" => {
                    // DEPYLER-0540: Pass object for serde_json::Value detection
                    return self.convert_dict_method(
                        object_expr,
                        object,
                        method,
                        arg_exprs,
                        hir_args,
                    );
                }
                _ => {}
            }
        }

        // Fallback to method name dispatch
        match method {
            // DEPYLER-0742: Deque-specific methods (must come before list methods)
            "appendleft" => {
                if arg_exprs.len() != 1 {
                    bail!("appendleft() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.push_front(#arg) })
            }
            "popleft" => {
                if !arg_exprs.is_empty() {
                    bail!("popleft() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.pop_front() })
            }

            // DEPYLER-0742: Handle append/pop for deque vs list
            "append" => {
                if self.is_deque_expr(object) {
                    if arg_exprs.len() != 1 {
                        bail!("append() requires exactly one argument");
                    }
                    let arg = &arg_exprs[0];
                    Ok(parse_quote! { #object_expr.push_back(#arg) })
                } else {
                    self.convert_list_method(object_expr, object, method, arg_exprs, hir_args, kwargs)
                }
            }
            "pop" => {
                if self.is_deque_expr(object) {
                    if !arg_exprs.is_empty() {
                        bail!("deque.pop() does not accept an index argument");
                    }
                    Ok(parse_quote! { #object_expr.pop_back().unwrap_or_default() })
                } else {
                    self.convert_list_method(object_expr, object, method, arg_exprs, hir_args, kwargs)
                }
            }

            // List methods (remaining)
            "extend" | "insert" | "remove" | "index" | "copy" | "clear"
            | "reverse" | "sort" => {
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
                    self.convert_set_method(object_expr, method, arg_exprs)
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
            | "split" | "rsplit" | "splitlines" | "join" | "replace" | "find" | "rfind" | "rindex"
            | "isdigit" | "isalpha" | "isalnum" | "isspace" | "isupper" | "islower" | "istitle"
            | "isnumeric" | "isascii" | "isdecimal" | "isidentifier" | "isprintable"
            | "title" | "capitalize" | "swapcase" | "casefold" | "center" | "ljust" | "rjust"
            | "zfill" | "hex" | "encode" | "decode" => {
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
            | "isdisjoint" => self.convert_set_method(object_expr, method, arg_exprs),

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
                Ok(parse_quote! { std::fs::read_to_string(#object_expr).unwrap() })
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

    /// DEPYLER-0188: Convert dynamic/subscript function call
    /// E.g., `handlers[name](args)` → `(handlers[&name])(args)`
    pub(crate) fn convert_dynamic_call(
        &mut self,
        callee: &HirExpr,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        let callee_expr = callee.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

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

        // DEPYLER-0964: Handle method calls on &mut Option<HashMap<K, V>> parameters
        // When a parameter is Dict[K,V] with default None, it becomes &mut Option<HashMap>
        // Method calls need to unwrap the Option first:
        // - memo.get(k) → memo.as_ref().unwrap().get(&k)
        // - memo.insert(k, v) → memo.as_mut().unwrap().insert(k, v)
        // - memo.contains_key(k) → memo.as_ref().unwrap().contains_key(&k)
        if let HirExpr::Var(var_name) = object {
            if self.ctx.mut_option_dict_params.contains(var_name) {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match method {
                    "get" => {
                        if args.is_empty() {
                            // dict.get() with no args - shouldn't happen for dict but handle gracefully
                            return Ok(parse_quote! { #var_ident.as_ref().unwrap().get() });
                        }
                        let key_expr = args[0].to_rust_expr(self.ctx)?;
                        // Check if we need default value (2-arg form)
                        if args.len() > 1 {
                            let default_expr = args[1].to_rust_expr(self.ctx)?;
                            return Ok(parse_quote! {
                                #var_ident.as_ref().unwrap().get(&#key_expr).cloned().unwrap_or(#default_expr)
                            });
                        } else {
                            // Single arg form - return Option<&V>
                            return Ok(parse_quote! {
                                #var_ident.as_ref().unwrap().get(&#key_expr).cloned()
                            });
                        }
                    }
                    "contains_key" | "__contains__" => {
                        if !args.is_empty() {
                            let key_expr = args[0].to_rust_expr(self.ctx)?;
                            return Ok(parse_quote! {
                                #var_ident.as_ref().unwrap().contains_key(&#key_expr)
                            });
                        }
                    }
                    "keys" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.as_ref().unwrap().keys() });
                    }
                    "values" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.as_ref().unwrap().values() });
                    }
                    "items" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.as_ref().unwrap().iter() });
                    }
                    "len" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.as_ref().unwrap().len() as i32 });
                    }
                    _ => {} // Fall through to other handlers
                }
            }
        }

        // DEPYLER-0108: Handle is_some/is_none on precomputed argparse Option fields
        // This prevents borrow-after-move when Option field is passed to a function then checked
        if (method == "is_some" || method == "is_none") && args.is_empty() {
            if let HirExpr::Attribute { value, attr } = object {
                if let HirExpr::Var(_) = value.as_ref() {
                    // Check if this field has been precomputed
                    if self.ctx.precomputed_option_fields.contains(attr) {
                        let has_var_name = format!("has_{}", attr);
                        let has_ident =
                            syn::Ident::new(&has_var_name, proc_macro2::Span::call_site());
                        if method == "is_some" {
                            return Ok(parse_quote! { #has_ident });
                        } else {
                            return Ok(parse_quote! { !#has_ident });
                        }
                    }
                }
            }
        }

        // DEPYLER-0931: Handle subprocess.Child methods (.wait(), .kill(), etc.)
        // proc.wait() → proc.as_mut().unwrap().wait().ok().and_then(|s| s.code()).unwrap_or(-1)
        // When proc is Option<Child>, we need to unwrap and extract exit code
        if method == "wait" && args.is_empty() {
            if let HirExpr::Var(var_name) = object {
                if let Some(var_type) = self.ctx.var_types.get(var_name) {
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
                    if is_subprocess_child {
                        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                        // Handle Option<Child> - unwrap, call wait, extract exit code
                        if matches!(var_type, Type::Optional(_)) {
                            return Ok(parse_quote! {
                                #var_ident.as_mut().unwrap().wait().ok().and_then(|s| s.code()).unwrap_or(-1)
                            });
                        } else {
                            return Ok(parse_quote! {
                                #var_ident.wait().ok().and_then(|s| s.code()).unwrap_or(-1)
                            });
                        }
                    }
                }
            }
        }

        // DEPYLER-0663: Handle serde_json::Value method calls
        // serde_json::Value doesn't have direct .len(), .iter(), .is_none(), .is_some() methods
        // We need to convert them to the appropriate serde_json::Value method chains
        // DEPYLER-0969: H₃ Error Cascade Prevention - comprehensive method coverage
        // This prevents E0599 cascades when Type::Unknown maps to serde_json::Value
        if self.is_serde_json_value_expr(object) || self.is_serde_json_value(object) {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            match method {
                // value.len() → value.as_array().map(|a| a.len()).unwrap_or_else(|| value.as_object().map(|o| o.len()).unwrap_or(0))
                "len" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array().map(|a| a.len()).unwrap_or_else(||
                            #object_expr.as_object().map(|o| o.len()).unwrap_or(0)
                        ) as i32
                    });
                }
                // value.iter() → value.as_array().into_iter().flatten()
                "iter" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array().into_iter().flatten()
                    });
                }
                // value.is_none() → value.is_null()
                "is_none" if args.is_empty() => {
                    return Ok(parse_quote! { #object_expr.is_null() });
                }
                // value.is_some() → !value.is_null()
                "is_some" if args.is_empty() => {
                    return Ok(parse_quote! { !#object_expr.is_null() });
                }
                // DEPYLER-0969: H₃ - List-like methods for serde_json::Value arrays
                // value.append(x) → value.as_array_mut().unwrap().push(x.into())
                "append" | "push" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| a.push(serde_json::json!(#arg)))
                    });
                }
                // value.pop() → value.as_array_mut().and_then(|a| a.pop())
                "pop" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().and_then(|a| a.pop()).unwrap_or(serde_json::Value::Null)
                    });
                }
                // value.pop_front/popleft() → value.as_array_mut().and_then(|a| if a.is_empty() { None } else { Some(a.remove(0)) })
                "pop_front" | "popleft" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().and_then(|a| if a.is_empty() { None } else { Some(a.remove(0)) }).unwrap_or(serde_json::Value::Null)
                    });
                }
                // value.push_back(x) → value.as_array_mut().map(|a| a.push(x.into()))
                "push_back" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| a.push(serde_json::json!(#arg)))
                    });
                }
                // value.push_front(x) → value.as_array_mut().map(|a| a.insert(0, x.into()))
                "push_front" | "appendleft" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| a.insert(0, serde_json::json!(#arg)))
                    });
                }
                // value.is_empty() → value.as_array().map(|a| a.is_empty()).unwrap_or(true)
                "is_empty" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array().map(|a| a.is_empty()).unwrap_or_else(||
                            #object_expr.as_object().map(|o| o.is_empty()).unwrap_or(true)
                        )
                    });
                }
                // DEPYLER-0969: H₃ - Dict-like methods for serde_json::Value objects
                // value.get(key) → value.get(key)
                "get" if !args.is_empty() => {
                    let key = &arg_exprs[0];
                    if args.len() > 1 {
                        let default = &arg_exprs[1];
                        return Ok(parse_quote! {
                            #object_expr.get(#key).cloned().unwrap_or(serde_json::json!(#default))
                        });
                    }
                    return Ok(parse_quote! { #object_expr.get(#key).cloned() });
                }
                // value.keys() → value.as_object().into_iter().flat_map(|o| o.keys())
                "keys" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_object().into_iter().flat_map(|o| o.keys().cloned()).collect::<Vec<_>>()
                    });
                }
                // value.values() → value.as_object().into_iter().flat_map(|o| o.values())
                "values" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_object().into_iter().flat_map(|o| o.values().cloned()).collect::<Vec<_>>()
                    });
                }
                // value.items() → value.as_object().into_iter().flat_map(|o| o.iter())
                "items" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_object().into_iter().flat_map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone()))).collect::<Vec<_>>()
                    });
                }
                // value.contains(x) → value.as_array().map(|a| a.contains(&x.into())).unwrap_or(false)
                "contains" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array().map(|a| a.iter().any(|v| v == &serde_json::json!(#arg))).unwrap_or(false)
                    });
                }
                // value.contains_key(k) → value.as_object().map(|o| o.contains_key(k)).unwrap_or(false)
                "contains_key" | "__contains__" if args.len() == 1 => {
                    let key = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_object().map(|o| o.contains_key(#key)).unwrap_or(false)
                    });
                }
                // value.insert(k, v) → value.as_object_mut().map(|o| o.insert(k, v.into()))
                "insert" if args.len() == 2 => {
                    let key = &arg_exprs[0];
                    let val = &arg_exprs[1];
                    return Ok(parse_quote! {
                        #object_expr.as_object_mut().map(|o| o.insert(#key.to_string(), serde_json::json!(#val)))
                    });
                }
                // value.remove(k) → value.as_object_mut().and_then(|o| o.remove(k))
                "remove" if args.len() == 1 => {
                    let key = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_object_mut().and_then(|o| o.remove(#key))
                    });
                }
                // value.clear() → value.as_array_mut().map(|a| a.clear())
                "clear" if args.is_empty() => {
                    return Ok(parse_quote! {
                        { if let Some(a) = #object_expr.as_array_mut() { a.clear() }
                          else if let Some(o) = #object_expr.as_object_mut() { o.clear() } }
                    });
                }
                // value.copy() / value.clone() → value.clone()
                "copy" | "clone" if args.is_empty() => {
                    return Ok(parse_quote! { #object_expr.clone() });
                }
                // value.extend(other) → merge JSON values
                "extend" if args.len() == 1 => {
                    let other = &arg_exprs[0];
                    return Ok(parse_quote! {
                        { if let (Some(a1), Some(a2)) = (#object_expr.as_array_mut(), #other.as_array()) {
                            a1.extend(a2.iter().cloned());
                        } else if let (Some(o1), Some(o2)) = (#object_expr.as_object_mut(), #other.as_object()) {
                            for (k, v) in o2 { o1.insert(k.clone(), v.clone()); }
                        } }
                    });
                }
                // value.add(x) → for sets, use array push
                "add" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| if !a.iter().any(|v| v == &serde_json::json!(#arg)) { a.push(serde_json::json!(#arg)) })
                    });
                }
                // value.discard(x) → for sets, remove if present
                "discard" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| a.retain(|v| v != &serde_json::json!(#arg)))
                    });
                }
                _ => {} // Fall through to other handlers
            }
        }

        // DEPYLER-0747: Handle asyncio module method calls
        // asyncio.sleep(secs) → tokio::time::sleep(Duration) or std::thread::sleep in NASA mode
        // asyncio.run(coro) → tokio runtime block_on or direct call in NASA mode
        if let HirExpr::Var(module) = object {
            if module == "asyncio" {
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
                            // DEPYLER-1024: Use std::thread::sleep in NASA mode
                            if let Some(arg) = arg_exprs.first() {
                                return Ok(parse_quote! {
                                    std::thread::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                                });
                            }
                            return Ok(parse_quote! {
                                std::thread::sleep(std::time::Duration::from_secs(0))
                            });
                        } else {
                            if let Some(arg) = arg_exprs.first() {
                                return Ok(parse_quote! {
                                    tokio::time::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                                });
                            }
                            return Ok(parse_quote! {
                                tokio::time::sleep(std::time::Duration::from_secs(0))
                            });
                        }
                    }
                    "run" => {
                        if nasa_mode {
                            // DEPYLER-1024: Just call the function directly in NASA mode
                            if let Some(arg) = arg_exprs.first() {
                                return Ok(parse_quote! { #arg });
                            }
                        } else if let Some(arg) = arg_exprs.first() {
                            return Ok(parse_quote! {
                                tokio::runtime::Runtime::new().unwrap().block_on(#arg)
                            });
                        }
                    }
                    _ => {} // Fall through for other asyncio methods
                }
            }
        }

        // DEPYLER-0912: Handle colorsys module method calls
        // colorsys.rgb_to_hsv(r, g, b) → inline HSV conversion
        // colorsys.hsv_to_rgb(h, s, v) → inline RGB conversion
        if let HirExpr::Var(module) = object {
            if module == "colorsys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "rgb_to_hsv" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        // Python colorsys.rgb_to_hsv formula
                        return Ok(parse_quote! {
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
                                    let h = if r == max_c {
                                        bc - gc
                                    } else if g == max_c {
                                        2.0 + rc - bc
                                    } else {
                                        4.0 + gc - rc
                                    };
                                    let h = (h / 6.0) % 1.0;
                                    let h = if h < 0.0 { h + 1.0 } else { h };
                                    (h, s, v)
                                }
                            }
                        });
                    }
                    "hsv_to_rgb" if arg_exprs.len() == 3 => {
                        let h = &arg_exprs[0];
                        let s = &arg_exprs[1];
                        let v = &arg_exprs[2];
                        // Python colorsys.hsv_to_rgb formula
                        return Ok(parse_quote! {
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
                                        0 => (v, t, p),
                                        1 => (q, v, p),
                                        2 => (p, v, t),
                                        3 => (p, q, v),
                                        4 => (t, p, v),
                                        _ => (v, p, q),
                                    }
                                }
                            }
                        });
                    }
                    "rgb_to_hls" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        // Python colorsys.rgb_to_hls formula
                        return Ok(parse_quote! {
                            {
                                let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                                let max_c = r.max(g).max(b);
                                let min_c = r.min(g).min(b);
                                let l = (min_c + max_c) / 2.0;
                                if min_c == max_c {
                                    (0.0, l, 0.0)
                                } else {
                                    let s = if l <= 0.5 {
                                        (max_c - min_c) / (max_c + min_c)
                                    } else {
                                        (max_c - min_c) / (2.0 - max_c - min_c)
                                    };
                                    let rc = (max_c - r) / (max_c - min_c);
                                    let gc = (max_c - g) / (max_c - min_c);
                                    let bc = (max_c - b) / (max_c - min_c);
                                    let h = if r == max_c {
                                        bc - gc
                                    } else if g == max_c {
                                        2.0 + rc - bc
                                    } else {
                                        4.0 + gc - rc
                                    };
                                    let h = (h / 6.0) % 1.0;
                                    let h = if h < 0.0 { h + 1.0 } else { h };
                                    (h, l, s)
                                }
                            }
                        });
                    }
                    "hls_to_rgb" if arg_exprs.len() == 3 => {
                        let h = &arg_exprs[0];
                        let l = &arg_exprs[1];
                        let s = &arg_exprs[2];
                        // Python colorsys.hls_to_rgb formula
                        return Ok(parse_quote! {
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
                        });
                    }
                    _ => {} // Fall through for other colorsys methods
                }
            }
        }

        // DEPYLER-0778: Handle dict.fromkeys(keys, default) class method
        // dict.fromkeys(keys, default) → keys.iter().map(|k| (k.clone(), default)).collect()
        if let HirExpr::Var(var_name) = object {
            if var_name == "dict" && method == "fromkeys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                if arg_exprs.len() >= 2 {
                    let keys_expr = &arg_exprs[0];
                    let default_expr = &arg_exprs[1];
                    return Ok(parse_quote! {
                        #keys_expr.iter().map(|k| (k.clone(), #default_expr)).collect()
                    });
                } else if arg_exprs.len() == 1 {
                    // dict.fromkeys(keys) with implicit None default
                    let keys_expr = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #keys_expr.iter().map(|k| (k.clone(), ())).collect()
                    });
                }
            }
        }

        // DEPYLER-0933: Handle int.from_bytes(bytes, byteorder) class method
        // int.from_bytes(bytes, "big") → i64::from_be_bytes(bytes.try_into().unwrap())
        // int.from_bytes(bytes, "little") → i64::from_le_bytes(bytes.try_into().unwrap())
        if let HirExpr::Var(var_name) = object {
            if var_name == "int" && method == "from_bytes" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                if arg_exprs.len() >= 2 {
                    let bytes_expr = &arg_exprs[0];
                    // Check if second arg is "big" or "little" string literal
                    let is_big_endian = if let HirExpr::Literal(Literal::String(s)) = &args[1] {
                        s == "big"
                    } else {
                        true // Default to big endian
                    };

                    if is_big_endian {
                        return Ok(parse_quote! {
                            i64::from_be_bytes({
                                let mut arr = [0u8; 8];
                                let bytes: &[u8] = #bytes_expr.as_ref();
                                let start = 8usize.saturating_sub(bytes.len());
                                arr[start..].copy_from_slice(bytes);
                                arr
                            })
                        });
                    } else {
                        return Ok(parse_quote! {
                            i64::from_le_bytes({
                                let mut arr = [0u8; 8];
                                let bytes: &[u8] = #bytes_expr.as_ref();
                                arr[..bytes.len().min(8)].copy_from_slice(&bytes[..bytes.len().min(8)]);
                                arr
                            })
                        });
                    }
                }
            }
        }

        // DEPYLER-0558: Handle hasher methods (hexdigest, update) for incremental hashing
        // DEPYLER-1002: Use finalize_reset() for Box<dyn DynDigest> compatibility
        if method == "hexdigest" {
            self.ctx.needs_hex = true;
            self.ctx.needs_digest = true;
            let object_expr = object.to_rust_expr(self.ctx)?;
            // hexdigest() on hasher → hex::encode(hasher.finalize_reset())
            // finalize_reset() works with Box<dyn DynDigest>
            return Ok(parse_quote! {
                {
                    use digest::DynDigest;
                    hex::encode(#object_expr.finalize_reset())
                }
            });
        }

        // DEPYLER-0750: Handle Counter.most_common(n)
        // counter.most_common(n) → sort HashMap by value descending, take n
        if method == "most_common" {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;

            if let Some(n_arg) = arg_exprs.first() {
                // With n argument: take top n
                return Ok(parse_quote! {
                    {
                        let mut entries: Vec<_> = #object_expr.iter().map(|(k, v)| (k.clone(), *v)).collect();
                        entries.sort_by(|a, b| b.1.cmp(&a.1));
                        entries.into_iter().take(#n_arg as usize).collect::<Vec<_>>()
                    }
                });
            } else {
                // No argument: return all sorted
                return Ok(parse_quote! {
                    {
                        let mut entries: Vec<_> = #object_expr.iter().map(|(k, v)| (k.clone(), *v)).collect();
                        entries.sort_by(|a, b| b.1.cmp(&a.1));
                        entries
                    }
                });
            }
        }

        // DEPYLER-0728: hasher.update() handler should NOT intercept dict/set.update()
        // Only apply to hash objects (Sha256, Md5, etc.), not collections
        if method == "update" && !args.is_empty() && !self.is_dict_expr(object) && !self.is_set_expr(object) {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            let data = &arg_exprs[0];
            // DEPYLER-0558: hasher.update(data) needs borrow for DynDigest trait
            // DynDigest::update takes &[u8], so always add borrow
            return Ok(parse_quote! {
                #object_expr.update(&#data)
            });
        }

        // DEPYLER-0413: Handle string methods FIRST before any other checks
        // This ensures string methods like upper/lower are converted even when
        // inside class methods where parameters might be mistyped as class instances
        // DEPYLER-1070: Skip if object is a known stdlib module (re, json, etc.) to allow
        // module method handling later (e.g., re.split() should be regex split, not str.split())
        let is_stdlib_module = if let HirExpr::Var(name) = object {
            matches!(
                name.as_str(),
                "re" | "json" | "math" | "random" | "os" | "sys" | "time" | "datetime"
                    | "pathlib" | "struct" | "statistics" | "fractions" | "decimal"
                    | "collections" | "itertools" | "functools" | "shutil" | "csv"
                    | "base64" | "hashlib" | "subprocess" | "string" | "tempfile"
            )
        } else {
            false
        };

        if !is_stdlib_module
            && matches!(
                method,
                "upper"
                    | "lower"
                    | "strip"
                    | "lstrip"
                    | "rstrip"
                    | "startswith"
                    | "endswith"
                    | "split"
                    | "splitlines"
                    | "join"
                    | "replace"
                    | "find"
                    | "rfind"
                    | "rindex"
                    | "isdigit"
                    | "isalpha"
                    | "isalnum"
                    | "title"
                    | "center"
                    | "ljust"
                    | "rjust"
                    | "zfill"
                    | "hex"
                    | "format"
            )
        {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return self.convert_string_method(object, &object_expr, method, &arg_exprs, args);
        }

        // DEPYLER-0829: Handle pathlib methods on Path variables (not just module calls)
        // Python: p = Path("/foo"); p.write_text(content)
        // Rust: PathBuf doesn't have write_text, must use std::fs::write
        // This catches path methods when object is a variable, not the pathlib module
        // DEPYLER-0956: Exclude "os" module - os.mkdir/os.rmdir are os module functions, not Path methods
        let is_os_module = matches!(object, HirExpr::Var(name) if name == "os");
        if !is_os_module
            && matches!(
                method,
                "write_text"
                    | "read_text"
                    | "read_bytes"
                    | "write_bytes"
                    | "exists"
                    | "is_file"
                    | "is_dir"
                    | "mkdir"
                    | "rmdir"
                    | "unlink"
                    | "iterdir"
                    | "glob"
                    | "rglob"
                    | "with_name"
                    | "with_suffix"
                    | "with_stem"
                    | "resolve"
                    | "absolute"
                    | "relative_to"
            )
        {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return self.convert_pathlib_instance_method(&object_expr, method, &arg_exprs);
        }

        // DEPYLER-0830: Handle datetime/timedelta instance methods on variables
        // Python: td = datetime.timedelta(seconds=100); td.total_seconds()
        // Rust: TimeDelta.num_seconds() as f64
        // This catches datetime methods when object is a variable, not the datetime module
        if matches!(
            method,
            "total_seconds"
                | "fromisoformat"
                | "isoformat"
                | "strftime"
                | "timestamp"
                | "timetuple"
                | "weekday"
                | "isoweekday"
                | "isocalendar"
                | "replace"
        ) {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return self.convert_datetime_instance_method(&object_expr, method, args, &arg_exprs);
        }

        // DEPYLER-0416: Check if this is a static method call on a class (e.g., Point.origin())
        // Convert to ClassName::method(args)
        // DEPYLER-0458 FIX: Exclude CONST_NAMES (all uppercase) from static method conversion
        // Constants like DEFAULT_CONFIG should use instance methods (.clone()) not static (::copy())
        if let HirExpr::Var(class_name) = object {
            let is_const = class_name.chars().all(|c| c.is_uppercase() || c == '_');
            let starts_uppercase = class_name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false);

            if starts_uppercase && !is_const {
                // DEPYLER-0900: Rename class if it shadows stdlib type (e.g., Box -> PyBox)
                // This is likely a static method call - convert to ClassName::method(args)
                let safe_name = crate::direct_rules::safe_class_name(class_name);
                let class_ident = syn::Ident::new(&safe_name, proc_macro2::Span::call_site());
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { #class_ident::#method_ident(#(#arg_exprs),*) });
            }
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

        // DEPYLER-0200: Handle .decode() on base64 encode calls
        // base64.b64encode() in Rust returns String, not bytes - no decode needed
        if method == "decode" {
            if let HirExpr::MethodCall {
                object: inner_obj,
                method: inner_method,
                ..
            } = object
            {
                if let HirExpr::Var(module) = inner_obj.as_ref() {
                    if module == "base64"
                        && (inner_method.contains("b64encode")
                            || inner_method.contains("urlsafe_b64encode"))
                    {
                        // base64::encode() returns String - just return the object expression
                        return object.to_rust_expr(self.ctx);
                    }
                }
            }
        }

        // DEPYLER-1115: Handle external module function calls with Rust path syntax
        // When calling module functions like requests.get(url), use requests::get(url)
        // This enables phantom binding generation to provide type-safe stubs
        if let HirExpr::Var(module_name) = object {
            // Check if this is an imported module (not a local variable)
            // Use all_imported_modules which includes external unmapped modules like requests
            if self.ctx.all_imported_modules.contains(module_name) {
                // Generate module::function() syntax instead of module.function()
                let module_ident = crate::rust_gen::keywords::safe_ident(module_name);
                let method_ident = crate::rust_gen::keywords::safe_ident(method);

                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                return Ok(parse_quote! { #module_ident::#method_ident(#(#arg_exprs),*) });
            }
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

        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0445: Pass original args and kwargs separately to convert_instance_method
        // Some methods like sort(key=func) need to preserve keyword argument names
        // For other methods, they can merge kwargs as positional if needed
        self.convert_instance_method(object, &object_expr, method, &arg_exprs, args, kwargs)
    }

    pub(crate) fn convert_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        // CITL: Trace subscript/indexing strategy decision
        trace_decision!(
            category = DecisionCategory::BorrowStrategy,
            name = "subscript_access",
            chosen = "get_or_index",
            alternatives = ["direct_index", "get_method", "get_unchecked", "slice"],
            confidence = 0.85
        );

        // DEPYLER-0386: Handle os.environ['VAR'] → std::env::var('VAR').unwrap_or_default()
        // Must check this before evaluating base_expr to avoid trying to convert os.environ
        if let HirExpr::Attribute { value, attr } = base {
            if let HirExpr::Var(module_name) = &**value {
                if module_name == "os" && attr == "environ" {
                    let index_expr = index.to_rust_expr(self.ctx)?;
                    return Ok(parse_quote! { std::env::var(#index_expr).unwrap_or_default() });
                }
            }
        }

        // DEPYLER-0964: Handle subscript access on &mut Option<HashMap<K, V>> parameters
        // When accessing `memo[key]` where memo is a mut_option_dict_param,
        // we need to unwrap the Option first: memo.as_ref().unwrap().get(&key).cloned()
        if let HirExpr::Var(var_name) = base {
            if self.ctx.mut_option_dict_params.contains(var_name) {
                let base_ident = crate::rust_gen::keywords::safe_ident(var_name);
                let index_expr = index.to_rust_expr(self.ctx)?;
                // Use .get() which returns Option<&V>, then .cloned() for owned value
                return Ok(parse_quote! {
                    #base_ident.as_ref().unwrap().get(&#index_expr).cloned().unwrap_or_default()
                });
            }
        }

        let mut base_expr = base.to_rust_expr(self.ctx)?;

        // DEPYLER-0270: Auto-unwrap Result-returning function calls
        // When base is a function call that returns Result<HashMap/Vec, E>,
        // we need to unwrap it with ? before calling .get() or indexing
        // Example: get_config()["name"] → get_config()?.get("name")...
        if let HirExpr::Call { func, .. } = base {
            if self.ctx.result_returning_functions.contains(func) {
                base_expr = parse_quote! { #base_expr? };
            }
        }

        // DEPYLER-1106: Use PyOps trait methods for DepylerValue indexing
        // This provides Python-semantic indexing with negative index support
        let base_is_depyler = self.expr_returns_depyler_value(base);
        if base_is_depyler && self.ctx.type_mapper.nasa_mode {
            let index_expr = index.to_rust_expr(self.ctx)?;
            // Use .py_index() for DepylerValue - handles negative indices and type coercion
            let index_for_pyops = if matches!(index, HirExpr::Literal(Literal::Int(_))) {
                parse_quote! { DepylerValue::Int(#index_expr as i64) }
            } else if self.expr_returns_depyler_value(index) {
                index_expr.clone()
            } else {
                parse_quote! { DepylerValue::Int(#index_expr as i64) }
            };
            return Ok(parse_quote! { #base_expr.clone().py_index(#index_for_pyops) });
        }

        // DEPYLER-0422 Fix #3 & #4: Handle tuple indexing with actual type information
        // Python: tuple[0], tuple[1] → Rust: tuple.0, tuple.1
        // Also handles chained indexing: list_of_tuples[i][j] → list_of_tuples.get(i).0
        let should_use_tuple_syntax = if let HirExpr::Literal(Literal::Int(idx)) = index {
            if *idx >= 0 {
                if let HirExpr::Var(var_name) = base {
                    // Case 1: Direct variable access (e.g., position[0] where position: Tuple)
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        matches!(var_type, Type::Tuple(_))
                    } else {
                        // Fallback heuristic: variable names suggesting tuple iteration
                        matches!(
                            var_name.as_str(),
                            "pair" | "entry" | "item" | "elem" | "tuple" | "row"
                        )
                    }
                } else if let HirExpr::Index {
                    base: inner_base, ..
                } = base
                {
                    // DEPYLER-0422 Fix #4: Case 2: Chained indexing (e.g., word_counts[j][1])
                    // Check if we're indexing into a List[Tuple]
                    if let HirExpr::Var(var_name) = &**inner_base {
                        if let Some(Type::List(element_type)) = self.ctx.var_types.get(var_name) {
                            // If the list contains tuples, second index is tuple field access
                            matches!(**element_type, Type::Tuple(_))
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if should_use_tuple_syntax {
            if let HirExpr::Literal(Literal::Int(idx)) = index {
                let field_idx = syn::Index::from(*idx as usize);
                return Ok(parse_quote! { #base_expr.#field_idx });
            }
        }

        // DEPYLER-0299 Pattern #3 FIX: Check if base is a String type for character access
        let is_string_base = self.is_string_base(base);

        // Discriminate between HashMap and Vec access based on base type or index type
        let is_string_key = self.is_string_index(base, index)?;

        if is_string_key {
            // HashMap/Dict access with string keys
            match index {
                HirExpr::Literal(Literal::String(s)) => {
                    // String literal - use it directly without .to_string()
                    Ok(parse_quote! {
                        #base_expr.get(#s).cloned().unwrap_or_default()
                    })
                }
                _ => {
                    // String variable - needs proper referencing
                    // HashMap.get() expects &K, so we need to borrow the key
                    // DEPYLER-0521: Don't add & if variable is already &str type
                    // DEPYLER-0528: Fixed logic - owned String NEEDS borrow, &str does NOT
                    let index_expr = index.to_rust_expr(self.ctx)?;
                    // DEPYLER-0539: Fix dict key borrowing for &str parameters
                    // Check is_borrowed_str_param FIRST - &str params are tracked as Type::String
                    // but should NOT be borrowed again
                    let needs_borrow = if let HirExpr::Var(var_name) = index {
                        if self.is_borrowed_str_param(var_name) {
                            false // Already &str from function parameter, no borrow needed
                        } else if matches!(
                            self.ctx.var_types.get(var_name),
                            Some(Type::String) // owned String → needs &
                        ) {
                            true // Owned String needs borrow
                        } else {
                            // Unknown type - default to borrowing for safety
                            true
                        }
                    } else {
                        true // Non-variable expressions typically need borrowing
                    };
                    if needs_borrow {
                        Ok(parse_quote! {
                            #base_expr.get(&#index_expr).cloned().unwrap_or_default()
                        })
                    } else {
                        Ok(parse_quote! {
                            #base_expr.get(#index_expr).cloned().unwrap_or_default()
                        })
                    }
                }
            }
        } else if is_string_base {
            // DEPYLER-0299 Pattern #3: String character access with numeric index
            // Strings cannot use .get(usize), must use .chars().nth()
            let index_expr = index.to_rust_expr(self.ctx)?;

            // DEPYLER-0267 FIX: Use .chars().nth() for proper character access
            // This returns Option<char>, then convert to String
            Ok(parse_quote! {
                {
                    // DEPYLER-0307 Fix #11: Use borrow to avoid moving the base expression
                    let base = &#base_expr;
                    let idx: i32 = #index_expr;
                    let actual_idx = if idx < 0 {
                        base.chars().count().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.chars().nth(actual_idx).map(|c| c.to_string()).unwrap_or_default()
                }
            })
        } else {
            // DEPYLER-0701: Check if base is a tuple type with variable index
            // Tuples don't have .get() method, so we convert to array for runtime indexing
            // Python: t[idx] where t = (1, 2) → Rust: [t.0, t.1][idx as usize]
            let is_tuple_base = self.is_tuple_base(base);

            if is_tuple_base && !matches!(index, HirExpr::Literal(Literal::Int(_))) {
                // Variable index on tuple - convert tuple to array
                // Get tuple element count from type info if available
                let tuple_size = self.get_tuple_size(base).unwrap_or(2);
                let index_expr = index.to_rust_expr(self.ctx)?;

                // Generate array from tuple elements: [t.0, t.1, ...]
                let indices: Vec<syn::Index> =
                    (0..tuple_size).map(syn::Index::from).collect();

                return Ok(parse_quote! {
                    [#(#base_expr.#indices),*][#index_expr as usize]
                });
            }

            // DEPYLER-1060: Check for dict with non-string keys (e.g., d = {1: "a"})
            // DEPYLER-1073: Handle float keys using DepylerValue::Float
            if self.is_dict_expr(base) {
                let index_expr = index.to_rust_expr(self.ctx)?;
                // Check if dict has float keys
                let has_float_keys = if let HirExpr::Var(var_name) = base {
                    self.ctx.var_types.get(var_name).is_some_and(|t| {
                        matches!(t, Type::Dict(key_type, _) if matches!(key_type.as_ref(), Type::Float))
                    })
                } else {
                    false
                };

                // HashMap<DepylerValue, _>.get() expects &DepylerValue
                if has_float_keys {
                    // Float keys use DepylerValue::Float
                    if matches!(index, HirExpr::Literal(Literal::Float(_))) {
                        return Ok(parse_quote! {
                            #base_expr.get(&DepylerValue::Float(#index_expr)).cloned().unwrap_or_default()
                        });
                    } else if matches!(index, HirExpr::Literal(Literal::Int(_))) {
                        // Int used as float key - convert
                        return Ok(parse_quote! {
                            #base_expr.get(&DepylerValue::Float(#index_expr as f64)).cloned().unwrap_or_default()
                        });
                    } else {
                        // Variable - use From trait
                        return Ok(parse_quote! {
                            #base_expr.get(&DepylerValue::from(#index_expr)).cloned().unwrap_or_default()
                        });
                    }
                } else {
                    // Integer keys use DepylerValue::Int
                    return Ok(parse_quote! {
                        #base_expr.get(&DepylerValue::Int(#index_expr as i64)).cloned().unwrap_or_default()
                    });
                }
            }

            // Vec/List access with numeric index
            let index_expr = index.to_rust_expr(self.ctx)?;

            // Check if index is a negative literal
            if let HirExpr::Unary {
                op: UnaryOp::Neg,
                operand,
            } = index
            {
                if let HirExpr::Literal(Literal::Int(n)) = **operand {
                    // Negative index literal: arr[-1] → arr.get(arr.len() - 1)
                    let offset = n as usize;
                    return Ok(parse_quote! {
                        {
                            // DEPYLER-0307 Fix #11: Use borrow to avoid moving the base expression
                            let base = &#base_expr;
                            // DEPYLER-0267: Use .cloned() instead of .copied() for non-Copy types (String, Vec, etc.)
                            base.get(base.len().saturating_sub(#offset)).cloned().unwrap_or_default()
                        }
                    });
                }
            }

            // DEPYLER-0357: Check if index is a positive integer literal
            // For literal indices like p[0], generate simple inline code: .get(0)
            // This avoids unnecessary temporary variables and runtime checks
            // DEPYLER-0730: Use .expect() instead of .unwrap_or_default() to:
            //   1. Match Python semantics (IndexError on out of bounds, not default)
            //   2. Avoid requiring Default trait bound on generic T
            if let HirExpr::Literal(Literal::Int(n)) = index {
                let idx_value = *n as usize;
                return Ok(parse_quote! {
                    #base_expr.get(#idx_value).cloned().expect("IndexError: list index out of range")
                });
            }

            // DEPYLER-0306 FIX: Check if index is a simple variable (not a complex expression)
            // Simple variables in for loops like `for i in range(len(arr))` are guaranteed >= 0
            // For these, we can use simpler inline code that works in range contexts
            let is_simple_var = matches!(index, HirExpr::Var(_));

            if is_simple_var {
                // Simple variable index - use inline expression (works in range contexts)
                // This avoids block expressions that break in `for j in 0..matrix[i].len()`
                // DEPYLER-0730: Use .expect() for Python IndexError semantics
                Ok(parse_quote! {
                    #base_expr.get(#index_expr as usize).cloned().expect("IndexError: list index out of range")
                })
            } else {
                // Complex expression - use block with full negative index handling
                // DEPYLER-0288: Explicitly type idx as i32 to support negation
                // DEPYLER-0730: Use .expect() for Python IndexError semantics
                Ok(parse_quote! {
                    {
                        // DEPYLER-0307 Fix #11: Use borrow to avoid moving the base expression
                        let base = &#base_expr;
                        let idx: i32 = #index_expr;
                        let actual_idx = if idx < 0 {
                            // Use .abs() instead of negation to avoid "Neg not implemented for usize" error
                            base.len().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        // DEPYLER-0267: Use .cloned() instead of .copied() for non-Copy types (String, Vec, etc.)
                        base.get(actual_idx).cloned().expect("IndexError: list index out of range")
                    }
                })
            }
        }
    }

    /// Check if the index expression is a string key (for HashMap access)
    /// Returns true if: index is string literal, OR index variable is string type
    /// DEPYLER-1060: Does NOT return true just because base is Dict - that could have non-string keys
    pub(crate) fn is_string_index(&self, base: &HirExpr, index: &HirExpr) -> Result<bool> {
        // Check 1: Is index a string literal?
        if matches!(index, HirExpr::Literal(Literal::String(_))) {
            return Ok(true);
        }

        // DEPYLER-1060: If index is an integer literal, this is NOT string indexing
        // Even for dicts - they might have integer keys like {1: "a"}
        if matches!(index, HirExpr::Literal(Literal::Int(_))) {
            return Ok(false);
        }

        // Check 2: Is base expression a Dict/HashMap type?
        // We need to look at the base's inferred type
        if let HirExpr::Var(sym) = base {
            // DEPYLER-0449: First check actual variable type if known
            if let Some(var_type) = self.ctx.var_types.get(sym) {
                // DEPYLER-1060: For Dict types, only use string indexing if index is a string variable
                // Not just because base is Dict - dict could have non-string keys
                if matches!(var_type, Type::Dict(_, _)) {
                    // Check if index is a string variable
                    return Ok(self.is_string_variable(index));
                }
            }

            // Try to find the variable's type in the current function context
            // For parameters, we can check the function signature
            // For local variables, this is harder without full type inference
            //
            // DEPYLER-0422: Removed "data" from heuristic - too broad, catches sorted_data, dataset, etc.
            // Only use "dict" or "map" which are more specific to HashMap variables
            let name = sym.as_str();
            if (name.contains("dict")
                || name.contains("map")
                || name.contains("config")
                || name.contains("value"))
                && !self.is_numeric_index(index)
            {
                return Ok(true);
            }
        }

        // Check 3: Does the index expression look like a string variable?
        if self.is_string_variable(index) {
            return Ok(true);
        }

        // Default: assume numeric index (Vec/List access)
        Ok(false)
    }

    /// Check if expression is likely a string variable (heuristic)
    pub(crate) fn is_string_variable(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(sym) => {
                // DEPYLER-0449: First check actual variable type if known
                if let Some(var_type) = self.ctx.var_types.get(sym) {
                    // If variable is typed as String, it's a string index
                    if matches!(var_type, Type::String) {
                        return true;
                    }
                }

                // Fallback to heuristics
                let name = sym.as_str();
                // DEPYLER-0449: Expanded to include common loop variables like "k"
                // Heuristic: variable names like "key", "name", "id", "word", etc.
                name == "key"
                    || name == "k" // Common loop variable for keys
                    || name == "name"
                    || name == "id"
                    || name == "word"
                    || name == "text"
                    || name.ends_with("_key")
                    || name.ends_with("_name")
            }
            _ => false,
        }
    }

    /// Check if expression is likely numeric (heuristic)
    pub(crate) fn is_numeric_index(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::Int(_)) => true,
            HirExpr::Var(sym) => {
                let name = sym.as_str();
                // Common numeric index names
                name == "i"
                    || name == "j"
                    || name == "k"
                    || name == "idx"
                    || name == "index"
                    || name.starts_with("idx_")
                    || name.ends_with("_idx")
                    || name.ends_with("_index")
            }
            HirExpr::Binary { .. } => true, // Arithmetic expressions are numeric
            HirExpr::Call { .. } => false,  // Could be anything
            _ => false,
        }
    }

    /// DEPYLER-0299 Pattern #3: Check if base expression is a String type (heuristic)
    /// Returns true if base is likely a String/str type (not Vec/List)
    pub(crate) fn is_string_base(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(sym) => {
                // DEPYLER-0479: Check type system first (most reliable)
                if let Some(ty) = self.ctx.var_types.get(sym) {
                    // Return true if definitely String, false if definitely NOT string
                    // Fall through to heuristics for Unknown/Any types
                    match ty {
                        Type::String => return true,
                        // DEPYLER-0579: Optional<String> is still string-like
                        Type::Optional(inner) if matches!(**inner, Type::String) => return true,
                        Type::Int | Type::Float | Type::Bool | Type::List(_) | Type::Dict(_, _) => {
                            return false;
                        }
                        _ => {} // Unknown/Any - fall through to heuristics
                    }
                }

                // DEPYLER-0267 FIX: Only match singular string-like names, NOT plurals
                // "words" (plural) is likely list[str], not str!
                // "word" (singular) without 's' ending is likely str
                let name = sym.as_str();
                // Only match if: singular AND string-like name
                let is_singular = !name.ends_with('s');
                name == "text"
                    || name == "s"
                    || name == "string"
                    || name == "line"
                    || name == "content"     // DEPYLER-0538: File content is usually String
                    || name == "timestamp"  // GH-70: Common string field (ISO 8601, etc.)
                    || name == "message"     // GH-70: Log messages are strings
                    || name == "level"       // GH-70: Log levels are strings ("INFO", "ERROR")
                    || name == "prefix"      // String prefix for startswith operations
                    || name == "suffix"      // String suffix for endswith operations
                    || name == "pattern"     // String pattern for matching
                    || name == "char"        // Single character string
                    || name == "delimiter"   // String delimiter
                    || name == "separator"   // String separator
                    || (name == "word" && is_singular)
                    || (name.starts_with("text") && is_singular)
                    || (name.starts_with("str") && is_singular)
                    || (name.ends_with("_str") && is_singular)
                    || (name.ends_with("_string") && is_singular)
                    || (name.ends_with("_word") && is_singular)
                    || (name.ends_with("_text") && is_singular)
                    || (name.ends_with("timestamp") && is_singular)  // GH-70: created_timestamp, etc.
                    || (name.ends_with("_message") && is_singular) // GH-70: error_message, etc.
            }
            // DEPYLER-0577: Handle attribute access (e.g., args.text, args.prefix)
            HirExpr::Attribute { attr, .. } => {
                let name = attr.as_str();
                let is_singular = !name.ends_with('s');
                name == "text"
                    || name == "s"
                    || name == "string"
                    || name == "line"
                    || name == "content"
                    || name == "message"
                    || name == "prefix"      // String prefix for startswith operations
                    || name == "suffix"      // String suffix for endswith operations
                    || name == "pattern"     // String pattern for matching
                    || name == "char"        // Single character string
                    || name == "delimiter"   // String delimiter
                    || name == "separator"   // String separator
                    || name == "old"         // String replacement old value
                    || name == "new"         // String replacement new value
                    || (name.starts_with("text") && is_singular)
                    || (name.ends_with("_text") && is_singular)
                    || (name.ends_with("_string") && is_singular)
            }
            HirExpr::MethodCall { method, .. }
                if method.as_str().contains("upper")
                    || method.as_str().contains("lower")
                    || method.as_str().contains("strip")
                    || method.as_str().contains("lstrip")
                    || method.as_str().contains("rstrip")
                    || method.as_str().contains("title") =>
            {
                true
            }
            HirExpr::Call { func, .. } if func.as_str() == "str" => true,
            // DEPYLER-0573: Dict value access with string-like keys
            // Pattern: dict["hash"], dict.get("hash")... - these return string values
            HirExpr::Index { base, index } if self.is_dict_expr(base) => {
                // Check if key suggests string value
                if let HirExpr::Literal(Literal::String(key)) = index.as_ref() {
                    let k = key.to_lowercase();
                    k.contains("hash")
                        || k.contains("name")
                        || k.contains("path")
                        || k.contains("text")
                        || k.contains("message")
                        || k.contains("algorithm")
                        || k.contains("filename")
                        || k.contains("modified")
                } else {
                    false
                }
            }
            // DEPYLER-0573: Dict.get() chain with string-like keys
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } if (method == "get" || method == "cloned" || method == "unwrap_or_default")
                && self.is_dict_value_access(object) =>
            {
                // If it's a get() call, check the key
                if method == "get" && !args.is_empty() {
                    if let HirExpr::Literal(Literal::String(key)) = &args[0] {
                        let k = key.to_lowercase();
                        return k.contains("hash")
                            || k.contains("name")
                            || k.contains("path")
                            || k.contains("text")
                            || k.contains("message")
                            || k.contains("algorithm")
                            || k.contains("filename")
                            || k.contains("modified");
                    }
                }
                // For cloned/unwrap_or_default, check the chain
                self.is_string_base(object)
            }
            _ => false,
        }
    }

    // DEPYLER-REFACTOR-001: is_string_method_call moved to builtin_conversions module

    /// DEPYLER-0701: Check if base expression is a tuple type
    /// Used to detect tuple[idx] patterns that need special handling
    pub(crate) fn is_tuple_base(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Tuple(_) => true,
            HirExpr::Var(sym) => {
                // Check type system for Tuple type
                if let Some(ty) = self.ctx.var_types.get(sym) {
                    matches!(ty, Type::Tuple(_))
                } else {
                    // Heuristic: common tuple variable names
                    let name = sym.as_str();
                    matches!(name, "pair" | "tuple" | "entry" | "item" | "elem" | "row" | "t")
                }
            }
            // Method call returning tuple (e.g., dict.items() element)
            HirExpr::MethodCall { object, method, .. } => {
                // Enumerate returns (index, value) tuples
                if method == "enumerate" {
                    return true;
                }
                // Dict.items() returns (key, value) tuples
                if method == "items" && self.is_dict_expr(object) {
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0701: Get the size of a tuple type for array conversion
    /// Returns the number of elements in the tuple, or None if unknown
    pub(crate) fn get_tuple_size(&self, expr: &HirExpr) -> Option<usize> {
        match expr {
            HirExpr::Tuple(elements) => Some(elements.len()),
            HirExpr::Var(sym) => {
                if let Some(Type::Tuple(types)) = self.ctx.var_types.get(sym) {
                    Some(types.len())
                } else {
                    None // Default will be used
                }
            }
            _ => None,
        }
    }

    pub(crate) fn convert_slice(
        &mut self,
        base: &HirExpr,
        start: &Option<Box<HirExpr>>,
        stop: &Option<Box<HirExpr>>,
        step: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let base_expr = base.to_rust_expr(self.ctx)?;

        // DEPYLER-0302 Phase 3: Check if we're slicing a string
        let is_string = self.is_string_base(base);

        // Convert slice parameters
        let start_expr = if let Some(s) = start {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        let stop_expr = if let Some(s) = stop {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        let step_expr = if let Some(s) = step {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        // DEPYLER-0302 Phase 3: Generate string-specific slice code
        if is_string {
            // DEPYLER-0573: If base is dict value access returning Value, convert to owned String
            // Value.as_str() returns &str with limited lifetime, so convert to String
            let final_base_expr = if self.is_dict_value_access(base) {
                parse_quote! { #base_expr.as_str().map(|s| s.to_string()).unwrap_or_default() }
            } else {
                base_expr
            };
            return self.convert_string_slice(final_base_expr, start_expr, stop_expr, step_expr);
        }

        // Generate slice code based on the parameters (for Vec/List)
        match (start_expr, stop_expr, step_expr) {
            // Full slice with step: base[::step]
            (None, None, Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        // DEPYLER-0812: Use i32 for step to support negative values
                        let step: i32 = #step;
                        if step == 1 {
                            base.clone()
                        } else if step > 0 {
                            base.iter().step_by(step as usize).cloned().collect::<Vec<_>>()
                        } else if step == -1 {
                            base.iter().rev().cloned().collect::<Vec<_>>()
                        } else {
                            // Negative step with abs value
                            let abs_step = (-step) as usize;
                            base.iter().rev().step_by(abs_step).cloned().collect::<Vec<_>>()
                        }
                    }
                })
            }

            // Start and stop: base[start:stop]
            (Some(start), Some(stop), None) => Ok(parse_quote! {
                {
                    // DEPYLER-0473: Borrow to avoid moving base (allows reuse later)
                    let base = &#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    // DEPYLER-1083: Parenthesize to avoid cast precedence issues (i + size as isize parses as i + (size as isize))
                    let start_idx = (#start) as isize;
                    let stop_idx = (#stop) as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    let stop = if stop_idx < 0 {
                        (base.len() as isize + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    if start < base.len() {
                        base[start..stop.min(base.len())].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Start only: base[start:]
            (Some(start), None, None) => Ok(parse_quote! {
                {
                    // DEPYLER-0473: Borrow to avoid moving base (allows reuse later)
                    let base = &#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    // DEPYLER-1083: Parenthesize to avoid cast precedence issues
                    let start_idx = (#start) as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    if start < base.len() {
                        base[start..].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop only: base[:stop]
            (None, Some(stop), None) => Ok(parse_quote! {
                {
                    // DEPYLER-0473: Borrow to avoid moving base (allows reuse later)
                    let base = &#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    // DEPYLER-1083: Parenthesize to avoid cast precedence issues
                    let stop_idx = (#stop) as isize;
                    let stop = if stop_idx < 0 {
                        (base.len() as isize + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    base[..stop.min(base.len())].to_vec()
                }
            }),

            // Full slice: base[:]
            (None, None, None) => Ok(parse_quote! { #base_expr.clone() }),

            // Start, stop, and step: base[start:stop:step]
            (Some(start), Some(stop), Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        // DEPYLER-0459: Cast to isize first to handle negative indices
                        // DEPYLER-1083: Parenthesize to avoid cast precedence issues
                        let start_idx = (#start) as isize;
                        let stop_idx = (#stop) as isize;
                        let start = if start_idx < 0 {
                            (base.len() as isize + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        let stop = if stop_idx < 0 {
                            (base.len() as isize + stop_idx).max(0) as usize
                        } else {
                            stop_idx as usize
                        };
                        // DEPYLER-0812: Use i32 for step to support negative values
                        let step: i32 = #step;

                        if step == 1 {
                            if start < base.len() {
                                base[start..stop.min(base.len())].to_vec()
                            } else {
                                Vec::new()
                            }
                        } else if step > 0 {
                            base[start..stop.min(base.len())]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            // Negative step - slice in reverse
                            let abs_step = (-step) as usize;
                            if start < base.len() {
                                base[start..stop.min(base.len())]
                                    .iter()
                                    .rev()
                                    .step_by(abs_step)
                                    .cloned()
                                    .collect::<Vec<_>>()
                            } else {
                                Vec::new()
                            }
                        }
                    }
                })
            }

            // Start and step: base[start::step]
            (Some(start), None, Some(step)) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    // DEPYLER-1083: Parenthesize to avoid cast precedence issues
                    let start_idx = (#start) as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    // DEPYLER-0812: Use i32 for step to support negative values
                    let step: i32 = #step;

                    if start < base.len() {
                        if step == 1 {
                            base[start..].to_vec()
                        } else if step > 0 {
                            base[start..]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else if step == -1 {
                            base[start..]
                                .iter()
                                .rev()
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            let abs_step = (-step) as usize;
                            base[start..]
                                .iter()
                                .rev()
                                .step_by(abs_step)
                                .cloned()
                                .collect::<Vec<_>>()
                        }
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop and step: base[:stop:step]
            (None, Some(stop), Some(step)) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let stop = (#stop).max(0) as usize;
                    // DEPYLER-0812: Use i32 for step to support negative values
                    let step: i32 = #step;

                    if step == 1 {
                        base[..stop.min(base.len())].to_vec()
                    } else if step > 0 {
                        base[..stop.min(base.len())]
                            .iter()
                            .step_by(step as usize)
                            .cloned()
                            .collect::<Vec<_>>()
                    } else if step == -1 {
                        base[..stop.min(base.len())]
                            .iter()
                            .rev()
                            .cloned()
                            .collect::<Vec<_>>()
                    } else {
                        let abs_step = (-step) as usize;
                        base[..stop.min(base.len())]
                            .iter()
                            .rev()
                            .step_by(abs_step)
                            .cloned()
                            .collect::<Vec<_>>()
                    }
                }
            }),
        }
    }

    /// DEPYLER-0302 Phase 3: String-specific slice code generation
    /// Handles string slicing with proper char boundaries and negative indices
    pub(crate) fn convert_string_slice(
        &mut self,
        base_expr: syn::Expr,
        start_expr: Option<syn::Expr>,
        stop_expr: Option<syn::Expr>,
        step_expr: Option<syn::Expr>,
    ) -> Result<syn::Expr> {
        match (start_expr, stop_expr, step_expr) {
            // Full slice with step: s[::step]
            (None, None, Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        let step: i32 = #step;
                        if step == 1 {
                            base.to_string()
                        } else if step > 0 {
                            base.chars().step_by(step as usize).collect::<String>()
                        } else if step == -1 {
                            base.chars().rev().collect::<String>()
                        } else {
                            // Negative step with abs value
                            let abs_step = step.abs() as usize;
                            base.chars().rev().step_by(abs_step).collect::<String>()
                        }
                    }
                })
            }

            // Start and stop: s[start:stop]
            (Some(start), Some(stop), None) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let start_idx: i32 = #start;
                    let stop_idx: i32 = #stop;
                    let len = base.chars().count() as i32;

                    // Handle negative indices
                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };

                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };

                    if actual_start < actual_stop {
                        base.chars().skip(actual_start).take(actual_stop - actual_start).collect::<String>()
                    } else {
                        String::new()
                    }
                }
            }),

            // Start only: s[start:]
            (Some(start), None, None) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let start_idx: i32 = #start;
                    let len = base.chars().count() as i32;

                    // Handle negative index for s[-n:]
                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };

                    base.chars().skip(actual_start).collect::<String>()
                }
            }),

            // Stop only: s[:stop]
            (None, Some(stop), None) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let stop_idx: i32 = #stop;
                    let len = base.chars().count() as i32;

                    // Handle negative index for s[:-n]
                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };

                    base.chars().take(actual_stop).collect::<String>()
                }
            }),

            // Full slice: s[:]
            (None, None, None) => Ok(parse_quote! { #base_expr.to_string() }),

            // Start, stop, and step: s[start:stop:step]
            (Some(start), Some(stop), Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        let start_idx: i32 = #start;
                        let stop_idx: i32 = #stop;
                        let step: i32 = #step;
                        let len = base.chars().count() as i32;

                        // Handle negative indices
                        let actual_start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx.min(len) as usize
                        };

                        let actual_stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx.min(len) as usize
                        };

                        if step == 1 {
                            if actual_start < actual_stop {
                                base.chars().skip(actual_start).take(actual_stop - actual_start).collect::<String>()
                            } else {
                                String::new()
                            }
                        } else if step > 0 {
                            base.chars()
                                .skip(actual_start)
                                .take(actual_stop.saturating_sub(actual_start))
                                .step_by(step as usize)
                                .collect::<String>()
                        } else {
                            // Negative step - collect range then reverse
                            let abs_step = step.abs() as usize;
                            if actual_start < actual_stop {
                                base.chars()
                                    .skip(actual_start)
                                    .take(actual_stop - actual_start)
                                    .rev()
                                    .step_by(abs_step)
                                    .collect::<String>()
                            } else {
                                String::new()
                            }
                        }
                    }
                })
            }

            // Start and step: s[start::step]
            (Some(start), None, Some(step)) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let start_idx: i32 = #start;
                    let step: i32 = #step;
                    let len = base.chars().count() as i32;

                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };

                    if step == 1 {
                        base.chars().skip(actual_start).collect::<String>()
                    } else if step > 0 {
                        base.chars().skip(actual_start).step_by(step as usize).collect::<String>()
                    } else if step == -1 {
                        base.chars().skip(actual_start).rev().collect::<String>()
                    } else {
                        let abs_step = step.abs() as usize;
                        base.chars().skip(actual_start).rev().step_by(abs_step).collect::<String>()
                    }
                }
            }),

            // Stop and step: s[:stop:step]
            (None, Some(stop), Some(step)) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let stop_idx: i32 = #stop;
                    let step: i32 = #step;
                    let len = base.chars().count() as i32;

                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };

                    if step == 1 {
                        base.chars().take(actual_stop).collect::<String>()
                    } else if step > 0 {
                        base.chars().take(actual_stop).step_by(step as usize).collect::<String>()
                    } else if step == -1 {
                        base.chars().take(actual_stop).rev().collect::<String>()
                    } else {
                        let abs_step = step.abs() as usize;
                        base.chars().take(actual_stop).rev().step_by(abs_step).collect::<String>()
                    }
                }
            }),
        }
    }

    pub(crate) fn convert_list(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        // CITL: Trace list construction decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "list_construction",
            chosen = "vec_macro",
            alternatives = ["Vec_new", "array_literal", "smallvec", "tinyvec"],
            confidence = 0.90
        );

        // DEPYLER-0269 FIX: Convert string literals to owned Strings
        // List literals with string elements should use Vec<String> not Vec<&str>
        // This ensures they can be passed to functions expecting &Vec<String>

        // DEPYLER-0572: Detect if list has mixed types (dict access Value + format! String)
        // If so, unify to String by converting Value elements via .to_string()
        let has_dict_access = elts.iter().any(|e| self.is_dict_value_access(e));
        let has_fstring = elts.iter().any(|e| matches!(e, HirExpr::FString { .. }));
        let needs_string_unify = has_dict_access && has_fstring;

        // DEPYLER-0711: Detect heterogeneous list literals (mixed primitive types)
        // Rust's vec![] requires all elements to be the same type
        // For mixed types like [1, "hello", 3.14, true], use Vec<serde_json::Value>
        let has_mixed_types = self.list_has_mixed_types(elts);

        // DEPYLER-0741: Detect if list contains dicts and if ANY dict has None values
        // If so, ALL dicts must use Option<V> for type consistency
        let any_dict_has_none = elts.iter().any(|e| {
            if let HirExpr::Dict(items) = e {
                items
                    .iter()
                    .any(|(_, v)| matches!(v, HirExpr::Literal(Literal::None)))
            } else {
                false
            }
        });

        // Set flag before processing so convert_dict knows to wrap values in Some()
        if any_dict_has_none {
            self.ctx.force_dict_value_option_wrap = true;
        }

        // Scope guard: reset flag after processing list elements
        let result = self.convert_list_elements(elts, has_mixed_types, needs_string_unify);
        self.ctx.force_dict_value_option_wrap = false;
        result
    }

    /// DEPYLER-0741: Helper to convert list elements, allowing the flag to be reset afterward
    pub(crate) fn convert_list_elements(
        &mut self,
        elts: &[HirExpr],
        has_mixed_types: bool,
        needs_string_unify: bool,
    ) -> Result<syn::Expr> {
        let nasa_mode = self.ctx.type_mapper.nasa_mode;

        if has_mixed_types {
            // DEPYLER-1033: In NASA mode, convert all elements to String instead of using serde_json
            if nasa_mode {
                let elt_exprs: Vec<syn::Expr> = elts
                    .iter()
                    .map(|e| {
                        let expr = e.to_rust_expr(self.ctx)?;
                        // Convert all elements to String for NASA mode compatibility
                        Ok(parse_quote! { format!("{:?}", #expr) })
                    })
                    .collect::<Result<Vec<_>>>()?;

                return Ok(parse_quote! { vec![#(#elt_exprs),*] });
            }

            // DEPYLER-0711: Convert to Vec<serde_json::Value> for heterogeneous lists
            self.ctx.needs_serde_json = true;

            let elt_exprs: Vec<syn::Expr> = elts
                .iter()
                .map(|e| {
                    let expr = e.to_rust_expr(self.ctx)?;
                    // Wrap each element in serde_json::json!()
                    Ok(parse_quote! { serde_json::json!(#expr) })
                })
                .collect::<Result<Vec<_>>>()?;

            return Ok(parse_quote! { vec![#(#elt_exprs),*] });
        }

        // DEPYLER-0739: Detect if list contains None elements
        // If so, wrap non-None elements in Some() to create Vec<Option<T>>
        let has_none = elts
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(Literal::None)));

        if has_none {
            let elt_exprs: Vec<syn::Expr> = elts
                .iter()
                .map(|e| {
                    if matches!(e, HirExpr::Literal(Literal::None)) {
                        // None stays as None
                        Ok(parse_quote! { None })
                    } else {
                        // Non-None elements get wrapped in Some()
                        let mut expr = e.to_rust_expr(self.ctx)?;
                        // Convert string literals to owned Strings
                        if matches!(e, HirExpr::Literal(Literal::String(_))) {
                            expr = parse_quote! { #expr.to_string() };
                        }
                        Ok(parse_quote! { Some(#expr) })
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            return Ok(parse_quote! { vec![#(#elt_exprs),*] });
        }

        // DEPYLER-0782: Check if list has string literals to determine if it's Vec<String>
        let has_string_literals = elts
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(Literal::String(_))));

        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let mut expr = e.to_rust_expr(self.ctx)?;
                // Check if element is a string literal
                if matches!(e, HirExpr::Literal(Literal::String(_))) {
                    expr = parse_quote! { #expr.to_string() };
                }
                // DEPYLER-0782: Variables need .to_string() in Vec<String> context
                // Both constants (SCRIPT: &str) and parameters (arg: &str) need conversion
                // String.to_string() is a no-op, so safe to call on any string type
                if matches!(e, HirExpr::Var(_)) && has_string_literals {
                    expr = parse_quote! { #expr.to_string() };
                }
                // DEPYLER-0572: Convert dict Value to String when mixed with f-strings
                if needs_string_unify && self.is_dict_value_access(e) {
                    expr = parse_quote! { #expr.to_string() };
                }
                Ok(expr)
            })
            .collect::<Result<Vec<_>>>()?;

        // Always use vec! for now to ensure mutability works
        // In the future, we should analyze if the list is mutated before deciding
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    /// DEPYLER-0711: Check if list has heterogeneous element types
    /// Returns true if elements have different primitive types (int, string, float, bool)
    pub(crate) fn list_has_mixed_types(&self, elts: &[HirExpr]) -> bool {
        if elts.len() <= 1 {
            return false; // Single element or empty - no mixing possible
        }

        let mut has_bool_literal = false;
        let mut has_int_literal = false;
        let mut has_float_literal = false;
        let mut has_string_literal = false;

        for elem in elts {
            match elem {
                HirExpr::Literal(Literal::Bool(_)) => has_bool_literal = true,
                HirExpr::Literal(Literal::Int(_)) => has_int_literal = true,
                HirExpr::Literal(Literal::Float(_)) => has_float_literal = true,
                HirExpr::Literal(Literal::String(_)) => has_string_literal = true,
                _ => {}
            }
        }

        // Count how many distinct literal types we have
        let distinct_types = [has_bool_literal, has_int_literal, has_float_literal, has_string_literal]
            .iter()
            .filter(|&&b| b)
            .count();

        // Mixed types if we have more than one distinct type
        distinct_types > 1
    }

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
        let target_needs_depyler_value = if let Some(Type::Dict(_, val_type)) = &self.ctx.current_assign_type {
            // Unknown value type means bare `dict` annotation → DepylerValue
            matches!(val_type.as_ref(), Type::Unknown)
        } else {
            false
        };

        // DEPYLER-1050: Check if function return type requires DepylerValue
        // When function returns HashMap<String, DepylerValue>, ALL dict literals in the
        // function body must use DepylerValue wrapping (even in nested return statements)
        let return_needs_depyler_value = if let Some(Type::Dict(_, val_type)) = &self.ctx.current_return_type {
            // Unknown value type means bare `dict` return → DepylerValue
            matches!(val_type.as_ref(), Type::Unknown)
        } else {
            false
        };

        // DEPYLER-1060: Check if dict has non-string keys
        // Point 14: {1: "a"} requires DepylerValue keys, not String keys
        let has_non_string_keys = items.iter().any(|(key, _)| {
            !matches!(key, HirExpr::Literal(Literal::String(_)))
        });

        // DEPYLER-1023: In NASA mode with mixed types, use DepylerValue enum
        // This ensures proper type fidelity for heterogeneous Python dicts
        // DEPYLER-1045: Also use DepylerValue when target type annotation requires it
        // DEPYLER-1050: Also use DepylerValue when function return type requires it
        // DEPYLER-1060: Also use DepylerValue when dict has non-string keys
        if nasa_mode && (has_mixed_types || target_needs_depyler_value || return_needs_depyler_value || has_non_string_keys) {
            self.ctx.needs_hashmap = true;
            self.ctx.needs_depyler_value_enum = true;

            // DEPYLER-1047: Check if return/target type expects String keys
            // Pattern: `fn f() -> HashMap<String, DepylerValue>` should use String keys
            // NOT DepylerValue keys, even when values are DepylerValue
            // NOTE: Bare `dict` return type parses as Dict(Unknown, Unknown) but generates
            // HashMap<String, DepylerValue>, so Unknown key type also means String keys
            let return_expects_string_keys = if let Some(Type::Dict(key_type, _)) = &self.ctx.current_return_type {
                matches!(key_type.as_ref(), Type::String | Type::Unknown)
            } else {
                false
            };
            let target_expects_string_keys = if let Some(Type::Dict(key_type, _)) = &self.ctx.current_assign_type {
                matches!(key_type.as_ref(), Type::String | Type::Unknown)
            } else {
                false
            };
            let use_string_keys = (return_expects_string_keys || target_expects_string_keys) && !has_non_string_keys;

            let mut insert_stmts = Vec::new();
            for (key, value) in items {
                let key_expr_raw = key.to_rust_expr(self.ctx)?;
                let val_expr = value.to_rust_expr(self.ctx)?;

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
                            Some(Type::Float) => parse_quote! { DepylerValue::Float(#val_expr as f64) },
                            Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#val_expr) },
                            Some(Type::String) => parse_quote! { DepylerValue::Str(#val_expr.to_string()) },
                            _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) }
                        }
                    }
                    // DEPYLER-1040: Handle struct field access (e.g., args.debug, args.count)
                    HirExpr::Attribute { attr, .. } => {
                        if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                            match field_type {
                                Type::Int => parse_quote! { DepylerValue::Int(#val_expr as i64) },
                                Type::Float => parse_quote! { DepylerValue::Float(#val_expr as f64) },
                                Type::Bool => parse_quote! { DepylerValue::Bool(#val_expr) },
                                Type::String => parse_quote! { DepylerValue::Str(#val_expr.to_string()) },
                                Type::Optional(inner) => {
                                    match inner.as_ref() {
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
                                        _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) }
                                    }
                                }
                                Type::List(_) => parse_quote! {
                                    DepylerValue::List(#val_expr.iter().map(|v| DepylerValue::Str(v.to_string())).collect())
                                },
                                _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) }
                            }
                        } else {
                            // DEPYLER-1040: Without explicit type info, use safe stringify
                            // Name-based heuristics are unreliable because fields might be Option<T>
                            // (e.g., args.count could be Option<i32>, not i32)
                            // Using format!("{:?}") is safe for any type
                            parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) }
                        }
                    }
                    // TODO: Handle nested List/Dict by recursively enabling DepylerValue context?
                    // For now, fallback to string representation for complex nested types to avoid compile errors
                    _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_expr)) }
                };

                insert_stmts.push(quote! { map.insert(#key_expr, #wrapped_val); });
            }

            return Ok(parse_quote! {
                {
                    let mut map = HashMap::new();
                    #(#insert_stmts)*
                    map
                }
            });
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
                let wrapped_val = if val_str.contains("HashMap") || val_str.contains("let mut map") {
                    // Use serde_json::to_value() for HashMap block expressions
                    quote! { serde_json::to_value(#val_expr).unwrap() }
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
        let has_none_value = items
            .iter()
            .any(|(_, v)| matches!(v, HirExpr::Literal(Literal::None)));

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
        let mut insert_stmts = Vec::new();
        for (key, value) in items {
            let mut key_expr = key.to_rust_expr(self.ctx)?;
            let mut val_expr = value.to_rust_expr(self.ctx)?;

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
            Ok(parse_quote! {
                {
                    let mut map = HashMap::new();
                    #(#insert_stmts)*
                    map
                }
            })
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
        if items.len() <= 1 {
            return Ok(false); // Single type or empty
        }

        // DEPYLER-0461: Check for nested dict expressions (recursively)
        // If any value is a Dict (or contains a Dict), we need serde_json::json!()
        // This ensures ALL levels of nested dicts use json!() for consistency
        if self.dict_contains_nested_dict(items) {
            return Ok(true);
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
                // Look up field type from class_field_types or infer from attribute name
                HirExpr::Attribute { attr, .. } => {
                    if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                        match field_type {
                            Type::Bool => has_bool_literal = true,
                            Type::Int => has_int_literal = true,
                            Type::Float => has_float_literal = true,
                            Type::String => has_string_literal = true,
                            Type::List(_) => has_complex_expr = true,
                            Type::Optional(_) => has_complex_expr = true,
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
        let distinct_list_types = [
            has_list_of_int,
            has_list_of_string,
            has_list_of_bool,
            has_list_of_float,
        ]
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

    pub(crate) fn convert_tuple(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        // DEPYLER-0682: Convert string literals in tuples to owned Strings
        // When tuples are used in lists (e.g., Vec<(i32, i32, String)>), string
        // elements need to be owned Strings, not &str references.
        // This ensures type consistency across all tuple elements in a Vec.
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let mut expr = e.to_rust_expr(self.ctx)?;
                // Convert string literals to .to_string() for owned String
                if matches!(e, HirExpr::Literal(Literal::String(_))) {
                    expr = parse_quote! { #expr.to_string() };
                }
                Ok(expr)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }

    pub(crate) fn convert_set(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;

        // DEPYLER-0742: Detect if set contains None
        let has_none = elts
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(Literal::None)));

        let mut insert_stmts = Vec::new();
        for elem in elts {
            // DEPYLER-0742: Wrap non-None elements in Some() when set has None
            if has_none {
                if matches!(elem, HirExpr::Literal(Literal::None)) {
                    insert_stmts.push(quote! { set.insert(None); });
                } else {
                    let elem_expr = elem.to_rust_expr(self.ctx)?;
                    insert_stmts.push(quote! { set.insert(Some(#elem_expr)); });
                }
            } else {
                let elem_expr = elem.to_rust_expr(self.ctx)?;
                insert_stmts.push(quote! { set.insert(#elem_expr); });
            }
        }
        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_stmts)*
                set
            }
        })
    }

    pub(crate) fn convert_frozenset(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        self.ctx.needs_arc = true;

        // DEPYLER-0742: Detect if frozenset contains None
        let has_none = elts
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(Literal::None)));

        let mut insert_stmts = Vec::new();
        for elem in elts {
            // DEPYLER-0742: Wrap non-None elements in Some() when set has None
            if has_none {
                if matches!(elem, HirExpr::Literal(Literal::None)) {
                    insert_stmts.push(quote! { set.insert(None); });
                } else {
                    let elem_expr = elem.to_rust_expr(self.ctx)?;
                    insert_stmts.push(quote! { set.insert(Some(#elem_expr)); });
                }
            } else {
                let elem_expr = elem.to_rust_expr(self.ctx)?;
                insert_stmts.push(quote! { set.insert(#elem_expr); });
            }
        }
        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_stmts)*
                std::sync::Arc::new(set)
            }
        })
    }

    pub(crate) fn convert_attribute(&mut self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        // DEPYLER-0608: In cmd_* handlers, args.X → X (field is now a direct parameter)
        // This is because subcommand fields live in Commands::Variant, not on Args
        // The handler function now takes individual field parameters instead of &Args
        if self.ctx.in_cmd_handler {
            if let HirExpr::Var(var_name) = value {
                if var_name == "args" && self.ctx.cmd_handler_args_fields.contains(&attr.to_string())
                {
                    // Transform args.field → field (the field is now a direct parameter)
                    // DEPYLER-0941: Handle Rust keywords like "type" with raw identifier syntax
                    let attr_ident = if keywords::is_rust_keyword(attr) {
                        syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                    } else {
                        syn::Ident::new(attr, proc_macro2::Span::call_site())
                    };
                    return Ok(parse_quote! { #attr_ident });
                }
            }
        }

        // DEPYLER-0627: subprocess.run() now returns CompletedProcess struct
        // with .returncode, .stdout, .stderr fields - no conversion needed,
        // struct field access works directly

        // DEPYLER-0200: Handle os.environ direct access
        // os.environ → std::env::vars() as a HashMap-like collection
        if let HirExpr::Var(var_name) = value {
            if var_name == "os" && attr == "environ" {
                // os.environ returns an environment dict-like object
                // Convert to HashMap<String, String> for dict-like operations
                return Ok(parse_quote! {
                    std::env::vars().collect::<std::collections::HashMap<String, String>>()
                });
            }
        }

        // DEPYLER-1069: Handle datetime class constants (min, max, resolution)
        // date.min → DepylerDate::new(1, 1, 1)
        // datetime.min → DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0)
        // time.min → (0, 0, 0, 0)
        // timedelta.min → DepylerTimeDelta::new(-999999999, 0, 0)
        if let HirExpr::Var(var_name) = value {
            let nasa_mode = self.ctx.type_mapper.nasa_mode;
            if (var_name == "date" || var_name == "datetime" || var_name == "time" || var_name == "timedelta")
                && (attr == "min" || attr == "max" || attr == "resolution")
            {
                if var_name == "date" {
                    self.ctx.needs_depyler_date = true;
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            parse_quote! { DepylerDate::new(1, 1, 1) }
                        } else {
                            parse_quote! { DepylerDate::new(9999, 12, 31) }
                        });
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::NaiveDate::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDate::MAX }
                        });
                    }
                } else if var_name == "datetime" {
                    self.ctx.needs_depyler_datetime = true;
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            parse_quote! { DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0) }
                        } else {
                            parse_quote! { DepylerDateTime::new(9999, 12, 31, 23, 59, 59, 999999) }
                        });
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::NaiveDateTime::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDateTime::MAX }
                        });
                    }
                } else if var_name == "time" {
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            parse_quote! { (0u32, 0u32, 0u32, 0u32) }
                        } else if attr == "max" {
                            parse_quote! { (23u32, 59u32, 59u32, 999999u32) }
                        } else {
                            // resolution
                            parse_quote! { (0u32, 0u32, 0u32, 1u32) }
                        });
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::NaiveTime::MIN }
                        } else if attr == "max" {
                            parse_quote! { chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999999).unwrap() }
                        } else {
                            // resolution
                            parse_quote! { chrono::Duration::microseconds(1) }
                        });
                    }
                } else if var_name == "timedelta" {
                    self.ctx.needs_depyler_timedelta = true;
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            // timedelta.min = timedelta(-999999999)
                            parse_quote! { DepylerTimeDelta::new(-999999999, 0, 0) }
                        } else if attr == "max" {
                            // timedelta.max = timedelta(days=999999999, hours=23, minutes=59, seconds=59, microseconds=999999)
                            parse_quote! { DepylerTimeDelta::new(999999999, 86399, 999999) }
                        } else {
                            // resolution = timedelta(microseconds=1)
                            parse_quote! { DepylerTimeDelta::new(0, 0, 1) }
                        });
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::Duration::min_value() }
                        } else if attr == "max" {
                            parse_quote! { chrono::Duration::max_value() }
                        } else {
                            // resolution
                            parse_quote! { chrono::Duration::microseconds(1) }
                        });
                    }
                }
            }
        }

        if let HirExpr::Var(var_name) = value {
            // DEPYLER-0517: Handle exception variable attributes
            // Python: `except CalledProcessError as e: e.returncode`
            // Rust: Box<dyn Error> doesn't have returncode, use fallback
            // Common exception variable names: e, err, error, exc, exception
            let is_likely_exception = var_name == "e"
                || var_name == "err"
                || var_name == "error"
                || var_name == "exc"
                || var_name == "exception";

            if is_likely_exception && attr == "returncode" {
                // Use 1 as a generic non-zero exit code for errors
                return Ok(parse_quote! { 1 });
            }

            // DEPYLER-0535: Handle tempfile file handle attributes
            // Python: f.name → Rust: f.path().to_string_lossy().to_string()
            // Common tempfile variable names: f, temp, temp_file, tmpfile
            let is_likely_tempfile = var_name == "f"
                || var_name == "temp"
                || var_name == "tmp"
                || var_name.contains("temp")
                || var_name.contains("tmp");

            if is_likely_tempfile && attr == "name" {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #var_ident.path().to_string_lossy().to_string() });
            }

            // DEPYLER-0551: Handle os.stat_result attributes (from path.stat() / std::fs::metadata)
            // Python: stats.st_size → Rust: stats.len()
            // Python: stats.st_mtime → Rust: stats.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
            let is_likely_stats =
                var_name == "stats" || var_name == "stat" || var_name.ends_with("_stats");

            if is_likely_stats {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match attr {
                    "st_size" => {
                        // DEPYLER-0693: Cast file size to i64 (Python int can be large)
                        return Ok(parse_quote! { #var_ident.len() as i64 });
                    }
                    "st_mtime" => {
                        return Ok(parse_quote! {
                            #var_ident.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()
                        });
                    }
                    "st_ctime" => {
                        // Creation time (use modified as fallback on Unix)
                        return Ok(parse_quote! {
                            #var_ident.created().unwrap_or_else(|_| #var_ident.modified().unwrap())
                                .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()
                        });
                    }
                    "st_atime" => {
                        return Ok(parse_quote! {
                            #var_ident.accessed().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()
                        });
                    }
                    "st_mode" => {
                        // File permissions
                        return Ok(parse_quote! { #var_ident.permissions().mode() });
                    }
                    _ => {} // Fall through
                }
            }

            // DEPYLER-0551: Handle pathlib.Path attributes
            // Python: path.name → Rust: path.file_name().and_then(|n| n.to_str()).unwrap_or("")
            // Python: path.suffix → Rust: path.extension().map(|e| format!(".{}", e.to_str().unwrap())).unwrap_or_default()
            // DEPYLER-0706: Removed `var_name == "p"` - too many false positives (e.g., Person p, Point p)
            // Only use explicit path naming patterns to avoid confusing struct field access with path operations
            // DEPYLER-0942: Also check var_types for PathBuf/Path type assignment
            let is_named_path = var_name == "path" || var_name.ends_with("_path");
            let is_typed_path = self
                .ctx
                .var_types
                .get(var_name)
                .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                .unwrap_or(false);
            let is_likely_path = is_named_path || is_typed_path;

            if is_likely_path {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match attr {
                    "name" => {
                        return Ok(parse_quote! {
                            #var_ident.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string()
                        });
                    }
                    "suffix" => {
                        return Ok(parse_quote! {
                            #var_ident.extension().map(|e| format!(".{}", e.to_str().unwrap())).unwrap_or_default()
                        });
                    }
                    "stem" => {
                        return Ok(parse_quote! {
                            #var_ident.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string()
                        });
                    }
                    "parent" => {
                        return Ok(parse_quote! {
                            #var_ident.parent().map(|p| p.to_path_buf()).unwrap_or_default()
                        });
                    }
                    _ => {} // Fall through to regular attribute handling
                }
            }
        }

        // DEPYLER-0425: Handle subcommand field access (args.url → url)
        // If this is accessing a subcommand-specific field on args parameter,
        // generate just the field name (it's extracted via pattern matching)
        if let HirExpr::Var(var_name) = value {
            // Check if var_name is an args parameter
            // (heuristic: variable ending in "args" or exactly "args")
            if (var_name == "args" || var_name.ends_with("args"))
                && self.ctx.argparser_tracker.has_subcommands()
            {
                // Check if this field belongs to any subcommand
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
                    // Generate just the field name (extracted via pattern matching in func wrapper)
                    let attr_ident = if keywords::is_rust_keyword(attr) {
                        syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                    } else {
                        syn::Ident::new(attr, proc_macro2::Span::call_site())
                    };
                    return Ok(parse_quote! { #attr_ident });
                }
            }
        }

        // Handle classmethod cls.ATTR → Self::ATTR
        if let HirExpr::Var(var_name) = value {
            if var_name == "cls" && self.ctx.is_classmethod {
                // DEPYLER-0306 FIX: Use raw identifiers for attributes that are Rust keywords
                let attr_ident = if keywords::is_rust_keyword(attr) {
                    syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                } else {
                    syn::Ident::new(attr, proc_macro2::Span::call_site())
                };
                return Ok(parse_quote! { Self::#attr_ident });
            }

            // DEPYLER-0422 Fix #11: Detect enum constant access patterns
            // TypeName.CONSTANT → TypeName::CONSTANT
            // Five-Whys Root Cause:
            // 1. Why: E0423 - expected value, found struct 'Color'
            // 2. Why: Code generates Color.RED (field access) instead of Color::RED
            // 3. Why: Default attribute access uses dot syntax
            // 4. Why: No detection for type constant access vs field access
            // 5. ROOT CAUSE: Need to use :: for type-level constants
            //
            // Heuristic: If name starts with uppercase and attr is ALL_CAPS, it's likely an enum constant
            let first_char = var_name.chars().next().unwrap_or('a');
            let is_type_name = first_char.is_uppercase();
            let is_constant = attr.chars().all(|c| c.is_uppercase() || c == '_');

            if is_type_name && is_constant {
                let type_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #type_ident::#attr_ident });
            }
        }

        // Check if this is a module attribute access
        if let HirExpr::Var(module_name) = value {
            // DEPYLER-STDLIB-MATH: Handle math module constants
            // math.pi → std::f64::consts::PI
            // math.e → std::f64::consts::E
            // math.inf → f64::INFINITY
            // math.nan → f64::NAN
            if module_name == "math" {
                let result = match attr {
                    "pi" => parse_quote! { std::f64::consts::PI },
                    "e" => parse_quote! { std::f64::consts::E },
                    "tau" => parse_quote! { std::f64::consts::TAU },
                    "inf" => parse_quote! { f64::INFINITY },
                    "nan" => parse_quote! { f64::NAN },
                    // DEPYLER-0595: Math functions as first-class values
                    "sin" => parse_quote! { f64::sin },
                    "cos" => parse_quote! { f64::cos },
                    "tan" => parse_quote! { f64::tan },
                    "asin" => parse_quote! { f64::asin },
                    "acos" => parse_quote! { f64::acos },
                    "atan" => parse_quote! { f64::atan },
                    "sqrt" => parse_quote! { f64::sqrt },
                    "exp" => parse_quote! { f64::exp },
                    "log" => parse_quote! { f64::ln },
                    "log10" => parse_quote! { f64::log10 },
                    "floor" => parse_quote! { f64::floor },
                    "ceil" => parse_quote! { f64::ceil },
                    "abs" => parse_quote! { f64::abs },
                    _ => {
                        // If it's not a recognized constant/function, it might be a typo
                        bail!("math.{} is not a recognized constant or method", attr);
                    }
                };
                return Ok(result);
            }

            // DEPYLER-STDLIB-STRING: Handle string module constants
            // string.ascii_letters → "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            // string.digits → "0123456789"
            // string.punctuation → "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"
            if module_name == "string" {
                let result = match attr {
                    "ascii_lowercase" => parse_quote! { "abcdefghijklmnopqrstuvwxyz" },
                    "ascii_uppercase" => parse_quote! { "ABCDEFGHIJKLMNOPQRSTUVWXYZ" },
                    "ascii_letters" => {
                        parse_quote! { "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" }
                    }
                    "digits" => parse_quote! { "0123456789" },
                    "hexdigits" => parse_quote! { "0123456789abcdefABCDEF" },
                    "octdigits" => parse_quote! { "01234567" },
                    "punctuation" => parse_quote! { "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~" },
                    "whitespace" => parse_quote! { " \t\n\r\x0b\x0c" },
                    "printable" => {
                        parse_quote! { "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c" }
                    }
                    _ => {
                        // Not a string constant - might be a method like capwords
                        bail!("string.{} is not a recognized constant", attr);
                    }
                };
                return Ok(result);
            }

            // DEPYLER-0518: Handle re module constants
            // Python regex flags: re.IGNORECASE, re.MULTILINE, etc.
            // These are integer flags in Python but Rust regex uses builder methods.
            // For now, map them to constants that can be used in conditional checks.
            if module_name == "re" {
                let result = match attr {
                    // Map to integer constants (matching Python values for compatibility)
                    "IGNORECASE" | "I" => parse_quote! { 2i32 },
                    "MULTILINE" | "M" => parse_quote! { 8i32 },
                    "DOTALL" | "S" => parse_quote! { 16i32 },
                    "VERBOSE" | "X" => parse_quote! { 64i32 },
                    "ASCII" | "A" => parse_quote! { 256i32 },
                    "LOCALE" | "L" => parse_quote! { 4i32 },
                    "UNICODE" | "U" => parse_quote! { 32i32 },
                    _ => {
                        // Not a recognized constant - fall through to default handling
                        let module_ident =
                            syn::Ident::new(module_name, proc_macro2::Span::call_site());
                        let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                        return Ok(parse_quote! { #module_ident.#attr_ident });
                    }
                };
                return Ok(result);
            }

            // DEPYLER-STDLIB-SYS: Handle sys module attributes
            // sys.argv → std::env::args().collect()
            // sys.platform → compile-time platform string
            // DEPYLER-0381: sys.stdin/stdout/stderr → std::io::stdin()/stdout()/stderr()
            if module_name == "sys" {
                let result = match attr {
                    "argv" => parse_quote! { std::env::args().collect::<Vec<String>>() },
                    "platform" => {
                        // Return platform name based on target OS as String
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
                        parse_quote! { #platform.to_string() }
                    }
                    // DEPYLER-0381: I/O stream attributes (functions in Rust, not objects)
                    "stdin" => parse_quote! { std::io::stdin() },
                    "stdout" => parse_quote! { std::io::stdout() },
                    "stderr" => parse_quote! { std::io::stderr() },
                    // DEPYLER-0381: version_info as a tuple (major, minor)
                    // Note: Python's sys.version_info is a 5-tuple (major, minor, micro, releaselevel, serial)
                    // but most comparisons use only (major, minor), so we return a 2-tuple for compatibility
                    "version_info" => {
                        // Rust doesn't have runtime version info by default
                        // Return a compile-time constant tuple matching Python 3.11
                        parse_quote! { (3, 11) }
                    }
                    _ => {
                        bail!("sys.{} is not a recognized attribute", attr);
                    }
                };
                return Ok(result);
            }

            // DEPYLER-0335 FIX #2: Get rust_path and rust_name (clone to avoid borrow issues)
            let module_info = self
                .ctx
                .imported_modules
                .get(module_name)
                .and_then(|mapping| {
                    mapping
                        .item_map
                        .get(attr)
                        .map(|rust_name| (mapping.rust_path.clone(), rust_name.clone()))
                });

            if let Some((rust_path, rust_name)) = module_info {
                // Map to the Rust equivalent
                let path_parts: Vec<&str> = rust_name.split("::").collect();
                if path_parts.len() > 1 {
                    // DEPYLER-0335 FIX #2: Use rust_path from mapping instead of hardcoding "std"
                    let base_path: syn::Path =
                        syn::parse_str(&rust_path).unwrap_or_else(|_| parse_quote! { std });
                    let mut path = quote! { #base_path };
                    for part in path_parts {
                        let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                        path = quote! { #path::#part_ident };
                    }
                    return Ok(parse_quote! { #path });
                } else {
                    // Simple identifier
                    let ident = syn::Ident::new(&rust_name, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #ident });
                }
            }
        }

        // DEPYLER-STDLIB-DATETIME: Handle datetime/date/time/timedelta properties
        // In chrono, properties are accessed as methods: dt.year → dt.year()
        // This handles properties for fractions, pathlib, datetime, date, time, and timedelta instances
        let value_expr = value.to_rust_expr(self.ctx)?;
        match attr {
            // DEPYLER-STDLIB-FRACTIONS: Fraction properties
            "numerator" => {
                // f.numerator → *f.numer()
                return Ok(parse_quote! { *#value_expr.numer() });
            }

            "denominator" => {
                // f.denominator → *f.denom()
                return Ok(parse_quote! { *#value_expr.denom() });
            }

            // DEPYLER-STDLIB-PATHLIB: Path properties
            // DEPYLER-0357: Removed overly-aggressive "name" special case
            // The .name attribute should only map to .file_name() for Path types
            // For generic objects (like in sorted(people, key=lambda p: p.name)),
            // .name should be preserved as-is and fall through to default handling
            "stem" => {
                // p.stem → p.file_stem().unwrap().to_str().unwrap().to_string()
                return Ok(parse_quote! {
                    #value_expr.file_stem().unwrap().to_str().unwrap().to_string()
                });
            }

            "suffix" => {
                // p.suffix → p.extension().map(|e| format!(".{}", e.to_str().unwrap())).unwrap_or_default()
                return Ok(parse_quote! {
                    #value_expr.extension()
                        .map(|e| format!(".{}", e.to_str().unwrap()))
                        .unwrap_or_default()
                });
            }

            "parent" => {
                // p.parent → p.parent().unwrap().to_path_buf()
                return Ok(parse_quote! {
                    #value_expr.parent().unwrap().to_path_buf()
                });
            }

            "parts" => {
                // p.parts → p.components().map(|c| c.as_os_str().to_str().unwrap().to_string()).collect()
                return Ok(parse_quote! {
                    #value_expr.components()
                        .map(|c| c.as_os_str().to_str().unwrap().to_string())
                        .collect::<Vec<_>>()
                });
            }

            // datetime/date properties (require method calls in chrono)
            "year" | "month" | "day" | "hour" | "minute" | "second" | "microsecond" => {
                // Check if this might be a datetime/date/time object
                // We convert: dt.year → dt.year()
                let method_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #value_expr.#method_ident() as i32 });
            }

            // DEPYLER-1068: timedelta properties with DepylerTimeDelta
            // These now work correctly with the DepylerTimeDelta wrapper struct
            // which provides proper .days(), .seconds(), .microseconds() methods
            "days" => {
                // td.days → td.days() as i32 (DepylerTimeDelta returns i64)
                return Ok(parse_quote! { #value_expr.days() as i32 });
            }

            "seconds" => {
                // td.seconds → td.seconds() as i32
                return Ok(parse_quote! { #value_expr.seconds() as i32 });
            }

            "microseconds" => {
                // td.microseconds → td.microseconds() as i32
                return Ok(parse_quote! { #value_expr.microseconds() as i32 });
            }

            _ => {
                // Not a datetime property, continue with default handling
            }
        }

        // DEPYLER-0452: Check stdlib API mappings before default fallback
        // Try common CSV patterns (heuristic-based for now)
        if let Some(mapping) = self.ctx.stdlib_mappings.lookup("csv", "DictReader", attr) {
            // Found a CSV DictReader mapping - apply it
            let rust_code =
                mapping.generate_rust_code(&value_expr.to_token_stream().to_string(), &[]);
            if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
                return Ok(expr);
            }
        }

        // Also try generic Reader patterns
        if let Some(mapping) = self.ctx.stdlib_mappings.lookup("csv", "Reader", attr) {
            let rust_code =
                mapping.generate_rust_code(&value_expr.to_token_stream().to_string(), &[]);
            if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
                return Ok(expr);
            }
        }

        // Default behavior for non-module attributes
        // DEPYLER-0306 FIX: Use raw identifiers for attributes that are Rust keywords
        let attr_ident = if keywords::is_rust_keyword(attr) {
            syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(attr, proc_macro2::Span::call_site())
        };

        // DEPYLER-1138: DepylerValue proxy methods require () for property-like access
        // When accessing .tag, .text, .find, .findall on module alias values (e.g., ET.Element()),
        // these are methods not fields, so we need method call syntax
        // This promotes Python property access (root.tag) to Rust method call (root.tag())
        let depyler_value_properties = ["tag", "text", "find", "findall", "set"];
        if depyler_value_properties.contains(&attr) && !self.ctx.module_aliases.is_empty() {
            return Ok(parse_quote! { #value_expr.#attr_ident() });
        }

        // DEPYLER-0737: Check if this is a @property method access
        // In Python, @property allows method access without (), but Rust requires ()
        if self.ctx.property_methods.contains(attr) {
            Ok(parse_quote! { #value_expr.#attr_ident() })
        } else {
            Ok(parse_quote! { #value_expr.#attr_ident })
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

    pub(crate) fn convert_list_comp(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // DEPYLER-0504: Support multiple generators in list comprehensions
        // Strategy: Single generator → simple chain, Multiple → flat_map nesting
        // Same pattern as convert_generator_expression but with .collect::<Vec<_>>()

        if generators.is_empty() {
            bail!("List comprehension must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let element_expr = element.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-0454: Detect CSV reader variables in list comprehensions
            let is_csv_reader = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "reader"
                    || var_name.contains("csv")
                    || var_name.ends_with("_reader")
                    || var_name.starts_with("reader_")
            } else {
                false
            };

            // DEPYLER-0523: Detect file variables for BufReader wrapping
            // Same heuristics as stmt_gen.rs for loop file iteration
            let is_file_iter = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "f"
                    || var_name == "file"
                    || var_name == "input"
                    || var_name == "output"
                    || var_name.ends_with("_file")
                    || var_name.starts_with("file_")
            } else {
                false
            };

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr =
                if !is_csv_reader && !is_file_iter && !matches!(&*gen.iter, HirExpr::Var(_)) {
                    self.wrap_range_in_parens(iter_expr)
                } else {
                    iter_expr
                };

            let mut chain: syn::Expr = if is_csv_reader {
                // DEPYLER-0454: CSV reader - use deserialize pattern
                self.ctx.needs_csv = true;
                parse_quote! { #iter_expr.deserialize::<std::collections::HashMap<String, String>>().filter_map(|result| result.ok()) }
            } else if is_file_iter {
                // DEPYLER-0523: File variable - use BufReader for line iteration
                self.ctx.needs_bufread = true;
                parse_quote! { std::io::BufReader::new(#iter_expr).lines().map(|l| l.unwrap_or_default()) }
            } else if self.is_numpy_array_expr(&gen.iter) {
                // DEPYLER-0575: trueno Vector uses .as_slice().iter()
                // DEPYLER-0909: Use .cloned() instead of .copied() for compatibility with non-Copy types
                parse_quote! { #iter_expr.as_slice().iter().cloned() }
            } else if self.is_json_value_iteration(&gen.iter) {
                // DEPYLER-0607: JSON Value iteration in list comprehension
                // serde_json::Value doesn't implement IntoIterator, must convert first
                parse_quote! { #iter_expr.as_array().unwrap_or(&vec![]).iter().cloned() }
            } else if matches!(&*gen.iter, HirExpr::Var(_)) {
                // DEPYLER-1077: Check if variable is a string type - strings use .chars()
                let is_string_type = if let HirExpr::Var(var_name) = &*gen.iter {
                    self.ctx
                        .var_types
                        .get(var_name)
                        .map(|ty| matches!(ty, crate::hir::Type::String))
                        .unwrap_or(false)
                } else {
                    false
                };
                if is_string_type {
                    // DEPYLER-1077: String iteration uses .chars() not .iter()
                    // Also register target as a char iteration variable for ord() handling
                    self.ctx.char_iter_vars.insert(gen.target.clone());
                    parse_quote! { #iter_expr.chars() }
                } else {
                    // DEPYLER-0674: Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                    // .cloned() works for both Copy and Clone types, .copied() only works for Copy
                    parse_quote! { #iter_expr.iter().cloned() }
                }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-0792: Check if any condition contains a walrus operator (:=)
            // and if the element expression uses that walrus variable.
            // If so, we must use filter_map instead of filter + map, because
            // the walrus variable is defined in the filter closure but needed in map.
            let walrus_vars_in_conditions = walrus_helpers::collect_walrus_vars_from_conditions(&gen.conditions);
            let element_uses_walrus = !walrus_vars_in_conditions.is_empty()
                && walrus_helpers::expr_uses_any_var(element, &walrus_vars_in_conditions);

            if element_uses_walrus && gen.conditions.len() == 1 {
                // DEPYLER-0792: Single condition with walrus - use filter_map pattern
                // Python: [(w, length) for w in words if (length := len(w)) > 3]
                // Rust: words.iter().cloned().filter_map(|w| {
                //           let length = w.len() as i32;
                //           if length > 3 { Some((w, length)) } else { None }
                //       }).collect::<Vec<_>>()
                let cond = &gen.conditions[0];
                let cond_expr = cond.to_rust_expr(self.ctx)?;

                // Collect walrus variable assignments as let bindings
                let walrus_bindings = Self::generate_walrus_bindings(cond, self.ctx)?;

                chain = parse_quote! {
                    #chain.filter_map(|#target_pat| {
                        #walrus_bindings
                        if #cond_expr { Some(#element_expr) } else { None }
                    })
                };

                // Collect into Vec
                return Ok(parse_quote! { #chain.collect::<Vec<_>>() });
            }

            // DEPYLER-0691: Add filters for each condition (no walrus in element)
            // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            // After .iter().cloned(), filter receives &T reference, but condition expects T
            // Solution: let target = target.clone() inside closure shadows ref with owned value
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr }) };
            }

            // Add the map transformation
            chain = parse_quote! { #chain.map(|#target_pat| #element_expr) };

            // Collect into Vec
            return Ok(parse_quote! { #chain.collect::<Vec<_>>() });
        }

        // Multiple generators case (nested iteration with flat_map)
        // Pattern: [x + y for x in range(3) for y in range(3)]
        // Becomes: (0..3).flat_map(|x| (0..3).map(move |y| x + y)).collect::<Vec<_>>()

        let chain = self.convert_nested_generators_for_list_comp(element, generators)?;
        Ok(parse_quote! { #chain.collect::<Vec<_>>() })
    }

    pub(crate) fn convert_nested_generators_for_list_comp(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Start with the outermost generator
        let first_gen = &generators[0];
        let first_iter = first_gen.iter.to_rust_expr(self.ctx)?;
        let first_pat = self.parse_target_pattern(&first_gen.target)?;

        // Build the nested expression recursively
        let inner_expr = self.build_nested_chain(element, generators, 1)?;

        // Start the chain with the first generator
        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let first_iter = self.wrap_range_in_parens(first_iter);
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // DEPYLER-0691: Add filters for first generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        for cond in &first_gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#first_pat| #cond_expr) };
        }

        // Use flat_map for the first generator
        chain = parse_quote! { #chain.flat_map(|#first_pat| #inner_expr) };

        Ok(chain)
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

    /// Add dereference (*) to uses of target variable in expression
    /// This is needed because filter closures receive &T even when the iterator yields T
    /// Example: transforms `x > 0` to `*x > 0` when x is the target variable
    ///
    /// Note: Currently unused but kept for potential future use with filter optimization
    #[allow(dead_code)]
    pub(crate) fn add_deref_to_var_uses(&mut self, expr: &HirExpr, target: &str) -> Result<syn::Expr> {
        use crate::hir::{BinOp, HirExpr, UnaryOp};

        match expr {
            HirExpr::Var(name) if name == target => {
                // This is the target variable - add dereference
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                Ok(parse_quote! { *#ident })
            }
            HirExpr::Binary { op, left, right } => {
                // Recursively add derefs to both sides
                let left_expr = self.add_deref_to_var_uses(left, target)?;
                let right_expr = self.add_deref_to_var_uses(right, target)?;

                // Generate the operator token
                let result = match op {
                    BinOp::Add => parse_quote! { #left_expr + #right_expr },
                    BinOp::Sub => parse_quote! { #left_expr - #right_expr },
                    BinOp::Mul => parse_quote! { #left_expr * #right_expr },
                    BinOp::Div => parse_quote! { #left_expr / #right_expr },
                    BinOp::FloorDiv => parse_quote! { #left_expr / #right_expr },
                    BinOp::Mod => parse_quote! { #left_expr % #right_expr },
                    BinOp::Pow => parse_quote! { #left_expr.pow(#right_expr as u32) },
                    BinOp::Eq => parse_quote! { #left_expr == #right_expr },
                    BinOp::NotEq => parse_quote! { #left_expr != #right_expr },
                    BinOp::Lt => parse_quote! { #left_expr < #right_expr },
                    BinOp::LtEq => parse_quote! { #left_expr <= #right_expr },
                    BinOp::Gt => parse_quote! { #left_expr > #right_expr },
                    BinOp::GtEq => parse_quote! { #left_expr >= #right_expr },
                    BinOp::And => parse_quote! { #left_expr && #right_expr },
                    BinOp::Or => parse_quote! { #left_expr || #right_expr },
                    BinOp::BitAnd => parse_quote! { #left_expr & #right_expr },
                    BinOp::BitOr => parse_quote! { #left_expr | #right_expr },
                    BinOp::BitXor => parse_quote! { #left_expr ^ #right_expr },
                    BinOp::LShift => parse_quote! { #left_expr << #right_expr },
                    BinOp::RShift => parse_quote! { #left_expr >> #right_expr },
                    BinOp::In => parse_quote! { #right_expr.contains(&#left_expr) },
                    BinOp::NotIn => parse_quote! { !#right_expr.contains(&#left_expr) },
                };
                Ok(result)
            }
            HirExpr::Unary { op, operand } => {
                // Recursively add derefs to operand
                let operand_expr = self.add_deref_to_var_uses(operand, target)?;

                let result = match op {
                    UnaryOp::Not => parse_quote! { !#operand_expr },
                    UnaryOp::Neg => parse_quote! { -#operand_expr },
                    UnaryOp::Pos => parse_quote! { +#operand_expr },
                    UnaryOp::BitNot => parse_quote! { !#operand_expr },
                };
                Ok(result)
            }
            // For any other expression, convert normally (no deref needed)
            _ => expr.to_rust_expr(self.ctx),
        }
    }

    pub(crate) fn is_set_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Set(_) | HirExpr::FrozenSet(_) => true,
            HirExpr::Call { func, .. } if func == "set" || func == "frozenset" => true,
            HirExpr::Var(_) => {
                // Check type information in context for variables
                self.is_set_var(expr)
            }
            _ => false,
        }
    }

    /// DEPYLER-0575: Check if expression is a numpy array (trueno Vector)
    pub(crate) fn is_numpy_array_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // np.array() call
            HirExpr::Call { func, .. } if func == "array" => true,
            // DEPYLER-1044: np.zeros(), np.ones() always return vectors
            HirExpr::Call { func, .. } if matches!(func.as_str(), "zeros" | "ones") => true,
            // DEPYLER-1044: abs, sqrt, etc. return vector ONLY if argument is vector
            // abs(scalar) -> scalar, abs(array) -> array
            HirExpr::Call { func, args, .. }
                if matches!(
                    func.as_str(),
                    "abs" | "sqrt" | "sin" | "cos" | "exp" | "log" | "clip" | "clamp" | "normalize"
                ) =>
            {
                args.first().is_some_and(|arg| self.is_numpy_array_expr(arg))
            }
            // DEPYLER-1044: Method calls on numpy arrays return numpy arrays
            // BUT scalar.abs() returns scalar, not vector
            HirExpr::MethodCall { object, method, .. } => {
                // unwrap/abs/sqrt/etc preserve array nature of object
                if matches!(
                    method.as_str(),
                    "unwrap" | "abs" | "sqrt" | "sin" | "cos" | "exp" | "log" | "clamp" | "clip"
                ) {
                    return self.is_numpy_array_expr(object);
                }
                false
            }
            // DEPYLER-0804: Check var_types first to avoid false positives
            // Variables with known scalar types (Float, Int) are NOT numpy arrays
            HirExpr::Var(name) => {
                // DEPYLER-1044: FIRST check CSE temps - they are NEVER numpy arrays
                // This must happen before any other checks to prevent false positives
                let n = name.as_str();
                if n.starts_with("_cse_temp_") {
                    return false;
                }

                // DEPYLER-0932: Check numpy_vars set (most reliable)
                // This tracks variables explicitly assigned from numpy operations
                if self.ctx.numpy_vars.contains(name) {
                    return true;
                }

                // Next check var_types for definitive type info
                if let Some(ty) = self.ctx.var_types.get(name) {
                    // Scalar types are never numpy arrays
                    if matches!(ty, Type::Float | Type::Int | Type::Bool | Type::String) {
                        return false;
                    }
                    // DEPYLER-1044: Check for Rust-specific scalar types stored as Custom
                    // Parameters with explicit type annotations (e.g., a: i32) are stored as
                    // Type::Custom("i32"), not Type::Int. These are also scalars!
                    if let Type::Custom(type_name) = ty {
                        let tn = type_name.as_str();
                        // Rust scalar types are NOT numpy arrays
                        if matches!(
                            tn,
                            "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
                                | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
                                | "f32" | "f64"
                                | "bool"
                        ) {
                            return false;
                        }
                        // DEPYLER-0836: trueno::Vector<T> types are numpy arrays
                        if tn.starts_with("Vector<") || tn == "Vector" {
                            return true;
                        }
                    }
                    // DEPYLER-0955: Only treat List types as numpy arrays if they contain
                    // numeric primitives (Int, Float). Lists of tuples, strings, etc.
                    // should NOT use .copied() which requires Copy trait.
                    if let Type::List(inner) = ty {
                        // Only numeric inner types are numpy-like
                        if matches!(inner.as_ref(), Type::Int | Type::Float) {
                            return true;
                        }
                        // Non-numeric lists (tuples, strings, etc.) are NOT numpy arrays
                        return false;
                    }
                }
                // Fall back to name heuristics only for unknown types
                // DEPYLER-0804: Removed "x", "y" - too generic, often scalars
                // DEPYLER-1044: Removed "a", "b", "result" - WAY too generic, causes CSE failures
                // Only use truly unambiguous numpy-like names
                matches!(n, "arr" | "array" | "data" | "values" | "vec" | "vector")
                    || n.starts_with("arr_") || n.ends_with("_arr")
                    || n.starts_with("vec_") || n.ends_with("_vec")
            }
            // Recursive: binary op on vector yields vector
            HirExpr::Binary { left, .. } => self.is_numpy_array_expr(left),
            _ => false,
        }
    }

    /// DEPYLER-0188: Check if expression is a pathlib Path (std::path::PathBuf)
    ///
    /// Python's pathlib.Path uses `/` operator (via __truediv__) for path concatenation.
    /// Rust's PathBuf doesn't implement Div, so we convert to .join().
    pub(crate) fn is_path_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Path() or pathlib.Path() call
            HirExpr::Call { func, .. } => {
                matches!(func.as_str(), "Path" | "PurePath" | "PurePosixPath" | "PureWindowsPath")
            }
            // Method calls that return paths
            // Note: "resolve" and "absolute" are NOT included because they are converted
            // with .to_string_lossy().to_string() and thus return String, not PathBuf
            HirExpr::MethodCall { method, .. } => {
                matches!(method.as_str(), "parent" | "expanduser" |
                         "with_name" | "with_suffix" | "with_stem" | "joinpath")
            }
            // Attribute access like Path(__file__).parent
            HirExpr::Attribute { attr, .. } => {
                matches!(attr.as_str(), "parent" | "root" | "anchor")
            }
            // Variable named 'path' or with path-like semantics
            // DEPYLER-0188: Include common module-level path constants (SCRIPT, FILE, etc.)
            // DEPYLER-0930: Also check var_types for PathBuf type (e.g., result = Path(...))
            HirExpr::Var(name) => {
                // First check if variable is typed as PathBuf/Path
                let is_typed_path = self
                    .ctx
                    .var_types
                    .get(name)
                    .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                    .unwrap_or(false);
                if is_typed_path {
                    return true;
                }
                // Fall back to name-based heuristics
                let n = name.as_str();
                let n_lower = n.to_lowercase();
                matches!(n, "path" | "filepath" | "dir_path" | "file_path" | "base_path" | "root_path"
                         | "SCRIPT" | "SCRIPT_PATH" | "SCRIPT_DIR" | "SCRIPT_FILE"
                         | "ROOT" | "ROOT_DIR" | "ROOT_PATH" | "BASE" | "BASE_DIR")
                    || n.starts_with("path_") || n.ends_with("_path")
                    || n.starts_with("dir_") || n.ends_with("_dir")
                    || n_lower.ends_with("_path") || n_lower.ends_with("_dir")
                    || n_lower.starts_with("script")
            }
            // Recursive: path / segment is still a path
            HirExpr::Binary { left, op: BinOp::Div, .. } => self.is_path_expr(left),
            _ => false,
        }
    }

    /// DEPYLER-0607: Check if expression yields serde_json::Value that needs iteration conversion
    ///
    /// serde_json::Value doesn't implement IntoIterator, so we need to detect when
    /// the iteration expression is a JSON Value and wrap it with .as_array().
    ///
    /// Returns true for:
    /// - Variables with dict/JSON Value types in context
    /// - Method chains like data.get("items").cloned().unwrap_or_default()
    /// - Dict index expressions like data["items"]
    pub(crate) fn is_json_value_iteration(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable - check if it has a JSON/dict type in context
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Dict(_, v) if
                        matches!(v.as_ref(), Type::Unknown) ||
                        matches!(v.as_ref(), Type::Custom(n) if n.contains("Value") || n.contains("json")))
                } else {
                    // Heuristic: if needs_serde_json is set, variables may be JSON Values
                    self.ctx.needs_serde_json
                }
            }
            // Dict index expression - likely yields JSON Value
            HirExpr::Index { base, .. } => {
                match base.as_ref() {
                    HirExpr::Var(var_name) => {
                        if let Some(t) = self.ctx.var_types.get(var_name) {
                            matches!(t, Type::Dict(_, v) if
                                matches!(v.as_ref(), Type::Unknown) ||
                                matches!(v.as_ref(), Type::Custom(n) if n.contains("Value") || n.contains("json")))
                        } else {
                            self.ctx.needs_serde_json
                        }
                    }
                    HirExpr::Dict(_) => true, // Dict literal
                    _ => false,
                }
            }
            // Method chains that yield JSON Value
            HirExpr::MethodCall { object, method, .. } => {
                let is_chain_method = matches!(method.as_str(),
                    "get" | "cloned" | "unwrap_or_default" | "unwrap_or" | "unwrap"
                );
                if is_chain_method {
                    self.is_json_value_iteration(object.as_ref())
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Check if a variable has a set type based on type information in context
    pub(crate) fn is_set_var(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                // Check var_types in context to see if this variable is a set
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Set(_))
                // DEPYLER-1060: Check module_constant_types for module-level static sets
                } else if let Some(var_type) = self.ctx.module_constant_types.get(name.as_str()) {
                    matches!(var_type, Type::Set(_))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0321: Check if expression is a string type
    /// Used to distinguish string.contains() from HashMap.contains_key()
    ///
    /// # Complexity
    /// 4 (match + type lookup + variant check + attribute check)
    pub(crate) fn is_string_type(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a string
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::String)
                } else {
                    // Fallback to heuristic for cases without type info
                    self.is_string_base(expr)
                }
            }
            // DEPYLER-0649: Handle attribute access for known string fields
            HirExpr::Attribute { attr, .. } => {
                // Known string attributes from common types:
                // - CompletedProcess.stdout, CompletedProcess.stderr
                // - Exception.args (often treated as string)
                // - argparse Namespace string fields
                matches!(
                    attr.as_str(),
                    "stdout" | "stderr" | "text" | "output" | "message" | "name"
                )
            }
            // DEPYLER-0675: Handle str() function call - returns String
            // Python: list(str(num)) → Rust: num.to_string().chars().collect()
            HirExpr::Call { func, .. } => {
                // str() builtin returns a string
                func == "str"
            }
            // DEPYLER-0676: Handle method calls that return strings
            // Python: list(num.to_string()) → Rust: num.to_string().chars().collect()
            HirExpr::MethodCall { method, .. } => {
                // Methods that return strings
                matches!(
                    method.as_str(),
                    "to_string" | "format" | "upper" | "lower" | "strip" | "replace" | "join"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0498: Check if expression is an Option type
    /// Used to determine if unwrap_or is needed in binary operations
    ///
    /// Returns true if:
    /// - Expression is a variable with Option<T> type
    /// - Expression is an attribute access that returns Option
    ///
    /// # Complexity
    /// 2 (match + type lookup)
    pub(crate) fn expr_is_option(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable: check if type is Optional
            HirExpr::Var(var_name) => {
                if let Some(var_type) = self.ctx.var_types.get(var_name) {
                    matches!(var_type, Type::Optional(_))
                } else {
                    false
                }
            }
            // Attribute access: check if field type is Optional
            HirExpr::Attribute { value, attr } => {
                // DEPYLER-0498: Check if self.field is Option in generator context
                if let HirExpr::Var(obj_name) = value.as_ref() {
                    if obj_name == "self" && self.ctx.in_generator {
                        // Check if this field is a generator state variable with Optional type
                        if self.ctx.generator_state_vars.contains(attr) {
                            // Field is a generator state var - check its type in var_types
                            if let Some(field_type) = self.ctx.var_types.get(attr) {
                                return matches!(field_type, Type::Optional(_));
                            }
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0303 Phase 3 Fix #7: Check if expression is a dict/HashMap
    /// Used for dict merge operator (|) and other dict-specific operations
    ///
    /// # Complexity
    /// 3 (match + type lookup + variant check)
    pub(crate) fn is_dict_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Dict(_) => true,
            HirExpr::Call { func, .. } if func == "dict" => true,
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a dict/HashMap
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    match var_type {
                        Type::Dict(_, _) => true,
                        // DEPYLER-1004: Handle bare Dict from typing import (becomes Type::Custom("Dict"))
                        Type::Custom(s) if s == "Dict" => true,
                        // DEPYLER-1004: json.loads() returns serde_json::Value which is dict-like
                        // When assigned from json.loads(), variables get Type::Custom("serde_json::Value")
                        Type::Custom(s) if s == "serde_json::Value" || s == "Value" => true,
                        _ => false,
                    }
                // DEPYLER-1060: Check module_constant_types for module-level static dicts
                // var_types is cleared per-function, but module_constant_types persists
                } else if let Some(var_type) = self.ctx.module_constant_types.get(name.as_str()) {
                    matches!(var_type, Type::Dict(_, _))
                } else {
                    false
                }
            }
            // DEPYLER-1044: Handle attribute access (e.g., self.config)
            HirExpr::Attribute { attr, .. } => {
                if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                    matches!(field_type, Type::Dict(_, _))
                        || matches!(field_type, Type::Custom(s) if s == "Dict")
                } else {
                    // Heuristic: common dict-like attribute names
                    let name = attr.as_str();
                    name == "config"
                        || name == "settings"
                        || name == "options"
                        || name == "data"
                        || name == "metadata"
                        || name == "headers"
                        || name == "params"
                        || name == "kwargs"
                        || name.ends_with("_dict")
                        || name.ends_with("_map")
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0572: Check if expression is a dict value access (returns serde_json::Value)
    /// Pattern: dict[key] or dict.get(key).cloned().unwrap_or_default()
    /// These return Value which needs .to_string() when mixed with String in lists
    pub(crate) fn is_dict_value_access(&self, expr: &HirExpr) -> bool {
        match expr {
            // dict[key] index access
            HirExpr::Index { base, .. } => self.is_dict_expr(base),
            // dict.get(key)... chain
            HirExpr::MethodCall { object, method, .. } => {
                if method == "get" {
                    self.is_dict_expr(object)
                } else if method == "cloned" || method == "unwrap_or_default" || method == "unwrap"
                {
                    // Check the chain for dict access
                    self.is_dict_value_access(object)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0540: Check if expression is typed as serde_json::Value
    /// serde_json::Value needs special handling for .keys(), .values(), .items()
    /// because it requires .as_object().unwrap() before iteration methods.
    /// DEPYLER-0969: H₃ Error Cascade Prevention - Type::Unknown maps to serde_json::Value
    /// so ALL Unknown-typed variables should use JSON method translations.
    /// DEPYLER-1017: In NASA mode, skip serde_json - Unknown maps to String
    pub(crate) fn is_serde_json_value(&self, expr: &HirExpr) -> bool {
        // DEPYLER-1017: In NASA mode, never treat anything as serde_json::Value
        if self.ctx.type_mapper.nasa_mode {
            return false;
        }
        if let HirExpr::Var(name) = expr {
            // Check explicit type info first - this is authoritative
            if let Some(var_type) = self.ctx.var_types.get(name) {
                // Check for explicit serde_json::Value type
                if matches!(var_type, Type::Custom(ref s) if s == "serde_json::Value") {
                    return true;
                }
                // DEPYLER-0708: Removed overly aggressive check for Dict(_, Unknown)
                // A plain `dict` annotation creates Dict(Unknown, Unknown) but should NOT
                // trigger serde_json::Value treatment. Only explicit serde_json::Value should.
                // Dict types use regular HashMap.iter(), not .as_object().
                if matches!(var_type, Type::Dict(_, _)) {
                    return false;
                }
                // DEPYLER-0969: H₃ Error Cascade Prevention
                // Type::Unknown maps to serde_json::Value in type_mapper.rs
                // Therefore, ALL Unknown-typed variables need JSON method translations
                // to prevent E0599 "no method named X found" cascading errors
                if matches!(var_type, Type::Unknown) {
                    return true;
                }
                // For other explicitly typed variables, not a JSON value
                return false;
            }

            // DEPYLER-0540: Use name heuristic when NO type info
            // (e.g., in nested closures where parent param types aren't tracked)
            // Be conservative - only match explicitly json-like names
            // Note: "filters", "config" are commonly used for serde_json::Value dicts
            let is_json_by_name = matches!(
                name.as_str(),
                "filters" | "json_data" | "json_obj" | "json_value" | "json_config" | "config"
            );
            if is_json_by_name {
                return true;
            }
        }
        false
    }

    /// DEPYLER-0550: Check if expression could be a serde_json::Value
    /// Used for comparison handling when .get() returns Option<String>
    /// but the other side is a JSON Value from .items() iteration
    pub(crate) fn is_serde_json_value_expr(&self, expr: &HirExpr) -> bool {
        // First check using the existing helper
        if self.is_serde_json_value(expr) {
            return true;
        }

        // DEPYLER-0550: Check for pattern variables from JSON iteration
        // When iterating over filters.items(), we get (col, val) where val is Value
        // The variable "val" in this context is a JSON Value
        if let HirExpr::Var(name) = expr {
            // Variables commonly used for JSON values in iteration patterns
            // "val" is the most common from: for col, val in filters.items()
            if matches!(name.as_str(), "val" | "v" | "value" | "json_val") {
                // Additional context check: if there's no type info, assume JSON in iteration
                if !self.ctx.var_types.contains_key(name) {
                    return true;
                }
            }
        }

        false
    }

    /// DEPYLER-0700: Check if dict expression has serde_json::Value values
    ///
    /// Returns true if the dict maps to HashMap<String, serde_json::Value>,
    /// which happens when:
    /// - Dict has heterogeneous value types (e.g., {"name": "Alice", "age": 42})
    /// - Dict value type is Unknown (untyped dict)
    /// - Dict uses serde_json expressions
    ///
    /// This is used to wrap default values in dict.get(key, default) with json!()
    /// for type compatibility.
    /// DEPYLER-1017: In NASA mode, never use serde_json::Value
    pub(crate) fn dict_has_json_value_values(&self, expr: &HirExpr) -> bool {
        // DEPYLER-1017: In NASA mode, dicts never have JSON values
        if self.ctx.type_mapper.nasa_mode {
            return false;
        }
        match expr {
            // Variable dict - check type info
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    // Dict with Unknown value type uses serde_json::Value
                    if matches!(var_type, Type::Dict(_, ref v) if matches!(**v, Type::Unknown)) {
                        return true;
                    }
                    // Custom type that is serde_json::Value or HashMap with Value
                    if matches!(var_type, Type::Custom(ref s) if s.contains("serde_json::Value") || (s.contains("HashMap") && s.contains("Value"))) {
                        return true;
                    }
                }
                // If serde_json is already needed, this dict might use Value
                // Conservative: if we're generating serde_json code, assume mixed types
                self.ctx.needs_serde_json
            }
            // Dict literal - check if it has mixed value types
            HirExpr::Dict(items) => {
                if let Ok(has_mixed) = self.dict_has_mixed_types(items) {
                    has_mixed
                } else {
                    // Error checking - assume needs json for safety
                    self.ctx.needs_serde_json
                }
            }
            // Method call on dict - check base object
            HirExpr::MethodCall { object, .. } => self.dict_has_json_value_values(object),
            // Index into another dict
            HirExpr::Index { base, .. } => self.dict_has_json_value_values(base),
            _ => {
                // Fallback: if serde_json is in use, assume might be Value type
                self.ctx.needs_serde_json
            }
        }
    }

    /// DEPYLER-0729: Check if dict value type is String (not &str)
    /// Used to determine if string literal defaults in dict.get() need .to_string()
    pub(crate) fn dict_value_type_is_string(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Dict(_, ref v) if matches!(**v, Type::String))
                } else {
                    false
                }
            }
            HirExpr::MethodCall { object, .. } => self.dict_value_type_is_string(object),
            HirExpr::Index { base, .. } => self.dict_value_type_is_string(base),
            _ => false,
        }
    }

    pub(crate) fn is_list_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::List(_) => true,
            HirExpr::Call { func, .. } if func == "list" => true,
            // DEPYLER-0811: Function calls that return list types
            HirExpr::Call { func, .. } => {
                if let Some(ret_type) = self.ctx.function_return_types.get(func) {
                    matches!(ret_type, Type::List(_))
                } else {
                    false
                }
            }
            HirExpr::Var(name) => {
                // DEPYLER-169: Check var_types for List type
                // This enables proper `item in list_var` detection
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::List(_))
                // DEPYLER-1060: Check module_constant_types for module-level static lists
                } else if let Some(var_type) = self.ctx.module_constant_types.get(name.as_str()) {
                    matches!(var_type, Type::List(_))
                } else {
                    // Fall back to conservative: only treat explicit list literals as lists
                    false
                }
            }
            // DEPYLER-0811: Binary Add of lists produces a list (for chained concat)
            HirExpr::Binary {
                op: BinOp::Add,
                left,
                right,
            } => self.is_list_expr(left) || self.is_list_expr(right),
            // DEPYLER-1044: Handle attribute access (e.g., self.permissions)
            // Check class_field_types for the attribute's type
            HirExpr::Attribute { attr, .. } => {
                if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                    matches!(field_type, Type::List(_))
                } else {
                    // Heuristic: common list-like attribute names
                    let name = attr.as_str();
                    name.ends_with("s") && !name.ends_with("ss")
                        || name.ends_with("list")
                        || name.ends_with("items")
                        || name.ends_with("elements")
                        || name == "permissions"
                        || name == "values"
                        || name == "keys"
                        || name == "children"
                        || name == "args"
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0742: Check if expression is a deque type (VecDeque)
    /// Used to generate correct VecDeque methods instead of Vec methods.
    pub(crate) fn is_deque_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Call to deque() constructor
            HirExpr::Call { func, .. }
                if func == "deque" || func == "collections.deque" || func == "Deque" =>
            {
                true
            }
            HirExpr::Var(name) => {
                // Check var_types for Deque type annotation
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    // Check if the type string contains "deque" or "VecDeque"
                    let type_str = format!("{:?}", var_type);
                    type_str.contains("deque") || type_str.contains("VecDeque")
                } else {
                    // Fallback: common deque variable names
                    matches!(
                        name.as_str(),
                        "d" | "dq" | "deque" | "queue" | "buffer" | "deck"
                    )
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0521: Check if variable is a borrowed string parameter (&str)
    ///
    /// Function parameters with Python `str` type annotation become `&str` in Rust.
    /// When used as dict keys, they should NOT have `&` added (already borrowed).
    ///
    /// Heuristic: If variable not in var_types and has a string-key-like name,
    /// it's likely a function parameter that's &str.
    ///
    /// # Complexity
    /// 2 (lookup + name check)
    pub(crate) fn is_borrowed_str_param(&self, var_name: &str) -> bool {
        // DEPYLER-0543: Check if variable is a function param with str type
        // These become &str in Rust and should NOT have & added
        if self.ctx.fn_str_params.contains(var_name) {
            return true; // already &str, don't add &
        }

        // When we have type info, use it
        if let Some(var_type) = self.ctx.var_types.get(var_name) {
            match var_type {
                Type::String => {
                    // Variable has Type::String but is NOT in fn_str_params
                    // This means it's a local variable (loop var, assignment) → owned String
                    return false; // needs borrowing
                }
                Type::Unknown => {
                    // Unknown type - use name heuristic as fallback
                }
                _ => {
                    // Other types - likely not a string key situation
                    return false;
                }
            }
        }

        // DEPYLER-0550: Removed "col" from heuristic - commonly used as loop variable
        // when iterating over dict items: for col, val in filters.items()
        // In that context, col is owned String from k.clone(), NOT a borrowed param
        // No type info or Unknown type - use name heuristics for function params
        // These are function parameters that typically become &str in Rust
        // Keep list minimal - only include names that are DEFINITELY function params
        let fn_param_names = matches!(var_name, "column" | "field" | "attr" | "property");

        if fn_param_names {
            return true;
        }

        // Variable not in var_types and not a known borrowed name
        // Default: assume needs borrowing (safer)
        false
    }

    /// DEPYLER-0496: Check if expression returns a Result type
    /// Used to determine if ? operator is needed in binary operations
    ///
    /// Returns true if:
    /// - Expression is a function call to a Result-returning function
    /// - Expression is a method call that might return Result
    ///
    /// # Complexity
    /// 2 (match + HashSet lookup)
    pub(crate) fn expr_returns_result(&self, expr: &HirExpr) -> bool {
        match expr {
            // Function calls: check if function is tracked as Result-returning
            HirExpr::Call { func, .. } => self.ctx.result_returning_functions.contains(func),
            // Method calls: Some method calls return Result (e.g., parse(), read_to_string())
            // For now, be conservative and don't assume method calls return Result
            // This can be enhanced later with specific method tracking
            HirExpr::MethodCall { .. } => false,
            // Other expressions don't return Result
            _ => false,
        }
    }

    /// DEPYLER-0575: Check if expression returns a float type
    /// Used to coerce integer literals to floats in comparisons
    pub(crate) fn expr_returns_float(&self, expr: &HirExpr) -> bool {
        match expr {
            // Float literals
            HirExpr::Literal(Literal::Float(_)) => true,
            // Variable with Float type, or variable from numpy float methods
            HirExpr::Var(name) => {
                // DEPYLER-1026: If we have explicit type info, use it exclusively
                // Don't fall through to heuristics when type is known
                if let Some(ty) = self.ctx.var_types.get(name) {
                    return matches!(ty, Type::Float);
                }
                // Common float result variable names from numpy operations
                // ONLY used when no type info is available
                // DEPYLER-0668: Remove "result" - too general, often used for ints/bools
                // DEPYLER-0927: Sync with expr_returns_f32 - include norm_a, norm_b, dot etc.
                // DEPYLER-0928: Added min_val, max_val for Vector-scalar operations
                matches!(
                    name.as_str(),
                    "mean" | "std" | "variance" | "sum" | "norm" | "norm_a" | "norm_b"
                        | "stddev" | "var" | "denom" | "dot" | "min_val" | "max_val"
                )
            }
            // DEPYLER-0577: Attribute access (e.g., args.x) - check if attr is float type
            // DEPYLER-0720: Also check class_field_types for self.X attribute access
            HirExpr::Attribute { attr, value, .. } => {
                // Check var_types first (for non-self attributes)
                if matches!(self.ctx.var_types.get(attr), Some(Type::Float)) {
                    return true;
                }
                // Check class_field_types for self.X patterns
                if matches!(value.as_ref(), HirExpr::Var(name) if name == "self")
                    && matches!(self.ctx.class_field_types.get(attr), Some(Type::Float))
                {
                    return true;
                }
                false
            }
            // NumPy/trueno methods that return f32
            // DEPYLER-0927: Added norm_l2 and dot for trueno compatibility
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "mean" | "sum" | "std" | "stddev" | "var" | "variance" | "min" | "max"
                        | "norm" | "norm_l2" | "dot"
                )
            }
            // DEPYLER-0799: Function calls - check return type from function_return_types
            // This handles cases like f(a) * f(b) > 0 where f returns float
            HirExpr::Call { func, .. } => {
                // Check module-level function return types first
                if let Some(ret_type) = self.ctx.function_return_types.get(func) {
                    if matches!(ret_type, Type::Float) {
                        return true;
                    }
                }
                // DEPYLER-0800: Check if func is a Callable parameter with Float return type
                // Example: f: Callable[[float], float] -> f(x) returns Float
                if let Some(Type::Function { ret, .. }) = self.ctx.var_types.get(func) {
                    if matches!(ret.as_ref(), Type::Float) {
                        return true;
                    }
                }
                // Callable is stored as Generic { base: "Callable", params: [param_types, return_type] }
                if let Some(Type::Generic { base, params }) = self.ctx.var_types.get(func) {
                    if base == "Callable" && params.len() == 2 && matches!(params[1], Type::Float) {
                        return true;
                    }
                }
                // Also check for math builtin functions that return float
                // DEPYLER-0816: Removed "abs" - Python abs() preserves input type (int→int, float→float)
                // The math functions below ALWAYS return float, but abs() is type-preserving
                matches!(
                    func.as_str(),
                    "sqrt" | "sin" | "cos" | "tan" | "exp" | "log" | "log10" | "log2"
                        | "floor"
                        | "ceil"
                        | "pow"
                        | "float"
                )
            }
            // DEPYLER-0694: Binary expression with float operand returns float
            // This handles chained operations like (principal * rate) * years
            HirExpr::Binary { left, right, .. } => {
                self.expr_returns_float(left)
                    || self.expr_returns_float(right)
                    || self.is_float_var(left)
                    || self.is_float_var(right)
            }
            _ => false,
        }
    }

    /// DEPYLER-0920: Check if expression returns f32 specifically (trueno/numpy results)
    /// Used to generate f32 literals instead of f64 in comparisons
    /// DEPYLER-0927: Synced with expr_returns_float for consistent detection
    pub(crate) fn expr_returns_f32(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable names commonly used for trueno f32 results
            HirExpr::Var(name) => {
                // DEPYLER-1026: If we have explicit type info, don't use heuristics
                if self.ctx.var_types.contains_key(name) {
                    return false; // f32 detection is only for trueno contexts without type info
                }
                matches!(
                    name.as_str(),
                    "mean" | "std" | "variance" | "sum" | "norm" | "norm_a" | "norm_b"
                        | "stddev" | "var" | "denom" | "dot"
                )
            }
            // Method calls on trueno Vectors return f32
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "mean" | "sum" | "std" | "stddev" | "var" | "variance" | "min" | "max"
                        | "norm" | "norm_l2" | "dot"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-1085: Check if expression returns DepylerValue type
    /// Used for Value Lifting in if/else branch unification
    pub(crate) fn expr_returns_depyler_value(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variables with Unknown type become DepylerValue
            HirExpr::Var(name) => {
                matches!(self.ctx.var_types.get(name), Some(Type::Unknown))
            }
            // Index access on collections with Unknown element type
            HirExpr::Index { base, .. } => {
                if let HirExpr::Var(name) = base.as_ref() {
                    if let Some(ty) = self.ctx.var_types.get(name) {
                        return match ty {
                            Type::List(elem) => matches!(elem.as_ref(), Type::Unknown),
                            Type::Dict(_, value) => matches!(value.as_ref(), Type::Unknown),
                            _ => false,
                        };
                    }
                }
                false
            }
            // Method calls on Unknown-typed objects
            HirExpr::MethodCall { object, .. } => {
                if let HirExpr::Var(name) = object.as_ref() {
                    return matches!(self.ctx.var_types.get(name), Some(Type::Unknown));
                }
                false
            }
            // IfExpr where either branch returns DepylerValue
            HirExpr::IfExpr { body, orelse, .. } => {
                self.expr_returns_depyler_value(body) || self.expr_returns_depyler_value(orelse)
            }
            _ => false,
        }
    }

    /// DEPYLER-1085: Check if expression returns a concrete (non-DepylerValue) type
    /// Returns the concrete type if known, None if Unknown/DepylerValue
    #[allow(dead_code)] // May be used for future value lifting optimizations
    pub(crate) fn expr_concrete_type(&self, expr: &HirExpr) -> Option<Type> {
        match expr {
            HirExpr::Literal(Literal::Int(_)) => Some(Type::Int),
            HirExpr::Literal(Literal::Float(_)) => Some(Type::Float),
            HirExpr::Literal(Literal::String(_)) => Some(Type::String),
            HirExpr::Literal(Literal::Bool(_)) => Some(Type::Bool),
            HirExpr::Var(name) => {
                self.ctx.var_types.get(name).and_then(|ty| {
                    if matches!(ty, Type::Unknown) {
                        None
                    } else {
                        Some(ty.clone())
                    }
                })
            }
            _ => None,
        }
    }

    /// DEPYLER-1085: Wrap a concrete expression in DepylerValue::from()
    /// Used for Value Lifting when branch types don't match
    pub(crate) fn lift_to_depyler_value(&self, expr: &HirExpr, rust_expr: syn::Expr) -> syn::Expr {
        // Use DepylerValue::from() which handles most common types
        // For specific types, use explicit variants for better performance
        match expr {
            HirExpr::Literal(Literal::Int(_)) => {
                parse_quote! { DepylerValue::Int(#rust_expr as i64) }
            }
            HirExpr::Literal(Literal::Float(_)) => {
                parse_quote! { DepylerValue::Float(#rust_expr as f64) }
            }
            HirExpr::Literal(Literal::String(_)) => {
                parse_quote! { DepylerValue::Str(#rust_expr.to_string()) }
            }
            HirExpr::Literal(Literal::Bool(_)) => {
                parse_quote! { DepylerValue::Bool(#rust_expr) }
            }
            HirExpr::Var(name) => {
                // Check the concrete type and wrap appropriately
                if let Some(ty) = self.ctx.var_types.get(name) {
                    match ty {
                        Type::Int => parse_quote! { DepylerValue::Int(#rust_expr as i64) },
                        Type::Float => parse_quote! { DepylerValue::Float(#rust_expr as f64) },
                        Type::String => parse_quote! { DepylerValue::Str(#rust_expr.to_string()) },
                        Type::Bool => parse_quote! { DepylerValue::Bool(#rust_expr) },
                        Type::List(_) => parse_quote! { DepylerValue::List(#rust_expr.into_iter().map(|x| DepylerValue::from(x)).collect()) },
                        _ => parse_quote! { DepylerValue::from(#rust_expr) },
                    }
                } else {
                    parse_quote! { DepylerValue::from(#rust_expr) }
                }
            }
            // For method calls that return known types
            _ if self.expr_returns_float(expr) => {
                parse_quote! { DepylerValue::Float(#rust_expr as f64) }
            }
            // Default: use DepylerValue::from() which requires Into trait
            _ => {
                parse_quote! { DepylerValue::Str(format!("{:?}", #rust_expr)) }
            }
        }
    }

    /// DEPYLER-0786: Check if expression is a string type
    /// Used to determine if `or` operator should return string instead of bool
    pub(crate) fn expr_is_string_type(&self, expr: &HirExpr) -> bool {
        match expr {
            // String literals
            HirExpr::Literal(Literal::String(_)) => true,
            // Variable with String type
            HirExpr::Var(name) => {
                matches!(self.ctx.var_types.get(name), Some(Type::String))
            }
            // Attribute access with String type
            HirExpr::Attribute { attr, .. } => {
                matches!(self.ctx.var_types.get(attr), Some(Type::String))
            }
            // Method calls that return strings
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "strip" | "lower" | "upper" | "replace" | "join" | "format"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-1127: Check if expression is a boolean-returning expression
    /// Used to determine if `or`/`and` should return bool or value
    ///
    /// Returns true for:
    /// - Boolean literals: True, False
    /// - Comparison expressions: a > b, a == b, etc.
    /// - Logical not: not x
    /// - Type checks: isinstance(), hasattr()
    /// - in/not in expressions
    ///
    /// Returns false for expressions that return non-boolean values
    pub(crate) fn expr_is_boolean_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Boolean literals always return bool
            HirExpr::Literal(Literal::Bool(_)) => true,
            // Comparison operators return bool
            HirExpr::Binary { op, .. } => matches!(
                op,
                BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq
                    | BinOp::In
                    | BinOp::NotIn
            ),
            // not x returns bool
            HirExpr::Unary {
                op: UnaryOp::Not, ..
            } => true,
            // isinstance, hasattr, callable return bool
            HirExpr::Call { func, .. } => {
                matches!(
                    func.as_str(),
                    "isinstance"
                        | "hasattr"
                        | "callable"
                        | "issubclass"
                        | "all"
                        | "any"
                        | "bool"
                )
            }
            // Method calls that return bool
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "startswith"
                        | "endswith"
                        | "isalpha"
                        | "isdigit"
                        | "isalnum"
                        | "isspace"
                        | "islower"
                        | "isupper"
                        | "istitle"
                        | "isidentifier"
                        | "isnumeric"
                        | "isdecimal"
                        | "isascii"
                        | "is_empty"
                        | "is_some"
                        | "is_none"
                        | "is_ok"
                        | "is_err"
                        | "contains"
                        | "exists"
                )
            }
            // Variables with Bool type
            HirExpr::Var(name) => matches!(self.ctx.var_types.get(name), Some(Type::Bool)),
            _ => false,
        }
    }

    /// DEPYLER-1127: Check if expression is DepylerValue type
    /// Used to determine if `or`/`and` needs DepylerValue wrapping
    pub(crate) fn expr_is_depyler_value(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variables with Unknown type default to DepylerValue
            HirExpr::Var(name) => {
                matches!(self.ctx.var_types.get(name), Some(Type::Unknown))
            }
            // Dict/List subscript often returns DepylerValue
            HirExpr::Index { base, .. } => {
                if let HirExpr::Var(name) = base.as_ref() {
                    // Dict access returns DepylerValue for heterogeneous dicts
                    matches!(
                        self.ctx.var_types.get(name),
                        Some(Type::Dict(_, _)) | Some(Type::Unknown)
                    )
                } else {
                    false
                }
            }
            // Method calls on collections that return DepylerValue
            HirExpr::MethodCall { object, method, .. } => {
                if method == "get" || method == "pop" || method == "cloned" {
                    // Check if object is a dict with DepylerValue values
                    if let HirExpr::Var(name) = object.as_ref() {
                        matches!(
                            self.ctx.var_types.get(name),
                            Some(Type::Dict(_, _)) | Some(Type::Unknown)
                        )
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-1127: Heuristic check if expression MIGHT return DepylerValue
    /// More permissive than expr_is_depyler_value - used to safely wrap literals
    /// in DepylerValue when there's uncertainty.
    ///
    /// Returns true for:
    /// - All cases from expr_is_depyler_value
    /// - Method chains with .get(), .cloned(), .unwrap_or() patterns
    /// - Variables not in var_types (unknown type)
    pub(crate) fn expr_might_be_depyler_value(&self, expr: &HirExpr) -> bool {
        // First check the strict version
        if self.expr_is_depyler_value(expr) {
            return true;
        }

        match expr {
            // Method chains that commonly return DepylerValue
            HirExpr::MethodCall { method, object, .. } => {
                // Check if this method typically returns uncertain types
                let uncertain_methods = matches!(
                    method.as_str(),
                    "get" | "cloned" | "unwrap_or" | "unwrap_or_default" | "pop" | "to_string"
                );
                if uncertain_methods {
                    return true;
                }
                // Recursively check the object
                self.expr_might_be_depyler_value(object)
            }
            // Variables not tracked in var_types - could be anything
            HirExpr::Var(name) => {
                !self.ctx.var_types.contains_key(name)
            }
            // Any subscript access could return DepylerValue
            HirExpr::Index { .. } => true,
            // Any attribute access on unknown objects
            HirExpr::Attribute { value, .. } => {
                self.expr_might_be_depyler_value(value)
            }
            _ => false,
        }
    }

    /// DEPYLER-0303 Phase 3 Fix #6: Check if expression is an owned collection
    /// Used to determine if zip() should use .into_iter() (owned) vs .iter() (borrowed)
    ///
    /// Returns true if:
    /// - Expression is a Var with type List (Vec<T>) - function parameters are owned
    /// - Expression is a list literal - always owned
    /// - Expression is a list() call - creates owned Vec
    ///
    /// # Complexity
    /// 3 (match + type lookup + variant check)
    pub(crate) fn is_owned_collection(&self, expr: &HirExpr) -> bool {
        match expr {
            // List literals are always owned
            HirExpr::List(_) => true,
            // list() calls create owned Vec
            HirExpr::Call { func, .. } if func == "list" => true,
            // Check if variable has List type (function parameters of type Vec<T>)
            HirExpr::Var(name) => {
                if let Some(ty) = self.ctx.var_types.get(name) {
                    matches!(ty, Type::List(_))
                } else {
                    // No type info - conservative default is borrowed
                    false
                }
            }
            _ => false,
        }
    }

    /// Check if an expression is a user-defined class instance
    pub(crate) fn is_class_instance(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a user-defined class
                if let Some(Type::Custom(class_name)) = self.ctx.var_types.get(name) {
                    // Check if this is a user-defined class (not a builtin)
                    self.ctx.class_names.contains(class_name)
                } else {
                    false
                }
            }
            HirExpr::Call { func, .. } => {
                // Direct constructor call like Calculator(10)
                self.ctx.class_names.contains(func)
            }
            _ => false,
        }
    }

    // DEPYLER-REFACTOR-001: is_bool_expr moved to builtin_conversions module

    pub(crate) fn convert_set_operation(
        &self,
        op: BinOp,
        left: syn::Expr,
        right: syn::Expr,
    ) -> Result<syn::Expr> {
        // DEPYLER-0412: Add explicit type annotation to collect() for set operations
        match op {
            BinOp::BitAnd => Ok(parse_quote! {
                #left.intersection(&#right).cloned().collect::<std::collections::HashSet<_>>()
            }),
            BinOp::BitOr => Ok(parse_quote! {
                #left.union(&#right).cloned().collect::<std::collections::HashSet<_>>()
            }),
            BinOp::Sub => Ok(parse_quote! {
                #left.difference(&#right).cloned().collect::<std::collections::HashSet<_>>()
            }),
            BinOp::BitXor => Ok(parse_quote! {
                #left.symmetric_difference(&#right).cloned().collect::<std::collections::HashSet<_>>()
            }),
            _ => bail!("Invalid set operator"),
        }
    }

    pub(crate) fn convert_set_comp(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // DEPYLER-0504: Support multiple generators in set comprehensions
        // Same pattern as convert_list_comp but collecting to HashSet

        self.ctx.needs_hashset = true;

        if generators.is_empty() {
            bail!("Set comprehension must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let element_expr = element.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr = if !matches!(&*gen.iter, HirExpr::Var(_)) {
                self.wrap_range_in_parens(iter_expr)
            } else {
                iter_expr
            };

            let mut chain: syn::Expr = if self.is_numpy_array_expr(&gen.iter) {
                // DEPYLER-0575: trueno Vector uses .as_slice().iter()
                parse_quote! { #iter_expr.as_slice().iter().copied() }
            } else if matches!(&*gen.iter, HirExpr::Var(_)) {
                // DEPYLER-0674: Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                parse_quote! { #iter_expr.iter().cloned() }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-0691: Add filters for each condition
            // DEPYLER-0820: Use |pattern| not |&pattern| - after .cloned() values are owned,
            // filter() receives &Item, using |pattern| binds as references avoiding E0507
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr }) };
            }

            // Add the map transformation
            chain = parse_quote! { #chain.map(|#target_pat| #element_expr) };

            // Collect into HashSet
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            return Ok(parse_quote! { #chain.collect::<std::collections::HashSet<_>>() });
        }

        // Multiple generators case (nested iteration with flat_map)
        let chain = self.convert_nested_generators_for_list_comp(element, generators)?;
        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        Ok(parse_quote! { #chain.collect::<std::collections::HashSet<_>>() })
    }

    pub(crate) fn convert_dict_comp(
        &mut self,
        key: &HirExpr,
        value: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // DEPYLER-0504: Support multiple generators in dict comprehensions
        // Same pattern as convert_list_comp but collecting to HashMap with (key, value) tuples

        self.ctx.needs_hashmap = true;

        if generators.is_empty() {
            bail!("Dict comprehension must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let key_expr = key.to_rust_expr(self.ctx)?;
            let value_expr = value.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr = if !matches!(&*gen.iter, HirExpr::Var(_)) {
                self.wrap_range_in_parens(iter_expr)
            } else {
                iter_expr
            };

            // DEPYLER-0955: Dict comprehensions iterate over tuples which may contain String
            // (e.g., {k: v for k, v in items} where items is List[(str, int)])
            // Tuples with String don't implement Copy, so always use .cloned() for dict comp
            // This avoids the "Copy is not satisfied for String" error with .copied()
            let mut chain: syn::Expr = if matches!(&*gen.iter, HirExpr::Var(_)) {
                // Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                parse_quote! { #iter_expr.iter().cloned() }
            } else {
                // Direct expression (list literals, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-0691: Add filters for each condition
            // DEPYLER-0820: Use |pattern| not |&pattern| - after .cloned() values are owned,
            // filter() receives &Item, using |pattern| binds as references avoiding E0507
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr }) };
            }

            // DEPYLER-0706: Add the map transformation (to key-value tuple)
            // Compute value before key to avoid borrow-after-move when value_expr
            // references the key variable (e.g., {word: len(word) for word in words})
            chain = parse_quote! { #chain.map(|#target_pat| { let _v = #value_expr; (#key_expr, _v) }) };

            // DEPYLER-0685: Use fully qualified path for HashMap to avoid import issues
            return Ok(parse_quote! { #chain.collect::<std::collections::HashMap<_, _>>() });
        }

        // Multiple generators case (nested iteration with flat_map)
        // Build nested chain that generates (key, value) tuples
        let chain = self.convert_nested_generators_for_dict_comp(key, value, generators)?;
        // DEPYLER-0685: Use fully qualified path for HashMap
        Ok(parse_quote! { #chain.collect::<std::collections::HashMap<_, _>>() })
    }

    pub(crate) fn convert_nested_generators_for_dict_comp(
        &mut self,
        key: &HirExpr,
        value: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Start with the outermost generator
        let first_gen = &generators[0];
        let first_iter = first_gen.iter.to_rust_expr(self.ctx)?;
        let first_pat = self.parse_target_pattern(&first_gen.target)?;

        // Build nested chain that produces (key, value) tuples
        let inner_expr = self.build_nested_chain_for_dict(key, value, generators, 1)?;

        // Start the chain with the first generator
        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let first_iter = self.wrap_range_in_parens(first_iter);
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // DEPYLER-0691: Add filters for first generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        for cond in &first_gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#first_pat| #cond_expr) };
        }

        // Use flat_map for the first generator
        chain = parse_quote! { #chain.flat_map(|#first_pat| #inner_expr) };

        Ok(chain)
    }

    pub(crate) fn build_nested_chain_for_dict(
        &mut self,
        key: &HirExpr,
        value: &HirExpr,
        generators: &[crate::hir::HirComprehension],
        depth: usize,
    ) -> Result<syn::Expr> {
        if depth >= generators.len() {
            // Base case: no more generators, return (key, value) tuple
            let key_expr = key.to_rust_expr(self.ctx)?;
            let value_expr = value.to_rust_expr(self.ctx)?;
            // DEPYLER-0706: Compute value before key to avoid borrow-after-move
            return Ok(parse_quote! { std::iter::once({ let _v = #value_expr; (#key_expr, _v) }) });
        }

        // Recursive case: process current generator
        let gen = &generators[depth];
        let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
        let target_pat = self.parse_target_pattern(&gen.target)?;

        // Build inner chain recursively
        let inner_chain = self.build_nested_chain_for_dict(key, value, generators, depth + 1)?;

        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let iter_expr = self.wrap_range_in_parens(iter_expr);

        // Start with iterator
        let mut chain: syn::Expr = parse_quote! { #iter_expr.into_iter() };

        // DEPYLER-0691: Add filters for current generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        for cond in &gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
        }

        // Use flat_map to nest the inner chain
        chain = parse_quote! { #chain.flat_map(move |#target_pat| #inner_chain) };

        Ok(chain)
    }

    /// DEPYLER-1117: Infer lambda parameter type from body expression
    ///
    /// Analyzes the body to determine what type a parameter should be:
    /// - If parameter.iter() is called -> Vec<i64> (iterable)
    /// - If parameter is used directly in PyOps (py_add, py_mul, etc.) -> i64
    /// - Default -> i64 (most common in Python numeric code)
    fn infer_lambda_param_type(&self, param: &str, body: &HirExpr) -> Option<syn::Type> {
        // DEPYLER-1117: Check iterator methods FIRST - if param.iter() is called,
        // it's a collection, not a scalar
        if self.body_uses_iter_on_param(param, body) {
            return Some(parse_quote! { Vec<i64> });
        }

        // DEPYLER-1130: Check if parameter is used as a boolean condition
        // Pattern: `lambda is_add: (expr) if is_add else (expr)` → is_add: bool
        if self.param_used_as_condition(param, body) {
            return Some(parse_quote! { bool });
        }

        // Check if parameter is used directly in PyOps (not in nested lambdas)
        if self.param_directly_in_pyops(param, body) {
            return Some(parse_quote! { i64 });
        }

        // Default to i64 for standalone lambdas to resolve E0282
        // This is safe because PyOps traits are implemented for i64
        Some(parse_quote! { i64 })
    }

    /// DEPYLER-1130: Check if parameter is used as a boolean condition
    /// Detects patterns like `if param:` or `if param else` in lambda body
    fn param_used_as_condition(&self, param: &str, body: &HirExpr) -> bool {
        match body {
            // Direct use as condition: `(expr) if param else (expr)`
            HirExpr::IfExpr { test, body, orelse } => {
                // Check if param IS the test condition (not just contained in it)
                if self.is_direct_var(param, test) {
                    return true;
                }
                // Also recurse to check nested conditionals
                self.param_used_as_condition(param, body)
                    || self.param_used_as_condition(param, orelse)
            }
            // Check in nested expressions
            HirExpr::Binary { left, right, .. } => {
                self.param_used_as_condition(param, left)
                    || self.param_used_as_condition(param, right)
            }
            HirExpr::Call { args, .. } => args
                .iter()
                .any(|arg| self.param_used_as_condition(param, arg)),
            HirExpr::MethodCall { object, args, .. } => {
                self.param_used_as_condition(param, object)
                    || args
                        .iter()
                        .any(|arg| self.param_used_as_condition(param, arg))
            }
            _ => false,
        }
    }

    /// Check if parameter is used DIRECTLY in a binary operation (not nested in closures)
    fn param_directly_in_pyops(&self, param: &str, body: &HirExpr) -> bool {
        match body {
            // Direct binary operation with the parameter
            HirExpr::Binary { left, right, .. } => {
                self.is_direct_var(param, left) || self.is_direct_var(param, right)
            }
            // Direct method call on the parameter (but not iter/map/filter)
            HirExpr::MethodCall { object, method, .. } => {
                // Skip iterator methods - those indicate collection type
                if matches!(method.as_str(), "iter" | "into_iter" | "map" | "filter" | "cloned") {
                    return false;
                }
                self.is_direct_var(param, object)
            }
            // Ternary expression - check all branches
            HirExpr::IfExpr {
                test,
                body: if_body,
                orelse,
            } => {
                self.param_directly_in_pyops(param, test)
                    || self.param_directly_in_pyops(param, if_body)
                    || self.param_directly_in_pyops(param, orelse)
            }
            _ => false,
        }
    }

    /// Check if expr is directly the variable (not nested)
    fn is_direct_var(&self, var_name: &str, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Var(name) if name == var_name)
    }

    /// Check if body uses the parameter as an iterable (list comprehension, for loop, iter() call)
    fn body_uses_iter_on_param(&self, param: &str, body: &HirExpr) -> bool {
        match body {
            // DEPYLER-1117: List comprehension `[x for x in param]` - param is iterable
            HirExpr::ListComp { generators, .. } => {
                generators.iter().any(|gen| self.is_direct_var(param, &gen.iter))
            }
            // Set comprehension `{x for x in param}` - param is iterable
            HirExpr::SetComp { generators, .. } => {
                generators.iter().any(|gen| self.is_direct_var(param, &gen.iter))
            }
            // Dict comprehension `{k: v for k, v in param}` - param is iterable
            HirExpr::DictComp { generators, .. } => {
                generators.iter().any(|gen| self.is_direct_var(param, &gen.iter))
            }
            // Generator expression `(x for x in param)` - param is iterable
            HirExpr::GeneratorExp { generators, .. } => {
                generators.iter().any(|gen| self.is_direct_var(param, &gen.iter))
            }
            // Method call - check for iter() or chained iterator methods
            HirExpr::MethodCall { object, method, .. } => {
                // Check if this is an iterator method call directly on the parameter
                if matches!(method.as_str(), "iter" | "into_iter") {
                    return self.is_direct_var(param, object);
                }
                // Also check if this is a chained call like param.iter().map()...
                if let HirExpr::MethodCall {
                    object: inner_obj,
                    method: inner_method,
                    ..
                } = object.as_ref()
                {
                    if matches!(inner_method.as_str(), "iter" | "into_iter") {
                        return self.is_direct_var(param, inner_obj);
                    }
                }
                false
            }
            // Check in nested expressions
            HirExpr::Call { args, .. } => {
                args.iter().any(|arg| self.body_uses_iter_on_param(param, arg))
            }
            _ => false,
        }
    }

    /// Check if expression contains a variable reference
    fn expr_contains_var(&self, var_name: &str, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => name == var_name,
            HirExpr::Binary { left, right, .. } => {
                self.expr_contains_var(var_name, left) || self.expr_contains_var(var_name, right)
            }
            HirExpr::Unary { operand, .. } => self.expr_contains_var(var_name, operand),
            HirExpr::Call { args, .. } => {
                args.iter().any(|arg| self.expr_contains_var(var_name, arg))
            }
            HirExpr::MethodCall { object, args, .. } => {
                self.expr_contains_var(var_name, object)
                    || args.iter().any(|arg| self.expr_contains_var(var_name, arg))
            }
            HirExpr::Index { base, index, .. } => {
                self.expr_contains_var(var_name, base) || self.expr_contains_var(var_name, index)
            }
            HirExpr::Attribute { value, .. } => self.expr_contains_var(var_name, value),
            HirExpr::IfExpr {
                test, body, orelse, ..
            } => {
                self.expr_contains_var(var_name, test)
                    || self.expr_contains_var(var_name, body)
                    || self.expr_contains_var(var_name, orelse)
            }
            HirExpr::Tuple(elems) | HirExpr::List(elems) => {
                elems.iter().any(|e| self.expr_contains_var(var_name, e))
            }
            _ => false,
        }
    }

    pub(crate) fn convert_lambda(&mut self, params: &[String], body: &HirExpr) -> Result<syn::Expr> {
        // CITL: Trace lambda/closure conversion decision
        trace_decision!(
            category = DecisionCategory::Ownership,
            name = "lambda_closure",
            chosen = "closure",
            alternatives = ["fn_pointer", "closure_move", "closure_ref", "boxed_fn"],
            confidence = 0.87
        );

        // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
        // DEPYLER-1117: Add type annotations to fix E0282 errors
        // Parameters are typed based on body expression analysis
        let param_tokens: Vec<proc_macro2::TokenStream> = params
            .iter()
            .map(|p| {
                let ident = crate::rust_gen::keywords::safe_ident(p);
                // DEPYLER-1117: Infer type from body usage
                if let Some(ty) = self.infer_lambda_param_type(p, body) {
                    quote::quote! { #ident: #ty }
                } else {
                    quote::quote! { #ident }
                }
            })
            .collect();

        // Convert body expression
        let body_expr = body.to_rust_expr(self.ctx)?;

        // Generate closure
        // DEPYLER-0837: Use `move` closures to match Python's closure semantics
        // Python closures capture variables by reference but extend their lifetime
        // Rust requires `move` when returning closures that capture local variables
        if params.is_empty() {
            // No parameters
            Ok(parse_quote! { move || #body_expr })
        } else if params.len() == 1 {
            // Single parameter with type annotation
            let param = &param_tokens[0];
            Ok(parse_quote! { move |#param| #body_expr })
        } else {
            // Multiple parameters with type annotations
            Ok(parse_quote! { move |#(#param_tokens),*| #body_expr })
        }
    }

    /// Check if an expression is a len() call
    pub(crate) fn is_len_call(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Call { func, args , ..} if func == "len" && args.len() == 1)
    }

    pub(crate) fn convert_await(&mut self, value: &HirExpr) -> Result<syn::Expr> {
        let value_expr = value.to_rust_expr(self.ctx)?;
        // DEPYLER-1024: In NASA mode, strip .await since async is converted to sync
        if self.ctx.type_mapper.nasa_mode {
            Ok(value_expr)
        } else {
            Ok(parse_quote! { #value_expr.await })
        }
    }

    pub(crate) fn convert_yield(&mut self, value: &Option<Box<HirExpr>>) -> Result<syn::Expr> {
        if self.ctx.in_generator {
            // Inside Iterator::next() - convert to return Some(value)
            if let Some(v) = value {
                let value_expr = v.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { return Some(#value_expr) })
            } else {
                Ok(parse_quote! { return None })
            }
        } else {
            // Outside generator context - keep as yield (placeholder for future)
            if let Some(v) = value {
                let value_expr = v.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { yield #value_expr })
            } else {
                Ok(parse_quote! { yield })
            }
        }
    }

    pub(crate) fn convert_fstring(&mut self, parts: &[FStringPart]) -> Result<syn::Expr> {
        // Handle empty f-strings
        if parts.is_empty() {
            return Ok(parse_quote! { "".to_string() });
        }

        // Check if it's just a plain string (no expressions)
        let has_expressions = parts.iter().any(|p| matches!(p, FStringPart::Expr(_)));

        if !has_expressions {
            // Just literal parts - concatenate them
            let mut result = String::new();
            for part in parts {
                if let FStringPart::Literal(s) = part {
                    result.push_str(s);
                }
            }
            return Ok(parse_quote! { #result.to_string() });
        }

        // Build format string template and collect arguments
        let mut template = String::new();
        let mut args = Vec::new();

        for part in parts {
            match part {
                FStringPart::Literal(s) => {
                    template.push_str(s);
                }
                FStringPart::Expr(expr) => {
                    // DEPYLER-0438/0441/0446: Smart formatting based on expression type
                    // - Collections (Vec, HashMap, HashSet): Use {:?} debug formatting
                    // - Scalars (String, i32, f64, bool): Use {} Display formatting
                    // - Option types: Unwrap with .unwrap_or_default() or display "None"
                    // This matches Python semantics where lists/dicts have their own repr
                    let arg_expr = expr.to_rust_expr(self.ctx)?;

                    // DEPYLER-0446: Check if this is an argparse Option<T> field (should be wrapped to String)
                    let is_argparse_option = match expr.as_ref() {
                        HirExpr::Attribute { value, attr } => {
                            if let HirExpr::Var(obj_name) = value.as_ref() {
                                let is_args_var = self.ctx.argparser_tracker.parsers.values().any(
                                    |parser_info| {
                                        parser_info
                                            .args_var
                                            .as_ref()
                                            .is_some_and(|args_var| args_var == obj_name)
                                    },
                                );

                                if is_args_var {
                                    // Check if this argument is optional (Option<T> type, not boolean)
                                    self.ctx
                                        .argparser_tracker
                                        .parsers
                                        .values()
                                        .any(|parser_info| {
                                            parser_info.arguments.iter().any(|arg| {
                                                let field_name = arg.rust_field_name();
                                                if field_name != *attr {
                                                    return false;
                                                }

                                                // Argument is NOT an Option if it has action="store_true" or "store_false"
                                                if matches!(
                                                    arg.action.as_deref(),
                                                    Some("store_true") | Some("store_false")
                                                ) {
                                                    return false;
                                                }

                                                // Argument is an Option<T> if: not required AND no default value AND not positional
                                                !arg.is_positional
                                                    && !arg.required.unwrap_or(false)
                                                    && arg.default.is_none()
                                            })
                                        })
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    // DEPYLER-0497: Determine if expression needs {:?} Debug formatting
                    // Required for: collections, Result, Option, Vec, and any non-Display type
                    let needs_debug_fmt = match expr.as_ref() {
                        // Case 1: Simple variable (e.g., targets)
                        HirExpr::Var(var_name) => {
                            if let Some(var_type) = self.ctx.var_types.get(var_name) {
                                // Known type: check if it needs Debug formatting
                                // DEPYLER-0712: Added Tuple - tuples don't implement Display
                                matches!(
                                    var_type,
                                    Type::List(_)
                                        | Type::Dict(_, _)
                                        | Type::Set(_)
                                        | Type::Tuple(_)     // DEPYLER-0712: Tuples need {:?}
                                        | Type::Optional(_) // DEPYLER-0497: Options need {:?}
                                )
                            } else {
                                // DEPYLER-0497 WORKAROUND: Unknown type - default to {:?} (defensive)
                                // This is safer because Debug is more universally implemented than Display
                                // Most types implement Debug: Option<T>, Result<T,E>, Vec<T>, primitives
                                // Only a few types need Display: i32, String, etc (which also have Debug)
                                // This prevents E0277 errors for Option/Result/Vec variables
                                true
                            }
                        }
                        // DEPYLER-0497: Function calls that return Result<T> OR Option<T> need {:?}
                        HirExpr::Call { func, .. } => {
                            self.ctx.result_returning_functions.contains(func)
                                || self.ctx.option_returning_functions.contains(func)
                        }
                        // DEPYLER-0519: Method calls that return Vec types need {:?}
                        HirExpr::MethodCall { method, .. } => {
                            let vec_returning_methods = [
                                "groups",
                                "split",
                                "split_whitespace",
                                "splitlines",
                                "findall",
                                "keys",
                                "values",
                                "items",
                            ];
                            vec_returning_methods.contains(&method.as_str())
                        }
                        // Case 2: Attribute access (e.g., args.targets)
                        HirExpr::Attribute { value, attr } => {
                            // Check if this is accessing a field from argparse Args struct
                            if let HirExpr::Var(obj_name) = value.as_ref() {
                                // Check if obj_name is the args variable from ArgumentParser
                                let is_args_var = self.ctx.argparser_tracker.parsers.values().any(
                                    |parser_info| {
                                        parser_info
                                            .args_var
                                            .as_ref()
                                            .is_some_and(|args_var| args_var == obj_name)
                                    },
                                );

                                if is_args_var {
                                    // Look up the field type in argparse arguments
                                    self.ctx
                                        .argparser_tracker
                                        .parsers
                                        .values()
                                        .any(|parser_info| {
                                            parser_info.arguments.iter().any(|arg| {
                                                // Match field name (normalized from Python argument name)
                                                let field_name = arg.rust_field_name();
                                                if field_name == *attr {
                                                    // Check if this field is a collection type
                                                    // Either explicit type annotation OR inferred from nargs
                                                    let is_vec_from_nargs = matches!(
                                                        arg.nargs.as_deref(),
                                                        Some("+") | Some("*")
                                                    );
                                                    let is_collection_type =
                                                        if let Some(ref arg_type) = arg.arg_type {
                                                            matches!(
                                                                arg_type,
                                                                Type::List(_)
                                                                    | Type::Dict(_, _)
                                                                    | Type::Set(_)
                                                            )
                                                        } else {
                                                            false
                                                        };
                                                    is_vec_from_nargs || is_collection_type
                                                } else {
                                                    false
                                                }
                                            })
                                        })
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    // DEPYLER-0446: Wrap argparse Option types to handle Display trait
                    // Only wrap argparse Optional fields, not regular Option variables
                    // DEPYLER-0930: Check if expression is a PathBuf type that needs .display()
                    // PathBuf doesn't implement Display, so we need to call .display() to format it
                    let is_pathbuf = match expr.as_ref() {
                        HirExpr::Var(var_name) => self
                            .ctx
                            .var_types
                            .get(var_name)
                            .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                            .unwrap_or(false),
                        HirExpr::MethodCall { method, .. } => {
                            // Methods that return PathBuf
                            matches!(
                                method.as_str(),
                                "parent" | "with_name" | "with_suffix" | "with_stem" | "join"
                            )
                        }
                        HirExpr::Attribute { value, attr } => {
                            // path.parent returns PathBuf
                            if matches!(attr.as_str(), "parent") {
                                if let HirExpr::Var(var_name) = value.as_ref() {
                                    self.ctx
                                        .var_types
                                        .get(var_name)
                                        .map(|t| {
                                            matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path")
                                        })
                                        .unwrap_or(false)
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    let final_arg = if is_argparse_option {
                        // Argparse Option<T> should display as value or "None" string
                        parse_quote! {
                            {
                                match &#arg_expr {
                                    Some(v) => format!("{}", v),
                                    None => "None".to_string(),
                                }
                            }
                        }
                    } else if is_pathbuf {
                        // DEPYLER-0930: PathBuf needs .display() to implement Display
                        parse_quote! { #arg_expr.display() }
                    } else {
                        arg_expr
                    };

                    // DEPYLER-0497: Use {:?} for non-Display types (Result, Vec, collections, Option)
                    // Use {} for Display types (primitives, String, wrapped argparse Options)
                    // DEPYLER-0930: PathBuf with .display() can use {} (Display trait)
                    if is_argparse_option || is_pathbuf {
                        // Argparse Option was wrapped to String, PathBuf has .display(), use {}
                        template.push_str("{}");
                    } else if needs_debug_fmt {
                        // Non-Display types (Vec, Result, Option, collections) need {:?}
                        template.push_str("{:?}");
                    } else {
                        // Regular Display types (i32, String, etc.)
                        template.push_str("{}");
                    }

                    args.push(final_arg);
                }
            }
        }

        // Generate format!() macro call
        if args.is_empty() {
            // No arguments (shouldn't happen but be safe)
            Ok(parse_quote! { #template.to_string() })
        } else {
            // Build the format! call with template and arguments
            Ok(parse_quote! { format!(#template, #(#args),*) })
        }
    }

    pub(crate) fn convert_ifexpr(
        &mut self,
        test: &HirExpr,
        body: &HirExpr,
        orelse: &HirExpr,
    ) -> Result<syn::Expr> {
        // DEPYLER-0377: Optimize `x if x else default` pattern
        // Python: `args.include if args.include else []` (check if list is non-empty)
        // Rust: Just `args.include` (clap initializes Vec to empty, so redundant check)
        // This pattern is common with argparse + Vec/Option fields
        if test == body {
            // Pattern: `x if x else y` → just use `x` (the condition is redundant)
            // This avoids type errors where Vec/Option can't be used as bool
            return body.to_rust_expr(self.ctx);
        }

        // DEPYLER-1071: Handle Option variable ternary with method call on Option
        // Pattern: `option_var.method() if option_var else default`
        // Python: `m.group(0) if m else None` where m = re.search(...)
        // Rust: `if let Some(ref m_val) = m { Some(m_val.group(0)) } else { None }`
        if let HirExpr::Var(var_name) = test {
            let is_option_var = self.is_option_variable(var_name);
            if is_option_var {
                // Check if body uses this variable in a method call
                if self.body_uses_option_var_method(body, var_name) {
                    return self.generate_option_if_let_expr(var_name, body, orelse);
                }
            }
        }

        let mut test_expr = test.to_rust_expr(self.ctx)?;
        let body_expr = body.to_rust_expr(self.ctx)?;
        let orelse_expr = orelse.to_rust_expr(self.ctx)?;

        // DEPYLER-0377: Apply Python truthiness conversion to ternary expressions
        // Python: `val if val else default` where val is String/List/Dict/Set/Optional/Int/Float
        // Without conversion: `if val` fails (expected bool, found Vec/String/etc)
        // With conversion: `if !val.is_empty()` / `if val.is_some()` / `if val != 0`
        test_expr = Self::apply_truthiness_conversion(test, test_expr, self.ctx);

        // DEPYLER-0544: Detect File vs Stdout type mismatch
        // Python: `open(path, "w") if path else sys.stdout`
        // Rust: Needs Box<dyn Write> to unify File and Stdout types
        let body_is_file = self.is_file_creating_expr(body);
        let orelse_is_stdout = self.is_stdout_expr(orelse);
        let orelse_is_file = self.is_file_creating_expr(orelse);
        let body_is_stdout = self.is_stdout_expr(body);

        if (body_is_file && orelse_is_stdout) || (body_is_stdout && orelse_is_file) {
            // Wrap both sides in Box::new() for trait object unification
            return Ok(parse_quote! {
                if #test_expr { Box::new(#body_expr) as Box<dyn std::io::Write> } else { Box::new(#orelse_expr) }
            });
        }

        // DEPYLER-0927: Type unification for numeric IfExpr branches
        // When body returns float and orelse is integer literal, coerce orelse to float
        // Example: `dot / (norm_a * norm_b) if cond else 0` → `... else 0.0`
        let body_is_float = self.expr_returns_float(body);
        let body_is_f32 = self.expr_returns_f32(body);
        let orelse_is_int_literal = matches!(orelse, HirExpr::Literal(Literal::Int(_)));

        if body_is_float && orelse_is_int_literal {
            if let HirExpr::Literal(Literal::Int(n)) = orelse {
                let coerced_orelse: syn::Expr = if body_is_f32 {
                    let float_val = *n as f32;
                    parse_quote! { #float_val }
                } else {
                    let float_val = *n as f64;
                    parse_quote! { #float_val }
                };
                return Ok(parse_quote! {
                    if #test_expr { #body_expr } else { #coerced_orelse }
                });
            }
        }

        // DEPYLER-1085: Value Lifting for DepylerValue/concrete type mismatches
        // When one branch yields DepylerValue and the other a concrete type,
        // wrap the concrete branch in DepylerValue to unify types
        let body_is_depyler_value = self.expr_returns_depyler_value(body);
        let orelse_is_depyler_value = self.expr_returns_depyler_value(orelse);

        if body_is_depyler_value && !orelse_is_depyler_value {
            // Body is DepylerValue, orelse is concrete - lift orelse
            let lifted_orelse = self.lift_to_depyler_value(orelse, orelse_expr);
            return Ok(parse_quote! {
                if #test_expr { #body_expr } else { #lifted_orelse }
            });
        }

        if !body_is_depyler_value && orelse_is_depyler_value {
            // Orelse is DepylerValue, body is concrete - lift body
            let lifted_body = self.lift_to_depyler_value(body, body_expr);
            return Ok(parse_quote! {
                if #test_expr { #lifted_body } else { #orelse_expr }
            });
        }

        Ok(parse_quote! {
            if #test_expr { #body_expr } else { #orelse_expr }
        })
    }

    /// DEPYLER-0544: Check if expression creates a File (open() or File::create())
    pub(crate) fn is_file_creating_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Call { func: Symbol, .. } - func is a simple function name like "open"
            HirExpr::Call { func, .. } => {
                // Check for open() builtin
                func == "open"
            }
            // MethodCall { object, method, .. } - e.g., File.create()
            HirExpr::MethodCall { object, method, .. } => {
                if method == "create" || method == "open" {
                    if let HirExpr::Var(name) = object.as_ref() {
                        return name == "File";
                    }
                    // std.fs.File.create()
                    if let HirExpr::Attribute { attr, .. } = object.as_ref() {
                        return attr == "File";
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0544: Check if expression is sys.stdout
    pub(crate) fn is_stdout_expr(&self, expr: &HirExpr) -> bool {
        if let HirExpr::Attribute { value, attr } = expr {
            if attr == "stdout" {
                if let HirExpr::Var(name) = value.as_ref() {
                    return name == "sys";
                }
            }
        }
        false
    }

    /// DEPYLER-1071: Check if a variable is an Option type (by type tracking or name heuristic)
    fn is_option_variable(&self, var_name: &str) -> bool {
        // First check if we have type info
        if let Some(var_type) = self.ctx.var_types.get(var_name) {
            if matches!(var_type, Type::Optional(_)) {
                return true;
            }
        }
        // Fall back to name heuristic for regex match results
        is_option_var_name(var_name)
    }

    /// DEPYLER-1071: Check if the body expression uses the given variable in a method call
    /// This detects patterns like `m.group(0)` where m is the Option variable
    fn body_uses_option_var_method(&self, body: &HirExpr, var_name: &str) -> bool {
        match body {
            // Direct method call on the variable: m.group(0)
            HirExpr::MethodCall { object, .. } => {
                if let HirExpr::Var(obj_name) = object.as_ref() {
                    return obj_name == var_name;
                }
                // Check nested method calls
                self.body_uses_option_var_method(object, var_name)
            }
            // Attribute access on the variable
            HirExpr::Attribute { value, .. } => {
                if let HirExpr::Var(obj_name) = value.as_ref() {
                    return obj_name == var_name;
                }
                self.body_uses_option_var_method(value, var_name)
            }
            // Variable used directly in body
            HirExpr::Var(name) => name == var_name,
            _ => false,
        }
    }

    /// DEPYLER-1071: Generate `if let Some(ref val) = option_var { body } else { orelse }`
    /// with the option variable replaced by the unwrapped val in the body
    fn generate_option_if_let_expr(
        &mut self,
        var_name: &str,
        body: &HirExpr,
        orelse: &HirExpr,
    ) -> Result<syn::Expr> {
        let var_ident = keywords::safe_ident(var_name);
        let val_name = format!("{}_val", var_name);
        let val_ident = keywords::safe_ident(&val_name);

        // Create a temporary context with the unwrapped variable name
        // We'll transform the body to use the unwrapped value
        let body_with_substitution = self.substitute_var_in_expr(body, var_name, &val_name);
        let body_expr = body_with_substitution.to_rust_expr(self.ctx)?;
        let orelse_expr = orelse.to_rust_expr(self.ctx)?;

        // Check if orelse is None - if so, use Option::map pattern
        if matches!(orelse, HirExpr::Literal(Literal::None)) {
            // Pattern: `x.method() if x else None` → `x.map(|x_val| x_val.method())`
            Ok(parse_quote! {
                #var_ident.as_ref().map(|#val_ident| #body_expr)
            })
        } else {
            // Pattern: `x.method() if x else default` → `if let Some(ref x_val) = x { body } else { orelse }`
            Ok(parse_quote! {
                if let Some(ref #val_ident) = #var_ident { #body_expr } else { #orelse_expr }
            })
        }
    }

    /// DEPYLER-1071: Recursively substitute a variable name in an expression
    fn substitute_var_in_expr(&self, expr: &HirExpr, old_name: &str, new_name: &str) -> HirExpr {
        match expr {
            HirExpr::Var(name) if name == old_name => HirExpr::Var(new_name.to_string()),
            HirExpr::MethodCall {
                object,
                method,
                args,
                kwargs,
            } => HirExpr::MethodCall {
                object: Box::new(self.substitute_var_in_expr(object, old_name, new_name)),
                method: method.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_var_in_expr(a, old_name, new_name))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| (k.clone(), self.substitute_var_in_expr(v, old_name, new_name)))
                    .collect(),
            },
            HirExpr::Attribute { value, attr } => HirExpr::Attribute {
                value: Box::new(self.substitute_var_in_expr(value, old_name, new_name)),
                attr: attr.clone(),
            },
            HirExpr::Call { func, args, kwargs } => HirExpr::Call {
                func: func.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_var_in_expr(a, old_name, new_name))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| (k.clone(), self.substitute_var_in_expr(v, old_name, new_name)))
                    .collect(),
            },
            // For other expression types, return as-is (could be extended if needed)
            _ => expr.clone(),
        }
    }

    /// Apply Python truthiness conversion to non-boolean conditions
    /// Python: `if val:` where val is String/List/Dict/Set/Optional/Int/Float
    /// Rust: `if !val.is_empty()` / `if val.is_some()` / `if val != 0`
    pub(crate) fn apply_truthiness_conversion(
        condition: &HirExpr,
        cond_expr: syn::Expr,
        ctx: &CodeGenContext,
    ) -> syn::Expr {
        // Check if this is a variable reference that needs truthiness conversion
        if let HirExpr::Var(var_name) = condition {
            if let Some(var_type) = ctx.var_types.get(var_name) {
                match var_type {
                    // Already boolean - no conversion needed
                    Type::Bool => return cond_expr,

                    // String/List/Dict/Set - check if empty
                    Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                        return parse_quote! { !#cond_expr.is_empty() };
                    }

                    // Optional - check if Some
                    Type::Optional(_) => {
                        return parse_quote! { #cond_expr.is_some() };
                    }

                    // Numeric types - check if non-zero
                    Type::Int => {
                        return parse_quote! { #cond_expr != 0 };
                    }
                    Type::Float => {
                        return parse_quote! { #cond_expr != 0.0 };
                    }

                    // DEPYLER-1071: Custom types that are collections
                    Type::Custom(type_name) => {
                        if is_collection_type_name(type_name) {
                            return parse_quote! { !#cond_expr.is_empty() };
                        }
                        // Fall through to heuristics
                    }

                    // DEPYLER-1071: Generic types that are collections
                    Type::Generic { base, .. } => {
                        if is_collection_generic_base(base) {
                            return parse_quote! { !#cond_expr.is_empty() };
                        }
                        // Fall through to heuristics
                    }

                    // Unknown - fall through to heuristics
                    Type::Unknown => {}

                    // Other types - fall through to heuristics
                    _ => {}
                }
            }

            // DEPYLER-1071: Heuristic fallback for common string variable names
            if is_string_var_name(var_name) {
                return parse_quote! { !#cond_expr.is_empty() };
            }

            // DEPYLER-1071: Heuristic fallback for common collection variable names
            if is_collection_var_name(var_name) {
                return parse_quote! { !#cond_expr.is_empty() };
            }

            // DEPYLER-1071: Heuristic fallback for common Option variable names
            // This handles regex match results and other optional values
            // Pattern: `if m:` where m is a regex match result (Option<Match>)
            if is_option_var_name(var_name) {
                return parse_quote! { #cond_expr.is_some() };
            }
        }

        // Not a variable or no type info - use as-is
        cond_expr
    }

    pub(crate) fn convert_sort_by_key(
        &mut self,
        iterable: &HirExpr,
        key_params: &[String],
        key_body: &HirExpr,
        reverse_expr: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let iter_expr = iterable.to_rust_expr(self.ctx)?;

        // DEPYLER-0502: Convert reverse_expr to Rust expression (supports variables and expressions)
        // If None, default to false (no reversal)
        let reverse_rust_expr = if let Some(expr) = reverse_expr {
            expr.to_rust_expr(self.ctx)?
        } else {
            parse_quote! { false }
        };

        // DEPYLER-0307: Check if this is an identity function (lambda x: x)
        // If so, use simple .sort() instead of .sort_by_key()
        let is_identity =
            key_params.len() == 1 && matches!(key_body, HirExpr::Var(v) if v == &key_params[0]);

        if is_identity {
            // Identity function: just sort() + conditional reverse()
            return Ok(parse_quote! {
                {
                    let mut __sorted_result = #iter_expr.clone();
                    __sorted_result.sort();
                    if #reverse_rust_expr {
                        __sorted_result.reverse();
                    }
                    __sorted_result
                }
            });
        }

        // Non-identity key function: use sort_by_key
        let body_expr = key_body.to_rust_expr(self.ctx)?;

        // DEPYLER-0597: Use safe_ident to escape Rust keywords in sorted key lambda parameters
        let param_pat: syn::Pat = if key_params.len() == 1 {
            let param = crate::rust_gen::keywords::safe_ident(&key_params[0]);
            parse_quote! { #param }
        } else {
            bail!("sorted() key lambda must have exactly one parameter");
        };

        // DEPYLER-0502: Generate code with runtime conditional reverse
        // { let mut result = iterable.clone(); result.sort_by_key(|param| body); if reverse_expr { result.reverse(); } result }
        Ok(parse_quote! {
            {
                let mut __sorted_result = #iter_expr.clone();
                __sorted_result.sort_by_key(|#param_pat| #body_expr);
                if #reverse_rust_expr {
                    __sorted_result.reverse();
                }
                __sorted_result
            }
        })
    }

    pub(crate) fn convert_generator_expression(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Strategy: Simple cases use iterator chains, nested use flat_map

        if generators.is_empty() {
            bail!("Generator expression must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-1077: Pre-register char_iter_vars BEFORE converting element expression
            // This ensures ord(c) knows c is a char when iterating over a string
            let is_string_iter_precheck = if let HirExpr::Var(var_name) = &*gen.iter {
                self.ctx
                    .var_types
                    .get(var_name)
                    .map(|ty| matches!(ty, crate::hir::Type::String))
                    .unwrap_or(false)
            } else {
                false
            };
            if is_string_iter_precheck {
                self.ctx.char_iter_vars.insert(gen.target.clone());
            }

            // Now convert element expression (with char_iter_vars populated)
            let element_expr = element.to_rust_expr(self.ctx)?;

            // DEPYLER-0454: Detect CSV reader variables in generator expressions
            let is_csv_reader = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "reader"
                    || var_name.contains("csv")
                    || var_name.ends_with("_reader")
                    || var_name.starts_with("reader_")
            } else {
                false
            };

            // DEPYLER-0523: Detect file variables for BufReader wrapping
            // Same heuristics as stmt_gen.rs for loop file iteration
            let is_file_iter = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "f"
                    || var_name == "file"
                    || var_name == "input"
                    || var_name == "output"
                    || var_name.ends_with("_file")
                    || var_name.starts_with("file_")
            } else {
                false
            };

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr =
                if !is_csv_reader && !is_file_iter && !matches!(&*gen.iter, HirExpr::Var(_)) {
                    self.wrap_range_in_parens(iter_expr)
                } else {
                    iter_expr
                };

            // DEPYLER-0307 Fix #10: Use .iter().copied() for borrowed collections
            // DEPYLER-0454 Extension: Use .deserialize() for CSV readers
            // DEPYLER-0523: Use BufReader for file iteration
            // When the iterator is a variable (likely a borrowed parameter like &Vec<i32>),
            // use .iter().copied() to get owned values instead of references
            // This prevents type mismatches like `&i32` vs `i32` in generator expressions
            let mut chain: syn::Expr = if is_csv_reader {
                // DEPYLER-0454: CSV reader - use deserialize pattern
                self.ctx.needs_csv = true;
                parse_quote! { #iter_expr.deserialize::<std::collections::HashMap<String, String>>().filter_map(|result| result.ok()) }
            } else if is_file_iter {
                // DEPYLER-0523: File variable - use BufReader for line iteration
                self.ctx.needs_bufread = true;
                parse_quote! { std::io::BufReader::new(#iter_expr).lines().map(|l| l.unwrap_or_default()) }
            } else if self.is_numpy_array_expr(&gen.iter) {
                // DEPYLER-0575: trueno Vector uses .as_slice().iter()
                // DEPYLER-0909: Use .cloned() instead of .copied() for compatibility with non-Copy types
                parse_quote! { #iter_expr.as_slice().iter().cloned() }
            } else if self.is_json_value_iteration(&gen.iter) {
                // DEPYLER-0607: JSON Value iteration in generator expression
                // serde_json::Value doesn't implement IntoIterator, must convert first
                parse_quote! { #iter_expr.as_array().unwrap_or(&vec![]).iter().cloned() }
            } else if matches!(&*gen.iter, HirExpr::Var(_)) {
                // DEPYLER-1077: Check if variable is a string type - strings use .chars()
                let is_string_type = if let HirExpr::Var(var_name) = &*gen.iter {
                    self.ctx
                        .var_types
                        .get(var_name)
                        .map(|ty| matches!(ty, crate::hir::Type::String))
                        .unwrap_or(false)
                } else {
                    false
                };
                if is_string_type {
                    // DEPYLER-1077: String iteration uses .chars() not .iter()
                    // Also register target as a char iteration variable for ord() handling
                    self.ctx.char_iter_vars.insert(gen.target.clone());
                    parse_quote! { #iter_expr.chars() }
                } else {
                    // DEPYLER-0674: Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                    parse_quote! { #iter_expr.iter().cloned() }
                }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-1079: Check if iterator is a zip() call on reference types
            // zip() on &Vec produces (&T, &T) tuples that need dereferencing for owned returns
            // Pattern: (a, b) for (a, b) in zip(list1, list2) where list1/list2 are &Vec
            let is_zip_call = matches!(&*gen.iter, HirExpr::Call { func, .. } if func == "zip");

            if is_zip_call && gen.target.contains(',') {
                // Parse target pattern to extract tuple variable names
                // Target is like "(a, b)" or "a, b" - strip parens and split
                let target_clean = gen.target.trim_start_matches('(').trim_end_matches(')');
                let vars: Vec<&str> = target_clean.split(',').map(|s| s.trim()).collect();
                if vars.len() == 2 && !vars[0].is_empty() && !vars[1].is_empty() {
                    let a = syn::Ident::new(vars[0], proc_macro2::Span::call_site());
                    let b = syn::Ident::new(vars[1], proc_macro2::Span::call_site());
                    // Add map to clone/dereference tuple elements
                    chain = parse_quote! { #chain.map(|(#a, #b)| (#a.clone(), #b.clone())) };
                }
            }

            // DEPYLER-0691: Add filters for each condition
            // DEPYLER-0820/1074: filter() receives &Item (even after .cloned())
            // Use |&#target_pat| to destructure the reference, getting owned value
            // This allows comparisons like x > 0 to work without type errors
            //
            // DEPYLER-1074: Register target variable's element type so numeric coercion works
            // When iterating over List[float], target x is float, so x > 0 should coerce to x > 0.0
            let element_type = if let HirExpr::Var(iter_var) = &*gen.iter {
                self.ctx
                    .var_types
                    .get(iter_var)
                    .and_then(|ty| match ty {
                        crate::hir::Type::List(elem) => Some(elem.as_ref().clone()),
                        crate::hir::Type::Set(elem) => Some(elem.as_ref().clone()),
                        _ => None,
                    })
            } else {
                None
            };

            // Temporarily register target variable with element type for condition conversion
            let target_var_name = gen.target.clone();
            if let Some(ref elem_ty) = element_type {
                self.ctx.var_types.insert(target_var_name.clone(), elem_ty.clone());
            }

            // DEPYLER-1076: When function returns impl Iterator, closures need `move`
            // to take ownership of captured local variables (like min_val, factor, etc.)
            let needs_move = self.ctx.returns_impl_iterator;

            // DEPYLER-1081: Check if target is a tuple pattern
            // For tuples like (i, v), using |&(i, v)| causes E0507 for non-Copy elements
            // Instead, use |(i, v)| which receives references without trying to move
            let is_tuple_pattern = gen.target.contains(',');

            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                if is_tuple_pattern {
                    // DEPYLER-1081: Tuple patterns - use |(a, b)| to avoid move out of shared ref
                    // Rust's match ergonomics will handle &(A, B) with |(a, b)| pattern
                    if needs_move {
                        chain = parse_quote! { #chain.filter(move |#target_pat| #cond_expr) };
                    } else {
                        chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
                    }
                } else if needs_move {
                    chain = parse_quote! { #chain.filter(move |&#target_pat| #cond_expr) };
                } else {
                    chain = parse_quote! { #chain.filter(|&#target_pat| #cond_expr) };
                }
            }

            // Clean up: remove the temporary target variable
            if element_type.is_some() {
                self.ctx.var_types.remove(&target_var_name);
            }

            // Add the map transformation
            // DEPYLER-1076: Use move when returning impl Iterator
            if needs_move {
                chain = parse_quote! { #chain.map(move |#target_pat| #element_expr) };
            } else {
                chain = parse_quote! { #chain.map(|#target_pat| #element_expr) };
            }

            return Ok(chain);
        }

        // Multiple generators case (nested iteration with flat_map)
        // Pattern: (x + y for x in range(3) for y in range(3))
        // Becomes: (0..3).flat_map(|x| (0..3).map(move |y| x + y))

        self.convert_nested_generators(element, generators)
    }

    pub(crate) fn convert_nested_generators(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Start with the outermost generator
        let first_gen = &generators[0];
        let first_iter = first_gen.iter.to_rust_expr(self.ctx)?;
        let first_pat = self.parse_target_pattern(&first_gen.target)?;

        // Build the nested expression recursively
        let inner_expr = self.build_nested_chain(element, generators, 1)?;

        // Start the chain with the first generator
        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let first_iter = self.wrap_range_in_parens(first_iter);
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // DEPYLER-1076: When function returns impl Iterator, closures need `move`
        let needs_move = self.ctx.returns_impl_iterator;

        // DEPYLER-0691: Add filters for first generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        for cond in &first_gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            if needs_move {
                chain = parse_quote! { #chain.filter(move |#first_pat| #cond_expr) };
            } else {
                chain = parse_quote! { #chain.filter(|#first_pat| #cond_expr) };
            }
        }

        // Use flat_map for the first generator
        // DEPYLER-1076: Use move when returning impl Iterator
        if needs_move {
            chain = parse_quote! { #chain.flat_map(move |#first_pat| #inner_expr) };
        } else {
            chain = parse_quote! { #chain.flat_map(|#first_pat| #inner_expr) };
        }

        Ok(chain)
    }

    pub(crate) fn build_nested_chain(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
        depth: usize,
    ) -> Result<syn::Expr> {
        if depth >= generators.len() {
            // Base case: no more generators, return the element expression
            let element_expr = element.to_rust_expr(self.ctx)?;
            return Ok(element_expr);
        }

        let gen = &generators[depth];
        let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
        let target_pat = self.parse_target_pattern(&gen.target)?;

        // Build the inner expression (recursive)
        let inner_expr = self.build_nested_chain(element, generators, depth + 1)?;

        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let iter_expr = self.wrap_range_in_parens(iter_expr);

        // Build the chain for this level
        let mut chain: syn::Expr = parse_quote! { #iter_expr.into_iter() };

        // DEPYLER-0691: Add filters for this generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        // DEPYLER-1076: Use move when returning impl Iterator (for captured locals)
        for cond in &gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            if self.ctx.returns_impl_iterator {
                chain = parse_quote! { #chain.filter(move |#target_pat| #cond_expr) };
            } else {
                chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
            }
        }

        // Use flat_map for intermediate generators, map for the last
        // Note: These already use `move` for capturing outer loop variables
        if depth < generators.len() - 1 {
            // Intermediate generator: use flat_map
            chain = parse_quote! { #chain.flat_map(move |#target_pat| #inner_expr) };
        } else {
            // Last generator: use map
            // DEPYLER-1082: Check if element is just the target variable (identity pattern)
            // In this case, use .copied() instead of .map(|x| x) to dereference
            // This handles (x for lst in lists for x in lst) where lst is &Vec<i32>
            let is_identity = matches!(element, HirExpr::Var(v) if v == &gen.target);
            if is_identity {
                // DEPYLER-1082: Use .copied() for primitive types to dereference
                // This converts Iterator<Item=&T> to Iterator<Item=T> for Copy types
                chain = parse_quote! { #chain.copied() };
            } else {
                chain = parse_quote! { #chain.map(move |#target_pat| #inner_expr) };
            }
        }

        Ok(chain)
    }

    pub(crate) fn parse_target_pattern(&self, target: &str) -> Result<syn::Pat> {
        // Handle simple variable: x
        // Handle tuple: (x, y)
        if target.starts_with('(') && target.ends_with(')') {
            // Tuple pattern
            let inner = &target[1..target.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            let idents: Vec<syn::Ident> = parts
                .iter()
                .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                .collect();
            Ok(parse_quote! { ( #(#idents),* ) })
        } else {
            // Simple variable
            let ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            Ok(parse_quote! { #ident })
        }
    }

    /// DEPYLER-0188: Convert walrus operator (assignment expression)
    /// Python: (x := expr) assigns expr to x and evaluates to expr
    /// Rust: { let x = expr; x } - block expression that assigns and returns
    pub(crate) fn convert_named_expr(&mut self, target: &str, value: &HirExpr) -> Result<syn::Expr> {
        let ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let value_expr = value.to_rust_expr(self.ctx)?;

        // Generate: { let target = value; target }
        // This assigns the value and returns it, matching Python's walrus semantics
        Ok(parse_quote! {
            {
                let #ident = #value_expr;
                #ident
            }
        })
    }
}

// ============================================================================
// EXTREME TDD TESTS - DEPYLER-COVERAGE-95
// ============================================================================
#[cfg(test)]
mod tests {
    use crate::DepylerPipeline;

    fn transpile(code: &str) -> String {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(code).expect("transpilation should succeed")
    }

    fn transpile_ok(code: &str) -> bool {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(code).is_ok()
    }

    // ========================================================================
    // LIST METHOD TESTS - convert_list_method
    // ========================================================================

    #[test]
    fn test_list_append() {
        let code = transpile(
            r#"
def append_test():
    items = [1, 2, 3]
    items.append(4)
    return items
"#,
        );
        assert!(code.contains("push"));
    }

    #[test]
    fn test_list_append_string() {
        let code = transpile(
            r#"
def append_string():
    items = ["a", "b"]
    items.append("c")
    return items
"#,
        );
        assert!(code.contains("push"));
    }

    #[test]
    fn test_list_extend() {
        let code = transpile(
            r#"
def extend_test():
    items = [1, 2]
    items.extend([3, 4])
    return items
"#,
        );
        assert!(code.contains("extend"));
    }

    #[test]
    fn test_list_pop_no_args() {
        let code = transpile(
            r#"
def pop_test():
    items = [1, 2, 3]
    return items.pop()
"#,
        );
        assert!(code.contains("pop"));
    }

    #[test]
    fn test_list_pop_with_index() {
        let code = transpile(
            r#"
def pop_index():
    items = [1, 2, 3]
    return items.pop(0)
"#,
        );
        assert!(code.contains("remove"));
    }

    #[test]
    fn test_list_insert() {
        let code = transpile(
            r#"
def insert_test():
    items = [1, 3]
    items.insert(1, 2)
    return items
"#,
        );
        assert!(code.contains("insert"));
    }

    #[test]
    fn test_list_remove() {
        let code = transpile(
            r#"
def remove_test():
    items = [1, 2, 3]
    items.remove(2)
    return items
"#,
        );
        assert!(code.contains("remove") || code.contains("position"));
    }

    #[test]
    fn test_list_index() {
        let code = transpile(
            r#"
def index_test():
    items = [1, 2, 3]
    return items.index(2)
"#,
        );
        assert!(code.contains("position"));
    }

    #[test]
    fn test_list_count() {
        let code = transpile(
            r#"
def count_test():
    items = [1, 2, 2, 3]
    return items.count(2)
"#,
        );
        assert!(code.contains("filter") || code.contains("count"));
    }

    #[test]
    fn test_list_copy() {
        let code = transpile(
            r#"
def copy_test():
    items = [1, 2, 3]
    return items.copy()
"#,
        );
        assert!(code.contains("clone"));
    }

    #[test]
    fn test_list_clear() {
        let code = transpile(
            r#"
def clear_test():
    items = [1, 2, 3]
    items.clear()
    return items
"#,
        );
        assert!(code.contains("clear"));
    }

    #[test]
    fn test_list_reverse() {
        let code = transpile(
            r#"
def reverse_test():
    items = [1, 2, 3]
    items.reverse()
    return items
"#,
        );
        assert!(code.contains("reverse"));
    }

    #[test]
    fn test_list_sort() {
        let code = transpile(
            r#"
def sort_test():
    items = [3, 1, 2]
    items.sort()
    return items
"#,
        );
        assert!(code.contains("sort"));
    }

    #[test]
    fn test_list_sort_reverse() {
        let code = transpile(
            r#"
def sort_reverse():
    items = [1, 2, 3]
    items.sort(reverse=True)
    return items
"#,
        );
        assert!(code.contains("sort"));
    }

    // ========================================================================
    // DICT METHOD TESTS - convert_dict_method
    // ========================================================================

    #[test]
    fn test_dict_get_single_arg() {
        let code = transpile(
            r#"
def get_test():
    d = {"a": 1}
    return d.get("a")
"#,
        );
        assert!(code.contains("get"));
    }

    #[test]
    fn test_dict_get_with_default() {
        let code = transpile(
            r#"
def get_default():
    d = {"a": 1}
    return d.get("b", 0)
"#,
        );
        assert!(code.contains("get") || code.contains("unwrap_or"));
    }

    #[test]
    fn test_dict_keys() {
        let code = transpile(
            r#"
def keys_test():
    d = {"a": 1, "b": 2}
    return d.keys()
"#,
        );
        assert!(code.contains("keys"));
    }

    #[test]
    fn test_dict_values() {
        let code = transpile(
            r#"
def values_test():
    d = {"a": 1, "b": 2}
    return d.values()
"#,
        );
        assert!(code.contains("values"));
    }

    #[test]
    fn test_dict_items() {
        let code = transpile(
            r#"
def items_test():
    d = {"a": 1, "b": 2}
    return d.items()
"#,
        );
        assert!(code.contains("iter") || code.contains("items"));
    }

    #[test]
    fn test_dict_update() {
        let code = transpile(
            r#"
def update_test():
    d = {"a": 1}
    d.update({"b": 2})
    return d
"#,
        );
        assert!(code.contains("insert") || code.contains("update"));
    }

    #[test]
    fn test_dict_clear() {
        let code = transpile(
            r#"
def clear_dict():
    d = {"a": 1}
    d.clear()
    return d
"#,
        );
        assert!(code.contains("clear"));
    }

    #[test]
    fn test_dict_copy() {
        let code = transpile(
            r#"
def copy_dict():
    d = {"a": 1}
    return d.copy()
"#,
        );
        assert!(code.contains("clone"));
    }

    // ========================================================================
    // STRING METHOD TESTS - convert_string_method
    // ========================================================================

    #[test]
    fn test_string_upper() {
        let code = transpile(
            r#"
def upper_test():
    s = "hello"
    return s.upper()
"#,
        );
        assert!(code.contains("to_uppercase"));
    }

    #[test]
    fn test_string_lower() {
        let code = transpile(
            r#"
def lower_test():
    s = "HELLO"
    return s.lower()
"#,
        );
        assert!(code.contains("to_lowercase"));
    }

    #[test]
    fn test_string_strip() {
        let code = transpile(
            r#"
def strip_test():
    s = "  hello  "
    return s.strip()
"#,
        );
        assert!(code.contains("trim"));
    }

    #[test]
    fn test_string_startswith() {
        let code = transpile(
            r#"
def startswith_test():
    s = "hello"
    return s.startswith("he")
"#,
        );
        assert!(code.contains("starts_with"));
    }

    #[test]
    fn test_string_endswith() {
        let code = transpile(
            r#"
def endswith_test():
    s = "hello"
    return s.endswith("lo")
"#,
        );
        assert!(code.contains("ends_with"));
    }

    #[test]
    fn test_string_split_no_args() {
        let code = transpile(
            r#"
def split_test():
    s = "a b c"
    return s.split()
"#,
        );
        assert!(code.contains("split"));
    }

    #[test]
    fn test_string_split_with_sep() {
        let code = transpile(
            r#"
def split_sep():
    s = "a,b,c"
    return s.split(",")
"#,
        );
        assert!(code.contains("split"));
    }

    #[test]
    fn test_string_join() {
        let code = transpile(
            r#"
def join_test():
    items = ["a", "b", "c"]
    return ",".join(items)
"#,
        );
        assert!(code.contains("join"));
    }

    #[test]
    fn test_string_replace() {
        let code = transpile(
            r#"
def replace_test():
    s = "hello"
    return s.replace("l", "x")
"#,
        );
        assert!(code.contains("replace") || code.contains("replacen"));
    }

    #[test]
    fn test_string_find() {
        let code = transpile(
            r#"
def find_test():
    s = "hello"
    return s.find("l")
"#,
        );
        assert!(code.contains("find") || code.contains("position"));
    }

    #[test]
    fn test_string_count() {
        let code = transpile(
            r#"
def count_str():
    s = "hello"
    return s.count("l")
"#,
        );
        assert!(code.contains("matches") || code.contains("count"));
    }

    #[test]
    fn test_string_isdigit() {
        let code = transpile(
            r#"
def isdigit_test():
    s = "123"
    return s.isdigit()
"#,
        );
        assert!(code.contains("is_ascii_digit") || code.contains("chars"));
    }

    #[test]
    fn test_string_isalpha() {
        let code = transpile(
            r#"
def isalpha_test():
    s = "abc"
    return s.isalpha()
"#,
        );
        assert!(code.contains("is_alphabetic") || code.contains("chars"));
    }

    #[test]
    fn test_string_lstrip() {
        let code = transpile(
            r#"
def lstrip_test():
    s = "  hello"
    return s.lstrip()
"#,
        );
        assert!(code.contains("trim_start"));
    }

    #[test]
    fn test_string_rstrip() {
        let code = transpile(
            r#"
def rstrip_test():
    s = "hello  "
    return s.rstrip()
"#,
        );
        assert!(code.contains("trim_end"));
    }

    #[test]
    fn test_string_capitalize() {
        assert!(transpile_ok(
            r#"
def cap_test():
    s = "hello"
    return s.capitalize()
"#
        ));
    }

    #[test]
    fn test_string_title() {
        assert!(transpile_ok(
            r#"
def title_test():
    s = "hello world"
    return s.title()
"#
        ));
    }

    #[test]
    fn test_string_center() {
        assert!(transpile_ok(
            r#"
def center_test():
    s = "hi"
    return s.center(10)
"#
        ));
    }

    #[test]
    fn test_string_ljust() {
        assert!(transpile_ok(
            r#"
def ljust_test():
    s = "hi"
    return s.ljust(10)
"#
        ));
    }

    #[test]
    fn test_string_rjust() {
        assert!(transpile_ok(
            r#"
def rjust_test():
    s = "hi"
    return s.rjust(10)
"#
        ));
    }

    #[test]
    fn test_string_zfill() {
        assert!(transpile_ok(
            r#"
def zfill_test():
    s = "42"
    return s.zfill(5)
"#
        ));
    }

    // ========================================================================
    // SET METHOD TESTS - convert_set_method
    // ========================================================================

    #[test]
    fn test_set_add() {
        let code = transpile(
            r#"
def add_test():
    s = {1, 2}
    s.add(3)
    return s
"#,
        );
        assert!(code.contains("insert"));
    }

    #[test]
    fn test_set_remove() {
        let code = transpile(
            r#"
def remove_set():
    s = {1, 2, 3}
    s.remove(2)
    return s
"#,
        );
        assert!(code.contains("remove"));
    }

    #[test]
    fn test_set_discard() {
        let code = transpile(
            r#"
def discard_test():
    s = {1, 2, 3}
    s.discard(2)
    return s
"#,
        );
        assert!(code.contains("remove"));
    }

    #[test]
    fn test_set_pop() {
        let code = transpile(
            r#"
def pop_set():
    s = {1, 2, 3}
    return s.pop()
"#,
        );
        assert!(code.contains("iter") || code.contains("next"));
    }

    #[test]
    fn test_set_clear() {
        let code = transpile(
            r#"
def clear_set():
    s = {1, 2, 3}
    s.clear()
    return s
"#,
        );
        assert!(code.contains("clear"));
    }

    #[test]
    fn test_set_union() {
        let code = transpile(
            r#"
def union_test():
    s1 = {1, 2}
    s2 = {3, 4}
    return s1.union(s2)
"#,
        );
        assert!(code.contains("union") || code.contains("extend"));
    }

    #[test]
    fn test_set_intersection() {
        let code = transpile(
            r#"
def intersection_test():
    s1 = {1, 2, 3}
    s2 = {2, 3, 4}
    return s1.intersection(s2)
"#,
        );
        assert!(code.contains("intersection") || code.contains("filter"));
    }

    #[test]
    fn test_set_difference() {
        let code = transpile(
            r#"
def difference_test():
    s1 = {1, 2, 3}
    s2 = {2, 3, 4}
    return s1.difference(s2)
"#,
        );
        assert!(code.contains("difference") || code.contains("filter"));
    }

    // ========================================================================
    // INDEX CONVERSION TESTS - convert_index
    // ========================================================================

    #[test]
    fn test_list_index_access() {
        let code = transpile(
            r#"
def index_access():
    items = [1, 2, 3]
    return items[0]
"#,
        );
        assert!(code.contains("[") && code.contains("]"));
    }

    #[test]
    fn test_dict_index_access() {
        let code = transpile(
            r#"
def dict_access():
    d = {"a": 1}
    return d["a"]
"#,
        );
        assert!(code.contains("get") || code.contains("["));
    }

    #[test]
    fn test_string_index_access() {
        let code = transpile(
            r#"
def string_access():
    s = "hello"
    return s[0]
"#,
        );
        assert!(code.contains("chars") || code.contains("nth"));
    }

    #[test]
    fn test_negative_index() {
        let code = transpile(
            r#"
def neg_index():
    items = [1, 2, 3]
    return items[-1]
"#,
        );
        assert!(code.contains("len") || code.contains("-"));
    }

    // ========================================================================
    // SLICE CONVERSION TESTS - convert_slice
    // ========================================================================

    #[test]
    fn test_slice_basic() {
        let code = transpile(
            r#"
def slice_basic():
    items = [1, 2, 3, 4, 5]
    return items[1:3]
"#,
        );
        assert!(code.contains("[") || code.contains(".."));
    }

    #[test]
    fn test_slice_from_start() {
        let code = transpile(
            r#"
def slice_from_start():
    items = [1, 2, 3, 4, 5]
    return items[:3]
"#,
        );
        assert!(code.contains("[") || code.contains(".."));
    }

    #[test]
    fn test_slice_to_end() {
        let code = transpile(
            r#"
def slice_to_end():
    items = [1, 2, 3, 4, 5]
    return items[2:]
"#,
        );
        assert!(code.contains("[") || code.contains(".."));
    }

    #[test]
    fn test_slice_full_copy() {
        let code = transpile(
            r#"
def slice_copy():
    items = [1, 2, 3]
    return items[:]
"#,
        );
        assert!(code.contains("clone") || code.contains("to_vec"));
    }

    #[test]
    fn test_string_slice() {
        let code = transpile(
            r#"
def string_slice():
    s = "hello"
    return s[1:4]
"#,
        );
        assert!(code.contains("[") || code.contains(".."));
    }

    // ========================================================================
    // LIST COMPREHENSION TESTS - convert_list_comp
    // ========================================================================

    #[test]
    fn test_list_comp_simple() {
        let code = transpile(
            r#"
def list_comp():
    return [x * 2 for x in [1, 2, 3]]
"#,
        );
        assert!(code.contains("map") || code.contains("collect"));
    }

    #[test]
    fn test_list_comp_with_filter() {
        let code = transpile(
            r#"
def list_comp_filter():
    return [x for x in [1, 2, 3, 4] if x > 2]
"#,
        );
        assert!(code.contains("filter") || code.contains("collect"));
    }

    #[test]
    fn test_list_comp_nested() {
        let code = transpile(
            r#"
def nested_comp():
    return [x + y for x in [1, 2] for y in [10, 20]]
"#,
        );
        assert!(code.contains("flat_map") || code.contains("map"));
    }

    // ========================================================================
    // DICT COMPREHENSION TESTS - convert_dict_comp
    // ========================================================================

    #[test]
    fn test_dict_comp_simple() {
        let code = transpile(
            r#"
def dict_comp():
    return {x: x * 2 for x in [1, 2, 3]}
"#,
        );
        assert!(code.contains("map") || code.contains("collect") || code.contains("HashMap"));
    }

    #[test]
    fn test_dict_comp_with_filter() {
        let code = transpile(
            r#"
def dict_comp_filter():
    return {x: x * 2 for x in [1, 2, 3, 4] if x > 2}
"#,
        );
        assert!(code.contains("filter") || code.contains("collect"));
    }

    // ========================================================================
    // SET COMPREHENSION TESTS - convert_set_comp
    // ========================================================================

    #[test]
    fn test_set_comp_simple() {
        let code = transpile(
            r#"
def set_comp():
    return {x * 2 for x in [1, 2, 3]}
"#,
        );
        assert!(code.contains("map") || code.contains("collect") || code.contains("HashSet"));
    }

    // ========================================================================
    // GENERATOR EXPRESSION TESTS - convert_generator_expression
    // ========================================================================

    #[test]
    fn test_generator_in_sum() {
        let code = transpile(
            r#"
def gen_sum():
    return sum(x for x in [1, 2, 3])
"#,
        );
        assert!(code.contains("sum") || code.contains("fold"));
    }

    #[test]
    fn test_generator_in_any() {
        let code = transpile(
            r#"
def gen_any():
    return any(x > 2 for x in [1, 2, 3])
"#,
        );
        assert!(code.contains("any"));
    }

    #[test]
    fn test_generator_in_all() {
        let code = transpile(
            r#"
def gen_all():
    return all(x > 0 for x in [1, 2, 3])
"#,
        );
        assert!(code.contains("all"));
    }

    // ========================================================================
    // TUPLE CONVERSION TESTS - convert_tuple
    // ========================================================================

    #[test]
    fn test_tuple_creation() {
        let code = transpile(
            r#"
def tuple_test():
    return (1, 2, 3)
"#,
        );
        assert!(code.contains("(") && code.contains(")"));
    }

    #[test]
    fn test_tuple_mixed() {
        let code = transpile(
            r#"
def tuple_mixed():
    return (1, "hello", 3.14)
"#,
        );
        assert!(code.contains("("));
    }

    // ========================================================================
    // SET CONVERSION TESTS - convert_set
    // ========================================================================

    #[test]
    fn test_set_creation() {
        let code = transpile(
            r#"
def set_create():
    return {1, 2, 3}
"#,
        );
        assert!(code.contains("HashSet") || code.contains("from"));
    }

    // ========================================================================
    // ATTRIBUTE CONVERSION TESTS - convert_attribute
    // ========================================================================

    #[test]
    fn test_attribute_access() {
        assert!(transpile_ok(
            r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def get_x(p: Point) -> int:
    return p.x
"#
        ));
    }

    // ========================================================================
    // F-STRING TESTS - convert_fstring
    // ========================================================================

    #[test]
    fn test_fstring_simple() {
        let code = transpile(
            r#"
def fstring_test():
    name = "world"
    return f"Hello, {name}!"
"#,
        );
        assert!(code.contains("format!"));
    }

    #[test]
    fn test_fstring_expression() {
        let code = transpile(
            r#"
def fstring_expr():
    x = 5
    return f"Value: {x + 1}"
"#,
        );
        assert!(code.contains("format!"));
    }

    #[test]
    fn test_fstring_multiple() {
        let code = transpile(
            r#"
def fstring_multi():
    a = 1
    b = 2
    return f"{a} + {b} = {a + b}"
"#,
        );
        assert!(code.contains("format!"));
    }

    // ========================================================================
    // IF EXPRESSION TESTS - convert_ifexpr
    // ========================================================================

    #[test]
    fn test_ifexpr_simple() {
        let code = transpile(
            r#"
def ifexpr_test():
    x = 5
    return "big" if x > 3 else "small"
"#,
        );
        assert!(code.contains("if") && code.contains("else"));
    }

    #[test]
    fn test_ifexpr_nested() {
        let code = transpile(
            r#"
def ifexpr_nested():
    x = 5
    return "big" if x > 10 else "medium" if x > 3 else "small"
"#,
        );
        assert!(code.contains("if") && code.contains("else"));
    }

    // ========================================================================
    // LAMBDA TESTS - convert_lambda
    // ========================================================================

    #[test]
    fn test_lambda_simple() {
        let code = transpile(
            r#"
def lambda_test():
    f = lambda x: x * 2
    return f(5)
"#,
        );
        assert!(code.contains("|") || code.contains("Fn"));
    }

    #[test]
    fn test_lambda_multi_args() {
        let code = transpile(
            r#"
def lambda_multi():
    f = lambda x, y: x + y
    return f(2, 3)
"#,
        );
        assert!(code.contains("|"));
    }

    // ========================================================================
    // BOOLEAN HELPER TESTS
    // ========================================================================

    #[test]
    fn test_is_len_call_detection() {
        let code = transpile(
            r#"
def len_test():
    items = [1, 2, 3]
    return len(items)
"#,
        );
        assert!(code.contains("len()"));
    }

    // ========================================================================
    // WALRUS OPERATOR TESTS - convert_named_expr
    // ========================================================================

    #[test]
    fn test_walrus_in_if() {
        let code = transpile(
            r#"
def walrus_test():
    items = [1, 2, 3]
    if (n := len(items)) > 2:
        return n
    return 0
"#,
        );
        assert!(code.contains("let n"));
    }

    #[test]
    fn test_walrus_in_while() {
        let code = transpile(
            r#"
def walrus_while():
    i = 0
    while (x := i) < 5:
        i += 1
    return x
"#,
        );
        assert!(code.contains("let x") || code.contains("while"));
    }

    // ========================================================================
    // INSTANCE METHOD TESTS - convert_instance_method
    // ========================================================================

    #[test]
    fn test_bytes_decode() {
        assert!(transpile_ok(
            r#"
def decode_test():
    b = b"hello"
    return b.decode("utf-8")
"#
        ));
    }

    #[test]
    fn test_str_encode() {
        assert!(transpile_ok(
            r#"
def encode_test():
    s = "hello"
    return s.encode("utf-8")
"#
        ));
    }

    // ========================================================================
    // ADDITIONAL STRING METHOD TESTS
    // ========================================================================

    #[test]
    fn test_string_isupper() {
        assert!(transpile_ok(
            r#"
def isupper_test():
    s = "HELLO"
    return s.isupper()
"#
        ));
    }

    #[test]
    fn test_string_islower() {
        assert!(transpile_ok(
            r#"
def islower_test():
    s = "hello"
    return s.islower()
"#
        ));
    }

    #[test]
    fn test_string_isalnum() {
        assert!(transpile_ok(
            r#"
def isalnum_test():
    s = "abc123"
    return s.isalnum()
"#
        ));
    }

    #[test]
    fn test_string_isspace() {
        assert!(transpile_ok(
            r#"
def isspace_test():
    s = "   "
    return s.isspace()
"#
        ));
    }

    #[test]
    fn test_string_format() {
        assert!(transpile_ok(
            r#"
def format_test():
    return "{} {}".format("hello", "world")
"#
        ));
    }

    // ========================================================================
    // ADDITIONAL COLLECTION TESTS
    // ========================================================================

    #[test]
    fn test_list_multiplication() {
        let code = transpile(
            r#"
def list_mul():
    return [0] * 5
"#,
        );
        assert!(code.contains("vec!") || code.contains("*") || code.contains("repeat"));
    }

    #[test]
    fn test_list_concatenation() {
        let code = transpile(
            r#"
def list_concat():
    return [1, 2] + [3, 4]
"#,
        );
        assert!(code.contains("extend") || code.contains("concat") || code.contains("+"));
    }

    #[test]
    fn test_dict_setdefault() {
        let code = transpile(
            r#"
def setdefault_test():
    d = {}
    d.setdefault("a", 0)
    return d
"#,
        );
        assert!(code.contains("entry") || code.contains("or_insert"));
    }

    // ========================================================================
    // NUMERIC TYPE TESTS
    // ========================================================================

    #[test]
    fn test_int_bit_length() {
        assert!(transpile_ok(
            r#"
def bit_length_test():
    n = 255
    return n.bit_length()
"#
        ));
    }

    #[test]
    fn test_float_is_integer() {
        assert!(transpile_ok(
            r#"
def is_integer_test():
    f = 3.0
    return f.is_integer()
"#
        ));
    }

    // ========================================================================
    // ITERATOR METHODS
    // ========================================================================

    #[test]
    fn test_enumerate() {
        let code = transpile(
            r#"
def enumerate_test():
    items = ["a", "b", "c"]
    result = []
    for i, item in enumerate(items):
        result.append((i, item))
    return result
"#,
        );
        assert!(code.contains("enumerate"));
    }

    #[test]
    fn test_zip() {
        let code = transpile(
            r#"
def zip_test():
    a = [1, 2, 3]
    b = ["a", "b", "c"]
    return list(zip(a, b))
"#,
        );
        assert!(code.contains("zip"));
    }

    #[test]
    fn test_reversed() {
        let code = transpile(
            r#"
def reversed_test():
    items = [1, 2, 3]
    return list(reversed(items))
"#,
        );
        assert!(code.contains("rev"));
    }

    #[test]
    fn test_sorted() {
        let code = transpile(
            r#"
def sorted_test():
    items = [3, 1, 2]
    return sorted(items)
"#,
        );
        assert!(code.contains("sort"));
    }

    // ========================================================================
    // REGEX METHOD TESTS (convert_regex_method)
    // ========================================================================

    #[test]
    fn test_regex_findall() {
        assert!(transpile_ok(
            r#"
import re

def findall_test():
    text = "hello world"
    return re.findall(r"\w+", text)
"#
        ));
    }

    #[test]
    fn test_regex_match() {
        assert!(transpile_ok(
            r#"
import re

def match_test():
    text = "hello"
    return re.match(r"he", text)
"#
        ));
    }

    #[test]
    fn test_regex_search() {
        assert!(transpile_ok(
            r#"
import re

def search_test():
    text = "hello world"
    return re.search(r"world", text)
"#
        ));
    }

    #[test]
    fn test_regex_sub() {
        assert!(transpile_ok(
            r#"
import re

def sub_test():
    text = "hello world"
    return re.sub(r"world", "there", text)
"#
        ));
    }

    // ========================================================================
    // TRUTHINESS CONVERSION TESTS
    // ========================================================================

    #[test]
    fn test_truthiness_list() {
        let code = transpile(
            r#"
def truthiness_list():
    items = [1, 2, 3]
    if items:
        return True
    return False
"#,
        );
        assert!(code.contains("is_empty") || code.contains("!"));
    }

    #[test]
    fn test_truthiness_string() {
        let code = transpile(
            r#"
def truthiness_str():
    s = "hello"
    if s:
        return True
    return False
"#,
        );
        assert!(code.contains("is_empty") || code.contains("!"));
    }

    #[test]
    fn test_truthiness_dict() {
        let code = transpile(
            r#"
def truthiness_dict():
    d = {"a": 1}
    if d:
        return True
    return False
"#,
        );
        assert!(code.contains("is_empty") || code.contains("!"));
    }

    // ========================================================================
    // BORROW CONVERSION TESTS - convert_borrow
    // ========================================================================

    #[test]
    fn test_immutable_borrow() {
        let code = transpile(
            r#"
def borrow_test(items: list):
    for item in items:
        print(item)
"#,
        );
        assert!(code.contains("&") || code.contains("iter"));
    }

    // ========================================================================
    // DEQUE TESTS
    // ========================================================================

    #[test]
    fn test_deque_append() {
        assert!(transpile_ok(
            r#"
from collections import deque

def deque_test():
    d = deque([1, 2, 3])
    d.append(4)
    return d
"#
        ));
    }

    #[test]
    fn test_deque_appendleft() {
        assert!(transpile_ok(
            r#"
from collections import deque

def deque_left():
    d = deque([1, 2, 3])
    d.appendleft(0)
    return d
"#
        ));
    }

    #[test]
    fn test_deque_popleft() {
        assert!(transpile_ok(
            r#"
from collections import deque

def deque_popleft():
    d = deque([1, 2, 3])
    return d.popleft()
"#
        ));
    }

    // ========================================================================
    // COUNTER TESTS
    // ========================================================================

    #[test]
    fn test_counter_creation() {
        assert!(transpile_ok(
            r#"
from collections import Counter

def counter_test():
    c = Counter([1, 1, 2, 2, 2, 3])
    return c
"#
        ));
    }

    #[test]
    fn test_counter_most_common() {
        assert!(transpile_ok(
            r#"
from collections import Counter

def counter_common():
    c = Counter([1, 1, 2, 2, 2, 3])
    return c.most_common(2)
"#
        ));
    }

    // ========================================================================
    // AWAIT TESTS - convert_await
    // ========================================================================

    #[test]
    fn test_async_await() {
        assert!(transpile_ok(
            r#"
async def fetch_data():
    return 42

async def main():
    result = await fetch_data()
    return result
"#
        ));
    }

    // ========================================================================
    // MORE EDGE CASE TESTS
    // ========================================================================

    #[test]
    fn test_empty_list() {
        let code = transpile(
            r#"
def empty_list():
    return []
"#,
        );
        assert!(code.contains("vec!") || code.contains("Vec::new"));
    }

    #[test]
    fn test_empty_dict() {
        let code = transpile(
            r#"
def empty_dict():
    return {}
"#,
        );
        assert!(code.contains("HashMap::new") || code.contains("HashMap"));
    }

    #[test]
    fn test_empty_set() {
        let code = transpile(
            r#"
def empty_set():
    return set()
"#,
        );
        assert!(code.contains("HashSet::new") || code.contains("HashSet"));
    }

    #[test]
    fn test_nested_list() {
        let code = transpile(
            r#"
def nested_list():
    return [[1, 2], [3, 4]]
"#,
        );
        assert!(code.contains("vec!"));
    }

    #[test]
    fn test_nested_dict() {
        let code = transpile(
            r#"
def nested_dict():
    return {"a": {"b": 1}}
"#,
        );
        assert!(code.contains("HashMap") || code.contains("insert"));
    }

    // ========================================================================
    // PARSE TARGET PATTERN TESTS - parse_target_pattern
    // ========================================================================

    #[test]
    fn test_for_tuple_unpacking() {
        let code = transpile(
            r#"
def tuple_unpack():
    pairs = [(1, 2), (3, 4)]
    result = 0
    for a, b in pairs:
        result += a + b
    return result
"#,
        );
        assert!(code.contains("(") && code.contains(")"));
    }

    #[test]
    fn test_for_dict_items() {
        let code = transpile(
            r#"
def dict_iter():
    d = {"a": 1, "b": 2}
    result = []
    for k, v in d.items():
        result.append((k, v))
    return result
"#,
        );
        assert!(code.contains("iter") || code.contains("items"));
    }

    // ========================================================================
    // RETURN TYPE DETECTION TESTS
    // ========================================================================

    #[test]
    fn test_expr_returns_result() {
        assert!(transpile_ok(
            r#"
def file_read():
    with open("test.txt") as f:
        return f.read()
"#
        ));
    }

    // ========================================================================
    // STRING METHOD EDGE CASES
    // ========================================================================

    #[test]
    fn test_string_split_maxsplit() {
        let code = transpile(
            r#"
def split_max():
    s = "a,b,c,d"
    return s.split(",", 2)
"#,
        );
        assert!(code.contains("splitn") || code.contains("split"));
    }

    #[test]
    fn test_string_rsplit() {
        let code = transpile(
            r#"
def rsplit_test():
    s = "a,b,c"
    return s.rsplit(",")
"#,
        );
        assert!(code.contains("rsplit") || code.contains("split"));
    }

    #[test]
    fn test_string_partition() {
        assert!(transpile_ok(
            r#"
def partition_test():
    s = "hello world"
    return s.partition(" ")
"#
        ));
    }

    #[test]
    fn test_string_rpartition() {
        assert!(transpile_ok(
            r#"
def rpartition_test():
    s = "hello world hello"
    return s.rpartition(" ")
"#
        ));
    }

    #[test]
    fn test_string_swapcase() {
        assert!(transpile_ok(
            r#"
def swapcase_test():
    s = "Hello World"
    return s.swapcase()
"#
        ));
    }

    #[test]
    fn test_string_expandtabs() {
        assert!(transpile_ok(
            r#"
def expandtabs_test():
    s = "a\tb\tc"
    return s.expandtabs(4)
"#
        ));
    }

    // ========================================================================
    // FROZENSET TESTS - convert_frozenset
    // ========================================================================

    #[test]
    fn test_frozenset_creation() {
        assert!(transpile_ok(
            r#"
def frozenset_test():
    return frozenset([1, 2, 3])
"#
        ));
    }

    // ========================================================================
    // MORE DICT METHOD TESTS
    // ========================================================================

    #[test]
    fn test_dict_pop_with_default() {
        let code = transpile(
            r#"
def dict_pop_default():
    d = {"a": 1}
    return d.pop("b", 0)
"#,
        );
        assert!(code.contains("remove") || code.contains("unwrap_or"));
    }

    #[test]
    fn test_dict_popitem() {
        let code = transpile(
            r#"
def dict_popitem():
    d = {"a": 1, "b": 2}
    return d.popitem()
"#,
        );
        assert!(code.contains("keys") || code.contains("remove"));
    }

    // ========================================================================
    // SYS IO METHOD TESTS - convert_sys_io_method
    // ========================================================================

    #[test]
    fn test_stdout_write() {
        assert!(transpile_ok(
            r#"
import sys

def stdout_test():
    sys.stdout.write("hello")
"#
        ));
    }

    #[test]
    fn test_stderr_write() {
        assert!(transpile_ok(
            r#"
import sys

def stderr_test():
    sys.stderr.write("error")
"#
        ));
    }

    // ========================================================================
    // ADDITIONAL CONVERSION TESTS
    // ========================================================================

    #[test]
    fn test_range_in_parens() {
        let code = transpile(
            r#"
def range_paren():
    return [x for x in range(10)]
"#,
        );
        assert!(code.contains("..") || code.contains("range"));
    }

    #[test]
    fn test_owned_collection_detection() {
        let code = transpile(
            r#"
def owned_test():
    items = [1, 2, 3]
    return items
"#,
        );
        assert!(code.contains("vec!"));
    }
}
