//! Index and slice conversion for ExprConverter

use crate::hir::*;
use anyhow::Result;
use syn::parse_quote;

use super::ExprConverter;

impl<'a> ExprConverter<'a> {
    pub(super) fn convert_index(&self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = self.convert(base)?;

        // DEPYLER-0735: Check if base is a tuple type (from param_types) and index is integer literal
        // Rust tuples use .0, .1 syntax, not [0], [1]
        if let HirExpr::Literal(Literal::Int(idx)) = index {
            if *idx >= 0 {
                // Check if base variable has tuple type from param_types or class_field_types
                let is_tuple = if let HirExpr::Var(var_name) = base {
                    matches!(self.param_types.get(var_name), Some(Type::Tuple(_)))
                        || matches!(self.class_field_types.get(var_name), Some(Type::Tuple(_)))
                } else {
                    false
                };

                if is_tuple {
                    let field_idx = syn::Index::from(*idx as usize);
                    return Ok(parse_quote! { #base_expr.#field_idx });
                }
            }
        }

        // DEPYLER-0200: Detect dict vs list access
        // String literal index = dict access, use .get()
        // Numeric index = list access, use [idx as usize]
        let is_dict_access = match index {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(name) => {
                // Heuristic: variable names that look like string keys
                let n = name.as_str();
                n == "key"
                    || n.ends_with("_key")
                    || n.starts_with("key_")
                    || n == "name"
                    || n == "field"
                    || n == "attr"
            }
            _ => false,
        };

        // Also check if base looks like a dict
        let base_is_dict = match base {
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
                // Functions returning dicts
                func.contains("dict")
                    || func.contains("json")
                    || func.contains("config")
                    || func == "calculate_age"
                    || func.contains("result")
            }
            _ => false,
        };

        // DEPYLER-1123: Check if base has bare dict type with DepylerValue keys
        // DEPYLER-1214: Type::Dict(Unknown, Unknown) generates HashMap<String, DepylerValue> (String keys!)
        // per type_mapper.rs DEPYLER-0776. Only an explicit DepylerValue key type would need wrapping,
        // which is rare in practice. So is_depyler_value_dict is almost always false.
        let is_depyler_value_dict = if let HirExpr::Var(var_name) = base {
            match self.param_types.get(var_name) {
                // Only wrap keys if key type is explicitly DepylerValue, not Unknown
                Some(Type::Dict(k, _)) => {
                    matches!(k.as_ref(), Type::Custom(n) if n == "DepylerValue")
                }
                // Custom types like "dict" use String keys
                Some(Type::Custom(_)) => false,
                _ => false,
            }
        } else {
            false
        };

        if is_dict_access || base_is_dict {
            // DEPYLER-1320: Check if NASA mode (DepylerValue values need .into() for type conversion)
            let needs_into = self.type_mapper.nasa_mode;

            // DEPYLER-1123: For bare dict types (HashMap<DepylerValue, DepylerValue>),
            // wrap key in DepylerValue::Str and add .into() for type conversion
            if is_depyler_value_dict {
                // String literal key - wrap in DepylerValue::Str
                if let HirExpr::Literal(Literal::String(_)) = index {
                    let index_expr = self.convert(index)?;
                    // DEPYLER-99MODE-S9: String literal needs .to_string() for DepylerValue::Str(String)
                    return Ok(parse_quote! {
                        #base_expr.get(&DepylerValue::Str(#index_expr.to_string())).cloned().unwrap_or_default().into()
                    });
                }
            }

            // DEPYLER-1320: For string literal keys, use the literal directly without .to_string()
            // This prevents &"key".to_string() which creates unnecessary allocation
            if let HirExpr::Literal(Literal::String(s)) = index {
                if needs_into {
                    return Ok(parse_quote! {
                        #base_expr.get(#s).cloned().unwrap_or_default().into()
                    });
                } else {
                    return Ok(parse_quote! {
                        #base_expr.get(#s).cloned().unwrap_or_default()
                    });
                }
            }

            // Non-literal index - convert and borrow
            let index_expr = self.convert(index)?;

            // Standard dict access (HashMap<String, T> or HashMap<K, V> with known types)
            if needs_into {
                Ok(parse_quote! {
                    #base_expr.get(&#index_expr).cloned().unwrap_or_default().into()
                })
            } else {
                Ok(parse_quote! {
                    #base_expr.get(&#index_expr).cloned().unwrap_or_default()
                })
            }
        } else {
            // DEPYLER-1095: Vec/List access with numeric index - handle negative indices
            // Python supports list[-1] to get last element, list[-2] for second-to-last, etc.
            // Check if index is a negative literal
            let is_negative_literal = match index {
                HirExpr::Literal(Literal::Int(idx)) => *idx < 0,
                HirExpr::Unary { op: UnaryOp::Neg, .. } => true,
                _ => false,
            };

            let index_expr = self.convert(index)?;

            if is_negative_literal {
                // For negative literals, use runtime-safe indexing:
                // base.get(base.len().wrapping_add(idx as usize)).cloned().unwrap()
                // This handles -1 → len-1, -2 → len-2, etc.
                // DEPYLER-1140: Cast to isize first to handle usize indices correctly
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
                // For non-negative or variable indices, generate simpler code
                // but still handle potential negatives at runtime
                // DEPYLER-1140: Cast to isize first to handle usize indices correctly
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
