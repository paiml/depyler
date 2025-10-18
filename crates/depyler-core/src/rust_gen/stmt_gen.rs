//! Statement code generation
//!
//! This module handles converting HIR statements to Rust token streams.
//! It includes all statement conversion helpers and the HirStmt RustCodeGen trait implementation.

use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, RustCodeGen, ToRustExpr};
use crate::rust_gen::type_gen::rust_type_to_syn;
use anyhow::{bail, Result};
use quote::quote;
use syn::{self, parse_quote};

/// Helper to build nested dictionary access for assignment
/// Returns (base_expr, access_chain) where access_chain is a vec of index expressions
fn extract_nested_indices_tokens(
    expr: &HirExpr,
    ctx: &mut CodeGenContext,
) -> Result<(syn::Expr, Vec<syn::Expr>)> {
    let mut indices = Vec::new();
    let mut current = expr;

    // Walk up the chain collecting indices
    loop {
        match current {
            HirExpr::Index { base, index } => {
                let index_expr = index.to_rust_expr(ctx)?;
                indices.push(index_expr);
                current = base;
            }
            _ => {
                // We've reached the base
                let base_expr = current.to_rust_expr(ctx)?;
                indices.reverse(); // We collected from inner to outer, need outer to inner
                return Ok((base_expr, indices));
            }
        }
    }
}

/// Check if a type annotation requires explicit conversion
///
/// For numeric types like Int, we always apply conversion to ensure correctness
/// even after optimizations like CSE transform the expression.
/// Complexity: 2 (simple match)
fn needs_type_conversion(target_type: &Type) -> bool {
    // For Int annotations, always apply conversion to handle cases where
    // the value might be usize (from len(), range, etc.)
    matches!(target_type, Type::Int)
}

/// Apply type conversion to value expression
///
/// Wraps the expression with appropriate conversion (e.g., `as i32`)
/// Complexity: 2 (simple match)
fn apply_type_conversion(value_expr: syn::Expr, target_type: &Type) -> syn::Expr {
    match target_type {
        Type::Int => {
            // Convert to i32 using 'as' cast
            // This handles usize->i32 conversions and is a no-op if already i32
            // Note: Removed defensive parens - Rust's precedence rules handle this correctly
            parse_quote! { #value_expr as i32 }
        }
        _ => value_expr,
    }
}

// ============================================================================
// Statement Code Generation Helpers (DEPYLER-0140 Phase 1)
// Extracted to reduce complexity of HirStmt::to_rust_tokens
// ============================================================================

/// Generate code for Pass statement (no-op)
#[inline]
pub(crate) fn codegen_pass_stmt() -> Result<proc_macro2::TokenStream> {
    Ok(quote! {})
}

/// Generate code for Assert statement
#[inline]
pub(crate) fn codegen_assert_stmt(
    test: &HirExpr,
    msg: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let test_expr = test.to_rust_expr(ctx)?;

    if let Some(message_expr) = msg {
        let msg_tokens = message_expr.to_rust_expr(ctx)?;
        Ok(quote! { assert!(#test_expr, "{}", #msg_tokens); })
    } else {
        Ok(quote! { assert!(#test_expr); })
    }
}

/// Generate code for Break statement with optional label
#[inline]
pub(crate) fn codegen_break_stmt(label: &Option<String>) -> Result<proc_macro2::TokenStream> {
    if let Some(label_name) = label {
        let label_ident =
            syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
        Ok(quote! { break #label_ident; })
    } else {
        Ok(quote! { break; })
    }
}

/// Generate code for Continue statement with optional label
#[inline]
pub(crate) fn codegen_continue_stmt(label: &Option<String>) -> Result<proc_macro2::TokenStream> {
    if let Some(label_name) = label {
        let label_ident =
            syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
        Ok(quote! { continue #label_ident; })
    } else {
        Ok(quote! { continue; })
    }
}

/// Generate code for expression statement
#[inline]
pub(crate) fn codegen_expr_stmt(
    expr: &HirExpr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let expr_tokens = expr.to_rust_expr(ctx)?;
    Ok(quote! { #expr_tokens; })
}

// ============================================================================
// Statement Code Generation Helpers (DEPYLER-0140 Phase 2)
// Medium-complexity handlers extracted from HirStmt::to_rust_tokens
// ============================================================================

/// Generate code for Return statement with optional expression
#[inline]
pub(crate) fn codegen_return_stmt(
    expr: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    if let Some(e) = expr {
        let expr_tokens = e.to_rust_expr(ctx)?;

        // Check if return type is Optional and wrap value in Some()
        let is_optional_return =
            matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));

        // Check if the expression is None literal
        let is_none_literal = matches!(e, HirExpr::Literal(Literal::None));

        if ctx.current_function_can_fail {
            if is_optional_return && !is_none_literal {
                // Wrap value in Some() for Optional return types
                Ok(quote! { return Ok(Some(#expr_tokens)); })
            } else {
                Ok(quote! { return Ok(#expr_tokens); })
            }
        } else if is_optional_return && !is_none_literal {
            // Wrap value in Some() for Optional return types
            Ok(quote! { return Some(#expr_tokens); })
        } else {
            Ok(quote! { return #expr_tokens; })
        }
    } else if ctx.current_function_can_fail {
        // No expression - check if return type is Optional
        let is_optional_return =
            matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));
        if is_optional_return {
            Ok(quote! { return Ok(None); })
        } else {
            Ok(quote! { return Ok(()); })
        }
    } else {
        Ok(quote! { return; })
    }
}

/// Generate code for While loop statement
#[inline]
pub(crate) fn codegen_while_stmt(
    condition: &HirExpr,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let cond = condition.to_rust_expr(ctx)?;
    ctx.enter_scope();
    let body_stmts: Vec<_> = body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();
    Ok(quote! {
        while #cond {
            #(#body_stmts)*
        }
    })
}

/// Generate code for Raise (exception) statement
#[inline]
pub(crate) fn codegen_raise_stmt(
    exception: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // For V1, we'll implement basic error handling
    if let Some(exc) = exception {
        let exc_expr = exc.to_rust_expr(ctx)?;
        Ok(quote! { return Err(#exc_expr); })
    } else {
        // Re-raise or bare raise - use generic error
        Ok(quote! { return Err("Exception raised".into()); })
    }
}

/// Generate code for With (context manager) statement
#[inline]
pub(crate) fn codegen_with_stmt(
    context: &HirExpr,
    target: &Option<String>,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Convert context expression
    let context_expr = context.to_rust_expr(ctx)?;

    // Convert body statements
    let body_stmts: Vec<_> = body
        .iter()
        .map(|stmt| stmt.to_rust_tokens(ctx))
        .collect::<Result<_>>()?;

    // Generate code that calls __enter__() and binds the result
    // Note: __exit__() is not yet called (Drop trait implementation pending)
    if let Some(var_name) = target {
        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
        ctx.declare_var(var_name);
        Ok(quote! {
            {
                let _context = #context_expr;
                let #var_ident = _context.__enter__();
                #(#body_stmts)*
            }
        })
    } else {
        Ok(quote! {
            {
                let _context = #context_expr;
                #(#body_stmts)*
            }
        })
    }
}

// ============================================================================
// Statement Code Generation Helpers (DEPYLER-0140 Phase 3)
// Complex handlers extracted from HirStmt::to_rust_tokens
// ============================================================================

/// Generate code for If statement with optional else clause
#[inline]
pub(crate) fn codegen_if_stmt(
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let cond = condition.to_rust_expr(ctx)?;
    ctx.enter_scope();
    let then_stmts: Vec<_> = then_body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    if let Some(else_stmts) = else_body {
        ctx.enter_scope();
        let else_tokens: Vec<_> = else_stmts
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();
        Ok(quote! {
            if #cond {
                #(#then_stmts)*
            } else {
                #(#else_tokens)*
            }
        })
    } else {
        Ok(quote! {
            if #cond {
                #(#then_stmts)*
            }
        })
    }
}

/// Generate code for For loop statement
#[inline]
pub(crate) fn codegen_for_stmt(
    target: &AssignTarget,
    iter: &HirExpr,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Generate target pattern based on AssignTarget type
    let target_pattern: syn::Pat = match target {
        AssignTarget::Symbol(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        AssignTarget::Tuple(targets) => {
            let idents: Vec<syn::Ident> = targets
                .iter()
                .map(|t| match t {
                    AssignTarget::Symbol(s) => {
                        syn::Ident::new(s, proc_macro2::Span::call_site())
                    }
                    _ => panic!("Nested tuple unpacking not supported in for loops"),
                })
                .collect();
            parse_quote! { (#(#idents),*) }
        }
        _ => bail!("Unsupported for loop target type"),
    };

    let mut iter_expr = iter.to_rust_expr(ctx)?;

    // Check if we're iterating over a borrowed collection
    // If iter is a simple variable that refers to a borrowed collection (e.g., &Vec<T>),
    // we need to add .iter() to properly iterate over it
    if let HirExpr::Var(_var_name) = iter {
        // This is a simple heuristic: if the expression is just a variable name,
        // it's likely a parameter or local var that might be borrowed
        // The generated code already has the variable as borrowed (e.g., data: &Vec<T>)
        // so we need to call .iter() on it
        iter_expr = parse_quote! { #iter_expr.iter() };
    }

    ctx.enter_scope();
    // Declare all variables from the target pattern
    match target {
        AssignTarget::Symbol(name) => ctx.declare_var(name),
        AssignTarget::Tuple(targets) => {
            for t in targets {
                if let AssignTarget::Symbol(s) = t {
                    ctx.declare_var(s);
                }
            }
        }
        _ => {}
    }
    let body_stmts: Vec<_> = body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();
    Ok(quote! {
        for #target_pattern in #iter_expr {
            #(#body_stmts)*
        }
    })
}

/// Generate code for Assign statement (variable/index/attribute/tuple assignment)
#[inline]
pub(crate) fn codegen_assign_stmt(
    target: &AssignTarget,
    value: &HirExpr,
    type_annotation: &Option<Type>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0232: Track variable types for class instances
    // This allows proper method dispatch for user-defined classes
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::Call { func, .. } = value {
            // Check if this is a user-defined class constructor
            if ctx.class_names.contains(func) {
                ctx.var_types.insert(var_name.clone(), Type::Custom(func.clone()));
            }
        }
    }

    let mut value_expr = value.to_rust_expr(ctx)?;

    // If there's a type annotation, handle type conversions
    let type_annotation_tokens = if let Some(target_type) = type_annotation {
        let target_rust_type = ctx.type_mapper.map_type(target_type);
        let target_syn_type = rust_type_to_syn(&target_rust_type)?;

        // Check if we need type conversion (e.g., usize to i32)
        if needs_type_conversion(target_type) {
            value_expr = apply_type_conversion(value_expr, target_type);
        }

        Some(quote! { : #target_syn_type })
    } else {
        None
    };

    match target {
        AssignTarget::Symbol(symbol) => {
            codegen_assign_symbol(symbol, value_expr, type_annotation_tokens, ctx)
        }
        AssignTarget::Index { base, index } => codegen_assign_index(base, index, value_expr, ctx),
        AssignTarget::Attribute { value, attr } => {
            codegen_assign_attribute(value, attr, value_expr, ctx)
        }
        AssignTarget::Tuple(targets) => {
            codegen_assign_tuple(targets, value_expr, type_annotation_tokens, ctx)
        }
    }
}

/// Generate code for symbol (variable) assignment
#[inline]
pub(crate) fn codegen_assign_symbol(
    symbol: &str,
    value_expr: syn::Expr,
    type_annotation_tokens: Option<proc_macro2::TokenStream>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let target_ident = syn::Ident::new(symbol, proc_macro2::Span::call_site());

    // Inside generators, check if variable is a state variable
    if ctx.in_generator && ctx.generator_state_vars.contains(symbol) {
        // State variable assignment: self.field = value
        Ok(quote! { self.#target_ident = #value_expr; })
    } else if ctx.is_declared(symbol) {
        // Variable already exists, just assign
        Ok(quote! { #target_ident = #value_expr; })
    } else {
        // First declaration - check if variable needs mut
        ctx.declare_var(symbol);
        if ctx.mutable_vars.contains(symbol) {
            if let Some(type_ann) = type_annotation_tokens {
                Ok(quote! { let mut #target_ident #type_ann = #value_expr; })
            } else {
                Ok(quote! { let mut #target_ident = #value_expr; })
            }
        } else if let Some(type_ann) = type_annotation_tokens {
            Ok(quote! { let #target_ident #type_ann = #value_expr; })
        } else {
            Ok(quote! { let #target_ident = #value_expr; })
        }
    }
}

/// Generate code for index (dictionary/list subscript) assignment
#[inline]
pub(crate) fn codegen_assign_index(
    base: &HirExpr,
    index: &HirExpr,
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let final_index = index.to_rust_expr(ctx)?;

    // Extract the base and all intermediate indices
    let (base_expr, indices) = extract_nested_indices_tokens(base, ctx)?;

    if indices.is_empty() {
        // Simple assignment: d[k] = v
        Ok(quote! { #base_expr.insert(#final_index, #value_expr); })
    } else {
        // Nested assignment: build chain of get_mut calls
        let mut chain = quote! { #base_expr };
        for idx in &indices {
            chain = quote! {
                #chain.get_mut(&#idx).unwrap()
            };
        }

        Ok(quote! { #chain.insert(#final_index, #value_expr); })
    }
}

/// Generate code for attribute (struct field) assignment
#[inline]
pub(crate) fn codegen_assign_attribute(
    base: &HirExpr,
    attr: &str,
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let base_expr = base.to_rust_expr(ctx)?;
    let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
    Ok(quote! { #base_expr.#attr_ident = #value_expr; })
}

/// Generate code for tuple unpacking assignment
#[inline]
pub(crate) fn codegen_assign_tuple(
    targets: &[AssignTarget],
    value_expr: syn::Expr,
    _type_annotation_tokens: Option<proc_macro2::TokenStream>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Check if all targets are simple symbols
    let all_symbols: Option<Vec<&str>> = targets
        .iter()
        .map(|t| match t {
            AssignTarget::Symbol(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    match all_symbols {
        Some(symbols) => {
            let all_declared = symbols.iter().all(|s| ctx.is_declared(s));

            if all_declared {
                // All variables exist, do reassignment
                let idents: Vec<_> = symbols
                    .iter()
                    .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                    .collect();
                Ok(quote! { (#(#idents),*) = #value_expr; })
            } else {
                // First declaration - mark each variable individually
                symbols.iter().for_each(|s| ctx.declare_var(s));
                let idents_with_mut: Vec<_> = symbols
                    .iter()
                    .map(|s| {
                        let ident = syn::Ident::new(s, proc_macro2::Span::call_site());
                        if ctx.mutable_vars.contains(*s) {
                            quote! { mut #ident }
                        } else {
                            quote! { #ident }
                        }
                    })
                    .collect();
                Ok(quote! { let (#(#idents_with_mut),*) = #value_expr; })
            }
        }
        None => {
            bail!("Complex tuple unpacking not yet supported")
        }
    }
}

/// Generate code for Try/except/finally statement
#[inline]
pub(crate) fn codegen_try_stmt(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Convert try body to statements
    ctx.enter_scope();
    let try_stmts: Vec<_> = body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    // Generate except handler code
    let mut handler_tokens = Vec::new();
    for handler in handlers {
        ctx.enter_scope();

        // If there's a name binding, declare it in scope
        if let Some(var_name) = &handler.name {
            ctx.declare_var(var_name);
        }

        let handler_stmts: Vec<_> = handler
            .body
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();

        handler_tokens.push(quote! { #(#handler_stmts)* });
    }

    // Generate finally clause if present
    let finally_stmts = if let Some(finally_body) = finalbody {
        let stmts: Vec<_> = finally_body
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;
        Some(quote! { #(#stmts)* })
    } else {
        None
    };

    // Generate try/except/finally pattern
    if handlers.is_empty() {
        // Try/finally without except
        if let Some(finally_code) = finally_stmts {
            Ok(quote! {
                {
                    #(#try_stmts)*
                    #finally_code
                }
            })
        } else {
            // Just try block
            Ok(quote! { #(#try_stmts)* })
        }
    } else if handlers.len() == 1 {
        let handler_code = &handler_tokens[0];
        if let Some(finally_code) = finally_stmts {
            Ok(quote! {
                {
                    let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                        #(#try_stmts)*
                        Ok(())
                    })();
                    if let Err(_e) = _result {
                        #handler_code
                    }
                    #finally_code
                }
            })
        } else {
            Ok(quote! {
                {
                    let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                        #(#try_stmts)*
                        Ok(())
                    })();
                    if let Err(_e) = _result {
                        #handler_code
                    }
                }
            })
        }
    } else {
        // Multiple handlers
        if let Some(finally_code) = finally_stmts {
            Ok(quote! {
                {
                    let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                        #(#try_stmts)*
                        Ok(())
                    })();
                    if let Err(_e) = _result {
                        #(#handler_tokens)*
                    }
                    #finally_code
                }
            })
        } else {
            Ok(quote! {
                {
                    let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                        #(#try_stmts)*
                        Ok(())
                    })();
                    if let Err(_e) = _result {
                        #(#handler_tokens)*
                    }
                }
            })
        }
    }
}

impl RustCodeGen for HirStmt {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        match self {
            HirStmt::Assign {
                target,
                value,
                type_annotation,
            } => codegen_assign_stmt(target, value, type_annotation, ctx),
            HirStmt::Return(expr) => codegen_return_stmt(expr, ctx),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => codegen_if_stmt(condition, then_body, else_body, ctx),
            HirStmt::While { condition, body } => codegen_while_stmt(condition, body, ctx),
            HirStmt::For { target, iter, body } => codegen_for_stmt(target, iter, body, ctx),
            HirStmt::Expr(expr) => codegen_expr_stmt(expr, ctx),
            HirStmt::Raise {
                exception,
                cause: _,
            } => codegen_raise_stmt(exception, ctx),
            HirStmt::Break { label } => codegen_break_stmt(label),
            HirStmt::Continue { label } => codegen_continue_stmt(label),
            HirStmt::With {
                context,
                target,
                body,
            } => codegen_with_stmt(context, target, body, ctx),
            HirStmt::Try {
                body,
                handlers,
                orelse: _,
                finalbody,
            } => codegen_try_stmt(body, handlers, finalbody, ctx),
            HirStmt::Assert { test, msg } => codegen_assert_stmt(test, msg, ctx),
            HirStmt::Pass => codegen_pass_stmt(),
        }
    }
}
