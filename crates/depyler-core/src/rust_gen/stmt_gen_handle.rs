fn apply_truthiness_attribute_access(
    obj_name: &str,
    attr: &str,
    cond_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> Option<syn::Expr> {
    // DEPYLER-0904: Handle self.* attribute access for class fields
    if obj_name == "self" {
        if let Some(result) = apply_truthiness_self_field(attr, cond_expr.clone(), ctx) {
            return Some(result);
        }
    }

    // Check if this is accessing an args variable from ArgumentParser
    let is_args_var = ctx.argparser_tracker.parsers.values().any(|parser_info| {
        parser_info.args_var.as_ref().is_some_and(|args_var| args_var == obj_name)
    });

    if is_args_var {
        if let Some(result) = apply_truthiness_argparser_field(attr, cond_expr.clone(), ctx) {
            return Some(result);
        }
    }

    // DEPYLER-0950: Heuristic for common String attribute names
    if is_common_string_attr_name(attr) {
        return Some(parse_quote! { !#cond_expr.is_empty() });
    }
    None
}

fn is_common_string_attr_name(attr: &str) -> bool {
    const STRING_ATTR_NAMES: &[&str] = &[
        "email", "name", "text", "content", "message", "title", "description",
        "path", "url", "value", "data", "body", "subject", "address", "filename",
        "username", "password", "token", "key", "secret", "label", "output",
        "input", "stdout", "stderr", "error", "warning", "info", "debug",
    ];
    STRING_ATTR_NAMES.contains(&attr)
}

fn apply_truthiness_self_field(
    attr: &str,
    cond_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> Option<syn::Expr> {
    if let Some(field_type) = ctx.class_field_types.get(attr) {
        return Some(match field_type {
            Type::Bool => cond_expr,
            Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                parse_quote! { !#cond_expr.is_empty() }
            }
            Type::Optional(_) => parse_quote! { #cond_expr.is_some() },
            Type::Int => parse_quote! { #cond_expr != 0 },
            Type::Float => parse_quote! { #cond_expr != 0.0 },
            _ => cond_expr,
        });
    }
    if is_common_string_attr_name(attr) {
        return Some(parse_quote! { !#cond_expr.is_empty() });
    }
    if is_collection_attr_name(attr) {
        return Some(parse_quote! { !#cond_expr.is_empty() });
    }
    None
}

fn apply_truthiness_argparser_field(
    attr: &str,
    cond_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> Option<syn::Expr> {
    // Check optional fields
    let check_optional = |arg: &super::argparse_transform::ArgParserArgument| -> bool {
        let field_name = arg.rust_field_name();
        if field_name != attr {
            return false;
        }
        if matches!(arg.action.as_deref(), Some("store_true") | Some("store_false")) {
            return false;
        }
        !arg.is_positional
            && !arg.required.unwrap_or(false)
            && arg.default.is_none()
            && !matches!(arg.nargs.as_deref(), Some("+") | Some("*"))
    };

    let is_optional_in_parser = ctx
        .argparser_tracker
        .parsers
        .values()
        .any(|pi| pi.arguments.iter().any(&check_optional));
    let is_optional_in_subcommand = ctx
        .argparser_tracker
        .subcommands
        .values()
        .any(|si| si.arguments.iter().any(&check_optional));

    if is_optional_in_parser || is_optional_in_subcommand {
        if ctx.precomputed_option_fields.contains(attr) {
            let has_var_name = format!("has_{}", attr);
            let has_ident = syn::Ident::new(&has_var_name, proc_macro2::Span::call_site());
            return Some(parse_quote! { #has_ident });
        }
        return Some(parse_quote! { #cond_expr.is_some() });
    }

    // Check Vec fields (nargs='+' or nargs='*')
    let is_vec_field = ctx.argparser_tracker.parsers.values().any(|pi| {
        pi.arguments.iter().any(|arg| {
            arg.rust_field_name() == attr
                && matches!(arg.nargs.as_deref(), Some("+") | Some("*"))
        })
    });
    if is_vec_field {
        return Some(parse_quote! { !#cond_expr.is_empty() });
    }

    // Check String with default value
    let is_string_with_default = ctx.argparser_tracker.parsers.values().any(|pi| {
        pi.arguments.iter().any(|arg| {
            arg.rust_field_name() == attr
                && arg.default.is_some()
                && arg.arg_type.is_none()
                && !matches!(arg.action.as_deref(), Some("store_true") | Some("store_false"))
        })
    });
    if is_string_with_default {
        return Some(parse_quote! { !#cond_expr.is_empty() });
    }

    None
}
