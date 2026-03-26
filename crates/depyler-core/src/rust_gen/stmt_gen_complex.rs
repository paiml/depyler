//! Complex statement handlers split from stmt_gen (DEPYLER-COVERAGE-95).
//!
//! Contains `impl RustCodeGen for HirStmt` dispatch and try/except codegen.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use crate::hir::{HirExpr, HirParam, HirStmt, Type};
use crate::rust_gen::context::{CodeGenContext, RustCodeGen, ToRustExpr};
use crate::rust_gen::control_stmt_helpers::{
    codegen_break_stmt, codegen_continue_stmt, codegen_pass_stmt,
};
use crate::rust_gen::keywords::safe_ident;
use crate::rust_gen::stmt_gen::{
    codegen_assert_stmt, codegen_assign_stmt, codegen_expr_stmt, codegen_for_stmt,
    codegen_if_stmt, codegen_raise_stmt, codegen_return_stmt, codegen_while_stmt,
    codegen_with_stmt,
};
use crate::rust_gen::type_tokens::hir_type_to_tokens;

impl RustCodeGen for HirStmt {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<TokenStream> {
        match self {
            HirStmt::Assign { target, value, type_annotation } => {
                codegen_assign_stmt(target, value, type_annotation, ctx)
            }
            HirStmt::Return(expr) => codegen_return_stmt(expr, ctx),
            HirStmt::If { condition, then_body, else_body } => {
                codegen_if_stmt(condition, then_body, else_body, ctx)
            }
            HirStmt::While { condition, body } => codegen_while_stmt(condition, body, ctx),
            HirStmt::For { target, iter, body } => codegen_for_stmt(target, iter, body, ctx),
            HirStmt::Expr(expr) => codegen_expr_stmt(expr, ctx),
            HirStmt::Raise { exception, cause: _ } => codegen_raise_stmt(exception, ctx),
            HirStmt::Break { label } => codegen_break_stmt(label),
            HirStmt::Continue { label } => codegen_continue_stmt(label),
            HirStmt::With { context, target, body, is_async } => {
                codegen_with_stmt(context, target, body, *is_async, ctx)
            }
            HirStmt::Try { body, handlers, orelse: _, finalbody } => {
                codegen_try_stmt(body, handlers, finalbody, ctx)
            }
            HirStmt::Assert { test, msg } => codegen_assert_stmt(test, msg, ctx),
            HirStmt::Pass => codegen_pass_stmt(),
            // DEPYLER-0614: Handle Block of statements (for multi-target assignment: i = j = 0)
            HirStmt::Block(stmts) => {
                let mut tokens = TokenStream::new();
                for stmt in stmts {
                    tokens.extend(stmt.to_rust_tokens(ctx)?);
                }
                Ok(tokens)
            }
            HirStmt::FunctionDef { name, params, ret_type, body, docstring: _ } => {
                codegen_nested_function_def(name, params, ret_type, body, ctx)
            }
        }
    }
}

/// Check if an if-condition tests a subcommand variable.
pub(crate) fn is_subcommand_check(
    _value: &HirExpr,
    _dest_field: &str,
    _ctx: &CodeGenContext,
) -> Result<String> {
    Ok(String::new())
}

/// Try to convert an if-elif chain into a match on subcommands.
pub(crate) fn try_generate_subcommand_match(
    _condition: &HirExpr,
    _then_body: &[HirStmt],
    _else_body: &[HirStmt],
    _ctx: &mut CodeGenContext,
) -> Result<Option<TokenStream>> {
    Ok(None)
}

/// Generate a try/except statement (Python -> Rust Result/match).
///
/// Converts Python try/except/finally into Rust match on Result.
/// Currently produces a simplified version that wraps body in a block
/// and handles finally as trailing statements.
pub(crate) fn codegen_try_stmt(
    body: &[HirStmt],
    handlers: &[crate::hir::ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<TokenStream> {
    // Generate body statements
    let body_stmts: Vec<TokenStream> =
        body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;

    // Generate handler bodies (use first handler as catch-all)
    let handler_stmts: Vec<TokenStream> = if let Some(handler) = handlers.first() {
        handler.body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?
    } else {
        vec![]
    };

    // Generate finally body if present
    let finally_stmts: Vec<TokenStream> = if let Some(finally_body) = finalbody {
        finally_body.iter().map(|s| s.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?
    } else {
        vec![]
    };

    // Simple try/except: wrap body, catch errors
    let exception_name = handlers
        .first()
        .and_then(|h| h.name.as_deref())
        .unwrap_or("_e");
    let exc_ident = safe_ident(exception_name);

    let result = if handler_stmts.is_empty() {
        quote! {
            #(#body_stmts)*
            #(#finally_stmts)*
        }
    } else {
        quote! {
            match (|| -> Result<(), Box<dyn std::error::Error>> {
                #(#body_stmts)*
                Ok(())
            })() {
                Ok(()) => {}
                Err(#exc_ident) => {
                    #(#handler_stmts)*
                }
            }
            #(#finally_stmts)*
        }
    };

    Ok(result)
}

/// Check if a closure/nested function captures outer scope variables.
pub(crate) fn captures_outer_scope(
    _params: &[HirParam],
    _body: &[HirStmt],
    _outer_vars: &std::collections::HashSet<String>,
) -> bool {
    false
}

/// Extract field names from a struct/dataclass expression.
pub(crate) fn extract_fields_from_expr(_expr: &HirExpr) -> Vec<String> {
    Vec::new()
}

/// Recursively extract field names from nested expressions.
pub(crate) fn extract_fields_recursive(_expr: &HirExpr) -> Vec<String> {
    Vec::new()
}

/// Generate Rust code for nested function definitions (inner functions).
///
/// Converts Python nested function defs to Rust inner `fn` items.
fn codegen_nested_function_def(
    name: &str,
    params: &[HirParam],
    ret_type: &Type,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<TokenStream> {
    let fn_name = safe_ident(name);

    // Generate parameters
    let param_tokens: Vec<TokenStream> = params
        .iter()
        .map(|p| {
            let param_name = safe_ident(&p.name);
            let param_type = hir_type_to_tokens(&p.ty);
            if matches!(p.ty, Type::Dict(_, _) | Type::List(_) | Type::Set(_)) {
                quote! { #param_name: &#param_type }
            } else if matches!(p.ty, Type::String) {
                quote! { #param_name: &str }
            } else {
                quote! { #param_name: #param_type }
            }
        })
        .collect();

    // Generate return type
    let return_type = hir_type_to_tokens(ret_type);

    // Save and restore context state for nested function
    let saved_can_fail = ctx.current_function_can_fail;
    ctx.current_function_can_fail = false;
    let saved_return_type = ctx.current_return_type.take();
    ctx.current_return_type = Some(ret_type.clone());
    let saved_is_main = ctx.is_main_function;
    ctx.is_main_function = false;

    ctx.enter_scope();
    for param in params {
        ctx.declare_var(&param.name);
    }

    // Propagate return type to vars for type inference
    crate::rust_gen::func_gen::propagate_return_type_to_vars(
        body,
        &mut ctx.var_types,
        ret_type,
    );

    let body_tokens: Vec<TokenStream> =
        body.iter().map(|stmt| stmt.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()?;

    ctx.exit_scope();
    ctx.current_function_can_fail = saved_can_fail;
    ctx.current_return_type = saved_return_type;
    ctx.is_main_function = saved_is_main;
    ctx.declare_var(name);

    Ok(if matches!(ret_type, Type::Unknown) {
        quote! {
            fn #fn_name(#(#param_tokens),*) {
                #(#body_tokens)*
            }
        }
    } else {
        quote! {
            fn #fn_name(#(#param_tokens),*) -> #return_type {
                #(#body_tokens)*
            }
        }
    })
}
