//! Complex statement codegen: try/except, subcommand handling, nested functions
//!
//! DEPYLER-COVERAGE-95: Extracted from stmt_gen.rs to reduce file size
//! and improve testability. Contains try/except and complex control flow.

use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, RustCodeGen, ToRustExpr};
use crate::rust_gen::control_stmt_helpers::{codegen_break_stmt, codegen_continue_stmt, codegen_pass_stmt};
use crate::rust_gen::expr_analysis::{
    contains_floor_div, extract_divisor_from_floor_div, handler_contains_raise,
    is_nested_function_recursive, to_pascal_case,
};
use crate::rust_gen::keywords::safe_ident;
use crate::rust_gen::rust_type_to_syn;
use crate::rust_gen::stmt_gen::{
    codegen_assert_stmt, codegen_assign_stmt, codegen_expr_stmt, codegen_for_stmt,
    codegen_if_stmt, codegen_raise_stmt, codegen_return_stmt, codegen_while_stmt,
    codegen_with_stmt, find_variable_type, infer_try_body_return_type,
    try_generate_json_stdin_match, try_return_type_to_tokens,
};
use crate::rust_gen::type_tokens::hir_type_to_tokens;
use crate::rust_gen::var_analysis::extract_toplevel_assigned_symbols;
#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::trace_decision;
use anyhow::Result;
use quote::quote;

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

    // DEPYLER-0681: Variable hoisting for try/except blocks
    // Variables assigned in try/except blocks need to be accessible after the block.
    // In Python, variables defined in try/except escape their scope. In Rust, they don't.
    // We hoist variable declarations before the try block to fix this.
    let try_vars = extract_toplevel_assigned_symbols(body);
    let handler_vars: std::collections::HashSet<String> = handlers
        .iter()
        .flat_map(|h| extract_toplevel_assigned_symbols(&h.body))
        .collect();

    // Variables assigned in try body that might be assigned in handlers too (common pattern)
    // or any variable assigned in try that's not just a loop variable
    let hoisted_try_vars: Vec<String> = try_vars
        .union(&handler_vars)
        .filter(|v| !ctx.is_declared(v))
        .cloned()
        .collect();

    // Generate hoisted variable declarations
    let mut hoisted_decls = Vec::new();
    for var_name in &hoisted_try_vars {
        let var_ident = safe_ident(var_name);
        
        // Find the variable's type from the first assignment in either try block or handlers
        let var_type = find_variable_type(var_name, body).or_else(|| {
            handlers.iter().find_map(|h| find_variable_type(var_name, &h.body))
        });

        if let Some(ty) = var_type {
            // DEPYLER-0931: Check if type implements Default
            // Types like std::process::Child don't implement Default, so wrap in Option
            let needs_option_wrap = matches!(
                &ty,
                Type::Custom(s) if s == "std::process::Child" || s == "Child"
            );

            if needs_option_wrap {
                // Wrap non-Default types in Option
                let opt_type = Type::Optional(Box::new(ty.clone()));
                let rust_type = ctx.type_mapper.map_type(&opt_type);
                let syn_type = rust_type_to_syn(&rust_type)?;
                hoisted_decls.push(quote! { let mut #var_ident: #syn_type = None; });
                ctx.var_types.insert(var_name.clone(), opt_type);
            } else {
                let rust_type = ctx.type_mapper.map_type(&ty);
                let syn_type = rust_type_to_syn(&rust_type)?;
                // DEPYLER-0931: Initialize with Default::default() to prevent E0381
                // Variables hoisted from try/except must be initialized because the try block might fail
                // before assignment, and we need a valid state for the except block (or after).
                // This also allows closure capturing by reference.
                hoisted_decls.push(quote! { let mut #var_ident: #syn_type = Default::default(); });
                ctx.var_types.insert(var_name.clone(), ty);
            }
        } else {
            // No type annotation - DEPYLER-0931: Default to Option<serde_json::Value>
            // This ensures we have a safe container for whatever value is assigned
            // and handles the "uninitialized" state via None.
            let value_type = crate::hir::Type::Custom("serde_json::Value".to_string());
            let opt_type = crate::hir::Type::Optional(Box::new(value_type));
            
            ctx.var_types.insert(var_name.clone(), opt_type);
            hoisted_decls.push(quote! { let mut #var_ident: Option<serde_json::Value> = None; });

            // DEPYLER-0455 Bug 2: Track hoisted inference vars
            ctx.hoisted_inference_vars.insert(var_name.clone());
        }

        // Declare variable in outer scope so it's accessible after try block
        ctx.declare_var(var_name);
    }

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

        // DEPYLER-0931: Transform handler returns to wrap in Ok() when inside nested exception scope
        // Handler code returns from the outer try/except closure which expects Result<T, E>
        let is_nested = ctx.exception_nesting_depth() > 0;
        let handler_stmts_transformed: Vec<_> = if is_nested {
            handler_stmts
                .iter()
                .map(|stmt| {
                    let stmt_str = stmt.to_string();
                    if stmt_str.starts_with("return ") && !stmt_str.starts_with("return Ok (") {
                        if let Some(expr_part) = stmt_str.strip_prefix("return ") {
                            if let Some(expr) = expr_part.strip_suffix(" ;") {
                                let wrapped = format!("return Ok({}) ;", expr);
                                return wrapped.parse().unwrap_or_else(|_| stmt.clone());
                            }
                        }
                    }
                    stmt.clone()
                })
                .collect()
        } else {
            handler_stmts
        };

        handler_tokens.push(quote! { #(#handler_stmts_transformed)* });
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
                #(#hoisted_decls)*
                {
                    #(#try_stmts)*
                    #finally_code
                }
            })
        } else {
            // DEPYLER-0681: Include hoisted declarations for try block variables
            Ok(quote! {
                #(#hoisted_decls)*
                #(#try_stmts)*
            })
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

                // DEPYLER-0437: Include hoisted variable declarations
                if let Some(finally_code) = finally_stmts {
                    Ok(quote! {
                        {
                            #(#hoisted_decls)*
                            #fixed_tokens
                            #finally_code
                        }
                    })
                } else if hoisted_decls.is_empty() {
                    Ok(fixed_tokens)
                } else {
                    Ok(quote! {
                        #(#hoisted_decls)*
                        #fixed_tokens
                    })
                }
            } else {
                // Pattern matched but no unwrap_or_default found
                // This means it's not a parse operation, so fall through to normal concatenation
                // to include the exception handler code
                // DEPYLER-0437: Include hoisted variable declarations
                let handler_code = &handler_tokens[0];
                if let Some(finally_code) = finally_stmts {
                    Ok(quote! {
                        {
                            #(#hoisted_decls)*
                            #(#try_stmts)*
                            #handler_code
                            #finally_code
                        }
                    })
                } else {
                    Ok(quote! {
                        {
                            #(#hoisted_decls)*
                            #(#try_stmts)*
                            #handler_code
                        }
                    })
                }
            }
        } else {
            // DEPYLER-0931: Always use closure pattern for robust control flow & scoping
            // This guarantees that:
            // 1. Variables are hoisted and accessible after the block (declared outside)
            // 2. Control flow (return/raise) inside try block correctly jumps to handler or exits
            // 3. Variables assigned in try block are correctly captured by mutable reference

            // Infer return type from try body
            let try_return_type = infer_try_body_return_type(body, ctx);
            let return_type_tokens = try_return_type
                .as_ref()
                .map(try_return_type_to_tokens)
                .unwrap_or_else(|| quote! { () });
            let ok_value = try_return_type
                .as_ref()
                .map(|_| quote! { _result })
                .unwrap_or_else(|| quote! { () });

            // The Ok arm extracts _result from Result
            // DEPYLER-0819: When handlers contain raise, the function returns Result<T, E>
            // and we must wrap the success value in Ok()
            // DEPYLER-0931: Always use Ok(_result) when returning from try/except closure
            // because we're inside a Result-returning closure (even for nested try/except)
            let any_handler_raises = handlers.iter().any(|h| handler_contains_raise(&h.body));
            let ok_arm_body = if try_return_type.is_some() {
                // Always wrap in Ok() - we're returning from a Result<T, E> closure
                // If any_handler_raises, the outer function also returns Result, but that's handled by transform
                if any_handler_raises || ctx.exception_nesting_depth() > 0 {
                    quote! { return Ok(_result); }
                } else {
                    quote! { return _result; }
                }
            } else {
                quote! {}
            };

            // Transform try body return statements to wrap values in Ok()
            // The closure returns Result<T, E>, so `return expr;` must become `return Ok(expr);`
            let try_stmts_transformed: Vec<_> = try_stmts
                .iter()
                .map(|stmt| {
                    let stmt_str = stmt.to_string();
                    // Transform `return expr ;` to `return Ok ( expr ) ;`
                    // Simple text-based transformation for now (robust enough for generated code)
                    if stmt_str.starts_with("return ") && !stmt_str.starts_with("return Ok (") {
                        if let Some(expr_part) = stmt_str.strip_prefix("return ") {
                            if let Some(expr) = expr_part.strip_suffix(" ;") {
                                let wrapped = format!("return Ok({}) ;", expr);
                                return wrapped.parse().unwrap_or_else(|_| stmt.clone());
                            }
                        }
                    }
                    stmt.clone()
                })
                .collect();

            // Check if try body always returns (to avoid unreachable code warning)
            let always_returns = body.iter().any(|s| matches!(s, HirStmt::Return(_)));

            // Only add fallback Ok(Default::default()) when try body has no return
            // If try body has returns, they're already wrapped in Ok() and there's no need for fallback
            let closure_fallback = if always_returns {
                quote! {}
            } else if try_return_type.is_none() {
                quote! { Ok(()) } // Return unit for fallthrough
            } else {
                // If try body returns a value, we need a fallback for fallthrough path
                // (e.g., if try block finishes without returning)
                // Use Default::default() for the return type
                quote! { Ok(Default::default()) }
            };

            // Generate handler matching logic
            let match_expr = if handlers.len() == 1 {
                // Single handler - use match pattern
                let err_pattern = if let Some(exc_var) = &handlers[0].name {
                    let exc_ident = safe_ident(exc_var);
                    quote! { Err(#exc_ident) }
                } else {
                    quote! { Err(_) }
                };

                let handler_code = &handler_tokens[0];

                quote! {
                    match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                        #(#try_stmts_transformed)*
                        #closure_fallback
                    })() {
                        Ok(#ok_value) => { #ok_arm_body },
                        #err_pattern => { #handler_code }
                    }
                }
            } else {
                // Multiple handlers - find one with binding or fallback to catch-all
                // TODO: Implement proper type-based dispatch for multiple handlers
                let exc_var_opt = handlers.iter().find_map(|h| h.name.as_ref());
                let handler_code = if let Some(idx) = handlers.iter().position(|h| h.name.is_some()) {
                    &handler_tokens[idx]
                } else {
                    &handler_tokens[0]
                };

                if let Some(exc_var) = exc_var_opt {
                    let exc_ident = safe_ident(exc_var);
                    quote! {
                        match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                            #(#try_stmts_transformed)*
                            #closure_fallback
                        })() {
                            Ok(#ok_value) => { #ok_arm_body },
                            Err(#exc_ident) => { #handler_code }
                        }
                    }
                } else {
                    quote! {
                        match (|| -> Result<#return_type_tokens, Box<dyn std::error::Error>> {
                            #(#try_stmts_transformed)*
                            #closure_fallback
                        })() {
                            Ok(#ok_value) => { #ok_arm_body },
                            Err(_) => { #handler_code }
                        }
                    }
                }
            };

            // DEPYLER-0931: Emit hoisted declarations OUTSIDE the match/closure
            // This ensures variables are captured by mutable reference and retain values
            // after the try/except block.
            if let Some(finally_code) = finally_stmts {
                Ok(quote! {
                    #(#hoisted_decls)*
                    {
                        #match_expr
                        #finally_code
                    }
                })
            } else {
                Ok(quote! {
                    #(#hoisted_decls)*
                    #match_expr
                })
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

    // Pattern 1: let var_name = something . parse :: < i32 > () . unwrap_or_default () ;
    // Pattern 2: var_name = something . parse :: < i32 > () . unwrap_or_default () ;
    // Note: TokenStream.to_string() adds spaces between tokens
    // DEPYLER-0437: Handle both declaration and assignment patterns
    if first_stmt.contains("parse") && first_stmt.contains("unwrap_or_default") {
        // Try to extract variable name from both patterns
        let var_name = if let Some(let_pos) = first_stmt.find("let ") {
            // Pattern: let var_name = ...
            first_stmt[let_pos..].find(" =").map(|eq_pos| {
                first_stmt[let_pos + 4..let_pos + eq_pos].trim().to_string()
            })
        } else {
            // Pattern: var_name = ... (assignment without let, used with hoisted decls)
            first_stmt.find(" = ").map(|eq_pos| first_stmt[..eq_pos].trim().to_string())
        };

        if let Some(var_name) = var_name {
            // Extract parse expression (between "= " and "unwrap_or_default")
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

    None
}

// DEPYLER-COVERAGE-95: extract_divisor_from_floor_div moved to expr_analysis module
// DEPYLER-COVERAGE-95: extract_string_literal, extract_kwarg_string, extract_kwarg_bool
// moved to crate::rust_gen::expr_analysis module for testability

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
pub(crate) fn extract_fields_recursive(
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
                context,
                body: with_body,
                ..
            } => {
                // DEPYLER-0931: Extract fields from With context expression
                // Pattern: `with open(args.file) as f:` - args.file is in context
                // This was missing, causing E0425 errors for fields used in context
                extract_fields_from_expr(context, args_var, dest_field, fields);
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
pub(crate) fn extract_fields_from_expr(
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
/// ```rust,ignore
/// match args.command {
///     Commands::Clone { url } => {
///         handle_clone(args);
///     }
///     Commands::Push { remote } => {
///         handle_push(args);
///     }
/// }
/// ```
pub(crate) fn try_generate_subcommand_match(
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
    // DEPYLER-0940: Filter out empty command names to prevent panic in format_ident!()
    let arms: Vec<proc_macro2::TokenStream> = branches
        .iter()
        .filter(|(cmd_name, _)| !cmd_name.is_empty())
        .map(|(cmd_name, body)| {
            // Convert command name to PascalCase variant
            let variant_name = format_ident!("{}", to_pascal_case(cmd_name));

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
                        // DEPYLER-0762: Use rust_field_name() to properly sanitize flag names
                        // This strips leading dashes and converts hyphens to underscores
                        // e.g., "--format" → "format", "--no-color" → "no_color"
                        let field_name = arg.rust_field_name();
                        accessed_fields.push(field_name);
                    }
                }
            }

            // DEPYLER-0608: Set context flags for handler call transformation
            // When in a subcommand match arm that calls a handler, expr_gen will
            // transform cmd_X(args) → cmd_X(field1, field2, ...)
            // DEPYLER-0665: Always set subcommand_match_fields for ref-pattern bindings
            // This allows clone detection in stmt_gen when assigning mutable vars from refs
            let was_in_match_arm = ctx.in_subcommand_match_arm;
            let old_match_fields = std::mem::take(&mut ctx.subcommand_match_fields);
            if calls_cmd_handler {
                ctx.in_subcommand_match_arm = true;
            }
            // Always track ref-pattern bindings, not just when calling handler
            if !accessed_fields.is_empty() {
                ctx.subcommand_match_fields = accessed_fields.clone();
            }

            // Generate body statements
            ctx.enter_scope();

            // DEPYLER-0577: Register field types in var_types before processing body
            // This allows type-aware codegen (e.g., float vs int comparisons)
            // DEPYLER-0605: Use filter + max_by_key to find the SubcommandInfo with most arguments
            // DEPYLER-0722: Handle Optional types and boolean flags correctly
            for field_name in &accessed_fields {
                if let Some(subcommand) = ctx
                    .argparser_tracker
                    .subcommands
                    .values()
                    .filter(|sc| sc.name == *cmd_name)
                    .max_by_key(|sc| sc.arguments.len())
                {
                    if let Some(arg) = subcommand.arguments.iter().find(|a| {
                        // DEPYLER-0722: Strip dashes from short options (-n → n)
                        let arg_name = a.long.as_ref()
                            .map(|s| s.trim_start_matches('-').to_string())
                            .unwrap_or_else(|| a.name.trim_start_matches('-').to_string());
                        &arg_name == field_name
                    }) {
                        // DEPYLER-0722: Determine actual type including Optional wrapper
                        let base_type = if let Some(ref ty) = arg.arg_type {
                            Some(ty.clone())
                        } else if matches!(arg.action.as_deref(), Some("store_true") | Some("store_false")) {
                            Some(Type::Bool)
                        } else {
                            None
                        };

                        if let Some(ty) = base_type {
                            // DEPYLER-0722: Check if this is actually Option<T> in Clap
                            // An argument is Option<T> if: NOT required AND NO default AND NOT positional
                            // AND NOT a boolean flag (store_true/store_false)
                            let is_bool_flag = matches!(arg.action.as_deref(), Some("store_true") | Some("store_false"));
                            let is_option_type = !arg.is_positional
                                && !arg.required.unwrap_or(false)
                                && arg.default.is_none()
                                && !is_bool_flag;

                            let actual_type = if is_option_type {
                                Type::Optional(Box::new(ty))
                            } else {
                                ty
                            };
                            ctx.var_types.insert(field_name.clone(), actual_type);
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
                // DEPYLER-0843: Use safe_ident for keyword escaping in match patterns
                // If a field is named 'type', it needs to be escaped as 'r#type' in patterns
                let ref_field_patterns: Vec<proc_macro2::TokenStream> = accessed_fields
                    .iter()
                    .map(|f| {
                        let ident = safe_ident(f);
                        quote! { ref #ident }
                    })
                    .collect();

                // DEPYLER-0526: Generate field conversion bindings for borrowed match variables
                // When matching &args.command, destructured fields are references (&String, &bool)
                // Convert to owned values so they work with functions expecting either owned or borrowed:
                // - String fields: .to_string() converts &String → String
                //   String can then deref-coerce to &str if needed
                // - bool/primitives: dereference with *
                // DEPYLER-0843: Also use safe_ident for field bindings after match
                let field_bindings: Vec<proc_macro2::TokenStream> = accessed_fields
                    .iter()
                    .map(|field_name| {
                        let field_ident = safe_ident(field_name);

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
                                    // DEPYLER-0722: Also strip dashes from short options (-n → n)
                                    let arg_field_name = arg
                                        .long
                                        .as_ref()
                                        .map(|s| s.trim_start_matches('-').to_string())
                                        .unwrap_or_else(|| arg.name.trim_start_matches('-').to_string());
                                    arg_field_name == *field_name
                                })
                            });

                        // Determine type: check arg_type first, then action for bool flags
                        let field_type = maybe_arg
                            .and_then(|arg| {
                                // If arg_type is set, use it
                                if let Some(ref base_type) = arg.arg_type {
                                    // DEPYLER-0768: nargs="+" or nargs="*" wraps type in List
                                    // Python: add_argument("values", type=int, nargs="+")
                                    // Rust: Vec<i32> (not i32)
                                    let is_multi = matches!(
                                        arg.nargs.as_deref(),
                                        Some("+") | Some("*")
                                    );
                                    if is_multi {
                                        return Some(Type::List(Box::new(base_type.clone())));
                                    }
                                    return Some(base_type.clone());
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
                                // DEPYLER-0722: Also check for Option<T> without default (NOT required AND NOT positional)
                                let has_default = maybe_arg
                                    .as_ref()
                                    .map(|a| a.default.is_some())
                                    .unwrap_or(false);
                                let is_required = maybe_arg
                                    .as_ref()
                                    .map(|a| a.required.unwrap_or(false))
                                    .unwrap_or(false);
                                let is_positional = maybe_arg
                                    .as_ref()
                                    .map(|a| a.is_positional)
                                    .unwrap_or(false);
                                // DEPYLER-0722: An argument is Option<T> if NOT required AND NOT positional AND NO default
                                let is_option_without_default = !is_required && !is_positional && !has_default;

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
                                } else if is_option_without_default {
                                    // DEPYLER-0722: Option<T> without default - clone the &Option<T> to Option<T>
                                    // Body code can then use .is_some() for truthiness
                                    quote! { let #field_ident = #field_ident.clone(); }
                                } else {
                                    // Required field (not Option), just dereference
                                    quote! { let #field_ident = *#field_ident; }
                                }
                            }
                            Some(Type::String) => {
                                // DEPYLER-0933: First check if this is an Optional String field
                                // (NOT required AND NOT positional AND NO default)
                                let has_default = maybe_arg
                                    .as_ref()
                                    .map(|a| a.default.is_some())
                                    .unwrap_or(false);
                                let is_required = maybe_arg
                                    .as_ref()
                                    .map(|a| a.required.unwrap_or(false))
                                    .unwrap_or(false);
                                let is_positional = maybe_arg
                                    .as_ref()
                                    .map(|a| a.is_positional)
                                    .unwrap_or(false);
                                let is_option_without_default =
                                    !is_required && !is_positional && !has_default;

                                // DEPYLER-0526: Name-based heuristics for String handling
                                let field_lower = field_name.to_lowercase();
                                let owned_indicators = [
                                    "file", "path", "filepath", "input", "output", "dir",
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

                                if is_option_without_default || has_default {
                                    // DEPYLER-0933: Option<String> field - unwrap with default
                                    // Use as_deref() to get Option<&str>, then unwrap_or("")
                                    // This gives &str which works for both &str and String params
                                    quote! { let #field_ident = #field_ident.as_deref().unwrap_or_default(); }
                                } else {
                                    // Required field - regular String handling
                                    if needs_borrowed {
                                        // Keep as &String, auto-derefs to &str
                                        quote! {}
                                    } else if needs_owned {
                                        // Convert to owned String
                                        quote! { let #field_ident = #field_ident.to_string(); }
                                    } else {
                                        // Default: convert to owned (safer for function calls)
                                        quote! { let #field_ident = #field_ident.to_string(); }
                                    }
                                }
                            }
                            Some(Type::Optional(_))
                            | Some(Type::List(_))
                            | Some(Type::Dict(_, _)) => {
                                // For complex container types, clone the reference
                                quote! { let #field_ident = #field_ident.clone(); }
                            }
                            None => {
                                // DEPYLER-0933: Check if this is an Optional field (unknown type)
                                let has_default = maybe_arg
                                    .as_ref()
                                    .map(|a| a.default.is_some())
                                    .unwrap_or(false);
                                let is_required = maybe_arg
                                    .as_ref()
                                    .map(|a| a.required.unwrap_or(false))
                                    .unwrap_or(false);
                                let is_positional = maybe_arg
                                    .as_ref()
                                    .map(|a| a.is_positional)
                                    .unwrap_or(false);
                                let is_option_without_default =
                                    !is_required && !is_positional && !has_default;

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
                                    "suffix", // DEPYLER-0933: suffix is a string field
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

                                // DEPYLER-0933: If optional without default and looks like string,
                                // use as_deref() to get &str which works for both &str and String params
                                if is_option_without_default && looks_like_string {
                                    quote! { let #field_ident = #field_ident.as_deref().unwrap_or_default(); }
                                } else if is_option_without_default && (needs_owned || needs_borrowed) {
                                    // Optional string-like field, unwrap with as_deref for &str
                                    quote! { let #field_ident = #field_ident.as_deref().unwrap_or_default(); }
                                } else if needs_borrowed || (is_positional && needs_owned) {
                                    // DEPYLER-0933: Keep as reference for:
                                    // - borrowed indicators (content, pattern, text, etc.)
                                    // - positional string fields that match owned indicators
                                    //   (file, path, output, etc.) - these are &String, not &Option<String>
                                    quote! {}
                                } else if needs_owned || looks_like_string {
                                    // Convert to owned String (for optional fields that weren't caught above)
                                    quote! { let #field_ident = #field_ident.to_string(); }
                                } else if needs_numeric_unwrap {
                                    // DEPYLER-0576: Likely numeric Option<T> field, unwrap with default
                                    // DEPYLER-0677: Use Default::default() for type-safe numeric defaults
                                    quote! { let #field_ident = #field_ident.unwrap_or_default(); }
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
pub(crate) fn is_subcommand_check(expr: &HirExpr, dest_field: &str, ctx: &CodeGenContext) -> Option<String> {
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
                is_async,
            } => codegen_with_stmt(context, target, body, *is_async, ctx),
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

// Note: hir_type_to_tokens is imported from crate::rust_gen::type_tokens
// for better testability (DEPYLER-0759)

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

    // DEPYLER-0842: Generate function name with keyword escaping
    // Previously used syn::Ident::new() which doesn't escape keywords.
    // If Python has `def const(...)`, this creates `let const = ...` which fails
    // because `const` is a Rust keyword. Using safe_ident produces `let r#const = ...`
    let fn_name = safe_ident(name);

    // GH-70: Use inferred parameters from context if available
    // DEPYLER-0687: Clone params to avoid borrow conflicts with ctx.declare_var
    let effective_params: Vec<HirParam> = ctx
        .nested_function_params
        .get(name)
        .cloned()
        .unwrap_or_else(|| params.to_vec());

    // GH-70: Populate ctx.var_types with inferred param types so that
    // expressions in body (like item[0]) can use proper type info
    // to decide between tuple syntax (.0) and array syntax ([0])
    for param in &effective_params {
        ctx.var_types.insert(param.name.clone(), param.ty.clone());
    }

    // Generate parameters
    // DEPYLER-0550: For collection types (Dict, List), use references
    // This is more idiomatic in Rust and works correctly with filter() closures
    // DEPYLER-0769: Use safe_ident to escape Rust keywords in closure parameters
    let param_tokens: Vec<proc_macro2::TokenStream> = effective_params
        .iter()
        .map(|p| {
            let param_name = safe_ident(&p.name);
            let param_type = hir_type_to_tokens(&p.ty);

            // For collection types and strings, take by reference for idiomatic Rust
            // This is necessary for closures used with filter() which provides &T
            // DEPYLER-0774: Also use &str for String params in nested functions
            // Parent functions receive &str but nested func expects String → use &str
            if matches!(p.ty, Type::Dict(_, _) | Type::List(_) | Type::Set(_)) {
                quote! { #param_name: &#param_type }
            } else if matches!(p.ty, Type::String) {
                // DEPYLER-0774: Use &str instead of String for nested function params
                // This matches how parent functions receive string args (&str)
                quote! { #param_name: &str }
            } else {
                quote! { #param_name: #param_type }
            }
        })
        .collect();

    // Generate return type
    let return_type = hir_type_to_tokens(ret_type);

    // DEPYLER-0550: Save and restore can_fail flag for nested closures
    // Nested closures should NOT inherit can_fail from parent function
    // Otherwise return statements get incorrectly wrapped in Ok()
    let saved_can_fail = ctx.current_function_can_fail;
    ctx.current_function_can_fail = false;

    // DEPYLER-0731: Save and restore return type for nested functions
    // Without this, nested function body uses outer function's return type,
    // causing `return x * 2` to become `return;` when outer returns None
    let saved_return_type = ctx.current_return_type.take();
    ctx.current_return_type = Some(ret_type.clone());

    // DEPYLER-0731: Save and restore is_main_function for nested functions
    // Without this, nested function inside main() would trigger main-specific
    // return handling (DEPYLER-0617) that discards the return value
    let saved_is_main = ctx.is_main_function;
    ctx.is_main_function = false;

    // DEPYLER-0687: Enter new scope for nested function body
    // This isolates variable declarations so they don't leak between closures.
    // Without this, a variable like `result` declared in one closure would be
    // considered "already declared" in sibling closures, causing E0425 errors.
    ctx.enter_scope();

    // Declare parameters in this scope
    for param in &effective_params {
        ctx.declare_var(&param.name);
    }

    // DEPYLER-0766: Analyze mutability for nested function body
    // Without this, variables reassigned inside closures don't get `let mut`,
    // causing E0384 "cannot assign twice to immutable variable" errors.
    crate::rust_gen::analyze_mutable_vars(body, ctx, &effective_params);

    // Generate body
    let body_tokens: Vec<proc_macro2::TokenStream> = body
        .iter()
        .map(|stmt| stmt.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;

    // Exit scope before restoring context
    ctx.exit_scope();

    // Restore can_fail flag
    ctx.current_function_can_fail = saved_can_fail;

    // DEPYLER-0731: Restore outer function's return type and is_main flag
    ctx.current_return_type = saved_return_type;
    ctx.is_main_function = saved_is_main;

    // DEPYLER-0790: Check if this nested function is recursive (calls itself)
    // Recursive functions cannot be closures because closures can't reference themselves
    // For recursive nested functions that DON'T capture outer variables,
    // generate as `fn name(...)` instead of closure
    let is_recursive = is_nested_function_recursive(name, body);

    // Get outer scope variables to check for captures
    // Collect all declared vars from all scopes
    let outer_vars: std::collections::HashSet<String> = ctx
        .declared_vars
        .iter()
        .flat_map(|scope| scope.iter().cloned())
        .collect();
    let has_captures = captures_outer_scope(params, body, &outer_vars);

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

    // DEPYLER-0790: For recursive functions that DON'T capture outer variables,
    // generate as proper fn instead of closure
    // If recursive AND captures, we can't easily fix - keep as closure (will produce E0425)
    if is_recursive && !has_captures {
        // Declare the function name in context so sibling nested functions can detect it
        ctx.declare_var(name);
        return Ok(if matches!(ret_type, Type::Unknown) {
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
        });
    }

    // Declare the function name in context so sibling nested functions can detect it as a capture
    ctx.declare_var(name);

    // DEPYLER-0783: Always use `move` for nested function closures
    // This is required when:
    // 1. The closure captures variables from outer scope AND is returned (E0373)
    // 2. For Copy types (i32, bool), move just copies - no harm
    // 3. For non-Copy types, move is required if captured and returned
    // Using `move` universally is safe and fixes the common closure capture issue
    Ok(if matches!(ret_type, Type::Unknown) {
        if is_declared {
            quote! {
                #fn_name = move |#(#param_tokens),*| {
                    #(#body_tokens)*
                };
            }
        } else {
            quote! {
                let #fn_name = move |#(#param_tokens),*| {
                    #(#body_tokens)*
                };
            }
        }
    } else if is_declared {
        quote! {
            #fn_name = move |#(#param_tokens),*| -> #return_type {
                #(#body_tokens)*
            };
        }
    } else {
        quote! {
            let #fn_name = move |#(#param_tokens),*| -> #return_type {
                #(#body_tokens)*
            };
        }
    })
}

/// DEPYLER-0790: Check if a nested function captures outer scope variables
/// Returns true if the function body references any variables that are NOT:
/// - The function's own parameters
/// - Local variables defined within the function
pub(crate) fn captures_outer_scope(
    params: &[crate::hir::HirParam],
    body: &[HirStmt],
    outer_vars: &std::collections::HashSet<String>,
) -> bool {
    use crate::hir::HirExpr;

    // Collect parameter names
    let mut local_vars: std::collections::HashSet<&str> =
        params.iter().map(|p| p.name.as_str()).collect();

    // Collect locally defined variables from assignments
    fn collect_local_vars<'a>(
        stmt: &'a HirStmt,
        locals: &mut std::collections::HashSet<&'a str>,
    ) {
        match stmt {
            HirStmt::Assign { target: crate::hir::AssignTarget::Symbol(name), .. } => {
                locals.insert(name.as_str());
            }
            HirStmt::Assign { .. } => {}
            HirStmt::For { target, body, .. } => {
                if let crate::hir::AssignTarget::Symbol(name) = target {
                    locals.insert(name.as_str());
                }
                for s in body {
                    collect_local_vars(s, locals);
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                for s in then_body {
                    collect_local_vars(s, locals);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        collect_local_vars(s, locals);
                    }
                }
            }
            HirStmt::While { body, .. } => {
                for s in body {
                    collect_local_vars(s, locals);
                }
            }
            HirStmt::With { body, target, .. } => {
                if let Some(t) = target {
                    locals.insert(t.as_str());
                }
                for s in body {
                    collect_local_vars(s, locals);
                }
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                for s in body {
                    collect_local_vars(s, locals);
                }
                for h in handlers {
                    if let Some(name) = &h.name {
                        locals.insert(name.as_str());
                    }
                    for s in &h.body {
                        collect_local_vars(s, locals);
                    }
                }
                if let Some(els) = orelse {
                    for s in els {
                        collect_local_vars(s, locals);
                    }
                }
                if let Some(fin) = finalbody {
                    for s in fin {
                        collect_local_vars(s, locals);
                    }
                }
            }
            HirStmt::FunctionDef { name, .. } => {
                locals.insert(name.as_str());
            }
            HirStmt::Block(stmts) => {
                for s in stmts {
                    collect_local_vars(s, locals);
                }
            }
            _ => {}
        }
    }

    // First pass: collect all locally defined variables
    for stmt in body {
        collect_local_vars(stmt, &mut local_vars);
    }

    // Now check if body references any outer scope variables
    fn check_expr_for_capture(
        expr: &HirExpr,
        local_vars: &std::collections::HashSet<&str>,
        outer_vars: &std::collections::HashSet<String>,
    ) -> bool {
        match expr {
            HirExpr::Var(name) => {
                // If variable is not local and IS in outer scope, it's captured
                !local_vars.contains(name.as_str()) && outer_vars.contains(name)
            }
            HirExpr::Binary { left, right, .. } => {
                check_expr_for_capture(left, local_vars, outer_vars)
                    || check_expr_for_capture(right, local_vars, outer_vars)
            }
            HirExpr::Unary { operand, .. } => {
                check_expr_for_capture(operand, local_vars, outer_vars)
            }
            HirExpr::Call { func, args, kwargs, .. } => {
                // Check if calling a function defined in outer scope
                let captures_func = !local_vars.contains(func.as_str()) && outer_vars.contains(func);
                captures_func
                    || args
                        .iter()
                        .any(|a| check_expr_for_capture(a, local_vars, outer_vars))
                    || kwargs
                        .iter()
                        .any(|(_, v)| check_expr_for_capture(v, local_vars, outer_vars))
            }
            HirExpr::DynamicCall { callee, args, kwargs } => {
                check_expr_for_capture(callee, local_vars, outer_vars)
                    || args
                        .iter()
                        .any(|a| check_expr_for_capture(a, local_vars, outer_vars))
                    || kwargs
                        .iter()
                        .any(|(_, v)| check_expr_for_capture(v, local_vars, outer_vars))
            }
            HirExpr::MethodCall { object, args, kwargs, .. } => {
                check_expr_for_capture(object, local_vars, outer_vars)
                    || args
                        .iter()
                        .any(|a| check_expr_for_capture(a, local_vars, outer_vars))
                    || kwargs
                        .iter()
                        .any(|(_, v)| check_expr_for_capture(v, local_vars, outer_vars))
            }
            HirExpr::Attribute { value, .. } => {
                check_expr_for_capture(value, local_vars, outer_vars)
            }
            HirExpr::Index { base, index } => {
                check_expr_for_capture(base, local_vars, outer_vars)
                    || check_expr_for_capture(index, local_vars, outer_vars)
            }
            HirExpr::IfExpr { test, body, orelse } => {
                check_expr_for_capture(test, local_vars, outer_vars)
                    || check_expr_for_capture(body, local_vars, outer_vars)
                    || check_expr_for_capture(orelse, local_vars, outer_vars)
            }
            HirExpr::List(items)
            | HirExpr::Tuple(items)
            | HirExpr::Set(items)
            | HirExpr::FrozenSet(items) => items
                .iter()
                .any(|i| check_expr_for_capture(i, local_vars, outer_vars)),
            HirExpr::Dict(pairs) => pairs.iter().any(|(k, v)| {
                check_expr_for_capture(k, local_vars, outer_vars)
                    || check_expr_for_capture(v, local_vars, outer_vars)
            }),
            HirExpr::ListComp { element, generators }
            | HirExpr::SetComp { element, generators }
            | HirExpr::GeneratorExp { element, generators } => {
                check_expr_for_capture(element, local_vars, outer_vars)
                    || generators.iter().any(|g| {
                        check_expr_for_capture(&g.iter, local_vars, outer_vars)
                            || g.conditions
                                .iter()
                                .any(|c| check_expr_for_capture(c, local_vars, outer_vars))
                    })
            }
            HirExpr::DictComp { key, value, generators } => {
                check_expr_for_capture(key, local_vars, outer_vars)
                    || check_expr_for_capture(value, local_vars, outer_vars)
                    || generators.iter().any(|g| {
                        check_expr_for_capture(&g.iter, local_vars, outer_vars)
                            || g.conditions
                                .iter()
                                .any(|c| check_expr_for_capture(c, local_vars, outer_vars))
                    })
            }
            HirExpr::Lambda { body, .. } => {
                check_expr_for_capture(body, local_vars, outer_vars)
            }
            HirExpr::Await { value } => check_expr_for_capture(value, local_vars, outer_vars),
            HirExpr::Slice { base, start, stop, step } => {
                check_expr_for_capture(base, local_vars, outer_vars)
                    || start
                        .as_ref()
                        .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
                    || stop
                        .as_ref()
                        .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
                    || step
                        .as_ref()
                        .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
            }
            HirExpr::Borrow { expr, .. } => check_expr_for_capture(expr, local_vars, outer_vars),
            HirExpr::FString { parts } => parts.iter().any(|p| {
                if let crate::hir::FStringPart::Expr(e) = p {
                    check_expr_for_capture(e, local_vars, outer_vars)
                } else {
                    false
                }
            }),
            HirExpr::Yield { value } => value
                .as_ref()
                .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars)),
            HirExpr::SortByKey { iterable, key_body, reverse_expr, .. } => {
                check_expr_for_capture(iterable, local_vars, outer_vars)
                    || check_expr_for_capture(key_body, local_vars, outer_vars)
                    || reverse_expr
                        .as_ref()
                        .is_some_and(|e| check_expr_for_capture(e, local_vars, outer_vars))
            }
            HirExpr::NamedExpr { value, .. } => {
                check_expr_for_capture(value, local_vars, outer_vars)
            }
            _ => false,
        }
    }

    fn check_stmt_for_capture(
        stmt: &HirStmt,
        local_vars: &std::collections::HashSet<&str>,
        outer_vars: &std::collections::HashSet<String>,
    ) -> bool {
        match stmt {
            HirStmt::Expr(expr) | HirStmt::Return(Some(expr)) => {
                check_expr_for_capture(expr, local_vars, outer_vars)
            }
            HirStmt::Assign { value, .. } => {
                check_expr_for_capture(value, local_vars, outer_vars)
            }
            HirStmt::If { condition, then_body, else_body } => {
                check_expr_for_capture(condition, local_vars, outer_vars)
                    || then_body
                        .iter()
                        .any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
                    || else_body.as_ref().is_some_and(|b| {
                        b.iter()
                            .any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
                    })
            }
            HirStmt::While { condition, body } => {
                check_expr_for_capture(condition, local_vars, outer_vars)
                    || body
                        .iter()
                        .any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
            }
            HirStmt::For { iter, body, .. } => {
                check_expr_for_capture(iter, local_vars, outer_vars)
                    || body
                        .iter()
                        .any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
            }
            HirStmt::With { context, body, .. } => {
                check_expr_for_capture(context, local_vars, outer_vars)
                    || body
                        .iter()
                        .any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                body.iter()
                    .any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
                    || handlers.iter().any(|h| {
                        h.body
                            .iter()
                            .any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
                    })
                    || orelse.as_ref().is_some_and(|b| {
                        b.iter()
                            .any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
                    })
                    || finalbody.as_ref().is_some_and(|b| {
                        b.iter()
                            .any(|s| check_stmt_for_capture(s, local_vars, outer_vars))
                    })
            }
            HirStmt::FunctionDef { body, .. } => body
                .iter()
                .any(|s| check_stmt_for_capture(s, local_vars, outer_vars)),
            HirStmt::Block(stmts) => stmts
                .iter()
                .any(|s| check_stmt_for_capture(s, local_vars, outer_vars)),
            HirStmt::Assert { test, msg } => {
                check_expr_for_capture(test, local_vars, outer_vars)
                    || msg.as_ref().is_some_and(|m| {
                        check_expr_for_capture(m, local_vars, outer_vars)
                    })
            }
            HirStmt::Raise { exception, cause } => {
                exception.as_ref().is_some_and(|e| {
                    check_expr_for_capture(e, local_vars, outer_vars)
                }) || cause
                    .as_ref()
                    .is_some_and(|c| check_expr_for_capture(c, local_vars, outer_vars))
            }
            _ => false,
        }
    }

    body.iter()
        .any(|stmt| check_stmt_for_capture(stmt, &local_vars, outer_vars))
}

