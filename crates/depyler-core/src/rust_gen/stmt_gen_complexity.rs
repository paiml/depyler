fn generate_hoisted_decls(
    hoisted_vars: &std::collections::HashSet<String>,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut hoisted_decls = Vec::new();
    for var_name in hoisted_vars {
        if ctx.is_declared(var_name) {
            continue;
        }

        let var_ident = safe_ident(var_name);

        // DEPYLER-0625: Check if variable needs Box<dyn Write>
        if let Some(else_stmts) = else_body {
            if needs_boxed_dyn_write(var_name, then_body, else_stmts) {
                hoisted_decls.push(quote! { let mut #var_ident: Box<dyn std::io::Write>; });
                ctx.boxed_dyn_write_vars.insert(var_name.clone());
                ctx.declare_var(var_name);
                continue;
            }
        }

        let var_type = find_variable_type(var_name, then_body).or_else(|| {
            if let Some(else_stmts) = else_body {
                find_variable_type(var_name, else_stmts)
            } else {
                None
            }
        });

        generate_single_hoisted_decl(
            var_name,
            &var_ident,
            var_type,
            else_body,
            ctx,
            &mut hoisted_decls,
        )?;

        ctx.declare_var(var_name);
    }
    Ok(hoisted_decls)
}

fn generate_single_hoisted_decl(
    var_name: &str,
    var_ident: &syn::Ident,
    var_type: Option<Type>,
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
    hoisted_decls: &mut Vec<proc_macro2::TokenStream>,
) -> Result<()> {
    if let Some(ty) = var_type {
        let rust_type = ctx.type_mapper.map_type(&ty);
        let syn_type = rust_type_to_syn(&rust_type)?;
        if else_body.is_none() {
            hoisted_decls.push(quote! { let mut #var_ident: #syn_type = Default::default(); });
        } else {
            hoisted_decls.push(quote! { let mut #var_ident: #syn_type; });
        }
    } else {
        if else_body.is_none() && ctx.none_placeholder_vars.contains(var_name) {
            hoisted_decls.push(quote! { let mut #var_ident: String = Default::default(); });
        } else {
            hoisted_decls.push(quote! { let mut #var_ident; });
        }
        ctx.hoisted_inference_vars.insert(var_name.to_string());
    }
    Ok(())
}

pub(crate) fn find_variable_type(var_name: &str, stmts: &[HirStmt]) -> Option<Type> {
    for stmt in stmts {
        match stmt {
            HirStmt::Assign { target: AssignTarget::Symbol(name), type_annotation, value }
                if name == var_name =>
            {
                // First try explicit type annotation
                if type_annotation.is_some() {
                    return type_annotation.clone();
                }
                // DEPYLER-0823: If no annotation, infer from the assigned value
                let inferred = crate::rust_gen::func_gen::infer_expr_type_simple(value);
                if !matches!(inferred, Type::Unknown) {
                    return Some(inferred);
                }
                return None;
            }
            // DEPYLER-0931: Handle tuple unpacking (a, b, c) = (1, 2, 3)
            // Extract individual element types from the tuple
            HirStmt::Assign { target: AssignTarget::Tuple(targets), value, .. } => {
                // Find var_name position in the targets
                if let Some(pos) = find_var_position_in_tuple(var_name, targets) {
                    // Infer type from the RHS tuple
                    let rhs_type = crate::rust_gen::func_gen::infer_expr_type_simple(value);
                    if let Type::Tuple(elem_types) = rhs_type {
                        if pos < elem_types.len() {
                            let elem_type = elem_types[pos].clone();
                            if !matches!(elem_type, Type::Unknown) {
                                return Some(elem_type);
                            }
                        }
                    }
                    // If RHS is not a tuple, check if it's a literal tuple expression
                    if let HirExpr::Tuple(elems) = value {
                        if pos < elems.len() {
                            let elem_type =
                                crate::rust_gen::func_gen::infer_expr_type_simple(&elems[pos]);
                            if !matches!(elem_type, Type::Unknown) {
                                return Some(elem_type);
                            }
                        }
                    }
                }
            }
            // DEPYLER-0931: Recursively search nested try/except blocks
            HirStmt::Try { body, handlers, finalbody, .. } => {
                // Search in try body
                if let Some(ty) = find_variable_type(var_name, body) {
                    return Some(ty);
                }
                // Search in handlers
                for handler in handlers {
                    if let Some(ty) = find_variable_type(var_name, &handler.body) {
                        return Some(ty);
                    }
                }
                // Search in finally
                if let Some(finally) = finalbody {
                    if let Some(ty) = find_variable_type(var_name, finally) {
                        return Some(ty);
                    }
                }
            }
            // DEPYLER-0931: Recursively search if/else blocks
            HirStmt::If { then_body, else_body, .. } => {
                if let Some(ty) = find_variable_type(var_name, then_body) {
                    return Some(ty);
                }
                if let Some(else_stmts) = else_body {
                    if let Some(ty) = find_variable_type(var_name, else_stmts) {
                        return Some(ty);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn try_return_type_to_tokens(ty: &Type) -> proc_macro2::TokenStream {
    // Delegate to the main type mapper to support all types (Option, Vec, etc.)
    // Create a temporary context since type mapping shouldn't depend on it for basic types
    // (hir_type_to_tokens only uses ctx for advanced resolution which we might miss here,
    // but it's better than hardcoding)
    // Actually, hir_type_to_tokens ignores ctx in most cases.
    // We can't easily construct a dummy context here, but we can call it if we update the signature.
    // However, since we can't update signature easily without changing all calls, let's duplicate
    // the delegation logic or simplified version.

    // Better: Update to match hir_type_to_tokens logic for common types
    match ty {
        Type::Int => quote! { i32 },
        Type::Float => quote! { f64 },
        Type::String => quote! { String },
        Type::Bool => quote! { bool },
        Type::None => quote! { () },
        Type::Unknown => quote! { () },
        _ => {
            // Fallback: We can't call hir_type_to_tokens because we don't have ctx.
            // But we can handle the Option<Value> case which is common for hoisted vars.
            if let Type::Optional(inner) = ty {
                let inner_tokens = try_return_type_to_tokens(inner);
                quote! { Option<#inner_tokens> }
            } else if let Type::Custom(name) = ty {
                if name == "serde_json::Value" || name == "Value" {
                    quote! { serde_json::Value }
                } else if name == "Dict" {
                    // DEPYLER-1203: Bare Dict annotation -> HashMap<String, DepylerValue>
                    // Keys default to String (most common pattern), values to DepylerValue
                    // This matches dict literal generation for consistency
                    quote! { std::collections::HashMap<String, DepylerValue> }
                } else if name == "List" {
                    // DEPYLER-1157: Bare List annotation -> Vec<DepylerValue>
                    quote! { Vec<DepylerValue> }
                } else {
                    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                    quote! { #ident }
                }
            } else if let Type::Tuple(elems) = ty {
                let elem_tokens: Vec<_> = elems.iter().map(try_return_type_to_tokens).collect();
                quote! { (#(#elem_tokens),*) }
            } else if let Type::List(elem) = ty {
                // DEPYLER-1157: Handle List return types in try/except closures
                let elem_tokens = try_return_type_to_tokens(elem);
                quote! { Vec<#elem_tokens> }
            } else if let Type::Dict(key, value) = ty {
                // DEPYLER-1157: Handle Dict return types in try/except closures
                let key_tokens = try_return_type_to_tokens(key);
                let value_tokens = try_return_type_to_tokens(value);
                quote! { std::collections::HashMap<#key_tokens, #value_tokens> }
            } else {
                quote! { () }
            }
        }
    }
}

pub(crate) fn try_generate_json_stdin_match(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<Option<proc_macro2::TokenStream>> {
    // Must have single assignment in try body
    if body.len() != 1 {
        return Ok(None);
    }

    // Must have single handler that ends with exit
    if handlers.len() != 1 || !handler_ends_with_exit(&handlers[0].body) {
        return Ok(None);
    }

    // Check for: data = json.load(sys.stdin) OR data = json.load(file)
    let (var_name, is_json_load) = match &body[0] {
        HirStmt::Assign { target: AssignTarget::Symbol(name), value, .. } => {
            // Check if value is json.load(sys.stdin) or json.load(file)
            let is_json = match value {
                HirExpr::MethodCall { object, method, args, .. } => {
                    if method == "load" {
                        if let HirExpr::Var(module) = &**object {
                            if module == "json" && args.len() == 1 {
                                // Check if argument is sys.stdin
                                match &args[0] {
                                    HirExpr::Attribute { value: v, attr } => {
                                        if let HirExpr::Var(m) = &**v {
                                            m == "sys" && attr == "stdin"
                                        } else {
                                            false
                                        }
                                    }
                                    // Also allow json.load(f) where f is a file variable
                                    HirExpr::Var(_) => true,
                                    _ => false,
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
                }
                _ => false,
            };
            (name.clone(), is_json)
        }
        _ => return Ok(None),
    };

    if !is_json_load {
        return Ok(None);
    }

    // Generate handler body statements
    ctx.enter_scope();
    if let Some(exc_var) = &handlers[0].name {
        ctx.declare_var(exc_var);
    }

    let handler_stmts: Vec<_> =
        handlers[0].body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    // Generate the error pattern
    let err_pattern = if let Some(exc_var) = &handlers[0].name {
        let exc_ident = safe_ident(exc_var);
        quote! { Err(#exc_ident) }
    } else {
        quote! { Err(_) }
    };

    // Variable identifier
    let var_ident = safe_ident(&var_name);

    // Mark that we need serde_json
    ctx.needs_serde_json = true;

    // Generate the match expression for json.load(sys.stdin)
    let match_expr = quote! {
        let #var_ident = match serde_json::from_reader::<_, serde_json::Value>(std::io::stdin()) {
            Ok(__json_data) => __json_data,
            #err_pattern => {
                #(#handler_stmts)*
            }
        };
    };

    // Add finally block if present
    let result = if let Some(finally_body) = finalbody {
        let finally_stmts: Vec<_> =
            finally_body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;
        quote! {
            #match_expr
            #(#finally_stmts)*
        }
    } else {
        match_expr
    };

    // Declare the variable in context so it's accessible after the try/except
    ctx.declare_var(&var_name);

    Ok(Some(result))
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
