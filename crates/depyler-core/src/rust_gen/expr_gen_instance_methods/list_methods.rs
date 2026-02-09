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
            "append" => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];

                // DEPYLER-1134: Constraint-Aware Coercion
                // Check for CONCRETE element type FIRST (from Oracle or type annotations)
                // Only fall back to DepylerValue wrapping if type is truly Unknown
                // DEPYLER-1207: Pattern matching correction - use **elem to dereference Box
                // DEPYLER-1209: Also treat UnificationVar as non-concrete (needs DepylerValue)
                let concrete_element_type = if let HirExpr::Attribute { value: _, attr } = object {
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
                };

                // DEPYLER-1134: If we have a concrete element type, generate type-aware push
                // This is the "bridge" that respects Oracle/annotation types
                if let Some(elem_type) = concrete_element_type {
                    // Generate the appropriate push based on element type
                    let push_expr =
                        self.generate_typed_push(object_expr, arg, &elem_type, hir_args)?;
                    return Ok(push_expr);
                }

                // DEPYLER-1051: Check if target is Vec<DepylerValue> (e.g., untyped class field)
                // If so, wrap the argument in appropriate DepylerValue variant
                // DEPYLER-1207: Pattern matching correction - elem.as_ref() returns &Type,
                // so we need to dereference with *elem.as_ref() or use &Type::Unknown pattern
                // DEPYLER-1209: Also check for UnificationVar which indicates incomplete inference
                let is_vec_depyler_value = if let HirExpr::Attribute { value: _, attr } = object {
                    // Check class field type
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
                };

                if is_vec_depyler_value {
                    // DEPYLER-1051: Wrap argument in DepylerValue based on argument type
                    // DEPYLER-1210: Avoid double .to_string() - arg_exprs already converts literals
                    let wrapped_arg: syn::Expr = if !hir_args.is_empty() {
                        match &hir_args[0] {
                            HirExpr::Literal(Literal::Int(_)) => {
                                parse_quote! { DepylerValue::Int(#arg as i64) }
                            }
                            HirExpr::Literal(Literal::Float(_)) => {
                                parse_quote! { DepylerValue::Float(#arg as f64) }
                            }
                            // String literals are already converted to String by arg_exprs, just wrap
                            HirExpr::Literal(Literal::String(_)) => {
                                parse_quote! { DepylerValue::Str(#arg) }
                            }
                            HirExpr::Literal(Literal::Bool(_)) => {
                                parse_quote! { DepylerValue::Bool(#arg) }
                            }
                            HirExpr::Var(name) => {
                                // Check variable type
                                match self.ctx.var_types.get(name) {
                                    Some(Type::Int) => {
                                        parse_quote! { DepylerValue::Int(#arg as i64) }
                                    }
                                    Some(Type::Float) => {
                                        parse_quote! { DepylerValue::Float(#arg as f64) }
                                    }
                                    Some(Type::String) => {
                                        parse_quote! { DepylerValue::Str(#arg.to_string()) }
                                    }
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

                    // DEPYLER-99MODE-S9: Check if argument is a char iteration variable
                    // When iterating over string chars and pushing to Vec<String>,
                    // the char needs .to_string() conversion
                    let is_char_iter_var = if let HirExpr::Var(name) = &hir_args[0] {
                        self.ctx.char_iter_vars.contains(name)
                    } else {
                        false
                    };

                    // Check if object is a Vec<String> by examining variable type
                    let is_vec_string = if let HirExpr::Var(var_name) = object {
                        matches!(
                            self.ctx.var_types.get(var_name),
                            Some(Type::List(element_type)) if matches!(**element_type, Type::String)
                        )
                    } else {
                        false
                    };

                    (is_str_literal || is_char_iter_var) && is_vec_string
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
                        // list.sort(key=func) -> list.sort_by_key(|x| func(x))
                        // Convert key_expr to Rust callable
                        let key_rust = key_expr.to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { #object_expr.sort_by_key(|x| #key_rust(x)) })
                    }
                    (Some(key_expr), true) => {
                        // list.sort(key=func, reverse=True) -> list.sort_by_key(|x| std::cmp::Reverse(func(x)))
                        let key_rust = key_expr.to_rust_expr(self.ctx)?;
                        Ok(
                            parse_quote! { #object_expr.sort_by_key(|x| std::cmp::Reverse(#key_rust(x))) },
                        )
                    }
                    (None, false) => {
                        // list.sort() -> list.sort()
                        Ok(parse_quote! { #object_expr.sort() })
                    }
                    (None, true) => {
                        // list.sort(reverse=True) -> list.sort_by(|a, b| b.cmp(a))
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
                    // Assume arg is already String type
                    Ok(parse_quote! { #object_expr.push(#arg) })
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
