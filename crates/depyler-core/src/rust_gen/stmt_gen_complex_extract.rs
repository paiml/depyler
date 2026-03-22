fn extract_accessed_subcommand_fields(
    body: &[HirStmt],
    args_var: &str,
    dest_field: &str,
    cmd_name: &str,
    ctx: &CodeGenContext,
) -> Vec<String> {
    let mut fields = std::collections::HashSet::new();
    extract_fields_recursive(body, args_var, dest_field, &mut fields);

    // DEPYLER-0481: Filter out top-level args that don't belong to this subcommand
    // Only keep fields that are actual arguments of the subcommand
    // DEPYLER-0605: Fix duplicate SubcommandInfo issue - prefer the one with arguments
    // When preregister_subcommands_from_hir runs, it may create an empty SubcommandInfo
    // with KEY = command_name. Later, assignment processing creates another with
    // KEY = variable_name and the actual arguments. We need to find the one with args.
    let subcommand_arg_names: std::collections::HashSet<String> = ctx
        .argparser_tracker
        .subcommands
        .values()
        .filter(|sub| sub.name == cmd_name)
        .max_by_key(|sub| sub.arguments.len())
        .map(|sub| {
            sub.arguments
                .iter()
                .map(|arg| {
                    // Extract dest name from argument
                    arg.dest.clone().unwrap_or_else(|| {
                        // If no dest, use the name (for positional) or long option without dashes
                        if arg.is_positional {
                            arg.name.clone()
                        } else if let Some(long) = &arg.long {
                            long.trim_start_matches("--").replace('-', "_")
                        } else {
                            arg.name.trim_start_matches('-').replace('-', "_")
                        }
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let mut result: Vec<_> =
        fields.into_iter().filter(|f| subcommand_arg_names.contains(f)).collect();
    result.sort(); // Deterministic order
    result
}

pub(crate) fn extract_fields_recursive(
    stmts: &[HirStmt],
    args_var: &str,
    dest_field: &str,
    fields: &mut std::collections::HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Expr(expr) => extract_fields_from_expr(expr, args_var, dest_field, fields),
            HirStmt::Assign { value, .. } => {
                extract_fields_from_expr(value, args_var, dest_field, fields)
            }
            HirStmt::If { condition, then_body, else_body } => {
                // DEPYLER-0518: Also extract fields from condition
                // Example: `if not validate_email(args.address)` has args.address in condition
                extract_fields_from_expr(condition, args_var, dest_field, fields);
                extract_fields_recursive(then_body, args_var, dest_field, fields);
                if let Some(else_stmts) = else_body {
                    extract_fields_recursive(else_stmts, args_var, dest_field, fields);
                }
            }
            // DEPYLER-0577: Recurse into While condition (may contain args.field)
            HirStmt::While { condition, body: loop_body } => {
                extract_fields_from_expr(condition, args_var, dest_field, fields);
                extract_fields_recursive(loop_body, args_var, dest_field, fields);
            }
            // DEPYLER-0577: Recurse into For iterator (may contain args.field)
            HirStmt::For { iter, body: loop_body, .. } => {
                extract_fields_from_expr(iter, args_var, dest_field, fields);
                extract_fields_recursive(loop_body, args_var, dest_field, fields);
            }
            HirStmt::Try { body: try_body, handlers, orelse, finalbody } => {
                extract_fields_recursive(try_body, args_var, dest_field, fields);
                for handler in handlers {
                    extract_fields_recursive(&handler.body, args_var, dest_field, fields);
                }
                if let Some(orelse_stmts) = orelse {
                    extract_fields_recursive(orelse_stmts, args_var, dest_field, fields);
                }
                if let Some(finally_stmts) = finalbody {
                    extract_fields_recursive(finally_stmts, args_var, dest_field, fields);
                }
            }
            HirStmt::With { context, body: with_body, .. } => {
                // DEPYLER-0931: Extract fields from With context expression
                // Pattern: `with open(args.file) as f:` - args.file is in context
                // This was missing, causing E0425 errors for fields used in context
                extract_fields_from_expr(context, args_var, dest_field, fields);
                extract_fields_recursive(with_body, args_var, dest_field, fields);
            }
            _ => {}
        }
    }
}

pub(crate) fn extract_fields_from_expr(
    expr: &HirExpr,
    args_var: &str,
    dest_field: &str,
    fields: &mut std::collections::HashSet<String>,
) {
    match expr {
        // Pattern: args.field
        HirExpr::Attribute { value, attr } => {
            if let HirExpr::Var(var) = value.as_ref() {
                if var == args_var {
                    // DEPYLER-0480: Filter out the dest field dynamically
                    // The dest field (e.g., "command" or "action") is the match discriminant,
                    // so it shouldn't be included in the extracted fields list
                    if attr != dest_field {
                        fields.insert(attr.clone());
                    }
                }
            }
        }
        // Recurse into nested expressions
        HirExpr::Call { args: call_args, .. } => {
            for arg in call_args {
                extract_fields_from_expr(arg, args_var, dest_field, fields);
            }
        }
        HirExpr::Binary { left, right, .. } => {
            extract_fields_from_expr(left, args_var, dest_field, fields);
            extract_fields_from_expr(right, args_var, dest_field, fields);
        }
        HirExpr::Unary { operand, .. } => {
            extract_fields_from_expr(operand, args_var, dest_field, fields);
        }
        HirExpr::IfExpr { test, body, orelse } => {
            extract_fields_from_expr(test, args_var, dest_field, fields);
            extract_fields_from_expr(body, args_var, dest_field, fields);
            extract_fields_from_expr(orelse, args_var, dest_field, fields);
        }
        HirExpr::Index { base, index } => {
            extract_fields_from_expr(base, args_var, dest_field, fields);
            extract_fields_from_expr(index, args_var, dest_field, fields);
        }
        HirExpr::List(elements) | HirExpr::Tuple(elements) | HirExpr::Set(elements) => {
            for elem in elements {
                extract_fields_from_expr(elem, args_var, dest_field, fields);
            }
        }
        HirExpr::Dict(pairs) => {
            for (key, value) in pairs {
                extract_fields_from_expr(key, args_var, dest_field, fields);
                extract_fields_from_expr(value, args_var, dest_field, fields);
            }
        }
        HirExpr::MethodCall { object, args: method_args, .. } => {
            extract_fields_from_expr(object, args_var, dest_field, fields);
            for arg in method_args {
                extract_fields_from_expr(arg, args_var, dest_field, fields);
            }
        }
        // DEPYLER-0577: Handle f-strings - recurse into expression parts
        HirExpr::FString { parts } => {
            for part in parts {
                if let crate::hir::FStringPart::Expr(expr) = part {
                    extract_fields_from_expr(expr, args_var, dest_field, fields);
                }
            }
        }
        _ => {}
    }
}
