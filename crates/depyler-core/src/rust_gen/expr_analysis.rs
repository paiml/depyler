//! Expression Analysis Helpers for Code Generation
//!
//! This module contains pure functions for analyzing HIR expressions.
//! These are extracted from stmt_gen.rs for better testability and reuse.
//!
//! PMAT Strategy: 100% unit test coverage for these pure functions.

use crate::hir::{BinOp, HirExpr, Literal, Type};
use crate::rust_gen::context::CodeGenContext;

/// Check if an HIR expression returns usize (needs cast to i32)
///
/// DEPYLER-0272: Only add casts for expressions that actually return usize.
/// This prevents unnecessary casts like `(a: i32) as i32`.
pub fn expr_returns_usize(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::MethodCall { method, .. } => {
            matches!(method.as_str(), "len" | "count" | "capacity")
        }
        HirExpr::Call { func, .. } => {
            matches!(func.as_str(), "len" | "range")
        }
        HirExpr::Binary { left, right, .. } => {
            expr_returns_usize(left) || expr_returns_usize(right)
        }
        _ => false,
    }
}

/// DEPYLER-0520: Check if an expression produces an iterator (not a collection)
pub fn is_iterator_producing_expr(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::GeneratorExp { .. } => true,
        HirExpr::MethodCall { method, object, .. } => {
            let is_iterator_method = matches!(
                method.as_str(),
                "iter"
                    | "iter_mut"
                    | "into_iter"
                    | "map"
                    | "filter"
                    | "filter_map"
                    | "flat_map"
                    | "enumerate"
                    | "zip"
                    | "chain"
                    | "take"
                    | "skip"
                    | "take_while"
                    | "skip_while"
                    | "peekable"
                    | "fuse"
                    | "inspect"
                    | "by_ref"
                    | "rev"
                    | "cycle"
            );
            is_iterator_method || is_iterator_producing_expr(object)
        }
        HirExpr::Call { func, .. } => {
            matches!(
                func.as_str(),
                "iter" | "map" | "filter" | "enumerate" | "zip" | "reversed"
            )
        }
        _ => false,
    }
}

/// DEPYLER-0785: Check if an HIR expression returns float type
pub fn expr_infers_float(expr: &HirExpr, ctx: &CodeGenContext) -> bool {
    match expr {
        HirExpr::Literal(Literal::Float(_)) => true,
        HirExpr::Var(name) => {
            matches!(ctx.var_types.get(name), Some(Type::Float))
        }
        HirExpr::Call { func, .. } => {
            if matches!(ctx.function_return_types.get(func), Some(Type::Float)) {
                return true;
            }
            if let Some(Type::Function { ret, .. }) = ctx.var_types.get(func) {
                return matches!(ret.as_ref(), Type::Float);
            }
            if let Some(Type::Generic { base, params }) = ctx.var_types.get(func) {
                if base == "Callable" && params.len() == 2 {
                    return matches!(params[1], Type::Float);
                }
            }
            false
        }
        HirExpr::Binary { op, left, right } => {
            matches!(
                op,
                BinOp::Mul | BinOp::Div | BinOp::Add | BinOp::Sub | BinOp::Mod | BinOp::Pow
            ) && (expr_infers_float(left, ctx) || expr_infers_float(right, ctx))
        }
        HirExpr::Unary { operand, .. } => expr_infers_float(operand, ctx),
        HirExpr::IfExpr { body, orelse, .. } => {
            expr_infers_float(body, ctx) && expr_infers_float(orelse, ctx)
        }
        _ => false,
    }
}

/// DEPYLER-0932: Check if an expression produces a numpy/trueno Vector value
/// DEPYLER-1044: Does not flag abs(scalar), sqrt(scalar), etc. as numpy
pub fn is_numpy_value_expr(expr: &HirExpr, ctx: &CodeGenContext) -> bool {
    match expr {
        // These always create vectors
        HirExpr::Call { func, .. }
            if matches!(
                func.as_str(),
                "array" | "zeros" | "ones" | "empty" | "linspace" | "arange" | "full" | "copy"
            ) =>
        {
            true
        }
        // DEPYLER-1044: abs, sqrt, etc. return vector ONLY if argument is vector
        // abs(scalar) -> scalar, abs(array) -> array
        HirExpr::Call { func, args, .. }
            if matches!(
                func.as_str(),
                "abs" | "sqrt" | "sin" | "cos" | "exp" | "log" | "clip" | "clamp" | "normalize"
            ) =>
        {
            args.first()
                .is_some_and(|arg| is_numpy_value_expr(arg, ctx))
        }
        // DEPYLER-1044: Method calls preserve numpy nature of object
        HirExpr::MethodCall { object, method, .. }
            if matches!(
                method.as_str(),
                "abs"
                    | "sqrt"
                    | "sin"
                    | "cos"
                    | "exp"
                    | "log"
                    | "clip"
                    | "clamp"
                    | "unwrap"
                    | "scale"
            ) =>
        {
            is_numpy_value_expr(object, ctx)
        }
        // These method calls always create vectors
        HirExpr::MethodCall { method, .. }
            if matches!(method.as_str(), "array" | "zeros" | "ones") =>
        {
            true
        }
        HirExpr::Binary { left, right, .. } => {
            is_numpy_value_expr(left, ctx) || is_numpy_value_expr(right, ctx)
        }
        HirExpr::IfExpr { body, orelse, .. } => {
            is_numpy_value_expr(body, ctx) || is_numpy_value_expr(orelse, ctx)
        }
        HirExpr::Var(name) => {
            ctx.numpy_vars.contains(name)
                || matches!(
                    name.as_str(),
                    "arr" | "array" | "data" | "values" | "vec" | "vector"
                )
        }
        _ => false,
    }
}

/// DEPYLER-1135: Check if expression returns f64 (needs cast to i32 when target is Int)
/// Detects numpy operations that always return f64 after numeric promotion
pub fn expr_returns_f64(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::MethodCall { method, object, .. } => {
            // DEPYLER-1135: Numpy array aggregation methods return f64
            let is_numpy_agg = matches!(
                method.as_str(),
                "sum" | "mean" | "std" | "var" | "min" | "max" | "prod" | "dot" | "norm"
            );
            // Check for iterator methods that produce f64
            let is_f64_iter = method == "sum" && {
                // Check if chain includes map to f64
                match object.as_ref() {
                    HirExpr::MethodCall {
                        method: inner_method,
                        ..
                    } => inner_method == "map" || inner_method == "iter",
                    _ => false,
                }
            };
            is_numpy_agg || is_f64_iter
        }
        HirExpr::Call { func, .. } => {
            // DEPYLER-1135: Module-qualified numpy calls that return f64
            let func_parts: Vec<&str> = func.split('.').collect();
            if func_parts.len() >= 2 {
                let module = func_parts[0];
                let method = func_parts[func_parts.len() - 1];
                if matches!(module, "np" | "numpy") {
                    return matches!(
                        method,
                        "sum" | "mean" | "std" | "var" | "min" | "max" | "prod" | "dot" | "norm"
                    );
                }
            }
            false
        }
        // Propagate through parenthesized expressions
        HirExpr::Unary { operand, .. } => expr_returns_f64(operand),
        _ => false,
    }
}

/// Check if a type annotation requires explicit conversion
pub fn needs_type_conversion(target_type: &Type, expr: &HirExpr) -> bool {
    match target_type {
        // DEPYLER-1135: Extended to check f64-returning numpy operations
        Type::Int => expr_returns_usize(expr) || expr_returns_f64(expr),
        Type::String => matches!(expr, HirExpr::Var(_)),
        _ => false,
    }
}

/// Check if expression is a pure expression (no side effects)
pub fn is_pure_expression(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Literal(_) | HirExpr::Var(_) => true,
        HirExpr::Binary { left, right, .. } => {
            is_pure_expression(left) && is_pure_expression(right)
        }
        HirExpr::Unary { operand, .. } => is_pure_expression(operand),
        HirExpr::Tuple(elements) => elements.iter().all(is_pure_expression),
        HirExpr::List(elements) => elements.iter().all(is_pure_expression),
        HirExpr::Attribute { value, .. } => is_pure_expression(value),
        HirExpr::Call { .. } | HirExpr::MethodCall { .. } => false,
        _ => false,
    }
}

/// Check if expression looks like it produces an Option
///
/// Detects patterns like:
/// - dict.get(key) without default → Option
/// - result.ok() → Option
/// - Chained calls like std::env::var(...).ok()
/// - Collection methods like first/last/pop/next
///
/// DEPYLER-0455: Enhanced to detect chained method calls
/// DEPYLER-0632: dict.get(key, default) with default returns concrete type, not Option
pub fn looks_like_option_expr(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::MethodCall {
            method,
            object,
            args,
            ..
        } => {
            // Method call ending in .ok() → definitely Option
            if method == "ok" {
                return true;
            }
            // .get() only returns Option when no default value provided
            if method == "get" && args.len() == 1 {
                return true;
            }
            // Collection methods that return Option
            if matches!(method.as_str(), "first" | "last" | "pop" | "next") {
                return true;
            }
            // DEPYLER-1071: Regex methods that return Option<Match>
            // Python: m = re.match(pattern, text) / pattern.match(text)
            // Rust: Option<Match> or Option<DepylerRegexMatch>
            if matches!(method.as_str(), "match_" | "search" | "fullmatch") {
                return true;
            }
            // Recursively check if the object is an Option-returning expression
            looks_like_option_expr(object)
        }
        HirExpr::Call { func, .. } => matches!(func.as_str(), "next"),
        _ => false,
    }
}

/// DEPYLER-COVERAGE-95: Check if expression creates a file handle
///
/// Detects patterns like:
/// - open("file.txt")
/// - File::create("file.txt")
/// - File::open("file.txt")
pub fn is_file_creating_expr(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Call { func, .. } => func == "open",
        HirExpr::MethodCall { object, method, .. } => {
            if method == "create" || method == "open" {
                if let HirExpr::Var(name) = object.as_ref() {
                    return name == "File";
                }
                if let HirExpr::Attribute { attr, .. } = object.as_ref() {
                    return attr == "File";
                }
            }
            false
        }
        _ => false,
    }
}

/// DEPYLER-0625: Check if expression is sys.stdout or sys.stderr
pub fn is_stdio_expr(expr: &HirExpr) -> bool {
    if let HirExpr::Attribute { value, attr } = expr {
        if attr == "stdout" || attr == "stderr" {
            if let HirExpr::Var(name) = value.as_ref() {
                return name == "sys";
            }
        }
    }
    false
}

/// DEPYLER-COVERAGE-95: Extract string literal from expression
pub fn extract_string_literal(expr: &HirExpr) -> String {
    match expr {
        HirExpr::Literal(Literal::String(s)) => s.clone(),
        _ => String::new(),
    }
}

/// DEPYLER-0399: Extract string value from kwarg by name
pub fn extract_kwarg_string(kwargs: &[(String, HirExpr)], key: &str) -> Option<String> {
    kwargs
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| match v {
            HirExpr::Literal(Literal::String(s)) => Some(s.clone()),
            _ => None,
        })
}

/// DEPYLER-0399: Extract boolean value from kwarg by name
pub fn extract_kwarg_bool(kwargs: &[(String, HirExpr)], key: &str) -> Option<bool> {
    kwargs
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| match v {
            HirExpr::Var(s) if s == "True" => Some(true),
            HirExpr::Var(s) if s == "False" => Some(false),
            _ => None,
        })
}

/// DEPYLER-E0282-FIX: Check if expression contains chained PyOps (binary operations)
/// When we have chains like ((a).py_add(b)).py_add(c), Rust can't infer intermediate types.
/// This detects expressions with 2+ nested Binary operations using arithmetic operators.
/// Also looks through Call wrappers like Ok(...) and Some(...) to find nested chains.
pub fn has_chained_pyops(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Binary { left, op, right } => {
            // Check if this is an arithmetic operation (PyOps eligible)
            let is_arithmetic = matches!(
                op,
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod
            );
            if !is_arithmetic {
                return false;
            }
            // Check if either operand is also a Binary arithmetic expression (chain)
            let left_is_binary = matches!(left.as_ref(), HirExpr::Binary { op, .. } if matches!(op, BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod));
            let right_is_binary = matches!(right.as_ref(), HirExpr::Binary { op, .. } if matches!(op, BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod));
            left_is_binary || right_is_binary
        }
        // Look through Call wrappers like Ok(...), Some(...), Result::Ok(...)
        HirExpr::Call { args, .. } => args.iter().any(has_chained_pyops),
        // Look through tuples
        HirExpr::Tuple(elements) => elements.iter().any(has_chained_pyops),
        _ => false,
    }
}

/// DEPYLER-E0282-FIX: Extract inner type from Optional<T>
/// For returns with chained PyOps wrapped in Some, we need to know the inner type T.
/// Note: HIR Type enum doesn't have a Result variant (yet), only Optional.
#[allow(dead_code)]
pub fn get_inner_optional_type(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Optional(inner) => Some(inner.as_ref()),
        _ => None,
    }
}

/// DEPYLER-E0282-FIX: Check if expression is Ok(expr) or Some(expr) with chained PyOps inside
/// Returns the inner binary expression if it's wrapped and has chained ops
pub fn get_wrapped_chained_pyops(expr: &HirExpr) -> Option<&HirExpr> {
    match expr {
        HirExpr::Call { func, args, .. } if (func == "Ok" || func == "Some") && args.len() == 1 => {
            let inner = &args[0];
            if is_direct_chained_pyops(inner) {
                Some(inner)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Check if this is directly a chained binary arithmetic expression (not through Call)
fn is_direct_chained_pyops(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Binary { left, op, right } => {
            let is_arithmetic = matches!(
                op,
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod
            );
            if !is_arithmetic {
                return false;
            }
            let left_is_binary = matches!(left.as_ref(), HirExpr::Binary { op, .. } if matches!(op, BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod));
            let right_is_binary = matches!(right.as_ref(), HirExpr::Binary { op, .. } if matches!(op, BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod));
            left_is_binary || right_is_binary
        }
        _ => false,
    }
}

/// DEPYLER-0943: Check if expression is a dict/HashMap index access
/// Dict subscript like `config["name"]` returns serde_json::Value, which needs
/// conversion when returning as String.
pub fn is_dict_index_access(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Index { base, index } => {
            // String key indicates dict access (not list/array)
            let has_string_key = matches!(index.as_ref(), HirExpr::Literal(Literal::String(_)));
            if has_string_key {
                return true;
            }
            // Check if base is a known dict-like variable
            if let HirExpr::Var(name) = base.as_ref() {
                let n = name.as_str();
                return n.contains("dict")
                    || n.contains("config")
                    || n.contains("data")
                    || n.contains("settings")
                    || n.contains("params")
                    || n.contains("options")
                    || n.contains("env")
                    || n.contains("json")
                    || n == "d"
                    || n == "m";
            }
            false
        }
        _ => false,
    }
}

/// DEPYLER-0359: Check if an expression contains floor division operation
pub fn contains_floor_div(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Binary {
            op: BinOp::FloorDiv,
            ..
        } => true,
        HirExpr::Binary { left, right, .. } => {
            contains_floor_div(left) || contains_floor_div(right)
        }
        HirExpr::Unary { operand, .. } => contains_floor_div(operand),
        HirExpr::Call { args, .. } => args.iter().any(contains_floor_div),
        HirExpr::MethodCall { object, args, .. } => {
            contains_floor_div(object) || args.iter().any(contains_floor_div)
        }
        HirExpr::Index { base, index } => contains_floor_div(base) || contains_floor_div(index),
        HirExpr::List(elements) | HirExpr::Tuple(elements) | HirExpr::Set(elements) => {
            elements.iter().any(contains_floor_div)
        }
        _ => false,
    }
}

/// DEPYLER-0819: Check if handler body ends with an exit call
pub fn handler_ends_with_exit(handler_body: &[crate::hir::HirStmt]) -> bool {
    use crate::hir::HirStmt;
    if let Some(last_stmt) = handler_body.last() {
        match last_stmt {
            HirStmt::Expr(HirExpr::Call { func, .. }) => func == "exit" || func == "sys.exit",
            HirStmt::Expr(HirExpr::MethodCall { object, method, .. }) => {
                if let HirExpr::Var(module) = &**object {
                    module == "sys" && method == "exit"
                } else {
                    false
                }
            }
            _ => false,
        }
    } else {
        false
    }
}

/// DEPYLER-0819: Check if handler body contains a raise statement
pub fn handler_contains_raise(handler_body: &[crate::hir::HirStmt]) -> bool {
    use crate::hir::HirStmt;
    handler_body
        .iter()
        .any(|stmt| matches!(stmt, HirStmt::Raise { .. }))
}

/// DEPYLER-0399: Convert string to PascalCase for enum variants
pub fn to_pascal_case(s: &str) -> String {
    s.split(&['-', '_'][..])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// DEPYLER-0607: Check if a dict has Value type values (explicit Value or Unknown)
pub fn is_dict_with_value_type(t: &crate::hir::Type) -> bool {
    use crate::hir::Type;
    match t {
        Type::Dict(_, v) => {
            matches!(v.as_ref(),
                Type::Custom(n) if n.contains("Value") || n.contains("json"))
                || matches!(v.as_ref(), Type::Unknown)
        }
        _ => false,
    }
}

/// DEPYLER-0749: Check if this is a dict augmented assignment pattern (dict[key] op= value)
/// Returns true if target is Index and value is Binary with left being an Index to same location
pub fn is_dict_augassign_pattern(target: &crate::hir::AssignTarget, value: &HirExpr) -> bool {
    use crate::hir::AssignTarget;
    if let AssignTarget::Index {
        base: target_base,
        index: target_index,
    } = target
    {
        if let HirExpr::Binary { left, .. } = value {
            if let HirExpr::Index {
                base: value_base,
                index: value_index,
            } = left.as_ref()
            {
                // Check if both indices refer to the same dict[key] location
                return matches!((target_base.as_ref(), value_base.as_ref()),
                    (HirExpr::Var(t_var), HirExpr::Var(v_var)) if t_var == v_var)
                    && matches!((target_index.as_ref(), value_index.as_ref()),
                        (HirExpr::Var(t_idx), HirExpr::Var(v_idx)) if t_idx == v_idx);
            }
        }
    }
    false
}

/// DEPYLER-0790: Check if a nested function is recursive (calls itself)
/// Returns true if the function body contains a call to the function itself
pub fn is_nested_function_recursive(name: &str, body: &[crate::hir::HirStmt]) -> bool {
    use crate::hir::{HirExpr, HirStmt};

    fn check_expr(expr: &HirExpr, name: &str) -> bool {
        match expr {
            // Direct call to the function by name - func is Symbol, not Box<HirExpr>
            HirExpr::Call { func, args, kwargs } => {
                if func == name {
                    return true;
                }
                // Check args and kwargs recursively
                args.iter().any(|a| check_expr(a, name))
                    || kwargs.iter().any(|(_, v)| check_expr(v, name))
            }
            // Dynamic call - check callee expression
            HirExpr::DynamicCall {
                callee,
                args,
                kwargs,
            } => {
                check_expr(callee, name)
                    || args.iter().any(|a| check_expr(a, name))
                    || kwargs.iter().any(|(_, v)| check_expr(v, name))
            }
            // Recurse into all expression types
            HirExpr::Binary { left, right, .. } => {
                check_expr(left, name) || check_expr(right, name)
            }
            HirExpr::Unary { operand, .. } => check_expr(operand, name),
            HirExpr::MethodCall {
                object,
                args,
                kwargs,
                ..
            } => {
                check_expr(object, name)
                    || args.iter().any(|a| check_expr(a, name))
                    || kwargs.iter().any(|(_, v)| check_expr(v, name))
            }
            HirExpr::Attribute { value, .. } => check_expr(value, name),
            HirExpr::Index { base, index } => check_expr(base, name) || check_expr(index, name),
            HirExpr::IfExpr { test, body, orelse } => {
                check_expr(test, name) || check_expr(body, name) || check_expr(orelse, name)
            }
            HirExpr::List(items)
            | HirExpr::Tuple(items)
            | HirExpr::Set(items)
            | HirExpr::FrozenSet(items) => items.iter().any(|i| check_expr(i, name)),
            HirExpr::Dict(pairs) => pairs
                .iter()
                .any(|(k, v)| check_expr(k, name) || check_expr(v, name)),
            HirExpr::ListComp {
                element,
                generators,
            }
            | HirExpr::SetComp {
                element,
                generators,
            }
            | HirExpr::GeneratorExp {
                element,
                generators,
            } => {
                check_expr(element, name)
                    || generators.iter().any(|g| {
                        check_expr(&g.iter, name)
                            || g.conditions.iter().any(|c| check_expr(c, name))
                    })
            }
            HirExpr::DictComp {
                key,
                value,
                generators,
            } => {
                check_expr(key, name)
                    || check_expr(value, name)
                    || generators.iter().any(|g| {
                        check_expr(&g.iter, name)
                            || g.conditions.iter().any(|c| check_expr(c, name))
                    })
            }
            HirExpr::Lambda { body, .. } => check_expr(body, name),
            HirExpr::Await { value } => check_expr(value, name),
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => {
                check_expr(base, name)
                    || start.as_ref().is_some_and(|e| check_expr(e, name))
                    || stop.as_ref().is_some_and(|e| check_expr(e, name))
                    || step.as_ref().is_some_and(|e| check_expr(e, name))
            }
            HirExpr::Borrow { expr, .. } => check_expr(expr, name),
            HirExpr::FString { parts } => parts.iter().any(|p| {
                if let crate::hir::FStringPart::Expr(e) = p {
                    check_expr(e, name)
                } else {
                    false
                }
            }),
            HirExpr::Yield { value } => value.as_ref().is_some_and(|e| check_expr(e, name)),
            HirExpr::SortByKey {
                iterable,
                key_body,
                reverse_expr,
                ..
            } => {
                check_expr(iterable, name)
                    || check_expr(key_body, name)
                    || reverse_expr.as_ref().is_some_and(|e| check_expr(e, name))
            }
            HirExpr::NamedExpr { value, .. } => check_expr(value, name),
            _ => false,
        }
    }

    fn check_stmt(stmt: &HirStmt, name: &str) -> bool {
        match stmt {
            HirStmt::Expr(expr) | HirStmt::Return(Some(expr)) => check_expr(expr, name),
            HirStmt::Assign { value, .. } => check_expr(value, name),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                check_expr(condition, name)
                    || then_body.iter().any(|s| check_stmt(s, name))
                    || else_body
                        .as_ref()
                        .is_some_and(|b| b.iter().any(|s| check_stmt(s, name)))
            }
            HirStmt::While { condition, body } => {
                check_expr(condition, name) || body.iter().any(|s| check_stmt(s, name))
            }
            HirStmt::For { iter, body, .. } => {
                check_expr(iter, name) || body.iter().any(|s| check_stmt(s, name))
            }
            HirStmt::With { context, body, .. } => {
                check_expr(context, name) || body.iter().any(|s| check_stmt(s, name))
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                body.iter().any(|s| check_stmt(s, name))
                    || handlers
                        .iter()
                        .any(|h| h.body.iter().any(|s| check_stmt(s, name)))
                    || orelse
                        .as_ref()
                        .is_some_and(|b| b.iter().any(|s| check_stmt(s, name)))
                    || finalbody
                        .as_ref()
                        .is_some_and(|b| b.iter().any(|s| check_stmt(s, name)))
            }
            HirStmt::FunctionDef { body, .. } => body.iter().any(|s| check_stmt(s, name)),
            HirStmt::Block(stmts) => stmts.iter().any(|s| check_stmt(s, name)),
            HirStmt::Assert { test, msg } => {
                check_expr(test, name) || msg.as_ref().is_some_and(|m| check_expr(m, name))
            }
            HirStmt::Raise { exception, cause } => {
                exception.as_ref().is_some_and(|e| check_expr(e, name))
                    || cause.as_ref().is_some_and(|c| check_expr(c, name))
            }
            _ => false,
        }
    }

    body.iter().any(|stmt| check_stmt(stmt, name))
}

/// DEPYLER-0360: Extract the divisor (right operand) from a floor division expression
pub fn extract_divisor_from_floor_div(expr: &HirExpr) -> anyhow::Result<&HirExpr> {
    use crate::hir::BinOp;
    use anyhow::bail;
    match expr {
        HirExpr::Binary {
            op: BinOp::FloorDiv,
            right,
            ..
        } => Ok(right),
        HirExpr::Binary { left, right, .. } => {
            // Recursively search for floor division
            if contains_floor_div(left) {
                extract_divisor_from_floor_div(left)
            } else if contains_floor_div(right) {
                extract_divisor_from_floor_div(right)
            } else {
                bail!("No floor division found in expression")
            }
        }
        HirExpr::Unary { operand, .. } => extract_divisor_from_floor_div(operand),
        _ => bail!("No floor division found in expression"),
    }
}

/// DEPYLER-1054: Check if an expression produces a DepylerValue
/// This occurs when:
/// 1. Subscript access on a dict with Unknown value type (heterogeneous)
/// 2. Variable typed as Type::Unknown in NASA mode
/// 3. Arithmetic operations on DepylerValue operands
pub fn expr_produces_depyler_value(
    expr: &HirExpr,
    ctx: &crate::rust_gen::context::CodeGenContext,
) -> bool {
    use crate::hir::Type;

    // Not in NASA mode - DepylerValue is not used
    if !ctx.type_mapper.nasa_mode {
        return false;
    }

    match expr {
        // Index/subscript on a dict with Unknown/Any value type produces DepylerValue
        // DEPYLER-1054: Also check for Type::Custom("Any") after DEPYLER-0725 fix
        HirExpr::Index { base, .. } => {
            if let Some(Type::Dict(_, val_type)) = get_expr_type(base, ctx) {
                return is_depyler_value_type(val_type.as_ref());
            }
            false
        }
        // Variable with Unknown/Any type is DepylerValue
        HirExpr::Var(name) => ctx.var_types.get(name).is_some_and(is_depyler_value_type),
        // Arithmetic on DepylerValue produces DepylerValue
        HirExpr::Binary { left, right: _, op } => {
            use crate::hir::BinOp;
            // Skip comparison operators - they return bool
            if matches!(
                op,
                BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq
                    | BinOp::In
                    | BinOp::NotIn
            ) {
                return false;
            }
            // DEPYLER-E0599-FIX: Only check LEFT operand's type
            // The result type depends on which type the trait is implemented on:
            // - DepylerValue.py_add(anything) -> DepylerValue
            // - i32.py_add(DepylerValue) -> i64 (NOT DepylerValue!)
            // So result is DepylerValue IFF left operand produces DepylerValue
            expr_produces_depyler_value(left, ctx)
        }
        HirExpr::Unary { operand, .. } => expr_produces_depyler_value(operand, ctx),
        // DEPYLER-1064: Function call returning Unknown/Any produces DepylerValue
        // Note: Tuple[Any, ...] produces a NATIVE tuple, not DepylerValue, so it's handled separately
        // by is_native_depyler_tuple()
        HirExpr::Call { func, .. } => {
            // Check if function has a return type that is Unknown/Any
            // Tuple types are NOT DepylerValue - they're native tuples of DepylerValue elements
            if let Some(ret_type) = ctx.function_return_types.get(func) {
                match ret_type {
                    Type::Unknown => true,
                    Type::Custom(name) if name == "Any" || name == "object" => true,
                    // Tuple[Any, ...] produces native tuple (DepylerValue, ...), NOT DepylerValue
                    // These are handled by is_native_depyler_tuple() instead
                    Type::Tuple(_) => false,
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

/// DEPYLER-1064: Check if expression produces a NATIVE tuple of DepylerValue elements
///
/// This is different from `expr_produces_depyler_value` which checks if the expression
/// produces a DepylerValue directly. This checks specifically for Tuple[Any, ...] return types
/// which become native Rust tuples `(DepylerValue, DepylerValue, ...)` rather than DepylerValue::Tuple.
///
/// For native tuples, we use positional access (.0, .1, .2) instead of get_tuple_elem().
pub fn is_native_depyler_tuple(
    expr: &HirExpr,
    ctx: &crate::rust_gen::context::CodeGenContext,
) -> bool {
    use crate::hir::Type;

    // Not in NASA mode - DepylerValue is not used
    if !ctx.type_mapper.nasa_mode {
        return false;
    }

    match expr {
        // Function call returning Tuple[Any, ...] produces native tuple
        HirExpr::Call { func, .. } => {
            if let Some(Type::Tuple(elems)) = ctx.function_return_types.get(func) {
                // Check if tuple contains DepylerValue types
                return elems.iter().any(is_depyler_value_type);
            }
            false
        }
        // Variable with Tuple[Any, ...] type
        HirExpr::Var(name) => {
            if let Some(Type::Tuple(elems)) = ctx.var_types.get(name) {
                return elems.iter().any(is_depyler_value_type);
            }
            false
        }
        _ => false,
    }
}

/// DEPYLER-1054: Get expression type from context
fn get_expr_type(
    expr: &HirExpr,
    ctx: &crate::rust_gen::context::CodeGenContext,
) -> Option<crate::hir::Type> {
    match expr {
        HirExpr::Var(name) => ctx.var_types.get(name).cloned(),
        _ => None,
    }
}

/// DEPYLER-1054: Check if a type represents DepylerValue
/// Matches Type::Unknown and Type::Custom("Any"/"object") which become DepylerValue in NASA mode
fn is_depyler_value_type(ty: &crate::hir::Type) -> bool {
    use crate::hir::Type;
    matches!(ty, Type::Unknown) || matches!(ty, Type::Custom(s) if s == "Any" || s == "object")
}

/// DEPYLER-1054: Get the extraction call for a target type
/// Returns the method to call on DepylerValue to extract the concrete value
pub fn get_depyler_extraction_for_type(target_type: &crate::hir::Type) -> Option<&'static str> {
    use crate::hir::Type;
    match target_type {
        Type::Int => Some(".to_i64() as i32"),
        Type::Float => Some(".to_f64()"),
        Type::String => Some(".to_string()"),
        Type::Bool => Some(".to_bool()"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::UnaryOp;

    // ============ expr_returns_usize tests ============

    #[test]
    fn test_expr_returns_usize_len_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_count_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "count".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_capacity_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("vec".to_string())),
            method: "capacity".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_other_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "strip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_len_call() {
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_range_call() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(10))],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_other_call() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_binary_left() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("vec".to_string())),
                method: "len".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_binary_right() {
        let expr = HirExpr::Binary {
            op: BinOp::Sub,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("arr".to_string())],
                kwargs: vec![],
            }),
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_binary_neither() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(!expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_var() {
        assert!(!expr_returns_usize(&HirExpr::Var("x".to_string())));
    }

    #[test]
    fn test_expr_returns_usize_literal() {
        assert!(!expr_returns_usize(&HirExpr::Literal(Literal::Int(42))));
    }

    // ============ is_iterator_producing_expr tests ============

    #[test]
    fn test_is_iterator_gen_expr() {
        let expr = HirExpr::GeneratorExp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_iter_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "iter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_map_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "map".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_filter_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "filter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_enumerate_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "enumerate".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_zip_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "zip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_chain_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "chain".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_take_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "take".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_skip_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "skip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_rev_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "rev".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_cycle_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "cycle".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_chained() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("items".to_string())),
                method: "iter".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            method: "collect".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_non_iterator_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_call_iter() {
        let expr = HirExpr::Call {
            func: "iter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_call_map() {
        let expr = HirExpr::Call {
            func: "map".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_call_filter() {
        let expr = HirExpr::Call {
            func: "filter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_call_enumerate() {
        let expr = HirExpr::Call {
            func: "enumerate".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_call_zip() {
        let expr = HirExpr::Call {
            func: "zip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_call_reversed() {
        let expr = HirExpr::Call {
            func: "reversed".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_call_other() {
        let expr = HirExpr::Call {
            func: "list".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_var() {
        assert!(!is_iterator_producing_expr(&HirExpr::Var(
            "items".to_string()
        )));
    }

    #[test]
    fn test_is_iterator_literal() {
        assert!(!is_iterator_producing_expr(&HirExpr::Literal(
            Literal::Int(42)
        )));
    }

    // ============ is_pure_expression tests ============

    #[test]
    fn test_is_pure_literal() {
        assert!(is_pure_expression(&HirExpr::Literal(Literal::Int(42))));
        assert!(is_pure_expression(&HirExpr::Literal(Literal::String(
            "hello".to_string()
        ))));
        assert!(is_pure_expression(&HirExpr::Literal(Literal::Bool(true))));
        assert!(is_pure_expression(&HirExpr::Literal(Literal::Float(3.15))));
        assert!(is_pure_expression(&HirExpr::Literal(Literal::None)));
    }

    #[test]
    fn test_is_pure_var() {
        assert!(is_pure_expression(&HirExpr::Var("x".to_string())));
    }

    #[test]
    fn test_is_pure_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_binary_impure() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Call {
                func: "f".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(!is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_unary() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_unary_impure() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Call {
                func: "f".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        };
        assert!(!is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_tuple() {
        let expr = HirExpr::Tuple(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Literal(Literal::Int(1)),
        ]);
        assert!(is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_tuple_impure() {
        let expr = HirExpr::Tuple(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Call {
                func: "f".to_string(),
                args: vec![],
                kwargs: vec![],
            },
        ]);
        assert!(!is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_list() {
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        assert!(is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_list_impure() {
        let expr = HirExpr::List(vec![HirExpr::Call {
            func: "f".to_string(),
            args: vec![],
            kwargs: vec![],
        }]);
        assert!(!is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_attribute() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "x".to_string(),
        };
        assert!(is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_attribute_impure() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Call {
                func: "get_obj".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            attr: "x".to_string(),
        };
        assert!(!is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_call() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_method_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "append".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression(&expr));
    }

    // ============ looks_like_option_expr tests ============

    #[test]
    fn test_looks_like_option_ok() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("result".to_string())),
            method: "ok".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_get_no_default() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_get_with_default() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "get".to_string(),
            args: vec![
                HirExpr::Literal(Literal::String("key".to_string())),
                HirExpr::Literal(Literal::Int(0)),
            ],
            kwargs: vec![],
        };
        assert!(!looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_first() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "first".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_last() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "last".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_pop() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "pop".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_next_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "next".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_other_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "append".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_next_call() {
        let expr = HirExpr::Call {
            func: "next".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_other_call() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_var() {
        assert!(!looks_like_option_expr(&HirExpr::Var("x".to_string())));
    }

    // ============ needs_type_conversion tests ============

    #[test]
    fn test_needs_conversion_int_from_usize() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("vec".to_string())),
            method: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(needs_type_conversion(&Type::Int, &expr));
    }

    #[test]
    fn test_needs_conversion_int_from_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!needs_type_conversion(&Type::Int, &expr));
    }

    #[test]
    fn test_needs_conversion_string_from_var() {
        let expr = HirExpr::Var("s".to_string());
        assert!(needs_type_conversion(&Type::String, &expr));
    }

    #[test]
    fn test_needs_conversion_string_from_literal() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert!(!needs_type_conversion(&Type::String, &expr));
    }

    #[test]
    fn test_needs_conversion_float() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!needs_type_conversion(&Type::Float, &expr));
    }

    #[test]
    fn test_needs_conversion_bool() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!needs_type_conversion(&Type::Bool, &expr));
    }

    // ============ expr_infers_float tests ============

    #[test]
    fn test_expr_infers_float_float_literal() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Float(3.15));
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_int_literal() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_string_literal() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_var_with_float_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("x".to_string(), Type::Float);
        let expr = HirExpr::Var("x".to_string());
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_var_with_int_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("x".to_string(), Type::Int);
        let expr = HirExpr::Var("x".to_string());
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_var_unknown() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("x".to_string());
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_with_float_return() {
        let mut ctx = CodeGenContext::default();
        ctx.function_return_types
            .insert("compute".to_string(), Type::Float);
        let expr = HirExpr::Call {
            func: "compute".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_with_int_return() {
        let mut ctx = CodeGenContext::default();
        ctx.function_return_types
            .insert("compute".to_string(), Type::Int);
        let expr = HirExpr::Call {
            func: "compute".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_callable_with_float_return() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "f".to_string(),
            Type::Function {
                params: vec![Type::Float],
                ret: Box::new(Type::Float),
            },
        );
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_callable_with_int_return() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "f".to_string(),
            Type::Function {
                params: vec![Type::Float],
                ret: Box::new(Type::Int),
            },
        );
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_generic_callable_with_float_return() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "f".to_string(),
            Type::Generic {
                base: "Callable".to_string(),
                params: vec![Type::List(Box::new(Type::Float)), Type::Float],
            },
        );
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_generic_callable_with_int_return() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "f".to_string(),
            Type::Generic {
                base: "Callable".to_string(),
                params: vec![Type::List(Box::new(Type::Float)), Type::Int],
            },
        );
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_non_callable_generic() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "f".to_string(),
            Type::Generic {
                base: "List".to_string(),
                params: vec![Type::Float],
            },
        );
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_unknown() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "unknown".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_mul_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Literal(Literal::Float(1.0))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_div_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Float(2.0))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_add_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Float(1.5))),
            right: Box::new(HirExpr::Literal(Literal::Float(2.5))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_sub_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Sub,
            left: Box::new(HirExpr::Literal(Literal::Float(5.0))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_mod_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Mod,
            left: Box::new(HirExpr::Literal(Literal::Float(10.0))),
            right: Box::new(HirExpr::Literal(Literal::Float(3.0))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_pow_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Pow,
            left: Box::new(HirExpr::Literal(Literal::Float(2.0))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_mul_no_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Literal(Literal::Int(2))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_comparison_ignored() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Lt,
            left: Box::new(HirExpr::Literal(Literal::Float(1.0))),
            right: Box::new(HirExpr::Literal(Literal::Float(2.0))),
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_bitwise_ignored() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::BitAnd,
            left: Box::new(HirExpr::Literal(Literal::Float(1.0))),
            right: Box::new(HirExpr::Literal(Literal::Float(2.0))),
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_unary_neg() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Float(3.15))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_unary_not() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Literal(Literal::Float(3.15))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_unary_no_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_if_expr_both_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Float(1.0))),
            orelse: Box::new(HirExpr::Literal(Literal::Float(2.0))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_if_expr_only_body_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Float(1.0))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_if_expr_only_else_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Float(2.0))),
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_if_expr_neither_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_list() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::List(vec![HirExpr::Literal(Literal::Float(1.0))]);
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_method_call() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("x".to_string())),
            method: "sum".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    // ============ is_numpy_value_expr tests ============

    #[test]
    fn test_is_numpy_call_array() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "array".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_zeros() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "zeros".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_ones() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "ones".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_empty() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "empty".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_linspace() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "linspace".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_arange() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "arange".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_full() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "full".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_copy() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "copy".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_abs() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "abs".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_sqrt() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "sqrt".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_sin() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "sin".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_cos() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "cos".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_exp() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "exp".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_log() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "log".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_clip() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "clip".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_clamp() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "clamp".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_normalize() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "normalize".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_call_other() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_array() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("np".to_string())),
            method: "array".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_zeros() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("np".to_string())),
            method: "zeros".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_ones() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("np".to_string())),
            method: "ones".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_abs() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "abs".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_sqrt() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "sqrt".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_sin() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "sin".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_cos() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "cos".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_exp() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "exp".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_log() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "log".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_clip() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "clip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_clamp() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "clamp".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_unwrap() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "unwrap".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_scale() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "scale".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_method_other() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "append".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_binary_left() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Call {
                func: "array".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_binary_right() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Literal(Literal::Int(2))),
            right: Box::new(HirExpr::Call {
                func: "zeros".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_binary_neither() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(!is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_if_expr_body() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Call {
                func: "array".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_if_expr_orelse() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(false))),
            body: Box::new(HirExpr::Literal(Literal::Int(0))),
            orelse: Box::new(HirExpr::Call {
                func: "zeros".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_if_expr_neither() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(!is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_var_tracked() {
        let mut ctx = CodeGenContext::default();
        ctx.numpy_vars.insert("my_array".to_string());
        let expr = HirExpr::Var("my_array".to_string());
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_var_named_arr() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("arr".to_string());
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_var_named_array() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("array".to_string());
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_var_named_data() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("data".to_string());
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_var_named_values() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("values".to_string());
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_var_named_vec() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("vec".to_string());
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_var_named_vector() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("vector".to_string());
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_var_other() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_literal() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_tuple() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Tuple(vec![HirExpr::Literal(Literal::Int(1))]);
        assert!(!is_numpy_value_expr(&expr, &ctx));
    }

    // ============ is_file_creating_expr tests ============

    #[test]
    fn test_is_file_creating_open_call() {
        let expr = HirExpr::Call {
            func: "open".to_string(),
            args: vec![HirExpr::Literal(Literal::String("file.txt".to_string()))],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_file_create() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "create".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_file_open() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "open".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_std_file() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("std".to_string())),
                attr: "File".to_string(),
            }),
            method: "create".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_other_call() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_other_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "read".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_var() {
        let expr = HirExpr::Var("file".to_string());
        assert!(!is_file_creating_expr(&expr));
    }

    // ============ is_stdio_expr tests ============

    #[test]
    fn test_is_stdio_stdout() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "stdout".to_string(),
        };
        assert!(is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_stderr() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "stderr".to_string(),
        };
        assert!(is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_not_sys() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("other".to_string())),
            attr: "stdout".to_string(),
        };
        assert!(!is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_other_attr() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "argv".to_string(),
        };
        assert!(!is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_var() {
        let expr = HirExpr::Var("stdout".to_string());
        assert!(!is_stdio_expr(&expr));
    }

    // ============ extract_string_literal tests ============

    #[test]
    fn test_extract_string_literal_string() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert_eq!(extract_string_literal(&expr), "hello");
    }

    #[test]
    fn test_extract_string_literal_int() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert_eq!(extract_string_literal(&expr), "");
    }

    #[test]
    fn test_extract_string_literal_var() {
        let expr = HirExpr::Var("s".to_string());
        assert_eq!(extract_string_literal(&expr), "");
    }

    // ============ extract_kwarg_string tests ============

    #[test]
    fn test_extract_kwarg_string_found() {
        let kwargs = vec![(
            "name".to_string(),
            HirExpr::Literal(Literal::String("test".to_string())),
        )];
        assert_eq!(
            extract_kwarg_string(&kwargs, "name"),
            Some("test".to_string())
        );
    }

    #[test]
    fn test_extract_kwarg_string_not_found() {
        let kwargs = vec![(
            "name".to_string(),
            HirExpr::Literal(Literal::String("test".to_string())),
        )];
        assert_eq!(extract_kwarg_string(&kwargs, "other"), None);
    }

    #[test]
    fn test_extract_kwarg_string_not_string() {
        let kwargs = vec![("count".to_string(), HirExpr::Literal(Literal::Int(5)))];
        assert_eq!(extract_kwarg_string(&kwargs, "count"), None);
    }

    #[test]
    fn test_extract_kwarg_string_empty() {
        let kwargs: Vec<(String, HirExpr)> = vec![];
        assert_eq!(extract_kwarg_string(&kwargs, "name"), None);
    }

    // ============ extract_kwarg_bool tests ============

    #[test]
    fn test_extract_kwarg_bool_true() {
        let kwargs = vec![("enabled".to_string(), HirExpr::Var("True".to_string()))];
        assert_eq!(extract_kwarg_bool(&kwargs, "enabled"), Some(true));
    }

    #[test]
    fn test_extract_kwarg_bool_false() {
        let kwargs = vec![("disabled".to_string(), HirExpr::Var("False".to_string()))];
        assert_eq!(extract_kwarg_bool(&kwargs, "disabled"), Some(false));
    }

    #[test]
    fn test_extract_kwarg_bool_not_found() {
        let kwargs = vec![("enabled".to_string(), HirExpr::Var("True".to_string()))];
        assert_eq!(extract_kwarg_bool(&kwargs, "other"), None);
    }

    #[test]
    fn test_extract_kwarg_bool_not_bool() {
        let kwargs = vec![("enabled".to_string(), HirExpr::Literal(Literal::Int(1)))];
        assert_eq!(extract_kwarg_bool(&kwargs, "enabled"), None);
    }

    #[test]
    fn test_extract_kwarg_bool_empty() {
        let kwargs: Vec<(String, HirExpr)> = vec![];
        assert_eq!(extract_kwarg_bool(&kwargs, "enabled"), None);
    }

    // ============ is_dict_index_access tests ============

    #[test]
    fn test_is_dict_index_access_string_key() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("config".to_string())),
            index: Box::new(HirExpr::Literal(Literal::String("name".to_string()))),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_dict_var() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("my_dict".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_config_var() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("config".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_list_int() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("items".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(!is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_not_index() {
        let expr = HirExpr::Var("config".to_string());
        assert!(!is_dict_index_access(&expr));
    }

    // ============ contains_floor_div tests ============

    #[test]
    fn test_contains_floor_div_direct() {
        let expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_nested() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Binary {
                op: BinOp::FloorDiv,
                left: Box::new(HirExpr::Literal(Literal::Int(10))),
                right: Box::new(HirExpr::Literal(Literal::Int(3))),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_not_present() {
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(!contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_call() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Binary {
                op: BinOp::FloorDiv,
                left: Box::new(HirExpr::Literal(Literal::Int(10))),
                right: Box::new(HirExpr::Literal(Literal::Int(3))),
            }],
            kwargs: vec![],
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_list() {
        let expr = HirExpr::List(vec![HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        }]);
        assert!(contains_floor_div(&expr));
    }

    // ============ handler_ends_with_exit tests ============

    #[test]
    fn test_handler_ends_with_exit_call() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_sys_exit() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "sys.exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_method() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("sys".to_string())),
            method: "exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_other() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_empty() {
        assert!(!handler_ends_with_exit(&[]));
    }

    // ============ handler_contains_raise tests ============

    #[test]
    fn test_handler_contains_raise_true() {
        use crate::hir::HirStmt;
        let body = vec![
            HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            HirStmt::Raise {
                exception: None,
                cause: None,
            },
        ];
        assert!(handler_contains_raise(&body));
    }

    #[test]
    fn test_handler_contains_raise_false() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!handler_contains_raise(&body));
    }

    #[test]
    fn test_handler_contains_raise_empty() {
        assert!(!handler_contains_raise(&[]));
    }

    // ============ to_pascal_case tests ============

    #[test]
    fn test_to_pascal_case_snake() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    }

    #[test]
    fn test_to_pascal_case_kebab() {
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
    }

    #[test]
    fn test_to_pascal_case_mixed() {
        assert_eq!(to_pascal_case("hello_world-test"), "HelloWorldTest");
    }

    #[test]
    fn test_to_pascal_case_single() {
        assert_eq!(to_pascal_case("hello"), "Hello");
    }

    #[test]
    fn test_to_pascal_case_empty() {
        assert_eq!(to_pascal_case(""), "");
    }

    // ============ is_dict_with_value_type tests ============

    #[test]
    fn test_is_dict_with_value_type_json_value() {
        use crate::hir::Type;
        let dict_type = Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Custom("serde_json::Value".to_string())),
        );
        assert!(is_dict_with_value_type(&dict_type));
    }

    #[test]
    fn test_is_dict_with_value_type_unknown() {
        use crate::hir::Type;
        let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::Unknown));
        assert!(is_dict_with_value_type(&dict_type));
    }

    #[test]
    fn test_is_dict_with_value_type_string() {
        use crate::hir::Type;
        let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::String));
        assert!(!is_dict_with_value_type(&dict_type));
    }

    #[test]
    fn test_is_dict_with_value_type_not_dict() {
        use crate::hir::Type;
        assert!(!is_dict_with_value_type(&Type::String));
    }

    // ============ is_dict_augassign_pattern tests ============

    #[test]
    fn test_is_dict_augassign_pattern_true() {
        use crate::hir::{AssignTarget, BinOp};
        // dict[key] = dict[key] + value
        let target = AssignTarget::Index {
            base: Box::new(HirExpr::Var("dict".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        let value = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("dict".to_string())),
                index: Box::new(HirExpr::Var("key".to_string())),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_dict_augassign_pattern(&target, &value));
    }

    #[test]
    fn test_is_dict_augassign_pattern_different_dict() {
        use crate::hir::{AssignTarget, BinOp};
        // dict[key] = other[key] + value - not same dict
        let target = AssignTarget::Index {
            base: Box::new(HirExpr::Var("dict".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        let value = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("other".to_string())),
                index: Box::new(HirExpr::Var("key".to_string())),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(!is_dict_augassign_pattern(&target, &value));
    }

    #[test]
    fn test_is_dict_augassign_pattern_not_index() {
        use crate::hir::AssignTarget;
        let target = AssignTarget::Symbol("x".to_string());
        let value = HirExpr::Literal(Literal::Int(1));
        assert!(!is_dict_augassign_pattern(&target, &value));
    }

    // ============ is_nested_function_recursive tests ============

    #[test]
    fn test_is_nested_function_recursive_true() {
        use crate::hir::HirStmt;
        // Function body calls itself
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "factorial".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(is_nested_function_recursive("factorial", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_false() {
        use crate::hir::HirStmt;
        // Function body doesn't call itself
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!is_nested_function_recursive("factorial", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_in_binary() {
        use crate::hir::{BinOp, HirStmt};
        // Recursive call in binary expression: n * factorial(n-1)
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Var("n".to_string())),
            right: Box::new(HirExpr::Call {
                func: "factorial".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            }),
        }))];
        assert!(is_nested_function_recursive("factorial", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_in_if() {
        use crate::hir::HirStmt;
        // Recursive call inside if body
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Expr(HirExpr::Call {
                func: "recursive_fn".to_string(),
                args: vec![],
                kwargs: vec![],
            })],
            else_body: None,
        }];
        assert!(is_nested_function_recursive("recursive_fn", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_empty() {
        assert!(!is_nested_function_recursive("factorial", &[]));
    }

    // ============ extract_divisor_from_floor_div tests ============

    #[test]
    fn test_extract_divisor_floor_div_direct() {
        use crate::hir::BinOp;
        let expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), HirExpr::Literal(Literal::Int(2))));
    }

    #[test]
    fn test_extract_divisor_floor_div_nested_left() {
        use crate::hir::BinOp;
        // (a // 2) + b - floor div in left subtree
        let floor_div = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(floor_div),
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), HirExpr::Literal(Literal::Int(3))));
    }

    #[test]
    fn test_extract_divisor_floor_div_not_found() {
        use crate::hir::BinOp;
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_divisor_floor_div_var() {
        let expr = HirExpr::Var("x".to_string());
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_err());
    }

    // ============ DEPYLER-1054: get_depyler_extraction_for_type tests ============

    #[test]
    fn test_depyler_1054_extraction_int() {
        use crate::hir::Type;
        let result = get_depyler_extraction_for_type(&Type::Int);
        assert_eq!(result, Some(".to_i64() as i32"));
    }

    #[test]
    fn test_depyler_1054_extraction_float() {
        use crate::hir::Type;
        let result = get_depyler_extraction_for_type(&Type::Float);
        assert_eq!(result, Some(".to_f64()"));
    }

    #[test]
    fn test_depyler_1054_extraction_string() {
        use crate::hir::Type;
        let result = get_depyler_extraction_for_type(&Type::String);
        assert_eq!(result, Some(".to_string()"));
    }

    #[test]
    fn test_depyler_1054_extraction_bool() {
        use crate::hir::Type;
        let result = get_depyler_extraction_for_type(&Type::Bool);
        assert_eq!(result, Some(".to_bool()"));
    }

    #[test]
    fn test_depyler_1054_extraction_unsupported() {
        use crate::hir::Type;
        // List type shouldn't have an extraction method
        assert!(get_depyler_extraction_for_type(&Type::List(Box::new(Type::Int))).is_none());
        // Dict type shouldn't have an extraction method
        assert!(get_depyler_extraction_for_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        ))
        .is_none());
    }
}
