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
                    #base_ident.as_ref().expect("value is None").get(&#index_expr).cloned().unwrap_or_default()
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
        // DEPYLER-1316: For string keys, use get_str() instead of py_index()
        // DEPYLER-1319: Add .into() for type conversion from DepylerValue to expected type
        let base_is_depyler = self.expr_returns_depyler_value(base);
        if base_is_depyler && self.ctx.type_mapper.nasa_mode {
            // DEPYLER-1316: Check for string literal keys - use get_str() for dict access
            // DEPYLER-1319: Add .into() to convert DepylerValue to target type
            if let HirExpr::Literal(Literal::String(s)) = index {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                return Ok(
                    parse_quote! { #base_expr.get_str(#lit).cloned().unwrap_or_default().into() },
                );
            }
            // DEPYLER-1316: Check for string variable keys - use get_str() for dict access
            // DEPYLER-1319: Add .into() to convert DepylerValue to target type
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

        // DEPYLER-1145: Wrap index in DepylerValue if base is HashMap<DepylerValue, ...>
        // Check if base is a variable with DepylerValue key type
        // DEPYLER-1214: Type::Dict(Unknown, _) actually generates HashMap<String, ...> (see type_tokens.rs)
        // so we should NOT wrap keys in DepylerValue. Only Type::Dict with explicit DepylerValue
        // key type would need wrapping (which is rare in practice).
        let is_depyler_value_key = if let HirExpr::Var(var_name) = base {
            self.ctx.var_types.get(var_name).is_some_and(|t| {
                // Only wrap if key type is explicitly DepylerValue, not Unknown
                // Unknown keys map to String in the type_tokens.rs (see DEPYLER-1214)
                matches!(t, Type::Dict(key_type, _) if matches!(key_type.as_ref(), Type::Custom(n) if n == "DepylerValue"))
            })
        } else {
            false
        };

        if is_depyler_value_key {
            // Need to convert index to Rust expression if we haven't yet (we haven't in this block)
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
                    // For variables, fallback to generic conversion
                    parse_quote! { &DepylerValue::Str(format!("{:?}", #index_expr)) }
                }
            };
            // DEPYLER-1146: Add unwrap_or_default() to match Python's dict["key"] semantics
            // Python raises KeyError for missing keys, but in Rust we use Default (DepylerValue::None)
            // to avoid panics. This matches the string-key case at line 3752.
            return Ok(
                parse_quote! { #base_expr.get(#wrapped_index).cloned().unwrap_or_default() },
            );
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
                            || matches!(var_type, Type::Optional(inner) if matches!(&**inner, Type::Tuple(_)))
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
                // DEPYLER-99MODE-S9: Unwrap Option before tuple field access
                // Pattern: isect[0] where isect: Optional[Tuple[float, float]]
                // → isect.unwrap().0
                if let HirExpr::Var(var_name) = base {
                    if let Some(Type::Optional(inner)) = self.ctx.var_types.get(var_name) {
                        if matches!(&**inner, Type::Tuple(_)) {
                            return Ok(parse_quote! { #base_expr.unwrap().#field_idx });
                        }
                    }
                }
                return Ok(parse_quote! { #base_expr.#field_idx });
            }
        }

        // DEPYLER-0299 Pattern #3 FIX: Check if base is a String type for character access
        let is_string_base = self.is_string_base(base);

        // Discriminate between HashMap and Vec access based on base type or index type
        let is_string_key = self.is_string_index(base, index)?;

        if is_string_key {
            // HashMap/Dict access with string keys
            // DEPYLER-1316: When base is DepylerValue (NASA mode), use get_str() for string keys
            // DepylerValue.get(&DepylerValue) vs DepylerValue.get_str(&str)
            let base_is_depyler_value = self.expr_returns_depyler_value(base);

            // DEPYLER-1319: Check if dict values are DepylerValue (needs .into() for conversion)
            // This enables type coercion: dict["key"] → DepylerValue → primitive type
            let needs_into = base_is_depyler_value || self.dict_has_depyler_value_values(base);

            match index {
                HirExpr::Literal(Literal::String(s)) => {
                    // String literal - use it directly without .to_string()
                    if base_is_depyler_value && self.ctx.type_mapper.nasa_mode {
                        // DEPYLER-1316: DepylerValue has get_str(&str) method for string keys
                        // DEPYLER-1319: Add .into() for type conversion
                        Ok(parse_quote! {
                            #base_expr.get_str(#s).cloned().unwrap_or_default().into()
                        })
                    } else if needs_into {
                        // DEPYLER-1319: Dict has DepylerValue values - add .into()
                        Ok(parse_quote! {
                            #base_expr.get(#s).cloned().unwrap_or_default().into()
                        })
                    } else {
                        Ok(parse_quote! {
                            #base_expr.get(#s).cloned().unwrap_or_default()
                        })
                    }
                }
                _ => {
                    // DEPYLER-1320: Defensive check for string literals that reach this branch
                    // In classmethods, string literals may not match the earlier pattern due to
                    // differences in HIR construction. Handle them here to ensure correct codegen.
                    if let HirExpr::Literal(Literal::String(s)) = index {
                        if base_is_depyler_value && self.ctx.type_mapper.nasa_mode {
                            // DepylerValue with string key - use get_str + .into()
                            return Ok(parse_quote! {
                                #base_expr.get_str(#s).cloned().unwrap_or_default().into()
                            });
                        } else if needs_into {
                            // Dict with DepylerValue values - add .into()
                            return Ok(parse_quote! {
                                #base_expr.get(#s).cloned().unwrap_or_default().into()
                            });
                        } else {
                            return Ok(parse_quote! {
                                #base_expr.get(#s).cloned().unwrap_or_default()
                            });
                        }
                    }

                    // String variable - needs proper referencing
                    // HashMap.get() expects &K, so we need to borrow the key
                    // DEPYLER-0521: Don't add & if variable is already &str type
                    // DEPYLER-0528: Borrow logic - owned String NEEDS borrow, &str does NOT
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

                    if base_is_depyler_value && self.ctx.type_mapper.nasa_mode {
                        // DEPYLER-1316: Use get_str for DepylerValue with string variable keys
                        // DEPYLER-1319: Add .into() for type conversion
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
                        // DEPYLER-1319: Dict has DepylerValue values - add .into()
                        if needs_borrow {
                            // DEPYLER-99MODE-S9: Wrap index_expr in parens so & applies to full expression
                            Ok(parse_quote! {
                                #base_expr.get(&(#index_expr)).cloned().unwrap_or_default().into()
                            })
                        } else {
                            Ok(parse_quote! {
                                #base_expr.get(#index_expr).cloned().unwrap_or_default().into()
                            })
                        }
                    } else if needs_borrow {
                        // DEPYLER-99MODE-S9: Wrap index_expr in parens so & applies to full expression
                        Ok(parse_quote! {
                            #base_expr.get(&(#index_expr)).cloned().unwrap_or_default()
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
                let indices: Vec<syn::Index> = (0..tuple_size).map(syn::Index::from).collect();

                return Ok(parse_quote! {
                    [#(#base_expr.#indices),*][#index_expr as usize]
                });
            }

            // DEPYLER-1060: Check for dict with non-string keys (e.g., d = {1: "a"})
            // DEPYLER-1073: Handle float keys using DepylerValue::Float
            if self.is_dict_expr(base) {
                let index_expr = index.to_rust_expr(self.ctx)?;
                // DEPYLER-99MODE-S9: Check if dict has CONCRETE typed keys
                // For Dict[int, int] → HashMap<i32, i32>, use &key directly (no DepylerValue wrapping)
                // DEPYLER-99MODE-S9: Also resolve nested subscript bases like trie[k][ik]
                // where trie: Dict[str, Dict[str, str]] → inner dict has String keys
                let key_type = if let HirExpr::Var(var_name) = base {
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
                        // Walk through Index levels to find the value type at this level
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
                        // Extract key type from the resolved inner dict type
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
                };

                // Concrete typed keys: use native key types directly
                match &key_type {
                    Some(Type::Int) | Some(Type::Bool) => {
                        // DEPYLER-99MODE-S9: Wrap in parens so & applies to full expression (e.g., i-1)
                        return Ok(parse_quote! {
                            #base_expr.get(&(#index_expr)).cloned().unwrap_or_default()
                        });
                    }
                    Some(Type::String) => {
                        // DEPYLER-99MODE-S9: String-keyed dicts use &key directly
                        // lookup[encoded[i]] → lookup.get(&(encoded_i)).cloned()
                        return Ok(parse_quote! {
                            #base_expr.get(&(#index_expr)).cloned().unwrap_or_default()
                        });
                    }
                    Some(Type::Float) => {
                        // Float keys use DepylerValue::Float for ordered-float compat
                        if matches!(index, HirExpr::Literal(Literal::Float(_))) {
                            return Ok(parse_quote! {
                                #base_expr.get(&DepylerValue::Float(#index_expr)).cloned().unwrap_or_default()
                            });
                        } else if matches!(index, HirExpr::Literal(Literal::Int(_))) {
                            return Ok(parse_quote! {
                                #base_expr.get(&DepylerValue::Float(#index_expr as f64)).cloned().unwrap_or_default()
                            });
                        } else {
                            return Ok(parse_quote! {
                                #base_expr.get(&DepylerValue::from(#index_expr)).cloned().unwrap_or_default()
                            });
                        }
                    }
                    _ => {
                        // Unknown or DepylerValue keys: wrap with DepylerValue::Int
                        return Ok(parse_quote! {
                            #base_expr.get(&DepylerValue::Int(#index_expr as i64)).cloned().unwrap_or_default()
                        });
                    }
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

            // DEPYLER-1162: Check if index expression returns DepylerValue
            // In this case, we need to use .to_i64() as usize instead of direct cast
            let index_is_depyler_value =
                self.expr_returns_depyler_value(index) && self.ctx.type_mapper.nasa_mode;

            if is_simple_var {
                // Simple variable index - use inline expression (works in range contexts)
                // This avoids block expressions that break in `for j in 0..matrix[i].len()`
                // DEPYLER-0730: Use .expect() for Python IndexError semantics
                // DEPYLER-1162: Handle DepylerValue indices with .to_i64() conversion
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
                // DEPYLER-1162: Complex DepylerValue expression - use .to_i64() conversion
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
}
