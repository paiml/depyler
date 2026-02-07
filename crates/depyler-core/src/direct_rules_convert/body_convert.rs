//! Body and assignment conversion for direct rules
//!
//! Contains body-level conversion functions: convert_body, find_mutable_vars,
//! assignment conversion, and expression purity checks.

use crate::direct_rules::{extract_nested_indices, make_ident};
use crate::hir::*;
use crate::type_mapper::TypeMapper;
use anyhow::{bail, Result};
use quote::quote;
use syn::parse_quote;

use super::{convert_expr, convert_stmt_with_mutable_vars};

pub(crate) fn convert_body(stmts: &[HirStmt], type_mapper: &TypeMapper) -> Result<Vec<syn::Stmt>> {
    // Use empty vararg_functions and param_types for backward compatibility
    static EMPTY_SET: std::sync::OnceLock<std::collections::HashSet<String>> =
        std::sync::OnceLock::new();
    static EMPTY_MAP: std::sync::OnceLock<std::collections::HashMap<String, Type>> =
        std::sync::OnceLock::new();
    convert_body_with_context(
        stmts,
        type_mapper,
        false,
        EMPTY_SET.get_or_init(std::collections::HashSet::new),
        EMPTY_MAP.get_or_init(std::collections::HashMap::new),
    )
}

/// DEPYLER-0713: Analyze which variables need to be mutable
/// Variables are mutable if they are reassigned or have mutating method calls
pub(crate) fn find_mutable_vars_in_body(stmts: &[HirStmt]) -> std::collections::HashSet<String> {
    let mut declared: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut mutable: std::collections::HashSet<String> = std::collections::HashSet::new();

    fn analyze_stmt(
        stmt: &HirStmt,
        declared: &mut std::collections::HashSet<String>,
        mutable: &mut std::collections::HashSet<String>,
    ) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                // Check for mutating method calls in value
                check_mutating_methods(value, mutable);

                match target {
                    AssignTarget::Symbol(name) => {
                        if declared.contains(name) {
                            // Reassignment - mark as mutable
                            mutable.insert(name.clone());
                        } else {
                            declared.insert(name.clone());
                        }
                    }
                    AssignTarget::Attribute { value: base, .. }
                    | AssignTarget::Index { base, .. } => {
                        // Attribute/index assignment requires base to be mutable
                        if let HirExpr::Var(var_name) = base.as_ref() {
                            mutable.insert(var_name.clone());
                        }
                    }
                    AssignTarget::Tuple(targets) => {
                        for t in targets {
                            if let AssignTarget::Symbol(name) = t {
                                if declared.contains(name) {
                                    mutable.insert(name.clone());
                                } else {
                                    declared.insert(name.clone());
                                }
                            }
                        }
                    }
                }
            }
            HirStmt::Expr(expr) => {
                check_mutating_methods(expr, mutable);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                check_mutating_methods(condition, mutable);
                for s in then_body {
                    analyze_stmt(s, declared, mutable);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        analyze_stmt(s, declared, mutable);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                check_mutating_methods(condition, mutable);
                for s in body {
                    analyze_stmt(s, declared, mutable);
                }
            }
            HirStmt::For { body, .. } => {
                for s in body {
                    analyze_stmt(s, declared, mutable);
                }
            }
            HirStmt::With { body, .. } => {
                for s in body {
                    analyze_stmt(s, declared, mutable);
                }
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
                ..
            } => {
                for s in body {
                    analyze_stmt(s, declared, mutable);
                }
                for h in handlers {
                    for s in &h.body {
                        analyze_stmt(s, declared, mutable);
                    }
                }
                if let Some(else_stmts) = orelse {
                    for s in else_stmts {
                        analyze_stmt(s, declared, mutable);
                    }
                }
                if let Some(final_stmts) = finalbody {
                    for s in final_stmts {
                        analyze_stmt(s, declared, mutable);
                    }
                }
            }
            _ => {}
        }
    }

    fn check_mutating_methods(expr: &HirExpr, mutable: &mut std::collections::HashSet<String>) {
        match expr {
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => {
                // Check if this is a mutating method
                // DEPYLER-1002: Added hexdigest/digest for hashlib - finalize_reset() requires &mut self
                let is_mutating = matches!(
                    method.as_str(),
                    "append"
                        | "extend"
                        | "insert"
                        | "remove"
                        | "pop"
                        | "clear"
                        | "reverse"
                        | "sort"
                        | "update"
                        | "setdefault"
                        | "popitem"
                        | "add"
                        | "discard"
                        | "write"
                        | "write_all"
                        | "writelines"
                        | "flush"
                        | "hexdigest"
                        | "digest"
                        | "finalize"
                        | "finalize_reset"
                );
                if is_mutating {
                    if let HirExpr::Var(var_name) = object.as_ref() {
                        mutable.insert(var_name.clone());
                    }
                }
                check_mutating_methods(object, mutable);
                for arg in args {
                    check_mutating_methods(arg, mutable);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                check_mutating_methods(left, mutable);
                check_mutating_methods(right, mutable);
            }
            HirExpr::Unary { operand, .. } => {
                check_mutating_methods(operand, mutable);
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    check_mutating_methods(arg, mutable);
                }
            }
            HirExpr::List(items) | HirExpr::Tuple(items) | HirExpr::Set(items) => {
                for item in items {
                    check_mutating_methods(item, mutable);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    check_mutating_methods(k, mutable);
                    check_mutating_methods(v, mutable);
                }
            }
            _ => {}
        }
    }

    for stmt in stmts {
        analyze_stmt(stmt, &mut declared, &mut mutable);
    }

    mutable
}

/// DEPYLER-0704: Added param_types parameter for type coercion in binary operations
/// DEPYLER-0713: Added mutable_vars analysis for proper mutability
pub(crate) fn convert_body_with_context(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
) -> Result<Vec<syn::Stmt>> {
    // DEPYLER-0713: Pre-analyze which variables need to be mutable
    let mutable_vars = find_mutable_vars_in_body(stmts);

    stmts
        .iter()
        .map(|stmt| {
            convert_stmt_with_mutable_vars(
                stmt,
                type_mapper,
                is_classmethod,
                vararg_functions,
                param_types,
                &mutable_vars,
            )
        })
        .collect()
}

/// Convert simple variable assignment: `x = value`
/// DEPYLER-0713: Only add `mut` if variable is in mutable_vars set
///
/// Complexity: 2 (with branching for mutability)
pub(crate) fn convert_symbol_assignment(
    symbol: &str,
    value_expr: syn::Expr,
    mutable_vars: &std::collections::HashSet<String>,
) -> Result<syn::Stmt> {
    let target_ident = make_ident(symbol);
    // DEPYLER-0713: Only add mut if variable is reassigned or has mutating method calls
    let mutability = if mutable_vars.contains(symbol) {
        Some(Default::default())
    } else {
        None
    };
    let stmt = syn::Stmt::Local(syn::Local {
        attrs: vec![],
        let_token: Default::default(),
        pat: syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability,
            ident: target_ident,
            subpat: None,
        }),
        init: Some(syn::LocalInit {
            eq_token: Default::default(),
            expr: Box::new(value_expr),
            diverge: None,
        }),
        semi_token: Default::default(),
    });
    Ok(stmt)
}

/// Convert subscript assignment: `d[k] = value` or `d[k1][k2] = value`
///
/// Handles both simple and nested subscript assignments.
/// Complexity: 3 (if + loop with nested if)
pub(crate) fn convert_index_assignment(
    base: &HirExpr,
    index: &HirExpr,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    let final_index = convert_expr(index, type_mapper)?;
    let (base_expr, indices) = extract_nested_indices(base, type_mapper)?;

    if indices.is_empty() {
        // Simple assignment: d[k] = v
        let assign_expr = parse_quote! {
            #base_expr.insert(#final_index, #value_expr)
        };
        Ok(syn::Stmt::Expr(assign_expr, Some(Default::default())))
    } else {
        // Nested assignment: build chain of get_mut calls
        let mut chain = base_expr;
        for idx in &indices {
            // DEPYLER-E0277-FIX: String literals are already &str, don't add &
            // For HashMap<String, V>, get_mut expects &Q where String: Borrow<Q>
            // - "key" is &str, String: Borrow<str> ✓ → pass "key" directly
            // - var is String, &String auto-derefs to &str ✓ → pass &var
            // Check for both bare literals AND literals with .to_string() wrapping
            // quote! adds spaces around tokens, so check first non-space char is quote
            let idx_str = quote! { #idx }.to_string();
            let first_char = idx_str.trim_start().chars().next();
            let is_str_lit = first_char == Some('"');
            chain = if is_str_lit {
                parse_quote! { #chain.get_mut(#idx).expect("index out of bounds") }
            } else {
                parse_quote! { #chain.get_mut(&#idx).expect("index out of bounds") }
            };
        }

        let assign_expr = parse_quote! {
            #chain.insert(#final_index, #value_expr)
        };

        Ok(syn::Stmt::Expr(assign_expr, Some(Default::default())))
    }
}

/// Convert attribute assignment: `obj.attr = value`
///
/// Complexity: 1 (no branching)
pub(crate) fn convert_attribute_assignment(
    base: &HirExpr,
    attr: &str,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    let base_expr = convert_expr(base, type_mapper)?;
    let attr_ident = make_ident(attr);

    let assign_expr = parse_quote! {
        #base_expr.#attr_ident = #value_expr
    };

    Ok(syn::Stmt::Expr(assign_expr, Some(Default::default())))
}

/// Convert Python assignment statement to Rust
///
/// Handles 3 assignment target types:
/// - Symbol: `x = value`
/// - Index: `d[k] = value` or `d[k1][k2] = value`
/// - Attribute: `obj.attr = value`
///
/// Complexity: ~8 (match with 3 arms + nested complexity from index helper)
#[allow(dead_code)]
pub(crate) fn convert_assign_stmt(
    target: &AssignTarget,
    value: &HirExpr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    let value_expr = convert_expr(value, type_mapper)?;
    convert_assign_stmt_with_expr(target, value_expr, type_mapper)
}

pub(crate) fn convert_assign_stmt_with_expr(
    target: &AssignTarget,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    // Backward compatibility - use empty mutable_vars (all variables get mut)
    static EMPTY_MUTABLE: std::sync::OnceLock<std::collections::HashSet<String>> =
        std::sync::OnceLock::new();
    convert_assign_stmt_with_mutable_vars(
        target,
        value_expr,
        type_mapper,
        EMPTY_MUTABLE.get_or_init(std::collections::HashSet::new),
    )
}

/// DEPYLER-0713: Convert assignment with proper mutability tracking
pub(crate) fn convert_assign_stmt_with_mutable_vars(
    target: &AssignTarget,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
    mutable_vars: &std::collections::HashSet<String>,
) -> Result<syn::Stmt> {
    match target {
        AssignTarget::Symbol(symbol) => convert_symbol_assignment(symbol, value_expr, mutable_vars),
        AssignTarget::Index { base, index } => {
            convert_index_assignment(base, index, value_expr, type_mapper)
        }
        AssignTarget::Attribute { value: base, attr } => {
            convert_attribute_assignment(base, attr, value_expr, type_mapper)
        }
        AssignTarget::Tuple(targets) => {
            // Tuple unpacking - simplified version
            let all_symbols: Option<Vec<&str>> = targets
                .iter()
                .map(|t| match t {
                    AssignTarget::Symbol(s) => Some(s.as_str()),
                    _ => None,
                })
                .collect();

            match all_symbols {
                Some(symbols) => {
                    // DEPYLER-0713: Only add mut if variable is in mutable_vars
                    let idents_with_mut: Vec<_> = symbols
                        .iter()
                        .map(|s| (make_ident(s), mutable_vars.contains(*s)))
                        .collect();
                    let pat = syn::Pat::Tuple(syn::PatTuple {
                        attrs: vec![],
                        paren_token: syn::token::Paren::default(),
                        elems: idents_with_mut
                            .iter()
                            .map(|(ident, needs_mut)| {
                                syn::Pat::Ident(syn::PatIdent {
                                    attrs: vec![],
                                    by_ref: None,
                                    mutability: if *needs_mut {
                                        Some(syn::token::Mut::default())
                                    } else {
                                        None
                                    },
                                    ident: ident.clone(),
                                    subpat: None,
                                })
                            })
                            .collect(),
                    });
                    Ok(syn::Stmt::Local(syn::Local {
                        attrs: vec![],
                        let_token: syn::token::Let::default(),
                        pat,
                        init: Some(syn::LocalInit {
                            eq_token: syn::token::Eq::default(),
                            expr: Box::new(value_expr),
                            diverge: None,
                        }),
                        semi_token: syn::token::Semi::default(),
                    }))
                }
                None => {
                    // GH-109: Handle tuple unpacking with Index targets
                    // Pattern: list[i], list[j] = list[j], list[i] (swap)
                    let all_indices: Option<Vec<_>> = targets
                        .iter()
                        .map(|t| match t {
                            AssignTarget::Index { base, index } => Some((base, index)),
                            _ => None,
                        })
                        .collect();

                    if let Some(indices) = all_indices {
                        // All targets are subscripts - generate intermediate-based assignment
                        let temp_ident = make_ident("_swap_temp");

                        // Build assignments for each target from intermediate tuple
                        let mut stmts: Vec<syn::Stmt> = Vec::new();

                        // First: let _swap_temp = value_expr;
                        stmts.push(syn::Stmt::Local(syn::Local {
                            attrs: vec![],
                            let_token: syn::token::Let::default(),
                            pat: syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: temp_ident.clone(),
                                subpat: None,
                            }),
                            init: Some(syn::LocalInit {
                                eq_token: syn::token::Eq::default(),
                                expr: Box::new(value_expr),
                                diverge: None,
                            }),
                            semi_token: syn::token::Semi::default(),
                        }));

                        // Then: base[index] = _swap_temp.N for each target
                        for (idx, (base, index)) in indices.iter().enumerate() {
                            let base_expr = convert_expr(base, type_mapper)?;
                            let index_expr = convert_expr(index, type_mapper)?;
                            let tuple_index = syn::Index::from(idx);

                            let assign_expr: syn::Expr = parse_quote! {
                                #base_expr[(#index_expr) as usize] = #temp_ident.#tuple_index
                            };
                            stmts.push(syn::Stmt::Expr(assign_expr, Some(Default::default())));
                        }

                        // Return a block containing all statements
                        // Note: We return just the first statement; caller may need to handle block
                        if stmts.len() == 1 {
                            return Ok(stmts.remove(0));
                        } else {
                            // Wrap in a block expression
                            let block = syn::Block {
                                brace_token: syn::token::Brace::default(),
                                stmts,
                            };
                            return Ok(syn::Stmt::Expr(
                                syn::Expr::Block(syn::ExprBlock {
                                    attrs: vec![],
                                    label: None,
                                    block,
                                }),
                                None,
                            ));
                        }
                    }

                    bail!("Complex tuple unpacking not yet supported")
                }
            }
        }
    }
}

/// DEPYLER-0701: Check if an expression is "pure" (has no side effects)
/// Pure expressions used as statements need `let _ =` to silence warnings
pub(crate) fn is_pure_expression_direct(expr: &HirExpr) -> bool {
    match expr {
        // Bare variable references have no side effects
        HirExpr::Var(_) => true,
        // Literals have no side effects
        HirExpr::Literal(_) => true,
        // Attribute access (self.x) has no side effects
        HirExpr::Attribute { .. } => true,
        // Binary operations with pure operands are pure
        HirExpr::Binary { left, right, .. } => {
            is_pure_expression_direct(left) && is_pure_expression_direct(right)
        }
        // Unary operations with pure operands are pure
        HirExpr::Unary { operand, .. } => is_pure_expression_direct(operand),
        // Index access (arr[i]) has no side effects
        HirExpr::Index { base, index } => {
            is_pure_expression_direct(base) && is_pure_expression_direct(index)
        }
        // Tuple access is pure
        HirExpr::Tuple(elts) => elts.iter().all(is_pure_expression_direct),
        // Everything else (calls, method calls, etc.) may have side effects
        _ => false,
    }
}
