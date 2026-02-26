//! Method call routing for ExpressionConverter
//!
//! Contains convert_dynamic_call and convert_method_call - routing method calls
//! to appropriate handlers based on receiver type and method name.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::trace_decision;
use anyhow::Result;
use syn::{self, parse_quote};

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_dynamic_call(
        &mut self,
        callee: &HirExpr,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        let callee_expr = callee.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

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

        // DEPYLER-1205: Usage-Based Type Inference
        // If a variable is Unknown/DepylerValue and calls a method that implies a type, infer it.
        // This is "Inference by Usage" - we help the compiler by telling it what type the variable must be.
        if let HirExpr::Var(var_name) = object {
            let current_type = self.ctx.var_types.get(var_name).cloned();
            let is_unknown = matches!(current_type, None | Some(Type::Unknown));

            if is_unknown {
                // List-indicator methods
                match method {
                    // DEPYLER-1211: Recursive Type Propagation for append
                    // If .append(arg) is called, infer the element type from arg
                    "append" => {
                        let element_type = if !args.is_empty() {
                            self.infer_type_from_hir_expr(&args[0])
                        } else {
                            Type::Unknown
                        };
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::List(Box::new(element_type.clone())));
                        tracing::debug!(
                            "DEPYLER-1211: Inferred {} as List<{:?}> (via append())",
                            var_name,
                            element_type
                        );
                    }
                    // DEPYLER-1211: For insert(idx, arg), infer element type from second arg
                    "insert" => {
                        let element_type = if args.len() >= 2 {
                            self.infer_type_from_hir_expr(&args[1])
                        } else {
                            Type::Unknown
                        };
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::List(Box::new(element_type.clone())));
                        tracing::debug!(
                            "DEPYLER-1211: Inferred {} as List<{:?}> (via insert())",
                            var_name,
                            element_type
                        );
                    }
                    // Other list methods - element type remains unknown
                    "extend" | "pop" | "remove" | "sort" | "reverse" | "clear" | "copy"
                    | "index" | "count" => {
                        // This variable must be a list
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::List(Box::new(Type::Unknown)));
                        tracing::debug!(
                            "DEPYLER-1205: Inferred {} as List (via {}())",
                            var_name,
                            method
                        );
                    }
                    // String-indicator methods
                    "lower" | "upper" | "strip" | "lstrip" | "rstrip" | "split" | "join"
                    | "replace" | "startswith" | "endswith" | "find" | "rfind" | "isdigit"
                    | "isalpha" | "isalnum" | "isupper" | "islower" | "title" | "capitalize"
                    | "swapcase" | "center" | "ljust" | "rjust" | "zfill" | "encode" => {
                        // This variable must be a string
                        self.ctx.var_types.insert(var_name.clone(), Type::String);
                        tracing::debug!(
                            "DEPYLER-1205: Inferred {} as String (via {}())",
                            var_name,
                            method
                        );
                    }
                    // Dict-indicator methods
                    "keys" | "values" | "items" | "get" | "setdefault" | "update" | "popitem" => {
                        // This variable must be a dict
                        self.ctx.var_types.insert(
                            var_name.clone(),
                            Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
                        );
                        tracing::debug!(
                            "DEPYLER-1205: Inferred {} as Dict (via {}())",
                            var_name,
                            method
                        );
                    }
                    // Iterator-indicator methods (could be list, set, or dict)
                    "iter" => {
                        // Default to list for iter()
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::List(Box::new(Type::Unknown)));
                        tracing::debug!("DEPYLER-1205: Inferred {} as List (via iter())", var_name);
                    }
                    // Set-indicator methods
                    "add"
                    | "discard"
                    | "difference"
                    | "intersection"
                    | "union"
                    | "symmetric_difference"
                    | "issubset"
                    | "issuperset"
                    | "isdisjoint" => {
                        // This variable must be a set
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::Set(Box::new(Type::Unknown)));
                        tracing::debug!(
                            "DEPYLER-1205: Inferred {} as Set (via {}())",
                            var_name,
                            method
                        );
                    }
                    _ => {}
                }
            }
        }

        // DEPYLER-GH208: Handle is_none/is_some on &mut Option<T> parameters
        // When a parameter with Optional type is mutated, it becomes &mut Option<T>.
        // is_none()/is_some() work via auto-deref, but we generate explicit code for clarity.
        // Also handle method calls on the Option's inner value (e.g., datetime methods).
        if let HirExpr::Var(var_name) = object {
            if self.ctx.mut_option_params.contains(var_name) {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match method {
                    "is_none" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.is_none() });
                    }
                    "is_some" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.is_some() });
                    }
                    // For other methods on Option inner value, unwrap first
                    _ => {
                        // Check if this is a method that should be called on the inner value
                        // Common datetime methods that need unwrapping
                        let needs_unwrap = matches!(
                            method,
                            "year"
                                | "month"
                                | "day"
                                | "hour"
                                | "minute"
                                | "second"
                                | "weekday"
                                | "isoweekday"
                                | "timestamp"
                                | "date"
                                | "time"
                                | "replace"
                                | "strftime"
                                | "isoformat"
                        );
                        if needs_unwrap {
                            let method_ident =
                                syn::Ident::new(method, proc_macro2::Span::call_site());
                            let arg_exprs: Vec<syn::Expr> = args
                                .iter()
                                .map(|arg| arg.to_rust_expr(self.ctx))
                                .collect::<Result<Vec<_>>>()?;
                            if arg_exprs.is_empty() {
                                return Ok(
                                    parse_quote! { #var_ident.as_ref().expect("value is None").#method_ident() },
                                );
                            } else {
                                return Ok(
                                    parse_quote! { #var_ident.as_ref().expect("value is None").#method_ident(#(#arg_exprs),*) },
                                );
                            }
                        }
                    }
                }
            }
        }

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
                            return Ok(
                                parse_quote! { #var_ident.as_ref().expect("value is None").get() },
                            );
                        }
                        let key_expr = args[0].to_rust_expr(self.ctx)?;
                        // Check if we need default value (2-arg form)
                        if args.len() > 1 {
                            let default_expr = args[1].to_rust_expr(self.ctx)?;
                            return Ok(parse_quote! {
                                #var_ident.as_ref().expect("value is None").get(&#key_expr).cloned().unwrap_or(#default_expr)
                            });
                        } else {
                            // Single arg form - return Option<&V>
                            return Ok(parse_quote! {
                                #var_ident.as_ref().expect("value is None").get(&#key_expr).cloned()
                            });
                        }
                    }
                    "contains_key" | "__contains__" => {
                        if !args.is_empty() {
                            let key_expr = args[0].to_rust_expr(self.ctx)?;
                            return Ok(parse_quote! {
                                #var_ident.as_ref().expect("value is None").contains_key(&#key_expr)
                            });
                        }
                    }
                    "keys" if args.is_empty() => {
                        return Ok(
                            parse_quote! { #var_ident.as_ref().expect("value is None").keys() },
                        );
                    }
                    "values" if args.is_empty() => {
                        return Ok(
                            parse_quote! { #var_ident.as_ref().expect("value is None").values() },
                        );
                    }
                    "items" if args.is_empty() => {
                        return Ok(
                            parse_quote! { #var_ident.as_ref().expect("value is None").iter() },
                        );
                    }
                    "len" if args.is_empty() => {
                        return Ok(
                            parse_quote! { #var_ident.as_ref().expect("value is None").len() as i32 },
                        );
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

        // DEPYLER-99MODE-S9: Handle is_some/is_none on Result-returning function calls
        // Pattern: `func_call(...) is not None` → HIR: MethodCall { object: Call(func), method: "is_some" }
        // When the called function returns Result<Option<T>>, we need `?` to unwrap Result first:
        //   func_call(...).is_some()  → func_call(...)?.is_some()
        if (method == "is_some" || method == "is_none") && args.is_empty() {
            if let HirExpr::Call { func, .. } = object {
                if self.ctx.type_mapper.nasa_mode
                    && self.ctx.result_returning_functions.contains(func)
                    && self.ctx.current_function_can_fail
                {
                    let object_expr = object.to_rust_expr(self.ctx)?;
                    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #object_expr?.#method_ident() });
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
                                #var_ident.as_mut().expect("value is None").wait().ok().and_then(|s| s.code()).unwrap_or(-1)
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
            let arg_exprs: Vec<syn::Expr> =
                args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
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
                                tokio::runtime::Runtime::new().expect("operation failed").block_on(#arg)
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
            let arg_exprs: Vec<syn::Expr> =
                args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

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
        if method == "update"
            && !args.is_empty()
            && !self.is_dict_expr(object)
            && !self.is_set_expr(object)
        {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> =
                args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
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
        // DEPYLER-99MODE-S9: Skip stdlib module routing if the variable is a known local variable.
        // This prevents local variables named 'copy', 'calendar', 'string', etc. from
        // being mistakenly treated as stdlib module references.
        let is_stdlib_module = if let HirExpr::Var(name) = object {
            !self.ctx.is_declared(name)
                && matches!(
                    name.as_str(),
                    "re" | "json"
                        | "math"
                        | "random"
                        | "os"
                        | "sys"
                        | "time"
                        | "datetime"
                        | "pathlib"
                        | "struct"
                        | "statistics"
                        | "fractions"
                        | "decimal"
                        | "collections"
                        | "itertools"
                        | "functools"
                        | "shutil"
                        | "csv"
                        | "base64"
                        | "hashlib"
                        | "subprocess"
                        | "string"
                        | "tempfile"
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
                    // DEPYLER-99MODE-FIX: "replace" removed from here - handled earlier with arg count check
                    // to allow datetime.replace(kwargs) to fall through to datetime handler
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
            let arg_exprs: Vec<syn::Expr> =
                args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
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
            let arg_exprs: Vec<syn::Expr> =
                args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
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
            let arg_exprs: Vec<syn::Expr> =
                args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
            return self.convert_datetime_instance_method(&object_expr, method, args, &arg_exprs);
        }

        // DEPYLER-0416: Check if this is a static method call on a class (e.g., Point.origin())
        // Convert to ClassName::method(args)
        // DEPYLER-0458 FIX: Exclude CONST_NAMES (all uppercase) from static method conversion
        // Constants like DEFAULT_CONFIG should use instance methods (.clone()) not static (::copy())
        if let HirExpr::Var(class_name) = object {
            let is_const = class_name.chars().all(|c| c.is_uppercase() || c == '_');
            let starts_uppercase =
                class_name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);

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
            if let HirExpr::MethodCall { object: inner_obj, method: inner_method, .. } = object {
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
        // GH-204: Extended to properly use module mappings for stdlib modules like logging
        if let HirExpr::Var(module_name) = object {
            // GH-204: Handle collections module constructor patterns FIRST
            // Route collections.Counter, collections.deque, collections.defaultdict
            // to proper builtin converter functions instead of generic module::function()
            // This MUST be checked before the generic module mapping handling below
            if module_name == "collections" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                match method {
                    "Counter" => {
                        return crate::rust_gen::collection_constructors::convert_counter_builtin(
                            self.ctx, &arg_exprs,
                        );
                    }
                    "deque" => {
                        return crate::rust_gen::collection_constructors::convert_deque_builtin(
                            self.ctx, &arg_exprs,
                        );
                    }
                    "defaultdict" => {
                        return crate::rust_gen::collection_constructors::convert_defaultdict_builtin(
                            self.ctx,
                            &arg_exprs,
                        );
                    }
                    _ => {} // Fall through to generic handling for other collections methods
                }
            }

            // Check if this is an imported module (not a local variable)
            // Use all_imported_modules which includes external unmapped modules like requests
            if self.ctx.all_imported_modules.contains(module_name) {
                // GH-204: Check if this module has a mapping with a Rust equivalent
                if let Some(mapping) = self.ctx.imported_modules.get(module_name) {
                    // Get the rust path and item mapping
                    let rust_path = &mapping.rust_path;
                    let rust_method = mapping.item_map.get(method);

                    // GH-204: If we have a mapped method and non-empty rust_path
                    if let Some(rust_name) = rust_method {
                        // Check if this is a macro call (ends with !)
                        if rust_name.ends_with('!') {
                            // Macro call - generate macro_name!(args)
                            let macro_name_str = rust_name.trim_end_matches('!');
                            let macro_ident =
                                syn::Ident::new(macro_name_str, proc_macro2::Span::call_site());

                            let arg_exprs: Vec<syn::Expr> = args
                                .iter()
                                .map(|arg| arg.to_rust_expr(self.ctx))
                                .collect::<Result<Vec<_>>>()?;

                            return Ok(parse_quote! { #macro_ident!(#(#arg_exprs),*) });
                        } else if !rust_path.is_empty() {
                            // Regular function call with full path
                            let full_path: syn::Path =
                                syn::parse_str(&format!("{}::{}", rust_path, rust_name))?;

                            let arg_exprs: Vec<syn::Expr> = args
                                .iter()
                                .map(|arg| arg.to_rust_expr(self.ctx))
                                .collect::<Result<Vec<_>>>()?;

                            return Ok(parse_quote! { #full_path(#(#arg_exprs),*) });
                        }
                    }
                }

                // Fallback: Generate module::function() syntax for unmapped modules
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

        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // DEPYLER-0445: Pass original args and kwargs separately to convert_instance_method
        // Some methods like sort(key=func) need to preserve keyword argument names
        // For other methods, they can merge kwargs as positional if needed
        self.convert_instance_method(object, &object_expr, method, &arg_exprs, args, kwargs)
    }
}
