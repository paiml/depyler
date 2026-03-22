fn get_subscript_value_type(expr: &HirExpr, ctx: &CodeGenContext) -> Option<Type> {
    match expr {
        HirExpr::Var(name) => {
            // Look up the variable's type and extract dict value type
            if let Some(Type::Dict(_, val_type)) = ctx.var_types.get(name) {
                return Some(val_type.as_ref().clone());
            }
            None
        }
        HirExpr::Index { base, .. } => {
            // Recursively get the value type, then extract its dict value type
            // For d["k1"]["k2"], first get value type of d["k1"], then its value type
            if let Type::Dict(_, inner_val_type) = get_subscript_value_type(base.as_ref(), ctx)? {
                return Some(inner_val_type.as_ref().clone());
            }
            None
        }
        _ => None,
    }
}

fn track_option_returning_func(target: &AssignTarget, value: &HirExpr, ctx: &mut CodeGenContext) {
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::Call { func, .. } = value {
            if ctx.option_returning_functions.contains(func) {
                if let Some(ret_type) = ctx.function_return_types.get(func) {
                    ctx.var_types.insert(var_name.clone(), ret_type.clone());
                }
            }
        }
    }
}

fn track_counter_string_var(target: &AssignTarget, value: &HirExpr, ctx: &mut CodeGenContext) {
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::Call { func, args, .. } = value {
            if func == "Counter" && args.len() == 1 {
                let is_string_arg = is_counter_string_argument(&args[0], ctx);
                if is_string_arg {
                    ctx.char_counter_vars.insert(var_name.clone());
                }
            }
        }
    }
}

fn is_counter_string_argument(arg: &HirExpr, ctx: &CodeGenContext) -> bool {
    match arg {
        HirExpr::Var(arg_name) => {
            ctx.var_types.get(arg_name).is_some_and(|t| matches!(t, Type::String))
                || arg_name == "text"
                || arg_name == "s"
                || arg_name == "string"
                || arg_name.ends_with("_text")
        }
        HirExpr::MethodCall { method, .. } => {
            method == "read" || method == "strip" || method == "lower" || method == "upper"
        }
        _ => false,
    }
}

fn track_callable_float_return(target: &AssignTarget, value: &HirExpr, ctx: &mut CodeGenContext) {
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::Call { func, .. } = value {
            // Check Callable[[...], float] pattern
            if let Some(Type::Generic { base, params }) = ctx.var_types.get(func) {
                if base == "Callable" && params.len() == 2 && matches!(params[1], Type::Float) {
                    ctx.var_types.insert(var_name.clone(), Type::Float);
                    return;
                }
            }
            // Check Type::Function pattern
            if let Some(Type::Function { ret, .. }) = ctx.var_types.get(func) {
                if matches!(ret.as_ref(), Type::Float) {
                    ctx.var_types.insert(var_name.clone(), Type::Float);
                    return;
                }
            }
            // Check module-level function return types
            if let Some(Type::Float) = ctx.function_return_types.get(func) {
                ctx.var_types.insert(var_name.clone(), Type::Float);
            }
        }
    }
}

fn track_iterator_var(target: &AssignTarget, value: &HirExpr, ctx: &mut CodeGenContext) {
    if let AssignTarget::Symbol(var_name) = target {
        if is_iterator_producing_expr(value) {
            ctx.iterator_vars.insert(var_name.clone());
            ctx.mutable_vars.insert(var_name.clone());
        }
    }
}

fn track_numpy_var(target: &AssignTarget, value: &HirExpr, ctx: &mut CodeGenContext) {
    if let AssignTarget::Symbol(var_name) = target {
        if is_numpy_value_expr(value, ctx) {
            ctx.numpy_vars.insert(var_name.clone());
        }
    }
}

fn track_csv_reader_mutable(target: &AssignTarget, value: &HirExpr, ctx: &mut CodeGenContext) {
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::MethodCall { object, method, .. } = value {
            if method == "DictReader" || method == "reader" {
                if let HirExpr::Var(module_name) = object.as_ref() {
                    if module_name == "csv" {
                        ctx.mutable_vars.insert(var_name.clone());
                    }
                }
            }
        }
    }
}

fn handle_none_placeholder(
    target: &AssignTarget,
    value: &HirExpr,
    ctx: &mut CodeGenContext,
) -> bool {
    if let AssignTarget::Symbol(var_name) = target {
        let is_none_literal = matches!(value, HirExpr::Literal(Literal::None));
        let is_mutable = ctx.mutable_vars.contains(var_name);
        if is_none_literal && is_mutable {
            ctx.none_placeholder_vars.insert(var_name.clone());
            return true;
        }
    }
    false
}

fn track_os_environ_type(var_name: &str, value: &HirExpr, ctx: &mut CodeGenContext) {
    if let HirExpr::MethodCall { object, method, args, .. } = value {
        // os.environ.get(key, default) - 2 args means String
        if method == "get" && args.len() == 2 && is_os_environ_attr(object.as_ref()) {
            ctx.var_types.insert(var_name.to_string(), Type::String);
            return;
        }
        // os.getenv(key, default) - 2 args means String
        if method == "getenv" && args.len() == 2 {
            if let HirExpr::Var(module) = object.as_ref() {
                if module == "os" {
                    ctx.var_types.insert(var_name.to_string(), Type::String);
                    return;
                }
            }
        }
        // os.environ.get(key) - 1 arg means Option<String>
        if method == "get" && args.len() == 1 && is_os_environ_attr(object.as_ref()) {
            ctx.var_types.insert(var_name.to_string(), Type::Optional(Box::new(Type::String)));
        }
    }
}

fn is_os_environ_attr(expr: &HirExpr) -> bool {
    if let HirExpr::Attribute { value: attr_obj, attr } = expr {
        if let HirExpr::Var(module) = attr_obj.as_ref() {
            return module == "os" && attr == "environ";
        }
    }
    false
}

fn track_json_loads_type(var_name: &str, object: &HirExpr, method: &str, ctx: &mut CodeGenContext) {
    if matches!(method, "loads" | "load") {
        if let HirExpr::Var(obj_var) = object {
            if obj_var == "json" {
                ctx.var_types
                    .insert(var_name.to_string(), Type::Custom("serde_json::Value".to_string()));
            }
        }
    }
}

fn track_value_index_type(var_name: &str, base: &HirExpr, ctx: &mut CodeGenContext) {
    if let HirExpr::Var(base_var) = base {
        if let Some(base_type) = ctx.var_types.get(base_var) {
            let is_value_type = matches!(base_type, Type::Custom(name) if name == "serde_json::Value" || name == "Value");
            if is_value_type {
                ctx.var_types
                    .insert(var_name.to_string(), Type::Custom("serde_json::Value".to_string()));
                return;
            }
            if let Type::Dict(_, v) = base_type {
                let val_is_value = matches!(v.as_ref(), Type::Custom(name) if name == "serde_json::Value" || name == "Value");
                if val_is_value {
                    ctx.var_types.insert(
                        var_name.to_string(),
                        Type::Custom("serde_json::Value".to_string()),
                    );
                }
            }
        }
    }
}

pub(crate) fn codegen_assign_stmt(
    target: &AssignTarget,
    value: &HirExpr,
    type_annotation: &Option<Type>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // CITL: Trace assignment target strategy
    trace_decision!(
        category = DecisionCategory::Ownership,
        name = "assign_target",
        chosen = "let_binding",
        alternatives = ["let_mut", "reassign", "destructure", "augmented"],
        confidence = 0.90
    );

    // DEPYLER-0399: Transform CSE assignments for subcommand comparisons
    // DEPYLER-0456 #2: Use dest_field instead of hardcoded "command"
    // When we have subcommands, assignments like `_cse_temp_0 = args.action == "clone"`
    // would try to compare Commands enum to string (won't compile).
    // Transform into a match expression that returns bool:
    // let _cse_temp_0 = matches!(args.action, Commands::Clone { .. });
    if ctx.argparser_tracker.has_subcommands() {
        // Get dest_field from subparser info
        let dest_field = ctx
            .argparser_tracker
            .subparsers
            .values()
            .next()
            .map(|sp| sp.dest_field.clone())
            .unwrap_or_else(|| "command".to_string());

        if let Some(cmd_name) = is_subcommand_check(value, &dest_field, ctx) {
            // DEPYLER-0940: Skip if cmd_name is empty to prevent panic in format_ident!()
            if !cmd_name.is_empty() {
                if let AssignTarget::Symbol(cse_var) = target {
                    use quote::{format_ident, quote};
                    let variant_name = format_ident!("{}", to_pascal_case(&cmd_name));
                    let var_ident = safe_ident(cse_var);

                    // DEPYLER-0456 #2: Track this CSE temp so is_subcommand_check() can find it
                    ctx.cse_subcommand_temps.insert(cse_var.clone(), cmd_name.clone());

                    // DEPYLER-0456 Bug #3 FIX: Always use "command" as Rust field name
                    // DEPYLER-1063: args.command is Option<Commands>, wrap pattern in Some()
                    return Ok(quote! {
                        let #var_ident = matches!(args.command, Some(Commands::#variant_name { .. }));
                    });
                }
            }
        }
    }

    // DEPYLER-REFACTOR: Use extracted helper functions for variable tracking
    track_option_returning_func(target, value, ctx);
    track_counter_string_var(target, value, ctx);
    track_callable_float_return(target, value, ctx);
    track_iterator_var(target, value, ctx);
    track_numpy_var(target, value, ctx);

    // DEPYLER-REFACTOR: Use extracted helpers for csv and None handling
    track_csv_reader_mutable(target, value, ctx);
    if handle_none_placeholder(target, value, ctx) {
        return Ok(quote! {});
    }

    // DEPYLER-REFACTOR: Use extracted helper for ArgumentParser patterns
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::MethodCall { method, object, args, kwargs, .. } = value {
            if let Some(result) = handle_argparser_method_call(
                target,
                var_name,
                method,
                object.as_ref(),
                args,
                kwargs,
                ctx,
            ) {
                return Ok(result);
            }
        }
    }

    // DEPYLER-0279: Detect and handle dict augmented assignment pattern
    // If we have dict[key] += value, avoid borrow-after-move by evaluating old value first
    // DEPYLER-99MODE-S9: Skip dict pattern for List bases (use direct index instead)
    let base_is_list = if let AssignTarget::Index { base, .. } = target {
        if let HirExpr::Var(name) = base.as_ref() {
            matches!(ctx.var_types.get(name), Some(Type::List(_)))
        } else {
            false
        }
    } else {
        false
    };
    if !base_is_list && is_dict_augassign_pattern(target, value) {
        if let AssignTarget::Index { base, index } = target {
            if let HirExpr::Binary { op, left: _, right } = value {
                // Generate: let old_val = dict.get(&key).cloned().unwrap_or_default();
                //           dict.insert(key, old_val + right_value);
                let base_expr = base.to_rust_expr(ctx)?;
                let index_expr = index.to_rust_expr(ctx)?;
                let right_expr = right.to_rust_expr(ctx)?;
                let op_token = match op {
                    BinOp::Add => quote! { + },
                    BinOp::Sub => quote! { - },
                    BinOp::Mul => quote! { * },
                    BinOp::Div => quote! { / },
                    BinOp::Mod => quote! { % },
                    _ => bail!("Unsupported augmented assignment operator for dict"),
                };

                // DEPYLER-1188: Clone the key to avoid borrow-after-move
                // when #right_expr may also reference the original key variable
                // Pattern: result[key] = result[key] + counter2[key]
                // The key is used in both LHS index and RHS expression
                return Ok(quote! {
                    {
                        let _key = #index_expr.clone();
                        let _old_val = #base_expr.get(&_key).cloned().unwrap_or_default();
                        #base_expr.insert(_key, _old_val #op_token #right_expr);
                    }
                });
            }
        }
    }

    // DEPYLER-0232: Track variable types for class instances
    // This allows proper method dispatch for user-defined classes
    // DEPYLER-0224: Also track types for set/dict/list literals for proper method dispatch
    // DEPYLER-0301: Track list/vec types from slicing operations
    // DEPYLER-0327 Fix #1: Track String type from Vec<String>.get() method calls
    if let AssignTarget::Symbol(var_name) = target {
        // DEPYLER-0272: Track type from type annotation for function return values
        // This enables correct {:?} vs {} selection in println! for collections
        // Example: result = merge(&a, &b) where merge returns Vec<i32>
        // DEPYLER-0709: Also track Tuple types for correct field access (.0, .1)
        // DEPYLER-1219: Also track Optional types to prevent double Some() wrapping on return
        if let Some(annot_type) = type_annotation {
            match annot_type {
                // DEPYLER-1045: Added Type::String to enable auto-borrow in function calls
                // Without this, `text: str = "hello world"` wouldn't be tracked,
                // causing `capitalize_words(text)` to miss the `&` borrow.
                // DEPYLER-1219: Added Type::Optional to track when variables are already Option<T>
                // This prevents double wrapping in return statements (Some(Some(x)))
                // DEPYLER-E0599-PYOPS-FIX: Added Type::Int and Type::Float for primitive tracking
                // Without this, loop body reassignments like `total = total + item` can't find
                // the variable's type for skipping DepylerValue extraction (.to_i64())
                Type::List(_)
                | Type::Dict(_, _)
                | Type::Set(_)
                | Type::Tuple(_)
                | Type::String
                | Type::Optional(_)
                | Type::Int
                | Type::Float
                | Type::Bool => {
                    // DEPYLER-99MODE-S9: Don't overwrite concrete parameter types
                    // with HM-inferred annotations of a different kind.
                    // E.g., `prefix: str` from param should not be overwritten by
                    // `prefix = prefix[:-1]` with annotation List(Int) from HM.
                    let should_insert = match ctx.var_types.get(var_name) {
                        None | Some(Type::Unknown) => true,
                        Some(existing) => {
                            std::mem::discriminant(existing) == std::mem::discriminant(annot_type)
                        }
                    };
                    if should_insert {
                        ctx.var_types.insert(var_name.clone(), annot_type.clone());
                    }
                }
                _ => {}
            }
        } else {
            // DEPYLER-E0599-PYOPS-FIX: Fallback - infer type from literal value expression
            // When constraint_collector doesn't set type_annotation (e.g., NASA mode edge cases),
            // infer the type from the value if it's a literal (0 → Type::Int, 0.0 → Type::Float).
            // This ensures var_types has the correct type for loop body reassignments.
            match value {
                HirExpr::Literal(Literal::Int(_)) => {
                    ctx.var_types.insert(var_name.clone(), Type::Int);
                }
                HirExpr::Literal(Literal::Float(_)) => {
                    ctx.var_types.insert(var_name.clone(), Type::Float);
                }
                // DEPYLER-99MODE-S9: Track bool literals to prevent .is_empty() on bool vars
                HirExpr::Literal(Literal::Bool(_)) => {
                    ctx.var_types.insert(var_name.clone(), Type::Bool);
                }
                // DEPYLER-99MODE-S9: Track string literals for correct truthiness
                HirExpr::Literal(Literal::String(_)) => {
                    ctx.var_types.insert(var_name.clone(), Type::String);
                }
                _ => {}
            }
        }

        // DEPYLER-REFACTOR: Use extracted helper for os.environ tracking
        track_os_environ_type(var_name, value, ctx);

        // DEPYLER-0455: Track Option types from method calls like .ok() and .get()
        // This enables proper truthiness conversion (if option → if option.is_some())
        // Example: config_file = os.environ.get("CONFIG_FILE")
        //          or: config_file = std::env::var(...).ok()
        // DEPYLER-0479: Skip if already tracked (e.g., unwrap_or_else handled above)
        if !ctx.var_types.contains_key(var_name) && looks_like_option_expr(value) {
            // Track as Option<String> for now (generic placeholder)
            // The exact inner type doesn't matter for truthiness conversion
            ctx.var_types.insert(var_name.clone(), Type::Optional(Box::new(Type::String)));
        }

        match value {
            HirExpr::Call { func, .. } => {
                // Check if this is a user-defined class constructor
                if ctx.class_names.contains(func) {
                    ctx.var_types.insert(var_name.clone(), Type::Custom(func.clone()));
                }
                // DEPYLER-0309: Track builtin collection constructors for proper method dispatch
                // This enables correct HashSet.contains() vs HashMap.contains_key() selection
                else if func == "set" {
                    // Infer element type from type annotation or default to Int
                    let elem_type = if let Some(Type::Set(elem)) = type_annotation {
                        elem.as_ref().clone()
                    } else {
                        Type::Int // Default for untyped sets
                    };
                    ctx.var_types.insert(var_name.clone(), Type::Set(Box::new(elem_type)));
                }
                // DEPYLER-REFACTOR: Use extracted helper for deque/Queue tracking
                else if matches!(
                    func.as_str(),
                    "deque"
                        | "collections.deque"
                        | "Deque"
                        | "Queue"
                        | "LifoQueue"
                        | "PriorityQueue"
                        | "heappush"
                        | "heapify"
                ) {
                    track_deque_constructor(var_name, func, type_annotation, ctx);
                }
                // DEPYLER-0269: Track user-defined function return types
                // Lookup function return type and track it for Display trait selection
                // Enables: result = merge(&a, &b) where merge returns list[int]
                // DEPYLER-0709: Also track Tuple return types for correct field access (.0, .1)
                else if let Some(ret_type) = ctx.function_return_types.get(func) {
                    if matches!(
                        ret_type,
                        Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::Tuple(_)
                    ) {
                        ctx.var_types.insert(var_name.clone(), ret_type.clone());
                    }
                }
                // DEPYLER-0431: Track re.search(), re.match(), re.find() module functions
                // These all return Option<Match> in Rust
                else if matches!(func.as_str(), "search" | "match" | "find") {
                    // Only track if this looks like a regex call (needs more context to be sure)
                    // For now, track any call to search/match/find as Optional
                    // This is a heuristic - could be improved with module tracking
                    ctx.var_types.insert(var_name.clone(), Type::Optional(Box::new(Type::Unknown)));
                }
                // DEPYLER-0785: Track float return types for CSE comparison coercion
                // When f() returns float, result = f(x) should track result as Float
                else if matches!(ctx.function_return_types.get(func), Some(Type::Float)) {
                    ctx.var_types.insert(var_name.clone(), Type::Float);
                }
            }
            // DEPYLER-0785: Track Binary expressions that return float
            // When CSE extracts `f(a) * f(b)` into `_cse_temp_0`, we need to track
            // that the temp is float so `_cse_temp_0 > 0` coerces 0 to 0f64.
            HirExpr::Binary { .. } => {
                if expr_infers_float(value, ctx) {
                    ctx.var_types.insert(var_name.clone(), Type::Float);
                }
            }
            // DEPYLER-0803: Propagate types from variable-to-variable assignments
            // When dx = _cse_temp_0 where _cse_temp_0 is Float, dx should also be Float
            // This enables proper coercion in expressions like i * dx where i is int
            HirExpr::Var(source_var) => {
                if let Some(source_type) = ctx.var_types.get(source_var) {
                    ctx.var_types.insert(var_name.clone(), source_type.clone());
                }
            }
            HirExpr::List(elements) => {
                // DEPYLER-0269: Track list type from literal for auto-borrowing
                // When v = [1, 2], mark v as List(Int) so it gets borrowed when calling f(&v)
                // DEPYLER-1206: Also check current_return_type for type propagation
                // Pattern: def f() -> List[ValidationError]: errors = []; return errors
                // The empty list should get List[ValidationError] from return type, not Unknown
                let elem_type = if let Some(Type::List(elem)) = type_annotation {
                    elem.as_ref().clone()
                } else if let Some(Type::List(elem)) = &ctx.current_return_type {
                    // Use return type's list element type (handles List[CustomType])
                    elem.as_ref().clone()
                } else if !elements.is_empty() {
                    // Infer from first element (assume homogeneous list)
                    // For int literals, use Int type
                    Type::Int
                } else {
                    Type::Unknown
                };

                // DEPYLER-1134: Guard against overwriting Oracle-seeded types
                // If var_types already has a CONCRETE type (from Oracle or return type propagation),
                // don't overwrite it with Unknown. This protects the Oracle's wisdom from local inference.
                // DEPYLER-1207: Pattern matching correction - use **existing_elem to dereference Box
                let should_update = match ctx.var_types.get(var_name) {
                    None => true,                // No existing type, safe to insert
                    Some(Type::Unknown) => true, // Unknown can be overwritten
                    Some(Type::List(existing_elem)) => {
                        // Only overwrite if existing element type is Unknown AND new type is concrete
                        matches!(**existing_elem, Type::Unknown)
                            && !matches!(elem_type, Type::Unknown)
                    }
                    _ => false, // Don't overwrite any other concrete type with a list type
                };

                if should_update {
                    ctx.var_types.insert(var_name.clone(), Type::List(Box::new(elem_type)));
                }
            }
            HirExpr::Dict(items) => {
                // DEPYLER-0269: Track dict type from literal for auto-borrowing
                // When info = {"a": 1}, mark info as Dict(String, Int) so it gets borrowed
                // DEPYLER-0560: Check function return type for Dict[str, Any] pattern
                // DEPYLER-1219: Preserve Optional wrapper when annotation is Optional[Dict[...]]
                let (key_type, val_type, is_optional) =
                    if let Some(Type::Dict(k, v)) = type_annotation {
                        (k.as_ref().clone(), v.as_ref().clone(), false)
                    } else if let Some(Type::Optional(inner)) = type_annotation {
                        // Handle Optional[Dict[K, V]] - extract types but preserve Optional wrapper
                        if let Type::Dict(k, v) = inner.as_ref() {
                            (k.as_ref().clone(), v.as_ref().clone(), true)
                        } else {
                            (Type::Unknown, Type::Unknown, true)
                        }
                    } else if let Some(Type::Dict(k, v)) = &ctx.current_return_type {
                        // Use return type's dict value type (handles Dict[str, Any] → Unknown)
                        (k.as_ref().clone(), v.as_ref().clone(), false)
                    } else if !items.is_empty() {
                        // DEPYLER-1143: Use Unknown value type for dicts without annotation
                        // This allows dict_has_mixed_types to properly detect heterogeneous dicts
                        // and use DepylerValue for argparse result dicts like:
                        //   result = {"name": args.name, "debug": args.debug}  # String + bool
                        // Using Type::Int here would incorrectly trigger target_has_concrete_value_type
                        (Type::String, Type::Unknown, false)
                    } else {
                        (Type::Unknown, Type::Unknown, false)
                    };
                // DEPYLER-1219: Insert with Optional wrapper if annotation was Optional[Dict[...]]
                let dict_type = Type::Dict(Box::new(key_type), Box::new(val_type));
                let final_type =
                    if is_optional { Type::Optional(Box::new(dict_type)) } else { dict_type };
                ctx.var_types.insert(var_name.clone(), final_type);
            }
            HirExpr::Set(elements) | HirExpr::FrozenSet(elements) => {
                // Track set type from literal for proper method dispatch (DEPYLER-0224)
                // Use type annotation if available, otherwise infer from elements
                let elem_type = if let Some(Type::Set(elem)) = type_annotation {
                    elem.as_ref().clone()
                } else if !elements.is_empty() {
                    // Infer from first element (assume homogeneous set)
                    // For int literals, use Int type
                    Type::Int
                } else {
                    Type::Unknown
                };
                ctx.var_types.insert(var_name.clone(), Type::Set(Box::new(elem_type)));
            }
            // DEPYLER-0600 #6: Track type from comprehension expressions
            // Enables correct {:?} vs {} selection in println! for dict/list/set comprehensions
            HirExpr::DictComp { key, value, .. } => {
                // Use type inference from func_gen module for comprehension types
                let key_type = crate::rust_gen::func_gen::infer_expr_type_simple(key);
                let val_type = crate::rust_gen::func_gen::infer_expr_type_simple(value);
                ctx.var_types
                    .insert(var_name.clone(), Type::Dict(Box::new(key_type), Box::new(val_type)));
            }
            HirExpr::ListComp { element, .. } => {
                // DEPYLER-1206: Use return type if element inference gives Unknown
                let inferred_elem = crate::rust_gen::func_gen::infer_expr_type_simple(element);
                let elem_type = if matches!(inferred_elem, Type::Unknown) {
                    // Fall back to return type's element type
                    if let Some(Type::List(ret_elem)) = &ctx.current_return_type {
                        ret_elem.as_ref().clone()
                    } else {
                        inferred_elem
                    }
                } else {
                    inferred_elem
                };
                ctx.var_types.insert(var_name.clone(), Type::List(Box::new(elem_type)));
            }
            HirExpr::SetComp { element, .. } => {
                let elem_type = crate::rust_gen::func_gen::infer_expr_type_simple(element);
                ctx.var_types.insert(var_name.clone(), Type::Set(Box::new(elem_type)));
            }
            // DEPYLER-0709: Track tuple type from literal for correct field access (.0, .1)
            // Example: result = (1, 2) → result.0, not result.get(0)
            HirExpr::Tuple(elements) => {
                // Infer element types from tuple elements
                let elem_types: Vec<Type> = if let Some(Type::Tuple(types)) = type_annotation {
                    types.clone()
                } else {
                    elements.iter().map(crate::rust_gen::func_gen::infer_expr_type_simple).collect()
                };
                ctx.var_types.insert(var_name.clone(), Type::Tuple(elem_types));
            }
            HirExpr::Slice { base, .. } => {
                // DEPYLER-0301: Track sliced lists as owned Vec types
                // When rest = numbers[1:], mark rest as List(Int) so it gets borrowed on call
                // DEPYLER-99MODE-S9: String slices produce String, not List
                // When prefix = prefix[:-1] where prefix is str, keep String type
                let base_is_string = if let HirExpr::Var(base_var) = base.as_ref() {
                    matches!(ctx.var_types.get(base_var), Some(Type::String))
                } else {
                    false
                };
                if base_is_string {
                    ctx.var_types.insert(var_name.clone(), Type::String);
                } else {
                    // Infer element type from base variable if available
                    let elem_type = if let HirExpr::Var(base_var) = base.as_ref() {
                        if let Some(Type::List(elem)) = ctx.var_types.get(base_var) {
                            elem.as_ref().clone()
                        } else {
                            Type::Int // Default to Int for untyped slices
                        }
                    } else {
                        Type::Int // Default to Int
                    };
                    ctx.var_types.insert(var_name.clone(), Type::List(Box::new(elem_type)));
                }
            }
            // DEPYLER-0327 Fix #1: Track types for method call results
            // E.g., value_str = data.get(...) where data: Vec<String> → value_str: String
            HirExpr::MethodCall { object, method, args, .. } => {
                // Track .get() on Vec<String> returning String
                if method == "get" {
                    if let HirExpr::Var(obj_var) = object.as_ref() {
                        if let Some(Type::List(elem_type)) = ctx.var_types.get(obj_var) {
                            // .get() returns Option<&T>, but after .cloned().unwrap_or_default()
                            // it becomes T, so track the element type
                            ctx.var_types.insert(var_name.clone(), elem_type.as_ref().clone());
                        }
                        // GH-226: Track dict.get(key, default) with 2 args as value type, not Optional
                        // When dict.get has a default, the result is the value type, not Option
                        else if args.len() == 2 {
                            if let Some(Type::Dict(_, val_type)) = ctx.var_types.get(obj_var) {
                                // dict.get(key, default) returns value type directly
                                ctx.var_types.insert(var_name.clone(), val_type.as_ref().clone());
                            } else {
                                // Dict type not tracked, default to String for str(val) compatibility
                                ctx.var_types.insert(var_name.clone(), Type::String);
                            }
                        }
                    }
                }
                // DEPYLER-0421: String methods that return Vec<String> (for truthiness)
                // Track .split() and .split_whitespace() as List(String) for truthiness conversion
                else if matches!(method.as_str(), "split" | "split_whitespace" | "splitlines") {
                    ctx.var_types.insert(var_name.clone(), Type::List(Box::new(Type::String)));
                }
                // String methods that return String
                else if matches!(
                    method.as_str(),
                    "upper"
                        | "lower"
                        | "strip"
                        | "lstrip"
                        | "rstrip"
                        | "title"
                        | "replace"
                        | "format"
                ) {
                    ctx.var_types.insert(var_name.clone(), Type::String);
                }
                // DEPYLER-0431: Regex methods that return Option<Match>
                // Track .find(), .search(), .match() as Optional for truthiness conversion
                else if matches!(method.as_str(), "find" | "search" | "match") {
                    // Check if this is a regex method call (on compiled regex object)
                    // We don't have a specific regex type, so use Optional as a marker
                    ctx.var_types.insert(var_name.clone(), Type::Optional(Box::new(Type::Unknown)));
                }
                // DEPYLER-REFACTOR: Use extracted helper for json.loads tracking
                else if matches!(method.as_str(), "loads" | "load") {
                    track_json_loads_type(var_name, object.as_ref(), method, ctx);
                }
            }
            // DEPYLER-REFACTOR: Use extracted helper for Index/Value tracking
            HirExpr::Index { base, .. } => {
                track_value_index_type(var_name, base.as_ref(), ctx);
            }
            // DEPYLER-0713: Catch-all for primitive literals and other expressions
            // This high-ROI fix prevents UnificationVar → serde_json::Value fallback
            // by tracking concrete types (Int, Float, String, Bool) from literals
            // and expressions. This addresses 600+ E0308 errors in the codebase.
            _ => {
                // DEPYLER-0713 Part 5: Preserve serde_json::Value tracking
                // When a variable was previously assigned from json.loads() or data["key"],
                // it's tracked as Value. If we now assign a primitive like `x = 5`,
                // we should NOT overwrite the Value tracking - it means the variable
                // is being reassigned and the new value needs json!() wrapping.
                let should_preserve_value_type = ctx
                    .var_types
                    .get(var_name)
                    .map(|t| {
                        matches!(t, Type::Custom(name) if name == "serde_json::Value" || name == "Value")
                    })
                    .unwrap_or(false);

                if !should_preserve_value_type {
                    // Use infer_expr_type_simple for all unhandled expressions
                    // This tracks primitive types like Int, Float, String, Bool
                    let inferred = crate::rust_gen::func_gen::infer_expr_type_simple(value);
                    // Only insert if we got a concrete type (not Unknown)
                    if !matches!(inferred, Type::Unknown) {
                        ctx.var_types.insert(var_name.clone(), inferred);
                    }
                }
            }
        }
    }

    // DEPYLER-0472: Set json context when assigning to serde_json::Value dicts
    // This ensures dict literals use json!({}) instead of HashMap::new()
    let prev_json_context = ctx.in_json_context;

    // DEPYLER-1015: In NASA mode, never set json context - use std-only types
    let nasa_mode = ctx.type_mapper.nasa_mode;

    // DEPYLER-0713: Check if target variable is typed as serde_json::Value
    // If so, set json context so literals get wrapped with json!()
    // DEPYLER-1015: Skip in NASA mode
    if !nasa_mode {
        if let AssignTarget::Symbol(var_name) = target {
            // Check if variable is tracked as Value type
            if let Some(var_type) = ctx.var_types.get(var_name) {
                if matches!(var_type, Type::Custom(name) if name == "serde_json::Value" || name == "Value")
                {
                    ctx.in_json_context = true;
                }
            }
            // Also check type annotation
            if let Some(annot) = type_annotation {
                if matches!(annot, Type::Custom(name) if name == "serde_json::Value" || name == "Value" || name == "Any" || name == "any")
                {
                    ctx.in_json_context = true;
                }
            }
        }

        if let AssignTarget::Index { base, .. } = target {
            // DEPYLER-0714: Check actual type FIRST before falling back to name heuristic
            if let HirExpr::Var(base_name) = base.as_ref() {
                // DEPYLER-0713: Check if base is typed as Value
                if let Some(base_type) = ctx.var_types.get(base_name) {
                    if matches!(base_type, Type::Custom(name) if name == "serde_json::Value" || name == "Value")
                    {
                        ctx.in_json_context = true;
                    }
                    // Check if it's a HashMap with Value values OR Unknown values
                    if let Type::Dict(_, v) = base_type {
                        let val_is_json = match v.as_ref() {
                            Type::Unknown => true,
                            Type::Custom(name)
                                if name == "serde_json::Value" || name == "Value" =>
                            {
                                true
                            }
                            _ => false,
                        };
                        if val_is_json {
                            ctx.in_json_context = true;
                        }
                    }
                } else {
                    // DEPYLER-0714: Only use name heuristic when type is NOT known
                    let name_str = base_name.as_str();
                    // Variables commonly used with serde_json::Value
                    if name_str == "config"
                        || name_str == "data"
                        || name_str == "value"
                        || name_str == "current"
                        || name_str == "obj"
                        || name_str == "json"
                    {
                        ctx.in_json_context = true;
                    }
                }
            }
        }
    }

    // DEPYLER-0727: Set assignment target type for dict literal Value wrapping
    // This allows dict codegen to check if target is Dict[str, Any] → HashMap<String, Value>
    let prev_assign_type = ctx.current_assign_type.take();

    // DEPYLER-E0308-FIX: For mut_option_dict_params with Optional(Dict) annotation,
    // extract the inner Dict type. This happens when type inference propagates the
    // parameter type `memo: Dict[int, int] = None` to the assignment `memo = {}`.
    // The empty dict `{}` should use Dict(Int, Int), not Optional(Dict(Int, Int)),
    // because the Some() wrapping is added separately in the code generation.
    if let AssignTarget::Symbol(name) = target {
        if ctx.mut_option_dict_params.contains(name.as_str()) {
            if let Some(Type::Optional(inner)) = type_annotation {
                if let Type::Dict(_, _) = inner.as_ref() {
                    ctx.current_assign_type = Some(inner.as_ref().clone());
                } else {
                    ctx.current_assign_type = type_annotation.clone();
                }
            } else {
                ctx.current_assign_type = type_annotation.clone();
            }
        } else {
            ctx.current_assign_type = type_annotation.clone();
        }
    } else {
        ctx.current_assign_type = type_annotation.clone();
    }

    // DEPYLER-1042: For subscript assignments, propagate inner type from base dict
    // Pattern: d["level1"] = {} where d: Dict[str, Dict[str, str]]
    // The empty dict should get type Dict[str, str], not default HashMap<String, String>
    if ctx.current_assign_type.is_none() {
        if let AssignTarget::Index { base, .. } = target {
            // Extract the value type from the base expression's dict type
            let base_value_type = get_subscript_value_type(base.as_ref(), ctx);
            if base_value_type.is_some() {
                ctx.current_assign_type = base_value_type;
            }
        }
    }

    // DEPYLER-1045: For simple variable assignments, look up the target's type from var_types
    // Pattern: memo = {} where memo: Dict[int, int] (from parameter)
    // The empty dict should get type Dict[int, int], not default HashMap<String, String>
    // DEPYLER-1206: Also handle List types for Smart Coercion boundary enforcement
    // Pattern: nums = [a, b, c] where nums: List[Any] → Vec<DepylerValue>
    // Elements a, b, c (i32) need .into() to become DepylerValue
    if ctx.current_assign_type.is_none() {
        if let AssignTarget::Symbol(name) = target {
            if let Some(var_type) = ctx.var_types.get(name.as_str()) {
                // Handle Option<Dict<K, V>> by extracting the inner Dict type
                // Parameter types may be wrapped in Optional from Optional annotations
                if let Type::Optional(inner) = var_type {
                    if let Type::Dict(_, _) = inner.as_ref() {
                        ctx.current_assign_type = Some(inner.as_ref().clone());
                    } else if let Type::List(_) = inner.as_ref() {
                        // DEPYLER-1206: Also handle Optional<List<T>>
                        ctx.current_assign_type = Some(inner.as_ref().clone());
                    }
                } else if let Type::Dict(_, _) = var_type {
                    ctx.current_assign_type = Some(var_type.clone());
                } else if let Type::List(_) = var_type {
                    // DEPYLER-1206: Handle List types for Smart Coercion
                    ctx.current_assign_type = Some(var_type.clone());
                }
            }
        }
    }

    let mut value_expr = value.to_rust_expr(ctx)?;

    // DEPYLER-1315: Clone non-Copy variables when used as RHS of assignment
    // Python: x = y creates a reference to the same object
    // Rust: x = y moves y (ownership transfer) for non-Copy types
    // Solution: x = y.clone() to preserve Python semantics (both vars valid)
    // This fixes E0382 "use of moved value" when the same variable is used later
    if ctx.type_mapper.nasa_mode {
        if let HirExpr::Var(source_var) = value {
            // Check if source variable has a non-Copy type
            // Non-Copy types: String, Vec, HashMap, HashSet, Box, custom structs
            if let Some(source_type) = ctx.var_types.get(source_var) {
                let needs_clone = matches!(
                    source_type,
                    Type::String
                        | Type::List(_)
                        | Type::Dict(_, _)
                        | Type::Set(_)
                        | Type::Custom(_)
                        | Type::Unknown // Conservative: Unknown might be non-Copy
                );
                if needs_clone {
                    value_expr = parse_quote! { #value_expr.clone() };
                }
            } else {
                // Variable type not known - conservatively clone if it looks like
                // it could be a non-Copy type based on common naming patterns
                // This handles cases where var_types wasn't populated
                let var_lower = source_var.to_lowercase();
                let likely_non_copy = var_lower.contains("path")
                    || var_lower.contains("str")
                    || var_lower.contains("list")
                    || var_lower.contains("vec")
                    || var_lower.contains("dict")
                    || var_lower.contains("map")
                    || var_lower.contains("set")
                    || var_lower.contains("original")
                    || var_lower.contains("copy")
                    || var_lower.contains("data")
                    || var_lower.contains("text")
                    || var_lower.contains("name")
                    || var_lower.contains("word");
                if likely_non_copy {
                    value_expr = parse_quote! { #value_expr.clone() };
                }
            }
        }
    }

    // DEPYLER-1113: Propagate external library return type to var_types
    // When the expression was a MethodCall on an external module (e.g., requests.get),
    // the TypeDB lookup stored the return type. Now propagate it to var_types so
    // subsequent expressions (e.g., resp.json()) have type info.
    if let AssignTarget::Symbol(var_name) = target {
        if let Some(return_type_str) = ctx.last_external_call_return_type.take() {
            // Map the external type string to a Type
            // For now, use Type::Custom with the full qualified name
            ctx.var_types.insert(var_name.clone(), Type::Custom(return_type_str));
        }
    }

    // DEPYLER-0727: Restore previous assignment type
    ctx.current_assign_type = prev_assign_type;

    // DEPYLER-0472: Restore previous json context
    ctx.in_json_context = prev_json_context;

    // DEPYLER-0270: Auto-unwrap Result-returning function calls in assignments
    // When assigning from a function that returns Result<T, E> in a non-Result context,
    // we need to unwrap it.
    //
    // DEPYLER-0422 Fix #8: Also add `?` when BOTH caller and callee return Result
    // Fix #6 removed automatic `?` from expr_gen.rs, so we need to add it here at the
    // statement level where we know the variable type context.
    //
    // Five-Whys Root Cause:
    // 1. Why: expected `i32`, found `Result<i32, Box<dyn Error>>`
    // 2. Why: Variable `position: i32` assigned Result-returning function without unwrap
    // 3. Why: Neither `?` nor `.unwrap()` added to function call
    // 4. Why: Fix #6 removed `?` from expr_gen, and DEPYLER-0270 only adds `.unwrap()` for non-Result callers
    // 5. ROOT CAUSE: Missing `?` for Result→Result propagation after Fix #6
    if let HirExpr::Call { func, .. } = value {
        if ctx.result_returning_functions.contains(func) {
            if ctx.current_function_can_fail {
                // Current function also returns Result - add ? to propagate error
                value_expr = parse_quote! { #value_expr? };
            } else {
                // Current function doesn't return Result - add .unwrap() to extract the value
                value_expr =
                    parse_quote! { #value_expr.expect("function call result unwrap failed") };
            }
        }
    }

    // NOTE: DEPYLER-99MODE-E0308-P2 automatic .into() coercion was REMOVED because it conflicts
    // with DEPYLER-1054's .to_i64()/.to_f64() extraction logic. The P2 fix for cross-type
    // comparisons (DepylerValue > i32) is handled via PartialOrd trait impls in rust_gen.rs.
    // Assignment coercion is handled by DEPYLER-1054 below using explicit extraction methods.

    // If there's a type annotation, handle type conversions
    let (type_annotation_tokens, is_final) = if let Some(target_type) = type_annotation {
        // Check if this is a Final type annotation
        let (actual_type, is_const) = match target_type {
            Type::Final(inner) => (inner.as_ref(), true),
            _ => (target_type, false),
        };

        // DEPYLER-0719: When type annotation is bare `tuple` (empty tuple), infer from
        // function call return type. Python `x: tuple = func()` should use func's return type.
        let actual_type = if matches!(actual_type, Type::Tuple(elems) if elems.is_empty()) {
            // Check if value is a function call
            if let HirExpr::Call { func, .. } = value {
                // Look up function return type
                if let Some(ret_type) = ctx.function_return_types.get(func) {
                    // Use the function's actual return type instead of empty tuple
                    ret_type
                } else {
                    actual_type
                }
            } else {
                actual_type
            }
        } else {
            actual_type
        };

        // DEPYLER-CI-FIX: User-defined class constructor calls with incorrect type inference
        // When HM inference gives a Set/UnificationVar type but value is a class constructor,
        // the type annotation should be the class name, not HashSet<DepylerValue>.
        // Pattern: `calc = Calculator(10)` with inferred Set(UnificationVar(6)) type
        // → Use `let calc: Calculator = Calculator::new(10);` instead of `HashSet<DepylerValue>`
        //
        // We detect this case by checking:
        // 1. actual_type contains UnificationVar (incomplete type inference)
        // 2. value is HirExpr::Call where func is a known class name
        let class_constructor_override: Option<proc_macro2::TokenStream> =
            if let HirExpr::Call { func, .. } = value {
                if ctx.class_names.contains(func) {
                    // DEPYLER-99MODE-S9: If the value is a known class constructor,
                    // always use the class type regardless of HM inference.
                    // HM inference often gives wrong types for class constructors
                    // (Dict, Set, List with UnificationVars).
                    let class_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
                    Some(quote! { : #class_ident })
                } else {
                    None
                }
            } else {
                None
            };

        // If we have a class constructor override, use it instead of type inference
        if let Some(class_type_tokens) = class_constructor_override {
            // Skip the rest of the type inference logic for class constructors
            // The value_expr is already correct from to_rust_expr
            // Just set up the type annotation and mutability
            let _is_first =
                if let AssignTarget::Symbol(name) = target { !ctx.is_declared(name) } else { true };
            let is_mut = if let AssignTarget::Symbol(name) = target {
                ctx.mutable_vars.contains(name)
            } else {
                false
            };

            // Track the variable type for later method calls
            if let AssignTarget::Symbol(name) = target {
                if let HirExpr::Call { func, .. } = value {
                    ctx.var_types.insert(name.clone(), Type::Custom(func.clone()));
                }
            }

            let mutability = if is_mut {
                quote! { mut }
            } else {
                quote! {}
            };
            if let AssignTarget::Symbol(name) = target {
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                ctx.declare_var(name);
                return Ok(quote! {
                    let #mutability #ident #class_type_tokens = #value_expr;
                });
            }
        }

        // DEPYLER-1167: When type annotation is Set(UnificationVar) (unresolved type),
        // infer element type from the set() call's list argument to avoid HashSet<DepylerValue>
        // Pattern: `from_list = set([1, 2, 3])` with inferred Set(UnificationVar) type
        // → Infer HashSet<i32> from the list elements, not HashSet<DepylerValue>
        //
        // DEPYLER-1168: Same for List(UnificationVar) - avoid Vec<DepylerValue>
        // Pattern: `from_range = list(range(5))` → Infer Vec<i32>, not Vec<DepylerValue>
        // DEPYLER-1207: Pattern matching correction - use **elem to dereference Box
        let inferred_collection_type: Option<Type> = if let Type::Set(elem) = actual_type {
            if matches!(**elem, Type::UnificationVar(_) | Type::Unknown) {
                // Try to infer from the value expression
                if let HirExpr::Call { func, args, .. } = value {
                    if func == "set" && args.len() == 1 {
                        if let HirExpr::List(elems) = &args[0] {
                            let inferred = infer_collection_element_type(elems);
                            match inferred {
                                Type::Int | Type::Float | Type::String | Type::Bool => {
                                    Some(Type::Set(Box::new(inferred)))
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else if let Type::List(elem) = actual_type {
            // DEPYLER-1168: List with unresolved element type
            // DEPYLER-1207: Pattern matching correction - use **elem to dereference Box
            if matches!(**elem, Type::UnificationVar(_) | Type::Unknown) {
                if let HirExpr::Call { func, args, .. } = value {
                    if func == "list" && args.len() == 1 {
                        // Check if argument is range() - produces integers
                        if let HirExpr::Call { func: inner_func, .. } = &args[0] {
                            if inner_func == "range" {
                                Some(Type::List(Box::new(Type::Int)))
                            } else {
                                None
                            }
                        } else if let HirExpr::List(elems) = &args[0] {
                            // list([1, 2, 3]) - infer from elements
                            let inferred = infer_collection_element_type(elems);
                            match inferred {
                                Type::Int | Type::Float | Type::String | Type::Bool => {
                                    Some(Type::List(Box::new(inferred)))
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        let actual_type = inferred_collection_type.as_ref().unwrap_or(actual_type);

        // DEPYLER-0760: When value is None literal, wrap type annotation in Option<>
        // Pattern: `x: str = None` in Python → `let x: Option<String> = None;` in Rust
        // This ensures the type annotation matches the None value which requires Option<T>
        let is_none_value = matches!(value, HirExpr::Literal(Literal::None));
        let needs_option_wrap = is_none_value && !matches!(actual_type, Type::Optional(_));
        let target_rust_type = if needs_option_wrap {
            // Wrap the type in Option since the value is None
            crate::type_mapper::RustType::Option(Box::new(ctx.type_mapper.map_type(actual_type)))
        } else {
            ctx.type_mapper.map_type(actual_type)
        };
        let target_syn_type = rust_type_to_syn(&target_rust_type)?;

        // DEPYLER-0760: Update var_types with Optional type so subsequent usage is correct
        // This ensures is_none() and print() calls handle the variable correctly
        if needs_option_wrap {
            if let AssignTarget::Symbol(var_name) = target {
                ctx.var_types
                    .insert(var_name.clone(), Type::Optional(Box::new(actual_type.clone())));
            }
        }

        // DEPYLER-0272: Check if we need type conversion (e.g., usize to i32)
        // DEPYLER-0455 Bug 7: Also pass ctx for validator function detection
        // Pass the value expression to determine if cast is actually needed
        if needs_type_conversion(actual_type, value) {
            value_expr = apply_type_conversion(value_expr, actual_type);
        }

        // DEPYLER-1054: Assignment Coercion - extract concrete types from DepylerValue
        // When RHS is a DepylerValue expression and LHS has a concrete type annotation,
        // automatically inject .to_i64()/.to_f64()/.to_string()/.to_bool() extraction.
        // Example: x: int = data["count"] → let x: i32 = data["count"].to_i64() as i32;
        // DEPYLER-1064: Wrap in parentheses to ensure correct precedence for complex expressions
        // Example: x: int = count + 1 → let x: i32 = (count + 1).to_i64() as i32;
        //
        // DEPYLER-1321: Skip extraction for dict index access - dict access already generates
        // .into() which handles type conversion via From<DepylerValue> trait. Adding extra
        // .to_i64() or .to_string() after .into() breaks type inference (E0282).
        // The type annotation on the LHS (e.g., `let x: i32 = ...`) is sufficient for
        // Rust to infer the target type of .into().
        let is_dict_access = matches!(value, HirExpr::Index { .. }) && is_dict_index_access(value);

        // DEPYLER-E0599-PYOPS-FIX: Skip extraction for Binary expressions with primitive target
        // When target variable is a known primitive (i32, f64), the PyOps result is i64/f64, NOT DepylerValue.
        // Adding .to_i64() to an i64 result causes E0599: method not found.
        // Example: total = total + item where total: i32 → py_add returns i64, NOT DepylerValue
        // Check both type_annotation (for first assignment) and var_types (for reassignments).
        let is_binary_with_primitive_target = if let HirExpr::Binary { op, .. } = value {
            use crate::hir::BinOp;
            let is_arithmetic =
                matches!(op, BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod);
            if is_arithmetic {
                // Check if TARGET has a known primitive type
                // Priority: 1. type_annotation (current assignment), 2. var_types (prior declaration)
                // Note: type_annotation is &Option<Type>, need to handle reference correctly
                // CRITICAL: Must dereference t with *t since get() returns Option<&Type>
                let from_annotation =
                    type_annotation.as_ref().is_some_and(|t| matches!(*t, Type::Int | Type::Float));
                let from_var_types = if let AssignTarget::Symbol(name) = target {
                    ctx.var_types.get(name).is_some_and(|t| matches!(*t, Type::Int | Type::Float))
                } else {
                    false
                };
                from_annotation || from_var_types
            } else {
                false
            }
        } else {
            false
        };

        if expr_produces_depyler_value(value, ctx)
            && !is_dict_access
            && !is_binary_with_primitive_target
        {
            if let Some(extraction) = get_depyler_extraction_for_type(actual_type) {
                let extraction_tokens: proc_macro2::TokenStream = extraction.parse().unwrap();
                value_expr = parse_quote! { (#value_expr) #extraction_tokens };
            }
        }

        // DEPYLER-0380 #1: String literal to String conversion
        // When assigning a string literal to a String typed variable, add .to_string()
        // Example: `let version: String = "Python 3.x"` should become
        //          `let version: String = "Python 3.x".to_string()`
        if matches!(value, HirExpr::Literal(Literal::String(_)))
            && matches!(target_rust_type, crate::type_mapper::RustType::String)
        {
            value_expr = parse_quote! { #value_expr.to_string() };
        }

        (Some(quote! { : #target_syn_type }), is_const)
    } else {
        // DEPYLER-0952: When value is bare None literal without type annotation,
        // Rust needs a type annotation because it can't infer Option<_>.
        // Python: `x = None` → Rust: `let x: Option<()> = None;`
        if matches!(value, HirExpr::Literal(Literal::None)) {
            (Some(quote! { : Option<()> }), false)
        } else if ctx.type_mapper.nasa_mode {
            // DEPYLER-1174: Aggressive Defaulting - force DepylerValue for ambiguous types
            // Rationale: E0282 (type annotation needed) stops compilation immediately.
            // E0308 (type mismatch) from DepylerValue can be fixed later.
            // It's better to compile with potential type mismatches than not compile at all.
            match value {
                // Empty dict literal: `x = {}` - DEPYLER-1218 Smart type inference
                // Priority: 1. Variable's known type (for params/prior assignments)
                //           2. Function return type (if Dict)
                //           3. Default to HashMap<String, DepylerValue>
                HirExpr::Dict(entries) if entries.is_empty() => {
                    // DEPYLER-1314: Helper to check if dict type has Unknown key/val types
                    let has_unknown_types = |k: &Type, v: &Type| {
                        matches!(k, Type::Unknown | Type::UnificationVar(_))
                            || matches!(v, Type::Unknown | Type::UnificationVar(_))
                    };
                    // First check if var_types has a known type for this variable
                    let var_dict_type = if let AssignTarget::Symbol(name) = target {
                        if let Some(Type::Dict(key, val)) = ctx.var_types.get(name) {
                            // DEPYLER-1314: Skip type annotation if key/val are Unknown
                            // Let Rust infer from the expression to avoid HashMap<DepylerValue, DepylerValue>
                            // conflicting with generated HashMap<String, DepylerValue>
                            if has_unknown_types(key, val) {
                                None // Fall through to default
                            } else {
                                let key_ty = hir_type_to_tokens(key);
                                let val_ty = hir_type_to_tokens(val);
                                Some(quote! { : std::collections::HashMap<#key_ty, #val_ty> })
                            }
                        } else if let Some(Type::Optional(inner)) = ctx.var_types.get(name) {
                            // Handle Option<Dict> case (like memo: Dict[int, int] = None)
                            if let Type::Dict(key, val) = inner.as_ref() {
                                if has_unknown_types(key, val) {
                                    None
                                } else {
                                    let key_ty = hir_type_to_tokens(key);
                                    let val_ty = hir_type_to_tokens(val);
                                    Some(quote! { : std::collections::HashMap<#key_ty, #val_ty> })
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    // Then check current_return_type for Dict type info
                    // DEPYLER-1314: Skip if return type has Unknown key/val
                    let return_dict_type = match &ctx.current_return_type {
                        Some(Type::Dict(key, val)) if !has_unknown_types(key, val) => {
                            let key_ty = hir_type_to_tokens(key);
                            let val_ty = hir_type_to_tokens(val);
                            Some(quote! { : std::collections::HashMap<#key_ty, #val_ty> })
                        }
                        _ => None,
                    };
                    if let Some(ty) = var_dict_type.or(return_dict_type) {
                        (Some(ty), false)
                    } else {
                        (Some(quote! { : std::collections::HashMap<String, DepylerValue> }), false)
                    }
                }
                // Dict literal with entries: infer from usage or default to DepylerValue values
                HirExpr::Dict(_) => {
                    // DEPYLER-1314: Helper for unknown type check
                    let has_unknown = |k: &Type, v: &Type| {
                        matches!(k, Type::Unknown | Type::UnificationVar(_))
                            || matches!(v, Type::Unknown | Type::UnificationVar(_))
                    };
                    // First check if var_types has a known type for this variable
                    let var_dict_type = if let AssignTarget::Symbol(name) = target {
                        if let Some(Type::Dict(key, val)) = ctx.var_types.get(name) {
                            if has_unknown(key, val) {
                                None
                            } else {
                                let key_ty = hir_type_to_tokens(key);
                                let val_ty = hir_type_to_tokens(val);
                                Some(quote! { : std::collections::HashMap<#key_ty, #val_ty> })
                            }
                        } else if let Some(Type::Optional(inner)) = ctx.var_types.get(name) {
                            if let Type::Dict(key, val) = inner.as_ref() {
                                if has_unknown(key, val) {
                                    None
                                } else {
                                    let key_ty = hir_type_to_tokens(key);
                                    let val_ty = hir_type_to_tokens(val);
                                    Some(quote! { : std::collections::HashMap<#key_ty, #val_ty> })
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let return_dict_type = match &ctx.current_return_type {
                        Some(Type::Dict(key, val)) if !has_unknown(key, val) => {
                            let key_ty = hir_type_to_tokens(key);
                            let val_ty = hir_type_to_tokens(val);
                            Some(quote! { : std::collections::HashMap<#key_ty, #val_ty> })
                        }
                        _ => None,
                    };
                    if let Some(ty) = var_dict_type.or(return_dict_type) {
                        (Some(ty), false)
                    } else {
                        (Some(quote! { : std::collections::HashMap<String, DepylerValue> }), false)
                    }
                }
                // Dict comprehension: `x = {k: v for ...}` → infer from return type or default
                HirExpr::DictComp { .. } => {
                    let var_dict_type = if let AssignTarget::Symbol(name) = target {
                        if let Some(Type::Dict(key, val)) = ctx.var_types.get(name) {
                            let key_ty = hir_type_to_tokens(key);
                            let val_ty = hir_type_to_tokens(val);
                            Some(quote! { : std::collections::HashMap<#key_ty, #val_ty> })
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let return_dict_type = match &ctx.current_return_type {
                        Some(Type::Dict(key, val)) => {
                            let key_ty = hir_type_to_tokens(key);
                            let val_ty = hir_type_to_tokens(val);
                            Some(quote! { : std::collections::HashMap<#key_ty, #val_ty> })
                        }
                        _ => None,
                    };
                    if let Some(ty) = var_dict_type.or(return_dict_type) {
                        (Some(ty), false)
                    } else {
                        (Some(quote! { : std::collections::HashMap<String, DepylerValue> }), false)
                    }
                }
                // Empty list literal: `x = []` - DEPYLER-1206 Smart type inference
                // Priority: 1. Variable's known type from var_types
                //           2. Function return type (if List)
                //           3. Default to Vec<DepylerValue>
                HirExpr::List(elems) if elems.is_empty() => {
                    // First check if var_types has a known type for this variable
                    let var_type = if let AssignTarget::Symbol(name) = target {
                        ctx.var_types.get(name).cloned()
                    } else {
                        None
                    };

                    // Then check current_return_type for List element type
                    let return_list_type = match &ctx.current_return_type {
                        Some(Type::List(elem)) => Some(elem.as_ref().clone()),
                        Some(Type::Tuple(elems)) => {
                            // For tuple returns, check if any element is a list we might be assigning
                            elems.iter().find_map(|t| {
                                if let Type::List(elem) = t {
                                    Some(elem.as_ref().clone())
                                } else {
                                    None
                                }
                            })
                        }
                        _ => None,
                    };

                    // Use var_type if it's a List, else return_list_type, else default
                    let elem_type = if let Some(Type::List(elem)) = var_type {
                        Some(elem.as_ref().clone())
                    } else {
                        return_list_type
                    };

                    match elem_type {
                        Some(Type::Int) => (Some(quote! { : Vec<i32> }), false),
                        Some(Type::Float) => (Some(quote! { : Vec<f64> }), false),
                        Some(Type::String) => (Some(quote! { : Vec<String> }), false),
                        Some(Type::Bool) => (Some(quote! { : Vec<bool> }), false),
                        Some(Type::Tuple(types)) => {
                            // For List[Tuple[...]], generate Vec<(T1, T2, ...)>
                            let tuple_types: Vec<proc_macro2::TokenStream> = types
                                .iter()
                                .map(|t| match t {
                                    Type::String => quote! { String },
                                    Type::Int => quote! { i32 },
                                    Type::Float => quote! { f64 },
                                    Type::Bool => quote! { bool },
                                    Type::Custom(name) => {
                                        let ident =
                                            syn::Ident::new(name, proc_macro2::Span::call_site());
                                        quote! { #ident }
                                    }
                                    _ => quote! { DepylerValue },
                                })
                                .collect();
                            (Some(quote! { : Vec<(#(#tuple_types),*)> }), false)
                        }
                        Some(Type::Custom(name)) => {
                            // For List[CustomType], generate Vec<CustomType>
                            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
                            (Some(quote! { : Vec<#ident> }), false)
                        }
                        _ => (Some(quote! { : Vec<DepylerValue> }), false),
                    }
                }
                // DEPYLER-0467: Non-empty list literal with type inference
                // `x = [a, b, c]` where a,b,c are i32 → `Vec<i32>` (NOT Vec<DepylerValue>)
                // This resolves the type inference issue for variable-containing lists
                // DEPYLER-1313: Extended to handle nested lists (Vec<Vec<T>>)
                HirExpr::List(elems) => {
                    let elem_type = infer_collection_element_type_with_ctx(elems, &ctx.var_types);
                    match &elem_type {
                        Type::Int => (Some(quote! { : Vec<i32> }), false),
                        Type::Float => (Some(quote! { : Vec<f64> }), false),
                        Type::String => (Some(quote! { : Vec<String> }), false),
                        Type::Bool => (Some(quote! { : Vec<bool> }), false),
                        // DEPYLER-1313: Handle nested lists - generate Vec<Vec<T>>
                        Type::List(inner) => {
                            if let Some(inner_tokens) = type_to_vec_annotation(inner) {
                                (Some(quote! { : Vec<#inner_tokens> }), false)
                            } else {
                                (Some(quote! { : Vec<DepylerValue> }), false)
                            }
                        }
                        // DEPYLER-1313: Handle tuples in lists - generate Vec<(T1, T2, ...)>
                        Type::Tuple(types) => {
                            let tuple_types: Vec<proc_macro2::TokenStream> =
                                types.iter().map(type_to_simple_token).collect();
                            (Some(quote! { : Vec<(#(#tuple_types),*)> }), false)
                        }
                        _ => {
                            // Unknown or mixed types - fall back to DepylerValue
                            (Some(quote! { : Vec<DepylerValue> }), false)
                        }
                    }
                }
                // List comprehension: `x = [v for ...]` → infer from iterable type if possible
                HirExpr::ListComp { generators, .. } => {
                    // DEPYLER-1176: Smart list comprehension typing
                    // Try to infer element type from the iterable being iterated over
                    // If iter is a Var with known List[T] type, result is Vec<T>
                    // Also handles slices: `arr[1:]` preserves element type
                    let inferred_type = generators.first().and_then(|gen| {
                        // Helper to get element type from a list type
                        let get_list_elem_type = |var_name: &str| {
                            ctx.var_types.get(var_name).and_then(|t| {
                                if let Type::List(elem_ty) = t {
                                    Some(elem_ty.as_ref().clone())
                                } else {
                                    None
                                }
                            })
                        };

                        match gen.iter.as_ref() {
                            // Direct variable: `for x in arr`
                            HirExpr::Var(iter_name) => get_list_elem_type(iter_name),
                            // Slice: `for x in arr[1:]` - base var determines type
                            HirExpr::Slice { base, .. } => {
                                if let HirExpr::Var(base_name) = base.as_ref() {
                                    get_list_elem_type(base_name)
                                } else {
                                    None
                                }
                            }
                            // Index: `for x in arr[0]` (nested list)
                            HirExpr::Index { base, .. } => {
                                if let HirExpr::Var(base_name) = base.as_ref() {
                                    // If base is List[List[T]], indexing gives List[T], so element is T
                                    ctx.var_types.get(base_name).and_then(|t| {
                                        if let Type::List(inner) = t {
                                            if let Type::List(elem_ty) = inner.as_ref() {
                                                Some(elem_ty.as_ref().clone())
                                            } else {
                                                // Base is List[T], indexing gives T (not a list)
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    })
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    });

                    if let Some(elem_type) = inferred_type {
                        // Inferred element type - use it
                        let rust_type = ctx.type_mapper.map_type(&elem_type);
                        match rust_type_to_syn(&rust_type) {
                            Ok(syn_type) => (Some(quote! { : Vec<#syn_type> }), false),
                            Err(_) => (Some(quote! { : Vec<DepylerValue> }), false),
                        }
                    } else {
                        // Couldn't infer - fall back to DepylerValue
                        (Some(quote! { : Vec<DepylerValue> }), false)
                    }
                }
                // DEPYLER-1149: Set literal with element type inference
                // `x = {1, 2, 3}` → `HashSet<i32>` if all ints
                // `x = {"a", "b"}` → `HashSet<String>` if all strings
                // `x = {1, "a"}` → `HashSet<DepylerValue>` if mixed
                // DEPYLER-0467: Also infers type from variables (e.g., `{a, b, c}` where a,b,c are i32)
                HirExpr::Set(elems) => {
                    let elem_type = infer_collection_element_type_with_ctx(elems, &ctx.var_types);
                    match elem_type {
                        Type::Int => (Some(quote! { : std::collections::HashSet<i32> }), false),
                        Type::Float => (Some(quote! { : std::collections::HashSet<f64> }), false),
                        Type::String => {
                            (Some(quote! { : std::collections::HashSet<String> }), false)
                        }
                        Type::Bool => (Some(quote! { : std::collections::HashSet<bool> }), false),
                        _ => {
                            // Unknown or mixed types - fall back to DepylerValue
                            (Some(quote! { : std::collections::HashSet<DepylerValue> }), false)
                        }
                    }
                }
                // Set comprehension: `x = {v for ...}` → `HashSet<DepylerValue>`
                HirExpr::SetComp { .. } => {
                    (Some(quote! { : std::collections::HashSet<DepylerValue> }), false)
                }
                // Generator expression assigned to variable: force Vec<DepylerValue>
                HirExpr::GeneratorExp { .. } => (Some(quote! { : Vec<DepylerValue> }), false),
                // DEPYLER-1167: set() call with list argument - infer element type from list
                // `x = set([1, 2, 3])` → `HashSet<i32>` if all ints (NOT HashSet<DepylerValue>)
                // This prevents E0308 type mismatch between HashSet<DepylerValue> and HashSet<{integer}>
                // DEPYLER-0467: Also supports variables in the list
                HirExpr::Call { func, args, .. } if func == "set" => {
                    // If set() has a list argument, infer element type from the list
                    if args.len() == 1 {
                        if let HirExpr::List(elems) = &args[0] {
                            let elem_type =
                                infer_collection_element_type_with_ctx(elems, &ctx.var_types);
                            match elem_type {
                                Type::Int => {
                                    (Some(quote! { : std::collections::HashSet<i32> }), false)
                                }
                                Type::Float => {
                                    (Some(quote! { : std::collections::HashSet<f64> }), false)
                                }
                                Type::String => {
                                    (Some(quote! { : std::collections::HashSet<String> }), false)
                                }
                                Type::Bool => {
                                    (Some(quote! { : std::collections::HashSet<bool> }), false)
                                }
                                _ => {
                                    // Mixed or unknown types - fall back to DepylerValue
                                    (
                                        Some(quote! { : std::collections::HashSet<DepylerValue> }),
                                        false,
                                    )
                                }
                            }
                        } else {
                            // set() with non-list argument - let Rust infer
                            (None, false)
                        }
                    } else {
                        // Empty set() - use i32 as default (DEPYLER-0409)
                        (Some(quote! { : std::collections::HashSet<i32> }), false)
                    }
                }
                // DEPYLER-1168: list() call with range argument - infer element type
                // `x = list(range(5))` → `Vec<i32>` (NOT Vec<DepylerValue>)
                // DEPYLER-0467: Also supports variables in the list
                HirExpr::Call { func, args, .. } if func == "list" => {
                    if args.len() == 1 {
                        // Check if argument is range()
                        if let HirExpr::Call { func: inner_func, .. } = &args[0] {
                            if inner_func == "range" {
                                // range() produces integers
                                (Some(quote! { : Vec<i32> }), false)
                            } else {
                                // Other function call - let Rust infer
                                (None, false)
                            }
                        } else if let HirExpr::List(elems) = &args[0] {
                            // list([1, 2, 3]) - infer from elements
                            let elem_type =
                                infer_collection_element_type_with_ctx(elems, &ctx.var_types);
                            match elem_type {
                                Type::Int => (Some(quote! { : Vec<i32> }), false),
                                Type::Float => (Some(quote! { : Vec<f64> }), false),
                                Type::String => (Some(quote! { : Vec<String> }), false),
                                Type::Bool => (Some(quote! { : Vec<bool> }), false),
                                _ => (Some(quote! { : Vec<DepylerValue> }), false),
                            }
                        } else {
                            // list() with other argument - let Rust infer
                            (None, false)
                        }
                    } else {
                        // Empty list() - use default
                        (Some(quote! { : Vec<i32> }), false)
                    }
                }
                // DEPYLER-CI-FIX: User-defined class constructor calls
                // `calc = Calculator(10)` → `let calc: Calculator = Calculator::new(10);`
                // This prevents E0308 type mismatch when class instance is typed as HashSet<DepylerValue>
                HirExpr::Call { func, .. } if ctx.class_names.contains(func) => {
                    let class_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
                    (Some(quote! { : #class_ident }), false)
                }
                // Default: let Rust infer if possible
                _ => (None, false),
            }
        } else {
            (None, false)
        }
    };

    // DEPYLER-0455 #2: String literal normalization for hoisted inference variables
    // When a variable is hoisted without type annotation, string literals must be
    // normalized to String to ensure consistent type inference across if/else branches
    // Example: let mut format;
    //          if x { format = "json"; }  // &str
    //          else { format = s.to_lowercase(); }  // String - TYPE MISMATCH!
    // Solution: Convert all string literals to String: format = "json".to_string();
    if let AssignTarget::Symbol(var_name) = target {
        if ctx.hoisted_inference_vars.contains(var_name)
            && matches!(value, HirExpr::Literal(Literal::String(_)))
        {
            value_expr = parse_quote! { #value_expr.to_string() };
        }
    }

    // DEPYLER-0598: String literal normalization for mutable variables
    // When a mutable variable is first assigned a string literal (no type annotation),
    // convert to .to_string() to avoid &str vs String type mismatch on reassignment
    // Example: let mut result = "hello";  // &str
    //          result = format!(...);     // String - TYPE MISMATCH!
    // Solution: let mut result = "hello".to_string();
    if let AssignTarget::Symbol(var_name) = target {
        let is_first_assignment = !ctx.is_declared(var_name);
        let is_mutable = ctx.mutable_vars.contains(var_name);
        let no_type_annotation = type_annotation.is_none();
        let is_string_literal = matches!(value, HirExpr::Literal(Literal::String(_)));

        if is_first_assignment && is_mutable && no_type_annotation && is_string_literal {
            value_expr = parse_quote! { #value_expr.to_string() };
        }
    }

    // DEPYLER-0625: Wrap File/Stdout values in Box::new() for trait object unification
    // When a variable is declared as Box<dyn Write> (heterogeneous IO types),
    // wrap the assigned value with Box::new() to satisfy the type
    if let AssignTarget::Symbol(var_name) = target {
        if ctx.boxed_dyn_write_vars.contains(var_name) {
            value_expr = parse_quote! { Box::new(#value_expr) };
        }
    }

    // DEPYLER-1027/DEPYLER-1029: Handle dict value assignment to dict in NASA mode
    // Only wrap dict values in format!() if the target dict expects String values
    // If target is Dict[str, Dict[...]], keep dict values as HashMap
    let value_expr = if ctx.type_mapper.nasa_mode {
        if let AssignTarget::Index { base, .. } = target {
            if matches!(value, HirExpr::Dict(_)) {
                // DEPYLER-1027/1034/1042: Handle dict value assignment in NASA mode
                // Only DON'T wrap if we KNOW the outer dict expects Dict values (not String)
                // DEPYLER-1042: Use get_subscript_value_type for nested subscripts
                // DEPYLER-1314: Also skip wrapping for DepylerValue targets (handled later)
                let skip_format_wrap = {
                    // Get the value type from the subscript chain
                    if let Some(val_type) = get_subscript_value_type(base.as_ref(), ctx) {
                        // Value type is Dict or Unknown/DepylerValue → skip format wrapping
                        // DepylerValue dicts handle nested dicts via DepylerValue::Dict later
                        matches!(
                            val_type,
                            Type::Dict(_, _) | Type::Unknown | Type::UnificationVar(_)
                        )
                    } else if let HirExpr::Var(base_name) = &**base {
                        // Fallback: direct Var check for simple cases
                        ctx.var_types.get(base_name).is_some_and(|t| {
                            match t {
                                // Dict[K, Dict[...]] - skip wrapping
                                Type::Dict(_, val_type)
                                    if matches!(val_type.as_ref(), Type::Dict(_, _)) =>
                                {
                                    true
                                }
                                // Dict[K, Unknown] or Dict[K, DepylerValue-like] - skip wrapping
                                Type::Dict(_, val_type)
                                    if matches!(
                                        val_type.as_ref(),
                                        Type::Unknown | Type::UnificationVar(_)
                                    ) =>
                                {
                                    true
                                }
                                _ => false,
                            }
                        })
                    } else {
                        false
                    }
                };

                if skip_format_wrap {
                    // Keep as HashMap - later code handles DepylerValue::Dict wrapping
                    value_expr
                } else {
                    // Default: wrap in format! for HashMap<String, String> compatibility
                    parse_quote! { format!("{:?}", #value_expr) }
                }
            } else {
                value_expr
            }
        } else {
            value_expr
        }
    } else {
        value_expr
    };

    // DEPYLER-E0308-PYOPS-FIX: Cast PyOps results (i64) to i32 when target variable is i32
    // PyOps traits (py_add, py_sub, py_mul, py_div, py_mod) return i64 for overflow safety
    // But when the target variable is typed as i32, we need to cast
    // Pattern: `total: i32 = 0; total = (total).py_add(item)` → add `as i32`
    // DEPYLER-CI-FIX: Skip cast for string concatenation (string + string should not cast to i32)
    let value_expr = if ctx.type_mapper.nasa_mode {
        let target_is_i32 = match target {
            AssignTarget::Symbol(name) => {
                // Check explicit type annotation first, then check var_types for declared variables
                matches!(type_annotation, Some(Type::Int))
                    || matches!(ctx.var_types.get(name.as_str()), Some(Type::Int))
            }
            _ => false,
        };

        // DEPYLER-CI-FIX: Check if target is a string type - never cast strings to i32
        let target_is_string = match target {
            AssignTarget::Symbol(name) => {
                matches!(type_annotation, Some(Type::String))
                    || matches!(ctx.var_types.get(name.as_str()), Some(Type::String))
            }
            _ => false,
        };

        // Check if value is a binary arithmetic expression (generates PyOps)
        let is_pyops = matches!(
            value,
            HirExpr::Binary {
                op: BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod,
                ..
            }
        );

        // Only cast to i32 if target is i32 AND value is pyops AND target is NOT a string
        if target_is_i32 && is_pyops && !target_is_string {
            parse_quote! { (#value_expr) as i32 }
        } else {
            value_expr
        }
    } else {
        value_expr
    };

    match target {
        AssignTarget::Symbol(symbol) => {
            codegen_assign_symbol(symbol, value_expr, type_annotation_tokens, is_final, ctx)
        }
        // DEPYLER-1203: Pass original HirExpr for type-based DepylerValue wrapping
        AssignTarget::Index { base, index } => {
            codegen_assign_index(base, index, value_expr, value, ctx)
        }
        AssignTarget::Attribute { value, attr } => {
            codegen_assign_attribute(value, attr, value_expr, ctx)
        }
        AssignTarget::Tuple(targets) => {
            codegen_assign_tuple(targets, value, value_expr, type_annotation_tokens, ctx)
        }
    }
}

pub(crate) fn codegen_assign_attribute(
    base: &HirExpr,
    attr: &str,
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let base_expr = base.to_rust_expr(ctx)?;
    let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
    Ok(quote! { #base_expr.#attr_ident = #value_expr; })
}
