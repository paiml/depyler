//! Parameter type inference from usage patterns in function body
//!
//! Extracted from rust_gen::func_gen to break cross-crate dependency.
//! Used by both lifetime_analysis (depyler-analysis) and func_gen (depyler-core).

use depyler_hir::hir::{
    AssignTarget, BinOp, FStringPart, HirComprehension, HirExpr, HirStmt, Literal, Symbol, Type,
};

/// GH-70: Infer parameter type from usage patterns in function body
/// Detects patterns:
/// - `a, b, c = param` -> param is 3-tuple of strings
/// - `print(param)` -> param needs Display trait -> String
/// - `re.match(param, ...)` -> param is String
/// - Other usage patterns
pub fn infer_param_type_from_body(param_name: &str, body: &[HirStmt]) -> Option<Type> {
    for stmt in body {
        // Pattern 1: Tuple unpacking - `a, b, c = param`
        if let HirStmt::Assign {
            target,
            value: HirExpr::Var(var),
            type_annotation: _,
        } = stmt
        {
            if var == param_name {
                if let AssignTarget::Tuple(elements) = target {
                    let elem_types = vec![Type::String; elements.len()];
                    return Some(Type::Tuple(elem_types));
                }
            }
        }

        // Pattern 1b: Assignment where value is an expression using param
        if let HirStmt::Assign { value, .. } = stmt {
            if let Some(ty) = infer_type_from_expr_usage(param_name, value) {
                return Some(ty);
            }
        }

        // Pattern 2: Expression statement with print/println call
        if let HirStmt::Expr(expr) = stmt {
            if let Some(ty) = infer_type_from_expr_usage(param_name, expr) {
                return Some(ty);
            }
        }

        // Pattern 3: Return statement with expression using param
        if let HirStmt::Return(Some(expr)) = stmt {
            if let Some(ty) = infer_type_from_expr_usage(param_name, expr) {
                return Some(ty);
            }
        }

        // Pattern 4: If statement - check condition and body
        if let HirStmt::If {
            condition,
            then_body,
            else_body,
        } = stmt
        {
            if let Some(ty) = infer_type_from_expr_usage(param_name, condition) {
                return Some(ty);
            }
            if let Some(ty) = infer_param_type_from_body(param_name, then_body) {
                return Some(ty);
            }
            if let Some(else_stmts) = else_body {
                if let Some(ty) = infer_param_type_from_body(param_name, else_stmts) {
                    return Some(ty);
                }
            }
        }

        // Pattern 5: With statement - check body for parameter usage
        if let HirStmt::With { body, .. } = stmt {
            if let Some(ty) = infer_param_type_from_body(param_name, body) {
                return Some(ty);
            }
        }

        // Pattern 6: For loop - check body for parameter usage
        if let HirStmt::For { body, .. } = stmt {
            if let Some(ty) = infer_param_type_from_body(param_name, body) {
                return Some(ty);
            }
        }

        // Pattern 7: While loop - check condition and body
        if let HirStmt::While {
            condition, body, ..
        } = stmt
        {
            if let Some(ty) = infer_type_from_expr_usage(param_name, condition) {
                return Some(ty);
            }
            if let Some(ty) = infer_param_type_from_body(param_name, body) {
                return Some(ty);
            }
        }

        // Pattern 8: Try/except - check all bodies
        if let HirStmt::Try {
            body,
            handlers,
            orelse,
            finalbody,
        } = stmt
        {
            if let Some(ty) = infer_param_type_from_body(param_name, body) {
                return Some(ty);
            }
            for handler in handlers {
                if let Some(ty) = infer_param_type_from_body(param_name, &handler.body) {
                    return Some(ty);
                }
            }
            if let Some(else_stmts) = orelse {
                if let Some(ty) = infer_param_type_from_body(param_name, else_stmts) {
                    return Some(ty);
                }
            }
            if let Some(finally_stmts) = finalbody {
                if let Some(ty) = infer_param_type_from_body(param_name, finally_stmts) {
                    return Some(ty);
                }
            }
        }
    }
    None
}

fn infer_type_from_expr_usage(param_name: &str, expr: &HirExpr) -> Option<Type> {
    match expr {
        HirExpr::Call { func, args, kwargs } => infer_from_call_expr(param_name, func, args, kwargs),
        HirExpr::MethodCall {
            object,
            method,
            args,
            kwargs,
        } => infer_from_method_call(param_name, object, method, args, kwargs),
        HirExpr::FString { parts } => infer_from_fstring(param_name, parts),
        HirExpr::Index { base, index } => infer_from_index_expr(param_name, base, index),
        HirExpr::Slice { base, .. } => infer_from_slice_expr(param_name, base),
        HirExpr::Binary {
            left, right, op, ..
        } => infer_from_binary_expr(param_name, op, left, right),
        HirExpr::Unary { operand, .. } => infer_type_from_expr_usage(param_name, operand),
        HirExpr::ListComp {
            element,
            generators,
        } => infer_from_list_comp(param_name, element, generators),
        HirExpr::GeneratorExp {
            element,
            generators,
        } => infer_from_generator_exp(param_name, element, generators),
        _ => None,
    }
}

fn infer_from_call_expr(
    param_name: &str,
    func: &str,
    args: &[HirExpr],
    kwargs: &[(Symbol, HirExpr)],
) -> Option<Type> {
    if func == param_name {
        let param_types: Vec<Type> = args.iter().map(|_| Type::Int).collect();
        return Some(Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::Tuple(param_types), Type::Int],
        });
    }

    if func == "print" || func == "println" {
        for arg in args {
            if let HirExpr::Var(var_name) = arg {
                if var_name == param_name {
                    return Some(Type::String);
                }
            }
        }
    }

    if func.starts_with("re.") || func == "re" {
        for arg in args {
            if let HirExpr::Var(var_name) = arg {
                if var_name == param_name {
                    return Some(Type::String);
                }
            }
        }
    }

    if func == "subprocess.run" {
        for (kwarg_name, kwarg_value) in kwargs {
            if kwarg_name == "cwd" {
                if let HirExpr::Var(var_name) = kwarg_value {
                    if var_name == param_name {
                        return Some(Type::String);
                    }
                }
            }
        }
    }

    for arg in args {
        if let Some(ty) = infer_type_from_expr_usage(param_name, arg) {
            return Some(ty);
        }
    }
    for (_, kwarg_value) in kwargs {
        if let Some(ty) = infer_type_from_expr_usage(param_name, kwarg_value) {
            return Some(ty);
        }
    }
    None
}

fn infer_from_method_call(
    param_name: &str,
    object: &HirExpr,
    method: &str,
    args: &[HirExpr],
    kwargs: &[(Symbol, HirExpr)],
) -> Option<Type> {
    if let Some(ty) = infer_from_object_method(param_name, object, method) {
        return Some(ty);
    }

    if let Some(ty) = infer_from_module_method(param_name, object, method, args, kwargs) {
        return Some(ty);
    }

    if let Some(ty) = infer_from_string_arg_method(param_name, method, args) {
        return Some(ty);
    }

    for arg in args {
        if let Some(ty) = infer_type_from_expr_usage(param_name, arg) {
            return Some(ty);
        }
    }
    for (_, kwarg_value) in kwargs {
        if let Some(ty) = infer_type_from_expr_usage(param_name, kwarg_value) {
            return Some(ty);
        }
    }
    infer_type_from_expr_usage(param_name, object)
}

fn infer_from_object_method(
    param_name: &str,
    object: &HirExpr,
    method: &str,
) -> Option<Type> {
    let HirExpr::Var(var_name) = object else {
        return None;
    };
    if var_name != param_name {
        return None;
    }

    const FILE_METHODS: &[&str] = &[
        "write", "writelines", "read", "readline", "readlines",
        "flush", "close", "seek", "tell", "truncate",
    ];
    if FILE_METHODS.contains(&method) {
        return Some(Type::Custom("File".to_string()));
    }

    const STRING_METHODS: &[&str] = &[
        "strip", "lstrip", "rstrip", "startswith", "endswith", "split",
        "splitlines", "join", "upper", "lower", "title", "capitalize",
        "replace", "find", "rfind", "index", "rindex", "count",
        "isalpha", "isdigit", "isalnum", "isspace", "isupper", "islower",
        "encode", "format", "center", "ljust", "rjust", "zfill",
        "partition", "rpartition", "expandtabs", "swapcase", "casefold",
    ];
    if STRING_METHODS.contains(&method) {
        return Some(Type::String);
    }

    const DICT_METHODS: &[&str] = &[
        "get", "items", "keys", "values", "pop", "popitem",
        "update", "setdefault", "clear", "copy",
    ];
    if DICT_METHODS.contains(&method) {
        return Some(Type::Dict(Box::new(Type::String), Box::new(Type::String)));
    }

    None
}

fn infer_from_module_method(
    param_name: &str,
    object: &HirExpr,
    method: &str,
    args: &[HirExpr],
    kwargs: &[(Symbol, HirExpr)],
) -> Option<Type> {
    if let HirExpr::Var(module_name) = object {
        if let Some(ty) = infer_from_regex_module(param_name, module_name, method, args) {
            return Some(ty);
        }
        if let Some(ty) = infer_from_datetime_module(param_name, module_name, method, args) {
            return Some(ty);
        }
        if let Some(ty) = infer_from_subprocess_module(param_name, module_name, method, kwargs) {
            return Some(ty);
        }
    }

    if let HirExpr::Attribute { value, attr } = object {
        if let HirExpr::Var(module_name) = value.as_ref() {
            if module_name == "datetime" && attr == "datetime" && method == "fromtimestamp" {
                if let Some(HirExpr::Var(var_name)) = args.first() {
                    if var_name == param_name {
                        return Some(Type::Float);
                    }
                }
            }
        }
    }

    None
}

fn infer_from_regex_module(
    param_name: &str,
    module_name: &str,
    method: &str,
    args: &[HirExpr],
) -> Option<Type> {
    const REGEX_MODULES: &[&str] = &["re", "regex"];
    const REGEX_METHODS: &[&str] = &[
        "match", "search", "findall", "sub", "subn", "split", "compile",
    ];

    if !REGEX_MODULES.contains(&module_name) || !REGEX_METHODS.contains(&method) {
        return None;
    }

    for arg in args.iter().take(2) {
        if let HirExpr::Var(var_name) = arg {
            if var_name == param_name {
                return Some(Type::String);
            }
        }
    }
    None
}

fn infer_from_datetime_module(
    param_name: &str,
    module_name: &str,
    method: &str,
    args: &[HirExpr],
) -> Option<Type> {
    if module_name == "datetime" && method == "fromtimestamp" {
        if let Some(HirExpr::Var(var_name)) = args.first() {
            if var_name == param_name {
                return Some(Type::Float);
            }
        }
    }
    None
}

fn infer_from_subprocess_module(
    param_name: &str,
    module_name: &str,
    method: &str,
    kwargs: &[(Symbol, HirExpr)],
) -> Option<Type> {
    if module_name != "subprocess" || method != "run" {
        return None;
    }
    for (kwarg_name, kwarg_value) in kwargs {
        if kwarg_name == "cwd" {
            if let HirExpr::Var(var_name) = kwarg_value {
                if var_name == param_name {
                    return Some(Type::String);
                }
            }
        }
    }
    None
}

fn infer_from_string_arg_method(
    param_name: &str,
    method: &str,
    args: &[HirExpr],
) -> Option<Type> {
    const STRING_ARG_METHODS: &[&str] = &[
        "find", "search", "match", "sub", "replace", "replace_all",
        "is_match", "captures", "find_iter", "split", "strip",
        "lstrip", "rstrip", "startswith", "endswith", "contains",
        "encode", "decode",
    ];
    if !STRING_ARG_METHODS.contains(&method) {
        return None;
    }
    for arg in args {
        if let HirExpr::Var(var_name) = arg {
            if var_name == param_name {
                return Some(Type::String);
            }
        }
    }
    None
}

fn infer_from_fstring(param_name: &str, parts: &[FStringPart]) -> Option<Type> {
    for part in parts {
        if let FStringPart::Expr(val_expr) = part {
            if let HirExpr::Var(var_name) = val_expr.as_ref() {
                if var_name == param_name {
                    return Some(Type::String);
                }
            }
        }
    }
    None
}

fn infer_from_index_expr(
    param_name: &str,
    base: &HirExpr,
    index: &HirExpr,
) -> Option<Type> {
    if let HirExpr::Var(var_name) = base {
        if var_name == param_name {
            let is_string_key = matches!(
                index,
                HirExpr::Literal(Literal::String(_)) | HirExpr::FString { .. }
            );
            let is_likely_string_key = if let HirExpr::Var(idx_name) = index {
                idx_name == "key" || idx_name == "k" || idx_name.ends_with("_key")
            } else {
                false
            };

            if is_string_key || is_likely_string_key {
                return Some(Type::Dict(
                    Box::new(Type::String),
                    Box::new(Type::Custom("serde_json::Value".to_string())),
                ));
            }
            return Some(Type::List(Box::new(Type::Int)));
        }
    }
    infer_type_from_expr_usage(param_name, base)
}

fn infer_from_slice_expr(param_name: &str, base: &HirExpr) -> Option<Type> {
    if let HirExpr::Var(var_name) = base {
        if var_name == param_name {
            return Some(Type::String);
        }
    }
    infer_type_from_expr_usage(param_name, base)
}

fn infer_from_binary_expr(
    param_name: &str,
    op: &BinOp,
    left: &HirExpr,
    right: &HirExpr,
) -> Option<Type> {
    if let Some(ty) = infer_from_equality_op(param_name, op, left, right) {
        return Some(ty);
    }
    if let Some(ty) = infer_from_logical_op(param_name, op, left, right) {
        return Some(ty);
    }
    if let Some(ty) = infer_from_membership_op(param_name, op, left, right) {
        return Some(ty);
    }
    if let Some(ty) = infer_from_arithmetic_op(param_name, op, left, right) {
        return Some(ty);
    }
    infer_type_from_expr_usage(param_name, left)
        .or_else(|| infer_type_from_expr_usage(param_name, right))
}

fn infer_from_equality_op(
    param_name: &str,
    op: &BinOp,
    left: &HirExpr,
    right: &HirExpr,
) -> Option<Type> {
    if !matches!(op, BinOp::Eq | BinOp::NotEq) {
        return None;
    }
    if let HirExpr::Var(var_name) = left {
        if var_name == param_name
            && matches!(right, HirExpr::Literal(Literal::String(_)))
        {
            return Some(Type::String);
        }
    }
    if let HirExpr::Var(var_name) = right {
        if var_name == param_name
            && matches!(left, HirExpr::Literal(Literal::String(_)))
        {
            return Some(Type::String);
        }
    }
    None
}

fn infer_from_logical_op(
    param_name: &str,
    op: &BinOp,
    left: &HirExpr,
    right: &HirExpr,
) -> Option<Type> {
    if !matches!(op, BinOp::And | BinOp::Or) {
        return None;
    }
    if let HirExpr::Var(var_name) = left {
        if var_name == param_name {
            return Some(Type::Bool);
        }
    }
    if let HirExpr::Var(var_name) = right {
        if var_name == param_name {
            return Some(Type::Bool);
        }
    }
    None
}

fn infer_from_membership_op(
    param_name: &str,
    op: &BinOp,
    left: &HirExpr,
    right: &HirExpr,
) -> Option<Type> {
    if !matches!(op, BinOp::In | BinOp::NotIn) {
        return None;
    }
    if let HirExpr::Var(var_name) = left {
        if var_name == param_name {
            if let HirExpr::List(elements) = right {
                if elements
                    .iter()
                    .all(|e| matches!(e, HirExpr::Literal(Literal::String(_))))
                {
                    return Some(Type::String);
                }
            }
            return Some(Type::String);
        }
    }
    None
}

fn infer_from_arithmetic_op(
    param_name: &str,
    op: &BinOp,
    left: &HirExpr,
    right: &HirExpr,
) -> Option<Type> {
    if !matches!(
        op,
        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::FloorDiv | BinOp::Mod
    ) {
        return None;
    }
    if let HirExpr::Var(var_name) = left {
        if var_name == param_name {
            return Some(Type::Int);
        }
    }
    if let HirExpr::Var(var_name) = right {
        if var_name == param_name {
            return Some(Type::Int);
        }
    }
    None
}

fn infer_from_list_comp(
    param_name: &str,
    element: &HirExpr,
    generators: &[HirComprehension],
) -> Option<Type> {
    if let Some(ty) = infer_type_from_expr_usage(param_name, element) {
        return Some(ty);
    }
    for gen in generators {
        if let HirExpr::Var(var_name) = &*gen.iter {
            if var_name == param_name {
                return Some(Type::String);
            }
        }
        for cond in &gen.conditions {
            if let Some(ty) = infer_type_from_expr_usage(param_name, cond) {
                return Some(ty);
            }
        }
    }
    None
}

fn infer_from_generator_exp(
    param_name: &str,
    element: &HirExpr,
    generators: &[HirComprehension],
) -> Option<Type> {
    if let Some(ty) = infer_type_from_expr_usage(param_name, element) {
        return Some(ty);
    }
    for gen in generators {
        for cond in &gen.conditions {
            if let Some(ty) = infer_type_from_expr_usage(param_name, cond) {
                return Some(ty);
            }
        }
    }
    None
}
