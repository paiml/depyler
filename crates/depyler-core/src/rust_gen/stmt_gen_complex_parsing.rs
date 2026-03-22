fn extract_parse_from_tokens(
    try_stmts: &[proc_macro2::TokenStream],
) -> Option<(String, String, Vec<proc_macro2::TokenStream>)> {
    if try_stmts.is_empty() {
        return None;
    }

    let first_token_stream = &try_stmts[0];

    // Attempt to parse as a Stmt. If it fails (e.g. partial tokens), we bail.
    let stmt: syn::Stmt = syn::parse2(first_token_stream.clone()).ok()?;

    match stmt {
        // Handle: let var = expr;
        syn::Stmt::Local(local) => {
            // Extract variable name from pattern
            let var_name = if let syn::Pat::Ident(pat_ident) = &local.pat {
                pat_ident.ident.to_string()
            } else {
                return None;
            };

            // Check init expression
            if let Some(init) = &local.init {
                let parse_expr = extract_parse_expr(&init.expr)?;
                let remaining = try_stmts[1..].to_vec();
                return Some((var_name, parse_expr, remaining));
            }
        }
        // Handle: var = expr; or expr;
        syn::Stmt::Expr(expr, _) => {
            if let syn::Expr::Assign(assign) = expr {
                // Extract var name from LHS
                let var_name = if let syn::Expr::Path(path) = *assign.left {
                    path.path.segments.last()?.ident.to_string()
                } else {
                    return None;
                };

                let parse_expr = extract_parse_expr(&assign.right)?;
                let remaining = try_stmts[1..].to_vec();
                return Some((var_name, parse_expr, remaining));
            }
        }
        // DEPYLER-FIX-RC1: Explicitly ignore Item (fn, struct) and Mac (macro)
        // This ensures we never match a "for loop" or "while loop" even if it contains "parse"
        _ => return None,
    }

    None
}

fn extract_parse_expr(expr: &syn::Expr) -> Option<String> {
    // We are looking for: MethodCall(unwrap_or_default) -> MethodCall(parse)

    // 1. Check outer method: unwrap_or_default()
    if let syn::Expr::MethodCall(unwrap_call) = expr {
        if unwrap_call.method != "unwrap_or_default" {
            return None;
        }

        // 2. Check inner receiver: parse()
        if let syn::Expr::MethodCall(parse_call) = &*unwrap_call.receiver {
            if parse_call.method == "parse" {
                // Found it! Return the receiver of the parse call (the string being parsed)
                // We reconstruct it to a string to match the original function signature
                return Some(quote::quote!(#parse_call).to_string());
            }
        }
    }
    None
}
