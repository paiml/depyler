fn handle_argparser_method_call(
    target: &AssignTarget,
    var_name: &str,
    method: &str,
    object: &HirExpr,
    args: &[HirExpr],
    kwargs: &[(String, HirExpr)],
    ctx: &mut CodeGenContext,
) -> Option<proc_macro2::TokenStream> {
    // Pattern 1: ArgumentParser constructor
    if method == "ArgumentParser" {
        return handle_argumentparser_constructor(var_name, object, kwargs, ctx);
    }

    // Pattern 2: args = parser.parse_args()
    if method == "parse_args" {
        return handle_parse_args_call(var_name, object, ctx);
    }

    // Pattern 3: add_argument_group, add_mutually_exclusive_group, set_defaults
    if let Some(result) = handle_argument_group_method(target, method, object, ctx) {
        return Some(result);
    }

    // Pattern 4: add_subparsers
    if method == "add_subparsers" {
        return handle_add_subparsers_call(target, object, kwargs, ctx);
    }

    // Pattern 5: add_parser (subcommand)
    if method == "add_parser" {
        return handle_add_parser_call(target, object, args, kwargs, ctx);
    }

    None
}

fn handle_argumentparser_constructor(
    var_name: &str,
    object: &HirExpr,
    kwargs: &[(String, HirExpr)],
    ctx: &mut CodeGenContext,
) -> Option<proc_macro2::TokenStream> {
    if let HirExpr::Var(module_name) = object {
        if module_name == "argparse" {
            let mut info =
                crate::rust_gen::argparse_transform::ArgParserInfo::new(var_name.to_string());
            for (key, value_expr) in kwargs {
                if key == "description" {
                    if let HirExpr::Literal(Literal::String(s)) = value_expr {
                        info.description = Some(s.clone());
                    }
                } else if key == "epilog" {
                    if let HirExpr::Literal(Literal::String(s)) = value_expr {
                        info.epilog = Some(s.clone());
                    }
                }
            }
            ctx.argparser_tracker.register_parser(var_name.to_string(), info);
            return Some(quote! {});
        }
    }
    None
}

fn handle_parse_args_call(
    var_name: &str,
    object: &HirExpr,
    ctx: &mut CodeGenContext,
) -> Option<proc_macro2::TokenStream> {
    if let HirExpr::Var(parser_var) = object {
        if let Some(parser_info) = ctx.argparser_tracker.get_parser_mut(parser_var) {
            parser_info.set_args_var(var_name.to_string());
            let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
            return Some(quote! { let #var_ident = Args::parse(); });
        }
    }
    None
}

fn handle_argument_group_method(
    target: &AssignTarget,
    method: &str,
    object: &HirExpr,
    ctx: &mut CodeGenContext,
) -> Option<proc_macro2::TokenStream> {
    if !matches!(method, "add_argument_group" | "add_mutually_exclusive_group" | "set_defaults") {
        return None;
    }
    if let HirExpr::Var(parent_var) = object {
        let is_parser_or_group = ctx.argparser_tracker.get_parser(parent_var).is_some()
            || ctx.argparser_tracker.get_parser_for_group(parent_var).is_some();
        if is_parser_or_group {
            if let AssignTarget::Symbol(group_var) = target {
                ctx.argparser_tracker.register_group(group_var.clone(), parent_var.clone());
            }
            return Some(quote! {});
        }
    }
    None
}

fn handle_add_subparsers_call(
    target: &AssignTarget,
    object: &HirExpr,
    kwargs: &[(String, HirExpr)],
    ctx: &mut CodeGenContext,
) -> Option<proc_macro2::TokenStream> {
    if let HirExpr::Var(parser_var) = object {
        if ctx.argparser_tracker.get_parser(parser_var).is_some() {
            let dest_field =
                extract_kwarg_string(kwargs, "dest").unwrap_or_else(|| "command".to_string());
            let required = extract_kwarg_bool(kwargs, "required").unwrap_or(false);
            let help = extract_kwarg_string(kwargs, "help");
            if let AssignTarget::Symbol(subparsers_var) = target {
                use crate::rust_gen::argparse_transform::SubparserInfo;
                ctx.argparser_tracker.register_subparsers(
                    subparsers_var.clone(),
                    SubparserInfo { parser_var: parser_var.clone(), dest_field, required, help },
                );
            }
            return Some(quote! {});
        }
    }
    None
}

fn handle_add_parser_call(
    target: &AssignTarget,
    object: &HirExpr,
    args: &[HirExpr],
    kwargs: &[(String, HirExpr)],
    ctx: &mut CodeGenContext,
) -> Option<proc_macro2::TokenStream> {
    if let HirExpr::Var(subparsers_var) = object {
        if ctx.argparser_tracker.get_subparsers(subparsers_var).is_some() && !args.is_empty() {
            let command_name = extract_string_literal(&args[0]);
            let help = extract_kwarg_string(kwargs, "help");
            if let AssignTarget::Symbol(subcommand_var) = target {
                use crate::rust_gen::argparse_transform::SubcommandInfo;
                let cmd_name = command_name.clone();
                if ctx.argparser_tracker.get_subcommand(&cmd_name).is_none() {
                    ctx.argparser_tracker.register_subcommand(
                        cmd_name.clone(),
                        SubcommandInfo {
                            name: command_name,
                            help,
                            arguments: vec![],
                            subparsers_var: subparsers_var.clone(),
                        },
                    );
                }
                ctx.argparser_tracker
                    .subcommand_var_to_cmd
                    .insert(subcommand_var.clone(), cmd_name);
            }
            return Some(quote! {});
        }
    }
    None
}
