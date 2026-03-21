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
            "add" => self.convert_set_add(object_expr, object, arg_exprs, hir_args),
            "remove" => Self::convert_set_remove(object_expr, arg_exprs),
            "discard" => Self::convert_set_discard(object_expr, arg_exprs),
            "clear" => Self::convert_set_clear(object_expr, arg_exprs),
            "update" => Self::convert_set_update(object_expr, arg_exprs),
            "intersection_update" => Self::convert_set_intersection_update(object_expr, arg_exprs),
            "difference_update" => Self::convert_set_difference_update(object_expr, arg_exprs),
            "union" => Self::convert_set_binary_op(object_expr, arg_exprs, "union"),
            "intersection" => Self::convert_set_binary_op(object_expr, arg_exprs, "intersection"),
            "difference" => Self::convert_set_binary_op(object_expr, arg_exprs, "difference"),
            "symmetric_difference" => {
                Self::convert_set_binary_op(object_expr, arg_exprs, "symmetric_difference")
            }
            "issubset" => Self::convert_set_predicate(object_expr, arg_exprs, "is_subset"),
            "issuperset" => Self::convert_set_predicate(object_expr, arg_exprs, "is_superset"),
            "isdisjoint" => Self::convert_set_predicate(object_expr, arg_exprs, "is_disjoint"),
            _ => bail!("Unknown set method: {}", method),
        }
    }

    fn convert_set_add(
        &mut self,
        object_expr: &syn::Expr,
        object: &HirExpr,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("add() requires exactly one argument");
        }
        let arg = &arg_exprs[0];

        let is_set_depyler_value = if let HirExpr::Var(var_name) = object {
            matches!(
                self.ctx.var_types.get(var_name),
                Some(Type::Set(elem)) if matches!(**elem, Type::Unknown | Type::UnificationVar(_))
            )
        } else {
            false
        };

        if is_set_depyler_value && !hir_args.is_empty() {
            let wrapped_arg = self.wrap_depyler_value_for_set(arg, &hir_args[0]);
            self.ctx.needs_depyler_value_enum = true;
            return Ok(parse_quote! { #object_expr.insert(#wrapped_arg) });
        }

        if !hir_args.is_empty() {
            if let HirExpr::Literal(Literal::String(s)) = &hir_args[0] {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #object_expr.insert(#lit.to_string()) });
            }
        }

        Ok(parse_quote! { #object_expr.insert(#arg) })
    }

    fn wrap_depyler_value_for_set(&self, arg: &syn::Expr, hir_arg: &HirExpr) -> syn::Expr {
        match hir_arg {
            HirExpr::Literal(Literal::Int(_)) => parse_quote! { DepylerValue::Int(#arg as i64) },
            HirExpr::Literal(Literal::Float(_)) => {
                parse_quote! { DepylerValue::Float(#arg as f64) }
            }
            HirExpr::Literal(Literal::String(_)) => {
                parse_quote! { DepylerValue::Str(#arg.to_string()) }
            }
            HirExpr::Literal(Literal::Bool(_)) => parse_quote! { DepylerValue::Bool(#arg) },
            HirExpr::Var(name) => self.wrap_depyler_value_for_var(arg, name),
            _ => parse_quote! { DepylerValue::from(#arg) },
        }
    }

    fn wrap_depyler_value_for_var(&self, arg: &syn::Expr, name: &str) -> syn::Expr {
        match self.ctx.var_types.get(name) {
            Some(Type::Int) => parse_quote! { DepylerValue::Int(#arg as i64) },
            Some(Type::Float) => parse_quote! { DepylerValue::Float(#arg as f64) },
            Some(Type::String) => parse_quote! { DepylerValue::Str(#arg.to_string()) },
            Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#arg) },
            _ => parse_quote! { DepylerValue::from(#arg) },
        }
    }

    fn convert_set_remove(object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("remove() requires exactly one argument");
        }
        let arg = &arg_exprs[0];
        let is_str_lit = matches!(arg, syn::Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Str(_)));
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

    fn convert_set_discard(object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("discard() requires exactly one argument");
        }
        let arg = &arg_exprs[0];
        let is_str_lit = matches!(arg, syn::Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Str(_)));
        if is_str_lit {
            Ok(parse_quote! { #object_expr.remove(#arg) })
        } else {
            Ok(parse_quote! { #object_expr.remove(&#arg) })
        }
    }

    fn convert_set_clear(object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("clear() takes no arguments");
        }
        Ok(parse_quote! { #object_expr.clear() })
    }

    fn convert_set_update(object_expr: &syn::Expr, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
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

    fn convert_set_intersection_update(
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("intersection_update() requires exactly one argument");
        }
        let other = &arg_exprs[0];
        Ok(parse_quote! {
            {
                let filtered: std::collections::HashSet<_> = #object_expr.intersection(&#other).cloned().collect();
                #object_expr.clear();
                #object_expr.extend(filtered);
            }
        })
    }

    fn convert_set_difference_update(
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("difference_update() requires exactly one argument");
        }
        let other = &arg_exprs[0];
        Ok(parse_quote! {
            {
                let filtered: std::collections::HashSet<_> = #object_expr.difference(&#other).cloned().collect();
                #object_expr.clear();
                #object_expr.extend(filtered);
            }
        })
    }

    fn convert_set_binary_op(
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        op_name: &str,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("{}() requires exactly one argument", op_name);
        }
        let other = &arg_exprs[0];
        let method_ident = syn::Ident::new(op_name, proc_macro2::Span::call_site());
        Ok(parse_quote! {
            #object_expr.#method_ident(&#other).cloned().collect::<std::collections::HashSet<_>>()
        })
    }

    fn convert_set_predicate(
        object_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        rust_method: &str,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("{}() requires exactly one argument", rust_method);
        }
        let other = &arg_exprs[0];
        let method_ident = syn::Ident::new(rust_method, proc_macro2::Span::call_site());
        Ok(parse_quote! {
            #object_expr.#method_ident(&#other)
        })
    }
}
