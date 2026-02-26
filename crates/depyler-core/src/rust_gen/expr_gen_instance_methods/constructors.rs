//! Collection constructor handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size and improve maintainability.
//! Contains handlers for: list, tuple, set, frozenset constructors.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::trace_decision;
use anyhow::Result;
use quote::quote;
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_list(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        // CITL: Trace list construction decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "list_construction",
            chosen = "vec_macro",
            alternatives = ["Vec_new", "array_literal", "smallvec", "tinyvec"],
            confidence = 0.90
        );

        // DEPYLER-0269 FIX: Convert string literals to owned Strings
        // List literals with string elements should use Vec<String> not Vec<&str>
        // This ensures they can be passed to functions expecting &Vec<String>

        // DEPYLER-0572: Detect if list has mixed types (dict access Value + format! String)
        // If so, unify to String by converting Value elements via .to_string()
        let has_dict_access = elts.iter().any(|e| self.is_dict_value_access(e));
        let has_fstring = elts.iter().any(|e| matches!(e, HirExpr::FString { .. }));
        let needs_string_unify = has_dict_access && has_fstring;

        // DEPYLER-0711: Detect heterogeneous list literals (mixed primitive types)
        // Rust's vec![] requires all elements to be the same type
        // For mixed types like [1, "hello", 3.14, true], use Vec<serde_json::Value>
        let has_mixed_types = self.list_has_mixed_types(elts);

        // DEPYLER-0741: Detect if list contains dicts and if ANY dict has None values
        // If so, ALL dicts must use Option<V> for type consistency
        let any_dict_has_none = elts.iter().any(|e| {
            if let HirExpr::Dict(items) = e {
                items.iter().any(|(_, v)| matches!(v, HirExpr::Literal(Literal::None)))
            } else {
                false
            }
        });

        // Set flag before processing so convert_dict knows to wrap values in Some()
        if any_dict_has_none {
            self.ctx.force_dict_value_option_wrap = true;
        }

        // Scope guard: reset flag after processing list elements
        let result = self.convert_list_elements(elts, has_mixed_types, needs_string_unify);
        self.ctx.force_dict_value_option_wrap = false;
        result
    }

    /// DEPYLER-0741: Helper to convert list elements, allowing the flag to be reset afterward
    pub(crate) fn convert_list_elements(
        &mut self,
        elts: &[HirExpr],
        has_mixed_types: bool,
        needs_string_unify: bool,
    ) -> Result<syn::Expr> {
        let nasa_mode = self.ctx.type_mapper.nasa_mode;

        // DEPYLER-1201: Boundary Enforcement for Vec<DepylerValue>
        // When target type expects Vec<DepylerValue> (from List[Any], List[Unknown], etc.),
        // wrap ALL elements in DepylerValue constructors to prevent E0308 type mismatches.
        // This enforces strict type boundaries: `1` → `DepylerValue::Int(1)` when needed.
        let target_needs_depyler_value = if let Some(Type::List(elem_type)) =
            &self.ctx.current_assign_type
        {
            matches!(elem_type.as_ref(), Type::Unknown)
                || matches!(elem_type.as_ref(), Type::UnificationVar(_))
                || matches!(elem_type.as_ref(), Type::Custom(name) if name == "DepylerValue" || name == "Any" || name == "object")
        } else {
            false
        };

        // Also check current_return_type for function return context
        let return_needs_depyler_value = if let Some(Type::List(elem_type)) =
            &self.ctx.current_return_type
        {
            matches!(elem_type.as_ref(), Type::Unknown)
                || matches!(elem_type.as_ref(), Type::UnificationVar(_))
                || matches!(elem_type.as_ref(), Type::Custom(name) if name == "DepylerValue" || name == "Any" || name == "object")
        } else {
            false
        };

        // DEPYLER-1212: If no target type is specified AND elements have unknown/mixed types,
        // default to DepylerValue wrapping to prevent E0308 type mismatches.
        // This is the "genesis fix" - catching the problem at list literal creation.
        let inferred_needs_depyler_value = if self.ctx.current_assign_type.is_none()
            && self.ctx.current_return_type.is_none()
            && nasa_mode
        {
            self.list_needs_depyler_value(elts)
        } else {
            false
        };

        if nasa_mode
            && (target_needs_depyler_value
                || return_needs_depyler_value
                || inferred_needs_depyler_value)
        {
            self.ctx.needs_depyler_value_enum = true;

            let elt_exprs: Vec<syn::Expr> = elts
                .iter()
                .map(|e| {
                    let expr = e.to_rust_expr(self.ctx)?;
                    // Wrap each element in appropriate DepylerValue constructor
                    let wrapped = match e {
                        HirExpr::Literal(Literal::Int(_)) => {
                            parse_quote! { DepylerValue::Int(#expr as i64) }
                        }
                        HirExpr::Literal(Literal::Float(_)) => {
                            parse_quote! { DepylerValue::Float(#expr) }
                        }
                        HirExpr::Literal(Literal::String(_)) => {
                            parse_quote! { DepylerValue::Str(#expr.to_string()) }
                        }
                        HirExpr::Literal(Literal::Bool(_)) => {
                            parse_quote! { DepylerValue::Bool(#expr) }
                        }
                        HirExpr::Literal(Literal::None) => {
                            parse_quote! { DepylerValue::None }
                        }
                        HirExpr::List(_) => {
                            // Nested list: recursively convert and wrap
                            parse_quote! { DepylerValue::List(#expr) }
                        }
                        HirExpr::Dict(_) => {
                            // Nested dict: wrap in DepylerValue::Dict
                            parse_quote! { DepylerValue::Dict(#expr) }
                        }
                        // DEPYLER-1212: For variables, look up type and use appropriate constructor
                        HirExpr::Var(name) => {
                            match self.ctx.var_types.get(name) {
                                Some(Type::Int) => parse_quote! { DepylerValue::Int(#expr as i64) },
                                Some(Type::Float) => {
                                    parse_quote! { DepylerValue::Float(#expr as f64) }
                                }
                                Some(Type::String) => {
                                    parse_quote! { DepylerValue::Str(#expr.to_string()) }
                                }
                                Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#expr) },
                                // For &String references (from match arms), clone and wrap
                                _ => parse_quote! { DepylerValue::Str(#expr.to_string()) },
                            }
                        }
                        _ => {
                            // For complex expressions, default to Str with Debug format
                            parse_quote! { DepylerValue::Str(format!("{:?}", #expr)) }
                        }
                    };
                    Ok(wrapped)
                })
                .collect::<Result<Vec<_>>>()?;

            return Ok(parse_quote! { vec![#(#elt_exprs),*] });
        }

        // DEPYLER-0270: Check if return type explicitly specifies a concrete list element type
        // If so, trust the annotation and skip mixed-type fallback
        // DEPYLER-99MODE-S9: Also match nested List/Dict/Set/Tuple types as concrete
        let has_concrete_return_type = matches!(
            &self.ctx.current_return_type,
            Some(Type::List(elem_type)) if matches!(
                elem_type.as_ref(),
                Type::Int | Type::Float | Type::String | Type::Bool
                    | Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::Tuple(_)
            )
        ) || matches!(
            &self.ctx.current_assign_type,
            Some(Type::List(elem_type)) if matches!(
                elem_type.as_ref(),
                Type::Int | Type::Float | Type::String | Type::Bool
                    | Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::Tuple(_)
            )
        );

        if has_mixed_types && !has_concrete_return_type {
            // DEPYLER-1033: In NASA mode, convert all elements to String instead of using serde_json
            if nasa_mode {
                let elt_exprs: Vec<syn::Expr> = elts
                    .iter()
                    .map(|e| {
                        let expr = e.to_rust_expr(self.ctx)?;
                        // Convert all elements to String for NASA mode compatibility
                        Ok(parse_quote! { format!("{:?}", #expr) })
                    })
                    .collect::<Result<Vec<_>>>()?;

                return Ok(parse_quote! { vec![#(#elt_exprs),*] });
            }

            // DEPYLER-0711: Convert to Vec<serde_json::Value> for heterogeneous lists
            self.ctx.needs_serde_json = true;

            let elt_exprs: Vec<syn::Expr> = elts
                .iter()
                .map(|e| {
                    let expr = e.to_rust_expr(self.ctx)?;
                    // Wrap each element in serde_json::json!()
                    Ok(parse_quote! { serde_json::json!(#expr) })
                })
                .collect::<Result<Vec<_>>>()?;

            return Ok(parse_quote! { vec![#(#elt_exprs),*] });
        }

        // DEPYLER-0739: Detect if list contains None elements
        // If so, wrap non-None elements in Some() to create Vec<Option<T>>
        let has_none = elts.iter().any(|e| matches!(e, HirExpr::Literal(Literal::None)));

        if has_none {
            let elt_exprs: Vec<syn::Expr> = elts
                .iter()
                .map(|e| {
                    if matches!(e, HirExpr::Literal(Literal::None)) {
                        // None stays as None
                        Ok(parse_quote! { None })
                    } else {
                        // Non-None elements get wrapped in Some()
                        let mut expr = e.to_rust_expr(self.ctx)?;
                        // Convert string literals to owned Strings
                        if matches!(e, HirExpr::Literal(Literal::String(_))) {
                            expr = parse_quote! { #expr.to_string() };
                        }
                        Ok(parse_quote! { Some(#expr) })
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            return Ok(parse_quote! { vec![#(#elt_exprs),*] });
        }

        // DEPYLER-0782: Check if list has string literals to determine if it's Vec<String>
        let has_string_literals =
            elts.iter().any(|e| matches!(e, HirExpr::Literal(Literal::String(_))));

        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let mut expr = e.to_rust_expr(self.ctx)?;
                // Check if element is a string literal
                if matches!(e, HirExpr::Literal(Literal::String(_))) {
                    expr = parse_quote! { #expr.to_string() };
                }
                // DEPYLER-0782: Variables need .to_string() in Vec<String> context
                // Both constants (SCRIPT: &str) and parameters (arg: &str) need conversion
                // String.to_string() is a no-op, so safe to call on any string type
                if matches!(e, HirExpr::Var(_)) && has_string_literals {
                    expr = parse_quote! { #expr.to_string() };
                }
                // DEPYLER-99MODE-S9: Clone String/Vec/HashMap variables in list literals
                // Python list creation never moves the original, so clone to prevent E0382
                if let HirExpr::Var(name) = e {
                    if !has_string_literals {
                        // Only clone if not already handled by .to_string() above
                        if let Some(ty) = self.ctx.var_types.get(name) {
                            if matches!(
                                ty,
                                Type::String
                                    | Type::List(_)
                                    | Type::Dict(_, _)
                                    | Type::Set(_)
                                    | Type::Custom(_)
                            ) {
                                expr = parse_quote! { #expr.clone() };
                            }
                        }
                    }
                }
                // DEPYLER-0572: Convert dict Value to String when mixed with f-strings
                if needs_string_unify && self.is_dict_value_access(e) {
                    expr = parse_quote! { #expr.to_string() };
                }
                Ok(expr)
            })
            .collect::<Result<Vec<_>>>()?;

        // Always use vec! for now to ensure mutability works
        // In the future, we should analyze if the list is mutated before deciding
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    /// DEPYLER-1144: Convert list literal with explicit f64 coercion for integer elements
    /// Used when the target type is known to be Vec<f64> (e.g., class field typed as list[float])
    /// Converts `[0, 1, 2]` → `vec![0.0, 1.0, 2.0]`
    pub(crate) fn convert_list_with_float_coercion(
        &mut self,
        elts: &[HirExpr],
    ) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                match e {
                    // Integer literals: convert to f64 literal
                    HirExpr::Literal(Literal::Int(n)) => {
                        let float_val = *n as f64;
                        Ok(parse_quote! { #float_val })
                    }
                    // Float literals: use as-is
                    HirExpr::Literal(Literal::Float(f)) => Ok(parse_quote! { #f }),
                    // Variables: cast to f64 at runtime
                    HirExpr::Var(name) => {
                        let var_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                        // Check if variable is already known to be float
                        if let Some(var_type) = self.ctx.var_types.get(name) {
                            if matches!(var_type, Type::Float) {
                                return Ok(parse_quote! { #var_ident });
                            }
                        }
                        Ok(parse_quote! { (#var_ident as f64) })
                    }
                    // Other expressions: convert and cast
                    _ => {
                        let expr = e.to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { (#expr as f64) })
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    /// DEPYLER-0711: Check if list has heterogeneous element types
    /// Returns true if elements have different primitive types (int, string, float, bool)
    /// DEPYLER-1212: Also checks variable types from var_types context
    pub(crate) fn list_has_mixed_types(&self, elts: &[HirExpr]) -> bool {
        if elts.len() <= 1 {
            return false; // Single element or empty - no mixing possible
        }

        let mut has_bool = false;
        let mut has_int = false;
        let mut has_float = false;
        let mut has_string = false;
        let mut has_unknown = false;

        for elem in elts {
            match elem {
                HirExpr::Literal(Literal::Bool(_)) => has_bool = true,
                HirExpr::Literal(Literal::Int(_)) => has_int = true,
                HirExpr::Literal(Literal::Float(_)) => has_float = true,
                HirExpr::Literal(Literal::String(_)) => has_string = true,
                // DEPYLER-1212: Check variable types from context
                HirExpr::Var(name) => {
                    match self.ctx.var_types.get(name) {
                        Some(Type::Bool) => has_bool = true,
                        Some(Type::Int) => has_int = true,
                        Some(Type::Float) => has_float = true,
                        Some(Type::String) => has_string = true,
                        _ => has_unknown = true, // Unknown or complex type
                    }
                }
                // DEPYLER-99MODE-S9: Try to infer type from complex expressions
                _ => match self.infer_element_type_from_expr(elem) {
                    Some(Type::Bool) => has_bool = true,
                    Some(Type::Int) => has_int = true,
                    Some(Type::Float) => has_float = true,
                    Some(Type::String) => has_string = true,
                    _ => has_unknown = true,
                },
            }
        }

        // Count how many distinct known types we have
        let distinct_types =
            [has_bool, has_int, has_float, has_string].iter().filter(|&&b| b).count();

        // Mixed types if:
        // 1. We have more than one distinct known type, OR
        // 2. We have at least one known type AND unknown types
        distinct_types > 1 || (distinct_types > 0 && has_unknown)
    }

    /// DEPYLER-99MODE-S9: Infer type of a complex expression for mixed-type detection
    /// Used in list_has_mixed_types to avoid false-positive mixed type detection
    fn infer_element_type_from_expr(&self, expr: &HirExpr) -> Option<Type> {
        match expr {
            // Index into a typed collection: list[i] → element type
            HirExpr::Index { base, .. } => {
                if let HirExpr::Var(name) = base.as_ref() {
                    match self.ctx.var_types.get(name) {
                        Some(Type::List(elem)) => Some(elem.as_ref().clone()),
                        Some(Type::Dict(_, val)) => Some(val.as_ref().clone()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            // Call to a function with known return type
            HirExpr::Call { func, .. } => self.ctx.function_return_types.get(func).cloned(),
            // Binary: if both sides are int, result is int; if either float, result is float
            HirExpr::Binary { left, right, .. } => {
                let lt = self.infer_element_type_from_expr(left);
                let rt = self.infer_element_type_from_expr(right);
                match (lt, rt) {
                    (Some(Type::Float), _) | (_, Some(Type::Float)) => Some(Type::Float),
                    (Some(Type::Int), Some(Type::Int)) => Some(Type::Int),
                    (Some(Type::String), _) | (_, Some(Type::String)) => Some(Type::String),
                    _ => None,
                }
            }
            // Literals (shouldn't reach here but be safe)
            HirExpr::Literal(lit) => match lit {
                Literal::Int(_) => Some(Type::Int),
                Literal::Float(_) => Some(Type::Float),
                Literal::String(_) => Some(Type::String),
                Literal::Bool(_) => Some(Type::Bool),
                _ => None,
            },
            // Variables (shouldn't reach here but be safe)
            HirExpr::Var(name) => self.ctx.var_types.get(name).cloned(),
            // MethodCall: check if method is known to return a specific type
            HirExpr::MethodCall { method, .. } => match method.as_str() {
                "len" | "count" | "index" | "find" => Some(Type::Int),
                "lower" | "upper" | "strip" | "replace" | "join" => Some(Type::String),
                _ => None,
            },
            _ => None,
        }
    }

    /// DEPYLER-1212: Infer unified element type from list elements
    /// Returns Some(Type) if all elements have the same type, None otherwise
    fn infer_list_element_type(&self, elts: &[HirExpr]) -> Option<Type> {
        if elts.is_empty() {
            return None;
        }

        // Collect all element types
        let mut types: Vec<Type> = Vec::new();
        for elem in elts {
            let elem_type = match elem {
                HirExpr::Literal(Literal::Bool(_)) => Type::Bool,
                HirExpr::Literal(Literal::Int(_)) => Type::Int,
                HirExpr::Literal(Literal::Float(_)) => Type::Float,
                HirExpr::Literal(Literal::String(_)) => Type::String,
                HirExpr::Literal(Literal::None) => continue, // Skip None for type inference
                HirExpr::Var(name) => {
                    self.ctx.var_types.get(name).cloned().unwrap_or(Type::Unknown)
                }
                _ => Type::Unknown,
            };
            types.push(elem_type);
        }

        if types.is_empty() {
            return None;
        }

        // Check if all types are the same (ignoring Unknown)
        let first_known = types.iter().find(|t| !matches!(t, Type::Unknown));
        if let Some(first) = first_known {
            if types.iter().all(|t| matches!(t, Type::Unknown) || t == first) {
                return Some(first.clone());
            }
        }

        // Mixed or all unknown - no unified type
        None
    }

    /// DEPYLER-1212: Check if list needs DepylerValue wrapping (for NASA mode)
    /// Returns true if:
    /// 1. Elements have mixed types (can't determine uniform type)
    /// 2. All elements are non-literal and their types are unknown
    fn list_needs_depyler_value(&self, elts: &[HirExpr]) -> bool {
        if elts.is_empty() {
            return false;
        }

        // Check if any element is a non-literal with unknown type
        let has_unknown_element = elts.iter().any(|e| {
            match e {
                HirExpr::Literal(_) => false,
                HirExpr::Var(name) => {
                    matches!(
                        self.ctx.var_types.get(name),
                        None | Some(Type::Unknown) | Some(Type::UnificationVar(_))
                    )
                }
                _ => true, // Complex expressions - unknown type
            }
        });

        // If we have unknown elements and can't infer a uniform type, need DepylerValue
        has_unknown_element && self.infer_list_element_type(elts).is_none()
    }

    pub(crate) fn convert_tuple(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        // DEPYLER-0682: Convert string literals in tuples to owned Strings
        // When tuples are used in lists (e.g., Vec<(i32, i32, String)>), string
        // elements need to be owned Strings, not &str references.
        // This ensures type consistency across all tuple elements in a Vec.

        // DEPYLER-1188: Detect variables used multiple times across tuple elements
        // Pattern: (key, dict.get(&key)) - key is moved in position 0, borrowed in position 1
        // Fix: Clone variables that appear both as direct Var AND elsewhere in the tuple
        use crate::rust_gen::var_analysis::collect_vars_in_expr;

        // Step 1: Collect vars from each element and find multi-use vars
        let mut all_vars = std::collections::HashSet::new();
        let mut multi_use_vars = std::collections::HashSet::new();

        for elt in elts {
            let vars_in_elt = collect_vars_in_expr(elt);
            for var in &vars_in_elt {
                if all_vars.contains(var) {
                    // This var appears in multiple elements
                    multi_use_vars.insert(var.clone());
                } else {
                    all_vars.insert(var.clone());
                }
            }
        }

        // Step 2: Also check vars that appear multiple times within the same element
        // (e.g., a single complex expression using 'key' twice)
        for elt in elts {
            if let HirExpr::Var(name) = elt {
                // If this element is a direct var, check if it's also used elsewhere
                for other_elt in elts {
                    if !std::ptr::eq(elt, other_elt) {
                        let other_vars = collect_vars_in_expr(other_elt);
                        if other_vars.contains(name) {
                            multi_use_vars.insert(name.clone());
                        }
                    }
                }
            }
        }

        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let mut expr = e.to_rust_expr(self.ctx)?;

                // DEPYLER-1188: Clone direct variable references that are used multiple times
                // Only clone owned types (String, Vec, HashMap, etc.) - not Copy types
                if let HirExpr::Var(name) = e {
                    if multi_use_vars.contains(name) {
                        // Check if this is an owned type that needs cloning
                        let needs_clone = if let Some(var_type) = self.ctx.var_types.get(name) {
                            matches!(
                                var_type,
                                Type::String
                                    | Type::List(_)
                                    | Type::Dict(_, _)
                                    | Type::Set(_)
                                    | Type::Tuple(_)
                                    | Type::Custom(_)
                            )
                        } else {
                            // Unknown type - assume String for safety (common case)
                            true
                        };

                        if needs_clone {
                            expr = parse_quote! { #expr.clone() };
                        }
                    }
                }

                // Convert string literals to .to_string() for owned String
                if matches!(e, HirExpr::Literal(Literal::String(_))) {
                    expr = parse_quote! { #expr.to_string() };
                }
                Ok(expr)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }

    pub(crate) fn convert_set(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;

        // DEPYLER-0742: Detect if set contains None
        let has_none = elts.iter().any(|e| matches!(e, HirExpr::Literal(Literal::None)));

        // DEPYLER-1163: Detect if elements need DepylerValue wrapping
        // In NASA mode, sets with mixed or unknown types use HashSet<DepylerValue>
        // We must wrap elements in DepylerValue to match the type annotation in stmt_gen
        let needs_depyler_value_wrap =
            self.ctx.type_mapper.nasa_mode && !has_none && !self.elements_are_homogeneous(elts);

        let mut insert_stmts = Vec::new();
        for elem in elts {
            // DEPYLER-0742: Wrap non-None elements in Some() when set has None
            if has_none {
                if matches!(elem, HirExpr::Literal(Literal::None)) {
                    insert_stmts.push(quote! { set.insert(None); });
                } else {
                    let elem_expr = elem.to_rust_expr(self.ctx)?;
                    insert_stmts.push(quote! { set.insert(Some(#elem_expr)); });
                }
            } else if needs_depyler_value_wrap {
                // DEPYLER-1163: Wrap elements in DepylerValue for mixed-type sets
                let wrapped = self.wrap_in_depyler_value(elem)?;
                insert_stmts.push(quote! { set.insert(#wrapped); });
            } else if let HirExpr::Literal(Literal::String(s)) = elem {
                // DEPYLER-SET-STR-FIX: Convert string literals to owned Strings for HashSet<String>
                // Python set literals {"apple", "banana"} with str type need .to_string() in Rust
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                insert_stmts.push(quote! { set.insert(#lit.to_string()); });
            } else {
                let elem_expr = elem.to_rust_expr(self.ctx)?;
                insert_stmts.push(quote! { set.insert(#elem_expr); });
            }
        }
        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_stmts)*
                set
            }
        })
    }

    /// DEPYLER-1163: Check if all elements in a collection are of the same type
    /// Returns true if all elements are literals of the same type (Int, Float, String, Bool)
    fn elements_are_homogeneous(&self, elts: &[HirExpr]) -> bool {
        if elts.is_empty() {
            return false;
        }

        // Determine the first element's type category
        let first_type = match &elts[0] {
            HirExpr::Literal(Literal::Int(_)) => Some("int"),
            HirExpr::Literal(Literal::Float(_)) => Some("float"),
            HirExpr::Literal(Literal::String(_)) => Some("string"),
            HirExpr::Literal(Literal::Bool(_)) => Some("bool"),
            _ => None,
        };

        // If first element isn't a simple literal, not homogeneous
        let Some(expected_type) = first_type else {
            return false;
        };

        // Check all elements match the first type
        elts.iter().all(|e| {
            let elem_type = match e {
                HirExpr::Literal(Literal::Int(_)) => Some("int"),
                HirExpr::Literal(Literal::Float(_)) => Some("float"),
                HirExpr::Literal(Literal::String(_)) => Some("string"),
                HirExpr::Literal(Literal::Bool(_)) => Some("bool"),
                _ => None,
            };
            elem_type == Some(expected_type)
        })
    }

    /// DEPYLER-1163: Wrap an expression in the appropriate DepylerValue variant
    fn wrap_in_depyler_value(&mut self, expr: &HirExpr) -> Result<syn::Expr> {
        match expr {
            HirExpr::Literal(Literal::Int(n)) => Ok(parse_quote! { DepylerValue::Int(#n as i64) }),
            HirExpr::Literal(Literal::Float(f)) => Ok(parse_quote! { DepylerValue::Float(#f) }),
            HirExpr::Literal(Literal::String(s)) => {
                Ok(parse_quote! { DepylerValue::Str(#s.to_string()) })
            }
            HirExpr::Literal(Literal::Bool(b)) => Ok(parse_quote! { DepylerValue::Bool(#b) }),
            HirExpr::Literal(Literal::None) => Ok(parse_quote! { DepylerValue::None }),
            _ => {
                // For complex expressions, convert to Rust and wrap with From trait
                let inner_expr = expr.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { DepylerValue::from(#inner_expr) })
            }
        }
    }

    pub(crate) fn convert_frozenset(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        self.ctx.needs_arc = true;

        // DEPYLER-0742: Detect if frozenset contains None
        let has_none = elts.iter().any(|e| matches!(e, HirExpr::Literal(Literal::None)));

        let mut insert_stmts = Vec::new();
        for elem in elts {
            // DEPYLER-0742: Wrap non-None elements in Some() when set has None
            if has_none {
                if matches!(elem, HirExpr::Literal(Literal::None)) {
                    insert_stmts.push(quote! { set.insert(None); });
                } else {
                    let elem_expr = elem.to_rust_expr(self.ctx)?;
                    insert_stmts.push(quote! { set.insert(Some(#elem_expr)); });
                }
            } else {
                let elem_expr = elem.to_rust_expr(self.ctx)?;
                insert_stmts.push(quote! { set.insert(#elem_expr); });
            }
        }
        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_stmts)*
                std::sync::Arc::new(set)
            }
        })
    }
}
