//! Index and slice conversion for ExprConverter

use crate::hir::*;
use anyhow::Result;
use syn::parse_quote;

use super::ExprConverter;

impl<'a> ExprConverter<'a> {
    pub(super) fn convert_index(&self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = self.convert(base)?;

        if let Some(tuple_access) = self.try_tuple_index(base, index, &base_expr)? {
            return Ok(tuple_access);
        }

        let is_dict_access = self.index_looks_like_dict_key(index);
        let base_is_dict = self.base_looks_like_dict(base);
        let is_depyler_value_dict = self.base_has_depyler_value_keys(base);

        if is_dict_access || base_is_dict {
            self.convert_dict_access(index, &base_expr, is_depyler_value_dict)
        } else {
            self.convert_vec_access(index, &base_expr)
        }
    }

    fn try_tuple_index(
        &self,
        base: &HirExpr,
        index: &HirExpr,
        base_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        let HirExpr::Literal(Literal::Int(idx)) = index else {
            return Ok(None);
        };
        if *idx < 0 {
            return Ok(None);
        }
        let is_tuple = if let HirExpr::Var(var_name) = base {
            matches!(self.param_types.get(var_name), Some(Type::Tuple(_)))
                || matches!(self.class_field_types.get(var_name), Some(Type::Tuple(_)))
        } else {
            false
        };
        if !is_tuple {
            return Ok(None);
        }
        let field_idx = syn::Index::from(*idx as usize);
        Ok(Some(parse_quote! { #base_expr.#field_idx }))
    }

    fn index_looks_like_dict_key(&self, index: &HirExpr) -> bool {
        match index {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(name) => {
                let n = name.as_str();
                n == "key"
                    || n.ends_with("_key")
                    || n.starts_with("key_")
                    || n == "name"
                    || n == "field"
                    || n == "attr"
            }
            _ => false,
        }
    }

    fn base_looks_like_dict(&self, base: &HirExpr) -> bool {
        match base {
            HirExpr::Var(name) => {
                let n = name.as_str();
                n.contains("dict")
                    || n.contains("map")
                    || n.contains("data")
                    || n == "result"
                    || n == "config"
                    || n == "settings"
                    || n == "params"
                    || n == "options"
                    || n == "env"
            }
            HirExpr::Call { func, .. } => {
                func.contains("dict")
                    || func.contains("json")
                    || func.contains("config")
                    || func == "calculate_age"
                    || func.contains("result")
            }
            _ => false,
        }
    }

    fn base_has_depyler_value_keys(&self, base: &HirExpr) -> bool {
        let HirExpr::Var(var_name) = base else {
            return false;
        };
        match self.param_types.get(var_name) {
            Some(Type::Dict(k, _)) => {
                matches!(k.as_ref(), Type::Custom(n) if n == "DepylerValue")
            }
            _ => false,
        }
    }

    fn convert_dict_access(
        &self,
        index: &HirExpr,
        base_expr: &syn::Expr,
        is_depyler_value_dict: bool,
    ) -> Result<syn::Expr> {
        let needs_into = self.type_mapper.nasa_mode;

        if is_depyler_value_dict {
            if let HirExpr::Literal(Literal::String(_)) = index {
                let index_expr = self.convert(index)?;
                return Ok(parse_quote! {
                    #base_expr.get(&DepylerValue::Str(#index_expr.to_string())).cloned().unwrap_or_default().into()
                });
            }
        }

        if let HirExpr::Literal(Literal::String(s)) = index {
            return if needs_into {
                Ok(parse_quote! { #base_expr.get(#s).cloned().unwrap_or_default().into() })
            } else {
                Ok(parse_quote! { #base_expr.get(#s).cloned().unwrap_or_default() })
            };
        }

        let index_expr = self.convert(index)?;
        if needs_into {
            Ok(parse_quote! { #base_expr.get(&#index_expr).cloned().unwrap_or_default().into() })
        } else {
            Ok(parse_quote! { #base_expr.get(&#index_expr).cloned().unwrap_or_default() })
        }
    }

    fn convert_vec_access(&self, index: &HirExpr, base_expr: &syn::Expr) -> Result<syn::Expr> {
        let is_negative_literal = match index {
            HirExpr::Literal(Literal::Int(idx)) => *idx < 0,
            HirExpr::Unary { op: UnaryOp::Neg, .. } => true,
            _ => false,
        };

        let index_expr = self.convert(index)?;

        if is_negative_literal {
            Ok(parse_quote! {
                {
                    let _base = &#base_expr;
                    let _idx = (#index_expr) as isize;
                    let _actual_idx = if _idx < 0 {
                        _base.len().wrapping_sub((-_idx) as usize)
                    } else {
                        _idx as usize
                    };
                    _base[_actual_idx].clone()
                }
            })
        } else {
            Ok(parse_quote! {
                {
                    let _base = &#base_expr;
                    let _idx = (#index_expr) as isize;
                    if _idx < 0 {
                        _base[_base.len().wrapping_sub((-_idx) as usize)].clone()
                    } else {
                        _base[_idx as usize].clone()
                    }
                }
            })
        }
    }

    /// DEPYLER-1177: Check if expression has a List type (for proper slice codegen)
    pub(super) fn is_list_type(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                matches!(self.param_types.get(name), Some(Type::List(_)) | Some(Type::Set(_)))
                    || matches!(
                        self.class_field_types.get(name),
                        Some(Type::List(_)) | Some(Type::Set(_))
                    )
            }
            HirExpr::Attribute { value, attr } => {
                if matches!(value.as_ref(), HirExpr::Var(name) if name == "self") {
                    matches!(
                        self.class_field_types.get(attr),
                        Some(Type::List(_)) | Some(Type::Set(_))
                    )
                } else {
                    false
                }
            }
            HirExpr::List(_) => true,
            _ => false,
        }
    }

    /// DEPYLER-0596: Convert slice expression (e.g., value[1:-1])
    pub(super) fn convert_slice(
        &self,
        base: &HirExpr,
        start: &Option<Box<HirExpr>>,
        stop: &Option<Box<HirExpr>>,
        step: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let base_expr = self.convert(base)?;

        // DEPYLER-1177: Check if base is a Vec type - use Vec slicing instead of String
        if self.is_list_type(base) {
            return self.convert_vec_slice(base_expr, start, stop, step);
        }

        // Convert start/stop/step expressions
        let start_expr = start.as_ref().map(|e| self.convert(e)).transpose()?;
        let stop_expr = stop.as_ref().map(|e| self.convert(e)).transpose()?;
        let _step_expr = step.as_ref().map(|e| self.convert(e)).transpose()?;

        // For strings: use chars().skip().take() pattern with negative index handling
        // This handles cases like value[1:-1] (remove first and last chars)
        match (start_expr, stop_expr) {
            (Some(start), Some(stop)) => {
                // value[start:stop] - handles negative indices
                // DEPYLER-0603: Wrap expressions in parens to ensure proper type casting
                // Without parens, `a + b as isize` parses as `a + (b as isize)`
                Ok(parse_quote! {
                    {
                        let s = &#base_expr;
                        let len = s.chars().count() as isize;
                        let start_idx = (#start) as isize;
                        let stop_idx = (#stop) as isize;
                        let start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        let stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx as usize
                        };
                        if stop > start {
                            s.chars().skip(start).take(stop - start).collect::<String>()
                        } else {
                            String::new()
                        }
                    }
                })
            }
            (Some(start), None) => {
                // value[start:] - from start to end
                // DEPYLER-0603: Wrap expression in parens for type casting
                Ok(parse_quote! {
                    {
                        let s = &#base_expr;
                        let len = s.chars().count() as isize;
                        let start_idx = (#start) as isize;
                        let start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        s.chars().skip(start).collect::<String>()
                    }
                })
            }
            (None, Some(stop)) => {
                // value[:stop] - from beginning to stop
                // DEPYLER-0603: Wrap expression in parens for type casting
                Ok(parse_quote! {
                    {
                        let s = &#base_expr;
                        let len = s.chars().count() as isize;
                        let stop_idx = (#stop) as isize;
                        let stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx as usize
                        };
                        s.chars().take(stop).collect::<String>()
                    }
                })
            }
            (None, None) => {
                // value[:] - full clone
                Ok(parse_quote! { #base_expr.clone() })
            }
        }
    }

    /// DEPYLER-1177: Convert Vec slice expression (e.g., list[start:stop])
    pub(super) fn convert_vec_slice(
        &self,
        base_expr: syn::Expr,
        start: &Option<Box<HirExpr>>,
        stop: &Option<Box<HirExpr>>,
        _step: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let start_expr = start.as_ref().map(|e| self.convert(e)).transpose()?;
        let stop_expr = stop.as_ref().map(|e| self.convert(e)).transpose()?;

        match (start_expr, stop_expr) {
            (Some(start), Some(stop)) => {
                // list[start:stop] - handles negative indices
                Ok(parse_quote! {
                    {
                        let v = &#base_expr;
                        let len = v.len() as isize;
                        let start_idx = (#start) as isize;
                        let stop_idx = (#stop) as isize;
                        let start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        let stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx.min(len) as usize
                        };
                        if stop > start {
                            v[start..stop].to_vec()
                        } else {
                            vec![]
                        }
                    }
                })
            }
            (Some(start), None) => {
                // list[start:] - from start to end
                Ok(parse_quote! {
                    {
                        let v = &#base_expr;
                        let len = v.len() as isize;
                        let start_idx = (#start) as isize;
                        let start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        v[start..].to_vec()
                    }
                })
            }
            (None, Some(stop)) => {
                // list[:stop] - from beginning to stop
                Ok(parse_quote! {
                    {
                        let v = &#base_expr;
                        let len = v.len() as isize;
                        let stop_idx = (#stop) as isize;
                        let stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx.min(len) as usize
                        };
                        v[..stop].to_vec()
                    }
                })
            }
            (None, None) => {
                // list[:] - full clone
                Ok(parse_quote! { #base_expr.clone() })
            }
        }
    }
}
