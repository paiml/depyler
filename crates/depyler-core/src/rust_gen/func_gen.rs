//! Function code generation
//!
//! This module handles converting HIR functions to Rust token streams.
//! It includes all function conversion helpers and the HirFunction RustCodeGen trait implementation.

use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, RustCodeGen};
// DEPYLER-COVERAGE-95: Advanced inference helpers extracted to func_gen_inference
use crate::rust_gen::func_gen_inference::detect_returns_nested_function;
#[allow(unused_imports)] // DEPYLER-COVERAGE-95: Some imports only used in tests
use crate::rust_gen::control_flow_analysis::{
    collect_all_assigned_variables, collect_if_escaping_variables,
    collect_loop_escaping_variables, collect_nested_function_names,
    extract_toplevel_assigned_symbols, stmt_always_returns,
};
use crate::rust_gen::keywords::{is_rust_keyword, safe_ident}; // DEPYLER-0023: Centralized
use crate::rust_gen::type_gen::{rust_type_to_syn, update_import_needs};
use anyhow::Result;
use quote::quote;
use syn::{self, parse_quote};

/// DEPYLER-0608: Extract field names accessed via args.X pattern in a function body
/// Used to generate individual parameters for cmd_* handler functions
/// instead of taking &Args (which doesn't have subcommand fields)
fn extract_args_field_accesses(body: &[HirStmt], args_name: &str) -> Vec<String> {
    use std::collections::HashSet;
    let mut fields: HashSet<String> = HashSet::new();

    fn walk_expr(expr: &HirExpr, args_name: &str, fields: &mut HashSet<String>) {
        match expr {
            HirExpr::Attribute { value, attr } => {
                // Check if this is args.X pattern
                if let HirExpr::Var(name) = value.as_ref() {
                    if name == args_name {
                        fields.insert(attr.clone());
                    }
                }
                walk_expr(value, args_name, fields);
            }
            HirExpr::Binary { left, right, .. } => {
                walk_expr(left, args_name, fields);
                walk_expr(right, args_name, fields);
            }
            HirExpr::Unary { operand, .. } => {
                walk_expr(operand, args_name, fields);
            }
            HirExpr::Call { args: call_args, kwargs, .. } => {
                // Note: func is Symbol, not Box<HirExpr>, so don't walk it
                for arg in call_args {
                    walk_expr(arg, args_name, fields);
                }
                for (_, kwarg_val) in kwargs {
                    walk_expr(kwarg_val, args_name, fields);
                }
            }
            HirExpr::MethodCall { object, args: method_args, kwargs, .. } => {
                walk_expr(object, args_name, fields);
                for arg in method_args {
                    walk_expr(arg, args_name, fields);
                }
                for (_, kwarg_val) in kwargs {
                    walk_expr(kwarg_val, args_name, fields);
                }
            }
            HirExpr::List(elems) | HirExpr::Tuple(elems) | HirExpr::Set(elems) => {
                for elem in elems {
                    walk_expr(elem, args_name, fields);
                }
            }
            HirExpr::Dict(items) => {
                for (key, value) in items {
                    walk_expr(key, args_name, fields);
                    walk_expr(value, args_name, fields);
                }
            }
            HirExpr::Index { base, index } => {
                walk_expr(base, args_name, fields);
                walk_expr(index, args_name, fields);
            }
            HirExpr::IfExpr { test, body, orelse } => {
                walk_expr(test, args_name, fields);
                walk_expr(body, args_name, fields);
                walk_expr(orelse, args_name, fields);
            }
            HirExpr::FString { parts } => {
                for part in parts {
                    if let crate::hir::FStringPart::Expr(fstring_expr) = part {
                        walk_expr(fstring_expr, args_name, fields);
                    }
                }
            }
            HirExpr::Slice { base, start, stop, step } => {
                walk_expr(base, args_name, fields);
                if let Some(s) = start {
                    walk_expr(s, args_name, fields);
                }
                if let Some(s) = stop {
                    walk_expr(s, args_name, fields);
                }
                if let Some(s) = step {
                    walk_expr(s, args_name, fields);
                }
            }
            HirExpr::ListComp { element, generators } | HirExpr::SetComp { element, generators } => {
                walk_expr(element, args_name, fields);
                for gen in generators {
                    walk_expr(&gen.iter, args_name, fields);
                    for cond in &gen.conditions {
                        walk_expr(cond, args_name, fields);
                    }
                }
            }
            HirExpr::DictComp { key, value, generators } => {
                walk_expr(key, args_name, fields);
                walk_expr(value, args_name, fields);
                for gen in generators {
                    walk_expr(&gen.iter, args_name, fields);
                    for cond in &gen.conditions {
                        walk_expr(cond, args_name, fields);
                    }
                }
            }
            HirExpr::Lambda { body, .. } => {
                walk_expr(body, args_name, fields);
            }
            HirExpr::Borrow { expr: borrow_expr, .. } => {
                walk_expr(borrow_expr, args_name, fields);
            }
            HirExpr::Yield { value: Some(v) } => {
                walk_expr(v, args_name, fields);
            }
            HirExpr::Yield { value: None } => {}
            HirExpr::Await { value } => {
                walk_expr(value, args_name, fields);
            }
            _ => {}
        }
    }

    fn walk_stmt(stmt: &HirStmt, args_name: &str, fields: &mut HashSet<String>) {
        match stmt {
            HirStmt::Expr(expr) => walk_expr(expr, args_name, fields),
            HirStmt::Assign { value, .. } => walk_expr(value, args_name, fields),
            HirStmt::Return(Some(expr)) => walk_expr(expr, args_name, fields),
            HirStmt::If { condition, then_body, else_body } => {
                walk_expr(condition, args_name, fields);
                for s in then_body {
                    walk_stmt(s, args_name, fields);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        walk_stmt(s, args_name, fields);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                walk_expr(condition, args_name, fields);
                for s in body {
                    walk_stmt(s, args_name, fields);
                }
            }
            HirStmt::For { iter, body, .. } => {
                walk_expr(iter, args_name, fields);
                for s in body {
                    walk_stmt(s, args_name, fields);
                }
            }
            HirStmt::With { context, body, .. } => {
                walk_expr(context, args_name, fields);
                for s in body {
                    walk_stmt(s, args_name, fields);
                }
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                for s in body {
                    walk_stmt(s, args_name, fields);
                }
                for handler in handlers {
                    for s in &handler.body {
                        walk_stmt(s, args_name, fields);
                    }
                }
                if let Some(else_stmts) = orelse {
                    for s in else_stmts {
                        walk_stmt(s, args_name, fields);
                    }
                }
                if let Some(final_stmts) = finalbody {
                    for s in final_stmts {
                        walk_stmt(s, args_name, fields);
                    }
                }
            }
            HirStmt::FunctionDef { body, .. } => {
                for s in body {
                    walk_stmt(s, args_name, fields);
                }
            }
            _ => {}
        }
    }

    for stmt in body {
        walk_stmt(stmt, args_name, &mut fields);
    }

    // Sort for deterministic output
    let mut result: Vec<String> = fields.into_iter().collect();
    result.sort();
    result
}

// DEPYLER-COVERAGE-95: stmt_always_returns moved to control_flow_analysis module
// DEPYLER-COVERAGE-95: codegen_generic_params, codegen_where_clause, codegen_function_attrs
//                      moved to func_gen_helpers module for testability

// ============================================================================
// DEPYLER-0141 Phase 2: Medium Complexity Helpers
// ============================================================================
// DEPYLER-COVERAGE-95: collect_nested_function_names moved to control_flow_analysis module

// ============================================================================
// DEPYLER-0762, DEPYLER-0834: Block-Escaping Variable Hoisting
// ============================================================================
// DEPYLER-COVERAGE-95: collect_if_escaping_variables, collect_loop_escaping_variables,
// collect_all_assigned_variables, extract_toplevel_assigned_symbols, is_var_used_in_remaining_stmts,
// is_var_used_anywhere moved to control_flow_analysis module

/// DEPYLER-0963: Find the type of a variable from its assignments in the function body.
/// Searches for the first assignment to the variable and infers the type from the value.
/// Returns Some(Type) if found, None otherwise.
///
/// Takes optional parameter types map to resolve variable references like `result = n`
/// where `n` is a function parameter with a known type.
#[allow(dead_code)]
fn find_var_type_in_body(var_name: &str, stmts: &[HirStmt]) -> Option<Type> {
    find_var_type_in_body_with_params(var_name, stmts, &std::collections::HashMap::new())
}

/// DEPYLER-0963: Find the type of a variable from its assignments, with parameter type context.
/// DEPYLER-0965: Fixed to look at ALL assignments, not just the first one.
/// When a variable has multiple assignments (e.g., `x = None` then `x = "hello"`),
/// we should find the first assignment that gives us a concrete type.
fn find_var_type_in_body_with_params(
    var_name: &str,
    stmts: &[HirStmt],
    param_types: &std::collections::HashMap<String, Type>,
) -> Option<Type> {
    // DEPYLER-0965: Collect all types from assignments to this variable
    // Return the first non-Unknown type found
    for stmt in stmts {
        match stmt {
            // Simple assignment: x = expr
            HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                type_annotation,
                value,
            } if name == var_name => {
                // First try explicit type annotation
                if type_annotation.is_some() {
                    return type_annotation.clone();
                }
                // If assigned from a variable, check if it's a known parameter
                if let HirExpr::Var(source_var) = value {
                    if let Some(ty) = param_types.get(source_var) {
                        return Some(ty.clone());
                    }
                }
                // DEPYLER-0265: If assigned from subscript (e.g., longest = words[0]),
                // infer element type from the base variable's type
                if let HirExpr::Index { base, .. } = value {
                    if let HirExpr::Var(base_var) = base.as_ref() {
                        if let Some(base_type) = param_types.get(base_var) {
                            let elem_type = match base_type {
                                Type::List(elem) => Some(*elem.clone()),
                                Type::Dict(_, val) => Some(*val.clone()),
                                Type::String => Some(Type::String),
                                Type::Tuple(elems) => elems.first().cloned(),
                                _ => None,
                            };
                            if let Some(ty) = elem_type {
                                if !matches!(ty, Type::Unknown) {
                                    return Some(ty);
                                }
                            }
                        }
                    }
                }
                // If no annotation, infer from the assigned value
                let inferred = infer_expr_type_simple(value);
                // DEPYLER-0965: Skip Unknown and None types - continue looking for concrete types
                // When variable is first assigned None (Type::None) but later assigned a string,
                // we want to infer String, not None (which maps to () in Rust)
                if !matches!(inferred, Type::Unknown | Type::None) {
                    return Some(inferred);
                }
                // DEPYLER-0965: Don't return None here - continue looking for more assignments
                // The first assignment might be `x = None` but later ones might have concrete types
            }
            // Tuple unpacking: (a, b) = (1, 2)
            HirStmt::Assign {
                target: AssignTarget::Tuple(targets),
                value,
                ..
            } => {
                // Find var_name position in the targets
                if let Some(pos) = targets.iter().position(|t| {
                    matches!(t, AssignTarget::Symbol(name) if name == var_name)
                }) {
                    // Check if RHS is a tuple expression
                    if let HirExpr::Tuple(elems) = value {
                        if pos < elems.len() {
                            let elem_type = infer_expr_type_simple(&elems[pos]);
                            if !matches!(elem_type, Type::Unknown) {
                                return Some(elem_type);
                            }
                        }
                    }
                    // Infer type from the RHS tuple type
                    let rhs_type = infer_expr_type_simple(value);
                    if let Type::Tuple(elem_types) = rhs_type {
                        if pos < elem_types.len() {
                            let elem_type = elem_types[pos].clone();
                            if !matches!(elem_type, Type::Unknown) {
                                return Some(elem_type);
                            }
                        }
                    }
                }
            }
            // Recurse into for loops
            HirStmt::For { body, .. } => {
                if let Some(ty) = find_var_type_in_body_with_params(var_name, body, param_types) {
                    return Some(ty);
                }
            }
            // Recurse into while loops
            HirStmt::While { body, .. } => {
                if let Some(ty) = find_var_type_in_body_with_params(var_name, body, param_types) {
                    return Some(ty);
                }
            }
            // Recurse into if/else blocks
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                if let Some(ty) = find_var_type_in_body_with_params(var_name, then_body, param_types) {
                    return Some(ty);
                }
                if let Some(else_stmts) = else_body {
                    if let Some(ty) = find_var_type_in_body_with_params(var_name, else_stmts, param_types) {
                        return Some(ty);
                    }
                }
            }
            // Recurse into try/except blocks
            HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => {
                if let Some(ty) = find_var_type_in_body_with_params(var_name, body, param_types) {
                    return Some(ty);
                }
                for handler in handlers {
                    if let Some(ty) = find_var_type_in_body_with_params(var_name, &handler.body, param_types) {
                        return Some(ty);
                    }
                }
                if let Some(finally) = finalbody {
                    if let Some(ty) = find_var_type_in_body_with_params(var_name, finally, param_types) {
                        return Some(ty);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Check if a variable is used in any of the remaining statements
#[allow(dead_code)]
fn is_var_used_in_remaining_stmts(var_name: &str, stmts: &[HirStmt]) -> bool {
    stmts.iter().any(|stmt| is_var_used_anywhere(var_name, stmt))
}

/// Check if a variable is used anywhere in a statement (recursive)
#[allow(dead_code)]
fn is_var_used_anywhere(var_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            is_var_used_in_target(var_name, target) || is_var_used_in_expr_any(var_name, value)
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            is_var_used_in_expr_any(var_name, condition)
                || then_body.iter().any(|s| is_var_used_anywhere(var_name, s))
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(|s| is_var_used_anywhere(var_name, s)))
        }
        HirStmt::While { condition, body } => {
            is_var_used_in_expr_any(var_name, condition)
                || body.iter().any(|s| is_var_used_anywhere(var_name, s))
        }
        HirStmt::For { iter, body, .. } => {
            is_var_used_in_expr_any(var_name, iter)
                || body.iter().any(|s| is_var_used_anywhere(var_name, s))
        }
        HirStmt::Return(Some(expr)) => is_var_used_in_expr_any(var_name, expr),
        HirStmt::Expr(expr) => is_var_used_in_expr_any(var_name, expr),
        HirStmt::Raise { exception, .. } => exception
            .as_ref()
            .is_some_and(|e| is_var_used_in_expr_any(var_name, e)),
        HirStmt::Assert { test, msg, .. } => {
            is_var_used_in_expr_any(var_name, test)
                || msg
                    .as_ref()
                    .is_some_and(|m| is_var_used_in_expr_any(var_name, m))
        }
        HirStmt::Try {
            body,
            handlers,
            orelse,
            finalbody,
        } => {
            body.iter().any(|s| is_var_used_anywhere(var_name, s))
                || handlers
                    .iter()
                    .any(|h| h.body.iter().any(|s| is_var_used_anywhere(var_name, s)))
                || orelse
                    .as_ref()
                    .is_some_and(|stmts| stmts.iter().any(|s| is_var_used_anywhere(var_name, s)))
                || finalbody
                    .as_ref()
                    .is_some_and(|stmts| stmts.iter().any(|s| is_var_used_anywhere(var_name, s)))
        }
        HirStmt::With { context, body, .. } => {
            is_var_used_in_expr_any(var_name, context)
                || body.iter().any(|s| is_var_used_anywhere(var_name, s))
        }
        _ => false,
    }
}

/// Check if variable is used in an assign target (e.g., d[x] = v uses x)
#[allow(dead_code)]
fn is_var_used_in_target(var_name: &str, target: &AssignTarget) -> bool {
    match target {
        AssignTarget::Symbol(_) => false, // Target itself doesn't use the var
        AssignTarget::Index { base, index } => {
            is_var_used_in_expr_any(var_name, base) || is_var_used_in_expr_any(var_name, index)
        }
        AssignTarget::Attribute { value, .. } => is_var_used_in_expr_any(var_name, value),
        AssignTarget::Tuple(targets) => targets.iter().any(|t| is_var_used_in_target(var_name, t)),
    }
}

/// Check if a variable is used in an expression (recursive)
#[allow(dead_code)]
fn is_var_used_in_expr_any(var_name: &str, expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Var(name) => name == var_name,
        HirExpr::Literal(_) => false,
        HirExpr::Binary { left, right, .. } => {
            is_var_used_in_expr_any(var_name, left) || is_var_used_in_expr_any(var_name, right)
        }
        HirExpr::Unary { operand, .. } => is_var_used_in_expr_any(var_name, operand),
        HirExpr::Call { func, args, kwargs } => {
            func == var_name
                || args.iter().any(|a| is_var_used_in_expr_any(var_name, a))
                || kwargs
                    .iter()
                    .any(|(_, v)| is_var_used_in_expr_any(var_name, v))
        }
        HirExpr::MethodCall {
            object,
            args,
            kwargs,
            ..
        } => {
            is_var_used_in_expr_any(var_name, object)
                || args.iter().any(|a| is_var_used_in_expr_any(var_name, a))
                || kwargs
                    .iter()
                    .any(|(_, v)| is_var_used_in_expr_any(var_name, v))
        }
        HirExpr::DynamicCall {
            callee,
            args,
            kwargs,
        } => {
            is_var_used_in_expr_any(var_name, callee)
                || args.iter().any(|a| is_var_used_in_expr_any(var_name, a))
                || kwargs
                    .iter()
                    .any(|(_, v)| is_var_used_in_expr_any(var_name, v))
        }
        HirExpr::Attribute { value, .. } => is_var_used_in_expr_any(var_name, value),
        HirExpr::Index { base, index } => {
            is_var_used_in_expr_any(var_name, base) || is_var_used_in_expr_any(var_name, index)
        }
        HirExpr::Slice {
            base,
            start,
            stop,
            step,
        } => {
            is_var_used_in_expr_any(var_name, base)
                || start
                    .as_ref()
                    .is_some_and(|s| is_var_used_in_expr_any(var_name, s))
                || stop
                    .as_ref()
                    .is_some_and(|s| is_var_used_in_expr_any(var_name, s))
                || step
                    .as_ref()
                    .is_some_and(|s| is_var_used_in_expr_any(var_name, s))
        }
        HirExpr::List(items)
        | HirExpr::Tuple(items)
        | HirExpr::Set(items)
        | HirExpr::FrozenSet(items) => items.iter().any(|i| is_var_used_in_expr_any(var_name, i)),
        HirExpr::Dict(pairs) => pairs
            .iter()
            .any(|(k, v)| is_var_used_in_expr_any(var_name, k) || is_var_used_in_expr_any(var_name, v)),
        HirExpr::Borrow { expr, .. } => is_var_used_in_expr_any(var_name, expr),
        HirExpr::IfExpr {
            test,
            body,
            orelse,
        } => {
            is_var_used_in_expr_any(var_name, test)
                || is_var_used_in_expr_any(var_name, body)
                || is_var_used_in_expr_any(var_name, orelse)
        }
        HirExpr::ListComp { element, generators }
        | HirExpr::SetComp { element, generators }
        | HirExpr::GeneratorExp { element, generators } => {
            is_var_used_in_expr_any(var_name, element)
                || generators.iter().any(|g| {
                    is_var_used_in_expr_any(var_name, &g.iter)
                        || g.conditions
                            .iter()
                            .any(|i| is_var_used_in_expr_any(var_name, i))
                })
        }
        HirExpr::DictComp {
            key,
            value,
            generators,
        } => {
            is_var_used_in_expr_any(var_name, key)
                || is_var_used_in_expr_any(var_name, value)
                || generators.iter().any(|g| {
                    is_var_used_in_expr_any(var_name, &g.iter)
                        || g.conditions
                            .iter()
                            .any(|i| is_var_used_in_expr_any(var_name, i))
                })
        }
        HirExpr::Lambda { body, .. } => is_var_used_in_expr_any(var_name, body),
        HirExpr::Await { value } => is_var_used_in_expr_any(var_name, value),
        HirExpr::FString { parts } => parts.iter().any(|p| match p {
            crate::hir::FStringPart::Expr(e) => is_var_used_in_expr_any(var_name, e),
            _ => false,
        }),
        HirExpr::Yield { value } => value
            .as_ref()
            .is_some_and(|v| is_var_used_in_expr_any(var_name, v)),
        HirExpr::SortByKey {
            iterable,
            key_body,
            reverse_expr,
            ..
        } => {
            is_var_used_in_expr_any(var_name, iterable)
                || is_var_used_in_expr_any(var_name, key_body)
                || reverse_expr
                    .as_ref()
                    .is_some_and(|r| is_var_used_in_expr_any(var_name, r))
        }
        HirExpr::NamedExpr { target, value } => {
            target == var_name || is_var_used_in_expr_any(var_name, value)
        }
    }
}

/// Process function body statements with proper scoping
#[inline]
pub(crate) fn codegen_function_body(
    func: &HirFunction,
    can_fail: bool,
    error_type: Option<crate::rust_gen::context::ErrorType>,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    // Enter function scope and declare parameters
    ctx.enter_scope();
    ctx.current_function_can_fail = can_fail;

    // GH-70: Pre-populate nested function parameter types with inference
    // This must happen before processing body statements so that nested function
    // code generation can use the inferred types from ctx.nested_function_params
    let _ = detect_returns_nested_function(func, ctx);

    // DEPYLER-0460: Infer return type from body if not explicitly annotated
    // This must happen before setting ctx.current_return_type so that return
    // statement generation uses the correct type (e.g., wrapping in Some() for Optional)
    // Use the SAME inference logic as signature generation for consistency
    // DEPYLER-0460: Also infer when ret_type is None (could be Optional pattern)
    // DEPYLER-0662: Also infer when ret_type is empty tuple (from `-> tuple` annotation)
    // DEPYLER-0662: Python `-> tuple` parses to Type::Custom("tuple"), not Type::Tuple
    let should_infer = matches!(func.ret_type, Type::Unknown | Type::None)
        || matches!(&func.ret_type, Type::Tuple(elems) if elems.is_empty() || elems.iter().any(|t| matches!(t, Type::Unknown)))
        || matches!(&func.ret_type, Type::List(elem) if matches!(**elem, Type::Unknown))
        || matches!(&func.ret_type, Type::Custom(name) if name == "tuple");

    let effective_return_type = if should_infer {
        // No explicit annotation - try to infer from function body
        if let Some(inferred) = infer_return_type_from_body_with_params(func, ctx) {
            inferred
        } else {
            func.ret_type.clone()
        }
    } else {
        func.ret_type.clone()
    };
    ctx.current_return_type = Some(effective_return_type.clone());

    // DEPYLER-1076: Set flag when function returns impl Iterator/IntoIterator
    // This triggers `move` keyword on closures that capture local variables
    ctx.returns_impl_iterator = match &effective_return_type {
        Type::Custom(name) => {
            name.starts_with("Generator")
                || name.starts_with("Iterator")
                || name.contains("Iterator")
                || name.contains("Iterable")
        }
        Type::Generic { base, .. } => {
            base == "Generator" || base == "Iterator" || base == "Iterable"
        }
        _ => false,
    };

    // DEPYLER-0310: Set error type for raise statement wrapping
    ctx.current_error_type = error_type;

    // DEPYLER-1045: Clear fn_str_params from previous function
    // Without this, &str params from previous functions pollute later functions,
    // causing auto-borrow logic to skip local String variables with same names
    ctx.fn_str_params.clear();

    // DEPYLER-1044: Clear numpy_vars from previous function
    // Without this, numpy tracking from one function pollutes subsequent functions
    // especially CSE temps like _cse_temp_0 which are reused across functions
    ctx.numpy_vars.clear();

    for param in &func.params {
        ctx.declare_var(&param.name);
        // Store parameter type information for set/dict disambiguation
        ctx.var_types.insert(param.name.clone(), param.ty.clone());

        // DEPYLER-0543: Track function params with str type (become &str in Rust)
        // These should NOT have & added when used as dict keys
        if matches!(param.ty, Type::String) {
            ctx.fn_str_params.insert(param.name.clone());
        }
    }

    // DEPYLER-0690: Build var_types from local variable assignments BEFORE codegen
    // This enables type-aware string concatenation detection (format! vs +)
    // and other type-based code generation decisions
    build_var_type_env(&func.body, &mut ctx.var_types);

    // DEPYLER-1134: Propagate return type annotation to returned variables
    // This enables Constraint-Aware Coercion - if function returns List[List[str]],
    // the variable being returned (e.g., `rows`) gets that concrete type
    if let Some(ref ret_type) = ctx.current_return_type {
        propagate_return_type_to_vars(&func.body, &mut ctx.var_types, ret_type);
    }

    // DEPYLER-0312 NOTE: analyze_mutable_vars is now called in impl RustCodeGen BEFORE
    // codegen_function_params, so ctx.mutable_vars is already populated here

    // DEPYLER-0784: Don't hoist nested functions
    // Previous DEPYLER-0613 hoisted ALL nested functions to fix E0425 (forward references),
    // but this causes E0282 (type annotations needed) because Rust can't infer closure
    // types without an initializer (`let x;` for closures doesn't work).
    // Since most Python code defines functions before calling them, we remove hoisting.
    // If forward references are needed, the Rust compiler will give a clear E0425 error.
    let mut all_nested_fns = Vec::new();
    collect_nested_function_names(&func.body, &mut all_nested_fns);

    // Start with an empty body
    let mut body_stmts: Vec<proc_macro2::TokenStream> = Vec::new();

    // DEPYLER-0784: Don't pre-declare nested functions
    // The closure will be declared inline when processing the FunctionDef statement
    // Note: We collect names but don't call ctx.declare_var() so that
    // codegen_nested_function_def will emit `let name = move |...|` instead of `name = ...`
    let _ = all_nested_fns; // Silence unused warning

    // DEPYLER-0963: Build parameter types map for variable type inference
    // This allows us to infer types for variables assigned from parameters (e.g., result = n)
    let param_types: std::collections::HashMap<String, Type> = func
        .params
        .iter()
        .map(|p| (p.name.clone(), p.ty.clone()))
        .collect();

    // DEPYLER-0762: Hoist loop-escaping variables
    // Python has function-level scoping, so variables assigned in for/while loops
    // are visible after the loop. Rust has block-level scoping, so we must hoist
    // these variable declarations to function scope.
    // We initialize with Default::default() to avoid E0381 "possibly-uninitialized"
    // errors, since Rust can't prove the loop will always execute.
    // DEPYLER-0763: Use safe_ident to escape Rust keywords (e.g. match -> r#match)
    // DEPYLER-0963: Add type annotations to avoid E0790 (can't call Default::default() on trait)
    let loop_escaping_vars = collect_loop_escaping_variables(&func.body);
    for var_name in &loop_escaping_vars {
        if !ctx.is_declared(var_name) {
            let ident = safe_ident(var_name);
            // Try to infer the variable's type from its assignments (with param context)
            if let Some(ty) = find_var_type_in_body_with_params(var_name, &func.body, &param_types) {
                let rust_type = ctx.type_mapper.map_type(&ty);
                if let Ok(syn_type) = rust_type_to_syn(&rust_type) {
                    body_stmts.push(quote! { let mut #ident: #syn_type = Default::default(); });
                } else {
                    // DEPYLER-1065: Hybrid Fallback - use DepylerValue instead of ()
                    let fallback_type = if ctx.type_mapper.nasa_mode {
                        quote! { DepylerValue }
                    } else {
                        quote! { serde_json::Value }
                    };
                    body_stmts.push(quote! { let mut #ident: #fallback_type = Default::default(); });
                }
            } else {
                // DEPYLER-1065: Hybrid Fallback - use DepylerValue instead of ()
                let fallback_type = if ctx.type_mapper.nasa_mode {
                    quote! { DepylerValue }
                } else {
                    quote! { serde_json::Value }
                };
                body_stmts.push(quote! { let mut #ident: #fallback_type = Default::default(); });
            }
            ctx.declare_var(var_name);
        }
    }

    // DEPYLER-0834: Hoist if-escaping variables
    // Python has function-level scoping, so variables assigned in if/else blocks
    // are visible after the if statement. Rust has block-level scoping, so we must
    // hoist these variable declarations to function scope.
    // We initialize with Default::default() to avoid E0381 "possibly-uninitialized"
    // errors, since the if branch may not be taken.
    // DEPYLER-0963: Add type annotations to avoid E0790 (can't call Default::default() on trait)
    let if_escaping_vars = collect_if_escaping_variables(&func.body);
    for var_name in &if_escaping_vars {
        if !ctx.is_declared(var_name) {
            let ident = safe_ident(var_name);
            // Try to infer the variable's type from its assignments (with param context)
            if let Some(ty) = find_var_type_in_body_with_params(var_name, &func.body, &param_types) {
                let rust_type = ctx.type_mapper.map_type(&ty);
                if let Ok(syn_type) = rust_type_to_syn(&rust_type) {
                    body_stmts.push(quote! { let mut #ident: #syn_type = Default::default(); });
                } else {
                    // DEPYLER-1065: Hybrid Fallback - use DepylerValue instead of ()
                    let fallback_type = if ctx.type_mapper.nasa_mode {
                        quote! { DepylerValue }
                    } else {
                        quote! { serde_json::Value }
                    };
                    body_stmts.push(quote! { let mut #ident: #fallback_type = Default::default(); });
                }
            } else {
                // DEPYLER-1065: Hybrid Fallback - use DepylerValue instead of ()
                let fallback_type = if ctx.type_mapper.nasa_mode {
                    quote! { DepylerValue }
                } else {
                    quote! { serde_json::Value }
                };
                body_stmts.push(quote! { let mut #ident: #fallback_type = Default::default(); });
            }
            ctx.declare_var(var_name);
        }
    }

    // DEPYLER-0688: Emit statements in original order, preserving Python semantics
    // Nested functions that capture outer variables must be emitted AFTER those variables
    // are declared. Forward declarations (let mut fib;) are already emitted above.
    let body_len = func.body.len();
    for (i, stmt) in func.body.iter().enumerate() {
        // Mark final statement for idiomatic expression-based return
        // (only if it's not a FunctionDef, as those are assignments not returns)
        ctx.is_final_statement = i == body_len - 1 && !matches!(stmt, HirStmt::FunctionDef { .. });

        // DEPYLER-1168: Populate vars_used_later for call-site clone detection
        // Before processing each statement, compute which variables are used in remaining statements
        ctx.vars_used_later.clear();
        let remaining_stmts = &func.body[i + 1..];
        for var_name in ctx.var_types.keys() {
            if is_var_used_in_remaining_stmts(var_name, remaining_stmts) {
                ctx.vars_used_later.insert(var_name.clone());
            }
        }

        let tokens = stmt.to_rust_tokens(ctx)?;
        body_stmts.push(tokens);
    }

    ctx.exit_scope();
    ctx.current_function_can_fail = false;
    ctx.current_return_type = None;
    ctx.returns_impl_iterator = false;

    Ok(body_stmts)
}

// ============================================================================
// DEPYLER-0141 Phase 3: Complex Sections
// ============================================================================

// ========== Phase 3a: Parameter Conversion ==========

/// Convert function parameters with lifetime and borrowing analysis
#[inline]
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

/// DEPYLER-0789: Look up correct type for an argparse field from tracker or body usage
/// Searches all subcommands for the field, or infers type from how args.field is used
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

/// DEPYLER-0789: Check if a field is used as a boolean condition in the body
/// Patterns: `if args.field:`, `args.field and ...`, `not args.field`
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

/// DEPYLER-0914: Infer numeric type from arithmetic operations on args.field
/// Patterns: args.r / 255 → i32, args.h * 6.0 → f64
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
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
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

/// DEPYLER-0757: Check if a variable is used anywhere in the function body
/// Used to detect unused parameters so we can prefix them with underscore
fn is_param_used_in_body(param_name: &str, body: &[HirStmt]) -> bool {
    body.iter().any(|stmt| is_param_used_in_stmt(param_name, stmt))
}

/// Check if a parameter is used in a statement (recursive)
fn is_param_used_in_stmt(param_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            is_param_used_in_assign_target(param_name, target)
                || is_param_used_in_expr(param_name, value)
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            is_param_used_in_expr(param_name, condition)
                || then_body.iter().any(|s| is_param_used_in_stmt(param_name, s))
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(|s| is_param_used_in_stmt(param_name, s)))
        }
        HirStmt::While { condition, body } => {
            is_param_used_in_expr(param_name, condition)
                || body.iter().any(|s| is_param_used_in_stmt(param_name, s))
        }
        HirStmt::For { iter, body, .. } => {
            is_param_used_in_expr(param_name, iter)
                || body.iter().any(|s| is_param_used_in_stmt(param_name, s))
        }
        HirStmt::Return(Some(expr)) => is_param_used_in_expr(param_name, expr),
        HirStmt::Expr(expr) => is_param_used_in_expr(param_name, expr),
        HirStmt::Try {
            body,
            handlers,
            orelse,
            finalbody,
            ..
        } => {
            body.iter().any(|s| is_param_used_in_stmt(param_name, s))
                || handlers
                    .iter()
                    .any(|h| h.body.iter().any(|s| is_param_used_in_stmt(param_name, s)))
                || orelse.as_ref().is_some_and(|stmts| {
                    stmts.iter().any(|s| is_param_used_in_stmt(param_name, s))
                })
                || finalbody.as_ref().is_some_and(|stmts| {
                    stmts.iter().any(|s| is_param_used_in_stmt(param_name, s))
                })
        }
        // DEPYLER-0833: Also check the context expression in With statements
        // Example: `with open(path) as f:` - path is used in context, not body
        HirStmt::With { context, body, .. } => {
            is_param_used_in_expr(param_name, context)
                || body.iter().any(|s| is_param_used_in_stmt(param_name, s))
        }
        // DEPYLER-0758: Check nested function bodies for closure captures
        // If outer param is used in nested function, it's a closure capture and must not be renamed
        HirStmt::FunctionDef { body, .. } => {
            body.iter().any(|s| is_param_used_in_stmt(param_name, s))
        }
        // DEPYLER-0950: Check assert statements for parameter usage
        HirStmt::Assert { test, msg } => {
            is_param_used_in_expr(param_name, test)
                || msg
                    .as_ref()
                    .is_some_and(|e| is_param_used_in_expr(param_name, e))
        }
        _ => false,
    }
}

/// Check if parameter is used in an expression
fn is_param_used_in_expr(param_name: &str, expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Var(name) => name == param_name,
        HirExpr::Binary { left, right, .. } => {
            is_param_used_in_expr(param_name, left) || is_param_used_in_expr(param_name, right)
        }
        HirExpr::Unary { operand, .. } => is_param_used_in_expr(param_name, operand),
        // DEPYLER-0761: Check if param is used as the function being called (Callable params)
        HirExpr::Call { func, args, kwargs } => {
            func == param_name
                || args.iter().any(|a| is_param_used_in_expr(param_name, a))
                || kwargs.iter().any(|(_, v)| is_param_used_in_expr(param_name, v))
        }
        // DEPYLER-0761: Must check kwargs too - parameter used in kwargs was being missed
        // causing param to be renamed with underscore but body still using original name
        HirExpr::MethodCall { object, args, kwargs, .. } => {
            is_param_used_in_expr(param_name, object)
                || args.iter().any(|a| is_param_used_in_expr(param_name, a))
                || kwargs.iter().any(|(_, v)| is_param_used_in_expr(param_name, v))
        }
        HirExpr::Attribute { value, .. } => is_param_used_in_expr(param_name, value),
        HirExpr::Index { base, index } => {
            is_param_used_in_expr(param_name, base) || is_param_used_in_expr(param_name, index)
        }
        HirExpr::Slice {
            base,
            start,
            stop,
            step,
        } => {
            is_param_used_in_expr(param_name, base)
                || start
                    .as_ref()
                    .is_some_and(|e| is_param_used_in_expr(param_name, e))
                || stop
                    .as_ref()
                    .is_some_and(|e| is_param_used_in_expr(param_name, e))
                || step
                    .as_ref()
                    .is_some_and(|e| is_param_used_in_expr(param_name, e))
        }
        HirExpr::Borrow { expr, .. } => is_param_used_in_expr(param_name, expr),
        HirExpr::FrozenSet(items) => items.iter().any(|i| is_param_used_in_expr(param_name, i)),
        HirExpr::FString { parts } => parts.iter().any(|p| {
            if let crate::hir::FStringPart::Expr(e) = p {
                is_param_used_in_expr(param_name, e)
            } else {
                false
            }
        }),
        HirExpr::Yield { value } => value
            .as_ref()
            .is_some_and(|e| is_param_used_in_expr(param_name, e)),
        // DEPYLER-0761: Must check kwargs too for dynamic calls
        HirExpr::DynamicCall { callee, args, kwargs } => {
            is_param_used_in_expr(param_name, callee)
                || args.iter().any(|a| is_param_used_in_expr(param_name, a))
                || kwargs.iter().any(|(_, v)| is_param_used_in_expr(param_name, v))
        }
        HirExpr::SortByKey {
            iterable,
            key_body,
            reverse_expr,
            ..
        } => {
            is_param_used_in_expr(param_name, iterable)
                || is_param_used_in_expr(param_name, key_body)
                || reverse_expr
                    .as_ref()
                    .is_some_and(|e| is_param_used_in_expr(param_name, e))
        }
        // DEPYLER-0766: Check element, iterator, AND conditions for generator expressions
        HirExpr::GeneratorExp {
            element,
            generators,
        } => {
            is_param_used_in_expr(param_name, element)
                || generators.iter().any(|g| {
                    is_param_used_in_expr(param_name, &g.iter)
                        || g.conditions
                            .iter()
                            .any(|cond| is_param_used_in_expr(param_name, cond))
                })
        }
        HirExpr::NamedExpr { value, .. } => is_param_used_in_expr(param_name, value),
        HirExpr::List(items) | HirExpr::Tuple(items) => {
            items.iter().any(|i| is_param_used_in_expr(param_name, i))
        }
        HirExpr::Dict(pairs) => {
            pairs.iter().any(|(k, v)| {
                is_param_used_in_expr(param_name, k) || is_param_used_in_expr(param_name, v)
            })
        }
        HirExpr::Set(items) => items.iter().any(|i| is_param_used_in_expr(param_name, i)),
        HirExpr::IfExpr { test, body, orelse } => {
            is_param_used_in_expr(param_name, test)
                || is_param_used_in_expr(param_name, body)
                || is_param_used_in_expr(param_name, orelse)
        }
        HirExpr::Lambda { body, .. } => is_param_used_in_expr(param_name, body),
        // DEPYLER-0766: Check element, iterator, AND conditions for comprehensions
        HirExpr::ListComp { element, generators, .. }
        | HirExpr::SetComp { element, generators, .. } => {
            is_param_used_in_expr(param_name, element)
                || generators.iter().any(|g| {
                    is_param_used_in_expr(param_name, &g.iter)
                        || g.conditions
                            .iter()
                            .any(|cond| is_param_used_in_expr(param_name, cond))
                })
        }
        // DEPYLER-0766: Check key, value, iterator, AND conditions for dict comprehensions
        HirExpr::DictComp {
            key,
            value,
            generators,
        } => {
            is_param_used_in_expr(param_name, key)
                || is_param_used_in_expr(param_name, value)
                || generators.iter().any(|g| {
                    is_param_used_in_expr(param_name, &g.iter)
                        || g.conditions
                            .iter()
                            .any(|cond| is_param_used_in_expr(param_name, cond))
                })
        }
        HirExpr::Await { value } => is_param_used_in_expr(param_name, value),
        _ => false,
    }
}

/// Check if parameter is used in an assignment target
fn is_param_used_in_assign_target(param_name: &str, target: &AssignTarget) -> bool {
    match target {
        AssignTarget::Symbol(name) => name == param_name,
        AssignTarget::Index { base, index } => {
            is_param_used_in_expr(param_name, base) || is_param_used_in_expr(param_name, index)
        }
        AssignTarget::Attribute { value, .. } => is_param_used_in_expr(param_name, value),
        AssignTarget::Tuple(targets) => {
            targets.iter().any(|t| is_param_used_in_assign_target(param_name, t))
        }
    }
}

/// Convert a single parameter with all borrowing strategies
fn codegen_single_param(
    param: &HirParam,
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0757: Check if parameter is used in the function body
    // If not used, prefix with underscore to avoid unused variable warnings
    let is_used = is_param_used_in_body(&param.name, &func.body);

    // DEPYLER-0357: Parameter names in signature must match how they're referenced in body
    // DEPYLER-0757: But if NOT used at all, prefix with underscore to suppress warning
    // DEPYLER-0611: Use raw identifiers for parameter names that are Rust keywords
    // DEPYLER-0630: self/Self cannot be raw identifiers, rename to self_ instead
    let param_name = if is_used {
        param.name.clone()
    } else {
        format!("_{}", param.name)
    };

    let param_ident = if param_name == "self" || param_name == "Self" {
        // self/Self are special - they cannot be raw identifiers, rename them
        syn::Ident::new(&format!("{}_", param_name), proc_macro2::Span::call_site())
    } else if is_rust_keyword(&param_name) {
        syn::Ident::new_raw(&param_name, proc_macro2::Span::call_site())
    } else {
        syn::Ident::new(&param_name, proc_macro2::Span::call_site())
    };

    // DEPYLER-0477: Handle varargs parameters (*args in Python)
    // DEPYLER-0487: Generate &[T] instead of Vec<T> for better ergonomics
    // This allows calling from match patterns where the value is borrowed
    // Python: def func(*args) → Rust: fn func(args: &[T])
    if param.is_vararg {
        // DEPYLER-1150: Track this parameter as a slice for return type conversion
        // When returning a slice param in a function that returns Vec<T>, add .to_vec()
        ctx.slice_params.insert(param.name.clone());

        // Extract element type from Type::List
        let elem_type = if let Type::List(elem) = &param.ty {
            rust_type_to_syn(&ctx.type_mapper.map_type(elem))?
        } else {
            // Fallback: If not Type::List, use String as default
            // This shouldn't happen if AST bridge is correct
            parse_quote! { String }
        };

        // Varargs parameters as slices (more idiomatic Rust)
        return Ok(quote! { #param_ident: &[#elem_type] });
    }

    // DEPYLER-0424: Check if this parameter is the argparse args variable
    // If so, type it as &Args instead of default type mapping
    let is_argparse_args = ctx.argparser_tracker.parsers.values().any(|parser_info| {
        parser_info
            .args_var
            .as_ref()
            .is_some_and(|args_var| args_var == &param.name)
    });

    if is_argparse_args {
        // Use &Args for argparse result parameters
        return Ok(quote! { #param_ident: &Args });
    }

    // DEPYLER-0488: Special case for set_nested_value's value parameter
    // The parameter is NOT mutated (only used on RHS of `dict[key] = value`)
    // Override incorrect mutability analysis for this specific function
    if func.name == "set_nested_value" && param.name == "value" {
        if let Some(inferred) = lifetime_result.param_lifetimes.get(&param.name) {
            let rust_type = &inferred.rust_type;

            // Force immutable even if analysis incorrectly flagged as mutable
            let mut inferred_immutable = inferred.clone();
            inferred_immutable.needs_mut = false;

            let ty = apply_param_borrowing_strategy(
                &param.name,
                rust_type,
                &inferred_immutable,
                lifetime_result,
                ctx,
            )?;

            return Ok(quote! { #param_ident: #ty });
        }
    }

    // DEPYLER-0312: Use mutable_vars populated by analyze_mutable_vars
    // This handles ALL mutation patterns: direct assignment, method calls, and parameter reassignments
    // The analyze_mutable_vars function already checked all mutation patterns in codegen_function_body
    let is_mutated_in_body = ctx.mutable_vars.contains(&param.name);

    // Only apply `mut` if ownership is taken (not borrowed)
    // Borrowed parameters (&T, &mut T) handle mutability in the type itself
    let takes_ownership = matches!(
        lifetime_result.borrowing_strategies.get(&param.name),
        Some(crate::borrowing_context::BorrowingStrategy::TakeOwnership) | None
    );

    let is_param_mutated = is_mutated_in_body && takes_ownership;

    // DEPYLER-0447: Detect argparse validator functions (tracked at add_argument() call sites)
    // These should ALWAYS have &str parameter type regardless of type inference
    // Validators are detected when processing add_argument(type=validator_func)
    let is_argparse_validator = ctx.validator_functions.contains(&func.name);

    if is_argparse_validator {
        // Argparse validators always receive string arguments from clap
        let ty = if is_param_mutated {
            quote! { mut #param_ident: &str }
        } else {
            quote! { #param_ident: &str }
        };
        return Ok(ty);
    }

    // DEPYLER-0607: Infer Args type for argparse command handler functions
    // When a function takes "args" parameter with Unknown type and it's a command handler,
    // the parameter should be &Args (reference to clap Args struct)
    // Pattern: def cmd_list(args): args.archive → fn cmd_list(args: &Args)
    // Heuristic: Function starts with "cmd_" or "handle_" and has "args" parameter
    // This must run BEFORE lifetime inference check to override serde_json::Value fallback
    let is_cmd_handler = func.name.starts_with("cmd_") || func.name.starts_with("handle_");
    if param.name == "args" && is_cmd_handler && matches!(param.ty, Type::Unknown) {
        let ty: syn::Type = syn::parse_quote! { &Args };
        return Ok(if is_param_mutated {
            quote! { mut #param_ident: #ty }
        } else {
            quote! { #param_ident: #ty }
        });
    }

    // Get the inferred parameter info
    if let Some(inferred) = lifetime_result.param_lifetimes.get(&param.name) {
        // DEPYLER-0716: Check if we have a substituted type from generic inference
        // This overrides the type from lifetime analysis when T is inferred to be a concrete type
        let rust_type = if let Some(substituted_ty) = ctx.var_types.get(&param.name) {
            // Use substituted type from generic inference
            ctx.type_mapper.map_type(substituted_ty)
        } else {
            inferred.rust_type.clone()
        };

        // Handle Union type placeholders
        let actual_rust_type =
            if let crate::type_mapper::RustType::Enum { name, variants: _ } = &rust_type {
                if name == "UnionType" {
                    if let Type::Union(types) = &param.ty {
                        let enum_name = ctx.process_union_type(types);
                        crate::type_mapper::RustType::Custom(enum_name)
                    } else {
                        rust_type.clone()
                    }
                } else {
                    rust_type.clone()
                }
            } else {
                rust_type.clone()
            };

        update_import_needs(ctx, &actual_rust_type);

        // DEPYLER-0330: Override needs_mut for borrowed parameters that are mutated
        // If analyze_mutable_vars detected mutation (via .remove(), .clear(), etc.)
        // and this parameter will be borrowed (&T), upgrade to &mut T
        let mut inferred_with_mut = inferred.clone();
        if is_mutated_in_body && inferred.should_borrow {
            inferred_with_mut.needs_mut = true;
        }

        let ty = apply_param_borrowing_strategy(
            &param.name,
            &actual_rust_type,
            &inferred_with_mut,
            lifetime_result,
            ctx,
        )?;

        Ok(if is_param_mutated {
            quote! { mut #param_ident: #ty }
        } else {
            quote! { #param_ident: #ty }
        })
    } else {
        // DEPYLER-0524/0716: Check if we have an inferred/substituted type from body usage analysis
        // This allows inferring String for parameters used with .endswith(), etc.
        // DEPYLER-0716: Also check for type substitutions (e.g., List(Unknown) -> List(String))
        let effective_type = if let Some(substituted) = ctx.var_types.get(&param.name) {
            // Use substituted type from type inference (DEPYLER-0716)
            substituted.clone()
        } else if matches!(param.ty, Type::Unknown) {
            // DEPYLER-0607: Infer Args type for argparse command handler functions
            // When a function takes "args" parameter with Unknown type and it's a command handler,
            // the parameter should be &Args (reference to clap Args struct)
            // Pattern: def cmd_list(args): args.archive → fn cmd_list(args: &Args)
            // Heuristic: Function starts with "cmd_" and has "args" parameter
            // This works even when argparse detection hasn't run yet (functions processed before main)
            let is_cmd_handler = func.name.starts_with("cmd_") || func.name.starts_with("handle_");
            if param.name == "args" && is_cmd_handler {
                Type::Custom("Args".to_string())
            } else {
                param.ty.clone()
            }
        } else {
            param.ty.clone()
        };

        // Fallback to original mapping using effective (possibly inferred) type
        let rust_type = ctx
            .annotation_aware_mapper
            .map_type_with_annotations(&effective_type, &func.annotations);
        update_import_needs(ctx, &rust_type);
        let ty = rust_type_to_syn(&rust_type)?;

        Ok(if is_param_mutated {
            quote! { mut #param_ident: #ty }
        } else {
            quote! { #param_ident: #ty }
        })
    }
}

/// Apply borrowing strategy to parameter type
fn apply_param_borrowing_strategy(
    param_name: &str,
    rust_type: &crate::type_mapper::RustType,
    inferred: &crate::lifetime_analysis::InferredParam,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) -> Result<syn::Type> {
    let mut ty = rust_type_to_syn(rust_type)?;

    // DEPYLER-0275: Check if lifetimes should be elided
    // If lifetime_params is empty, Rust's elision rules apply - don't add explicit lifetimes
    let should_elide_lifetimes = lifetime_result.lifetime_params.is_empty();

    // Check if we have a borrowing strategy
    if let Some(strategy) = lifetime_result.borrowing_strategies.get(param_name) {
        match strategy {
            crate::borrowing_context::BorrowingStrategy::UseCow { lifetime } => {
                ctx.needs_cow = true;

                // DEPYLER-0282 FIX: Parameters should NEVER use 'static lifetime
                // For parameters, we need borrowed data that can be passed from local scope
                // Use generic lifetime or elide it - never 'static for parameters
                if should_elide_lifetimes {
                    // Elide lifetime - let Rust infer it
                    ty = parse_quote! { Cow<'_, str> };
                } else if lifetime == "'static" {
                    // CRITICAL FIX: Don't use 'static for parameters!
                    // If inference suggested 'static, use generic lifetime instead
                    // This allows passing local Strings/&str to the function
                    if let Some(first_lifetime) = lifetime_result.lifetime_params.first() {
                        let lt = syn::Lifetime::new(first_lifetime, proc_macro2::Span::call_site());
                        ty = parse_quote! { Cow<#lt, str> };
                    } else {
                        // No explicit lifetimes - use elision
                        ty = parse_quote! { Cow<'_, str> };
                    }
                } else {
                    // Use the provided non-static lifetime
                    let lt = syn::Lifetime::new(lifetime, proc_macro2::Span::call_site());
                    ty = parse_quote! { Cow<#lt, str> };
                }
            }
            _ => {
                // Apply normal borrowing if needed
                if inferred.should_borrow {
                    ty = apply_borrowing_to_type(ty, rust_type, inferred, should_elide_lifetimes)?;
                }
            }
        }
    } else {
        // Fallback to normal borrowing
        if inferred.should_borrow {
            ty = apply_borrowing_to_type(ty, rust_type, inferred, should_elide_lifetimes)?;
        }
    }

    Ok(ty)
}

/// Apply borrowing (&, &mut, with lifetime) to a type
/// DEPYLER-0275: Added should_elide_lifetimes parameter to respect Rust elision rules
fn apply_borrowing_to_type(
    mut ty: syn::Type,
    rust_type: &crate::type_mapper::RustType,
    inferred: &crate::lifetime_analysis::InferredParam,
    should_elide_lifetimes: bool,
) -> Result<syn::Type> {
    // DEPYLER-0525: If the type is already a reference, don't add another reference
    // This happens when the type mapper returns RustType::Reference (e.g., for File types)
    if matches!(rust_type, crate::type_mapper::RustType::Reference { .. }) {
        return Ok(ty);
    }

    // DEPYLER-0566: Primitive types implement Copy, so pass by value (no reference needed)
    // bool, i32, i64, f32, f64, char, etc. should NOT be borrowed
    if matches!(
        rust_type,
        crate::type_mapper::RustType::Primitive(_) | crate::type_mapper::RustType::Unit
    ) {
        return Ok(ty);
    }

    // DEPYLER-1075: impl Trait types cannot be borrowed - they're opaque return types
    // Don't try to wrap `impl Iterator<Item=T> + '_` in a reference
    if let crate::type_mapper::RustType::Custom(name) = rust_type {
        if name.starts_with("impl ") {
            return Ok(ty);
        }
    }

    // Special case for strings: use &str instead of &String
    if matches!(rust_type, crate::type_mapper::RustType::String) {
        // DEPYLER-0275: Elide lifetime if elision rules apply
        if should_elide_lifetimes || inferred.lifetime.is_none() {
            ty = if inferred.needs_mut {
                parse_quote! { &mut str }
            } else {
                parse_quote! { &str }
            };
        } else if let Some(ref lifetime) = inferred.lifetime {
            let lt = syn::Lifetime::new(lifetime.as_str(), proc_macro2::Span::call_site());
            ty = if inferred.needs_mut {
                parse_quote! { &#lt mut str }
            } else {
                parse_quote! { &#lt str }
            };
        } else {
            ty = if inferred.needs_mut {
                parse_quote! { &mut str }
            } else {
                parse_quote! { &str }
            };
        }
    } else {
        // Non-string types
        // DEPYLER-0275: Elide lifetime if elision rules apply
        if should_elide_lifetimes || inferred.lifetime.is_none() {
            ty = if inferred.needs_mut {
                parse_quote! { &mut #ty }
            } else {
                parse_quote! { &#ty }
            };
        } else if let Some(ref lifetime) = inferred.lifetime {
            let lt = syn::Lifetime::new(lifetime.as_str(), proc_macro2::Span::call_site());
            ty = if inferred.needs_mut {
                parse_quote! { &#lt mut #ty }
            } else {
                parse_quote! { &#lt #ty }
            };
        } else {
            ty = if inferred.needs_mut {
                parse_quote! { &mut #ty }
            } else {
                parse_quote! { &#ty }
            };
        }
    }

    Ok(ty)
}

// ========== String Method Return Type Analysis (v3.16.0) ==========

/// Classification of string methods by their return type semantics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringMethodReturnType {
    /// Returns owned String (e.g., upper, lower, strip, replace)
    Owned,
    /// Returns borrowed &str or bool (e.g., starts_with, is_digit)
    Borrowed,
}

/// Classify a string method by its return type semantics
fn classify_string_method(method_name: &str) -> StringMethodReturnType {
    match method_name {
        // Transformation methods that return owned String
        "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "replace" | "format" | "title"
        | "capitalize" | "swapcase" | "expandtabs" | "center" | "ljust" | "rjust" | "zfill" => {
            StringMethodReturnType::Owned
        }

        // Query/test methods that return bool or &str (borrowed)
        "startswith" | "endswith" | "isalpha" | "isdigit" | "isalnum" | "isspace" | "islower"
        | "isupper" | "istitle" | "isascii" | "isprintable" | "find" | "rfind" | "index"
        | "rindex" | "count" => StringMethodReturnType::Borrowed,

        // Default: assume owned to be safe
        _ => StringMethodReturnType::Owned,
    }
}

/// Check if an expression contains a string method call that returns owned String
/// DEPYLER-0598: Also detect string literals (which get .to_string() in codegen)
fn contains_owned_string_method(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::MethodCall { method, .. } => {
            // Check if this method returns owned String
            classify_string_method(method) == StringMethodReturnType::Owned
        }
        HirExpr::Binary { left, right, .. } => {
            // Check both sides of binary operations
            contains_owned_string_method(left) || contains_owned_string_method(right)
        }
        HirExpr::Unary { operand, .. } => contains_owned_string_method(operand),
        HirExpr::IfExpr { body, orelse, .. } => {
            // Check both branches of conditional
            contains_owned_string_method(body) || contains_owned_string_method(orelse)
        }
        // DEPYLER-0598: String literals get .to_string() in codegen, so they're owned
        HirExpr::Literal(crate::hir::Literal::String(_)) => true,
        // F-strings generate format!() which returns owned String
        HirExpr::FString { .. } => true,
        HirExpr::Call { .. }
        | HirExpr::Var(_)
        | HirExpr::Literal(_) // Non-string literals
        | HirExpr::List(_)
        | HirExpr::Dict(_)
        | HirExpr::Tuple(_)
        | HirExpr::Set(_)
        | HirExpr::FrozenSet(_)
        | HirExpr::Index { .. }
        | HirExpr::Slice { .. }
        | HirExpr::Attribute { .. }
        | HirExpr::Borrow { .. }
        | HirExpr::ListComp { .. }
        | HirExpr::SetComp { .. }
        | HirExpr::DictComp { .. }
        | HirExpr::Lambda { .. }
        | HirExpr::Await { .. }
        | HirExpr::Yield { .. }
        | HirExpr::SortByKey { .. }
        | HirExpr::GeneratorExp { .. }
        | HirExpr::NamedExpr { .. }
        | HirExpr::DynamicCall { .. } => false,
    }
}

/// Check if the function's return expressions contain owned-returning string methods
/// DEPYLER-0598: Now recursively checks nested blocks (if/while/for)
pub(crate) fn function_returns_owned_string(func: &HirFunction) -> bool {
    // Recursively check all return statements in the function body
    stmt_block_returns_owned_string(&func.body)
}

/// Helper to recursively check a block of statements for owned string returns
fn stmt_block_returns_owned_string(stmts: &[HirStmt]) -> bool {
    for stmt in stmts {
        if stmt_returns_owned_string(stmt) {
            return true;
        }
    }
    false
}

/// Check if a single statement returns an owned string (recursively checks nested blocks)
fn stmt_returns_owned_string(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Return(Some(expr)) => contains_owned_string_method(expr),
        HirStmt::If {
            then_body,
            else_body,
            ..
        } => {
            stmt_block_returns_owned_string(then_body)
                || else_body
                    .as_ref()
                    .is_some_and(|body| stmt_block_returns_owned_string(body))
        }
        HirStmt::While { body, .. } => stmt_block_returns_owned_string(body),
        HirStmt::For { body, .. } => stmt_block_returns_owned_string(body),
        HirStmt::With { body, .. } => stmt_block_returns_owned_string(body),
        HirStmt::Try {
            body,
            handlers,
            orelse,
            finalbody,
        } => {
            stmt_block_returns_owned_string(body)
                || handlers
                    .iter()
                    .any(|h| stmt_block_returns_owned_string(&h.body))
                || orelse
                    .as_ref()
                    .is_some_and(|body| stmt_block_returns_owned_string(body))
                || finalbody
                    .as_ref()
                    .is_some_and(|body| stmt_block_returns_owned_string(body))
        }
        _ => false,
    }
}

// DEPYLER-0270: String Concatenation Detection

/// Check if an expression contains string concatenation (which returns owned String)
fn contains_string_concatenation(expr: &HirExpr) -> bool {
    match expr {
        // String concatenation: a + b (Add operator generates format!() for strings)
        HirExpr::Binary { op: BinOp::Add, .. } => {
            // Binary Add on strings generates format!() which returns String
            // We detect this by assuming any Add at top level is string concat
            // (numeric Add is handled differently in code generation)
            true
        }
        // F-strings generate format!() which returns String
        HirExpr::FString { .. } => true,
        // Recursive checks for nested expressions
        HirExpr::Binary { left, right, .. } => {
            contains_string_concatenation(left) || contains_string_concatenation(right)
        }
        HirExpr::Unary { operand, .. } => contains_string_concatenation(operand),
        HirExpr::IfExpr { body, orelse, .. } => {
            contains_string_concatenation(body) || contains_string_concatenation(orelse)
        }
        _ => false,
    }
}

/// Check if function returns string concatenation
pub(crate) fn function_returns_string_concatenation(func: &HirFunction) -> bool {
    for stmt in &func.body {
        if let HirStmt::Return(Some(expr)) = stmt {
            if contains_string_concatenation(expr) {
                return true;
            }
        }
    }
    false
}

/// Check if a type expects float values (recursively checks Option, Result, etc.)
pub(crate) fn return_type_expects_float(ty: &Type) -> bool {
    match ty {
        Type::Float => true,
        Type::Optional(inner) => return_type_expects_float(inner),
        Type::List(inner) => return_type_expects_float(inner),
        Type::Tuple(types) => types.iter().any(return_type_expects_float),
        _ => false,
    }
}

/// DEPYLER-1163: Check if a type expects int values (recursively checks Option, Result, etc.)
/// Used to determine when py_div result should be cast from f64 to i32
pub(crate) fn return_type_expects_int(ty: &Type) -> bool {
    match ty {
        Type::Int => true,
        Type::Optional(inner) => return_type_expects_int(inner),
        Type::List(inner) => return_type_expects_int(inner),
        Type::Tuple(types) => types.iter().any(return_type_expects_int),
        _ => false,
    }
}

/// DEPYLER-0936: Rewrite ADT child types to parent enum types
/// When a Python ABC hierarchy is converted to a Rust enum (e.g., Iter with ListIter, RangeIter),
/// return types mentioning child classes must be rewritten to parent enum names.
/// Example: `ListIter[T]` → `Iter[T]` when ListIter is a variant of Iter
pub(crate) fn rewrite_adt_child_type(
    ty: &Type,
    child_to_parent: &std::collections::HashMap<String, String>,
) -> Type {
    match ty {
        // Check if Custom type name is an ADT child - rewrite to parent
        Type::Custom(name) => {
            // Extract base name from generics (e.g., "ListIter" from "ListIter[T]")
            let base_name = name.split('[').next().unwrap_or(name);
            if let Some(parent_name) = child_to_parent.get(base_name) {
                // Preserve generic params: "ListIter[T]" → "Iter[T]"
                if let Some(generic_part) = name.strip_prefix(base_name) {
                    Type::Custom(format!("{}{}", parent_name, generic_part))
                } else {
                    Type::Custom(parent_name.clone())
                }
            } else {
                ty.clone()
            }
        }
        // DEPYLER-0936: Handle Generic type with base name that's an ADT child
        // Example: Generic { base: "ListIter", params: [T] } → Generic { base: "Iter", params: [T] }
        Type::Generic { base, params } => {
            if let Some(parent_name) = child_to_parent.get(base) {
                // Rewrite base to parent, keep params with recursive rewriting
                Type::Generic {
                    base: parent_name.clone(),
                    params: params
                        .iter()
                        .map(|t| rewrite_adt_child_type(t, child_to_parent))
                        .collect(),
                }
            } else {
                // No rewrite needed, but still recursively process params
                Type::Generic {
                    base: base.clone(),
                    params: params
                        .iter()
                        .map(|t| rewrite_adt_child_type(t, child_to_parent))
                        .collect(),
                }
            }
        }
        // Recursively handle container types
        Type::List(inner) => Type::List(Box::new(rewrite_adt_child_type(inner, child_to_parent))),
        Type::Optional(inner) => {
            Type::Optional(Box::new(rewrite_adt_child_type(inner, child_to_parent)))
        }
        Type::Tuple(types) => Type::Tuple(
            types
                .iter()
                .map(|t| rewrite_adt_child_type(t, child_to_parent))
                .collect(),
        ),
        Type::Dict(k, v) => Type::Dict(
            Box::new(rewrite_adt_child_type(k, child_to_parent)),
            Box::new(rewrite_adt_child_type(v, child_to_parent)),
        ),
        Type::Union(types) => Type::Union(
            types
                .iter()
                .map(|t| rewrite_adt_child_type(t, child_to_parent))
                .collect(),
        ),
        // Other types pass through unchanged
        _ => ty.clone(),
    }
}

// ========== DEPYLER-0410: Return Type Inference from Body ==========

/// Infer return type from function body when no annotation is provided
/// Returns None if type cannot be inferred or there are no return statements
#[allow(dead_code)] // Reserved for future type inference improvements
fn infer_return_type_from_body(body: &[HirStmt]) -> Option<Type> {
    // DEPYLER-0415: Build type environment from variable assignments
    let mut var_types: std::collections::HashMap<String, Type> = std::collections::HashMap::new();
    build_var_type_env(body, &mut var_types);

    let mut return_types = Vec::new();
    collect_return_types_with_env(body, &mut return_types, &var_types);

    // DEPYLER-1084: Do NOT infer return type from trailing expressions
    // Python does NOT have implicit returns like Rust - expression statements
    // just evaluate and discard their value. Only explicit `return x` statements
    // contribute to the return type.
    //
    // Previous DEPYLER-0412 incorrectly treated `x + y` as an implicit return,
    // causing functions like `def compute(): x = 10; y = 20; x + y` to have
    // inferred return type i32 instead of () (None).
    //
    // Explicit returns are already collected by collect_return_types_with_env() above.

    if return_types.is_empty() {
        return None;
    }

    // If all return types are the same (ignoring Unknown), use that type
    let first_known = return_types.iter().find(|t| !matches!(t, Type::Unknown));
    if let Some(first) = first_known {
        if return_types
            .iter()
            .all(|t| matches!(t, Type::Unknown) || t == first)
        {
            return Some(first.clone());
        }
    }

    // DEPYLER-0448: Do NOT default Unknown to Int - this causes dict/list/Value returns
    // to be incorrectly typed as i32. Instead, return None and let the type mapper
    // handle the fallback (which will use serde_json::Value for complex types).
    //
    // Previous behavior (DEPYLER-0422): Defaulted Unknown → Int for lambda returns
    // Problem: This also affected dict/list returns, causing E0308 errors
    // New behavior: Return None for Unknown types, allowing proper Value fallback
    if return_types.iter().all(|t| matches!(t, Type::Unknown)) {
        // We have return statements but all returned Unknown types
        // Don't assume Int - let type mapper decide the appropriate fallback
        return None;
    }

    // Mixed types - return the first known type
    first_known.cloned()
}

/// DEPYLER-0455 Bug 7: Infer return type from body including parameter types
/// Wrapper for infer_return_type_from_body that includes function parameters in the type environment
pub(crate) fn infer_return_type_from_body_with_params(
    func: &HirFunction,
    ctx: &CodeGenContext,
) -> Option<Type> {
    // Build initial type environment with function parameters
    let mut var_types: std::collections::HashMap<String, Type> = std::collections::HashMap::new();

    // Add parameter types to environment
    // For argparse validators, parameters are typically strings
    // DEPYLER-0455 Bug 7: Validator functions receive &str parameters
    let is_validator = ctx.validator_functions.contains(&func.name);
    for param in &func.params {
        let param_type = if is_validator && matches!(param.ty, Type::Unknown) {
            // Validator function parameters without type annotations are strings
            Type::String
        } else {
            param.ty.clone()
        };
        var_types.insert(param.name.clone(), param_type);
    }

    // Build additional types from variable assignments
    // DEPYLER-1007: Use full class-aware version to recognize:
    // - Constructor calls like `p = Point(3, 4)` → p has type Type::Custom("Point")
    // - Method calls like `dist_sq = p.distance_squared()` → dist_sq has type Int
    build_var_type_env_full(
        &func.body,
        &mut var_types,
        &ctx.function_return_types,
        &ctx.class_method_return_types,
    );

    // DEPYLER-1007: Collect return types with class method return type awareness
    // This enables proper return type inference for expressions like `p.distance_squared()`
    let mut return_types = Vec::new();
    collect_return_types_with_class_methods(
        &func.body,
        &mut return_types,
        &var_types,
        &ctx.class_method_return_types,
    );

    // DEPYLER-1084: Do NOT infer return type from trailing expressions
    // Python does NOT have implicit returns - expression statements just evaluate
    // and discard their value. Only explicit `return x` contributes to return type.
    // See comment in infer_return_type_from_body() for details.

    if return_types.is_empty() {
        return None;
    }

    // DEPYLER-0460: Check for Optional pattern BEFORE homogeneous type check
    // If function returns None in some paths and a consistent type in others,
    // infer return type as Optional<T>
    // This MUST come before the homogeneous type check to avoid returning Type::None
    // when we should return Type::Optional
    let has_none = return_types.iter().any(|t| matches!(t, Type::None));
    if has_none {
        // Find all non-None, non-Unknown types
        let non_none_types: Vec<&Type> = return_types
            .iter()
            .filter(|t| !matches!(t, Type::None | Type::Unknown))
            .collect();

        if !non_none_types.is_empty() {
            // Check if all non-None types are the same
            let first_non_none = non_none_types[0];
            if non_none_types.iter().all(|t| *t == first_non_none) {
                // Pattern detected: return None | return T → Option<T>
                return Some(Type::Optional(Box::new(first_non_none.clone())));
            }
        }

        // DEPYLER-0460: If we have None + only Unknown types, still infer Optional
        // Example: def get(d, key): if ...: return d[key]  else: return None
        // d[key] type is Unknown, but the pattern is clearly Optional
        let has_only_unknown = return_types
            .iter()
            .all(|t| matches!(t, Type::None | Type::Unknown));
        if has_only_unknown && return_types.len() > 1 {
            // At least one None and one Unknown -> Optional<Unknown>
            return Some(Type::Optional(Box::new(Type::Unknown)));
        }

        // If all returns are only None (no Unknown), return Type::None
        if return_types.iter().all(|t| matches!(t, Type::None)) {
            return Some(Type::None);
        }
    }

    // DEPYLER-0744: Handle T and Option<Unknown> → Option<T>
    // When a function returns both a typed value and an Option<Unknown> (from a param with default=None),
    // unify to Option<T> where T is the non-Optional type
    // Example: def f(x: int, fallback=None): return x OR return fallback
    //   → return types: [Int, Optional(Unknown)] → Option<Int>
    let has_optional_unknown = return_types.iter().any(|t| {
        matches!(t, Type::Optional(inner) if matches!(inner.as_ref(), Type::Unknown))
    });
    if has_optional_unknown {
        // Find the concrete non-Optional, non-Unknown type
        let concrete_type = return_types.iter().find(|t| {
            !matches!(t, Type::Optional(_) | Type::Unknown | Type::None)
        });
        if let Some(t) = concrete_type {
            // Unify: T + Option<Unknown> → Option<T>
            return Some(Type::Optional(Box::new(t.clone())));
        }
    }

    // If all types are Unknown, return None
    if return_types.iter().all(|t| matches!(t, Type::Unknown)) {
        return None;
    }

    // Check for homogeneous type (all return types are the same, ignoring Unknown)
    // This runs AFTER Optional detection to avoid misclassifying Optional patterns
    let first_known = return_types.iter().find(|t| !matches!(t, Type::Unknown));
    if let Some(first) = first_known {
        if return_types
            .iter()
            .all(|t| matches!(t, Type::Unknown) || t == first)
        {
            return Some(first.clone());
        }
    }

    // Mixed types - return the first known type
    first_known.cloned()
}

// ========== DEPYLER-0415: Variable Type Environment ==========

/// Build a type environment by collecting variable assignments
pub(crate) fn build_var_type_env(stmts: &[HirStmt], var_types: &mut std::collections::HashMap<String, Type>) {
    build_var_type_env_with_classes(stmts, var_types, &std::collections::HashMap::new());
}

/// DEPYLER-1007: Build type environment with class constructor and method type awareness
/// This version recognizes class constructor calls and method calls, assigns the correct types
pub(crate) fn build_var_type_env_with_classes(
    stmts: &[HirStmt],
    var_types: &mut std::collections::HashMap<String, Type>,
    function_return_types: &std::collections::HashMap<String, Type>,
) {
    // Call the full version with empty class_method_return_types for backward compat
    build_var_type_env_full(stmts, var_types, function_return_types, &std::collections::HashMap::new());
}

/// DEPYLER-1007: Full type environment builder with both constructor and method type awareness
pub(crate) fn build_var_type_env_full(
    stmts: &[HirStmt],
    var_types: &mut std::collections::HashMap<String, Type>,
    function_return_types: &std::collections::HashMap<String, Type>,
    class_method_return_types: &std::collections::HashMap<(String, String), Type>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol(name),
                value,
                type_annotation,
            } => {
                // DEPYLER-0714: Prefer explicit type annotation over inferred type
                // For `data: Dict[str, int] = {}`, use Dict(String, Int) not Dict(Unknown, Unknown)
                let value_type = if let Some(annot) = type_annotation {
                    annot.clone()
                } else {
                    // DEPYLER-1007: Check if this is a class constructor call
                    // e.g., `p = Point(3, 4)` → p should have type Type::Custom("Point")
                    if let HirExpr::Call { func, .. } = value {
                        if let Some(ctor_type) = function_return_types.get(func) {
                            ctor_type.clone()
                        } else {
                            // Use class-aware type inference for method calls
                            infer_expr_type_with_class_methods(value, var_types, class_method_return_types)
                        }
                    } else {
                        // DEPYLER-1007: Use class-aware type inference for method calls
                        // e.g., `dist_sq = p.distance_squared()` → dist_sq should have Int type
                        infer_expr_type_with_class_methods(value, var_types, class_method_return_types)
                    }
                };
                if !matches!(value_type, Type::Unknown) {
                    var_types.insert(name.clone(), value_type);
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                build_var_type_env_full(then_body, var_types, function_return_types, class_method_return_types);
                if let Some(else_stmts) = else_body {
                    build_var_type_env_full(else_stmts, var_types, function_return_types, class_method_return_types);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                build_var_type_env_full(body, var_types, function_return_types, class_method_return_types);
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                build_var_type_env_full(body, var_types, function_return_types, class_method_return_types);
                for handler in handlers {
                    build_var_type_env_full(&handler.body, var_types, function_return_types, class_method_return_types);
                }
                if let Some(orelse_stmts) = orelse {
                    build_var_type_env_full(orelse_stmts, var_types, function_return_types, class_method_return_types);
                }
                if let Some(finally_stmts) = finalbody {
                    build_var_type_env_full(finally_stmts, var_types, function_return_types, class_method_return_types);
                }
            }
            HirStmt::With { body, .. } => {
                build_var_type_env_full(body, var_types, function_return_types, class_method_return_types);
            }
            _ => {}
        }
    }
}

/// Collect return types with access to variable type environment
pub(crate) fn collect_return_types_with_env(
    stmts: &[HirStmt],
    types: &mut Vec<Type>,
    var_types: &std::collections::HashMap<String, Type>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                types.push(infer_expr_type_with_env(expr, var_types));
            }
            HirStmt::Return(None) => {
                types.push(Type::None);
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_return_types_with_env(then_body, types, var_types);
                if let Some(else_stmts) = else_body {
                    collect_return_types_with_env(else_stmts, types, var_types);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                collect_return_types_with_env(body, types, var_types);
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                collect_return_types_with_env(body, types, var_types);
                for handler in handlers {
                    collect_return_types_with_env(&handler.body, types, var_types);
                }
                if let Some(orelse_stmts) = orelse {
                    collect_return_types_with_env(orelse_stmts, types, var_types);
                }
                if let Some(finally_stmts) = finalbody {
                    collect_return_types_with_env(finally_stmts, types, var_types);
                }
            }
            HirStmt::With { body, .. } => {
                collect_return_types_with_env(body, types, var_types);
            }
            _ => {}
        }
    }
}

/// DEPYLER-1160: Collect all variable names that are returned from a function
/// Used for target-typed inference of empty lists - we only infer the return type
/// for variables that are actually returned, not just any empty list assignment
fn collect_returned_var_names(stmts: &[HirStmt]) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    collect_returned_var_names_impl(stmts, &mut names);
    names
}

fn collect_returned_var_names_impl(stmts: &[HirStmt], names: &mut std::collections::HashSet<String>) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(HirExpr::Var(name))) => {
                names.insert(name.clone());
            }
            HirStmt::If { then_body, else_body, .. } => {
                collect_returned_var_names_impl(then_body, names);
                if let Some(else_stmts) = else_body {
                    collect_returned_var_names_impl(else_stmts, names);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                collect_returned_var_names_impl(body, names);
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                collect_returned_var_names_impl(body, names);
                for handler in handlers {
                    collect_returned_var_names_impl(&handler.body, names);
                }
                if let Some(orelse_stmts) = orelse {
                    collect_returned_var_names_impl(orelse_stmts, names);
                }
                if let Some(finally_stmts) = finalbody {
                    collect_returned_var_names_impl(finally_stmts, names);
                }
            }
            HirStmt::With { body, .. } => {
                collect_returned_var_names_impl(body, names);
            }
            _ => {}
        }
    }
}

/// DEPYLER-1134: Propagate return type annotation to returned variables
/// DEPYLER-1160: Also propagate to empty list assignments that are later returned
/// This enables Constraint-Aware Coercion by ensuring variables like `rows`
/// get their type from `-> List[List[str]]` return annotation
pub(crate) fn propagate_return_type_to_vars(
    stmts: &[HirStmt],
    var_types: &mut std::collections::HashMap<String, Type>,
    return_type: &Type,
) {
    // Only propagate if we have a concrete return type
    if matches!(return_type, Type::Unknown | Type::None) {
        return;
    }

    // DEPYLER-1160: First, collect all returned variable names
    // This enables the "Short Circuit" heuristic: when we see `result = []`
    // and result is returned, we infer its type from the return type
    let returned_vars = collect_returned_var_names(stmts);

    propagate_return_type_impl(stmts, var_types, return_type, &returned_vars);
}

/// Internal implementation of return type propagation
fn propagate_return_type_impl(
    stmts: &[HirStmt],
    var_types: &mut std::collections::HashMap<String, Type>,
    return_type: &Type,
    returned_vars: &std::collections::HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            // DEPYLER-1160: Target-typed inference for empty list assignments
            // When `result = []` is assigned and `result` is returned from a function
            // with return type List[T], infer result's type as List[T]
            // DEPYLER-1164: Extended to empty dict assignments
            HirStmt::Assign { target: AssignTarget::Symbol(name), value, .. } => {
                if let HirExpr::List(elements) = value {
                    if elements.is_empty() && returned_vars.contains(name) {
                        // Only propagate if return type is a List with concrete element type
                        if let Type::List(_elem_type) = return_type {
                            // Don't override if we already have a concrete type
                            let should_update = match var_types.get(name) {
                                None => true,
                                Some(Type::Unknown) => true,
                                Some(Type::List(inner)) if matches!(inner.as_ref(), Type::Unknown) => true,
                                _ => false,
                            };
                            if should_update {
                                var_types.insert(name.clone(), return_type.clone());
                            }
                        }
                    }
                }
                // DEPYLER-1164: Target-typed inference for empty dict assignments
                // When `result = {}` is assigned and `result` is returned from a function
                // with return type Dict[K, V], infer result's type as Dict[K, V]
                if let HirExpr::Dict(items) = value {
                    if items.is_empty() && returned_vars.contains(name) {
                        // Only propagate if return type is a Dict with concrete key/value types
                        if let Type::Dict(_key_type, _val_type) = return_type {
                            // Don't override if we already have a concrete type
                            let should_update = match var_types.get(name) {
                                None => true,
                                Some(Type::Unknown) => true,
                                Some(Type::Dict(k, v)) if matches!(k.as_ref(), Type::Unknown) || matches!(v.as_ref(), Type::Unknown) => true,
                                _ => false,
                            };
                            if should_update {
                                var_types.insert(name.clone(), return_type.clone());
                            }
                        }
                    }
                }
            }
            HirStmt::Return(Some(expr)) => {
                // If returning a simple variable, propagate return type to it
                if let HirExpr::Var(var_name) = expr {
                    // Check if the variable has an Unknown or weaker type
                    let should_update = match var_types.get(var_name) {
                        None => true,
                        Some(Type::Unknown) => true,
                        Some(Type::List(elem)) if matches!(elem.as_ref(), Type::Unknown) => true,
                        Some(Type::Dict(k, v)) if matches!(k.as_ref(), Type::Unknown) || matches!(v.as_ref(), Type::Unknown) => true,
                        _ => false,
                    };
                    if should_update {
                        var_types.insert(var_name.clone(), return_type.clone());
                    }
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                propagate_return_type_impl(then_body, var_types, return_type, returned_vars);
                if let Some(else_stmts) = else_body {
                    propagate_return_type_impl(else_stmts, var_types, return_type, returned_vars);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                propagate_return_type_impl(body, var_types, return_type, returned_vars);
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                propagate_return_type_impl(body, var_types, return_type, returned_vars);
                for handler in handlers {
                    propagate_return_type_impl(&handler.body, var_types, return_type, returned_vars);
                }
                if let Some(orelse_stmts) = orelse {
                    propagate_return_type_impl(orelse_stmts, var_types, return_type, returned_vars);
                }
                if let Some(finally_stmts) = finalbody {
                    propagate_return_type_impl(finally_stmts, var_types, return_type, returned_vars);
                }
            }
            HirStmt::With { body, .. } => {
                propagate_return_type_impl(body, var_types, return_type, returned_vars);
            }
            _ => {}
        }
    }
}

/// Infer expression type with access to variable type environment
pub(crate) fn infer_expr_type_with_env(
    expr: &HirExpr,
    var_types: &std::collections::HashMap<String, Type>,
) -> Type {
    match expr {
        // DEPYLER-0415: Look up variable types in the environment
        HirExpr::Var(name) => {
            // First, try to find in environment
            if let Some(ty) = var_types.get(name) {
                return ty.clone();
            }
            // GH-70: Fallback heuristic for common string variable names
            // (useful when variables come from tuple unpacking not tracked in environment)
            let name_str = name.as_str();
            if name_str == "timestamp"
                || name_str == "message"
                || name_str == "level"
                || name_str.ends_with("_str")
                || name_str.ends_with("_string")
                || name_str.ends_with("_message")
                || name_str.ends_with("timestamp")
            {
                Type::String
            } else {
                Type::Unknown
            }
        }
        // For other expressions, delegate to the simple version
        // but recurse with environment for nested expressions
        HirExpr::Binary { op, left, right } => {
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
                return Type::Bool;
            }

            // DEPYLER-0420/1132: Detect list repeat patterns: [elem] * n or n * [elem]
            // DEPYLER-1132: Always return List (Vec) since py_mul trait returns Vec<T>
            if matches!(op, BinOp::Mul) {
                match (left.as_ref(), right.as_ref()) {
                    // Pattern: [elem] * n → Vec<T>
                    (HirExpr::List(elems), HirExpr::Literal(Literal::Int(n)))
                        if elems.len() == 1 && *n > 0 =>
                    {
                        let elem_type = infer_expr_type_with_env(&elems[0], var_types);
                        return Type::List(Box::new(elem_type));
                    }
                    // Pattern: n * [elem] → Vec<T>
                    (HirExpr::Literal(Literal::Int(n)), HirExpr::List(elems))
                        if elems.len() == 1 && *n > 0 =>
                    {
                        let elem_type = infer_expr_type_with_env(&elems[0], var_types);
                        return Type::List(Box::new(elem_type));
                    }
                    _ => {}
                }
            }

            // DEPYLER-0808: Power with negative exponent always returns float
            if matches!(op, BinOp::Pow) && is_negative_int_expr(right) {
                return Type::Float;
            }

            let left_type = infer_expr_type_with_env(left, var_types);
            let right_type = infer_expr_type_with_env(right, var_types);
            if matches!(left_type, Type::Float) || matches!(right_type, Type::Float) {
                Type::Float
            } else if !matches!(left_type, Type::Unknown) {
                left_type
            } else {
                right_type
            }
        }
        HirExpr::IfExpr { body, orelse, .. } => {
            let body_type = infer_expr_type_with_env(body, var_types);
            if !matches!(body_type, Type::Unknown) {
                body_type
            } else {
                infer_expr_type_with_env(orelse, var_types)
            }
        }
        // DEPYLER-0420: Handle tuples with environment for variable lookups
        HirExpr::Tuple(elems) => {
            let elem_types: Vec<Type> = elems
                .iter()
                .map(|e| infer_expr_type_with_env(e, var_types))
                .collect();
            Type::Tuple(elem_types)
        }
        // DEPYLER-REARCH-001: Handle MethodCall with environment for variable type lookups
        HirExpr::MethodCall { object, method, .. } => {
            // DEPYLER-REARCH-001: Check if this is a module method call (e.g., json.load(), csv.reader())
            // These need special handling because the module itself doesn't have a type
            if let HirExpr::Var(module_name) = object.as_ref() {
                match (module_name.as_str(), method.as_str()) {
                    // json module methods
                    // json.load/loads returns arbitrary JSON (dict, list, string, number, bool, null)
                    // which maps to serde_json::Value, not HashMap
                    ("json", "load") | ("json", "loads") => {
                        return Type::Custom("serde_json::Value".to_string());
                    }
                    ("json", "dump") => return Type::None,
                    ("json", "dumps") => return Type::String,
                    // csv module methods
                    ("csv", "reader") => {
                        return Type::List(Box::new(Type::List(Box::new(Type::String))));
                    }
                    ("csv", "DictReader") => {
                        return Type::List(Box::new(Type::Dict(
                            Box::new(Type::String),
                            Box::new(Type::String),
                        )));
                    }
                    ("csv", "writer") | ("csv", "DictWriter") => return Type::Unknown,
                    // DEPYLER-0646: subprocess.run() returns CompletedProcess struct
                    // Updated from tuple to struct per DEPYLER-0627
                    ("subprocess", "run") => {
                        return Type::Custom("CompletedProcess".to_string());
                    }
                    // DEPYLER-0532: regex module methods
                    ("re", "findall") | ("regex", "findall") => {
                        return Type::List(Box::new(Type::String));
                    }
                    ("re", "match")
                    | ("re", "search")
                    | ("regex", "match")
                    | ("regex", "search") => {
                        return Type::Optional(Box::new(Type::Custom("Match".to_string())));
                    }
                    ("re", "split") | ("regex", "split") => {
                        return Type::List(Box::new(Type::String));
                    }
                    ("re", "sub") | ("regex", "sub") | ("re", "replace") | ("regex", "replace") => {
                        return Type::String;
                    }
                    _ => {} // Fall through to regular method handling
                }
            }

            // For non-module method calls, infer the object type using the environment
            let object_type = infer_expr_type_with_env(object, var_types);

            match method.as_str() {
                // .copy() returns same type as object
                "copy" => object_type,
                // String methods that return String
                "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "replace" | "title"
                | "capitalize" | "join" | "format" => Type::String,
                // String methods that return bool
                "startswith" | "endswith" | "isdigit" | "isalpha" | "isalnum" | "isspace"
                | "isupper" | "islower" => Type::Bool,
                // String methods that return int
                "find" | "rfind" | "index" | "rindex" | "count" => Type::Int,
                // String methods that return list
                "split" | "splitlines" => Type::List(Box::new(Type::String)),
                // List/Dict methods
                "get" => {
                    // DEPYLER-0463: Special handling for serde_json::Value.get()
                    // Returns Option<&Value>, but for type inference we treat as Value
                    if matches!(object_type, Type::Custom(ref s) if s == "serde_json::Value") {
                        return Type::Custom("serde_json::Value".to_string());
                    }
                    // DEPYLER-0767: dict.get(key) returns Optional[T] in Python, not T
                    // This enables proper truthiness conversion (if value: → if value.is_some())
                    match object_type {
                        Type::Dict(_, val) => Type::Optional(val),
                        Type::List(elem) => *elem,
                        _ => Type::Unknown,
                    }
                }
                "pop" => match object_type {
                    Type::List(elem) => *elem,
                    Type::Dict(_, val) => *val,
                    _ => Type::Unknown,
                },
                "keys" => Type::List(Box::new(Type::Unknown)),
                "values" => Type::List(Box::new(Type::Unknown)),
                "items" => Type::List(Box::new(Type::Tuple(vec![Type::Unknown, Type::Unknown]))),
                // DEPYLER-0532: Regex methods that return lists
                "findall" | "finditer" => Type::List(Box::new(Type::String)),
                "groups" => Type::List(Box::new(Type::String)),
                // DEPYLER-0555: Additional string-returning methods for return type inference
                // DEPYLER-0565: Added hexdigest for hashlib
                // DEPYLER-0620: Added file read methods that return String
                // Note: upper/lower/strip/etc already covered above
                "isoformat" | "strftime" | "to_string" | "to_str" | "encode" | "decode"
                | "hexdigest" | "digest" | "read" | "readline" => Type::String,
                // DEPYLER-0620: File readlines returns list of strings
                "readlines" => Type::List(Box::new(Type::String)),
                // datetime methods that return other types
                "timestamp" => Type::Float,
                // DEPYLER-0592/1025: Use std types (NASA mode default)
                "date" => Type::Custom("std::time::SystemTime".to_string()),
                "time" => Type::Custom("std::time::SystemTime".to_string()),
                // DEPYLER-0750: Counter.most_common() returns list of (key, count) tuples
                "most_common" => Type::List(Box::new(Type::Tuple(vec![Type::String, Type::Int]))),
                _ => {
                    // DEPYLER-1007: Check if this is a user-defined class method call
                    // If object_type is Custom("ClassName"), look up (ClassName, method) in class_method_return_types
                    // This enables return type inference for expressions like `p.distance_squared()`
                    // Note: class_method_return_types needs to be passed via infer_expr_type_with_class_methods
                    // For backward compatibility, this returns Unknown when no class info is available
                    Type::Unknown
                }
            }
        }
        // DEPYLER-0463: Handle Index with environment for serde_json::Value preservation
        HirExpr::Index { base, .. } => {
            let base_type = infer_expr_type_with_env(base, var_types);
            // When indexing into serde_json::Value, result is also Value (could be any JSON type)
            if matches!(base_type, Type::Custom(ref s) if s == "serde_json::Value") {
                return Type::Custom("serde_json::Value".to_string());
            }
            // For other containers, extract element type
            match base_type {
                Type::List(elem) => *elem,
                Type::Dict(_, val) => *val,
                Type::Tuple(elems) => elems.first().cloned().unwrap_or(Type::Unknown),
                Type::String => Type::String,
                _ => Type::Unknown, // Changed from Type::Int to Unknown (more conservative)
            }
        }
        // GH-70: Handle Slice with environment-aware inference for string variables
        HirExpr::Slice { base, .. } => {
            // Use environment to resolve variables like "timestamp"
            let base_type = infer_expr_type_with_env(base, var_types);
            // String slicing returns String
            if matches!(base_type, Type::String) {
                Type::String
            } else {
                // For other types (lists, etc.), slicing returns same type
                base_type
            }
        }
        // DEPYLER-0517: Handle Attribute with environment-aware inference
        // This is needed to resolve types like `result.returncode` where `result`
        // is a subprocess.run() result stored in a variable
        HirExpr::Attribute { value, attr } => {
            // DEPYLER-0690: Handle module attribute access (sys.argv, sys.path, etc.)
            // Check if value is a module name and attr is a known module attribute
            if let HirExpr::Var(module_name) = value.as_ref() {
                match (module_name.as_str(), attr.as_str()) {
                    // sys module attributes
                    ("sys", "argv") => return Type::List(Box::new(Type::String)),
                    ("sys", "path") => return Type::List(Box::new(Type::String)),
                    ("sys", "version") => return Type::String,
                    ("sys", "version_info") => {
                        return Type::Tuple(vec![Type::Int, Type::Int, Type::Int])
                    }
                    ("sys", "maxsize") => return Type::Int,
                    ("sys", "platform") => return Type::String,
                    // os module attributes
                    ("os", "environ") => {
                        return Type::Dict(Box::new(Type::String), Box::new(Type::String))
                    }
                    ("os", "name") => return Type::String,
                    ("os", "sep") | ("os", "pathsep") | ("os", "linesep") => return Type::String,
                    _ => {} // Fall through to existing handling
                }
            }

            // Get the base type using the environment
            let base_type = infer_expr_type_with_env(value, var_types);

            // Handle subprocess.run() result tuple attributes (returncode, stdout, stderr)
            // Type is now Tuple([Int, String, String]), attributes map to tuple indices
            if let Type::Tuple(ref types) = base_type {
                if types.len() == 3 {
                    return match attr.as_str() {
                        "returncode" => Type::Int,    // .0
                        "stdout" => Type::String,     // .1
                        "stderr" => Type::String,     // .2
                        _ => Type::Unknown,
                    };
                }
            }

            // Common attributes with known types
            match attr.as_str() {
                "real" | "imag" => Type::Float,
                // DEPYLER-0517: Common subprocess result attributes (fallback)
                "returncode" => Type::Int,
                "stdout" | "stderr" => Type::String,
                _ => Type::Unknown,
            }
        }
        // DEPYLER-0609: Handle ListComp with JSON Value propagation for return type inference
        HirExpr::ListComp { element, generators } => {
            // Create extended environment with loop variable bindings
            let mut extended_env = var_types.clone();

            // Bind loop variables based on iterator type
            for gen in generators {
                let iter_type = infer_expr_type_with_env(&gen.iter, &extended_env);

                // Determine the element type from the iterator
                let elem_type = match &iter_type {
                    // JSON Value iteration yields Value elements
                    Type::Custom(s) if s == "serde_json::Value" || s.contains("Value") => {
                        Type::Custom("serde_json::Value".to_string())
                    }
                    // List iteration yields element type
                    Type::List(inner) => *inner.clone(),
                    // Dict iteration yields keys
                    Type::Dict(k, _) => *k.clone(),
                    // Set iteration yields element type
                    Type::Set(inner) => *inner.clone(),
                    _ => Type::Unknown,
                };

                // Bind the target variable to the element type
                // gen.target is a Symbol (String) representing the variable name
                extended_env.insert(gen.target.clone(), elem_type);
            }

            // Infer element type with the extended environment
            let elem_type = infer_expr_type_with_env(element, &extended_env);
            Type::List(Box::new(elem_type))
        }
        // For other cases, use the simple version
        _ => infer_expr_type_simple(expr),
    }
}

// NOTE: collect_return_types() removed - replaced by collect_return_types_with_env()
// which provides better type inference using variable type environment (DEPYLER-0415)

/// DEPYLER-1007: Infer expression type with access to both variable types and class method return types
/// This enables return type inference for user-defined class method calls like `p.distance_squared()`
pub(crate) fn infer_expr_type_with_class_methods(
    expr: &HirExpr,
    var_types: &std::collections::HashMap<String, Type>,
    class_method_return_types: &std::collections::HashMap<(String, String), Type>,
) -> Type {
    match expr {
        // DEPYLER-1007: Handle method calls on typed variables (e.g., p.distance_squared())
        HirExpr::MethodCall { object, method, .. } => {
            // First try to get the object's type from the environment
            let object_type = infer_expr_type_with_class_methods(object, var_types, class_method_return_types);

            // If object is a Custom type (user-defined class), look up the method return type
            if let Type::Custom(class_name) = &object_type {
                if let Some(ret_type) = class_method_return_types.get(&(class_name.clone(), method.clone())) {
                    return ret_type.clone();
                }
            }

            // Fall back to standard inference
            infer_expr_type_with_env(expr, var_types)
        }
        // For all other expressions, delegate to standard inference
        _ => infer_expr_type_with_env(expr, var_types),
    }
}

/// DEPYLER-1007: Collect return types with class method return type awareness
/// This version uses infer_expr_type_with_class_methods for proper class method return type inference
pub(crate) fn collect_return_types_with_class_methods(
    stmts: &[HirStmt],
    types: &mut Vec<Type>,
    var_types: &std::collections::HashMap<String, Type>,
    class_method_return_types: &std::collections::HashMap<(String, String), Type>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                types.push(infer_expr_type_with_class_methods(expr, var_types, class_method_return_types));
            }
            HirStmt::Return(None) => {
                types.push(Type::None);
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_return_types_with_class_methods(then_body, types, var_types, class_method_return_types);
                if let Some(else_stmts) = else_body {
                    collect_return_types_with_class_methods(else_stmts, types, var_types, class_method_return_types);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                collect_return_types_with_class_methods(body, types, var_types, class_method_return_types);
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                collect_return_types_with_class_methods(body, types, var_types, class_method_return_types);
                for handler in handlers {
                    collect_return_types_with_class_methods(&handler.body, types, var_types, class_method_return_types);
                }
                if let Some(orelse_stmts) = orelse {
                    collect_return_types_with_class_methods(orelse_stmts, types, var_types, class_method_return_types);
                }
                if let Some(finally_stmts) = finalbody {
                    collect_return_types_with_class_methods(finally_stmts, types, var_types, class_method_return_types);
                }
            }
            HirStmt::With { body, .. } => {
                collect_return_types_with_class_methods(body, types, var_types, class_method_return_types);
            }
            _ => {}
        }
    }
}

/// Simple expression type inference without context
/// Handles common cases like literals, comparisons, and arithmetic
/// DEPYLER-0600: Made pub(crate) for stmt_gen comprehension type tracking
pub(crate) fn infer_expr_type_simple(expr: &HirExpr) -> Type {
    match expr {
        HirExpr::Literal(lit) => literal_to_type(lit),
        HirExpr::Binary { op, left, right } => {
            // Comparison operators always return bool
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
                return Type::Bool;
            }

            // DEPYLER-0420/1132: Detect list repeat patterns: [elem] * n or n * [elem]
            // DEPYLER-1132: Always return List (Vec) since py_mul trait returns Vec<T>
            if matches!(op, BinOp::Mul) {
                match (left.as_ref(), right.as_ref()) {
                    // Pattern: [elem] * n → Vec<T>
                    (HirExpr::List(elems), HirExpr::Literal(Literal::Int(n)))
                        if elems.len() == 1 && *n > 0 =>
                    {
                        let elem_type = infer_expr_type_simple(&elems[0]);
                        return Type::List(Box::new(elem_type));
                    }
                    // Pattern: n * [elem] → Vec<T>
                    (HirExpr::Literal(Literal::Int(n)), HirExpr::List(elems))
                        if elems.len() == 1 && *n > 0 =>
                    {
                        let elem_type = infer_expr_type_simple(&elems[0]);
                        return Type::List(Box::new(elem_type));
                    }
                    _ => {}
                }
            }

            // DEPYLER-0808: Power with negative exponent always returns float
            // In Python: 2 ** -1 = 0.5, even for int ** int with negative exp
            if matches!(op, BinOp::Pow) && is_negative_int_expr(right) {
                return Type::Float;
            }

            // For arithmetic, infer from operands
            let left_type = infer_expr_type_simple(left);
            let right_type = infer_expr_type_simple(right);
            // Float takes precedence
            if matches!(left_type, Type::Float) || matches!(right_type, Type::Float) {
                Type::Float
            } else if !matches!(left_type, Type::Unknown) {
                left_type
            } else {
                right_type
            }
        }
        HirExpr::Unary { op, operand } => {
            if matches!(op, UnaryOp::Not) {
                Type::Bool
            } else {
                infer_expr_type_simple(operand)
            }
        }
        HirExpr::List(elems) => {
            if elems.is_empty() {
                Type::List(Box::new(Type::Unknown))
            } else {
                Type::List(Box::new(infer_expr_type_simple(&elems[0])))
            }
        }
        HirExpr::Tuple(elems) => {
            let elem_types: Vec<Type> = elems.iter().map(infer_expr_type_simple).collect();
            Type::Tuple(elem_types)
        }
        HirExpr::Set(elems) => {
            if elems.is_empty() {
                Type::Set(Box::new(Type::Unknown))
            } else {
                Type::Set(Box::new(infer_expr_type_simple(&elems[0])))
            }
        }
        HirExpr::Dict(pairs) => {
            if pairs.is_empty() {
                Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown))
            } else {
                let key_type = infer_expr_type_simple(&pairs[0].0);
                // DEPYLER-1051: Check ALL value types for heterogeneous dicts
                // If any value has a different type, use Unknown (→ DepylerValue)
                let first_val_type = infer_expr_type_simple(&pairs[0].1);
                let is_homogeneous = pairs.iter().skip(1).all(|(_, v)| {
                    let v_type = infer_expr_type_simple(v);
                    // Unknown can coexist with any type (it will be inferred)
                    matches!(v_type, Type::Unknown) || v_type == first_val_type
                });
                let val_type = if is_homogeneous {
                    first_val_type
                } else {
                    // Mixed types → DepylerValue (via Unknown mapping)
                    Type::Unknown
                };
                Type::Dict(Box::new(key_type), Box::new(val_type))
            }
        }
        HirExpr::IfExpr { body, orelse, .. } => {
            // Try to infer from either branch
            let body_type = infer_expr_type_simple(body);
            if !matches!(body_type, Type::Unknown) {
                body_type
            } else {
                infer_expr_type_simple(orelse)
            }
        }
        // DEPYLER-0414: Add Index expression type inference
        HirExpr::Index { base, .. } => {
            // For arr[i], return element type of the container
            match infer_expr_type_simple(base) {
                Type::List(elem) => *elem,
                Type::Tuple(elems) => elems.first().cloned().unwrap_or(Type::Unknown),
                Type::Dict(_, val) => *val,
                Type::String => Type::String, // string indexing returns char/string
                _ => Type::Int,               // Default to Int for array-like indexing
            }
        }
        // DEPYLER-0414: Add Slice expression type inference
        HirExpr::Slice { base, .. } => {
            // Slicing returns same container type
            infer_expr_type_simple(base)
        }
        // DEPYLER-0414: Add FString type inference (always String)
        HirExpr::FString { .. } => Type::String,
        // DEPYLER-0414: Add Call expression type inference
        HirExpr::Call { func, .. } => {
            // DEPYLER-REARCH-001: Handle module function calls
            // Check both qualified (json.load) and unqualified (load) names
            match func.as_str() {
                // json module functions (qualified names)
                // DEPYLER-0609: json.load/loads returns serde_json::Value (not Dict)
                // because JSON can be dict, array, string, number, bool, or null
                "json.load" | "json.loads" => {
                    Type::Custom("serde_json::Value".to_string())
                }
                "json.dump" => Type::None,
                "json.dumps" => Type::String,
                // csv module functions (qualified names)
                "csv.reader" => Type::List(Box::new(Type::List(Box::new(Type::String)))),
                "csv.writer" => Type::Unknown,
                "csv.DictReader" => Type::List(Box::new(Type::Dict(
                    Box::new(Type::String),
                    Box::new(Type::String),
                ))),
                "csv.DictWriter" => Type::Unknown,
                // Common builtin functions with known return types
                "len" | "int" | "abs" | "ord" | "hash" => Type::Int,
                "float" => Type::Float,
                "str" | "repr" | "chr" | "input" => Type::String,
                "bool" => Type::Bool,
                "list" => Type::List(Box::new(Type::Unknown)),
                "dict" => Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)),
                "set" => Type::Set(Box::new(Type::Unknown)),
                "tuple" => Type::Tuple(vec![]),
                "range" => Type::List(Box::new(Type::Int)),
                "sum" | "min" | "max" => Type::Int, // Common numeric aggregations
                "zeros" | "ones" | "full" => Type::List(Box::new(Type::Int)),
                // DEPYLER-0623: open() returns a file handle (owned std::fs::File)
                "open" => Type::Custom("std::fs::File".to_string()),
                // DEPYLER-0942: pathlib.Path() and variants return PathBuf
                "Path" | "PurePath" | "PurePosixPath" | "PureWindowsPath" => {
                    Type::Custom("PathBuf".to_string())
                }
                _ => Type::Unknown,
            }
        }
        // DEPYLER-0414: Add MethodCall expression type inference
        HirExpr::MethodCall { object, method, .. } => {
            // DEPYLER-0931: Check if this is subprocess module call
            if let HirExpr::Var(obj_var) = object.as_ref() {
                if obj_var == "subprocess" {
                    return match method.as_str() {
                        // subprocess.Popen() returns std::process::Child
                        "Popen" => Type::Custom("std::process::Child".to_string()),
                        // subprocess.run() returns CompletedProcess
                        "run" => Type::Custom("CompletedProcess".to_string()),
                        // subprocess.call() returns int (exit code)
                        "call" | "check_call" => Type::Int,
                        // subprocess.check_output() returns bytes/string
                        "check_output" => Type::String,
                        _ => Type::Unknown,
                    };
                }
            }
            // DEPYLER-0931: Check if object is a subprocess Child (from proc = subprocess.Popen(...))
            // In Python: proc.wait() returns int (exit code)
            // In Rust: Child::wait() returns io::Result<ExitStatus>, we extract code()
            if method == "wait" {
                // Check if object type is Child (it will be after assignment from Popen)
                let obj_type = infer_expr_type_simple(object);
                if matches!(&obj_type, Type::Custom(s) if s.contains("Child")) {
                    return Type::Int;
                }
            }
            match method.as_str() {
                // DEPYLER-REARCH-001: .copy() returns same type as object
                "copy" => infer_expr_type_simple(object),
                // DEPYLER-0931: Process management methods
                "wait" => Type::Int, // Child.wait() returns exit code
                "poll" => Type::Optional(Box::new(Type::Int)), // Child.poll() returns Option<i32>
                // String methods that return String
                "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "replace" | "title"
                | "capitalize" | "join" | "format" => Type::String,
                // String methods that return bool
                "startswith" | "endswith" | "isdigit" | "isalpha" | "isalnum" | "isspace"
                | "isupper" | "islower" => Type::Bool,
                // String methods that return int
                "find" | "rfind" | "index" | "rindex" | "count" => Type::Int,
                // String methods that return list
                "split" | "splitlines" => Type::List(Box::new(Type::String)),
                // DEPYLER-0620: File read methods
                "read" | "readline" => Type::String,
                "readlines" => Type::List(Box::new(Type::String)),
                // List/Dict methods
                "get" => {
                    // dict.get() returns element type
                    match infer_expr_type_simple(object) {
                        Type::Dict(_, val) => *val,
                        Type::List(elem) => *elem,
                        _ => Type::Unknown,
                    }
                }
                "pop" => match infer_expr_type_simple(object) {
                    Type::List(elem) => *elem,
                    Type::Dict(_, val) => *val,
                    _ => Type::Unknown,
                },
                "keys" => Type::List(Box::new(Type::Unknown)),
                "values" => Type::List(Box::new(Type::Unknown)),
                "items" => Type::List(Box::new(Type::Tuple(vec![Type::Unknown, Type::Unknown]))),
                // DEPYLER-0713: json.loads() and json.load() return serde_json::Value
                // This is critical for type tracking: data = json.loads(s) → data is Value
                "loads" | "load" => {
                    // Check if this is json.loads() (object is Var("json"))
                    if let HirExpr::Var(obj_var) = object.as_ref() {
                        if obj_var == "json" {
                            return Type::Custom("serde_json::Value".to_string());
                        }
                    }
                    Type::Unknown
                }
                _ => Type::Unknown,
            }
        }
        // DEPYLER-0414: Add ListComp type inference
        HirExpr::ListComp { element, .. } => Type::List(Box::new(infer_expr_type_simple(element))),
        // DEPYLER-0414: Add SetComp type inference
        HirExpr::SetComp { element, .. } => Type::Set(Box::new(infer_expr_type_simple(element))),
        // DEPYLER-0414: Add DictComp type inference
        HirExpr::DictComp { key, value, .. } => Type::Dict(
            Box::new(infer_expr_type_simple(key)),
            Box::new(infer_expr_type_simple(value)),
        ),
        // DEPYLER-0414: Add Attribute type inference
        HirExpr::Attribute { value, attr } => {
            // DEPYLER-0517: Check if this is an attribute access on a subprocess result
            // Since we don't have var_types here, check if the base is a method call
            // on the subprocess module
            if let HirExpr::MethodCall { object, method, .. } = value.as_ref() {
                if let HirExpr::Var(module) = object.as_ref() {
                    if module == "subprocess" && method == "run" {
                        return match attr.as_str() {
                            "returncode" => Type::Int,
                            "stdout" | "stderr" => Type::String,
                            _ => Type::Unknown,
                        };
                    }
                }
            }

            // Common attributes with known types
            match attr.as_str() {
                "real" | "imag" => Type::Float,
                // DEPYLER-0517: Common subprocess result attributes (fallback)
                "returncode" => Type::Int,
                "stdout" | "stderr" => Type::String,
                _ => Type::Unknown,
            }
        }
        _ => Type::Unknown,
    }
}

/// DEPYLER-0808: Check if expression is a negative integer
/// Handles both direct negative literals and unary negation of positive integers
fn is_negative_int_expr(expr: &HirExpr) -> bool {
    match expr {
        // Direct negative literal: -1, -2, etc.
        HirExpr::Literal(Literal::Int(n)) => *n < 0,
        // Unary negation: -(1), -(x)
        HirExpr::Unary { op, operand } => {
            matches!(op, UnaryOp::Neg)
                && matches!(operand.as_ref(), HirExpr::Literal(Literal::Int(n)) if *n > 0)
        }
        _ => false,
    }
}

/// Convert literal to type
fn literal_to_type(lit: &Literal) -> Type {
    match lit {
        Literal::Int(_) => Type::Int,
        Literal::Float(_) => Type::Float,
        Literal::String(_) => Type::String,
        Literal::Bool(_) => Type::Bool,
        Literal::None => Type::None,
        Literal::Bytes(_) => Type::Unknown, // No direct Bytes type in Type enum
    }
}

// ========== Phase 3b: Return Type Generation ==========

/// GH-70: Infer parameter type from usage patterns in function body
/// Detects patterns:
/// - `a, b, c = param` → param is 3-tuple of strings
/// - `print(param)` → param needs Display trait → String
/// - `re.match(param, ...)` → param is String
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
            // Check if value is our parameter and target is tuple unpacking
            if var == param_name {
                if let AssignTarget::Tuple(elements) = target {
                    // Infer as tuple with N String elements (common case)
                    let elem_types = vec![Type::String; elements.len()];
                    return Some(Type::Tuple(elem_types));
                }
            }
        }

        // DEPYLER-0518: Pattern 1b: Assignment where value is an expression using param
        // Example: match = re.match(pattern, text, flags)
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
        // GH-70: `return item[0]` → infer item is indexable
        if let HirStmt::Return(Some(expr)) = stmt {
            if let Some(ty) = infer_type_from_expr_usage(param_name, expr) {
                return Some(ty);
            }
        }

        // DEPYLER-0518: Pattern 4: If statement - check condition and body
        if let HirStmt::If {
            condition,
            then_body,
            else_body,
        } = stmt
        {
            if let Some(ty) = infer_type_from_expr_usage(param_name, condition) {
                return Some(ty);
            }
            // Recursively check then body
            if let Some(ty) = infer_param_type_from_body(param_name, then_body) {
                return Some(ty);
            }
            // Recursively check else body
            if let Some(else_stmts) = else_body {
                if let Some(ty) = infer_param_type_from_body(param_name, else_stmts) {
                    return Some(ty);
                }
            }
        }

        // DEPYLER-0524: Pattern 5: With statement - check body for parameter usage
        // Example: with open(...) as f: f.write(content); content.endswith("\n")
        if let HirStmt::With { body, .. } = stmt {
            if let Some(ty) = infer_param_type_from_body(param_name, body) {
                return Some(ty);
            }
        }

        // DEPYLER-0524: Pattern 6: For loop - check body for parameter usage
        if let HirStmt::For { body, .. } = stmt {
            if let Some(ty) = infer_param_type_from_body(param_name, body) {
                return Some(ty);
            }
        }

        // DEPYLER-0524: Pattern 7: While loop - check condition and body
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

        // DEPYLER-0524: Pattern 8: Try/except - check all bodies
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

/// GH-70: Helper to infer type from expression usage
fn infer_type_from_expr_usage(param_name: &str, expr: &HirExpr) -> Option<Type> {
    match expr {
        // Pattern: print(param) or println(param) → param needs Display → String
        HirExpr::Call { func, args, kwargs } => {
            // DEPYLER-0950: Pattern: param(args...) → param is Callable
            // When a parameter is used as the callee of a function call, it's a callable
            // Example: def apply(f, x): return f(x) → f is Callable[[int], int]
            if func == param_name {
                // Infer param types from args (default to Int for untyped)
                // For now, use simple heuristic: count args and default to Int types
                let param_types: Vec<Type> = args.iter().map(|_| Type::Int).collect();
                // Return type defaults to Int (most common case)
                // This could be refined with more context
                return Some(Type::Generic {
                    base: "Callable".to_string(),
                    params: vec![Type::Tuple(param_types), Type::Int],
                });
            }

            // func is a Symbol (String), check if it's print/println
            if func == "print" || func == "println" {
                // Check if our parameter is used as an argument
                for arg in args {
                    if let HirExpr::Var(var_name) = arg {
                        if var_name == param_name {
                            return Some(Type::String);
                        }
                    }
                }
            }

            // DEPYLER-0518: Pattern: re.match(pattern, text), re.search(pattern, text), etc.
            // Both pattern and text parameters should be strings
            if func.starts_with("re.") || func == "re" {
                for arg in args {
                    if let HirExpr::Var(var_name) = arg {
                        if var_name == param_name {
                            return Some(Type::String);
                        }
                    }
                }
            }

            // DEPYLER-0737: subprocess.run(..., cwd=param) → param is String (path-like)
            // When a parameter is used as the cwd kwarg in subprocess.run, it's a path string
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

            // Recursively check arguments
            for arg in args {
                if let Some(ty) = infer_type_from_expr_usage(param_name, arg) {
                    return Some(ty);
                }
            }
            // Also recursively check kwargs values
            for (_, kwarg_value) in kwargs {
                if let Some(ty) = infer_type_from_expr_usage(param_name, kwarg_value) {
                    return Some(ty);
                }
            }
            None
        }

        // DEPYLER-0518: Pattern: method_call(param) where method expects string
        // Example: regex::Regex::new(pattern), compiled.find(text), re.match(pattern, text)
        HirExpr::MethodCall {
            object,
            method,
            args,
            kwargs,
        } => {
            // DEPYLER-0525: If param IS the object and method is a file I/O method,
            // then param must be a file-like object that implements Write or Read
            // Example: f.write(msg), f.read(), f.readline(), f.flush()
            let file_object_methods = [
                "write",
                "writelines",
                "read",
                "readline",
                "readlines",
                "flush",
                "close",
                "seek",
                "tell",
                "truncate",
            ];
            if let HirExpr::Var(var_name) = object.as_ref() {
                if var_name == param_name && file_object_methods.contains(&method.as_str()) {
                    // Return a custom type for file handles
                    // This will be mapped to &mut impl Write in code generation
                    return Some(Type::Custom("File".to_string()));
                }
            }

            // DEPYLER-0524: If param IS the object and method is a string method,
            // then param must be a string. Example: content.endswith("\n")
            let string_object_methods = [
                "strip",
                "lstrip",
                "rstrip",
                "startswith",
                "endswith",
                "split",
                "splitlines",
                "join",
                "upper",
                "lower",
                "title",
                "capitalize",
                "replace",
                "find",
                "rfind",
                "index",
                "rindex",
                "count",
                "isalpha",
                "isdigit",
                "isalnum",
                "isspace",
                "isupper",
                "islower",
                "encode",
                "format",
                "center",
                "ljust",
                "rjust",
                "zfill",
                "partition",
                "rpartition",
                "expandtabs",
                "swapcase",
                "casefold",
            ];
            if let HirExpr::Var(var_name) = object.as_ref() {
                if var_name == param_name && string_object_methods.contains(&method.as_str()) {
                    return Some(Type::String);
                }
            }

            // DEPYLER-0550: If param IS the object and method is a dict method,
            // then param must be a dict. Example: row.get(col), row.items()
            // This is critical for csv filter predicates like: row.get(column) == value
            let dict_object_methods = [
                "get",
                "items",
                "keys",
                "values",
                "pop",
                "popitem",
                "update",
                "setdefault",
                "clear",
                "copy",
            ];
            if let HirExpr::Var(var_name) = object.as_ref() {
                if var_name == param_name && dict_object_methods.contains(&method.as_str()) {
                    // Default to Dict<String, String> which is most common for CSV rows
                    return Some(Type::Dict(Box::new(Type::String), Box::new(Type::String)));
                }
            }

            // DEPYLER-0518: Check if this is a module method call like re.match(), re.search()
            // These expect string arguments
            if let HirExpr::Var(module_name) = object.as_ref() {
                let regex_modules = ["re", "regex"];
                let regex_methods = [
                    "match", "search", "findall", "sub", "subn", "split", "compile",
                ];

                if regex_modules.contains(&module_name.as_str())
                    && regex_methods.contains(&method.as_str())
                {
                    // First two args (pattern, text) are strings
                    for arg in args.iter().take(2) {
                        if let HirExpr::Var(var_name) = arg {
                            if var_name == param_name {
                                return Some(Type::String);
                            }
                        }
                    }
                }

                // DEPYLER-0554: datetime.datetime.fromtimestamp(param) → param is f64
                // datetime.datetime.now() doesn't have param, but fromtimestamp does
                if module_name == "datetime" && method == "fromtimestamp" {
                    if let Some(HirExpr::Var(var_name)) = args.first() {
                        if var_name == param_name {
                            return Some(Type::Float);
                        }
                    }
                }
            }

            // DEPYLER-0554: Handle datetime.datetime attribute access → fromtimestamp method
            // Pattern: datetime.datetime.fromtimestamp(timestamp) where datetime.datetime is the object
            if let HirExpr::Attribute { value, attr } = object.as_ref() {
                if let HirExpr::Var(module_name) = value.as_ref() {
                    if module_name == "datetime" && attr == "datetime" && method == "fromtimestamp"
                    {
                        if let Some(HirExpr::Var(var_name)) = args.first() {
                            if var_name == param_name {
                                return Some(Type::Float);
                            }
                        }
                    }
                }
            }

            // DEPYLER-0737: subprocess.run(..., cwd=param) → param is String (path-like)
            // When a parameter is used as the cwd kwarg in subprocess.run, it's a path string
            // This prevents incorrect generic inference (Option<T> → Option<String>)
            if let HirExpr::Var(module_name) = object.as_ref() {
                if module_name == "subprocess" && method == "run" {
                    // Check kwargs for cwd parameter
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
            }

            // Methods that expect string arguments (for method calls on objects)
            let string_methods = [
                "find",
                "search",
                "match",
                "sub",
                "replace",
                "replace_all",
                "is_match",
                "captures",
                "find_iter",
                "split",
                "strip",
                "lstrip",
                "rstrip",
                "startswith",
                "endswith",
                "contains",
                "encode",
                "decode",
            ];
            if string_methods.contains(&method.as_str()) {
                for arg in args {
                    if let HirExpr::Var(var_name) = arg {
                        if var_name == param_name {
                            return Some(Type::String);
                        }
                    }
                }
            }

            // Recursively check arguments
            for arg in args {
                if let Some(ty) = infer_type_from_expr_usage(param_name, arg) {
                    return Some(ty);
                }
            }
            // Also recursively check kwargs values
            for (_, kwarg_value) in kwargs {
                if let Some(ty) = infer_type_from_expr_usage(param_name, kwarg_value) {
                    return Some(ty);
                }
            }
            // Also check the object expression
            infer_type_from_expr_usage(param_name, object)
        }
        // Pattern: f-string with param → param needs Display → String
        HirExpr::FString { parts } => {
            for part in parts {
                if let crate::hir::FStringPart::Expr(val_expr) = part {
                    if let HirExpr::Var(var_name) = val_expr.as_ref() {
                        if var_name == param_name {
                            return Some(Type::String);
                        }
                    }
                }
            }
            None
        }
        // Pattern: param[index] → param is indexable
        // GH-70 + DEPYLER-0552: When a parameter is used with indexing:
        // - param["key"] (string index) → Dict<String, Value> (dictionary access)
        // - param[0] (integer index) → Vec<Int> (list access)
        HirExpr::Index { base, index } => {
            if let HirExpr::Var(var_name) = base.as_ref() {
                if var_name == param_name {
                    // DEPYLER-0552: Check if index is a string literal (dict access)
                    // or an f-string (also dict access)
                    let is_string_key = matches!(
                        index.as_ref(),
                        HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::FString { .. }
                    );
                    // Also check for string variable patterns like info[key] where key is "path"
                    let is_likely_string_key = if let HirExpr::Var(idx_name) = index.as_ref() {
                        // Common string key variable names
                        idx_name == "key" || idx_name == "k" || idx_name.ends_with("_key")
                    } else {
                        false
                    };

                    if is_string_key || is_likely_string_key {
                        // Dict access: param["key"] → HashMap<String, serde_json::Value>
                        return Some(Type::Dict(
                            Box::new(Type::String),
                            Box::new(Type::Custom("serde_json::Value".to_string())),
                        ));
                    }
                    // Default to Vec<i64> for integer indexing
                    return Some(Type::List(Box::new(Type::Int)));
                }
            }
            // Recursively check base expression
            infer_type_from_expr_usage(param_name, base)
        }
        // Pattern: param[start:stop] → param is sliceable → String or Vec
        HirExpr::Slice { base, .. } => {
            if let HirExpr::Var(var_name) = base.as_ref() {
                if var_name == param_name {
                    // Slicing is common on strings, default to String
                    return Some(Type::String);
                }
            }
            infer_type_from_expr_usage(param_name, base)
        }
        // Pattern: param * N, param + N, etc. → param is numeric → Int
        // GH-70: Binary operations with param suggest numeric type
        HirExpr::Binary {
            left, right, op, ..
        } => {
            use crate::hir::BinOp;

            // DEPYLER-0554: Pattern: param == "literal" or param != "literal" → param is String
            // Example: if algorithm == "md5": → algorithm must be String/&str
            if matches!(op, BinOp::Eq | BinOp::NotEq) {
                // Check if param is compared to a string literal
                if let HirExpr::Var(var_name) = left.as_ref() {
                    if var_name == param_name
                        && matches!(
                            right.as_ref(),
                            HirExpr::Literal(crate::hir::Literal::String(_))
                        )
                    {
                        return Some(Type::String);
                    }
                }
                // Also check the reverse: "literal" == param
                if let HirExpr::Var(var_name) = right.as_ref() {
                    if var_name == param_name
                        && matches!(
                            left.as_ref(),
                            HirExpr::Literal(crate::hir::Literal::String(_))
                        )
                    {
                        return Some(Type::String);
                    }
                }
            }

            // DEPYLER-0566: Pattern: param and something, param or something → param is Bool
            // Example: if include_hash and "hash" in info: → include_hash must be bool
            if matches!(op, BinOp::And | BinOp::Or) {
                // Check if param is used directly as a boolean operand
                if let HirExpr::Var(var_name) = left.as_ref() {
                    if var_name == param_name {
                        return Some(Type::Bool);
                    }
                }
                if let HirExpr::Var(var_name) = right.as_ref() {
                    if var_name == param_name {
                        return Some(Type::Bool);
                    }
                }
            }

            // DEPYLER-0524: Pattern: param in string → param is String (substring check)
            // Example: if pattern in line: → pattern must be String for .contains()
            // DEPYLER-0554: Pattern: param in ["a", "b"] or param not in [...] → param is String
            if matches!(op, BinOp::In | BinOp::NotIn) {
                if let HirExpr::Var(var_name) = left.as_ref() {
                    if var_name == param_name {
                        // Check if right side is a list of strings
                        if let HirExpr::List(elements) = right.as_ref() {
                            if elements.iter().all(|e| {
                                matches!(e, HirExpr::Literal(crate::hir::Literal::String(_)))
                            }) {
                                return Some(Type::String);
                            }
                        }
                        // In Python, "x in y" where y is string → x is also string
                        return Some(Type::String);
                    }
                }
            }

            // Check if param is used on left side
            if let HirExpr::Var(var_name) = left.as_ref() {
                if var_name == param_name {
                    // For arithmetic ops, infer numeric type
                    if matches!(
                        op,
                        BinOp::Add
                            | BinOp::Sub
                            | BinOp::Mul
                            | BinOp::Div
                            | BinOp::FloorDiv
                            | BinOp::Mod
                    ) {
                        return Some(Type::Int);
                    }
                }
            }
            // Check if param is used on right side
            if let HirExpr::Var(var_name) = right.as_ref() {
                if var_name == param_name
                    && matches!(
                        op,
                        BinOp::Add
                            | BinOp::Sub
                            | BinOp::Mul
                            | BinOp::Div
                            | BinOp::FloorDiv
                            | BinOp::Mod
                    )
                {
                    return Some(Type::Int);
                }
            }
            // Recursively check subexpressions
            infer_type_from_expr_usage(param_name, left)
                .or_else(|| infer_type_from_expr_usage(param_name, right))
        }
        // DEPYLER-0524: Unary expressions - check the operand
        // Example: not content.endswith("\n") → check content.endswith("\n")
        HirExpr::Unary { operand, .. } => infer_type_from_expr_usage(param_name, operand),
        // DEPYLER-0524: List comprehensions - check element and generators
        HirExpr::ListComp {
            element,
            generators,
        } => {
            // Check element expression
            if let Some(ty) = infer_type_from_expr_usage(param_name, element) {
                return Some(ty);
            }
            // Check generator conditions
            for gen in generators {
                if let HirExpr::Var(var_name) = &*gen.iter {
                    if var_name == param_name {
                        return Some(Type::String); // Iterating over param suggests it's iterable
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
        // DEPYLER-0524: Generator expressions - same as list comprehensions
        HirExpr::GeneratorExp {
            element,
            generators,
        } => {
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
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, HirExpr, HirStmt, Literal, UnaryOp, Type, AssignTarget, HirFunction, FunctionProperties, ExceptHandler};
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    // ==========================================================================
    // Tests for is_rust_keyword
    // ==========================================================================

    #[test]
    fn test_is_rust_keyword_basic_keywords() {
        assert!(is_rust_keyword("fn"));
        assert!(is_rust_keyword("let"));
        assert!(is_rust_keyword("mut"));
        assert!(is_rust_keyword("if"));
        assert!(is_rust_keyword("else"));
        assert!(is_rust_keyword("match"));
        assert!(is_rust_keyword("loop"));
        assert!(is_rust_keyword("while"));
        assert!(is_rust_keyword("for"));
        assert!(is_rust_keyword("return"));
    }

    #[test]
    fn test_is_rust_keyword_type_keywords() {
        assert!(is_rust_keyword("struct"));
        assert!(is_rust_keyword("enum"));
        assert!(is_rust_keyword("trait"));
        assert!(is_rust_keyword("impl"));
        assert!(is_rust_keyword("type"));
        assert!(is_rust_keyword("Self"));
        assert!(is_rust_keyword("self"));
    }

    #[test]
    fn test_is_rust_keyword_async_await() {
        assert!(is_rust_keyword("async"));
        assert!(is_rust_keyword("await"));
    }

    #[test]
    fn test_is_rust_keyword_reserved_for_future() {
        assert!(is_rust_keyword("abstract"));
        assert!(is_rust_keyword("become"));
        assert!(is_rust_keyword("box"));
        assert!(is_rust_keyword("do"));
        assert!(is_rust_keyword("final"));
        assert!(is_rust_keyword("macro"));
        assert!(is_rust_keyword("override"));
        assert!(is_rust_keyword("priv"));
        assert!(is_rust_keyword("typeof"));
        assert!(is_rust_keyword("unsized"));
        assert!(is_rust_keyword("virtual"));
        assert!(is_rust_keyword("yield"));
        assert!(is_rust_keyword("try"));
    }

    #[test]
    fn test_is_rust_keyword_not_keywords() {
        assert!(!is_rust_keyword("foo"));
        assert!(!is_rust_keyword("bar"));
        assert!(!is_rust_keyword("my_func"));
        assert!(!is_rust_keyword("String"));
        assert!(!is_rust_keyword("Vec"));
        assert!(!is_rust_keyword("Option"));
        assert!(!is_rust_keyword("Result"));
    }

    // ==========================================================================
    // Tests for stmt_always_returns
    // ==========================================================================

    #[test]
    fn test_stmt_always_returns_return_stmt() {
        let stmt = HirStmt::Return(None);
        assert!(stmt_always_returns(&stmt));

        let stmt_with_value = HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))));
        assert!(stmt_always_returns(&stmt_with_value));
    }

    #[test]
    fn test_stmt_always_returns_raise_stmt() {
        let stmt = HirStmt::Raise {
            exception: Some(HirExpr::Call {
                func: "ValueError".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            cause: None,
        };
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_expression_does_not() {
        let stmt = HirStmt::Expr(HirExpr::Literal(Literal::Int(42)));
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_assignment_does_not() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        };
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_try_with_all_branches_returning() {
        // try:
        //     return 1
        // except:
        //     return 2
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            handlers: vec![ExceptHandler {
                exception_type: None,
                name: None,
                body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(2))))],
            }],
            orelse: None,
            finalbody: None,
        };
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_try_without_handler_returning() {
        // try:
        //     return 1
        // except:
        //     pass  <-- doesn't return
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            handlers: vec![ExceptHandler {
                exception_type: None,
                name: None,
                body: vec![HirStmt::Pass],
            }],
            orelse: None,
            finalbody: None,
        };
        assert!(!stmt_always_returns(&stmt));
    }

    // ==========================================================================
    // Tests for classify_string_method
    // ==========================================================================

    #[test]
    fn test_classify_string_method_owned_methods() {
        assert_eq!(classify_string_method("upper"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("lower"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("strip"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("lstrip"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("rstrip"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("replace"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("format"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("title"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("capitalize"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("swapcase"), StringMethodReturnType::Owned);
    }

    #[test]
    fn test_classify_string_method_borrowed_methods() {
        assert_eq!(classify_string_method("startswith"), StringMethodReturnType::Borrowed);
        assert_eq!(classify_string_method("endswith"), StringMethodReturnType::Borrowed);
        assert_eq!(classify_string_method("isalpha"), StringMethodReturnType::Borrowed);
        assert_eq!(classify_string_method("isdigit"), StringMethodReturnType::Borrowed);
        assert_eq!(classify_string_method("isalnum"), StringMethodReturnType::Borrowed);
        assert_eq!(classify_string_method("isspace"), StringMethodReturnType::Borrowed);
        assert_eq!(classify_string_method("islower"), StringMethodReturnType::Borrowed);
        assert_eq!(classify_string_method("isupper"), StringMethodReturnType::Borrowed);
        assert_eq!(classify_string_method("find"), StringMethodReturnType::Borrowed);
        assert_eq!(classify_string_method("count"), StringMethodReturnType::Borrowed);
    }

    #[test]
    fn test_classify_string_method_unknown_defaults_to_owned() {
        assert_eq!(classify_string_method("custom_method"), StringMethodReturnType::Owned);
        assert_eq!(classify_string_method("my_transform"), StringMethodReturnType::Owned);
    }

    // ==========================================================================
    // Tests for contains_owned_string_method
    // ==========================================================================

    #[test]
    fn test_contains_owned_string_method_upper() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "upper".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(contains_owned_string_method(&expr));
    }

    #[test]
    fn test_contains_owned_string_method_startswith() {
        // startswith returns bool, so borrowed
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "startswith".to_string(),
            args: vec![HirExpr::Literal(Literal::String("prefix".to_string()))],
            kwargs: vec![],
        };
        assert!(!contains_owned_string_method(&expr));
    }

    #[test]
    fn test_contains_owned_string_method_string_literal() {
        // String literals get .to_string() and are owned
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert!(contains_owned_string_method(&expr));
    }

    #[test]
    fn test_contains_owned_string_method_fstring() {
        // F-strings generate format!() which returns owned
        let expr = HirExpr::FString {
            parts: vec![crate::hir::FStringPart::Literal("hello ".to_string())],
        };
        assert!(contains_owned_string_method(&expr));
    }

    #[test]
    fn test_contains_owned_string_method_int_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!contains_owned_string_method(&expr));
    }

    #[test]
    fn test_contains_owned_string_method_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!contains_owned_string_method(&expr));
    }

    #[test]
    fn test_contains_owned_string_method_in_binary() {
        // Binary with owned method on one side
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("s".to_string())),
                method: "strip".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            op: BinOp::Add,
            right: Box::new(HirExpr::Var("other".to_string())),
        };
        assert!(contains_owned_string_method(&expr));
    }

    // ==========================================================================
    // Tests for contains_string_concatenation
    // ==========================================================================

    #[test]
    fn test_contains_string_concatenation_add() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("a".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        assert!(contains_string_concatenation(&expr));
    }

    #[test]
    fn test_contains_string_concatenation_fstring() {
        let expr = HirExpr::FString {
            parts: vec![crate::hir::FStringPart::Literal("hello".to_string())],
        };
        assert!(contains_string_concatenation(&expr));
    }

    #[test]
    fn test_contains_string_concatenation_sub_does_not() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("a".to_string())),
            op: BinOp::Sub,
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        assert!(!contains_string_concatenation(&expr));
    }

    #[test]
    fn test_contains_string_concatenation_var_does_not() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!contains_string_concatenation(&expr));
    }

    // ==========================================================================
    // Tests for is_negative_int_expr
    // ==========================================================================

    #[test]
    fn test_is_negative_int_expr_negative_literal() {
        let expr = HirExpr::Literal(Literal::Int(-42));
        assert!(is_negative_int_expr(&expr));
    }

    #[test]
    fn test_is_negative_int_expr_positive_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_negative_int_expr(&expr));
    }

    #[test]
    fn test_is_negative_int_expr_zero() {
        let expr = HirExpr::Literal(Literal::Int(0));
        assert!(!is_negative_int_expr(&expr));
    }

    #[test]
    fn test_is_negative_int_expr_unary_neg_positive() {
        // -(1) should be negative
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_negative_int_expr(&expr));
    }

    #[test]
    fn test_is_negative_int_expr_unary_not() {
        // !(x) is not a negative int
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(!is_negative_int_expr(&expr));
    }

    // ==========================================================================
    // Tests for literal_to_type
    // ==========================================================================

    #[test]
    fn test_literal_to_type_int() {
        assert_eq!(literal_to_type(&Literal::Int(42)), Type::Int);
        assert_eq!(literal_to_type(&Literal::Int(-1)), Type::Int);
        assert_eq!(literal_to_type(&Literal::Int(0)), Type::Int);
    }

    #[test]
    fn test_literal_to_type_float() {
        assert_eq!(literal_to_type(&Literal::Float(3.14)), Type::Float);
        assert_eq!(literal_to_type(&Literal::Float(-1.0)), Type::Float);
    }

    #[test]
    fn test_literal_to_type_string() {
        assert_eq!(literal_to_type(&Literal::String("hello".to_string())), Type::String);
        assert_eq!(literal_to_type(&Literal::String("".to_string())), Type::String);
    }

    #[test]
    fn test_literal_to_type_bool() {
        assert_eq!(literal_to_type(&Literal::Bool(true)), Type::Bool);
        assert_eq!(literal_to_type(&Literal::Bool(false)), Type::Bool);
    }

    #[test]
    fn test_literal_to_type_none() {
        assert_eq!(literal_to_type(&Literal::None), Type::None);
    }

    #[test]
    fn test_literal_to_type_bytes() {
        assert_eq!(literal_to_type(&Literal::Bytes(vec![1, 2, 3])), Type::Unknown);
    }

    // ==========================================================================
    // Tests for extract_args_field_accesses
    // ==========================================================================

    #[test]
    fn test_extract_args_field_accesses_simple() {
        // args.name access
        let body = vec![HirStmt::Expr(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "name".to_string(),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert_eq!(fields, vec!["name".to_string()]);
    }

    #[test]
    fn test_extract_args_field_accesses_multiple() {
        let body = vec![
            HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "name".to_string(),
            }),
            HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "verbose".to_string(),
            }),
        ];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"name".to_string()));
        assert!(fields.contains(&"verbose".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_no_args() {
        let body = vec![HirStmt::Expr(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("other".to_string())),
            attr: "name".to_string(),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_args_field_accesses_empty_body() {
        let body: Vec<HirStmt> = vec![];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_args_field_accesses_in_if() {
        // if args.verbose: ...
        let body = vec![HirStmt::If {
            condition: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "verbose".to_string(),
            },
            then_body: vec![],
            else_body: None,
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert_eq!(fields, vec!["verbose".to_string()]);
    }

    // ==========================================================================
    // Tests for return_type_expects_float
    // ==========================================================================

    #[test]
    fn test_return_type_expects_float_float() {
        assert!(return_type_expects_float(&Type::Float));
    }

    #[test]
    fn test_return_type_expects_float_int() {
        assert!(!return_type_expects_float(&Type::Int));
    }

    #[test]
    fn test_return_type_expects_float_optional_float() {
        let ty = Type::Optional(Box::new(Type::Float));
        assert!(return_type_expects_float(&ty));
    }

    #[test]
    fn test_return_type_expects_float_list_of_float() {
        let ty = Type::List(Box::new(Type::Float));
        assert!(return_type_expects_float(&ty));
    }

    #[test]
    fn test_return_type_expects_float_tuple_with_float() {
        let ty = Type::Tuple(vec![Type::Int, Type::Float]);
        assert!(return_type_expects_float(&ty));
    }

    #[test]
    fn test_return_type_expects_float_tuple_without_float() {
        let ty = Type::Tuple(vec![Type::Int, Type::String]);
        assert!(!return_type_expects_float(&ty));
    }

    // ==========================================================================
    // Tests for rewrite_adt_child_type
    // ==========================================================================

    #[test]
    fn test_rewrite_adt_child_type_simple() {
        use std::collections::HashMap;
        let mut child_to_parent = HashMap::new();
        child_to_parent.insert("ListIter".to_string(), "Iter".to_string());

        let ty = Type::Custom("ListIter".to_string());
        let result = rewrite_adt_child_type(&ty, &child_to_parent);
        assert_eq!(result, Type::Custom("Iter".to_string()));
    }

    #[test]
    fn test_rewrite_adt_child_type_with_generic() {
        use std::collections::HashMap;
        let mut child_to_parent = HashMap::new();
        child_to_parent.insert("ListIter".to_string(), "Iter".to_string());

        let ty = Type::Custom("ListIter[T]".to_string());
        let result = rewrite_adt_child_type(&ty, &child_to_parent);
        assert_eq!(result, Type::Custom("Iter[T]".to_string()));
    }

    #[test]
    fn test_rewrite_adt_child_type_no_match() {
        use std::collections::HashMap;
        let child_to_parent: HashMap<String, String> = HashMap::new();

        let ty = Type::Custom("SomeType".to_string());
        let result = rewrite_adt_child_type(&ty, &child_to_parent);
        assert_eq!(result, Type::Custom("SomeType".to_string()));
    }

    #[test]
    fn test_rewrite_adt_child_type_primitive() {
        use std::collections::HashMap;
        let mut child_to_parent = HashMap::new();
        child_to_parent.insert("ListIter".to_string(), "Iter".to_string());

        // Primitives should remain unchanged
        let ty = Type::Int;
        let result = rewrite_adt_child_type(&ty, &child_to_parent);
        assert_eq!(result, Type::Int);
    }

    // ==========================================================================
    // Tests for function_returns_owned_string
    // ==========================================================================

    #[test]
    fn test_function_returns_owned_string_with_upper() {
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("s".to_string())),
                method: "upper".to_string(),
                args: vec![],
                kwargs: vec![],
            }))],
            docstring: None,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
        };
        assert!(function_returns_owned_string(&func));
    }

    #[test]
    fn test_function_returns_owned_string_simple_return() {
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::Var("s".to_string())))],
            docstring: None,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
        };
        // Simple var return is NOT owned string method
        assert!(!function_returns_owned_string(&func));
    }

    #[test]
    fn test_function_returns_owned_string_in_if() {
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::String,
            body: vec![HirStmt::If {
                condition: HirExpr::Var("cond".to_string()),
                then_body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("s".to_string())),
                    method: "strip".to_string(),
                    args: vec![],
                    kwargs: vec![],
                }))],
                else_body: None,
            }],
            docstring: None,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
        };
        assert!(function_returns_owned_string(&func));
    }

    // ==========================================================================
    // Tests for function_returns_string_concatenation
    // ==========================================================================

    #[test]
    fn test_function_returns_string_concatenation_add() {
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                left: Box::new(HirExpr::Var("a".to_string())),
                op: BinOp::Add,
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            docstring: None,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
        };
        assert!(function_returns_string_concatenation(&func));
    }

    #[test]
    fn test_function_returns_string_concatenation_no_return() {
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Binary {
                left: Box::new(HirExpr::Var("a".to_string())),
                op: BinOp::Add,
                right: Box::new(HirExpr::Var("b".to_string())),
            })],
            docstring: None,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
        };
        // Expression statement, not return
        assert!(!function_returns_string_concatenation(&func));
    }

    // ==========================================================================
    // Tests for collect_nested_function_names
    // ==========================================================================

    #[test]
    fn test_collect_nested_function_names_basic() {
        let stmts = vec![HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec![]),
            ret_type: Type::None,
            body: vec![],
            docstring: None,
        }];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert_eq!(names, vec!["inner".to_string()]);
    }

    #[test]
    fn test_collect_nested_function_names_multiple() {
        let stmts = vec![
            HirStmt::FunctionDef {
                name: "first".to_string(),
                params: Box::new(smallvec![]),
                ret_type: Type::None,
                body: vec![],
                docstring: None,
            },
            HirStmt::FunctionDef {
                name: "second".to_string(),
                params: Box::new(smallvec![]),
                ret_type: Type::None,
                body: vec![],
                docstring: None,
            },
        ];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"first".to_string()));
        assert!(names.contains(&"second".to_string()));
    }

    #[test]
    fn test_collect_nested_function_names_empty() {
        let stmts: Vec<HirStmt> = vec![];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert!(names.is_empty());
    }

    #[test]
    fn test_collect_nested_function_names_in_if() {
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::FunctionDef {
                name: "in_if".to_string(),
                params: Box::new(smallvec![]),
                ret_type: Type::None,
                body: vec![],
                docstring: None,
            }],
            else_body: None,
        }];
        let mut names = Vec::new();
        collect_nested_function_names(&stmts, &mut names);
        assert_eq!(names, vec!["in_if".to_string()]);
    }

    // ==========================================================================
    // Tests for collect_if_escaping_variables
    // ==========================================================================

    #[test]
    fn test_collect_if_escaping_variables_basic() {
        // if cond:
        //     x = 1
        // else:
        //     x = 2
        // print(x)  # x escapes the if because it's used after
        let stmts = vec![
            HirStmt::If {
                condition: HirExpr::Var("cond".to_string()),
                then_body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Literal(Literal::Int(1)),
                    type_annotation: None,
                }],
                else_body: Some(vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Literal(Literal::Int(2)),
                    type_annotation: None,
                }]),
            },
            // Statement using x after the if
            HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![HirExpr::Var("x".to_string())],
                kwargs: vec![],
            }),
        ];
        let escaping = collect_if_escaping_variables(&stmts);
        assert!(escaping.contains("x"));
    }

    #[test]
    fn test_collect_if_escaping_variables_empty() {
        let stmts: Vec<HirStmt> = vec![];
        let escaping = collect_if_escaping_variables(&stmts);
        assert!(escaping.is_empty());
    }

    // ==========================================================================
    // Tests for extract_toplevel_assigned_symbols
    // ==========================================================================

    #[test]
    fn test_extract_toplevel_assigned_symbols_simple() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }];
        let assigned = extract_toplevel_assigned_symbols(&stmts);
        assert!(assigned.contains("x"));
    }

    #[test]
    fn test_extract_toplevel_assigned_symbols_multiple() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Literal(Literal::Int(2)),
                type_annotation: None,
            },
        ];
        let assigned = extract_toplevel_assigned_symbols(&stmts);
        assert!(assigned.contains("x"));
        assert!(assigned.contains("y"));
    }

    #[test]
    fn test_extract_toplevel_assigned_symbols_empty() {
        let stmts: Vec<HirStmt> = vec![];
        let assigned = extract_toplevel_assigned_symbols(&stmts);
        assert!(assigned.is_empty());
    }

    // ==========================================================================
    // Tests for collect_all_assigned_variables
    // ==========================================================================

    #[test]
    fn test_collect_all_assigned_variables_basic() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }];
        let assigned = collect_all_assigned_variables(&stmts);
        assert!(assigned.contains("x"));
    }

    #[test]
    fn test_collect_all_assigned_variables_in_nested() {
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("inner".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            }],
            else_body: None,
        }];
        let assigned = collect_all_assigned_variables(&stmts);
        assert!(assigned.contains("inner"));
    }

    #[test]
    fn test_collect_all_assigned_variables_tuple() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Symbol("b".to_string()),
            ]),
            value: HirExpr::Tuple(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ]),
            type_annotation: None,
        }];
        let assigned = collect_all_assigned_variables(&stmts);
        assert!(assigned.contains("a"));
        assert!(assigned.contains("b"));
    }

    // ==========================================================================
    // Tests for is_var_used_in_expr_any
    // ==========================================================================

    #[test]
    fn test_is_var_used_in_expr_any_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(is_var_used_in_expr_any("x", &expr));
        assert!(!is_var_used_in_expr_any("y", &expr));
    }

    #[test]
    fn test_is_var_used_in_expr_any_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_var_used_in_expr_any("x", &expr));
    }

    #[test]
    fn test_is_var_used_in_expr_any_call() {
        let expr = HirExpr::Call {
            func: "func".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_var_used_in_expr_any("x", &expr));
    }

    #[test]
    fn test_is_var_used_in_expr_any_list() {
        let expr = HirExpr::List(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Literal(Literal::Int(1)),
        ]);
        assert!(is_var_used_in_expr_any("x", &expr));
    }

    // ==========================================================================
    // Tests for is_var_used_in_target
    // ==========================================================================

    #[test]
    fn test_is_var_used_in_target_symbol() {
        let target = AssignTarget::Symbol("x".to_string());
        assert!(!is_var_used_in_target("x", &target));
    }

    #[test]
    fn test_is_var_used_in_target_index() {
        let target = AssignTarget::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("i".to_string())),
        };
        assert!(is_var_used_in_target("arr", &target));
        assert!(is_var_used_in_target("i", &target));
    }

    // ==========================================================================
    // Tests for is_var_used_anywhere
    // ==========================================================================

    #[test]
    fn test_is_var_used_anywhere_expr_stmt() {
        let stmt = HirStmt::Expr(HirExpr::Var("x".to_string()));
        assert!(is_var_used_anywhere("x", &stmt));
    }

    #[test]
    fn test_is_var_used_anywhere_return() {
        let stmt = HirStmt::Return(Some(HirExpr::Var("x".to_string())));
        assert!(is_var_used_anywhere("x", &stmt));
    }

    // ==========================================================================
    // Tests for is_negative_int_expr
    // ==========================================================================

    #[test]
    fn test_is_negative_int_expr_positive_lit() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_negative_int_expr(&expr));
    }

    // ==========================================================================
    // Tests for return_type_expects_float
    // ==========================================================================

    #[test]
    fn test_return_type_expects_float_yes() {
        assert!(return_type_expects_float(&Type::Float));
    }

    #[test]
    fn test_return_type_expects_float_no() {
        assert!(!return_type_expects_float(&Type::Int));
    }

    #[test]
    fn test_return_type_expects_float_optional() {
        let ty = Type::Optional(Box::new(Type::Float));
        assert!(return_type_expects_float(&ty));
    }

    // ==========================================================================
    // Tests for classify_string_method
    // ==========================================================================

    #[test]
    fn test_classify_string_method_owned_upper() {
        assert_eq!(classify_string_method("upper"), StringMethodReturnType::Owned);
    }

    #[test]
    fn test_classify_string_method_owned_lower() {
        assert_eq!(classify_string_method("lower"), StringMethodReturnType::Owned);
    }

    #[test]
    fn test_classify_string_method_borrowed() {
        assert_eq!(classify_string_method("find"), StringMethodReturnType::Borrowed);
    }

    // ==========================================================================
    // Tests for contains_owned_string_method
    // ==========================================================================

    #[test]
    fn test_contains_owned_string_method_yes() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "upper".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(contains_owned_string_method(&expr));
    }

    #[test]
    fn test_contains_owned_string_method_no() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "find".to_string(),
            args: vec![HirExpr::Literal(Literal::String("x".to_string()))],
            kwargs: vec![],
        };
        assert!(!contains_owned_string_method(&expr));
    }

    // ==========================================================================
    // Tests for contains_string_concatenation
    // ==========================================================================

    #[test]
    fn test_contains_string_concatenation_yes() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::String("a".to_string()))),
            right: Box::new(HirExpr::Literal(Literal::String("b".to_string()))),
        };
        assert!(contains_string_concatenation(&expr));
    }

    #[test]
    fn test_contains_string_concatenation_int_add_treated_as_concat() {
        // Note: The function conservatively treats ANY Add as potential string concat
        // This is by design - numeric Add is handled differently in codegen
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        // Function returns true for ALL Add ops (conservative approach)
        assert!(contains_string_concatenation(&expr));
    }

    // ==========================================================================
    // Tests for find_var_type_in_body
    // ==========================================================================

    #[test]
    fn test_find_var_type_found() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        assert_eq!(find_var_type_in_body("x", &stmts), Some(Type::Int));
    }

    #[test]
    fn test_find_var_type_not_found() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("y".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        assert_eq!(find_var_type_in_body("x", &stmts), None);
    }

    // ==========================================================================
    // Tests for collect_loop_escaping_variables
    // ==========================================================================

    #[test]
    fn test_collect_loop_escaping_empty() {
        let stmts: Vec<HirStmt> = vec![];
        let escaping = collect_loop_escaping_variables(&stmts);
        assert!(escaping.is_empty());
    }

    // ==========================================================================
    // Tests for is_param_used_in_body
    // ==========================================================================

    #[test]
    fn test_param_used_in_body_yes() {
        let body = vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))];
        assert!(is_param_used_in_body("x", &body));
    }

    #[test]
    fn test_param_used_in_body_no() {
        let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
        assert!(!is_param_used_in_body("x", &body));
    }

    // ==========================================================================
    // Tests for is_param_used_in_expr
    // ==========================================================================

    #[test]
    fn test_param_used_in_expr_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_param_used_in_expr("x", &expr));
    }

    // ==========================================================================
    // Tests for infer_expr_type_simple
    // ==========================================================================

    #[test]
    fn test_infer_simple_int() {
        assert_eq!(infer_expr_type_simple(&HirExpr::Literal(Literal::Int(42))), Type::Int);
    }

    #[test]
    fn test_infer_simple_float() {
        assert_eq!(infer_expr_type_simple(&HirExpr::Literal(Literal::Float(3.14))), Type::Float);
    }

    #[test]
    fn test_infer_simple_string() {
        assert_eq!(infer_expr_type_simple(&HirExpr::Literal(Literal::String("hi".to_string()))), Type::String);
    }

    #[test]
    fn test_infer_simple_bool() {
        assert_eq!(infer_expr_type_simple(&HirExpr::Literal(Literal::Bool(true))), Type::Bool);
    }

    #[test]
    fn test_infer_simple_list() {
        let expr = HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))]);
        assert!(matches!(infer_expr_type_simple(&expr), Type::List(_)));
    }

    #[test]
    fn test_infer_simple_dict() {
        let expr = HirExpr::Dict(vec![
            (HirExpr::Literal(Literal::String("k".to_string())), HirExpr::Literal(Literal::Int(1)))
        ]);
        assert!(matches!(infer_expr_type_simple(&expr), Type::Dict(_, _)));
    }

    // ==========================================================================
    // Tests for stmt_always_returns - additional edge cases
    // ==========================================================================

    #[test]
    fn test_stmt_always_returns_if_both_branches_return() {
        let stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(2))))]),
        };
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_if_no_else_branch() {
        let stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: None,
        };
        // Without else, the if doesn't guarantee a return
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_if_else_doesnt_return() {
        let stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: Some(vec![HirStmt::Pass]),
        };
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_with_block_returns() {
        let stmt = HirStmt::With {
            context: HirExpr::Var("file".to_string()),
            target: None,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            is_async: false,
        };
        assert!(stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_with_block_no_return() {
        let stmt = HirStmt::With {
            context: HirExpr::Var("file".to_string()),
            target: None,
            body: vec![HirStmt::Pass],
            is_async: false,
        };
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_while_does_not() {
        let stmt = HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
        };
        // Loops don't guarantee return
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_for_does_not() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
        };
        // Loops don't guarantee return
        assert!(!stmt_always_returns(&stmt));
    }

    #[test]
    fn test_stmt_always_returns_pass() {
        assert!(!stmt_always_returns(&HirStmt::Pass));
    }

    #[test]
    fn test_stmt_always_returns_break() {
        assert!(!stmt_always_returns(&HirStmt::Break { label: None }));
    }

    #[test]
    fn test_stmt_always_returns_continue() {
        assert!(!stmt_always_returns(&HirStmt::Continue { label: None }));
    }

    // ==========================================================================
    // Tests for collect_loop_escaping_variables - additional cases
    // ==========================================================================

    #[test]
    fn test_collect_loop_escaping_with_for_loop() {
        // for i in range(10):
        //     x = i
        // print(x)  <-- x escapes loop
        let stmts = vec![
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Var("range".to_string()),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Var("i".to_string()),
                    type_annotation: None,
                }],
            },
            HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![HirExpr::Var("x".to_string())],
                kwargs: vec![],
            }),
        ];
        let escaping = collect_loop_escaping_variables(&stmts);
        assert!(escaping.contains("x"));
    }

    #[test]
    fn test_collect_loop_escaping_var_not_used_after() {
        // for i in range(10):
        //     x = i
        // # x not used after
        let stmts = vec![
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Var("range".to_string()),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Var("i".to_string()),
                    type_annotation: None,
                }],
            },
        ];
        let escaping = collect_loop_escaping_variables(&stmts);
        // x is not used after the loop, so it doesn't escape
        assert!(!escaping.contains("x"));
    }

    #[test]
    fn test_collect_loop_escaping_in_while() {
        // while cond:
        //     result = compute()
        // return result  <-- result escapes loop
        let stmts = vec![
            HirStmt::While {
                condition: HirExpr::Var("cond".to_string()),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("result".to_string()),
                    value: HirExpr::Call {
                        func: "compute".to_string(),
                        args: vec![],
                        kwargs: vec![],
                    },
                    type_annotation: None,
                }],
            },
            HirStmt::Return(Some(HirExpr::Var("result".to_string()))),
        ];
        let escaping = collect_loop_escaping_variables(&stmts);
        assert!(escaping.contains("result"));
    }

    #[test]
    fn test_collect_loop_escaping_multiple_vars() {
        // for i in range(10):
        //     x = i
        //     y = i * 2
        // print(x, y)  <-- both x and y escape loop
        let stmts = vec![
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Var("range".to_string()),
                body: vec![
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("x".to_string()),
                        value: HirExpr::Var("i".to_string()),
                        type_annotation: None,
                    },
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("y".to_string()),
                        value: HirExpr::Binary {
                            left: Box::new(HirExpr::Var("i".to_string())),
                            op: BinOp::Mul,
                            right: Box::new(HirExpr::Literal(Literal::Int(2))),
                        },
                        type_annotation: None,
                    },
                ],
            },
            HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![HirExpr::Var("x".to_string()), HirExpr::Var("y".to_string())],
                kwargs: vec![],
            }),
        ];
        let escaping = collect_loop_escaping_variables(&stmts);
        assert!(escaping.contains("x"));
        assert!(escaping.contains("y"));
    }

    // ==========================================================================
    // Tests for is_param_used_in_stmt - additional edge cases
    // ==========================================================================

    #[test]
    fn test_param_used_in_stmt_if_condition() {
        let stmt = HirStmt::If {
            condition: HirExpr::Var("x".to_string()),
            then_body: vec![],
            else_body: None,
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_if_then_body() {
        let stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            else_body: None,
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_if_else_body() {
        let stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))]),
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_while_condition() {
        let stmt = HirStmt::While {
            condition: HirExpr::Var("x".to_string()),
            body: vec![],
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_while_body() {
        let stmt = HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_for_iter() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![],
        };
        assert!(is_param_used_in_stmt("items", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_for_body() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("other".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_return() {
        let stmt = HirStmt::Return(Some(HirExpr::Var("x".to_string())));
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_return_none() {
        let stmt = HirStmt::Return(None);
        assert!(!is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_try_body() {
        let stmt = HirStmt::Try {
            body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
            handlers: vec![],
            orelse: None,
            finalbody: None,
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_try_handler() {
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![ExceptHandler {
                exception_type: None,
                name: None,
                body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
            }],
            orelse: None,
            finalbody: None,
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_try_finally() {
        let stmt = HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Expr(HirExpr::Var("x".to_string()))]),
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_with_context() {
        let stmt = HirStmt::With {
            context: HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Var("path".to_string())],
                kwargs: vec![],
            },
            target: None,
            body: vec![],
            is_async: false,
        };
        assert!(is_param_used_in_stmt("path", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_with_body() {
        let stmt = HirStmt::With {
            context: HirExpr::Var("ctx".to_string()),
            target: None,
            body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
            is_async: false,
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_nested_function() {
        let stmt = HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec![]),
            body: vec![HirStmt::Return(Some(HirExpr::Var("captured".to_string())))],
            ret_type: Type::Unknown,
            docstring: None,
        };
        assert!(is_param_used_in_stmt("captured", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_assert() {
        let stmt = HirStmt::Assert {
            test: HirExpr::Var("x".to_string()),
            msg: None,
        };
        assert!(is_param_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_param_used_in_stmt_assert_msg() {
        let stmt = HirStmt::Assert {
            test: HirExpr::Literal(Literal::Bool(true)),
            msg: Some(HirExpr::Var("msg".to_string())),
        };
        assert!(is_param_used_in_stmt("msg", &stmt));
    }

    // ==========================================================================
    // Tests for is_param_used_in_expr - additional edge cases
    // ==========================================================================

    #[test]
    fn test_param_used_in_expr_binary_left() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_binary_right() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Add,
            right: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_unary() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_call_func() {
        // callback()
        let expr = HirExpr::Call {
            func: "callback".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_param_used_in_expr("callback", &expr));
    }

    #[test]
    fn test_param_used_in_expr_call_args() {
        let expr = HirExpr::Call {
            func: "func".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_call_kwargs() {
        let expr = HirExpr::Call {
            func: "func".to_string(),
            args: vec![],
            kwargs: vec![("key".to_string(), HirExpr::Var("x".to_string()))],
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_method_call_object() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_param_used_in_expr("obj", &expr));
    }

    #[test]
    fn test_param_used_in_expr_method_call_args() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("other".to_string())),
            method: "method".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_method_call_kwargs() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("other".to_string())),
            method: "method".to_string(),
            args: vec![],
            kwargs: vec![("key".to_string(), HirExpr::Var("x".to_string()))],
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_attribute() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        assert!(is_param_used_in_expr("obj", &expr));
    }

    #[test]
    fn test_param_used_in_expr_index_base() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(is_param_used_in_expr("arr", &expr));
    }

    #[test]
    fn test_param_used_in_expr_index_index() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("other".to_string())),
            index: Box::new(HirExpr::Var("idx".to_string())),
        };
        assert!(is_param_used_in_expr("idx", &expr));
    }

    #[test]
    fn test_param_used_in_expr_slice_base() {
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: None,
            stop: None,
            step: None,
        };
        assert!(is_param_used_in_expr("arr", &expr));
    }

    #[test]
    fn test_param_used_in_expr_slice_start() {
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("other".to_string())),
            start: Some(Box::new(HirExpr::Var("x".to_string()))),
            stop: None,
            step: None,
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_slice_stop() {
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("other".to_string())),
            start: None,
            stop: Some(Box::new(HirExpr::Var("x".to_string()))),
            step: None,
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_slice_step() {
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("other".to_string())),
            start: None,
            stop: None,
            step: Some(Box::new(HirExpr::Var("x".to_string()))),
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_borrow() {
        let expr = HirExpr::Borrow {
            expr: Box::new(HirExpr::Var("x".to_string())),
            mutable: false,
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_fstring() {
        let expr = HirExpr::FString {
            parts: vec![crate::hir::FStringPart::Expr(Box::new(HirExpr::Var("x".to_string())))],
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    #[test]
    fn test_param_used_in_expr_yield() {
        let expr = HirExpr::Yield {
            value: Some(Box::new(HirExpr::Var("x".to_string()))),
        };
        assert!(is_param_used_in_expr("x", &expr));
    }

    // ==========================================================================
    // Tests for is_param_used_in_assign_target
    // ==========================================================================

    #[test]
    fn test_param_used_in_assign_target_symbol_same() {
        let target = AssignTarget::Symbol("x".to_string());
        assert!(is_param_used_in_assign_target("x", &target));
    }

    #[test]
    fn test_param_used_in_assign_target_symbol_different() {
        let target = AssignTarget::Symbol("y".to_string());
        assert!(!is_param_used_in_assign_target("x", &target));
    }

    #[test]
    fn test_param_used_in_assign_target_index() {
        let target = AssignTarget::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("idx".to_string())),
        };
        assert!(is_param_used_in_assign_target("idx", &target));
    }

    #[test]
    fn test_param_used_in_assign_target_tuple() {
        let target = AssignTarget::Tuple(vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("x".to_string()),
        ]);
        assert!(is_param_used_in_assign_target("x", &target));
    }

    // ==========================================================================
    // Tests for stmt_returns_owned_string
    // ==========================================================================

    #[test]
    fn test_stmt_returns_owned_string_return_upper() {
        let stmt = HirStmt::Return(Some(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "upper".to_string(),
            args: vec![],
            kwargs: vec![],
        }));
        assert!(stmt_returns_owned_string(&stmt));
    }

    #[test]
    fn test_stmt_returns_owned_string_return_string_literal() {
        let stmt = HirStmt::Return(Some(HirExpr::Literal(Literal::String("hello".to_string()))));
        assert!(stmt_returns_owned_string(&stmt));
    }

    #[test]
    fn test_stmt_returns_owned_string_return_int() {
        let stmt = HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))));
        assert!(!stmt_returns_owned_string(&stmt));
    }

    #[test]
    fn test_stmt_returns_owned_string_if_then_returns_owned() {
        let stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String("hi".to_string()))))],
            else_body: None,
        };
        assert!(stmt_returns_owned_string(&stmt));
    }

    #[test]
    fn test_stmt_returns_owned_string_expression_doesnt_return() {
        let stmt = HirStmt::Expr(HirExpr::Literal(Literal::String("hi".to_string())));
        assert!(!stmt_returns_owned_string(&stmt));
    }

    // ==========================================================================
    // Tests for stmt_block_returns_owned_string
    // ==========================================================================

    #[test]
    fn test_stmt_block_returns_owned_string_yes() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            HirStmt::Return(Some(HirExpr::Literal(Literal::String("result".to_string())))),
        ];
        assert!(stmt_block_returns_owned_string(&stmts));
    }

    #[test]
    fn test_stmt_block_returns_owned_string_no() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42)))),
        ];
        assert!(!stmt_block_returns_owned_string(&stmts));
    }

    #[test]
    fn test_stmt_block_returns_owned_string_empty() {
        let stmts: Vec<HirStmt> = vec![];
        assert!(!stmt_block_returns_owned_string(&stmts));
    }

    // ==========================================================================
    // Tests for infer_return_type_from_body
    // ==========================================================================

    #[test]
    fn test_infer_return_type_int() {
        let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
        assert_eq!(infer_return_type_from_body(&body), Some(Type::Int));
    }

    #[test]
    fn test_infer_return_type_string() {
        let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String("hi".to_string()))))];
        assert_eq!(infer_return_type_from_body(&body), Some(Type::String));
    }

    #[test]
    fn test_infer_return_type_no_return() {
        let body = vec![HirStmt::Pass];
        assert_eq!(infer_return_type_from_body(&body), None);
    }

    #[test]
    fn test_infer_return_type_return_none_value() {
        let body = vec![HirStmt::Return(None)];
        // Return None statement returns Some(Type::None)
        assert_eq!(infer_return_type_from_body(&body), Some(Type::None));
    }

    #[test]
    fn test_infer_return_type_in_if() {
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Float(3.14))))],
            else_body: None,
        }];
        assert_eq!(infer_return_type_from_body(&body), Some(Type::Float));
    }

    // ==========================================================================
    // Tests for is_field_used_as_bool_condition
    // ==========================================================================

    #[test]
    fn test_field_used_as_bool_condition_if() {
        // if args.verbose:
        let body = vec![HirStmt::If {
            condition: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "verbose".to_string(),
            },
            then_body: vec![],
            else_body: None,
        }];
        assert!(is_field_used_as_bool_condition("verbose", &body));
    }

    #[test]
    fn test_field_used_as_bool_condition_while() {
        // while args.running:
        let body = vec![HirStmt::While {
            condition: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "running".to_string(),
            },
            body: vec![],
        }];
        assert!(is_field_used_as_bool_condition("running", &body));
    }

    #[test]
    fn test_field_used_as_bool_condition_not_used() {
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![],
            else_body: None,
        }];
        assert!(!is_field_used_as_bool_condition("verbose", &body));
    }

    #[test]
    fn test_field_used_as_bool_condition_nested() {
        // if cond:
        //     if args.debug:
        //         pass
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::If {
                condition: HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "debug".to_string(),
                },
                then_body: vec![],
                else_body: None,
            }],
            else_body: None,
        }];
        assert!(is_field_used_as_bool_condition("debug", &body));
    }

    // ==========================================================================
    // Tests for infer_expr_type_simple - additional edge cases
    // ==========================================================================

    #[test]
    fn test_infer_simple_none() {
        assert_eq!(infer_expr_type_simple(&HirExpr::Literal(Literal::None)), Type::None);
    }

    #[test]
    fn test_infer_simple_tuple() {
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("a".to_string())),
        ]);
        assert!(matches!(infer_expr_type_simple(&expr), Type::Tuple(_)));
    }

    #[test]
    fn test_infer_simple_set() {
        let expr = HirExpr::Set(vec![HirExpr::Literal(Literal::Int(1))]);
        assert!(matches!(infer_expr_type_simple(&expr), Type::Set(_)));
    }

    #[test]
    fn test_infer_simple_empty_list() {
        let expr = HirExpr::List(vec![]);
        assert!(matches!(infer_expr_type_simple(&expr), Type::List(_)));
    }

    #[test]
    fn test_infer_simple_empty_dict() {
        let expr = HirExpr::Dict(vec![]);
        assert!(matches!(infer_expr_type_simple(&expr), Type::Dict(_, _)));
    }

    #[test]
    fn test_infer_simple_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(matches!(infer_expr_type_simple(&expr), Type::Unknown));
    }

    #[test]
    fn test_infer_simple_binary_add() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(matches!(infer_expr_type_simple(&expr), Type::Int));
    }

    #[test]
    fn test_infer_simple_binary_comparison() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Eq,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(matches!(infer_expr_type_simple(&expr), Type::Bool));
    }

    #[test]
    fn test_infer_simple_unary_not() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Literal(Literal::Bool(true))),
        };
        assert!(matches!(infer_expr_type_simple(&expr), Type::Bool));
    }

    // ==========================================================================
    // DEPYLER-COVERAGE-95: Tests for codegen_generic_params and codegen_where_clause
    // moved to func_gen_helpers module for isolated testability
    // ==========================================================================

    // ==========================================================================
    // Tests for collect_all_assigned_variables - additional
    // ==========================================================================

    #[test]
    fn test_collect_all_assigned_variables_in_if() {
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            }],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Literal(Literal::Int(2)),
                type_annotation: None,
            }]),
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
    }

    #[test]
    fn test_collect_all_assigned_variables_in_for() {
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Var("i".to_string()),
                type_annotation: None,
            }],
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("x"));
    }

    #[test]
    fn test_collect_all_assigned_variables_in_while() {
        let stmts = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("count".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            }],
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("count"));
    }

    #[test]
    fn test_collect_all_assigned_variables_in_try_finally() {
        // With isn't handled by collect_all_assigned_variables, but finally is
        let stmts = vec![HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("data".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            }]),
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("data"));
    }

    #[test]
    fn test_collect_all_assigned_variables_in_try() {
        let stmts = vec![HirStmt::Try {
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("result".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            }],
            handlers: vec![ExceptHandler {
                exception_type: None,
                name: None,
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("error".to_string()),
                    value: HirExpr::Literal(Literal::Int(1)),
                    type_annotation: None,
                }],
            }],
            orelse: None,
            finalbody: None,
        }];
        let vars = collect_all_assigned_variables(&stmts);
        assert!(vars.contains("result"));
        assert!(vars.contains("error"));
    }

    // ==========================================================================
    // DEPYLER-COVERAGE-95: Tests for codegen_function_attrs moved to
    // func_gen_helpers module for isolated testability
    // ==========================================================================

    // ==========================================================================
    // EXTREME TDD: Tests for find_var_type_in_body_with_params
    // Covers lines 587-700+ of func_gen.rs
    // ==========================================================================

    #[test]
    fn test_find_var_type_in_body_explicit_annotation() {
        use std::collections::HashMap;
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        let params: HashMap<String, Type> = HashMap::new();
        let result = find_var_type_in_body_with_params("x", &stmts, &params);
        assert_eq!(result, Some(Type::Int));
    }

    #[test]
    fn test_find_var_type_in_body_from_param() {
        use std::collections::HashMap;
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("result".to_string()),
            value: HirExpr::Var("n".to_string()),
            type_annotation: None,
        }];
        let mut params: HashMap<String, Type> = HashMap::new();
        params.insert("n".to_string(), Type::Int);
        let result = find_var_type_in_body_with_params("result", &stmts, &params);
        assert_eq!(result, Some(Type::Int));
    }

    #[test]
    fn test_find_var_type_in_body_from_list_index() {
        use std::collections::HashMap;
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("first".to_string()),
            value: HirExpr::Index {
                base: Box::new(HirExpr::Var("items".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            type_annotation: None,
        }];
        let mut params: HashMap<String, Type> = HashMap::new();
        params.insert("items".to_string(), Type::List(Box::new(Type::String)));
        let result = find_var_type_in_body_with_params("first", &stmts, &params);
        assert_eq!(result, Some(Type::String));
    }

    #[test]
    fn test_find_var_type_in_body_not_found() {
        use std::collections::HashMap;
        let stmts = vec![HirStmt::Pass];
        let params: HashMap<String, Type> = HashMap::new();
        let result = find_var_type_in_body_with_params("nonexistent", &stmts, &params);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_var_type_in_body_wrapper() {
        // Test the find_var_type_in_body wrapper function
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Float),
        }];
        let result = find_var_type_in_body("x", &stmts);
        assert_eq!(result, Some(Type::Float));
    }

    #[test]
    fn test_find_var_type_in_body_from_dict_index() {
        use std::collections::HashMap;
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("value".to_string()),
            value: HirExpr::Index {
                base: Box::new(HirExpr::Var("data".to_string())),
                index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
            },
            type_annotation: None,
        }];
        let mut params: HashMap<String, Type> = HashMap::new();
        params.insert("data".to_string(), Type::Dict(Box::new(Type::String), Box::new(Type::Int)));
        let result = find_var_type_in_body_with_params("value", &stmts, &params);
        assert_eq!(result, Some(Type::Int));
    }

    #[test]
    fn test_find_var_type_in_body_from_string_index() {
        use std::collections::HashMap;
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("char".to_string()),
            value: HirExpr::Index {
                base: Box::new(HirExpr::Var("text".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            type_annotation: None,
        }];
        let mut params: HashMap<String, Type> = HashMap::new();
        params.insert("text".to_string(), Type::String);
        let result = find_var_type_in_body_with_params("char", &stmts, &params);
        assert_eq!(result, Some(Type::String));
    }

    #[test]
    fn test_find_var_type_in_body_from_tuple_index() {
        use std::collections::HashMap;
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("first".to_string()),
            value: HirExpr::Index {
                base: Box::new(HirExpr::Var("pair".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            type_annotation: None,
        }];
        let mut params: HashMap<String, Type> = HashMap::new();
        params.insert("pair".to_string(), Type::Tuple(vec![Type::Int, Type::String]));
        let result = find_var_type_in_body_with_params("first", &stmts, &params);
        assert_eq!(result, Some(Type::Int));
    }

    // ==========================================================================
    // EXTREME TDD: Tests for extract_args_field_accesses
    // Covers lines 23-200 of func_gen.rs
    // ==========================================================================

    #[test]
    fn test_extract_args_field_accesses_simple_attribute() {
        // args.verbose -> ["verbose"]
        let body = vec![HirStmt::Expr(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "verbose".to_string(),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"verbose".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_multiple_fields() {
        // args.input and args.output
        let body = vec![
            HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "input".to_string(),
            }),
            HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "output".to_string(),
            }),
        ];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"input".to_string()));
        assert!(fields.contains(&"output".to_string()));
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_extract_args_field_accesses_wrong_name() {
        // other.verbose -> [] (not "args")
        let body = vec![HirStmt::Expr(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("other".to_string())),
            attr: "verbose".to_string(),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_args_field_accesses_in_binary() {
        // args.x + args.y
        let body = vec![HirStmt::Expr(HirExpr::Binary {
            left: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "x".to_string(),
            }),
            op: BinOp::Add,
            right: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "y".to_string(),
            }),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"x".to_string()));
        assert!(fields.contains(&"y".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_unary() {
        // -args.value
        let body = vec![HirStmt::Expr(HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "value".to_string(),
            }),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"value".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_call_args() {
        // print(args.msg)
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "msg".to_string(),
            }],
            kwargs: vec![],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"msg".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_call_kwargs() {
        // func(key=args.value)
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "func".to_string(),
            args: vec![],
            kwargs: vec![("key".to_string(), HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "value".to_string(),
            })],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"value".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_method_call_object() {
        // args.data.process()
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "data".to_string(),
            }),
            method: "process".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"data".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_method_call_args() {
        // obj.method(args.param)
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "param".to_string(),
            }],
            kwargs: vec![],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"param".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_method_call_kwargs() {
        // obj.method(k=args.v)
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![],
            kwargs: vec![("k".to_string(), HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "v".to_string(),
            })],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"v".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_list() {
        // [args.a, args.b]
        let body = vec![HirStmt::Expr(HirExpr::List(vec![
            HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "a".to_string(),
            },
            HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "b".to_string(),
            },
        ]))];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"a".to_string()));
        assert!(fields.contains(&"b".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_tuple() {
        // (args.x, args.y)
        let body = vec![HirStmt::Expr(HirExpr::Tuple(vec![
            HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "x".to_string(),
            },
            HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "y".to_string(),
            },
        ]))];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"x".to_string()));
        assert!(fields.contains(&"y".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_set() {
        // {args.item}
        let body = vec![HirStmt::Expr(HirExpr::Set(vec![
            HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "item".to_string(),
            },
        ]))];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"item".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_dict() {
        // {args.key: args.val}
        let body = vec![HirStmt::Expr(HirExpr::Dict(vec![
            (
                HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "key".to_string(),
                },
                HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "val".to_string(),
                },
            ),
        ]))];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"key".to_string()));
        assert!(fields.contains(&"val".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_index() {
        // args.data[args.idx]
        let body = vec![HirStmt::Expr(HirExpr::Index {
            base: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "data".to_string(),
            }),
            index: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "idx".to_string(),
            }),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"data".to_string()));
        assert!(fields.contains(&"idx".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_if_expr() {
        // args.a if args.cond else args.b
        let body = vec![HirStmt::Expr(HirExpr::IfExpr {
            test: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "cond".to_string(),
            }),
            body: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "a".to_string(),
            }),
            orelse: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "b".to_string(),
            }),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"cond".to_string()));
        assert!(fields.contains(&"a".to_string()));
        assert!(fields.contains(&"b".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_fstring() {
        use crate::hir::FStringPart;
        // f"{args.name}"
        let body = vec![HirStmt::Expr(HirExpr::FString {
            parts: vec![FStringPart::Expr(Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "name".to_string(),
            }))],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"name".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_fstring_literal() {
        use crate::hir::FStringPart;
        // f"literal text"
        let body = vec![HirStmt::Expr(HirExpr::FString {
            parts: vec![FStringPart::Literal("text".to_string())],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_args_field_accesses_in_slice() {
        // args.data[args.start:args.stop:args.step]
        let body = vec![HirStmt::Expr(HirExpr::Slice {
            base: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "data".to_string(),
            }),
            start: Some(Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "start".to_string(),
            })),
            stop: Some(Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "stop".to_string(),
            })),
            step: Some(Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "step".to_string(),
            })),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"data".to_string()));
        assert!(fields.contains(&"start".to_string()));
        assert!(fields.contains(&"stop".to_string()));
        assert!(fields.contains(&"step".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_slice_partial() {
        // data[:args.end]
        let body = vec![HirStmt::Expr(HirExpr::Slice {
            base: Box::new(HirExpr::Var("data".to_string())),
            start: None,
            stop: Some(Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "end".to_string(),
            })),
            step: None,
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"end".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_list_comp() {
        use crate::hir::HirComprehension;
        // [x for x in args.items if args.filter]
        let body = vec![HirStmt::Expr(HirExpr::ListComp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "items".to_string(),
                }),
                conditions: vec![HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "filter".to_string(),
                }],
            }],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"items".to_string()));
        assert!(fields.contains(&"filter".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_set_comp() {
        use crate::hir::HirComprehension;
        // {args.x for _ in items}
        let body = vec![HirStmt::Expr(HirExpr::SetComp {
            element: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "x".to_string(),
            }),
            generators: vec![HirComprehension {
                target: "_".to_string(),
                iter: Box::new(HirExpr::Var("items".to_string())),
                conditions: vec![],
            }],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"x".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_dict_comp() {
        use crate::hir::HirComprehension;
        // {args.k: args.v for _ in items if args.cond}
        let body = vec![HirStmt::Expr(HirExpr::DictComp {
            key: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "k".to_string(),
            }),
            value: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "v".to_string(),
            }),
            generators: vec![HirComprehension {
                target: "_".to_string(),
                iter: Box::new(HirExpr::Var("items".to_string())),
                conditions: vec![HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "cond".to_string(),
                }],
            }],
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"k".to_string()));
        assert!(fields.contains(&"v".to_string()));
        assert!(fields.contains(&"cond".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_lambda() {
        // lambda: args.value
        let body = vec![HirStmt::Expr(HirExpr::Lambda {
            params: vec![],
            body: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "value".to_string(),
            }),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"value".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_borrow() {
        // &args.data
        let body = vec![HirStmt::Expr(HirExpr::Borrow {
            expr: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "data".to_string(),
            }),
            mutable: false,
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"data".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_yield() {
        // yield args.item
        let body = vec![HirStmt::Expr(HirExpr::Yield {
            value: Some(Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "item".to_string(),
            })),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"item".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_yield_none() {
        // yield
        let body = vec![HirStmt::Expr(HirExpr::Yield { value: None })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_args_field_accesses_in_await() {
        // await args.future
        let body = vec![HirStmt::Expr(HirExpr::Await {
            value: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "future".to_string(),
            }),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"future".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_assign() {
        // x = args.value
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "value".to_string(),
            },
            type_annotation: None,
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"value".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_return() {
        // return args.result
        let body = vec![HirStmt::Return(Some(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "result".to_string(),
        }))];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"result".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_if_stmt() {
        // if args.cond: pass
        let body = vec![HirStmt::If {
            condition: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "cond".to_string(),
            },
            then_body: vec![HirStmt::Pass],
            else_body: None,
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"cond".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_if_else_stmt() {
        // if cond: x = args.a else: y = args.b
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "a".to_string(),
                },
                type_annotation: None,
            }],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "b".to_string(),
                },
                type_annotation: None,
            }]),
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"a".to_string()));
        assert!(fields.contains(&"b".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_while() {
        // while args.running: pass
        let body = vec![HirStmt::While {
            condition: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "running".to_string(),
            },
            body: vec![HirStmt::Pass],
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"running".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_for() {
        // for x in args.items: pass
        let body = vec![HirStmt::For {
            target: AssignTarget::Symbol("x".to_string()),
            iter: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "items".to_string(),
            },
            body: vec![HirStmt::Pass],
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"items".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_with() {
        // with args.resource as r: pass
        let body = vec![HirStmt::With {
            context: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "resource".to_string(),
            },
            target: Some("r".to_string()),
            body: vec![HirStmt::Pass],
            is_async: false,
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"resource".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_try() {
        // try: x = args.a except: y = args.b
        let body = vec![HirStmt::Try {
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "a".to_string(),
                },
                type_annotation: None,
            }],
            handlers: vec![ExceptHandler {
                exception_type: None,
                name: None,
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("y".to_string()),
                    value: HirExpr::Attribute {
                        value: Box::new(HirExpr::Var("args".to_string())),
                        attr: "b".to_string(),
                    },
                    type_annotation: None,
                }],
            }],
            orelse: None,
            finalbody: None,
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"a".to_string()));
        assert!(fields.contains(&"b".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_try_orelse() {
        // try: pass else: x = args.value
        let body = vec![HirStmt::Try {
            body: vec![HirStmt::Pass],
            handlers: vec![],
            orelse: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "value".to_string(),
                },
                type_annotation: None,
            }]),
            finalbody: None,
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"value".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_try_finally() {
        // try: pass finally: x = args.cleanup
        let body = vec![HirStmt::Try {
            body: vec![HirStmt::Pass],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "cleanup".to_string(),
                },
                type_annotation: None,
            }]),
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"cleanup".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_in_nested_function() {
        // def inner(): return args.inner_val
        let body = vec![HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec![]),
            ret_type: Type::None,
            body: vec![HirStmt::Return(Some(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "inner_val".to_string(),
            }))],
            docstring: None,
        }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"inner_val".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_pass_stmt() {
        let body = vec![HirStmt::Pass];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_args_field_accesses_break_stmt() {
        let body = vec![HirStmt::Break { label: None }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_args_field_accesses_continue_stmt() {
        let body = vec![HirStmt::Continue { label: None }];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_args_field_accesses_nested_attribute() {
        // args.config.value - should find "config", nested walks find nothing extra
        let body = vec![HirStmt::Expr(HirExpr::Attribute {
            value: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "config".to_string(),
            }),
            attr: "value".to_string(),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.contains(&"config".to_string()));
        // "value" is NOT directly on args
        assert!(!fields.contains(&"value".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_deduplicates() {
        // args.x + args.x - should only have one "x"
        let body = vec![HirStmt::Expr(HirExpr::Binary {
            left: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "x".to_string(),
            }),
            op: BinOp::Add,
            right: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "x".to_string(),
            }),
        })];
        let fields = extract_args_field_accesses(&body, "args");
        assert_eq!(fields.len(), 1);
        assert!(fields.contains(&"x".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_custom_args_name() {
        // params.value (using "params" instead of "args")
        let body = vec![HirStmt::Expr(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("params".to_string())),
            attr: "value".to_string(),
        })];
        let fields = extract_args_field_accesses(&body, "params");
        assert!(fields.contains(&"value".to_string()));
    }

    #[test]
    fn test_extract_args_field_accesses_other_expr_types() {
        // Just a literal, no args access
        let body = vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(42)))];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_args_field_accesses_var_not_attribute() {
        // Just "args" variable, not args.X
        let body = vec![HirStmt::Expr(HirExpr::Var("args".to_string()))];
        let fields = extract_args_field_accesses(&body, "args");
        assert!(fields.is_empty());
    }
}
