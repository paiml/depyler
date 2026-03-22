pub(crate) fn codegen_return_stmt(
    expr: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // CITL: Trace return type handling decision
    trace_decision!(
        category = DecisionCategory::TypeMapping,
        name = "return_stmt",
        chosen = "return_expr",
        alternatives = ["return_unit", "return_result", "return_option", "implicit_return"],
        confidence = 0.92
    );

    if let Some(e) = expr {
        // DEPYLER-1064: Handle Tuple[Any, ...] returns - wrap elements in DepylerValue
        // When return type is Tuple containing Any types, each element needs DepylerValue wrapping
        // DEPYLER-1158: Clone elem_types to avoid borrow conflict with ctx
        let elem_types_owned: Option<Vec<Type>> =
            if let Some(Type::Tuple(types)) = ctx.current_return_type.as_ref() {
                Some(types.clone())
            } else {
                None
            };
        if let (HirExpr::Tuple(elts), Some(elem_types)) = (e, elem_types_owned.as_ref()) {
            // DEPYLER-1158: Trigger wrapping for tuples with Unknown, Any, or list of Unknown types
            // This ensures proper element conversion when function signature differs from expression types
            // Key insight: HIR uses Type::Unknown, not Type::Custom("DepylerValue") - the DepylerValue
            // tokens are generated during code emission, not stored in the type system
            let has_depyler_related_types = elem_types.iter().any(|t| {
                matches!(t, Type::Unknown)
                    || matches!(t, Type::Custom(name) if name == "Any" || name == "object")
                    || matches!(t, Type::List(inner) if matches!(inner.as_ref(), Type::Unknown))
            });
            if ctx.type_mapper.nasa_mode && has_depyler_related_types {
                // DEPYLER-1158: Generate tuple with each element respecting expected type
                // Use indexed iteration to avoid borrow checker issues with ctx
                let mut wrapped_elems = Vec::with_capacity(elts.len());
                for (i, elem) in elts.iter().enumerate() {
                    let elem_expr = elem.to_rust_expr(ctx)?;
                    let expected_type = elem_types.get(i);
                    // DEPYLER-1158: Check expected type and infer from expression for Unknown
                    // For list expressions with Unknown type, produce Vec<DepylerValue>
                    // Key: HIR uses Type::List(Type::Unknown), not Type::List(Type::Custom("DepylerValue"))
                    let wrapped = match expected_type {
                        // DEPYLER-1158: List<Unknown> maps to Vec<DepylerValue>
                        Some(Type::List(inner)) if matches!(inner.as_ref(), Type::Unknown) => {
                            // Expected: Vec<DepylerValue> - convert each element
                            quote! { #elem_expr.into_iter().map(DepylerValue::from).collect::<Vec<_>>() }
                        }
                        // DEPYLER-1158: Scalar Unknown maps to DepylerValue
                        Some(Type::Unknown) => {
                            // Check if expression is a list or likely produces a list
                            let is_list_expr = matches!(elem, HirExpr::List(_))
                                || matches!(elem, HirExpr::ListComp { .. })
                                || match elem {
                                    HirExpr::Var(name) => {
                                        // Check if var is known to be a list type
                                        ctx.var_types
                                            .get(name)
                                            .map(|t| matches!(t, Type::List(_)))
                                            .unwrap_or(false)
                                    }
                                    HirExpr::MethodCall { method, .. } => {
                                        // Methods like map, filter, collect return lists
                                        matches!(
                                            method.as_str(),
                                            "collect" | "map" | "filter" | "sorted" | "list"
                                        )
                                    }
                                    _ => false,
                                };
                            if is_list_expr {
                                // List expression with Unknown type -> Vec<DepylerValue>
                                quote! { #elem_expr.into_iter().map(DepylerValue::from).collect::<Vec<_>>() }
                            } else {
                                // Non-list: wrap in DepylerValue::from()
                                quote! { DepylerValue::from(#elem_expr) }
                            }
                        }
                        _ => {
                            // Default: wrap in DepylerValue::from()
                            quote! { DepylerValue::from(#elem_expr) }
                        }
                    };
                    wrapped_elems.push(wrapped);
                }

                return Ok(quote! { return (#(#wrapped_elems),*); });
            }
        }

        // DEPYLER-1036: Set current_assign_type for Dict expressions in return statements
        // This ensures empty dicts use the function return type for type inference
        let prev_assign_type = ctx.current_assign_type.take();
        if matches!(e, HirExpr::Dict(_)) {
            if let Some(return_type) = &ctx.current_return_type {
                // Extract Dict type from Optional<Dict> or use directly if Dict
                let dict_type = match return_type {
                    Type::Optional(inner) => match inner.as_ref() {
                        Type::Dict(_, _) => Some(inner.as_ref().clone()),
                        _ => None,
                    },
                    Type::Dict(_, _) => Some(return_type.clone()),
                    _ => None,
                };
                if let Some(dt) = dict_type {
                    ctx.current_assign_type = Some(dt);
                }
            }
        }
        let mut expr_tokens = e.to_rust_expr(ctx)?;
        ctx.current_assign_type = prev_assign_type;

        // DEPYLER-0626: Wrap return value with Box::new() for heterogeneous IO types
        // When function returns Box<dyn Write> (e.g., sys.stdout vs File), wrap the value
        if ctx.function_returns_boxed_write {
            expr_tokens = parse_quote! { Box::new(#expr_tokens) };
        }

        // DEPYLER-1124: Convert concrete type to Union type via .into()
        // When return type is Union[A, B] and expression is concrete A or B,
        // add .into() to let Rust's From trait handle the conversion.
        // Union types generate enum with From impls for each variant.
        let is_union_return = matches!(ctx.current_return_type.as_ref(), Some(Type::Union(_)));
        if is_union_return {
            expr_tokens = parse_quote! { #expr_tokens.into() };
        }

        // DEPYLER-0241: Apply type conversion if needed (e.g., usize -> i32 from enumerate())
        if let Some(return_type) = &ctx.current_return_type {
            // Unwrap Optional to get the underlying type
            let target_type = match return_type {
                Type::Optional(inner) => inner.as_ref(),
                other => other,
            };

            // DEPYLER-0272: Pass expression to check if cast is actually needed
            // DEPYLER-0455 #7: Also pass ctx to check validator function context
            if needs_type_conversion(target_type, e) {
                expr_tokens = apply_type_conversion(expr_tokens, target_type);
            }

            // DEPYLER-E0308-FIX: String literal to String conversion in return statements
            // When returning a string literal and the return type is String, add .to_string()
            // Example: `return ""` should become `"".to_string()` when return type is String
            if matches!(e, HirExpr::Literal(Literal::String(_)))
                && matches!(target_type, Type::String)
            {
                expr_tokens = parse_quote! { #expr_tokens.to_string() };
            }

            // DEPYLER-E0282-FIX: Add type hint for chained PyOps expressions in return statements
            // When returning expressions like ((a).py_add(b)).py_add(c), Rust can't infer the
            // intermediate types because PyOps traits have multiple impls (i32+i32, i32+f64, etc.)
            // Also handles Ok(...) wrapped expressions where inner type is Int/Float.
            // Fix: Wrap in type assertion block { let _r: <type> = expr; _r }
            // DEPYLER-99MODE-S9: Skip type assertion for Result-returning function calls.
            // The function's return type is already known, and wrapping in {let _r: T = call(); _r}
            // would assign Result<T> to T. The PyOps in arguments are resolved at arg level.
            let is_result_call = matches!(e, HirExpr::Call { func, .. }
                if ctx.result_returning_functions.contains(func));
            if ctx.type_mapper.nasa_mode && has_chained_pyops(e) && !is_result_call {
                // Check if target_type is Result/Option and extract inner type
                let effective_type = get_inner_optional_type(target_type).unwrap_or(target_type);
                let type_tokens: Option<syn::Type> = match effective_type {
                    Type::Int => Some(parse_quote! { i32 }),
                    Type::Float => Some(parse_quote! { f64 }),
                    _ => None,
                };
                if let Some(ty) = type_tokens {
                    expr_tokens = parse_quote! { { let _r: #ty = #expr_tokens; _r } };
                }
            }
        }

        // DEPYLER-1150: Convert slice params to Vec when returning in Vec-returning function
        // Pattern: def func(*args) -> List[T]: return args
        // Rust: fn func(args: &[T]) -> Vec<T> { args.to_vec() }
        // Without this, we get E0308: expected Vec<T>, found &[T]
        if let HirExpr::Var(var_name) = e {
            let is_slice_param = ctx.slice_params.contains(var_name);
            let is_vec_return = matches!(
                ctx.current_return_type.as_ref(),
                Some(Type::List(_)) | Some(Type::Tuple(_))
            );
            if is_slice_param && is_vec_return {
                expr_tokens = parse_quote! { #expr_tokens.to_vec() };
            }

            // DEPYLER-99MODE-S9: Convert &str params to String when returning String
            // Pattern: def func(s: str) -> str: return s
            // Rust: fn func(s: &str) -> String { s.to_string() }
            let is_str_param = ctx.fn_str_params.contains(var_name);
            let is_string_return_type =
                matches!(ctx.current_return_type.as_ref(), Some(Type::String));
            if is_str_param && is_string_return_type {
                expr_tokens = parse_quote! { #expr_tokens.to_string() };
            }
        }

        // DEPYLER-0757: Wrap return values when function returns serde_json::Value (Python's `any`)
        // When return type is serde_json::Value, use json!() macro to convert any value
        // Note: ctx.current_return_type contains HIR type (e.g., "any") not the mapped Rust type
        let is_json_value_return = matches!(
            ctx.current_return_type.as_ref(),
            Some(Type::Custom(name)) if name == "serde_json::Value"
                || name == "any"
                || name == "Any"
                || name == "typing.Any"
        );
        // DEPYLER-1017: Skip serde_json in NASA mode
        if is_json_value_return && !ctx.type_mapper.nasa_mode {
            // Use serde_json::json!() macro to convert the expression to Value
            // This handles bool, int, float, string, arrays, etc. automatically
            expr_tokens = parse_quote! { serde_json::json!(#expr_tokens) };
        }

        // DEPYLER-0943: Convert serde_json::Value to String when return type is String
        // Dict subscript access returns serde_json::Value, but if the function return type
        // is String, we need to extract the string value from the JSON Value.
        // DEPYLER-1221: Only apply this conversion when dict VALUE type is serde_json::Value.
        // If the dict value type is already String, skip the conversion.
        // DEPYLER-1320: Skip this conversion in NASA mode - DepylerValue uses .into() for type
        // conversion which already handles String extraction via From<DepylerValue> trait.
        let is_string_return = matches!(ctx.current_return_type.as_ref(), Some(Type::String));
        let is_dict_subscript = is_dict_index_access(e);
        let needs_json_to_string_conversion = if ctx.type_mapper.nasa_mode {
            // In NASA mode, .into() already handles DepylerValue to String conversion
            false
        } else if is_string_return && is_dict_subscript {
            // Check if the dict has serde_json::Value value type
            if let HirExpr::Index { base, .. } = e {
                if let HirExpr::Var(var_name) = base.as_ref() {
                    // Look up the dict type in var_types
                    if let Some(dict_type) = ctx.var_types.get(var_name) {
                        // Only convert if value type is serde_json::Value or Unknown
                        is_dict_with_value_type(dict_type)
                    } else {
                        // Unknown dict type - assume needs conversion for safety
                        true
                    }
                } else {
                    // Complex base expression - assume needs conversion
                    true
                }
            } else {
                false
            }
        } else {
            false
        };
        if needs_json_to_string_conversion {
            // Convert Value to String: value.as_str().unwrap_or("").to_string()
            expr_tokens = parse_quote! { #expr_tokens.as_str().unwrap_or("").to_string() };
        }

        // Check if return type is Optional and wrap value in Some()
        let is_optional_return =
            matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));

        // DEPYLER-0330: DISABLED - Heuristic too broad, breaks plain int variables named "result"
        // Original logic: Unwrap Option-typed variables when returning from non-Optional function
        // Problem: Can't distinguish between:
        //   1. result = d.get(key)  # Option<T> - needs unwrap
        //   2. result = 0           # i32 - breaks with unwrap
        // NOTE: Re-enable unwrap_or optimization when HIR has type tracking (tracked in DEPYLER-0424)
        //
        // if !is_optional_return {
        //     if let HirExpr::Var(var_name) = e {
        //         let is_primitive_return = matches!(
        //             ctx.current_return_type.as_ref(),
        //             Some(Type::Int | Type::Float | Type::Bool | Type::String)
        //         );
        //         if ctx.is_final_statement && var_name == "result" && is_primitive_return {
        //             expr_tokens = parse_quote! { #expr_tokens.unwrap() };
        //         }
        //     }
        // }

        // Check if the expression is None literal
        let is_none_literal = matches!(e, HirExpr::Literal(Literal::None));

        // DEPYLER-0744: Check if expression is already Option-typed (e.g., param with default=None)
        // DEPYLER-0951: Extended to check method calls that return Option
        // Don't wrap in Some() if the expression is already Option<T>
        let is_already_optional = if let HirExpr::Var(var_name) = e {
            ctx.var_types.get(var_name).map(|ty| matches!(ty, Type::Optional(_))).unwrap_or(false)
        } else if let HirExpr::MethodCall { method, args, .. } = e {
            // DEPYLER-0951: These methods return Option<T>, don't wrap in Some()
            // - dict.get(key) -> Option<&V>
            // - environ.get(key) -> Option<String> (via std::env::var().ok())
            // - Result.ok() -> Option<T>
            // - Option.cloned() -> Option<T>
            // DEPYLER-1036: dict.get(key, default) returns value type, NOT Option
            // Python's dict.get with 2 args has built-in default, so it never returns None
            let is_get_with_default = method == "get" && args.len() == 2;
            if is_get_with_default {
                false // dict.get(key, default) returns value, not Option
            } else {
                matches!(
                    method.as_str(),
                    "get" | "ok" | "cloned" | "copied" | "pop" | "first" | "last"
                )
            }
        } else {
            // DEPYLER-0951: Also check if the generated tokens end with .ok() or contain .get(
            // This catches cases where the HIR doesn't directly show the Option-returning method
            let expr_str = quote!(#expr_tokens).to_string();
            // DEPYLER-1036: Check for unwrapping methods that convert Option<T> to T
            // If expression ends with .unwrap_or(...) or .unwrap_or_default(), it's NOT optional
            // DEPYLER-1078: .next() also returns Option<T>
            let has_option_method = expr_str.ends_with(". ok ()")
                || expr_str.ends_with(". next ()")
                || expr_str.contains(". get (");
            let has_unwrap_method = expr_str.contains(". unwrap_or (")
                || expr_str.contains(". unwrap_or_default (")
                || expr_str.contains(". unwrap (")
                || expr_str.contains(". expect (");
            has_option_method && !has_unwrap_method
        };

        // DEPYLER-0498: Check if expression is if-expr with None arm (ternary with None)
        // Pattern: `return x if cond else None` -> should be `if cond { Some(x) } else { None }`
        // NOT: `Some(if cond { x } else { None })`
        let is_if_expr_with_none = matches!(
            e,
            HirExpr::IfExpr { orelse, .. } if matches!(&**orelse, HirExpr::Literal(Literal::None))
        );

        // DEPYLER-0271: For final statement in function, omit `return` keyword (idiomatic Rust)
        // Early returns (not final) keep the `return` keyword
        let use_return_keyword = !ctx.is_final_statement;

        // DEPYLER-0357: Check if function returns void (None in Python -> () in Rust)
        // Must check this BEFORE is_optional_return to avoid false positive
        // Python `-> None` maps to Rust `()`, not `Option<T>`
        let is_void_return = matches!(ctx.current_return_type.as_ref(), Some(Type::None));

        // DEPYLER-1147: Unwrap Optional parameters when return type is the inner type
        // Pattern: `def foo(x: Optional[int]) -> int: ... return x`
        // The parameter is &Option<T> and return type is T, so we need to unwrap
        // This is safe when preceded by `if x is None: return default` pattern
        if is_already_optional && !is_optional_return && !is_void_return {
            if let HirExpr::Var(var_name) = e {
                // Check if this variable is typed as Optional in var_types
                // (function parameters with Optional type are tracked there)
                let is_optional_typed =
                    ctx.var_types.get(var_name).is_some_and(|t| matches!(t, Type::Optional(_)));
                if is_optional_typed {
                    // For &Option<T> parameters, need (*var).unwrap() to get T
                    // Since optional params are passed by reference: &Option<T>
                    expr_tokens =
                        parse_quote! { (*#expr_tokens).expect("optional parameter unwrap failed") };
                }
            }
        }

        if ctx.current_function_can_fail {
            if is_void_return && is_none_literal {
                // Void function with can_fail: return Ok(()) for `return None`
                if use_return_keyword {
                    Ok(quote! { return Ok(()); })
                } else {
                    Ok(quote! { Ok(()) })
                }
            } else if is_optional_return
                && !is_none_literal
                && !is_if_expr_with_none
                && !is_already_optional
            {
                // Wrap value in Some() for Optional return types
                // DEPYLER-1079: Skip wrapping if if-expr has None arm (handled separately below)
                if use_return_keyword {
                    Ok(quote! { return Ok(Some(#expr_tokens)); })
                } else {
                    Ok(quote! { Ok(Some(#expr_tokens)) })
                }
            } else if is_optional_return && is_if_expr_with_none {
                // DEPYLER-1079: If-expr with None arm in Result context
                // Pattern: `return x if cond else None` -> `Ok(if cond { Some(x) } else { None })`
                if let HirExpr::IfExpr { test, body, orelse: _ } = e {
                    // DEPYLER-1071: Check if test is an Option variable (regex match result)
                    // Pattern: `return m.group(0) if m else None` where m is Option<Match>
                    // Should generate: `Ok(m.as_ref().map(|m_val| m_val.group(0)))`
                    if let HirExpr::Var(var_name) = test.as_ref() {
                        let is_option = ctx
                            .var_types
                            .get(var_name)
                            .is_some_and(|t| matches!(t, Type::Optional(_)))
                            || is_option_var_name(var_name);

                        if is_option && body_uses_option_var(body, var_name) {
                            let var_ident = crate::rust_gen::keywords::safe_ident(var_name);
                            let val_name = format!("{}_val", var_name);
                            let val_ident = crate::rust_gen::keywords::safe_ident(&val_name);

                            // Substitute variable in body
                            let body_substituted = substitute_var_in_hir(body, var_name, &val_name);
                            let body_tokens = body_substituted.to_rust_expr(ctx)?;

                            if use_return_keyword {
                                return Ok(
                                    quote! { return Ok(#var_ident.as_ref().map(|#val_ident| #body_tokens)); },
                                );
                            } else {
                                return Ok(
                                    quote! { Ok(#var_ident.as_ref().map(|#val_ident| #body_tokens)) },
                                );
                            }
                        }
                    }

                    let test_tokens = test.to_rust_expr(ctx)?;
                    // DEPYLER-1079: Apply truthiness conversion for collection/string/optional conditions
                    let test_tokens = apply_truthiness_conversion(test.as_ref(), test_tokens, ctx);
                    let body_tokens = body.to_rust_expr(ctx)?;

                    if use_return_keyword {
                        Ok(
                            quote! { return Ok(if #test_tokens { Some(#body_tokens) } else { None }); },
                        )
                    } else {
                        Ok(quote! { Ok(if #test_tokens { Some(#body_tokens) } else { None }) })
                    }
                } else {
                    unreachable!("is_if_expr_with_none should only match IfExpr")
                }
            } else if is_optional_return && is_already_optional {
                // DEPYLER-0744: Expression is already Option<T>, just wrap in Ok()
                if use_return_keyword {
                    Ok(quote! { return Ok(#expr_tokens); })
                } else {
                    Ok(quote! { Ok(#expr_tokens) })
                }
            } else if is_optional_return && is_none_literal {
                // DEPYLER-0277: Return None for Optional types (not ())
                if use_return_keyword {
                    Ok(quote! { return Ok(None); })
                } else {
                    Ok(quote! { Ok(None) })
                }
            } else if ctx.is_main_function {
                // DEPYLER-0617: Handle exit code returns in main() function
                // Python pattern: `def main() -> int: ... return 1`
                // Rust main() can only return () or Result<(), E>, so integer returns
                // must be converted to process::exit() for non-zero or Ok(()) for zero
                //
                // DEPYLER-0950: Only apply main() special handling for int/void returns.
                // If main() returns other types like f64, treat it as a normal function.
                let is_main_entry_point_return = matches!(
                    ctx.current_return_type.as_ref(),
                    None | Some(Type::Int) | Some(Type::None)
                );
                if !is_main_entry_point_return {
                    // main() with non-int/non-void return type (e.g., `def main() -> float`)
                    // Treat as normal function - generate standard return with Ok() wrapper
                    if use_return_keyword {
                        Ok(quote! { return Ok(#expr_tokens); })
                    } else {
                        Ok(quote! { Ok(#expr_tokens) })
                    }
                } else if let HirExpr::Literal(Literal::Int(exit_code)) = e {
                    if *exit_code == 0 {
                        // Success exit code -> Ok(())
                        if use_return_keyword {
                            Ok(quote! { return Ok(()); })
                        } else {
                            Ok(quote! { Ok(()) })
                        }
                    } else {
                        // Non-zero exit code -> std::process::exit(N)
                        let code = *exit_code as i32;
                        Ok(quote! { std::process::exit(#code) })
                    }
                } else {
                    // DEPYLER-0703: Other expressions in main - evaluate for side effects
                    // and return Ok(()). Use explicit semicolon to prevent DEPYLER-0694 wrap.
                    if use_return_keyword {
                        Ok(quote! { let _ = #expr_tokens; return Ok(()); })
                    } else {
                        Ok(quote! { let _ = #expr_tokens; Ok(()) })
                    }
                }
            } else if is_call_to_result_returning_fn(e, ctx) {
                // DEPYLER-99MODE-S9: Don't double-wrap Result-returning function calls in Ok()
                // If the return expression is a call to a function that already returns Result,
                // just return the call directly (possibly with ?)
                if use_return_keyword {
                    Ok(quote! { return #expr_tokens; })
                } else {
                    Ok(quote! { #expr_tokens })
                }
            } else if use_return_keyword {
                Ok(quote! { return Ok(#expr_tokens); })
            } else {
                Ok(quote! { Ok(#expr_tokens) })
            }
        } else if is_void_return {
            // Void functions (Python -> None): no return value (non-fallible)
            if use_return_keyword {
                // Early return from void function: use empty return
                Ok(quote! { return; })
            } else {
                // Final statement in void function: use unit value ()
                Ok(quote! { () })
            }
        } else if is_optional_return
            && !is_none_literal
            && !is_if_expr_with_none
            && !is_already_optional
        {
            // Wrap value in Some() for Optional return types
            // DEPYLER-0498: Skip wrapping if if-expr has None arm (handled separately)
            // DEPYLER-0744: Skip wrapping if expression is already Option<T>
            if use_return_keyword {
                Ok(quote! { return Some(#expr_tokens); })
            } else {
                Ok(quote! { Some(#expr_tokens) })
            }
        } else if is_optional_return && is_already_optional {
            // DEPYLER-0744: Expression is already Option<T>, don't double-wrap
            if use_return_keyword {
                Ok(quote! { return #expr_tokens; })
            } else {
                Ok(quote! { #expr_tokens })
            }
        } else if is_optional_return && is_if_expr_with_none {
            // DEPYLER-0498: If-expr with None arm - manually wrap true arm in Some()
            // Pattern: `return x if cond else None` -> `if cond { Some(x) } else { None }`
            if let HirExpr::IfExpr { test, body, orelse: _ } = e {
                // DEPYLER-1071: Check if test is an Option variable (regex match result)
                // Pattern: `return m.group(0) if m else None` where m is Option<Match>
                // Should generate: `m.as_ref().map(|m_val| m_val.group(0))`
                if let HirExpr::Var(var_name) = test.as_ref() {
                    let is_option =
                        ctx.var_types.get(var_name).is_some_and(|t| matches!(t, Type::Optional(_)))
                            || is_option_var_name(var_name);

                    if is_option && body_uses_option_var(body, var_name) {
                        let var_ident = crate::rust_gen::keywords::safe_ident(var_name);
                        let val_name = format!("{}_val", var_name);
                        let val_ident = crate::rust_gen::keywords::safe_ident(&val_name);

                        // Substitute variable in body
                        let body_substituted = substitute_var_in_hir(body, var_name, &val_name);
                        let body_tokens = body_substituted.to_rust_expr(ctx)?;

                        if use_return_keyword {
                            return Ok(
                                quote! { return #var_ident.as_ref().map(|#val_ident| #body_tokens); },
                            );
                        } else {
                            return Ok(
                                quote! { #var_ident.as_ref().map(|#val_ident| #body_tokens) },
                            );
                        }
                    }
                }

                let test_tokens = test.to_rust_expr(ctx)?;
                // DEPYLER-1079: Apply truthiness conversion for collection/string/optional conditions
                let test_tokens = apply_truthiness_conversion(test.as_ref(), test_tokens, ctx);
                let body_tokens = body.to_rust_expr(ctx)?;

                if use_return_keyword {
                    Ok(quote! { return if #test_tokens { Some(#body_tokens) } else { None }; })
                } else {
                    Ok(quote! { if #test_tokens { Some(#body_tokens) } else { None } })
                }
            } else {
                unreachable!("is_if_expr_with_none should only match IfExpr")
            }
        } else if is_optional_return && is_none_literal {
            // DEPYLER-0277: Return None for Optional types (not ()) - non-Result case
            if use_return_keyword {
                Ok(quote! { return None; })
            } else {
                Ok(quote! { None })
            }
        } else if ctx.is_main_function {
            // DEPYLER-0617: Handle exit code returns in main() function (non-fallible case)
            // Python pattern: `def main() -> int: ... return 0`
            // Rust main() can only return () or Result<(), E>, so integer returns
            // must be converted to process::exit() for non-zero or () for zero
            //
            // DEPYLER-0950: Only apply main() special handling for int/void returns.
            // If main() returns other types like f64, treat it as a normal function.
            let is_main_entry_point_return = matches!(
                ctx.current_return_type.as_ref(),
                None | Some(Type::Int) | Some(Type::None)
            );
            if !is_main_entry_point_return {
                // main() with non-int/non-void return type (e.g., `def main() -> float`)
                // Treat as normal function - generate standard return
                if use_return_keyword {
                    Ok(quote! { return #expr_tokens; })
                } else {
                    Ok(quote! { #expr_tokens })
                }
            } else if let HirExpr::Literal(Literal::Int(exit_code)) = e {
                if *exit_code == 0 {
                    // Success exit code -> ()
                    if use_return_keyword {
                        Ok(quote! { return; })
                    } else {
                        Ok(quote! { () })
                    }
                } else {
                    // Non-zero exit code -> std::process::exit(N)
                    let code = *exit_code as i32;
                    Ok(quote! { std::process::exit(#code) })
                }
            } else {
                // DEPYLER-0703: Other expressions in main - evaluate for side effects
                // and return (). Use explicit (); to prevent DEPYLER-0694 from wrapping again.
                if use_return_keyword {
                    Ok(quote! { let _ = #expr_tokens; return; })
                } else {
                    // Note: Use explicit statement with semicolon to prevent
                    // DEPYLER-0694 from adding another let _ =
                    Ok(quote! { let _ = #expr_tokens; })
                }
            }
        } else if use_return_keyword {
            Ok(quote! { return #expr_tokens; })
        } else {
            Ok(quote! { #expr_tokens })
        }
    } else if ctx.current_function_can_fail {
        // No expression - check if return type is Optional
        let is_optional_return =
            matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));
        let use_return_keyword = !ctx.is_final_statement;

        if is_optional_return {
            if use_return_keyword {
                Ok(quote! { return Ok(None); })
            } else {
                Ok(quote! { Ok(None) })
            }
        } else if use_return_keyword {
            Ok(quote! { return Ok(()); })
        } else {
            Ok(quote! { Ok(()) })
        }
    } else {
        let use_return_keyword = !ctx.is_final_statement;
        if use_return_keyword {
            Ok(quote! { return; })
        } else {
            // Final bare return becomes unit value (implicit)
            Ok(quote! {})
        }
    }
}

fn body_uses_option_var(body: &HirExpr, var_name: &str) -> bool {
    match body {
        // Direct method call on the variable: m.group(0)
        HirExpr::MethodCall { object, .. } => {
            if let HirExpr::Var(obj_name) = object.as_ref() {
                return obj_name == var_name;
            }
            body_uses_option_var(object, var_name)
        }
        // Attribute access on the variable
        HirExpr::Attribute { value, .. } => {
            if let HirExpr::Var(obj_name) = value.as_ref() {
                return obj_name == var_name;
            }
            body_uses_option_var(value, var_name)
        }
        // Variable used directly
        HirExpr::Var(name) => name == var_name,
        _ => false,
    }
}

fn substitute_var_in_hir(expr: &HirExpr, old_name: &str, new_name: &str) -> HirExpr {
    match expr {
        HirExpr::Var(name) if name == old_name => HirExpr::Var(new_name.to_string()),
        HirExpr::MethodCall { object, method, args, kwargs } => HirExpr::MethodCall {
            object: Box::new(substitute_var_in_hir(object, old_name, new_name)),
            method: method.clone(),
            args: args.iter().map(|a| substitute_var_in_hir(a, old_name, new_name)).collect(),
            kwargs: kwargs
                .iter()
                .map(|(k, v)| (k.clone(), substitute_var_in_hir(v, old_name, new_name)))
                .collect(),
        },
        HirExpr::Attribute { value, attr } => HirExpr::Attribute {
            value: Box::new(substitute_var_in_hir(value, old_name, new_name)),
            attr: attr.clone(),
        },
        HirExpr::Call { func, args, kwargs } => HirExpr::Call {
            func: func.clone(),
            args: args.iter().map(|a| substitute_var_in_hir(a, old_name, new_name)).collect(),
            kwargs: kwargs
                .iter()
                .map(|(k, v)| (k.clone(), substitute_var_in_hir(v, old_name, new_name)))
                .collect(),
        },
        // For other expression types, return as-is
        _ => expr.clone(),
    }
}
