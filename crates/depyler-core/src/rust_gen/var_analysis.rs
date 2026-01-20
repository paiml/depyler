//! Variable Analysis Module - EXTREME TDD (PMAT v3.21.0)
//!
//! Pure functions for analyzing variable usage, reassignment, and extraction
//! from HIR expressions and statements. These functions are essential for:
//! - Detecting unused loop variables (underscore prefixing)
//! - Determining mutability requirements for loop variables
//! - Hoisting variable declarations for scope-based patterns
//! - Extracting walrus operator assignments

use crate::hir::{AssignTarget, BinOp, HirExpr, HirStmt, FStringPart};
use std::collections::HashSet;

/// Check if a variable is used in an expression
///
/// DEPYLER-0307, DEPYLER-0569, DEPYLER-0619, DEPYLER-0768
pub fn is_var_used_in_expr(var_name: &str, expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Var(name) => name == var_name,
        HirExpr::Binary { left, right, .. } => {
            is_var_used_in_expr(var_name, left) || is_var_used_in_expr(var_name, right)
        }
        HirExpr::Unary { operand, .. } => is_var_used_in_expr(var_name, operand),
        HirExpr::Call { func: _, args, .. } => {
            args.iter().any(|arg| is_var_used_in_expr(var_name, arg))
        }
        HirExpr::MethodCall { object, args, .. } => {
            // DEPYLER-0307 Fix #6: Check method receiver and arguments
            is_var_used_in_expr(var_name, object)
                || args.iter().any(|arg| is_var_used_in_expr(var_name, arg))
        }
        HirExpr::Index { base, index } => {
            is_var_used_in_expr(var_name, base) || is_var_used_in_expr(var_name, index)
        }
        HirExpr::Attribute { value, .. } => is_var_used_in_expr(var_name, value),
        HirExpr::List(elements)
        | HirExpr::Tuple(elements)
        | HirExpr::Set(elements)
        | HirExpr::FrozenSet(elements) => {
            elements.iter().any(|e| is_var_used_in_expr(var_name, e))
        }
        HirExpr::Dict(pairs) => pairs
            .iter()
            .any(|(k, v)| is_var_used_in_expr(var_name, k) || is_var_used_in_expr(var_name, v)),
        HirExpr::IfExpr { test, body, orelse } => {
            is_var_used_in_expr(var_name, test)
                || is_var_used_in_expr(var_name, body)
                || is_var_used_in_expr(var_name, orelse)
        }
        HirExpr::Lambda { params: _, body } => is_var_used_in_expr(var_name, body),
        HirExpr::Slice {
            base,
            start,
            stop,
            step,
        } => {
            is_var_used_in_expr(var_name, base)
                || start.as_ref().is_some_and(|s| is_var_used_in_expr(var_name, s))
                || stop.as_ref().is_some_and(|s| is_var_used_in_expr(var_name, s))
                || step.as_ref().is_some_and(|s| is_var_used_in_expr(var_name, s))
        }
        HirExpr::FString { parts } => parts.iter().any(|part| match part {
            FStringPart::Expr(expr) => is_var_used_in_expr(var_name, expr),
            FStringPart::Literal(_) => false,
        }),
        // DEPYLER-0569: Handle generator expressions and comprehensions
        HirExpr::GeneratorExp { element, generators }
        | HirExpr::ListComp { element, generators }
        | HirExpr::SetComp { element, generators } => {
            is_var_used_in_expr(var_name, element)
                || generators.iter().any(|gen| {
                    is_var_used_in_expr(var_name, &gen.iter)
                        || gen.conditions.iter().any(|cond| is_var_used_in_expr(var_name, cond))
                })
        }
        HirExpr::DictComp { key, value, generators } => {
            is_var_used_in_expr(var_name, key)
                || is_var_used_in_expr(var_name, value)
                || generators.iter().any(|gen| {
                    is_var_used_in_expr(var_name, &gen.iter)
                        || gen.conditions.iter().any(|cond| is_var_used_in_expr(var_name, cond))
                })
        }
        // DEPYLER-0619: Handle await expressions
        HirExpr::Await { value } => is_var_used_in_expr(var_name, value),
        // DEPYLER-0768: Handle yield expressions
        HirExpr::Yield { value } => {
            value.as_ref().is_some_and(|v| is_var_used_in_expr(var_name, v))
        }
        _ => false, // Literals and other expressions don't reference variables
    }
}

/// Check if a variable is used in an assignment target
pub fn is_var_used_in_assign_target(var_name: &str, target: &AssignTarget) -> bool {
    match target {
        AssignTarget::Symbol(s) => s == var_name,
        AssignTarget::Index { base, index } => {
            is_var_used_in_expr(var_name, base) || is_var_used_in_expr(var_name, index)
        }
        AssignTarget::Attribute { value, .. } => is_var_used_in_expr(var_name, value),
        AssignTarget::Tuple(targets) => targets
            .iter()
            .any(|t| is_var_used_in_assign_target(var_name, t)),
    }
}

/// Helper to check if variable is directly referenced (not nested)
pub fn is_var_direct_or_simple_in_expr(var_name: &str, expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Var(name) => name == var_name,
        _ => false,
    }
}

/// DEPYLER-0715: Check if a variable is used as a dictionary key
/// Returns true if the variable appears as the index in dict[var] or as arg to dict.get(var)
pub fn is_var_used_as_dict_key_in_expr(var_name: &str, expr: &HirExpr) -> bool {
    match expr {
        // Check dict[var] pattern - the variable is the INDEX, not the base
        HirExpr::Index { base, index } => {
            // If var_name is used as the index, check if base looks like a dict
            if is_var_direct_or_simple_in_expr(var_name, index) {
                // Check if base is likely a dict (variable or method call result)
                matches!(base.as_ref(), HirExpr::Var(_) | HirExpr::MethodCall { .. })
            } else {
                // Recurse into both parts
                is_var_used_as_dict_key_in_expr(var_name, base)
                    || is_var_used_as_dict_key_in_expr(var_name, index)
            }
        }
        // Check dict.get(var) pattern
        HirExpr::MethodCall { method, args, object, .. } if method == "get" => {
            if args.first().is_some_and(|arg| is_var_direct_or_simple_in_expr(var_name, arg)) {
                true
            } else {
                is_var_used_as_dict_key_in_expr(var_name, object)
                    || args.iter().any(|arg| is_var_used_as_dict_key_in_expr(var_name, arg))
            }
        }
        // Recurse into other expressions
        HirExpr::Binary { left, right, .. } => {
            is_var_used_as_dict_key_in_expr(var_name, left)
                || is_var_used_as_dict_key_in_expr(var_name, right)
        }
        HirExpr::Unary { operand, .. } => is_var_used_as_dict_key_in_expr(var_name, operand),
        HirExpr::Call { args, .. } => {
            args.iter().any(|arg| is_var_used_as_dict_key_in_expr(var_name, arg))
        }
        HirExpr::MethodCall { object, args, .. } => {
            is_var_used_as_dict_key_in_expr(var_name, object)
                || args.iter().any(|arg| is_var_used_as_dict_key_in_expr(var_name, arg))
        }
        HirExpr::IfExpr { test, body, orelse } => {
            is_var_used_as_dict_key_in_expr(var_name, test)
                || is_var_used_as_dict_key_in_expr(var_name, body)
                || is_var_used_as_dict_key_in_expr(var_name, orelse)
        }
        HirExpr::List(elements) | HirExpr::Tuple(elements) | HirExpr::Set(elements) => {
            elements.iter().any(|e| is_var_used_as_dict_key_in_expr(var_name, e))
        }
        _ => false,
    }
}

/// DEPYLER-0715: Check if a variable is used as a dict key in a statement
pub fn is_var_used_as_dict_key_in_stmt(var_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            // Check if var is used as key in target (e.g., dict[var] = x)
            if let AssignTarget::Index { base, index } = target {
                if is_var_direct_or_simple_in_expr(var_name, index)
                    && matches!(base.as_ref(), HirExpr::Var(_))
                {
                    return true;
                }
            }
            is_var_used_as_dict_key_in_expr(var_name, value)
        }
        HirStmt::If { condition, then_body, else_body } => {
            is_var_used_as_dict_key_in_expr(var_name, condition)
                || then_body.iter().any(|s| is_var_used_as_dict_key_in_stmt(var_name, s))
                || else_body.as_ref().is_some_and(|body| {
                    body.iter().any(|s| is_var_used_as_dict_key_in_stmt(var_name, s))
                })
        }
        HirStmt::While { condition, body } => {
            is_var_used_as_dict_key_in_expr(var_name, condition)
                || body.iter().any(|s| is_var_used_as_dict_key_in_stmt(var_name, s))
        }
        HirStmt::For { iter, body, .. } => {
            is_var_used_as_dict_key_in_expr(var_name, iter)
                || body.iter().any(|s| is_var_used_as_dict_key_in_stmt(var_name, s))
        }
        HirStmt::Return(Some(expr)) => is_var_used_as_dict_key_in_expr(var_name, expr),
        HirStmt::Expr(expr) => is_var_used_as_dict_key_in_expr(var_name, expr),
        _ => false,
    }
}

/// DEPYLER-1045: Check if a variable is used as a function argument in an expression
/// This is used to detect when a char loop variable needs to be converted to String
/// for functions that expect &str arguments
pub fn is_var_used_as_func_arg_in_expr(var_name: &str, expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Call { args, .. } => {
            // Check if var is directly used as an argument
            args.iter().any(|arg| matches!(arg, HirExpr::Var(name) if name == var_name))
        }
        HirExpr::MethodCall { args, .. } => {
            args.iter().any(|arg| matches!(arg, HirExpr::Var(name) if name == var_name))
        }
        HirExpr::Binary { left, right, .. } => {
            is_var_used_as_func_arg_in_expr(var_name, left)
                || is_var_used_as_func_arg_in_expr(var_name, right)
        }
        HirExpr::Unary { operand, .. } => is_var_used_as_func_arg_in_expr(var_name, operand),
        HirExpr::IfExpr { test, body, orelse } => {
            is_var_used_as_func_arg_in_expr(var_name, test)
                || is_var_used_as_func_arg_in_expr(var_name, body)
                || is_var_used_as_func_arg_in_expr(var_name, orelse)
        }
        _ => false,
    }
}

/// DEPYLER-1045: Check if a variable is used as a function argument in a statement
pub fn is_var_used_as_func_arg_in_stmt(var_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { value, .. } => is_var_used_as_func_arg_in_expr(var_name, value),
        HirStmt::If { condition, then_body, else_body } => {
            is_var_used_as_func_arg_in_expr(var_name, condition)
                || then_body.iter().any(|s| is_var_used_as_func_arg_in_stmt(var_name, s))
                || else_body.as_ref().is_some_and(|body| {
                    body.iter().any(|s| is_var_used_as_func_arg_in_stmt(var_name, s))
                })
        }
        HirStmt::While { condition, body } => {
            is_var_used_as_func_arg_in_expr(var_name, condition)
                || body.iter().any(|s| is_var_used_as_func_arg_in_stmt(var_name, s))
        }
        HirStmt::For { iter, body, .. } => {
            is_var_used_as_func_arg_in_expr(var_name, iter)
                || body.iter().any(|s| is_var_used_as_func_arg_in_stmt(var_name, s))
        }
        HirStmt::Return(Some(expr)) => is_var_used_as_func_arg_in_expr(var_name, expr),
        HirStmt::Expr(expr) => is_var_used_as_func_arg_in_expr(var_name, expr),
        _ => false,
    }
}

/// DEPYLER-1045: Check if a variable is used in a comparison (== or !=) in an expression
/// This helps detect when a char loop variable is compared with a &str and needs conversion
pub fn is_var_in_comparison_in_expr(var_name: &str, expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Binary { left, right, op } => {
            // Check if this is a comparison operation
            let is_comparison = matches!(op, BinOp::Eq | BinOp::NotEq | BinOp::In | BinOp::NotIn);
            if is_comparison {
                // Check if var_name is directly one of the operands
                let var_is_left = matches!(left.as_ref(), HirExpr::Var(name) if name == var_name);
                let var_is_right = matches!(right.as_ref(), HirExpr::Var(name) if name == var_name);
                if var_is_left || var_is_right {
                    return true;
                }
            }
            // Recursively check sub-expressions
            is_var_in_comparison_in_expr(var_name, left)
                || is_var_in_comparison_in_expr(var_name, right)
        }
        HirExpr::Unary { operand, .. } => is_var_in_comparison_in_expr(var_name, operand),
        HirExpr::IfExpr { test, body, orelse } => {
            is_var_in_comparison_in_expr(var_name, test)
                || is_var_in_comparison_in_expr(var_name, body)
                || is_var_in_comparison_in_expr(var_name, orelse)
        }
        HirExpr::Call { args, .. } => {
            args.iter().any(|arg| is_var_in_comparison_in_expr(var_name, arg))
        }
        HirExpr::MethodCall { object, args, .. } => {
            is_var_in_comparison_in_expr(var_name, object)
                || args.iter().any(|arg| is_var_in_comparison_in_expr(var_name, arg))
        }
        _ => false,
    }
}

/// DEPYLER-1045: Check if a variable is used in a comparison in a statement
pub fn is_var_in_comparison_in_stmt(var_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { value, .. } => is_var_in_comparison_in_expr(var_name, value),
        HirStmt::If { condition, then_body, else_body } => {
            is_var_in_comparison_in_expr(var_name, condition)
                || then_body.iter().any(|s| is_var_in_comparison_in_stmt(var_name, s))
                || else_body.as_ref().is_some_and(|body| {
                    body.iter().any(|s| is_var_in_comparison_in_stmt(var_name, s))
                })
        }
        HirStmt::While { condition, body } => {
            is_var_in_comparison_in_expr(var_name, condition)
                || body.iter().any(|s| is_var_in_comparison_in_stmt(var_name, s))
        }
        HirStmt::For { iter, body, .. } => {
            is_var_in_comparison_in_expr(var_name, iter)
                || body.iter().any(|s| is_var_in_comparison_in_stmt(var_name, s))
        }
        HirStmt::Return(Some(expr)) => is_var_in_comparison_in_expr(var_name, expr),
        HirStmt::Expr(expr) => is_var_in_comparison_in_expr(var_name, expr),
        _ => false,
    }
}

/// Check if a variable is reassigned in a statement
/// DEPYLER-0756: Loop variables that are reassigned need `mut` in for pattern
pub fn is_var_reassigned_in_stmt(var_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, .. } => {
            // Only count as reassignment if the target is the exact variable
            matches!(target, AssignTarget::Symbol(name) if name == var_name)
        }
        HirStmt::If { then_body, else_body, .. } => {
            then_body.iter().any(|s| is_var_reassigned_in_stmt(var_name, s))
                || else_body.as_ref().is_some_and(|body| {
                    body.iter().any(|s| is_var_reassigned_in_stmt(var_name, s))
                })
        }
        HirStmt::While { body, .. } => body.iter().any(|s| is_var_reassigned_in_stmt(var_name, s)),
        HirStmt::For { body, .. } => body.iter().any(|s| is_var_reassigned_in_stmt(var_name, s)),
        HirStmt::Try { body, handlers, orelse, finalbody, .. } => {
            body.iter().any(|s| is_var_reassigned_in_stmt(var_name, s))
                || handlers.iter().any(|h| {
                    h.body.iter().any(|s| is_var_reassigned_in_stmt(var_name, s))
                })
                || orelse.as_ref().is_some_and(|stmts| {
                    stmts.iter().any(|s| is_var_reassigned_in_stmt(var_name, s))
                })
                || finalbody.as_ref().is_some_and(|stmts| {
                    stmts.iter().any(|s| is_var_reassigned_in_stmt(var_name, s))
                })
        }
        HirStmt::With { body, .. } => body.iter().any(|s| is_var_reassigned_in_stmt(var_name, s)),
        _ => false,
    }
}

/// Check if a variable is used in a statement
/// DEPYLER-0303 Phase 2: Fixed to check assignment targets too (for `d[k] = v`)
pub fn is_var_used_in_stmt(var_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            // Check both target (e.g., d[k]) and value (e.g., v)
            is_var_used_in_assign_target(var_name, target) || is_var_used_in_expr(var_name, value)
        }
        HirStmt::If { condition, then_body, else_body } => {
            is_var_used_in_expr(var_name, condition)
                || then_body.iter().any(|s| is_var_used_in_stmt(var_name, s))
                || else_body.as_ref().is_some_and(|body| {
                    body.iter().any(|s| is_var_used_in_stmt(var_name, s))
                })
        }
        HirStmt::While { condition, body } => {
            is_var_used_in_expr(var_name, condition)
                || body.iter().any(|s| is_var_used_in_stmt(var_name, s))
        }
        HirStmt::For { iter, body, .. } => {
            is_var_used_in_expr(var_name, iter)
                || body.iter().any(|s| is_var_used_in_stmt(var_name, s))
        }
        HirStmt::Return(Some(expr)) => is_var_used_in_expr(var_name, expr),
        HirStmt::Expr(expr) => is_var_used_in_expr(var_name, expr),
        HirStmt::Raise { exception, .. } => exception
            .as_ref()
            .is_some_and(|e| is_var_used_in_expr(var_name, e)),
        HirStmt::Assert { test, msg, .. } => {
            is_var_used_in_expr(var_name, test)
                || msg.as_ref().is_some_and(|m| is_var_used_in_expr(var_name, m))
        }
        // DEPYLER-0593: Handle Try statements for variable usage detection
        HirStmt::Try { body, handlers, orelse, finalbody, .. } => {
            body.iter().any(|s| is_var_used_in_stmt(var_name, s))
                || handlers.iter().any(|h| {
                    h.body.iter().any(|s| is_var_used_in_stmt(var_name, s))
                })
                || orelse.as_ref().is_some_and(|stmts| {
                    stmts.iter().any(|s| is_var_used_in_stmt(var_name, s))
                })
                || finalbody.as_ref().is_some_and(|stmts| {
                    stmts.iter().any(|s| is_var_used_in_stmt(var_name, s))
                })
        }
        HirStmt::With { body, .. } => body.iter().any(|s| is_var_used_in_stmt(var_name, s)),
        _ => false,
    }
}

/// Extract all symbols assigned in statements (including nested blocks)
pub fn extract_assigned_symbols(stmts: &[HirStmt]) -> HashSet<String> {
    let mut symbols = HashSet::new();

    for stmt in stmts {
        match stmt {
            HirStmt::Assign { target: AssignTarget::Symbol(name), .. } => {
                symbols.insert(name.clone());
            }
            HirStmt::If { then_body, else_body, .. } => {
                symbols.extend(extract_assigned_symbols(then_body));
                if let Some(else_stmts) = else_body {
                    symbols.extend(extract_assigned_symbols(else_stmts));
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                symbols.extend(extract_assigned_symbols(body));
            }
            HirStmt::Try { body, handlers, finalbody, .. } => {
                symbols.extend(extract_assigned_symbols(body));
                for handler in handlers {
                    symbols.extend(extract_assigned_symbols(&handler.body));
                }
                if let Some(finally) = finalbody {
                    symbols.extend(extract_assigned_symbols(finally));
                }
            }
            _ => {}
        }
    }

    symbols
}

/// Extract symbols assigned ONLY at the top level (not in nested for/while loops)
///
/// DEPYLER-0476: Fix variable hoisting for variables with incompatible types in nested scopes.
/// Variables assigned inside for/while loops should NOT be hoisted to the parent if/else scope
/// because they may have different types than variables with the same name in the if branch.
pub fn extract_toplevel_assigned_symbols(stmts: &[HirStmt]) -> HashSet<String> {
    let mut symbols = HashSet::new();

    for stmt in stmts {
        match stmt {
            HirStmt::Assign { target: AssignTarget::Symbol(name), .. } => {
                symbols.insert(name.clone());
            }
            // DEPYLER-0939: Handle tuple unpacking assignments
            HirStmt::Assign { target: AssignTarget::Tuple(targets), .. } => {
                for t in targets {
                    if let AssignTarget::Symbol(name) = t {
                        symbols.insert(name.clone());
                    }
                }
            }
            // Recursively check nested if/else blocks
            HirStmt::If { then_body, else_body, .. } => {
                symbols.extend(extract_toplevel_assigned_symbols(then_body));
                if let Some(else_stmts) = else_body {
                    symbols.extend(extract_toplevel_assigned_symbols(else_stmts));
                }
            }
            // Recursively check try/except blocks
            HirStmt::Try { body, handlers, finalbody, .. } => {
                symbols.extend(extract_toplevel_assigned_symbols(body));
                for handler in handlers {
                    symbols.extend(extract_toplevel_assigned_symbols(&handler.body));
                }
                if let Some(finally) = finalbody {
                    symbols.extend(extract_toplevel_assigned_symbols(finally));
                }
            }
            // DEPYLER-0476: DO NOT recurse into for/while loops
            HirStmt::While { .. } | HirStmt::For { .. } => {
                // Skip - don't extract symbols from loop bodies
            }
            _ => {}
        }
    }

    symbols
}

/// DEPYLER-1188: Collect all variable names used in an expression
/// Returns a set of all variable names that appear as HirExpr::Var
/// Used to detect when a variable is used multiple times in a tuple expression
pub fn collect_vars_in_expr(expr: &HirExpr) -> HashSet<String> {
    let mut vars = HashSet::new();
    collect_vars_recursive(expr, &mut vars);
    vars
}

/// DEPYLER-1188: Recursive helper to collect variable names
fn collect_vars_recursive(expr: &HirExpr, vars: &mut HashSet<String>) {
    match expr {
        HirExpr::Var(name) => {
            vars.insert(name.clone());
        }
        HirExpr::Binary { left, right, .. } => {
            collect_vars_recursive(left, vars);
            collect_vars_recursive(right, vars);
        }
        HirExpr::Unary { operand, .. } => {
            collect_vars_recursive(operand, vars);
        }
        HirExpr::Call { args, kwargs, .. } => {
            for arg in args {
                collect_vars_recursive(arg, vars);
            }
            for (_, v) in kwargs {
                collect_vars_recursive(v, vars);
            }
        }
        HirExpr::MethodCall { object, args, kwargs, .. } => {
            collect_vars_recursive(object, vars);
            for arg in args {
                collect_vars_recursive(arg, vars);
            }
            for (_, v) in kwargs {
                collect_vars_recursive(v, vars);
            }
        }
        HirExpr::Index { base, index } => {
            collect_vars_recursive(base, vars);
            collect_vars_recursive(index, vars);
        }
        HirExpr::Attribute { value, .. } => {
            collect_vars_recursive(value, vars);
        }
        HirExpr::List(elements)
        | HirExpr::Tuple(elements)
        | HirExpr::Set(elements)
        | HirExpr::FrozenSet(elements) => {
            for e in elements {
                collect_vars_recursive(e, vars);
            }
        }
        HirExpr::Dict(pairs) => {
            for (k, v) in pairs {
                collect_vars_recursive(k, vars);
                collect_vars_recursive(v, vars);
            }
        }
        HirExpr::IfExpr { test, body, orelse } => {
            collect_vars_recursive(test, vars);
            collect_vars_recursive(body, vars);
            collect_vars_recursive(orelse, vars);
        }
        HirExpr::Lambda { body, .. } => {
            collect_vars_recursive(body, vars);
        }
        HirExpr::Slice { base, start, stop, step } => {
            collect_vars_recursive(base, vars);
            if let Some(s) = start {
                collect_vars_recursive(s, vars);
            }
            if let Some(s) = stop {
                collect_vars_recursive(s, vars);
            }
            if let Some(s) = step {
                collect_vars_recursive(s, vars);
            }
        }
        HirExpr::FString { parts } => {
            for part in parts {
                if let FStringPart::Expr(e) = part {
                    collect_vars_recursive(e, vars);
                }
            }
        }
        HirExpr::GeneratorExp { element, generators }
        | HirExpr::ListComp { element, generators }
        | HirExpr::SetComp { element, generators } => {
            collect_vars_recursive(element, vars);
            for gen in generators {
                collect_vars_recursive(&gen.iter, vars);
                for cond in &gen.conditions {
                    collect_vars_recursive(cond, vars);
                }
            }
        }
        HirExpr::DictComp { key, value, generators } => {
            collect_vars_recursive(key, vars);
            collect_vars_recursive(value, vars);
            for gen in generators {
                collect_vars_recursive(&gen.iter, vars);
                for cond in &gen.conditions {
                    collect_vars_recursive(cond, vars);
                }
            }
        }
        HirExpr::Await { value } => {
            collect_vars_recursive(value, vars);
        }
        HirExpr::Yield { value: Some(v) } => {
            collect_vars_recursive(v, vars);
        }
        HirExpr::Yield { value: None } => {}
        HirExpr::Borrow { expr, .. } => {
            collect_vars_recursive(expr, vars);
        }
        HirExpr::NamedExpr { value, .. } => {
            collect_vars_recursive(value, vars);
        }
        HirExpr::DynamicCall { callee, args, .. } => {
            collect_vars_recursive(callee, vars);
            for arg in args {
                collect_vars_recursive(arg, vars);
            }
        }
        HirExpr::SortByKey { iterable, key_body, reverse_expr, .. } => {
            collect_vars_recursive(iterable, vars);
            collect_vars_recursive(key_body, vars);
            if let Some(r) = reverse_expr {
                collect_vars_recursive(r, vars);
            }
        }
        // Literals and other non-variable expressions
        _ => {}
    }
}

/// DEPYLER-0188: Extract NamedExpr (walrus operator) assignments from a condition expression
/// Returns: (hoisted_lets, simplified_condition)
pub fn extract_walrus_from_condition(condition: &HirExpr) -> (Vec<(String, HirExpr)>, HirExpr) {
    let mut walrus_assigns = Vec::new();
    let simplified = extract_walrus_recursive(condition, &mut walrus_assigns);
    (walrus_assigns, simplified)
}

/// Recursive helper to extract NamedExpr from expression tree
pub fn extract_walrus_recursive(expr: &HirExpr, assigns: &mut Vec<(String, HirExpr)>) -> HirExpr {
    match expr {
        // DEPYLER-0188: When we find a walrus operator, extract it and replace with Var
        HirExpr::NamedExpr { target, value } => {
            // Recursively process the value in case it contains nested walrus
            let simplified_value = extract_walrus_recursive(value, assigns);
            assigns.push((target.clone(), simplified_value));
            // Replace with just a variable reference
            HirExpr::Var(target.clone())
        }
        // Recursively process binary expressions
        HirExpr::Binary { op, left, right } => HirExpr::Binary {
            op: *op,
            left: Box::new(extract_walrus_recursive(left, assigns)),
            right: Box::new(extract_walrus_recursive(right, assigns)),
        },
        // Recursively process unary expressions
        HirExpr::Unary { op, operand } => HirExpr::Unary {
            op: *op,
            operand: Box::new(extract_walrus_recursive(operand, assigns)),
        },
        // Recursively process call arguments
        HirExpr::Call { func, args, kwargs } => HirExpr::Call {
            func: func.clone(),
            args: args.iter().map(|a| extract_walrus_recursive(a, assigns)).collect(),
            kwargs: kwargs
                .iter()
                .map(|(k, v)| (k.clone(), extract_walrus_recursive(v, assigns)))
                .collect(),
        },
        // Recursively process method call arguments
        HirExpr::MethodCall { object, method, args, kwargs } => HirExpr::MethodCall {
            object: Box::new(extract_walrus_recursive(object, assigns)),
            method: method.clone(),
            args: args.iter().map(|a| extract_walrus_recursive(a, assigns)).collect(),
            kwargs: kwargs
                .iter()
                .map(|(k, v)| (k.clone(), extract_walrus_recursive(v, assigns)))
                .collect(),
        },
        // For other expressions, return as-is
        _ => expr.clone(),
    }
}

/// DEPYLER-0931: Find position of variable in tuple assignment target
pub fn find_var_position_in_tuple(var_name: &str, targets: &[AssignTarget]) -> Option<usize> {
    for (i, target) in targets.iter().enumerate() {
        if let AssignTarget::Symbol(name) = target {
            if name == var_name {
                return Some(i);
            }
        }
    }
    None
}

/// DEPYLER-0625: Find the expression assigned to a variable in a statement block
pub fn find_assigned_expr<'a>(var_name: &str, stmts: &'a [HirStmt]) -> Option<&'a HirExpr> {
    for stmt in stmts {
        if let HirStmt::Assign {
            target: AssignTarget::Symbol(name),
            value,
            ..
        } = stmt
        {
            if name == var_name {
                return Some(value);
            }
        }
    }
    None
}

/// DEPYLER-0625: Check if a variable needs Box<dyn Write> due to heterogeneous IO types
/// Returns true if variable is assigned File in one branch and Stdout/Stderr in another
pub fn needs_boxed_dyn_write(var_name: &str, then_body: &[HirStmt], else_body: &[HirStmt]) -> bool {
    use crate::rust_gen::expr_analysis::{is_file_creating_expr, is_stdio_expr};
    let then_expr = find_assigned_expr(var_name, then_body);
    let else_expr = find_assigned_expr(var_name, else_body);

    match (then_expr, else_expr) {
        (Some(then_e), Some(else_e)) => {
            let then_file = is_file_creating_expr(then_e);
            let then_stdio = is_stdio_expr(then_e);
            let else_file = is_file_creating_expr(else_e);
            let else_stdio = is_stdio_expr(else_e);
            // Different IO types: File in one, Stdout/Stderr in another
            (then_file && else_stdio) || (then_stdio && else_file)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, UnaryOp, Literal, HirComprehension};

    // Helper functions for creating test expressions
    fn lit_int(n: i64) -> HirExpr {
        HirExpr::Literal(Literal::Int(n))
    }

    fn lit_str(s: &str) -> HirExpr {
        HirExpr::Literal(Literal::String(s.to_string()))
    }

    fn lit_bool(b: bool) -> HirExpr {
        HirExpr::Literal(Literal::Bool(b))
    }

    // ============================================================================
    // is_var_used_in_expr tests
    // ============================================================================

    #[test]
    fn test_var_used_simple_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(is_var_used_in_expr("x", &expr));
        assert!(!is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_used_in_binary_left() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(lit_int(1)),
        };
        assert!(is_var_used_in_expr("x", &expr));
        assert!(!is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_used_in_binary_right() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(lit_int(1)),
            right: Box::new(HirExpr::Var("y".to_string())),
        };
        assert!(is_var_used_in_expr("y", &expr));
        assert!(!is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_unary() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_call_args() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_var_used_in_expr("x", &expr));
        assert!(!is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_used_in_method_call_object() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".to_string())),
            method: "append".to_string(),
            args: vec![lit_int(1)],
            kwargs: vec![],
        };
        assert!(is_var_used_in_expr("lst", &expr));
    }

    #[test]
    fn test_var_used_in_method_call_args() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_index_base() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(lit_int(0)),
        };
        assert!(is_var_used_in_expr("arr", &expr));
    }

    #[test]
    fn test_var_used_in_index_index() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("i".to_string())),
        };
        assert!(is_var_used_in_expr("i", &expr));
    }

    #[test]
    fn test_var_used_in_attribute() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        assert!(is_var_used_in_expr("obj", &expr));
    }

    #[test]
    fn test_var_used_in_list() {
        let expr = HirExpr::List(vec![
            lit_int(1),
            HirExpr::Var("x".to_string()),
            lit_int(3),
        ]);
        assert!(is_var_used_in_expr("x", &expr));
        assert!(!is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_used_in_tuple() {
        let expr = HirExpr::Tuple(vec![HirExpr::Var("a".to_string()), HirExpr::Var("b".to_string())]);
        assert!(is_var_used_in_expr("a", &expr));
        assert!(is_var_used_in_expr("b", &expr));
    }

    #[test]
    fn test_var_used_in_set() {
        let expr = HirExpr::Set(vec![HirExpr::Var("x".to_string())]);
        assert!(is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_frozenset() {
        let expr = HirExpr::FrozenSet(vec![HirExpr::Var("x".to_string())]);
        assert!(is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_dict_key() {
        let expr = HirExpr::Dict(vec![(HirExpr::Var("k".to_string()), lit_int(1))]);
        assert!(is_var_used_in_expr("k", &expr));
    }

    #[test]
    fn test_var_used_in_dict_value() {
        let expr = HirExpr::Dict(vec![(lit_str("key"), HirExpr::Var("v".to_string()))]);
        assert!(is_var_used_in_expr("v", &expr));
    }

    #[test]
    fn test_var_used_in_if_expr_test() {
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(lit_int(1)),
            orelse: Box::new(lit_int(0)),
        };
        assert!(is_var_used_in_expr("cond", &expr));
    }

    #[test]
    fn test_var_used_in_if_expr_body() {
        let expr = HirExpr::IfExpr {
            test: Box::new(lit_bool(true)),
            body: Box::new(HirExpr::Var("x".to_string())),
            orelse: Box::new(lit_int(0)),
        };
        assert!(is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_if_expr_orelse() {
        let expr = HirExpr::IfExpr {
            test: Box::new(lit_bool(false)),
            body: Box::new(lit_int(1)),
            orelse: Box::new(HirExpr::Var("y".to_string())),
        };
        assert!(is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_used_in_lambda() {
        let expr = HirExpr::Lambda {
            params: vec![],
            body: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_slice_base() {
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: None,
            stop: None,
            step: None,
        };
        assert!(is_var_used_in_expr("arr", &expr));
    }

    #[test]
    fn test_var_used_in_slice_start() {
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: Some(Box::new(HirExpr::Var("i".to_string()))),
            stop: None,
            step: None,
        };
        assert!(is_var_used_in_expr("i", &expr));
    }

    #[test]
    fn test_var_used_in_slice_stop() {
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: None,
            stop: Some(Box::new(HirExpr::Var("j".to_string()))),
            step: None,
        };
        assert!(is_var_used_in_expr("j", &expr));
    }

    #[test]
    fn test_var_used_in_slice_step() {
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: None,
            stop: None,
            step: Some(Box::new(HirExpr::Var("s".to_string()))),
        };
        assert!(is_var_used_in_expr("s", &expr));
    }

    #[test]
    fn test_var_used_in_await() {
        let expr = HirExpr::Await {
            value: Box::new(HirExpr::Var("future".to_string())),
        };
        assert!(is_var_used_in_expr("future", &expr));
    }

    #[test]
    fn test_var_used_in_yield_some() {
        let expr = HirExpr::Yield {
            value: Some(Box::new(HirExpr::Var("x".to_string()))),
        };
        assert!(is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_yield_none() {
        let expr = HirExpr::Yield { value: None };
        assert!(!is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_not_used_in_literal_int() {
        let expr = lit_int(42);
        assert!(!is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_not_used_in_literal_str() {
        let expr = lit_str("hello");
        assert!(!is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_not_used_in_literal_bool() {
        let expr = lit_bool(true);
        assert!(!is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_generator_element() {
        let expr = HirExpr::GeneratorExp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![HirComprehension {
                target: "i".to_string(),
                iter: Box::new(HirExpr::Var("items".to_string())),
                conditions: vec![],
            }],
        };
        assert!(is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_var_used_in_generator_iter() {
        let expr = HirExpr::GeneratorExp {
            element: Box::new(lit_int(1)),
            generators: vec![HirComprehension {
                target: "i".to_string(),
                iter: Box::new(HirExpr::Var("data".to_string())),
                conditions: vec![],
            }],
        };
        assert!(is_var_used_in_expr("data", &expr));
    }

    #[test]
    fn test_var_used_in_generator_condition() {
        let expr = HirExpr::GeneratorExp {
            element: Box::new(lit_int(1)),
            generators: vec![HirComprehension {
                target: "i".to_string(),
                iter: Box::new(HirExpr::List(vec![])),
                conditions: vec![HirExpr::Var("filter_val".to_string())],
            }],
        };
        assert!(is_var_used_in_expr("filter_val", &expr));
    }

    #[test]
    fn test_var_used_in_list_comp() {
        let expr = HirExpr::ListComp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(HirExpr::Var("items".to_string())),
                conditions: vec![],
            }],
        };
        assert!(is_var_used_in_expr("items", &expr));
    }

    #[test]
    fn test_var_used_in_set_comp() {
        let expr = HirExpr::SetComp {
            element: Box::new(HirExpr::Var("y".to_string())),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(HirExpr::Var("source".to_string())),
                conditions: vec![],
            }],
        };
        assert!(is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_used_in_dict_comp_key() {
        let expr = HirExpr::DictComp {
            key: Box::new(HirExpr::Var("k".to_string())),
            value: Box::new(lit_int(1)),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(HirExpr::List(vec![])),
                conditions: vec![],
            }],
        };
        assert!(is_var_used_in_expr("k", &expr));
    }

    #[test]
    fn test_var_used_in_dict_comp_value() {
        let expr = HirExpr::DictComp {
            key: Box::new(lit_str("key")),
            value: Box::new(HirExpr::Var("v".to_string())),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(HirExpr::List(vec![])),
                conditions: vec![],
            }],
        };
        assert!(is_var_used_in_expr("v", &expr));
    }

    // ============================================================================
    // is_var_used_in_assign_target tests
    // ============================================================================

    #[test]
    fn test_assign_target_symbol() {
        let target = AssignTarget::Symbol("x".to_string());
        assert!(is_var_used_in_assign_target("x", &target));
        assert!(!is_var_used_in_assign_target("y", &target));
    }

    #[test]
    fn test_assign_target_index_base() {
        let target = AssignTarget::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(lit_int(0)),
        };
        assert!(is_var_used_in_assign_target("arr", &target));
    }

    #[test]
    fn test_assign_target_index_index() {
        let target = AssignTarget::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("i".to_string())),
        };
        assert!(is_var_used_in_assign_target("i", &target));
    }

    #[test]
    fn test_assign_target_attribute() {
        let target = AssignTarget::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        assert!(is_var_used_in_assign_target("obj", &target));
    }

    #[test]
    fn test_assign_target_tuple() {
        let target = AssignTarget::Tuple(vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ]);
        assert!(is_var_used_in_assign_target("a", &target));
        assert!(is_var_used_in_assign_target("b", &target));
        assert!(!is_var_used_in_assign_target("c", &target));
    }

    // ============================================================================
    // is_var_direct_or_simple_in_expr tests
    // ============================================================================

    #[test]
    fn test_direct_var_match() {
        let expr = HirExpr::Var("x".to_string());
        assert!(is_var_direct_or_simple_in_expr("x", &expr));
    }

    #[test]
    fn test_direct_var_no_match() {
        let expr = HirExpr::Var("y".to_string());
        assert!(!is_var_direct_or_simple_in_expr("x", &expr));
    }

    #[test]
    fn test_direct_not_var() {
        let expr = lit_int(42);
        assert!(!is_var_direct_or_simple_in_expr("x", &expr));
    }

    #[test]
    fn test_direct_nested_not_direct() {
        // Binary expression containing var is NOT direct
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(lit_int(1)),
        };
        assert!(!is_var_direct_or_simple_in_expr("x", &expr));
    }

    // ============================================================================
    // is_var_used_as_dict_key_in_expr tests
    // ============================================================================

    #[test]
    fn test_dict_key_index_pattern() {
        // dict[key] pattern
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("mydict".to_string())),
            index: Box::new(HirExpr::Var("k".to_string())),
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_get_pattern() {
        // dict.get(key) pattern
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("mydict".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Var("k".to_string())],
            kwargs: vec![],
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }),
            right: Box::new(lit_int(1)),
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_not_base() {
        // Using var as base (arr[0]) is not dict key usage
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(lit_int(0)),
        };
        assert!(!is_var_used_as_dict_key_in_expr("arr", &expr));
    }

    // ============================================================================
    // is_var_used_as_dict_key_in_stmt tests
    // ============================================================================

    #[test]
    fn test_dict_key_in_assign_target() {
        // dict[key] = value
        let stmt = HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            },
            value: lit_int(1),
            type_annotation: None,
        };
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    #[test]
    fn test_dict_key_in_if_then() {
        let stmt = HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Expr(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            })],
            else_body: None,
        };
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    // ============================================================================
    // is_var_reassigned_in_stmt tests
    // ============================================================================

    #[test]
    fn test_var_reassigned_simple() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: lit_int(1),
            type_annotation: None,
        };
        assert!(is_var_reassigned_in_stmt("x", &stmt));
        assert!(!is_var_reassigned_in_stmt("y", &stmt));
    }

    #[test]
    fn test_var_reassigned_in_if_then() {
        let stmt = HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            else_body: None,
        };
        assert!(is_var_reassigned_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_reassigned_in_if_else() {
        let stmt = HirStmt::If {
            condition: lit_bool(false),
            then_body: vec![],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: lit_int(2),
                type_annotation: None,
            }]),
        };
        assert!(is_var_reassigned_in_stmt("y", &stmt));
    }

    #[test]
    fn test_var_reassigned_in_while() {
        let stmt = HirStmt::While {
            condition: lit_bool(true),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("i".to_string()),
                value: HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("i".to_string())),
                    right: Box::new(lit_int(1)),
                },
                type_annotation: None,
            }],
        };
        assert!(is_var_reassigned_in_stmt("i", &stmt));
    }

    #[test]
    fn test_var_reassigned_in_for() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("x".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(0),
                type_annotation: None,
            }],
        };
        assert!(is_var_reassigned_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_not_reassigned_in_index_target() {
        // arr[i] = val is NOT reassignment of arr
        let stmt = HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(lit_int(0)),
            },
            value: lit_int(1),
            type_annotation: None,
        };
        assert!(!is_var_reassigned_in_stmt("arr", &stmt));
    }

    // ============================================================================
    // is_var_used_in_stmt tests
    // ============================================================================

    #[test]
    fn test_var_used_in_assign_value() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("y".to_string()),
            value: HirExpr::Var("x".to_string()),
            type_annotation: None,
        };
        assert!(is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_used_in_assign_target_index() {
        // d[k] = v - k is used in target
        let stmt = HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            },
            value: lit_int(1),
            type_annotation: None,
        };
        assert!(is_var_used_in_stmt("k", &stmt));
        assert!(is_var_used_in_stmt("d", &stmt));
    }

    #[test]
    fn test_var_used_in_if_condition() {
        let stmt = HirStmt::If {
            condition: HirExpr::Var("cond".to_string()),
            then_body: vec![],
            else_body: None,
        };
        assert!(is_var_used_in_stmt("cond", &stmt));
    }

    #[test]
    fn test_var_used_in_while_condition() {
        let stmt = HirStmt::While {
            condition: HirExpr::Var("running".to_string()),
            body: vec![],
        };
        assert!(is_var_used_in_stmt("running", &stmt));
    }

    #[test]
    fn test_var_used_in_for_iter() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![],
        };
        assert!(is_var_used_in_stmt("items", &stmt));
    }

    #[test]
    fn test_var_used_in_return() {
        let stmt = HirStmt::Return(Some(HirExpr::Var("result".to_string())));
        assert!(is_var_used_in_stmt("result", &stmt));
    }

    #[test]
    fn test_var_used_in_expr_stmt() {
        let stmt = HirStmt::Expr(HirExpr::Var("x".to_string()));
        assert!(is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_used_in_raise() {
        let stmt = HirStmt::Raise {
            exception: Some(HirExpr::Var("err".to_string())),
            cause: None,
        };
        assert!(is_var_used_in_stmt("err", &stmt));
    }

    #[test]
    fn test_var_used_in_assert_test() {
        let stmt = HirStmt::Assert {
            test: HirExpr::Var("condition".to_string()),
            msg: None,
        };
        assert!(is_var_used_in_stmt("condition", &stmt));
    }

    #[test]
    fn test_var_used_in_assert_msg() {
        let stmt = HirStmt::Assert {
            test: lit_bool(true),
            msg: Some(HirExpr::Var("msg".to_string())),
        };
        assert!(is_var_used_in_stmt("msg", &stmt));
    }

    // ============================================================================
    // extract_assigned_symbols tests
    // ============================================================================

    #[test]
    fn test_extract_simple_assignment() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: lit_int(1),
            type_annotation: None,
        }];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
        assert_eq!(symbols.len(), 1);
    }

    #[test]
    fn test_extract_multiple_assignments() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(1),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: lit_int(2),
                type_annotation: None,
            },
        ];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
        assert!(symbols.contains("y"));
        assert_eq!(symbols.len(), 2);
    }

    #[test]
    fn test_extract_from_if_body() {
        let stmts = vec![HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("a".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("b".to_string()),
                value: lit_int(2),
                type_annotation: None,
            }]),
        }];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("a"));
        assert!(symbols.contains("b"));
    }

    #[test]
    fn test_extract_from_while_body() {
        let stmts = vec![HirStmt::While {
            condition: lit_bool(true),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("i".to_string()),
                value: lit_int(0),
                type_annotation: None,
            }],
        }];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("i"));
    }

    #[test]
    fn test_extract_from_for_body() {
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("x".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("sum".to_string()),
                value: lit_int(0),
                type_annotation: None,
            }],
        }];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("sum"));
    }

    #[test]
    fn test_extract_empty() {
        let stmts: Vec<HirStmt> = vec![];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.is_empty());
    }

    // ============================================================================
    // extract_toplevel_assigned_symbols tests
    // ============================================================================

    #[test]
    fn test_toplevel_simple() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: lit_int(1),
            type_annotation: None,
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
    }

    #[test]
    fn test_toplevel_tuple_unpacking() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Symbol("b".to_string()),
            ]),
            value: HirExpr::Tuple(vec![lit_int(1), lit_int(2)]),
            type_annotation: None,
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.contains("a"));
        assert!(symbols.contains("b"));
    }

    #[test]
    fn test_toplevel_includes_if() {
        let stmts = vec![HirStmt::If {
            condition: lit_bool(true),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            else_body: None,
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
    }

    #[test]
    fn test_toplevel_excludes_for() {
        // DEPYLER-0476: Variables in for loops should NOT be extracted
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("loop_var".to_string()),
                value: lit_int(0),
                type_annotation: None,
            }],
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(!symbols.contains("loop_var"));
    }

    #[test]
    fn test_toplevel_excludes_while() {
        // DEPYLER-0476: Variables in while loops should NOT be extracted
        let stmts = vec![HirStmt::While {
            condition: lit_bool(true),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("loop_var".to_string()),
                value: lit_int(0),
                type_annotation: None,
            }],
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(!symbols.contains("loop_var"));
    }

    // ============================================================================
    // extract_walrus_from_condition tests
    // ============================================================================

    #[test]
    fn test_walrus_simple() {
        // if (x := 5) > 0
        let condition = HirExpr::Binary {
            op: BinOp::Gt,
            left: Box::new(HirExpr::NamedExpr {
                target: "x".to_string(),
                value: Box::new(lit_int(5)),
            }),
            right: Box::new(lit_int(0)),
        };
        let (assigns, simplified) = extract_walrus_from_condition(&condition);

        assert_eq!(assigns.len(), 1);
        assert_eq!(assigns[0].0, "x");
        assert!(matches!(assigns[0].1, HirExpr::Literal(Literal::Int(5))));

        // Simplified should have Var("x") instead of NamedExpr
        if let HirExpr::Binary { left, .. } = simplified {
            assert!(matches!(*left, HirExpr::Var(ref n) if n == "x"));
        } else {
            panic!("Expected Binary");
        }
    }

    #[test]
    fn test_walrus_no_walrus() {
        let condition = HirExpr::Binary {
            op: BinOp::Gt,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(lit_int(0)),
        };
        let (assigns, _simplified) = extract_walrus_from_condition(&condition);
        assert!(assigns.is_empty());
    }

    #[test]
    fn test_walrus_in_call() {
        // if len(x := get_data()) > 0
        let condition = HirExpr::Binary {
            op: BinOp::Gt,
            left: Box::new(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::NamedExpr {
                    target: "x".to_string(),
                    value: Box::new(HirExpr::Call {
                        func: "get_data".to_string(),
                        args: vec![],
                        kwargs: vec![],
                    }),
                }],
                kwargs: vec![],
            }),
            right: Box::new(lit_int(0)),
        };
        let (assigns, _simplified) = extract_walrus_from_condition(&condition);

        assert_eq!(assigns.len(), 1);
        assert_eq!(assigns[0].0, "x");
    }

    #[test]
    fn test_walrus_recursive_processes_value() {
        let mut assigns = Vec::new();
        let expr = HirExpr::NamedExpr {
            target: "y".to_string(),
            value: Box::new(HirExpr::NamedExpr {
                target: "x".to_string(),
                value: Box::new(lit_int(10)),
            }),
        };
        let _simplified = extract_walrus_recursive(&expr, &mut assigns);

        // Both x and y should be extracted
        assert_eq!(assigns.len(), 2);
    }

    #[test]
    fn test_walrus_in_unary() {
        let mut assigns = Vec::new();
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::NamedExpr {
                target: "flag".to_string(),
                value: Box::new(lit_bool(true)),
            }),
        };
        let simplified = extract_walrus_recursive(&expr, &mut assigns);

        assert_eq!(assigns.len(), 1);
        assert_eq!(assigns[0].0, "flag");

        if let HirExpr::Unary { operand, .. } = simplified {
            assert!(matches!(*operand, HirExpr::Var(ref n) if n == "flag"));
        }
    }

    #[test]
    fn test_walrus_in_method_call() {
        let mut assigns = Vec::new();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::NamedExpr {
                target: "obj".to_string(),
                value: Box::new(HirExpr::Call {
                    func: "get_obj".to_string(),
                    args: vec![],
                    kwargs: vec![],
                }),
            }),
            method: "process".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        let simplified = extract_walrus_recursive(&expr, &mut assigns);

        assert_eq!(assigns.len(), 1);
        assert_eq!(assigns[0].0, "obj");

        if let HirExpr::MethodCall { object, .. } = simplified {
            assert!(matches!(*object, HirExpr::Var(ref n) if n == "obj"));
        }
    }

    #[test]
    fn test_walrus_preserves_other_exprs() {
        let mut assigns = Vec::new();
        let expr = lit_int(42);
        let simplified = extract_walrus_recursive(&expr, &mut assigns);

        assert!(assigns.is_empty());
        assert!(matches!(simplified, HirExpr::Literal(Literal::Int(42))));
    }

    // ============================================================================
    // Additional coverage tests for FString
    // ============================================================================

    #[test]
    fn test_var_used_in_fstring_expr_part() {
        let expr = HirExpr::FString {
            parts: vec![
                FStringPart::Literal("Value: ".to_string()),
                FStringPart::Expr(Box::new(HirExpr::Var("x".to_string()))),
            ],
        };
        assert!(is_var_used_in_expr("x", &expr));
        assert!(!is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_var_not_used_in_fstring_literal() {
        let expr = HirExpr::FString {
            parts: vec![FStringPart::Literal("hello world".to_string())],
        };
        assert!(!is_var_used_in_expr("x", &expr));
    }

    // ============================================================================
    // Additional is_var_used_as_dict_key tests
    // ============================================================================

    #[test]
    fn test_dict_key_in_unary() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }),
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_call_args() {
        let expr = HirExpr::Call {
            func: "process".to_string(),
            args: vec![HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }],
            kwargs: vec![],
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_method_call_object() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }),
            method: "process".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_method_call_args() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "process".to_string(),
            args: vec![HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }],
            kwargs: vec![],
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_if_expr_test() {
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }),
            body: Box::new(lit_int(1)),
            orelse: Box::new(lit_int(0)),
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_if_expr_body() {
        let expr = HirExpr::IfExpr {
            test: Box::new(lit_bool(true)),
            body: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }),
            orelse: Box::new(lit_int(0)),
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_if_expr_orelse() {
        let expr = HirExpr::IfExpr {
            test: Box::new(lit_bool(false)),
            body: Box::new(lit_int(1)),
            orelse: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }),
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_list() {
        let expr = HirExpr::List(vec![HirExpr::Index {
            base: Box::new(HirExpr::Var("d".to_string())),
            index: Box::new(HirExpr::Var("k".to_string())),
        }]);
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_tuple() {
        let expr = HirExpr::Tuple(vec![HirExpr::Index {
            base: Box::new(HirExpr::Var("d".to_string())),
            index: Box::new(HirExpr::Var("k".to_string())),
        }]);
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_set() {
        let expr = HirExpr::Set(vec![HirExpr::Index {
            base: Box::new(HirExpr::Var("d".to_string())),
            index: Box::new(HirExpr::Var("k".to_string())),
        }]);
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_not_found_literal() {
        let expr = lit_int(42);
        assert!(!is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_recurse_into_nested_index() {
        // Nested index: dict[other_dict[k]] - k is in nested index
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("d".to_string())),
            index: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("other".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }),
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    #[test]
    fn test_dict_key_in_get_object_recurse() {
        // d.get(key).method() - key is in get
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("d".to_string())),
                method: "get".to_string(),
                args: vec![lit_str("other_key")],
                kwargs: vec![],
            }),
            method: "process".to_string(),
            args: vec![HirExpr::Index {
                base: Box::new(HirExpr::Var("inner".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            }],
            kwargs: vec![],
        };
        assert!(is_var_used_as_dict_key_in_expr("k", &expr));
    }

    // ============================================================================
    // is_var_used_as_dict_key_in_stmt additional tests
    // ============================================================================

    #[test]
    fn test_dict_key_in_if_condition() {
        let stmt = HirStmt::If {
            condition: HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            },
            then_body: vec![],
            else_body: None,
        };
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    #[test]
    fn test_dict_key_in_if_else_body() {
        let stmt = HirStmt::If {
            condition: lit_bool(false),
            then_body: vec![],
            else_body: Some(vec![HirStmt::Expr(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            })]),
        };
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    #[test]
    fn test_dict_key_in_while_condition() {
        let stmt = HirStmt::While {
            condition: HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            },
            body: vec![],
        };
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    #[test]
    fn test_dict_key_in_while_body() {
        let stmt = HirStmt::While {
            condition: lit_bool(true),
            body: vec![HirStmt::Expr(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            })],
        };
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    #[test]
    fn test_dict_key_in_for_iter() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            },
            body: vec![],
        };
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    #[test]
    fn test_dict_key_in_for_body() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Expr(HirExpr::Index {
                base: Box::new(HirExpr::Var("d".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            })],
        };
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    #[test]
    fn test_dict_key_in_return() {
        let stmt = HirStmt::Return(Some(HirExpr::Index {
            base: Box::new(HirExpr::Var("d".to_string())),
            index: Box::new(HirExpr::Var("k".to_string())),
        }));
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    #[test]
    fn test_dict_key_in_expr_stmt() {
        let stmt = HirStmt::Expr(HirExpr::Index {
            base: Box::new(HirExpr::Var("d".to_string())),
            index: Box::new(HirExpr::Var("k".to_string())),
        });
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    #[test]
    fn test_dict_key_not_found_break() {
        let stmt = HirStmt::Break { label: None };
        assert!(!is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    // ============================================================================
    // is_var_reassigned_in_stmt Try/With tests
    // ============================================================================

    #[test]
    fn test_var_reassigned_in_try_body() {
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            handlers: vec![],
            orelse: None,
            finalbody: None,
        };
        assert!(is_var_reassigned_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_reassigned_in_try_handler() {
        use crate::hir::ExceptHandler;
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: lit_int(1),
                    type_annotation: None,
                }],
            }],
            orelse: None,
            finalbody: None,
        };
        assert!(is_var_reassigned_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_reassigned_in_try_orelse() {
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: lit_int(2),
                type_annotation: None,
            }]),
            finalbody: None,
        };
        assert!(is_var_reassigned_in_stmt("y", &stmt));
    }

    #[test]
    fn test_var_reassigned_in_try_finalbody() {
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("z".to_string()),
                value: lit_int(3),
                type_annotation: None,
            }]),
        };
        assert!(is_var_reassigned_in_stmt("z", &stmt));
    }

    #[test]
    fn test_var_reassigned_in_with_body() {
        let stmt = HirStmt::With {
            context: HirExpr::Call {
                func: "open".to_string(),
                args: vec![lit_str("file.txt")],
                kwargs: vec![],
            },
            target: Some("f".to_string()),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("data".to_string()),
                value: lit_str("content"),
                type_annotation: None,
            }],
            is_async: false,
        };
        assert!(is_var_reassigned_in_stmt("data", &stmt));
    }

    #[test]
    fn test_var_not_reassigned_in_pass() {
        let stmt = HirStmt::Pass;
        assert!(!is_var_reassigned_in_stmt("x", &stmt));
    }

    // ============================================================================
    // is_var_used_in_stmt Try/With tests
    // ============================================================================

    #[test]
    fn test_var_used_in_try_body() {
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
            handlers: vec![],
            orelse: None,
            finalbody: None,
        };
        assert!(is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_used_in_try_handler() {
        use crate::hir::ExceptHandler;
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Expr(HirExpr::Var("y".to_string()))],
            }],
            orelse: None,
            finalbody: None,
        };
        assert!(is_var_used_in_stmt("y", &stmt));
    }

    #[test]
    fn test_var_used_in_try_orelse() {
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: Some(vec![HirStmt::Expr(HirExpr::Var("z".to_string()))]),
            finalbody: None,
        };
        assert!(is_var_used_in_stmt("z", &stmt));
    }

    #[test]
    fn test_var_used_in_try_finalbody() {
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Expr(HirExpr::Var("w".to_string()))]),
        };
        assert!(is_var_used_in_stmt("w", &stmt));
    }

    #[test]
    fn test_var_used_in_with_body() {
        let stmt = HirStmt::With {
            context: HirExpr::Call {
                func: "open".to_string(),
                args: vec![lit_str("file.txt")],
                kwargs: vec![],
            },
            target: Some("f".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Var("data".to_string()))],
            is_async: false,
        };
        assert!(is_var_used_in_stmt("data", &stmt));
    }

    #[test]
    fn test_var_not_used_in_return_none() {
        let stmt = HirStmt::Return(None);
        assert!(!is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_not_used_in_raise_no_exception() {
        let stmt = HirStmt::Raise {
            exception: None,
            cause: None,
        };
        assert!(!is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_not_used_in_pass() {
        let stmt = HirStmt::Pass;
        assert!(!is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_not_used_in_break() {
        let stmt = HirStmt::Break { label: None };
        assert!(!is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_var_not_used_in_continue() {
        let stmt = HirStmt::Continue { label: None };
        assert!(!is_var_used_in_stmt("x", &stmt));
    }

    // ============================================================================
    // extract_assigned_symbols Try tests
    // ============================================================================

    #[test]
    fn test_extract_from_try_body() {
        let stmts = vec![HirStmt::Try {
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            handlers: vec![],
            orelse: None,
            finalbody: None,
        }];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
    }

    #[test]
    fn test_extract_from_try_handler() {
        use crate::hir::ExceptHandler;
        let stmts = vec![HirStmt::Try {
            body: vec![],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("y".to_string()),
                    value: lit_int(2),
                    type_annotation: None,
                }],
            }],
            orelse: None,
            finalbody: None,
        }];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("y"));
    }

    #[test]
    fn test_extract_from_try_finalbody() {
        let stmts = vec![HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("z".to_string()),
                value: lit_int(3),
                type_annotation: None,
            }]),
        }];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("z"));
    }

    #[test]
    fn test_extract_ignores_non_symbol_targets() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(lit_int(0)),
            },
            value: lit_int(1),
            type_annotation: None,
        }];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.is_empty());
    }

    // ============================================================================
    // extract_toplevel_assigned_symbols Try tests
    // ============================================================================

    #[test]
    fn test_toplevel_from_try_body() {
        let stmts = vec![HirStmt::Try {
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: lit_int(1),
                type_annotation: None,
            }],
            handlers: vec![],
            orelse: None,
            finalbody: None,
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
    }

    #[test]
    fn test_toplevel_from_try_handler() {
        use crate::hir::ExceptHandler;
        let stmts = vec![HirStmt::Try {
            body: vec![],
            handlers: vec![ExceptHandler {
                exception_type: Some("Exception".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("y".to_string()),
                    value: lit_int(2),
                    type_annotation: None,
                }],
            }],
            orelse: None,
            finalbody: None,
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.contains("y"));
    }

    #[test]
    fn test_toplevel_from_try_finalbody() {
        let stmts = vec![HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("z".to_string()),
                value: lit_int(3),
                type_annotation: None,
            }]),
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.contains("z"));
    }

    #[test]
    fn test_toplevel_nested_tuple_in_tuple() {
        // Only extracts first-level symbols
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Tuple(vec![
                    AssignTarget::Symbol("b".to_string()),
                    AssignTarget::Symbol("c".to_string()),
                ]),
            ]),
            value: HirExpr::Tuple(vec![lit_int(1), HirExpr::Tuple(vec![lit_int(2), lit_int(3)])]),
            type_annotation: None,
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        // Only "a" should be extracted (first-level Symbol)
        // Nested tuples are not recursively traversed
        assert!(symbols.contains("a"));
    }

    #[test]
    fn test_toplevel_ignores_non_symbol_targets() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(lit_int(0)),
            },
            value: lit_int(1),
            type_annotation: None,
        }];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_toplevel_empty_stmts() {
        let stmts: Vec<HirStmt> = vec![];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_toplevel_ignores_pass() {
        let stmts = vec![HirStmt::Pass];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.is_empty());
    }

    // ============ find_var_position_in_tuple tests ============

    #[test]
    fn test_find_var_position_first() {
        let targets = vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
            AssignTarget::Symbol("c".to_string()),
        ];
        assert_eq!(find_var_position_in_tuple("a", &targets), Some(0));
    }

    #[test]
    fn test_find_var_position_middle() {
        let targets = vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
            AssignTarget::Symbol("c".to_string()),
        ];
        assert_eq!(find_var_position_in_tuple("b", &targets), Some(1));
    }

    #[test]
    fn test_find_var_position_not_found() {
        let targets = vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ];
        assert_eq!(find_var_position_in_tuple("c", &targets), None);
    }

    #[test]
    fn test_find_var_position_empty() {
        let targets: Vec<AssignTarget> = vec![];
        assert_eq!(find_var_position_in_tuple("a", &targets), None);
    }

    // ============ find_assigned_expr tests ============

    #[test]
    fn test_find_assigned_expr_found() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: lit_int(42),
            type_annotation: None,
        }];
        let result = find_assigned_expr("x", &stmts);
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), HirExpr::Literal(Literal::Int(42))));
    }

    #[test]
    fn test_find_assigned_expr_not_found() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: lit_int(42),
            type_annotation: None,
        }];
        assert!(find_assigned_expr("y", &stmts).is_none());
    }

    #[test]
    fn test_find_assigned_expr_empty() {
        let stmts: Vec<HirStmt> = vec![];
        assert!(find_assigned_expr("x", &stmts).is_none());
    }

    // ============ needs_boxed_dyn_write tests ============

    #[test]
    fn test_needs_boxed_dyn_write_file_and_stdout() {
        // file = open("test.txt", "w") in then branch
        // file = sys.stdout in else branch
        let then_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("file".to_string()),
            value: HirExpr::Call {
                func: "open".to_string(),
                args: vec![
                    HirExpr::Literal(Literal::String("test.txt".to_string())),
                    HirExpr::Literal(Literal::String("w".to_string())),
                ],
                kwargs: vec![],
            },
            type_annotation: None,
        }];
        let else_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("file".to_string()),
            value: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("sys".to_string())),
                attr: "stdout".to_string(),
            },
            type_annotation: None,
        }];
        assert!(needs_boxed_dyn_write("file", &then_body, &else_body));
    }

    #[test]
    fn test_needs_boxed_dyn_write_same_type() {
        // Both branches assign stdout - same type, no boxing needed
        let then_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("file".to_string()),
            value: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("sys".to_string())),
                attr: "stdout".to_string(),
            },
            type_annotation: None,
        }];
        let else_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("file".to_string()),
            value: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("sys".to_string())),
                attr: "stderr".to_string(),
            },
            type_annotation: None,
        }];
        // Both are stdio, so no boxing needed
        assert!(!needs_boxed_dyn_write("file", &then_body, &else_body));
    }

    #[test]
    fn test_needs_boxed_dyn_write_missing_assignment() {
        let then_body: Vec<HirStmt> = vec![];
        let else_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("file".to_string()),
            value: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("sys".to_string())),
                attr: "stdout".to_string(),
            },
            type_annotation: None,
        }];
        assert!(!needs_boxed_dyn_write("file", &then_body, &else_body));
    }
}
