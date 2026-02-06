//! Set method handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size and improve maintainability.
//! Contains handlers for: add, remove, discard, clear, update,
//! intersection_update, difference_update, union, intersection,
//! difference, symmetric_difference, issubset, issuperset, isdisjoint.

use crate::hir::*;
use crate::rust_gen::expr_gen::ExpressionConverter;
use anyhow::{bail, Result};
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Handle set methods (add, discard, clear)
    #[inline]
    pub(super) fn convert_set_method(
        &mut self,
        object_expr: &syn::Expr,
        object: &HirExpr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        match method {
            "add" => {
                if arg_exprs.len() != 1 {
                    bail!("add() requires exactly one argument");
                }
                let arg = &arg_exprs[0];

                // DEPYLER-FIX: Check if target is HashSet<DepylerValue>
                // If so, wrap the argument in appropriate DepylerValue variant
                // DEPYLER-1207: Pattern matching correction - use **elem to dereference Box
                // DEPYLER-1209: Also check for UnificationVar
                let is_set_depyler_value = if let HirExpr::Var(var_name) = object {
                    matches!(
                        self.ctx.var_types.get(var_name),
                        Some(Type::Set(elem)) if matches!(**elem, Type::Unknown | Type::UnificationVar(_))
                    )
                } else {
                    false
                };

                if is_set_depyler_value && !hir_args.is_empty() {
                    // Wrap argument in DepylerValue based on argument type
                    let wrapped_arg: syn::Expr = match &hir_args[0] {
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
                            Some(Type::String) => {
                                parse_quote! { DepylerValue::Str(#arg.to_string()) }
                            }
                            Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#arg) },
                            _ => parse_quote! { DepylerValue::from(#arg) },
                        },
                        _ => parse_quote! { DepylerValue::from(#arg) },
                    };
                    self.ctx.needs_depyler_value_enum = true;
                    return Ok(parse_quote! { #object_expr.insert(#wrapped_arg) });
                }

                // DEPYLER-SET-ADD-STR-FIX: Convert string literals to owned Strings for HashSet<String>.add()
                // Python: fruits.add("cherry") → Rust: fruits.insert("cherry".to_string())
                if !hir_args.is_empty() {
                    if let HirExpr::Literal(Literal::String(s)) = &hir_args[0] {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        return Ok(parse_quote! { #object_expr.insert(#lit.to_string()) });
                    }
                }

                Ok(parse_quote! { #object_expr.insert(#arg) })
            }
            "remove" => {
                // DEPYLER-0224: Set.remove(value) - remove value or panic if not found
                // DEPYLER-E0277-FIX: String literals are already &str, other values need &
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // Check if arg is a string literal (already a reference)
                let is_str_lit =
                    matches!(arg, syn::Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Str(_)));
                if is_str_lit {
                    Ok(parse_quote! {
                        if !#object_expr.remove(#arg) {
                            panic!("KeyError: element not in set")
                        }
                    })
                } else {
                    Ok(parse_quote! {
                        if !#object_expr.remove(&#arg) {
                            panic!("KeyError: element not in set")
                        }
                    })
                }
            }
            "discard" => {
                if arg_exprs.len() != 1 {
                    bail!("discard() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // DEPYLER-E0277-FIX: String literals are already &str, other values need &
                let is_str_lit =
                    matches!(arg, syn::Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Str(_)));
                if is_str_lit {
                    Ok(parse_quote! { #object_expr.remove(#arg) })
                } else {
                    Ok(parse_quote! { #object_expr.remove(&#arg) })
                }
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
}
