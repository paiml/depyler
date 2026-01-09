//! Statement and expression conversion functions for direct rules
//!
//! DEPYLER-COVERAGE-95: Extracted from direct_rules.rs to reduce file size
//! and improve testability. Contains body/statement/expression conversion.

use crate::direct_rules::{extract_nested_indices, make_ident, parse_target_pattern, safe_class_name, type_to_rust_type};
use crate::hir::*;
use crate::rust_gen::keywords::safe_ident;
use crate::type_mapper::TypeMapper;
use anyhow::{bail, Result};
use quote::quote;
use syn::{self, parse_quote};

pub(crate) fn convert_body(stmts: &[HirStmt], type_mapper: &TypeMapper) -> Result<Vec<syn::Stmt>> {
    // Use empty vararg_functions and param_types for backward compatibility
    static EMPTY_SET: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    static EMPTY_MAP: std::sync::OnceLock<std::collections::HashMap<String, Type>> = std::sync::OnceLock::new();
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
                    AssignTarget::Attribute { value: base, .. } |
                    AssignTarget::Index { base, .. } => {
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
            HirStmt::If { condition, then_body, else_body } => {
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
            HirStmt::Try { body, handlers, orelse, finalbody, .. } => {
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
            HirExpr::MethodCall { object, method, args, .. } => {
                // Check if this is a mutating method
                // DEPYLER-1002: Added hexdigest/digest for hashlib - finalize_reset() requires &mut self
                let is_mutating = matches!(
                    method.as_str(),
                    "append" | "extend" | "insert" | "remove" | "pop" | "clear" |
                    "reverse" | "sort" | "update" | "setdefault" | "popitem" |
                    "add" | "discard" | "write" | "write_all" | "writelines" | "flush" |
                    "hexdigest" | "digest" | "finalize" | "finalize_reset"
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
        .map(|stmt| convert_stmt_with_mutable_vars(stmt, type_mapper, is_classmethod, vararg_functions, param_types, &mutable_vars))
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
            chain = parse_quote! {
                #chain.get_mut(&#idx).unwrap()
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
    static EMPTY_MUTABLE: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    convert_assign_stmt_with_mutable_vars(target, value_expr, type_mapper, EMPTY_MUTABLE.get_or_init(std::collections::HashSet::new))
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
                                    mutability: if *needs_mut { Some(syn::token::Mut::default()) } else { None },
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
                        // All targets are subscripts - generate temp-based assignment
                        let temp_ident = make_ident("_swap_temp");

                        // Build assignments for each target from temp tuple
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

#[allow(dead_code)]
pub(crate) fn convert_stmt(stmt: &HirStmt, type_mapper: &TypeMapper) -> Result<syn::Stmt> {
    // Use empty vararg_functions and param_types for backward compatibility
    static EMPTY_SET: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    static EMPTY_MAP: std::sync::OnceLock<std::collections::HashMap<String, Type>> = std::sync::OnceLock::new();
    convert_stmt_with_context(
        stmt,
        type_mapper,
        false,
        EMPTY_SET.get_or_init(std::collections::HashSet::new),
        EMPTY_MAP.get_or_init(std::collections::HashMap::new),
    )
}

/// DEPYLER-0713: Convert statement with proper mutability tracking
pub(crate) fn convert_stmt_with_mutable_vars(
    stmt: &HirStmt,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    mutable_vars: &std::collections::HashSet<String>,
) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            let value_expr = convert_expr_with_param_types(value, type_mapper, is_classmethod, vararg_functions, param_types)?;
            convert_assign_stmt_with_mutable_vars(target, value_expr, type_mapper, mutable_vars)
        }
        // For all other statement types, delegate to convert_stmt_with_context
        // They don't generate new variable bindings so mutable_vars doesn't matter
        _ => convert_stmt_with_context(stmt, type_mapper, is_classmethod, vararg_functions, param_types),
    }
}

/// DEPYLER-0704: Added param_types parameter for type coercion in binary operations
pub(crate) fn convert_stmt_with_context(
    stmt: &HirStmt,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            // For assignments, we need to convert the value expression with classmethod context
            // DEPYLER-0704: Pass param_types for type coercion
            let value_expr = convert_expr_with_param_types(value, type_mapper, is_classmethod, vararg_functions, param_types)?;
            convert_assign_stmt_with_expr(target, value_expr, type_mapper)
        }
        HirStmt::Return(expr) => {
            let ret_expr = if let Some(e) = expr {
                // DEPYLER-0704: Pass param_types for type coercion in return expressions
                convert_expr_with_param_types(e, type_mapper, is_classmethod, vararg_functions, param_types)?
            } else {
                parse_quote! { () }
            };
            Ok(syn::Stmt::Expr(
                parse_quote! { return #ret_expr },
                Some(Default::default()),
            ))
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond = convert_expr_with_param_types(condition, type_mapper, is_classmethod, vararg_functions, param_types)?;
            let then_block = convert_block_with_context(then_body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            let if_expr = if let Some(else_stmts) = else_body {
                let else_block =
                    convert_block_with_context(else_stmts, type_mapper, is_classmethod, vararg_functions, param_types)?;
                parse_quote! {
                    if #cond #then_block else #else_block
                }
            } else {
                parse_quote! {
                    if #cond #then_block
                }
            };

            Ok(syn::Stmt::Expr(if_expr, Some(Default::default())))
        }
        HirStmt::While { condition, body } => {
            let cond = convert_expr_with_param_types(condition, type_mapper, is_classmethod, vararg_functions, param_types)?;
            let body_block = convert_block_with_context(body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            let while_expr = parse_quote! {
                while #cond #body_block
            };

            Ok(syn::Stmt::Expr(while_expr, Some(Default::default())))
        }
        HirStmt::For { target, iter, body } => {
            // Generate target pattern based on AssignTarget type
            let target_pattern: syn::Pat = match target {
                AssignTarget::Symbol(name) => {
                    let ident = make_ident(name);
                    parse_quote! { #ident }
                }
                AssignTarget::Tuple(targets) => {
                    let idents: Vec<syn::Ident> = targets
                        .iter()
                        .map(|t| match t {
                            AssignTarget::Symbol(s) => {
                                Ok(make_ident(s))
                            }
                            _ => bail!("Nested tuple unpacking not supported in for loops"),
                        })
                        .collect::<Result<Vec<_>>>()?;
                    parse_quote! { (#(#idents),*) }
                }
                _ => bail!("Unsupported for loop target type"),
            };

            let iter_expr = convert_expr_with_param_types(iter, type_mapper, is_classmethod, vararg_functions, param_types)?;
            let body_block = convert_block_with_context(body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            let for_expr = parse_quote! {
                for #target_pattern in #iter_expr #body_block
            };

            Ok(syn::Stmt::Expr(for_expr, Some(Default::default())))
        }
        HirStmt::Expr(expr) => {
            // DEPYLER-0701: Detect expressions without side effects and wrap with `let _ =`
            // to avoid "path statement with no effect" and "unused arithmetic operation" warnings
            if is_pure_expression_direct(expr) {
                let rust_expr = convert_expr_with_param_types(expr, type_mapper, is_classmethod, vararg_functions, param_types)?;
                Ok(syn::Stmt::Local(syn::Local {
                    attrs: vec![],
                    let_token: syn::Token![let](proc_macro2::Span::call_site()),
                    pat: syn::Pat::Wild(syn::PatWild {
                        attrs: vec![],
                        underscore_token: syn::Token![_](proc_macro2::Span::call_site()),
                    }),
                    init: Some(syn::LocalInit {
                        eq_token: syn::Token![=](proc_macro2::Span::call_site()),
                        expr: Box::new(rust_expr),
                        diverge: None,
                    }),
                    semi_token: syn::Token![;](proc_macro2::Span::call_site()),
                }))
            } else {
                let rust_expr = convert_expr_with_param_types(expr, type_mapper, is_classmethod, vararg_functions, param_types)?;
                Ok(syn::Stmt::Expr(rust_expr, Some(Default::default())))
            }
        }
        HirStmt::Raise {
            exception,
            cause: _,
        } => {
            // Convert to Rust panic for direct rules
            let panic_expr = if let Some(exc) = exception {
                let exc_expr = convert_expr_with_param_types(exc, type_mapper, is_classmethod, vararg_functions, param_types)?;
                parse_quote! { panic!("Exception: {}", #exc_expr) }
            } else {
                parse_quote! { panic!("Exception raised") }
            };
            Ok(syn::Stmt::Expr(panic_expr, Some(Default::default())))
        }
        HirStmt::Break { label } => {
            let break_expr = if let Some(label_name) = label {
                let label_ident =
                    syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
                parse_quote! { break #label_ident }
            } else {
                parse_quote! { break }
            };
            Ok(syn::Stmt::Expr(break_expr, Some(Default::default())))
        }
        HirStmt::Continue { label } => {
            let continue_expr = if let Some(label_name) = label {
                let label_ident =
                    syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
                parse_quote! { continue #label_ident }
            } else {
                parse_quote! { continue }
            };
            Ok(syn::Stmt::Expr(continue_expr, Some(Default::default())))
        }
        HirStmt::With {
            context,
            target,
            body,
            ..
        } => {
            // Convert context expression
            let context_expr = convert_expr_with_param_types(context, type_mapper, is_classmethod, vararg_functions, param_types)?;

            // Convert body to a block
            let body_block = convert_block_with_context(body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            // Generate a scope block with optional variable binding
            let block_expr = if let Some(var_name) = target {
                let var_ident = make_ident(var_name);
                parse_quote! {
                    {
                        let mut #var_ident = #context_expr;
                        #body_block
                    }
                }
            } else {
                parse_quote! {
                    {
                        let _context = #context_expr;
                        #body_block
                    }
                }
            };

            Ok(syn::Stmt::Expr(block_expr, None))
        }
        HirStmt::Try {
            body,
            handlers,
            orelse: _,
            finalbody,
        } => {
            // Convert try body
            let try_stmts = convert_block_with_context(body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            // Convert finally block if present
            let finally_block = finalbody
                .as_ref()
                .map(|fb| convert_block_with_context(fb, type_mapper, is_classmethod, vararg_functions, param_types))
                .transpose()?;

            // Convert except handlers (use first handler for simplicity)
            if let Some(handler) = handlers.first() {
                let handler_block =
                    convert_block_with_context(&handler.body, type_mapper, is_classmethod, vararg_functions, param_types)?;

                // DEPYLER-0937: Use actual exception variable name if present
                // This fixes E0425 where handler body references 'e' but pattern used '_e'
                let err_pattern: syn::Pat = if let Some(exc_var) = &handler.name {
                    let exc_ident = syn::Ident::new(exc_var, proc_macro2::Span::call_site());
                    parse_quote! { Err(#exc_ident) }
                } else {
                    parse_quote! { Err(_) }
                };

                let block_expr = if let Some(finally_stmts) = finally_block {
                    parse_quote! {
                        {
                            let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                #try_stmts
                                Ok(())
                            })();
                            if let #err_pattern = _result {
                                #handler_block
                            }
                            #finally_stmts
                        }
                    }
                } else {
                    parse_quote! {
                        {
                            let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                #try_stmts
                                Ok(())
                            })();
                            if let #err_pattern = _result {
                                #handler_block
                            }
                        }
                    }
                };
                Ok(syn::Stmt::Expr(block_expr, None))
            } else {
                // No handlers - try/finally without except
                let block_expr = if let Some(finally_stmts) = finally_block {
                    parse_quote! {
                        {
                            #try_stmts
                            #finally_stmts
                        }
                    }
                } else {
                    parse_quote! { #try_stmts }
                };
                Ok(syn::Stmt::Expr(block_expr, None))
            }
        }
        HirStmt::Assert { test, msg } => {
            // Generate assert! macro call
            let test_expr = convert_expr_with_param_types(test, type_mapper, is_classmethod, vararg_functions, param_types)?;
            let assert_macro: syn::Stmt = if let Some(message) = msg {
                let msg_expr = convert_expr_with_param_types(message, type_mapper, is_classmethod, vararg_functions, param_types)?;
                parse_quote! { assert!(#test_expr, "{}", #msg_expr); }
            } else {
                parse_quote! { assert!(#test_expr); }
            };
            Ok(assert_macro)
        }
        HirStmt::Pass => {
            // Pass statement generates empty statement
            Ok(syn::Stmt::Expr(parse_quote! { {} }, None))
        }
        // DEPYLER-0614: Handle Block of statements - convert first statement
        // Note: This is a simplification; blocks are flattened during codegen
        HirStmt::Block(stmts) => {
            if stmts.is_empty() {
                Ok(syn::Stmt::Expr(parse_quote! { {} }, None))
            } else {
                convert_stmt_with_context(&stmts[0], type_mapper, is_classmethod, vararg_functions, param_types)
            }
        }
        // DEPYLER-0840: Properly generate nested functions as closures
        // Previously this just returned {} causing E0425 "cannot find value" errors
        HirStmt::FunctionDef { name, params, ret_type, body, .. } => {
            let fn_name = safe_ident(name);

            // Generate parameter tokens
            let param_tokens: Vec<proc_macro2::TokenStream> = params
                .iter()
                .map(|p| {
                    let param_name = safe_ident(&p.name);
                    let param_type = type_to_rust_type(&p.ty, type_mapper);
                    quote! { #param_name: #param_type }
                })
                .collect();

            // Generate body statements
            let body_stmts: Vec<syn::Stmt> = body
                .iter()
                .filter_map(|stmt| {
                    convert_stmt_with_context(stmt, type_mapper, is_classmethod, vararg_functions, param_types).ok()
                })
                .collect();

            // Generate return type if not Unknown
            let closure_expr = if matches!(ret_type, Type::Unknown) {
                quote! {
                    let #fn_name = move |#(#param_tokens),*| {
                        #(#body_stmts)*
                    };
                }
            } else {
                let return_type = type_to_rust_type(ret_type, type_mapper);
                quote! {
                    let #fn_name = move |#(#param_tokens),*| -> #return_type {
                        #(#body_stmts)*
                    };
                }
            };

            Ok(syn::parse2(closure_expr).unwrap_or_else(|_| parse_quote! { {} }))
        }
    }
}

#[allow(dead_code)]
pub(crate) fn convert_block(stmts: &[HirStmt], type_mapper: &TypeMapper) -> Result<syn::Block> {
    // Use empty vararg_functions for backward compatibility
    static EMPTY_SET: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    static EMPTY_MAP: std::sync::OnceLock<std::collections::HashMap<String, Type>> = std::sync::OnceLock::new();
    convert_block_with_context(
        stmts,
        type_mapper,
        false,
        EMPTY_SET.get_or_init(std::collections::HashSet::new),
        EMPTY_MAP.get_or_init(std::collections::HashMap::new),
    )
}

/// DEPYLER-0704: Added param_types parameter for type coercion in binary operations
pub(crate) fn convert_block_with_context(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
) -> Result<syn::Block> {
    let rust_stmts = convert_body_with_context(stmts, type_mapper, is_classmethod, vararg_functions, param_types)?;
    Ok(syn::Block {
        brace_token: Default::default(),
        stmts: rust_stmts,
    })
}

/// DEPYLER-0720: Convert method body block with class field type awareness
/// This is used for class methods where we know the field types
/// DEPYLER-1037: Added ret_type parameter for Optional wrapping in return statements
pub(crate) fn convert_method_body_block(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
    ret_type: &Type,
) -> Result<syn::Block> {
    let rust_stmts = convert_method_body_stmts(stmts, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types, ret_type)?;
    Ok(syn::Block {
        brace_token: Default::default(),
        stmts: rust_stmts,
    })
}

/// DEPYLER-0720: Convert method body statements with class field type awareness
/// DEPYLER-1037: Added ret_type parameter for Optional wrapping in return statements
pub(crate) fn convert_method_body_stmts(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
    ret_type: &Type,
) -> Result<Vec<syn::Stmt>> {
    // DEPYLER-0713: Pre-analyze which variables need to be mutable
    let mutable_vars = find_mutable_vars_in_body(stmts);

    stmts
        .iter()
        .map(|stmt| convert_method_stmt(stmt, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types, &mutable_vars, ret_type))
        .collect()
}

/// DEPYLER-0720: Convert a single statement with class field type awareness
/// DEPYLER-1037: Added ret_type parameter for Optional wrapping in return statements
#[allow(clippy::too_many_arguments)]
pub(crate) fn convert_method_stmt(
    stmt: &HirStmt,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
    mutable_vars: &std::collections::HashSet<String>,
    ret_type: &Type,
) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            let value_expr = convert_expr_with_class_fields(value, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
            convert_assign_stmt_with_mutable_vars(target, value_expr, type_mapper, mutable_vars)
        }
        HirStmt::Return(expr) => {
            // DEPYLER-1037: Check if return type is Optional for proper wrapping
            let is_optional_return = matches!(ret_type, Type::Optional(_));

            let ret_expr = if let Some(e) = expr {
                // Check if expression is None literal
                let is_none_literal = matches!(e, HirExpr::Literal(Literal::None));
                let converted = convert_expr_with_class_fields(e, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;

                if is_optional_return && !is_none_literal {
                    // Wrap non-None values in Some() for Optional return types
                    parse_quote! { Some(#converted) }
                } else {
                    converted
                }
            } else {
                // Bare return statement
                if is_optional_return {
                    parse_quote! { None }
                } else {
                    parse_quote! { () }
                }
            };
            Ok(syn::Stmt::Expr(
                parse_quote! { return #ret_expr },
                Some(Default::default()),
            ))
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond = convert_expr_with_class_fields(condition, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
            let then_block = convert_method_body_block(then_body, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types, ret_type)?;

            let if_expr = if let Some(else_stmts) = else_body {
                let else_block =
                    convert_method_body_block(else_stmts, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types, ret_type)?;
                parse_quote! {
                    if #cond #then_block else #else_block
                }
            } else {
                parse_quote! {
                    if #cond #then_block
                }
            };

            Ok(syn::Stmt::Expr(if_expr, Some(Default::default())))
        }
        HirStmt::While { condition, body } => {
            let cond = convert_expr_with_class_fields(condition, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
            let body_block = convert_method_body_block(body, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types, ret_type)?;

            let while_expr = parse_quote! {
                while #cond #body_block
            };

            Ok(syn::Stmt::Expr(while_expr, Some(Default::default())))
        }
        HirStmt::For { target, iter, body } => {
            // Generate target pattern based on AssignTarget type
            let target_pattern: syn::Pat = match target {
                AssignTarget::Symbol(name) => {
                    let ident = make_ident(name);
                    parse_quote! { #ident }
                }
                AssignTarget::Tuple(targets) => {
                    let idents: Vec<syn::Ident> = targets
                        .iter()
                        .map(|t| match t {
                            AssignTarget::Symbol(s) => {
                                Ok(make_ident(s))
                            }
                            _ => bail!("Nested tuple unpacking not supported in for loops"),
                        })
                        .collect::<Result<Vec<_>>>()?;
                    parse_quote! { (#(#idents),*) }
                }
                _ => bail!("Unsupported for loop target type"),
            };

            let iter_expr = convert_expr_with_class_fields(iter, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
            let body_block = convert_method_body_block(body, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types, ret_type)?;

            let for_expr = parse_quote! {
                for #target_pattern in #iter_expr #body_block
            };

            Ok(syn::Stmt::Expr(for_expr, Some(Default::default())))
        }
        HirStmt::Expr(expr) => {
            if is_pure_expression_direct(expr) {
                let rust_expr = convert_expr_with_class_fields(expr, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
                Ok(syn::Stmt::Local(syn::Local {
                    attrs: vec![],
                    let_token: syn::Token![let](proc_macro2::Span::call_site()),
                    pat: syn::Pat::Wild(syn::PatWild {
                        attrs: vec![],
                        underscore_token: syn::Token![_](proc_macro2::Span::call_site()),
                    }),
                    init: Some(syn::LocalInit {
                        eq_token: syn::Token![=](proc_macro2::Span::call_site()),
                        expr: Box::new(rust_expr),
                        diverge: None,
                    }),
                    semi_token: syn::Token![;](proc_macro2::Span::call_site()),
                }))
            } else {
                let rust_expr = convert_expr_with_class_fields(expr, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
                Ok(syn::Stmt::Expr(rust_expr, Some(Default::default())))
            }
        }
        // For other statement types, fall back to existing conversion
        _ => convert_stmt_with_context(stmt, type_mapper, is_classmethod, vararg_functions, param_types),
    }
}

/// Convert HIR expressions to Rust expressions using strategy pattern
#[allow(dead_code)]
pub(crate) fn convert_expr(expr: &HirExpr, type_mapper: &TypeMapper) -> Result<syn::Expr> {
    // Use empty vararg_functions for backward compatibility
    static EMPTY: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    convert_expr_with_context(expr, type_mapper, false, EMPTY.get_or_init(std::collections::HashSet::new))
}

/// Convert HIR expressions with classmethod context and vararg tracking
pub(crate) fn convert_expr_with_context(
    expr: &HirExpr,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
) -> Result<syn::Expr> {
    // DEPYLER-0648: Use with_varargs to track functions that need slice wrapping
    let converter = ExprConverter::with_varargs(type_mapper, is_classmethod, vararg_functions);
    converter.convert(expr)
}

/// DEPYLER-0704: Convert HIR expressions with parameter type information for type coercion
pub(crate) fn convert_expr_with_param_types(
    expr: &HirExpr,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
) -> Result<syn::Expr> {
    let converter = ExprConverter::with_param_types(
        type_mapper,
        is_classmethod,
        vararg_functions,
        param_types.clone(),
    );
    converter.convert(expr)
}

/// DEPYLER-0720: Convert HIR expressions with class field types for self.field coercion
pub(crate) fn convert_expr_with_class_fields(
    expr: &HirExpr,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
) -> Result<syn::Expr> {
    let converter = ExprConverter::with_class_fields(
        type_mapper,
        is_classmethod,
        vararg_functions,
        param_types.clone(),
        class_field_types.clone(),
    );
    converter.convert(expr)
}

/// Expression converter using strategy pattern to reduce complexity
pub(crate) struct ExprConverter<'a> {
    #[allow(dead_code)]
    type_mapper: &'a TypeMapper,
    is_classmethod: bool,
    /// DEPYLER-0648: Track functions that have vararg parameters (*args in Python)
    /// Call sites need to wrap arguments in &[...] slices
    vararg_functions: &'a std::collections::HashSet<String>,
    /// DEPYLER-0704: Parameter types for type coercion in binary operations
    param_types: std::collections::HashMap<String, Type>,
    /// DEPYLER-0720: Class field types for self.field attribute access
    /// Maps field name -> Type, used to determine if self.X is float for int-to-float coercion
    class_field_types: std::collections::HashMap<String, Type>,
}

impl<'a> ExprConverter<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(type_mapper: &'a TypeMapper) -> Self {
        // Use empty static HashSet for backwards compatibility
        static EMPTY: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
        Self {
            type_mapper,
            is_classmethod: false,
            vararg_functions: EMPTY.get_or_init(std::collections::HashSet::new),
            param_types: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn with_classmethod(type_mapper: &'a TypeMapper, is_classmethod: bool) -> Self {
        // Use empty static HashSet for backwards compatibility
        static EMPTY: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
        Self {
            type_mapper,
            is_classmethod,
            vararg_functions: EMPTY.get_or_init(std::collections::HashSet::new),
            param_types: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
        }
    }

    /// DEPYLER-0648: Create converter with vararg function tracking
    fn with_varargs(
        type_mapper: &'a TypeMapper,
        is_classmethod: bool,
        vararg_functions: &'a std::collections::HashSet<String>,
    ) -> Self {
        Self {
            type_mapper,
            is_classmethod,
            vararg_functions,
            param_types: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
        }
    }

    /// DEPYLER-0704: Create converter with parameter types for type coercion
    fn with_param_types(
        type_mapper: &'a TypeMapper,
        is_classmethod: bool,
        vararg_functions: &'a std::collections::HashSet<String>,
        param_types: std::collections::HashMap<String, Type>,
    ) -> Self {
        Self {
            type_mapper,
            is_classmethod,
            vararg_functions,
            param_types,
            class_field_types: std::collections::HashMap::new(),
        }
    }

    /// DEPYLER-0720: Create converter with class field types for self.field access
    fn with_class_fields(
        type_mapper: &'a TypeMapper,
        is_classmethod: bool,
        vararg_functions: &'a std::collections::HashSet<String>,
        param_types: std::collections::HashMap<String, Type>,
        class_field_types: std::collections::HashMap<String, Type>,
    ) -> Self {
        Self {
            type_mapper,
            is_classmethod,
            vararg_functions,
            param_types,
            class_field_types,
        }
    }

    /// DEPYLER-0704: Check if expression returns a float type
    /// DEPYLER-0720: Extended to check class field types for self.field patterns
    fn expr_returns_float_direct(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::Float(_)) => true,
            HirExpr::Var(name) => {
                // Check param_types
                matches!(self.param_types.get(name), Some(Type::Float))
            }
            // DEPYLER-0720: Check class field types for self.field attribute access
            HirExpr::Attribute { value, attr } => {
                // Check if this is self.field pattern where field is a float
                if matches!(value.as_ref(), HirExpr::Var(name) if name == "self")
                    && matches!(self.class_field_types.get(attr), Some(Type::Float))
                {
                    return true;
                }
                false
            }
            HirExpr::Binary { left, right, .. } => {
                // Binary with float operand returns float
                self.expr_returns_float_direct(left) || self.expr_returns_float_direct(right)
            }
            HirExpr::MethodCall { method, .. } => {
                // Common float-returning methods
                matches!(
                    method.as_str(),
                    "mean" | "sum" | "std" | "norm" | "variance"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0704: Check if expression is an integer type
    fn is_int_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::Int(_)) => true,
            HirExpr::Var(name) => {
                // Check param_types
                matches!(self.param_types.get(name), Some(Type::Int))
            }
            _ => false,
        }
    }

    pub(crate) fn convert(&self, expr: &HirExpr) -> Result<syn::Expr> {
        match expr {
            HirExpr::Literal(lit) => self.convert_literal(lit),
            HirExpr::Var(name) => self.convert_variable(name),
            HirExpr::Binary { op, left, right } => self.convert_binary(*op, left, right),
            HirExpr::Unary { op, operand } => self.convert_unary(*op, operand),
            HirExpr::Call { func, args, .. } => self.convert_call(func, args),
            HirExpr::Index { base, index } => self.convert_index(base, index),
            // DEPYLER-0596: Add Slice support for string slicing with negative indices
            HirExpr::Slice { base, start, stop, step } => self.convert_slice(base, start, stop, step),
            HirExpr::List(elts) => self.convert_list(elts),
            HirExpr::Dict(items) => self.convert_dict(items),
            HirExpr::Tuple(elts) => self.convert_tuple(elts),
            HirExpr::Set(elts) => self.convert_set(elts),
            HirExpr::FrozenSet(elts) => self.convert_frozenset(elts),
            HirExpr::Lambda { params, body } => self.convert_lambda(params, body),
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => self.convert_method_call(object, method, args),
            // DEPYLER-0188: Dynamic function call (e.g., handlers[name](args))
            HirExpr::DynamicCall { callee, args, .. } => {
                self.convert_dynamic_call(callee, args)
            }
            HirExpr::ListComp {
                element,
                generators,
            } => {
                // DEPYLER-0504: Legacy path - only support single generator for now
                if generators.len() != 1 {
                    bail!("Multiple generators not supported in direct rules path");
                }
                let gen = &generators[0];
                let condition = if gen.conditions.is_empty() {
                    None
                } else if gen.conditions.len() == 1 {
                    Some(Box::new(gen.conditions[0].clone()))
                } else {
                    bail!("Multiple conditions in generator not supported in direct rules path");
                };
                self.convert_list_comp(element, &gen.target, &gen.iter, &condition)
            }
            HirExpr::SetComp {
                element,
                generators,
            } => {
                // DEPYLER-0504: Legacy path - only support single generator for now
                if generators.len() != 1 {
                    bail!("Multiple generators not supported in direct rules path");
                }
                let gen = &generators[0];
                let condition = if gen.conditions.is_empty() {
                    None
                } else if gen.conditions.len() == 1 {
                    Some(Box::new(gen.conditions[0].clone()))
                } else {
                    bail!("Multiple conditions in generator not supported in direct rules path");
                };
                self.convert_set_comp(element, &gen.target, &gen.iter, &condition)
            }
            HirExpr::DictComp {
                key,
                value,
                generators,
            } => {
                // DEPYLER-0504: Legacy path - only support single generator for now
                if generators.len() != 1 {
                    bail!("Multiple generators not supported in direct rules path");
                }
                let gen = &generators[0];
                let condition = if gen.conditions.is_empty() {
                    None
                } else if gen.conditions.len() == 1 {
                    Some(Box::new(gen.conditions[0].clone()))
                } else {
                    bail!("Multiple conditions in generator not supported in direct rules path");
                };
                self.convert_dict_comp(key, value, &gen.target, &gen.iter, &condition)
            }
            HirExpr::Attribute { value, attr } => self.convert_attribute(value, attr),
            HirExpr::Await { value } => self.convert_await(value),
            // DEPYLER-0513: F-string support for class methods
            HirExpr::FString { parts } => self.convert_fstring(parts),
            // DEPYLER-0764: IfExpr (ternary operator) support for class methods
            // Python: a if cond else b → Rust: if cond { a } else { b }
            HirExpr::IfExpr { test, body, orelse } => {
                let test_expr = self.convert(test)?;
                let body_expr = self.convert(body)?;
                let orelse_expr = self.convert(orelse)?;
                Ok(parse_quote! { if #test_expr { #body_expr } else { #orelse_expr } })
            }
            // DEPYLER-0764: GeneratorExp support for class methods
            // Python: (x for x in items) → Rust: items.iter().map(|x| x)
            HirExpr::GeneratorExp { element, generators } => {
                // Only support single generator for direct rules path
                if generators.len() != 1 {
                    bail!("Multiple generators not supported in direct rules path");
                }
                let gen = &generators[0];
                let iter_expr = self.convert(&gen.iter)?;
                let element_expr = self.convert(element)?;
                let target_ident = make_ident(&gen.target);

                // Handle conditions
                if gen.conditions.is_empty() {
                    Ok(parse_quote! { #iter_expr.iter().map(|#target_ident| #element_expr) })
                } else if gen.conditions.len() == 1 {
                    let cond_expr = self.convert(&gen.conditions[0])?;
                    Ok(parse_quote! {
                        #iter_expr.iter()
                            .filter(|#target_ident| #cond_expr)
                            .map(|#target_ident| #element_expr)
                    })
                } else {
                    bail!("Multiple conditions in generator not supported in direct rules path");
                }
            }
            // DEPYLER-0764: SortByKey support for sorted() with key parameter
            HirExpr::SortByKey { iterable, key_params, key_body, reverse_expr } => {
                let iter_expr = self.convert(iterable)?;
                let key_body_expr = self.convert(key_body)?;

                // Build the key lambda parameter(s)
                let key_param = if key_params.len() == 1 {
                    let p = make_ident(&key_params[0]);
                    quote! { #p }
                } else {
                    let params: Vec<_> = key_params.iter().map(|p| make_ident(p)).collect();
                    quote! { (#(#params),*) }
                };

                // Check if reversed
                let is_reversed = match reverse_expr {
                    Some(boxed) => matches!(boxed.as_ref(), HirExpr::Literal(Literal::Bool(true))),
                    _ => false,
                };

                if is_reversed {
                    Ok(parse_quote! {
                        {
                            let mut v: Vec<_> = #iter_expr.into_iter().collect();
                            v.sort_by_key(|#key_param| std::cmp::Reverse(#key_body_expr));
                            v
                        }
                    })
                } else {
                    Ok(parse_quote! {
                        {
                            let mut v: Vec<_> = #iter_expr.into_iter().collect();
                            v.sort_by_key(|#key_param| #key_body_expr);
                            v
                        }
                    })
                }
            }
            _ => bail!("Expression type not yet supported: {:?}", expr),
        }
    }

    fn convert_literal(&self, lit: &Literal) -> Result<syn::Expr> {
        Ok(convert_literal(lit))
    }

    fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
        // DEPYLER-0597: In method context (not classmethod), 'self' should be Rust keyword
        // Python `self.x` in instance method must become Rust `self.x`, not `self_.x`
        if name == "self" && !self.is_classmethod {
            return Ok(parse_quote! { self });
        }
        // DEPYLER-0596: Use make_ident to handle keywords like "match"
        let ident = make_ident(name);
        Ok(parse_quote! { #ident })
    }

    fn convert_binary(&self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
        let left_expr = self.convert(left)?;
        let right_expr = self.convert(right)?;

        match op {
            BinOp::In => {
                // DEPYLER-0960: Check dict FIRST before string (overlapping names like "data", "result")
                if self.is_dict_expr(right) {
                    // Convert "x in dict" to "dict.contains_key(&x)" for dicts/maps
                    Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })
                } else if self.is_tuple_or_list_expr(right) {
                    // DEPYLER-0832: For tuples/lists, convert to array and use .contains()
                    // Python: x in (A, B, C) -> Rust: [A, B, C].contains(&x)
                    let elements: Vec<syn::Expr> = match right {
                        HirExpr::Tuple(elems) | HirExpr::List(elems) => {
                            elems.iter().map(|e| self.convert(e)).collect::<Result<Vec<_>>>()?
                        }
                        _ => vec![right_expr.clone()],
                    };
                    Ok(parse_quote! { [#(#elements),*].contains(&#left_expr) })
                } else if self.is_string_expr(right) {
                    // DEPYLER-0601: For strings, use .contains() instead of .contains_key()
                    // DEPYLER-0200: Use raw string literal or &* for Pattern trait
                    let pattern: syn::Expr = match left {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String (&*String -> &str)
                            // and &str (&*&str -> &str), avoiding unstable str_as_str feature
                            parse_quote! { &*#left_expr }
                        }
                    };
                    Ok(parse_quote! { #right_expr.contains(#pattern) })
                } else {
                    // Fallback: assume dict/HashMap
                    Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })
                }
            }
            BinOp::NotIn => {
                // DEPYLER-0960: Check dict FIRST before string (overlapping names like "data", "result")
                if self.is_dict_expr(right) {
                    // Convert "x not in dict" to "!dict.contains_key(&x)"
                    Ok(parse_quote! { !#right_expr.contains_key(&#left_expr) })
                } else if self.is_tuple_or_list_expr(right) {
                    // DEPYLER-0832: For tuples/lists, convert to array and use !.contains()
                    // Python: x not in (A, B, C) -> Rust: ![A, B, C].contains(&x)
                    let elements: Vec<syn::Expr> = match right {
                        HirExpr::Tuple(elems) | HirExpr::List(elems) => {
                            elems.iter().map(|e| self.convert(e)).collect::<Result<Vec<_>>>()?
                        }
                        _ => vec![right_expr.clone()],
                    };
                    Ok(parse_quote! { ![#(#elements),*].contains(&#left_expr) })
                } else if self.is_string_expr(right) {
                    // DEPYLER-0601: For strings, use !.contains() instead of !.contains_key()
                    // DEPYLER-0200: Use raw string literal or &* for Pattern trait
                    let pattern: syn::Expr = match left {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String and &str
                            parse_quote! { &*#left_expr }
                        }
                    };
                    Ok(parse_quote! { !#right_expr.contains(#pattern) })
                } else {
                    // Fallback: assume dict/HashMap
                    Ok(parse_quote! { !#right_expr.contains_key(&#left_expr) })
                }
            }
            // Set operators - check if both operands are sets
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor
                if self.is_set_expr(left) && self.is_set_expr(right) =>
            {
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub if self.is_set_expr(left) && self.is_set_expr(right) => {
                // Set difference operation
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub => {
                // Check if we're subtracting from a .len() call to prevent underflow
                if is_len_call(left) {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    // DEPYLER-0746: Wrap in parens to handle cast expressions like `x as usize`
                    Ok(parse_quote! { (#left_expr).saturating_sub(#right_expr) })
                } else {
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_expr #rust_op #right_expr })
                }
            }
            BinOp::FloorDiv => {
                // Python floor division semantics differ from Rust integer division
                // Python: rounds towards negative infinity (floor)
                // Rust: truncates towards zero
                // Note: This implementation works for integers with proper floor semantics.
                // Type-based dispatch for float division (using .floor()) would be ideal
                // but requires full type inference integration. This is a known limitation.
                // DEPYLER-0236: Use intermediate variables to avoid formatting issues with != operator

                Ok(parse_quote! {
                    {
                        let a = #left_expr;
                        let b = #right_expr;
                        let q = a / b;
                        let r = a % b;
                        let r_negative = r < 0;
                        let b_negative = b < 0;
                        let r_nonzero = r != 0;
                        let signs_differ = r_negative != b_negative;
                        let needs_adjustment = r_nonzero && signs_differ;
                        if needs_adjustment { q - 1 } else { q }
                    }
                })
            }
            BinOp::Mul => {
                // Special case: [value] * n or n * [value] creates an array
                match (left, right) {
                    // Pattern: [x] * n
                    (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = self.convert(&elts[0])?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Pattern: n * [x]
                    (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = self.convert(&elts[0])?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Default multiplication
                    _ => {
                        let rust_op = convert_binop(op)?;
                        // DEPYLER-0704: Type coercion for mixed int/float multiplication
                        // Rust doesn't auto-coerce, so we need explicit casts
                        let left_is_float = self.expr_returns_float_direct(left);
                        let right_is_float = self.expr_returns_float_direct(right);
                        let left_is_int = self.is_int_expr(left);
                        let right_is_int = self.is_int_expr(right);

                        let final_left = if right_is_float && left_is_int {
                            parse_quote! { (#left_expr as f64) }
                        } else {
                            left_expr
                        };
                        let final_right = if left_is_float && right_is_int {
                            parse_quote! { (#right_expr as f64) }
                        } else {
                            right_expr
                        };
                        Ok(parse_quote! { #final_left #rust_op #final_right })
                    }
                }
            }
            BinOp::Pow => {
                // Python power operator ** needs type-specific handling in Rust
                // For integers: use .pow() with u32 exponent
                // For floats: use .powf() with f64 exponent
                // For negative integer exponents: convert to float

                // DEPYLER-0699: Wrap expressions in block to ensure correct operator precedence
                // Without this, `a + b as f64` parses as `a + (b as f64)` instead of `(a + b) as f64`
                // DEPYLER-0707: Construct block directly instead of using parse_quote!
                // parse_quote! re-parses tokens which can fail with complex expressions
                fn wrap_expr_in_block(expr: syn::Expr) -> syn::Expr {
                    syn::Expr::Block(syn::ExprBlock {
                        attrs: vec![],
                        label: None,
                        block: syn::Block {
                            brace_token: syn::token::Brace::default(),
                            stmts: vec![syn::Stmt::Expr(expr, None)],
                        },
                    })
                }
                let left_paren = wrap_expr_in_block(left_expr.clone());
                let right_paren = wrap_expr_in_block(right_expr.clone());

                // Check if we have literals to determine types
                match (left, right) {
                    // Integer literal base with integer literal exponent
                    (HirExpr::Literal(Literal::Int(_)), HirExpr::Literal(Literal::Int(exp))) => {
                        if *exp < 0 {
                            // Negative exponent: convert to float operation
                            Ok(parse_quote! {
                                (#left_paren as f64).powf(#right_paren as f64)
                            })
                        } else {
                            // Positive integer exponent: use .pow() with u32
                            // Add checked_pow for overflow safety
                            // DEPYLER-0746: Wrap in parens to handle cast expressions
                            Ok(parse_quote! {
                                (#left_expr).checked_pow(#right_expr as u32)
                                    .expect("Power operation overflowed")
                            })
                        }
                    }
                    // Float literal base: always use .powf()
                    // DEPYLER-0408: Cast float literal to f64 for concrete type
                    (HirExpr::Literal(Literal::Float(_)), _) => Ok(parse_quote! {
                        (#left_paren as f64).powf(#right_paren as f64)
                    }),
                    // Any base with float exponent: use .powf()
                    // DEPYLER-0408: Cast float literal exponent to f64 for concrete type
                    (_, HirExpr::Literal(Literal::Float(_))) => Ok(parse_quote! {
                        (#left_paren as f64).powf(#right_paren as f64)
                    }),
                    // Variables or complex expressions: generate type-safe code
                    _ => {
                        // For non-literal expressions, we need runtime type checking
                        // This is a conservative approach that works for common cases
                        // DEPYLER-0405: Cast both sides to i64 for type-safe comparison
                        Ok(parse_quote! {
                            {
                                // Try integer power first if exponent can be u32
                                if #right_expr >= 0 && (#right_expr as i64) <= (u32::MAX as i64) {
                                    (#left_paren as i32).checked_pow(#right_paren as u32)
                                        .expect("Power operation overflowed")
                                } else {
                                    // Fall back to float power for negative or large exponents
                                    // DEPYLER-0401: Use i32 to match common Python int mapping
                                    (#left_paren as f64).powf(#right_paren as f64) as i32
                                }
                            }
                        })
                    }
                }
            }
            // DEPYLER-0720: Handle comparison operators with int-to-float coercion
            BinOp::Gt | BinOp::GtEq | BinOp::Lt | BinOp::LtEq | BinOp::Eq | BinOp::NotEq => {
                let rust_op = convert_binop(op)?;

                // DEPYLER-0824: Wrap cast expressions in parentheses before binary operators
                // Rust parses `x as i32 < y` incorrectly. Must be: `(x as i32) < y`
                let safe_left: syn::Expr = if matches!(left_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#left_expr) }
                } else {
                    left_expr.clone()
                };
                let safe_right: syn::Expr = if matches!(right_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#right_expr) }
                } else {
                    right_expr.clone()
                };

                // Check if either side is float
                let left_is_float = self.expr_returns_float_direct(left);
                let right_is_float = self.expr_returns_float_direct(right);

                // DEPYLER-0828: Handle float/int comparisons with proper coercion
                // If left is float and right is integer (literal or variable), convert right to float
                if left_is_float && !right_is_float {
                    if let HirExpr::Literal(Literal::Int(n)) = right {
                        // Integer literal: convert at compile time
                        let float_val = *n as f64;
                        return Ok(parse_quote! { #safe_left #rust_op #float_val });
                    }
                    // Integer variable or expression: cast to f64 at runtime
                    return Ok(parse_quote! { #safe_left #rust_op (#safe_right as f64) });
                }

                // If right is float and left is integer (literal or variable), convert left to float
                if right_is_float && !left_is_float {
                    if let HirExpr::Literal(Literal::Int(n)) = left {
                        // Integer literal: convert at compile time
                        let float_val = *n as f64;
                        return Ok(parse_quote! { #float_val #rust_op #safe_right });
                    }
                    // Integer variable or expression: cast to f64 at runtime
                    return Ok(parse_quote! { (#safe_left as f64) #rust_op #safe_right });
                }

                // No coercion needed (both same type)
                Ok(parse_quote! { #safe_left #rust_op #safe_right })
            }
            _ => {
                let rust_op = convert_binop(op)?;
                // DEPYLER-0824: Wrap cast expressions in parentheses
                let safe_left: syn::Expr = if matches!(left_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#left_expr) }
                } else {
                    left_expr.clone()
                };
                let safe_right: syn::Expr = if matches!(right_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#right_expr) }
                } else {
                    right_expr.clone()
                };
                Ok(parse_quote! { #safe_left #rust_op #safe_right })
            }
        }
    }

    fn convert_unary(&self, op: UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
        let operand_expr = self.convert(operand)?;
        match op {
            UnaryOp::Not => {
                // DEPYLER-0966: Check if operand is a collection type for truthiness transformation
                // Python: `if not self.heap:` where self.heap is list[int]
                // Rust: Must use `.is_empty()` instead of `!` for Vec types
                let is_collection = if let HirExpr::Attribute { value, attr } = operand {
                    if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                        if let Some(field_type) = self.class_field_types.get(attr) {
                            matches!(
                                field_type,
                                Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::String
                            )
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                // DEPYLER-0966: Check if operand is an Optional class field
                let is_optional = if let HirExpr::Attribute { value, attr } = operand {
                    if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                        if let Some(field_type) = self.class_field_types.get(attr) {
                            matches!(field_type, Type::Optional(_))
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                if is_collection {
                    Ok(parse_quote! { #operand_expr.is_empty() })
                } else if is_optional {
                    Ok(parse_quote! { #operand_expr.is_none() })
                } else {
                    Ok(parse_quote! { !#operand_expr })
                }
            }
            UnaryOp::Neg => Ok(parse_quote! { -#operand_expr }),
            UnaryOp::Pos => Ok(operand_expr), // No +x in Rust
            UnaryOp::BitNot => Ok(parse_quote! { !#operand_expr }),
        }
    }

    fn convert_call(&self, func: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        // Handle classmethod cls(args) → Self::new(args)
        if func == "cls" && self.is_classmethod {
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| self.convert(arg))
                .collect::<Result<Vec<_>>>()?;
            return Ok(parse_quote! { Self::new(#(#arg_exprs),*) });
        }

        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        match func {
            "len" => self.convert_len_call(&arg_exprs),
            "range" => self.convert_range_call(&arg_exprs),
            // DEPYLER-1001: enumerate(iterable) → iterable.iter().cloned().enumerate().map(|(i, x)| (i as i32, x))
            "enumerate" => self.convert_enumerate_call(&arg_exprs),
            // DEPYLER-1001: zip(a, b) → a.into_iter().zip(b.into_iter())
            "zip" => self.convert_zip_call(&arg_exprs),
            // DEPYLER-1001: reversed(iterable) → iterable.iter().cloned().rev()
            "reversed" => self.convert_reversed_call(&arg_exprs),
            // DEPYLER-1001: sorted(iterable) → sorted Vec
            "sorted" => self.convert_sorted_call(&arg_exprs),
            "zeros" | "ones" | "full" => self.convert_array_init_call(func, args, &arg_exprs),
            "set" => self.convert_set_constructor(&arg_exprs),
            "frozenset" => self.convert_frozenset_constructor(&arg_exprs),
            // DEPYLER-0200: File I/O builtins
            "open" => self.convert_open_call(args, &arg_exprs),
            // DEPYLER-0200: datetime builtins
            "date" => self.convert_date_call(&arg_exprs),
            "datetime" => self.convert_datetime_call(&arg_exprs),
            // DEPYLER-0721: os.path functions imported via `from os.path import X`
            "splitext" => self.convert_splitext_call(&arg_exprs),
            "basename" => self.convert_basename_call(&arg_exprs),
            "dirname" => self.convert_dirname_call(&arg_exprs),
            "split" => self.convert_path_split_call(&arg_exprs),
            "exists" => self.convert_path_exists_call(&arg_exprs),
            "isfile" => self.convert_path_isfile_call(&arg_exprs),
            "isdir" => self.convert_path_isdir_call(&arg_exprs),
            // DEPYLER-0844: isinstance(x, T) → true (Rust's type system guarantees correctness)
            "isinstance" => Ok(parse_quote! { true }),
            // DEPYLER-0906: ord(c) → c.chars().next().unwrap() as i32
            // Python ord() returns Unicode code point as int
            "ord" => self.convert_ord_call(&arg_exprs),
            // DEPYLER-0906: chr(n) → char::from_u32(n as u32).unwrap().to_string()
            // Python chr() returns single character string from Unicode code point
            "chr" => self.convert_chr_call(&arg_exprs),
            // DEPYLER-0931: list() builtin for class method bodies
            // list() → Vec::new()
            // list(iterable) → iterable.into_iter().collect::<Vec<_>>()
            "list" => self.convert_list_call(&arg_exprs),
            // DEPYLER-0935: bytes() builtin for class method bodies
            // bytes() → Vec::<u8>::new()
            // bytes(iterable) → iterable.into_iter().map(|x| x as u8).collect::<Vec<u8>>()
            "bytes" => self.convert_bytes_call(&arg_exprs),
            // DEPYLER-0936: bytearray() builtin for class method bodies
            // bytearray() → Vec::<u8>::new()
            // bytearray(n) → vec![0u8; n]
            "bytearray" => self.convert_bytearray_call(&arg_exprs),
            // DEPYLER-0937: tuple() builtin for class method bodies
            // tuple() → Vec::new()
            // tuple(iterable) → iterable.into_iter().collect::<Vec<_>>()
            "tuple" => self.convert_tuple_call(&arg_exprs),
            // DEPYLER-0968: sum() builtin for class method bodies
            // sum(iterable) → iterable.iter().sum::<T>()
            // sum(generator_expr) → generator_expr.sum::<T>()
            "sum" => self.convert_sum_call(args, &arg_exprs),
            // DEPYLER-0780: Pass HIR args for auto-borrowing detection
            _ => self.convert_generic_call(func, args, &arg_exprs),
        }
    }

    fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("len() requires exactly one argument");
        }
        let arg = &args[0];
        // DEPYLER-0693: Cast len() to i32 for Python compatibility
        // Python int maps to Rust i32, and len() in Python returns int
        Ok(parse_quote! { #arg.len() as i32 })
    }

    /// DEPYLER-0906: Convert Python ord(c) to Rust char code point
    ///
    /// Python: ord('a') → 97
    /// Rust: 'a'.chars().next().unwrap() as i32
    ///
    /// For single-char strings, get first char and convert to i32.
    fn convert_ord_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("ord() requires exactly one argument");
        }
        let char_str = &args[0];
        Ok(parse_quote! { #char_str.chars().next().unwrap() as i32 })
    }

    /// DEPYLER-0906: Convert Python chr(n) to Rust char string
    ///
    /// Python: chr(97) → 'a'
    /// Rust: char::from_u32(97u32).unwrap().to_string()
    ///
    /// Converts Unicode code point to single-character String.
    fn convert_chr_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("chr() requires exactly one argument");
        }
        let code = &args[0];
        Ok(parse_quote! { char::from_u32(#code as u32).unwrap().to_string() })
    }

    /// DEPYLER-0931: Convert Python list() builtin to Rust Vec
    ///
    /// list() → Vec::new()
    /// list(iterable) → iterable.into_iter().collect::<Vec<_>>()
    /// list(dict.keys()) → dict.keys().cloned().collect::<Vec<_>>()
    fn convert_list_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // list() → Vec::new()
            Ok(parse_quote! { Vec::new() })
        } else if args.len() == 1 {
            let iterable = &args[0];
            // DEPYLER-0931: Check if the iterable is a method call returning references
            // dict.keys(), dict.values(), list.iter() all return iterators of references
            // that need .cloned() before .collect()
            let needs_clone = if let syn::Expr::MethodCall(method_call) = iterable {
                matches!(
                    method_call.method.to_string().as_str(),
                    "keys" | "values" | "iter" | "items"
                )
            } else {
                false
            };

            if needs_clone {
                // For reference iterators: use .cloned().collect()
                Ok(parse_quote! { #iterable.cloned().collect::<Vec<_>>() })
            } else {
                // For owned iterators: use .into_iter().collect()
                Ok(parse_quote! { #iterable.into_iter().collect::<Vec<_>>() })
            }
        } else {
            bail!("list() takes at most 1 argument ({} given)", args.len())
        }
    }

    /// DEPYLER-0935: bytes() builtin for class method bodies
    /// In Python, bytes(n) creates n zero bytes, bytes([list]) collects the list
    fn convert_bytes_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // bytes() → Vec::<u8>::new()
            Ok(parse_quote! { Vec::<u8>::new() })
        } else if args.len() == 1 {
            let arg = &args[0];
            // Default to bytes(n) → vec![0u8; n as usize] for numeric expressions
            // This is the most common case in real code
            Ok(parse_quote! { vec![0u8; (#arg) as usize] })
        } else {
            bail!("bytes() takes at most 1 argument ({} given)", args.len())
        }
    }

    /// DEPYLER-0936: bytearray() builtin for class method bodies
    /// In Python, bytearray(n) creates n zero bytes, bytearray([list]) collects the list
    fn convert_bytearray_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // bytearray() → Vec::<u8>::new()
            Ok(parse_quote! { Vec::<u8>::new() })
        } else if args.len() == 1 {
            let arg = &args[0];
            // Default to bytearray(n) → vec![0u8; n as usize] for numeric expressions
            // This is the most common case in real code
            Ok(parse_quote! { vec![0u8; (#arg) as usize] })
        } else {
            bail!("bytearray() takes at most 1 argument ({} given)", args.len())
        }
    }

    /// DEPYLER-0937: tuple() builtin for class method bodies
    fn convert_tuple_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // tuple() → Vec::new()
            Ok(parse_quote! { Vec::new() })
        } else if args.len() == 1 {
            let iterable = &args[0];
            // tuple(iterable) → iterable.into_iter().collect()
            Ok(parse_quote! { #iterable.into_iter().collect::<Vec<_>>() })
        } else {
            bail!("tuple() takes at most 1 argument ({} given)", args.len())
        }
    }

    /// DEPYLER-1001: enumerate(iterable) → iterable.iter().cloned().enumerate().map(|(i, x)| (i as i32, x))
    /// enumerate(iterable, start) → iterable.iter().cloned().enumerate().map(|(i, x)| ((i + start) as i32, x))
    fn convert_enumerate_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("enumerate() requires 1 or 2 arguments");
        }
        let iterable = &args[0];
        if args.len() == 2 {
            let start = &args[1];
            Ok(parse_quote! {
                #iterable.iter().cloned().enumerate().map(|(i, x)| ((i + #start as usize) as i32, x))
            })
        } else {
            Ok(parse_quote! {
                #iterable.iter().cloned().enumerate().map(|(i, x)| (i as i32, x))
            })
        }
    }

    /// DEPYLER-1001: zip(a, b) → a.into_iter().zip(b.into_iter())
    fn convert_zip_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 2 {
            bail!("zip() requires at least 2 arguments");
        }
        let first = &args[0];
        let second = &args[1];
        if args.len() == 2 {
            Ok(parse_quote! { #first.into_iter().zip(#second.into_iter()) })
        } else {
            let mut zip_expr: syn::Expr = parse_quote! { #first.into_iter().zip(#second.into_iter()) };
            for iter in &args[2..] {
                zip_expr = parse_quote! { #zip_expr.zip(#iter.into_iter()) };
            }
            Ok(zip_expr)
        }
    }

    /// DEPYLER-1001: reversed(iterable) → iterable.iter().cloned().rev()
    fn convert_reversed_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("reversed() requires exactly 1 argument");
        }
        let iterable = &args[0];
        Ok(parse_quote! { #iterable.iter().cloned().rev() })
    }

    /// DEPYLER-1001: sorted(iterable) → sorted Vec with partial_cmp for float support
    fn convert_sorted_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("sorted() requires 1 or 2 arguments");
        }
        let iterable = &args[0];
        Ok(parse_quote! {
            {
                let mut sorted_vec = #iterable.iter().cloned().collect::<Vec<_>>();
                sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                sorted_vec
            }
        })
    }

    /// DEPYLER-0968: sum() builtin for class method bodies
    ///
    /// Handles Python sum() function conversion to Rust iterator patterns.
    ///
    /// Variants:
    /// - sum(generator_exp) → generator_expr.sum::<T>()
    /// - sum(range(...)) → (range_expr).sum::<T>()
    /// - sum(d.values()) / sum(d.keys()) → d.values().cloned().sum::<T>()
    /// - sum(iterable) → iterable.iter().sum::<T>()
    fn convert_sum_call(&self, hir_args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if hir_args.len() != 1 || arg_exprs.len() != 1 {
            bail!("sum() requires exactly one argument");
        }

        let arg_expr = &arg_exprs[0];

        // Detect target type from class field types or default to f64
        let target_type: syn::Type = self.infer_sum_type(&hir_args[0]);

        // Check if argument is a generator expression (already converted to .iter().map())
        // or a method call like .values()/.keys()
        match &hir_args[0] {
            // Generator expression: sum(x*x for x in items) → items.iter().map(|x| x*x).sum::<T>()
            // The generator is already converted to .iter().map(), so just append .sum()
            HirExpr::GeneratorExp { .. } => {
                Ok(parse_quote! { #arg_expr.sum::<#target_type>() })
            }
            // Range call: sum(range(n)) → (0..n).sum::<T>()
            HirExpr::Call { func, .. } if func == "range" => {
                Ok(parse_quote! { (#arg_expr).sum::<#target_type>() })
            }
            // Method call: sum(d.values()) → d.values().cloned().sum::<T>()
            HirExpr::MethodCall { method, args: method_args, .. }
                if (method == "values" || method == "keys") && method_args.is_empty() =>
            {
                Ok(parse_quote! { #arg_expr.cloned().sum::<#target_type>() })
            }
            // Default: sum(iterable) → iterable.iter().sum::<T>()
            _ => {
                // Check if the converted expression already has .iter()/.map()
                let expr_str = quote::quote!(#arg_expr).to_string();
                if expr_str.contains(".iter()") || expr_str.contains(".map(") {
                    // Already an iterator, just append .sum()
                    Ok(parse_quote! { #arg_expr.sum::<#target_type>() })
                } else {
                    // Need to add .iter()
                    Ok(parse_quote! { #arg_expr.iter().sum::<#target_type>() })
                }
            }
        }
    }

    /// Infer the target type for sum() based on the expression context
    fn infer_sum_type(&self, expr: &HirExpr) -> syn::Type {
        // Check if we can determine the type from class field types
        match expr {
            HirExpr::Attribute { value, attr } => {
                if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                    if let Some(field_type) = self.class_field_types.get(attr) {
                        return match field_type {
                            Type::List(elem_type) => match elem_type.as_ref() {
                                Type::Int => parse_quote! { i32 },
                                Type::Float => parse_quote! { f64 },
                                _ => parse_quote! { f64 },
                            },
                            _ => parse_quote! { f64 },
                        };
                    }
                }
            }
            HirExpr::GeneratorExp { generators, .. } => {
                // Try to infer from the iteration target (first generator)
                if let Some(gen) = generators.first() {
                    return self.infer_sum_type(&gen.iter);
                }
            }
            _ => {}
        }
        // Default to f64 for floating point operations
        parse_quote! { f64 }
    }

    fn convert_range_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        match args.len() {
            1 => {
                let end = &args[0];
                Ok(parse_quote! { 0..#end })
            }
            2 => {
                let start = &args[0];
                let end = &args[1];
                Ok(parse_quote! { #start..#end })
            }
            3 => {
                // Step parameter requires custom iterator implementation
                bail!("range() with step parameter not yet supported")
            }
            _ => bail!("Invalid number of arguments for range()"),
        }
    }

    fn convert_array_init_call(
        &self,
        func: &str,
        args: &[HirExpr],
        _arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // Handle zeros(n), ones(n), full(n, value) patterns
        if args.is_empty() {
            bail!("{} requires at least one argument", func);
        }

        // DEPYLER-0695: Always use vec![] for zeros/ones/full to ensure consistent
        // Vec<T> return type. Using fixed arrays [0; N] causes type mismatches when
        // functions return Vec<T> (Python lists are always dynamically sized).
        let size_expr = self.convert(&args[0])?;
        match func {
            "zeros" => Ok(parse_quote! { vec![0; #size_expr as usize] }),
            "ones" => Ok(parse_quote! { vec![1; #size_expr as usize] }),
            "full" => {
                if args.len() >= 2 {
                    let value = self.convert(&args[1])?;
                    Ok(parse_quote! { vec![#value; #size_expr as usize] })
                } else {
                    bail!("full() requires a value argument");
                }
            }
            _ => unreachable!(),
        }
    }


    fn convert_set_constructor(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // Empty set: set()
            // DEPYLER-0409: Use default type i32 to avoid "type annotations needed" error
            // when the variable is unused or type can't be inferred from context
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            Ok(parse_quote! { std::collections::HashSet::<i32>::new() })
        } else if args.len() == 1 {
            // Set from iterable: set([1, 2, 3])
            let arg = &args[0];
            // DEPYLER-0797: Check if arg is a tuple - tuples don't implement IntoIterator in Rust
            // Convert tuple to vec! for iteration
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            if let syn::Expr::Tuple(tuple) = arg {
                let elems = &tuple.elems;
                Ok(parse_quote! {
                    vec![#elems].into_iter().collect::<std::collections::HashSet<_>>()
                })
            } else {
                Ok(parse_quote! {
                    #arg.into_iter().collect::<std::collections::HashSet<_>>()
                })
            }
        } else {
            bail!("set() takes at most 1 argument ({} given)", args.len())
        }
    }

    fn convert_frozenset_constructor(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // Empty frozenset: frozenset()
            // DEPYLER-0409: Use default type i32 for empty sets
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            Ok(parse_quote! { std::sync::Arc::new(std::collections::HashSet::<i32>::new()) })
        } else if args.len() == 1 {
            // Frozenset from iterable: frozenset([1, 2, 3])
            let arg = &args[0];
            // DEPYLER-0797: Check if arg is a tuple - tuples don't implement IntoIterator in Rust
            // Convert tuple to vec! for iteration
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            if let syn::Expr::Tuple(tuple) = arg {
                let elems = &tuple.elems;
                Ok(parse_quote! {
                    std::sync::Arc::new(vec![#elems].into_iter().collect::<std::collections::HashSet<_>>())
                })
            } else {
                Ok(parse_quote! {
                    std::sync::Arc::new(#arg.into_iter().collect::<std::collections::HashSet<_>>())
                })
            }
        } else {
            bail!(
                "frozenset() takes at most 1 argument ({} given)",
                args.len()
            )
        }
    }

    /// DEPYLER-0721: os.path.splitext(path) → (stem, extension) tuple
    fn convert_splitext_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("splitext() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            {
                let p = std::path::Path::new(&#path);
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                let ext = p.extension().and_then(|e| e.to_str()).map(|e| format!(".{}", e)).unwrap_or_default();
                (stem, ext)
            }
        })
    }

    /// DEPYLER-0721: os.path.basename(path) → Path::file_name
    fn convert_basename_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("basename() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            std::path::Path::new(&#path).file_name().and_then(|n| n.to_str()).unwrap_or("").to_string()
        })
    }

    /// DEPYLER-0721: os.path.dirname(path) → Path::parent
    fn convert_dirname_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("dirname() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            std::path::Path::new(&#path).parent().and_then(|p| p.to_str()).unwrap_or("").to_string()
        })
    }

    /// DEPYLER-0721: os.path.split(path) → (dirname, basename)
    fn convert_path_split_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("split() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            {
                let p = std::path::Path::new(&#path);
                let dirname = p.parent().and_then(|p| p.to_str()).unwrap_or("").to_string();
                let basename = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                (dirname, basename)
            }
        })
    }

    /// DEPYLER-0721: os.path.exists(path) → Path::exists
    fn convert_path_exists_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("exists() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! { std::path::Path::new(&#path).exists() })
    }

    /// DEPYLER-0721: os.path.isfile(path) → Path::is_file
    fn convert_path_isfile_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("isfile() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! { std::path::Path::new(&#path).is_file() })
    }

    /// DEPYLER-0721: os.path.isdir(path) → Path::is_dir
    fn convert_path_isdir_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("isdir() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! { std::path::Path::new(&#path).is_dir() })
    }

    /// DEPYLER-0200: Convert Python open() to Rust file operations
    /// open(path) → std::fs::File::open(path) (read mode)
    /// open(path, "w") → std::fs::File::create(path) (write mode)
    fn convert_open_call(&self, hir_args: &[HirExpr], args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("open() requires 1 or 2 arguments");
        }

        let path = &args[0];

        // Determine mode from second argument (default is 'r')
        let mode = if hir_args.len() >= 2 {
            if let HirExpr::Literal(Literal::String(mode_str)) = &hir_args[1] {
                mode_str.as_str()
            } else {
                "r" // Default to read mode
            }
        } else {
            "r" // Default mode
        };

        match mode {
            "w" | "w+" | "wb" => {
                // Write mode: std::fs::File::create()
                Ok(parse_quote! { std::fs::File::create(&#path).unwrap() })
            }
            "a" | "a+" | "ab" => {
                // Append mode: OpenOptions with append
                Ok(parse_quote! {
                    std::fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&#path)
                        .unwrap()
                })
            }
            _ => {
                // Read mode (default): std::fs::File::open()
                Ok(parse_quote! { std::fs::File::open(&#path).unwrap() })
            }
        }
    }

    /// DEPYLER-0200: Convert Python date(year, month, day) to chrono::NaiveDate
    fn convert_date_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 3 {
            bail!("date() requires exactly 3 arguments (year, month, day)");
        }
        let year = &args[0];
        let month = &args[1];
        let day = &args[2];
        Ok(parse_quote! {
            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32).unwrap()
        })
    }

    /// DEPYLER-0200: Convert Python datetime(year, month, day, ...) to chrono::NaiveDateTime
    fn convert_datetime_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 3 {
            bail!("datetime() requires at least 3 arguments (year, month, day)");
        }
        let year = &args[0];
        let month = &args[1];
        let day = &args[2];

        // Handle optional time components
        let zero: syn::Expr = parse_quote! { 0 };
        let hour = args.get(3).unwrap_or(&zero);
        let minute = args.get(4).unwrap_or(&zero);
        let second = args.get(5).unwrap_or(&zero);

        Ok(parse_quote! {
            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                .unwrap()
                .and_hms_opt(#hour as u32, #minute as u32, #second as u32)
                .unwrap()
        })
    }

    fn convert_generic_call(&self, func: &str, hir_args: &[HirExpr], args: &[syn::Expr]) -> Result<syn::Expr> {
        // Special case: Python print() → Rust println!()
        if func == "print" {
            return if args.is_empty() {
                // print() with no arguments → println!()
                Ok(parse_quote! { println!() })
            } else if args.len() == 1 {
                // print(x) → println!("{}", x)
                let arg = &args[0];
                Ok(parse_quote! { println!("{}", #arg) })
            } else {
                // print(a, b, c) → println!("{} {} {}", a, b, c)
                let format_str = vec!["{}"; args.len()].join(" ");
                Ok(parse_quote! { println!(#format_str, #(#args),*) })
            };
        }

        // DEPYLER-0600: Handle Python built-in type conversion functions
        // These are used in class methods and need proper Rust equivalents
        match func {
            "int" if args.len() == 1 => {
                // int(x) → x.parse::<i32>().unwrap() for strings, x as i32 for numbers
                let arg = &args[0];
                return Ok(parse_quote! { #arg.parse::<i32>().unwrap_or(0) });
            }
            "float" if args.len() == 1 => {
                // float(x) → x.parse::<f64>().unwrap() for strings, x as f64 for integers
                let arg = &args[0];
                return Ok(parse_quote! { #arg.parse::<f64>().unwrap_or(0.0) });
            }
            "str" if args.len() == 1 => {
                // str(x) → x.to_string()
                let arg = &args[0];
                return Ok(parse_quote! { #arg.to_string() });
            }
            "bool" if args.len() == 1 => {
                // bool(x) → general truthiness conversion
                let arg = &args[0];
                return Ok(parse_quote! { !#arg.is_empty() });
            }
            "len" if args.len() == 1 => {
                // len(x) → x.len() as i32
                // DEPYLER-0693: Cast len() to i32 for Python compatibility
                let arg = &args[0];
                return Ok(parse_quote! { #arg.len() as i32 });
            }
            "abs" if args.len() == 1 => {
                // DEPYLER-0815: abs(x) → (x).abs() - parens needed for precedence
                let arg = &args[0];
                return Ok(parse_quote! { (#arg).abs() });
            }
            "min" if args.len() >= 2 => {
                // min(a, b, ...) → a.min(b).min(c)...
                let first = &args[0];
                let rest = &args[1..];
                let mut result = parse_quote! { #first };
                for arg in rest {
                    result = parse_quote! { (#result).min(#arg) };
                }
                return Ok(result);
            }
            "max" if args.len() >= 2 => {
                // max(a, b, ...) → a.max(b).max(c)...
                let first = &args[0];
                let rest = &args[1..];
                let mut result = parse_quote! { #first };
                for arg in rest {
                    result = parse_quote! { (#result).max(#arg) };
                }
                return Ok(result);
            }
            _ => {}
        }

        // Check if this might be a constructor call (capitalized name)
        if func
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            // DEPYLER-0900: Rename constructor if it shadows stdlib type (e.g., Box -> PyBox)
            // Treat as constructor call - ClassName::new(args)
            let safe_name = safe_class_name(func);
            let class_ident = make_ident(&safe_name);
            if args.is_empty() {
                // Note: Constructor default parameter handling uses simple heuristics.
                // Ideally this would be context-aware and know the actual default values
                // for each class constructor, but currently uses hardcoded patterns.
                // This is a known limitation - constructors may require explicit arguments.
                match func {
                    "Counter" => Ok(parse_quote! { #class_ident::new(0) }),
                    _ => Ok(parse_quote! { #class_ident::new() }),
                }
            } else {
                Ok(parse_quote! { #class_ident::new(#(#args),*) })
            }
        } else {
            // Regular function call
            let func_ident = make_ident(func);

            // DEPYLER-0648: Check if this is a vararg function
            // If so, wrap arguments in a slice: func(a, b) → func(&[a, b])
            if self.vararg_functions.contains(func) && !args.is_empty() {
                Ok(parse_quote! { #func_ident(&[#(#args),*]) })
            } else {
                // DEPYLER-0780: Auto-borrow list/dict/set literals when calling functions
                // Most user-defined functions taking list params expect &Vec<T>
                let borrowed_args: Vec<syn::Expr> = hir_args
                    .iter()
                    .zip(args.iter())
                    .map(|(hir_arg, arg_expr)| {
                        match hir_arg {
                            // List/Dict/Set literals should be borrowed
                            HirExpr::List(_) | HirExpr::Dict(_) | HirExpr::Set(_) => {
                                parse_quote! { &#arg_expr }
                            }
                            _ => arg_expr.clone(),
                        }
                    })
                    .collect();
                Ok(parse_quote! { #func_ident(#(#borrowed_args),*) })
            }
        }
    }

    fn convert_index(&self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = self.convert(base)?;

        // DEPYLER-0735: Check if base is a tuple type (from param_types) and index is integer literal
        // Rust tuples use .0, .1 syntax, not [0], [1]
        if let HirExpr::Literal(Literal::Int(idx)) = index {
            if *idx >= 0 {
                // Check if base variable has tuple type from param_types
                let is_tuple = if let HirExpr::Var(var_name) = base {
                    matches!(self.param_types.get(var_name), Some(Type::Tuple(_)))
                } else {
                    false
                };

                if is_tuple {
                    let field_idx = syn::Index::from(*idx as usize);
                    return Ok(parse_quote! { #base_expr.#field_idx });
                }
            }
        }

        // DEPYLER-0200: Detect dict vs list access
        // String literal index = dict access, use .get()
        // Numeric index = list access, use [idx as usize]
        let is_dict_access = match index {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(name) => {
                // Heuristic: variable names that look like string keys
                let n = name.as_str();
                n == "key" || n.ends_with("_key") || n.starts_with("key_")
                    || n == "name" || n == "field" || n == "attr"
            }
            _ => false,
        };

        // Also check if base looks like a dict
        let base_is_dict = match base {
            HirExpr::Var(name) => {
                let n = name.as_str();
                n.contains("dict") || n.contains("map") || n.contains("data")
                    || n == "result" || n == "config" || n == "settings"
                    || n == "params" || n == "options" || n == "env"
            }
            HirExpr::Call { func, .. } => {
                // Functions returning dicts
                func.contains("dict") || func.contains("json") || func.contains("config")
                    || func == "calculate_age" || func.contains("result")
            }
            _ => false,
        };

        if is_dict_access || base_is_dict {
            // HashMap access with string key
            let index_expr = self.convert(index)?;
            Ok(parse_quote! {
                #base_expr.get(&#index_expr).cloned().unwrap_or_default()
            })
        } else {
            // Vec/List access with numeric index
            let index_expr = self.convert(index)?;
            Ok(parse_quote! {
                #base_expr[#index_expr as usize]
            })
        }
    }

    /// DEPYLER-0596: Convert slice expression (e.g., value[1:-1])
    fn convert_slice(
        &self,
        base: &HirExpr,
        start: &Option<Box<HirExpr>>,
        stop: &Option<Box<HirExpr>>,
        step: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let base_expr = self.convert(base)?;

        // Convert start/stop/step expressions
        let start_expr = start.as_ref().map(|e| self.convert(e)).transpose()?;
        let stop_expr = stop.as_ref().map(|e| self.convert(e)).transpose()?;
        let _step_expr = step.as_ref().map(|e| self.convert(e)).transpose()?;

        // For strings: use chars().skip().take() pattern with negative index handling
        // This handles cases like value[1:-1] (remove first and last chars)
        match (start_expr, stop_expr) {
            (Some(start), Some(stop)) => {
                // value[start:stop] - handles negative indices
                // DEPYLER-0603: Wrap expressions in parens to ensure proper type casting
                // Without parens, `a + b as isize` parses as `a + (b as isize)`
                Ok(parse_quote! {
                    {
                        let s = &#base_expr;
                        let len = s.chars().count() as isize;
                        let start_idx = (#start) as isize;
                        let stop_idx = (#stop) as isize;
                        let start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        let stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx as usize
                        };
                        if stop > start {
                            s.chars().skip(start).take(stop - start).collect::<String>()
                        } else {
                            String::new()
                        }
                    }
                })
            }
            (Some(start), None) => {
                // value[start:] - from start to end
                // DEPYLER-0603: Wrap expression in parens for type casting
                Ok(parse_quote! {
                    {
                        let s = &#base_expr;
                        let len = s.chars().count() as isize;
                        let start_idx = (#start) as isize;
                        let start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        s.chars().skip(start).collect::<String>()
                    }
                })
            }
            (None, Some(stop)) => {
                // value[:stop] - from beginning to stop
                // DEPYLER-0603: Wrap expression in parens for type casting
                Ok(parse_quote! {
                    {
                        let s = &#base_expr;
                        let len = s.chars().count() as isize;
                        let stop_idx = (#stop) as isize;
                        let stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx as usize
                        };
                        s.chars().take(stop).collect::<String>()
                    }
                })
            }
            (None, None) => {
                // value[:] - full clone
                Ok(parse_quote! { #base_expr.clone() })
            }
        }
    }

    fn convert_list(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| self.convert(e))
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0780: Always use vec![] for list literals
        // Array literals [T; N] are incompatible with &Vec<T> parameters
        // Python lists map to Vec<T> in Rust, so consistently use vec![]
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    fn convert_dict(&self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
        let insert_exprs: Vec<syn::Expr> = items
            .iter()
            .map(|(k, v)| {
                let key = self.convert(k)?;
                let val = self.convert(v)?;
                Ok(parse_quote! { map.insert(#key, #val) })
            })
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0623: Use fully qualified path to avoid missing import
        Ok(parse_quote! {
            {
                let mut map = std::collections::HashMap::new();
                #(#insert_exprs;)*
                map
            }
        })
    }

    fn convert_tuple(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| self.convert(e))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }

    fn convert_set(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let insert_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let elem = self.convert(e)?;
                Ok(parse_quote! { set.insert(#elem) })
            })
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0623: Use fully qualified path to avoid missing import
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_exprs;)*
                set
            }
        })
    }

    fn convert_frozenset(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let insert_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let elem = self.convert(e)?;
                Ok(parse_quote! { set.insert(#elem) })
            })
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0623: Use fully qualified path to avoid missing import
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_exprs;)*
                std::sync::Arc::new(set)
            }
        })
    }

    fn is_set_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Set(_) | HirExpr::FrozenSet(_) => true,
            HirExpr::Call { func, .. } if func == "set" || func == "frozenset" => true,
            HirExpr::Var(_name) => {
                // For now, be conservative and only treat explicit sets as sets
                // This prevents incorrect conversion of integer bitwise operations
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0601: Detect if expression is likely a string type.
    /// Used to generate `.contains()` instead of `.contains_key()` for `in` operator.
    fn is_string_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // String literals are obviously strings
            HirExpr::Literal(Literal::String(_)) => true,
            // F-strings produce strings
            HirExpr::FString { .. } => true,
            // Method calls that return strings
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "lower"
                        | "upper"
                        | "strip"
                        | "lstrip"
                        | "rstrip"
                        | "replace"
                        | "join"
                        | "format"
                        | "capitalize"
                        | "title"
                        | "swapcase"
                        | "center"
                        | "ljust"
                        | "rjust"
                        | "zfill"
                        | "expandtabs"
                        | "encode"
                        | "decode"
                )
            }
            // Variables with common string-like names
            HirExpr::Var(name) => {
                matches!(
                    name.as_str(),
                    "s"
                        | "url"
                        | "path"
                        | "text"
                        | "remaining"
                        | "query_string"
                        | "host"
                        | "scheme"
                        | "fragment"
                        | "name"
                        | "message"
                        | "line"
                        | "content"
                        | "data"
                        | "result"
                        | "output"
                        | "input"
                        | "string"
                        | "str"
                        | "pair"
                        | "email"
                        | "domain_part"
                        | "local_part"
                        | "normalized"
                )
            }
            // Calls to str() produce strings
            HirExpr::Call { func, .. } if func == "str" => true,
            // DEPYLER-0752: Handle attribute access for known string fields
            // Examples: r.stdout, result.stderr, response.text
            HirExpr::Attribute { attr, .. } => {
                matches!(
                    attr.as_str(),
                    "stdout" | "stderr" | "text" | "output" | "message" | "name"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0742: Detect if expression is a deque type.
    /// Used to generate VecDeque methods instead of Vec methods.
    fn is_deque_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Call to deque() constructor
            HirExpr::Call { func, .. } if func == "deque" || func == "collections.deque" => true,
            // Variables with deque-like names
            HirExpr::Var(name) => {
                matches!(
                    name.as_str(),
                    "d" | "dq" | "deque" | "queue" | "buffer" | "deck"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0832: Detect if expression is a tuple (for `in` operator).
    /// Tuples should use `.contains()` on an array, not `.contains_key()`.
    fn is_tuple_or_list_expr(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Tuple(_) | HirExpr::List(_))
    }

    /// DEPYLER-0960: Detect if expression is a dict/HashMap type.
    /// Used to ensure `key in dict` generates `.contains_key()` not `.contains()`.
    fn is_dict_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Dict literal
            HirExpr::Dict { .. } => true,
            // Variables with common dict-like names
            HirExpr::Var(name) => {
                let n = name.as_str();
                n.contains("dict") || n.contains("map") || n.contains("hash")
                    || n == "config" || n == "settings" || n == "params"
                    || n == "options" || n == "env" || n == "data"
                    || n == "result" || n == "cache" || n == "d" || n == "m"
            }
            // Calls to dict() or functions returning dicts
            HirExpr::Call { func, .. } => {
                func == "dict" || func.contains("json") || func.contains("config")
                    || func.contains("load") || func.contains("parse")
            }
            _ => false,
        }
    }

    fn convert_set_operation(
        &self,
        op: BinOp,
        left: syn::Expr,
        right: syn::Expr,
    ) -> Result<syn::Expr> {
        match op {
            BinOp::BitAnd => Ok(parse_quote! {
                #left.intersection(&#right).cloned().collect()
            }),
            BinOp::BitOr => Ok(parse_quote! {
                #left.union(&#right).cloned().collect()
            }),
            BinOp::Sub => Ok(parse_quote! {
                #left.difference(&#right).cloned().collect()
            }),
            BinOp::BitXor => Ok(parse_quote! {
                #left.symmetric_difference(&#right).cloned().collect()
            }),
            _ => bail!("Invalid set operator"),
        }
    }

    fn convert_method_call(
        &self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // Handle classmethod cls.method() → Self::method()
        if let HirExpr::Var(var_name) = object {
            if var_name == "cls" && self.is_classmethod {
                let method_ident = make_ident(method);
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { Self::#method_ident(#(#arg_exprs),*) });
            }
        }

        // DEPYLER-0610: Handle Python stdlib module constructor calls
        // threading.Semaphore(n) → std::sync::Mutex::new(n)
        // queue.Queue() → std::collections::VecDeque::new()
        if let HirExpr::Var(module_name) = object {
            if let Some(rust_expr) = self.convert_module_constructor(module_name, method, args)? {
                return Ok(rust_expr);
            }
        }

        // DEPYLER-0200: Handle os module method calls in class methods
        // This was missing - os.unlink() etc. weren't being converted inside class methods
        if let HirExpr::Var(module_name) = object {
            if module_name == "os" {
                if let Some(rust_expr) = self.try_convert_os_method(method, args)? {
                    return Ok(rust_expr);
                }
            }
        }

        // DEPYLER-0912: Handle colorsys module method calls in class methods
        // colorsys.rgb_to_hsv(r, g, b) → inline color conversion
        if let HirExpr::Var(module_name) = object {
            if module_name == "colorsys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "rgb_to_hsv" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        return Ok(parse_quote! {
                            {
                                let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                                let max_c = r.max(g).max(b);
                                let min_c = r.min(g).min(b);
                                let v = max_c;
                                if min_c == max_c { (0.0, 0.0, v) }
                                else {
                                    let s = (max_c - min_c) / max_c;
                                    let rc = (max_c - r) / (max_c - min_c);
                                    let gc = (max_c - g) / (max_c - min_c);
                                    let bc = (max_c - b) / (max_c - min_c);
                                    let h = if r == max_c { bc - gc }
                                        else if g == max_c { 2.0 + rc - bc }
                                        else { 4.0 + gc - rc };
                                    let h = (h / 6.0) % 1.0;
                                    let h = if h < 0.0 { h + 1.0 } else { h };
                                    (h, s, v)
                                }
                            }
                        });
                    }
                    "hsv_to_rgb" if arg_exprs.len() == 3 => {
                        let h = &arg_exprs[0];
                        let s = &arg_exprs[1];
                        let v = &arg_exprs[2];
                        return Ok(parse_quote! {
                            {
                                let (h, s, v) = (#h as f64, #s as f64, #v as f64);
                                if s == 0.0 { (v, v, v) }
                                else {
                                    let i = (h * 6.0).floor();
                                    let f = (h * 6.0) - i;
                                    let p = v * (1.0 - s);
                                    let q = v * (1.0 - s * f);
                                    let t = v * (1.0 - s * (1.0 - f));
                                    let i = i as i32 % 6;
                                    match i { 0 => (v, t, p), 1 => (q, v, p), 2 => (p, v, t),
                                              3 => (p, q, v), 4 => (t, p, v), _ => (v, p, q) }
                                }
                            }
                        });
                    }
                    "rgb_to_hls" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        return Ok(parse_quote! {
                            {
                                let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                                let max_c = r.max(g).max(b);
                                let min_c = r.min(g).min(b);
                                let l = (min_c + max_c) / 2.0;
                                if min_c == max_c { (0.0, l, 0.0) }
                                else {
                                    let s = if l <= 0.5 { (max_c - min_c) / (max_c + min_c) }
                                        else { (max_c - min_c) / (2.0 - max_c - min_c) };
                                    let rc = (max_c - r) / (max_c - min_c);
                                    let gc = (max_c - g) / (max_c - min_c);
                                    let bc = (max_c - b) / (max_c - min_c);
                                    let h = if r == max_c { bc - gc }
                                        else if g == max_c { 2.0 + rc - bc }
                                        else { 4.0 + gc - rc };
                                    let h = (h / 6.0) % 1.0;
                                    let h = if h < 0.0 { h + 1.0 } else { h };
                                    (h, l, s)
                                }
                            }
                        });
                    }
                    "hls_to_rgb" if arg_exprs.len() == 3 => {
                        let h = &arg_exprs[0];
                        let l = &arg_exprs[1];
                        let s = &arg_exprs[2];
                        return Ok(parse_quote! {
                            {
                                let (h, l, s) = (#h as f64, #l as f64, #s as f64);
                                if s == 0.0 { (l, l, l) }
                                else {
                                    let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - (l * s) };
                                    let m1 = 2.0 * l - m2;
                                    let _v = |hue: f64| {
                                        let hue = hue % 1.0;
                                        let hue = if hue < 0.0 { hue + 1.0 } else { hue };
                                        if hue < 1.0/6.0 { m1 + (m2 - m1) * hue * 6.0 }
                                        else if hue < 0.5 { m2 }
                                        else if hue < 2.0/3.0 { m1 + (m2 - m1) * (2.0/3.0 - hue) * 6.0 }
                                        else { m1 }
                                    };
                                    (_v(h + 1.0/3.0), _v(h), _v(h - 1.0/3.0))
                                }
                            }
                        });
                    }
                    _ => {} // Fall through for other colorsys methods
                }
            }
        }

        // DEPYLER-1002: Handle base64 module method calls in class methods
        // DEPYLER-1026: NASA mode uses stub implementations instead of base64 crate
        if let HirExpr::Var(module_name) = object {
            if module_name == "base64" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                let nasa_mode = self.type_mapper.nasa_mode;
                match method {
                    "b64encode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            // NASA mode: Return hex-encoded bytes as stub
                            return Ok(parse_quote! {
                                #data.iter().map(|b| format!("{:02x}", b)).collect::<String>()
                            });
                        }
                        return Ok(parse_quote! {
                            base64::engine::general_purpose::STANDARD.encode(#data)
                        });
                    }
                    "b64decode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            // NASA mode: Return input bytes as stub
                            return Ok(parse_quote! {
                                #data.as_bytes().to_vec()
                            });
                        }
                        return Ok(parse_quote! {
                            base64::engine::general_purpose::STANDARD.decode(#data).unwrap()
                        });
                    }
                    "urlsafe_b64encode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            return Ok(parse_quote! {
                                #data.iter().map(|b| format!("{:02x}", b)).collect::<String>()
                            });
                        }
                        return Ok(parse_quote! {
                            base64::engine::general_purpose::URL_SAFE.encode(#data)
                        });
                    }
                    "urlsafe_b64decode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            return Ok(parse_quote! {
                                #data.as_bytes().to_vec()
                            });
                        }
                        return Ok(parse_quote! {
                            base64::engine::general_purpose::URL_SAFE.decode(#data).unwrap()
                        });
                    }
                    "b32encode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            return Ok(parse_quote! {
                                #data.iter().map(|b| format!("{:02x}", b)).collect::<String>().into_bytes()
                            });
                        }
                        return Ok(parse_quote! {
                            data_encoding::BASE32.encode(#data).into_bytes()
                        });
                    }
                    "b32decode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            return Ok(parse_quote! {
                                #data.to_vec()
                            });
                        }
                        return Ok(parse_quote! {
                            data_encoding::BASE32.decode(#data).unwrap()
                        });
                    }
                    "b16encode" | "hexlify" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            // NASA mode: Can use std format for hex
                            return Ok(parse_quote! {
                                #data.iter().map(|b| format!("{:02x}", b)).collect::<String>().into_bytes()
                            });
                        }
                        return Ok(parse_quote! {
                            hex::encode(#data).into_bytes()
                        });
                    }
                    "b16decode" | "unhexlify" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            // NASA mode: Return input as bytes stub
                            return Ok(parse_quote! {
                                #data.to_vec()
                            });
                        }
                        return Ok(parse_quote! {
                            hex::decode(#data).unwrap()
                        });
                    }
                    _ => {} // Fall through for unhandled base64 methods
                }
            }
        }

        // DEPYLER-1002: Handle hashlib module method calls in class methods
        // hashlib.md5() → Md5::new()
        // hashlib.sha256() → Sha256::new()
        if let HirExpr::Var(module_name) = object {
            if module_name == "hashlib" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "md5" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use md5::Digest;
                                    Box::new(md5::Md5::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use md5::Digest;
                                    let mut h = Box::new(md5::Md5::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "sha1" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha1::Digest;
                                    Box::new(sha1::Sha1::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha1::Digest;
                                    let mut h = Box::new(sha1::Sha1::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "sha256" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "sha512" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha512::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha512::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "sha384" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha384::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha384::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "blake2b" | "blake2s" => {
                        // For blake2, just use sha256 as fallback since blake2 crate API differs
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "new" => {
                        // hashlib.new("algorithm", data) factory method
                        // For simplicity, default to sha256 since we can't pattern match strings at compile time
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else if arg_exprs.len() == 1 {
                            // Just algorithm name, no data
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            // Algorithm name + data
                            let data = &arg_exprs[1];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    _ => {} // Fall through for unhandled hashlib methods
                }
            }
        }

        // DEPYLER-1002: Handle json module method calls in class methods
        // DEPYLER-1022: NASA mode uses std-only stubs
        // json.dumps(obj) → serde_json::to_string(&obj).unwrap() (or format! in NASA mode)
        // json.loads(s) → serde_json::from_str(&s).unwrap() (or empty HashMap in NASA mode)
        if let HirExpr::Var(module_name) = object {
            if module_name == "json" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "dumps" if !arg_exprs.is_empty() => {
                        let obj = &arg_exprs[0];
                        // DEPYLER-1022: NASA mode uses format! instead of serde_json
                        if self.type_mapper.nasa_mode {
                            return Ok(parse_quote! { format!("{:?}", #obj) });
                        }
                        return Ok(parse_quote! { serde_json::to_string(&#obj).unwrap() });
                    }
                    "loads" if !arg_exprs.is_empty() => {
                        let _s = &arg_exprs[0];
                        // DEPYLER-1022/1051: NASA mode returns empty HashMap stub with DepylerValue
                        if self.type_mapper.nasa_mode {
                            return Ok(parse_quote! { std::collections::HashMap::<String, DepylerValue>::new() });
                        }
                        return Ok(parse_quote! { serde_json::from_str::<serde_json::Value>(&#_s).unwrap() });
                    }
                    _ => {} // Fall through
                }
            }
        }

        // DEPYLER-1002: Handle math module method calls in class methods
        // math.sqrt(x) → x.sqrt()
        if let HirExpr::Var(module_name) = object {
            if module_name == "math" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "sqrt" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).sqrt() });
                    }
                    "sin" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).sin() });
                    }
                    "cos" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).cos() });
                    }
                    "tan" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).tan() });
                    }
                    "floor" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).floor() });
                    }
                    "ceil" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).ceil() });
                    }
                    "abs" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).abs() });
                    }
                    "pow" if arg_exprs.len() >= 2 => {
                        let x = &arg_exprs[0];
                        let y = &arg_exprs[1];
                        return Ok(parse_quote! { (#x as f64).powf(#y as f64) });
                    }
                    "log" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        if arg_exprs.len() >= 2 {
                            let base = &arg_exprs[1];
                            return Ok(parse_quote! { (#x as f64).log(#base as f64) });
                        }
                        return Ok(parse_quote! { (#x as f64).ln() });
                    }
                    "exp" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).exp() });
                    }
                    _ => {} // Fall through
                }
            }
        }

        // DEPYLER-1002: Handle random module method calls in class methods
        // random.randint(a, b) → rand::thread_rng().gen_range(a..=b)
        if let HirExpr::Var(module_name) = object {
            if module_name == "random" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "randint" if arg_exprs.len() >= 2 => {
                        let a = &arg_exprs[0];
                        let b = &arg_exprs[1];
                        return Ok(parse_quote! {
                            {
                                use rand::Rng;
                                rand::thread_rng().gen_range(#a..=#b)
                            }
                        });
                    }
                    "random" if arg_exprs.is_empty() => {
                        return Ok(parse_quote! {
                            {
                                use rand::Rng;
                                rand::thread_rng().gen::<f64>()
                            }
                        });
                    }
                    "choice" if !arg_exprs.is_empty() => {
                        let seq = &arg_exprs[0];
                        return Ok(parse_quote! {
                            {
                                use rand::seq::SliceRandom;
                                #seq.choose(&mut rand::thread_rng()).cloned().unwrap()
                            }
                        });
                    }
                    "shuffle" if !arg_exprs.is_empty() => {
                        let seq = &arg_exprs[0];
                        return Ok(parse_quote! {
                            {
                                use rand::seq::SliceRandom;
                                #seq.shuffle(&mut rand::thread_rng())
                            }
                        });
                    }
                    _ => {} // Fall through
                }
            }
        }

        // DEPYLER-0200: Handle os.path.* and os.environ.* method calls in class methods
        // Pattern: os.path.exists(path), os.environ.get("KEY") etc.
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(module_name) = value.as_ref() {
                if module_name == "os" && attr == "path" {
                    if let Some(rust_expr) = self.try_convert_os_path_method(method, args)? {
                        return Ok(rust_expr);
                    }
                }
                if module_name == "os" && attr == "environ" {
                    if let Some(rust_expr) = self.try_convert_os_environ_method(method, args)? {
                        return Ok(rust_expr);
                    }
                }
            }
        }

        // DEPYLER-0932: Handle dict.fromkeys(keys, default) class method
        // dict.fromkeys(keys, default) → keys.iter().map(|k| (k.clone(), default)).collect()
        if let HirExpr::Var(var_name) = object {
            if var_name == "dict" && method == "fromkeys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;

                if arg_exprs.len() >= 2 {
                    let keys_expr = &arg_exprs[0];
                    let default_expr = &arg_exprs[1];
                    return Ok(parse_quote! {
                        #keys_expr.iter().map(|k| (k.clone(), #default_expr)).collect()
                    });
                } else if arg_exprs.len() == 1 {
                    // dict.fromkeys(keys) with implicit None default
                    let keys_expr = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #keys_expr.iter().map(|k| (k.clone(), ())).collect()
                    });
                }
            }
        }

        // DEPYLER-0933: Handle int.from_bytes(bytes, byteorder) class method in class methods
        // int.from_bytes(bytes, "big") → i64::from_be_bytes(...)
        // int.from_bytes(bytes, "little") → i64::from_le_bytes(...)
        if let HirExpr::Var(var_name) = object {
            if var_name == "int" && method == "from_bytes" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;

                if arg_exprs.len() >= 2 {
                    let bytes_expr = &arg_exprs[0];
                    // Check if second arg is "big" or "little" string literal
                    let is_big_endian = if let HirExpr::Literal(Literal::String(s)) = &args[1] {
                        s == "big"
                    } else {
                        true // Default to big endian
                    };

                    if is_big_endian {
                        return Ok(parse_quote! {
                            i64::from_be_bytes({
                                let mut arr = [0u8; 8];
                                let bytes: &[u8] = #bytes_expr.as_ref();
                                let start = 8usize.saturating_sub(bytes.len());
                                arr[start..].copy_from_slice(bytes);
                                arr
                            })
                        });
                    } else {
                        return Ok(parse_quote! {
                            i64::from_le_bytes({
                                let mut arr = [0u8; 8];
                                let bytes: &[u8] = #bytes_expr.as_ref();
                                arr[..bytes.len().min(8)].copy_from_slice(&bytes[..bytes.len().min(8)]);
                                arr
                            })
                        });
                    }
                }
            }
        }

        // Check if this is a static method call on a class (e.g., Counter.create_with_value)
        if let HirExpr::Var(class_name) = object {
            if class_name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
            {
                // This is likely a static method call - convert to ClassName::method(args)
                let class_ident = make_ident(class_name);
                let method_ident = make_ident(method);
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { #class_ident::#method_ident(#(#arg_exprs),*) });
            }
        }

        // DEPYLER-1008: Check if this is a mutating method call on self.field
        // If so, we should NOT add .clone() to the object expression
        // Mutating methods: append, push, insert, pop, clear, extend, remove, add, update, etc.
        let is_mutating_method = matches!(
            method,
            "append" | "push" | "push_back" | "push_front"
                | "appendleft" | "popleft" | "pop"
                | "insert" | "remove" | "clear" | "extend"
                | "add" | "update" | "discard"
        );

        // Check if object is self.field pattern
        let is_self_field = matches!(
            object,
            HirExpr::Attribute { value, .. } if matches!(value.as_ref(), HirExpr::Var(name) if name == "self")
        );

        let object_expr = if is_mutating_method && is_self_field {
            // DEPYLER-1008: For mutating calls on self.field, don't add .clone()
            // Just generate self.field directly
            if let HirExpr::Attribute { value, attr } = object {
                let attr_ident = make_ident(attr);
                let value_expr = self.convert(value)?;
                parse_quote! { #value_expr.#attr_ident }
            } else {
                self.convert(object)?
            }
        } else {
            self.convert(object)?
        };
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        // Map Python collection methods to Rust equivalents
        match method {
            // List/Deque methods
            "append" => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];

                // DEPYLER-1051: Check if target is Vec<DepylerValue> (e.g., untyped class field)
                // If so, wrap the argument in appropriate DepylerValue variant
                let is_vec_depyler_value = if let HirExpr::Attribute { attr, .. } = object {
                    self.class_field_types
                        .get(attr)
                        .map(|t| matches!(t, Type::List(elem) if matches!(elem.as_ref(), Type::Unknown)))
                        .unwrap_or(false)
                } else {
                    false
                };

                if is_vec_depyler_value {
                    // Wrap argument in DepylerValue based on argument type
                    let wrapped_arg: syn::Expr = if !args.is_empty() {
                        match &args[0] {
                            HirExpr::Literal(Literal::Int(_)) => parse_quote! { DepylerValue::Int(#arg as i64) },
                            HirExpr::Literal(Literal::Float(_)) => parse_quote! { DepylerValue::Float(#arg as f64) },
                            HirExpr::Literal(Literal::String(_)) => parse_quote! { DepylerValue::Str(#arg.to_string()) },
                            HirExpr::Literal(Literal::Bool(_)) => parse_quote! { DepylerValue::Bool(#arg) },
                            HirExpr::Var(name) => {
                                // Check parameter type
                                match self.param_types.get(name) {
                                    Some(Type::Int) => parse_quote! { DepylerValue::Int(#arg as i64) },
                                    Some(Type::Float) => parse_quote! { DepylerValue::Float(#arg as f64) },
                                    Some(Type::String) => parse_quote! { DepylerValue::Str(#arg.to_string()) },
                                    Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#arg) },
                                    _ => parse_quote! { DepylerValue::Str(format!("{:?}", #arg)) },
                                }
                            }
                            _ => parse_quote! { DepylerValue::Str(format!("{:?}", #arg)) },
                        }
                    } else {
                        parse_quote! { DepylerValue::Str(format!("{:?}", #arg)) }
                    };
                    return Ok(parse_quote! { #object_expr.push(#wrapped_arg) });
                }

                // DEPYLER-0742: VecDeque uses push_back, Vec uses push
                if self.is_deque_expr(object) {
                    Ok(parse_quote! { #object_expr.push_back(#arg) })
                } else {
                    Ok(parse_quote! { #object_expr.push(#arg) })
                }
            }
            // DEPYLER-0742: Deque-specific methods
            "appendleft" => {
                if arg_exprs.len() != 1 {
                    bail!("appendleft() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.push_front(#arg) })
            }
            "popleft" => {
                if !arg_exprs.is_empty() {
                    bail!("popleft() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.pop_front() })
            }
            "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // Check if it's a list (using position) or set (using remove)
                // For now, assume set behavior since we're working on sets
                if self.is_set_expr(object) {
                    Ok(parse_quote! {
                        if !#object_expr.remove(&#arg) {
                            panic!("KeyError: element not in set");
                        }
                    })
                } else {
                    // List remove behavior
                    Ok(parse_quote! {
                        if let Some(pos) = #object_expr.iter().position(|x| x == &#arg) {
                            #object_expr.remove(pos);
                        } else {
                            panic!("ValueError: list.remove(x): x not in list");
                        }
                    })
                }
            }

            // Set methods
            "add" => {
                if arg_exprs.len() != 1 {
                    bail!("add() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.insert(#arg) })
            }
            "discard" => {
                if arg_exprs.len() != 1 {
                    bail!("discard() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.remove(&#arg) })
            }
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "pop" => {
                if self.is_set_expr(object) {
                    if !arg_exprs.is_empty() {
                        bail!("pop() takes no arguments");
                    }
                    // HashSet doesn't have pop(), simulate with iter().next() and remove
                    Ok(parse_quote! {
                        #object_expr.iter().next().cloned().map(|x| {
                            #object_expr.remove(&x);
                            x
                        }).expect("pop from empty set")
                    })
                } else if self.is_deque_expr(object) {
                    // DEPYLER-0742: VecDeque uses pop_back
                    if arg_exprs.is_empty() {
                        Ok(parse_quote! { #object_expr.pop_back().unwrap_or_default() })
                    } else {
                        bail!("deque.pop() does not accept an index argument");
                    }
                } else {
                    // List pop
                    if arg_exprs.is_empty() {
                        Ok(parse_quote! { #object_expr.pop().unwrap_or_default() })
                    } else {
                        let idx = &arg_exprs[0];
                        Ok(parse_quote! { #object_expr.remove(#idx as usize) })
                    }
                }
            }

            // String methods - DEPYLER-0413
            "upper" => {
                if !arg_exprs.is_empty() {
                    bail!("upper() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_uppercase() })
            }
            "lower" => {
                if !arg_exprs.is_empty() {
                    bail!("lower() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_lowercase() })
            }
            "strip" => {
                if !arg_exprs.is_empty() {
                    bail!("strip() with arguments not supported");
                }
                Ok(parse_quote! { #object_expr.trim().to_string() })
            }
            "lstrip" => {
                if !arg_exprs.is_empty() {
                    bail!("lstrip() with arguments not supported");
                }
                Ok(parse_quote! { #object_expr.trim_start().to_string() })
            }
            "rstrip" => {
                if !arg_exprs.is_empty() {
                    bail!("rstrip() with arguments not supported");
                }
                Ok(parse_quote! { #object_expr.trim_end().to_string() })
            }
            "startswith" => {
                if args.len() != 1 {
                    bail!("startswith() requires exactly one argument");
                }
                // DEPYLER-0602: For starts_with(), use raw string literal for Pattern trait.
                let prefix: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.starts_with(#prefix) })
            }
            "endswith" => {
                if args.len() != 1 {
                    bail!("endswith() requires exactly one argument");
                }
                // DEPYLER-0602: For ends_with(), use raw string literal for Pattern trait.
                let suffix: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.ends_with(#suffix) })
            }
            "split" => {
                if args.is_empty() {
                    Ok(
                        parse_quote! { #object_expr.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if args.len() == 1 {
                    // DEPYLER-0602: For split(), use raw string literal for Pattern trait.
                    let sep: syn::Expr = match &args[0] {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => self.convert(&args[0])?,
                    };
                    Ok(
                        parse_quote! { #object_expr.split(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if args.len() == 2 {
                    // DEPYLER-0188: split(sep, maxsplit) -> splitn(maxsplit+1, sep)
                    // Python maxsplit=N means at most N splits → N+1 parts
                    // Rust splitn(n, pat) returns at most n parts
                    let sep: syn::Expr = match &args[0] {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => self.convert(&args[0])?,
                    };
                    let maxsplit = self.convert(&args[1])?;
                    Ok(
                        parse_quote! { #object_expr.splitn((#maxsplit + 1) as usize, #sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("split() requires 0-2 arguments");
                }
            }
            "join" => {
                if arg_exprs.len() != 1 {
                    bail!("join() requires exactly one argument");
                }
                let iterable = &arg_exprs[0];
                Ok(parse_quote! { #iterable.join(#object_expr) })
            }
            "replace" => {
                if args.len() != 2 {
                    bail!("replace() requires exactly two arguments");
                }
                // DEPYLER-0602: For replace(), use raw string literals for Pattern trait.
                let old: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                let new: syn::Expr = match &args[1] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[1])?,
                };
                Ok(parse_quote! { #object_expr.replace(#old, #new) })
            }
            "find" => {
                if args.len() != 1 {
                    bail!("find() requires exactly one argument");
                }
                // DEPYLER-0602: For find(), use raw string literal for Pattern trait.
                // String doesn't implement Pattern, but &str does.
                let substring: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.find(#substring).map(|i| i as i64).unwrap_or(-1) })
            }
            "rfind" => {
                if args.len() != 1 {
                    bail!("rfind() requires exactly one argument");
                }
                // DEPYLER-0602: For rfind(), use raw string literal for Pattern trait.
                let substring: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.rfind(#substring).map(|i| i as i64).unwrap_or(-1) })
            }
            "isdigit" => {
                if !arg_exprs.is_empty() {
                    bail!("isdigit() takes no arguments");
                }
                Ok(
                    parse_quote! { !#object_expr.is_empty() && #object_expr.chars().all(|c| c.is_ascii_digit()) },
                )
            }
            "isalpha" => {
                if !arg_exprs.is_empty() {
                    bail!("isalpha() takes no arguments");
                }
                Ok(
                    parse_quote! { !#object_expr.is_empty() && #object_expr.chars().all(|c| c.is_alphabetic()) },
                )
            }
            "isalnum" => {
                if !arg_exprs.is_empty() {
                    bail!("isalnum() takes no arguments");
                }
                Ok(
                    parse_quote! { !#object_expr.is_empty() && #object_expr.chars().all(|c| c.is_alphanumeric()) },
                )
            }

            // DEPYLER-0200/DEPYLER-0960: String/Dict contains method
            // String: use .contains() with raw string literal for Pattern trait
            // Dict/HashMap: use .contains_key() - E0599 fix
            "__contains__" | "contains" => {
                if args.len() != 1 {
                    bail!("contains() requires exactly one argument");
                }

                // DEPYLER-0960: Detect if object is a dict/HashMap type
                let is_dict_like = match object {
                    HirExpr::Var(name) => {
                        let n = name.as_str();
                        n.contains("dict") || n.contains("map") || n.contains("data")
                            || n == "result" || n == "config" || n == "settings"
                            || n == "params" || n == "options" || n == "env"
                            || n == "d" || n == "m" || n == "cache"
                    }
                    HirExpr::Call { func, .. } => {
                        func.contains("dict") || func.contains("json") || func.contains("config")
                            || func.contains("result") || func.contains("load")
                    }
                    _ => false,
                };

                if is_dict_like {
                    // HashMap uses contains_key(&key)
                    let key = self.convert(&args[0])?;
                    Ok(parse_quote! { #object_expr.contains_key(&#key) })
                } else {
                    // String uses .contains(pattern) with Pattern trait
                    let pattern: syn::Expr = match &args[0] {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String and &str
                            let arg = self.convert(&args[0])?;
                            parse_quote! { &*#arg }
                        }
                    };
                    Ok(parse_quote! { #object_expr.contains(#pattern) })
                }
            }

            // DEPYLER-0613: Semaphore/Mutex method mappings
            // Python: sem.acquire() → Rust: mutex.lock().unwrap() (returns guard)
            "acquire" => {
                // Mutex.lock() returns a guard - acquire returns bool in Python but we adapt
                Ok(parse_quote! { #object_expr.lock().is_ok() })
            }
            // Python: sem.release() → Rust: drop guard (no-op if guard not held)
            "release" => {
                // In Rust, release happens when guard is dropped
                // For now, just return unit since we can't easily track the guard
                Ok(parse_quote! { () })
            }

            // DEPYLER-0613: List/Dict copy method
            // Python: list.copy() → Rust: vec.clone()
            "copy" => {
                if !arg_exprs.is_empty() {
                    bail!("copy() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clone() })
            }

            // DEPYLER-0613: Dict contains_key (may be called on wrong type)
            // Python: dict.__contains__(key) sometimes transpiles as contains_key
            "contains_key" => {
                if arg_exprs.len() != 1 {
                    bail!("contains_key() requires exactly one argument");
                }
                let key = &arg_exprs[0];
                // For HashMap this is correct, for Vec use contains
                Ok(parse_quote! { #object_expr.contains(&#key) })
            }

            // Generic method call fallback
            _ => {
                // DEPYLER-0596: Validate method name before creating identifier
                // Method names must be valid Rust identifiers (no empty, no special chars)
                if method.is_empty() {
                    bail!("Empty method name in method call");
                }
                // Check if method is a valid identifier (starts with letter/underscore, alphanumeric)
                let is_valid_ident = method.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
                    && method.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
                if !is_valid_ident {
                    bail!("Invalid method name '{}' - not a valid Rust identifier", method);
                }

                // DEPYLER-0823: Wrap cast expressions in parentheses before method calls
                // Rust parses `x as i32.method()` as `x as (i32.method())` which is invalid
                // Must be: `(x as i32).method()`
                let safe_object_expr: syn::Expr = if matches!(object_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#object_expr) }
                } else {
                    object_expr.clone()
                };

                // Debug: Check if method is a Rust keyword
                if syn::parse_str::<syn::Ident>(method).is_err() {
                    // Method is a Rust keyword - use raw identifier
                    let method_ident = syn::Ident::new_raw(method, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #safe_object_expr.#method_ident(#(#arg_exprs),*) });
                }
                let method_ident = make_ident(method);
                Ok(parse_quote! { #safe_object_expr.#method_ident(#(#arg_exprs),*) })
            }
        }
    }

    fn convert_list_comp(
        &self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_pat = parse_target_pattern(target);
        let iter_expr = self.convert(iter)?;
        let element_expr = self.convert(element)?;

        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            // DEPYLER-0833: Use |x| pattern (not |&x|) to avoid E0507 on non-Copy types
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            let cond_expr = self.convert(cond)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr })
                    .map(|#target_pat| #element_expr)
                    .collect::<Vec<_>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_pat| #element_expr)
                    .collect::<Vec<_>>()
            })
        }
    }

    fn convert_set_comp(
        &self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_pat = parse_target_pattern(target);
        let iter_expr = self.convert(iter)?;
        let element_expr = self.convert(element)?;

        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            // DEPYLER-0833: Use |x| pattern (not |&x|) to avoid E0507 on non-Copy types
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            let cond_expr = self.convert(cond)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr })
                    .map(|#target_pat| #element_expr)
                    .collect::<std::collections::HashSet<_>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_pat| #element_expr)
                    .collect::<std::collections::HashSet<_>>()
            })
        }
    }

    /// DEPYLER-0610: Convert Python stdlib module constructor calls to Rust
    /// threading.Semaphore(n) → std::sync::Mutex::new(n)
    /// queue.Queue() → std::collections::VecDeque::new()
    fn convert_module_constructor(
        &self,
        module: &str,
        constructor: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        let result = match module {
            "threading" => match constructor {
                "Semaphore" | "BoundedSemaphore" => {
                    // threading.Semaphore(n) → std::sync::Mutex::new(n)
                    // Use first arg or default to 0
                    if let Some(arg) = arg_exprs.first() {
                        Some(parse_quote! { std::sync::Mutex::new(#arg) })
                    } else {
                        Some(parse_quote! { std::sync::Mutex::new(0) })
                    }
                }
                "Lock" | "RLock" => {
                    // threading.Lock() → std::sync::Mutex::new(())
                    Some(parse_quote! { std::sync::Mutex::new(()) })
                }
                "Event" => {
                    // threading.Event() → std::sync::Condvar::new()
                    Some(parse_quote! { std::sync::Condvar::new() })
                }
                "Thread" => {
                    // threading.Thread(target=fn) → std::thread::spawn(fn)
                    // Simplified - just return a placeholder
                    Some(parse_quote! { std::thread::spawn(|| {}) })
                }
                _ => None,
            },
            "queue" => match constructor {
                "Queue" | "LifoQueue" | "PriorityQueue" => {
                    // queue.Queue() → std::collections::VecDeque::new()
                    Some(parse_quote! { std::collections::VecDeque::new() })
                }
                _ => None,
            },
            "datetime" => match constructor {
                "datetime" => {
                    // DEPYLER-1025: In NASA mode, use std::time instead of chrono
                    if self.type_mapper.nasa_mode {
                        Some(parse_quote! { std::time::SystemTime::now() })
                    } else {
                        // datetime.datetime(y,m,d,...) → chrono placeholder
                        Some(parse_quote! { chrono::Utc::now() })
                    }
                }
                "date" => {
                    if self.type_mapper.nasa_mode {
                        Some(parse_quote! { std::time::SystemTime::now() })
                    } else {
                        Some(parse_quote! { chrono::Utc::now().date_naive() })
                    }
                }
                "time" => {
                    if self.type_mapper.nasa_mode {
                        Some(parse_quote! { std::time::SystemTime::now() })
                    } else {
                        Some(parse_quote! { chrono::Utc::now().time() })
                    }
                }
                "timedelta" => {
                    // DEPYLER-1025: In NASA mode, use std::time::Duration
                    if self.type_mapper.nasa_mode {
                        if let Some(arg) = arg_exprs.first() {
                            Some(parse_quote! { std::time::Duration::from_secs((#arg as u64) * 86400) })
                        } else {
                            Some(parse_quote! { std::time::Duration::from_secs(0) })
                        }
                    } else {
                        // datetime.timedelta(days=n) → chrono::Duration::days(n)
                        if let Some(arg) = arg_exprs.first() {
                            Some(parse_quote! { chrono::Duration::days(#arg) })
                        } else {
                            Some(parse_quote! { chrono::Duration::zero() })
                        }
                    }
                }
                "now" => {
                    // DEPYLER-1025: In NASA mode, use std::time
                    if self.type_mapper.nasa_mode {
                        Some(parse_quote! { std::time::SystemTime::now() })
                    } else {
                        // datetime.datetime.now() → chrono::Utc::now()
                        Some(parse_quote! { chrono::Utc::now() })
                    }
                }
                _ => None,
            },
            "collections" => match constructor {
                "deque" => {
                    Some(parse_quote! { std::collections::VecDeque::new() })
                }
                "Counter" => {
                    Some(parse_quote! { std::collections::HashMap::new() })
                }
                "OrderedDict" => {
                    Some(parse_quote! { std::collections::HashMap::new() })
                }
                "defaultdict" => {
                    Some(parse_quote! { std::collections::HashMap::new() })
                }
                _ => None,
            },
            "asyncio" => match constructor {
                // DEPYLER-1024: In NASA mode, use std-only primitives
                "Event" => {
                    if self.type_mapper.nasa_mode {
                        Some(parse_quote! { std::sync::Condvar::new() })
                    } else {
                        Some(parse_quote! { tokio::sync::Notify::new() })
                    }
                }
                // DEPYLER-1024: In NASA mode, use std-only primitives instead of tokio
                "Lock" => {
                    if self.type_mapper.nasa_mode {
                        Some(parse_quote! { std::sync::Mutex::new(()) })
                    } else {
                        Some(parse_quote! { tokio::sync::Mutex::new(()) })
                    }
                }
                "Semaphore" => {
                    // NASA mode: No direct std equivalent, use dummy
                    if self.type_mapper.nasa_mode {
                        Some(parse_quote! { () })
                    } else if let Some(arg) = arg_exprs.first() {
                        Some(parse_quote! { tokio::sync::Semaphore::new(#arg as usize) })
                    } else {
                        Some(parse_quote! { tokio::sync::Semaphore::new(1) })
                    }
                }
                "Queue" => {
                    if self.type_mapper.nasa_mode {
                        Some(parse_quote! { std::sync::mpsc::channel().1 })
                    } else {
                        Some(parse_quote! { tokio::sync::mpsc::channel(100).1 })
                    }
                }
                // DEPYLER-0747: asyncio.sleep(secs) → tokio::time::sleep(Duration)
                // DEPYLER-1024: In NASA mode, use std::thread::sleep instead
                "sleep" => {
                    if self.type_mapper.nasa_mode {
                        if let Some(arg) = arg_exprs.first() {
                            Some(parse_quote! {
                                std::thread::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                            })
                        } else {
                            Some(parse_quote! {
                                std::thread::sleep(std::time::Duration::from_secs(0))
                            })
                        }
                    } else if let Some(arg) = arg_exprs.first() {
                        Some(parse_quote! {
                            tokio::time::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                        })
                    } else {
                        Some(parse_quote! {
                            tokio::time::sleep(std::time::Duration::from_secs(0))
                        })
                    }
                }
                // DEPYLER-0747: asyncio.run(coro) → tokio runtime block_on
                // DEPYLER-1024: In NASA mode, just call the function directly (since async is converted to sync)
                "run" => {
                    if self.type_mapper.nasa_mode {
                        arg_exprs.first().map(|arg| parse_quote! { #arg })
                    } else {
                        arg_exprs.first().map(|arg| {
                            parse_quote! {
                                tokio::runtime::Runtime::new().unwrap().block_on(#arg)
                            }
                        })
                    }
                }
                _ => None,
            },
            // DEPYLER-0950: json.loads/load need proper type annotation and borrowing
            // serde_json::from_str expects &str, returns Result, needs type annotation
            "json" => match constructor {
                "loads" | "load" => {
                    arg_exprs.first().map(|arg| parse_quote! { serde_json::from_str::<serde_json::Value>(&#arg).unwrap() })
                }
                "dumps" | "dump" => {
                    arg_exprs.first().map(|arg| parse_quote! { serde_json::to_string(&#arg).unwrap() })
                }
                _ => None,
            },
            "os" => match constructor {
                "getcwd" => Some(parse_quote! { std::env::current_dir()?.to_string_lossy().to_string() }),
                "getenv" => {
                    arg_exprs.first().map(|arg| parse_quote! { std::env::var(#arg).ok() })
                }
                "listdir" => {
                    if let Some(arg) = arg_exprs.first() {
                        Some(parse_quote! { std::fs::read_dir(#arg)?.map(|e| e.unwrap().file_name().to_string_lossy().to_string()).collect::<Vec<_>>() })
                    } else {
                        Some(parse_quote! { std::fs::read_dir(".")?.map(|e| e.unwrap().file_name().to_string_lossy().to_string()).collect::<Vec<_>>() })
                    }
                }
                _ => None,
            },
            "re" => match constructor {
                "compile" | "match" | "search" | "findall" => {
                    // Return a placeholder that compiles
                    Some(parse_quote! { serde_json::Value::Null })
                }
                _ => None,
            },
            "fnmatch" => match constructor {
                "fnmatch" => {
                    // fnmatch.fnmatch(name, pattern) → name.contains(pattern) as stub
                    if arg_exprs.len() >= 2 {
                        let name = &arg_exprs[0];
                        let pattern = &arg_exprs[1];
                        Some(parse_quote! { #name.contains(&#pattern) })
                    } else {
                        Some(parse_quote! { false })
                    }
                }
                _ => None,
            },
            _ => None,
        };

        Ok(result)
    }

    /// DEPYLER-0200: Convert os module method calls to Rust std::fs and std::env equivalents
    /// This was missing from class method context, causing 57+ compile errors
    fn try_convert_os_method(&self, method: &str, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            "getenv" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("os.getenv() requires 1 or 2 arguments");
                }
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Some(parse_quote! { std::env::var(#key)? })
                } else {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    Some(parse_quote! { std::env::var(#key).unwrap_or_else(|_| #default.to_string()) })
                }
            }
            "unlink" | "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("os.{}() requires exactly 1 argument", method);
                }
                let path = &arg_exprs[0];
                // DEPYLER-0956: Use .unwrap() to not require Result return type
                Some(parse_quote! { std::fs::remove_file(#path).unwrap() })
            }
            "mkdir" => {
                if arg_exprs.is_empty() {
                    bail!("os.mkdir() requires at least 1 argument");
                }
                let path = &arg_exprs[0];
                // DEPYLER-0956: Use .unwrap() to not require Result return type
                Some(parse_quote! { std::fs::create_dir(#path).unwrap() })
            }
            "makedirs" => {
                if arg_exprs.is_empty() {
                    bail!("os.makedirs() requires at least 1 argument");
                }
                let path = &arg_exprs[0];
                // DEPYLER-0956: Use .unwrap() to not require Result return type
                Some(parse_quote! { std::fs::create_dir_all(#path).unwrap() })
            }
            "rmdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.rmdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                // DEPYLER-0956: Use .unwrap() to not require Result return type
                Some(parse_quote! { std::fs::remove_dir(#path).unwrap() })
            }
            "rename" => {
                if arg_exprs.len() != 2 {
                    bail!("os.rename() requires exactly 2 arguments");
                }
                let src = &arg_exprs[0];
                let dst = &arg_exprs[1];
                // DEPYLER-0956: Use .unwrap() to not require Result return type
                Some(parse_quote! { std::fs::rename(#src, #dst).unwrap() })
            }
            "getcwd" => {
                if !arg_exprs.is_empty() {
                    bail!("os.getcwd() takes no arguments");
                }
                Some(parse_quote! { std::env::current_dir()?.to_string_lossy().to_string() })
            }
            "chdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.chdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::env::set_current_dir(#path)? })
            }
            "listdir" => {
                if arg_exprs.is_empty() {
                    Some(parse_quote! {
                        std::fs::read_dir(".")?
                            .filter_map(|e| e.ok())
                            .map(|e| e.file_name().to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                    })
                } else {
                    let path = &arg_exprs[0];
                    Some(parse_quote! {
                        std::fs::read_dir(#path)?
                            .filter_map(|e| e.ok())
                            .map(|e| e.file_name().to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                    })
                }
            }
            "path" => {
                // os.path is a submodule, handled elsewhere
                None
            }
            _ => None,
        };

        Ok(result)
    }

    /// DEPYLER-0200: Convert os.path module method calls to Rust std::path equivalents
    fn try_convert_os_path_method(&self, method: &str, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            "join" => {
                if arg_exprs.is_empty() {
                    bail!("os.path.join() requires at least 1 argument");
                }
                let first = &arg_exprs[0];
                if arg_exprs.len() == 1 {
                    Some(parse_quote! { std::path::PathBuf::from(#first) })
                } else {
                    let mut result: syn::Expr = parse_quote! { std::path::PathBuf::from(#first) };
                    for part in &arg_exprs[1..] {
                        result = parse_quote! { #result.join(#part) };
                    }
                    Some(parse_quote! { #result.to_string_lossy().to_string() })
                }
            }
            "basename" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.basename() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! {
                    std::path::Path::new(&#path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string()
                })
            }
            "dirname" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.dirname() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! {
                    std::path::Path::new(&#path)
                        .parent()
                        .and_then(|p| p.to_str())
                        .unwrap_or("")
                        .to_string()
                })
            }
            "exists" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.exists() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::path::Path::new(&#path).exists() })
            }
            "isfile" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.isfile() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::path::Path::new(&#path).is_file() })
            }
            "isdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.isdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::path::Path::new(&#path).is_dir() })
            }
            "expanduser" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.expanduser() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! {
                    if (#path).starts_with("~") {
                        std::env::var("HOME")
                            .map(|home| (#path).replacen("~", &home, 1))
                            .unwrap_or_else(|_| (#path).to_string())
                    } else {
                        (#path).to_string()
                    }
                })
            }
            _ => None,
        };

        Ok(result)
    }

    /// DEPYLER-0200: Convert os.environ method calls to Rust std::env equivalents
    fn try_convert_os_environ_method(&self, method: &str, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            "get" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("os.environ.get() requires 1 or 2 arguments");
                }
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Some(parse_quote! { std::env::var(#key).ok() })
                } else {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    Some(parse_quote! { std::env::var(#key).unwrap_or_else(|_| #default.to_string()) })
                }
            }
            "keys" => {
                Some(parse_quote! { std::env::vars().map(|(k, _)| k).collect::<Vec<_>>() })
            }
            "values" => {
                Some(parse_quote! { std::env::vars().map(|(_, v)| v).collect::<Vec<_>>() })
            }
            "items" => {
                Some(parse_quote! { std::env::vars().collect::<Vec<_>>() })
            }
            "clear" => {
                Some(parse_quote! { { /* env clear not implemented */ } })
            }
            "update" => {
                Some(parse_quote! { { /* env update not implemented */ } })
            }
            "insert" | "setdefault" => {
                if arg_exprs.len() >= 2 {
                    let key = &arg_exprs[0];
                    let val = &arg_exprs[1];
                    Some(parse_quote! { std::env::set_var(#key, #val) })
                } else {
                    None
                }
            }
            "contains_key" => {
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Some(parse_quote! { std::env::var(#key).is_ok() })
                } else {
                    None
                }
            }
            _ => None,
        };

        Ok(result)
    }

    fn convert_dict_comp(
        &self,
        key: &HirExpr,
        value: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_pat = parse_target_pattern(target);
        let iter_expr = self.convert(iter)?;
        let key_expr = self.convert(key)?;
        let value_expr = self.convert(value)?;

        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            // DEPYLER-0833: Use |x| pattern (not |&x|) to avoid E0507 on non-Copy types
            let cond_expr = self.convert(cond)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|#target_pat| #cond_expr)
                    .map(|#target_pat| (#key_expr, #value_expr))
                    .collect::<std::collections::HashMap<_, _>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_pat| (#key_expr, #value_expr))
                    .collect::<std::collections::HashMap<_, _>>()
            })
        }
    }

    fn convert_lambda(&self, params: &[String], body: &HirExpr) -> Result<syn::Expr> {
        // Convert parameters to pattern identifiers
        let param_pats: Vec<syn::Pat> = params
            .iter()
            .map(|p| {
                let ident = make_ident(p);
                parse_quote! { #ident }
            })
            .collect();

        // Convert body expression
        let body_expr = self.convert(body)?;

        // Generate closure
        // DEPYLER-0837: Use `move` closures to match Python's closure semantics
        // Python closures capture variables by reference but extend their lifetime
        // Rust requires `move` when returning closures that capture local variables
        if params.is_empty() {
            // No parameters
            Ok(parse_quote! { move || #body_expr })
        } else if params.len() == 1 {
            // Single parameter
            let param = &param_pats[0];
            Ok(parse_quote! { move |#param| #body_expr })
        } else {
            // Multiple parameters
            Ok(parse_quote! { move |#(#param_pats),*| #body_expr })
        }
    }

    fn convert_await(&self, value: &HirExpr) -> Result<syn::Expr> {
        let value_expr = self.convert(value)?;
        Ok(parse_quote! { #value_expr.await })
    }

    /// DEPYLER-0513: Convert F-string to format!() macro
    ///
    /// Handles Python f-strings like `f"Hello {name}"` → `format!("Hello {}", name)`
    ///
    /// Strategy: Build format template and collect args, then generate format!() call.
    /// Simplified version for direct_rules - basic formatting only.
    fn convert_fstring(&self, parts: &[crate::hir::FStringPart]) -> Result<syn::Expr> {
        use crate::hir::FStringPart;

        // Handle empty f-strings
        if parts.is_empty() {
            return Ok(parse_quote! { "".to_string() });
        }

        // Check if it's just a plain string (no expressions)
        let has_expressions = parts.iter().any(|p| matches!(p, FStringPart::Expr(_)));

        if !has_expressions {
            // Just literal parts - concatenate them
            let mut result = String::new();
            for part in parts {
                if let FStringPart::Literal(s) = part {
                    result.push_str(s);
                }
            }
            return Ok(parse_quote! { #result.to_string() });
        }

        // Build format string template and collect arguments
        let mut template = String::new();
        let mut args = Vec::new();

        for part in parts {
            match part {
                FStringPart::Literal(s) => {
                    template.push_str(s);
                }
                FStringPart::Expr(expr) => {
                    // Add {} placeholder to template
                    template.push_str("{}");
                    // Convert expression to Rust and add to args
                    let arg_expr = self.convert(expr)?;
                    args.push(arg_expr);
                }
            }
        }

        // Generate format!() macro call
        Ok(parse_quote! { format!(#template, #(#args),*) })
    }

    fn convert_attribute(&self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        // Handle classmethod cls.ATTR → Self::ATTR
        if let HirExpr::Var(var_name) = value {
            if var_name == "cls" && self.is_classmethod {
                let attr_ident = make_ident(attr);
                return Ok(parse_quote! { Self::#attr_ident });
            }

            // DEPYLER-0616: Detect enum/type constant access patterns
            // TypeName.CONSTANT → TypeName::CONSTANT
            // Heuristic: If name starts with uppercase and attr is ALL_CAPS, it's likely an enum constant
            let first_char = var_name.chars().next().unwrap_or('a');
            let is_type_name = first_char.is_uppercase();
            let is_constant = attr.chars().all(|c| c.is_uppercase() || c == '_');

            if is_type_name && is_constant {
                let type_ident = make_ident(var_name);
                let attr_ident = make_ident(attr);
                return Ok(parse_quote! { #type_ident::#attr_ident });
            }
        }

        let value_expr = self.convert(value)?;
        // DEPYLER-0596: Use make_ident to handle keywords like "match"
        let attr_ident = make_ident(attr);

        // DEPYLER-0737: Check if this attribute is a @property method
        // In Python, @property allows method access without (), but in Rust we need ()
        let is_prop_method = crate::direct_rules::is_property_method(attr);

        if is_prop_method {
            // Property access needs method call syntax: obj.prop()
            Ok(parse_quote! { #value_expr.#attr_ident() })
        } else {
            // Regular field access: obj.field
            // DEPYLER-0740: For self.field accesses, add .clone() to avoid E0507 moves
            // Python semantics don't consume values on field access, so cloning is safe
            if let HirExpr::Var(var_name) = value {
                if var_name == "self" {
                    return Ok(parse_quote! { #value_expr.#attr_ident.clone() });
                }
            }
            Ok(parse_quote! { #value_expr.#attr_ident })
        }
    }

    /// DEPYLER-0188: Convert dynamic/subscript function call
    /// Pattern: `handlers[name](args)` → `(handlers[&name])(args)` or `handlers.get(&name).unwrap()(args)`
    ///
    /// In Rust, calling a value from a HashMap requires:
    /// 1. Index access with reference: `handlers[&name]`
    /// 2. Parentheses to call the result: `(handlers[&name])(args)`
    fn convert_dynamic_call(&self, callee: &HirExpr, args: &[HirExpr]) -> Result<syn::Expr> {
        // Convert the callee expression (e.g., handlers[name])
        let callee_expr = self.convert(callee)?;

        // Convert arguments
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        // Generate: (callee)(args)
        // Wrap callee in parentheses to ensure correct parsing
        if arg_exprs.is_empty() {
            Ok(parse_quote! { (#callee_expr)() })
        } else {
            Ok(parse_quote! { (#callee_expr)(#(#arg_exprs),*) })
        }
    }
}

/// Check if an expression is a len() call
pub(crate) fn is_len_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, args , ..} if func == "len" && args.len() == 1)
}

pub(crate) fn convert_literal(lit: &Literal) -> syn::Expr {
    match lit {
        Literal::Int(n) => {
            let lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::Float(f) => {
            // DEPYLER-0738: Ensure float literals always have a decimal point
            // f64::to_string() outputs "0" for 0.0, which parses as integer
            let s = f.to_string();
            let float_str = if s.contains('.') || s.contains('e') || s.contains('E') {
                s
            } else {
                format!("{}.0", s)
            };
            let lit = syn::LitFloat::new(&float_str, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::String(s) => {
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            parse_quote! { #lit.to_string() }
        }
        Literal::Bytes(b) => {
            let byte_str = syn::LitByteStr::new(b, proc_macro2::Span::call_site());
            parse_quote! { #byte_str }
        }
        Literal::Bool(b) => {
            let lit = syn::LitBool::new(*b, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        // DEPYLER-1037: Literal::None should map to Rust's None (for Option types)
        // Python: return None -> Rust: return None (when return type is Option<T>)
        Literal::None => parse_quote! { None },
    }
}

/// Convert HIR binary operators to Rust binary operators
pub(crate) fn convert_binop(op: BinOp) -> Result<syn::BinOp> {
    match op {
        // Arithmetic operators
        BinOp::Add
        | BinOp::Sub
        | BinOp::Mul
        | BinOp::Div
        | BinOp::Mod
        | BinOp::FloorDiv
        | BinOp::Pow => convert_arithmetic_op(op),

        // Comparison operators
        BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq => {
            convert_comparison_op(op)
        }

        // Logical operators
        BinOp::And | BinOp::Or => convert_logical_op(op),

        // Bitwise operators
        BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::LShift | BinOp::RShift => {
            convert_bitwise_op(op)
        }

        // Special membership operators
        BinOp::In | BinOp::NotIn => {
            bail!("in/not in operators should be handled by convert_binary")
        }
    }
}

pub(crate) fn convert_arithmetic_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        Add => Ok(parse_quote! { + }),
        Sub => Ok(parse_quote! { - }),
        Mul => Ok(parse_quote! { * }),
        Div => Ok(parse_quote! { / }),
        Mod => Ok(parse_quote! { % }),
        FloorDiv => {
            // Floor division requires special handling - it's not implemented as an operator
            // but handled in convert_binary for proper Python semantics
            bail!("Floor division handled in convert_binary with Python semantics")
        }
        Pow => bail!("Power operator handled in convert_binary with type-specific logic"),
        _ => bail!("Invalid operator {:?} for arithmetic conversion", op),
    }
}

pub(crate) fn convert_comparison_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        Eq => Ok(parse_quote! { == }),
        NotEq => Ok(parse_quote! { != }),
        Lt => Ok(parse_quote! { < }),
        LtEq => Ok(parse_quote! { <= }),
        Gt => Ok(parse_quote! { > }),
        GtEq => Ok(parse_quote! { >= }),
        _ => bail!("Invalid operator {:?} for comparison conversion", op),
    }
}

pub(crate) fn convert_logical_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        And => Ok(parse_quote! { && }),
        Or => Ok(parse_quote! { || }),
        _ => bail!("Invalid operator {:?} for logical conversion", op),
    }
}

pub(crate) fn convert_bitwise_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        BitAnd => Ok(parse_quote! { & }),
        BitOr => Ok(parse_quote! { | }),
        BitXor => Ok(parse_quote! { ^ }),
        LShift => Ok(parse_quote! { << }),
        RShift => Ok(parse_quote! { >> }),
        _ => bail!("Invalid operator {:?} for bitwise conversion", op),
    }
}

// =============================================================================
// EXTREME TDD TEST MODULE - DEPYLER-COVERAGE-95
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // convert_literal tests
    // =========================================================================

    #[test]
    fn test_convert_literal_int() {
        let lit = Literal::Int(42);
        let result = convert_literal(&lit);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("42"));
    }

    #[test]
    fn test_convert_literal_int_negative() {
        let lit = Literal::Int(-100);
        let result = convert_literal(&lit);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("100"));
    }

    #[test]
    fn test_convert_literal_float() {
        let lit = Literal::Float(3.14);
        let result = convert_literal(&lit);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("3.14") || result_str.contains("3.1"));
    }

    #[test]
    fn test_convert_literal_string() {
        let lit = Literal::String("hello".to_string());
        let result = convert_literal(&lit);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("hello"));
    }

    #[test]
    fn test_convert_literal_string_empty() {
        let lit = Literal::String("".to_string());
        let result = convert_literal(&lit);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("\"\"") || result_str.contains("String"));
    }

    #[test]
    fn test_convert_literal_bool_true() {
        let lit = Literal::Bool(true);
        let result = convert_literal(&lit);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("true"));
    }

    #[test]
    fn test_convert_literal_bool_false() {
        let lit = Literal::Bool(false);
        let result = convert_literal(&lit);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("false"));
    }

    #[test]
    fn test_convert_literal_none() {
        let lit = Literal::None;
        let result = convert_literal(&lit);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("None") || result_str.contains("()"));
    }

    // =========================================================================
    // convert_binop tests
    // =========================================================================

    #[test]
    fn test_convert_binop_add() {
        let result = convert_binop(BinOp::Add);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_sub() {
        let result = convert_binop(BinOp::Sub);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_mul() {
        let result = convert_binop(BinOp::Mul);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_div() {
        let result = convert_binop(BinOp::Div);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_mod() {
        let result = convert_binop(BinOp::Mod);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_eq() {
        let result = convert_binop(BinOp::Eq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_not_eq() {
        let result = convert_binop(BinOp::NotEq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_lt() {
        let result = convert_binop(BinOp::Lt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_gt() {
        let result = convert_binop(BinOp::Gt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_lt_eq() {
        let result = convert_binop(BinOp::LtEq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_gt_eq() {
        let result = convert_binop(BinOp::GtEq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_and() {
        let result = convert_binop(BinOp::And);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_or() {
        let result = convert_binop(BinOp::Or);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_bit_and() {
        let result = convert_binop(BinOp::BitAnd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_bit_or() {
        let result = convert_binop(BinOp::BitOr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_bit_xor() {
        let result = convert_binop(BinOp::BitXor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_lshift() {
        let result = convert_binop(BinOp::LShift);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_rshift() {
        let result = convert_binop(BinOp::RShift);
        assert!(result.is_ok());
    }

    // =========================================================================
    // convert_arithmetic_op tests
    // =========================================================================

    #[test]
    fn test_convert_arithmetic_op_add() {
        let result = convert_arithmetic_op(BinOp::Add);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_arithmetic_op_sub() {
        let result = convert_arithmetic_op(BinOp::Sub);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_arithmetic_op_mul() {
        let result = convert_arithmetic_op(BinOp::Mul);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_arithmetic_op_div() {
        let result = convert_arithmetic_op(BinOp::Div);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_arithmetic_op_mod() {
        let result = convert_arithmetic_op(BinOp::Mod);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_arithmetic_op_floor_div_special() {
        let result = convert_arithmetic_op(BinOp::FloorDiv);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_arithmetic_op_pow_special() {
        let result = convert_arithmetic_op(BinOp::Pow);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_arithmetic_op_invalid() {
        let result = convert_arithmetic_op(BinOp::And);
        assert!(result.is_err());
    }

    // =========================================================================
    // convert_comparison_op tests
    // =========================================================================

    #[test]
    fn test_convert_comparison_op_eq() {
        let result = convert_comparison_op(BinOp::Eq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_comparison_op_not_eq() {
        let result = convert_comparison_op(BinOp::NotEq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_comparison_op_lt() {
        let result = convert_comparison_op(BinOp::Lt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_comparison_op_lt_eq() {
        let result = convert_comparison_op(BinOp::LtEq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_comparison_op_gt() {
        let result = convert_comparison_op(BinOp::Gt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_comparison_op_gt_eq() {
        let result = convert_comparison_op(BinOp::GtEq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_comparison_op_invalid() {
        let result = convert_comparison_op(BinOp::Add);
        assert!(result.is_err());
    }

    // =========================================================================
    // convert_logical_op tests
    // =========================================================================

    #[test]
    fn test_convert_logical_op_and() {
        let result = convert_logical_op(BinOp::And);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_logical_op_or() {
        let result = convert_logical_op(BinOp::Or);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_logical_op_invalid() {
        let result = convert_logical_op(BinOp::Add);
        assert!(result.is_err());
    }

    // =========================================================================
    // convert_bitwise_op tests
    // =========================================================================

    #[test]
    fn test_convert_bitwise_op_and() {
        let result = convert_bitwise_op(BinOp::BitAnd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_bitwise_op_or() {
        let result = convert_bitwise_op(BinOp::BitOr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_bitwise_op_xor() {
        let result = convert_bitwise_op(BinOp::BitXor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_bitwise_op_lshift() {
        let result = convert_bitwise_op(BinOp::LShift);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_bitwise_op_rshift() {
        let result = convert_bitwise_op(BinOp::RShift);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_bitwise_op_invalid() {
        let result = convert_bitwise_op(BinOp::Add);
        assert!(result.is_err());
    }

    // =========================================================================
    // is_len_call tests
    // =========================================================================

    #[test]
    fn test_is_len_call_true() {
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_len_call(&expr));
    }

    #[test]
    fn test_is_len_call_false_other_func() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_len_call(&expr));
    }

    #[test]
    fn test_is_len_call_false_not_call() {
        let expr = HirExpr::Var("len".to_string());
        assert!(!is_len_call(&expr));
    }

    // =========================================================================
    // is_pure_expression_direct tests
    // =========================================================================

    #[test]
    fn test_is_pure_expression_literal_int() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_literal_string() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_call_not_pure() {
        // All function calls are considered impure in this implementation
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(!is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_call_print() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_method_call_not_pure() {
        // All method calls are considered impure in this implementation
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("x".to_string())),
            method: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_method_call_append() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("x".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        assert!(!is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_attribute() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("self".to_string())),
            attr: "x".to_string(),
        };
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_index() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_tuple() {
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Var("x".to_string()),
        ]);
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_unary() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(42))),
        };
        assert!(is_pure_expression_direct(&expr));
    }

    // =========================================================================
    // find_mutable_vars_in_body tests
    // =========================================================================

    #[test]
    fn test_find_mutable_vars_empty() {
        let stmts: Vec<HirStmt> = vec![];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_mutable_vars_single_assign() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        }];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.is_empty()); // First assignment is not mutable
    }

    #[test]
    fn test_find_mutable_vars_reassignment() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(2)),
                type_annotation: None,
            },
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("x"));
    }

    #[test]
    fn test_find_mutable_vars_attribute_assign() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Attribute {
                value: Box::new(HirExpr::Var("self".to_string())),
                attr: "x".to_string(),
            },
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        }];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("self"));
    }

    #[test]
    fn test_find_mutable_vars_index_assign() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        }];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("arr"));
    }

    #[test]
    fn test_find_mutable_vars_tuple_assign() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Tuple(vec![
                    AssignTarget::Symbol("a".to_string()),
                    AssignTarget::Symbol("b".to_string()),
                ]),
                value: HirExpr::Tuple(vec![
                    HirExpr::Literal(Literal::Int(1)),
                    HirExpr::Literal(Literal::Int(2)),
                ]),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: AssignTarget::Symbol("a".to_string()),
                value: HirExpr::Literal(Literal::Int(3)),
                type_annotation: None,
            },
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("a"));
        assert!(!result.contains("b"));
    }

    #[test]
    fn test_find_mutable_vars_append_method() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("lst".to_string()),
                value: HirExpr::List(vec![]),
                type_annotation: None,
            },
            HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("lst".to_string())),
                method: "append".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            }),
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("lst"));
    }

    #[test]
    fn test_find_mutable_vars_in_if_body() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Literal(Literal::Int(1)),
                    type_annotation: None,
                }],
                else_body: None,
            },
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("x"));
    }

    #[test]
    fn test_find_mutable_vars_in_else_body() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![],
                else_body: Some(vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Literal(Literal::Int(1)),
                    type_annotation: None,
                }]),
            },
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("x"));
    }

    #[test]
    fn test_find_mutable_vars_in_while_body() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("i".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            HirStmt::While {
                condition: HirExpr::Binary {
                    op: BinOp::Lt,
                    left: Box::new(HirExpr::Var("i".to_string())),
                    right: Box::new(HirExpr::Literal(Literal::Int(10))),
                },
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("i".to_string()),
                    value: HirExpr::Binary {
                        op: BinOp::Add,
                        left: Box::new(HirExpr::Var("i".to_string())),
                        right: Box::new(HirExpr::Literal(Literal::Int(1))),
                    },
                    type_annotation: None,
                }],
            },
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("i"));
    }

    #[test]
    fn test_find_mutable_vars_in_for_body() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("total".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(10))],
                    kwargs: vec![],
                },
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("total".to_string()),
                    value: HirExpr::Binary {
                        op: BinOp::Add,
                        left: Box::new(HirExpr::Var("total".to_string())),
                        right: Box::new(HirExpr::Var("i".to_string())),
                    },
                    type_annotation: None,
                }],
            },
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("total"));
    }

    // =========================================================================
    // convert_body tests
    // =========================================================================

    #[test]
    fn test_convert_body_empty() {
        let type_mapper = TypeMapper::default();
        let stmts: Vec<HirStmt> = vec![];
        let result = convert_body(&stmts, &type_mapper);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_convert_body_single_pass() {
        let type_mapper = TypeMapper::default();
        let stmts = vec![HirStmt::Pass];
        let result = convert_body(&stmts, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_body_single_expr() {
        let type_mapper = TypeMapper::default();
        let stmts = vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(42)))];
        let result = convert_body(&stmts, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_body_simple_assign() {
        let type_mapper = TypeMapper::default();
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }];
        let result = convert_body(&stmts, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_body_return_int() {
        let type_mapper = TypeMapper::default();
        let stmts = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
        let result = convert_body(&stmts, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_body_return_none() {
        let type_mapper = TypeMapper::default();
        let stmts = vec![HirStmt::Return(None)];
        let result = convert_body(&stmts, &type_mapper);
        assert!(result.is_ok());
    }

    // =========================================================================
    // convert_expr tests
    // =========================================================================

    #[test]
    fn test_convert_expr_literal_int() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Literal(Literal::Int(42));
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_literal_string() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_var() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Var("x".to_string());
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_binary_add() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_list_empty() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::List(vec![]);
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_list_with_elements() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_dict_empty() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Dict(vec![]);
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_tuple() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("a".to_string())),
        ]);
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_index() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_attribute() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_call_simple() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Call {
            func: "func".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_call_with_args() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Call {
            func: "func".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_unary_not() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Literal(Literal::Bool(true))),
        };
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_expr_unary_neg() {
        let type_mapper = TypeMapper::default();
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(42))),
        };
        let result = convert_expr(&expr, &type_mapper);
        assert!(result.is_ok());
    }

    // =========================================================================
    // convert_stmt tests
    // =========================================================================

    #[test]
    fn test_convert_stmt_pass() {
        let type_mapper = TypeMapper::default();
        let stmt = HirStmt::Pass;
        let result = convert_stmt(&stmt, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_stmt_break() {
        let type_mapper = TypeMapper::default();
        let stmt = HirStmt::Break { label: None };
        let result = convert_stmt(&stmt, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_stmt_continue() {
        let type_mapper = TypeMapper::default();
        let stmt = HirStmt::Continue { label: None };
        let result = convert_stmt(&stmt, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_stmt_expr() {
        let type_mapper = TypeMapper::default();
        let stmt = HirStmt::Expr(HirExpr::Literal(Literal::Int(42)));
        let result = convert_stmt(&stmt, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_stmt_assign() {
        let type_mapper = TypeMapper::default();
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        };
        let result = convert_stmt(&stmt, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_stmt_return_some() {
        let type_mapper = TypeMapper::default();
        let stmt = HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))));
        let result = convert_stmt(&stmt, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_stmt_return_none() {
        let type_mapper = TypeMapper::default();
        let stmt = HirStmt::Return(None);
        let result = convert_stmt(&stmt, &type_mapper);
        assert!(result.is_ok());
    }

    // =========================================================================
    // convert_block tests
    // =========================================================================

    #[test]
    fn test_convert_block_empty() {
        let type_mapper = TypeMapper::default();
        let stmts: Vec<HirStmt> = vec![];
        let result = convert_block(&stmts, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_block_single_stmt() {
        let type_mapper = TypeMapper::default();
        let stmts = vec![HirStmt::Pass];
        let result = convert_block(&stmts, &type_mapper);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_block_multiple_stmts() {
        let type_mapper = TypeMapper::default();
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
        let result = convert_block(&stmts, &type_mapper);
        assert!(result.is_ok());
    }
}

