fn extract_args_field_accesses(body: &[HirStmt], args_name: &str) -> Vec<String> {
    use std::collections::HashSet;
    let mut fields: HashSet<String> = HashSet::new();

    for stmt in body {
        walk_stmt_for_args(stmt, args_name, &mut fields);
    }

    // Sort for deterministic output
    let mut result: Vec<String> = fields.into_iter().collect();
    result.sort();
    result
}

pub(crate) fn codegen_function_params(
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    // DEPYLER-0608: For cmd_* handler functions, replace the `args` parameter
    // with individual field parameters based on args.X accesses in the body.
    // This is because subcommand fields are on Commands::Variant, not on Args struct.
    let is_cmd_handler = func.name.starts_with("cmd_") || func.name.starts_with("handle_");
    let has_args_param = func.params.iter().any(|p| p.name == "args");

    if is_cmd_handler && has_args_param {
        // Extract which fields are accessed via args.X
        let accessed_fields = extract_args_field_accesses(&func.body, "args");

        // Mark that we're in a cmd handler so expr_gen knows to transform args.X → X
        ctx.in_cmd_handler = true;
        ctx.cmd_handler_args_fields = accessed_fields.clone();

        let mut params = Vec::new();

        // Process non-args params normally
        for param in &func.params {
            if param.name != "args" {
                params.push(codegen_single_param(param, func, lifetime_result, ctx)?);
            }
        }

        // Add individual field params for each accessed field
        // DEPYLER-0789: Look up correct types from argparser tracker
        // - store_true/store_false → bool
        // - type=int → i32
        // - nargs=*/+ → &[String]
        // - optional fields → Option<String>
        // - default → &str
        // Also infer from body usage if tracker doesn't have info
        for field in &accessed_fields {
            let field_ident = quote::format_ident!("{}", field);

            // Look up field type from argparser tracker or infer from body usage
            let param_tokens = lookup_argparse_field_type(field, &field_ident, ctx, &func.body);
            params.push(param_tokens);
        }

        return Ok(params);
    }

    func.params
        .iter()
        .map(|param| codegen_single_param(param, func, lifetime_result, ctx))
        .collect()
}
