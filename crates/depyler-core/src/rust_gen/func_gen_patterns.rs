fn lookup_argparse_field_type(
    field: &str,
    field_ident: &proc_macro2::Ident,
    ctx: &CodeGenContext,
    body: &[crate::hir::HirStmt],
) -> proc_macro2::TokenStream {
    use crate::hir::Type;

    // Search all subcommands for this field
    for subcommand in ctx.argparser_tracker.subcommands.values() {
        for arg in &subcommand.arguments {
            let arg_field_name = arg.rust_field_name();
            if arg_field_name == field {
                // Found the argument - determine its type
                if matches!(arg.action.as_deref(), Some("store_true") | Some("store_false")) {
                    return quote::quote! { #field_ident: bool };
                }
                if matches!(arg.nargs.as_deref(), Some("+") | Some("*")) {
                    return quote::quote! { #field_ident: &[String] };
                }
                if let Some(ref arg_type) = arg.arg_type {
                    match arg_type {
                        Type::Int => return quote::quote! { #field_ident: i32 },
                        Type::Float => return quote::quote! { #field_ident: f64 },
                        Type::Bool => return quote::quote! { #field_ident: bool },
                        _ => {}
                    }
                }
                break;
            }
        }
    }

    // Also check main parser arguments
    for parser in ctx.argparser_tracker.parsers.values() {
        for arg in &parser.arguments {
            let arg_field_name = arg.rust_field_name();
            if arg_field_name == field {
                if matches!(arg.action.as_deref(), Some("store_true") | Some("store_false")) {
                    return quote::quote! { #field_ident: bool };
                }
                if matches!(arg.nargs.as_deref(), Some("+") | Some("*")) {
                    return quote::quote! { #field_ident: &[String] };
                }
                if let Some(ref arg_type) = arg.arg_type {
                    match arg_type {
                        Type::Int => return quote::quote! { #field_ident: i32 },
                        Type::Float => return quote::quote! { #field_ident: f64 },
                        Type::Bool => return quote::quote! { #field_ident: bool },
                        _ => {}
                    }
                }
                break;
            }
        }
    }

    // DEPYLER-0789: Infer type from body usage if tracker doesn't have info
    // If args.field is used directly in if condition → bool
    if is_field_used_as_bool_condition(field, body) {
        return quote::quote! { #field_ident: bool };
    }

    // DEPYLER-0914: Infer numeric type if args.field is used in arithmetic operations
    // Pattern: args.r / 255, args.g * 2, etc. → i32
    // Pattern: args.h * 6.0, etc. → f64
    if let Some(numeric_type) = infer_numeric_type_from_arithmetic_usage(field, body) {
        match numeric_type {
            crate::hir::Type::Int => return quote::quote! { #field_ident: i32 },
            crate::hir::Type::Float => return quote::quote! { #field_ident: f64 },
            _ => {}
        }
    }

    // Default: string type with heuristic for lists
    let is_list_field =
        field.ends_with('s') && !["status", "args", "class", "process"].contains(&field);
    if is_list_field {
        quote::quote! { #field_ident: &[String] }
    } else {
        quote::quote! { #field_ident: &str }
    }
}

fn is_field_used_as_bool_condition(field: &str, body: &[crate::hir::HirStmt]) -> bool {
    use crate::hir::{HirExpr, HirStmt};

    fn check_expr_is_field_access(expr: &HirExpr, field: &str) -> bool {
        matches!(
            expr,
            HirExpr::Attribute { value, attr }
            if matches!(value.as_ref(), HirExpr::Var(v) if v == "args")
            && attr == field
        )
    }

    fn check_stmt(stmt: &HirStmt, field: &str) -> bool {
        match stmt {
            HirStmt::If { condition, then_body, else_body } => {
                // Check if condition is `args.field` directly (used as bool)
                if check_expr_is_field_access(condition, field) {
                    return true;
                }
                // Recurse into then/else
                if then_body.iter().any(|s| check_stmt(s, field)) {
                    return true;
                }
                if let Some(else_stmts) = else_body {
                    if else_stmts.iter().any(|s| check_stmt(s, field)) {
                        return true;
                    }
                }
                false
            }
            HirStmt::While { condition, body } => {
                if check_expr_is_field_access(condition, field) {
                    return true;
                }
                body.iter().any(|s| check_stmt(s, field))
            }
            HirStmt::For { body, .. } => body.iter().any(|s| check_stmt(s, field)),
            HirStmt::With { body, .. } => body.iter().any(|s| check_stmt(s, field)),
            HirStmt::Try { body, handlers, finalbody, .. } => {
                body.iter().any(|s| check_stmt(s, field))
                    || handlers.iter().any(|h| h.body.iter().any(|s| check_stmt(s, field)))
                    || finalbody.as_ref().is_some_and(|f| f.iter().any(|s| check_stmt(s, field)))
            }
            _ => false,
        }
    }

    body.iter().any(|stmt| check_stmt(stmt, field))
}

fn infer_numeric_type_from_arithmetic_usage(
    field: &str,
    body: &[crate::hir::HirStmt],
) -> Option<crate::hir::Type> {
    use crate::hir::{HirExpr, HirStmt};

    fn check_expr_is_field_access(expr: &HirExpr, field: &str) -> bool {
        matches!(
            expr,
            HirExpr::Attribute { value, attr }
            if matches!(value.as_ref(), HirExpr::Var(v) if v == "args")
            && attr == field
        )
    }

    fn infer_from_expr(expr: &HirExpr, field: &str) -> Option<crate::hir::Type> {
        match expr {
            // Binary operation: args.field op value OR value op args.field
            HirExpr::Binary { left, right, .. } => {
                let left_is_field = check_expr_is_field_access(left, field);
                let right_is_field = check_expr_is_field_access(right, field);

                if left_is_field {
                    // Check right operand type
                    return infer_type_from_operand(right);
                }
                if right_is_field {
                    // Check left operand type
                    return infer_type_from_operand(left);
                }

                // Recurse into sub-expressions
                infer_from_expr(left, field).or_else(|| infer_from_expr(right, field))
            }
            // Tuple unpacking: (args.r / 255, args.g / 255, args.b / 255)
            HirExpr::Tuple(elements) => {
                for elem in elements {
                    if let Some(ty) = infer_from_expr(elem, field) {
                        return Some(ty);
                    }
                }
                None
            }
            // List/array expressions
            HirExpr::List(elements) => {
                for elem in elements {
                    if let Some(ty) = infer_from_expr(elem, field) {
                        return Some(ty);
                    }
                }
                None
            }
            // Function calls - check arguments
            HirExpr::Call { args, .. } => {
                for arg in args {
                    if let Some(ty) = infer_from_expr(arg, field) {
                        return Some(ty);
                    }
                }
                None
            }
            // Method calls - check arguments
            HirExpr::MethodCall { args, .. } => {
                for arg in args {
                    if let Some(ty) = infer_from_expr(arg, field) {
                        return Some(ty);
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn infer_type_from_operand(expr: &HirExpr) -> Option<crate::hir::Type> {
        use crate::hir::Literal;
        match expr {
            HirExpr::Literal(Literal::Int(_)) => Some(crate::hir::Type::Int),
            HirExpr::Literal(Literal::Float(_)) => Some(crate::hir::Type::Float),
            // Binary with int/float on other side
            HirExpr::Binary { left, right, .. } => {
                infer_type_from_operand(left).or_else(|| infer_type_from_operand(right))
            }
            _ => None,
        }
    }

    fn infer_from_stmt(stmt: &HirStmt, field: &str) -> Option<crate::hir::Type> {
        match stmt {
            HirStmt::Assign { value, .. } => infer_from_expr(value, field),
            HirStmt::If { condition, then_body, else_body } => {
                if let Some(ty) = infer_from_expr(condition, field) {
                    return Some(ty);
                }
                for s in then_body {
                    if let Some(ty) = infer_from_stmt(s, field) {
                        return Some(ty);
                    }
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        if let Some(ty) = infer_from_stmt(s, field) {
                            return Some(ty);
                        }
                    }
                }
                None
            }
            HirStmt::Expr(expr) => infer_from_expr(expr, field),
            HirStmt::Return(Some(expr)) => infer_from_expr(expr, field),
            HirStmt::While { condition, body } => {
                if let Some(ty) = infer_from_expr(condition, field) {
                    return Some(ty);
                }
                for s in body {
                    if let Some(ty) = infer_from_stmt(s, field) {
                        return Some(ty);
                    }
                }
                None
            }
            HirStmt::For { iter, body, .. } => {
                if let Some(ty) = infer_from_expr(iter, field) {
                    return Some(ty);
                }
                for s in body {
                    if let Some(ty) = infer_from_stmt(s, field) {
                        return Some(ty);
                    }
                }
                None
            }
            _ => None,
        }
    }

    for stmt in body {
        if let Some(ty) = infer_from_stmt(stmt, field) {
            return Some(ty);
        }
    }
    None
}
