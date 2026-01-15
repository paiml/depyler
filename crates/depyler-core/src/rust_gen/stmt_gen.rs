//! Statement code generation
//!
//! This module handles converting HIR statements to Rust token streams.
//! It includes all statement conversion helpers and the HirStmt RustCodeGen trait implementation.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, RustCodeGen, ToRustExpr};
use crate::rust_gen::exception_helpers::extract_exception_type;
use crate::rust_gen::expr_analysis::{
    expr_infers_float, expr_produces_depyler_value, is_native_depyler_tuple,
    extract_kwarg_bool, extract_kwarg_string, extract_string_literal,
    get_depyler_extraction_for_type, handler_ends_with_exit, is_dict_augassign_pattern,
    is_dict_index_access, is_dict_with_value_type,
    is_iterator_producing_expr, is_numpy_value_expr,
    is_pure_expression, looks_like_option_expr, needs_type_conversion,
    to_pascal_case,
};
use crate::rust_gen::keywords::safe_ident; // DEPYLER-0023: Keyword escaping
use crate::rust_gen::type_gen::rust_type_to_syn;
use crate::rust_gen::type_conversion_helpers::apply_type_conversion; // DEPYLER-0455: Extracted
use crate::rust_gen::truthiness_helpers::{
    is_collection_attr_name, is_collection_generic_base, is_collection_type_name,
    is_collection_var_name, is_dict_var_name, is_option_var_name, is_string_var_name,
}; // DEPYLER-COVERAGE-95: Use centralized truthiness helpers
use crate::rust_gen::var_analysis::{
    extract_toplevel_assigned_symbols, extract_walrus_from_condition,
    find_var_position_in_tuple, is_var_reassigned_in_stmt,
    is_var_used_as_dict_key_in_stmt, is_var_used_in_stmt,
    needs_boxed_dyn_write,
}; // DEPYLER-0023: Centralized var analysis
use crate::rust_gen::stmt_gen_complex::{try_generate_subcommand_match, is_subcommand_check}; // DEPYLER-COVERAGE-95: Split from stmt_gen
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

// Note: expr_returns_usize, expr_infers_float, is_iterator_producing_expr,
// is_numpy_value_expr, is_pure_expression, looks_like_option_expr, and needs_type_conversion
// are imported from crate::rust_gen::expr_analysis for better testability

// Note: apply_type_conversion is imported from
// crate::rust_gen::type_conversion_helpers for testability (DEPYLER-0455)

// ============================================================================
// Statement Code Generation Helpers (DEPYLER-0140 Phase 1)
// Note: codegen_pass_stmt, codegen_break_stmt, codegen_continue_stmt
// are imported from crate::rust_gen::control_stmt_helpers for testability
// ============================================================================

/// Generate code for Assert statement
/// DEPYLER-1005: Handle binary comparison expressions specially to avoid = = tokenization issue
#[inline]
pub(crate) fn codegen_assert_stmt(
    test: &HirExpr,
    msg: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-1005: For binary comparison expressions, generate assert_eq!/assert_ne! macros
    // to avoid the = = tokenization issue when syn::ExprBinary is interpolated into quote!
    if let HirExpr::Binary { op, left, right } = test {
        match op {
            BinOp::Eq => {
                let left_expr = left.to_rust_expr(ctx)?;
                let right_expr = right.to_rust_expr(ctx)?;
                if let Some(message_expr) = msg {
                    let msg_tokens = message_expr.to_rust_expr(ctx)?;
                    return Ok(quote! { assert_eq!(#left_expr, #right_expr, "{}", #msg_tokens); });
                } else {
                    return Ok(quote! { assert_eq!(#left_expr, #right_expr); });
                }
            }
            BinOp::NotEq => {
                let left_expr = left.to_rust_expr(ctx)?;
                let right_expr = right.to_rust_expr(ctx)?;
                if let Some(message_expr) = msg {
                    let msg_tokens = message_expr.to_rust_expr(ctx)?;
                    return Ok(quote! { assert_ne!(#left_expr, #right_expr, "{}", #msg_tokens); });
                } else {
                    return Ok(quote! { assert_ne!(#left_expr, #right_expr); });
                }
            }
            _ => {} // Fall through to default handling
        }
    }

    let test_expr = test.to_rust_expr(ctx)?;

    if let Some(message_expr) = msg {
        let msg_tokens = message_expr.to_rust_expr(ctx)?;
        Ok(quote! { assert!(#test_expr, "{}", #msg_tokens); })
    } else {
        Ok(quote! { assert!(#test_expr); })
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

                        // DEPYLER-0930: Check for duplicate argument names before adding
                        // This mirrors the fix in argparse_transform.rs for DEPYLER-0929
                        // Without this check, E0416 "identifier bound more than once" occurs
                        if !subcommand_info
                            .arguments
                            .iter()
                            .any(|existing| existing.name == arg.name)
                        {
                            subcommand_info.arguments.push(arg);
                        }
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

    // DEPYLER-0701: Detect expressions without side effects and wrap with `let _ =`
    // to avoid "path statement with no effect" and "unused arithmetic operation" warnings
    if is_pure_expression(expr) {
        let expr_tokens = expr.to_rust_expr(ctx)?;
        Ok(quote! { let _ = #expr_tokens; })
    } else {
        let expr_tokens = expr.to_rust_expr(ctx)?;
        Ok(quote! { #expr_tokens; })
    }
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
        // DEPYLER-1064: Handle Tuple[Any, ...] returns - wrap elements in DepylerValue
        // When return type is Tuple containing Any types, each element needs DepylerValue wrapping
        if let (HirExpr::Tuple(elts), Some(Type::Tuple(elem_types))) =
            (e, ctx.current_return_type.as_ref())
        {
            if ctx.type_mapper.nasa_mode
                && elem_types.iter().any(|t| {
                    matches!(t, Type::Unknown)
                        || matches!(t, Type::Custom(name) if name == "Any" || name == "object")
                })
            {
                // Generate tuple with each element wrapped in DepylerValue
                let wrapped_elems: Vec<proc_macro2::TokenStream> = elts
                    .iter()
                    .map(|elem| {
                        let elem_expr = elem.to_rust_expr(ctx)?;
                        // Wrap based on expression type
                        let wrapped = match elem {
                            HirExpr::Literal(Literal::Int(_)) => {
                                quote! { DepylerValue::Int(#elem_expr as i64) }
                            }
                            HirExpr::Literal(Literal::Float(_)) => {
                                quote! { DepylerValue::Float(#elem_expr) }
                            }
                            HirExpr::Literal(Literal::String(_)) => {
                                quote! { DepylerValue::Str(#elem_expr) }
                            }
                            HirExpr::Literal(Literal::Bool(_)) => {
                                quote! { DepylerValue::Bool(#elem_expr) }
                            }
                            HirExpr::List(_) => {
                                quote! { DepylerValue::List(#elem_expr.into_iter().map(DepylerValue::from).collect()) }
                            }
                            _ => {
                                // For other expressions, try to wrap in from()
                                quote! { DepylerValue::from(#elem_expr) }
                            }
                        };
                        Ok(wrapped)
                    })
                    .collect::<Result<Vec<_>>>()?;

                return Ok(quote! { return (#(#wrapped_elems),*); });
            }
        }

        // DEPYLER-1036: Set current_assign_type for Dict expressions in return statements
        // This ensures empty dicts use the function return type for type inference
        let prev_assign_type = ctx.current_assign_type.take();
        if matches!(e, HirExpr::Dict(_)) {
            if let Some(return_type) = &ctx.current_return_type {
                // Extract Dict type from Optional<Dict> or use directly if Dict
                let dict_type = match return_type {
                    Type::Optional(inner) => match inner.as_ref() {
                        Type::Dict(_, _) => Some(inner.as_ref().clone()),
                        _ => None,
                    },
                    Type::Dict(_, _) => Some(return_type.clone()),
                    _ => None,
                };
                if let Some(dt) = dict_type {
                    ctx.current_assign_type = Some(dt);
                }
            }
        }
        let mut expr_tokens = e.to_rust_expr(ctx)?;
        ctx.current_assign_type = prev_assign_type;

        // DEPYLER-0626: Wrap return value with Box::new() for heterogeneous IO types
        // When function returns Box<dyn Write> (e.g., sys.stdout vs File), wrap the value
        if ctx.function_returns_boxed_write {
            expr_tokens = parse_quote! { Box::new(#expr_tokens) };
        }

        // DEPYLER-1124: Convert concrete type to Union type via .into()
        // When return type is Union[A, B] and expression is concrete A or B,
        // add .into() to let Rust's From trait handle the conversion.
        // Union types generate enum with From impls for each variant.
        let is_union_return = matches!(ctx.current_return_type.as_ref(), Some(Type::Union(_)));
        if is_union_return {
            expr_tokens = parse_quote! { #expr_tokens.into() };
        }

        // DEPYLER-0241: Apply type conversion if needed (e.g., usize -> i32 from enumerate())
        if let Some(return_type) = &ctx.current_return_type {
            // Unwrap Optional to get the underlying type
            let target_type = match return_type {
                Type::Optional(inner) => inner.as_ref(),
                other => other,
            };

            // DEPYLER-0272: Pass expression to check if cast is actually needed
            // DEPYLER-0455 Bug 7: Also pass ctx to check validator function context
            if needs_type_conversion(target_type, e) {
                expr_tokens = apply_type_conversion(expr_tokens, target_type);
            }
        }

        // DEPYLER-0757: Wrap return values when function returns serde_json::Value (Python's `any`)
        // When return type is serde_json::Value, use json!() macro to convert any value
        // Note: ctx.current_return_type contains HIR type (e.g., "any") not the mapped Rust type
        let is_json_value_return = matches!(
            ctx.current_return_type.as_ref(),
            Some(Type::Custom(name)) if name == "serde_json::Value"
                || name == "any"
                || name == "Any"
                || name == "typing.Any"
        );
        // DEPYLER-1017: Skip serde_json in NASA mode
        if is_json_value_return && !ctx.type_mapper.nasa_mode {
            // Use serde_json::json!() macro to convert the expression to Value
            // This handles bool, int, float, string, arrays, etc. automatically
            expr_tokens = parse_quote! { serde_json::json!(#expr_tokens) };
        }

        // DEPYLER-0943: Convert serde_json::Value to String when return type is String
        // Dict subscript access returns serde_json::Value, but if the function return type
        // is String, we need to extract the string value from the JSON Value.
        let is_string_return = matches!(ctx.current_return_type.as_ref(), Some(Type::String));
        let is_dict_subscript = is_dict_index_access(e);
        if is_string_return && is_dict_subscript {
            // Convert Value to String: value.as_str().unwrap_or("").to_string()
            expr_tokens = parse_quote! { #expr_tokens.as_str().unwrap_or("").to_string() };
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

        // DEPYLER-0744: Check if expression is already Option-typed (e.g., param with default=None)
        // DEPYLER-0951: Extended to check method calls that return Option
        // Don't wrap in Some() if the expression is already Option<T>
        let is_already_optional = if let HirExpr::Var(var_name) = e {
            ctx.var_types
                .get(var_name)
                .map(|ty| matches!(ty, Type::Optional(_)))
                .unwrap_or(false)
        } else if let HirExpr::MethodCall { method, args, .. } = e {
            // DEPYLER-0951: These methods return Option<T>, don't wrap in Some()
            // - dict.get(key) -> Option<&V>
            // - environ.get(key) -> Option<String> (via std::env::var().ok())
            // - Result.ok() -> Option<T>
            // - Option.cloned() -> Option<T>
            // DEPYLER-1036: dict.get(key, default) returns value type, NOT Option
            // Python's dict.get with 2 args has built-in default, so it never returns None
            let is_get_with_default = method == "get" && args.len() == 2;
            if is_get_with_default {
                false // dict.get(key, default) returns value, not Option
            } else {
                matches!(
                    method.as_str(),
                    "get" | "ok" | "cloned" | "copied" | "pop" | "first" | "last"
                )
            }
        } else {
            // DEPYLER-0951: Also check if the generated tokens end with .ok() or contain .get(
            // This catches cases where the HIR doesn't directly show the Option-returning method
            let expr_str = quote!(#expr_tokens).to_string();
            // DEPYLER-1036: Check for unwrapping methods that convert Option<T> to T
            // If expression ends with .unwrap_or(...) or .unwrap_or_default(), it's NOT optional
            // DEPYLER-1078: .next() also returns Option<T>
            let has_option_method = expr_str.ends_with(". ok ()")
                || expr_str.ends_with(". next ()")
                || expr_str.contains(". get (");
            let has_unwrap_method = expr_str.contains(". unwrap_or (")
                || expr_str.contains(". unwrap_or_default (")
                || expr_str.contains(". unwrap (")
                || expr_str.contains(". expect (");
            has_option_method && !has_unwrap_method
        };

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
            } else if is_optional_return
                && !is_none_literal
                && !is_if_expr_with_none
                && !is_already_optional
            {
                // Wrap value in Some() for Optional return types
                // DEPYLER-1079: Skip wrapping if if-expr has None arm (handled separately below)
                if use_return_keyword {
                    Ok(quote! { return Ok(Some(#expr_tokens)); })
                } else {
                    Ok(quote! { Ok(Some(#expr_tokens)) })
                }
            } else if is_optional_return && is_if_expr_with_none {
                // DEPYLER-1079: If-expr with None arm in Result context
                // Pattern: `return x if cond else None` -> `Ok(if cond { Some(x) } else { None })`
                if let HirExpr::IfExpr {
                    test,
                    body,
                    orelse: _,
                } = e
                {
                    // DEPYLER-1071: Check if test is an Option variable (regex match result)
                    // Pattern: `return m.group(0) if m else None` where m is Option<Match>
                    // Should generate: `Ok(m.as_ref().map(|m_val| m_val.group(0)))`
                    if let HirExpr::Var(var_name) = test.as_ref() {
                        let is_option = ctx
                            .var_types
                            .get(var_name)
                            .is_some_and(|t| matches!(t, Type::Optional(_)))
                            || is_option_var_name(var_name);

                        if is_option && body_uses_option_var(body, var_name) {
                            let var_ident = crate::rust_gen::keywords::safe_ident(var_name);
                            let val_name = format!("{}_val", var_name);
                            let val_ident = crate::rust_gen::keywords::safe_ident(&val_name);

                            // Substitute variable in body
                            let body_substituted = substitute_var_in_hir(body, var_name, &val_name);
                            let body_tokens = body_substituted.to_rust_expr(ctx)?;

                            if use_return_keyword {
                                return Ok(
                                    quote! { return Ok(#var_ident.as_ref().map(|#val_ident| #body_tokens)); },
                                );
                            } else {
                                return Ok(
                                    quote! { Ok(#var_ident.as_ref().map(|#val_ident| #body_tokens)) },
                                );
                            }
                        }
                    }

                    let test_tokens = test.to_rust_expr(ctx)?;
                    // DEPYLER-1079: Apply truthiness conversion for collection/string/optional conditions
                    let test_tokens =
                        apply_truthiness_conversion(test.as_ref(), test_tokens, ctx);
                    let body_tokens = body.to_rust_expr(ctx)?;

                    if use_return_keyword {
                        Ok(quote! { return Ok(if #test_tokens { Some(#body_tokens) } else { None }); })
                    } else {
                        Ok(quote! { Ok(if #test_tokens { Some(#body_tokens) } else { None }) })
                    }
                } else {
                    unreachable!("is_if_expr_with_none should only match IfExpr")
                }
            } else if is_optional_return && is_already_optional {
                // DEPYLER-0744: Expression is already Option<T>, just wrap in Ok()
                if use_return_keyword {
                    Ok(quote! { return Ok(#expr_tokens); })
                } else {
                    Ok(quote! { Ok(#expr_tokens) })
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
                //
                // DEPYLER-0950: Only apply main() special handling for int/void returns.
                // If main() returns other types like f64, treat it as a normal function.
                let is_main_entry_point_return = matches!(
                    ctx.current_return_type.as_ref(),
                    None | Some(Type::Int) | Some(Type::None)
                );
                if !is_main_entry_point_return {
                    // main() with non-int/non-void return type (e.g., `def main() -> float`)
                    // Treat as normal function - generate standard return with Ok() wrapper
                    if use_return_keyword {
                        Ok(quote! { return Ok(#expr_tokens); })
                    } else {
                        Ok(quote! { Ok(#expr_tokens) })
                    }
                } else if let HirExpr::Literal(Literal::Int(exit_code)) = e {
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
                    // DEPYLER-0703: Other expressions in main - evaluate for side effects
                    // and return Ok(()). Use explicit semicolon to prevent DEPYLER-0694 wrap.
                    if use_return_keyword {
                        Ok(quote! { let _ = #expr_tokens; return Ok(()); })
                    } else {
                        Ok(quote! { let _ = #expr_tokens; Ok(()) })
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
        } else if is_optional_return && !is_none_literal && !is_if_expr_with_none && !is_already_optional {
            // Wrap value in Some() for Optional return types
            // DEPYLER-0498: Skip wrapping if if-expr has None arm (handled separately)
            // DEPYLER-0744: Skip wrapping if expression is already Option<T>
            if use_return_keyword {
                Ok(quote! { return Some(#expr_tokens); })
            } else {
                Ok(quote! { Some(#expr_tokens) })
            }
        } else if is_optional_return && is_already_optional {
            // DEPYLER-0744: Expression is already Option<T>, don't double-wrap
            if use_return_keyword {
                Ok(quote! { return #expr_tokens; })
            } else {
                Ok(quote! { #expr_tokens })
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
                // DEPYLER-1071: Check if test is an Option variable (regex match result)
                // Pattern: `return m.group(0) if m else None` where m is Option<Match>
                // Should generate: `m.as_ref().map(|m_val| m_val.group(0))`
                if let HirExpr::Var(var_name) = test.as_ref() {
                    let is_option = ctx
                        .var_types
                        .get(var_name)
                        .is_some_and(|t| matches!(t, Type::Optional(_)))
                        || is_option_var_name(var_name);

                    if is_option && body_uses_option_var(body, var_name) {
                        let var_ident = crate::rust_gen::keywords::safe_ident(var_name);
                        let val_name = format!("{}_val", var_name);
                        let val_ident = crate::rust_gen::keywords::safe_ident(&val_name);

                        // Substitute variable in body
                        let body_substituted = substitute_var_in_hir(body, var_name, &val_name);
                        let body_tokens = body_substituted.to_rust_expr(ctx)?;

                        if use_return_keyword {
                            return Ok(
                                quote! { return #var_ident.as_ref().map(|#val_ident| #body_tokens); },
                            );
                        } else {
                            return Ok(quote! { #var_ident.as_ref().map(|#val_ident| #body_tokens) });
                        }
                    }
                }

                let test_tokens = test.to_rust_expr(ctx)?;
                // DEPYLER-1079: Apply truthiness conversion for collection/string/optional conditions
                let test_tokens =
                    apply_truthiness_conversion(test.as_ref(), test_tokens, ctx);
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
        } else if ctx.is_main_function {
            // DEPYLER-0617: Handle exit code returns in main() function (non-fallible case)
            // Python pattern: `def main() -> int: ... return 0`
            // Rust main() can only return () or Result<(), E>, so integer returns
            // must be converted to process::exit() for non-zero or () for zero
            //
            // DEPYLER-0950: Only apply main() special handling for int/void returns.
            // If main() returns other types like f64, treat it as a normal function.
            let is_main_entry_point_return = matches!(
                ctx.current_return_type.as_ref(),
                None | Some(Type::Int) | Some(Type::None)
            );
            if !is_main_entry_point_return {
                // main() with non-int/non-void return type (e.g., `def main() -> float`)
                // Treat as normal function - generate standard return
                if use_return_keyword {
                    Ok(quote! { return #expr_tokens; })
                } else {
                    Ok(quote! { #expr_tokens })
                }
            } else if let HirExpr::Literal(Literal::Int(exit_code)) = e {
                if *exit_code == 0 {
                    // Success exit code -> ()
                    if use_return_keyword {
                        Ok(quote! { return; })
                    } else {
                        Ok(quote! { () })
                    }
                } else {
                    // Non-zero exit code -> std::process::exit(N)
                    let code = *exit_code as i32;
                    Ok(quote! { std::process::exit(#code) })
                }
            } else {
                // DEPYLER-0703: Other expressions in main - evaluate for side effects
                // and return (). Use explicit (); to prevent DEPYLER-0694 from wrapping again.
                if use_return_keyword {
                    Ok(quote! { let _ = #expr_tokens; return; })
                } else {
                    // Note: Use explicit statement with semicolon to prevent
                    // DEPYLER-0694 from adding another let _ =
                    Ok(quote! { let _ = #expr_tokens; })
                }
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
/// DEPYLER-0698: Converts `while True:` to `loop {}` for Rust idiom
#[inline]
pub(crate) fn codegen_while_stmt(
    condition: &HirExpr,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0791: Save and restore is_final_statement for loop body
    // Return statements inside loops are always early exits, never final expressions
    // Without this, `return count` inside `if` inside `loop` gets generated as just `count`
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

    // DEPYLER-0698: Convert `while True:` to `loop {}` for idiomatic Rust
    // Rust warns: "denote infinite loops with `loop { ... }`"
    if matches!(condition, HirExpr::Literal(Literal::Bool(true))) {
        ctx.enter_scope();
        let body_stmts: Vec<_> = body
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();
        ctx.is_final_statement = saved_is_final;
        return Ok(quote! {
            loop {
                #(#body_stmts)*
            }
        });
    }

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
    ctx.is_final_statement = saved_is_final;
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
        // GH-204: Added SyntaxError, TypeError, KeyError, IOError, AttributeError, StopIteration
        match exception_type.as_str() {
            "ValueError" => ctx.needs_valueerror = true,
            "ArgumentTypeError" => ctx.needs_argumenttypeerror = true,
            "ZeroDivisionError" => ctx.needs_zerodivisionerror = true,
            "IndexError" => ctx.needs_indexerror = true,
            "RuntimeError" => ctx.needs_runtimeerror = true,
            "FileNotFoundError" => ctx.needs_filenotfounderror = true,
            "SyntaxError" => ctx.needs_syntaxerror = true,
            "TypeError" => ctx.needs_typeerror = true,
            "KeyError" => ctx.needs_keyerror = true,
            "IOError" => ctx.needs_ioerror = true,
            "AttributeError" => ctx.needs_attributeerror = true,
            "StopIteration" => ctx.needs_stopiteration = true,
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
                // GH-204: Added ZeroDivisionError, SyntaxError, IOError, AttributeError, StopIteration
                if !is_already_wrapped
                    && (exception_type == "ValueError"
                        || exception_type == "ArgumentTypeError"
                        || exception_type == "TypeError"
                        || exception_type == "KeyError"
                        || exception_type == "IndexError"
                        || exception_type == "RuntimeError"
                        || exception_type == "FileNotFoundError"
                        || exception_type == "ZeroDivisionError"
                        || exception_type == "SyntaxError"
                        || exception_type == "IOError"
                        || exception_type == "AttributeError"
                        || exception_type == "StopIteration")
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
                // GH-204: Added ZeroDivisionError, SyntaxError, IOError, AttributeError, StopIteration
                if !is_already_wrapped
                    && (exception_type == "ValueError"
                        || exception_type == "ArgumentTypeError"
                        || exception_type == "TypeError"
                        || exception_type == "KeyError"
                        || exception_type == "IndexError"
                        || exception_type == "RuntimeError"
                        || exception_type == "FileNotFoundError"
                        || exception_type == "ZeroDivisionError"
                        || exception_type == "SyntaxError"
                        || exception_type == "IOError"
                        || exception_type == "AttributeError"
                        || exception_type == "StopIteration")
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

// Note: extract_exception_type is imported from crate::rust_gen::exception_helpers

/// Generate code for With (context manager) statement
/// DEPYLER-0188: Now supports async with statements (is_async flag)
#[inline]
pub(crate) fn codegen_with_stmt(
    context: &HirExpr,
    target: &Option<String>,
    body: &[HirStmt],
    is_async: bool,
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
        } else if is_async {
            // DEPYLER-0188: For async context managers, call __aenter__().await
            Ok(quote! {
                let mut _context = #context_expr;
                let #var_ident = _context.__aenter__().await;
                #(#body_stmts)*
                // Note: __aexit__().await should be called here (pending Drop trait async support)
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
        if is_async {
            Ok(quote! {
                let mut _context = #context_expr;
                let _ = _context.__aenter__().await;
                #(#body_stmts)*
            })
        } else {
            Ok(quote! {
                let mut _context = #context_expr;
                #(#body_stmts)*
            })
        }
    }
}

// ============================================================================
// Statement Code Generation Helpers (DEPYLER-0140 Phase 3)
// Complex handlers extracted from HirStmt::to_rust_tokens
// ============================================================================

/// DEPYLER-0966: Apply NEGATED truthiness conversion for `not x` patterns
///
/// When Python code uses `if not x:` where `x` is a collection/optional/numeric,
/// we need to generate the INVERTED truthiness check:
/// - `not collection` → `collection.is_empty()` (not `!collection.is_empty()`)
/// - `not optional` → `optional.is_none()` (not `!optional.is_some()`)
/// - `not number` → `number == 0` (not `number != 0`)
///
/// This prevents double-negation and generates cleaner Rust code.
fn apply_negated_truthiness(
    operand: &HirExpr,
    cond_expr: syn::Expr,
    ctx: &CodeGenContext,
) -> syn::Expr {
    // DEPYLER-1027: Check if cond_expr is already a truthiness-converted method call
    // to_rust_expr for UnaryOp::Not may have already converted `not x` to `x.is_empty()`
    // or `x.is_none()`. In that case, return it directly to avoid double conversion.
    if let syn::Expr::MethodCall(method_call) = &cond_expr {
        let method_name = method_call.method.to_string();
        if method_name == "is_empty" || method_name == "is_none" {
            // Already converted by to_rust_expr - return as-is
            return cond_expr;
        }
    }

    // Extract the inner expression (strip the `!` from the already-converted cond_expr)
    // The cond_expr is already `!inner_expr` from to_rust_expr, so we need the inner part
    let inner_expr = if let syn::Expr::Unary(syn::ExprUnary {
        op: syn::UnOp::Not(_),
        expr,
        ..
    }) = &cond_expr
    {
        expr.as_ref().clone()
    } else {
        // Fallback: use the whole expression
        cond_expr.clone()
    };

    // Helper to get the type for type-based truthiness
    let get_type_for_operand = |op: &HirExpr| -> Option<Type> {
        match op {
            HirExpr::Var(var_name) => ctx.var_types.get(var_name).cloned(),
            HirExpr::Attribute { value, attr } => {
                if let HirExpr::Var(obj_name) = value.as_ref() {
                    if obj_name == "self" {
                        return ctx.class_field_types.get(attr).cloned();
                    }
                }
                None
            }
            _ => None,
        }
    };

    // Get the type of the inner operand
    if let Some(operand_type) = get_type_for_operand(operand) {
        return match operand_type {
            // Bool: keep the negation as-is
            Type::Bool => cond_expr,

            // String/List/Dict/Set: `not x` → `x.is_empty()` (inverted truthiness)
            Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                parse_quote! { #inner_expr.is_empty() }
            }

            // Optional: `not x` → `x.is_none()` (inverted truthiness)
            Type::Optional(_) => {
                parse_quote! { #inner_expr.is_none() }
            }

            // Numeric: `not x` → `x == 0` (inverted truthiness)
            Type::Int => {
                parse_quote! { #inner_expr == 0 }
            }
            Type::Float => {
                parse_quote! { #inner_expr == 0.0 }
            }

            // Unknown or other types: keep the negation
            _ => cond_expr,
        };
    }

    // DEPYLER-0966: Heuristic for self.* fields with common list/collection names
    // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
    if let HirExpr::Attribute { value, attr } = operand {
        if let HirExpr::Var(obj_name) = value.as_ref() {
            if obj_name == "self" && is_collection_attr_name(attr) {
                return parse_quote! { #inner_expr.is_empty() };
            }
        }
    }

    // Fallback: keep the original expression (including the negation)
    cond_expr
}

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
    // DEPYLER-0966: Handle negated truthiness: `if not x:` where x is non-boolean
    // For `not collection`, we should generate `collection.is_empty()` (no double negation)
    // For `not optional`, we should generate `optional.is_none()`
    if let HirExpr::Unary {
        op: UnaryOp::Not,
        operand,
    } = condition
    {
        // Get the type of the inner operand and generate inverted truthiness
        return apply_negated_truthiness(operand, cond_expr, ctx);
    }

    // Check if this is a variable reference that needs truthiness conversion
    if let HirExpr::Var(var_name) = condition {
        // DEPYLER-0969: Check if type is tracked and handle known types
        if let Some(var_type) = ctx.var_types.get(var_name) {
            match var_type {
                // Already boolean - no conversion needed
                Type::Bool => return cond_expr,

                // String/List/Dict/Set - check if empty
                Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                    return parse_quote! { !#cond_expr.is_empty() };
                }

                // Optional - check if Some
                Type::Optional(_) => {
                    return parse_quote! { #cond_expr.is_some() };
                }

                // Numeric types - check if non-zero
                Type::Int => {
                    return parse_quote! { #cond_expr != 0 };
                }
                Type::Float => {
                    return parse_quote! { #cond_expr != 0.0 };
                }

                // DEPYLER-0969: Custom types that are collections (VecDeque, BinaryHeap, etc.)
                // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
                Type::Custom(type_name) => {
                    if is_collection_type_name(type_name) {
                        return parse_quote! { !#cond_expr.is_empty() };
                    }
                    // Fall through to heuristics for non-collection custom types
                }

                // DEPYLER-0969: Generic types that are collections
                // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
                Type::Generic { base, .. } => {
                    if is_collection_generic_base(base) {
                        return parse_quote! { !#cond_expr.is_empty() };
                    }
                    // Fall through to heuristics for non-collection generic types
                }

                // Unknown - fall through to heuristics below
                Type::Unknown => {
                    // Don't return - let heuristics handle it
                }

                // Other concrete types - fall through to heuristics
                _ => {}
            }
        }

        // DEPYLER-0517: Heuristic fallback for common string variable names
        // This handles variables from tuple unpacking that aren't tracked in var_types
        // e.g., `let (returncode, stdout, stderr) = run_command(...)`
        // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
        if is_string_var_name(var_name) {
            return parse_quote! { !#cond_expr.is_empty() };
        }

        // DEPYLER-0969: Heuristic fallback for common collection variable names
        // This is the ARCHITECTURAL FIX for truthiness - handles untracked collection types
        // Pattern: `while queue:` where queue is VecDeque/Vec/etc not in var_types
        // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
        if is_collection_var_name(var_name) {
            return parse_quote! { !#cond_expr.is_empty() };
        }

        // DEPYLER-1071: Heuristic fallback for common Option variable names
        // This handles regex match results and other optional values
        // Pattern: `if m:` where m is a regex match result (Option<Match>)
        if is_option_var_name(var_name) {
            return parse_quote! { #cond_expr.is_some() };
        }
    }

    // DEPYLER-0570: Handle dict index access in conditions
    // Python: `if info["extension"]:` checks if the value is truthy (non-empty string)
    // Rust: info.get("extension")... returns serde_json::Value, need to check truthiness
    // Convert to: `.as_str().is_some_and(|s| !s.is_empty())` for string values
    if let HirExpr::Index { base, index } = condition {
        // Check if using string key (dict-like access)
        let has_string_key = matches!(index.as_ref(), HirExpr::Literal(Literal::String(_)));

        // Check if base is a dict (HashMap) or common dict variable name
        // DEPYLER-COVERAGE-95: Centralized in truthiness_helpers
        let is_dict_access = if let HirExpr::Var(var_name) = base.as_ref() {
            // Known dict type
            if let Some(var_type) = ctx.var_types.get(var_name) {
                matches!(var_type, Type::Dict(_, _))
            } else {
                // Unknown type - use string key OR common dict variable names as heuristics
                has_string_key || is_dict_var_name(var_name)
            }
        } else {
            // Nested access or other expression - use string key as heuristic
            has_string_key
        };

        if is_dict_access {
            // Dict value access - check if the Value is truthy
            // serde_json::Value truthiness: string must be non-empty
            return parse_quote! { #cond_expr.as_str().is_some_and(|s| !s.is_empty()) };
        }
    }

    // DEPYLER-0904: Handle self.* attribute access for class fields
    // Python: if not self.heap (where heap is a list)
    // Rust: if self.heap.is_empty() (Vec truthiness = non-empty)
    if let HirExpr::Attribute { value, attr } = condition {
        if let HirExpr::Var(obj_name) = value.as_ref() {
            // Check for self.* access and use class_field_types
            if obj_name == "self" {
                if let Some(field_type) = ctx.class_field_types.get(attr) {
                    return match field_type {
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

                        // Unknown or other types - use as-is
                        _ => cond_expr,
                    };
                }

                // DEPYLER-0950: Fallback heuristic for self.* when field type is unknown
                // Common String field names that need !.is_empty() check
                let string_attr_names = [
                    "email", "name", "text", "content", "message", "title", "description",
                    "path", "url", "value", "data", "body", "subject", "address", "filename",
                    "username", "password", "token", "key", "secret", "label", "output",
                    "input", "stdout", "stderr", "error", "warning", "info", "debug",
                ];
                if string_attr_names.contains(&attr.as_str()) {
                    return parse_quote! { !#cond_expr.is_empty() };
                }
            }

            // Check if this is accessing an args variable from ArgumentParser
            let is_args_var = ctx.argparser_tracker.parsers.values().any(|parser_info| {
                parser_info
                    .args_var
                    .as_ref()
                    .is_some_and(|args_var| args_var == obj_name)
            });

            if is_args_var {
                // Check if this field is optional (Option<T> type, not boolean)
                // DEPYLER-0722: Check both main parsers AND subcommands
                // Helper closure to check if an argument is optional
                let check_optional = |arg: &super::argparse_transform::ArgParserArgument| -> bool {
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
                    // DEPYLER-0678: Exclude nargs='+' and nargs='*' which are Vec, not Option
                    !arg.is_positional
                        && !arg.required.unwrap_or(false)
                        && arg.default.is_none()
                        && !matches!(arg.nargs.as_deref(), Some("+") | Some("*"))
                };

                // Check main parsers
                let is_optional_in_parser = ctx.argparser_tracker.parsers.values().any(|parser_info| {
                    parser_info.arguments.iter().any(&check_optional)
                });

                // DEPYLER-0722: Also check subcommands for optional fields
                let is_optional_in_subcommand = ctx.argparser_tracker.subcommands.values().any(|subcommand_info| {
                    subcommand_info.arguments.iter().any(&check_optional)
                });

                let is_optional_field = is_optional_in_parser || is_optional_in_subcommand;

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

                // DEPYLER-0678: Check if this field is a Vec from nargs='+' or nargs='*'
                // Python: if args.files (where files has nargs='+')
                // Rust: if !args.files.is_empty() (Vec truthiness = non-empty)
                let is_vec_field = ctx.argparser_tracker.parsers.values().any(|parser_info| {
                    parser_info.arguments.iter().any(|arg| {
                        let field_name = arg.rust_field_name();
                        if field_name != *attr {
                            return false;
                        }
                        // nargs='+' or nargs='*' creates Vec<T>
                        matches!(arg.nargs.as_deref(), Some("+") | Some("*"))
                    })
                });

                if is_vec_field {
                    // Convert Vec<T> to boolean using !.is_empty()
                    return parse_quote! { !#cond_expr.is_empty() };
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

            // DEPYLER-0950: Heuristic for common String attribute names
            // Pattern: if obj.email, obj.name, obj.text, etc.
            // These are typically String fields that need !.is_empty() check
            let string_attr_names = [
                "email", "name", "text", "content", "message", "title", "description",
                "path", "url", "value", "data", "body", "subject", "address", "filename",
                "username", "password", "token", "key", "secret", "label", "output",
                "input", "stdout", "stderr", "error", "warning", "info", "debug",
            ];
            if string_attr_names.contains(&attr.as_str()) {
                return parse_quote! { !#cond_expr.is_empty() };
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
        return parse_quote! { #cond_expr.as_str().is_some_and(|s| !s.is_empty()) };
    }

    // Not a variable or no type info - use as-is
    cond_expr
}

/// DEPYLER-1071: Check if body expression uses the Option variable in a method call
/// Pattern: `m.group(0)` where m is the Option variable
fn body_uses_option_var(body: &HirExpr, var_name: &str) -> bool {
    match body {
        // Direct method call on the variable: m.group(0)
        HirExpr::MethodCall { object, .. } => {
            if let HirExpr::Var(obj_name) = object.as_ref() {
                return obj_name == var_name;
            }
            body_uses_option_var(object, var_name)
        }
        // Attribute access on the variable
        HirExpr::Attribute { value, .. } => {
            if let HirExpr::Var(obj_name) = value.as_ref() {
                return obj_name == var_name;
            }
            body_uses_option_var(value, var_name)
        }
        // Variable used directly
        HirExpr::Var(name) => name == var_name,
        _ => false,
    }
}

/// DEPYLER-1071: Substitute a variable name in a HIR expression
fn substitute_var_in_hir(expr: &HirExpr, old_name: &str, new_name: &str) -> HirExpr {
    match expr {
        HirExpr::Var(name) if name == old_name => HirExpr::Var(new_name.to_string()),
        HirExpr::MethodCall {
            object,
            method,
            args,
            kwargs,
        } => HirExpr::MethodCall {
            object: Box::new(substitute_var_in_hir(object, old_name, new_name)),
            method: method.clone(),
            args: args
                .iter()
                .map(|a| substitute_var_in_hir(a, old_name, new_name))
                .collect(),
            kwargs: kwargs
                .iter()
                .map(|(k, v)| (k.clone(), substitute_var_in_hir(v, old_name, new_name)))
                .collect(),
        },
        HirExpr::Attribute { value, attr } => HirExpr::Attribute {
            value: Box::new(substitute_var_in_hir(value, old_name, new_name)),
            attr: attr.clone(),
        },
        HirExpr::Call { func, args, kwargs } => HirExpr::Call {
            func: func.clone(),
            args: args
                .iter()
                .map(|a| substitute_var_in_hir(a, old_name, new_name))
                .collect(),
            kwargs: kwargs
                .iter()
                .map(|(k, v)| (k.clone(), substitute_var_in_hir(v, old_name, new_name)))
                .collect(),
        },
        // For other expression types, return as-is
        _ => expr.clone(),
    }
}

// DEPYLER-0379: Extract all simple symbol assignments from a statement block
// Returns a set of variable names that are assigned (not reassigned) in the block.
// Only captures simple symbol assignments like `x = value`, not `x[i] = value` or `x.attr = value`.
// Complexity: 4 (recursive traversal with set operations)
//
// DEPYLER-0023: Symbol extraction and walrus operator functions (extract_assigned_symbols,
// extract_toplevel_assigned_symbols, extract_walrus_from_condition, extract_walrus_recursive)
// imported from var_analysis module

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

    // DEPYLER-0627: Detect Option variable truthiness check and generate if-let pattern
    // Pattern: `if option_var:` -> `if let Some(ref val) = option_var { ... }`
    if let HirExpr::Var(var_name) = condition {
        if let Some(var_type) = ctx.var_types.get(var_name) {
            if matches!(var_type, Type::Optional(_)) {
                return codegen_option_if_let(var_name, then_body, else_body, ctx);
            }
        }
    }

    // DEPYLER-0188: Extract walrus operator assignments from condition
    // Python: if (n := len(text)) > 5: -> Rust: let n = text.len(); if n > 5 {
    let (walrus_assigns, simplified_condition) = extract_walrus_from_condition(condition);

    // Generate let statements for walrus operator assignments
    let mut walrus_lets = Vec::new();
    for (name, value_expr) in &walrus_assigns {
        let var_ident = safe_ident(name);
        let value_tokens = value_expr.to_rust_expr(ctx)?;
        walrus_lets.push(quote! { let #var_ident = #value_tokens; });
        // Register the variable in context so it's available in the if body
        ctx.declare_var(name);
    }

    // DEPYLER-0844: isinstance(x, T) → true (Rust's type system guarantees correctness)
    // Must intercept BEFORE to_rust_expr to avoid generating `isinstance(...)` call
    let mut cond = if let HirExpr::Call { func, .. } = &simplified_condition {
        if func == "isinstance" {
            parse_quote! { true }
        } else {
            simplified_condition.to_rust_expr(ctx)?
        }
    } else {
        simplified_condition.to_rust_expr(ctx)?
    };

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
    // DEPYLER-0823: Also hoist None-placeholder variables assigned in any branch
    let then_vars = extract_toplevel_assigned_symbols(then_body);
    let mut hoisted_vars: HashSet<String> = if let Some(else_stmts) = else_body {
        let else_vars = extract_toplevel_assigned_symbols(else_stmts);
        then_vars.intersection(&else_vars).cloned().collect()
    } else {
        HashSet::new()
    };

    // DEPYLER-0823: Add None-placeholder variables that are assigned in any branch
    // These variables had `var = None` skipped, but need to be declared before the if
    // so they're accessible after it (e.g., `if cond: var = x; use(var)`)
    for var_name in &then_vars {
        if ctx.none_placeholder_vars.contains(var_name) {
            hoisted_vars.insert(var_name.clone());
        }
    }
    if let Some(else_stmts) = else_body {
        let else_vars = extract_toplevel_assigned_symbols(else_stmts);
        for var_name in &else_vars {
            if ctx.none_placeholder_vars.contains(var_name) {
                hoisted_vars.insert(var_name.clone());
            }
        }
    }

    // DEPYLER-0379: Generate hoisted variable declarations
    // DEPYLER-0439: Skip variables already declared in parent scope (prevents shadowing)
    let mut hoisted_decls = Vec::new();
    for var_name in &hoisted_vars {
        // DEPYLER-0439: Skip if variable is already declared in parent scope
        let already_declared = ctx.is_declared(var_name);
        if already_declared {
            continue;
        }

        let var_ident = safe_ident(var_name); // DEPYLER-0023

        // DEPYLER-0625: Check if variable needs Box<dyn Write> due to heterogeneous IO types
        // (e.g., File in one branch, sys.stdout in another)
        if let Some(else_stmts) = else_body {
            if needs_boxed_dyn_write(var_name, then_body, else_stmts) {
                // Generate Box<dyn Write> type for heterogeneous IO
                hoisted_decls.push(quote! { let mut #var_ident: Box<dyn std::io::Write>; });
                ctx.boxed_dyn_write_vars.insert(var_name.clone());
                ctx.declare_var(var_name);
                continue;
            }
        }

        // Find the variable's type from the first assignment in either branch
        let var_type = find_variable_type(var_name, then_body).or_else(|| {
            if let Some(else_stmts) = else_body {
                find_variable_type(var_name, else_stmts)
            } else {
                None
            }
        });

        if let Some(ty) = var_type {
            let rust_type = ctx.type_mapper.map_type(&ty);
            let syn_type = rust_type_to_syn(&rust_type)?;
            // DEPYLER-0823: For if-only (no else), initialize with Default to prevent E0381
            // Rust requires initialization since the branch may not be taken
            if else_body.is_none() {
                hoisted_decls.push(quote! { let mut #var_ident: #syn_type = Default::default(); });
            } else {
                hoisted_decls.push(quote! { let mut #var_ident: #syn_type; });
            }
        } else {
            // No type annotation - use type inference placeholder
            // Rust will infer the type from the assignments in the branches
            // DEPYLER-0823: For if-only (no else), we need a typed default
            // Since we don't know the type, use String as common case for None placeholders
            if else_body.is_none() && ctx.none_placeholder_vars.contains(var_name) {
                hoisted_decls.push(quote! { let mut #var_ident: String = Default::default(); });
            } else {
                hoisted_decls.push(quote! { let mut #var_ident; });
            }

            // DEPYLER-0455 Bug 2: Track hoisted variables needing String normalization
            // When a variable is hoisted without type annotation, we need to normalize
            // string literals to String to avoid &str vs String type mismatches
            ctx.hoisted_inference_vars.insert(var_name.clone());
        }

        // Mark variable as declared so assignments use `var = value` not `let var = value`
        ctx.declare_var(var_name);
    }

    // DEPYLER-0935: Save and restore is_final_statement for if body
    // Return statements inside if bodies are NOT final statements of the function
    // because there may be code after the if statement
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

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
            #(#walrus_lets)*
            #(#hoisted_decls)*
            if #cond {
                #(#then_stmts)*
            } else {
                #(#else_tokens)*
            }
        })
    } else {
        Ok(quote! {
            #(#walrus_lets)*
            #(#hoisted_decls)*
            if #cond {
                #(#then_stmts)*
            }
        })
    };

    // DEPYLER-0935: Restore is_final_statement flag
    ctx.is_final_statement = saved_is_final;

    // DEPYLER-0455 Bug 2: Clean up hoisted inference vars after if-statement
    // Remove variables from tracking set since they're only relevant within this if-statement
    for var_name in &hoisted_vars {
        ctx.hoisted_inference_vars.remove(var_name);
    }

    result
}

/// DEPYLER-0627: Generate if-let pattern for Option variable truthiness check
///
/// Pattern: `if option_var:` -> `if let Some(ref val) = option_var { ... }`
///
/// This fixes issues where:
/// - Python: `if override: return override`
/// - Wrong Rust: `if override.is_some() { return override.to_string(); }`
/// - Correct Rust: `if let Some(ref val) = override { return val.clone(); }`
///
/// # Complexity
/// 3 (body processing with scope management)
fn codegen_option_if_let(
    var_name: &str,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Generate the variable identifier (handle Rust keywords)
    let var_ident = safe_ident(var_name);
    // Generate the unwrapped variable name
    let unwrapped_name = format!("{}_val", var_name);
    let unwrapped_ident = safe_ident(&unwrapped_name);

    // DEPYLER-0645: Inside generators, state variables need self. prefix
    let var_expr: proc_macro2::TokenStream =
        if ctx.in_generator && ctx.generator_state_vars.contains(var_name) {
            quote! { self.#var_ident }
        } else {
            quote! { #var_ident }
        };

    // Add mapping so variable references inside body use unwrapped name
    ctx.option_unwrap_map
        .insert(var_name.to_string(), unwrapped_name.clone());

    // Process then body with the mapping active
    ctx.enter_scope();
    let then_stmts: Vec<_> = then_body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    // Remove the mapping
    ctx.option_unwrap_map.remove(var_name);

    // Generate if-let pattern
    let result = if let Some(else_stmts) = else_body {
        ctx.enter_scope();
        let else_tokens: Vec<_> = else_stmts
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();

        quote! {
            if let Some(ref #unwrapped_ident) = #var_expr {
                #(#then_stmts)*
            } else {
                #(#else_tokens)*
            }
        }
    } else {
        quote! {
            if let Some(ref #unwrapped_ident) = #var_expr {
                #(#then_stmts)*
            }
        }
    };

    Ok(result)
}

/// DEPYLER-0379: Find the type annotation for a variable in a statement block
///
/// Searches for the first Assign statement that assigns to the given variable
/// and returns its type annotation if present, or infers from the value expression.
///
/// DEPYLER-0823: Enhanced to infer type from value when no annotation exists.
/// This fixes the None-placeholder hoisting bug where `value = None; if cond: value = 42`
/// was incorrectly typed as String instead of i64.
///
/// # Complexity
/// 3 (linear search with recursive check)
pub(crate) fn find_variable_type(var_name: &str, stmts: &[HirStmt]) -> Option<Type> {
    for stmt in stmts {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                type_annotation,
                value,
            } if name == var_name => {
                // First try explicit type annotation
                if type_annotation.is_some() {
                    return type_annotation.clone();
                }
                // DEPYLER-0823: If no annotation, infer from the assigned value
                let inferred = crate::rust_gen::func_gen::infer_expr_type_simple(value);
                if !matches!(inferred, Type::Unknown) {
                    return Some(inferred);
                }
                return None;
            }
            // DEPYLER-0931: Handle tuple unpacking (a, b, c) = (1, 2, 3)
            // Extract individual element types from the tuple
            HirStmt::Assign {
                target: AssignTarget::Tuple(targets),
                value,
                ..
            } => {
                // Find var_name position in the targets
                if let Some(pos) = find_var_position_in_tuple(var_name, targets) {
                    // Infer type from the RHS tuple
                    let rhs_type = crate::rust_gen::func_gen::infer_expr_type_simple(value);
                    if let Type::Tuple(elem_types) = rhs_type {
                        if pos < elem_types.len() {
                            let elem_type = elem_types[pos].clone();
                            if !matches!(elem_type, Type::Unknown) {
                                return Some(elem_type);
                            }
                        }
                    }
                    // If RHS is not a tuple, check if it's a literal tuple expression
                    if let HirExpr::Tuple(elems) = value {
                        if pos < elems.len() {
                            let elem_type =
                                crate::rust_gen::func_gen::infer_expr_type_simple(&elems[pos]);
                            if !matches!(elem_type, Type::Unknown) {
                                return Some(elem_type);
                            }
                        }
                    }
                }
            }
            // DEPYLER-0931: Recursively search nested try/except blocks
            HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => {
                // Search in try body
                if let Some(ty) = find_variable_type(var_name, body) {
                    return Some(ty);
                }
                // Search in handlers
                for handler in handlers {
                    if let Some(ty) = find_variable_type(var_name, &handler.body) {
                        return Some(ty);
                    }
                }
                // Search in finally
                if let Some(finally) = finalbody {
                    if let Some(ty) = find_variable_type(var_name, finally) {
                        return Some(ty);
                    }
                }
            }
            // DEPYLER-0931: Recursively search if/else blocks
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                if let Some(ty) = find_variable_type(var_name, then_body) {
                    return Some(ty);
                }
                if let Some(else_stmts) = else_body {
                    if let Some(ty) = find_variable_type(var_name, else_stmts) {
                        return Some(ty);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

// DEPYLER-COVERAGE-95: find_var_position_in_tuple, find_assigned_expr, needs_boxed_dyn_write moved to var_analysis module
// DEPYLER-COVERAGE-95: is_file_creating_expr, is_stdio_expr, is_dict_index_access moved to expr_analysis module

// DEPYLER-0023: is_var_used_in_expr and is_var_used_in_assign_target imported from var_analysis

// DEPYLER-0023: Variable analysis functions (is_var_used_as_dict_key_in_expr, is_var_direct_or_simple_in_expr,
// is_var_used_as_dict_key_in_stmt, is_var_reassigned_in_stmt, is_var_used_in_stmt) imported from var_analysis module

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

/// Generate code for For loop statement
#[inline]
pub(crate) fn codegen_for_stmt(
    target: &AssignTarget,
    iter: &HirExpr,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0791: Save and restore is_final_statement for loop body
    // Return statements inside loops are always early exits, never final expressions
    let saved_is_final = ctx.is_final_statement;
    ctx.is_final_statement = false;

    // CITL: Trace for loop iteration strategy
    trace_decision!(
        category = DecisionCategory::BorrowStrategy,
        name = "for_loop_iter",
        chosen = "for_in_iter",
        alternatives = ["iter", "into_iter", "iter_mut", "drain", "range"],
        confidence = 0.88
    );

    // DEPYLER-0272: Prefix unused loop variables with underscore to avoid warnings
    // DEPYLER-0683: But only if the variable is NOT used in the body
    // If the variable IS used, we must keep the original name to match body references.
    // This is the correct solution: check actual usage before deciding to prefix.

    // Helper to check if a variable is used in the loop body
    let is_used_in_body = |var_name: &str| -> bool {
        body.iter().any(|stmt| is_var_used_in_stmt(var_name, stmt))
    };

    // DEPYLER-0756: Helper to check if a variable is reassigned in the loop body
    // If a loop variable is reassigned (e.g., `line = line.strip()`), we need `mut`
    let is_reassigned_in_body = |var_name: &str| -> bool {
        body.iter()
            .any(|stmt| is_var_reassigned_in_stmt(var_name, stmt))
    };

    // DEPYLER-0803: Track loop variable type for int/float coercion
    // When `for i in range(n)` where n is int, track i as Int so that
    // expressions like `i * dx` can coerce i to f64 when dx is float
    if let AssignTarget::Symbol(name) = target {
        // Check if this is a range iteration - the loop variable is Int
        if matches!(iter, HirExpr::Call { func, .. } if func == "range") {
            ctx.var_types.insert(name.clone(), Type::Int);
        }
    }

    // DEPYLER-0821: Track char variables from Counter(string) iteration
    // When iterating over counter.items() or counter.most_common() where counter is from Counter(string),
    // the first tuple element is a char in Rust (not String)
    if let AssignTarget::Tuple(targets) = target {
        if let HirExpr::MethodCall { object, method, .. } = iter {
            // Handle both .items() and .most_common() (with optional arg)
            if method == "items" || method == "most_common" {
                if let HirExpr::Var(counter_name) = object.as_ref() {
                    // Check if this counter is from Counter(string)
                    if ctx.char_counter_vars.contains(counter_name) {
                        // Mark the first tuple element as a char variable
                        if let Some(AssignTarget::Symbol(first_var)) = targets.first() {
                            ctx.char_iter_vars.insert(first_var.clone());
                        }
                    }
                }
            }
        }
    }

    // Generate target pattern based on AssignTarget type
    let target_pattern: syn::Pat = match target {
        AssignTarget::Symbol(name) => {
            // DEPYLER-0272: Prefix with underscore if variable is not used in body
            let var_name = if is_used_in_body(name) {
                name.clone()
            } else {
                format!("_{}", name)
            };
            let ident = safe_ident(&var_name); // DEPYLER-0023
            // DEPYLER-0756: Add `mut` if variable is reassigned inside the loop
            if is_reassigned_in_body(name) {
                parse_quote! { mut #ident }
            } else {
                parse_quote! { #ident }
            }
        }
        AssignTarget::Tuple(targets) => {
            // For tuple unpacking, check each variable individually
            // DEPYLER-0756: Check if any tuple element is reassigned
            let patterns: Vec<syn::Pat> = targets
                .iter()
                .map(|t| match t {
                    AssignTarget::Symbol(s) => {
                        // DEPYLER-0272: Prefix with underscore if not used in body
                        let var_name = if is_used_in_body(s) {
                            s.clone()
                        } else {
                            format!("_{}", s)
                        };
                        let ident = safe_ident(&var_name); // DEPYLER-0023
                        // DEPYLER-0756: Add `mut` if this tuple element is reassigned
                        if is_reassigned_in_body(s) {
                            parse_quote! { mut #ident }
                        } else {
                            parse_quote! { #ident }
                        }
                    }
                    _ => parse_quote! { _nested }, // Nested tuple unpacking - use placeholder
                })
                .collect();
            parse_quote! { (#(#patterns),*) }
        }
        _ => bail!("Unsupported for loop target type"),
    };

    let mut iter_expr = iter.to_rust_expr(ctx)?;

    // DEPYLER-1082: Handle iteration over iterator-typed generator state vars
    // Box<dyn Iterator> doesn't implement IntoIterator, so we need while-let loop
    if ctx.in_generator {
        if let HirExpr::Var(var_name) = iter {
            if ctx.generator_iterator_state_vars.contains(var_name) {
                // Generate: while let Some(x) = self.var_name.next() { body }
                let var_ident = safe_ident(var_name);
                ctx.enter_scope();
                // Declare loop variable
                if let AssignTarget::Symbol(name) = target {
                    ctx.declare_var(name);
                }
                let body_stmts: Vec<_> = body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                ctx.exit_scope();
                ctx.is_final_statement = saved_is_final;

                return Ok(quote! {
                    while let Some(#target_pattern) = self.#var_ident.next() {
                        #(#body_stmts)*
                    }
                });
            }
        }
    }

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

                // DEPYLER-0795: Track loop variables that iterate over string.chars()
                // The loop variable is a `char` type in Rust, which affects ord() codegen.
                // Note: We track ALL char iter vars here. If the var is later converted to String
                // (via needs_char_to_string), it's still a char when passed to ord().
                if let AssignTarget::Symbol(loop_var_name) = target {
                    ctx.char_iter_vars.insert(loop_var_name.clone());
                }
            } else if is_json_value {
                // DEPYLER-0606: serde_json::Value needs .as_array().unwrap() before iteration
                // This handles: for item in json_value (where json_value is a JSON array)
                iter_expr = parse_quote! { #iter_expr.as_array().unwrap().iter().cloned() };
            } else if ctx.iterator_vars.contains(var_name) {
                // DEPYLER-0520: Variable is already an iterator (from .filter().map() etc.)
                // Don't add .iter().cloned() - iterators don't have .iter() method
                // Just iterate directly
            } else {
                // DEPYLER-0710: Check if iterating over a Dict type
                // Python's `for key in dict` iterates over keys only
                // HashMap's .iter() returns (&K, &V) tuples which don't work with .cloned()
                // Use .keys().cloned() to get an iterator over cloned keys
                let is_dict_type = ctx.var_types.get(var_name).is_some_and(|t| {
                    matches!(t, Type::Dict(_, _))
                });

                if is_dict_type {
                    // DEPYLER-0710: For dicts, iterate over keys only (Python semantics)
                    iter_expr = parse_quote! { #iter_expr.keys().cloned() };
                } else {
                    // DEPYLER-0836: Check if iterating over a trueno::Vector type
                    // Vector<T> doesn't have .iter() method, it uses .as_slice().iter()
                    let is_vector_type = ctx.var_types.get(var_name).is_some_and(|t| {
                        match t {
                            Type::Custom(name) => name.starts_with("Vector<") || name == "Vector",
                            _ => false,
                        }
                    });

                    if is_vector_type {
                        // DEPYLER-0836: trueno::Vector<T>.as_slice().iter().cloned()
                        iter_expr = parse_quote! { #iter_expr.as_slice().iter().cloned() };
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
        }

        // DEPYLER-1045: Handle string method calls that return String
        // When iterating over text.lower(), text.upper(), text.strip(), etc.,
        // we need to add .chars() because these methods return String which is not an iterator
        if let HirExpr::MethodCall { method, .. } = iter {
            // List of Python string methods that return strings (need .chars() for iteration)
            let is_string_returning_method = matches!(
                method.as_str(),
                "lower"
                    | "upper"
                    | "strip"
                    | "lstrip"
                    | "rstrip"
                    | "capitalize"
                    | "title"
                    | "swapcase"
                    | "casefold"
                    | "replace"
            );
            if is_string_returning_method {
                iter_expr = parse_quote! { #iter_expr.chars() };

                // Track the loop variable as char iterator
                if let AssignTarget::Symbol(loop_var_name) = target {
                    ctx.char_iter_vars.insert(loop_var_name.clone());
                }
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
        // DEPYLER-1051: Handle direct iteration over self.field
        // Look up field type in class_field_types to get element type
        HirExpr::Attribute { value, attr, .. } => {
            if matches!(value.as_ref(), HirExpr::Var(name) if name == "self") {
                ctx.class_field_types.get(attr).and_then(|t| match t {
                    Type::List(elem_t) => Some(*elem_t.clone()),
                    Type::Set(elem_t) => Some(*elem_t.clone()),
                    Type::Dict(key_t, _) => Some(*key_t.clone()),
                    _ => None,
                })
            } else {
                None
            }
        }
        HirExpr::Call { func, args, .. } if func == "enumerate" => {
            // enumerate(items) yields (int, elem_type)
            if let Some(arg) = args.first() {
                match arg {
                    HirExpr::Var(var_name) => {
                        ctx.var_types.get(var_name).and_then(|t| match t {
                            Type::List(elem_t) => Some(Type::Tuple(vec![Type::Int, *elem_t.clone()])),
                            Type::Set(elem_t) => Some(Type::Tuple(vec![Type::Int, *elem_t.clone()])),
                            _ => None,
                        })
                    }
                    // DEPYLER-1051: Handle enumerate(self.field) for struct field iteration
                    // Look up field type in class_field_types to get element type
                    HirExpr::Attribute { value, attr, .. } => {
                        if matches!(value.as_ref(), HirExpr::Var(name) if name == "self") {
                            ctx.class_field_types.get(attr).and_then(|t| match t {
                                Type::List(elem_t) => {
                                    Some(Type::Tuple(vec![Type::Int, *elem_t.clone()]))
                                }
                                Type::Set(elem_t) => {
                                    Some(Type::Tuple(vec![Type::Int, *elem_t.clone()]))
                                }
                                _ => None,
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        // DEPYLER-0930: Handle method call iteration
        HirExpr::MethodCall { object, method, .. } => {
            // Check for Path.glob() which yields PathBuf
            if method == "glob" {
                if let HirExpr::Var(var_name) = object.as_ref() {
                    let is_path = ctx
                        .var_types
                        .get(var_name)
                        .map(|t| {
                            matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path")
                        })
                        .unwrap_or(false);
                    if is_path {
                        Some(Type::Custom("PathBuf".to_string()))
                    } else {
                        None
                    }
                } else {
                    None
                }
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
    ctx.is_final_statement = saved_is_final;

    // DEPYLER-0307 Fix #8: Handle enumerate() usize index casting
    // When iterating with enumerate(), the first element of the tuple is usize
    // If we're destructuring a tuple and the iterator is enumerate(), cast the first variable to i32
    let needs_enumerate_cast = matches!(iter, HirExpr::Call { func, .. } if func == "enumerate")
        && matches!(target, AssignTarget::Tuple(targets) if !targets.is_empty());

    // DEPYLER-0317: Handle string iteration char→String conversion
    // When iterating over strings with .chars(), convert char to String for HashMap<String, _> compatibility
    // DEPYLER-0715: Fixed to use USAGE-based detection instead of name heuristics
    // DEPYLER-0716: Also ensure we're actually iterating over a STRING (not a range/list/etc.)
    // DEPYLER-0744: MUST check actual type, not just is_var_iteration!
    // Only convert char to String if:
    // 1. We're iterating over a string variable (will become .chars())
    // 2. NOT iterating over a range (range produces integers)
    // 3. The loop variable is actually used as a dict key
    let is_range_iteration = matches!(iter, HirExpr::Call { func, .. } if func == "range");
    // DEPYLER-0744: Check if iterable is actually a String type, not just any variable
    // This prevents List[int] from being treated as string iteration
    // DEPYLER-1045: Use char_iter_vars to detect string iteration (populated when .chars() is added)
    // This is more reliable than just checking var_types since it catches name-based heuristics too
    let is_string_iteration = if let AssignTarget::Symbol(loop_var_name) = target {
        ctx.char_iter_vars.contains(loop_var_name)
    } else {
        false
    };
    // Only consider string iteration if we're iterating over a STRING variable (not list/dict/etc.)
    // DEPYLER-1045: Only convert if loop variable is used as a dict key
    // Dictionary keys need String type for HashMap<String, _>
    // NOTE: We removed function arg and comparison detection because:
    // - Most functions (like ord()) work fine with char
    // - Char methods (is_alphabetic, is_uppercase) don't exist on String
    // - Comparisons can be handled by converting the comparison, not the variable
    let needs_char_to_string = !is_range_iteration && is_string_iteration && if let AssignTarget::Symbol(loop_var_name) = target {
        // Only convert to String if used as dictionary key
        body.iter().any(|stmt| is_var_used_as_dict_key_in_stmt(loop_var_name, stmt))
    } else {
        false
    };

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

/// DEPYLER-1042: Get the value type for a subscript expression
/// For `d["key"]` where `d: Dict[K, V]`, returns `Some(V)`
/// Handles nested subscripts: `d["k1"]["k2"]` returns innermost value type
fn get_subscript_value_type(expr: &HirExpr, ctx: &CodeGenContext) -> Option<Type> {
    match expr {
        HirExpr::Var(name) => {
            // Look up the variable's type and extract dict value type
            if let Some(Type::Dict(_, val_type)) = ctx.var_types.get(name) {
                return Some(val_type.as_ref().clone());
            }
            None
        }
        HirExpr::Index { base, .. } => {
            // Recursively get the value type, then extract its dict value type
            // For d["k1"]["k2"], first get value type of d["k1"], then its value type
            if let Type::Dict(_, inner_val_type) = get_subscript_value_type(base.as_ref(), ctx)? {
                return Some(inner_val_type.as_ref().clone());
            }
            None
        }
        _ => None,
    }
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
            // DEPYLER-0940: Skip if cmd_name is empty to prevent panic in format_ident!()
            if !cmd_name.is_empty() {
                if let AssignTarget::Symbol(cse_var) = target {
                    use quote::{format_ident, quote};
                    let variant_name = format_ident!("{}", to_pascal_case(&cmd_name));
                    let var_ident = safe_ident(cse_var);

                    // DEPYLER-0456 Bug #2: Track this CSE temp so is_subcommand_check() can find it
                    ctx.cse_subcommand_temps
                        .insert(cse_var.clone(), cmd_name.clone());

                    // DEPYLER-0456 Bug #3 FIX: Always use "command" as Rust field name
                    // DEPYLER-1063: args.command is Option<Commands>, wrap pattern in Some()
                    return Ok(quote! {
                        let #var_ident = matches!(args.command, Some(Commands::#variant_name { .. }));
                    });
                }
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

    // DEPYLER-0821: Track variables assigned from Counter(string)
    // When counter = Counter(text) where text is a string, mark counter in char_counter_vars
    // This is used to detect char iteration in for (k,v) in counter.items()
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::Call { func, args, .. } = value {
            if func == "Counter" && args.len() == 1 {
                // Check if the argument is a string type or derived from string operations
                let is_string_arg = match &args[0] {
                    HirExpr::Var(arg_name) => {
                        // Check known type or use heuristics for common string var names
                        ctx.var_types
                            .get(arg_name)
                            .is_some_and(|t| matches!(t, Type::String))
                            || arg_name == "text"
                            || arg_name == "s"
                            || arg_name == "string"
                            || arg_name.ends_with("_text")
                    }
                    // MethodCall like sys.stdin.read().strip() returns string
                    HirExpr::MethodCall { method, .. } => {
                        method == "read"
                            || method == "strip"
                            || method == "lower"
                            || method == "upper"
                    }
                    _ => false,
                };
                if is_string_arg {
                    ctx.char_counter_vars.insert(var_name.clone());
                }
            }
        }
    }

    // DEPYLER-0801: Track variables assigned from Callable calls that return Float
    // When fa = f(a) where f is Callable[[float], float], we need to track fa as Float
    // so that later comparisons like `fa * fc < 0` coerce 0 to 0.0.
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::Call { func, .. } = value {
            // Check if func is a Callable parameter with Float return type
            // Callable is stored as Type::Generic { base: "Callable", params: [input_types, return_type] }
            if let Some(Type::Generic { base, params }) = ctx.var_types.get(func) {
                if base == "Callable" && params.len() == 2 && matches!(params[1], Type::Float) {
                    ctx.var_types.insert(var_name.clone(), Type::Float);
                }
            }
            // Also handle Type::Function case (less common but possible)
            if let Some(Type::Function { ret, .. }) = ctx.var_types.get(func) {
                if matches!(ret.as_ref(), Type::Float) {
                    ctx.var_types.insert(var_name.clone(), Type::Float);
                }
            }
            // Also check module-level function return types
            if let Some(Type::Float) = ctx.function_return_types.get(func) {
                ctx.var_types.insert(var_name.clone(), Type::Float);
            }
        }
    }

    // DEPYLER-0520: Track variables assigned from iterator-producing expressions
    // Generator expressions and method chains ending in filter/map/etc produce iterators,
    // not collections. These variables should NOT have .iter().cloned() added in for loops.
    // DEPYLER-1078: Also mark iterator variables as mutable since .next() requires &mut self
    if let AssignTarget::Symbol(var_name) = target {
        if is_iterator_producing_expr(value) {
            ctx.iterator_vars.insert(var_name.clone());
            // DEPYLER-1078: Iterators need to be mutable for .next() calls
            ctx.mutable_vars.insert(var_name.clone());
        }
    }

    // DEPYLER-0932: Track variables assigned from numpy operations
    // When result = numpy_expr, add "result" to numpy_vars so is_numpy_array_expr() can detect it
    // This enables correct iteration with .as_slice().iter() instead of bare .iter()
    if let AssignTarget::Symbol(var_name) = target {
        if is_numpy_value_expr(value, ctx) {
            ctx.numpy_vars.insert(var_name.clone());
        }
    }

    // DEPYLER-0837: Mark csv.DictReader variables as mutable
    // csv::Reader needs mutable borrows for .headers() and iteration (.records())
    // Pattern: reader = csv.DictReader(f) → let mut reader = csv::ReaderBuilder::new()...
    if let AssignTarget::Symbol(var_name) = target {
        if let HirExpr::MethodCall { object, method, .. } = value {
            if method == "DictReader" || method == "reader" {
                if let HirExpr::Var(module_name) = object.as_ref() {
                    if module_name == "csv" {
                        ctx.mutable_vars.insert(var_name.clone());
                    }
                }
            }
        }
    }

    // DEPYLER-0440: Handle None-placeholder assignments
    // When a variable is initialized with None and later reassigned in if-elif-else,
    // Python uses None as a placeholder that will be overwritten.
    // Python pattern: var = None; if cond: var = x; else: var = y; use(var)
    // Rust pattern: Skip None assignment, let first real assignment be the declaration.
    // The mutable_vars check ensures the variable WILL be assigned a real value later.
    // DEPYLER-0823: Track the variable so it can be hoisted if assigned inside a conditional
    if let AssignTarget::Symbol(var_name) = target {
        let is_none_literal = matches!(value, HirExpr::Literal(Literal::None));
        let is_mutable = ctx.mutable_vars.contains(var_name);
        if is_none_literal && is_mutable {
            // Skip None placeholder assignment - the first real assignment will declare the var
            // This avoids type mismatch: `let mut x = None; x = "yes"` (Option vs &str)
            // DEPYLER-0823: Track this variable for potential hoisting in if statements
            ctx.none_placeholder_vars.insert(var_name.clone());
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
                                // DEPYLER-0822: Use command_name as key (not variable name)
                                // to avoid duplicate enum variants
                                let cmd_name = command_name.clone();
                                // Only register if not already registered (preregister may have
                                // already added this with argument info)
                                if ctx.argparser_tracker.get_subcommand(&cmd_name).is_none() {
                                    ctx.argparser_tracker.register_subcommand(
                                        cmd_name.clone(),
                                        SubcommandInfo {
                                            name: command_name,
                                            help,
                                            arguments: vec![],
                                            subparsers_var: subparsers_var.clone(),
                                        },
                                    );
                                }
                                // Also map variable name to command name
                                ctx.argparser_tracker
                                    .subcommand_var_to_cmd
                                    .insert(subcommand_var.clone(), cmd_name);
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
        // DEPYLER-0709: Also track Tuple types for correct field access (.0, .1)
        if let Some(annot_type) = type_annotation {
            match annot_type {
                // DEPYLER-1045: Added Type::String to enable auto-borrow in function calls
                // Without this, `text: str = "hello world"` wouldn't be tracked,
                // causing `capitalize_words(text)` to miss the `&` borrow.
                Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::Tuple(_) | Type::String => {
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
            // DEPYLER-0907: Check for os.environ.get(key) - 1 argument means returns Option<String>
            // Python: config_file = os.environ.get("CONFIG_FILE")  # Returns None if not set
            // Rust: config_file = std::env::var("CONFIG_FILE").ok()  # Returns Option<String>
            else if method == "get" && args.len() == 1 {
                if let HirExpr::Attribute {
                    value: attr_obj,
                    attr,
                } = object.as_ref()
                {
                    if let HirExpr::Var(module) = attr_obj.as_ref() {
                        if module == "os" && attr == "environ" {
                            // os.environ.get(key) returns Option<String>
                            ctx.var_types.insert(
                                var_name.clone(),
                                Type::Optional(Box::new(Type::String)),
                            );
                        }
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
                // DEPYLER-0969: Track deque() constructor for VecDeque truthiness conversion
                // This enables `while queue:` → `while !queue.is_empty()` conversion
                // Python: queue = deque([start]) → Rust: VecDeque::from(vec![start])
                else if func == "deque" || func == "collections.deque" || func == "Deque" {
                    // Track as Custom type for VecDeque - enables .is_empty() truthiness
                    ctx.var_types.insert(
                        var_name.clone(),
                        Type::Custom("std::collections::VecDeque<i32>".to_string()),
                    );
                }
                // DEPYLER-0969: Track queue.Queue() and similar constructors
                else if func == "Queue" || func == "LifoQueue" || func == "PriorityQueue" {
                    ctx.var_types.insert(
                        var_name.clone(),
                        Type::Custom("std::collections::VecDeque<i32>".to_string()),
                    );
                }
                // DEPYLER-0969: Track heapq-related variables (BinaryHeap)
                else if func == "heappush" || func == "heapify" {
                    // These don't create new variables, but let's track if called as constructor
                    ctx.var_types.insert(
                        var_name.clone(),
                        Type::Custom("std::collections::BinaryHeap<i32>".to_string()),
                    );
                }
                // DEPYLER-0269: Track user-defined function return types
                // Lookup function return type and track it for Display trait selection
                // Enables: result = merge(&a, &b) where merge returns list[int]
                // DEPYLER-0709: Also track Tuple return types for correct field access (.0, .1)
                else if let Some(ret_type) = ctx.function_return_types.get(func) {
                    if matches!(ret_type, Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::Tuple(_)) {
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
                // DEPYLER-0785: Track float return types for CSE comparison coercion
                // When f() returns float, result = f(x) should track result as Float
                else if matches!(ctx.function_return_types.get(func), Some(Type::Float)) {
                    ctx.var_types.insert(var_name.clone(), Type::Float);
                }
            }
            // DEPYLER-0785: Track Binary expressions that return float
            // When CSE extracts `f(a) * f(b)` into `_cse_temp_0`, we need to track
            // that the temp is float so `_cse_temp_0 > 0` coerces 0 to 0f64.
            HirExpr::Binary { .. } => {
                if expr_infers_float(value, ctx) {
                    ctx.var_types.insert(var_name.clone(), Type::Float);
                }
            }
            // DEPYLER-0803: Propagate types from variable-to-variable assignments
            // When dx = _cse_temp_0 where _cse_temp_0 is Float, dx should also be Float
            // This enables proper coercion in expressions like i * dx where i is int
            HirExpr::Var(source_var) => {
                if let Some(source_type) = ctx.var_types.get(source_var) {
                    ctx.var_types.insert(var_name.clone(), source_type.clone());
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
            // DEPYLER-0709: Track tuple type from literal for correct field access (.0, .1)
            // Example: result = (1, 2) → result.0, not result.get(0)
            HirExpr::Tuple(elements) => {
                // Infer element types from tuple elements
                let elem_types: Vec<Type> = if let Some(Type::Tuple(types)) = type_annotation {
                    types.clone()
                } else {
                    elements
                        .iter()
                        .map(crate::rust_gen::func_gen::infer_expr_type_simple)
                        .collect()
                };
                ctx.var_types
                    .insert(var_name.clone(), Type::Tuple(elem_types));
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
                // DEPYLER-0713 Part 4: Track json.loads() and json.load() as returning Value
                // data = json.loads(s) → data is serde_json::Value
                else if matches!(method.as_str(), "loads" | "load") {
                    if let HirExpr::Var(obj_var) = object.as_ref() {
                        if obj_var == "json" {
                            ctx.var_types.insert(
                                var_name.clone(),
                                Type::Custom("serde_json::Value".to_string()),
                            );
                        }
                    }
                }
            }
            // DEPYLER-0713 Part 3: Track Index access on Value-typed variables as Value
            // When we have `count = data["key"]` where `data` is serde_json::Value,
            // then `count` should also be tracked as Value so subsequent assignments
            // like `count = 10` get wrapped with json!()
            HirExpr::Index { base, .. } => {
                if let HirExpr::Var(base_var) = base.as_ref() {
                    if let Some(base_type) = ctx.var_types.get(base_var) {
                        // If base is Value type, the result is also Value
                        if matches!(base_type, Type::Custom(name) if name == "serde_json::Value" || name == "Value") {
                            ctx.var_types.insert(
                                var_name.clone(),
                                Type::Custom("serde_json::Value".to_string()),
                            );
                        }
                        // If base is Dict with Value values, result is Value
                        else if let Type::Dict(_, v) = base_type {
                            if matches!(v.as_ref(), Type::Custom(name) if name == "serde_json::Value" || name == "Value") {
                                ctx.var_types.insert(
                                    var_name.clone(),
                                    Type::Custom("serde_json::Value".to_string()),
                                );
                            }
                        }
                    }
                }
            }
            // DEPYLER-0713: Catch-all for primitive literals and other expressions
            // This high-ROI fix prevents UnificationVar → serde_json::Value fallback
            // by tracking concrete types (Int, Float, String, Bool) from literals
            // and expressions. This addresses 600+ E0308 errors in the codebase.
            _ => {
                // DEPYLER-0713 Part 5: Preserve serde_json::Value tracking
                // When a variable was previously assigned from json.loads() or data["key"],
                // it's tracked as Value. If we now assign a primitive like `x = 5`,
                // we should NOT overwrite the Value tracking - it means the variable
                // is being reassigned and the new value needs json!() wrapping.
                let should_preserve_value_type = ctx
                    .var_types
                    .get(var_name)
                    .map(|t| {
                        matches!(t, Type::Custom(name) if name == "serde_json::Value" || name == "Value")
                    })
                    .unwrap_or(false);

                if !should_preserve_value_type {
                    // Use infer_expr_type_simple for all unhandled expressions
                    // This tracks primitive types like Int, Float, String, Bool
                    let inferred = crate::rust_gen::func_gen::infer_expr_type_simple(value);
                    // Only insert if we got a concrete type (not Unknown)
                    if !matches!(inferred, Type::Unknown) {
                        ctx.var_types.insert(var_name.clone(), inferred);
                    }
                }
            }
        }
    }

    // DEPYLER-0472: Set json context when assigning to serde_json::Value dicts
    // This ensures dict literals use json!({}) instead of HashMap::new()
    let prev_json_context = ctx.in_json_context;

    // DEPYLER-1015: In NASA mode, never set json context - use std-only types
    let nasa_mode = ctx.type_mapper.nasa_mode;

    // DEPYLER-0713: Check if target variable is typed as serde_json::Value
    // If so, set json context so literals get wrapped with json!()
    // DEPYLER-1015: Skip in NASA mode
    if !nasa_mode {
        if let AssignTarget::Symbol(var_name) = target {
            // Check if variable is tracked as Value type
            if let Some(var_type) = ctx.var_types.get(var_name) {
                if matches!(var_type, Type::Custom(name) if name == "serde_json::Value" || name == "Value") {
                    ctx.in_json_context = true;
                }
            }
            // Also check type annotation
            if let Some(annot) = type_annotation {
                if matches!(annot, Type::Custom(name) if name == "serde_json::Value" || name == "Value" || name == "Any" || name == "any") {
                    ctx.in_json_context = true;
                }
            }
        }

        if let AssignTarget::Index { base, .. } = target {
            // DEPYLER-0714: Check actual type FIRST before falling back to name heuristic
            if let HirExpr::Var(base_name) = base.as_ref() {
                // DEPYLER-0713: Check if base is typed as Value
                if let Some(base_type) = ctx.var_types.get(base_name) {
                    if matches!(base_type, Type::Custom(name) if name == "serde_json::Value" || name == "Value") {
                        ctx.in_json_context = true;
                    }
                    // Check if it's a HashMap with Value values OR Unknown values
                    if let Type::Dict(_, v) = base_type {
                        let val_is_json = match v.as_ref() {
                            Type::Unknown => true,
                            Type::Custom(name) if name == "serde_json::Value" || name == "Value" => true,
                            _ => false,
                        };
                        if val_is_json {
                            ctx.in_json_context = true;
                        }
                    }
                } else {
                    // DEPYLER-0714: Only use name heuristic when type is NOT known
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
        }
    }

    // DEPYLER-0727: Set assignment target type for dict literal Value wrapping
    // This allows dict codegen to check if target is Dict[str, Any] → HashMap<String, Value>
    let prev_assign_type = ctx.current_assign_type.take();
    ctx.current_assign_type = type_annotation.clone();

    // DEPYLER-1042: For subscript assignments, propagate inner type from base dict
    // Pattern: d["level1"] = {} where d: Dict[str, Dict[str, str]]
    // The empty dict should get type Dict[str, str], not default HashMap<String, String>
    if ctx.current_assign_type.is_none() {
        if let AssignTarget::Index { base, .. } = target {
            // Extract the value type from the base expression's dict type
            let base_value_type = get_subscript_value_type(base.as_ref(), ctx);
            if base_value_type.is_some() {
                ctx.current_assign_type = base_value_type;
            }
        }
    }

    // DEPYLER-1045: For simple variable assignments, look up the target's type from var_types
    // Pattern: memo = {} where memo: Dict[int, int] (from parameter)
    // The empty dict should get type Dict[int, int], not default HashMap<String, String>
    if ctx.current_assign_type.is_none() {
        if let AssignTarget::Symbol(name) = target {
            if let Some(var_type) = ctx.var_types.get(name.as_str()) {
                // Handle Option<Dict<K, V>> by extracting the inner Dict type
                // Parameter types may be wrapped in Optional from Optional annotations
                if let Type::Optional(inner) = var_type {
                    if let Type::Dict(_, _) = inner.as_ref() {
                        ctx.current_assign_type = Some(inner.as_ref().clone());
                    }
                } else if let Type::Dict(_, _) = var_type {
                    ctx.current_assign_type = Some(var_type.clone());
                }
            }
        }
    }

    let mut value_expr = value.to_rust_expr(ctx)?;

    // DEPYLER-1113: Propagate external library return type to var_types
    // When the expression was a MethodCall on an external module (e.g., requests.get),
    // the TypeDB lookup stored the return type. Now propagate it to var_types so
    // subsequent expressions (e.g., resp.json()) have type info.
    if let AssignTarget::Symbol(var_name) = target {
        if let Some(return_type_str) = ctx.last_external_call_return_type.take() {
            // Map the external type string to a Type
            // For now, use Type::Custom with the full qualified name
            ctx.var_types.insert(var_name.clone(), Type::Custom(return_type_str));
        }
    }

    // DEPYLER-0727: Restore previous assignment type
    ctx.current_assign_type = prev_assign_type;

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

        // DEPYLER-0719: When type annotation is bare `tuple` (empty tuple), infer from
        // function call return type. Python `x: tuple = func()` should use func's return type.
        let actual_type = if matches!(actual_type, Type::Tuple(elems) if elems.is_empty()) {
            // Check if value is a function call
            if let HirExpr::Call { func, .. } = value {
                // Look up function return type
                if let Some(ret_type) = ctx.function_return_types.get(func) {
                    // Use the function's actual return type instead of empty tuple
                    ret_type
                } else {
                    actual_type
                }
            } else {
                actual_type
            }
        } else {
            actual_type
        };

        // DEPYLER-0760: When value is None literal, wrap type annotation in Option<>
        // Pattern: `x: str = None` in Python → `let x: Option<String> = None;` in Rust
        // This ensures the type annotation matches the None value which requires Option<T>
        let is_none_value = matches!(value, HirExpr::Literal(Literal::None));
        let needs_option_wrap = is_none_value && !matches!(actual_type, Type::Optional(_));
        let target_rust_type = if needs_option_wrap {
            // Wrap the type in Option since the value is None
            crate::type_mapper::RustType::Option(Box::new(ctx.type_mapper.map_type(actual_type)))
        } else {
            ctx.type_mapper.map_type(actual_type)
        };
        let target_syn_type = rust_type_to_syn(&target_rust_type)?;

        // DEPYLER-0760: Update var_types with Optional type so subsequent usage is correct
        // This ensures is_none() and print() calls handle the variable correctly
        if needs_option_wrap {
            if let AssignTarget::Symbol(var_name) = target {
                ctx.var_types
                    .insert(var_name.clone(), Type::Optional(Box::new(actual_type.clone())));
            }
        }

        // DEPYLER-0272: Check if we need type conversion (e.g., usize to i32)
        // DEPYLER-0455 Bug 7: Also pass ctx for validator function detection
        // Pass the value expression to determine if cast is actually needed
        if needs_type_conversion(actual_type, value) {
            value_expr = apply_type_conversion(value_expr, actual_type);
        }

        // DEPYLER-1054: Assignment Coercion - extract concrete types from DepylerValue
        // When RHS is a DepylerValue expression and LHS has a concrete type annotation,
        // automatically inject .to_i64()/.to_f64()/.to_string()/.to_bool() extraction.
        // Example: x: int = data["count"] → let x: i32 = data["count"].to_i64() as i32;
        // DEPYLER-1064: Wrap in parentheses to ensure correct precedence for complex expressions
        // Example: x: int = count + 1 → let x: i32 = (count + 1).to_i64() as i32;
        if expr_produces_depyler_value(value, ctx) {
            if let Some(extraction) = get_depyler_extraction_for_type(actual_type) {
                let extraction_tokens: proc_macro2::TokenStream = extraction.parse().unwrap();
                value_expr = parse_quote! { (#value_expr) #extraction_tokens };
            }
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
        // DEPYLER-0952: When value is bare None literal without type annotation,
        // Rust needs a type annotation because it can't infer Option<_>.
        // Python: `x = None` → Rust: `let x: Option<()> = None;`
        if matches!(value, HirExpr::Literal(Literal::None)) {
            (Some(quote! { : Option<()> }), false)
        } else {
            (None, false)
        }
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

    // DEPYLER-0625: Wrap File/Stdout values in Box::new() for trait object unification
    // When a variable is declared as Box<dyn Write> (heterogeneous IO types),
    // wrap the assigned value with Box::new() to satisfy the type
    if let AssignTarget::Symbol(var_name) = target {
        if ctx.boxed_dyn_write_vars.contains(var_name) {
            value_expr = parse_quote! { Box::new(#value_expr) };
        }
    }

    // DEPYLER-1027/DEPYLER-1029: Handle dict value assignment to dict in NASA mode
    // Only wrap dict values in format!() if the target dict expects String values
    // If target is Dict[str, Dict[...]], keep dict values as HashMap
    let value_expr = if ctx.type_mapper.nasa_mode {
        if let AssignTarget::Index { base, .. } = target {
            if matches!(value, HirExpr::Dict(_)) {
                // DEPYLER-1027/1034/1042: Handle dict value assignment in NASA mode
                // Only DON'T wrap if we KNOW the outer dict expects Dict values (not String)
                // DEPYLER-1042: Use get_subscript_value_type for nested subscripts
                let has_dict_value_type = {
                    // Get the value type from the subscript chain
                    if let Some(val_type) = get_subscript_value_type(base.as_ref(), ctx) {
                        // Value type is Dict → target expects Dict, skip wrapping
                        matches!(val_type, Type::Dict(_, _))
                    } else if let HirExpr::Var(base_name) = &**base {
                        // Fallback: direct Var check for simple cases
                        ctx.var_types.get(base_name).is_some_and(|t| {
                            matches!(t, Type::Dict(_, val_type) if matches!(val_type.as_ref(), Type::Dict(_, _)))
                        })
                    } else {
                        false
                    }
                };

                if has_dict_value_type {
                    // Keep as HashMap for Dict[K, Dict[...]] types
                    value_expr
                } else {
                    // Default: wrap in format! for HashMap<String, String> compatibility
                    parse_quote! { format!("{:?}", #value_expr) }
                }
            } else {
                value_expr
            }
        } else {
            value_expr
        }
    } else {
        value_expr
    };

    match target {
        AssignTarget::Symbol(symbol) => {
            codegen_assign_symbol(symbol, value_expr, type_annotation_tokens, is_final, ctx)
        }
        AssignTarget::Index { base, index } => codegen_assign_index(base, index, value_expr, ctx),
        AssignTarget::Attribute { value, attr } => {
            codegen_assign_attribute(value, attr, value_expr, ctx)
        }
        AssignTarget::Tuple(targets) => {
            codegen_assign_tuple(targets, value, value_expr, type_annotation_tokens, ctx)
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
        // DEPYLER-0777: Convert string literals to .to_string() when reassigning hoisted variables
        // Hoisted variables (e.g., loop-escaping vars) are declared with Default::default()
        // If the first "real" assignment is a string literal like "", the type would be &str
        // But later assignments with format!() return String → E0308 type mismatch
        // Solution: Convert string literal reassignments to owned String
        // DEPYLER-0787: Also handle &str parameter references (not just literals)
        // When `result = text` where text is &str param, we need `.to_string()` too
        // DEPYLER-0788: Also handle ref-pattern bindings from match arms (e.g., `ref text`)
        // When `result = text` where text is from `ref text`, it's &String and needs .to_string()
        let value_expr = {
            let value_str = quote!(#value_expr).to_string();
            // Check if it's a string literal
            let is_string_literal = value_str.starts_with('"') && value_str.ends_with('"');
            // Check if it's a &str parameter reference
            let is_str_param = ctx.fn_str_params.contains(&value_str);
            // Check if it's a ref-pattern binding from match arm (gives &String)
            let is_ref_binding = ctx.subcommand_match_fields.contains(&value_str);
            // DEPYLER-1130: Check if it's an interned string constant (e.g., STR_EMPTY)
            // These are &'static str constants that need .to_string() when assigned to String variables
            let is_interned_const = value_str.starts_with("STR_");

            if is_string_literal || is_str_param || is_ref_binding || is_interned_const {
                parse_quote! { #value_expr.to_string() }
            } else {
                value_expr
            }
        };
        // DEPYLER-0964: Handle &mut Option<Dict> parameter assignments
        // When assigning to a parameter that is `&mut Option<HashMap<K, V>>`,
        // we need to dereference and wrap in Some:
        // - Python: `memo = {}` → Rust: `*memo = Some(HashMap::new())`
        // This handles the common memoization pattern: `if memo is None: memo = {}`
        if ctx.mut_option_dict_params.contains(symbol) {
            // Check if value is already wrapped in Some or is None
            let value_str = quote!(#value_expr).to_string();
            if value_str.starts_with("Some") || value_str == "None" {
                // Already wrapped, just dereference
                return Ok(quote! { *#target_ident = #value_expr; });
            } else {
                // Wrap in Some and dereference
                return Ok(quote! { *#target_ident = Some(#value_expr); });
            }
        }

        // DEPYLER-1126: Handle &mut Option<T> parameter assignments (any T, not just Dict)
        // When assigning to a parameter that is `&mut Option<T>`,
        // we need to dereference and wrap in Some:
        // - Python: `as_of = date.today()` → Rust: `*as_of = Some(DepylerDate::today())`
        // This handles the common "optional with default None" pattern
        if ctx.mut_option_params.contains(symbol) {
            let value_str = quote!(#value_expr).to_string();
            if value_str.starts_with("Some") || value_str == "None" {
                // Already wrapped, just dereference
                return Ok(quote! { *#target_ident = #value_expr; });
            } else {
                // Wrap in Some and dereference
                return Ok(quote! { *#target_ident = Some(#value_expr); });
            }
        }

        // DEPYLER-0604: Check if variable has Optional type and wrap value in Some()
        let final_value = if let Some(Type::Optional(inner_type)) = ctx.var_types.get(symbol) {
            // Check if the value is already wrapped in Some or is None
            // DEPYLER-1093: Also check for expressions that already return Option<T>
            let value_str = quote!(#value_expr).to_string();

            // DEPYLER-1093: Check if source variable is also Optional type
            // When assigning from Option<T> variable to Option<T> target, don't wrap
            // Extract variable name from simple path expression (e.g., "default" from syn::Expr)
            let source_var_is_optional = if let syn::Expr::Path(ref path) = value_expr {
                if path.path.segments.len() == 1 {
                    let source_var = path.path.segments[0].ident.to_string();
                    ctx.var_types
                        .get(&source_var)
                        .map(|ty| matches!(ty, Type::Optional(_)))
                        .unwrap_or(false)
                } else {
                    false
                }
            } else {
                false
            };

            // Expression patterns that already return Option<T>
            let expr_returns_option = value_str.starts_with("Some")
                || value_str == "None"
                // .ok() converts Result<T, E> to Option<T>
                || value_str.ends_with(".ok()")
                || value_str.ends_with(". ok ()")
                // .get(...) on HashMap/Vec returns Option
                || value_str.contains(".get(")
                || value_str.contains(". get (")
                // .cloned() preserves Option
                || value_str.ends_with(".cloned()")
                || value_str.ends_with(". cloned ()")
                // .as_ref() on Option preserves Option
                || (value_str.contains(".as_ref()") && !value_str.contains(".unwrap()"));

            if source_var_is_optional || expr_returns_option {
                // DEPYLER-1093: If source is &Option<T> variable, need .clone() for owned Option<T>
                // But if it's an expression like .ok() or .get(), it returns owned Option
                if source_var_is_optional && !expr_returns_option {
                    parse_quote! { #value_expr.clone() }
                } else {
                    value_expr
                }
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
            // DEPYLER-0644: Also handle field access like `args.text` from ref pattern matching
            // DEPYLER-0665: Also handle ref-pattern bindings from match arms (e.g., `ref text`)
            let needs_clone = if let syn::Expr::Path(ref path) = value_expr {
                // Check if this is a simple path (single identifier)
                if path.path.segments.len() == 1 {
                    let ident = &path.path.segments[0].ident;
                    let var_name = ident.to_string();
                    // Check if:
                    // 1. Source is a ref-pattern binding from match arm (subcommand_match_fields)
                    // 2. OR source is already declared and different from target
                    // This handles `let mut result = text` where `text` is from `ref text`
                    ctx.subcommand_match_fields.contains(&var_name)
                        || (ctx.is_declared(&var_name) && var_name != symbol)
                } else {
                    false
                }
            } else if let syn::Expr::Field(ref field) = value_expr {
                // Handle field access like `args.text` from ref pattern matching
                // These are typically &String that need to be cloned for ownership
                if let syn::Expr::Path(ref base_path) = *field.base {
                    if base_path.path.segments.len() == 1 {
                        let base_name = base_path.path.segments[0].ident.to_string();
                        // If the base (e.g., `args`) is declared, the field is borrowed
                        ctx.is_declared(&base_name)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            // DEPYLER-0704: When initializing mutable var from &str param, use .to_string()
            // instead of .clone() because &str.clone() returns &str, not String.
            // This prevents type mismatch when the var is later assigned a String.
            let is_str_param = if let syn::Expr::Path(ref path) = value_expr {
                if path.path.segments.len() == 1 {
                    let var_name = path.path.segments[0].ident.to_string();
                    ctx.fn_str_params.contains(&var_name)
                } else {
                    false
                }
            } else {
                false
            };

            let init_expr = if is_str_param {
                // Convert &str to owned String for mutable vars
                parse_quote! { #value_expr.to_string() }
            } else if needs_clone {
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
    // DEPYLER-0964: Handle subscript assignment to &mut Option<HashMap<K, V>> parameters
    // When a parameter is Dict[K,V] with default None, it becomes &mut Option<HashMap>
    // Subscript assignment needs to unwrap the Option first:
    // - memo[k] = v → memo.as_mut().unwrap().insert(k, v)
    if let HirExpr::Var(base_name) = base {
        if ctx.mut_option_dict_params.contains(base_name) {
            let base_ident = safe_ident(base_name);
            let key_expr = index.to_rust_expr(ctx)?;
            // Clone key if it's a variable to avoid move issues
            let needs_clone = matches!(index, HirExpr::Var(_));
            if needs_clone {
                return Ok(quote! {
                    #base_ident.as_mut().unwrap().insert(#key_expr.clone(), #value_expr);
                });
            } else {
                return Ok(quote! {
                    #base_ident.as_mut().unwrap().insert(#key_expr, #value_expr);
                });
            }
        }
    }

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

    // DEPYLER-0403/DEPYLER-1027: Convert string literals to String for Dict values
    // In NASA mode (single-shot compile), always use HashMap<String, String> for dicts
    // so string literal values ALWAYS need .to_string() for type consistency
    let value_expr = if !is_numeric_index {
        // Check if value_expr is a string literal
        let is_string_literal =
            matches!(&value_expr, syn::Expr::Lit(lit) if matches!(&lit.lit, syn::Lit::Str(_)));

        // DEPYLER-1027: Always convert string literals to String when inserting into dicts
        // This handles the common case of HashMap<String, String> which is the default for
        // empty dict literals like `d = {}` in NASA single-shot compile mode
        if is_string_literal {
            parse_quote! { #value_expr.to_string() }
        } else {
            value_expr
        }
    } else {
        value_expr
    };

    // DEPYLER-0449: Detect if base is serde_json::Value (needs .as_object_mut())
    // DEPYLER-0560: Also detect if base is HashMap<String, serde_json::Value>
    // DEPYLER-1017: Skip serde_json logic in NASA mode
    let (needs_as_object_mut, needs_json_value_wrap) = if ctx.type_mapper.nasa_mode {
        (false, false)
    } else if let HirExpr::Var(base_name) = base {
        if !is_numeric_index {
            // DEPYLER-0449: Helper function to check if name looks like JSON value
            let check_json_name_heuristic = |name_str: &str| {
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
            };

            // Check actual type from var_types
            if let Some(base_type) = ctx.var_types.get(base_name) {
                match base_type {
                    // Pure serde_json::Value - needs .as_object_mut()
                    Type::Custom(s) if s == "serde_json::Value" || s == "Value" => (true, false),
                    // HashMap<String, serde_json::Value> - needs json!() wrap on values
                    // DEPYLER-0449: Dict with serde_json::Value values is actually a
                    // serde_json::Value JSON object, so we need .as_object_mut()
                    Type::Dict(_, val_type) => {
                        let val_is_json = match val_type.as_ref() {
                            Type::Unknown => true,
                            Type::Custom(s) => s == "serde_json::Value" || s == "Value",
                            _ => false,
                        };
                        // When dict has Value values, it's a serde_json::Value JSON object
                        (val_is_json, val_is_json)
                    }
                    // DEPYLER-0449: Unknown type - fall back to name heuristic
                    Type::Unknown => check_json_name_heuristic(base_name.as_str()),
                    // Any other type - also try heuristic for common JSON var names
                    _ => check_json_name_heuristic(base_name.as_str()),
                }
            } else {
                // Fallback heuristic: check variable name patterns
                check_json_name_heuristic(base_name.as_str())
            }
        } else {
            (false, false)
        }
    } else {
        (false, false)
    };

    // DEPYLER-0472: Wrap value in serde_json::Value when assigning to Value dicts
    // DEPYLER-0560: Also wrap when dict value type is serde_json::Value
    // DEPYLER-1017: Skip serde_json in NASA mode
    // Check if value needs wrapping (not already json!() or Value variant)
    let final_value_expr = if (needs_as_object_mut || needs_json_value_wrap) && !ctx.type_mapper.nasa_mode {
        // Check if value_expr is already a json!() or Value expression
        let value_str = quote! { #value_expr }.to_string();
        if value_str.contains("serde_json :: json !") || value_str.contains("serde_json :: Value") {
            // Already wrapped, use as-is
            value_expr
        } else if value_str.contains("HashMap") || value_str.contains("let mut map") {
            // DEPYLER-0669: HashMap block expressions can't go in json!() macro
            // Use serde_json::to_value() for proper conversion
            ctx.needs_serde_json = true;
            parse_quote! { serde_json::to_value(#value_expr).unwrap() }
        } else {
            // Need to wrap in serde_json::json!() for HashMap<String, Value>
            // Use json!() instead of to_value() for consistency with dict literals
            ctx.needs_serde_json = true;
            parse_quote! { serde_json::json!(#value_expr) }
        }
    } else {
        value_expr
    };

    // DEPYLER-1050: In NASA mode, wrap values in DepylerValue when function returns HashMap<String, DepylerValue>
    // This handles subscript assignments like `d["key"] = "value"` where d is returned as heterogeneous dict
    let final_value_expr = if ctx.type_mapper.nasa_mode {
        // Check if function return type requires DepylerValue
        let return_needs_depyler_value = if let Some(Type::Dict(_, val_type)) = &ctx.current_return_type {
            matches!(val_type.as_ref(), Type::Unknown)
        } else {
            false
        };

        // Check if target variable requires DepylerValue
        let var_needs_depyler_value = if let HirExpr::Var(base_name) = base {
            if let Some(Type::Dict(_, val_type)) = ctx.var_types.get(base_name) {
                matches!(val_type.as_ref(), Type::Unknown)
            } else {
                false
            }
        } else {
            false
        };

        if return_needs_depyler_value || var_needs_depyler_value {
            ctx.needs_depyler_value_enum = true;
            // Wrap based on value type
            let value_str = quote! { #final_value_expr }.to_string();
            if value_str.contains("DepylerValue::") {
                // Already wrapped
                final_value_expr
            } else if value_str.contains(".to_string()") || value_str.starts_with('"') {
                parse_quote! { DepylerValue::Str(#final_value_expr) }
            } else {
                // Default to Str wrapping with format for unknown types
                parse_quote! { DepylerValue::Str(format!("{:?}", #final_value_expr)) }
            }
        } else {
            final_value_expr
        }
    } else {
        final_value_expr
    };

    // DEPYLER-0567/DEPYLER-1027/DEPYLER-1029: Convert keys based on dict's key type
    // In NASA mode, only convert to String if the dict's key type is String
    // If dict has int keys (e.g., Dict[int, str]), keep keys as integers
    let dict_has_int_keys = if let HirExpr::Var(base_name) = base {
        ctx.var_types.get(base_name).is_some_and(|t| {
            matches!(t, Type::Dict(key_type, _) if matches!(key_type.as_ref(), Type::Int))
        })
    } else {
        false
    };

    // DEPYLER-1073: Check if dict has float keys (uses DepylerValue)
    let dict_has_float_keys = if let HirExpr::Var(base_name) = base {
        ctx.var_types.get(base_name).is_some_and(|t| {
            matches!(t, Type::Dict(key_type, _) if matches!(key_type.as_ref(), Type::Float))
        })
    } else {
        false
    };

    let final_index = if dict_has_float_keys {
        // DEPYLER-1073: Float keys use DepylerValue for Hash/Eq support
        ctx.needs_depyler_value_enum = true;
        // Wrap float literal in DepylerValue::Float()
        if matches!(index, HirExpr::Literal(crate::hir::Literal::Float(_))) {
            parse_quote! { DepylerValue::Float(#final_index) }
        } else if matches!(index, HirExpr::Literal(crate::hir::Literal::Int(_))) {
            // Int literal used as float key - convert to float
            parse_quote! { DepylerValue::Float(#final_index as f64) }
        } else {
            // Variable - use From trait
            parse_quote! { DepylerValue::from(#final_index) }
        }
    } else if !is_numeric_index && !dict_has_int_keys {
        let idx_str = quote! { #final_index }.to_string();
        // Check if already has .to_string() - handle spaces from quote! macro
        // quote! produces ". to_string ( )" with spaces, not ".to_string()"
        if idx_str.contains("to_string") {
            final_index
        } else if idx_str.starts_with('"') {
            // String literal like "key" → "key".to_string()
            parse_quote! { #final_index.to_string() }
        } else {
            // DEPYLER-1027: Non-string keys (integers, variables) also need .to_string()
            // for HashMap<String, ...> in NASA single-shot compile mode
            // Examples: d[42] = "v" → d.insert(42.to_string(), ...)
            //           d[n] = "v" → d.insert(n.to_string(), ...)
            parse_quote! { #final_index.to_string() }
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
            // DEPYLER-0661: Clone variable keys to avoid move-after-use errors
            // When key is a variable like `word_lower`, it may be used elsewhere
            // (e.g., words.push(word_lower)) so we clone to prevent move
            let needs_clone = matches!(index, HirExpr::Var(_));
            if needs_clone {
                Ok(quote! { #base_expr.insert(#final_index.clone(), #final_value_expr); })
            } else {
                Ok(quote! { #base_expr.insert(#final_index, #final_value_expr); })
            }
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
            // DEPYLER-0661: Clone variable keys to avoid move-after-use errors
            let needs_clone = matches!(index, HirExpr::Var(_));
            if needs_clone {
                Ok(quote! { #chain.insert(#final_index.clone(), #final_value_expr); })
            } else {
                Ok(quote! { #chain.insert(#final_index, #final_value_expr); })
            }
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
/// DEPYLER-1064: Handle DepylerValue tuple unpacking
/// When the value expression produces a DepylerValue (e.g., from dict access),
/// inject get_tuple_elem() calls with type coercion for each target.
///
/// # Complexity: 9 (within ≤10 target)
#[inline]
#[allow(clippy::unnecessary_to_owned)] // HashSet<String> requires owned String for contains()
pub(crate) fn codegen_assign_tuple(
    targets: &[AssignTarget],
    value: &HirExpr,
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

    // DEPYLER-1064: Handle native tuples of DepylerValue elements (e.g., Tuple[Any, Any, Any])
    // These become (DepylerValue, DepylerValue, DepylerValue) in Rust, use positional access .0, .1, .2
    if is_native_depyler_tuple(value, ctx) {
        if let Some(ref symbols) = all_symbols {
            // Generate: let _tuple_tmp = <value_expr>;
            //          let var0 = _tuple_tmp.0<extraction>;
            //          let var1 = _tuple_tmp.1<extraction>;
            let temp_var = syn::Ident::new("_tuple_tmp", proc_macro2::Span::call_site());

            // Pre-collect type info and mutability before mutable borrows
            let target_info: Vec<_> = symbols
                .iter()
                .map(|s| {
                    let target_type = ctx.var_types.get(*s).cloned();
                    let is_mutable = ctx.mutable_vars.contains(*s);
                    (*s, target_type, is_mutable)
                })
                .collect();

            // Declare all variables and register types for DepylerValue unpacking
            for (s, target_type, _) in &target_info {
                if !ctx.is_declared(s) {
                    ctx.declare_var(s);
                }
                // DEPYLER-1064: Register unpacked variables with Unknown type if no annotation
                // This ensures string method calls can detect DepylerValue variables later
                if target_type.is_none() {
                    ctx.var_types.insert(s.to_string(), Type::Unknown);
                }
            }

            let assignments: Vec<proc_macro2::TokenStream> = target_info
                .iter()
                .enumerate()
                .map(|(idx, (s, target_type, is_mutable))| {
                    let ident = safe_ident(s);
                    let index = syn::Index::from(idx);

                    let mut_token = if *is_mutable {
                        quote! { mut }
                    } else {
                        quote! {}
                    };

                    // Generate extraction with type coercion if target type is known
                    // Use positional access (.0, .1, .2) for native tuples
                    match target_type {
                        Some(Type::Int) => {
                            quote! { let #mut_token #ident: i32 = #temp_var.#index.to_i64() as i32; }
                        }
                        Some(Type::Float) => {
                            quote! { let #mut_token #ident: f64 = #temp_var.#index.to_f64(); }
                        }
                        Some(Type::String) => {
                            quote! { let #mut_token #ident: String = #temp_var.#index.to_string(); }
                        }
                        Some(Type::Bool) => {
                            quote! { let #mut_token #ident: bool = #temp_var.#index.to_bool(); }
                        }
                        _ => {
                            // No known type, keep as DepylerValue
                            quote! { let #mut_token #ident = #temp_var.#index.clone(); }
                        }
                    }
                })
                .collect();

            return Ok(quote! {
                let #temp_var = #value_expr;
                #(#assignments)*
            });
        }
    }

    // DEPYLER-1064: Check if value produces DepylerValue (e.g., from dict access)
    // If so, inject get_tuple_elem() calls with type coercion for each target
    if expr_produces_depyler_value(value, ctx) {
        if let Some(ref symbols) = all_symbols {
            // Generate: let _tuple_tmp = <value_expr>;
            //          let var0 = _tuple_tmp.get_tuple_elem(0)<extraction>;
            //          let var1 = _tuple_tmp.get_tuple_elem(1)<extraction>;
            let temp_var = syn::Ident::new("_tuple_tmp", proc_macro2::Span::call_site());
            let num_targets = symbols.len();

            // Pre-collect type info and mutability before mutable borrows
            let target_info: Vec<_> = symbols
                .iter()
                .map(|s| {
                    let target_type = ctx.var_types.get(*s).cloned();
                    let is_mutable = ctx.mutable_vars.contains(*s);
                    (*s, target_type, is_mutable)
                })
                .collect();

            // Declare all variables
            for s in symbols.iter() {
                if !ctx.is_declared(s) {
                    ctx.declare_var(s);
                }
            }

            let assignments: Vec<proc_macro2::TokenStream> = target_info
                .iter()
                .enumerate()
                .map(|(idx, (s, target_type, is_mutable))| {
                    let ident = safe_ident(s);
                    let idx_lit = syn::LitInt::new(&idx.to_string(), proc_macro2::Span::call_site());

                    let mut_token = if *is_mutable {
                        quote! { mut }
                    } else {
                        quote! {}
                    };

                    // Generate extraction with type coercion if target type is known
                    match target_type {
                        Some(Type::Int) => {
                            quote! { let #mut_token #ident: i32 = #temp_var.get_tuple_elem(#idx_lit).to_i64() as i32; }
                        }
                        Some(Type::Float) => {
                            quote! { let #mut_token #ident: f64 = #temp_var.get_tuple_elem(#idx_lit).to_f64(); }
                        }
                        Some(Type::String) => {
                            quote! { let #mut_token #ident: String = #temp_var.get_tuple_elem(#idx_lit).to_string(); }
                        }
                        Some(Type::Bool) => {
                            quote! { let #mut_token #ident: bool = #temp_var.get_tuple_elem(#idx_lit).to_bool(); }
                        }
                        _ => {
                            // No known type, keep as DepylerValue
                            quote! { let #mut_token #ident = #temp_var.get_tuple_elem(#idx_lit); }
                        }
                    }
                })
                .collect();

            // Add validation for tuple length at runtime
            let num_lit = syn::LitInt::new(&num_targets.to_string(), proc_macro2::Span::call_site());

            return Ok(quote! {
                let #temp_var = #value_expr;
                // Validate tuple has expected number of elements
                let _ = #temp_var.extract_tuple(#num_lit);
                #(#assignments)*
            });
        }
    }

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

                // DEPYLER-0671: Check if value_expr is a Vec from split/collect
                // Can't destructure Vec directly - need iterator-based unpacking
                let value_str = quote! { #value_expr }.to_string();
                let is_vec_from_split = value_str.contains("collect ::<Vec")
                    || value_str.contains("collect::< Vec")
                    || value_str.contains(".collect ()")
                    || (value_str.contains("splitn") && value_str.contains("collect"));

                if is_vec_from_split {
                    // DEPYLER-0671: Vec from split() can't be destructured directly
                    // Strategy: Store in temp Vec, then index into it
                    // From: let (a, b) = s.split(...).collect::<Vec<_>>()
                    // To: let _parts = s.split(...).collect::<Vec<_>>();
                    //     let a = _parts.get(0).cloned().unwrap_or_default();
                    //     let b = _parts.get(1).cloned().unwrap_or_default();
                    let parts_name =
                        syn::Ident::new("_split_parts", proc_macro2::Span::call_site());

                    let assignments: Vec<_> = symbols
                        .iter()
                        .enumerate()
                        .map(|(idx, s)| {
                            let ident = safe_ident(s);
                            ctx.declare_var(s);
                            let mut_token = if ctx.mutable_vars.contains(*s) {
                                quote! { mut }
                            } else {
                                quote! {}
                            };
                            let idx_lit = syn::Index::from(idx);
                            quote! { let #mut_token #ident = #parts_name.get(#idx_lit).cloned().unwrap_or_default(); }
                        })
                        .collect();

                    Ok(quote! {
                        let #parts_name = #value_expr;
                        #(#assignments)*
                    })
                } else if all_declared {
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
pub(crate) fn infer_try_body_return_type(body: &[HirStmt], ctx: &CodeGenContext) -> Option<Type> {
    for stmt in body {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                // Found a return with a value - infer its type
                return Some(infer_expr_return_type(expr, ctx));
            }
            HirStmt::While { body: inner, .. } | HirStmt::For { body: inner, .. } => {
                // Check inside loops
                if let Some(ty) = infer_try_body_return_type(inner, ctx) {
                    return Some(ty);
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                // Check inside if/else
                if let Some(ty) = infer_try_body_return_type(then_body, ctx) {
                    return Some(ty);
                }
                if let Some(else_stmts) = else_body {
                    if let Some(ty) = infer_try_body_return_type(else_stmts, ctx) {
                        return Some(ty);
                    }
                }
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                if let Some(ty) = infer_try_body_return_type(body, ctx) {
                    return Some(ty);
                }
                for h in handlers {
                    if let Some(ty) = infer_try_body_return_type(&h.body, ctx) {
                        return Some(ty);
                    }
                }
                if let Some(else_stmts) = orelse {
                    if let Some(ty) = infer_try_body_return_type(else_stmts, ctx) {
                        return Some(ty);
                    }
                }
                if let Some(final_stmts) = finalbody {
                    if let Some(ty) = infer_try_body_return_type(final_stmts, ctx) {
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
fn infer_expr_return_type(expr: &HirExpr, ctx: &CodeGenContext) -> Type {
    match expr {
        HirExpr::Var(name) => {
            // Look up variable type in context
            ctx.var_types.get(name).cloned().unwrap_or(Type::Unknown)
        }
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
                "len" | "count" | "wait" | "poll" | "returncode" => Type::Int,
                "is_empty" | "startswith" | "endswith" | "exists" | "is_file" | "is_dir" => {
                    Type::Bool
                }
                _ => Type::Unknown,
            }
        }
        HirExpr::Attribute { attr, .. } => {
            match attr.as_str() {
                "returncode" => Type::Int,
                "stdout" | "stderr" => Type::String,
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
        HirExpr::Binary { left, right, .. } => {
            let left_type = infer_expr_return_type(left, ctx);
            if !matches!(left_type, Type::Unknown) {
                left_type
            } else {
                infer_expr_return_type(right, ctx)
            }
        }
        _ => Type::Unknown,
    }
}

/// DEPYLER-0565: Convert HIR Type to closure return type tokens
#[inline]
pub(crate) fn try_return_type_to_tokens(ty: &Type) -> proc_macro2::TokenStream {
    // Delegate to the main type mapper to support all types (Option, Vec, etc.)
    // Create a temporary context since type mapping shouldn't depend on it for basic types
    // (hir_type_to_tokens only uses ctx for advanced resolution which we might miss here,
    // but it's better than hardcoding)
    // Actually, hir_type_to_tokens ignores ctx in most cases.
    // We can't easily construct a dummy context here, but we can call it if we update the signature.
    // However, since we can't update signature easily without changing all calls, let's duplicate
    // the delegation logic or simplified version.
    
    // Better: Update to match hir_type_to_tokens logic for common types
    match ty {
        Type::Int => quote! { i32 },
        Type::Float => quote! { f64 },
        Type::String => quote! { String },
        Type::Bool => quote! { bool },
        Type::None => quote! { () },
        Type::Unknown => quote! { () },
        _ => {
            // Fallback: We can't call hir_type_to_tokens because we don't have ctx.
            // But we can handle the Option<Value> case which is common for hoisted vars.
            if let Type::Optional(inner) = ty {
                let inner_tokens = try_return_type_to_tokens(inner);
                quote! { Option<#inner_tokens> }
            } else if let Type::Custom(name) = ty {
                if name == "serde_json::Value" || name == "Value" {
                    quote! { serde_json::Value }
                } else {
                    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                    quote! { #ident }
                }
            } else if let Type::Tuple(elems) = ty {
                let elem_tokens: Vec<_> = elems.iter().map(try_return_type_to_tokens).collect();
                quote! { (#(#elem_tokens),*) }
            } else {
                quote! { () }
            }
        }
    }
}

// DEPYLER-COVERAGE-95: handler_ends_with_exit and handler_contains_raise moved to expr_analysis module

/// DEPYLER-0578: Try to detect and generate json.load(sys.stdin) pattern
/// Pattern: try { data = json.load(sys.stdin) } except JSONDecodeError as e: { print; exit }
/// Returns: let data = match serde_json::from_reader(...) { Ok(d) => d, Err(e) => { ... } };
///
/// # Complexity
/// 8 (pattern matching + code generation)
#[inline]
pub(crate) fn try_generate_json_stdin_match(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, HirExpr, Literal, Type};
    use crate::rust_gen::expr_analysis::{
        contains_floor_div, expr_returns_usize, extract_divisor_from_floor_div,
        handler_contains_raise, is_file_creating_expr, is_nested_function_recursive,
        is_stdio_expr,
    };
    use crate::rust_gen::stmt_gen_complex::{
        captures_outer_scope, extract_fields_from_expr, extract_fields_recursive,
    };
    use crate::rust_gen::var_analysis::{
        extract_assigned_symbols, extract_walrus_recursive, find_assigned_expr,
        is_var_direct_or_simple_in_expr, is_var_used_as_dict_key_in_expr,
        is_var_used_in_assign_target, is_var_used_in_expr,
    };

    // ============ expr_returns_usize tests ============

    #[test]
    fn test_expr_returns_usize_len_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("arr".to_string())),
            method: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_count_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "count".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_capacity_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("vec".to_string())),
            method: "capacity".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_len_call() {
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_range_call() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(10))],
            kwargs: vec![],
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_binary_with_usize() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("vec".to_string())),
                method: "len".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_simple_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_simple_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_other_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("vec".to_string())),
            method: "push".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_returns_usize(&expr));
    }

    #[test]
    fn test_expr_returns_usize_other_call() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_returns_usize(&expr));
    }

    // ============ is_iterator_producing_expr tests ============

    #[test]
    fn test_is_iterator_producing_generator_exp() {
        let expr = HirExpr::GeneratorExp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_iter_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("vec".to_string())),
            method: "iter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_map_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("vec".to_string())),
                method: "iter".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            method: "map".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_filter_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "filter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_enumerate_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "enumerate".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_zip_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "zip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_take_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "take".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_skip_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "skip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_rev_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "rev".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_call_map() {
        let expr = HirExpr::Call {
            func: "map".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_call_filter() {
        let expr = HirExpr::Call {
            func: "filter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_call_reversed() {
        let expr = HirExpr::Call {
            func: "reversed".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_simple_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_producing_non_iter_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("vec".to_string())),
            method: "push".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_iterator_producing_expr(&expr));
    }

    // ============ looks_like_option_expr tests ============

    #[test]
    fn test_looks_like_option_ok_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("result".to_string())),
            method: "ok".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_get_one_arg() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_get_with_default() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "get".to_string(),
            args: vec![
                HirExpr::Literal(Literal::String("key".to_string())),
                HirExpr::Literal(Literal::Int(0)),
            ],
            kwargs: vec![],
        };
        assert!(!looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_other_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("vec".to_string())),
            method: "push".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_simple_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!looks_like_option_expr(&expr));
    }

    // ============ is_file_creating_expr tests ============

    #[test]
    fn test_is_file_creating_open_call() {
        let expr = HirExpr::Call {
            func: "open".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_file_create() {
        // File.create() is a method call, not a Call with "File::create" func
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "create".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_file_open_method() {
        // File.open() is a method call, not a Call
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "open".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_other_call() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_file_creating_expr(&expr));
    }

    // ============ is_stdio_expr tests ============

    #[test]
    fn test_is_stdio_stdout() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "stdout".to_string(),
        };
        assert!(is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_stderr() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "stderr".to_string(),
        };
        assert!(is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_other_attr() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "path".to_string(),
        };
        assert!(!is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_non_attribute() {
        let expr = HirExpr::Var("stdout".to_string());
        assert!(!is_stdio_expr(&expr));
    }

    // ============ is_dict_index_access tests ============

    #[test]
    fn test_is_dict_index_access_str_key() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("config".to_string())),
            index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_var_key_dict_name() {
        // Variable named "dict" with var key - still returns true because of dict-like name heuristics
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("dict".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_var_key_array_name() {
        // Variable with non-dict name and var key - returns false
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(!is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_int_key() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(!is_dict_index_access(&expr));
    }

    // ============ is_pure_expression tests ============

    #[test]
    fn test_is_pure_expr_literal() {
        assert!(is_pure_expression(&HirExpr::Literal(Literal::Int(42))));
        assert!(is_pure_expression(&HirExpr::Literal(Literal::Float(3.14))));
        assert!(is_pure_expression(&HirExpr::Literal(Literal::Bool(true))));
        assert!(is_pure_expression(&HirExpr::Literal(Literal::String("test".to_string()))));
    }

    #[test]
    fn test_is_pure_expr_var() {
        assert!(is_pure_expression(&HirExpr::Var("x".to_string())));
    }

    #[test]
    fn test_is_pure_expr_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_expr_call() {
        let expr = HirExpr::Call {
            func: "foo".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_expr_method_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression(&expr));
    }

    // ============ extract_exception_type tests ============

    #[test]
    fn test_extract_exception_type_var() {
        let expr = HirExpr::Var("ValueError".to_string());
        assert_eq!(extract_exception_type(&expr), "ValueError");
    }

    #[test]
    fn test_extract_exception_type_call() {
        let expr = HirExpr::Call {
            func: "RuntimeError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("message".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "RuntimeError");
    }

    #[test]
    fn test_extract_exception_type_other() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    // ============ Additional helper tests ============

    #[test]
    fn test_is_iterator_chain_multiple() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("vec".to_string())),
                    method: "iter".to_string(),
                    args: vec![],
                    kwargs: vec![],
                }),
                method: "map".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            method: "filter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_is_iterator_chain_non_iter_then_iter() {
        // vec.push(...).iter() - push doesn't produce iterator, but iter does
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("vec".to_string())),
                method: "push".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            method: "iter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_expr_returns_usize_nested_binary() {
        // (x + y.len()) + 1
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("y".to_string())),
                    method: "len".to_string(),
                    args: vec![],
                    kwargs: vec![],
                }),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(expr_returns_usize(&expr));
    }

    // ============ apply_type_conversion tests ============

    #[test]
    fn test_apply_type_conversion_to_string() {
        let value_expr: syn::Expr = syn::parse_quote!(x);
        let result = apply_type_conversion(value_expr, &Type::String);
        assert!(result.to_token_stream().to_string().contains("to_string"));
    }

    #[test]
    fn test_apply_type_conversion_to_other() {
        let value_expr: syn::Expr = syn::parse_quote!(x);
        let result = apply_type_conversion(value_expr, &Type::Bool);
        // For non-numeric non-string types, just returns the original
        assert_eq!(result.to_token_stream().to_string(), "x");
    }

    // ============ looks_like_option_expr tests ============

    #[test]
    fn test_looks_like_option_expr_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_expr_method_ok() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("result".to_string())),
            method: "ok".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_expr_method_get_no_default() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_expr_method_get_with_default() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "get".to_string(),
            args: vec![
                HirExpr::Literal(Literal::String("key".to_string())),
                HirExpr::Literal(Literal::Int(0)),
            ],
            kwargs: vec![],
        };
        // With default, it's not Option
        assert!(!looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_expr_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Eq,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::None)),
        };
        // Binary comparisons don't trigger option detection
        assert!(!looks_like_option_expr(&expr));
    }

    // ============ extract_assigned_symbols tests ============

    #[test]
    fn test_extract_assigned_symbols_simple() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
        ];
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
    }

    #[test]
    fn test_extract_assigned_symbols_multiple() {
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
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
        assert!(symbols.contains("y"));
    }

    #[test]
    fn test_extract_assigned_symbols_in_if() {
        let stmts = vec![
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
        let symbols = extract_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
    }

    // ============ extract_toplevel_assigned_symbols tests ============

    #[test]
    fn test_extract_toplevel_assigned_symbols_simple() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
        ];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        assert!(symbols.contains("x"));
    }

    #[test]
    fn test_extract_toplevel_assigned_symbols_in_if() {
        let stmts = vec![
            HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("nested".to_string()),
                    value: HirExpr::Literal(Literal::Int(1)),
                    type_annotation: None,
                }],
                else_body: None,
            },
        ];
        let symbols = extract_toplevel_assigned_symbols(&stmts);
        // Includes assignments from if body (considered same scope level)
        assert!(symbols.contains("nested"));
    }

    // ============ is_var_used_in_expr tests ============

    #[test]
    fn test_is_var_used_in_expr_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(is_var_used_in_expr("x", &expr));
        assert!(!is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_is_var_used_in_expr_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_var_used_in_expr("x", &expr));
        assert!(!is_var_used_in_expr("y", &expr));
    }

    #[test]
    fn test_is_var_used_in_expr_call() {
        let expr = HirExpr::Call {
            func: "foo".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_var_used_in_expr("x", &expr));
    }

    #[test]
    fn test_is_var_used_in_expr_index() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("i".to_string())),
        };
        assert!(is_var_used_in_expr("arr", &expr));
        assert!(is_var_used_in_expr("i", &expr));
    }

    // ============ is_var_used_in_assign_target tests ============

    #[test]
    fn test_is_var_used_in_assign_target_symbol_match() {
        let target = AssignTarget::Symbol("x".to_string());
        // Returns true when symbol name matches
        assert!(is_var_used_in_assign_target("x", &target));
    }

    #[test]
    fn test_is_var_used_in_assign_target_symbol_no_match() {
        let target = AssignTarget::Symbol("x".to_string());
        assert!(!is_var_used_in_assign_target("y", &target));
    }

    #[test]
    fn test_is_var_used_in_assign_target_index() {
        let target = AssignTarget::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("i".to_string())),
        };
        assert!(is_var_used_in_assign_target("i", &target));
        assert!(is_var_used_in_assign_target("arr", &target));
    }

    #[test]
    fn test_is_var_used_in_assign_target_tuple() {
        let target = AssignTarget::Tuple(vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ]);
        // Recursively checks tuple elements
        assert!(is_var_used_in_assign_target("a", &target));
        assert!(!is_var_used_in_assign_target("c", &target));
    }

    // ============ is_var_reassigned_in_stmt tests ============

    #[test]
    fn test_is_var_reassigned_in_stmt_assign() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        };
        assert!(is_var_reassigned_in_stmt("x", &stmt));
        assert!(!is_var_reassigned_in_stmt("y", &stmt));
    }

    #[test]
    fn test_is_var_reassigned_in_stmt_for_empty_body() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![],
        };
        // For checks body for reassignment, not the loop target
        assert!(!is_var_reassigned_in_stmt("i", &stmt));
    }

    #[test]
    fn test_is_var_reassigned_in_stmt_for_with_body() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            }],
        };
        assert!(is_var_reassigned_in_stmt("x", &stmt));
        assert!(!is_var_reassigned_in_stmt("i", &stmt));
    }

    // ============ is_var_used_in_stmt tests ============

    #[test]
    fn test_is_var_used_in_stmt_expr() {
        let stmt = HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        });
        assert!(is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_is_var_used_in_stmt_assign_value() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("y".to_string()),
            value: HirExpr::Var("x".to_string()),
            type_annotation: None,
        };
        assert!(is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_is_var_used_in_stmt_if_condition() {
        let stmt = HirStmt::If {
            condition: HirExpr::Var("x".to_string()),
            then_body: vec![],
            else_body: None,
        };
        assert!(is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_is_var_used_in_stmt_while() {
        let stmt = HirStmt::While {
            condition: HirExpr::Binary {
                op: BinOp::Lt,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(10))),
            },
            body: vec![],
        };
        assert!(is_var_used_in_stmt("x", &stmt));
    }

    #[test]
    fn test_is_var_used_in_stmt_return() {
        let stmt = HirStmt::Return(Some(HirExpr::Var("x".to_string())));
        assert!(is_var_used_in_stmt("x", &stmt));
    }

    // ============ is_dict_augassign_pattern tests ============

    #[test]
    fn test_is_dict_augassign_pattern_not_binary() {
        // is_dict_augassign_pattern requires value to be a Binary expr with Index left
        let target = AssignTarget::Index {
            base: Box::new(HirExpr::Var("counts".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        let value = HirExpr::Literal(Literal::Int(1));
        // Literal is not a Binary, so returns false
        assert!(!is_dict_augassign_pattern(&target, &value));
    }

    #[test]
    fn test_is_dict_augassign_pattern_non_subscript() {
        let target = AssignTarget::Symbol("x".to_string());
        let value = HirExpr::Literal(Literal::Int(1));
        assert!(!is_dict_augassign_pattern(&target, &value));
    }

    // ============ is_dict_with_value_type tests ============

    #[test]
    fn test_is_dict_with_value_type_unknown() {
        // is_dict_with_value_type returns true for Dict with Unknown or Value/json custom type
        let t = Type::Dict(Box::new(Type::String), Box::new(Type::Unknown));
        assert!(is_dict_with_value_type(&t));
    }

    #[test]
    fn test_is_dict_with_value_type_json_value() {
        let t = Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Custom("serde_json::Value".to_string())),
        );
        assert!(is_dict_with_value_type(&t));
    }

    #[test]
    fn test_is_dict_with_value_type_not_dict() {
        let t = Type::Int;
        assert!(!is_dict_with_value_type(&t));
    }

    // ============ find_var_position_in_tuple tests ============

    #[test]
    fn test_find_var_position_in_tuple_found() {
        let targets = vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
            AssignTarget::Symbol("c".to_string()),
        ];
        assert_eq!(find_var_position_in_tuple("b", &targets), Some(1));
    }

    #[test]
    fn test_find_var_position_in_tuple_not_found() {
        let targets = vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ];
        assert_eq!(find_var_position_in_tuple("x", &targets), None);
    }

    #[test]
    fn test_find_var_position_in_tuple_nested() {
        let targets = vec![
            AssignTarget::Tuple(vec![
                AssignTarget::Symbol("inner".to_string()),
            ]),
            AssignTarget::Symbol("outer".to_string()),
        ];
        // Does not search in nested tuples
        assert_eq!(find_var_position_in_tuple("inner", &targets), None);
        assert_eq!(find_var_position_in_tuple("outer", &targets), Some(1));
    }

    // Note: codegen_pass_stmt, codegen_break_stmt, codegen_continue_stmt tests
    // are in crate::rust_gen::control_stmt_helpers::tests

    // ============ extract_walrus tests ============

    #[test]
    fn test_extract_walrus_from_condition_no_walrus() {
        let condition = HirExpr::Var("x".to_string());
        let (assigns, new_cond) = extract_walrus_from_condition(&condition);
        assert!(assigns.is_empty());
        assert!(matches!(new_cond, HirExpr::Var(name) if name == "x"));
    }

    #[test]
    fn test_extract_walrus_from_condition_with_walrus() {
        let condition = HirExpr::NamedExpr {
            target: "y".to_string(),
            value: Box::new(HirExpr::Call {
                func: "get_value".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        };
        let (assigns, new_cond) = extract_walrus_from_condition(&condition);
        assert_eq!(assigns.len(), 1);
        assert_eq!(assigns[0].0, "y");
        // New condition should be the variable
        assert!(matches!(new_cond, HirExpr::Var(name) if name == "y"));
    }

    #[test]
    fn test_extract_walrus_recursive_in_binary() {
        let mut assigns = Vec::new();
        let expr = HirExpr::Binary {
            op: BinOp::And,
            left: Box::new(HirExpr::NamedExpr {
                target: "a".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            }),
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        let _result = extract_walrus_recursive(&expr, &mut assigns);
        assert_eq!(assigns.len(), 1);
        assert_eq!(assigns[0].0, "a");
    }

    // ============ needs_boxed_dyn_write tests ============

    #[test]
    fn test_needs_boxed_dyn_write_both_branches_use_var() {
        let then_body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("f".to_string())),
            method: "write".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        let else_body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("f".to_string())),
            method: "write".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        // Without knowing the assignments, this returns false
        // because f is not assigned in the branches
        assert!(!needs_boxed_dyn_write("f", &then_body, &else_body));
    }

    // ============ is_var_used_as_dict_key tests ============

    #[test]
    fn test_is_var_used_as_dict_key_in_expr_index() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("dict".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_var_used_as_dict_key_in_expr("key", &expr));
        assert!(!is_var_used_as_dict_key_in_expr("dict", &expr));
    }

    #[test]
    fn test_is_var_used_as_dict_key_in_expr_not_index() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_var_used_as_dict_key_in_expr("x", &expr));
    }

    #[test]
    fn test_is_var_used_as_dict_key_in_stmt_assign() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("dict".to_string())),
                index: Box::new(HirExpr::Var("k".to_string())),
            },
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        };
        assert!(is_var_used_as_dict_key_in_stmt("k", &stmt));
    }

    // ============ is_var_direct_or_simple_in_expr tests ============

    #[test]
    fn test_is_var_direct_simple_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(is_var_direct_or_simple_in_expr("x", &expr));
    }

    #[test]
    fn test_is_var_direct_simple_in_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        // Only matches direct Var, not nested in Binary
        assert!(!is_var_direct_or_simple_in_expr("x", &expr));
    }

    #[test]
    fn test_is_var_direct_simple_not_found() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_var_direct_or_simple_in_expr("x", &expr));
    }

    // ============ expr_infers_float tests ============

    #[test]
    fn test_expr_infers_float_literal_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Float(3.14));
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_literal_int() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_var_with_float_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("x".to_string(), Type::Float);
        let expr = HirExpr::Var("x".to_string());
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_var_without_float_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("x".to_string(), Type::Int);
        let expr = HirExpr::Var("x".to_string());
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_with_float_return() {
        let mut ctx = CodeGenContext::default();
        ctx.function_return_types
            .insert("compute".to_string(), Type::Float);
        let expr = HirExpr::Call {
            func: "compute".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_call_with_int_return() {
        let mut ctx = CodeGenContext::default();
        ctx.function_return_types
            .insert("compute".to_string(), Type::Int);
        let expr = HirExpr::Call {
            func: "compute".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_mul_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Literal(Literal::Float(2.0))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_div_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Float(6.0))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_add_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Float(1.5))),
            right: Box::new(HirExpr::Literal(Literal::Float(2.5))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_int_only() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_unary_neg_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Float(1.5))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_if_expr_both_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Float(1.0))),
            orelse: Box::new(HirExpr::Literal(Literal::Float(2.0))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_if_expr_mixed() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Float(1.0))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        // Both branches must be float
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_callable_with_float_return() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "f".to_string(),
            Type::Function {
                params: vec![Type::Float],
                ret: Box::new(Type::Float),
            },
        );
        let expr = HirExpr::Call {
            func: "f".to_string(),
            args: vec![HirExpr::Literal(Literal::Float(1.0))],
            kwargs: vec![],
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_generic_callable_with_float_return() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "g".to_string(),
            Type::Generic {
                base: "Callable".to_string(),
                params: vec![
                    Type::List(Box::new(Type::Float)), // param types
                    Type::Float,                       // return type
                ],
            },
        );
        let expr = HirExpr::Call {
            func: "g".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_pow_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Pow,
            left: Box::new(HirExpr::Literal(Literal::Float(2.0))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_mod_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Mod,
            left: Box::new(HirExpr::Literal(Literal::Float(5.5))),
            right: Box::new(HirExpr::Literal(Literal::Float(2.0))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    // ============ extract_fields_from_expr tests ============

    #[test]
    fn test_extract_fields_from_expr_attribute() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "input_file".to_string(),
        };
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.contains("input_file"));
    }

    #[test]
    fn test_extract_fields_from_expr_filters_dest_field() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "command".to_string(),
        };
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.is_empty()); // dest_field should be filtered out
    }

    #[test]
    fn test_extract_fields_from_expr_wrong_var() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("other".to_string())),
            attr: "field".to_string(),
        };
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.is_empty());
    }

    #[test]
    fn test_extract_fields_from_expr_call_args() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "output".to_string(),
            }],
            kwargs: vec![],
        };
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.contains("output"));
    }

    #[test]
    fn test_extract_fields_from_expr_binary() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "x".to_string(),
            }),
            right: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "y".to_string(),
            }),
        };
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.contains("x"));
        assert!(fields.contains("y"));
    }

    #[test]
    fn test_extract_fields_from_expr_unary() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "flag".to_string(),
            }),
        };
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.contains("flag"));
    }

    #[test]
    fn test_extract_fields_from_expr_if_expr() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "cond".to_string(),
            }),
            body: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "then_val".to_string(),
            }),
            orelse: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "else_val".to_string(),
            }),
        };
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.contains("cond"));
        assert!(fields.contains("then_val"));
        assert!(fields.contains("else_val"));
    }

    #[test]
    fn test_extract_fields_from_expr_index() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "list".to_string(),
            }),
            index: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "idx".to_string(),
            }),
        };
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.contains("list"));
        assert!(fields.contains("idx"));
    }

    #[test]
    fn test_extract_fields_from_expr_list() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::List(vec![
            HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "item1".to_string(),
            },
            HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "item2".to_string(),
            },
        ]);
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.contains("item1"));
        assert!(fields.contains("item2"));
    }

    #[test]
    fn test_extract_fields_from_expr_tuple() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::Tuple(vec![HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "val".to_string(),
        }]);
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.contains("val"));
    }

    #[test]
    fn test_extract_fields_from_expr_set() {
        let mut fields = std::collections::HashSet::new();
        let expr = HirExpr::Set(vec![HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "elem".to_string(),
        }]);
        extract_fields_from_expr(&expr, "args", "command", &mut fields);
        assert!(fields.contains("elem"));
    }

    // ============ extract_fields_recursive tests ============

    #[test]
    fn test_extract_fields_recursive_expr_stmt() {
        let mut fields = std::collections::HashSet::new();
        let stmts = vec![HirStmt::Expr(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "value".to_string(),
        })];
        extract_fields_recursive(&stmts, "args", "command", &mut fields);
        assert!(fields.contains("value"));
    }

    #[test]
    fn test_extract_fields_recursive_assign() {
        let mut fields = std::collections::HashSet::new();
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "source".to_string(),
            },
            type_annotation: None,
        }];
        extract_fields_recursive(&stmts, "args", "command", &mut fields);
        assert!(fields.contains("source"));
    }

    #[test]
    fn test_extract_fields_recursive_if_stmt() {
        let mut fields = std::collections::HashSet::new();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "cond".to_string(),
            },
            then_body: vec![HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "then".to_string(),
            })],
            else_body: Some(vec![HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "else".to_string(),
            })]),
        }];
        extract_fields_recursive(&stmts, "args", "command", &mut fields);
        assert!(fields.contains("cond"));
        assert!(fields.contains("then"));
        assert!(fields.contains("else"));
    }

    #[test]
    fn test_extract_fields_recursive_while_stmt() {
        let mut fields = std::collections::HashSet::new();
        let stmts = vec![HirStmt::While {
            condition: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "running".to_string(),
            },
            body: vec![HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "body_val".to_string(),
            })],
        }];
        extract_fields_recursive(&stmts, "args", "command", &mut fields);
        assert!(fields.contains("running"));
        assert!(fields.contains("body_val"));
    }

    #[test]
    fn test_extract_fields_recursive_for_stmt() {
        let mut fields = std::collections::HashSet::new();
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("item".to_string()),
            iter: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "items".to_string(),
            },
            body: vec![HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "loop_val".to_string(),
            })],
        }];
        extract_fields_recursive(&stmts, "args", "command", &mut fields);
        assert!(fields.contains("items"));
        assert!(fields.contains("loop_val"));
    }

    #[test]
    fn test_extract_fields_recursive_try_stmt() {
        let mut fields = std::collections::HashSet::new();
        let stmts = vec![HirStmt::Try {
            body: vec![HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "try_val".to_string(),
            })],
            handlers: vec![ExceptHandler {
                exception_type: Some("ValueError".to_string()),
                name: Some("e".to_string()),
                body: vec![HirStmt::Expr(HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "handler_val".to_string(),
                })],
            }],
            orelse: Some(vec![HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "else_val".to_string(),
            })]),
            finalbody: Some(vec![HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "finally_val".to_string(),
            })]),
        }];
        extract_fields_recursive(&stmts, "args", "command", &mut fields);
        assert!(fields.contains("try_val"));
        assert!(fields.contains("handler_val"));
        assert!(fields.contains("else_val"));
        assert!(fields.contains("finally_val"));
    }

    #[test]
    fn test_extract_fields_recursive_with_stmt() {
        let mut fields = std::collections::HashSet::new();
        let stmts = vec![HirStmt::With {
            context: HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("args".to_string())),
                    attr: "file".to_string(),
                }],
                kwargs: vec![],
            },
            target: Some("f".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "with_body".to_string(),
            })],
            is_async: false,
        }];
        extract_fields_recursive(&stmts, "args", "command", &mut fields);
        assert!(fields.contains("file"));
        assert!(fields.contains("with_body"));
    }

    // ============ extract_nested_indices_tokens tests ============

    #[test]
    fn test_extract_nested_indices_simple() {
        let mut ctx = CodeGenContext::default();
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("dict".to_string())),
            index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
        };
        let result = extract_nested_indices_tokens(&expr, &mut ctx);
        assert!(result.is_ok());
        let (_, indices) = result.unwrap();
        assert_eq!(indices.len(), 1);
    }

    #[test]
    fn test_extract_nested_indices_nested() {
        let mut ctx = CodeGenContext::default();
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("nested".to_string())),
                index: Box::new(HirExpr::Literal(Literal::String("outer".to_string()))),
            }),
            index: Box::new(HirExpr::Literal(Literal::String("inner".to_string()))),
        };
        let result = extract_nested_indices_tokens(&expr, &mut ctx);
        assert!(result.is_ok());
        let (_, indices) = result.unwrap();
        assert_eq!(indices.len(), 2);
    }

    // ============ expr_infers_float tests ============

    #[test]
    fn test_expr_infers_float_literal() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Float(3.14));
        assert!(expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_int_literal() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!expr_infers_float(&expr, &ctx));
    }

    #[test]
    fn test_expr_infers_float_binary_with_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Float(1.0))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(expr_infers_float(&expr, &ctx));
    }

    // ============ is_pure_expression tests ============

    #[test]
    fn test_is_pure_expression_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_expression_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_expression_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_expression_call_not_pure() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression(&expr));
    }

    #[test]
    fn test_is_pure_expression_method_call_not_pure() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        assert!(!is_pure_expression(&expr));
    }

    // ============ looks_like_option_expr tests ============

    #[test]
    fn test_looks_like_option_expr_get_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(looks_like_option_expr(&expr));
    }

    #[test]
    fn test_looks_like_option_expr_simple_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!looks_like_option_expr(&expr));
    }

    // ============ is_file_creating_expr tests ============

    #[test]
    fn test_is_file_creating_expr_open_call() {
        let expr = HirExpr::Call {
            func: "open".to_string(),
            args: vec![HirExpr::Literal(Literal::String("file.txt".to_string()))],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_expr_not_file() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_stdio_expr_stdout() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "stdout".to_string(),
        };
        assert!(is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_expr_stderr() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "stderr".to_string(),
        };
        assert!(is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_expr_not_stdio() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_stdio_expr(&expr));
    }

    // ============ handler_ends_with_exit extra tests ============

    #[test]
    fn test_handler_ends_with_exit_call() {
        let handler_body = vec![HirStmt::Expr(HirExpr::Call {
            func: "exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&handler_body));
    }

    #[test]
    fn test_handler_ends_with_sys_exit_call() {
        let handler_body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("sys".to_string())),
            method: "exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&handler_body));
    }

    #[test]
    fn test_handler_ends_with_no_exit() {
        let handler_body = vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!handler_ends_with_exit(&handler_body));
    }

    // ============ handler_contains_raise extra tests ============

    #[test]
    fn test_handler_raise_present() {
        let handler_body = vec![HirStmt::Raise {
            exception: Some(HirExpr::Var("ValueError".to_string())),
            cause: None,
        }];
        assert!(handler_contains_raise(&handler_body));
    }

    #[test]
    fn test_handler_no_raise_present() {
        let handler_body = vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!handler_contains_raise(&handler_body));
    }

    // ============ find_variable_type extra tests ============

    #[test]
    fn test_find_var_type_annotated() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        let result = find_variable_type("x", &stmts);
        assert_eq!(result, Some(Type::Int));
    }

    #[test]
    fn test_find_var_type_not_found() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("y".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        let result = find_variable_type("x", &stmts);
        assert_eq!(result, None);
    }


    // ============ infer_expr_return_type extra tests ============

    #[test]
    fn test_infer_return_type_int() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Int(42));
        let result = infer_expr_return_type(&expr, &ctx);
        assert_eq!(result, Type::Int);
    }

    #[test]
    fn test_infer_return_type_float() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Float(3.14));
        let result = infer_expr_return_type(&expr, &ctx);
        assert_eq!(result, Type::Float);
    }

    #[test]
    fn test_infer_return_type_string() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        let result = infer_expr_return_type(&expr, &ctx);
        assert_eq!(result, Type::String);
    }

    #[test]
    fn test_infer_return_type_bool() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Bool(true));
        let result = infer_expr_return_type(&expr, &ctx);
        assert_eq!(result, Type::Bool);
    }

    // ============ is_dict_augassign_pattern extra tests ============

    #[test]
    fn test_dict_augassign_pattern_positive() {
        let target = AssignTarget::Index {
            base: Box::new(HirExpr::Var("counter".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        let value = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("counter".to_string())),
                index: Box::new(HirExpr::Var("key".to_string())),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_dict_augassign_pattern(&target, &value));
    }

    #[test]
    fn test_dict_augassign_pattern_negative() {
        let target = AssignTarget::Symbol("x".to_string());
        let value = HirExpr::Literal(Literal::Int(1));
        assert!(!is_dict_augassign_pattern(&target, &value));
    }

    // ============ handler_ends_with_exit tests ============

    #[test]
    fn test_handler_ends_with_exit_true() {
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "sys.exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_false() {
        let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0))))];
        assert!(!handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_empty() {
        let body: Vec<HirStmt> = vec![];
        assert!(!handler_ends_with_exit(&body));
    }

    // ============ handler_contains_raise tests ============

    #[test]
    fn test_handler_contains_raise_true() {
        let body = vec![HirStmt::Raise {
            exception: Some(HirExpr::Call {
                func: "ValueError".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            cause: None,
        }];
        assert!(handler_contains_raise(&body));
    }

    #[test]
    fn test_handler_contains_raise_false() {
        let body = vec![HirStmt::Expr(HirExpr::Var("x".to_string()))];
        assert!(!handler_contains_raise(&body));
    }

    #[test]
    fn test_handler_contains_raise_empty() {
        let body: Vec<HirStmt> = vec![];
        assert!(!handler_contains_raise(&body));
    }

    // ============ contains_floor_div tests ============

    #[test]
    fn test_contains_floor_div_true() {
        let expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_false() {
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(!contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_nested() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Binary {
                op: BinOp::FloorDiv,
                left: Box::new(HirExpr::Literal(Literal::Int(10))),
                right: Box::new(HirExpr::Literal(Literal::Int(3))),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!contains_floor_div(&expr));
    }

    // ============ extract_string_literal tests ============

    #[test]
    fn test_extract_string_literal_string() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert_eq!(extract_string_literal(&expr), "hello");
    }

    #[test]
    fn test_extract_string_literal_non_string() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert_eq!(extract_string_literal(&expr), "");
    }

    #[test]
    fn test_extract_string_literal_var() {
        let expr = HirExpr::Var("x".to_string());
        assert_eq!(extract_string_literal(&expr), "");
    }

    // ============ extract_kwarg_string tests ============

    #[test]
    fn test_extract_kwarg_string_found() {
        let kwargs = vec![
            ("name".to_string(), HirExpr::Literal(Literal::String("test".to_string()))),
        ];
        assert_eq!(extract_kwarg_string(&kwargs, "name"), Some("test".to_string()));
    }

    #[test]
    fn test_extract_kwarg_string_not_found() {
        let kwargs = vec![
            ("other".to_string(), HirExpr::Literal(Literal::String("test".to_string()))),
        ];
        assert_eq!(extract_kwarg_string(&kwargs, "name"), None);
    }

    #[test]
    fn test_extract_kwarg_string_not_string() {
        let kwargs = vec![
            ("name".to_string(), HirExpr::Literal(Literal::Int(42))),
        ];
        assert_eq!(extract_kwarg_string(&kwargs, "name"), None);
    }

    // ============ extract_kwarg_bool tests ============

    #[test]
    fn test_extract_kwarg_bool_true() {
        // Function expects HirExpr::Var("True") not Literal::Bool
        let kwargs = vec![
            ("flag".to_string(), HirExpr::Var("True".to_string())),
        ];
        assert_eq!(extract_kwarg_bool(&kwargs, "flag"), Some(true));
    }

    #[test]
    fn test_extract_kwarg_bool_false() {
        // Function expects HirExpr::Var("False") not Literal::Bool
        let kwargs = vec![
            ("flag".to_string(), HirExpr::Var("False".to_string())),
        ];
        assert_eq!(extract_kwarg_bool(&kwargs, "flag"), Some(false));
    }

    #[test]
    fn test_extract_kwarg_bool_not_found() {
        let kwargs = vec![
            ("other".to_string(), HirExpr::Var("True".to_string())),
        ];
        assert_eq!(extract_kwarg_bool(&kwargs, "flag"), None);
    }

    #[test]
    fn test_extract_kwarg_bool_not_bool() {
        // Literal::Bool doesn't match the Var("True")/Var("False") pattern
        let kwargs = vec![
            ("flag".to_string(), HirExpr::Literal(Literal::Bool(true))),
        ];
        assert_eq!(extract_kwarg_bool(&kwargs, "flag"), None);
    }

    // ============ is_nested_function_recursive tests ============

    #[test]
    fn test_is_nested_function_recursive_true() {
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "factorial".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(5))],
            kwargs: vec![],
        })];
        assert!(is_nested_function_recursive("factorial", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_false() {
        let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))];
        assert!(!is_nested_function_recursive("factorial", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_different_name() {
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "other_func".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!is_nested_function_recursive("factorial", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_in_if() {
        let body = vec![HirStmt::If {
            condition: HirExpr::Var("cond".to_string()),
            then_body: vec![HirStmt::Expr(HirExpr::Call {
                func: "fib".to_string(),
                args: vec![],
                kwargs: vec![],
            })],
            else_body: None,
        }];
        assert!(is_nested_function_recursive("fib", &body));
    }

    // ============ to_pascal_case tests ============

    #[test]
    fn test_to_pascal_case_simple() {
        assert_eq!(to_pascal_case("hello"), "Hello");
    }

    #[test]
    fn test_to_pascal_case_underscore() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    }

    #[test]
    fn test_to_pascal_case_hyphen() {
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
    }

    #[test]
    fn test_to_pascal_case_mixed() {
        assert_eq!(to_pascal_case("run_all-tests"), "RunAllTests");
    }

    #[test]
    fn test_to_pascal_case_single_char() {
        assert_eq!(to_pascal_case("a"), "A");
    }

    #[test]
    fn test_to_pascal_case_empty() {
        assert_eq!(to_pascal_case(""), "");
    }

    // ============ find_assigned_expr tests ============

    #[test]
    fn test_find_assigned_expr_found() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }];
        let result = find_assigned_expr("x", &stmts);
        assert!(result.is_some());
    }

    #[test]
    fn test_find_assigned_expr_not_found() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("y".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }];
        let result = find_assigned_expr("x", &stmts);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_assigned_expr_empty() {
        let stmts: Vec<HirStmt> = vec![];
        let result = find_assigned_expr("x", &stmts);
        assert!(result.is_none());
    }

    // ============ is_numpy_value_expr tests ============

    #[test]
    fn test_is_numpy_value_expr_zeros() {
        let ctx = CodeGenContext::default();
        // Function matches bare function names only, not "np.zeros"
        let expr = HirExpr::Call {
            func: "zeros".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_value_expr_ones() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "ones".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_value_expr_arange() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "arange".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_value_expr_array() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "array".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_value_expr_not_numpy() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_numpy_value_expr(&expr, &ctx));
    }

    #[test]
    fn test_is_numpy_value_expr_literal() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_numpy_value_expr(&expr, &ctx));
    }

    // Note: needs_type_conversion tests are in expr_analysis.rs

    // ============ is_json_value_method_chain_or_fallback tests ============

    #[test]
    fn test_is_json_value_method_chain_as_str() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "as_str".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        // Check if the function recognizes json value method chains
        // The actual result depends on ctx state, but function should not panic
        let _ = is_json_value_method_chain_or_fallback(&expr, &ctx);
    }

    #[test]
    fn test_is_json_value_method_chain_as_i64() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "as_i64".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        let _ = is_json_value_method_chain_or_fallback(&expr, &ctx);
    }

    #[test]
    fn test_is_json_value_method_chain_regular_method() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "upper".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        // Regular string method, not JSON value
        assert!(!is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    // ============ extract_divisor_from_floor_div tests ============

    #[test]
    fn test_extract_divisor_from_floor_div_ok() {
        let expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_divisor_from_floor_div_not_floor_div() {
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_divisor_from_floor_div_not_binary() {
        let expr = HirExpr::Literal(Literal::Int(42));
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_err());
    }

    // ============ contains_floor_div comprehensive tests ============

    #[test]
    fn test_contains_floor_div_direct() {
        let expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_regular_div() {
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(!contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_nested_in_left() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(inner),
            right: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_nested_in_right() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(5))),
            right: Box::new(inner),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_unary() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(inner),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_call_args() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![inner],
            kwargs: vec![],
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_method_call_object() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::MethodCall {
            object: Box::new(inner),
            method: "to_string".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_method_call_args() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![inner],
            kwargs: vec![],
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_index_base() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Index {
            base: Box::new(inner),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_index_index() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(inner),
        };
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_list() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            inner,
        ]);
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_tuple() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            inner,
        ]);
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_in_set() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Set(vec![inner]);
        assert!(contains_floor_div(&expr));
    }

    #[test]
    fn test_contains_floor_div_none() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!contains_floor_div(&expr));
    }

    // ============ extract_divisor_from_floor_div recursive tests ============

    #[test]
    fn test_extract_divisor_nested_in_left() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(inner),
            right: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_divisor_nested_in_right() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(5))),
            right: Box::new(inner),
        };
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_divisor_in_unary() {
        let inner = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(inner),
        };
        let result = extract_divisor_from_floor_div(&expr);
        assert!(result.is_ok());
    }

    // ============ is_file_creating_expr additional tests ============

    #[test]
    fn test_is_file_creating_expr_file_create_via_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "create".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_expr_file_open_via_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "open".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_expr_via_attribute() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("std".to_string())),
                attr: "File".to_string(),
            }),
            method: "create".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_expr_literal_returns_false() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_file_creating_expr(&expr));
    }

    // ============ is_stdio_expr additional tests ============

    #[test]
    fn test_is_stdio_expr_not_sys_module() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("os".to_string())),
            attr: "stdout".to_string(),
        };
        assert!(!is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_expr_wrong_attr() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "path".to_string(),
        };
        assert!(!is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_expr_var_not_attr() {
        let expr = HirExpr::Var("stdout".to_string());
        assert!(!is_stdio_expr(&expr));
    }

    // ============ is_dict_index_access comprehensive tests ============

    #[test]
    fn test_is_dict_index_access_with_string_key() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("data".to_string())),
            index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_mydict_var() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("mydict".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_config() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("config".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_settings() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("settings".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_params() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("params".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_options() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("options".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_env() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("env".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_json() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("json_data".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_single_char_d() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("d".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_single_char_m() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("m".to_string())),
            index: Box::new(HirExpr::Var("key".to_string())),
        };
        assert!(is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_list_with_int_key() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(!is_dict_index_access(&expr));
    }

    #[test]
    fn test_is_dict_index_access_not_index_expr() {
        let expr = HirExpr::Var("data".to_string());
        assert!(!is_dict_index_access(&expr));
    }

    // ============ needs_boxed_dyn_write tests ============

    #[test]
    fn test_needs_boxed_dyn_write_heterogeneous() {
        use crate::hir::HirStmt;
        let then_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("output".to_string()),
            value: HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Literal(Literal::String("out.txt".to_string()))],
                kwargs: vec![],
            },
            type_annotation: None,
        }];
        let else_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("output".to_string()),
            value: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("sys".to_string())),
                attr: "stdout".to_string(),
            },
            type_annotation: None,
        }];
        assert!(needs_boxed_dyn_write("output", &then_body, &else_body));
    }

    #[test]
    fn test_needs_boxed_dyn_write_homogeneous_files() {
        use crate::hir::HirStmt;
        let then_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("output".to_string()),
            value: HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Literal(Literal::String("out.txt".to_string()))],
                kwargs: vec![],
            },
            type_annotation: None,
        }];
        let else_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("output".to_string()),
            value: HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Literal(Literal::String("other.txt".to_string()))],
                kwargs: vec![],
            },
            type_annotation: None,
        }];
        // Both are files - homogeneous, no Box needed
        assert!(!needs_boxed_dyn_write("output", &then_body, &else_body));
    }

    #[test]
    fn test_needs_boxed_dyn_write_missing_assignment() {
        use crate::hir::HirStmt;
        let then_body = vec![HirStmt::Expr(HirExpr::Var("x".to_string()))];
        let else_body = vec![HirStmt::Expr(HirExpr::Var("y".to_string()))];
        assert!(!needs_boxed_dyn_write("output", &then_body, &else_body));
    }

    // ============ to_pascal_case additional tests ============

    #[test]
    fn test_to_pascal_case_basic() {
        assert_eq!(to_pascal_case("hello"), "Hello");
    }

    #[test]
    fn test_to_pascal_case_hyphen_sep() {
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
    }

    #[test]
    fn test_to_pascal_case_underscore_sep() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    }

    #[test]
    fn test_to_pascal_case_mixed_delims() {
        assert_eq!(to_pascal_case("hello-world_test"), "HelloWorldTest");
    }

    #[test]
    fn test_to_pascal_case_empty_str() {
        assert_eq!(to_pascal_case(""), "");
    }

    #[test]
    fn test_to_pascal_case_single_letter() {
        assert_eq!(to_pascal_case("a"), "A");
    }

    #[test]
    fn test_to_pascal_case_already_capitalized() {
        assert_eq!(to_pascal_case("Hello"), "Hello");
    }

    // ============ handler_ends_with_exit tests ============

    #[test]
    fn test_handler_ends_with_exit_exit_call() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_sys_exit_call() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "sys.exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_sys_exit_method() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("sys".to_string())),
            method: "exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_no_exit() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_empty_body() {
        let body: Vec<crate::hir::HirStmt> = vec![];
        assert!(!handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_wrong_method_object() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("os".to_string())),
            method: "exit".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!handler_ends_with_exit(&body));
    }

    // ============ handler_contains_raise tests ============

    #[test]
    fn test_handler_contains_raise_with_raise() {
        use crate::hir::HirStmt;
        let body = vec![
            HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            HirStmt::Raise {
                exception: Some(HirExpr::Var("e".to_string())),
                cause: None,
            },
        ];
        assert!(handler_contains_raise(&body));
    }

    #[test]
    fn test_handler_contains_raise_without_raise() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!handler_contains_raise(&body));
    }

    #[test]
    fn test_handler_contains_raise_empty_body() {
        let body: Vec<crate::hir::HirStmt> = vec![];
        assert!(!handler_contains_raise(&body));
    }

    #[test]
    fn test_handler_contains_raise_bare_raise() {
        use crate::hir::HirStmt;
        let body = vec![HirStmt::Raise {
            exception: None,
            cause: None,
        }];
        assert!(handler_contains_raise(&body));
    }

    // ============ is_dict_with_value_type tests ============

    #[test]
    fn test_is_dict_with_value_type_dict_unknown() {
        assert!(is_dict_with_value_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Unknown)
        )));
    }

    #[test]
    fn test_is_dict_with_value_type_dict_value() {
        assert!(is_dict_with_value_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Custom("Value".to_string()))
        )));
    }

    #[test]
    fn test_is_dict_with_value_type_dict_json_value() {
        assert!(is_dict_with_value_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Custom("serde_json::Value".to_string()))
        )));
    }

    #[test]
    fn test_is_dict_with_value_type_dict_int() {
        assert!(!is_dict_with_value_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
    }

    #[test]
    fn test_is_dict_with_value_type_non_dict_type() {
        assert!(!is_dict_with_value_type(&Type::Int));
    }

    // ============ find_var_position_in_tuple tests ============

    #[test]
    fn test_find_var_position_first() {
        let targets = vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ];
        assert_eq!(find_var_position_in_tuple("a", &targets), Some(0));
    }

    #[test]
    fn test_find_var_position_second() {
        let targets = vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
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

    // ============ is_nested_function_recursive additional tests ============

    #[test]
    fn test_is_nested_function_recursive_via_direct_call() {
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "factorial".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(is_nested_function_recursive("factorial", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_via_return() {
        let body = vec![HirStmt::Return(Some(HirExpr::Call {
            func: "factorial".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        }))];
        assert!(is_nested_function_recursive("factorial", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_not_found() {
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "other_func".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!is_nested_function_recursive("factorial", &body));
    }

    #[test]
    fn test_is_nested_function_recursive_with_empty_body() {
        let body: Vec<HirStmt> = vec![];
        assert!(!is_nested_function_recursive("anything", &body));
    }

    // ============ needs_boxed_dyn_write tests ============

    #[test]
    fn test_needs_boxed_dyn_write_same_type() {
        // Both branches use open() - same file type, no boxing needed
        let then_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("writer".to_string()),
            value: HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Literal(Literal::String("a.txt".to_string()))],
                kwargs: vec![],
            },
            type_annotation: None,
        }];
        let else_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("writer".to_string()),
            value: HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Literal(Literal::String("b.txt".to_string()))],
                kwargs: vec![],
            },
            type_annotation: None,
        }];
        // Same type in both branches - no boxing needed
        assert!(!needs_boxed_dyn_write("writer", &then_body, &else_body));
    }

    #[test]
    fn test_needs_boxed_dyn_write_file_vs_stdout() {
        // One branch uses open() (file), other uses sys.stdout - heterogeneous types
        let then_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("writer".to_string()),
            value: HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Literal(Literal::String("out.txt".to_string()))],
                kwargs: vec![],
            },
            type_annotation: None,
        }];
        let else_body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("writer".to_string()),
            value: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("sys".to_string())),
                attr: "stdout".to_string(),
            },
            type_annotation: None,
        }];
        // Different types - needs boxing
        assert!(needs_boxed_dyn_write("writer", &then_body, &else_body));
    }

    // ============ find_variable_type tests ============

    #[test]
    fn test_find_variable_type_annotated() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        assert_eq!(find_variable_type("x", &stmts), Some(Type::Int));
    }

    #[test]
    fn test_find_variable_type_not_found() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("y".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        assert_eq!(find_variable_type("x", &stmts), None);
    }

    #[test]
    fn test_find_variable_type_no_annotation() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        }];
        // Without annotation, function infers from the value (Int literal -> Type::Int)
        let result = find_variable_type("x", &stmts);
        assert_eq!(result, Some(Type::Int));
    }

    // ============ Additional handler tests (not duplicates) ============

    #[test]
    fn test_handler_ends_with_exit_call_exit_func() {
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_call_sys_dot_exit() {
        let body = vec![HirStmt::Expr(HirExpr::Call {
            func: "sys.exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_method_call() {
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("sys".to_string())),
            method: "exit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })];
        assert!(handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_not_last() {
        let body = vec![
            HirStmt::Expr(HirExpr::Call {
                func: "exit".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        ];
        assert!(!handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_empty_vec() {
        let body: Vec<HirStmt> = vec![];
        assert!(!handler_ends_with_exit(&body));
    }

    #[test]
    fn test_handler_ends_with_exit_wrong_object() {
        let body = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("os".to_string())),
            method: "exit".to_string(),
            args: vec![],
            kwargs: vec![],
        })];
        assert!(!handler_ends_with_exit(&body));
    }

    // ============ handler_contains_raise additional tests ============

    #[test]
    fn test_handler_contains_raise_with_exception() {
        let body = vec![HirStmt::Raise {
            exception: Some(HirExpr::Var("ValueError".to_string())),
            cause: None,
        }];
        assert!(handler_contains_raise(&body));
    }

    #[test]
    fn test_handler_contains_raise_bare() {
        let body = vec![HirStmt::Raise {
            exception: None,
            cause: None,
        }];
        assert!(handler_contains_raise(&body));
    }

    #[test]
    fn test_handler_contains_raise_none() {
        let body = vec![
            HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        ];
        assert!(!handler_contains_raise(&body));
    }

    // ============ is_dict_with_value_type additional tests ============

    #[test]
    fn test_is_dict_with_value_type_with_json() {
        let t = Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Custom("serde_json::Value".to_string())),
        );
        assert!(is_dict_with_value_type(&t));
    }

    #[test]
    fn test_is_dict_with_value_type_with_unknown() {
        let t = Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Unknown),
        );
        assert!(is_dict_with_value_type(&t));
    }

    #[test]
    fn test_is_dict_with_value_type_string() {
        let t = Type::Dict(
            Box::new(Type::String),
            Box::new(Type::String),
        );
        assert!(!is_dict_with_value_type(&t));
    }

    #[test]
    fn test_is_dict_with_value_type_non_dict() {
        assert!(!is_dict_with_value_type(&Type::String));
        assert!(!is_dict_with_value_type(&Type::Int));
        assert!(!is_dict_with_value_type(&Type::List(Box::new(Type::Int))));
    }

    // ============ is_nested_function_recursive additional edge cases ============

    #[test]
    fn test_is_nested_recursive_in_binary() {
        let body = vec![HirStmt::Expr(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Call {
                func: "fib".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        })];
        assert!(is_nested_function_recursive("fib", &body));
    }

    #[test]
    fn test_is_nested_recursive_in_if() {
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Expr(HirExpr::Call {
                func: "recurse".to_string(),
                args: vec![],
                kwargs: vec![],
            })],
            else_body: None,
        }];
        assert!(is_nested_function_recursive("recurse", &body));
    }

    #[test]
    fn test_is_nested_recursive_not_found() {
        let body = vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(42)))];
        assert!(!is_nested_function_recursive("func", &body));
    }

    // ============ extract_string_literal edge cases ============

    #[test]
    fn test_extract_string_literal_success() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert_eq!(extract_string_literal(&expr), "hello");
    }

    #[test]
    fn test_extract_string_literal_empty() {
        let expr = HirExpr::Literal(Literal::String("".to_string()));
        assert_eq!(extract_string_literal(&expr), "");
    }

    #[test]
    fn test_extract_string_literal_int_and_var() {
        assert_eq!(extract_string_literal(&HirExpr::Literal(Literal::Int(42))), "");
        assert_eq!(extract_string_literal(&HirExpr::Var("x".to_string())), "");
    }

    // ============ extract_kwarg edge cases ============

    #[test]
    fn test_extract_kwarg_string_success() {
        let kwargs = vec![("key".to_string(), HirExpr::Literal(Literal::String("value".to_string())))];
        assert_eq!(extract_kwarg_string(&kwargs, "key"), Some("value".to_string()));
    }

    #[test]
    fn test_extract_kwarg_string_missing() {
        let kwargs = vec![("other".to_string(), HirExpr::Literal(Literal::String("value".to_string())))];
        assert_eq!(extract_kwarg_string(&kwargs, "key"), None);
    }

    #[test]
    fn test_extract_kwarg_bool_true_val() {
        let kwargs = vec![("flag".to_string(), HirExpr::Var("True".to_string()))];
        assert_eq!(extract_kwarg_bool(&kwargs, "flag"), Some(true));
    }

    #[test]
    fn test_extract_kwarg_bool_false_val() {
        let kwargs = vec![("flag".to_string(), HirExpr::Var("False".to_string()))];
        assert_eq!(extract_kwarg_bool(&kwargs, "flag"), Some(false));
    }

    #[test]
    fn test_extract_kwarg_bool_missing() {
        let kwargs: Vec<(String, HirExpr)> = vec![];
        assert_eq!(extract_kwarg_bool(&kwargs, "flag"), None);
    }

    // ============ is_file_creating_expr edge cases ============

    #[test]
    fn test_is_file_creating_via_attribute() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("std".to_string())),
                attr: "File".to_string(),
            }),
            method: "create".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_non_file() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("Buffer".to_string())),
            method: "create".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_file_creating_expr(&expr));
    }

    // ============ is_stdio_expr edge cases ============

    #[test]
    fn test_is_stdio_non_sys() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("os".to_string())),
            attr: "stdout".to_string(),
        };
        assert!(!is_stdio_expr(&expr));
    }

    #[test]
    fn test_is_stdio_nested() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("sys".to_string())),
                attr: "io".to_string(),
            }),
            attr: "stdout".to_string(),
        };
        assert!(!is_stdio_expr(&expr));
    }

    // ============ find_variable_type tuple unpacking tests (DEPYLER-0931) ============

    #[test]
    fn test_find_variable_type_tuple_unpacking_first_element() {
        // (a, b, c) = (1, 2, 3) -> a should be Type::Int
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Symbol("b".to_string()),
                AssignTarget::Symbol("c".to_string()),
            ]),
            value: HirExpr::Tuple(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
                HirExpr::Literal(Literal::Int(3)),
            ]),
            type_annotation: None,
        }];
        assert_eq!(find_variable_type("a", &stmts), Some(Type::Int));
    }

    #[test]
    fn test_find_variable_type_tuple_unpacking_second_element() {
        // (a, b, c) = (1, "hello", 3) -> b should be Type::String
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Symbol("b".to_string()),
                AssignTarget::Symbol("c".to_string()),
            ]),
            value: HirExpr::Tuple(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::String("hello".to_string())),
                HirExpr::Literal(Literal::Int(3)),
            ]),
            type_annotation: None,
        }];
        assert_eq!(find_variable_type("b", &stmts), Some(Type::String));
    }

    #[test]
    fn test_find_variable_type_tuple_unpacking_not_found() {
        // (a, b) = (1, 2) -> z should be None
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
        assert_eq!(find_variable_type("z", &stmts), None);
    }

    #[test]
    fn test_find_variable_type_tuple_unpacking_with_type_annotation() {
        // Test when RHS has Type::Tuple annotation
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("x".to_string()),
                AssignTarget::Symbol("y".to_string()),
            ]),
            value: HirExpr::Var("some_tuple".to_string()),
            type_annotation: Some(Type::Tuple(vec![Type::Int, Type::String])),
        }];
        // Variable "z" not in tuple - should be None
        assert_eq!(find_variable_type("z", &stmts), None);
    }

    // ============ find_variable_type recursive search in try/except (DEPYLER-0931) ============

    #[test]
    fn test_find_variable_type_in_try_body() {
        let stmts = vec![HirStmt::Try {
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(42)),
                type_annotation: Some(Type::Int),
            }],
            handlers: vec![],
            orelse: None,
            finalbody: None,
        }];
        assert_eq!(find_variable_type("x", &stmts), Some(Type::Int));
    }

    #[test]
    fn test_find_variable_type_in_except_handler() {
        let stmts = vec![HirStmt::Try {
            body: vec![],
            handlers: vec![crate::hir::ExceptHandler {
                exception_type: Some("ValueError".to_string()),
                name: None,
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("result".to_string()),
                    value: HirExpr::Literal(Literal::String("error".to_string())),
                    type_annotation: Some(Type::String),
                }],
            }],
            orelse: None,
            finalbody: None,
        }];
        assert_eq!(find_variable_type("result", &stmts), Some(Type::String));
    }

    #[test]
    fn test_find_variable_type_in_finally_block() {
        let stmts = vec![HirStmt::Try {
            body: vec![],
            handlers: vec![],
            orelse: None,
            finalbody: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("cleanup".to_string()),
                value: HirExpr::Literal(Literal::Bool(true)),
                type_annotation: Some(Type::Bool),
            }]),
        }];
        assert_eq!(find_variable_type("cleanup", &stmts), Some(Type::Bool));
    }

    // ============ find_variable_type recursive search in if/else (DEPYLER-0931) ============

    #[test]
    fn test_find_variable_type_in_then_body() {
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("count".to_string()),
                value: HirExpr::Literal(Literal::Int(10)),
                type_annotation: Some(Type::Int),
            }],
            else_body: None,
        }];
        assert_eq!(find_variable_type("count", &stmts), Some(Type::Int));
    }

    #[test]
    fn test_find_variable_type_in_else_body() {
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(false)),
            then_body: vec![],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("fallback".to_string()),
                value: HirExpr::Literal(Literal::Float(3.14)),
                type_annotation: Some(Type::Float),
            }]),
        }];
        assert_eq!(find_variable_type("fallback", &stmts), Some(Type::Float));
    }

    #[test]
    fn test_find_variable_type_prefers_then_over_else() {
        // If same variable defined in both, should find in then_body first
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("val".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: Some(Type::Int),
            }],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("val".to_string()),
                value: HirExpr::Literal(Literal::String("str".to_string())),
                type_annotation: Some(Type::String),
            }]),
        }];
        // Should find Int from then_body first
        assert_eq!(find_variable_type("val", &stmts), Some(Type::Int));
    }

    // ============ is_json_value_method_chain_or_fallback edge cases ============

    #[test]
    fn test_json_value_chain_get_on_unknown_value_type() {
        // Dict with Unknown value type -> should return true (treated as Value)
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "data".to_string(),
            Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
        );
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    #[test]
    fn test_json_value_chain_get_with_cloned_unwrap_or_default() {
        // data.get("key").cloned().unwrap_or_default() chain
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "data".to_string(),
            Type::Dict(
                Box::new(Type::String),
                Box::new(Type::Custom("serde_json::Value".to_string())),
            ),
        );
        let base = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        let cloned = HirExpr::MethodCall {
            object: Box::new(base),
            method: "cloned".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        let expr = HirExpr::MethodCall {
            object: Box::new(cloned),
            method: "unwrap_or_default".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    #[test]
    fn test_json_value_chain_fallback_when_needs_serde_json() {
        // Untracked local dict but context needs serde_json -> should return true
        let mut ctx = CodeGenContext::default();
        ctx.needs_serde_json = true;
        // No type info for "local_data"
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("local_data".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    #[test]
    fn test_json_value_chain_non_get_method() {
        // Non-chain method like .insert() should return false
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "data".to_string(),
            Type::Dict(
                Box::new(Type::String),
                Box::new(Type::Custom("serde_json::Value".to_string())),
            ),
        );
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "insert".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    #[test]
    fn test_json_value_chain_non_value_dict() {
        // Dict with String value type -> should return false
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "data".to_string(),
            Type::Dict(Box::new(Type::String), Box::new(Type::String)),
        );
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(!is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    #[test]
    fn test_json_value_chain_variable_not_found() {
        // Variable not in context and needs_serde_json is false
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("unknown_var".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(!is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    #[test]
    fn test_json_value_chain_unwrap_chain() {
        // data.get("key").unwrap() chain
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "data".to_string(),
            Type::Dict(
                Box::new(Type::String),
                Box::new(Type::Custom("json::Value".to_string())),
            ),
        );
        let base = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        let expr = HirExpr::MethodCall {
            object: Box::new(base),
            method: "unwrap".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    #[test]
    fn test_json_value_chain_unwrap_or_chain() {
        // data.get("key").unwrap_or(default) chain
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert(
            "data".to_string(),
            Type::Dict(
                Box::new(Type::String),
                Box::new(Type::Custom("Value".to_string())),
            ),
        );
        let base = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        let expr = HirExpr::MethodCall {
            object: Box::new(base),
            method: "unwrap_or".to_string(),
            args: vec![HirExpr::Literal(Literal::String("default".to_string()))],
            kwargs: vec![],
        };
        assert!(is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    #[test]
    fn test_json_value_chain_non_var_object() {
        // When object is not a Var - should return false
        let ctx = CodeGenContext::default();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Call {
                func: "get_data".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(!is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    #[test]
    fn test_json_value_chain_non_method_call_expr() {
        // Non-MethodCall expression should return false
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("data".to_string());
        assert!(!is_json_value_method_chain_or_fallback(&expr, &ctx));
    }

    // === Tests for apply_negated_truthiness (DEPYLER-COVERAGE-95) ===

    #[test]
    fn test_apply_negated_truthiness_bool_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("flag".to_string(), Type::Bool);
        let operand = HirExpr::Var("flag".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { !flag };
        let result = apply_negated_truthiness(&operand, cond_expr.clone(), &ctx);
        // Bool type should keep negation as-is
        assert_eq!(quote::quote!(#result).to_string(), quote::quote!(#cond_expr).to_string());
    }

    #[test]
    fn test_apply_negated_truthiness_string_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("s".to_string(), Type::String);
        let operand = HirExpr::Var("s".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { !s };
        let result = apply_negated_truthiness(&operand, cond_expr, &ctx);
        // String type should convert to is_empty()
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("is_empty"));
    }

    #[test]
    fn test_apply_negated_truthiness_list_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("items".to_string(), Type::List(Box::new(Type::Int)));
        let operand = HirExpr::Var("items".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { !items };
        let result = apply_negated_truthiness(&operand, cond_expr, &ctx);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("is_empty"));
    }

    #[test]
    fn test_apply_negated_truthiness_optional_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("opt".to_string(), Type::Optional(Box::new(Type::Int)));
        let operand = HirExpr::Var("opt".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { !opt };
        let result = apply_negated_truthiness(&operand, cond_expr, &ctx);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("is_none"));
    }

    #[test]
    fn test_apply_negated_truthiness_int_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("num".to_string(), Type::Int);
        let operand = HirExpr::Var("num".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { !num };
        let result = apply_negated_truthiness(&operand, cond_expr, &ctx);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("== 0"));
    }

    #[test]
    fn test_apply_negated_truthiness_float_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("val".to_string(), Type::Float);
        let operand = HirExpr::Var("val".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { !val };
        let result = apply_negated_truthiness(&operand, cond_expr, &ctx);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("== 0.0"));
    }

    #[test]
    fn test_apply_negated_truthiness_already_converted_is_empty() {
        let ctx = CodeGenContext::default();
        let operand = HirExpr::Var("items".to_string());
        // Already converted to is_empty()
        let cond_expr: syn::Expr = syn::parse_quote! { items.is_empty() };
        let result = apply_negated_truthiness(&operand, cond_expr.clone(), &ctx);
        // Should return as-is
        assert_eq!(quote::quote!(#result).to_string(), quote::quote!(#cond_expr).to_string());
    }

    #[test]
    fn test_apply_negated_truthiness_already_converted_is_none() {
        let ctx = CodeGenContext::default();
        let operand = HirExpr::Var("opt".to_string());
        // Already converted to is_none()
        let cond_expr: syn::Expr = syn::parse_quote! { opt.is_none() };
        let result = apply_negated_truthiness(&operand, cond_expr.clone(), &ctx);
        // Should return as-is
        assert_eq!(quote::quote!(#result).to_string(), quote::quote!(#cond_expr).to_string());
    }

    #[test]
    fn test_apply_negated_truthiness_self_items_attr() {
        let ctx = CodeGenContext::default();
        let operand = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("self".to_string())),
            attr: "items".to_string(),
        };
        let cond_expr: syn::Expr = syn::parse_quote! { !self.items };
        let result = apply_negated_truthiness(&operand, cond_expr, &ctx);
        // self.items should be treated as collection
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("is_empty"));
    }

    // === Tests for apply_truthiness_conversion (DEPYLER-COVERAGE-95) ===

    #[test]
    fn test_apply_truthiness_conversion_bool() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("flag".to_string(), Type::Bool);
        let condition = HirExpr::Var("flag".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { flag };
        let result = apply_truthiness_conversion(&condition, cond_expr.clone(), &ctx);
        // Bool should not be converted
        assert_eq!(quote::quote!(#result).to_string(), quote::quote!(#cond_expr).to_string());
    }

    #[test]
    fn test_apply_truthiness_conversion_string() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("s".to_string(), Type::String);
        let condition = HirExpr::Var("s".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { s };
        let result = apply_truthiness_conversion(&condition, cond_expr, &ctx);
        let result_str = quote::quote!(#result).to_string();
        // String should convert to !s.is_empty()
        assert!(result_str.contains("is_empty"));
    }

    #[test]
    fn test_apply_truthiness_conversion_optional() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("opt".to_string(), Type::Optional(Box::new(Type::Int)));
        let condition = HirExpr::Var("opt".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { opt };
        let result = apply_truthiness_conversion(&condition, cond_expr, &ctx);
        let result_str = quote::quote!(#result).to_string();
        // Optional should convert to opt.is_some()
        assert!(result_str.contains("is_some"));
    }

    #[test]
    fn test_apply_truthiness_conversion_int() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("n".to_string(), Type::Int);
        let condition = HirExpr::Var("n".to_string());
        let cond_expr: syn::Expr = syn::parse_quote! { n };
        let result = apply_truthiness_conversion(&condition, cond_expr, &ctx);
        let result_str = quote::quote!(#result).to_string();
        // Int should convert to n != 0
        assert!(result_str.contains("!= 0"));
    }

    #[test]
    fn test_apply_truthiness_conversion_not_expr() {
        // Test `not x` where x is non-boolean
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("items".to_string(), Type::List(Box::new(Type::Int)));
        let condition = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Var("items".to_string())),
        };
        let cond_expr: syn::Expr = syn::parse_quote! { !items };
        let result = apply_truthiness_conversion(&condition, cond_expr, &ctx);
        let result_str = quote::quote!(#result).to_string();
        // Should delegate to apply_negated_truthiness
        assert!(result_str.contains("is_empty"));
    }

    // === Tests for extract_nested_indices_tokens (DEPYLER-COVERAGE-95) ===

    #[test]
    fn test_extract_nested_indices_single_index() {
        let mut ctx = CodeGenContext::default();
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        let (base, indices) = extract_nested_indices_tokens(&expr, &mut ctx).unwrap();
        let base_str = quote::quote!(#base).to_string();
        assert!(base_str.contains("arr"));
        assert_eq!(indices.len(), 1);
    }

    #[test]
    fn test_extract_nested_indices_double_index() {
        let mut ctx = CodeGenContext::default();
        let inner = HirExpr::Index {
            base: Box::new(HirExpr::Var("matrix".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        let expr = HirExpr::Index {
            base: Box::new(inner),
            index: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        let (base, indices) = extract_nested_indices_tokens(&expr, &mut ctx).unwrap();
        let base_str = quote::quote!(#base).to_string();
        assert!(base_str.contains("matrix"));
        assert_eq!(indices.len(), 2);
    }

    #[test]
    fn test_extract_nested_indices_var_base() {
        let mut ctx = CodeGenContext::default();
        // Just a variable, no indices
        let expr = HirExpr::Var("x".to_string());
        let (base, indices) = extract_nested_indices_tokens(&expr, &mut ctx).unwrap();
        let base_str = quote::quote!(#base).to_string();
        assert!(base_str.contains("x"));
        assert_eq!(indices.len(), 0);
    }

    // === Tests for captures_outer_scope (DEPYLER-COVERAGE-95) ===

    #[test]
    fn test_captures_outer_scope_no_capture() {
        use crate::hir::HirParam;
        let params = vec![HirParam {
            name: "x".to_string(),
            ty: Type::Int,
            default: None,
            is_vararg: false,
        }];
        let body = vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))];
        let outer_vars = std::collections::HashSet::new();
        assert!(!captures_outer_scope(&params, &body, &outer_vars));
    }

    #[test]
    fn test_captures_outer_scope_with_capture() {
        use crate::hir::HirParam;
        let params = vec![HirParam {
            name: "x".to_string(),
            ty: Type::Int,
            default: None,
            is_vararg: false,
        }];
        // Uses 'y' which is not a parameter
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: crate::hir::BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Var("y".to_string())),
        }))];
        let mut outer_vars = std::collections::HashSet::new();
        outer_vars.insert("y".to_string());
        assert!(captures_outer_scope(&params, &body, &outer_vars));
    }

    #[test]
    fn test_captures_outer_scope_local_assignment() {
        let params = vec![];
        // Defines 'y' locally then uses it
        let body = vec![
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Literal(Literal::Int(10)),
                type_annotation: None,
            },
            HirStmt::Return(Some(HirExpr::Var("y".to_string()))),
        ];
        let outer_vars = std::collections::HashSet::new();
        assert!(!captures_outer_scope(&params, &body, &outer_vars));
    }

    // === Tests for find_variable_type (DEPYLER-COVERAGE-95) ===

    #[test]
    fn test_find_variable_type_from_assign() {
        let stmts = vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        let result = find_variable_type("x", &stmts);
        assert_eq!(result, Some(Type::Int));
    }

    #[test]
    fn test_find_variable_type_missing_var() {
        let stmts = vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        let result = find_variable_type("y", &stmts);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_variable_type_from_for_loop() {
        // Note: find_variable_type doesn't scan For loop targets
        // Only Assign statements are scanned
        let stmts = vec![HirStmt::For {
            target: crate::hir::AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![],
        }];
        let result = find_variable_type("i", &stmts);
        // For loops are not scanned - returns None
        assert!(result.is_none());
    }

    // === Tests for infer_expr_return_type (DEPYLER-COVERAGE-95) ===

    #[test]
    fn test_infer_expr_return_type_literal_int() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Int(42));
        let result = infer_expr_return_type(&expr, &ctx);
        assert_eq!(result, Type::Int);
    }

    #[test]
    fn test_infer_expr_return_type_literal_string() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        let result = infer_expr_return_type(&expr, &ctx);
        assert_eq!(result, Type::String);
    }

    #[test]
    fn test_infer_expr_return_type_literal_bool() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Literal(Literal::Bool(true));
        let result = infer_expr_return_type(&expr, &ctx);
        assert_eq!(result, Type::Bool);
    }

    #[test]
    fn test_infer_expr_return_type_var() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("x".to_string(), Type::Float);
        let expr = HirExpr::Var("x".to_string());
        let result = infer_expr_return_type(&expr, &ctx);
        assert_eq!(result, Type::Float);
    }

    #[test]
    fn test_infer_expr_return_type_unknown_var() {
        let ctx = CodeGenContext::default();
        let expr = HirExpr::Var("unknown".to_string());
        let result = infer_expr_return_type(&expr, &ctx);
        // Unknown vars default to Unknown type
        assert!(matches!(result, Type::Unknown));
    }

    // === Tests for try_return_type_to_tokens (DEPYLER-COVERAGE-95) ===

    #[test]
    fn test_try_return_type_to_tokens_int() {
        let tokens = try_return_type_to_tokens(&Type::Int);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("i64") || tokens_str.contains("i32"));
    }

    #[test]
    fn test_try_return_type_to_tokens_string() {
        let tokens = try_return_type_to_tokens(&Type::String);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("String"));
    }

    #[test]
    fn test_try_return_type_to_tokens_bool() {
        let tokens = try_return_type_to_tokens(&Type::Bool);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("bool"));
    }

    #[test]
    fn test_try_return_type_to_tokens_list() {
        // Note: try_return_type_to_tokens doesn't handle List, falls to ()
        let tokens = try_return_type_to_tokens(&Type::List(Box::new(Type::Int)));
        let tokens_str = tokens.to_string();
        // List not specifically handled, falls back to unit type
        assert!(tokens_str.contains("()"));
    }

    #[test]
    fn test_try_return_type_to_tokens_optional() {
        let tokens = try_return_type_to_tokens(&Type::Optional(Box::new(Type::String)));
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("Option"));
    }
}

