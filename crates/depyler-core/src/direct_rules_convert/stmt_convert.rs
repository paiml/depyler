//! Statement and block conversion for direct rules
//!
//! Contains convert_stmt, convert_stmt_with_context, convert_block functions.
//! Each HirStmt variant is handled by a dedicated private helper function
//! to keep cognitive complexity low.

use crate::direct_rules::{make_ident, type_to_rust_type};
use crate::hir::*;
use crate::rust_gen::keywords::safe_ident;
use crate::type_mapper::TypeMapper;
use anyhow::{bail, Result};
use quote::quote;
use syn::parse_quote;

use super::body_convert::*;
use super::{convert_condition_expr, convert_expr_with_param_types};

/// Bundles the common context parameters threaded through statement conversion.
struct StmtContext<'a> {
    type_mapper: &'a TypeMapper,
    is_classmethod: bool,
    vararg_functions: &'a std::collections::HashSet<String>,
    param_types: &'a std::collections::HashMap<String, Type>,
}

impl<'a> StmtContext<'a> {
    /// Convert an expression using the bundled context parameters.
    fn convert_expr(&self, expr: &HirExpr) -> Result<syn::Expr> {
        convert_expr_with_param_types(
            expr,
            self.type_mapper,
            self.is_classmethod,
            self.vararg_functions,
            self.param_types,
        )
    }

    /// Convert a condition expression with truthiness coercion.
    fn convert_condition(&self, expr: &HirExpr) -> Result<syn::Expr> {
        convert_condition_expr(
            expr,
            self.type_mapper,
            self.is_classmethod,
            self.vararg_functions,
            self.param_types,
        )
    }

    /// Convert a list of statements into a syn::Block.
    fn convert_block(&self, stmts: &[HirStmt]) -> Result<syn::Block> {
        convert_block_with_context(
            stmts,
            self.type_mapper,
            self.is_classmethod,
            self.vararg_functions,
            self.param_types,
        )
    }
}

#[allow(dead_code)] // Used by tests via pub(crate) use stmt_convert::*
pub(crate) fn convert_stmt(stmt: &HirStmt, type_mapper: &TypeMapper) -> Result<syn::Stmt> {
    // Use empty vararg_functions and param_types for backward compatibility
    static EMPTY_SET: std::sync::OnceLock<std::collections::HashSet<String>> =
        std::sync::OnceLock::new();
    static EMPTY_MAP: std::sync::OnceLock<std::collections::HashMap<String, Type>> =
        std::sync::OnceLock::new();
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
            let value_expr = convert_expr_with_param_types(
                value,
                type_mapper,
                is_classmethod,
                vararg_functions,
                param_types,
            )?;
            convert_assign_stmt_with_mutable_vars(target, value_expr, type_mapper, mutable_vars)
        }
        // For all other statement types, delegate to convert_stmt_with_context
        // They don't generate new variable bindings so mutable_vars doesn't matter
        _ => convert_stmt_with_context(
            stmt,
            type_mapper,
            is_classmethod,
            vararg_functions,
            param_types,
        ),
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
    let ctx = StmtContext {
        type_mapper,
        is_classmethod,
        vararg_functions,
        param_types,
    };
    match stmt {
        HirStmt::Assign { target, value, .. } => convert_assign(&ctx, target, value),
        HirStmt::Return(expr) => convert_return(&ctx, expr),
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => convert_if(&ctx, condition, then_body, else_body),
        HirStmt::While { condition, body } => convert_while(&ctx, condition, body),
        HirStmt::For { target, iter, body } => convert_for(&ctx, target, iter, body),
        HirStmt::Expr(expr) => convert_expr_stmt(&ctx, expr),
        HirStmt::Raise {
            exception,
            cause: _,
        } => convert_raise(&ctx, exception),
        HirStmt::Break { label } => convert_break(label),
        HirStmt::Continue { label } => convert_continue(label),
        HirStmt::With {
            context,
            target,
            body,
            ..
        } => convert_with(&ctx, context, target, body),
        HirStmt::Try {
            body,
            handlers,
            orelse: _,
            finalbody,
        } => convert_try(&ctx, body, handlers, finalbody),
        HirStmt::Assert { test, msg } => convert_assert(&ctx, test, msg),
        HirStmt::Pass => convert_pass(),
        HirStmt::Block(stmts) => convert_block_stmt(&ctx, stmts),
        HirStmt::FunctionDef {
            name,
            params,
            ret_type,
            body,
            ..
        } => convert_function_def(&ctx, name, params, ret_type, body),
    }
}

fn convert_assign(
    ctx: &StmtContext<'_>,
    target: &AssignTarget,
    value: &HirExpr,
) -> Result<syn::Stmt> {
    // For assignments, we need to convert the value expression with classmethod context
    // DEPYLER-0704: Pass param_types for type coercion
    let value_expr = ctx.convert_expr(value)?;
    convert_assign_stmt_with_expr(target, value_expr, ctx.type_mapper)
}

fn convert_return(ctx: &StmtContext<'_>, expr: &Option<HirExpr>) -> Result<syn::Stmt> {
    let ret_expr = if let Some(e) = expr {
        // DEPYLER-0704: Pass param_types for type coercion in return expressions
        ctx.convert_expr(e)?
    } else {
        parse_quote! { () }
    };
    Ok(syn::Stmt::Expr(
        parse_quote! { return #ret_expr },
        Some(Default::default()),
    ))
}

fn convert_if(
    ctx: &StmtContext<'_>,
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
) -> Result<syn::Stmt> {
    // DEPYLER-1096: Use convert_condition_expr for truthiness coercion
    let cond = ctx.convert_condition(condition)?;
    let then_block = ctx.convert_block(then_body)?;

    let if_expr = if let Some(else_stmts) = else_body {
        let else_block = ctx.convert_block(else_stmts)?;
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

fn convert_while(
    ctx: &StmtContext<'_>,
    condition: &HirExpr,
    body: &[HirStmt],
) -> Result<syn::Stmt> {
    // DEPYLER-1096: Use convert_condition_expr for truthiness coercion
    let cond = ctx.convert_condition(condition)?;
    let body_block = ctx.convert_block(body)?;

    let while_expr = parse_quote! {
        while #cond #body_block
    };

    Ok(syn::Stmt::Expr(while_expr, Some(Default::default())))
}

fn convert_for(
    ctx: &StmtContext<'_>,
    target: &AssignTarget,
    iter: &HirExpr,
    body: &[HirStmt],
) -> Result<syn::Stmt> {
    let target_pattern = convert_for_target(target)?;
    // GH-207-PHASE2: Handle dict.items() in for loop context
    // Python: for k, v in dict.items() -> Rust: for (k, v) in dict.iter()
    let iter_expr = convert_for_iter(ctx, iter)?;
    let body_block = ctx.convert_block(body)?;

    let for_expr = parse_quote! {
        for #target_pattern in #iter_expr #body_block
    };

    Ok(syn::Stmt::Expr(for_expr, Some(Default::default())))
}

/// Generate target pattern based on AssignTarget type for for-loops.
fn convert_for_target(target: &AssignTarget) -> Result<syn::Pat> {
    match target {
        AssignTarget::Symbol(name) => {
            let ident = make_ident(name);
            Ok(parse_quote! { #ident })
        }
        AssignTarget::Tuple(targets) => {
            let idents: Vec<syn::Ident> = targets
                .iter()
                .map(|t| match t {
                    AssignTarget::Symbol(s) => Ok(make_ident(s)),
                    _ => bail!("Nested tuple unpacking not supported in for loops"),
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(parse_quote! { (#(#idents),*) })
        }
        _ => bail!("Unsupported for loop target type"),
    }
}

/// Convert the iterator expression, handling dict method calls specially.
fn convert_for_iter(ctx: &StmtContext<'_>, iter: &HirExpr) -> Result<syn::Expr> {
    if let HirExpr::MethodCall { object, method, .. } = iter {
        if method == "items" {
            let obj_expr = ctx.convert_expr(object)?;
            return Ok(parse_quote! { #obj_expr.iter() });
        } else if method == "keys" {
            let obj_expr = ctx.convert_expr(object)?;
            return Ok(parse_quote! { #obj_expr.keys() });
        } else if method == "values" {
            let obj_expr = ctx.convert_expr(object)?;
            return Ok(parse_quote! { #obj_expr.values() });
        }
    }
    ctx.convert_expr(iter)
}

fn convert_expr_stmt(ctx: &StmtContext<'_>, expr: &HirExpr) -> Result<syn::Stmt> {
    // DEPYLER-0701: Detect expressions without side effects and wrap with `let _ =`
    // to avoid "path statement with no effect" and "unused arithmetic operation" warnings
    if is_pure_expression_direct(expr) {
        let rust_expr = ctx.convert_expr(expr)?;
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
        let rust_expr = ctx.convert_expr(expr)?;
        Ok(syn::Stmt::Expr(rust_expr, Some(Default::default())))
    }
}

fn convert_raise(ctx: &StmtContext<'_>, exception: &Option<HirExpr>) -> Result<syn::Stmt> {
    // Convert to Rust panic for direct rules
    let panic_expr = if let Some(exc) = exception {
        let exc_expr = ctx.convert_expr(exc)?;
        parse_quote! { panic!("Exception: {}", #exc_expr) }
    } else {
        parse_quote! { panic!("Exception raised") }
    };
    Ok(syn::Stmt::Expr(panic_expr, Some(Default::default())))
}

fn convert_break(label: &Option<String>) -> Result<syn::Stmt> {
    let break_expr = if let Some(label_name) = label {
        let label_ident =
            syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
        parse_quote! { break #label_ident }
    } else {
        parse_quote! { break }
    };
    Ok(syn::Stmt::Expr(break_expr, Some(Default::default())))
}

fn convert_continue(label: &Option<String>) -> Result<syn::Stmt> {
    let continue_expr = if let Some(label_name) = label {
        let label_ident =
            syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
        parse_quote! { continue #label_ident }
    } else {
        parse_quote! { continue }
    };
    Ok(syn::Stmt::Expr(continue_expr, Some(Default::default())))
}

fn convert_with(
    ctx: &StmtContext<'_>,
    context: &HirExpr,
    target: &Option<String>,
    body: &[HirStmt],
) -> Result<syn::Stmt> {
    // Convert context expression
    let context_expr = ctx.convert_expr(context)?;

    // Convert body to a block
    let body_block = ctx.convert_block(body)?;

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

fn convert_try(
    ctx: &StmtContext<'_>,
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
) -> Result<syn::Stmt> {
    // Convert try body
    let try_stmts = ctx.convert_block(body)?;

    // Convert finally block if present
    let finally_block = finalbody
        .as_ref()
        .map(|fb| ctx.convert_block(fb))
        .transpose()?;

    // Convert except handlers (use first handler for simplicity)
    if let Some(handler) = handlers.first() {
        convert_try_with_handler(ctx, &try_stmts, handler, finally_block)
    } else {
        convert_try_without_handler(&try_stmts, finally_block)
    }
}

fn convert_try_with_handler(
    ctx: &StmtContext<'_>,
    try_stmts: &syn::Block,
    handler: &ExceptHandler,
    finally_block: Option<syn::Block>,
) -> Result<syn::Stmt> {
    let handler_block = ctx.convert_block(&handler.body)?;

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
}

fn convert_try_without_handler(
    try_stmts: &syn::Block,
    finally_block: Option<syn::Block>,
) -> Result<syn::Stmt> {
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

fn convert_assert(
    ctx: &StmtContext<'_>,
    test: &HirExpr,
    msg: &Option<HirExpr>,
) -> Result<syn::Stmt> {
    // Generate assert! macro call
    let test_expr = ctx.convert_expr(test)?;
    let assert_macro: syn::Stmt = if let Some(message) = msg {
        let msg_expr = ctx.convert_expr(message)?;
        parse_quote! { assert!(#test_expr, "{}", #msg_expr); }
    } else {
        parse_quote! { assert!(#test_expr); }
    };
    Ok(assert_macro)
}

fn convert_pass() -> Result<syn::Stmt> {
    // Pass statement generates empty statement
    Ok(syn::Stmt::Expr(parse_quote! { {} }, None))
}

/// DEPYLER-0614: Handle Block of statements - convert first statement
/// Note: This is a simplification; blocks are flattened during codegen
fn convert_block_stmt(ctx: &StmtContext<'_>, stmts: &[HirStmt]) -> Result<syn::Stmt> {
    if stmts.is_empty() {
        Ok(syn::Stmt::Expr(parse_quote! { {} }, None))
    } else {
        convert_stmt_with_context(
            &stmts[0],
            ctx.type_mapper,
            ctx.is_classmethod,
            ctx.vararg_functions,
            ctx.param_types,
        )
    }
}

/// DEPYLER-0840: Properly generate nested functions as closures
/// Previously this just returned {} causing E0425 "cannot find value" errors
fn convert_function_def(
    ctx: &StmtContext<'_>,
    name: &str,
    params: &[HirParam],
    ret_type: &Type,
    body: &[HirStmt],
) -> Result<syn::Stmt> {
    let fn_name = safe_ident(name);

    // Generate parameter tokens
    let param_tokens: Vec<proc_macro2::TokenStream> = params
        .iter()
        .map(|p| {
            let param_name = safe_ident(&p.name);
            let param_type = type_to_rust_type(&p.ty, ctx.type_mapper);
            quote! { #param_name: #param_type }
        })
        .collect();

    // Generate body statements
    let body_stmts: Vec<syn::Stmt> = body
        .iter()
        .filter_map(|stmt| {
            convert_stmt_with_context(
                stmt,
                ctx.type_mapper,
                ctx.is_classmethod,
                ctx.vararg_functions,
                ctx.param_types,
            )
            .ok()
        })
        .collect();

    // Generate return type if not Unknown
    let closure_expr: proc_macro2::TokenStream = if matches!(ret_type, Type::Unknown) {
        quote! {
            let #fn_name = move |#(#param_tokens),*| {
                #(#body_stmts)*
            };
        }
    } else {
        let return_type = type_to_rust_type(ret_type, ctx.type_mapper);
        quote! {
            let #fn_name = move |#(#param_tokens),*| -> #return_type {
                #(#body_stmts)*
            };
        }
    };

    Ok(syn::parse2::<syn::Stmt>(closure_expr).unwrap_or_else(|_| parse_quote! { {} }))
}

#[allow(dead_code)]
pub(crate) fn convert_block(stmts: &[HirStmt], type_mapper: &TypeMapper) -> Result<syn::Block> {
    // Use empty vararg_functions for backward compatibility
    static EMPTY_SET: std::sync::OnceLock<std::collections::HashSet<String>> =
        std::sync::OnceLock::new();
    static EMPTY_MAP: std::sync::OnceLock<std::collections::HashMap<String, Type>> =
        std::sync::OnceLock::new();
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
    let rust_stmts = convert_body_with_context(
        stmts,
        type_mapper,
        is_classmethod,
        vararg_functions,
        param_types,
    )?;
    Ok(syn::Block {
        brace_token: Default::default(),
        stmts: rust_stmts,
    })
}
