pub(crate) fn try_generate_subcommand_match(
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<Option<proc_macro2::TokenStream>> {
    use quote::{format_ident, quote};

    // DEPYLER-0456 #2: Get dest_field from subparser info
    // Find the dest_field name (e.g., "action" or "command")
    let dest_field = ctx
        .argparser_tracker
        .subparsers
        .values()
        .next()
        .map(|sp| sp.dest_field.clone())
        .unwrap_or_else(|| "command".to_string()); // Default to "command" for backwards compatibility

    // Check if condition matches: args.<dest_field> == "string" OR CSE intermediate variable
    let command_name = match is_subcommand_check(condition, &dest_field, ctx) {
        Some(name) => name,
        None => return Ok(None),
    };

    // Collect all branches (if + elif chain)
    let mut branches = vec![(command_name, then_body)];

    // Check if else is another if statement (elif pattern)
    let mut current_else = else_body;
    while let Some(else_stmts) = current_else {
        // DEPYLER-0456 Bug #2 FIX: Handle CSE-optimized elif branches
        // CSE creates: [assignment: _cse_temp_N = check, if: _cse_temp_N { ... }]
        // Original (pre-CSE) elif is a single If statement
        let (elif_stmt, cse_cmd_name) = if else_stmts.len() == 1 {
            // Pre-CSE or direct elif: single If statement
            (&else_stmts[0], None)
        } else if else_stmts.len() == 2 {
            // CSE-optimized elif: [assignment, if]
            // Extract command name from the CSE assignment
            if let HirStmt::Assign { target: AssignTarget::Symbol(var), value, .. } = &else_stmts[0]
            {
                if var.starts_with("_cse_temp") {
                    // Extract command name from the assignment value
                    let cmd_name = is_subcommand_check(value, &dest_field, ctx);
                    (&else_stmts[1], cmd_name)
                } else {
                    // Not a CSE pattern, stop collecting
                    break;
                }
            } else {
                // Not a CSE pattern, stop collecting
                break;
            }
        } else {
            // Not an elif pattern, stop collecting
            break;
        };

        // Check if this is an If statement with subcommand check
        if let HirStmt::If { condition: elif_cond, then_body: elif_then, else_body: elif_else } =
            elif_stmt
        {
            // Use command name from CSE assignment if available, otherwise check condition
            let elif_name =
                cse_cmd_name.or_else(|| is_subcommand_check(elif_cond, &dest_field, ctx));

            if let Some(name) = elif_name {
                branches.push((name, elif_then.as_slice()));
                current_else = elif_else;
                continue;
            }
        }

        // Not an elif pattern, stop collecting
        break;
    }

    // DEPYLER-0482: Check if any branch has an early return
    // If so, don't add wildcard unreachable!() because execution continues to next match
    let has_early_return = branches
        .iter()
        .any(|(_, body)| body.iter().any(|stmt| matches!(stmt, HirStmt::Return { .. })));

    // Generate match arms
    // DEPYLER-0940: Filter out empty command names to prevent panic in format_ident!()
    let arms: Vec<proc_macro2::TokenStream> = branches
        .iter()
        .filter(|(cmd_name, _)| !cmd_name.is_empty())
        .map(|(cmd_name, body)| {
            // Convert command name to PascalCase variant
            let variant_name = format_ident!("{}", to_pascal_case(cmd_name));

            // DEPYLER-0425: Detect which fields are accessed in the body
            // This determines whether we use Pattern A ({ .. }) or Pattern B ({ field1, field2, ... })
            // DEPYLER-0480: Pass dest_field to dynamically filter based on actual dest parameter
            // DEPYLER-0481: Pass cmd_name and ctx to filter out top-level args
            let mut accessed_fields =
                extract_accessed_subcommand_fields(body, "args", &dest_field, cmd_name, ctx);

            // DEPYLER-0608: Detect if body calls a cmd_* handler
            // If so, get ALL subcommand fields since the handler accesses them internally
            // Pattern: the match arm body is `cmd_list(args)` which needs all `list` subcommand fields
            let calls_cmd_handler = body.iter().any(|stmt| {
                if let HirStmt::Expr(HirExpr::Call { func: func_name, args: call_args, .. }) = stmt {
                    // func is Symbol (String), not Box<HirExpr>
                    // Check if it's a cmd_* or handle_* function call with args parameter
                    let is_handler = func_name.starts_with("cmd_") || func_name.starts_with("handle_");
                    let has_args_param = call_args.iter().any(|a| matches!(a, HirExpr::Var(v) if v == "args"));
                    is_handler && has_args_param
                } else {
                    false
                }
            });

            if calls_cmd_handler && accessed_fields.is_empty() {
                // Get ALL fields for this subcommand
                if let Some(subcommand) = ctx
                    .argparser_tracker
                    .subcommands
                    .values()
                    .filter(|sc| sc.name == *cmd_name)
                    .max_by_key(|sc| sc.arguments.len())
                {
                    for arg in &subcommand.arguments {
                        // DEPYLER-0762: Use rust_field_name() to properly sanitize flag names
                        // This strips leading dashes and converts hyphens to underscores
                        // e.g., "--format" → "format", "--no-color" → "no_color"
                        let field_name = arg.rust_field_name();
                        accessed_fields.push(field_name);
                    }
                }
            }

            // DEPYLER-0608: Set context flags for handler call transformation
            // When in a subcommand match arm that calls a handler, expr_gen will
            // transform cmd_X(args) → cmd_X(field1, field2, ...)
            // DEPYLER-0665: Always set subcommand_match_fields for ref-pattern bindings
            // This allows clone detection in stmt_gen when assigning mutable vars from refs
            let was_in_match_arm = ctx.in_subcommand_match_arm;
            let old_match_fields = std::mem::take(&mut ctx.subcommand_match_fields);
            if calls_cmd_handler {
                ctx.in_subcommand_match_arm = true;
            }
            // Always track ref-pattern bindings, not just when calling handler
            if !accessed_fields.is_empty() {
                ctx.subcommand_match_fields = accessed_fields.clone();
            }

            // Generate body statements
            ctx.enter_scope();

            // DEPYLER-0577: Register field types in var_types before processing body
            // This allows type-aware codegen (e.g., float vs int comparisons)
            // DEPYLER-0605: Use filter + max_by_key to find the SubcommandInfo with most arguments
            // DEPYLER-0722: Handle Optional types and boolean flags correctly
            for field_name in &accessed_fields {
                if let Some(subcommand) = ctx
                    .argparser_tracker
                    .subcommands
                    .values()
                    .filter(|sc| sc.name == *cmd_name)
                    .max_by_key(|sc| sc.arguments.len())
                {
                    if let Some(arg) = subcommand.arguments.iter().find(|a| {
                        // DEPYLER-0722: Strip dashes from short options (-n → n)
                        let arg_name = a.long.as_ref()
                            .map(|s| s.trim_start_matches('-').to_string())
                            .unwrap_or_else(|| a.name.trim_start_matches('-').to_string());
                        &arg_name == field_name
                    }) {
                        // DEPYLER-0722: Determine actual type including Optional wrapper
                        let base_type = if let Some(ref ty) = arg.arg_type {
                            Some(ty.clone())
                        } else if matches!(arg.action.as_deref(), Some("store_true") | Some("store_false")) {
                            Some(Type::Bool)
                        } else {
                            None
                        };

                        if let Some(ty) = base_type {
                            // DEPYLER-0722: Check if this is actually Option<T> in Clap
                            // An argument is Option<T> if: NOT required AND NO default AND NOT positional
                            // AND NOT a boolean flag (store_true/store_false)
                            let is_bool_flag = matches!(arg.action.as_deref(), Some("store_true") | Some("store_false"));
                            let is_option_type = !arg.is_positional
                                && !arg.required.unwrap_or(false)
                                && arg.default.is_none()
                                && !is_bool_flag;

                            let actual_type = if is_option_type {
                                Type::Optional(Box::new(ty))
                            } else {
                                ty
                            };
                            ctx.var_types.insert(field_name.clone(), actual_type);
                        }
                    }
                }
            }

            let body_stmts: Vec<_> = body
                .iter()
                .filter_map(|s| {
                    match s.to_rust_tokens(ctx) {
                        Ok(tokens) => Some(tokens),
                        Err(e) => {
                            // DEPYLER-0593: Log conversion errors instead of silently dropping
                            tracing::warn!("argparse body stmt conversion failed: {}", e);
                            None
                        }
                    }
                })
                .collect();
            ctx.exit_scope();

            // DEPYLER-0608: Restore context flags
            ctx.in_subcommand_match_arm = was_in_match_arm;
            ctx.subcommand_match_fields = old_match_fields;

            // DEPYLER-0456 Bug #3 FIX: Always use struct variant syntax `{}`
            // Clap generates struct variants (e.g., `Init {}`) not unit variants (e.g., `Init`)
            //
            // DEPYLER-0425: Pattern selection based on field usage
            // - Pattern A: No fields accessed → { .. } (handler gets &args)
            // - Pattern B: Fields accessed → { field1, field2, ... } (handler gets individual fields)
            if accessed_fields.is_empty() {
                // Pattern A: No field access, use { .. }
                // DEPYLER-1063: args.command is Option<Commands>, wrap pattern in Some()
                quote! {
                    Some(Commands::#variant_name { .. }) => {
                        #(#body_stmts)*
                    }
                }
            } else {
                // Pattern B: Extract accessed fields with explicit ref patterns
                // Using `ref` ensures consistent binding as references regardless of match ergonomics
                let _field_idents: Vec<syn::Ident> = accessed_fields
                    .iter()
                    .map(|f| format_ident!("{}", f))
                    .collect();
                // DEPYLER-0843: Use safe_ident for keyword escaping in match patterns
                // If a field is named 'type', it needs to be escaped as 'r#type' in patterns
                let ref_field_patterns: Vec<proc_macro2::TokenStream> = accessed_fields
                    .iter()
                    .map(|f| {
                        let ident = safe_ident(f);
                        quote! { ref #ident }
                    })
                    .collect();

                // DEPYLER-0526: Generate field conversion bindings for borrowed match variables
                // When matching &args.command, destructured fields are references (&String, &bool)
                // Convert to owned values so they work with functions expecting either owned or borrowed:
                // - String fields: .to_string() converts &String → String
                //   String can then deref-coerce to &str if needed
                // - bool/primitives: dereference with *
                // DEPYLER-0843: Also use safe_ident for field bindings after match
                let field_bindings: Vec<proc_macro2::TokenStream> = accessed_fields
                    .iter()
                    .map(|field_name| {
                        let field_ident = safe_ident(field_name);

                        // Look up field type from subcommand arguments
                        // Check both arg_type and action (for store_true/store_false bool flags)
                        // DEPYLER-0605: Use filter + max_by_key to find the SubcommandInfo with most arguments
                        let maybe_arg = ctx
                            .argparser_tracker
                            .subcommands
                            .values()
                            .filter(|sc| sc.name == *cmd_name)
                            .max_by_key(|sc| sc.arguments.len())
                            .and_then(|sc| {
                                sc.arguments.iter().find(|arg| {
                                    // Match by field name (from long flag or positional name)
                                    // DEPYLER-0722: Also strip dashes from short options (-n → n)
                                    let arg_field_name = arg
                                        .long
                                        .as_ref()
                                        .map(|s| s.trim_start_matches('-').to_string())
                                        .unwrap_or_else(|| arg.name.trim_start_matches('-').to_string());
                                    arg_field_name == *field_name
                                })
                            });

                        // Determine type: check arg_type first, then action for bool flags
                        let field_type = maybe_arg
                            .and_then(|arg| {
                                // If arg_type is set, use it
                                if let Some(ref base_type) = arg.arg_type {
                                    // DEPYLER-0768: nargs="+" or nargs="*" wraps type in List
                                    // Python: add_argument("values", type=int, nargs="+")
                                    // Rust: Vec<i32> (not i32)
                                    let is_multi = matches!(
                                        arg.nargs.as_deref(),
                                        Some("+") | Some("*")
                                    );
                                    if is_multi {
                                        return Some(Type::List(Box::new(base_type.clone())));
                                    }
                                    return Some(base_type.clone());
                                }
                                // Check action for bool flags: store_true/store_false → Bool
                                if matches!(
                                    arg.action.as_deref(),
                                    Some("store_true") | Some("store_false") | Some("store_const")
                                ) {
                                    return Some(Type::Bool);
                                }
                                None
                            })
                            .or_else(|| {
                                // DEPYLER-0526: Name-based fallback for common boolean fields
                                // If argument lookup failed, use heuristics based on field name
                                let field_lower = field_name.to_lowercase();
                                let bool_indicators = [
                                    "binary",
                                    "append",
                                    "verbose",
                                    "quiet",
                                    "force",
                                    "dry_run",
                                    "recursive",
                                    "debug",
                                    "silent",
                                    "capture",
                                    "overwrite",
                                ];
                                if bool_indicators
                                    .iter()
                                    .any(|ind| field_lower == *ind || field_lower.ends_with(ind))
                                {
                                    Some(Type::Bool)
                                } else {
                                    None
                                }
                            });

                        // Generate conversion based on type
                        // NOTE: With explicit `ref` patterns, all fields are bound as references:
                        //   - Copy types (bool, int, float) need dereferencing: *field
                        //   - Non-Copy types (String, Vec) are already &T
                        match field_type {
                            Some(Type::Bool) => {
                                // With explicit `ref` pattern, bool is &bool - dereference to get bool
                                quote! { let #field_ident = *#field_ident; }
                            }
                            Some(Type::Int) | Some(Type::Float) => {
                                // DEPYLER-0576: Check if field has a default value (is Option<T>)
                                // Clap represents optional args with defaults as Option<T>
                                // ref binding gives &Option<T>, need to unwrap with default
                                // DEPYLER-0722: Also check for Option<T> without default (NOT required AND NOT positional)
                                let has_default = maybe_arg
                                    .as_ref()
                                    .map(|a| a.default.is_some())
                                    .unwrap_or(false);
                                let is_required = maybe_arg
                                    .as_ref()
                                    .map(|a| a.required.unwrap_or(false))
                                    .unwrap_or(false);
                                let is_positional = maybe_arg
                                    .as_ref()
                                    .map(|a| a.is_positional)
                                    .unwrap_or(false);
                                // DEPYLER-0722: An argument is Option<T> if NOT required AND NOT positional AND NO default
                                let is_option_without_default = !is_required && !is_positional && !has_default;

                                if has_default {
                                    // Field is Option<T>, unwrap with default
                                    // Clone the default expression to release borrow on ctx
                                    let default_expr_opt = maybe_arg
                                        .and_then(|a| a.default.clone());

                                    let default_val = if let Some(ref d) = default_expr_opt {
                                        d.to_rust_expr(ctx).ok()
                                    } else {
                                        None
                                    }.unwrap_or_else(|| {
                                        // Fallback to 0.0 for Float, 0 for Int
                                        if matches!(field_type, Some(Type::Float)) {
                                            syn::parse_quote! { 0.0 }
                                        } else {
                                            syn::parse_quote! { 0 }
                                        }
                                    });
                                    quote! { let #field_ident = #field_ident.unwrap_or(#default_val); }
                                } else if is_option_without_default {
                                    // DEPYLER-0722: Option<T> without default - clone the &Option<T> to Option<T>
                                    // Body code can then use .is_some() for truthiness
                                    quote! { let #field_ident = #field_ident.clone(); }
                                } else {
                                    // Required field (not Option), just dereference
                                    quote! { let #field_ident = *#field_ident; }
                                }
                            }
                            Some(Type::String) => {
                                // DEPYLER-0933: First check if this is an Optional String field
                                // (NOT required AND NOT positional AND NO default)
                                let has_default = maybe_arg
                                    .as_ref()
                                    .map(|a| a.default.is_some())
                                    .unwrap_or(false);
                                let is_required = maybe_arg
                                    .as_ref()
                                    .map(|a| a.required.unwrap_or(false))
                                    .unwrap_or(false);
                                let is_positional = maybe_arg
                                    .as_ref()
                                    .map(|a| a.is_positional)
                                    .unwrap_or(false);
                                let is_option_without_default =
                                    !is_required && !is_positional && !has_default;

                                // DEPYLER-0526: Name-based heuristics for String handling
                                let field_lower = field_name.to_lowercase();
                                let owned_indicators = [
                                    "file", "path", "filepath", "input", "output", "dir",
                                    "directory",
                                ];
                                let borrowed_indicators =
                                    ["content", "pattern", "text", "message", "data", "value"];

                                let needs_owned = owned_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                });
                                let needs_borrowed = borrowed_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                });

                                if is_option_without_default || has_default {
                                    // DEPYLER-0933: Option<String> field - unwrap with default
                                    // Use as_deref() to get Option<&str>, then unwrap_or("")
                                    // This gives &str which works for both &str and String params
                                    quote! { let #field_ident = #field_ident.as_deref().unwrap_or_default(); }
                                } else {
                                    // Required field - regular String handling
                                    if needs_borrowed {
                                        // Keep as &String, auto-derefs to &str
                                        quote! {}
                                    } else if needs_owned {
                                        // Convert to owned String
                                        quote! { let #field_ident = #field_ident.to_string(); }
                                    } else {
                                        // Default: convert to owned (safer for function calls)
                                        quote! { let #field_ident = #field_ident.to_string(); }
                                    }
                                }
                            }
                            Some(Type::Optional(_))
                            | Some(Type::List(_))
                            | Some(Type::Dict(_, _)) => {
                                // For complex container types, clone the reference
                                quote! { let #field_ident = #field_ident.clone(); }
                            }
                            None => {
                                // DEPYLER-0933: Check if this is an Optional field (unknown type)
                                let has_default = maybe_arg
                                    .as_ref()
                                    .map(|a| a.default.is_some())
                                    .unwrap_or(false);
                                let is_required = maybe_arg
                                    .as_ref()
                                    .map(|a| a.required.unwrap_or(false))
                                    .unwrap_or(false);
                                let is_positional = maybe_arg
                                    .as_ref()
                                    .map(|a| a.is_positional)
                                    .unwrap_or(false);
                                let is_option_without_default =
                                    !is_required && !is_positional && !has_default;

                                // Unknown type: use name-based heuristics
                                let field_lower = field_name.to_lowercase();
                                let owned_indicators = [
                                    "file",
                                    "path",
                                    "filepath",
                                    "input",
                                    "output",
                                    "dir",
                                    "directory",
                                ];
                                let borrowed_indicators =
                                    ["content", "pattern", "text", "message", "data", "value"];
                                // DEPYLER-0579: String-like field indicators (should NOT be numeric-unwrapped)
                                let string_indicators = [
                                    "str", "string", "name", "line", "word", "char", "cmd",
                                    "url", "uri", "host", "token", "key", "id", "code",
                                    "hex", "oct", // hex/oct values are string representations
                                    "suffix", // DEPYLER-0933: suffix is a string field
                                ];
                                // DEPYLER-0576: Numeric field indicators (likely f64 with defaults)
                                // DEPYLER-0592: Removed single letters - too ambiguous, often strings
                                let numeric_indicators = [
                                    "x1", "x2", "y1", "y2", "z1", "z2",
                                    "val", "num", "count", "rate", "coef", "factor",
                                    "min", "max", "sum", "avg", "mean", "std",
                                    "width", "height", "size", "len", "length",
                                    "alpha", "beta", "gamma", "theta", "lr",
                                ];

                                let needs_owned = owned_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                });
                                let needs_borrowed = borrowed_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                });
                                // DEPYLER-0579: Check if this looks like a string field
                                let looks_like_string = string_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                        || field_lower.contains(ind)
                                });
                                // Only apply numeric unwrap if NOT string-like
                                let needs_numeric_unwrap = !looks_like_string
                                    && numeric_indicators.iter().any(|ind| {
                                        field_lower == *ind
                                            || field_lower.ends_with(ind)
                                            || field_lower.starts_with(ind)
                                    });

                                // DEPYLER-0933: If optional without default and looks like string,
                                // use as_deref() to get &str which works for both &str and String params
                                if is_option_without_default && looks_like_string {
                                    quote! { let #field_ident = #field_ident.as_deref().unwrap_or_default(); }
                                } else if is_option_without_default && (needs_owned || needs_borrowed) {
                                    // Optional string-like field, unwrap with as_deref for &str
                                    quote! { let #field_ident = #field_ident.as_deref().unwrap_or_default(); }
                                } else if needs_borrowed || (is_positional && needs_owned) {
                                    // DEPYLER-0933: Keep as reference for:
                                    // - borrowed indicators (content, pattern, text, etc.)
                                    // - positional string fields that match owned indicators
                                    //   (file, path, output, etc.) - these are &String, not &Option<String>
                                    quote! {}
                                } else if needs_owned || looks_like_string {
                                    // Convert to owned String (for optional fields that weren't caught above)
                                    quote! { let #field_ident = #field_ident.to_string(); }
                                } else if needs_numeric_unwrap {
                                    // DEPYLER-0576: Likely numeric Option<T> field, unwrap with default
                                    // DEPYLER-0677: Use Default::default() for type-safe numeric defaults
                                    quote! { let #field_ident = #field_ident.unwrap_or_default(); }
                                } else {
                                    // Unknown: keep as reference (safer default)
                                    quote! {}
                                }
                            }
                            _ => {
                                // For other complex types, clone
                                quote! { let #field_ident = #field_ident.clone(); }
                            }
                        }
                    })
                    .collect();

                // DEPYLER-0578: Add `..` to pattern to ignore unmentioned fields (fixes E0027)
                // The subcommand may have more fields than we extract from body statements
                // DEPYLER-1063: args.command is Option<Commands>, wrap pattern in Some()
                quote! {
                    Some(Commands::#variant_name { #(#ref_field_patterns,)* .. }) => {
                        #(#field_bindings)*
                        #(#body_stmts)*
                    }
                }
            }
        })
        .collect();

    // DEPYLER-0456 Bug #3 FIX: Always use "command" as the Rust struct field name
    // The Args struct always has `command: Commands` regardless of Python's dest parameter
    // DEPYLER-0470: Add wildcard arm to make match exhaustive
    // When early returns split matches, not all Commands variants may be in this match
    // Use unreachable!() because split matches ensure mutually exclusive variants
    // DEPYLER-0474: Match by reference to avoid partial move errors
    // When handler functions take &args, we must borrow args.command, not move it
    // DEPYLER-0482: Only add wildcard if no early returns (otherwise execution continues to next match)
    Ok(Some(if has_early_return {
        // Early return present: Don't add wildcard, execution continues to next match
        quote! {
            match &args.command {
                #(#arms)*
                _ => {}
            }
        }
    } else {
        // No early returns: This is likely the final/complete match, add unreachable wildcard
        quote! {
            match &args.command {
                #(#arms)*
                _ => unreachable!("Other command variants handled elsewhere")
            }
        }
    }))
}

pub(crate) fn is_subcommand_check(
    expr: &HirExpr,
    dest_field: &str,
    ctx: &CodeGenContext,
) -> Option<String> {
    match expr {
        // Direct comparison: args.action == "init"
        HirExpr::Binary { op: BinOp::Eq, left, right } => {
            // DEPYLER-0456 #2: Check if left side is args.<dest_field>
            // (e.g., args.action, args.command, etc.)
            let is_dest_field_attr = matches!(
                left.as_ref(),
                HirExpr::Attribute { attr, .. } if attr == dest_field
            );

            // Check if right side is a string literal
            if is_dest_field_attr {
                if let HirExpr::Literal(Literal::String(cmd_name)) = right.as_ref() {
                    return Some(cmd_name.clone());
                }
            }
            None
        }
        // DEPYLER-0456 Bug #2 FIX: CSE temp variable (e.g., _cse_temp_0)
        // After CSE optimization, the condition becomes just a variable reference
        HirExpr::Var(var_name) => {
            // Look up in CSE subcommand temps map
            ctx.cse_subcommand_temps.get(var_name).cloned()
        }
        _ => None,
    }
}
