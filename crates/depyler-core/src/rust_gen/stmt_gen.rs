//! Statement code generation
//!
//! This module handles converting HIR statements to Rust token streams.
//! It includes all statement conversion helpers and the HirStmt RustCodeGen trait implementation.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, RustCodeGen, ToRustExpr};
use crate::rust_gen::keywords::safe_ident; // DEPYLER-0023: Keyword escaping
use crate::rust_gen::type_gen::rust_type_to_syn;
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
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

/// DEPYLER-0520: Check if an expression produces an iterator (not a collection)
///
/// Generator expressions and method chains ending in iterator adapters produce
/// iterators that should NOT have .iter().cloned() added when iterated over.
///
/// Complexity: 4 (recursive pattern matching)
fn is_iterator_producing_expr(expr: &HirExpr) -> bool {
    match expr {
        // Generator expressions always produce iterators
        HirExpr::GeneratorExp { .. } => true,

        // Method chains ending in iterator adapters
        HirExpr::MethodCall { method, object, .. } => {
            // Check if this method produces an iterator
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
            // Either this method produces an iterator, or the chain does
            is_iterator_method || is_iterator_producing_expr(object)
        }

        // Some builtin functions produce iterators
        HirExpr::Call { func, .. } => {
            matches!(
                func.as_str(),
                "iter" | "map" | "filter" | "enumerate" | "zip" | "reversed"
            )
        }

        _ => false,
    }
}

/// Check if a type annotation requires explicit conversion
///
/// DEPYLER-0272 FIX: Now checks the actual expression to determine if cast is needed.
/// Only adds cast when expression returns usize (from len(), count(), etc.)
/// Complexity: 3 (type check + expression check)
fn needs_type_conversion(target_type: &Type, expr: &HirExpr, _ctx: &CodeGenContext) -> bool {
    match target_type {
        Type::Int => {
            // Only convert if expression actually returns usize
            // This prevents unnecessary casts like `(x: i32) as i32`
            expr_returns_usize(expr)
        }
        Type::String => {
            // DEPYLER-0455 Bug 7: Convert &str to String for validator functions
            // When returning a parameter from a validator function, the parameter is &str
            // but the return type is Result<String, ...>, so we need .to_string()
            // Apply .to_string() for all variables returned as String - it's safe:
            // - If already String, it's a clone (acceptable overhead)
            // - If &str parameter, it converts correctly
            matches!(expr, HirExpr::Var(_))
        }
        _ => false,
    }
}

/// Apply type conversion to value expression
///
/// Wraps the expression with appropriate conversion (e.g., `as i32`, `.to_string()`)
/// Complexity: 3 (match with 2 arms)
fn apply_type_conversion(value_expr: syn::Expr, target_type: &Type) -> syn::Expr {
    match target_type {
        Type::Int => {
            // Convert to i32 using 'as' cast
            // This handles usize->i32 conversions
            parse_quote! { #value_expr as i32 }
        }
        Type::String => {
            // DEPYLER-0455 Bug 7: Convert &str to String using .to_string()
            // This handles validator function parameters (&str) returned as String
            parse_quote! { #value_expr.to_string() }
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
    if let HirExpr::MethodCall {
        object,
        method,
        args,
        kwargs,
    } = expr
    {
        // DEPYLER-0581: Handle chained method calls like subparsers.add_parser(...).set_defaults(...)
        // Pattern: subparsers.add_parser("step").set_defaults(func=cmd_step)
        // This creates HIR: MethodCall { object: MethodCall { object: Var("subparsers"), method: "add_parser" }, method: "set_defaults" }
        if method == "set_defaults" {
            if let HirExpr::MethodCall {
                object: inner_obj,
                method: inner_method,
                args: inner_args,
                ..
            } = object.as_ref()
            {
                if inner_method == "add_parser" {
                    if let HirExpr::Var(subparsers_var) = inner_obj.as_ref() {
                        if ctx.argparser_tracker.get_subparsers(subparsers_var).is_some() {
                            // Register the subcommand and skip code generation
                            if !inner_args.is_empty() {
                                let command_name = extract_string_literal(&inner_args[0]);
                                ctx.argparser_tracker.register_subcommand(
                                    command_name,
                                    crate::rust_gen::argparse_transform::SubcommandInfo {
                                        name: extract_string_literal(&inner_args[0]),
                                        help: None,
                                        arguments: vec![],
                                        subparsers_var: subparsers_var.clone(),
                                    },
                                );
                            }
                            return Ok(quote! {});
                        }
                    }
                }
            }
        }

        // DEPYLER-0394: Skip ALL parser method calls when using clap derive
        // ArgumentParser methods that should be ignored:
        // - add_argument() → accumulated into Args struct
        // - add_argument_group() → not needed with clap (uses struct fields)
        // - set_defaults() → not needed (use field defaults)
        // - add_mutually_exclusive_group() → use clap group attributes
        if let HirExpr::Var(var_name) = object.as_ref() {
            // DEPYLER-0399: Check if this is a subcommand parser first (highest priority)
            if let Some(subcommand_info) = ctx.argparser_tracker.get_subcommand_mut(var_name) {
                // This is a subcommand parser - route add_argument to subcommand
                if method == "add_argument" {
                    // Extract argument details (same as main parser)
                    if let Some(HirExpr::Literal(crate::hir::Literal::String(first_arg))) =
                        args.first()
                    {
                        let mut arg = crate::rust_gen::argparse_transform::ArgParserArgument::new(
                            first_arg.clone(),
                        );

                        // Check for second argument (long flag)
                        if let Some(HirExpr::Literal(crate::hir::Literal::String(second_arg))) =
                            args.get(1)
                        {
                            if second_arg.starts_with("--") {
                                arg.long = Some(second_arg.clone());
                            }
                        }

                        // Extract kwargs (same extraction logic as main parser)
                        for (kw_name, kw_value) in kwargs {
                            match kw_name.as_str() {
                                "help" => {
                                    if let HirExpr::Literal(crate::hir::Literal::String(help_val)) =
                                        kw_value
                                    {
                                        arg.help = Some(help_val.clone());
                                    }
                                }
                                "type" => {
                                    if let HirExpr::Var(type_name) = kw_value {
                                        match type_name.as_str() {
                                            "str" => arg.arg_type = Some(crate::hir::Type::String),
                                            "int" => arg.arg_type = Some(crate::hir::Type::Int),
                                            "float" => arg.arg_type = Some(crate::hir::Type::Float),
                                            "Path" => {
                                                arg.arg_type = Some(crate::hir::Type::Custom(
                                                    "PathBuf".to_string(),
                                                ))
                                            }
                                            _ => {
                                                // DEPYLER-0447: Track custom validator functions
                                                // e.g., type=email_address → track "email_address"
                                                ctx.validator_functions.insert(type_name.clone());
                                            }
                                        }
                                    }
                                }
                                "action" => {
                                    if let HirExpr::Literal(crate::hir::Literal::String(
                                        action_val,
                                    )) = kw_value
                                    {
                                        arg.action = Some(action_val.clone());
                                    }
                                }
                                "required" => {
                                    if let HirExpr::Literal(crate::hir::Literal::Bool(req)) =
                                        kw_value
                                    {
                                        arg.required = Some(*req);
                                    }
                                }
                                "nargs" => {
                                    // DEPYLER-0485: Handle nargs for subcommand arguments
                                    // Same logic as main parser (lines 336-348)
                                    match kw_value {
                                        HirExpr::Literal(crate::hir::Literal::String(
                                            nargs_val,
                                        )) => {
                                            arg.nargs = Some(nargs_val.clone());
                                        }
                                        HirExpr::Literal(crate::hir::Literal::Int(n)) => {
                                            arg.nargs = Some(n.to_string());
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }

                        subcommand_info.arguments.push(arg);
                    }
                    return Ok(quote! {});
                }
            }

            // DEPYLER-0396: Check if this variable is a tracked ArgumentParser OR an argument group
            // If it's a group, resolve to the parent parser (recursively for nested groups)
            let parser_var = if ctx.argparser_tracker.get_parser(var_name).is_some() {
                var_name.clone()
            } else if let Some(parent_parser) = ctx.argparser_tracker.get_parser_for_group(var_name)
            {
                parent_parser // Already returns owned String
            } else if ctx.argparser_tracker.get_subparsers(var_name).is_some() {
                // DEPYLER-0456 Bug #1 FIX: Handle subparsers.add_parser() expression statements
                // Don't early return here - let it fall through to add_parser handling at line ~435
                var_name.clone()
            } else {
                // Not a parser, group, subparsers, or subcommand - fall through to normal code generation
                let expr_tokens = expr.to_rust_expr(ctx)?;
                return Ok(quote! { #expr_tokens; });
            };

            // DEPYLER-0456 Bug #1 FIX: Check for subparsers.add_parser() FIRST
            // This must come before the parser check since subparsers variables are NOT in the parser list
            if ctx.argparser_tracker.get_subparsers(&parser_var).is_some() && method == "add_parser"
            {
                // Handle subparsers.add_parser() expression statements
                // Pattern: subparsers.add_parser("init", help="...")
                if !args.is_empty() {
                    let command_name = extract_string_literal(&args[0]);
                    let help = extract_kwarg_string(kwargs, "help");

                    // Register subcommand without a variable name (since it's not assigned)
                    use crate::rust_gen::argparse_transform::SubcommandInfo;
                    let subcommand_info = SubcommandInfo {
                        name: command_name.clone(),
                        help,
                        arguments: vec![],
                        subparsers_var: parser_var.clone(),
                    };

                    // Use the command name itself as the key (since there's no parser variable)
                    ctx.argparser_tracker
                        .register_subcommand(command_name, subcommand_info);
                }
                // Skip code generation for this statement
                return Ok(quote! {});
            }

            // Check if this is a parser configuration method
            if ctx.argparser_tracker.get_parser(&parser_var).is_some() {
                match method.as_str() {
                    "add_argument" => {
                        // Process add_argument to extract argument details
                        if let Some(_parser_info) =
                            ctx.argparser_tracker.get_parser_mut(&parser_var)
                        {
                            // DEPYLER-0365 Phase 5: Extract argument names (can be multiple: "-o", "--output")
                            // First arg is required, second is optional (for dual short+long flags)
                            if let Some(HirExpr::Literal(crate::hir::Literal::String(first_arg))) =
                                args.first()
                            {
                                let mut arg =
                                    crate::rust_gen::argparse_transform::ArgParserArgument::new(
                                        first_arg.clone(),
                                    );

                                // Check for second argument (long flag name in dual short+long pattern)
                                if let Some(HirExpr::Literal(crate::hir::Literal::String(
                                    second_arg,
                                ))) = args.get(1)
                                {
                                    // Pattern: add_argument("-o", "--output")
                                    // First is short, second is long
                                    if second_arg.starts_with("--") {
                                        arg.long = Some(second_arg.clone());
                                    }
                                }

                                // DEPYLER-0364: Extract keyword arguments from HIR
                                for (kw_name, kw_value) in kwargs {
                                    match kw_name.as_str() {
                                        "nargs" => {
                                            // DEPYLER-0370: Handle both string and int nargs
                                            match kw_value {
                                                HirExpr::Literal(crate::hir::Literal::String(
                                                    nargs_val,
                                                )) => {
                                                    arg.nargs = Some(nargs_val.clone());
                                                }
                                                HirExpr::Literal(crate::hir::Literal::Int(n)) => {
                                                    arg.nargs = Some(n.to_string());
                                                }
                                                _ => {}
                                            }
                                        }
                                        "type" => {
                                            // DEPYLER-0367: Map Python types to Rust types
                                            if let HirExpr::Var(type_name) = kw_value {
                                                match type_name.as_str() {
                                                    "str" => {
                                                        arg.arg_type =
                                                            Some(crate::hir::Type::String)
                                                    }
                                                    "int" => {
                                                        arg.arg_type = Some(crate::hir::Type::Int)
                                                    }
                                                    "float" => {
                                                        arg.arg_type = Some(crate::hir::Type::Float)
                                                    }
                                                    "Path" => {
                                                        // Path needs to map to PathBuf
                                                        arg.arg_type =
                                                            Some(crate::hir::Type::Custom(
                                                                "PathBuf".to_string(),
                                                            ));
                                                    }
                                                    _ => {
                                                        // DEPYLER-0447: Track custom validator functions
                                                        // e.g., type=email_address → track "email_address"
                                                        ctx.validator_functions
                                                            .insert(type_name.clone());
                                                    }
                                                }
                                            }
                                        }
                                        "action" => {
                                            if let HirExpr::Literal(crate::hir::Literal::String(
                                                action_val,
                                            )) = kw_value
                                            {
                                                arg.action = Some(action_val.clone());
                                            }
                                        }
                                        "help" => {
                                            if let HirExpr::Literal(crate::hir::Literal::String(
                                                help_val,
                                            )) = kw_value
                                            {
                                                arg.help = Some(help_val.clone());
                                            }
                                        }
                                        "default" => {
                                            arg.default = Some(kw_value.clone());
                                        }
                                        "required" => {
                                            // DEPYLER-0367: Handle required=True/False
                                            if let HirExpr::Literal(crate::hir::Literal::Bool(
                                                req,
                                            )) = kw_value
                                            {
                                                arg.required = Some(*req);
                                            }
                                        }
                                        "dest" => {
                                            // DEPYLER-0371: Handle dest="var_name"
                                            if let HirExpr::Literal(crate::hir::Literal::String(
                                                dest_name,
                                            )) = kw_value
                                            {
                                                arg.dest = Some(dest_name.clone());
                                            }
                                        }
                                        "metavar" => {
                                            // DEPYLER-0372: Handle metavar="FILE"
                                            if let HirExpr::Literal(crate::hir::Literal::String(
                                                metavar_name,
                                            )) = kw_value
                                            {
                                                arg.metavar = Some(metavar_name.clone());
                                            }
                                        }
                                        "choices" => {
                                            // DEPYLER-0373: Handle choices=["a", "b", "c"]
                                            if let HirExpr::List(items) = kw_value {
                                                let mut choices = Vec::new();
                                                for item in items {
                                                    if let HirExpr::Literal(
                                                        crate::hir::Literal::String(s),
                                                    ) = item
                                                    {
                                                        choices.push(s.clone());
                                                    }
                                                }
                                                if !choices.is_empty() {
                                                    arg.choices = Some(choices);
                                                }
                                            }
                                        }
                                        "const" => {
                                            // DEPYLER-0374/0375: Handle const value
                                            arg.const_value = Some(kw_value.clone());
                                        }
                                        _ => {
                                            // Ignore other kwargs (e.g., prog, formatter_class)
                                        }
                                    }
                                }

                                _parser_info.add_argument(arg);
                            }

                            // Skip generating this statement - arguments will be in Args struct
                            return Ok(quote! {});
                        }
                    }
                    "add_argument_group" | "add_mutually_exclusive_group" | "set_defaults" => {
                        // DEPYLER-0394: Skip these parser configuration methods
                        // With clap derive, argument groups are handled by struct field organization
                        // Mutually exclusive groups use #[group] attributes
                        // Defaults use field default values
                        return Ok(quote! {});
                    }
                    "add_parser" => {
                        // DEPYLER-0456: Handle subparsers.add_parser() expression statements
                        // Pattern: subparsers.add_parser("init", help="...")
                        // Register the subcommand and skip code generation
                        if ctx.argparser_tracker.get_subparsers(var_name).is_some() {
                            // Extract command name and help text
                            if !args.is_empty() {
                                let command_name = extract_string_literal(&args[0]);
                                let help = extract_kwarg_string(kwargs, "help");

                                // Register subcommand without a variable name (since it's not assigned)
                                use crate::rust_gen::argparse_transform::SubcommandInfo;
                                let subcommand_info = SubcommandInfo {
                                    name: command_name.clone(),
                                    help,
                                    arguments: vec![],
                                    subparsers_var: var_name.clone(),
                                };

                                // Use the command name itself as the key (since there's no parser variable)
                                ctx.argparser_tracker
                                    .register_subcommand(command_name, subcommand_info);
                            }
                            return Ok(quote! {});
                        }
                    }
                    _ => {
                        // Other parser methods - fall through to normal code generation
                    }
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
    // CITL: Trace return type handling decision
    trace_decision!(
        category = DecisionCategory::TypeMapping,
        name = "return_stmt",
        chosen = "return_expr",
        alternatives = ["return_unit", "return_result", "return_option", "implicit_return"],
        confidence = 0.92
    );

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
            // DEPYLER-0455 Bug 7: Also pass ctx to check validator function context
            if needs_type_conversion(target_type, e, ctx) {
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
        // NOTE: Re-enable unwrap_or optimization when HIR has type tracking (tracked in DEPYLER-0424)
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

        // DEPYLER-0498: Check if expression is if-expr with None arm (ternary with None)
        // Pattern: `return x if cond else None` -> should be `if cond { Some(x) } else { None }`
        // NOT: `Some(if cond { x } else { None })`
        let is_if_expr_with_none = matches!(
            e,
            HirExpr::IfExpr { orelse, .. } if matches!(&**orelse, HirExpr::Literal(Literal::None))
        );

        // DEPYLER-0271: For final statement in function, omit `return` keyword (idiomatic Rust)
        // Early returns (not final) keep the `return` keyword
        let use_return_keyword = !ctx.is_final_statement;

        // DEPYLER-0357: Check if function returns void (None in Python -> () in Rust)
        // Must check this BEFORE is_optional_return to avoid false positive
        // Python `-> None` maps to Rust `()`, not `Option<T>`
        let is_void_return = matches!(ctx.current_return_type.as_ref(), Some(Type::None));

        if ctx.current_function_can_fail {
            if is_void_return && is_none_literal {
                // Void function with can_fail: return Ok(()) for `return None`
                if use_return_keyword {
                    Ok(quote! { return Ok(()); })
                } else {
                    Ok(quote! { Ok(()) })
                }
            } else if is_optional_return && !is_none_literal {
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
            } else if ctx.is_main_function {
                // DEPYLER-0617: Handle exit code returns in main() function
                // Python pattern: `def main() -> int: ... return 1`
                // Rust main() can only return () or Result<(), E>, so integer returns
                // must be converted to process::exit() for non-zero or Ok(()) for zero
                if let HirExpr::Literal(Literal::Int(exit_code)) = e {
                    if *exit_code == 0 {
                        // Success exit code -> Ok(())
                        if use_return_keyword {
                            Ok(quote! { return Ok(()); })
                        } else {
                            Ok(quote! { Ok(()) })
                        }
                    } else {
                        // Non-zero exit code -> std::process::exit(N)
                        let code = *exit_code as i32;
                        Ok(quote! { std::process::exit(#code) })
                    }
                } else {
                    // Other expressions in main - just return Ok(())
                    if use_return_keyword {
                        Ok(quote! { return Ok(()); })
                    } else {
                        Ok(quote! { Ok(()) })
                    }
                }
            } else if use_return_keyword {
                Ok(quote! { return Ok(#expr_tokens); })
            } else {
                Ok(quote! { Ok(#expr_tokens) })
            }
        } else if is_void_return {
            // Void functions (Python -> None): no return value (non-fallible)
            if use_return_keyword {
                // Early return from void function: use empty return
                Ok(quote! { return; })
            } else {
                // Final statement in void function: use unit value ()
                Ok(quote! { () })
            }
        } else if is_optional_return && !is_none_literal && !is_if_expr_with_none {
            // Wrap value in Some() for Optional return types
            // DEPYLER-0498: Skip wrapping if if-expr has None arm (handled separately)
            if use_return_keyword {
                Ok(quote! { return Some(#expr_tokens); })
            } else {
                Ok(quote! { Some(#expr_tokens) })
            }
        } else if is_optional_return && is_if_expr_with_none {
            // DEPYLER-0498: If-expr with None arm - manually wrap true arm in Some()
            // Pattern: `return x if cond else None` -> `if cond { Some(x) } else { None }`
            if let HirExpr::IfExpr {
                test,
                body,
                orelse: _,
            } = e
            {
                let test_tokens = test.to_rust_expr(ctx)?;
                let body_tokens = body.to_rust_expr(ctx)?;

                if use_return_keyword {
                    Ok(quote! { return if #test_tokens { Some(#body_tokens) } else { None }; })
                } else {
                    Ok(quote! { if #test_tokens { Some(#body_tokens) } else { None } })
                }
            } else {
                unreachable!("is_if_expr_with_none should only match IfExpr")
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
///
/// DEPYLER-0421: Applies Python truthiness conversion to the condition
#[inline]
pub(crate) fn codegen_while_stmt(
    condition: &HirExpr,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let mut cond = condition.to_rust_expr(ctx)?;

    // DEPYLER-0421: Apply Python truthiness conversion for while loops
    // Convert non-boolean expressions to boolean (e.g., `while queue` where queue: VecDeque)
    cond = apply_truthiness_conversion(condition, cond, ctx);

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
        // DEPYLER-0398: Handle argparse.ArgumentTypeError specially
        // Pattern: raise argparse.ArgumentTypeError("message")
        // Extract message and use directly in panic!/error
        let exc_expr = match exc {
            // Pattern 1: argparse.ArgumentTypeError(msg)
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } if matches!(object.as_ref(), HirExpr::Var(v) if v == "argparse")
                && method == "ArgumentTypeError"
                && !args.is_empty() =>
            {
                // Extract the message argument and use it directly
                args[0].to_rust_expr(ctx)?
            }
            // Pattern 2: ArgumentTypeError(msg) - if imported
            HirExpr::Call { func, args, .. } if func == "ArgumentTypeError" && !args.is_empty() => {
                args[0].to_rust_expr(ctx)?
            }
            // Default: use exception as-is
            _ => exc.to_rust_expr(ctx)?,
        };

        // DEPYLER-0333: Extract exception type to check if it's handled
        let exception_type = extract_exception_type(exc);

        // DEPYLER-0438: Set error type flag for generation
        // DEPYLER-0551: Added RuntimeError and FileNotFoundError
        match exception_type.as_str() {
            "ValueError" => ctx.needs_valueerror = true,
            "ArgumentTypeError" => ctx.needs_argumenttypeerror = true,
            "ZeroDivisionError" => ctx.needs_zerodivisionerror = true,
            "IndexError" => ctx.needs_indexerror = true,
            "RuntimeError" => ctx.needs_runtimeerror = true,
            "FileNotFoundError" => ctx.needs_filenotfounderror = true,
            _ => {}
        }

        // DEPYLER-0333: Check if exception is caught by current try block
        if ctx.is_exception_handled(&exception_type) {
            // Exception is caught - for now use panic! (control flow jump is complex)
            // NOTE: Implement proper exception control flow to jump to handler (tracked in DEPYLER-0424)
            Ok(quote! { panic!("{}", #exc_expr); })
        } else if ctx.current_function_can_fail {
            // Exception propagates to caller - use return Err
            // DEPYLER-0310: Check if we need to wrap with Box::new()
            let needs_boxing = matches!(
                ctx.current_error_type,
                Some(crate::rust_gen::context::ErrorType::DynBox)
            );

            if needs_boxing {
                // DEPYLER-0438: Wrap exception in error type constructor if it's a known exception
                // format!() returns String which doesn't implement std::error::Error
                // Need to wrap in ValueError::new(), ArgumentTypeError::new(), etc.
                // DEPYLER-0472-FIX: Don't double-wrap if exc is already a Call to the exception type
                let is_already_wrapped = matches!(
                    exc,
                    HirExpr::Call { func, .. } if func == &exception_type
                );

                // DEPYLER-0551: Added RuntimeError and FileNotFoundError
                if !is_already_wrapped
                    && (exception_type == "ValueError"
                        || exception_type == "ArgumentTypeError"
                        || exception_type == "TypeError"
                        || exception_type == "KeyError"
                        || exception_type == "IndexError"
                        || exception_type == "RuntimeError"
                        || exception_type == "FileNotFoundError")
                {
                    let exc_type = safe_ident(&exception_type);
                    Ok(quote! { return Err(Box::new(#exc_type::new(#exc_expr))); })
                } else {
                    Ok(quote! { return Err(Box::new(#exc_expr)); })
                }
            } else {
                // DEPYLER-0455: Also wrap exception in type constructor when not boxing
                // Without this, `return Err(format!(...))` returns String instead of ExceptionType
                // DEPYLER-0472-FIX: Don't double-wrap if exc is already a Call to the exception type
                let is_already_wrapped = matches!(
                    exc,
                    HirExpr::Call { func, .. } if func == &exception_type
                );

                // DEPYLER-0551: Added RuntimeError and FileNotFoundError
                if !is_already_wrapped
                    && (exception_type == "ValueError"
                        || exception_type == "ArgumentTypeError"
                        || exception_type == "TypeError"
                        || exception_type == "KeyError"
                        || exception_type == "IndexError"
                        || exception_type == "RuntimeError"
                        || exception_type == "FileNotFoundError")
                {
                    let exc_type = safe_ident(&exception_type);
                    Ok(quote! { return Err(#exc_type::new(#exc_expr)); })
                } else {
                    Ok(quote! { return Err(#exc_expr); })
                }
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
        HirExpr::MethodCall { method, .. } => method.clone(),
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

    // DEPYLER-0387: Detect if context is from open() builtin
    // DEPYLER-0533: Also detect tempfile patterns (NamedTemporaryFile, TemporaryDirectory)
    // These return file-like objects that bind directly without __enter__()
    let is_file_context_manager = matches!(
        context,
        HirExpr::Call { func, .. } if func.as_str() == "open"
    ) || matches!(
        context,
        HirExpr::MethodCall { object, method, .. }
        if matches!(object.as_ref(), HirExpr::Var(module) if module == "tempfile")
            && (method == "NamedTemporaryFile" || method == "TemporaryDirectory"
                || method == "TemporaryFile" || method == "SpooledTemporaryFile"
                || method == "NamedTempFile")
    );

    // Generate code that calls __enter__() for custom context managers
    // or binds File directly for open() calls
    // Note: __exit__() is not yet called (Drop trait implementation pending)
    if let Some(var_name) = target {
        let var_ident = safe_ident(var_name); // DEPYLER-0023
        ctx.declare_var(var_name);

        if is_file_context_manager {
            // DEPYLER-0387: For open() calls, bind File directly (no __enter__)
            // DEPYLER-0533: Also for tempfile patterns
            // DEPYLER-0417: No block wrapper - Python allows accessing variables from with blocks
            // DEPYLER-0458: Add mut to file handles for Read/Write trait methods
            Ok(quote! {
                let mut #var_ident = #context_expr;
                #(#body_stmts)*
            })
        } else {
            // For custom context managers, call __enter__()
            // DEPYLER-0417: No block wrapper - Python allows accessing variables from with blocks
            // DEPYLER-0602: Context variable must be mutable since __enter__ takes &mut self
            Ok(quote! {
                let mut _context = #context_expr;
                let #var_ident = _context.__enter__();
                #(#body_stmts)*
            })
        }
    } else {
        // DEPYLER-0417: No block wrapper - Python allows accessing variables from with blocks
        // DEPYLER-0602: Context variable must be mutable for __enter__() if called
        Ok(quote! {
            let mut _context = #context_expr;
            #(#body_stmts)*
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

        // DEPYLER-0517: Heuristic fallback for common string variable names
        // This handles variables from tuple unpacking that aren't tracked in var_types
        // e.g., `let (returncode, stdout, stderr) = run_command(...)`
        let var_str = var_name.as_str();
        if var_str == "stdout"
            || var_str == "stderr"
            || var_str == "output"
            || var_str == "result"
            || var_str.ends_with("_output")
            || var_str.ends_with("_result")
            || var_str.ends_with("_str")
            || var_str.ends_with("_string")
        {
            return parse_quote! { !#cond_expr.is_empty() };
        }
    }

    // DEPYLER-0570: Handle dict index access in conditions
    // Python: `if info["extension"]:` checks if the value is truthy (non-empty string)
    // Rust: info.get("extension")... returns serde_json::Value, need to check truthiness
    // Convert to: `.as_str().map_or(false, |s| !s.is_empty())` for string values
    if let HirExpr::Index { base, index } = condition {
        // Check if using string key (dict-like access)
        let has_string_key = matches!(index.as_ref(), HirExpr::Literal(Literal::String(_)));

        // Check if base is a dict (HashMap) or common dict variable name
        let is_dict_access = if let HirExpr::Var(var_name) = base.as_ref() {
            // Known dict type
            if let Some(var_type) = ctx.var_types.get(var_name) {
                matches!(var_type, Type::Dict(_, _))
            } else {
                // Unknown type - use string key OR common dict variable names as heuristics
                let name = var_name.as_str();
                has_string_key
                    || name == "info"
                    || name == "data"
                    || name == "config"
                    || name == "options"
                    || name == "result"
                    || name == "response"
                    || name.ends_with("_info")
                    || name.ends_with("_data")
                    || name.ends_with("_dict")
            }
        } else {
            // Nested access or other expression - use string key as heuristic
            has_string_key
        };

        if is_dict_access {
            // Dict value access - check if the Value is truthy
            // serde_json::Value truthiness: string must be non-empty
            return parse_quote! { #cond_expr.as_str().map_or(false, |s| !s.is_empty()) };
        }
    }

    // DEPYLER-0446: Check if this is an attribute access to an optional argparse field
    // Python: if args.output (where output is optional)
    // Rust: if args.output.is_some()
    if let HirExpr::Attribute { value, attr } = condition {
        if let HirExpr::Var(obj_name) = value.as_ref() {
            // Check if this is accessing an args variable from ArgumentParser
            let is_args_var = ctx.argparser_tracker.parsers.values().any(|parser_info| {
                parser_info
                    .args_var
                    .as_ref()
                    .is_some_and(|args_var| args_var == obj_name)
            });

            if is_args_var {
                // Check if this field is optional (Option<T> type, not boolean)
                let is_optional_field = ctx.argparser_tracker.parsers.values().any(|parser_info| {
                    parser_info.arguments.iter().any(|arg| {
                        let field_name = arg.rust_field_name();
                        if field_name != *attr {
                            return false;
                        }

                        // Argument is NOT an Option if it has action="store_true" or "store_false"
                        if matches!(
                            arg.action.as_deref(),
                            Some("store_true") | Some("store_false")
                        ) {
                            return false;
                        }

                        // Argument is an Option<T> if: not required AND no default value AND not positional
                        // Positional arguments are always required (Vec for nargs)
                        !arg.is_positional
                            && !arg.required.unwrap_or(false)
                            && arg.default.is_none()
                    })
                });

                if is_optional_field {
                    // DEPYLER-0108: Check if this field has been precomputed
                    // to avoid borrow-after-move when Option is passed then checked
                    if ctx.precomputed_option_fields.contains(attr) {
                        let has_var_name = format!("has_{}", attr);
                        let has_ident =
                            syn::Ident::new(&has_var_name, proc_macro2::Span::call_site());
                        return parse_quote! { #has_ident };
                    }
                    // Convert Option<T> to boolean using .is_some()
                    return parse_quote! { #cond_expr.is_some() };
                }

                // DEPYLER-0455 Bug 8: Check if this field is a String with a default value
                // Python: if args.encoding (where encoding has default="utf-8")
                // Rust: if !args.encoding.is_empty() (String cannot be used as bool)
                let is_string_with_default =
                    ctx.argparser_tracker.parsers.values().any(|parser_info| {
                        parser_info.arguments.iter().any(|arg| {
                            let field_name = arg.rust_field_name();
                            if field_name != *attr {
                                return false;
                            }

                            // Check if:
                            // 1. Has a default value (arg.default.is_some())
                            // 2. Type is String (arg.arg_type.is_none() means default String type)
                            // 3. Not a boolean action (store_true/store_false)
                            arg.default.is_some()
                                && arg.arg_type.is_none()
                                && !matches!(
                                    arg.action.as_deref(),
                                    Some("store_true") | Some("store_false")
                                )
                        })
                    });

                if is_string_with_default {
                    // Convert String to boolean using !.is_empty()
                    // Note: This is technically redundant since default values are non-empty,
                    // but it's semantically correct for Python truthiness
                    return parse_quote! { !#cond_expr.is_empty() };
                }
            }
        }
    }

    // DEPYLER-0455: Fallback - detect Option types by method call patterns
    // DEPYLER-0519: Check for method calls that return Vec types (like groups())
    // Python: `if match.groups():` checks if groups is non-empty
    // Rust: `if !groups().is_empty()`
    if let HirExpr::MethodCall { method, .. } = condition {
        let vec_returning_methods = [
            "groups",
            "split",
            "split_whitespace",
            "splitlines",
            "findall",
        ];
        if vec_returning_methods.contains(&method.as_str()) {
            return parse_quote! { !#cond_expr.is_empty() };
        }
    }

    // Check if this looks like an Option<T> based on common patterns:
    // - Variable from `env::var(...).ok()` call
    // - Method calls that return Option (dict.get(), etc.)
    if looks_like_option_expr(condition) {
        return parse_quote! { #cond_expr.is_some() };
    }

    // DEPYLER-0570: Fallback - check if the generated expression looks like dict access
    // Pattern: something.get("key").cloned().unwrap_or_default()
    // This returns serde_json::Value which doesn't coerce to bool
    let cond_str = quote::quote!(#cond_expr).to_string();
    if cond_str.contains(".get(") && cond_str.contains("unwrap_or_default") {
        return parse_quote! { #cond_expr.as_str().map_or(false, |s| !s.is_empty()) };
    }

    // Not a variable or no type info - use as-is
    cond_expr
}

/// DEPYLER-0455: Heuristic to detect if an expression likely returns Option<T>
///
/// Checks for common patterns that return Option:
/// - Calls to methods ending with .ok() (Result → Option conversion)
/// - Calls to .get() methods (dict/map lookups)
/// - os.environ.get() / std::env::var().ok()
///
/// DEPYLER-0455: Enhanced to detect chained method calls like env::var(...).ok()
fn looks_like_option_expr(expr: &HirExpr) -> bool {
    match expr {
        // Method call ending in .ok() → definitely Option
        HirExpr::MethodCall { method, .. } if method == "ok" => true,
        // Method call to .get() → usually Option (dict/map lookup)
        HirExpr::MethodCall { method, .. } if method == "get" => true,
        // DEPYLER-0455: Check for chained calls like std::env::var(...).ok()
        // This handles cases where the RHS is a method chain
        HirExpr::MethodCall { object, method, .. } => {
            // Recursively check if the object is an Option-returning expression
            if method == "ok" || method == "get" {
                true
            } else {
                looks_like_option_expr(object)
            }
        }
        _ => false,
    }
}

/// DEPYLER-0379: Extract all simple symbol assignments from a statement block
///
/// Returns a set of variable names that are assigned (not reassigned) in the block.
/// Only captures simple symbol assignments like `x = value`, not `x[i] = value` or `x.attr = value`.
///
/// # Complexity
/// 4 (recursive traversal with set operations)
///
/// DEPYLER-0476: This function is currently unused after switching to extract_toplevel_assigned_symbols
/// for if/else hoisting. Kept for potential future use (e.g., for other optimization passes).
#[allow(dead_code)]
fn extract_assigned_symbols(stmts: &[HirStmt]) -> std::collections::HashSet<String> {
    use std::collections::HashSet;
    let mut symbols = HashSet::new();

    for stmt in stmts {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                ..
            } => {
                symbols.insert(name.clone());
            }
            // Recursively check nested if/else, while, for, try blocks
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                symbols.extend(extract_assigned_symbols(then_body));
                if let Some(else_stmts) = else_body {
                    symbols.extend(extract_assigned_symbols(else_stmts));
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                symbols.extend(extract_assigned_symbols(body));
            }
            HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => {
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
///
/// Example (Python):
/// ```python
/// if condition:
///     value = get_optional()  # Returns Option<String>
/// else:
///     for item in items:
///         value = get_required(item)  # Returns String
/// ```
///
/// The `value` in the for loop should NOT be hoisted because it has a different type.
fn extract_toplevel_assigned_symbols(stmts: &[HirStmt]) -> std::collections::HashSet<String> {
    use std::collections::HashSet;
    let mut symbols = HashSet::new();

    for stmt in stmts {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                ..
            } => {
                symbols.insert(name.clone());
            }
            // Recursively check nested if/else blocks (these are still at the same conceptual level)
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                symbols.extend(extract_toplevel_assigned_symbols(then_body));
                if let Some(else_stmts) = else_body {
                    symbols.extend(extract_toplevel_assigned_symbols(else_stmts));
                }
            }
            // Recursively check try/except blocks (these are still at the same conceptual level)
            HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => {
                symbols.extend(extract_toplevel_assigned_symbols(body));
                for handler in handlers {
                    symbols.extend(extract_toplevel_assigned_symbols(&handler.body));
                }
                if let Some(finally) = finalbody {
                    symbols.extend(extract_toplevel_assigned_symbols(finally));
                }
            }
            // DEPYLER-0476: DO NOT recurse into for/while loops - variables inside loops
            // should not be hoisted because they may have different types/scopes
            HirStmt::While { .. } | HirStmt::For { .. } => {
                // Skip - don't extract symbols from loop bodies
            }
            _ => {}
        }
    }

    symbols
}

/// Generate code for If statement with optional else clause
///
/// DEPYLER-0379: Implements variable hoisting for if/else blocks to fix scope issues.
/// Variables assigned in BOTH if and else branches are hoisted before the if statement.
/// DEPYLER-0476: Only hoist top-level assignments, not variables inside nested for/while loops.
#[inline]
pub(crate) fn codegen_if_stmt(
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    use std::collections::HashSet;

    // CITL: Trace if statement pattern decision
    trace_decision!(
        category = DecisionCategory::TypeMapping,
        name = "if_statement",
        chosen = "if_else",
        alternatives = ["match_pattern", "if_let", "guard", "ternary"],
        confidence = 0.85
    );

    // DEPYLER-0399: Detect subcommand dispatch pattern and convert to match
    if ctx.argparser_tracker.has_subcommands() {
        if let Some(match_stmt) =
            try_generate_subcommand_match(condition, then_body, else_body, ctx)?
        {
            return Ok(match_stmt);
        }
    }

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

    // DEPYLER-0379: Variable hoisting - find variables assigned in BOTH branches
    // DEPYLER-0476: Use toplevel extraction to avoid hoisting variables from nested for/while loops
    let hoisted_vars: HashSet<String> = if let Some(else_stmts) = else_body {
        let then_vars = extract_toplevel_assigned_symbols(then_body);
        let else_vars = extract_toplevel_assigned_symbols(else_stmts);
        then_vars.intersection(&else_vars).cloned().collect()
    } else {
        HashSet::new()
    };

    // DEPYLER-0379: Generate hoisted variable declarations
    // DEPYLER-0439: Skip variables already declared in parent scope (prevents shadowing)
    let mut hoisted_decls = Vec::new();
    for var_name in &hoisted_vars {
        // DEPYLER-0439: Skip if variable is already declared in parent scope
        if ctx.is_declared(var_name) {
            continue;
        }

        // Find the variable's type from the first assignment in either branch
        let var_type = find_variable_type(var_name, then_body).or_else(|| {
            if let Some(else_stmts) = else_body {
                find_variable_type(var_name, else_stmts)
            } else {
                None
            }
        });

        let var_ident = safe_ident(var_name); // DEPYLER-0023

        if let Some(ty) = var_type {
            let rust_type = ctx.type_mapper.map_type(&ty);
            let syn_type = rust_type_to_syn(&rust_type)?;
            hoisted_decls.push(quote! { let mut #var_ident: #syn_type; });
        } else {
            // No type annotation - use type inference placeholder
            // Rust will infer the type from the assignments in the branches
            hoisted_decls.push(quote! { let mut #var_ident; });

            // DEPYLER-0455 Bug 2: Track hoisted variables needing String normalization
            // When a variable is hoisted without type annotation, we need to normalize
            // string literals to String to avoid &str vs String type mismatches
            ctx.hoisted_inference_vars.insert(var_name.clone());
        }

        // Mark variable as declared so assignments use `var = value` not `let var = value`
        ctx.declare_var(var_name);
    }

    ctx.enter_scope();
    let then_stmts: Vec<_> = then_body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    let result = if let Some(else_stmts) = else_body {
        ctx.enter_scope();
        let else_tokens: Vec<_> = else_stmts
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();
        Ok(quote! {
            #(#hoisted_decls)*
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
    };

    // DEPYLER-0455 Bug 2: Clean up hoisted inference vars after if-statement
    // Remove variables from tracking set since they're only relevant within this if-statement
    for var_name in &hoisted_vars {
        ctx.hoisted_inference_vars.remove(var_name);
    }

    result
}

/// DEPYLER-0379: Find the type annotation for a variable in a statement block
///
/// Searches for the first Assign statement that assigns to the given variable
/// and returns its type annotation if present.
///
/// # Complexity
/// 3 (linear search with recursive check)
fn find_variable_type(var_name: &str, stmts: &[HirStmt]) -> Option<Type> {
    for stmt in stmts {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                type_annotation,
                ..
            } if name == var_name => {
                return type_annotation.clone();
            }
            _ => {}
        }
    }
    None
}

/// Check if a variable is used in an expression
fn is_var_used_in_expr(var_name: &str, expr: &HirExpr) -> bool {
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
        HirExpr::FString { parts } => parts.iter().any(|part| match part {
            crate::hir::FStringPart::Expr(expr) => is_var_used_in_expr(var_name, expr),
            crate::hir::FStringPart::Literal(_) => false,
        }),
        // DEPYLER-0569: Handle generator expressions and comprehensions
        // These can reference loop variables in their iterable or element expressions
        HirExpr::GeneratorExp {
            element,
            generators,
        }
        | HirExpr::ListComp {
            element,
            generators,
        }
        | HirExpr::SetComp {
            element,
            generators,
        } => {
            is_var_used_in_expr(var_name, element)
                || generators.iter().any(|gen| {
                    is_var_used_in_expr(var_name, &gen.iter)
                        || gen
                            .conditions
                            .iter()
                            .any(|cond| is_var_used_in_expr(var_name, cond))
                })
        }
        HirExpr::DictComp {
            key,
            value,
            generators,
        } => {
            is_var_used_in_expr(var_name, key)
                || is_var_used_in_expr(var_name, value)
                || generators.iter().any(|gen| {
                    is_var_used_in_expr(var_name, &gen.iter)
                        || gen
                            .conditions
                            .iter()
                            .any(|cond| is_var_used_in_expr(var_name, cond))
                })
        }
        // DEPYLER-0619: Handle await expressions for variable usage detection
        // Without this, loop variables used in async function calls inside await
        // are incorrectly marked as unused and prefixed with underscore
        HirExpr::Await { value } => is_var_used_in_expr(var_name, value),
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
        // DEPYLER-0593: Handle Try statements for variable usage detection
        // Without this, loop variables used inside try/except are incorrectly marked unused
        HirStmt::Try {
            body,
            handlers,
            orelse,
            finalbody,
        } => {
            body.iter().any(|s| is_var_used_in_stmt(var_name, s))
                || handlers
                    .iter()
                    .any(|h| h.body.iter().any(|s| is_var_used_in_stmt(var_name, s)))
                || orelse
                    .as_ref()
                    .is_some_and(|stmts| stmts.iter().any(|s| is_var_used_in_stmt(var_name, s)))
                || finalbody
                    .as_ref()
                    .is_some_and(|stmts| stmts.iter().any(|s| is_var_used_in_stmt(var_name, s)))
        }
        // DEPYLER-0593: Handle With statements for variable usage detection
        HirStmt::With { context, body, .. } => {
            is_var_used_in_expr(var_name, context)
                || body.iter().any(|s| is_var_used_in_stmt(var_name, s))
        }
        _ => false,
    }
}

/// DEPYLER-0607: Check if a method chain leads back to a dict.get() on HashMap<_, serde_json::Value>
/// This handles patterns like: data.get("key").cloned().unwrap_or_default()
/// Includes fallback for untracked local variables when serde_json is in use.
fn is_json_value_method_chain_or_fallback(expr: &HirExpr, ctx: &CodeGenContext) -> bool {
    match expr {
        // Reached the base: check if it's a dict.get() on Value-containing HashMap
        HirExpr::MethodCall { object, method, .. } if method == "get" => {
            if let HirExpr::Var(var_name) = object.as_ref() {
                if let Some(t) = ctx.var_types.get(var_name) {
                    // DEPYLER-0607: A dict with Unknown value type maps to HashMap<_, serde_json::Value>
                    // So we should treat Unknown as potentially Value
                    matches!(t, Type::Dict(_, v) if matches!(v.as_ref(),
                        Type::Custom(n) if n.contains("Value") || n.contains("json"))
                        || matches!(v.as_ref(), Type::Unknown))
                } else {
                    // DEPYLER-0607: Fallback for untracked local dicts
                    ctx.needs_serde_json
                }
            } else {
                false
            }
        }
        // Continue traversing the chain
        HirExpr::MethodCall { object, method, .. } => {
            let is_chain_method = method == "cloned"
                || method == "unwrap_or_default"
                || method == "unwrap_or"
                || method == "unwrap";
            if is_chain_method {
                is_json_value_method_chain_or_fallback(object.as_ref(), ctx)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// DEPYLER-0607: Check if a dict has Value type values (explicit Value or Unknown)
fn is_dict_with_value_type(t: &Type) -> bool {
    match t {
        Type::Dict(_, v) => {
            matches!(v.as_ref(),
                Type::Custom(n) if n.contains("Value") || n.contains("json"))
            || matches!(v.as_ref(), Type::Unknown)
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
    // CITL: Trace for loop iteration strategy
    trace_decision!(
        category = DecisionCategory::BorrowStrategy,
        name = "for_loop_iter",
        chosen = "for_in_iter",
        alternatives = ["iter", "into_iter", "iter_mut", "drain", "range"],
        confidence = 0.88
    );

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
                    _ => safe_ident("_nested"), // Nested tuple unpacking - use placeholder
                })
                .collect();
            parse_quote! { (#(#idents),*) }
        }
        _ => bail!("Unsupported for loop target type"),
    };

    let mut iter_expr = iter.to_rust_expr(ctx)?;

    // DEPYLER-0388: Handle sys.stdin iteration
    // Python: for line in sys.stdin:
    // Rust: for line in std::io::stdin().lock().lines()
    let is_stdin_iter = matches!(iter, HirExpr::Attribute { value, attr }
        if matches!(&**value, HirExpr::Var(m) if m == "sys") && attr == "stdin");

    // DEPYLER-0388: Handle File object iteration from open()
    // Python: for line in f: (where f = open(...))
    // Rust: use BufReader for efficient line-by-line reading
    // Check if this variable might be a File object
    // Heuristic: variables named 'f', 'file', 'input', 'output', or ending in '_file'
    let is_file_iter = if let HirExpr::Var(var_name) = iter {
        var_name == "f"
            || var_name == "file"
            || var_name == "input"
            || var_name == "output"
            || var_name.ends_with("_file")
            || var_name.starts_with("file_")
    } else {
        false
    };

    if is_stdin_iter {
        // Wrap stdin with .lines() to get line iterator
        // Stdin::lines() method provides buffered line-by-line reading
        // Returns Iterator<Item = Result<String, io::Error>>
        // We map to unwrap_or_default() to handle errors gracefully
        iter_expr = parse_quote! { #iter_expr.lines().map(|l| l.unwrap_or_default()) };
    } else if is_file_iter {
        // DEPYLER-0452 Phase 3: Use BufReader::new(f).lines() for File iteration
        // This is the idiomatic Rust way to iterate over file lines
        // Method call syntax (.lines()) is preferred over trait syntax (BufRead::lines())
        // DEPYLER-0522: .lines() requires BufRead trait to be in scope
        ctx.needs_bufread = true;
        iter_expr = parse_quote! {
            std::io::BufReader::new(#iter_expr).lines()
                .map(|l| l.unwrap_or_default())
        };
    }

    // DEPYLER-0452: Handle CSV Reader iteration
    // Check if variable name suggests CSV reader (heuristic-based)
    let is_csv_reader = if let HirExpr::Var(var_name) = iter {
        var_name == "reader"
            || var_name.contains("csv")
            || var_name.ends_with("_reader")
            || var_name.starts_with("reader_")
    } else {
        false
    };

    // Track if CSV pattern yields Results (need to unwrap in loop)
    let mut csv_yields_results = false;

    if !is_stdin_iter && !is_file_iter && is_csv_reader {
        // Try to apply CSV iteration mapping from stdlib_mappings
        // This transforms: for row in reader
        // Into: for result in reader.deserialize::<HashMap<String, String>>()
        if let Some(pattern) = ctx
            .stdlib_mappings
            .get_iteration_pattern("csv", "DictReader")
        {
            // Check if pattern yields Results
            if let crate::stdlib_mappings::RustPattern::IterationPattern {
                yields_results, ..
            } = pattern
            {
                csv_yields_results = *yields_results;
            }

            let rust_code =
                pattern.generate_rust_code(&iter_expr.to_token_stream().to_string(), &[]);
            if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
                // Set needs_csv flag
                ctx.needs_csv = true;
                // Wrap in iteration that handles Results
                iter_expr = expr;
            }
        }
    }

    // Check if we're iterating over a borrowed collection
    // If iter is a simple variable that refers to a borrowed collection (e.g., &Vec<T>),
    // we need to add .iter() to properly iterate over it
    // Skip this for stdin/file/csv iterators which are already properly wrapped
    if !is_stdin_iter && !is_file_iter && !is_csv_reader {
        if let HirExpr::Var(var_name) = iter {
            // DEPYLER-0419: First check type information from context
            // This is more reliable than name heuristics
            let is_string_type = ctx
                .var_types
                .get(var_name)
                .is_some_and(|t| matches!(t, Type::String));

            // DEPYLER-0300/0302: Fall back to name-based heuristics if type not available
            // Strings use .chars() instead of .iter().cloned()
            // DEPYLER-0302: Exclude plurals (strings, words, etc.) which are collections
            let is_string_name = {
                let n = var_name.as_str();
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
            };

            // DEPYLER-0606: Check if iterating over serde_json::Value
            // JSON values are represented as Custom("Value") or dict values from heterogeneous dicts
            let is_json_value = ctx.var_types.get(var_name).is_some_and(|t| {
                matches!(t, Type::Custom(name) if name == "Value" || name == "serde_json::Value" || name.contains("json"))
            });

            if is_string_type || is_string_name {
                // For strings, use .chars() to iterate over characters
                iter_expr = parse_quote! { #iter_expr.chars() };
            } else if is_json_value {
                // DEPYLER-0606: serde_json::Value needs .as_array().unwrap() before iteration
                // This handles: for item in json_value (where json_value is a JSON array)
                iter_expr = parse_quote! { #iter_expr.as_array().unwrap().iter().cloned() };
            } else if ctx.iterator_vars.contains(var_name) {
                // DEPYLER-0520: Variable is already an iterator (from .filter().map() etc.)
                // Don't add .iter().cloned() - iterators don't have .iter() method
                // Just iterate directly
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
    }

    // DEPYLER-0607: Handle JSON Value iteration for dict index and method chains
    // Patterns: data["items"], data.get("items").cloned().unwrap_or_default()
    // serde_json::Value doesn't implement IntoIterator, need .as_array().unwrap().iter()
    if !is_stdin_iter && !is_file_iter && !is_csv_reader {
        // First check HIR-level patterns
        let is_json_value_iteration = match iter {
            // Case 1: dict["key"] - Index into HashMap<String, serde_json::Value>
            HirExpr::Index { base, .. } => {
                match base.as_ref() {
                    HirExpr::Var(var_name) => {
                        // Check if base is a HashMap with Value values (including Unknown → Value)
                        if let Some(t) = ctx.var_types.get(var_name) {
                            is_dict_with_value_type(t)
                        } else {
                            // DEPYLER-0607: For untracked local variables, check if serde_json is in use
                            // If so, dict literals with heterogeneous values use serde_json::Value
                            // This handles: data = {"key": [1,2,3]}; for item in data["key"]
                            ctx.needs_serde_json
                        }
                    }
                    HirExpr::Dict { .. } => true, // Dict literal - definitely has Value type
                    _ => false,
                }
            }
            // Case 2: dict.get("key")... method chain returning Value
            HirExpr::MethodCall { object, method, .. } => {
                // Check for common patterns: .cloned(), .unwrap_or_default(), .unwrap_or()
                // that might be chained after dict.get()
                let is_value_chain = method == "cloned"
                    || method == "unwrap_or_default"
                    || method == "unwrap_or"
                    || method == "unwrap";

                if is_value_chain {
                    // Check if there's a get() call somewhere in the chain on a dict with Value
                    is_json_value_method_chain_or_fallback(object.as_ref(), ctx)
                } else if method == "get" {
                    // Direct dict.get() call
                    if let HirExpr::Var(var_name) = object.as_ref() {
                        if let Some(t) = ctx.var_types.get(var_name) {
                            is_dict_with_value_type(t)
                        } else {
                            // DEPYLER-0607: Fallback for untracked local dicts
                            ctx.needs_serde_json
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        };

        // DEPYLER-0607: Also detect from generated Rust code patterns
        // This catches cases where dict index was converted to .get().cloned().unwrap_or_default()
        let iter_expr_str = quote!(#iter_expr).to_string();
        let has_value_pattern = iter_expr_str.contains("unwrap_or_default")
            || iter_expr_str.contains("unwrap_or (")
            || (iter_expr_str.contains(".get") && iter_expr_str.contains(".cloned"));

        if is_json_value_iteration || (has_value_pattern && ctx.needs_serde_json) {
            // DEPYLER-0607: Wrap JSON Value with .as_array().unwrap_or(&vec![]).iter().cloned()
            iter_expr = parse_quote! {
                #iter_expr.as_array().unwrap_or(&vec![]).iter().cloned()
            };
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
        HirExpr::Call { func, args, .. } if func == "enumerate" => {
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
        (AssignTarget::Tuple(targets), Some(Type::Tuple(elem_types)))
            if targets.len() == elem_types.len() =>
        {
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
    } else if csv_yields_results {
        // DEPYLER-0452: Unwrap Results from CSV deserialize iteration
        // Python: for row in reader
        // Rust: for result in reader.deserialize() { let row = result?; ... }
        if let AssignTarget::Symbol(var_name) = target {
            let var_ident = safe_ident(var_name); // DEPYLER-0023
            let result_ident = safe_ident("result"); // DEPYLER-0023
            Ok(quote! {
                for #result_ident in #iter_expr {
                    let #var_ident = #result_ident?;
                    #(#body_stmts)*
                }
            })
        } else {
            // Fallback if target is not a simple symbol
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
    // CITL: Trace assignment target strategy
    trace_decision!(
        category = DecisionCategory::Ownership,
        name = "assign_target",
        chosen = "let_binding",
        alternatives = ["let_mut", "reassign", "destructure", "augmented"],
        confidence = 0.90
    );

    // DEPYLER-0399: Transform CSE assignments for subcommand comparisons
    // DEPYLER-0456 Bug #2: Use dest_field instead of hardcoded "command"
    // When we have subcommands, assignments like `_cse_temp_0 = args.action == "clone"`
    // would try to compare Commands enum to string (won't compile).
    // Transform into a match expression that returns bool:
    // let _cse_temp_0 = matches!(args.action, Commands::Clone { .. });
    if ctx.argparser_tracker.has_subcommands() {
        // Get dest_field from subparser info
        let dest_field = ctx
            .argparser_tracker
            .subparsers
            .values()
            .next()
            .map(|sp| sp.dest_field.clone())
            .unwrap_or_else(|| "command".to_string());

        if let Some(cmd_name) = is_subcommand_check(value, &dest_field, ctx) {
            if let AssignTarget::Symbol(cse_var) = target {
                use quote::{format_ident, quote};
                let variant_name = format_ident!("{}", to_pascal_case_subcommand(&cmd_name));
                let var_ident = safe_ident(cse_var);

                // DEPYLER-0456 Bug #2: Track this CSE temp so is_subcommand_check() can find it
                ctx.cse_subcommand_temps
                    .insert(cse_var.clone(), cmd_name.clone());

                // DEPYLER-0456 Bug #3 FIX: Always use "command" as Rust field name
                return Ok(quote! {
                    let #var_ident = matches!(args.command, Commands::#variant_name { .. });
                });
            }
        }
    }

    // DEPYLER-0497: Track variables assigned from Option-returning functions
    // When a variable is assigned from a function that returns Option<T>, we need to record
    // its type in var_types so format! can detect it needs unwrapping.
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::Call { func, .. } = value {
            if ctx.option_returning_functions.contains(func) {
                // Variable is assigned from Option-returning function
                // Get the function's return type and add to var_types
                if let Some(ret_type) = ctx.function_return_types.get(func) {
                    ctx.var_types.insert(var_name.clone(), ret_type.clone());
                }
            }
        }
    }

    // DEPYLER-0520: Track variables assigned from iterator-producing expressions
    // Generator expressions and method chains ending in filter/map/etc produce iterators,
    // not collections. These variables should NOT have .iter().cloned() added in for loops.
    if let AssignTarget::Symbol(var_name) = target {
        if is_iterator_producing_expr(value) {
            ctx.iterator_vars.insert(var_name.clone());
        }
    }

    // DEPYLER-0440: Skip None-placeholder assignments
    // When a variable is initialized with None and later reassigned in if-elif-else,
    // skip the initial None assignment to avoid Option<T> type mismatch.
    // The hoisting logic (DEPYLER-0439) will handle the declaration with correct type.
    if let AssignTarget::Symbol(var_name) = target {
        if matches!(value, HirExpr::Literal(Literal::None)) && ctx.mutable_vars.contains(var_name) {
            // This is a None placeholder that will be reassigned - skip it
            return Ok(quote! {});
        }
    }

    // DEPYLER-0363: Detect ArgumentParser patterns for clap transformation
    // Pattern 1: parser = argparse.ArgumentParser(...) [MethodCall with object=argparse]
    // Pattern 2: args = parser.parse_args() [MethodCall with object=parser]
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::MethodCall {
            method,
            object,
            args,
            kwargs,
            ..
        } = value
        {
            // Pattern 1: ArgumentParser constructor
            if method == "ArgumentParser" {
                if let HirExpr::Var(module_name) = object.as_ref() {
                    if module_name == "argparse" {
                        // Register this as an ArgumentParser instance
                        let mut info = crate::rust_gen::argparse_transform::ArgParserInfo::new(
                            var_name.clone(),
                        );

                        // Extract description and epilog from kwargs
                        for (key, value_expr) in kwargs {
                            if key == "description" {
                                if let HirExpr::Literal(crate::hir::Literal::String(s)) = value_expr
                                {
                                    info.description = Some(s.clone());
                                }
                            } else if key == "epilog" {
                                if let HirExpr::Literal(crate::hir::Literal::String(s)) = value_expr
                                {
                                    info.epilog = Some(s.clone());
                                }
                            }
                        }

                        ctx.argparser_tracker
                            .register_parser(var_name.clone(), info);

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

            // DEPYLER-0394/0396: Skip assignments to parser configuration method results
            // Pattern: group = parser.add_argument_group(...)
            //      OR: nested_group = group.add_mutually_exclusive_group(...)
            // These methods aren't needed with clap derive - skip the assignment
            if matches!(
                method.as_str(),
                "add_argument_group" | "add_mutually_exclusive_group" | "set_defaults"
            ) {
                if let HirExpr::Var(parent_var) = object.as_ref() {
                    // Check if parent_var is a parser OR a group
                    let is_parser_or_group = ctx.argparser_tracker.get_parser(parent_var).is_some()
                        || ctx
                            .argparser_tracker
                            .get_parser_for_group(parent_var)
                            .is_some();

                    if is_parser_or_group {
                        // DEPYLER-0396: Register the group variable so we can track
                        // add_argument() calls on it later (e.g., input_group.add_argument())
                        // This handles both:
                        //   - group = parser.add_argument_group() → register group → parser
                        //   - nested = group.add_mutually_exclusive_group() → register nested → group
                        // Recursive resolution will handle nested → group → parser chain
                        if let AssignTarget::Symbol(group_var) = target {
                            ctx.argparser_tracker
                                .register_group(group_var.clone(), parent_var.clone());
                        }
                        // Skip this assignment - not needed with clap
                        return Ok(quote! {});
                    }
                }
            }

            // DEPYLER-0399: Detect subparsers = parser.add_subparsers(dest="command", required=True)
            if method == "add_subparsers" {
                if let HirExpr::Var(parser_var) = object.as_ref() {
                    if ctx.argparser_tracker.get_parser(parser_var).is_some() {
                        // Extract dest and required from kwargs
                        let dest_field = extract_kwarg_string(kwargs, "dest")
                            .unwrap_or_else(|| "command".to_string());
                        let required = extract_kwarg_bool(kwargs, "required").unwrap_or(false);
                        let help = extract_kwarg_string(kwargs, "help");

                        if let AssignTarget::Symbol(subparsers_var) = target {
                            use crate::rust_gen::argparse_transform::SubparserInfo;
                            ctx.argparser_tracker.register_subparsers(
                                subparsers_var.clone(),
                                SubparserInfo {
                                    parser_var: parser_var.clone(),
                                    dest_field,
                                    required,
                                    help,
                                },
                            );
                        }
                        // Skip this assignment - not needed with clap
                        return Ok(quote! {});
                    }
                }
            }

            // DEPYLER-0399: Detect parser_clone = subparsers.add_parser("clone", help="...")
            if method == "add_parser" {
                if let HirExpr::Var(subparsers_var) = object.as_ref() {
                    if ctx
                        .argparser_tracker
                        .get_subparsers(subparsers_var)
                        .is_some()
                    {
                        // Extract command name from first positional arg
                        if !args.is_empty() {
                            let command_name = extract_string_literal(&args[0]);
                            let help = extract_kwarg_string(kwargs, "help");

                            if let AssignTarget::Symbol(subcommand_var) = target {
                                use crate::rust_gen::argparse_transform::SubcommandInfo;
                                ctx.argparser_tracker.register_subcommand(
                                    subcommand_var.clone(),
                                    SubcommandInfo {
                                        name: command_name,
                                        help,
                                        arguments: vec![],
                                        subparsers_var: subparsers_var.clone(),
                                    },
                                );
                            }
                        }
                        // Skip this assignment - not needed with clap
                        return Ok(quote! {});
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

        // DEPYLER-0479: Track String type from os.environ.get(key, default) with default value
        // Example: value = os.environ.get(var, "default")
        //       → value = std::env::var(var).unwrap_or_else(|_| "default".to_string())
        // This should track as String, NOT Option<String>
        if let HirExpr::MethodCall {
            object,
            method,
            args,
            kwargs: _,
        } = value
        {
            // Check for os.environ.get(key, default) - 2 arguments means default provided
            if method == "get" && args.len() == 2 {
                if let HirExpr::Attribute {
                    value: attr_obj,
                    attr,
                } = object.as_ref()
                {
                    if let HirExpr::Var(module) = attr_obj.as_ref() {
                        if module == "os" && attr == "environ" {
                            // os.environ.get(key, default) returns String (not Option)
                            ctx.var_types.insert(var_name.clone(), Type::String);
                        }
                    }
                }
            }
            // Also check for os.getenv(key, default)
            else if method == "getenv" && args.len() == 2 {
                if let HirExpr::Var(module) = object.as_ref() {
                    if module == "os" {
                        ctx.var_types.insert(var_name.clone(), Type::String);
                    }
                }
            }
        }

        // DEPYLER-0455: Track Option types from method calls like .ok() and .get()
        // This enables proper truthiness conversion (if option → if option.is_some())
        // Example: config_file = os.environ.get("CONFIG_FILE")
        //          or: config_file = std::env::var(...).ok()
        // DEPYLER-0479: Skip if already tracked (e.g., unwrap_or_else handled above)
        if !ctx.var_types.contains_key(var_name) && looks_like_option_expr(value) {
            // Track as Option<String> for now (generic placeholder)
            // The exact inner type doesn't matter for truthiness conversion
            ctx.var_types
                .insert(var_name.clone(), Type::Optional(Box::new(Type::String)));
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
                // DEPYLER-0431: Track re.search(), re.match(), re.find() module functions
                // These all return Option<Match> in Rust
                else if matches!(func.as_str(), "search" | "match" | "find") {
                    // Only track if this looks like a regex call (needs more context to be sure)
                    // For now, track any call to search/match/find as Optional
                    // This is a heuristic - could be improved with module tracking
                    ctx.var_types
                        .insert(var_name.clone(), Type::Optional(Box::new(Type::Unknown)));
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
                // DEPYLER-0560: Check function return type for Dict[str, Any] pattern
                let (key_type, val_type) = if let Some(Type::Dict(k, v)) = type_annotation {
                    (k.as_ref().clone(), v.as_ref().clone())
                } else if let Some(Type::Dict(k, v)) = &ctx.current_return_type {
                    // Use return type's dict value type (handles Dict[str, Any] → Unknown)
                    (k.as_ref().clone(), v.as_ref().clone())
                } else if !items.is_empty() {
                    // Infer from first item (assume homogeneous dict)
                    // For string literal keys and int values
                    (Type::String, Type::Int)
                } else {
                    (Type::Unknown, Type::Unknown)
                };
                ctx.var_types.insert(
                    var_name.clone(),
                    Type::Dict(Box::new(key_type), Box::new(val_type)),
                );
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
            // DEPYLER-0600 Bug #6: Track type from comprehension expressions
            // Enables correct {:?} vs {} selection in println! for dict/list/set comprehensions
            HirExpr::DictComp { key, value, .. } => {
                // Use type inference from func_gen module for comprehension types
                let key_type = crate::rust_gen::func_gen::infer_expr_type_simple(key);
                let val_type = crate::rust_gen::func_gen::infer_expr_type_simple(value);
                ctx.var_types.insert(
                    var_name.clone(),
                    Type::Dict(Box::new(key_type), Box::new(val_type)),
                );
            }
            HirExpr::ListComp { element, .. } => {
                let elem_type = crate::rust_gen::func_gen::infer_expr_type_simple(element);
                ctx.var_types
                    .insert(var_name.clone(), Type::List(Box::new(elem_type)));
            }
            HirExpr::SetComp { element, .. } => {
                let elem_type = crate::rust_gen::func_gen::infer_expr_type_simple(element);
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
                            ctx.var_types
                                .insert(var_name.clone(), elem_type.as_ref().clone());
                        }
                    }
                }
                // DEPYLER-0421: String methods that return Vec<String> (for truthiness)
                // Track .split() and .split_whitespace() as List(String) for truthiness conversion
                else if matches!(method.as_str(), "split" | "split_whitespace" | "splitlines") {
                    ctx.var_types
                        .insert(var_name.clone(), Type::List(Box::new(Type::String)));
                }
                // String methods that return String
                else if matches!(
                    method.as_str(),
                    "upper"
                        | "lower"
                        | "strip"
                        | "lstrip"
                        | "rstrip"
                        | "title"
                        | "replace"
                        | "format"
                ) {
                    ctx.var_types.insert(var_name.clone(), Type::String);
                }
                // DEPYLER-0431: Regex methods that return Option<Match>
                // Track .find(), .search(), .match() as Optional for truthiness conversion
                else if matches!(method.as_str(), "find" | "search" | "match") {
                    // Check if this is a regex method call (on compiled regex object)
                    // We don't have a specific regex type, so use Optional as a marker
                    ctx.var_types
                        .insert(var_name.clone(), Type::Optional(Box::new(Type::Unknown)));
                }
            }
            _ => {}
        }
    }

    // DEPYLER-0472: Set json context when assigning to serde_json::Value dicts
    // This ensures dict literals use json!({}) instead of HashMap::new()
    let prev_json_context = ctx.in_json_context;
    if let AssignTarget::Index { base, .. } = target {
        // Check if base variable suggests serde_json::Value type
        if let HirExpr::Var(base_name) = base.as_ref() {
            let name_str = base_name.as_str();
            // Variables commonly used with serde_json::Value
            if name_str == "config"
                || name_str == "data"
                || name_str == "value"
                || name_str == "current"
                || name_str == "obj"
                || name_str == "json"
            {
                ctx.in_json_context = true;
            }
        }
    }

    let mut value_expr = value.to_rust_expr(ctx)?;

    // DEPYLER-0472: Restore previous json context
    ctx.in_json_context = prev_json_context;

    // DEPYLER-0270: Auto-unwrap Result-returning function calls in assignments
    // When assigning from a function that returns Result<T, E> in a non-Result context,
    // we need to unwrap it.
    //
    // DEPYLER-0422 Fix #8: Also add `?` when BOTH caller and callee return Result
    // Fix #6 removed automatic `?` from expr_gen.rs, so we need to add it here at the
    // statement level where we know the variable type context.
    //
    // Five-Whys Root Cause:
    // 1. Why: expected `i32`, found `Result<i32, Box<dyn Error>>`
    // 2. Why: Variable `position: i32` assigned Result-returning function without unwrap
    // 3. Why: Neither `?` nor `.unwrap()` added to function call
    // 4. Why: Fix #6 removed `?` from expr_gen, and DEPYLER-0270 only adds `.unwrap()` for non-Result callers
    // 5. ROOT CAUSE: Missing `?` for Result→Result propagation after Fix #6
    if let HirExpr::Call { func, .. } = value {
        if ctx.result_returning_functions.contains(func) {
            if ctx.current_function_can_fail {
                // Current function also returns Result - add ? to propagate error
                value_expr = parse_quote! { #value_expr? };
            } else {
                // Current function doesn't return Result - add .unwrap() to extract the value
                value_expr = parse_quote! { #value_expr.unwrap() };
            }
        }
    }

    // If there's a type annotation, handle type conversions
    let (type_annotation_tokens, is_final) = if let Some(target_type) = type_annotation {
        // Check if this is a Final type annotation
        let (actual_type, is_const) = match target_type {
            Type::Final(inner) => (inner.as_ref(), true),
            _ => (target_type, false),
        };

        let target_rust_type = ctx.type_mapper.map_type(actual_type);
        let target_syn_type = rust_type_to_syn(&target_rust_type)?;

        // DEPYLER-0272: Check if we need type conversion (e.g., usize to i32)
        // DEPYLER-0455 Bug 7: Also pass ctx for validator function detection
        // Pass the value expression to determine if cast is actually needed
        if needs_type_conversion(actual_type, value, ctx) {
            value_expr = apply_type_conversion(value_expr, actual_type);
        }

        // DEPYLER-0380 Bug #1: String literal to String conversion
        // When assigning a string literal to a String typed variable, add .to_string()
        // Example: `let version: String = "Python 3.x"` should become
        //          `let version: String = "Python 3.x".to_string()`
        if matches!(value, HirExpr::Literal(Literal::String(_)))
            && matches!(target_rust_type, crate::type_mapper::RustType::String)
        {
            value_expr = parse_quote! { #value_expr.to_string() };
        }

        (Some(quote! { : #target_syn_type }), is_const)
    } else {
        (None, false)
    };

    // DEPYLER-0455 Bug 2: String literal normalization for hoisted inference variables
    // When a variable is hoisted without type annotation, string literals must be
    // normalized to String to ensure consistent type inference across if/else branches
    // Example: let mut format;
    //          if x { format = "json"; }  // &str
    //          else { format = s.to_lowercase(); }  // String - TYPE MISMATCH!
    // Solution: Convert all string literals to String: format = "json".to_string();
    if let AssignTarget::Symbol(var_name) = target {
        if ctx.hoisted_inference_vars.contains(var_name)
            && matches!(value, HirExpr::Literal(Literal::String(_)))
        {
            value_expr = parse_quote! { #value_expr.to_string() };
        }
    }

    // DEPYLER-0598: String literal normalization for mutable variables
    // When a mutable variable is first assigned a string literal (no type annotation),
    // convert to .to_string() to avoid &str vs String type mismatch on reassignment
    // Example: let mut result = "hello";  // &str
    //          result = format!(...);     // String - TYPE MISMATCH!
    // Solution: let mut result = "hello".to_string();
    if let AssignTarget::Symbol(var_name) = target {
        let is_first_assignment = !ctx.is_declared(var_name);
        let is_mutable = ctx.mutable_vars.contains(var_name);
        let no_type_annotation = type_annotation.is_none();
        let is_string_literal = matches!(value, HirExpr::Literal(Literal::String(_)));

        if is_first_assignment && is_mutable && no_type_annotation && is_string_literal {
            value_expr = parse_quote! { #value_expr.to_string() };
        }
    }

    match target {
        AssignTarget::Symbol(symbol) => {
            codegen_assign_symbol(symbol, value_expr, type_annotation_tokens, is_final, ctx)
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
    is_final: bool,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0023: Use safe_ident to escape Rust keywords (match, type, impl, etc.)
    let target_ident = safe_ident(symbol);

    // Inside generators, check if variable is a state variable
    if ctx.in_generator && ctx.generator_state_vars.contains(symbol) {
        // State variable assignment: self.field = value
        Ok(quote! { self.#target_ident = #value_expr; })
    } else if is_final {
        // Final type annotation - generate const instead of let
        if let Some(type_ann) = type_annotation_tokens {
            Ok(quote! { const #target_ident #type_ann = #value_expr; })
        } else {
            // Final without explicit type annotation - shouldn't happen, but handle gracefully
            Ok(quote! { const #target_ident = #value_expr; })
        }
    } else if ctx.is_declared(symbol) {
        // Variable already exists, just assign
        // DEPYLER-0604: Check if variable has Optional type and wrap value in Some()
        let final_value = if let Some(Type::Optional(inner_type)) = ctx.var_types.get(symbol) {
            // Check if the value is already wrapped in Some or is None
            let value_str = quote!(#value_expr).to_string();
            if value_str.starts_with("Some") || value_str == "None" {
                value_expr
            } else {
                // Wrap non-Optional value in Some()
                // DEPYLER-0604: Handle string conversion for Optional<String>
                if matches!(inner_type.as_ref(), Type::String) {
                    // Check if it's a string literal that needs .to_string()
                    let value_str = quote!(#value_expr).to_string();
                    if value_str.starts_with('"') && value_str.ends_with('"') {
                        parse_quote! { Some(#value_expr.to_string()) }
                    } else {
                        parse_quote! { Some(#value_expr) }
                    }
                } else {
                    parse_quote! { Some(#value_expr) }
                }
            }
        } else {
            value_expr
        };
        Ok(quote! { #target_ident = #final_value; })
    } else {
        // First declaration - check if variable needs mut
        ctx.declare_var(symbol);
        if ctx.mutable_vars.contains(symbol) {
            // DEPYLER-0464: When initializing from a borrowed dict/json parameter
            // that will be reassigned with .cloned() later, clone it to create an owned value
            // Pattern: `let mut value = config` where config is a parameter
            let needs_clone = if let syn::Expr::Path(ref path) = value_expr {
                // Check if this is a simple path (single identifier)
                if path.path.segments.len() == 1 {
                    let ident = &path.path.segments[0].ident;
                    let var_name = ident.to_string();
                    // Check if:
                    // 1. Source is already declared (it's a parameter)
                    // 2. Source name != target name (assigning to a new variable)
                    // This is the pattern: `let mut value = param` which will later be reassigned
                    ctx.is_declared(&var_name) && var_name != symbol
                } else {
                    false
                }
            } else {
                false
            };

            let init_expr = if needs_clone {
                parse_quote! { #value_expr.clone() }
            } else {
                value_expr
            };

            if let Some(type_ann) = type_annotation_tokens {
                Ok(quote! { let mut #target_ident #type_ann = #init_expr; })
            } else {
                Ok(quote! { let mut #target_ident = #init_expr; })
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
                Type::List(_) => true,     // List/Vec → numeric index
                Type::Dict(_, _) => false, // Dict/HashMap → key (not numeric)
                _ => {
                    // Fall back to index heuristic for other types
                    // DEPYLER-0449: Check if index looks like a string key before assuming numeric
                    match index {
                        HirExpr::Var(name) => {
                            let name_str = name.as_str();
                            // String-like variable names → NOT numeric
                            if name_str == "key"
                                || name_str == "k"
                                || name_str == "name"
                                || name_str == "id"
                                || name_str == "word"
                                || name_str == "text"
                                || name_str == "char"
                                || name_str == "character"
                                || name_str == "c"
                                || name_str.ends_with("_key")
                                || name_str.ends_with("_name")
                            {
                                false
                            } else {
                                // Default: assume numeric for other variables
                                true
                            }
                        }
                        HirExpr::Binary { .. } | HirExpr::Literal(crate::hir::Literal::Int(_)) => {
                            true
                        }
                        _ => false,
                    }
                }
            }
        } else {
            // No type info - use heuristic
            // DEPYLER-0449: Check if index looks like a string key before assuming numeric
            match index {
                HirExpr::Var(name) => {
                    let name_str = name.as_str();
                    // String-like variable names → NOT numeric
                    if name_str == "key"
                        || name_str == "k"
                        || name_str == "name"
                        || name_str == "id"
                        || name_str == "word"
                        || name_str == "text"
                        || name_str == "char"
                        || name_str == "character"
                        || name_str == "c"
                        || name_str.ends_with("_key")
                        || name_str.ends_with("_name")
                    {
                        false
                    } else {
                        // Default: assume numeric for other variables
                        true
                    }
                }
                HirExpr::Binary { .. } | HirExpr::Literal(crate::hir::Literal::Int(_)) => true,
                _ => false,
            }
        }
    } else {
        // Base is not a simple variable - use heuristic
        // DEPYLER-0449: Check if index looks like a string key before assuming numeric
        match index {
            HirExpr::Var(name) => {
                let name_str = name.as_str();
                // String-like variable names → NOT numeric
                if name_str == "key"
                    || name_str == "k"
                    || name_str == "name"
                    || name_str == "id"
                    || name_str == "word"
                    || name_str == "text"
                    || name_str == "char"
                    || name_str == "character"
                    || name_str == "c"
                    || name_str.ends_with("_key")
                    || name_str.ends_with("_name")
                {
                    false
                } else {
                    // Default: assume numeric for other variables
                    true
                }
            }
            HirExpr::Binary { .. } | HirExpr::Literal(crate::hir::Literal::Int(_)) => true,
            _ => false,
        }
    };

    // Extract the base and all intermediate indices
    let (base_expr, indices) = extract_nested_indices_tokens(base, ctx)?;

    // DEPYLER-0403: Convert string literals to String for Dict<String, String> values
    // Check if value_expr is a string literal and the dict value type is String
    let value_expr = if !is_numeric_index {
        // Get the base variable name to look up its type
        let base_name = match base {
            HirExpr::Var(name) => Some(name.as_str()),
            HirExpr::Index {
                base: inner_base, ..
            } => {
                // For nested subscripts, get the root variable
                fn get_root_var(expr: &HirExpr) -> Option<&str> {
                    match expr {
                        HirExpr::Var(name) => Some(name.as_str()),
                        HirExpr::Index { base, .. } => get_root_var(base),
                        _ => None,
                    }
                }
                get_root_var(inner_base)
            }
            _ => None,
        };

        // Check if we need to convert string literal to String
        let needs_string_conversion = if let Some(name) = base_name {
            if let Some(base_type) = ctx.var_types.get(name) {
                // Navigate through nested Dict types to find the innermost value type
                let depth = indices.len() + 1; // +1 for the final index
                let mut current_type = base_type.clone();
                for _ in 0..depth {
                    if let Type::Dict(_, val_type) = current_type {
                        current_type = (*val_type).clone();
                    } else {
                        break;
                    }
                }
                // Check if innermost value type is String
                matches!(current_type, Type::String)
            } else {
                false
            }
        } else {
            false
        };

        // Check if value_expr is a string literal
        let is_string_literal =
            matches!(&value_expr, syn::Expr::Lit(lit) if matches!(&lit.lit, syn::Lit::Str(_)));

        if needs_string_conversion && is_string_literal {
            parse_quote! { #value_expr.to_string() }
        } else {
            value_expr
        }
    } else {
        value_expr
    };

    // DEPYLER-0449: Detect if base is serde_json::Value (needs .as_object_mut())
    // DEPYLER-0560: Also detect if base is HashMap<String, serde_json::Value>
    let (needs_as_object_mut, needs_json_value_wrap) = if let HirExpr::Var(base_name) = base {
        if !is_numeric_index {
            // Check actual type from var_types
            if let Some(base_type) = ctx.var_types.get(base_name) {
                match base_type {
                    // Pure serde_json::Value - needs .as_object_mut()
                    Type::Custom(s) if s == "serde_json::Value" || s == "Value" => (true, false),
                    // HashMap<String, serde_json::Value> - needs json!() wrap on values
                    Type::Dict(_, val_type) => {
                        let val_needs_json = match val_type.as_ref() {
                            Type::Unknown => true,
                            Type::Custom(s) => s == "serde_json::Value" || s == "Value",
                            _ => false,
                        };
                        (false, val_needs_json)
                    }
                    _ => (false, false),
                }
            } else {
                // Fallback heuristic: check variable name patterns
                let name_str = base_name.as_str();
                let is_value_name = name_str == "config"
                    || name_str == "data"
                    || name_str == "value"
                    || name_str == "current"
                    || name_str == "obj"
                    || name_str == "json";
                // DEPYLER-0560: Also check common dict names that may have Value type
                let is_dict_value_name = name_str == "info"
                    || name_str == "result"
                    || name_str == "stats"
                    || name_str == "metadata"
                    || name_str == "output"
                    || name_str == "response";
                (is_value_name, is_dict_value_name)
            }
        } else {
            (false, false)
        }
    } else {
        (false, false)
    };

    // DEPYLER-0472: Wrap value in serde_json::Value when assigning to Value dicts
    // DEPYLER-0560: Also wrap when dict value type is serde_json::Value
    // Check if value needs wrapping (not already json!() or Value variant)
    let final_value_expr = if needs_as_object_mut || needs_json_value_wrap {
        // Check if value_expr is already a json!() or Value expression
        let value_str = quote! { #value_expr }.to_string();
        if value_str.contains("serde_json :: json !") || value_str.contains("serde_json :: Value") {
            // Already wrapped, use as-is
            value_expr
        } else {
            // Need to wrap in serde_json::json!() for HashMap<String, Value>
            // Use json!() instead of to_value() for consistency with dict literals
            ctx.needs_serde_json = true;
            parse_quote! { serde_json::json!(#value_expr) }
        }
    } else {
        value_expr
    };

    // DEPYLER-0567: Convert string literal keys to String for HashMap<String, ...>
    // Check if the index is a string literal that needs .to_string()
    let final_index = if !is_numeric_index {
        let idx_str = quote! { #final_index }.to_string();
        // If it's a string literal like "key", convert to "key".to_string()
        if idx_str.starts_with('"') && !idx_str.contains(".to_string()") {
            parse_quote! { #final_index.to_string() }
        } else {
            final_index
        }
    } else {
        final_index
    };

    if indices.is_empty() {
        // Simple assignment: d[k] = v OR list[i] = x
        if is_numeric_index {
            // DEPYLER-0314: Vec.insert(index as usize, value)
            // Wrap in parentheses to ensure correct operator precedence
            Ok(quote! { #base_expr.insert((#final_index) as usize, #final_value_expr); })
        } else if needs_as_object_mut {
            // DEPYLER-0449: serde_json::Value needs .as_object_mut() for insert
            // DEPYLER-0473: Clone key to avoid move-after-use errors
            Ok(
                quote! { #base_expr.as_object_mut().unwrap().insert((#final_index).clone(), #final_value_expr); },
            )
        } else {
            // HashMap.insert(key, value)
            Ok(quote! { #base_expr.insert(#final_index, #final_value_expr); })
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
            Ok(quote! { #chain.insert((#final_index) as usize, #final_value_expr); })
        } else if needs_as_object_mut {
            // DEPYLER-0449: serde_json::Value needs .as_object_mut() for insert
            // DEPYLER-0473: Clone key to avoid move-after-use errors
            Ok(
                quote! { #chain.as_object_mut().unwrap().insert((#final_index).clone(), #final_value_expr); },
            )
        } else {
            // HashMap.insert(key, value)
            Ok(quote! { #chain.insert(#final_index, #final_value_expr); })
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
///
/// DEPYLER-0494: Handle generator state variables in tuple assignments
/// When inside a generator, state variables must be assigned via self.field
/// instead of local variable destructuring.
///
/// # Complexity: 9 (within ≤10 target)
#[inline]
#[allow(clippy::unnecessary_to_owned)] // HashSet<String> requires owned String for contains()
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
            // DEPYLER-0494 FIX: Check if we're in a generator with state variables
            // Pattern: (a, b) = (0, 1) where a, b are generator state variables
            // Must generate: let _temp = (0, 1); self.a = _temp.0; self.b = _tuple.1;
            let generator_state_vars: Vec<_> = if ctx.in_generator {
                symbols
                    .iter()
                    .filter(|s| ctx.generator_state_vars.contains(&s.to_string()))
                    .collect()
            } else {
                vec![]
            };

            if !generator_state_vars.is_empty() {
                // At least one variable is a generator state variable
                // Destructure into temporary, then assign each field
                let temp_var = syn::Ident::new("_tuple_temp", proc_macro2::Span::call_site());
                let assignments: Vec<_> = symbols
                    .iter()
                    .enumerate()
                    .map(|(idx, s)| {
                        let ident = safe_ident(s);
                        let index = syn::Index::from(idx);

                        if ctx.generator_state_vars.contains(&s.to_string()) {
                            // State variable: self.field = temp.N;
                            quote! { self.#ident = #temp_var.#index; }
                        } else {
                            // Local variable: ident = temp.N; (if already declared)
                            // or: let ident = temp.N; (if first declaration)
                            if ctx.is_declared(s) {
                                quote! { #ident = #temp_var.#index; }
                            } else {
                                ctx.declare_var(s);
                                let mut_token = if ctx.mutable_vars.contains(*s) {
                                    quote! { mut }
                                } else {
                                    quote! {}
                                };
                                quote! { let #mut_token #ident = #temp_var.#index; }
                            }
                        }
                    })
                    .collect();

                Ok(quote! {
                    let #temp_var = #value_expr;
                    #(#assignments)*
                })
            } else {
                // No generator state variables - original logic
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
        }
        None => {
            // GH-109: Handle tuple unpacking with Index targets
            // Pattern: list[i], list[j] = list[j], list[i] (swap)
            // Strategy: Store RHS in temp tuple, then assign each element

            // Check if all targets are Index expressions (subscripts)
            let all_indices: Option<Vec<_>> = targets
                .iter()
                .map(|t| match t {
                    AssignTarget::Index { base, index } => Some((base, index)),
                    _ => None,
                })
                .collect();

            if let Some(indices) = all_indices {
                // All targets are subscripts - generate temp-based assignment
                let temp_var = syn::Ident::new("_swap_temp", proc_macro2::Span::call_site());

                // Generate assignments for each target from temp tuple
                let mut assignments = Vec::new();
                for (idx, (base, index)) in indices.iter().enumerate() {
                    let base_expr = base.to_rust_expr(ctx)?;
                    let index_expr = index.to_rust_expr(ctx)?;
                    let tuple_idx = syn::Index::from(idx);

                    // Check if base is a Vec (numeric index) or HashMap (string key)
                    let is_numeric = matches!(index.as_ref(), HirExpr::Literal(Literal::Int(_)));

                    if is_numeric {
                        // Vec assignment: base[index as usize] = temp.N
                        assignments.push(quote! {
                            #base_expr[(#index_expr) as usize] = #temp_var.#tuple_idx;
                        });
                    } else {
                        // HashMap assignment: base.insert(key, temp.N)
                        assignments.push(quote! {
                            #base_expr.insert(#index_expr, #temp_var.#tuple_idx);
                        });
                    }
                }

                return Ok(quote! {
                    let #temp_var = #value_expr;
                    #(#assignments)*
                });
            }

            bail!("Complex tuple unpacking not yet supported")
        }
    }
}

/// DEPYLER-0562: Generate code for tuple unpacking from regex match.groups()
///
/// Python: timestamp, level, message = match.groups()
/// Rust: Extract capture groups from Option<Captures<'_>>
///
/// NOTE: Currently unused - requires DEPYLER-0563 (regex type tracking) to be activated.
/// This function is ready for use when proper regex type inference is implemented.
///
/// # Arguments
/// * `targets` - The tuple target variables (e.g., [timestamp, level, message])
/// * `match_obj` - The HirExpr representing the match object (source of .groups())
/// * `ctx` - Code generation context
///
/// # Complexity: 8 (within ≤10 target)
#[inline]
#[allow(dead_code)] // Waiting for DEPYLER-0563
#[allow(clippy::unnecessary_to_owned)]
pub(crate) fn codegen_assign_tuple_groups(
    targets: &[AssignTarget],
    match_obj: &HirExpr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Convert match object to Rust expression
    let match_expr = match_obj.to_rust_expr(ctx)?;

    // Extract symbol names from targets
    let symbols: Vec<&str> = targets
        .iter()
        .filter_map(|t| match t {
            AssignTarget::Symbol(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    if symbols.len() != targets.len() {
        bail!("Complex tuple unpacking in match.groups() not yet supported");
    }

    // Generate capture group extraction for each target
    // Capture groups are 1-indexed (group 0 is the full match)
    let assignments: Vec<proc_macro2::TokenStream> = symbols
        .iter()
        .enumerate()
        .map(|(idx, s)| {
            let ident = safe_ident(s);
            let group_idx = idx + 1; // 1-indexed capture groups

            // Generate: var = caps.as_ref().and_then(|c| c.get(N)).map(|m| m.as_str().to_string()).unwrap_or_default()
            let extraction = quote! {
                #match_expr.as_ref().and_then(|c| c.get(#group_idx)).map(|m| m.as_str().to_string()).unwrap_or_default()
            };

            if ctx.in_generator && ctx.generator_state_vars.contains(&s.to_string()) {
                // Generator state variable: self.field = extraction
                quote! { self.#ident = #extraction; }
            } else if ctx.is_declared(s) {
                // Already declared: assignment
                quote! { #ident = #extraction; }
            } else {
                // New declaration
                ctx.declare_var(s);
                let mut_token = if ctx.mutable_vars.contains(*s) {
                    quote! { mut }
                } else {
                    quote! {}
                };
                quote! { let #mut_token #ident = #extraction; }
            }
        })
        .collect();

    Ok(quote! { #(#assignments)* })
}

/// DEPYLER-0565: Infer the return type from try block body statements
/// Returns None if the try block doesn't return a value, Some(type) otherwise
/// This is used to generate the correct closure return type for try/except
#[inline]
fn infer_try_body_return_type(body: &[HirStmt]) -> Option<Type> {
    for stmt in body {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                // Found a return with a value - infer its type
                return Some(infer_expr_return_type(expr));
            }
            HirStmt::While { body: inner, .. } | HirStmt::For { body: inner, .. } => {
                // Check inside loops
                if let Some(ty) = infer_try_body_return_type(inner) {
                    return Some(ty);
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                // Check inside if/else
                if let Some(ty) = infer_try_body_return_type(then_body) {
                    return Some(ty);
                }
                if let Some(else_stmts) = else_body {
                    if let Some(ty) = infer_try_body_return_type(else_stmts) {
                        return Some(ty);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// DEPYLER-0565: Infer the type of an expression for try/except closure return type
#[inline]
fn infer_expr_return_type(expr: &HirExpr) -> Type {
    match expr {
        HirExpr::Literal(lit) => match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Bool(_) => Type::Bool,
            Literal::None => Type::None,
            Literal::Bytes(_) => Type::Custom("bytes".to_string()),
        },
        HirExpr::MethodCall { method, .. } => {
            // Common method return types
            match method.as_str() {
                "hexdigest" | "encode" | "decode" | "strip" | "upper" | "lower" | "to_string" => {
                    Type::String
                }
                "len" | "count" => Type::Int,
                "is_empty" | "startswith" | "endswith" | "exists" | "is_file" | "is_dir" => {
                    Type::Bool
                }
                _ => Type::Unknown,
            }
        }
        HirExpr::Call { func, .. } => {
            // Common function return types
            match func.as_str() {
                "str" | "format" | "hex::encode" => Type::String,
                "int" | "len" => Type::Int,
                "float" => Type::Float,
                "bool" => Type::Bool,
                _ => Type::Unknown,
            }
        }
        _ => Type::Unknown,
    }
}

/// DEPYLER-0565: Convert HIR Type to closure return type tokens
#[inline]
fn try_return_type_to_tokens(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Int => quote! { i64 },
        Type::Float => quote! { f64 },
        Type::String => quote! { String },
        Type::Bool => quote! { bool },
        Type::None | Type::Unknown => quote! { () },
        _ => quote! { () },
    }
}

/// DEPYLER-0578: Check if handler ends with sys.exit() or process::exit()
/// This indicates the variable WILL be assigned if we reach code after the try/except
#[inline]
fn handler_ends_with_exit(handler_body: &[HirStmt]) -> bool {
    if let Some(last_stmt) = handler_body.last() {
        match last_stmt {
            // sys.exit(N) or exit(N)
            HirStmt::Expr(HirExpr::Call { func, .. }) => {
                func == "exit" || func == "sys.exit"
            }
            HirStmt::Expr(HirExpr::MethodCall { object, method, .. }) => {
                // sys.exit() as method call
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

/// DEPYLER-0578: Try to detect and generate json.load(sys.stdin) pattern
/// Pattern: try { data = json.load(sys.stdin) } except JSONDecodeError as e: { print; exit }
/// Returns: let data = match serde_json::from_reader(...) { Ok(d) => d, Err(e) => { ... } };
///
/// # Complexity
/// 8 (pattern matching + code generation)
#[inline]
fn try_generate_json_stdin_match(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<Option<proc_macro2::TokenStream>> {
    // Must have single assignment in try body
    if body.len() != 1 {
        return Ok(None);
    }

    // Must have single handler that ends with exit
    if handlers.len() != 1 || !handler_ends_with_exit(&handlers[0].body) {
        return Ok(None);
    }

    // Check for: data = json.load(sys.stdin) OR data = json.load(file)
    let (var_name, is_json_load) = match &body[0] {
        HirStmt::Assign {
            target: AssignTarget::Symbol(name),
            value,
            ..
        } => {
            // Check if value is json.load(sys.stdin) or json.load(file)
            let is_json = match value {
                HirExpr::MethodCall {
                    object,
                    method,
                    args,
                    ..
                } => {
                    if method == "load" {
                        if let HirExpr::Var(module) = &**object {
                            if module == "json" && args.len() == 1 {
                                // Check if argument is sys.stdin
                                match &args[0] {
                                    HirExpr::Attribute { value: v, attr } => {
                                        if let HirExpr::Var(m) = &**v {
                                            m == "sys" && attr == "stdin"
                                        } else {
                                            false
                                        }
                                    }
                                    // Also allow json.load(f) where f is a file variable
                                    HirExpr::Var(_) => true,
                                    _ => false,
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            };
            (name.clone(), is_json)
        }
        _ => return Ok(None),
    };

    if !is_json_load {
        return Ok(None);
    }

    // Generate handler body statements
    ctx.enter_scope();
    if let Some(exc_var) = &handlers[0].name {
        ctx.declare_var(exc_var);
    }

    let handler_stmts: Vec<_> = handlers[0]
        .body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    // Generate the error pattern
    let err_pattern = if let Some(exc_var) = &handlers[0].name {
        let exc_ident = safe_ident(exc_var);
        quote! { Err(#exc_ident) }
    } else {
        quote! { Err(_) }
    };

    // Variable identifier
    let var_ident = safe_ident(&var_name);

    // Mark that we need serde_json
    ctx.needs_serde_json = true;

    // Generate the match expression for json.load(sys.stdin)
    let match_expr = quote! {
        let #var_ident = match serde_json::from_reader::<_, serde_json::Value>(std::io::stdin()) {
            Ok(__json_data) => __json_data,
            #err_pattern => {
                #(#handler_stmts)*
            }
        };
    };

    // Add finally block if present
    let result = if let Some(finally_body) = finalbody {
        let finally_stmts: Vec<_> = finally_body
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;
        quote! {
            #match_expr
            #(#finally_stmts)*
        }
    } else {
        match_expr
    };

    // Declare the variable in context so it's accessible after the try/except
    ctx.declare_var(&var_name);

    Ok(Some(result))
}

/// Generate code for Try/except/finally statement
#[inline]
pub(crate) fn codegen_try_stmt(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // CITL: Trace error handling strategy
    trace_decision!(
        category = DecisionCategory::ErrorHandling,
        name = "try_except",
        chosen = "match_result",
        alternatives = ["unwrap_or", "question_mark", "anyhow_context", "custom_error"],
        confidence = 0.80
    );

    // DEPYLER-0578: Detect json.load(sys.stdin) pattern with exit handler
    // Pattern: try { data = json.load(sys.stdin) } except JSONDecodeError as e: { print; exit }
    // This pattern assigns a variable that must be accessible AFTER the try/except block
    // Generate: let data = match serde_json::from_reader(...) { Ok(d) => d, Err(e) => { handler } };
    if let Some(result) = try_generate_json_stdin_match(body, handlers, finalbody, ctx)? {
        return Ok(result);
    }

    // DEPYLER-0358: Detect simple try-except pattern for optimization
    // Pattern: try { return int(str_var) } except ValueError { return literal }
    // We can optimize this to: s.parse::<i32>().unwrap_or(literal)
    // DEPYLER-0359: Exclude patterns with exception binding (except E as e:)
    // Those need proper match with Err(e) binding
    let simple_pattern_info = if body.len() == 1
        && handlers.len() == 1
        && handlers[0].body.len() == 1
        && handlers[0].name.is_none()
    // No exception variable binding
    {
        // Check if handler body is a Return statement with a simple value
        match &handlers[0].body[0] {
            // Direct literal: return 42, return "error", etc.
            HirStmt::Return(Some(HirExpr::Literal(lit))) => Some((
                (match lit {
                    Literal::Int(n) => n.to_string(),
                    Literal::Float(f) => f.to_string(),
                    Literal::String(s) => format!("\"{}\"", s),
                    Literal::Bool(b) => b.to_string(),
                    _ => "Default::default()".to_string(),
                })
                .to_string(),
                handlers[0].exception_type.clone(),
            )),
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
    // DEPYLER-0395: Try block statements should include 'return' keyword
    // Save and temporarily disable is_final_statement so return statements
    // in try blocks get the explicit 'return' keyword (needed for proper exception handling)
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

    ctx.enter_scope();
    let try_stmts: Vec<_> = body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    // Restore is_final_statement flag
    ctx.is_final_statement = saved_is_final;

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
        // DEPYLER-0437/0429: Generate proper match expressions for parse() patterns
        // Check if try_stmts contains a .parse() call that we can convert to match
        if handlers.len() == 1 {
            if let Some((var_name, parse_expr_str, remaining_stmts)) =
                extract_parse_from_tokens(&try_stmts)
            {
                // Parse the expression string back to token stream
                let parse_expr: proc_macro2::TokenStream = match parse_expr_str.parse() {
                    Ok(ts) => ts,
                    Err(_) => return Ok(quote! { #(#try_stmts)* }), // Fallback on parse error
                };
                let ok_var = safe_ident(&var_name);

                // Generate Ok branch (remaining statements after parse)
                let ok_body = quote! { #(#remaining_stmts)* };

                // Generate Err branch (handler body)
                let err_body = &handler_tokens[0];

                // DEPYLER-0429: Check if exception variable should be bound
                let err_pattern = if let Some(exc_var) = &handlers[0].name {
                    // Bind exception variable: Err(e) => { ... }
                    let exc_ident = safe_ident(exc_var);
                    quote! { Err(#exc_ident) }
                } else {
                    // No exception variable: Err(_) => { ... }
                    quote! { Err(_) }
                };

                // Build match expression
                let match_expr = quote! {
                    match #parse_expr {
                        Ok(#ok_var) => { #ok_body },
                        #err_pattern => { #err_body }
                    }
                };

                // Wrap with finally if present
                if let Some(finally_code) = finally_stmts {
                    return Ok(quote! {
                        {
                            #match_expr
                            #finally_code
                        }
                    });
                } else {
                    return Ok(match_expr);
                }
            }
        }

        // Fall through to existing simple_pattern_info logic
        if let Some((exception_value_str, _exception_type)) = simple_pattern_info {
            // Fall through to existing unwrap_or logic if not a match pattern
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
                    .replace(
                        "unwrap_or_default ()",
                        &format!("unwrap_or ({})", exception_value_str),
                    )
                    .replace(
                        "unwrap_or_default()",
                        &format!("unwrap_or({})", exception_value_str),
                    );

                // Parse back to token stream
                let fixed_tokens: proc_macro2::TokenStream = fixed_code.parse().unwrap_or(try_code);

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
            // DEPYLER-0489: General exception handling with variable binding
            // Generate proper match expression for all handlers, not just special cases
            // This ensures exception variables (except E as e:) are always available

            // Check if any handler needs exception variable binding
            let needs_error_binding = handlers.iter().any(|h| h.name.is_some());

            if needs_error_binding {
                // DEPYLER-0489: Generate Result-returning closure + match pattern
                // DEPYLER-0565: Infer closure return type from try body return statements
                // Pattern: (|| -> Result<T, Box<dyn std::error::Error>> { try_body })().unwrap_or_else(|e| { handler })

                // DEPYLER-0565: Infer return type from try body
                let try_return_type = infer_try_body_return_type(body);
                let return_type_tokens = try_return_type
                    .as_ref()
                    .map(try_return_type_to_tokens)
                    .unwrap_or_else(|| quote! { () });
                let ok_value = try_return_type
                    .as_ref()
                    .map(|_| quote! { _result })
                    .unwrap_or_else(|| quote! { () });
                let ok_arm_body = if try_return_type.is_some() {
                    quote! { return Ok(_result); }
                } else {
                    quote! {}
                };

                let handler_body = if handlers.len() == 1 {
                    // Single handler - use match pattern
                    let err_pattern = if let Some(exc_var) = &handlers[0].name {
                        let exc_ident = safe_ident(exc_var);
                        quote! { Err(#exc_ident) }
                    } else {
                        quote! { Err(_) }
                    };

                    let handler_code = &handler_tokens[0];

                    if let Some(finally_code) = finally_stmts {
                        quote! {
                            {
                                match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                                    #(#try_stmts)*
                                    Ok(Default::default())
                                })() {
                                    Ok(#ok_value) => { #ok_arm_body },
                                    #err_pattern => { #handler_code }
                                }
                                #finally_code
                            }
                        }
                    } else {
                        quote! {
                            match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                                #(#try_stmts)*
                                Ok(Default::default())
                            })() {
                                Ok(#ok_value) => { #ok_arm_body },
                                #err_pattern => { #handler_code }
                            }
                        }
                    }
                } else {
                    // DEPYLER-0489: Multiple handlers with exception variable binding
                    // Find ANY handler that has a variable binding (not just the first)
                    // Full multi-handler support with type checking would require more complex codegen
                    let exc_var_opt = handlers.iter().find_map(|h| h.name.as_ref());

                    if let Some(exc_var) = exc_var_opt {
                        let exc_ident = safe_ident(exc_var);
                        let handler_code = quote! { #(#handler_tokens)* };

                        if let Some(finally_code) = finally_stmts {
                            quote! {
                                {
                                    match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                                        #(#try_stmts)*
                                        Ok(Default::default())
                                    })() {
                                        Ok(#ok_value) => { #ok_arm_body },
                                        Err(#exc_ident) => { #handler_code }
                                    }
                                    #finally_code
                                }
                            }
                        } else {
                            quote! {
                                match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                                    #(#try_stmts)*
                                    Ok(Default::default())
                                })() {
                                    Ok(#ok_value) => { #ok_arm_body },
                                    Err(#exc_ident) => { #handler_code }
                                }
                            }
                        }
                    } else {
                        // No binding needed - simple concatenation
                        let handler_code = quote! { #(#handler_tokens)* };
                        if let Some(finally_code) = finally_stmts {
                            quote! {
                                {
                                    #(#try_stmts)*
                                    #handler_code
                                    #finally_code
                                }
                            }
                        } else {
                            quote! {
                                {
                                    #(#try_stmts)*
                                    #handler_code
                                }
                            }
                        }
                    }
                };

                return Ok(handler_body);
            }

            // DEPYLER-0357: Non-simple patterns - use original concatenation logic
            // Execute try block statements, then if we have a single handler, use it
            if handlers.len() == 1 {
                // DEPYLER-0359: Check if handler has exception binding for proper match generation
                if handlers[0].name.is_some() && body.len() == 1 {
                    if let HirStmt::Return(Some(HirExpr::Call { func, args, .. })) = &body[0] {
                        if func == "int" && args.len() == 1 {
                            // Single handler with exception binding - use match with Err(e)
                            let arg_expr = args[0].to_rust_expr(ctx)?;
                            let handler_body = &handler_tokens[0];
                            let err_var = handlers[0]
                                .name
                                .as_ref()
                                .map(|s| {
                                    safe_ident(s) // DEPYLER-0023
                                })
                                .unwrap();

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

                // DEPYLER-0362/0444: Check if try block has error handling (unwrap_or_default, .expect())
                // If so, don't concatenate handler as it creates invalid syntax or unreachable code
                let try_code_str = quote! { #(#try_stmts)* }.to_string();
                let has_error_handling = try_code_str.contains("unwrap_or_default")
                    || try_code_str.contains("unwrap_or(")
                    || try_code_str.contains(".expect(");

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
                    // DEPYLER-0444: Check if handler has exception variable binding
                    // If so, skip handler code since we can't bind it in unconditional context
                    let has_exception_binding = handlers[0].name.is_some();

                    if has_exception_binding {
                        // Skip handler code - it would reference unbound exception variable
                        // NOTE: This means exception handlers are not fully implemented (tracked in DEPYLER-0424)
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
                            // NOTE: This executes both unconditionally - need proper conditional logic (tracked in DEPYLER-0424)
                            // based on which operations can panic (ZeroDivisionError, IndexError, etc.)
                            Ok(quote! {
                                {
                                    #(#try_stmts)*
                                    #handler_code
                                }
                            })
                        }
                    }
                }
            } else {
                // DEPYLER-0359: Multiple handlers - generate conditional error handling
                // For operations like int(data) with multiple exception types, we need proper
                // match-based error handling instead of simple unwrap_or

                // Check if try block is simple return with parse operation
                if body.len() == 1 {
                    if let HirStmt::Return(Some(HirExpr::Call { func, args, .. })) = &body[0] {
                        if func == "int" && args.len() == 1 {
                            let arg_expr = args[0].to_rust_expr(ctx)?;

                            // Check if any handler binds the exception variable
                            let has_exception_binding = handlers.iter().any(|h| h.name.is_some());

                            if has_exception_binding && handlers.len() == 1 {
                                // Single handler with exception binding - use match with Err(e)
                                let handler_body = &handler_tokens[0];
                                let err_var = handlers[0]
                                    .name
                                    .as_ref()
                                    .map(|s| {
                                        safe_ident(s) // DEPYLER-0023
                                    })
                                    .unwrap();

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
                                // This is semantically incorrect but compiles. NOTE: Improve error dispatch logic (tracked in DEPYLER-0424)

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

/// DEPYLER-0437: Extract .parse() call from generated token stream
///
/// Looks for pattern: `let var = expr.parse::<i32>().unwrap_or_default();`
/// Returns: (variable_name, parse_expression_without_unwrap_or, remaining_statements)
fn extract_parse_from_tokens(
    try_stmts: &[proc_macro2::TokenStream],
) -> Option<(String, String, Vec<proc_macro2::TokenStream>)> {
    if try_stmts.is_empty() {
        return None;
    }

    // Convert first statement to string (note: tokens have spaces between them)
    let first_stmt = try_stmts[0].to_string();

    // Pattern: let var_name = something . parse :: < i32 > () . unwrap_or_default () ;
    // Note: TokenStream.to_string() adds spaces between tokens
    if first_stmt.contains("parse") && first_stmt.contains("unwrap_or_default") {
        // Extract variable name (after "let " and before " =")
        if let Some(let_pos) = first_stmt.find("let ") {
            if let Some(eq_pos) = first_stmt[let_pos..].find(" =") {
                let var_name = first_stmt[let_pos + 4..let_pos + eq_pos].trim().to_string();

                // Extract parse expression (between "= " and "unwrap_or_default")
                // We need to find the start of unwrap_or_default and go back to find the parse call
                if let Some(eq_start) = first_stmt.find(" = ") {
                    if let Some(unwrap_pos) = first_stmt.find("unwrap_or_default") {
                        // Go back from unwrap_pos to skip ". " before it
                        let parse_end =
                            if unwrap_pos >= 2 && &first_stmt[unwrap_pos - 2..unwrap_pos] == ". " {
                                unwrap_pos - 2
                            } else {
                                unwrap_pos
                            };

                        let parse_expr = first_stmt[eq_start + 3..parse_end].trim().to_string();

                        // Collect remaining statements
                        let remaining: Vec<_> = try_stmts[1..].to_vec();

                        return Some((var_name, parse_expr, remaining));
                    }
                }
            }
        }
    }

    None
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

/// DEPYLER-0399: Extract string literal from HirExpr
///
/// # Complexity
/// 2 (pattern match + string clone)
fn extract_string_literal(expr: &HirExpr) -> String {
    match expr {
        HirExpr::Literal(Literal::String(s)) => s.clone(),
        _ => String::new(),
    }
}

/// DEPYLER-0399: Extract string value from kwarg by name
///
/// # Complexity
/// 4 (iterator + filter + match)
fn extract_kwarg_string(kwargs: &[(String, HirExpr)], key: &str) -> Option<String> {
    kwargs
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| match v {
            HirExpr::Literal(Literal::String(s)) => Some(s.clone()),
            _ => None,
        })
}

/// DEPYLER-0399: Extract boolean value from kwarg by name
///
/// # Complexity
/// 4 (iterator + filter + match)
fn extract_kwarg_bool(kwargs: &[(String, HirExpr)], key: &str) -> Option<bool> {
    kwargs
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| match v {
            HirExpr::Var(s) if s == "True" => Some(true),
            HirExpr::Var(s) if s == "False" => Some(false),
            _ => None,
        })
}

/// DEPYLER-0425: Extract subcommand fields accessed in handler body
/// Analyzes HIR statements to find args.field attribute accesses
///
/// DEPYLER-0480: Now accepts dest_field parameter to filter dynamically
/// DEPYLER-0481: Now accepts cmd_name and ctx to filter out top-level args
///
/// # Complexity
/// 10 (recursive HIR walk + HashSet operations)
fn extract_accessed_subcommand_fields(
    body: &[HirStmt],
    args_var: &str,
    dest_field: &str,
    cmd_name: &str,
    ctx: &CodeGenContext,
) -> Vec<String> {
    let mut fields = std::collections::HashSet::new();
    extract_fields_recursive(body, args_var, dest_field, &mut fields);

    // DEPYLER-0481: Filter out top-level args that don't belong to this subcommand
    // Only keep fields that are actual arguments of the subcommand
    // DEPYLER-0605: Fix duplicate SubcommandInfo issue - prefer the one with arguments
    // When preregister_subcommands_from_hir runs, it may create an empty SubcommandInfo
    // with KEY = command_name. Later, assignment processing creates another with
    // KEY = variable_name and the actual arguments. We need to find the one with args.
    let subcommand_arg_names: std::collections::HashSet<String> = ctx
        .argparser_tracker
        .subcommands
        .values()
        .filter(|sub| sub.name == cmd_name)
        .max_by_key(|sub| sub.arguments.len())
        .map(|sub| {
            sub.arguments
                .iter()
                .map(|arg| {
                    // Extract dest name from argument
                    arg.dest.clone().unwrap_or_else(|| {
                        // If no dest, use the name (for positional) or long option without dashes
                        if arg.is_positional {
                            arg.name.clone()
                        } else if let Some(long) = &arg.long {
                            long.trim_start_matches("--").replace('-', "_")
                        } else {
                            arg.name.trim_start_matches('-').replace('-', "_")
                        }
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let mut result: Vec<_> = fields
        .into_iter()
        .filter(|f| subcommand_arg_names.contains(f))
        .collect();
    result.sort(); // Deterministic order
    result
}

/// DEPYLER-0425: Recursively extract fields from HIR statements
///
/// DEPYLER-0480: Now accepts dest_field parameter to pass through
///
/// # Complexity
/// 8 (recursive statement traversal)
fn extract_fields_recursive(
    stmts: &[HirStmt],
    args_var: &str,
    dest_field: &str,
    fields: &mut std::collections::HashSet<String>,
) {
    for stmt in stmts {
        match stmt {
            HirStmt::Expr(expr) => extract_fields_from_expr(expr, args_var, dest_field, fields),
            HirStmt::Assign { value, .. } => {
                extract_fields_from_expr(value, args_var, dest_field, fields)
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                // DEPYLER-0518: Also extract fields from condition
                // Example: `if not validate_email(args.address)` has args.address in condition
                extract_fields_from_expr(condition, args_var, dest_field, fields);
                extract_fields_recursive(then_body, args_var, dest_field, fields);
                if let Some(else_stmts) = else_body {
                    extract_fields_recursive(else_stmts, args_var, dest_field, fields);
                }
            }
            // DEPYLER-0577: Recurse into While condition (may contain args.field)
            HirStmt::While {
                condition,
                body: loop_body,
            } => {
                extract_fields_from_expr(condition, args_var, dest_field, fields);
                extract_fields_recursive(loop_body, args_var, dest_field, fields);
            }
            // DEPYLER-0577: Recurse into For iterator (may contain args.field)
            HirStmt::For {
                iter,
                body: loop_body,
                ..
            } => {
                extract_fields_from_expr(iter, args_var, dest_field, fields);
                extract_fields_recursive(loop_body, args_var, dest_field, fields);
            }
            HirStmt::Try {
                body: try_body,
                handlers,
                orelse,
                finalbody,
            } => {
                extract_fields_recursive(try_body, args_var, dest_field, fields);
                for handler in handlers {
                    extract_fields_recursive(&handler.body, args_var, dest_field, fields);
                }
                if let Some(orelse_stmts) = orelse {
                    extract_fields_recursive(orelse_stmts, args_var, dest_field, fields);
                }
                if let Some(finally_stmts) = finalbody {
                    extract_fields_recursive(finally_stmts, args_var, dest_field, fields);
                }
            }
            HirStmt::With {
                body: with_body, ..
            } => {
                extract_fields_recursive(with_body, args_var, dest_field, fields);
            }
            _ => {}
        }
    }
}

/// DEPYLER-0425: Extract fields from HIR expression
/// Finds patterns like `args.field` and collects field names
///
/// DEPYLER-0480: Now uses dest_field parameter instead of hardcoded "command"/"action"
///
/// # Complexity
/// 10 (expression traversal + pattern matching)
fn extract_fields_from_expr(
    expr: &HirExpr,
    args_var: &str,
    dest_field: &str,
    fields: &mut std::collections::HashSet<String>,
) {
    match expr {
        // Pattern: args.field
        HirExpr::Attribute { value, attr } => {
            if let HirExpr::Var(var) = value.as_ref() {
                if var == args_var {
                    // DEPYLER-0480: Filter out the dest field dynamically
                    // The dest field (e.g., "command" or "action") is the match discriminant,
                    // so it shouldn't be included in the extracted fields list
                    if attr != dest_field {
                        fields.insert(attr.clone());
                    }
                }
            }
        }
        // Recurse into nested expressions
        HirExpr::Call {
            args: call_args, ..
        } => {
            for arg in call_args {
                extract_fields_from_expr(arg, args_var, dest_field, fields);
            }
        }
        HirExpr::Binary { left, right, .. } => {
            extract_fields_from_expr(left, args_var, dest_field, fields);
            extract_fields_from_expr(right, args_var, dest_field, fields);
        }
        HirExpr::Unary { operand, .. } => {
            extract_fields_from_expr(operand, args_var, dest_field, fields);
        }
        HirExpr::IfExpr { test, body, orelse } => {
            extract_fields_from_expr(test, args_var, dest_field, fields);
            extract_fields_from_expr(body, args_var, dest_field, fields);
            extract_fields_from_expr(orelse, args_var, dest_field, fields);
        }
        HirExpr::Index { base, index } => {
            extract_fields_from_expr(base, args_var, dest_field, fields);
            extract_fields_from_expr(index, args_var, dest_field, fields);
        }
        HirExpr::List(elements) | HirExpr::Tuple(elements) | HirExpr::Set(elements) => {
            for elem in elements {
                extract_fields_from_expr(elem, args_var, dest_field, fields);
            }
        }
        HirExpr::Dict(pairs) => {
            for (key, value) in pairs {
                extract_fields_from_expr(key, args_var, dest_field, fields);
                extract_fields_from_expr(value, args_var, dest_field, fields);
            }
        }
        HirExpr::MethodCall {
            object,
            args: method_args,
            ..
        } => {
            extract_fields_from_expr(object, args_var, dest_field, fields);
            for arg in method_args {
                extract_fields_from_expr(arg, args_var, dest_field, fields);
            }
        }
        // DEPYLER-0577: Handle f-strings - recurse into expression parts
        HirExpr::FString { parts } => {
            for part in parts {
                if let crate::hir::FStringPart::Expr(expr) = part {
                    extract_fields_from_expr(expr, args_var, dest_field, fields);
                }
            }
        }
        _ => {}
    }
}

/// DEPYLER-0399: Try to generate a match statement for subcommand dispatch
///
/// Detects patterns like:
/// ```python
/// if args.command == "clone":
///     handle_clone(args)
/// elif args.command == "push":
///     handle_push(args)
/// ```
///
/// And converts to:
/// ```rust
/// match args.command {
///     Commands::Clone { url } => {
///         handle_clone(args);
///     }
///     Commands::Push { remote } => {
///         handle_push(args);
///     }
/// }
/// ```
fn try_generate_subcommand_match(
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<Option<proc_macro2::TokenStream>> {
    use quote::{format_ident, quote};

    // DEPYLER-0456 Bug #2: Get dest_field from subparser info
    // Find the dest_field name (e.g., "action" or "command")
    let dest_field = ctx
        .argparser_tracker
        .subparsers
        .values()
        .next()
        .map(|sp| sp.dest_field.clone())
        .unwrap_or_else(|| "command".to_string()); // Default to "command" for backwards compatibility

    // Check if condition matches: args.<dest_field> == "string" OR CSE temp variable
    let command_name = match is_subcommand_check(condition, &dest_field, ctx) {
        Some(name) => name,
        None => return Ok(None),
    };

    // Collect all branches (if + elif chain)
    let mut branches = vec![(command_name, then_body)];

    // Check if else is another if statement (elif pattern)
    let mut current_else = else_body;
    while let Some(else_stmts) = current_else {
        // DEPYLER-0456 Bug #2 FIX: Handle CSE-optimized elif branches
        // CSE creates: [assignment: _cse_temp_N = check, if: _cse_temp_N { ... }]
        // Original (pre-CSE) elif is a single If statement
        let (elif_stmt, cse_cmd_name) = if else_stmts.len() == 1 {
            // Pre-CSE or direct elif: single If statement
            (&else_stmts[0], None)
        } else if else_stmts.len() == 2 {
            // CSE-optimized elif: [assignment, if]
            // Extract command name from the CSE assignment
            if let HirStmt::Assign {
                target: AssignTarget::Symbol(var),
                value,
                ..
            } = &else_stmts[0]
            {
                if var.starts_with("_cse_temp") {
                    // Extract command name from the assignment value
                    let cmd_name = is_subcommand_check(value, &dest_field, ctx);
                    (&else_stmts[1], cmd_name)
                } else {
                    // Not a CSE pattern, stop collecting
                    break;
                }
            } else {
                // Not a CSE pattern, stop collecting
                break;
            }
        } else {
            // Not an elif pattern, stop collecting
            break;
        };

        // Check if this is an If statement with subcommand check
        if let HirStmt::If {
            condition: elif_cond,
            then_body: elif_then,
            else_body: elif_else,
        } = elif_stmt
        {
            // Use command name from CSE assignment if available, otherwise check condition
            let elif_name =
                cse_cmd_name.or_else(|| is_subcommand_check(elif_cond, &dest_field, ctx));

            if let Some(name) = elif_name {
                branches.push((name, elif_then.as_slice()));
                current_else = elif_else;
                continue;
            }
        }

        // Not an elif pattern, stop collecting
        break;
    }

    // DEPYLER-0482: Check if any branch has an early return
    // If so, don't add wildcard unreachable!() because execution continues to next match
    let has_early_return = branches.iter().any(|(_, body)| {
        body.iter()
            .any(|stmt| matches!(stmt, HirStmt::Return { .. }))
    });

    // Generate match arms
    let arms: Vec<proc_macro2::TokenStream> = branches
        .iter()
        .map(|(cmd_name, body)| {
            // Convert command name to PascalCase variant
            let variant_name = format_ident!("{}", to_pascal_case_subcommand(cmd_name));

            // DEPYLER-0425: Detect which fields are accessed in the body
            // This determines whether we use Pattern A ({ .. }) or Pattern B ({ field1, field2, ... })
            // DEPYLER-0480: Pass dest_field to dynamically filter based on actual dest parameter
            // DEPYLER-0481: Pass cmd_name and ctx to filter out top-level args
            let mut accessed_fields =
                extract_accessed_subcommand_fields(body, "args", &dest_field, cmd_name, ctx);

            // DEPYLER-0608: Detect if body calls a cmd_* handler
            // If so, get ALL subcommand fields since the handler accesses them internally
            // Pattern: the match arm body is `cmd_list(args)` which needs all `list` subcommand fields
            let calls_cmd_handler = body.iter().any(|stmt| {
                if let HirStmt::Expr(HirExpr::Call { func: func_name, args: call_args, .. }) = stmt {
                    // func is Symbol (String), not Box<HirExpr>
                    // Check if it's a cmd_* or handle_* function call with args parameter
                    let is_handler = func_name.starts_with("cmd_") || func_name.starts_with("handle_");
                    let has_args_param = call_args.iter().any(|a| matches!(a, HirExpr::Var(v) if v == "args"));
                    is_handler && has_args_param
                } else {
                    false
                }
            });

            if calls_cmd_handler && accessed_fields.is_empty() {
                // Get ALL fields for this subcommand
                if let Some(subcommand) = ctx
                    .argparser_tracker
                    .subcommands
                    .values()
                    .filter(|sc| sc.name == *cmd_name)
                    .max_by_key(|sc| sc.arguments.len())
                {
                    for arg in &subcommand.arguments {
                        let field_name = arg.long.as_ref()
                            .map(|s| s.trim_start_matches('-').to_string())
                            .unwrap_or_else(|| arg.name.clone());
                        accessed_fields.push(field_name);
                    }
                }
            }

            // DEPYLER-0608: Set context flags for handler call transformation
            // When in a subcommand match arm that calls a handler, expr_gen will
            // transform cmd_X(args) → cmd_X(field1, field2, ...)
            let was_in_match_arm = ctx.in_subcommand_match_arm;
            let old_match_fields = std::mem::take(&mut ctx.subcommand_match_fields);
            if calls_cmd_handler {
                ctx.in_subcommand_match_arm = true;
                ctx.subcommand_match_fields = accessed_fields.clone();
            }

            // Generate body statements
            ctx.enter_scope();

            // DEPYLER-0577: Register field types in var_types before processing body
            // This allows type-aware codegen (e.g., float vs int comparisons)
            // DEPYLER-0605: Use filter + max_by_key to find the SubcommandInfo with most arguments
            for field_name in &accessed_fields {
                if let Some(subcommand) = ctx
                    .argparser_tracker
                    .subcommands
                    .values()
                    .filter(|sc| sc.name == *cmd_name)
                    .max_by_key(|sc| sc.arguments.len())
                {
                    if let Some(arg) = subcommand.arguments.iter().find(|a| {
                        let arg_name = a.long.as_ref()
                            .map(|s| s.trim_start_matches('-').to_string())
                            .unwrap_or_else(|| a.name.clone());
                        &arg_name == field_name
                    }) {
                        if let Some(ref ty) = arg.arg_type {
                            ctx.var_types.insert(field_name.clone(), ty.clone());
                        }
                    }
                }
            }

            let body_stmts: Vec<_> = body
                .iter()
                .filter_map(|s| {
                    match s.to_rust_tokens(ctx) {
                        Ok(tokens) => Some(tokens),
                        Err(e) => {
                            // DEPYLER-0593: Log conversion errors instead of silently dropping
                            tracing::warn!("argparse body stmt conversion failed: {}", e);
                            None
                        }
                    }
                })
                .collect();
            ctx.exit_scope();

            // DEPYLER-0608: Restore context flags
            ctx.in_subcommand_match_arm = was_in_match_arm;
            ctx.subcommand_match_fields = old_match_fields;

            // DEPYLER-0456 Bug #3 FIX: Always use struct variant syntax `{}`
            // Clap generates struct variants (e.g., `Init {}`) not unit variants (e.g., `Init`)
            //
            // DEPYLER-0425: Pattern selection based on field usage
            // - Pattern A: No fields accessed → { .. } (handler gets &args)
            // - Pattern B: Fields accessed → { field1, field2, ... } (handler gets individual fields)
            if accessed_fields.is_empty() {
                // Pattern A: No field access, use { .. }
                quote! {
                    Commands::#variant_name { .. } => {
                        #(#body_stmts)*
                    }
                }
            } else {
                // Pattern B: Extract accessed fields with explicit ref patterns
                // Using `ref` ensures consistent binding as references regardless of match ergonomics
                let _field_idents: Vec<syn::Ident> = accessed_fields
                    .iter()
                    .map(|f| format_ident!("{}", f))
                    .collect();
                let ref_field_patterns: Vec<proc_macro2::TokenStream> = accessed_fields
                    .iter()
                    .map(|f| {
                        let ident = format_ident!("{}", f);
                        quote! { ref #ident }
                    })
                    .collect();

                // DEPYLER-0526: Generate field conversion bindings for borrowed match variables
                // When matching &args.command, destructured fields are references (&String, &bool)
                // Convert to owned values so they work with functions expecting either owned or borrowed:
                // - String fields: .to_string() converts &String → String
                //   String can then deref-coerce to &str if needed
                // - bool/primitives: dereference with *
                let field_bindings: Vec<proc_macro2::TokenStream> = accessed_fields
                    .iter()
                    .map(|field_name| {
                        let field_ident = format_ident!("{}", field_name);

                        // Look up field type from subcommand arguments
                        // Check both arg_type and action (for store_true/store_false bool flags)
                        // DEPYLER-0605: Use filter + max_by_key to find the SubcommandInfo with most arguments
                        let maybe_arg = ctx
                            .argparser_tracker
                            .subcommands
                            .values()
                            .filter(|sc| sc.name == *cmd_name)
                            .max_by_key(|sc| sc.arguments.len())
                            .and_then(|sc| {
                                sc.arguments.iter().find(|arg| {
                                    // Match by field name (from long flag or positional name)
                                    let arg_field_name = arg
                                        .long
                                        .as_ref()
                                        .map(|s| s.trim_start_matches('-').to_string())
                                        .unwrap_or_else(|| arg.name.clone());
                                    arg_field_name == *field_name
                                })
                            });

                        // Determine type: check arg_type first, then action for bool flags
                        let field_type = maybe_arg
                            .and_then(|arg| {
                                // If arg_type is set, use it
                                if arg.arg_type.is_some() {
                                    return arg.arg_type.clone();
                                }
                                // Check action for bool flags: store_true/store_false → Bool
                                if matches!(
                                    arg.action.as_deref(),
                                    Some("store_true") | Some("store_false") | Some("store_const")
                                ) {
                                    return Some(Type::Bool);
                                }
                                None
                            })
                            .or_else(|| {
                                // DEPYLER-0526: Name-based fallback for common boolean fields
                                // If argument lookup failed, use heuristics based on field name
                                let field_lower = field_name.to_lowercase();
                                let bool_indicators = [
                                    "binary",
                                    "append",
                                    "verbose",
                                    "quiet",
                                    "force",
                                    "dry_run",
                                    "recursive",
                                    "debug",
                                    "silent",
                                    "capture",
                                    "overwrite",
                                ];
                                if bool_indicators
                                    .iter()
                                    .any(|ind| field_lower == *ind || field_lower.ends_with(ind))
                                {
                                    Some(Type::Bool)
                                } else {
                                    None
                                }
                            });

                        // Generate conversion based on type
                        // NOTE: With explicit `ref` patterns, all fields are bound as references:
                        //   - Copy types (bool, int, float) need dereferencing: *field
                        //   - Non-Copy types (String, Vec) are already &T
                        match field_type {
                            Some(Type::Bool) => {
                                // With explicit `ref` pattern, bool is &bool - dereference to get bool
                                quote! { let #field_ident = *#field_ident; }
                            }
                            Some(Type::Int) | Some(Type::Float) => {
                                // DEPYLER-0576: Check if field has a default value (is Option<T>)
                                // Clap represents optional args with defaults as Option<T>
                                // ref binding gives &Option<T>, need to unwrap with default
                                let has_default = maybe_arg
                                    .as_ref()
                                    .map(|a| a.default.is_some())
                                    .unwrap_or(false);

                                if has_default {
                                    // Field is Option<T>, unwrap with default
                                    // Clone the default expression to release borrow on ctx
                                    let default_expr_opt = maybe_arg
                                        .and_then(|a| a.default.clone());

                                    let default_val = if let Some(ref d) = default_expr_opt {
                                        d.to_rust_expr(ctx).ok()
                                    } else {
                                        None
                                    }.unwrap_or_else(|| {
                                        // Fallback to 0.0 for Float, 0 for Int
                                        if matches!(field_type, Some(Type::Float)) {
                                            syn::parse_quote! { 0.0 }
                                        } else {
                                            syn::parse_quote! { 0 }
                                        }
                                    });
                                    quote! { let #field_ident = #field_ident.unwrap_or(#default_val); }
                                } else {
                                    // Required field (not Option), just dereference
                                    quote! { let #field_ident = *#field_ident; }
                                }
                            }
                            Some(Type::String) => {
                                // DEPYLER-0526: Heuristic for known String fields
                                // File/path fields usually need owned String: convert with .to_string()
                                // Content/pattern fields usually need &str: keep as &String (auto-derefs)
                                let field_lower = field_name.to_lowercase();
                                let owned_indicators = [
                                    "file",
                                    "path",
                                    "filepath",
                                    "input",
                                    "output",
                                    "dir",
                                    "directory",
                                ];
                                let borrowed_indicators =
                                    ["content", "pattern", "text", "message", "data", "value"];

                                let needs_owned = owned_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                });
                                let needs_borrowed = borrowed_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                });

                                if needs_borrowed {
                                    // Keep as &String, auto-derefs to &str
                                    quote! {}
                                } else if needs_owned {
                                    // Convert to owned String
                                    quote! { let #field_ident = #field_ident.to_string(); }
                                } else {
                                    // Default for String: convert to owned (safer for function calls)
                                    quote! { let #field_ident = #field_ident.to_string(); }
                                }
                            }
                            Some(Type::Optional(_))
                            | Some(Type::List(_))
                            | Some(Type::Dict(_, _)) => {
                                // For complex container types, clone the reference
                                quote! { let #field_ident = #field_ident.clone(); }
                            }
                            None => {
                                // Unknown type: use name-based heuristics
                                let field_lower = field_name.to_lowercase();
                                let owned_indicators = [
                                    "file",
                                    "path",
                                    "filepath",
                                    "input",
                                    "output",
                                    "dir",
                                    "directory",
                                ];
                                let borrowed_indicators =
                                    ["content", "pattern", "text", "message", "data", "value"];
                                // DEPYLER-0579: String-like field indicators (should NOT be numeric-unwrapped)
                                let string_indicators = [
                                    "str", "string", "name", "line", "word", "char", "cmd",
                                    "url", "uri", "host", "token", "key", "id", "code",
                                    "hex", "oct", // hex/oct values are string representations
                                ];
                                // DEPYLER-0576: Numeric field indicators (likely f64 with defaults)
                                // DEPYLER-0592: Removed single letters - too ambiguous, often strings
                                let numeric_indicators = [
                                    "x1", "x2", "y1", "y2", "z1", "z2",
                                    "val", "num", "count", "rate", "coef", "factor",
                                    "min", "max", "sum", "avg", "mean", "std",
                                    "width", "height", "size", "len", "length",
                                    "alpha", "beta", "gamma", "theta", "lr",
                                ];

                                let needs_owned = owned_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                });
                                let needs_borrowed = borrowed_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                });
                                // DEPYLER-0579: Check if this looks like a string field
                                let looks_like_string = string_indicators.iter().any(|ind| {
                                    field_lower == *ind
                                        || field_lower.ends_with(ind)
                                        || field_lower.starts_with(ind)
                                        || field_lower.contains(ind)
                                });
                                // Only apply numeric unwrap if NOT string-like
                                let needs_numeric_unwrap = !looks_like_string
                                    && numeric_indicators.iter().any(|ind| {
                                        field_lower == *ind
                                            || field_lower.ends_with(ind)
                                            || field_lower.starts_with(ind)
                                    });

                                if needs_borrowed {
                                    // Keep as reference
                                    quote! {}
                                } else if needs_owned || looks_like_string {
                                    // Convert to owned String
                                    quote! { let #field_ident = #field_ident.to_string(); }
                                } else if needs_numeric_unwrap {
                                    // DEPYLER-0576: Likely numeric Option<f64> field, unwrap with default
                                    quote! { let #field_ident = #field_ident.unwrap_or(0.0); }
                                } else {
                                    // Unknown: keep as reference (safer default)
                                    quote! {}
                                }
                            }
                            _ => {
                                // For other complex types, clone
                                quote! { let #field_ident = #field_ident.clone(); }
                            }
                        }
                    })
                    .collect();

                // DEPYLER-0578: Add `..` to pattern to ignore unmentioned fields (fixes E0027)
                // The subcommand may have more fields than we extract from body statements
                quote! {
                    Commands::#variant_name { #(#ref_field_patterns,)* .. } => {
                        #(#field_bindings)*
                        #(#body_stmts)*
                    }
                }
            }
        })
        .collect();

    // DEPYLER-0456 Bug #3 FIX: Always use "command" as the Rust struct field name
    // The Args struct always has `command: Commands` regardless of Python's dest parameter
    // DEPYLER-0470: Add wildcard arm to make match exhaustive
    // When early returns split matches, not all Commands variants may be in this match
    // Use unreachable!() because split matches ensure mutually exclusive variants
    // DEPYLER-0474: Match by reference to avoid partial move errors
    // When handler functions take &args, we must borrow args.command, not move it
    // DEPYLER-0482: Only add wildcard if no early returns (otherwise execution continues to next match)
    Ok(Some(if has_early_return {
        // Early return present: Don't add wildcard, execution continues to next match
        quote! {
            match &args.command {
                #(#arms)*
                _ => {}
            }
        }
    } else {
        // No early returns: This is likely the final/complete match, add unreachable wildcard
        quote! {
            match &args.command {
                #(#arms)*
                _ => unreachable!("Other command variants handled elsewhere")
            }
        }
    }))
}

/// DEPYLER-0399: Check if expression is a subcommand check pattern
/// DEPYLER-0456 Bug #2: Accept dest_field parameter to support custom field names
///
/// Returns the command name if pattern matches: args.<dest_field> == "string"
fn is_subcommand_check(expr: &HirExpr, dest_field: &str, ctx: &CodeGenContext) -> Option<String> {
    match expr {
        // Direct comparison: args.action == "init"
        HirExpr::Binary {
            op: BinOp::Eq,
            left,
            right,
        } => {
            // DEPYLER-0456 Bug #2: Check if left side is args.<dest_field>
            // (e.g., args.action, args.command, etc.)
            let is_dest_field_attr = matches!(
                left.as_ref(),
                HirExpr::Attribute { attr, .. } if attr == dest_field
            );

            // Check if right side is a string literal
            if is_dest_field_attr {
                if let HirExpr::Literal(Literal::String(cmd_name)) = right.as_ref() {
                    return Some(cmd_name.clone());
                }
            }
            None
        }
        // DEPYLER-0456 Bug #2 FIX: CSE temp variable (e.g., _cse_temp_0)
        // After CSE optimization, the condition becomes just a variable reference
        HirExpr::Var(var_name) => {
            // Look up in CSE subcommand temps map
            ctx.cse_subcommand_temps.get(var_name).cloned()
        }
        _ => None,
    }
}

/// DEPYLER-0399: Convert string to PascalCase for enum variants
fn to_pascal_case_subcommand(s: &str) -> String {
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
            // DEPYLER-0614: Handle Block of statements (for multi-target assignment: i = j = 0)
            HirStmt::Block(stmts) => {
                let mut tokens = proc_macro2::TokenStream::new();
                for stmt in stmts {
                    tokens.extend(stmt.to_rust_tokens(ctx)?);
                }
                Ok(tokens)
            }
            HirStmt::FunctionDef {
                name,
                params,
                ret_type,
                body,
                docstring: _,
            } => codegen_nested_function_def(name, params, ret_type, body, ctx),
        }
    }
}

// ============================================================================
// DEPYLER-0427: Nested Function Code Generation
// ============================================================================

/// Convert HIR Type to proc_macro2::TokenStream for code generation
/// GH-70: Made public for use in func_gen.rs
pub(crate) fn hir_type_to_tokens(ty: &Type, _ctx: &CodeGenContext) -> proc_macro2::TokenStream {
    use quote::quote;

    match ty {
        Type::Int => quote! { i64 },
        Type::Float => quote! { f64 },
        Type::String => quote! { String },
        Type::Bool => quote! { bool },
        Type::None => quote! { () },
        Type::Unknown => quote! { () }, // Default to () for unknown types
        Type::List(elem) => {
            let elem_ty = hir_type_to_tokens(elem, _ctx);
            quote! { Vec<#elem_ty> }
        }
        Type::Dict(key, value) => {
            let key_ty = hir_type_to_tokens(key, _ctx);
            let val_ty = hir_type_to_tokens(value, _ctx);
            quote! { std::collections::HashMap<#key_ty, #val_ty> }
        }
        Type::Tuple(types) => {
            let elem_types: Vec<_> = types.iter().map(|t| hir_type_to_tokens(t, _ctx)).collect();
            quote! { (#(#elem_types),*) }
        }
        Type::Optional(inner) => {
            let inner_ty = hir_type_to_tokens(inner, _ctx);
            quote! { Option<#inner_ty> }
        }
        Type::Custom(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! { #ident }
        }
        _ => quote! { () }, // Fallback for other types (Set, Function, Generic, Union, Array, etc.)
    }
}

/// Generate Rust code for nested function definitions (inner functions)
///
/// Python nested functions are converted to Rust inner functions.
/// This enables code like csv_filter.py and log_analyzer.py to transpile.
///
/// # Examples
///
/// Python:
/// ```python
/// def outer():
///     def inner(x):
///         return x * 2
///     return inner(5)
/// ```
///
/// Rust:
/// ```rust
/// fn outer() -> i64 {
///     fn inner(x: i64) -> i64 {
///         x * 2
///     }
///     inner(5)
/// }
/// ```
fn codegen_nested_function_def(
    name: &str,
    params: &[HirParam],
    ret_type: &Type,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    use quote::quote;

    // Generate function name
    let fn_name = syn::Ident::new(name, proc_macro2::Span::call_site());

    // GH-70: Use inferred parameters from context if available
    let effective_params = ctx
        .nested_function_params
        .get(name)
        .map(|inferred| inferred.as_slice())
        .unwrap_or(params);

    // GH-70: Populate ctx.var_types with inferred param types so that
    // expressions in body (like item[0]) can use proper type info
    // to decide between tuple syntax (.0) and array syntax ([0])
    for param in effective_params {
        ctx.var_types.insert(param.name.clone(), param.ty.clone());
    }

    // Generate parameters
    // DEPYLER-0550: For collection types (Dict, List), use references
    // This is more idiomatic in Rust and works correctly with filter() closures
    let param_tokens: Vec<proc_macro2::TokenStream> = effective_params
        .iter()
        .map(|p| {
            let param_name = syn::Ident::new(&p.name, proc_macro2::Span::call_site());
            let param_type = hir_type_to_tokens(&p.ty, ctx);

            // For collection types, take by reference for idiomatic Rust
            // This is necessary for closures used with filter() which provides &T
            if matches!(p.ty, Type::Dict(_, _) | Type::List(_) | Type::Set(_)) {
                quote! { #param_name: &#param_type }
            } else {
                quote! { #param_name: #param_type }
            }
        })
        .collect();

    // Generate return type
    let return_type = hir_type_to_tokens(ret_type, ctx);

    // DEPYLER-0550: Save and restore can_fail flag for nested closures
    // Nested closures should NOT inherit can_fail from parent function
    // Otherwise return statements get incorrectly wrapped in Ok()
    let saved_can_fail = ctx.current_function_can_fail;
    ctx.current_function_can_fail = false;

    // Generate body
    let body_tokens: Vec<proc_macro2::TokenStream> = body
        .iter()
        .map(|stmt| stmt.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;

    // Restore can_fail flag
    ctx.current_function_can_fail = saved_can_fail;

    // GH-70 FIX: Generate as closure instead of fn item
    // Closures can be returned as values and have better type inference
    // This fixes the issue where nested functions had all types defaulting to ()
    //
    // GH-70 CRITICAL FIX: Omit return type annotation for Type::Unknown
    // When no type annotation exists in Python, ret_type is Type::Unknown.
    // Previously, this defaulted to () in hir_type_to_tokens, causing explicit
    // `-> ()` in closure definition which conflicted with actual return values.
    // Solution: Omit return type entirely, allowing Rust's type inference to
    // determine correct return type from closure body.
    //
    // DEPYLER-0613: Support hoisting - if variable is already declared, use assignment
    let is_declared = ctx.is_declared(name);
    
    Ok(if matches!(ret_type, Type::Unknown) {
        if is_declared {
            quote! {
                #fn_name = |#(#param_tokens),*| {
                    #(#body_tokens)*
                };
            }
        } else {
            quote! {
                let #fn_name = |#(#param_tokens),*| {
                    #(#body_tokens)*
                };
            }
        }
    } else if is_declared {
        quote! {
            #fn_name = |#(#param_tokens),*| -> #return_type {
                #(#body_tokens)*
            };
        }
    } else {
        quote! {
            let #fn_name = |#(#param_tokens),*| -> #return_type {
                #(#body_tokens)*
            };
        }
    })
}
