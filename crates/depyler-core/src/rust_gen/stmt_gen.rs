//! Statement code generation
//!
//! This module handles converting HIR statements to Rust token streams.
//! It includes all statement conversion helpers and the HirStmt RustCodeGen trait implementation.

use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, RustCodeGen, ToRustExpr};
use crate::rust_gen::keywords::safe_ident; // DEPYLER-0023: Keyword escaping
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

/// Check if an HIR expression returns usize (needs cast to i32)
///
/// DEPYLER-0272: Only add casts for expressions that actually return usize.
/// This prevents unnecessary casts like `(a: i32) as i32`.
/// Complexity: 4 (recursive pattern matching)
fn expr_returns_usize(expr: &HirExpr) -> bool {
    match expr {
        // Method calls that return usize
        HirExpr::MethodCall { method, .. } => {
            matches!(method.as_str(), "len" | "count" | "capacity")
        }
        // Builtin functions that return usize
        HirExpr::Call { func, .. } => {
            matches!(func.as_str(), "len" | "range")
        }
        // Binary operations might contain usize expressions
        HirExpr::Binary { left, right, .. } => {
            expr_returns_usize(left) || expr_returns_usize(right)
        }
        // All other expressions (Var, Literal, etc.) don't return usize in our HIR
        _ => false,
    }
}

/// Check if a type annotation requires explicit conversion
///
/// DEPYLER-0272 FIX: Now checks the actual expression to determine if cast is needed.
/// Only adds cast when expression returns usize (from len(), count(), etc.)
/// Complexity: 3 (type check + expression check)
fn needs_type_conversion(target_type: &Type, expr: &HirExpr) -> bool {
    match target_type {
        Type::Int => {
            // Only convert if expression actually returns usize
            // This prevents unnecessary casts like `(x: i32) as i32`
            expr_returns_usize(expr)
        }
        _ => false,
    }
}

/// Apply type conversion to value expression
///
/// Wraps the expression with appropriate conversion (e.g., `as i32`)
/// Complexity: 2 (simple match)
fn apply_type_conversion(value_expr: syn::Expr, target_type: &Type) -> syn::Expr {
    match target_type {
        Type::Int => {
            // Convert to i32 using 'as' cast
            // This handles usize->i32 conversions
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
    // DEPYLER-0363: Detect parser.add_argument(...) method calls
    // Pattern: parser.add_argument("files", nargs="+", type=Path, action="store_true", help="...")
    if let HirExpr::MethodCall { object, method, args, kwargs } = expr {
        if method == "add_argument" {
            if let HirExpr::Var(parser_var) = object.as_ref() {
                if let Some(_parser_info) = ctx.argparser_tracker.get_parser_mut(parser_var) {
                    // Extract argument name from first positional argument
                    if let Some(HirExpr::Literal(crate::hir::Literal::String(arg_name))) = args.first() {
                        let mut arg = crate::rust_gen::argparse_transform::ArgParserArgument::new(arg_name.clone());

                        // DEPYLER-0364: Extract keyword arguments from HIR
                        for (kw_name, kw_value) in kwargs {
                            match kw_name.as_str() {
                                "nargs" => {
                                    if let HirExpr::Literal(crate::hir::Literal::String(nargs_val)) = kw_value {
                                        arg.nargs = Some(nargs_val.clone());
                                    }
                                }
                                "type" => {
                                    // TODO: Map Python type to Rust Type
                                    // For now, just detect common types by variable name
                                    if let HirExpr::Var(type_name) = kw_value {
                                        match type_name.as_str() {
                                            "str" => arg.arg_type = Some(crate::hir::Type::String),
                                            "int" => arg.arg_type = Some(crate::hir::Type::Int),
                                            "Path" => {
                                                // Path needs to map to PathBuf
                                                arg.arg_type = Some(crate::hir::Type::Custom("PathBuf".to_string()));
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                "action" => {
                                    if let HirExpr::Literal(crate::hir::Literal::String(action_val)) = kw_value {
                                        arg.action = Some(action_val.clone());
                                    }
                                }
                                "help" => {
                                    if let HirExpr::Literal(crate::hir::Literal::String(help_val)) = kw_value {
                                        arg.help = Some(help_val.clone());
                                    }
                                }
                                "default" => {
                                    arg.default = Some(kw_value.clone());
                                }
                                _ => {
                                    // Ignore other kwargs for now (e.g., dest, required, choices)
                                }
                            }
                        }

                        _parser_info.add_argument(arg);
                    }

                    // Skip generating this statement - arguments will be in Args struct
                    return Ok(quote! {});
                }
            }
        }
    }

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
        let mut expr_tokens = e.to_rust_expr(ctx)?;

        // DEPYLER-0241: Apply type conversion if needed (e.g., usize -> i32 from enumerate())
        if let Some(return_type) = &ctx.current_return_type {
            // Unwrap Optional to get the underlying type
            let target_type = match return_type {
                Type::Optional(inner) => inner.as_ref(),
                other => other,
            };

            // DEPYLER-0272: Pass expression to check if cast is actually needed
            if needs_type_conversion(target_type, e) {
                expr_tokens = apply_type_conversion(expr_tokens, target_type);
            }
        }

        // Check if return type is Optional and wrap value in Some()
        let is_optional_return =
            matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));

        // DEPYLER-0330: DISABLED - Heuristic too broad, breaks plain int variables named "result"
        // Original logic: Unwrap Option-typed variables when returning from non-Optional function
        // Problem: Can't distinguish between:
        //   1. result = d.get(key)  # Option<T> - needs unwrap
        //   2. result = 0           # i32 - breaks with unwrap
        // TODO: Re-enable when HIR has type tracking to detect actual Option variables
        //
        // if !is_optional_return {
        //     if let HirExpr::Var(var_name) = e {
        //         let is_primitive_return = matches!(
        //             ctx.current_return_type.as_ref(),
        //             Some(Type::Int | Type::Float | Type::Bool | Type::String)
        //         );
        //         if ctx.is_final_statement && var_name == "result" && is_primitive_return {
        //             expr_tokens = parse_quote! { #expr_tokens.unwrap() };
        //         }
        //     }
        // }

        // Check if the expression is None literal
        let is_none_literal = matches!(e, HirExpr::Literal(Literal::None));

        // DEPYLER-0271: For final statement in function, omit `return` keyword (idiomatic Rust)
        // Early returns (not final) keep the `return` keyword
        let use_return_keyword = !ctx.is_final_statement;

        // DEPYLER-0357: Check if function returns void (None in Python -> () in Rust)
        // Must check this BEFORE is_optional_return to avoid false positive
        // Python `-> None` maps to Rust `()`, not `Option<T>`
        let is_void_return = matches!(ctx.current_return_type.as_ref(), Some(Type::None));

        if is_void_return {
            // Void functions (Python -> None): no return value
            if use_return_keyword {
                // Early return from void function: use empty return
                Ok(quote! { return; })
            } else {
                // Final statement in void function: use unit value ()
                // Cannot use None; because it requires type annotations
                Ok(quote! { () })
            }
        } else if ctx.current_function_can_fail {
            if is_optional_return && !is_none_literal {
                // Wrap value in Some() for Optional return types
                if use_return_keyword {
                    Ok(quote! { return Ok(Some(#expr_tokens)); })
                } else {
                    Ok(quote! { Ok(Some(#expr_tokens)) })
                }
            } else if is_optional_return && is_none_literal {
                // DEPYLER-0277: Return None for Optional types (not ())
                if use_return_keyword {
                    Ok(quote! { return Ok(None); })
                } else {
                    Ok(quote! { Ok(None) })
                }
            } else if use_return_keyword {
                Ok(quote! { return Ok(#expr_tokens); })
            } else {
                Ok(quote! { Ok(#expr_tokens) })
            }
        } else if is_optional_return && !is_none_literal {
            // Wrap value in Some() for Optional return types
            if use_return_keyword {
                Ok(quote! { return Some(#expr_tokens); })
            } else {
                Ok(quote! { Some(#expr_tokens) })
            }
        } else if is_optional_return && is_none_literal {
            // DEPYLER-0277: Return None for Optional types (not ()) - non-Result case
            if use_return_keyword {
                Ok(quote! { return None; })
            } else {
                Ok(quote! { None })
            }
        } else if use_return_keyword {
            Ok(quote! { return #expr_tokens; })
        } else {
            Ok(quote! { #expr_tokens })
        }
    } else if ctx.current_function_can_fail {
        // No expression - check if return type is Optional
        let is_optional_return =
            matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));
        let use_return_keyword = !ctx.is_final_statement;

        if is_optional_return {
            if use_return_keyword {
                Ok(quote! { return Ok(None); })
            } else {
                Ok(quote! { Ok(None) })
            }
        } else if use_return_keyword {
            Ok(quote! { return Ok(()); })
        } else {
            Ok(quote! { Ok(()) })
        }
    } else {
        let use_return_keyword = !ctx.is_final_statement;
        if use_return_keyword {
            Ok(quote! { return; })
        } else {
            // Final bare return becomes unit value (implicit)
            Ok(quote! {})
        }
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
///
/// DEPYLER-0310: Wraps exceptions with Box::new() when error type is Box<dyn Error>
/// DEPYLER-0333: Uses scope tracking to determine error handling strategy
#[inline]
pub(crate) fn codegen_raise_stmt(
    exception: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // For V1, we'll implement basic error handling
    if let Some(exc) = exception {
        let exc_expr = exc.to_rust_expr(ctx)?;

        // DEPYLER-0333: Extract exception type to check if it's handled
        let exception_type = extract_exception_type(exc);

        // DEPYLER-0333: Check if exception is caught by current try block
        if ctx.is_exception_handled(&exception_type) {
            // Exception is caught - for now use panic! (control flow jump is complex)
            // TODO: In future, implement proper control flow to jump to handler
            Ok(quote! { panic!("{}", #exc_expr); })
        } else if ctx.current_function_can_fail {
            // Exception propagates to caller - use return Err
            // DEPYLER-0310: Check if we need to wrap with Box::new()
            let needs_boxing = matches!(
                ctx.current_error_type,
                Some(crate::rust_gen::context::ErrorType::DynBox)
            );

            if needs_boxing {
                Ok(quote! { return Err(Box::new(#exc_expr)); })
            } else {
                Ok(quote! { return Err(#exc_expr); })
            }
        } else {
            // Function doesn't return Result - use panic!
            Ok(quote! { panic!("{}", #exc_expr); })
        }
    } else {
        // Re-raise or bare raise - use generic error
        Ok(quote! { return Err("Exception raised".into()); })
    }
}

/// DEPYLER-0333: Extract exception type from raise statement expression
///
/// # Complexity
/// 2 (match + clone)
fn extract_exception_type(exception: &HirExpr) -> String {
    match exception {
        HirExpr::Call { func, .. } => func.clone(),
        HirExpr::Var(name) => name.clone(),
        _ => "Exception".to_string(),
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

    // DEPYLER-0357: Save and restore is_final_statement flag so return statements
    // in with blocks get the explicit 'return' keyword (not treated as final statement)
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

    // Convert body statements
    let body_stmts: Vec<_> = body
        .iter()
        .map(|stmt| stmt.to_rust_tokens(ctx))
        .collect::<Result<_>>()?;

    // Restore is_final_statement flag
    ctx.is_final_statement = saved_is_final;

    // Generate code that calls __enter__() and binds the result
    // Note: __exit__() is not yet called (Drop trait implementation pending)
    if let Some(var_name) = target {
        let var_ident = safe_ident(var_name); // DEPYLER-0023
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

/// Apply Python truthiness conversion to a condition expression
///
/// In Python, any value can be used in a boolean context. This function
/// converts non-boolean expressions to boolean using Python semantics:
/// - String: !expr.is_empty()
/// - List/Dict/Set: !expr.is_empty()
/// - Optional: expr.is_some()
/// - Int: expr != 0
/// - Float: expr != 0.0
/// - Bool: expr (no conversion)
///
/// # DEPYLER-0339
/// Fixes: `if val` where `val: String` failing to compile
fn apply_truthiness_conversion(
    condition: &HirExpr,
    cond_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> syn::Expr {
    // Check if this is a variable reference that needs truthiness conversion
    if let HirExpr::Var(var_name) = condition {
        if let Some(var_type) = ctx.var_types.get(var_name) {
            return match var_type {
                // Already boolean - no conversion needed
                Type::Bool => cond_expr,

                // String/List/Dict/Set - check if empty
                Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                    parse_quote! { !#cond_expr.is_empty() }
                }

                // Optional - check if Some
                Type::Optional(_) => {
                    parse_quote! { #cond_expr.is_some() }
                }

                // Numeric types - check if non-zero
                Type::Int => {
                    parse_quote! { #cond_expr != 0 }
                }
                Type::Float => {
                    parse_quote! { #cond_expr != 0.0 }
                }

                // Unknown or other types - use as-is (may fail compilation)
                _ => cond_expr,
            };
        }
    }

    // Not a variable or no type info - use as-is
    cond_expr
}

/// Generate code for If statement with optional else clause
#[inline]
pub(crate) fn codegen_if_stmt(
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let mut cond = condition.to_rust_expr(ctx)?;

    // DEPYLER-0308: Auto-unwrap Result<bool> in if conditions
    // When a function returns Result<bool, E> (like is_even with modulo),
    // we need to unwrap it for use in boolean context
    // Check if the condition is a Call to a function that returns Result<bool>
    if let HirExpr::Call { func, .. } = condition {
        if ctx.result_bool_functions.contains(func) {
            // This function returns Result<bool>, so unwrap it
            // Use .unwrap_or(false) to handle potential errors gracefully
            cond = parse_quote! { #cond.unwrap_or(false) };
        }
    }

    // DEPYLER-0339: Apply Python truthiness conversion
    // Convert non-boolean expressions to boolean (e.g., `if val` where val: String)
    cond = apply_truthiness_conversion(condition, cond, ctx);

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

/// Check if a variable is used in an expression
fn is_var_used_in_expr(var_name: &str, expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Var(name) => name == var_name,
        HirExpr::Binary { left, right, .. } => {
            is_var_used_in_expr(var_name, left) || is_var_used_in_expr(var_name, right)
        }
        HirExpr::Unary { operand, .. } => is_var_used_in_expr(var_name, operand),
        HirExpr::Call { func: _, args , ..} => {
            args.iter().any(|arg| is_var_used_in_expr(var_name, arg))
        }
        HirExpr::MethodCall { object, args, .. } => {
            // DEPYLER-0307 Fix #6: Check method receiver and arguments for variable usage
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
        | HirExpr::FrozenSet(elements) => elements.iter().any(|e| is_var_used_in_expr(var_name, e)),
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
                || start
                    .as_ref()
                    .is_some_and(|s| is_var_used_in_expr(var_name, s))
                || stop
                    .as_ref()
                    .is_some_and(|s| is_var_used_in_expr(var_name, s))
                || step
                    .as_ref()
                    .is_some_and(|s| is_var_used_in_expr(var_name, s))
        }
        _ => false, // Literals and other expressions don't reference variables
    }
}

/// Check if a variable is used in an assignment target
fn is_var_used_in_assign_target(var_name: &str, target: &AssignTarget) -> bool {
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

/// Check if a variable is used in a statement
/// DEPYLER-0303 Phase 2: Fixed to check assignment targets too (for `d[k] = v`)
fn is_var_used_in_stmt(var_name: &str, stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            // Check both target (e.g., d[k]) and value (e.g., v)
            is_var_used_in_assign_target(var_name, target) || is_var_used_in_expr(var_name, value)
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            is_var_used_in_expr(var_name, condition)
                || then_body.iter().any(|s| is_var_used_in_stmt(var_name, s))
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(|s| is_var_used_in_stmt(var_name, s)))
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
                || msg
                    .as_ref()
                    .is_some_and(|m| is_var_used_in_expr(var_name, m))
        }
        _ => false,
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
    // DEPYLER-0272: Check if loop variable(s) are used in body
    // If unused, prefix with _ to avoid unused variable warnings with -D warnings

    // Generate target pattern based on AssignTarget type
    let target_pattern: syn::Pat = match target {
        AssignTarget::Symbol(name) => {
            // Check if this variable is used in the loop body
            let is_used = body.iter().any(|stmt| is_var_used_in_stmt(name, stmt));

            // If unused, prefix with underscore
            let var_name = if is_used {
                name.clone()
            } else {
                format!("_{}", name)
            };

            let ident = safe_ident(&var_name); // DEPYLER-0023
            parse_quote! { #ident }
        }
        AssignTarget::Tuple(targets) => {
            // For tuple unpacking, check each variable individually
            let idents: Vec<syn::Ident> = targets
                .iter()
                .map(|t| match t {
                    AssignTarget::Symbol(s) => {
                        // Check if this specific tuple element is used
                        let is_used = body.iter().any(|stmt| is_var_used_in_stmt(s, stmt));
                        let var_name = if is_used {
                            s.clone()
                        } else {
                            format!("_{}", s)
                        };
                        safe_ident(&var_name) // DEPYLER-0023
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
        // DEPYLER-0300/0302: Check if we're iterating over a string
        // Strings use .chars() instead of .iter().cloned()
        // DEPYLER-0302: Exclude plurals (strings, words, etc.) which are collections
        let is_string = matches!(iter, HirExpr::Var(name) if {
            let n = name.as_str();
            // Exact matches (singular forms only)
            (n == "s" || n == "string" || n == "text" || n == "word" || n == "line"
                || n == "char" || n == "character")
            // Prefixes (but not if followed by 's' for plural)
            || (n.starts_with("str") && !n.starts_with("strings"))
            || (n.starts_with("word") && !n.starts_with("words"))
            || (n.starts_with("text") && !n.starts_with("texts"))
            // Suffixes (but exclude plurals)
            || (n.ends_with("_str") && !n.ends_with("_strs"))
            || (n.ends_with("_string") && !n.ends_with("_strings"))
            || (n.ends_with("_word") && !n.ends_with("_words"))
            || (n.ends_with("_text") && !n.ends_with("_texts"))
        });

        if is_string {
            // For strings, use .chars() to iterate over characters
            iter_expr = parse_quote! { #iter_expr.chars() };
        } else {
            // For collections, use .iter().cloned()
            // DEPYLER-0265: Use .iter().cloned() to automatically clone items
            // This handles both Copy types (int, float, bool) and Clone types (String, Vec, etc.)
            // For Copy types, .cloned() is optimized to a simple bit-copy by the compiler.
            // For Clone types, it calls .clone() which is correct for Rust.
            // This matches Python semantics where loop variables are values, not references.
            iter_expr = parse_quote! { #iter_expr.iter().cloned() };
        }
    }

    ctx.enter_scope();

    // DEPYLER-0339: Track loop variable types for truthiness conversion
    // Extract element type from iterator and add to var_types
    let element_type = match iter {
        HirExpr::Var(var_name) => {
            // Simple case: for x in items
            // Look up items type, extract element type
            ctx.var_types.get(var_name).and_then(|t| match t {
                Type::List(elem_t) => Some(*elem_t.clone()),
                Type::Set(elem_t) => Some(*elem_t.clone()),
                Type::Dict(key_t, _) => Some(*key_t.clone()), // dict iteration yields keys
                _ => None,
            })
        }
        HirExpr::Call { func, args , ..} if func == "enumerate" => {
            // enumerate(items) yields (int, elem_type)
            if let Some(HirExpr::Var(var_name)) = args.first() {
                ctx.var_types.get(var_name).and_then(|t| match t {
                    Type::List(elem_t) => Some(Type::Tuple(vec![Type::Int, *elem_t.clone()])),
                    Type::Set(elem_t) => Some(Type::Tuple(vec![Type::Int, *elem_t.clone()])),
                    _ => None,
                })
            } else {
                None
            }
        }
        _ => None,
    };

    // Declare all variables from the target pattern and set their types
    match (target, element_type) {
        (AssignTarget::Symbol(name), Some(elem_type)) => {
            ctx.declare_var(name);
            ctx.var_types.insert(name.clone(), elem_type);
        }
        (AssignTarget::Symbol(name), None) => {
            ctx.declare_var(name);
        }
        (AssignTarget::Tuple(targets), Some(Type::Tuple(elem_types))) if targets.len() == elem_types.len() => {
            // Tuple unpacking with type info: (i, val) from enumerate
            for (t, typ) in targets.iter().zip(elem_types.iter()) {
                if let AssignTarget::Symbol(s) = t {
                    ctx.declare_var(s);
                    ctx.var_types.insert(s.clone(), typ.clone());
                }
            }
        }
        (AssignTarget::Tuple(targets), _) => {
            // Tuple unpacking without type info
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

    // DEPYLER-0307 Fix #8: Handle enumerate() usize index casting
    // When iterating with enumerate(), the first element of the tuple is usize
    // If we're destructuring a tuple and the iterator is enumerate(), cast the first variable to i32
    let needs_enumerate_cast = matches!(iter, HirExpr::Call { func, .. } if func == "enumerate")
        && matches!(target, AssignTarget::Tuple(targets) if !targets.is_empty());

    // DEPYLER-0317: Handle string iteration char→String conversion
    // When iterating over strings with .chars(), convert char to String for HashMap<String, _> compatibility
    // Check if we're iterating over a string (will use .chars()) AND target is a simple symbol
    let needs_char_to_string = matches!(iter, HirExpr::Var(name) if {
        let n = name.as_str();
        (n == "s" || n == "string" || n == "text" || n == "word" || n == "line")
            || (n.starts_with("str") && !n.starts_with("strings"))
            || (n.starts_with("word") && !n.starts_with("words"))
            || (n.starts_with("text") && !n.starts_with("texts"))
            || (n.ends_with("_str") && !n.ends_with("_strs"))
            || (n.ends_with("_string") && !n.ends_with("_strings"))
            || (n.ends_with("_word") && !n.ends_with("_words"))
            || (n.ends_with("_text") && !n.ends_with("_texts"))
    }) && matches!(target, AssignTarget::Symbol(_));

    if needs_enumerate_cast {
        // Get the first variable name from the tuple pattern (the index from enumerate)
        if let AssignTarget::Tuple(targets) = target {
            if let Some(AssignTarget::Symbol(index_var)) = targets.first() {
                // DEPYLER-0272 Fix: Only add cast if index variable is actually used
                // If unused, it will be prefixed with _ in target_pattern, so no cast needed
                let is_index_used = body.iter().any(|stmt| is_var_used_in_stmt(index_var, stmt));

                if is_index_used {
                    // Add a cast statement at the beginning of the loop body
                    let index_ident = safe_ident(index_var); // DEPYLER-0023
                    Ok(quote! {
                        for #target_pattern in #iter_expr {
                            let #index_ident = #index_ident as i32;
                            #(#body_stmts)*
                        }
                    })
                } else {
                    // Index is unused - don't generate cast statement
                    Ok(quote! {
                        for #target_pattern in #iter_expr {
                            #(#body_stmts)*
                        }
                    })
                }
            } else {
                Ok(quote! {
                    for #target_pattern in #iter_expr {
                        #(#body_stmts)*
                    }
                })
            }
        } else {
            Ok(quote! {
                for #target_pattern in #iter_expr {
                    #(#body_stmts)*
                }
            })
        }
    } else if needs_char_to_string {
        // DEPYLER-0317: Convert char to String for HashMap<String, _> operations
        // Python: for char in s: freq[char] = ...
        // Rust: for _char in s.chars() { let char = _char.to_string(); ... }
        if let AssignTarget::Symbol(var_name) = target {
            let var_ident = safe_ident(var_name); // DEPYLER-0023
            let temp_ident = safe_ident(&format!("_{}", var_name)); // DEPYLER-0023
            Ok(quote! {
                for #temp_ident in #iter_expr {
                    let #var_ident = #temp_ident.to_string();
                    #(#body_stmts)*
                }
            })
        } else {
            Ok(quote! {
                for #target_pattern in #iter_expr {
                    #(#body_stmts)*
                }
            })
        }
    } else {
        Ok(quote! {
            for #target_pattern in #iter_expr {
                #(#body_stmts)*
            }
        })
    }
}

/// Check if this is a dict augmented assignment pattern (dict[key] op= value)
/// Returns true if target is Index and value is Binary with left being an Index to same location
fn is_dict_augassign_pattern(target: &AssignTarget, value: &HirExpr) -> bool {
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
                // Simple heuristic: compare base and index expressions
                // (This is simplified - a full solution would do deeper structural comparison)
                return matches!((target_base.as_ref(), value_base.as_ref()),
                    (HirExpr::Var(t_var), HirExpr::Var(v_var)) if t_var == v_var)
                    && matches!((target_index.as_ref(), value_index.as_ref()),
                        (HirExpr::Var(t_idx), HirExpr::Var(v_idx)) if t_idx == v_idx);
            }
        }
    }
    false
}

/// Generate code for Assign statement (variable/index/attribute/tuple assignment)
#[inline]
pub(crate) fn codegen_assign_stmt(
    target: &AssignTarget,
    value: &HirExpr,
    type_annotation: &Option<Type>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0363: Detect ArgumentParser patterns for clap transformation
    // Pattern 1: parser = argparse.ArgumentParser(...) [MethodCall with object=argparse]
    // Pattern 2: args = parser.parse_args() [MethodCall with object=parser]
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::MethodCall { method, object, kwargs, .. } = value {
            // Pattern 1: ArgumentParser constructor
            if method == "ArgumentParser" {
                if let HirExpr::Var(module_name) = object.as_ref() {
                    if module_name == "argparse" {
                        // Register this as an ArgumentParser instance
                        let mut info = crate::rust_gen::argparse_transform::ArgParserInfo::new(var_name.clone());

                        // Extract description and epilog from kwargs
                        for (key, value_expr) in kwargs {
                            if key == "description" {
                                if let HirExpr::Literal(crate::hir::Literal::String(s)) = value_expr {
                                    info.description = Some(s.clone());
                                }
                            } else if key == "epilog" {
                                if let HirExpr::Literal(crate::hir::Literal::String(s)) = value_expr {
                                    info.epilog = Some(s.clone());
                                }
                            }
                        }

                        ctx.argparser_tracker.register_parser(var_name.clone(), info);

                        // Skip generating this statement - it will be replaced by Args struct
                        return Ok(quote! {});
                    }
                }
            }

            // Pattern 2: args = parser.parse_args()
            if method == "parse_args" {
                if let HirExpr::Var(parser_var) = object.as_ref() {
                    // Check if this parser is tracked
                    if let Some(parser_info) = ctx.argparser_tracker.get_parser_mut(parser_var) {
                        // Set the args variable name
                        parser_info.set_args_var(var_name.clone());

                        // Generate Args::parse() instead
                        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                        return Ok(quote! {
                            let #var_ident = Args::parse();
                        });
                    }
                }
            }
        }
    }

    // DEPYLER-0279: Detect and handle dict augmented assignment pattern
    // If we have dict[key] += value, avoid borrow-after-move by evaluating old value first
    if is_dict_augassign_pattern(target, value) {
        if let AssignTarget::Index { base, index } = target {
            if let HirExpr::Binary { op, left: _, right } = value {
                // Generate: let old_val = dict.get(&key).cloned().unwrap_or_default();
                //           dict.insert(key, old_val + right_value);
                let base_expr = base.to_rust_expr(ctx)?;
                let index_expr = index.to_rust_expr(ctx)?;
                let right_expr = right.to_rust_expr(ctx)?;
                let op_token = match op {
                    BinOp::Add => quote! { + },
                    BinOp::Sub => quote! { - },
                    BinOp::Mul => quote! { * },
                    BinOp::Div => quote! { / },
                    BinOp::Mod => quote! { % },
                    _ => bail!("Unsupported augmented assignment operator for dict"),
                };

                return Ok(quote! {
                    {
                        let _key = #index_expr;
                        let _old_val = #base_expr.get(&_key).cloned().unwrap_or_default();
                        #base_expr.insert(_key, _old_val #op_token #right_expr);
                    }
                });
            }
        }
    }

    // DEPYLER-0232: Track variable types for class instances
    // This allows proper method dispatch for user-defined classes
    // DEPYLER-0224: Also track types for set/dict/list literals for proper method dispatch
    // DEPYLER-0301: Track list/vec types from slicing operations
    // DEPYLER-0327 Fix #1: Track String type from Vec<String>.get() method calls
    if let AssignTarget::Symbol(var_name) = target {
        // DEPYLER-0272: Track type from type annotation for function return values
        // This enables correct {:?} vs {} selection in println! for collections
        // Example: result = merge(&a, &b) where merge returns Vec<i32>
        if let Some(annot_type) = type_annotation {
            match annot_type {
                Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                    ctx.var_types.insert(var_name.clone(), annot_type.clone());
                }
                _ => {}
            }
        }

        match value {
            HirExpr::Call { func, .. } => {
                // Check if this is a user-defined class constructor
                if ctx.class_names.contains(func) {
                    ctx.var_types
                        .insert(var_name.clone(), Type::Custom(func.clone()));
                }
                // DEPYLER-0309: Track builtin collection constructors for proper method dispatch
                // This enables correct HashSet.contains() vs HashMap.contains_key() selection
                else if func == "set" {
                    // Infer element type from type annotation or default to Int
                    let elem_type = if let Some(Type::Set(elem)) = type_annotation {
                        elem.as_ref().clone()
                    } else {
                        Type::Int // Default for untyped sets
                    };
                    ctx.var_types
                        .insert(var_name.clone(), Type::Set(Box::new(elem_type)));
                }
                // DEPYLER-0269: Track user-defined function return types
                // Lookup function return type and track it for Display trait selection
                // Enables: result = merge(&a, &b) where merge returns list[int]
                else if let Some(ret_type) = ctx.function_return_types.get(func) {
                    if matches!(ret_type, Type::List(_) | Type::Dict(_, _) | Type::Set(_)) {
                        ctx.var_types.insert(var_name.clone(), ret_type.clone());
                    }
                }
            }
            HirExpr::List(elements) => {
                // DEPYLER-0269: Track list type from literal for auto-borrowing
                // When v = [1, 2], mark v as List(Int) so it gets borrowed when calling f(&v)
                let elem_type = if let Some(Type::List(elem)) = type_annotation {
                    elem.as_ref().clone()
                } else if !elements.is_empty() {
                    // Infer from first element (assume homogeneous list)
                    // For int literals, use Int type
                    Type::Int
                } else {
                    Type::Unknown
                };
                ctx.var_types
                    .insert(var_name.clone(), Type::List(Box::new(elem_type)));
            }
            HirExpr::Dict(items) => {
                // DEPYLER-0269: Track dict type from literal for auto-borrowing
                // When info = {"a": 1}, mark info as Dict(String, Int) so it gets borrowed
                let (key_type, val_type) = if let Some(Type::Dict(k, v)) = type_annotation {
                    (k.as_ref().clone(), v.as_ref().clone())
                } else if !items.is_empty() {
                    // Infer from first item (assume homogeneous dict)
                    // For string literal keys and int values
                    (Type::String, Type::Int)
                } else {
                    (Type::Unknown, Type::Unknown)
                };
                ctx.var_types
                    .insert(var_name.clone(), Type::Dict(Box::new(key_type), Box::new(val_type)));
            }
            HirExpr::Set(elements) | HirExpr::FrozenSet(elements) => {
                // Track set type from literal for proper method dispatch (DEPYLER-0224)
                // Use type annotation if available, otherwise infer from elements
                let elem_type = if let Some(Type::Set(elem)) = type_annotation {
                    elem.as_ref().clone()
                } else if !elements.is_empty() {
                    // Infer from first element (assume homogeneous set)
                    // For int literals, use Int type
                    Type::Int
                } else {
                    Type::Unknown
                };
                ctx.var_types
                    .insert(var_name.clone(), Type::Set(Box::new(elem_type)));
            }
            HirExpr::Slice { base, .. } => {
                // DEPYLER-0301: Track sliced lists as owned Vec types
                // When rest = numbers[1:], mark rest as List(Int) so it gets borrowed on call
                // Infer element type from base variable if available
                let elem_type = if let HirExpr::Var(base_var) = base.as_ref() {
                    if let Some(Type::List(elem)) = ctx.var_types.get(base_var) {
                        elem.as_ref().clone()
                    } else {
                        Type::Int // Default to Int for untyped slices
                    }
                } else {
                    Type::Int // Default to Int
                };
                ctx.var_types
                    .insert(var_name.clone(), Type::List(Box::new(elem_type)));
            }
            // DEPYLER-0327 Fix #1: Track types for method call results
            // E.g., value_str = data.get(...) where data: Vec<String> → value_str: String
            HirExpr::MethodCall { object, method, .. } => {
                // Track .get() on Vec<String> returning String
                if method == "get" {
                    if let HirExpr::Var(obj_var) = object.as_ref() {
                        if let Some(Type::List(elem_type)) = ctx.var_types.get(obj_var) {
                            // .get() returns Option<&T>, but after .cloned().unwrap_or_default()
                            // it becomes T, so track the element type
                            ctx.var_types.insert(var_name.clone(), elem_type.as_ref().clone());
                        }
                    }
                }
                // String methods that return String
                else if matches!(
                    method.as_str(),
                    "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "title" | "replace" | "format"
                ) {
                    ctx.var_types.insert(var_name.clone(), Type::String);
                }
            }
            _ => {}
        }
    }

    let mut value_expr = value.to_rust_expr(ctx)?;

    // DEPYLER-0270: Auto-unwrap Result-returning function calls in assignments
    // When assigning from a function that returns Result<T, E> in a non-Result context,
    // we need to unwrap it. NOTE: expr_gen.rs already adds `?` for Result contexts,
    // so we ONLY add .unwrap() for non-Result contexts to avoid double unwrapping (??).
    if let HirExpr::Call { func, .. } = value {
        if ctx.result_returning_functions.contains(func) && !ctx.current_function_can_fail {
            // Current function doesn't return Result - add .unwrap() to extract the value
            value_expr = parse_quote! { #value_expr.unwrap() };
        }
    }

    // If there's a type annotation, handle type conversions
    let type_annotation_tokens = if let Some(target_type) = type_annotation {
        let target_rust_type = ctx.type_mapper.map_type(target_type);
        let target_syn_type = rust_type_to_syn(&target_rust_type)?;

        // DEPYLER-0272: Check if we need type conversion (e.g., usize to i32)
        // Pass the value expression to determine if cast is actually needed
        if needs_type_conversion(target_type, value) {
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
    // DEPYLER-0023: Use safe_ident to escape Rust keywords (match, type, impl, etc.)
    let target_ident = safe_ident(symbol);

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

    // DEPYLER-0304: Type-aware subscript assignment detection
    // Check base variable type to determine if this is Vec or HashMap
    // Vec.insert() requires usize index, HashMap.insert() takes key of any type
    let is_numeric_index = if let HirExpr::Var(base_name) = base {
        // Check if we have type information for this variable
        if let Some(base_type) = ctx.var_types.get(base_name) {
            // Type-based detection (most reliable)
            match base_type {
                Type::List(_) => true,  // List/Vec → numeric index
                Type::Dict(_, _) => false,  // Dict/HashMap → key (not numeric)
                _ => {
                    // Fall back to index heuristic for other types
                    match index {
                        HirExpr::Var(name) if name == "char" || name == "character" || name == "c" => false,
                        HirExpr::Var(_) | HirExpr::Binary { .. } | HirExpr::Literal(crate::hir::Literal::Int(_)) => true,
                        _ => false,
                    }
                }
            }
        } else {
            // No type info - use heuristic
            match index {
                HirExpr::Var(name) if name == "char" || name == "character" || name == "c" => false,
                HirExpr::Var(_) | HirExpr::Binary { .. } | HirExpr::Literal(crate::hir::Literal::Int(_)) => true,
                _ => false,
            }
        }
    } else {
        // Base is not a simple variable - use heuristic
        match index {
            HirExpr::Var(name) if name == "char" || name == "character" || name == "c" => false,
            HirExpr::Var(_) | HirExpr::Binary { .. } | HirExpr::Literal(crate::hir::Literal::Int(_)) => true,
            _ => false,
        }
    };

    // Extract the base and all intermediate indices
    let (base_expr, indices) = extract_nested_indices_tokens(base, ctx)?;

    if indices.is_empty() {
        // Simple assignment: d[k] = v OR list[i] = x
        if is_numeric_index {
            // DEPYLER-0314: Vec.insert(index as usize, value)
            // Wrap in parentheses to ensure correct operator precedence
            Ok(quote! { #base_expr.insert((#final_index) as usize, #value_expr); })
        } else {
            // HashMap.insert(key, value)
            Ok(quote! { #base_expr.insert(#final_index, #value_expr); })
        }
    } else {
        // Nested assignment: build chain of get_mut calls
        let mut chain = quote! { #base_expr };
        for idx in &indices {
            chain = quote! {
                #chain.get_mut(&#idx).unwrap()
            };
        }

        if is_numeric_index {
            // DEPYLER-0314: Vec.insert(index as usize, value)
            // Wrap in parentheses to ensure correct operator precedence
            Ok(quote! { #chain.insert((#final_index) as usize, #value_expr); })
        } else {
            // HashMap.insert(key, value)
            Ok(quote! { #chain.insert(#final_index, #value_expr); })
        }
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
                    .map(|s| safe_ident(s)) // DEPYLER-0023
                    .collect();
                Ok(quote! { (#(#idents),*) = #value_expr; })
            } else {
                // First declaration - mark each variable individually
                symbols.iter().for_each(|s| ctx.declare_var(s));
                let idents_with_mut: Vec<_> = symbols
                    .iter()
                    .map(|s| {
                        let ident = safe_ident(s); // DEPYLER-0023
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
    // DEPYLER-0358: Detect simple try-except pattern for optimization
    // Pattern: try { return int(str_var) } except ValueError { return literal }
    // We can optimize this to: s.parse::<i32>().unwrap_or(literal)
    // DEPYLER-0359: Exclude patterns with exception binding (except E as e:)
    // Those need proper match with Err(e) binding
    let simple_pattern_info = if body.len() == 1
        && handlers.len() == 1
        && handlers[0].body.len() == 1
        && handlers[0].name.is_none()  // No exception variable binding
    {
        // Check if handler body is a Return statement with a simple value
        match &handlers[0].body[0] {
            // Direct literal: return 42, return "error", etc.
            HirStmt::Return(Some(HirExpr::Literal(lit))) => {
                Some(((match lit {
                    Literal::Int(n) => n.to_string(),
                    Literal::Float(f) => f.to_string(),
                    Literal::String(s) => format!("\"{}\"", s),
                    Literal::Bool(b) => b.to_string(),
                    _ => "Default::default()".to_string(),
                }).to_string(), handlers[0].exception_type.clone()))
            }
            // Unary negation: return -1, return -42, etc.
            HirStmt::Return(Some(HirExpr::Unary { op, operand })) => {
                if let HirExpr::Literal(lit) = &**operand {
                    match (op, lit) {
                        (crate::hir::UnaryOp::Neg, Literal::Int(n)) => {
                            Some((format!("-{}", n), handlers[0].exception_type.clone()))
                        }
                        (crate::hir::UnaryOp::Neg, Literal::Float(f)) => {
                            Some((format!("-{}", f), handlers[0].exception_type.clone()))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    };

    // DEPYLER-0333: Extract handled exception types for scope tracking
    let handled_types: Vec<String> = handlers
        .iter()
        .filter_map(|h| h.exception_type.clone())
        .collect();

    // DEPYLER-0333: Enter try block scope with handled exception types
    // Empty list means bare except (catches all exceptions)
    ctx.enter_try_scope(handled_types.clone());

    // DEPYLER-0360: Check for floor division with ZeroDivisionError handler BEFORE generating try_stmts
    let has_zero_div_handler = handlers
        .iter()
        .any(|h| h.exception_type.as_deref() == Some("ZeroDivisionError"));

    if has_zero_div_handler && body.len() == 1 {
        if let HirStmt::Return(Some(expr)) = &body[0] {
            if contains_floor_div(expr) {
                // Extract divisor from floor division
                let divisor_expr = extract_divisor_from_floor_div(expr)?;
                let divisor_tokens = divisor_expr.to_rust_expr(ctx)?;

                // Find ZeroDivisionError handler
                let zero_div_handler_idx = handlers
                    .iter()
                    .position(|h| h.exception_type.as_deref() == Some("ZeroDivisionError"))
                    .unwrap();

                // Generate handler body
                ctx.enter_scope();
                // DEPYLER-0360: Ensure return keyword is included in handler
                let old_is_final = ctx.is_final_statement;
                ctx.is_final_statement = false;
                let handler_stmts: Vec<_> = handlers[zero_div_handler_idx]
                    .body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                ctx.is_final_statement = old_is_final;
                ctx.exit_scope();

                // Generate try block expression (with params shadowing)
                let floor_div_result = expr.to_rust_expr(ctx)?;

                // DEPYLER-0333: Exit try block scope
                ctx.exit_exception_scope();

                // Generate: if divisor == 0 { handler } else { floor_div_result }
                if let Some(finalbody) = finalbody {
                    ctx.enter_scope();
                    let finally_stmts: Vec<_> = finalbody
                        .iter()
                        .map(|s| s.to_rust_tokens(ctx))
                        .collect::<Result<Vec<_>>>()?;
                    ctx.exit_scope();

                    return Ok(quote! {
                        {
                            if #divisor_tokens == 0 {
                                #(#handler_stmts)*
                            } else {
                                return #floor_div_result;
                            }
                            #(#finally_stmts)*
                        }
                    });
                } else {
                    return Ok(quote! {
                        if #divisor_tokens == 0 {
                            #(#handler_stmts)*
                        } else {
                            return #floor_div_result;
                        }
                    });
                }
            }
        }
    }

    // Convert try body to statements
    ctx.enter_scope();
    let try_stmts: Vec<_> = body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    // DEPYLER-0333: Exit try block scope
    ctx.exit_exception_scope();

    // Generate except handler code
    let mut handler_tokens = Vec::new();
    for handler in handlers {
        // DEPYLER-0333: Enter handler scope for each except clause
        ctx.enter_handler_scope();
        ctx.enter_scope();

        // If there's a name binding, declare it in scope
        if let Some(var_name) = &handler.name {
            ctx.declare_var(var_name);
        }

        // DEPYLER-0357: Handler statements should include 'return' keyword
        // Save and temporarily disable is_final_statement so return statements
        // in handlers get the explicit 'return' keyword (needed for proper exception handling)
        let saved_is_final = ctx.is_final_statement;
        ctx.is_final_statement = false;

        let handler_stmts: Vec<_> = handler
            .body
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;

        // Restore is_final_statement flag
        ctx.is_final_statement = saved_is_final;
        ctx.exit_scope();
        // DEPYLER-0333: Exit handler scope
        ctx.exit_exception_scope();

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
    } else {
        // DEPYLER-0358: Handle simple try-except-return pattern
        // If we detected a simple pattern (try returns, except returns literal),
        // fix the unwrap_or_default() to use the actual exception value
        if let Some((exception_value_str, _exception_type)) = simple_pattern_info {
            // Convert try_stmts to string to post-process
            let try_code = quote! { #(#try_stmts)* };
            let try_str = try_code.to_string();

            // DEPYLER-0358: Replace unwrap_or_default() with unwrap_or(exception_value)
            // This handles the case where int(str) generates .parse().unwrap_or_default()
            // but we want .parse().unwrap_or(-1) based on the except clause
            if try_str.contains("unwrap_or_default") {
                // Parse the try code and replace unwrap_or_default with unwrap_or(value)
                // Handle both "unwrap_or_default ()" and "unwrap_or_default()"
                let fixed_code = try_str
                    .replace("unwrap_or_default ()", &format!("unwrap_or ({})", exception_value_str))
                    .replace("unwrap_or_default()", &format!("unwrap_or({})", exception_value_str));

                // Parse back to token stream
                let fixed_tokens: proc_macro2::TokenStream = fixed_code.parse()
                    .unwrap_or(try_code);

                if let Some(finally_code) = finally_stmts {
                    Ok(quote! {
                        {
                            #fixed_tokens
                            #finally_code
                        }
                    })
                } else {
                    Ok(fixed_tokens)
                }
            } else {
                // Pattern matched but no unwrap_or_default found
                // This means it's not a parse operation, so fall through to normal concatenation
                // to include the exception handler code
                let handler_code = &handler_tokens[0];
                if let Some(finally_code) = finally_stmts {
                    Ok(quote! {
                        {
                            #(#try_stmts)*
                            #handler_code
                            #finally_code
                        }
                    })
                } else {
                    Ok(quote! {
                        {
                            #(#try_stmts)*
                            #handler_code
                        }
                    })
                }
            }
        } else {
            // DEPYLER-0357: Non-simple patterns - use original concatenation logic
            // Execute try block statements, then if we have a single handler, use it
            if handlers.len() == 1 {
                // DEPYLER-0359: Check if handler has exception binding for proper match generation
                if handlers[0].name.is_some() && body.len() == 1 {
                    if let HirStmt::Return(Some(HirExpr::Call { func, args , ..})) = &body[0] {
                        if func == "int" && args.len() == 1 {
                            // Single handler with exception binding - use match with Err(e)
                            let arg_expr = args[0].to_rust_expr(ctx)?;
                            let handler_body = &handler_tokens[0];
                            let err_var = handlers[0].name.as_ref().map(|s| {
                                safe_ident(s) // DEPYLER-0023
                            }).unwrap();

                            if let Some(finally_body) = finalbody {
                                let finally_stmts: Vec<_> = finally_body
                                    .iter()
                                    .map(|s| s.to_rust_tokens(ctx))
                                    .collect::<Result<Vec<_>>>()?;
                                return Ok(quote! {
                                    {
                                        match #arg_expr.parse::<i32>() {
                                            Ok(__value) => __value,
                                            Err(#err_var) => {
                                                #handler_body
                                            }
                                        }
                                        #(#finally_stmts)*
                                    }
                                });
                            } else {
                                return Ok(quote! {
                                    match #arg_expr.parse::<i32>() {
                                        Ok(__value) => __value,
                                        Err(#err_var) => {
                                            #handler_body
                                        }
                                    }
                                });
                            }
                        }
                    }
                }

                // DEPYLER-0362: Check if try block has error handling (unwrap_or_default)
                // If so, don't concatenate handler as it creates invalid syntax
                let try_code_str = quote! { #(#try_stmts)* }.to_string();
                let has_error_handling = try_code_str.contains("unwrap_or_default")
                    || try_code_str.contains("unwrap_or(");

                if has_error_handling {
                    // Try block already handles errors, don't add handler
                    if let Some(finally_code) = finally_stmts {
                        Ok(quote! {
                            {
                                #(#try_stmts)*
                                #finally_code
                            }
                        })
                    } else {
                        Ok(quote! { #(#try_stmts)* })
                    }
                } else {
                    let handler_code = &handler_tokens[0];

                    if let Some(finally_code) = finally_stmts {
                        Ok(quote! {
                            {
                                #(#try_stmts)*
                                #handler_code
                                #finally_code
                            }
                        })
                    } else {
                        // DEPYLER-0357: Include handler code after try block
                        // TODO: This executes both unconditionally - need proper conditional logic
                        // based on which operations can panic (ZeroDivisionError, IndexError, etc.)
                        Ok(quote! {
                            {
                                #(#try_stmts)*
                                #handler_code
                            }
                        })
                    }
                }
            } else {
                // DEPYLER-0359: Multiple handlers - generate conditional error handling
                // For operations like int(data) with multiple exception types, we need proper
                // match-based error handling instead of simple unwrap_or

                // Check if try block is simple return with parse operation
                if body.len() == 1 {
                    if let HirStmt::Return(Some(HirExpr::Call { func, args , ..})) = &body[0] {
                        if func == "int" && args.len() == 1 {
                            let arg_expr = args[0].to_rust_expr(ctx)?;

                            // Check if any handler binds the exception variable
                            let has_exception_binding = handlers.iter().any(|h| h.name.is_some());

                            if has_exception_binding && handlers.len() == 1 {
                                // Single handler with exception binding - use match with Err(e)
                                let handler_body = &handler_tokens[0];
                                let err_var = handlers[0].name.as_ref().map(|s| {
                                    safe_ident(s) // DEPYLER-0023
                                }).unwrap();

                                if let Some(finally_code) = finally_stmts {
                                    return Ok(quote! {
                                        {
                                            match #arg_expr.parse::<i32>() {
                                                Ok(__value) => __value,
                                                Err(#err_var) => {
                                                    #handler_body
                                                }
                                            }
                                            #finally_code
                                        }
                                    });
                                } else {
                                    return Ok(quote! {
                                        match #arg_expr.parse::<i32>() {
                                            Ok(__value) => __value,
                                            Err(#err_var) => {
                                                #handler_body
                                            }
                                        }
                                    });
                                }
                            } else if handlers.len() >= 2 {
                                // DEPYLER-0361: Multiple handlers for int() - include ALL handlers
                                // Convert: try { return int(data) } except ValueError {...} except TypeError {...}
                                // To: if let Ok(v) = data.parse::<i32>() { v } else { handler1; handler2; }

                                // NOTE: Rust's parse() returns a single error type, so we can't dispatch
                                // to specific handlers. We execute all handlers sequentially.
                                // This is semantically incorrect but compiles. TODO: Improve error dispatch.

                                if let Some(finally_code) = finally_stmts {
                                    return Ok(quote! {
                                        {
                                            if let Ok(__parse_result) = #arg_expr.parse::<i32>() {
                                                __parse_result
                                            } else {
                                                #(#handler_tokens)*
                                            }
                                            #finally_code
                                        }
                                    });
                                } else {
                                    return Ok(quote! {
                                        {
                                            if let Ok(__parse_result) = #arg_expr.parse::<i32>() {
                                                __parse_result
                                            } else {
                                                #(#handler_tokens)*
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    }
                }

                // DEPYLER-0362: Check if try block already handles errors (e.g., unwrap_or_default)
                // In that case, don't concatenate handler tokens as it creates invalid syntax
                let try_code_str = quote! { #(#try_stmts)* }.to_string();
                let has_error_handling = try_code_str.contains("unwrap_or_default")
                    || try_code_str.contains("unwrap_or(");

                if has_error_handling {
                    // Try block has built-in error handling, don't add handlers
                    if let Some(finally_code) = finally_stmts {
                        Ok(quote! {
                            {
                                #(#try_stmts)*
                                #finally_code
                            }
                        })
                    } else {
                        Ok(quote! { #(#try_stmts)* })
                    }
                } else {
                    // DEPYLER-0359: Multiple handlers - include them all
                    // Note: Floor division with ZeroDivisionError is handled earlier (line 1366)
                    if let Some(finally_code) = finally_stmts {
                        Ok(quote! {
                            {
                                #(#try_stmts)*
                                #(#handler_tokens)*
                                #finally_code
                            }
                        })
                    } else {
                        Ok(quote! {
                            {
                                #(#try_stmts)*
                                #(#handler_tokens)*
                            }
                        })
                    }
                }
            }
        }
    }
}

/// DEPYLER-0359: Check if an expression contains floor division operation
fn contains_floor_div(expr: &HirExpr) -> bool {
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

/// DEPYLER-0360: Extract the divisor (right operand) from a floor division expression
fn extract_divisor_from_floor_div(expr: &HirExpr) -> Result<&HirExpr> {
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
