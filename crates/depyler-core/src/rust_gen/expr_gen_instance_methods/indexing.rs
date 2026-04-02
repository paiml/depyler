//! Indexing handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size. Contains `convert_index`.
//! Type-checking helpers (is_string_index, is_string_base, etc.) live in type_helpers.rs.
#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::trace_decision;
use anyhow::Result;
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
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
        if let HirExpr::Attribute { value, attr } = base {
            if let HirExpr::Var(module_name) = &**value {
                if module_name == "os" && attr == "environ" {
                    let index_expr = index.to_rust_expr(self.ctx)?;
                    return Ok(parse_quote! { std::env::var(#index_expr).unwrap_or_default() });
                }
            }
        }

        // DEPYLER-0964: Handle subscript access on &mut Option<HashMap<K, V>> parameters
        if let HirExpr::Var(var_name) = base {
            if self.ctx.mut_option_dict_params.contains(var_name) {
                let base_ident = crate::rust_gen::keywords::safe_ident(var_name);
                let index_expr = index.to_rust_expr(self.ctx)?;
                return Ok(parse_quote! {
                    #base_ident.as_ref().expect("value is None").get(&#index_expr).cloned().unwrap_or_default()
                });
            }
        }

        let mut base_expr = base.to_rust_expr(self.ctx)?;

        // DEPYLER-0270: Auto-unwrap Result-returning function calls
        if let HirExpr::Call { func, .. } = base {
            if self.ctx.result_returning_functions.contains(func) {
                base_expr = parse_quote! { #base_expr? };
            }
        }

        // DEPYLER-1106: Use PyOps trait methods for DepylerValue indexing
        let base_is_depyler = self.expr_returns_depyler_value(base);
        if base_is_depyler && self.ctx.type_mapper.nasa_mode {
            return self.convert_depyler_value_index(base, index, &base_expr);
        }

        // DEPYLER-1145: Wrap index in DepylerValue if base is HashMap<DepylerValue, ...>
        if let Some(result) = self.try_convert_depyler_key_index(base, index, &base_expr)? {
            return Ok(result);
        }

        // DEPYLER-0422: Handle tuple indexing with actual type information
        if let Some(result) = self.try_convert_tuple_index(base, index, &base_expr)? {
            return Ok(result);
        }

        // DEPYLER-0299 Pattern #3 FIX: Check if base is a String type for character access
        let is_string_base = self.is_string_base(base);

        // Discriminate between HashMap and Vec access based on base type or index type
        let is_string_key = self.is_string_index(base, index)?;

        if is_string_key {
            self.convert_string_key_index(base, index, &base_expr)
        } else if is_string_base {
            self.convert_string_char_index(index, &base_expr)
        } else {
            self.convert_numeric_index(base, index, &base_expr)
        }
    }

    /// Handle DepylerValue indexing (NASA mode) with PyOps trait methods
    fn convert_depyler_value_index(
        &mut self,
        _base: &HirExpr,
        index: &HirExpr,
        base_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        // DEPYLER-1316: Check for string literal keys - use get_str() for dict access
        if let HirExpr::Literal(Literal::String(s)) = index {
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            return Ok(
                parse_quote! { #base_expr.get_str(#lit).cloned().unwrap_or_default().into() },
            );
        }
        // DEPYLER-1316: Check for string variable keys - use get_str() for dict access
        if let HirExpr::Var(var_name) = index {
            let var_type = self.ctx.var_types.get(var_name);
            let is_string_var = matches!(var_type, Some(Type::String) | None);
            if is_string_var {
                let var_ident = crate::rust_gen::keywords::safe_ident(var_name);
                return Ok(
                    parse_quote! { #base_expr.get_str(&#var_ident).cloned().unwrap_or_default().into() },
                );
            }
        }
        // For numeric indices, use py_index for negative index support
        let index_expr = index.to_rust_expr(self.ctx)?;
        let index_for_pyops = if matches!(index, HirExpr::Literal(Literal::Int(_))) {
            parse_quote! { DepylerValue::Int(#index_expr as i64) }
        } else if self.expr_returns_depyler_value(index) {
            index_expr.clone()
        } else {
            parse_quote! { DepylerValue::Int(#index_expr as i64) }
        };
        Ok(parse_quote! { #base_expr.clone().py_index(#index_for_pyops) })
    }

    /// Try to handle HashMap<DepylerValue, ...> key wrapping
    fn try_convert_depyler_key_index(
        &mut self,
        base: &HirExpr,
        index: &HirExpr,
        base_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        let is_depyler_value_key = if let HirExpr::Var(var_name) = base {
            self.ctx.var_types.get(var_name).is_some_and(|t| {
                matches!(t, Type::Dict(key_type, _) if matches!(key_type.as_ref(), Type::Custom(n) if n == "DepylerValue"))
            })
        } else {
            false
        };

        if !is_depyler_value_key {
            return Ok(None);
        }

        let index_expr = index.to_rust_expr(self.ctx)?;

        // Wrap index in DepylerValue
        let wrapped_index: syn::Expr = match index {
            HirExpr::Literal(Literal::String(s)) => {
                parse_quote! { &DepylerValue::Str(#s.to_string()) }
            }
            HirExpr::Literal(Literal::Int(i)) => {
                parse_quote! { &DepylerValue::Int(#i) }
            }
            HirExpr::Literal(Literal::Float(f)) => {
                parse_quote! { &DepylerValue::Float(#f) }
            }
            HirExpr::Literal(Literal::Bool(b)) => {
                parse_quote! { &DepylerValue::Bool(#b) }
            }
            _ => {
                parse_quote! { &DepylerValue::Str(format!("{:?}", #index_expr)) }
            }
        };
        Ok(Some(parse_quote! { #base_expr.get(#wrapped_index).cloned().unwrap_or_default() }))
    }

    /// Try to handle tuple indexing (tuple.0, tuple.1 syntax)
    fn try_convert_tuple_index(
        &mut self,
        base: &HirExpr,
        index: &HirExpr,
        base_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        let should_use_tuple_syntax = if let HirExpr::Literal(Literal::Int(idx)) = index {
            if *idx >= 0 {
                if let HirExpr::Var(var_name) = base {
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        matches!(var_type, Type::Tuple(_))
                            || matches!(var_type, Type::Optional(inner) if matches!(&**inner, Type::Tuple(_)))
                    } else {
                        matches!(
                            var_name.as_str(),
                            "pair" | "entry" | "item" | "elem" | "tuple" | "row"
                        )
                    }
                } else if let HirExpr::Index { base: inner_base, .. } = base {
                    if let HirExpr::Var(var_name) = &**inner_base {
                        if let Some(Type::List(element_type)) = self.ctx.var_types.get(var_name) {
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

        if !should_use_tuple_syntax {
            return Ok(None);
        }

        if let HirExpr::Literal(Literal::Int(idx)) = index {
            let field_idx = syn::Index::from(*idx as usize);
            // DEPYLER-99MODE-S9: Unwrap Option before tuple field access
            if let HirExpr::Var(var_name) = base {
                if let Some(Type::Optional(inner)) = self.ctx.var_types.get(var_name) {
                    if matches!(&**inner, Type::Tuple(_)) {
                        return Ok(Some(parse_quote! { #base_expr.unwrap().#field_idx }));
                    }
                }
            }
            return Ok(Some(parse_quote! { #base_expr.#field_idx }));
        }

        Ok(None)
    }

    /// Handle HashMap/Dict access with string keys
    fn convert_string_key_index(
        &mut self,
        base: &HirExpr,
        index: &HirExpr,
        base_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        let base_is_depyler_value = self.expr_returns_depyler_value(base);
        let needs_into = base_is_depyler_value || self.dict_has_depyler_value_values(base);

        match index {
            HirExpr::Literal(Literal::String(s)) => {
                self.convert_string_lit_key(base_is_depyler_value, needs_into, s, base_expr)
            }
            _ => {
                // DEPYLER-1320: Defensive check for string literals that reach this branch
                if let HirExpr::Literal(Literal::String(s)) = index {
                    return self.convert_string_lit_key(
                        base_is_depyler_value,
                        needs_into,
                        s,
                        base_expr,
                    );
                }

                let index_expr = index.to_rust_expr(self.ctx)?;
                let needs_borrow = self.needs_key_borrow(index);

                self.emit_dict_get(
                    base_is_depyler_value,
                    needs_into,
                    needs_borrow,
                    &index_expr,
                    base_expr,
                )
            }
        }
    }

    /// Emit dict.get() for a string literal key
    fn convert_string_lit_key(
        &self,
        base_is_depyler_value: bool,
        needs_into: bool,
        s: &str,
        base_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        if base_is_depyler_value && self.ctx.type_mapper.nasa_mode {
            Ok(parse_quote! { #base_expr.get_str(#s).cloned().unwrap_or_default().into() })
        } else if needs_into {
            Ok(parse_quote! { #base_expr.get(#s).cloned().unwrap_or_default().into() })
        } else {
            Ok(parse_quote! { #base_expr.get(#s).cloned().unwrap_or_default() })
        }
    }

    /// Determine whether a dict key expression needs borrowing
    fn needs_key_borrow(&self, index: &HirExpr) -> bool {
        if let HirExpr::Var(var_name) = index {
            if self.is_borrowed_str_param(var_name) {
                false
            } else if matches!(self.ctx.var_types.get(var_name), Some(Type::String)) {
                true
            } else {
                true
            }
        } else {
            true
        }
    }

    /// Emit the appropriate dict.get() call with borrow/into handling
    fn emit_dict_get(
        &self,
        base_is_depyler_value: bool,
        needs_into: bool,
        needs_borrow: bool,
        index_expr: &syn::Expr,
        base_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        if base_is_depyler_value && self.ctx.type_mapper.nasa_mode {
            if needs_borrow {
                Ok(parse_quote! {
                    #base_expr.get_str(&#index_expr).cloned().unwrap_or_default().into()
                })
            } else {
                Ok(parse_quote! {
                    #base_expr.get_str(#index_expr).cloned().unwrap_or_default().into()
                })
            }
        } else if needs_into {
            if needs_borrow {
                Ok(parse_quote! {
                    #base_expr.get(&(#index_expr)).cloned().unwrap_or_default().into()
                })
            } else {
                Ok(parse_quote! {
                    #base_expr.get(#index_expr).cloned().unwrap_or_default().into()
                })
            }
        } else if needs_borrow {
            Ok(parse_quote! {
                #base_expr.get(&(#index_expr)).cloned().unwrap_or_default()
            })
        } else {
            Ok(parse_quote! {
                #base_expr.get(#index_expr).cloned().unwrap_or_default()
            })
        }
    }

    /// Handle string character access with numeric index
    fn convert_string_char_index(
        &mut self,
        index: &HirExpr,
        base_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        let index_expr = index.to_rust_expr(self.ctx)?;
        Ok(parse_quote! {
            {
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
    }

    /// Handle numeric indexing on lists/tuples/dicts with non-string keys
    fn convert_numeric_index(
        &mut self,
        base: &HirExpr,
        index: &HirExpr,
        base_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        // DEPYLER-0701: Tuple with variable index → convert to array
        let is_tuple_base = self.is_tuple_base(base);
        if is_tuple_base && !matches!(index, HirExpr::Literal(Literal::Int(_))) {
            let tuple_size = self.get_tuple_size(base).unwrap_or(2);
            let index_expr = index.to_rust_expr(self.ctx)?;
            let indices: Vec<syn::Index> = (0..tuple_size).map(syn::Index::from).collect();
            return Ok(parse_quote! {
                [#(#base_expr.#indices),*][#index_expr as usize]
            });
        }

        // DEPYLER-1060: Dict with non-string keys
        if self.is_dict_expr(base) {
            return self.convert_dict_nonstring_key_index(base, index, base_expr);
        }

        // Vec/List access with numeric index
        self.convert_vec_index(index, base_expr)
    }

    /// Handle dict indexing with non-string keys (int, float, etc.)
    fn convert_dict_nonstring_key_index(
        &mut self,
        base: &HirExpr,
        index: &HirExpr,
        base_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        let index_expr = index.to_rust_expr(self.ctx)?;
        let key_type = self.resolve_dict_key_type(base);

        match &key_type {
            Some(Type::Int) | Some(Type::Bool) | Some(Type::String) => Ok(parse_quote! {
                #base_expr.get(&(#index_expr)).cloned().unwrap_or_default()
            }),
            Some(Type::Float) => {
                if matches!(index, HirExpr::Literal(Literal::Float(_))) {
                    Ok(parse_quote! {
                        #base_expr.get(&DepylerValue::Float(#index_expr)).cloned().unwrap_or_default()
                    })
                } else if matches!(index, HirExpr::Literal(Literal::Int(_))) {
                    Ok(parse_quote! {
                        #base_expr.get(&DepylerValue::Float(#index_expr as f64)).cloned().unwrap_or_default()
                    })
                } else {
                    Ok(parse_quote! {
                        #base_expr.get(&DepylerValue::from(#index_expr)).cloned().unwrap_or_default()
                    })
                }
            }
            _ => Ok(parse_quote! {
                #base_expr.get(&DepylerValue::Int(#index_expr as i64)).cloned().unwrap_or_default()
            }),
        }
    }

    /// Resolve the key type for a dict expression (direct var or nested subscript)
    fn resolve_dict_key_type(&self, base: &HirExpr) -> Option<Type> {
        if let HirExpr::Var(var_name) = base {
            self.ctx.var_types.get(var_name).and_then(|t| {
                if let Type::Dict(k, _) = t {
                    Some(k.as_ref().clone())
                } else {
                    None
                }
            })
        } else if let HirExpr::Index { base: inner_base, .. } = base {
            // Walk to root variable and resolve the value type of the outer dict
            let mut current = inner_base.as_ref();
            while let HirExpr::Index { base: deeper, .. } = current {
                current = deeper;
            }
            if let HirExpr::Var(root_name) = current {
                let mut cur_type = self.ctx.var_types.get(root_name).cloned();
                let mut walk = inner_base.as_ref();
                while let HirExpr::Index { base: deeper, .. } = walk {
                    walk = deeper;
                }
                // Peel one Dict level (root → value type of root dict)
                if let Some(Type::Dict(_, val)) = &cur_type {
                    cur_type = Some(val.as_ref().clone());
                } else if let Some(Type::List(elem)) = &cur_type {
                    cur_type = Some(elem.as_ref().clone());
                }
                cur_type.and_then(|t| {
                    if let Type::Dict(k, _) = t {
                        Some(k.as_ref().clone())
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Handle Vec/List access with numeric index
    fn convert_vec_index(&mut self, index: &HirExpr, base_expr: &syn::Expr) -> Result<syn::Expr> {
        let index_expr = index.to_rust_expr(self.ctx)?;

        // Check if index is a negative literal
        if let HirExpr::Unary { op: UnaryOp::Neg, operand } = index {
            if let HirExpr::Literal(Literal::Int(n)) = **operand {
                let offset = n as usize;
                return Ok(parse_quote! {
                    {
                        let base = &#base_expr;
                        base.get(base.len().saturating_sub(#offset)).cloned().unwrap_or_default()
                    }
                });
            }
        }

        // DEPYLER-0357: Positive integer literal → simple .get()
        if let HirExpr::Literal(Literal::Int(n)) = index {
            let idx_value = *n as usize;
            return Ok(parse_quote! {
                #base_expr.get(#idx_value).cloned().expect("IndexError: list index out of range")
            });
        }

        let is_simple_var = matches!(index, HirExpr::Var(_));
        let index_is_depyler_value =
            self.expr_returns_depyler_value(index) && self.ctx.type_mapper.nasa_mode;

        if is_simple_var {
            if index_is_depyler_value {
                Ok(parse_quote! {
                    #base_expr.get(#index_expr.to_i64() as usize).cloned().expect("IndexError: list index out of range")
                })
            } else {
                Ok(parse_quote! {
                    #base_expr.get(#index_expr as usize).cloned().expect("IndexError: list index out of range")
                })
            }
        } else if index_is_depyler_value {
            Ok(parse_quote! {
                {
                    let base = &#base_expr;
                    let idx: i64 = #index_expr.to_i64();
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
                }
            })
        } else {
            Ok(parse_quote! {
                {
                    let base = &#base_expr;
                    let idx: i32 = #index_expr;
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx).cloned().expect("IndexError: list index out of range")
                }
            })
        }
    }
}
