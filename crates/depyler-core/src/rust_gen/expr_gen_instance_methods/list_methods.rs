//! List method handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size and improve maintainability.
//! Contains handlers for: append, extend, pop, insert, remove, sort,
//! index, count, copy, clear, reverse.

use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use anyhow::{bail, Result};
use quote::quote;
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
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
            "append" => self.convert_list_append(object_expr, object, arg_exprs, hir_args),
            "extend" => {
                if arg_exprs.len() != 1 {
                    bail!("extend() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                let arg_string = quote! { #arg }.to_string();
                if arg_string.contains("&") || !arg_string.contains(".into_iter()") {
                    Ok(parse_quote! { #object_expr.extend(#arg.iter().cloned()) })
                } else {
                    Ok(parse_quote! { #object_expr.extend(#arg) })
                }
            }
            "pop" => self.convert_list_pop(object_expr, object, arg_exprs, hir_args),
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
                if arg_exprs.len() != 1 {
                    bail!("count() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.iter().filter(|x| **x == #value).count() as i32
                })
            }
            "copy" => {
                if arg_exprs.len() == 1 {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! { #arg.clone() });
                }
                if !arg_exprs.is_empty() {
                    bail!("copy() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clone() })
            }
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "reverse" => {
                if !arg_exprs.is_empty() {
                    bail!("reverse() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.reverse() })
            }
            "sort" => self.convert_list_sort(object_expr, arg_exprs, kwargs),
            _ => bail!("Unknown list method: {}", method),
        }
    }

    /// Handle list.append() with type-aware coercion and DepylerValue wrapping
    fn convert_list_append(
        &mut self,
        object_expr: &syn::Expr,
        object: &HirExpr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("append() requires exactly one argument");
        }
        let arg = &arg_exprs[0];

        // DEPYLER-1134: Check for CONCRETE element type FIRST
        let concrete_element_type = self.resolve_list_element_type(object);
        if let Some(elem_type) = concrete_element_type {
            let push_expr = self.generate_typed_push(object_expr, arg, &elem_type, hir_args)?;
            return Ok(push_expr);
        }

        // DEPYLER-1051: Check if target is Vec<DepylerValue>
        let is_vec_depyler_value = self.is_vec_depyler_value(object);
        if is_vec_depyler_value {
            let wrapped_arg = self.wrap_depyler_value_push_arg(arg, hir_args);
            self.ctx.needs_depyler_value_enum = true;
            return Ok(parse_quote! { #object_expr.push(#wrapped_arg) });
        }

        // DEPYLER-0422 Fix #7: String conversion for Vec<String>
        if self.needs_to_string_for_push(object, hir_args) {
            return Ok(parse_quote! { #object_expr.push(#arg.to_string()) });
        }

        // DEPYLER-99MODE-S9: Clone non-Copy variables to prevent E0382
        if self.needs_clone_for_push(hir_args) {
            Ok(parse_quote! { #object_expr.push(#arg.clone()) })
        } else {
            Ok(parse_quote! { #object_expr.push(#arg) })
        }
    }

    /// Resolve the concrete element type for a list variable or class field
    fn resolve_list_element_type(&self, object: &HirExpr) -> Option<Type> {
        if let HirExpr::Attribute { value: _, attr } = object {
            self.ctx.class_field_types.get(attr).and_then(|t| {
                if let Type::List(elem) = t {
                    if !matches!(**elem, Type::Unknown | Type::UnificationVar(_)) {
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
                    if !matches!(**elem, Type::Unknown | Type::UnificationVar(_)) {
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
        }
    }

    /// Check whether the target list holds Vec<DepylerValue> (unknown/unification element type)
    fn is_vec_depyler_value(&self, object: &HirExpr) -> bool {
        if let HirExpr::Attribute { value: _, attr } = object {
            self.ctx
                .class_field_types
                .get(attr)
                .map(|t| matches!(t, Type::List(elem) if matches!(**elem, Type::Unknown | Type::UnificationVar(_))))
                .unwrap_or(false)
        } else if let HirExpr::Var(var_name) = object {
            matches!(
                self.ctx.var_types.get(var_name),
                Some(Type::List(elem)) if matches!(**elem, Type::Unknown | Type::UnificationVar(_))
            )
        } else {
            false
        }
    }

    /// Wrap a push argument in the appropriate DepylerValue variant
    fn wrap_depyler_value_push_arg(&self, arg: &syn::Expr, hir_args: &[HirExpr]) -> syn::Expr {
        if !hir_args.is_empty() {
            match &hir_args[0] {
                HirExpr::Literal(Literal::Int(_)) => {
                    parse_quote! { DepylerValue::Int(#arg as i64) }
                }
                HirExpr::Literal(Literal::Float(_)) => {
                    parse_quote! { DepylerValue::Float(#arg as f64) }
                }
                HirExpr::Literal(Literal::String(_)) => {
                    parse_quote! { DepylerValue::Str(#arg) }
                }
                HirExpr::Literal(Literal::Bool(_)) => {
                    parse_quote! { DepylerValue::Bool(#arg) }
                }
                HirExpr::Var(name) => match self.ctx.var_types.get(name) {
                    Some(Type::Int) => parse_quote! { DepylerValue::Int(#arg as i64) },
                    Some(Type::Float) => parse_quote! { DepylerValue::Float(#arg as f64) },
                    Some(Type::String) => {
                        parse_quote! { DepylerValue::Str(#arg.to_string()) }
                    }
                    Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#arg) },
                    _ => parse_quote! { DepylerValue::Str(format!("{}", #arg)) },
                },
                _ => parse_quote! { DepylerValue::Str(format!("{}", #arg)) },
            }
        } else {
            parse_quote! { DepylerValue::Str(format!("{}", #arg)) }
        }
    }

    /// Check if a push to Vec<String> needs .to_string() conversion
    fn needs_to_string_for_push(&self, object: &HirExpr, hir_args: &[HirExpr]) -> bool {
        if hir_args.is_empty() {
            return false;
        }
        let is_str_literal = matches!(&hir_args[0], HirExpr::Literal(Literal::String(_)));
        let is_char_iter_var = if let HirExpr::Var(name) = &hir_args[0] {
            self.ctx.char_iter_vars.contains(name)
        } else {
            false
        };
        let is_vec_string = if let HirExpr::Var(var_name) = object {
            matches!(
                self.ctx.var_types.get(var_name),
                Some(Type::List(element_type)) if matches!(**element_type, Type::String)
            )
        } else {
            false
        };
        (is_str_literal || is_char_iter_var) && is_vec_string
    }

    /// Check if a push needs .clone() to prevent move errors
    fn needs_clone_for_push(&self, hir_args: &[HirExpr]) -> bool {
        if let Some(HirExpr::Var(name)) = hir_args.first() {
            self.ctx
                .var_types
                .get(name)
                .map(|ty| {
                    matches!(
                        ty,
                        Type::String
                            | Type::List(_)
                            | Type::Dict(_, _)
                            | Type::Set(_)
                            | Type::Custom(_)
                    )
                })
                .unwrap_or(false)
        } else {
            false
        }
    }

    /// Handle pop() for sets, dicts, and lists
    fn convert_list_pop(
        &mut self,
        object_expr: &syn::Expr,
        object: &HirExpr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if arg_exprs.len() == 2 {
            // dict.pop(key, default)
            let key = &arg_exprs[0];
            let default = &arg_exprs[1];
            let needs_ref = self.pop_needs_ref(hir_args);
            if needs_ref {
                Ok(parse_quote! { #object_expr.remove(&#key).unwrap_or(#default) })
            } else {
                Ok(parse_quote! { #object_expr.remove(#key).unwrap_or(#default) })
            }
        } else if arg_exprs.len() > 2 {
            bail!("pop() takes at most 2 arguments");
        } else if self.is_set_expr(object) {
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
            if arg_exprs.len() != 1 {
                bail!("dict literal pop() requires exactly 1 argument (key)");
            }
            let key = &arg_exprs[0];
            let needs_ref = self.pop_needs_ref(hir_args);
            if needs_ref {
                Ok(parse_quote! { #object_expr.remove(&#key).expect("KeyError: key not found") })
            } else {
                Ok(parse_quote! { #object_expr.remove(#key).expect("KeyError: key not found") })
            }
        } else if arg_exprs.is_empty() {
            Ok(parse_quote! { #object_expr.pop().unwrap_or_default() })
        } else {
            self.convert_pop_single_arg(object_expr, object, arg_exprs, hir_args)
        }
    }

    /// Check if pop() key argument needs a reference prefix
    fn pop_needs_ref(&self, hir_args: &[HirExpr]) -> bool {
        !hir_args.is_empty()
            && !matches!(
                hir_args[0],
                HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
            )
    }

    /// Handle pop(arg) with 1 argument — disambiguate list.pop(index) vs dict.pop(key)
    fn convert_pop_single_arg(
        &mut self,
        object_expr: &syn::Expr,
        object: &HirExpr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        let arg = &arg_exprs[0];
        let is_list = self.is_list_expr(object);
        let is_dict = self.is_dict_expr(object);
        let arg_is_int = !hir_args.is_empty()
            && matches!(hir_args[0], HirExpr::Literal(crate::hir::Literal::Int(_)));

        if is_list || (!is_dict && arg_is_int) {
            Ok(parse_quote! { #object_expr.remove(#arg as usize) })
        } else {
            let needs_ref = self.pop_needs_ref(hir_args);
            if needs_ref {
                Ok(parse_quote! { #object_expr.remove(&#arg).expect("KeyError: key not found") })
            } else {
                Ok(parse_quote! { #object_expr.remove(#arg).expect("KeyError: key not found") })
            }
        }
    }

    /// Handle list.sort(key=func, reverse=False)
    fn convert_list_sort(
        &mut self,
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
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
                let key_rust = key_expr.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { #object_expr.sort_by_key(|x| #key_rust(x)) })
            }
            (Some(key_expr), true) => {
                let key_rust = key_expr.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { #object_expr.sort_by_key(|x| std::cmp::Reverse(#key_rust(x))) })
            }
            (None, false) => Ok(parse_quote! { #object_expr.sort() }),
            (None, true) => Ok(parse_quote! { #object_expr.sort_by(|a, b| b.cmp(a)) }),
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
                // DEPYLER-99MODE-S9: Check if arg is a char iteration variable
                // `for char in text.chars()` → char is Rust `char`, needs .to_string()
                let is_char_iter = if let Some(HirExpr::Var(name)) = hir_args.first() {
                    self.ctx.char_iter_vars.contains(name)
                } else {
                    false
                };
                if is_str_literal || is_char_iter {
                    Ok(parse_quote! { #object_expr.push(#arg.to_string()) })
                } else {
                    // DEPYLER-99MODE-S9: Clone String variables to prevent E0382 in loops
                    let is_string_var =
                        hir_args.first().map(|a| matches!(a, HirExpr::Var(_))).unwrap_or(false);
                    if is_string_var {
                        Ok(parse_quote! { #object_expr.push(#arg.clone()) })
                    } else {
                        Ok(parse_quote! { #object_expr.push(#arg) })
                    }
                }
            }
            // DEPYLER-1135: For Vec<int>, push directly without cast
            // The argument should already be typed correctly (i32 by default, or whatever
            // width was inferred). Casting to i64 causes E0308 when the Vec is Vec<i32>.
            // Trust the type system - if there's a mismatch, it's a type inference issue
            // that should be fixed at the source, not papered over with casts.
            Type::Int => Ok(parse_quote! { #object_expr.push(#arg) }),
            // DEPYLER-1135: For Vec<f64>, push directly without cast
            // Same reasoning - trust the type system
            Type::Float => Ok(parse_quote! { #object_expr.push(#arg) }),
            // For Vec<bool>, direct push
            Type::Bool => Ok(parse_quote! { #object_expr.push(#arg) }),
            // For nested Vec<Vec<T>>, push directly (arg should already be Vec<T>)
            Type::List(_inner) => {
                // The arg should be the inner list type already
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            // For Vec<HashMap<K, V>>, push directly
            Type::Dict(_k, _v) => Ok(parse_quote! { #object_expr.push(#arg) }),
            // For custom types, use try_into for safe conversion
            Type::Custom(type_name) => {
                let type_ident: syn::Type = syn::parse_str(type_name)?;
                Ok(
                    parse_quote! { #object_expr.push(<#type_ident>::try_from(#arg).expect("Type conversion failed")) },
                )
            }
            // For Optional types, push directly
            Type::Optional(_inner) => Ok(parse_quote! { #object_expr.push(#arg) }),
            // For Tuple types, push directly
            Type::Tuple(_) => Ok(parse_quote! { #object_expr.push(#arg) }),
            // Fallback: direct push (trust the type system)
            _ => Ok(parse_quote! { #object_expr.push(#arg) }),
        }
    }
}
