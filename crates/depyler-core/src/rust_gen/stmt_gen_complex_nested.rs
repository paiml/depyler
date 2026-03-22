fn codegen_nested_function_def(
    name: &str,
    params: &[HirParam],
    ret_type: &Type,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    use quote::quote;

    // DEPYLER-0842: Generate function name with keyword escaping
    // Previously used syn::Ident::new() which doesn't escape keywords.
    // If Python has `def const(...)`, this creates `let const = ...` which fails
    // because `const` is a Rust keyword. Using safe_ident produces `let r#const = ...`
    let fn_name = safe_ident(name);

    // GH-70: Use inferred parameters from context if available
    // DEPYLER-0687: Clone params to avoid borrow conflicts with ctx.declare_var
    let effective_params: Vec<HirParam> =
        ctx.nested_function_params.get(name).cloned().unwrap_or_else(|| params.to_vec());

    // GH-70: Populate ctx.var_types with inferred param types so that
    // expressions in body (like item[0]) can use proper type info
    // to decide between tuple syntax (.0) and array syntax ([0])
    for param in &effective_params {
        ctx.var_types.insert(param.name.clone(), param.ty.clone());
    }

    // Generate parameters
    // DEPYLER-0550: For collection types (Dict, List), use references
    // This is more idiomatic in Rust and works correctly with filter() closures
    // DEPYLER-0769: Use safe_ident to escape Rust keywords in closure parameters
    let param_tokens: Vec<proc_macro2::TokenStream> = effective_params
        .iter()
        .map(|p| {
            let param_name = safe_ident(&p.name);
            let param_type = hir_type_to_tokens(&p.ty);

            // For collection types and strings, take by reference for idiomatic Rust
            // This is necessary for closures used with filter() which provides &T
            // DEPYLER-0774: Also use &str for String params in nested functions
            // Parent functions receive &str but nested func expects String → use &str
            if matches!(p.ty, Type::Dict(_, _) | Type::List(_) | Type::Set(_)) {
                quote! { #param_name: &#param_type }
            } else if matches!(p.ty, Type::String) {
                // DEPYLER-0774: Use &str instead of String for nested function params
                // This matches how parent functions receive string args (&str)
                quote! { #param_name: &str }
            } else {
                quote! { #param_name: #param_type }
            }
        })
        .collect();

    // Generate return type
    let return_type = hir_type_to_tokens(ret_type);

    // DEPYLER-0550: Save and restore can_fail flag for nested closures
    // Nested closures should NOT inherit can_fail from parent function
    // Otherwise return statements get incorrectly wrapped in Ok()
    let saved_can_fail = ctx.current_function_can_fail;
    ctx.current_function_can_fail = false;

    // DEPYLER-0731: Save and restore return type for nested functions
    // Without this, nested function body uses outer function's return type,
    // causing `return x * 2` to become `return;` when outer returns None
    let saved_return_type = ctx.current_return_type.take();
    ctx.current_return_type = Some(ret_type.clone());

    // DEPYLER-0731: Save and restore is_main_function for nested functions
    // Without this, nested function inside main() would trigger main-specific
    // return handling (DEPYLER-0617) that discards the return value
    let saved_is_main = ctx.is_main_function;
    ctx.is_main_function = false;

    // DEPYLER-0687: Enter new scope for nested function body
    // This isolates variable declarations so they don't leak between closures.
    // Without this, a variable like `result` declared in one closure would be
    // considered "already declared" in sibling closures, causing E0425 errors.
    ctx.enter_scope();

    // Declare parameters in this scope
    for param in &effective_params {
        ctx.declare_var(&param.name);
    }

    // DEPYLER-0766: Analyze mutability for nested function body
    // Without this, variables reassigned inside closures don't get `let mut`,
    // causing E0384 "cannot assign twice to immutable variable" errors.
    // DEPYLER-99MODE-S9: Save outer function's mutable_vars before analyzing nested function.
    // analyze_mutable_vars clears ctx.mutable_vars, which would wipe the outer function's
    // analysis. Variables declared AFTER nested functions would lose their `mut`.
    let saved_mutable_vars = ctx.mutable_vars.clone();
    crate::rust_gen::analyze_mutable_vars(body, ctx, &effective_params);

    // DEPYLER-99MODE-S9: Detect FnMut closures — closures that mutate captured variables.
    // After analyze_mutable_vars, ctx.mutable_vars contains variables mutated in the closure.
    // If any of these are NOT closure params/locals, they are captured mutations → FnMut.
    let closure_param_names: std::collections::HashSet<String> =
        effective_params.iter().map(|p| p.name.clone()).collect();
    let closure_is_fn_mut = ctx.mutable_vars.iter().any(|var| {
        !closure_param_names.contains(var)
            && !body.iter().any(|stmt| {
                // Check if var is locally declared in the closure body
                matches!(stmt, HirStmt::Assign { target: AssignTarget::Symbol(n), .. } if n == var)
            })
    });

    // DEPYLER-1160: Propagate return type annotation to returned variables
    // This enables target-typed inference for empty lists in nested functions
    // e.g., `result = []` gets type Vec<i32> when function returns list[int]
    propagate_return_type_to_vars(body, &mut ctx.var_types, ret_type);

    // Generate body
    let body_tokens: Vec<proc_macro2::TokenStream> =
        body.iter().map(|stmt| stmt.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;

    // Exit scope before restoring context
    ctx.exit_scope();

    // DEPYLER-99MODE-S9: Restore outer function's mutable_vars after nested function codegen.
    // The nested function's analyze_mutable_vars cleared and repopulated mutable_vars for
    // its own body. We must restore the outer function's analysis so that variables
    // declared after nested functions correctly get `let mut`.
    ctx.mutable_vars = saved_mutable_vars;

    // Restore can_fail flag
    ctx.current_function_can_fail = saved_can_fail;

    // DEPYLER-0731: Restore outer function's return type and is_main flag
    ctx.current_return_type = saved_return_type;
    ctx.is_main_function = saved_is_main;

    // DEPYLER-0790: Check if this nested function is recursive (calls itself)
    // Recursive functions cannot be closures because closures can't reference themselves
    // For recursive nested functions that DON'T capture outer variables,
    // generate as `fn name(...)` instead of closure
    let is_recursive = is_nested_function_recursive(name, body);

    // Get outer scope variables to check for captures
    // Collect all declared vars from all scopes
    let outer_vars: std::collections::HashSet<String> =
        ctx.declared_vars.iter().flat_map(|scope| scope.iter().cloned()).collect();
    let has_captures = captures_outer_scope(params, body, &outer_vars);

    // GH-70 FIX: Generate as closure instead of fn item
    // Closures can be returned as values and have better type inference
    // This fixes the issue where nested functions had all types defaulting to ()
    //
    // GH-70 CRITICAL FIX: Omit return type annotation for Type::Unknown
    // When no type annotation exists in Python, ret_type is Type::Unknown.
    // Previously, this defaulted to () in hir_type_to_tokens, causing explicit
    // `-> ()` in closure definition which conflicted with actual return values.
    // Solution: Omit return type entirely, allowing Rust's type inference to
    // determine correct return type from closure body.
    //
    // DEPYLER-0613: Support hoisting - if variable is already declared, use assignment
    let is_declared = ctx.is_declared(name);

    // DEPYLER-0790: For recursive functions that DON'T capture outer variables,
    // generate as proper fn instead of closure
    // If recursive AND captures, we can't easily fix - keep as closure (will produce E0425)
    if is_recursive && !has_captures {
        // Declare the function name in context so sibling nested functions can detect it
        ctx.declare_var(name);
        return Ok(if matches!(ret_type, Type::Unknown) {
            quote! {
                fn #fn_name(#(#param_tokens),*) {
                    #(#body_tokens)*
                }
            }
        } else {
            quote! {
                fn #fn_name(#(#param_tokens),*) -> #return_type {
                    #(#body_tokens)*
                }
            }
        });
    }

    // Declare the function name in context so sibling nested functions can detect it as a capture
    ctx.declare_var(name);

    // DEPYLER-0783: Always use `move` for nested function closures
    // This is required when:
    // 1. The closure captures variables from outer scope AND is returned (E0373)
    // 2. For Copy types (i32, bool), move just copies - no harm
    // 3. For non-Copy types, move is required if captured and returned
    // Using `move` universally is safe and fixes the common closure capture issue
    //
    // DEPYLER-99MODE-S9: Use `let mut` for FnMut closures (those that mutate captured variables)
    let mutability = if closure_is_fn_mut {
        quote! { mut }
    } else {
        quote! {}
    };
    Ok(if matches!(ret_type, Type::Unknown) {
        if is_declared {
            quote! {
                #fn_name = move |#(#param_tokens),*| {
                    #(#body_tokens)*
                };
            }
        } else {
            quote! {
                let #mutability #fn_name = move |#(#param_tokens),*| {
                    #(#body_tokens)*
                };
            }
        }
    } else if is_declared {
        quote! {
            #fn_name = move |#(#param_tokens),*| -> #return_type {
                #(#body_tokens)*
            };
        }
    } else {
        quote! {
            let #mutability #fn_name = move |#(#param_tokens),*| -> #return_type {
                #(#body_tokens)*
            };
        }
    })
}

pub(crate) fn captures_outer_scope(
    params: &[crate::hir::HirParam],
    body: &[HirStmt],
    outer_vars: &std::collections::HashSet<String>,
) -> bool {
    use crate::hir::HirExpr;

    // Collect parameter names
    let mut local_vars: std::collections::HashSet<&str> =
        params.iter().map(|p| p.name.as_str()).collect();

    // Collect locally defined variables from assignments
    fn collect_local_vars<'a>(stmt: &'a HirStmt, locals: &mut std::collections::HashSet<&'a str>) {
        match stmt {
            HirStmt::Assign { target: crate::hir::AssignTarget::Symbol(name), .. } => {
                locals.insert(name.as_str());
            }
            HirStmt::Assign { .. } => {}
            HirStmt::For { target, body, .. } => {
                if let crate::hir::AssignTarget::Symbol(name) = target {
                    locals.insert(name.as_str());
                }
                for s in body {
                    collect_local_vars(s, locals);
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                for s in then_body {
                    collect_local_vars(s, locals);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        collect_local_vars(s, locals);
                    }
                }
            }
            HirStmt::While { body, .. } => {
                for s in body {
                    collect_local_vars(s, locals);
                }
            }
            HirStmt::With { body, target, .. } => {
                if let Some(t) = target {
                    locals.insert(t.as_str());
                }
                for s in body {
                    collect_local_vars(s, locals);
                }
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                for s in body {
                    collect_local_vars(s, locals);
                }
                for h in handlers {
                    if let Some(name) = &h.name {
                        locals.insert(name.as_str());
                    }
                    for s in &h.body {
                        collect_local_vars(s, locals);
                    }
                }
                if let Some(els) = orelse {
                    for s in els {
                        collect_local_vars(s, locals);
                    }
                }
                if let Some(fin) = finalbody {
                    for s in fin {
                        collect_local_vars(s, locals);
                    }
                }
            }
            HirStmt::FunctionDef { name, .. } => {
                locals.insert(name.as_str());
            }
            HirStmt::Block(stmts) => {
                for s in stmts {
                    collect_local_vars(s, locals);
                }
            }
            _ => {}
        }
    }

    // First pass: collect all locally defined variables
    for stmt in body {
        collect_local_vars(stmt, &mut local_vars);
    }

    body.iter().any(|stmt| check_stmt_for_capture(stmt, &local_vars, outer_vars))
}

    fn transpile(python_code: &str) -> String {
        use crate::ast_bridge::AstBridge;
        use crate::rust_gen::generate_rust_file;
        use crate::type_mapper::TypeMapper;
        use rustpython_parser::{parse, Mode};

        let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
        let (module, _) =
            AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
        let tm = TypeMapper::default();
        let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
        result
    }
