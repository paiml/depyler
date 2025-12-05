//! Function code generation
//!
//! This module handles converting HIR functions to Rust token streams.
//! It includes all function conversion helpers and the HirFunction RustCodeGen trait implementation.

use crate::hir::*;
use crate::lifetime_analysis::LifetimeInference;
use crate::rust_gen::context::{CodeGenContext, RustCodeGen};
use crate::rust_gen::generator_gen::codegen_generator_function;
use crate::rust_gen::type_gen::{rust_type_to_syn, update_import_needs};
use anyhow::Result;
use quote::quote;
use syn::{self, parse_quote};

// Import analyze_mutable_vars from parent module
use super::analyze_mutable_vars;

/// Check if a name is a Rust keyword that requires raw identifier syntax
/// DEPYLER-0306: Copied from expr_gen.rs to support method name keyword handling
fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    )
}

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

/// Check if a statement always returns or raises (never falls through)
/// Used to determine if Ok(()) needs to be appended to Result-returning functions
///
/// DEPYLER-0455 Bug 6: Fix unreachable Ok(()) in validator functions
/// Validator functions with try-except that return in all branches were getting
/// unreachable Ok(()) appended, causing type mismatch errors.
/// Example: fn port_validator(value: &str) -> Result<i32, Box<dyn Error>>
///          Both try body and except handler return, so Ok(()) is unreachable
fn stmt_always_returns(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Return(_) => true,
        HirStmt::Raise { .. } => true,
        HirStmt::Try {
            body,
            handlers,
            orelse,
            finalbody: _,
        } => {
            // Try always returns if:
            // 1. Body always returns AND
            // 2. All exception handlers always return AND
            // 3. Orelse (if present) always returns
            // Note: finalbody doesn't affect control flow (always executed)
            let body_returns = body.iter().any(stmt_always_returns);
            let handlers_return = !handlers.is_empty()
                && handlers
                    .iter()
                    .all(|h| h.body.iter().any(stmt_always_returns));
            let orelse_returns = orelse
                .as_ref()
                .map(|stmts| stmts.iter().any(stmt_always_returns))
                .unwrap_or(true);

            // All three conditions must be true
            // If there are no handlers, the try doesn't guarantee a return
            body_returns && handlers_return && orelse_returns
        }
        // DEPYLER-0622: With block always returns if its body always returns
        // Example: `with open(f) as file: return file.read()` always returns
        HirStmt::With { body, .. } => body.iter().any(stmt_always_returns),
        // DEPYLER-0622: If statement always returns if both branches always return
        HirStmt::If {
            then_body,
            else_body,
            ..
        } => {
            let then_returns = then_body.iter().any(stmt_always_returns);
            let else_returns = else_body
                .as_ref()
                .map(|stmts| stmts.iter().any(stmt_always_returns))
                .unwrap_or(false); // No else = might fall through
            then_returns && else_returns
        }
        // DEPYLER-0622: For/While loops don't guarantee return (loop might not execute)
        _ => false,
    }
}

/// Generate combined generic parameters (<'a, 'b, T, U: Bound>)
#[inline]
pub(crate) fn codegen_generic_params(
    type_params: &[crate::generic_inference::TypeParameter],
    lifetime_params: &[String],
) -> proc_macro2::TokenStream {
    if type_params.is_empty() && lifetime_params.is_empty() {
        return quote! {};
    }

    let mut all_params = Vec::new();

    // Add lifetime parameters first
    // Note: Filter out 'static as it's a reserved keyword in Rust and doesn't need to be declared
    for lt in lifetime_params {
        if lt != "'static" {
            let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
            all_params.push(quote! { #lt_ident });
        }
    }

    // Add type parameters with their bounds
    for type_param in type_params {
        let param_name = syn::Ident::new(&type_param.name, proc_macro2::Span::call_site());
        if type_param.bounds.is_empty() {
            all_params.push(quote! { #param_name });
        } else {
            let bounds: Vec<_> = type_param
                .bounds
                .iter()
                .map(|b| {
                    // DEPYLER-0715: Parse as TypeParamBound to support HRTB like for<'a> PartialEq<&'a str>
                    // First try as TypeParamBound (supports HRTB), then fall back to Path
                    syn::parse_str::<syn::TypeParamBound>(b)
                        .map(|bound| quote! { #bound })
                        .or_else(|_| {
                            syn::parse_str::<syn::Path>(b).map(|path| quote! { #path })
                        })
                        .unwrap_or_else(|_| quote! { Clone })
                })
                .collect();
            all_params.push(quote! { #param_name: #(#bounds)+* });
        }
    }

    quote! { <#(#all_params),*> }
}

/// Generate where clause for lifetime bounds (where 'a: 'b, 'c: 'd)
#[inline]
pub(crate) fn codegen_where_clause(
    lifetime_bounds: &[(String, String)],
) -> proc_macro2::TokenStream {
    if lifetime_bounds.is_empty() {
        return quote! {};
    }

    let bounds: Vec<_> = lifetime_bounds
        .iter()
        .map(|(from, to)| {
            let from_lt = syn::Lifetime::new(from, proc_macro2::Span::call_site());
            let to_lt = syn::Lifetime::new(to, proc_macro2::Span::call_site());
            quote! { #from_lt: #to_lt }
        })
        .collect();

    quote! { where #(#bounds),* }
}

/// Generate function attributes (doc comments, panic-free, termination proofs, custom attributes)
#[inline]
pub(crate) fn codegen_function_attrs(
    docstring: &Option<String>,
    properties: &crate::hir::FunctionProperties,
    custom_attributes: &[String],
) -> Vec<proc_macro2::TokenStream> {
    let mut attrs = vec![];

    // Add docstring as documentation if present
    if let Some(docstring) = docstring {
        attrs.push(quote! {
            #[doc = #docstring]
        });
    }

    if properties.panic_free {
        attrs.push(quote! {
            #[doc = " Depyler: verified panic-free"]
        });
    }

    if properties.always_terminates {
        attrs.push(quote! {
            #[doc = " Depyler: proven to terminate"]
        });
    }

    // Add custom Rust attributes
    for attr in custom_attributes {
        // Parse the attribute string as a TokenStream
        // This allows complex attributes like inline(always), repr(C), etc.
        if let Ok(tokens) = attr.parse::<proc_macro2::TokenStream>() {
            attrs.push(quote! {
                #[#tokens]
            });
        }
    }

    attrs
}

// ============================================================================
// DEPYLER-0141 Phase 2: Medium Complexity Helpers
// ============================================================================

/// DEPYLER-0613: Recursively extract all FunctionDef statements from a block
/// Returns the FunctionDef statements that should be hoisted to the top
/// Note: Rust closures can't be forward-declared, so we must emit full definitions first
/// Recursively collect all nested function names from a block of statements
/// This is used to hoist function declarations to the top level
fn collect_nested_function_names(stmts: &[HirStmt], names: &mut Vec<String>) {
    for stmt in stmts {
        match stmt {
            HirStmt::FunctionDef { name, body, .. } => {
                if !names.contains(name) {
                    names.push(name.clone());
                }
                collect_nested_function_names(body, names);
            }
            HirStmt::If { then_body, else_body, .. } => {
                collect_nested_function_names(then_body, names);
                if let Some(else_stmts) = else_body {
                    collect_nested_function_names(else_stmts, names);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } | HirStmt::With { body, .. } => {
                collect_nested_function_names(body, names);
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                collect_nested_function_names(body, names);
                for handler in handlers {
                    collect_nested_function_names(&handler.body, names);
                }
                if let Some(stmts) = orelse {
                    collect_nested_function_names(stmts, names);
                }
                if let Some(stmts) = finalbody {
                    collect_nested_function_names(stmts, names);
                }
            }
            _ => {}
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
    ctx.current_return_type = Some(effective_return_type);

    // DEPYLER-0310: Set error type for raise statement wrapping
    ctx.current_error_type = error_type;

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

    // DEPYLER-0312 NOTE: analyze_mutable_vars is now called in impl RustCodeGen BEFORE
    // codegen_function_params, so ctx.mutable_vars is already populated here

    // DEPYLER-0613: Recursively find and declare nested functions to support hoisting
    // This fixes E0425 where helper functions are called before they're defined,
    // even if they are defined inside blocks (if/else/try).
    let mut all_nested_fns = Vec::new();
    collect_nested_function_names(&func.body, &mut all_nested_fns);
    
    // Start with an empty body
    let mut body_stmts: Vec<proc_macro2::TokenStream> = Vec::new();

    // Emit declarations for all nested functions
    for name in &all_nested_fns {
        if !ctx.is_declared(name) {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            // Declare as mutable variable to allow assignment later
            // Use explicit type if known? No, closures have anonymous types.
            // Rust allows `let mut x; x = ||...` and infers the type.
            body_stmts.push(quote! { let mut #ident; });
            ctx.declare_var(name);
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
        let tokens = stmt.to_rust_tokens(ctx)?;
        body_stmts.push(tokens);
    }

    ctx.exit_scope();
    ctx.current_function_can_fail = false;
    ctx.current_return_type = None;

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
        // Type them as &str by default (most common for argparse fields)
        // For fields that look like lists (plural names), use &[String]
        for field in &accessed_fields {
            let field_ident = quote::format_ident!("{}", field);
            let is_list_field = field.ends_with('s')
                && !["status", "args", "class", "process"].contains(&field.as_str());
            let param_tokens = if is_list_field {
                quote::quote! { #field_ident: &[String] }
            } else {
                quote::quote! { #field_ident: &str }
            };
            params.push(param_tokens);
        }

        return Ok(params);
    }

    func.params
        .iter()
        .map(|param| codegen_single_param(param, func, lifetime_result, ctx))
        .collect()
}

/// Convert a single parameter with all borrowing strategies
fn codegen_single_param(
    param: &HirParam,
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Use parameter name directly to ensure signature matches body references
    // DEPYLER-0357: Removed underscore prefixing logic that was causing compilation errors
    // Parameter names in signature must match exactly how they're referenced in function body
    // DEPYLER-0611: Use raw identifiers for parameter names that are Rust keywords (e.g., override)
    // DEPYLER-0630: self/Self cannot be raw identifiers, rename to self_ instead
    let param_name = param.name.clone();
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
fn function_returns_owned_string(func: &HirFunction) -> bool {
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
fn function_returns_string_concatenation(func: &HirFunction) -> bool {
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

    // DEPYLER-0412: Also check for trailing expression (implicit return)
    // If the last statement is an expression without return, it's an implicit return
    if let Some(HirStmt::Expr(expr)) = body.last() {
        let trailing_type = infer_expr_type_with_env(expr, &var_types);
        if !matches!(trailing_type, Type::Unknown) {
            return_types.push(trailing_type);
        }
    }

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
fn infer_return_type_from_body_with_params(
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
    build_var_type_env(&func.body, &mut var_types);

    // Collect return types with the environment
    let mut return_types = Vec::new();
    collect_return_types_with_env(&func.body, &mut return_types, &var_types);

    // Check for trailing expression (implicit return)
    if let Some(HirStmt::Expr(expr)) = func.body.last() {
        let trailing_type = infer_expr_type_with_env(expr, &var_types);
        if !matches!(trailing_type, Type::Unknown) {
            return_types.push(trailing_type);
        }
    }

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
fn build_var_type_env(stmts: &[HirStmt], var_types: &mut std::collections::HashMap<String, Type>) {
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
                    // DEPYLER-0415: Use the environment we're building for lookups
                    infer_expr_type_with_env(value, var_types)
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
                build_var_type_env(then_body, var_types);
                if let Some(else_stmts) = else_body {
                    build_var_type_env(else_stmts, var_types);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                build_var_type_env(body, var_types);
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                build_var_type_env(body, var_types);
                for handler in handlers {
                    build_var_type_env(&handler.body, var_types);
                }
                if let Some(orelse_stmts) = orelse {
                    build_var_type_env(orelse_stmts, var_types);
                }
                if let Some(finally_stmts) = finalbody {
                    build_var_type_env(finally_stmts, var_types);
                }
            }
            HirStmt::With { body, .. } => {
                build_var_type_env(body, var_types);
            }
            _ => {}
        }
    }
}

/// Collect return types with access to variable type environment
fn collect_return_types_with_env(
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

/// Infer expression type with access to variable type environment
fn infer_expr_type_with_env(
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

            // DEPYLER-0420: Detect array repeat patterns: [elem] * n or n * [elem]
            if matches!(op, BinOp::Mul) {
                match (left.as_ref(), right.as_ref()) {
                    // Pattern: [elem] * n
                    (HirExpr::List(elems), &HirExpr::Literal(Literal::Int(size)))
                        if elems.len() == 1 && size > 0 =>
                    {
                        let elem_type = infer_expr_type_with_env(&elems[0], var_types);
                        return if size <= 32 {
                            Type::Array {
                                element_type: Box::new(elem_type),
                                size: ConstGeneric::Literal(size as usize),
                            }
                        } else {
                            Type::List(Box::new(elem_type))
                        };
                    }
                    // Pattern: n * [elem]
                    (&HirExpr::Literal(Literal::Int(size)), HirExpr::List(elems))
                        if elems.len() == 1 && size > 0 =>
                    {
                        let elem_type = infer_expr_type_with_env(&elems[0], var_types);
                        return if size <= 32 {
                            Type::Array {
                                element_type: Box::new(elem_type),
                                size: ConstGeneric::Literal(size as usize),
                            }
                        } else {
                            Type::List(Box::new(elem_type))
                        };
                    }
                    _ => {}
                }
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
                    // dict.get() returns element type
                    match object_type {
                        Type::Dict(_, val) => *val,
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
                // DEPYLER-0592: Use fully qualified chrono types
                "date" => Type::Custom("chrono::NaiveDate".to_string()),
                "time" => Type::Custom("chrono::NaiveTime".to_string()),
                _ => Type::Unknown,
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

            // DEPYLER-0420: Detect array repeat patterns: [elem] * n or n * [elem]
            if matches!(op, BinOp::Mul) {
                match (left.as_ref(), right.as_ref()) {
                    // Pattern: [elem] * n
                    (HirExpr::List(elems), &HirExpr::Literal(Literal::Int(size)))
                        if elems.len() == 1 && size > 0 =>
                    {
                        let elem_type = infer_expr_type_simple(&elems[0]);
                        return if size <= 32 {
                            Type::Array {
                                element_type: Box::new(elem_type),
                                size: ConstGeneric::Literal(size as usize),
                            }
                        } else {
                            Type::List(Box::new(elem_type))
                        };
                    }
                    // Pattern: n * [elem]
                    (&HirExpr::Literal(Literal::Int(size)), HirExpr::List(elems))
                        if elems.len() == 1 && size > 0 =>
                    {
                        let elem_type = infer_expr_type_simple(&elems[0]);
                        return if size <= 32 {
                            Type::Array {
                                element_type: Box::new(elem_type),
                                size: ConstGeneric::Literal(size as usize),
                            }
                        } else {
                            Type::List(Box::new(elem_type))
                        };
                    }
                    _ => {}
                }
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
                let val_type = infer_expr_type_simple(&pairs[0].1);
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
                _ => Type::Unknown,
            }
        }
        // DEPYLER-0414: Add MethodCall expression type inference
        HirExpr::MethodCall { object, method, .. } => {
            match method.as_str() {
                // DEPYLER-REARCH-001: .copy() returns same type as object
                "copy" => infer_expr_type_simple(object),
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
        HirExpr::Call { func, args, .. } => {
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

            // Recursively check arguments
            for arg in args {
                if let Some(ty) = infer_type_from_expr_usage(param_name, arg) {
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
            ..
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

/// GH-70: Detect if function returns a nested function/closure
/// Returns Some((nested_fn_name, params, ret_type)) if detected
/// Stores inferred params in ctx.nested_function_params for use during code generation
fn detect_returns_nested_function(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
) -> Option<(String, Vec<HirParam>, Type)> {
    // Look for pattern: function contains nested FunctionDef and ends with returning that name
    let mut nested_functions: std::collections::HashMap<String, (Vec<HirParam>, Type)> =
        std::collections::HashMap::new();

    // Collect nested function definitions with type inference
    for stmt in &func.body {
        if let HirStmt::FunctionDef {
            name,
            params,
            ret_type,
            body,
            ..
        } = stmt
        {
            // GH-70: Apply type inference to parameters
            let mut inferred_params = params.to_vec();
            for param in &mut inferred_params {
                if matches!(param.ty, Type::Unknown) {
                    // Try to infer from body usage
                    if let Some(inferred_ty) = infer_param_type_from_body(&param.name, body) {
                        param.ty = inferred_ty;
                    }
                }
            }

            // GH-70: Apply type inference to return type
            // Include inferred param types in the environment so that
            // expressions like `return item[0]` can infer the element type
            let inferred_ret_type = if matches!(ret_type, Type::Unknown) {
                // Build type env with inferred params
                let mut var_types: std::collections::HashMap<String, Type> =
                    std::collections::HashMap::new();
                for p in &inferred_params {
                    var_types.insert(p.name.clone(), p.ty.clone());
                }
                // Build from body assignments
                build_var_type_env(body, &mut var_types);

                // Collect return types using the enhanced environment
                let mut return_types = Vec::new();
                collect_return_types_with_env(body, &mut return_types, &var_types);

                // Check for trailing expression
                if let Some(HirStmt::Expr(expr)) = body.last() {
                    let trailing_type = infer_expr_type_with_env(expr, &var_types);
                    if !matches!(trailing_type, Type::Unknown) {
                        return_types.push(trailing_type);
                    }
                }

                // Get first known type
                return_types
                    .iter()
                    .find(|t| !matches!(t, Type::Unknown))
                    .cloned()
                    .unwrap_or_else(|| ret_type.clone())
            } else {
                ret_type.clone()
            };

            // Store inferred params in context for use during code generation
            ctx.nested_function_params
                .insert(name.clone(), inferred_params.clone());

            nested_functions.insert(name.clone(), (inferred_params, inferred_ret_type));
        }
    }

    // Check if last statement returns one of the nested functions
    if let Some(last_stmt) = func.body.last() {
        // Pattern 1: explicit return statement
        if let HirStmt::Return(Some(HirExpr::Var(var_name))) = last_stmt {
            if let Some((params, ret_type)) = nested_functions.get(var_name) {
                return Some((var_name.clone(), params.clone(), ret_type.clone()));
            }
        }
        // Pattern 2: implicit return (expression statement at end)
        if let HirStmt::Expr(HirExpr::Var(var_name)) = last_stmt {
            if let Some((params, ret_type)) = nested_functions.get(var_name) {
                return Some((var_name.clone(), params.clone(), ret_type.clone()));
            }
        }
    }

    None
}

/// DEPYLER-0626: Check if function returns heterogeneous IO types (File vs Stdout)
/// Returns true if function has return statements that return both file and stdio types
fn function_returns_heterogeneous_io(func: &HirFunction) -> bool {
    let mut has_file_return = false;
    let mut has_stdio_return = false;

    collect_io_return_types(&func.body, &mut has_file_return, &mut has_stdio_return);

    has_file_return && has_stdio_return
}

/// DEPYLER-0626: Helper to collect IO return types from statements
fn collect_io_return_types(stmts: &[HirStmt], has_file: &mut bool, has_stdio: &mut bool) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                if is_file_creating_return_expr(expr) {
                    *has_file = true;
                }
                if is_stdio_return_expr(expr) {
                    *has_stdio = true;
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                collect_io_return_types(then_body, has_file, has_stdio);
                if let Some(else_stmts) = else_body {
                    collect_io_return_types(else_stmts, has_file, has_stdio);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                collect_io_return_types(body, has_file, has_stdio);
            }
            _ => {}
        }
    }
}

/// DEPYLER-0626: Check if expression creates a File (open() or File::create())
fn is_file_creating_return_expr(expr: &HirExpr) -> bool {
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

/// DEPYLER-0626: Check if expression is sys.stdout or sys.stderr
fn is_stdio_return_expr(expr: &HirExpr) -> bool {
    if let HirExpr::Attribute { value, attr } = expr {
        if attr == "stdout" || attr == "stderr" {
            if let HirExpr::Var(name) = value.as_ref() {
                return name == "sys";
            }
        }
    }
    false
}

/// Generate return type with Result wrapper and lifetime handling
///
/// DEPYLER-0310: Now returns ErrorType (4th tuple element) for raise statement wrapping
/// GH-70: Now detects when function returns nested function and uses Box<dyn Fn>
#[inline]
pub(crate) fn codegen_return_type(
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) -> Result<(
    proc_macro2::TokenStream,
    crate::type_mapper::RustType,
    bool,
    Option<crate::rust_gen::context::ErrorType>,
)> {
    // GH-70: Check if function returns a nested function/closure
    if let Some((_nested_name, params, nested_ret_type)) = detect_returns_nested_function(func, ctx)
    {
        use quote::quote;

        // Build Box<dyn Fn(params) -> ret> type
        let param_types: Vec<proc_macro2::TokenStream> = params
            .iter()
            .map(|p| crate::rust_gen::stmt_gen::hir_type_to_tokens(&p.ty, ctx))
            .collect();

        let ret_ty_tokens = crate::rust_gen::stmt_gen::hir_type_to_tokens(&nested_ret_type, ctx);

        let fn_type = if params.is_empty() {
            quote! { -> Box<dyn Fn() -> #ret_ty_tokens> }
        } else {
            quote! { -> Box<dyn Fn(#(#param_types),*) -> #ret_ty_tokens> }
        };

        return Ok((
            fn_type.clone(),
            crate::type_mapper::RustType::Custom("BoxedFn".to_string()),
            false, // can_fail
            None,  // error_type
        ));
    }

    // DEPYLER-0626: Check if function returns heterogeneous IO types (File vs Stdout)
    // If so, return type should be Box<dyn std::io::Write>
    if function_returns_heterogeneous_io(func) {
        use quote::quote;
        ctx.function_returns_boxed_write = true;
        ctx.needs_io_write = true;

        // Check if function can fail (uses open() which can fail)
        let can_fail = func.properties.can_fail;
        let error_type = if can_fail {
            Some(crate::rust_gen::context::ErrorType::Concrete("std::io::Error".to_string()))
        } else {
            None
        };

        let return_type = if can_fail {
            quote! { -> Result<Box<dyn std::io::Write>, std::io::Error> }
        } else {
            quote! { -> Box<dyn std::io::Write> }
        };

        return Ok((
            return_type,
            crate::type_mapper::RustType::Custom("BoxedWrite".to_string()),
            can_fail,
            error_type,
        ));
    }

    // DEPYLER-0410: Infer return type from body when annotation is Unknown
    // DEPYLER-0420: Also infer when tuple/list contains Unknown elements
    // DEPYLER-0460: Use _with_params version for Optional pattern detection
    // DEPYLER-0460: Also infer when ret_type is None, because that could be:
    // 1. A function returning None in all paths → () in Rust
    // 2. A function returning None|T (Optional pattern) → Option<T> in Rust
    // DEPYLER-0662: Also infer when ret_type is empty tuple (from `-> tuple` annotation)
    // Python `-> tuple` without type params should be inferred from return statements
    // DEPYLER-0662: Python `-> tuple` parses to Type::Custom("tuple"), not Type::Tuple
    let should_infer = matches!(func.ret_type, Type::Unknown | Type::None)
        || matches!(&func.ret_type, Type::Tuple(elems) if elems.is_empty() || elems.iter().any(|t| matches!(t, Type::Unknown)))
        || matches!(&func.ret_type, Type::List(elem) if matches!(**elem, Type::Unknown))
        || matches!(&func.ret_type, Type::Custom(name) if name == "tuple");

    let effective_ret_type = if should_infer {
        // Try to infer from return statements in body (with parameter type tracking for Optional detection)
        infer_return_type_from_body_with_params(func, ctx).unwrap_or_else(|| func.ret_type.clone())
    } else {
        func.ret_type.clone()
    };

    // DEPYLER-0716: Apply type substitutions to return type
    // When generic parameters are substituted (e.g., T -> String), apply to return type too
    let effective_ret_type = if !ctx.type_substitutions.is_empty() {
        crate::generic_inference::TypeVarRegistry::apply_substitutions(
            &effective_ret_type,
            &ctx.type_substitutions,
        )
    } else {
        effective_ret_type
    };

    // Convert return type using annotation-aware mapping
    let mapped_ret_type = ctx
        .annotation_aware_mapper
        .map_return_type_with_annotations(&effective_ret_type, &func.annotations);

    // Check if this is a placeholder Union enum that needs proper generation
    let rust_ret_type = if let crate::type_mapper::RustType::Enum { name, .. } = &mapped_ret_type {
        if name == "UnionType" {
            // Generate a proper enum name and definition from the original Union type
            if let Type::Union(types) = &func.ret_type {
                let enum_name = ctx.process_union_type(types);
                crate::type_mapper::RustType::Custom(enum_name)
            } else {
                mapped_ret_type
            }
        } else {
            mapped_ret_type
        }
    } else {
        mapped_ret_type
    };

    // v3.16.0 Phase 1: Override return type to String if function returns owned via string methods
    // This prevents lifetime analysis from incorrectly converting to borrowed &str
    let rust_ret_type =
        if matches!(func.ret_type, Type::String) && function_returns_owned_string(func) {
            // Force owned String return, don't use lifetime borrowing
            crate::type_mapper::RustType::String
        } else {
            rust_ret_type
        };

    // Update import needs based on return type
    update_import_needs(ctx, &rust_ret_type);

    // Check if function can fail and needs Result wrapper
    let can_fail = func.properties.can_fail;
    let mut error_type_str = if can_fail && !func.properties.error_types.is_empty() {
        // Use first error type or generic for mixed types
        if func.properties.error_types.len() == 1 {
            func.properties.error_types[0].clone()
        } else {
            "Box<dyn std::error::Error>".to_string()
        }
    } else {
        "Box<dyn std::error::Error>".to_string()
    };

    // DEPYLER-0597: Map Python exception types to Rust error types
    // This ensures function signatures like `-> Result<T, OSError>` compile
    // Using Box<dyn std::error::Error> for most exceptions since it doesn't require external crates
    error_type_str = match error_type_str.as_str() {
        // File/IO related exceptions map to std::io::Error for idiomatic Rust
        "OSError" | "IOError" | "FileNotFoundError" | "PermissionError" => {
            "std::io::Error".to_string()
        }
        // General exceptions map to Box<dyn std::error::Error> (no external crate needed)
        "Exception" | "BaseException" | "ValueError" | "TypeError" | "KeyError"
        | "IndexError" | "RuntimeError" | "AttributeError" | "NotImplementedError"
        | "AssertionError" | "StopIteration" | "ZeroDivisionError" | "OverflowError"
        | "ArithmeticError" => "Box<dyn std::error::Error>".to_string(),
        // Keep other types as-is (might be custom error types)
        _ => error_type_str,
    };

    // DEPYLER-0447: Validators always use Box<dyn Error> for compatibility with clap
    if ctx.validator_functions.contains(&func.name) {
        error_type_str = "Box<dyn std::error::Error>".to_string();
    }

    // DEPYLER-0310: Determine ErrorType for raise statement wrapping
    // If Box<dyn Error>, we need to wrap exceptions with Box::new()
    // If concrete type, no wrapping needed
    let error_type = if can_fail {
        Some(if error_type_str.contains("Box<dyn") {
            crate::rust_gen::context::ErrorType::DynBox
        } else {
            crate::rust_gen::context::ErrorType::Concrete(error_type_str.clone())
        })
    } else {
        None
    };

    // DEPYLER-0327 Fix #5: Mark error types as needed for type generation
    // Check BOTH error_type_str (for functions that return Result) AND
    // func.properties.error_types (for types used in try/except blocks)
    // DEPYLER-0551: Added RuntimeError and FileNotFoundError
    if error_type_str.contains("ZeroDivisionError") {
        ctx.needs_zerodivisionerror = true;
    }
    if error_type_str.contains("IndexError") {
        ctx.needs_indexerror = true;
    }
    if error_type_str.contains("ValueError") {
        ctx.needs_valueerror = true;
    }
    if error_type_str.contains("RuntimeError") {
        ctx.needs_runtimeerror = true;
    }
    if error_type_str.contains("FileNotFoundError") {
        ctx.needs_filenotfounderror = true;
    }

    // Also check all error_types from properties (even if can_fail=false)
    // This ensures types used in try/except blocks are generated
    for err_type in &func.properties.error_types {
        if err_type.contains("ZeroDivisionError") {
            ctx.needs_zerodivisionerror = true;
        }
        if err_type.contains("IndexError") {
            ctx.needs_indexerror = true;
        }
        if err_type.contains("ValueError") {
            ctx.needs_valueerror = true;
        }
        if err_type.contains("RuntimeError") {
            ctx.needs_runtimeerror = true;
        }
        if err_type.contains("FileNotFoundError") {
            ctx.needs_filenotfounderror = true;
        }
    }

    let return_type = if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
        if can_fail {
            let error_type: syn::Type = syn::parse_str(&error_type_str)
                .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });

            // DEPYLER-0455 Bug 7: Infer return type from function body
            // Functions without type annotations but that return values (e.g., argparse validators)
            // should infer their return type from actual return statements
            //
            // Example: def email_address(value):
            //              return value  # <- Returns string, not None
            //
            // Before fix: Result<(), Box<dyn Error>>  [WRONG - type mismatch with returned value]
            // After fix:  Result<String, Box<dyn Error>>  [CORRECT - matches return value]
            if let Some(inferred_type) = infer_return_type_from_body_with_params(func, ctx) {
                // We found a return statement with a value!
                // Map the inferred HIR type to Rust type
                let inferred_rust_type = ctx
                    .annotation_aware_mapper
                    .map_return_type_with_annotations(&inferred_type, &func.annotations);

                // Convert to syn type
                if let Ok(ty) = rust_type_to_syn(&inferred_rust_type) {
                    // DEPYLER-0612: main() can only return () or Result<(), E>
                    if func.name == "main" {
                        quote! { -> Result<(), #error_type> }
                    } else {
                        // Use inferred type instead of ()
                        quote! { -> Result<#ty, #error_type> }
                    }
                } else {
                    // Fallback to () if conversion fails
                    quote! { -> Result<(), #error_type> }
                }
            } else {
                // No return value found, use ()
                quote! { -> Result<(), #error_type> }
            }
        } else {
            quote! {}
        }
    } else {
        let mut ty = rust_type_to_syn(&rust_ret_type)?;

        // DEPYLER-0270: Check if function returns string concatenation
        // String concatenation (format!(), a + b) always returns owned String
        // Never use Cow for concatenation results
        let returns_concatenation = matches!(func.ret_type, crate::hir::Type::String)
            && function_returns_string_concatenation(func);

        // Check if any parameter escapes through return and uses Cow
        let mut uses_cow_return = false;
        if !returns_concatenation {
            // Only consider Cow if NOT doing string concatenation
            for param in &func.params {
                if let Some(strategy) = lifetime_result.borrowing_strategies.get(&param.name) {
                    if matches!(
                        strategy,
                        crate::borrowing_context::BorrowingStrategy::UseCow { .. }
                    ) {
                        if let Some(_usage) = lifetime_result.param_lifetimes.get(&param.name) {
                            // If a Cow parameter escapes, return type should also be Cow
                            if matches!(func.ret_type, crate::hir::Type::String) {
                                uses_cow_return = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if uses_cow_return && !returns_concatenation {
            // Use the same Cow type for return
            ctx.needs_cow = true;
            if let Some(ref return_lt) = lifetime_result.return_lifetime {
                let lt = syn::Lifetime::new(return_lt.as_str(), proc_macro2::Span::call_site());
                ty = parse_quote! { Cow<#lt, str> };
            } else {
                ty = parse_quote! { Cow<'static, str> };
            }
        } else {
            // v3.16.0 Phase 1: Check if function returns owned String via transformation methods
            // If so, don't convert to borrowed &str even if lifetime analysis suggests it
            let returns_owned_string =
                matches!(func.ret_type, Type::String) && function_returns_owned_string(func);

            // Apply return lifetime if needed (unless returning owned String)
            if let Some(ref return_lt) = lifetime_result.return_lifetime {
                // Check if the return type needs lifetime substitution
                if matches!(
                    rust_ret_type,
                    crate::type_mapper::RustType::Str { .. }
                        | crate::type_mapper::RustType::Reference { .. }
                ) && !returns_owned_string
                {
                    // Only apply lifetime if NOT returning owned String
                    let lt = syn::Lifetime::new(return_lt.as_str(), proc_macro2::Span::call_site());
                    match &rust_ret_type {
                        crate::type_mapper::RustType::Str { .. } => {
                            ty = parse_quote! { &#lt str };
                        }
                        crate::type_mapper::RustType::Reference { mutable, inner, .. } => {
                            let inner_ty = rust_type_to_syn(inner)?;
                            ty = if *mutable {
                                parse_quote! { &#lt mut #inner_ty }
                            } else {
                                parse_quote! { &#lt #inner_ty }
                            };
                        }
                        _ => {}
                    }
                }
            }
            // If returns_owned_string is true, keep ty as String (already set from rust_type_to_syn)
        }

        if can_fail {
            let error_type: syn::Type = syn::parse_str(&error_type_str)
                .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });

            // DEPYLER-0612: main() can only return () or Result<(), E>
            // Convert Result<i32, E> to Result<(), E> for main
            if func.name == "main" {
                quote! { -> Result<(), #error_type> }
            } else {
                quote! { -> Result<#ty, #error_type> }
            }
        } else if func.name == "main" && matches!(func.ret_type, Type::Int) {
            // DEPYLER-0617: main() can only return () or Result<(), E>
            // Convert i32 return to () for non-fallible main
            quote! {}  // No return type annotation (defaults to ())
        } else {
            quote! { -> #ty }
        }
    };

    Ok((return_type, rust_ret_type, can_fail, error_type))
}

// ========== Phase 3c: Generator Implementation ==========
// (Moved to generator_gen.rs in v3.18.0 Phase 4)

impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        // DEPYLER-0717: Clear var_types at the start of each function to prevent type leaking
        // Without this, parameter types from one function can leak to the next function
        // when they share the same parameter name (e.g., both have `items` parameter)
        ctx.var_types.clear();
        ctx.type_substitutions.clear();

        // DEPYLER-0306 FIX: Use raw identifiers for function names that are Rust keywords
        let name = if is_rust_keyword(&self.name) {
            syn::Ident::new_raw(&self.name, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(&self.name, proc_macro2::Span::call_site())
        };

        // DEPYLER-0269: Track function return type for Display trait selection
        // Store function return type in ctx for later lookup when processing assignments
        // This enables tracking `result = merge(&a, &b)` where merge returns list[int]
        ctx.function_return_types
            .insert(self.name.clone(), self.ret_type.clone());

        // DEPYLER-0621: Track parameter defaults for call-site argument completion
        // When a function like `def f(x=None)` is called as `f()`, we need to supply `None`
        let param_defaults: Vec<Option<crate::hir::HirExpr>> = self
            .params
            .iter()
            .map(|p| p.default.clone())
            .collect();
        ctx.function_param_defaults
            .insert(self.name.clone(), param_defaults);

        // Perform generic type inference
        let mut generic_registry = crate::generic_inference::TypeVarRegistry::new();

        // DEPYLER-0716: Infer type substitutions (e.g., T -> String when comparing to strings)
        let type_substitutions = generic_registry.infer_type_substitutions(self)?;

        // DEPYLER-0716: Apply substitutions to parameter types in var_types
        // This ensures List(Unknown) becomes List(String) when elements are compared to strings
        if !type_substitutions.is_empty() {
            for param in &self.params {
                let substituted_ty = crate::generic_inference::TypeVarRegistry::apply_substitutions(
                    &param.ty,
                    &type_substitutions,
                );
                if substituted_ty != param.ty {
                    ctx.var_types.insert(param.name.clone(), substituted_ty);
                }
            }
            // DEPYLER-0716: Store substitutions in context for return type processing
            ctx.type_substitutions = type_substitutions;
        }

        let type_params = generic_registry.infer_function_generics(self)?;

        // Perform lifetime analysis with automatic elision (DEPYLER-0275)
        let mut lifetime_inference = LifetimeInference::new();
        let lifetime_result = lifetime_inference
            .apply_elision_rules(self, ctx.type_mapper)
            .unwrap_or_else(|| lifetime_inference.analyze_function(self, ctx.type_mapper));

        // Generate combined generic parameters (lifetimes + type params)
        let generic_params = codegen_generic_params(&type_params, &lifetime_result.lifetime_params);

        // Generate lifetime bounds
        let where_clause = codegen_where_clause(&lifetime_result.lifetime_bounds);

        // DEPYLER-0312: Analyze mutability BEFORE generating parameters
        // This populates ctx.mutable_vars which codegen_single_param uses to determine `mut` keyword
        analyze_mutable_vars(&self.body, ctx, &self.params);

        // DEPYLER-0524: Infer parameter types from usage in function body
        // This updates var_types so parameters with Unknown type can be inferred from usage
        // Must run BEFORE codegen_function_params to affect parameter type generation
        for param in &self.params {
            if matches!(param.ty, Type::Unknown) {
                if let Some(inferred_ty) = infer_param_type_from_body(&param.name, &self.body) {
                    ctx.var_types.insert(param.name.clone(), inferred_ty);
                }
            }
        }

        // Convert parameters using lifetime analysis results
        let params = codegen_function_params(self, &lifetime_result, ctx)?;

        // DEPYLER-0270: Extract parameter borrowing information for auto-borrow decisions
        // Check which parameters are references (borrowed) vs owned
        let param_borrows: Vec<bool> = self
            .params
            .iter()
            .map(|p| {
                lifetime_result
                    .param_lifetimes
                    .get(&p.name)
                    .map(|inf| inf.should_borrow)
                    .unwrap_or(false)
            })
            .collect();
        ctx.function_param_borrows
            .insert(self.name.clone(), param_borrows);

        // DEPYLER-0574: Extract parameter mutability information for &mut decisions
        // Check which borrowed parameters need &mut (mutable borrow)
        let param_muts: Vec<bool> = self
            .params
            .iter()
            .map(|p| {
                let is_mutated = ctx.mutable_vars.contains(&p.name);
                let should_borrow = lifetime_result
                    .param_lifetimes
                    .get(&p.name)
                    .map(|inf| inf.should_borrow)
                    .unwrap_or(false);
                // needs_mut = mutated in body AND borrowed (not owned)
                is_mutated && should_borrow
            })
            .collect();
        ctx.function_param_muts
            .insert(self.name.clone(), param_muts);

        // DEPYLER-0648: Track if function has vararg parameter (*args in Python)
        // These become &[T] in Rust, so call sites need to wrap args in &[...]
        if self.params.iter().any(|p| p.is_vararg) {
            ctx.vararg_functions.insert(self.name.clone());
        }

        // Generate return type with Result wrapper and lifetime handling
        let (return_type, rust_ret_type, can_fail, error_type) =
            codegen_return_type(self, &lifetime_result, ctx)?;

        // DEPYLER-0425: Analyze subcommand field access BEFORE generating body
        // This sets ctx.current_subcommand_fields so expression generation can rewrite args.field → field
        let subcommand_info = if ctx.argparser_tracker.has_subcommands() {
            crate::rust_gen::argparse_transform::analyze_subcommand_field_access(
                self,
                &ctx.argparser_tracker,
            )
        } else {
            None
        };

        // Set context for expression generation
        if let Some((_, ref fields)) = subcommand_info {
            ctx.current_subcommand_fields = Some(fields.iter().cloned().collect());
        }

        // DEPYLER-0456 Bug #1: Pre-register all add_parser() calls before body codegen
        // This ensures expression statement subcommands (no variable assignment) are included
        // in Commands enum generation. Must run BEFORE codegen_function_body() below.
        crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(
            self,
            &mut ctx.argparser_tracker,
        );

        // DEPYLER-0108: Pre-populate Option fields for substitution BEFORE body codegen
        // This must happen before codegen_function_body() so that convert_method_call
        // can substitute args.<field>.is_some() with has_<field>
        if ctx.argparser_tracker.has_parsers() {
            if let Some(parser_info) = ctx.argparser_tracker.get_first_parser() {
                for arg in &parser_info.arguments {
                    if arg.rust_type().starts_with("Option<") {
                        ctx.precomputed_option_fields
                            .insert(arg.rust_field_name().to_string());
                    }
                }
            }
        }

        // DEPYLER-0617: Set flag if we're generating main() function
        // This affects return statement handling (integer returns → process::exit)
        let was_main = ctx.is_main_function;
        ctx.is_main_function = self.name == "main";

        // Process function body with proper scoping (expressions will now be rewritten if needed)
        let mut body_stmts = codegen_function_body(self, can_fail, error_type, ctx)?;

        // DEPYLER-0694: If function returns unit type (no return annotation in Python),
        // ensure trailing expressions don't accidentally return a value.
        // Add semicolon to discard the expression's value when not returning.
        // DEPYLER-0702: Use `let _ = expr;` instead of `expr;` to avoid unused-must-use warnings
        if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
            if let Some(last) = body_stmts.last_mut() {
                let last_str = last.to_string();
                // If statement doesn't end with semicolon or closing brace, it's an expression
                // that would return a value - we need to discard it for Unit return types
                // DEPYLER-0711: Skip empty tokens (e.g., from `pass` statement)
                if !last_str.is_empty()
                    && !last_str.trim_end().ends_with(';')
                    && !last_str.trim_end().ends_with('}')
                {
                    use quote::quote;
                    let tokens = std::mem::take(last);
                    // Use `let _ = expr;` to discard the value without triggering
                    // "unused arithmetic operation" or similar warnings
                    *last = quote! { let _ = #tokens; };
                }
            }
        }

        // DEPYLER-0617: Restore flag after body generation
        ctx.is_main_function = was_main;

        // GH-70: Wrap returned closure in Box::new() if function returns Box<dyn Fn>
        if let Some((nested_name, _, _)) = detect_returns_nested_function(self, ctx) {
            // Find last statement and wrap if it's returning the nested function
            if let Some(last_stmt) = body_stmts.last_mut() {
                use quote::quote;
                let nested_ident = syn::Ident::new(&nested_name, proc_macro2::Span::call_site());
                // Check if last statement is just the variable name (implicit return)
                let last_stmt_str = last_stmt.to_string();
                if last_stmt_str.trim() == nested_name {
                    // Replace with Box::new(name)
                    *last_stmt = quote! { Box::new(#nested_ident) };
                }
            }
        }

        // Clear the subcommand fields context after body generation
        ctx.current_subcommand_fields = None;

        // DEPYLER-0363: Check if ArgumentParser was detected and generate Args struct
        // DEPYLER-0424: Store Args struct and Commands enum in context for module-level emission
        // (hoisted outside function to make Args accessible to handler functions)
        if ctx.argparser_tracker.has_parsers() {
            if let Some(parser_info) = ctx.argparser_tracker.get_first_parser() {
                // DEPYLER-0384: Set flag to include clap dependency in Cargo.toml
                ctx.needs_clap = true;

                // DEPYLER-0399: Generate Commands enum if subcommands exist
                let commands_enum = crate::rust_gen::argparse_transform::generate_commands_enum(
                    &ctx.argparser_tracker,
                );
                if !commands_enum.is_empty() {
                    ctx.generated_commands_enum = Some(commands_enum);
                }

                // Generate the Args struct definition
                let args_struct = crate::rust_gen::argparse_transform::generate_args_struct(
                    parser_info,
                    &ctx.argparser_tracker,
                );
                ctx.generated_args_struct = Some(args_struct);

                // DEPYLER-0108: Inject precompute statements for Option fields
                // This prevents borrow-after-move when Option is passed then checked with is_some()
                let precompute_stmts =
                    crate::rust_gen::argparse_transform::generate_option_precompute(parser_info);
                if !precompute_stmts.is_empty() {
                    // DEPYLER-0108: FIRST post-process body to replace args.<field>.is_some() with has_<field>
                    // This must happen BEFORE injecting precompute statements to avoid replacing them too
                    let option_fields: Vec<String> = parser_info
                        .arguments
                        .iter()
                        .filter(|arg| arg.rust_type().starts_with("Option<"))
                        .map(|arg| arg.rust_field_name().to_string())
                        .collect();

                    if !option_fields.is_empty() {
                        body_stmts = body_stmts
                            .into_iter()
                            .map(|stmt| {
                                let mut stmt_str = stmt.to_string();
                                for field in &option_fields {
                                    // Replace "args . <field> . is_some ()" with "has_<field>"
                                    let pattern = format!("args . {} . is_some ()", field);
                                    let replacement = format!("has_{}", field);
                                    stmt_str = stmt_str.replace(&pattern, &replacement);
                                    // Also handle is_none
                                    let pattern_none = format!("args . {} . is_none ()", field);
                                    let replacement_none = format!("! has_{}", field);
                                    stmt_str = stmt_str.replace(&pattern_none, &replacement_none);
                                }
                                syn::parse_str(&stmt_str).unwrap_or(stmt)
                            })
                            .collect();
                    }

                    // THEN inject precompute statements after replacement
                    // Find the Args::parse() statement index and insert after it
                    // The parse() call is typically the first statement in main()
                    let insert_idx = body_stmts
                        .iter()
                        .position(|s| s.to_string().contains("Args :: parse"))
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    for (offset, stmt) in precompute_stmts.into_iter().enumerate() {
                        body_stmts.insert(insert_idx + offset, stmt);
                    }
                }

                // Note: ArgumentParser-related statements are filtered in stmt_gen.rs
                // parse_args() calls are transformed in stmt_gen.rs::codegen_assign_stmt
            }

            // DO NOT clear tracker yet - we need it for parameter type resolution
            // It will be cleared after all functions are generated
        }

        // DEPYLER-0425: Wrap handler functions with subcommand pattern matching
        // If this function accesses subcommand-specific fields, wrap body in pattern matching
        if let Some((variant_name, fields)) = subcommand_info {
            // Get args parameter name (first parameter)
            if let Some(args_param) = self.params.first() {
                let args_param_name = args_param.name.as_ref();
                // Wrap body statements in pattern matching to extract fields from enum variant
                body_stmts = crate::rust_gen::argparse_transform::wrap_body_with_subcommand_pattern(
                    body_stmts,
                    &variant_name,
                    &fields,
                    args_param_name,
                );
            }
        }

        // DEPYLER-0270: Add Ok(()) for functions with Result<(), E> return type
        // When Python function has `-> None` but uses fallible operations (e.g., indexing),
        // the Rust return type becomes `Result<(), IndexError>` and needs Ok(()) at the end
        // Only add Ok(()) if the function doesn't already end with a return statement
        //
        // DEPYLER-0450: Extended to handle all Result return types, not just Type::None
        // This fixes functions with side effects that use error handling (raise/try/except)
        // Also handles Type::Unknown (functions without type annotations that don't explicitly return)
        //
        // DEPYLER-0455 Bug 6: Check if last statement always returns (including try-except)
        // Validator functions with try-except that return in all branches should not get Ok(())
        // Use stmt_always_returns() instead of simple Return check to handle exhaustive returns
        if can_fail {
            let needs_ok = self
                .body
                .last()
                .is_none_or(|stmt| !stmt_always_returns(stmt));
            if needs_ok {
                // For functions returning unit type (or Unknown which defaults to unit), add Ok(())
                // For functions returning values with explicit returns, they already have Ok() wrapping
                if matches!(self.ret_type, Type::None | Type::Unknown) {
                    body_stmts.push(parse_quote! { Ok(()) });
                }
            }
        }

        // Add documentation and custom attributes
        let attrs = codegen_function_attrs(
            &self.docstring,
            &self.properties,
            &self.annotations.custom_attributes,
        );

        // Check if function is a generator (contains yield)
        let func_tokens = if self.properties.is_generator {
            codegen_generator_function(
                self,
                &name,
                &generic_params,
                &where_clause,
                &params,
                &attrs,
                &rust_ret_type,
                ctx,
            )?
        } else if self.properties.is_async {
            quote! {
                #(#attrs)*
                pub async fn #name #generic_params(#(#params),*) #return_type #where_clause {
                    #(#body_stmts)*
                }
            }
        } else {
            quote! {
                #(#attrs)*
                pub fn #name #generic_params(#(#params),*) #return_type #where_clause {
                    #(#body_stmts)*
                }
            }
        };

        Ok(func_tokens)
    }
}
