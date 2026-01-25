// DEPYLER-COVERAGE-95: Extracted submodules for better test coverage
pub mod scope_tracker;
pub mod union_type_resolution;

use crate::hir::*;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use scope_tracker::ScopeTracker;
use syn;

pub fn generate_rust(file: syn::File) -> Result<String> {
    let tokens = file.to_token_stream();
    let rust_code = tokens.to_string();

    // Format the code (in a real implementation, we'd use rustfmt)
    Ok(prettify_rust_code(rust_code))
}

pub fn hir_to_rust(hir: &HirModule) -> Result<String> {
    let mut rust_items = Vec::new();

    // Add necessary imports
    if needs_std_collections(hir) {
        rust_items.push(quote! { use std::collections::HashMap; });
    }

    // Convert each function
    for func in &hir.functions {
        let rust_func = convert_function_to_rust(func)?;
        rust_items.push(rust_func);
    }

    let file = quote! {
        #(#rust_items)*
    };

    Ok(prettify_rust_code(file.to_string()))
}

fn needs_std_collections(hir: &HirModule) -> bool {
    hir.functions.iter().any(|f| {
        f.params.iter().any(|param| uses_hashmap(&param.ty))
            || uses_hashmap(&f.ret_type)
            || function_body_uses_hashmap(&f.body)
    })
}

fn uses_hashmap(ty: &Type) -> bool {
    match ty {
        Type::Dict(_, _) => true,
        Type::List(inner) | Type::Optional(inner) => uses_hashmap(inner),
        Type::Tuple(types) => types.iter().any(uses_hashmap),
        Type::Function { params, ret } => params.iter().any(uses_hashmap) || uses_hashmap(ret),
        _ => false,
    }
}

fn function_body_uses_hashmap(body: &[HirStmt]) -> bool {
    body.iter().any(stmt_uses_hashmap)
}

fn stmt_uses_hashmap(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { value, .. } => expr_uses_hashmap(value),
        HirStmt::Return(Some(expr)) => expr_uses_hashmap(expr),
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_uses_hashmap(condition)
                || function_body_uses_hashmap(then_body)
                || else_body
                    .as_ref()
                    .is_some_and(|body| function_body_uses_hashmap(body))
        }
        HirStmt::While { condition, body } => {
            expr_uses_hashmap(condition) || function_body_uses_hashmap(body)
        }
        HirStmt::For { iter, body, .. } => {
            expr_uses_hashmap(iter) || function_body_uses_hashmap(body)
        }
        HirStmt::Expr(expr) => expr_uses_hashmap(expr),
        _ => false,
    }
}

fn expr_uses_hashmap(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Dict(_) => true,
        HirExpr::Binary { left, right, .. } => expr_uses_hashmap(left) || expr_uses_hashmap(right),
        HirExpr::Unary { operand, .. } => expr_uses_hashmap(operand),
        HirExpr::Call { args, .. } => args.iter().any(expr_uses_hashmap),
        HirExpr::Index { base, index } => expr_uses_hashmap(base) || expr_uses_hashmap(index),
        HirExpr::List(items) | HirExpr::Tuple(items) => items.iter().any(expr_uses_hashmap),
        _ => false,
    }
}

// DEPYLER-COVERAGE-95: ScopeTracker extracted to scope_tracker module

fn convert_function_to_rust(func: &HirFunction) -> Result<proc_macro2::TokenStream> {
    let name = syn::Ident::new(&func.name, proc_macro2::Span::call_site());

    // Convert parameters
    // DEPYLER-0507: Handle variadic parameters (*args) as slices
    let params: Vec<_> = func
        .params
        .iter()
        .map(|param| {
            let param_ident = syn::Ident::new(&param.name, proc_macro2::Span::call_site());

            if param.is_vararg {
                // Variadic parameter → &[String] slice (most common type)
                // Future: Type inference to determine actual element type
                quote! { #param_ident: &[String] }
            } else {
                let rust_type = type_to_rust_type(&param.ty);
                quote! { #param_ident: #rust_type }
            }
        })
        .collect();

    // Convert return type
    let return_type = type_to_rust_type(&func.ret_type);

    // Convert body with scope tracking
    let mut scope_tracker = ScopeTracker::new();

    // Declare function parameters in the scope
    for param in &func.params {
        scope_tracker.declare_var(&param.name);
    }

    let body_stmts: Vec<_> = func
        .body
        .iter()
        .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, &mut scope_tracker))
        .collect::<Result<Vec<_>>>()?;

    // Add async if needed
    let func_tokens = if func.properties.is_async {
        quote! {
            pub async fn #name(#(#params),*) -> #return_type {
                #(#body_stmts)*
            }
        }
    } else {
        quote! {
            pub fn #name(#(#params),*) -> #return_type {
                #(#body_stmts)*
            }
        }
    };

    Ok(func_tokens)
}

fn type_to_rust_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Int => quote! { i32 },
        Type::Float => quote! { f64 },
        Type::String => quote! { String },
        Type::Bool => quote! { bool },
        Type::None => quote! { () },
        Type::List(inner) => {
            let inner_type = type_to_rust_type(inner);
            quote! { Vec<#inner_type> }
        }
        Type::Dict(key, value) => {
            let key_type = type_to_rust_type(key);
            let value_type = type_to_rust_type(value);
            quote! { HashMap<#key_type, #value_type> }
        }
        Type::Tuple(types) => {
            let rust_types: Vec<_> = types.iter().map(type_to_rust_type).collect();
            quote! { (#(#rust_types),*) }
        }
        Type::Optional(inner) => {
            let inner_type = type_to_rust_type(inner);
            quote! { Option<#inner_type> }
        }
        Type::Final(inner) => type_to_rust_type(inner), // Unwrap Final to get the actual type
        Type::Function { params, ret } => {
            let param_types: Vec<_> = params.iter().map(type_to_rust_type).collect();
            let ret_type = type_to_rust_type(ret);
            quote! { fn(#(#param_types),*) -> #ret_type }
        }
        Type::Custom(name) => {
            // DEPYLER-0525: Handle special custom types
            match name.as_str() {
                // File-like objects use std::fs::File with mutable reference for Write trait
                "File" => quote! { std::fs::File },
                // serde_json::Value needs the :: path separator
                "serde_json::Value" => quote! { serde_json::Value },
                _ => {
                    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                    quote! { #ident }
                }
            }
        }
        Type::Unknown => quote! { () },
        Type::TypeVar(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! { #ident }
        }
        Type::Generic { base, params } => {
            let base_ident = syn::Ident::new(base, proc_macro2::Span::call_site());
            let param_types: Vec<_> = params.iter().map(type_to_rust_type).collect();
            quote! { #base_ident<#(#param_types),*> }
        }
        // DEPYLER-0765: Resolve union types to valid Rust types
        Type::Union(types) => resolve_union_type(types),
        Type::Array { element_type, size } => {
            let element = type_to_rust_type(element_type);
            match size {
                crate::hir::ConstGeneric::Literal(n) => {
                    let size_lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
                    quote! { [#element; #size_lit] }
                }
                crate::hir::ConstGeneric::Parameter(name) => {
                    let param_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                    quote! { [#element; #param_ident] }
                }
                crate::hir::ConstGeneric::Expression(expr) => {
                    // For expressions, parse them as token streams
                    let expr_tokens: proc_macro2::TokenStream = expr.parse().unwrap_or_else(|_| {
                        quote! { /* invalid const expression */ }
                    });
                    quote! { [#element; #expr_tokens] }
                }
            }
        }
        Type::Set(inner) => {
            let inner_type = type_to_rust_type(inner);
            quote! { HashSet<#inner_type> }
        }
        Type::UnificationVar(id) => {
            // UnificationVar should never appear in final code generation
            panic!(
                "BUG: UnificationVar({}) encountered in codegen. Type inference incomplete.",
                id
            )
        }
    }
}

/// DEPYLER-0765: Resolve Python union types to valid Rust types
/// DEPYLER-COVERAGE-95: Delegate to extracted union_type_resolution module
fn resolve_union_type(types: &[Type]) -> proc_macro2::TokenStream {
    union_type_resolution::resolve_union_type(types, type_to_rust_type)
}

#[allow(dead_code)]
fn stmt_to_rust_tokens(stmt: &HirStmt) -> Result<proc_macro2::TokenStream> {
    // Legacy function - delegate to the new scope-aware version with a throwaway scope
    let mut scope_tracker = ScopeTracker::new();
    stmt_to_rust_tokens_with_scope(stmt, &mut scope_tracker)
}

// DEPYLER-0012: Helper functions to reduce complexity (extracted from stmt_to_rust_tokens_with_scope)
fn handle_assign_target(
    target: &AssignTarget,
    value_tokens: proc_macro2::TokenStream,
    scope_tracker: &mut ScopeTracker,
) -> Result<proc_macro2::TokenStream> {
    match target {
        AssignTarget::Symbol(symbol) => {
            let target_ident = syn::Ident::new(symbol, proc_macro2::Span::call_site());
            if scope_tracker.is_declared(symbol) {
                Ok(quote! { #target_ident = #value_tokens; })
            } else {
                scope_tracker.declare_var(symbol);
                Ok(quote! { let mut #target_ident = #value_tokens; })
            }
        }
        AssignTarget::Index { base, index } => {
            let base_tokens = expr_to_rust_tokens(base)?;
            let index_tokens = expr_to_rust_tokens(index)?;
            Ok(quote! { #base_tokens.insert(#index_tokens, #value_tokens); })
        }
        AssignTarget::Attribute { value, attr } => {
            // Struct field assignment: obj.field = value
            let base_tokens = expr_to_rust_tokens(value)?;
            let attr_ident = syn::Ident::new(attr.as_str(), proc_macro2::Span::call_site());
            Ok(quote! { #base_tokens.#attr_ident = #value_tokens; })
        }
        AssignTarget::Tuple(targets) => {
            // Tuple unpacking
            let all_symbols: Option<Vec<&str>> = targets
                .iter()
                .map(|t| match t {
                    AssignTarget::Symbol(s) => Some(s.as_str()),
                    _ => None,
                })
                .collect();

            match all_symbols {
                Some(symbols) => {
                    let all_declared = symbols.iter().all(|s| scope_tracker.is_declared(s));

                    if all_declared {
                        let idents: Vec<_> = symbols
                            .iter()
                            .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                            .collect();
                        Ok(quote! { (#(#idents),*) = #value_tokens; })
                    } else {
                        scope_tracker.declare_vars(symbols.iter());
                        let idents: Vec<_> = symbols
                            .iter()
                            .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                            .collect();
                        Ok(quote! { let (mut #(#idents),*) = #value_tokens; })
                    }
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
                        let temp_var =
                            syn::Ident::new("_swap_temp", proc_macro2::Span::call_site());

                        let mut assignments = Vec::new();
                        for (idx, (base, index)) in indices.iter().enumerate() {
                            let base_expr = expr_to_rust_tokens(base)?;
                            let index_expr = expr_to_rust_tokens(index)?;
                            let tuple_idx = syn::Index::from(idx);

                            // Vec assignment: base[index as usize] = temp.N
                            assignments.push(quote! {
                                #base_expr[(#index_expr) as usize] = #temp_var.#tuple_idx;
                            });
                        }

                        return Ok(quote! {
                            let #temp_var = #value_tokens;
                            #(#assignments)*
                        });
                    }

                    anyhow::bail!("Complex tuple unpacking not yet supported")
                }
            }
        }
    }
}

fn handle_if_stmt(
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    scope_tracker: &mut ScopeTracker,
) -> Result<proc_macro2::TokenStream> {
    let cond_tokens = expr_to_rust_tokens(condition)?;

    scope_tracker.enter_scope();
    let then_stmts: Vec<_> = then_body
        .iter()
        .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, scope_tracker))
        .collect::<Result<Vec<_>>>()?;
    scope_tracker.exit_scope();

    if let Some(else_stmts) = else_body {
        scope_tracker.enter_scope();
        let else_tokens: Vec<_> = else_stmts
            .iter()
            .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, scope_tracker))
            .collect::<Result<Vec<_>>>()?;
        scope_tracker.exit_scope();
        Ok(quote! {
            if #cond_tokens {
                #(#then_stmts)*
            } else {
                #(#else_tokens)*
            }
        })
    } else {
        Ok(quote! {
            if #cond_tokens {
                #(#then_stmts)*
            }
        })
    }
}

fn handle_while_stmt(
    condition: &HirExpr,
    body: &[HirStmt],
    scope_tracker: &mut ScopeTracker,
) -> Result<proc_macro2::TokenStream> {
    let cond_tokens = expr_to_rust_tokens(condition)?;
    scope_tracker.enter_scope();
    let body_stmts: Vec<_> = body
        .iter()
        .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, scope_tracker))
        .collect::<Result<Vec<_>>>()?;
    scope_tracker.exit_scope();
    Ok(quote! {
        while #cond_tokens {
            #(#body_stmts)*
        }
    })
}

fn handle_for_stmt(
    target: &AssignTarget,
    iter: &HirExpr,
    body: &[HirStmt],
    scope_tracker: &mut ScopeTracker,
) -> Result<proc_macro2::TokenStream> {
    let iter_tokens = expr_to_rust_tokens(iter)?;
    scope_tracker.enter_scope();

    // Generate target pattern and declare variables
    let target_pattern = match target {
        AssignTarget::Symbol(name) => {
            scope_tracker.declare_var(name);
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! { #ident }
        }
        AssignTarget::Tuple(targets) => {
            // Extract symbols and declare them - handle nested tuples gracefully
            let idents: Vec<syn::Ident> = targets
                .iter()
                .map(|t| match t {
                    AssignTarget::Symbol(s) => {
                        scope_tracker.declare_var(s);
                        Ok(syn::Ident::new(s, proc_macro2::Span::call_site()))
                    }
                    _ => bail!("Nested tuple unpacking not supported in for loops"),
                })
                .collect::<Result<Vec<_>>>()?;
            quote! { (#(#idents),*) }
        }
        _ => bail!("Unsupported for loop target type"),
    };

    let body_stmts: Vec<_> = body
        .iter()
        .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, scope_tracker))
        .collect::<Result<Vec<_>>>()?;
    scope_tracker.exit_scope();
    Ok(quote! {
        for #target_pattern in #iter_tokens {
            #(#body_stmts)*
        }
    })
}

fn handle_with_stmt(
    context: &HirExpr,
    target: &Option<String>,
    body: &[HirStmt],
) -> Result<proc_macro2::TokenStream> {
    let context_tokens = expr_to_rust_tokens(context)?;
    let body_tokens: Vec<_> = body
        .iter()
        .map(stmt_to_rust_tokens)
        .collect::<Result<_>>()?;

    if let Some(var_name) = target {
        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
        Ok(quote! {
            {
                let mut #var_ident = #context_tokens;
                #(#body_tokens)*
            }
        })
    } else {
        Ok(quote! {
            {
                let _context = #context_tokens;
                #(#body_tokens)*
            }
        })
    }
}

fn stmt_to_rust_tokens_with_scope(
    stmt: &HirStmt,
    scope_tracker: &mut ScopeTracker,
) -> Result<proc_macro2::TokenStream> {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            let value_tokens = expr_to_rust_tokens(value)?;
            handle_assign_target(target, value_tokens, scope_tracker)
        }
        HirStmt::Return(expr_opt) => {
            if let Some(expr) = expr_opt {
                let expr_tokens = expr_to_rust_tokens(expr)?;
                Ok(quote! { return #expr_tokens; })
            } else {
                Ok(quote! { return; })
            }
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => handle_if_stmt(condition, then_body, else_body, scope_tracker),
        HirStmt::While { condition, body } => handle_while_stmt(condition, body, scope_tracker),
        HirStmt::For { target, iter, body } => handle_for_stmt(target, iter, body, scope_tracker),
        HirStmt::Expr(expr) => {
            let expr_tokens = expr_to_rust_tokens(expr)?;
            Ok(quote! { #expr_tokens; })
        }
        HirStmt::Raise {
            exception,
            cause: _,
        } => {
            // Simple error handling for codegen - just generate a panic for now
            if let Some(exc) = exception {
                let exc_tokens = expr_to_rust_tokens(exc)?;
                Ok(quote! { panic!("Exception: {}", #exc_tokens); })
            } else {
                Ok(quote! { panic!("Exception raised"); })
            }
        }
        HirStmt::Break { label } => {
            if let Some(label_name) = label {
                let label_ident =
                    syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
                Ok(quote! { break #label_ident; })
            } else {
                Ok(quote! { break; })
            }
        }
        HirStmt::Continue { label } => {
            if let Some(label_name) = label {
                let label_ident =
                    syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
                Ok(quote! { continue #label_ident; })
            } else {
                Ok(quote! { continue; })
            }
        }
        HirStmt::With {
            context,
            target,
            body,
            ..
        } => handle_with_stmt(context, target, body),
        HirStmt::Try {
            body,
            handlers,
            orelse: _,
            finalbody,
        } => {
            // Generate try body statements
            let try_stmts: Vec<_> = body
                .iter()
                .map(|s| stmt_to_rust_tokens_with_scope(s, scope_tracker))
                .collect::<Result<Vec<_>>>()?;

            // Generate finally statements if present
            let finally_stmts = if let Some(finally_body) = finalbody {
                let stmts: Vec<_> = finally_body
                    .iter()
                    .map(|s| stmt_to_rust_tokens_with_scope(s, scope_tracker))
                    .collect::<Result<Vec<_>>>()?;
                Some(quote! { #(#stmts)* })
            } else {
                None
            };

            // Generate handler statements (just use first handler for simplicity)
            if let Some(handler) = handlers.first() {
                let handler_stmts: Vec<_> = handler
                    .body
                    .iter()
                    .map(|s| stmt_to_rust_tokens_with_scope(s, scope_tracker))
                    .collect::<Result<Vec<_>>>()?;

                // DEPYLER-0937: Use actual exception variable name if present
                // This fixes E0425 where handler body references 'e' but pattern used '_e'
                let err_pattern = if let Some(exc_var) = &handler.name {
                    let exc_ident = syn::Ident::new(exc_var, proc_macro2::Span::call_site());
                    quote! { Err(#exc_ident) }
                } else {
                    quote! { Err(_) }
                };

                if let Some(finally_code) = finally_stmts {
                    Ok(quote! {
                        {
                            let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                #(#try_stmts)*
                                Ok(())
                            })();
                            if let #err_pattern = _result {
                                #(#handler_stmts)*
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
                            if let #err_pattern = _result {
                                #(#handler_stmts)*
                            }
                        }
                    })
                }
            } else {
                // No handlers - try/finally without except
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
            }
        }
        HirStmt::Assert { test, msg } => {
            // Generate assert! macro call
            let test_expr = expr_to_rust_tokens(test)?;
            if let Some(message) = msg {
                let msg_expr = expr_to_rust_tokens(message)?;
                Ok(quote! { assert!(#test_expr, "{}", #msg_expr); })
            } else {
                Ok(quote! { assert!(#test_expr); })
            }
        }
        HirStmt::Pass => {
            // Pass statement generates no code
            Ok(quote! {})
        }
        // DEPYLER-0614: Handle Block of statements
        HirStmt::Block(stmts) => {
            let mut tokens = proc_macro2::TokenStream::new();
            for s in stmts {
                tokens.extend(stmt_to_rust_tokens_with_scope(s, scope_tracker)?);
            }
            Ok(tokens)
        }
        // DEPYLER-0427: Nested function support - delegate to rust_gen module
        HirStmt::FunctionDef { .. } => {
            // This is handled by the main rust_gen module
            // This codegen.rs module is a legacy simplified codegen path
            // For now, just return empty - nested functions use the main rust_gen path
            Ok(quote! {})
        }
    }
}

/// Convert binary expression to Rust tokens with special operator handling
/// Complexity: ~6-7 (within ≤10 target)
fn binary_expr_to_rust_tokens(
    op: &BinOp,
    left: &HirExpr,
    right: &HirExpr,
) -> Result<proc_macro2::TokenStream> {
    let left_tokens = expr_to_rust_tokens(left)?;
    let right_tokens = expr_to_rust_tokens(right)?;

    // Special handling for specific operators
    match op {
        BinOp::Sub if is_len_call(left) => {
            // Use saturating_sub to prevent underflow when subtracting from array length
            Ok(quote! { #left_tokens.saturating_sub(#right_tokens) })
        }
        BinOp::FloorDiv => {
            // Python floor division semantics
            // For now, assume numeric types and use the integer floor division formula
            // DEPYLER-0236: Use intermediate variables to avoid formatting issues with != operator
            Ok(quote! {
                {
                    let a = #left_tokens;
                    let b = #right_tokens;
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
        _ => {
            let op_tokens = binop_to_rust_tokens(op);
            Ok(quote! { (#left_tokens #op_tokens #right_tokens) })
        }
    }
}

/// Convert function call expression to Rust tokens
/// Complexity: 1 (within ≤10 target)
fn call_expr_to_rust_tokens(func: &str, args: &[HirExpr]) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0844: isinstance(x, T) → true (Rust's type system guarantees correctness)
    if func == "isinstance" {
        return Ok(quote! { true });
    }

    let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
    let arg_tokens: Vec<_> = args
        .iter()
        .map(expr_to_rust_tokens)
        .collect::<Result<Vec<_>>>()?;
    Ok(quote! { #func_ident(#(#arg_tokens),*) })
}

/// Convert list literal to Rust vec! macro
/// Complexity: 1 (within ≤10 target)
fn list_literal_to_rust_tokens(items: &[HirExpr]) -> Result<proc_macro2::TokenStream> {
    let item_tokens: Vec<_> = items
        .iter()
        .map(expr_to_rust_tokens)
        .collect::<Result<Vec<_>>>()?;
    Ok(quote! { vec![#(#item_tokens),*] })
}

/// Convert dict literal to Rust HashMap
/// Complexity: 2 (within ≤10 target)
fn dict_literal_to_rust_tokens(items: &[(HirExpr, HirExpr)]) -> Result<proc_macro2::TokenStream> {
    let mut entries = Vec::new();
    for (key, value) in items {
        let key_tokens = expr_to_rust_tokens(key)?;
        let value_tokens = expr_to_rust_tokens(value)?;
        entries.push(quote! { (#key_tokens, #value_tokens) });
    }
    // DEPYLER-0623: Use fully qualified path for consistent HashMap resolution
    Ok(quote! {
        {
            let mut map = std::collections::HashMap::new();
            #(map.insert #entries;)*
            map
        }
    })
}

/// Convert tuple literal to Rust tuple
/// Complexity: 1 (within ≤10 target)
fn tuple_literal_to_rust_tokens(items: &[HirExpr]) -> Result<proc_macro2::TokenStream> {
    let item_tokens: Vec<_> = items
        .iter()
        .map(expr_to_rust_tokens)
        .collect::<Result<Vec<_>>>()?;
    Ok(quote! { (#(#item_tokens),*) })
}

/// Convert borrow expression to Rust reference
/// Complexity: 2 (if-else, within ≤10 target)
fn borrow_expr_to_rust_tokens(expr: &HirExpr, mutable: bool) -> Result<proc_macro2::TokenStream> {
    let expr_tokens = expr_to_rust_tokens(expr)?;
    if mutable {
        Ok(quote! { &mut #expr_tokens })
    } else {
        Ok(quote! { &#expr_tokens })
    }
}

/// Convert method call expression to Rust method call
/// Complexity: 1 (within ≤10 target)
fn method_call_to_rust_tokens(
    object: &HirExpr,
    method: &str,
    args: &[HirExpr],
) -> Result<proc_macro2::TokenStream> {
    let obj_tokens = expr_to_rust_tokens(object)?;
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    let arg_tokens: Vec<_> = args
        .iter()
        .map(expr_to_rust_tokens)
        .collect::<Result<Vec<_>>>()?;
    Ok(quote! { #obj_tokens.#method_ident(#(#arg_tokens),*) })
}

/// Convert slice expression to Rust slice notation
/// Complexity: 5 (match arms, within ≤10 target)
fn slice_expr_to_rust_tokens(
    base: &HirExpr,
    start: &Option<Box<HirExpr>>,
    stop: &Option<Box<HirExpr>>,
    step: &Option<Box<HirExpr>>,
) -> Result<proc_macro2::TokenStream> {
    let base_tokens = expr_to_rust_tokens(base)?;
    // Simple codegen - just use slice notation where possible
    match (start, stop, step) {
        (None, None, None) => Ok(quote! { #base_tokens.clone() }),
        (Some(start), Some(stop), None) => {
            let start_tokens = expr_to_rust_tokens(start)?;
            let stop_tokens = expr_to_rust_tokens(stop)?;
            Ok(quote! { #base_tokens[#start_tokens..#stop_tokens].to_vec() })
        }
        (Some(start), None, None) => {
            let start_tokens = expr_to_rust_tokens(start)?;
            Ok(quote! { #base_tokens[#start_tokens..].to_vec() })
        }
        (None, Some(stop), None) => {
            let stop_tokens = expr_to_rust_tokens(stop)?;
            Ok(quote! { #base_tokens[..#stop_tokens].to_vec() })
        }
        _ => {
            // For complex cases with step, fall back to method call
            Ok(quote! { slice_complex(#base_tokens) })
        }
    }
}

/// Convert list comprehension to Rust iterator chain
/// Complexity: 2 (if-else for condition, within ≤10 target)
fn list_comp_to_rust_tokens(
    element: &HirExpr,
    target: &str,
    iter: &HirExpr,
    condition: &Option<Box<HirExpr>>,
) -> Result<proc_macro2::TokenStream> {
    let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
    let iter_tokens = expr_to_rust_tokens(iter)?;
    let element_tokens = expr_to_rust_tokens(element)?;

    if let Some(cond) = condition {
        // With condition: iter().filter().map().collect()
        // DEPYLER-0833: Use |x| pattern (not |&x|) to avoid E0507 on non-Copy types
        // filter() receives &Item, so closure param is a reference; condition works on reference
        let cond_tokens = expr_to_rust_tokens(cond)?;
        Ok(quote! {
            #iter_tokens
                .into_iter()
                .filter(|#target_ident| #cond_tokens)
                .map(|#target_ident| #element_tokens)
                .collect::<Vec<_>>()
        })
    } else {
        // Without condition: iter().map().collect()
        Ok(quote! {
            #iter_tokens
                .into_iter()
                .map(|#target_ident| #element_tokens)
                .collect::<Vec<_>>()
        })
    }
}

/// Convert lambda expression to Rust closure
/// Complexity: 2 (if-else for params, within ≤10 target)
fn lambda_to_rust_tokens(params: &[String], body: &HirExpr) -> Result<proc_macro2::TokenStream> {
    // Convert parameters to identifiers
    let param_idents: Vec<proc_macro2::Ident> = params
        .iter()
        .map(|p| quote::format_ident!("{}", p))
        .collect();

    // Convert body
    let body_tokens = expr_to_rust_tokens(body)?;

    // Generate closure
    // DEPYLER-0837: Use `move` closures to match Python's closure semantics
    // Python closures capture variables by reference but extend their lifetime
    // Rust requires `move` when returning closures that capture local variables
    if params.is_empty() {
        Ok(quote! { move || #body_tokens })
    } else {
        Ok(quote! { move |#(#param_idents),*| #body_tokens })
    }
}

/// Convert set literal to Rust HashSet
/// Complexity: 1 (within ≤10 target)
fn set_literal_to_rust_tokens(items: &[HirExpr]) -> Result<proc_macro2::TokenStream> {
    let item_tokens: Vec<_> = items
        .iter()
        .map(expr_to_rust_tokens)
        .collect::<Result<Vec<_>>>()?;
    // DEPYLER-0623: Use fully qualified path for consistent HashSet resolution
    Ok(quote! {
        {
            let mut set = std::collections::HashSet::new();
            #(set.insert(#item_tokens);)*
            set
        }
    })
}

/// Convert frozenset literal to Rust Arc<HashSet>
/// Complexity: 1 (within ≤10 target)
fn frozen_set_to_rust_tokens(items: &[HirExpr]) -> Result<proc_macro2::TokenStream> {
    let item_tokens: Vec<_> = items
        .iter()
        .map(expr_to_rust_tokens)
        .collect::<Result<Vec<_>>>()?;
    // DEPYLER-0623: Use fully qualified path for consistent HashSet resolution
    Ok(quote! {
        {
            let mut set = std::collections::HashSet::new();
            #(set.insert(#item_tokens);)*
            std::sync::Arc::new(set)
        }
    })
}

/// Convert set comprehension to Rust iterator chain
/// Complexity: 2 (if-else for condition, within ≤10 target)
fn set_comp_to_rust_tokens(
    element: &HirExpr,
    target: &str,
    iter: &HirExpr,
    condition: &Option<Box<HirExpr>>,
) -> Result<proc_macro2::TokenStream> {
    let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
    let iter_tokens = expr_to_rust_tokens(iter)?;
    let element_tokens = expr_to_rust_tokens(element)?;

    // DEPYLER-0831: Use fully-qualified path for E0412 resolution
    if let Some(cond) = condition {
        // With condition: iter().filter().map().collect()
        // DEPYLER-0833: Use |x| pattern (not |&x|) to avoid E0507 on non-Copy types
        let cond_tokens = expr_to_rust_tokens(cond)?;
        Ok(quote! {
            #iter_tokens
                .into_iter()
                .filter(|#target_ident| #cond_tokens)
                .map(|#target_ident| #element_tokens)
                .collect::<std::collections::HashSet<_>>()
        })
    } else {
        // Without condition: iter().map().collect()
        Ok(quote! {
            #iter_tokens
                .into_iter()
                .map(|#target_ident| #element_tokens)
                .collect::<std::collections::HashSet<_>>()
        })
    }
}

/// Convert dict comprehension to Rust iterator chain
/// Complexity: 2 (if-else for condition, within ≤10 target)
fn dict_comp_to_rust_tokens(
    key: &HirExpr,
    value: &HirExpr,
    target: &str,
    iter: &HirExpr,
    condition: &Option<Box<HirExpr>>,
) -> Result<proc_macro2::TokenStream> {
    let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
    let iter_tokens = expr_to_rust_tokens(iter)?;
    let key_tokens = expr_to_rust_tokens(key)?;
    let value_tokens = expr_to_rust_tokens(value)?;

    if let Some(cond) = condition {
        // With condition: iter().filter().map().collect()
        // DEPYLER-0833: Use |x| pattern (not |&x|) to avoid E0507 on non-Copy types
        let cond_tokens = expr_to_rust_tokens(cond)?;
        Ok(quote! {
            #iter_tokens
                .into_iter()
                .filter(|#target_ident| #cond_tokens)
                .map(|#target_ident| (#key_tokens, #value_tokens))
                .collect::<std::collections::HashMap<_, _>>()
        })
    } else {
        // Without condition: iter().map().collect()
        Ok(quote! {
            #iter_tokens
                .into_iter()
                .map(|#target_ident| (#key_tokens, #value_tokens))
                .collect::<std::collections::HashMap<_, _>>()
        })
    }
}

fn expr_to_rust_tokens(expr: &HirExpr) -> Result<proc_macro2::TokenStream> {
    match expr {
        HirExpr::Literal(lit) => literal_to_rust_tokens(lit),
        HirExpr::Var(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            Ok(quote! { #ident })
        }
        HirExpr::Binary { op, left, right } => binary_expr_to_rust_tokens(op, left, right),
        HirExpr::Unary { op, operand } => {
            let operand_tokens = expr_to_rust_tokens(operand)?;
            let op_tokens = unaryop_to_rust_tokens(op);
            Ok(quote! { (#op_tokens #operand_tokens) })
        }
        HirExpr::Call { func, args, .. } => call_expr_to_rust_tokens(func, args),
        HirExpr::Index { base, index } => {
            let base_tokens = expr_to_rust_tokens(base)?;
            let index_tokens = expr_to_rust_tokens(index)?;
            Ok(quote! { #base_tokens[#index_tokens] })
        }
        HirExpr::List(items) => list_literal_to_rust_tokens(items),
        HirExpr::Dict(items) => dict_literal_to_rust_tokens(items),
        HirExpr::Tuple(items) => tuple_literal_to_rust_tokens(items),
        HirExpr::Attribute { value, attr } => {
            let value_tokens = expr_to_rust_tokens(value)?;
            let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
            Ok(quote! { #value_tokens.#attr_ident })
        }
        HirExpr::Borrow { expr, mutable } => borrow_expr_to_rust_tokens(expr, *mutable),
        HirExpr::MethodCall {
            object,
            method,
            args,
            ..
        } => method_call_to_rust_tokens(object, method, args),
        HirExpr::Slice {
            base,
            start,
            stop,
            step,
        } => slice_expr_to_rust_tokens(base, start, stop, step),
        HirExpr::ListComp {
            element,
            generators,
        } => {
            // DEPYLER-0504: Legacy path - only support single generator for now
            if generators.len() != 1 {
                bail!("Multiple generators not supported in legacy codegen path");
            }
            let gen = &generators[0];
            let condition = if gen.conditions.is_empty() {
                None
            } else if gen.conditions.len() == 1 {
                Some(Box::new(gen.conditions[0].clone()))
            } else {
                bail!("Multiple conditions in generator not supported in legacy codegen path");
            };
            list_comp_to_rust_tokens(element, &gen.target, &gen.iter, &condition)
        }
        HirExpr::Lambda { params, body } => lambda_to_rust_tokens(params, body),
        HirExpr::Set(items) => set_literal_to_rust_tokens(items),
        HirExpr::FrozenSet(items) => frozen_set_to_rust_tokens(items),
        HirExpr::SetComp {
            element,
            generators,
        } => {
            // DEPYLER-0504: Legacy path - only support single generator for now
            if generators.len() != 1 {
                bail!("Multiple generators not supported in legacy codegen path");
            }
            let gen = &generators[0];
            let condition = if gen.conditions.is_empty() {
                None
            } else if gen.conditions.len() == 1 {
                Some(Box::new(gen.conditions[0].clone()))
            } else {
                bail!("Multiple conditions in generator not supported in legacy codegen path");
            };
            set_comp_to_rust_tokens(element, &gen.target, &gen.iter, &condition)
        }
        HirExpr::DictComp {
            key,
            value,
            generators,
        } => {
            // DEPYLER-0504: Legacy path - only support single generator for now
            if generators.len() != 1 {
                bail!("Multiple generators not supported in legacy codegen path");
            }
            let gen = &generators[0];
            let condition = if gen.conditions.is_empty() {
                None
            } else if gen.conditions.len() == 1 {
                Some(Box::new(gen.conditions[0].clone()))
            } else {
                bail!("Multiple conditions in generator not supported in legacy codegen path");
            };
            dict_comp_to_rust_tokens(key, value, &gen.target, &gen.iter, &condition)
        }
        HirExpr::Await { value } => {
            let value_tokens = expr_to_rust_tokens(value)?;
            Ok(quote! { #value_tokens.await })
        }
        HirExpr::Yield { value } => {
            if let Some(v) = value {
                let value_tokens = expr_to_rust_tokens(v)?;
                Ok(quote! { yield #value_tokens })
            } else {
                Ok(quote! { yield })
            }
        }
        HirExpr::FString { .. } => {
            anyhow::bail!("FString not yet implemented in codegen")
        }
        HirExpr::IfExpr { test, body, orelse } => {
            let test_tokens = expr_to_rust_tokens(test)?;
            let body_tokens = expr_to_rust_tokens(body)?;
            let orelse_tokens = expr_to_rust_tokens(orelse)?;
            Ok(quote! { if #test_tokens { #body_tokens } else { #orelse_tokens } })
        }
        HirExpr::SortByKey {
            iterable,
            key_params,
            key_body,
            reverse_expr,
        } => {
            let iter_tokens = expr_to_rust_tokens(iterable)?;
            let body_tokens = expr_to_rust_tokens(key_body)?;

            if key_params.len() != 1 {
                bail!("sorted() key lambda must have exactly one parameter");
            }

            let param = syn::Ident::new(&key_params[0], proc_macro2::Span::call_site());

            // DEPYLER-0502: Convert reverse_expr to Rust tokens (supports variables and expressions)
            let reverse_tokens = if let Some(expr) = reverse_expr {
                expr_to_rust_tokens(expr)?
            } else {
                quote! { false }
            };

            // Generate code with runtime conditional reverse
            Ok(quote! {
                {
                    let mut __sorted_result = #iter_tokens.clone();
                    __sorted_result.sort_by_key(|#param| #body_tokens);
                    if #reverse_tokens {
                        __sorted_result.reverse();
                    }
                    __sorted_result
                }
            })
        }
        HirExpr::GeneratorExp { .. } => {
            // Note: Generator expressions are fully implemented in rust_gen.rs (v3.13.0, 20/20 tests).
            // This codegen.rs path is legacy HIR-to-Rust conversion, not used in main transpiler pipeline.
            // The primary implementation is in crates/depyler-core/src/rust_gen.rs::convert_generator_expression()
            bail!("Generator expressions require rust_gen.rs (use DepylerPipeline instead of direct codegen)")
        }
        // DEPYLER-0188: Walrus operator (assignment expression)
        // Python: (x := expr) evaluates to expr and assigns to x
        // Rust: { let x = expr; x }
        HirExpr::NamedExpr { target, value } => {
            let ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            let value_tokens = expr_to_rust_tokens(value)?;
            Ok(quote! {
                {
                    let #ident = #value_tokens;
                    #ident
                }
            })
        }
        // DEPYLER-0188: Dynamic call: handlers[name](args) → (handlers[name])(args)
        HirExpr::DynamicCall { callee, args, .. } => {
            let callee_tokens = expr_to_rust_tokens(callee)?;
            let args_tokens: Vec<_> = args
                .iter()
                .map(expr_to_rust_tokens)
                .collect::<Result<Vec<_>>>()?;
            Ok(quote! { (#callee_tokens)(#(#args_tokens),*) })
        }
    }
}

fn literal_to_rust_tokens(lit: &Literal) -> Result<proc_macro2::TokenStream> {
    match lit {
        Literal::Int(i) => Ok(quote! { #i }),
        Literal::Float(f) => Ok(quote! { #f }),
        Literal::String(s) => Ok(quote! { #s.to_string() }),
        Literal::Bytes(b) => {
            // Generate byte string literal
            let byte_lit = proc_macro2::Literal::byte_string(b);
            Ok(quote! { #byte_lit })
        }
        Literal::Bool(b) => Ok(quote! { #b }),
        Literal::None => Ok(quote! { None }),
    }
}

fn binop_to_rust_tokens(op: &BinOp) -> proc_macro2::TokenStream {
    match op {
        BinOp::Add => quote! { + },
        BinOp::Sub => quote! { - },
        BinOp::Mul => quote! { * },
        BinOp::Div => quote! { / },
        BinOp::FloorDiv => quote! { / }, // Note: not exact equivalent
        BinOp::Mod => quote! { % },
        BinOp::Pow => quote! { .pow }, // Special handling needed
        BinOp::Eq => quote! { == },
        BinOp::NotEq => quote! { != },
        BinOp::Lt => quote! { < },
        BinOp::LtEq => quote! { <= },
        BinOp::Gt => quote! { > },
        BinOp::GtEq => quote! { >= },
        BinOp::And => quote! { && },
        BinOp::Or => quote! { || },
        BinOp::BitAnd => quote! { & },
        BinOp::BitOr => quote! { | },
        BinOp::BitXor => quote! { ^ },
        BinOp::LShift => quote! { << },
        BinOp::RShift => quote! { >> },
        BinOp::In => quote! { .contains }, // Special handling needed
        BinOp::NotIn => quote! { .not_contains }, // Special handling needed
    }
}

fn unaryop_to_rust_tokens(op: &UnaryOp) -> proc_macro2::TokenStream {
    match op {
        UnaryOp::Not => quote! { ! },
        UnaryOp::Neg => quote! { - },
        UnaryOp::Pos => quote! { + },
        UnaryOp::BitNot => quote! { ! },
    }
}

fn prettify_rust_code(code: String) -> String {
    // Very basic formatting - in production, use rustfmt
    code.replace(" ; ", ";\n    ")
        .replace(" { ", " {\n    ")
        .replace(" } ", "\n}\n")
        .replace("} ;", "};")
        .replace(
            "use std :: collections :: HashMap ;",
            "use std::collections::HashMap;",
        )
        // Fix method call spacing
        .replace(" . ", ".")
        // Fix operators with spaces BEFORE paren fixes
        // The syn pretty-printer sometimes generates ` ! = ` instead of ` != `
        .replace(" ! = ", " != ")
        .replace(" ! = (", " != (")
        .replace(") ! = ", ") != ")
        .replace(") ! = (", ") != (")
        .replace(" ! =", " !=")
        .replace("! = ", "!= ")
        .replace(" = = ", " == ")
        .replace(" = =", " ==")
        .replace("= = ", "== ")
        .replace(" < =", " <=")
        .replace(" > =", " >=")
        // Now fix spacing around parentheses
        .replace(" (", "(")
        .replace(" )", ")")
        // Fix != operator patterns created by paren removal (AFTER paren fixes)
        .replace(") ! =(", ") !=(")
        .replace(")! = (", ")!=(")
        .replace(") ! = (", ") !=(")
        .replace("&&((", "&&(")
        .replace(")!=(", ") !=(")
        // Now fix the !( and != spacing
        .replace("!=(", "!= (")
        // Add spaces around comparison operators (after paren fixes)
        .replace("(r<0", "(r < 0")
        .replace("(b<0", "(b < 0")
        .replace("r<0)", "r < 0)")
        .replace("b<0)", "b < 0)")
        .replace("r<0;", "r < 0;")
        .replace("b<0;", "b < 0;")
        .replace(" = r<0", " = r < 0")
        .replace(" = b<0", " = b < 0")
        .replace(" = n>0", " = n > 0")
        .replace(" = n<0", " = n < 0")
        // Generic comparison operator spacing - multiple passes to catch all patterns
        .replace(">0;", " > 0;")
        .replace("<0;", " < 0;")
        .replace(">=0;", " >= 0;")
        .replace("<=0;", " <= 0;")
        // Fix comparison operators in assignments/conditions (second pass after parens removed)
        .replace("n>0", "n > 0")
        .replace("n<0", "n < 0")
        .replace("r>0", "r > 0")
        .replace("b>0", "b > 0")
        .replace("a>0", "a > 0")
        .replace("x>0", "x > 0")
        .replace("y>0", "y > 0")
        .replace("<0)", " < 0)")
        .replace("<0))", " < 0))")
        // Fix specific common patterns
        .replace(".len ()", ".len()")
        .replace(".push (", ".push(")
        .replace(".insert (", ".insert(")
        .replace(".get (", ".get(")
        .replace(".contains_key (", ".contains_key(")
        .replace(".to_string ()", ".to_string()")
        // Fix control flow keywords (AFTER paren fixes to ensure proper spacing)
        .replace("if(", "if ")
        .replace("while(", "while ")
        .replace("for(", "for ")
        .replace("match(", "match ")
        .replace("} else", "}\nelse")
        // FINAL PASS: Catch any remaining != operator spacing issues
        .replace(" ! = ", " != ")
        .replace(") ! = ", ") != ")
        .replace(" ! =(", " !=(")
        .replace(") ! =(", ") !=(")
        .replace(" ! =", " !=")
        .replace("! = ", "!= ")
        // Fix spacing around operators in some contexts
        .replace(" ::", "::")
        // Fix attribute spacing
        .replace("# [", "#[")
        // Fix type annotations
        .replace(" : ", ": ")
        .replace(";\n    }", "\n}")
}

/// Check if an expression is a len() call
fn is_len_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, args , ..} if func == "len" && args.len() == 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_annotations::TranspilationAnnotations;

    #[test]
    fn test_simple_function_generation() {
        let func = HirFunction {
            name: "add".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int),
            ]
            .into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        let rust_code = hir_to_rust(&module).unwrap();
        assert!(rust_code.contains("pub fn add"));
        assert!(rust_code.contains("i32"));
        assert!(rust_code.contains("return(a + b)"));
    }

    #[test]
    fn test_type_conversion() {
        assert_eq!(type_to_rust_type(&Type::Int).to_string(), "i32");
        assert_eq!(type_to_rust_type(&Type::String).to_string(), "String");
        assert_eq!(
            type_to_rust_type(&Type::List(Box::new(Type::Int))).to_string(),
            "Vec < i32 >"
        );
        assert_eq!(
            type_to_rust_type(&Type::Optional(Box::new(Type::String))).to_string(),
            "Option < String >"
        );
    }

    #[test]
    fn test_literal_conversion() {
        let int_lit = literal_to_rust_tokens(&Literal::Int(42)).unwrap();
        assert_eq!(int_lit.to_string(), "42i64");

        let str_lit = literal_to_rust_tokens(&Literal::String("hello".to_string())).unwrap();
        assert_eq!(str_lit.to_string(), "\"hello\" . to_string ()");

        let bool_lit = literal_to_rust_tokens(&Literal::Bool(true)).unwrap();
        assert_eq!(bool_lit.to_string(), "true");
    }

    #[test]
    fn test_control_flow_generation() {
        let if_stmt = HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String(
                "positive".to_string(),
            ))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(
                Literal::String("negative".to_string()),
            )))]),
        };

        let tokens = stmt_to_rust_tokens(&if_stmt).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("if"));
        assert!(code.contains("else"));
        assert!(code.contains("return"));
    }

    #[test]
    fn test_needs_std_collections() {
        let module_with_dict = HirModule {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: vec![HirParam::new(
                    "data".to_string(),
                    Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
                )]
                .into(),
                ret_type: Type::None,
                body: vec![],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        assert!(needs_std_collections(&module_with_dict));

        let module_without_dict = HirModule {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: vec![HirParam::new("x".to_string(), Type::Int)].into(),
                ret_type: Type::Int,
                body: vec![],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        assert!(!needs_std_collections(&module_without_dict));
    }

    #[test]
    fn test_assignment_generation() {
        let assign = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        };

        let tokens = stmt_to_rust_tokens(&assign).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("let mut x = 42"));
    }

    #[test]
    fn test_function_call_generation() {
        let call = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))])],
            kwargs: vec![],
        };

        let tokens = expr_to_rust_tokens(&call).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("len"));
        assert!(code.contains("vec !") || code.contains("vec!"));
    }

    #[test]
    fn test_binary_operations() {
        let ops = vec![
            (BinOp::Add, "+"),
            (BinOp::Sub, "-"),
            (BinOp::Mul, "*"),
            (BinOp::Eq, "=="),
            (BinOp::Lt, "<"),
        ];

        for (op, expected) in ops {
            let tokens = binop_to_rust_tokens(&op);
            assert_eq!(tokens.to_string(), expected);
        }
    }

    #[test]
    fn test_floor_division_codegen() {
        // Test floor division with positive integers
        let floor_div = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(7))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };

        let tokens = expr_to_rust_tokens(&floor_div).unwrap();
        let code = tokens.to_string();
        // Should generate the floor division block
        assert!(code.contains("let a"));
        assert!(code.contains("let b"));
        assert!(code.contains("let q = a / b"));
        assert!(code.contains("let r = a % b"));

        // Test with negative operands
        let neg_floor_div = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(-7))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };

        let tokens = expr_to_rust_tokens(&neg_floor_div).unwrap();
        let code = tokens.to_string();
        // DEPYLER-0236: Floor division now uses intermediate boolean variables
        assert!(code.contains("let r_negative = r < 0"));
        assert!(code.contains("let b_negative = b < 0"));
        assert!(code.contains("let signs_differ = r_negative != b_negative"));
        assert!(code.contains("let needs_adjustment = r_nonzero && signs_differ"));
    }

    // DEPYLER-0012: Comprehensive tests for stmt_to_rust_tokens_with_scope
    // Target: Reduce complexity from 25 to ≤10
    // Coverage: All 10 statement types with scope tracking

    // 1. ASSIGN STATEMENTS (Symbol target - first declaration)
    #[test]
    fn test_assign_symbol_first_declaration() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("let mut x"));
        assert!(code.contains("42i64"));
        assert!(scope.is_declared("x"));
    }

    // 2. ASSIGN STATEMENTS (Symbol target - reassignment)
    #[test]
    fn test_assign_symbol_reassignment() {
        let mut scope = ScopeTracker::new();
        scope.declare_var("x");
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(99)),
            type_annotation: None,
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(!code.contains("let mut"));
        assert!(code.contains("x ="));
        assert!(code.contains("99i64"));
    }

    // 3. ASSIGN STATEMENTS (Index target)
    #[test]
    fn test_assign_index_target() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Assign {
            target: AssignTarget::Index {
                base: Box::new(HirExpr::Var("dict".to_string())),
                index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
            },
            value: HirExpr::Literal(Literal::Int(100)),
            type_annotation: None,
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("insert"));
        assert!(code.contains("dict"));
        assert!(code.contains("key"));
    }

    // 4. ASSIGN STATEMENTS (Attribute target - struct field assignment)
    #[test]
    fn test_assign_attribute() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Assign {
            target: AssignTarget::Attribute {
                value: Box::new(HirExpr::Var("obj".to_string())),
                attr: "field".to_string(),
            },
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        };
        let result = stmt_to_rust_tokens_with_scope(&stmt, &mut scope);
        assert!(result.is_ok());
        let code = result.unwrap().to_string();
        assert!(code.contains("obj"));
        assert!(code.contains("field"));
        assert!(code.contains("1"));
    }

    // 5. RETURN STATEMENTS (with expression)
    #[test]
    fn test_return_with_expression() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("a".to_string())),
            right: Box::new(HirExpr::Var("b".to_string())),
        }));
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("return"));
        assert!(code.contains("a + b"));
    }

    // 6. RETURN STATEMENTS (without expression)
    #[test]
    fn test_return_without_expression() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Return(None);
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert_eq!(code.trim(), "return ;");
    }

    // 7. IF STATEMENTS (with else)
    #[test]
    fn test_if_with_else_scope_tracking() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            }],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("z".to_string()),
                value: HirExpr::Literal(Literal::Int(-1)),
                type_annotation: None,
            }]),
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("if"));
        assert!(code.contains("x >"));
        assert!(code.contains("0i64"));
        assert!(code.contains("let mut y"));
        assert!(code.contains("else"));
        assert!(code.contains("let mut z"));
        // Variables declared in if/else scopes should not leak
        assert!(!scope.is_declared("y"));
        assert!(!scope.is_declared("z"));
    }

    // 8. IF STATEMENTS (without else)
    #[test]
    fn test_if_without_else() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::If {
            condition: HirExpr::Var("flag".to_string()),
            then_body: vec![HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![HirExpr::Literal(Literal::String("yes".to_string()))],
                kwargs: vec![],
            })],
            else_body: None,
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("if flag"));
        assert!(!code.contains("else"));
    }

    // 9. WHILE STATEMENTS
    #[test]
    fn test_while_loop_scope_tracking() {
        let mut scope = ScopeTracker::new();
        scope.declare_var("count");
        let stmt = HirStmt::While {
            condition: HirExpr::Binary {
                op: BinOp::Lt,
                left: Box::new(HirExpr::Var("count".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(10))),
            },
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("temp".to_string()),
                    value: HirExpr::Literal(Literal::Int(0)),
                    type_annotation: None,
                },
                HirStmt::Assign {
                    target: AssignTarget::Symbol("count".to_string()),
                    value: HirExpr::Binary {
                        op: BinOp::Add,
                        left: Box::new(HirExpr::Var("count".to_string())),
                        right: Box::new(HirExpr::Literal(Literal::Int(1))),
                    },
                    type_annotation: None,
                },
            ],
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("while"));
        assert!(code.contains("count <"));
        assert!(code.contains("10i64"));
        assert!(code.contains("let mut temp"));
        assert!(code.contains("count ="));
        // temp declared in while scope should not leak
        assert!(!scope.is_declared("temp"));
        assert!(scope.is_declared("count"));
    }

    // 10. FOR STATEMENTS
    #[test]
    fn test_for_loop_scope_tracking() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(5))],
                kwargs: vec![],
            },
            body: vec![HirStmt::Expr(HirExpr::Var("i".to_string()))],
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("for i in range"));
        assert!(code.contains("5i64"));
        // Loop variable should not leak to outer scope
        assert!(!scope.is_declared("i"));
    }

    // 11. EXPR STATEMENTS
    #[test]
    fn test_expr_statement() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Expr(HirExpr::Call {
            func: "println".to_string(),
            args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
            kwargs: vec![],
        });
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("println"));
        assert!(code.contains("hello"));
        assert!(code.ends_with(';') || code.trim().ends_with(';'));
    }

    // 12. RAISE STATEMENTS (with exception)
    #[test]
    fn test_raise_with_exception() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Raise {
            exception: Some(HirExpr::Literal(Literal::String("Error!".to_string()))),
            cause: None,
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("panic"));
        assert!(code.contains("Exception"));
        assert!(code.contains("Error!"));
    }

    // 13. RAISE STATEMENTS (without exception)
    #[test]
    fn test_raise_without_exception() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Raise {
            exception: None,
            cause: None,
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("panic"));
        assert!(code.contains("Exception raised"));
    }

    // 14. BREAK STATEMENTS (with label)
    #[test]
    fn test_break_with_label() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Break {
            label: Some("outer".to_string()),
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("break"));
        assert!(code.contains("'outer"));
    }

    // 15. BREAK STATEMENTS (without label)
    #[test]
    fn test_break_without_label() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Break { label: None };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert_eq!(code.trim(), "break ;");
    }

    // 16. CONTINUE STATEMENTS (with label)
    #[test]
    fn test_continue_with_label() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Continue {
            label: Some("outer".to_string()),
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("continue"));
        assert!(code.contains("'outer"));
    }

    // 17. CONTINUE STATEMENTS (without label)
    #[test]
    fn test_continue_without_label() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::Continue { label: None };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert_eq!(code.trim(), "continue ;");
    }

    // 18. WITH STATEMENTS (with target)
    #[test]
    fn test_with_statement_with_target() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::With {
            context: HirExpr::Call {
                func: "open".to_string(),
                args: vec![HirExpr::Literal(Literal::String("file.txt".to_string()))],
                kwargs: vec![],
            },
            target: Some("f".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "read".to_string(),
                args: vec![HirExpr::Var("f".to_string())],
                kwargs: vec![],
            })],
            is_async: false,
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("let mut f"));
        assert!(code.contains("open"));
        assert!(code.contains("file.txt"));
        assert!(code.contains("read"));
    }

    // 19. WITH STATEMENTS (without target)
    #[test]
    fn test_with_statement_without_target() {
        let mut scope = ScopeTracker::new();
        let stmt = HirStmt::With {
            context: HirExpr::Call {
                func: "lock".to_string(),
                args: vec![],
                kwargs: vec![],
            },
            target: None,
            body: vec![HirStmt::Expr(HirExpr::Literal(Literal::String(
                "critical".to_string(),
            )))],
            is_async: false,
        };
        let tokens = stmt_to_rust_tokens_with_scope(&stmt, &mut scope).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("let _context"));
        assert!(code.contains("lock"));
        assert!(code.contains("critical"));
    }

    // 20. SCOPE TRACKING - Nested scopes
    #[test]
    fn test_nested_scope_tracking() {
        let mut scope = ScopeTracker::new();

        // Declare x in outer scope
        scope.declare_var("x");
        assert!(scope.is_declared("x"));

        // Enter inner scope
        scope.enter_scope();

        // x should still be visible
        assert!(scope.is_declared("x"));

        // Declare y in inner scope
        scope.declare_var("y");
        assert!(scope.is_declared("y"));

        // Exit inner scope
        scope.exit_scope();

        // x should still be visible
        assert!(scope.is_declared("x"));

        // y should no longer be visible
        assert!(!scope.is_declared("y"));
    }

    // DEPYLER-COV-001: Additional tests for coverage improvement

    #[test]
    fn test_while_stmt_generation() {
        let while_stmt = HirStmt::While {
            condition: HirExpr::Binary {
                op: BinOp::Lt,
                left: Box::new(HirExpr::Var("i".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(10))),
            },
            body: vec![HirStmt::Expr(HirExpr::Var("i".to_string()))],
        };

        let tokens = stmt_to_rust_tokens(&while_stmt).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("while"));
    }

    #[test]
    fn test_for_stmt_generation() {
        let for_stmt = HirStmt::For {
            target: AssignTarget::Symbol("x".to_string()),
            iter: HirExpr::List(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ]),
            body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
        };

        let tokens = stmt_to_rust_tokens(&for_stmt).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("for"));
    }

    #[test]
    fn test_tuple_literal_generation() {
        let tuple = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("hello".to_string())),
        ]);

        let tokens = expr_to_rust_tokens(&tuple).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("1i64"));
        assert!(code.contains("hello"));
    }

    #[test]
    fn test_lambda_generation() {
        let lambda = HirExpr::Lambda {
            params: vec!["x".to_string()],
            body: Box::new(HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
        };

        let tokens = expr_to_rust_tokens(&lambda).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("|"));
    }

    #[test]
    fn test_list_comp_generation() {
        let list_comp = HirExpr::ListComp {
            element: Box::new(HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(HirExpr::List(vec![
                    HirExpr::Literal(Literal::Int(1)),
                    HirExpr::Literal(Literal::Int(2)),
                ])),
                conditions: vec![],
            }],
        };

        let tokens = expr_to_rust_tokens(&list_comp).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("iter") || code.contains("map") || code.contains("collect"));
    }

    #[test]
    fn test_slice_generation() {
        let slice = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
            stop: Some(Box::new(HirExpr::Literal(Literal::Int(3)))),
            step: None,
        };

        let tokens = expr_to_rust_tokens(&slice).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("arr") || code.contains("["));
    }

    #[test]
    fn test_method_call_generation() {
        let method_call = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "upper".to_string(),
            args: vec![],
            kwargs: vec![],
        };

        let tokens = expr_to_rust_tokens(&method_call).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("upper") || code.contains("to_uppercase"));
    }

    #[test]
    fn test_set_literal_generation() {
        let set_lit = HirExpr::Set(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);

        let tokens = expr_to_rust_tokens(&set_lit).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("HashSet") || code.contains("set") || code.contains("insert"));
    }

    #[test]
    fn test_dict_literal_generation() {
        let dict = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);

        let tokens = expr_to_rust_tokens(&dict).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("HashMap") || code.contains("insert") || code.contains("key"));
    }

    #[test]
    fn test_borrow_expr_generation() {
        let borrow = HirExpr::Borrow {
            expr: Box::new(HirExpr::Var("x".to_string())),
            mutable: true,
        };

        let tokens = expr_to_rust_tokens(&borrow).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("mut") || code.contains("&"));
    }

    #[test]
    fn test_index_expr_generation() {
        let index = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };

        let tokens = expr_to_rust_tokens(&index).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("arr"));
    }

    #[test]
    fn test_attribute_expr_generation() {
        let attr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };

        let tokens = expr_to_rust_tokens(&attr).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("obj") || code.contains("field"));
    }

    #[test]
    fn test_if_expr_generation() {
        let if_expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };

        let tokens = expr_to_rust_tokens(&if_expr).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("if") || code.contains("else"));
    }

    #[test]
    fn test_unary_op_not() {
        let not_expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };

        let tokens = expr_to_rust_tokens(&not_expr).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("!") || code.contains("not"));
    }

    #[test]
    fn test_unary_op_neg() {
        let neg_expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };

        let tokens = expr_to_rust_tokens(&neg_expr).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("-"));
    }

    #[test]
    fn test_uses_hashmap_nested() {
        // Dict in List
        let nested = Type::List(Box::new(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int),
        )));
        assert!(uses_hashmap(&nested));

        // Dict in Optional
        let optional_dict = Type::Optional(Box::new(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int),
        )));
        assert!(uses_hashmap(&optional_dict));

        // Dict in Tuple
        let tuple_dict = Type::Tuple(vec![
            Type::Int,
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
        ]);
        assert!(uses_hashmap(&tuple_dict));

        // Dict in Function type
        let func_dict = Type::Function {
            params: vec![Type::Dict(Box::new(Type::String), Box::new(Type::Int))],
            ret: Box::new(Type::Int),
        };
        assert!(uses_hashmap(&func_dict));

        // Function returning dict
        let func_ret_dict = Type::Function {
            params: vec![Type::Int],
            ret: Box::new(Type::Dict(Box::new(Type::String), Box::new(Type::Int))),
        };
        assert!(uses_hashmap(&func_ret_dict));
    }

    #[test]
    fn test_expr_uses_hashmap_in_call() {
        let call = HirExpr::Call {
            func: "test".to_string(),
            args: vec![HirExpr::Dict(vec![])],
            kwargs: vec![],
        };
        assert!(expr_uses_hashmap(&call));
    }

    #[test]
    fn test_expr_uses_hashmap_in_binary() {
        let binary = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Dict(vec![])),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(expr_uses_hashmap(&binary));
    }

    #[test]
    fn test_expr_uses_hashmap_in_unary() {
        let unary = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Dict(vec![])),
        };
        assert!(expr_uses_hashmap(&unary));
    }

    #[test]
    fn test_expr_uses_hashmap_in_index() {
        let index = HirExpr::Index {
            base: Box::new(HirExpr::Dict(vec![])),
            index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
        };
        assert!(expr_uses_hashmap(&index));
    }

    #[test]
    fn test_expr_uses_hashmap_in_list() {
        let list = HirExpr::List(vec![HirExpr::Dict(vec![])]);
        assert!(expr_uses_hashmap(&list));
    }

    #[test]
    fn test_expr_uses_hashmap_in_tuple() {
        let tuple = HirExpr::Tuple(vec![HirExpr::Dict(vec![])]);
        assert!(expr_uses_hashmap(&tuple));
    }

    #[test]
    fn test_stmt_uses_hashmap_in_return() {
        let return_stmt = HirStmt::Return(Some(HirExpr::Dict(vec![])));
        assert!(stmt_uses_hashmap(&return_stmt));
    }

    #[test]
    fn test_stmt_uses_hashmap_in_if_condition() {
        let if_stmt = HirStmt::If {
            condition: HirExpr::Dict(vec![]),
            then_body: vec![],
            else_body: None,
        };
        assert!(stmt_uses_hashmap(&if_stmt));
    }

    #[test]
    fn test_stmt_uses_hashmap_in_if_then() {
        let if_stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Expr(HirExpr::Dict(vec![]))],
            else_body: None,
        };
        assert!(stmt_uses_hashmap(&if_stmt));
    }

    #[test]
    fn test_stmt_uses_hashmap_in_if_else() {
        let if_stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![],
            else_body: Some(vec![HirStmt::Expr(HirExpr::Dict(vec![]))]),
        };
        assert!(stmt_uses_hashmap(&if_stmt));
    }

    #[test]
    fn test_stmt_uses_hashmap_in_while() {
        let while_stmt = HirStmt::While {
            condition: HirExpr::Dict(vec![]),
            body: vec![],
        };
        assert!(stmt_uses_hashmap(&while_stmt));

        let while_body = HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Expr(HirExpr::Dict(vec![]))],
        };
        assert!(stmt_uses_hashmap(&while_body));
    }

    #[test]
    fn test_stmt_uses_hashmap_in_for() {
        let for_iter = HirStmt::For {
            target: AssignTarget::Symbol("x".to_string()),
            iter: HirExpr::Dict(vec![]),
            body: vec![],
        };
        assert!(stmt_uses_hashmap(&for_iter));

        let for_body = HirStmt::For {
            target: AssignTarget::Symbol("x".to_string()),
            iter: HirExpr::List(vec![]),
            body: vec![HirStmt::Expr(HirExpr::Dict(vec![]))],
        };
        assert!(stmt_uses_hashmap(&for_body));
    }

    #[test]
    fn test_stmt_not_uses_hashmap() {
        let pass = HirStmt::Pass;
        assert!(!stmt_uses_hashmap(&pass));

        let break_stmt = HirStmt::Break { label: None };
        assert!(!stmt_uses_hashmap(&break_stmt));

        let continue_stmt = HirStmt::Continue { label: None };
        assert!(!stmt_uses_hashmap(&continue_stmt));

        let return_none = HirStmt::Return(None);
        assert!(!stmt_uses_hashmap(&return_none));
    }

    #[test]
    fn test_is_len_call() {
        let len_call = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_len_call(&len_call));

        // Not a len call
        let other_call = HirExpr::Call {
            func: "sum".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(!is_len_call(&other_call));

        // Wrong number of args
        let wrong_args = HirExpr::Call {
            func: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_len_call(&wrong_args));
    }

    #[test]
    fn test_prettify_rust_code() {
        let code = "fn test(){let x=1;x+2}".to_string();
        let prettified = prettify_rust_code(code);
        // Should add some whitespace
        assert!(!prettified.is_empty());
    }

    #[test]
    fn test_resolve_union_type_single() {
        let types = vec![Type::Int];
        let tokens = resolve_union_type(&types);
        let code = tokens.to_string();
        assert!(code.contains("i32") || code.contains("i64"));
    }

    #[test]
    fn test_resolve_union_type_multiple() {
        let types = vec![Type::Int, Type::String];
        let tokens = resolve_union_type(&types);
        let code = tokens.to_string();
        // Should create an enum or some union representation
        assert!(!code.is_empty());
    }

    #[test]
    fn test_type_to_rust_type_set() {
        let set_type = Type::Set(Box::new(Type::Int));
        let tokens = type_to_rust_type(&set_type);
        let code = tokens.to_string();
        assert!(code.contains("HashSet"));
    }

    #[test]
    fn test_type_to_rust_type_none() {
        let none_type = Type::None;
        let tokens = type_to_rust_type(&none_type);
        let code = tokens.to_string();
        assert!(code.contains("()") || code.contains("None"));
    }

    #[test]
    fn test_type_to_rust_type_unknown() {
        let unknown_type = Type::Unknown;
        let tokens = type_to_rust_type(&unknown_type);
        // Unknown type should produce some output
        assert!(!tokens.to_string().is_empty());
    }

    #[test]
    fn test_break_continue_pass_stmts() {
        let break_stmt = HirStmt::Break { label: None };
        let break_tokens = stmt_to_rust_tokens(&break_stmt).unwrap();
        assert!(break_tokens.to_string().contains("break"));

        let continue_stmt = HirStmt::Continue { label: None };
        let continue_tokens = stmt_to_rust_tokens(&continue_stmt).unwrap();
        assert!(continue_tokens.to_string().contains("continue"));

        let pass_stmt = HirStmt::Pass;
        let pass_tokens = stmt_to_rust_tokens(&pass_stmt).unwrap();
        // Pass becomes empty or {}
        let _ = pass_tokens; // Just check it doesn't panic
    }

    #[test]
    fn test_assign_with_binary_value() {
        // Test assignment with a binary expression
        let assign = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("y".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            type_annotation: None,
        };

        let tokens = stmt_to_rust_tokens(&assign).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("x") || code.contains("let"));
    }

    #[test]
    fn test_modulo_operation() {
        let mod_expr = HirExpr::Binary {
            op: BinOp::Mod,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };

        let tokens = expr_to_rust_tokens(&mod_expr).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("%"));
    }

    #[test]
    fn test_power_operation() {
        let pow_expr = HirExpr::Binary {
            op: BinOp::Pow,
            left: Box::new(HirExpr::Literal(Literal::Int(2))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };

        let tokens = expr_to_rust_tokens(&pow_expr).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("pow") || code.contains("**"));
    }

    #[test]
    fn test_logical_and_or() {
        let and_expr = HirExpr::Binary {
            op: BinOp::And,
            left: Box::new(HirExpr::Literal(Literal::Bool(true))),
            right: Box::new(HirExpr::Literal(Literal::Bool(false))),
        };

        let tokens = expr_to_rust_tokens(&and_expr).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("&&") || code.contains("and"));

        let or_expr = HirExpr::Binary {
            op: BinOp::Or,
            left: Box::new(HirExpr::Literal(Literal::Bool(true))),
            right: Box::new(HirExpr::Literal(Literal::Bool(false))),
        };

        let tokens = expr_to_rust_tokens(&or_expr).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("||") || code.contains("or"));
    }

    #[test]
    fn test_comparison_operations() {
        let ops = vec![
            (BinOp::Gt, ">"),
            (BinOp::GtEq, ">="),
            (BinOp::LtEq, "<="),
            (BinOp::NotEq, "!="),
        ];

        for (op, expected) in ops {
            let tokens = binop_to_rust_tokens(&op);
            assert_eq!(tokens.to_string(), expected);
        }
    }

    #[test]
    fn test_bitwise_operations() {
        let ops = vec![
            (BinOp::BitAnd, "&"),
            (BinOp::BitOr, "|"),
            (BinOp::BitXor, "^"),
        ];

        for (op, expected) in ops {
            let tokens = binop_to_rust_tokens(&op);
            assert_eq!(tokens.to_string(), expected);
        }
    }

    #[test]
    fn test_shift_operations() {
        let left_shift = HirExpr::Binary {
            op: BinOp::LShift,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };

        let tokens = expr_to_rust_tokens(&left_shift).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("<<"));

        let right_shift = HirExpr::Binary {
            op: BinOp::RShift,
            left: Box::new(HirExpr::Literal(Literal::Int(4))),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };

        let tokens = expr_to_rust_tokens(&right_shift).unwrap();
        let code = tokens.to_string();
        assert!(code.contains(">>"));
    }

    #[test]
    fn test_f_string_not_implemented() {
        let fstring = HirExpr::FString {
            parts: vec![
                FStringPart::Literal("Hello, ".to_string()),
                FStringPart::Expr(Box::new(HirExpr::Var("name".to_string()))),
                FStringPart::Literal("!".to_string()),
            ],
        };

        // FString is not yet implemented in codegen - test that it returns an error
        let result = expr_to_rust_tokens(&fstring);
        assert!(result.is_err());
    }

    #[test]
    fn test_literal_float() {
        let float_lit = Literal::Float(3.15);
        let tokens = literal_to_rust_tokens(&float_lit).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("3.15") || code.contains("f64"));
    }

    #[test]
    fn test_literal_none() {
        let none_lit = Literal::None;
        let tokens = literal_to_rust_tokens(&none_lit).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("None") || code.contains("()"));
    }

    #[test]
    fn test_function_type() {
        let func_type = Type::Function {
            params: vec![Type::Int, Type::String],
            ret: Box::new(Type::Bool),
        };
        let tokens = type_to_rust_type(&func_type);
        let code = tokens.to_string();
        assert!(code.contains("Fn") || code.contains("fn") || code.contains("impl"));
    }

    #[test]
    fn test_custom_type() {
        let custom = Type::Custom("MyClass".to_string());
        let tokens = type_to_rust_type(&custom);
        let code = tokens.to_string();
        assert!(code.contains("MyClass"));
    }

    #[test]
    fn test_type_var() {
        let type_var = Type::TypeVar("T".to_string());
        let tokens = type_to_rust_type(&type_var);
        let code = tokens.to_string();
        assert!(code.contains("T"));
    }

    #[test]
    fn test_union_type() {
        let union = Type::Union(vec![Type::Int, Type::String]);
        let tokens = type_to_rust_type(&union);
        // Union types should produce some output
        assert!(!tokens.to_string().is_empty());
    }

    #[test]
    fn test_array_type() {
        let array = Type::Array {
            element_type: Box::new(Type::Int),
            size: ConstGeneric::Literal(10),
        };
        let tokens = type_to_rust_type(&array);
        let code = tokens.to_string();
        assert!(code.contains("[") || code.contains("i32"));
    }

    // Tests for frozen_set_to_rust_tokens
    #[test]
    fn test_frozen_set_to_rust_tokens_empty() {
        let items: Vec<HirExpr> = vec![];
        let result = frozen_set_to_rust_tokens(&items).unwrap();
        let code = result.to_string();
        assert!(code.contains("HashSet"));
        assert!(code.contains("Arc"));
    }

    #[test]
    fn test_frozen_set_to_rust_tokens_with_items() {
        let items = vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ];
        let result = frozen_set_to_rust_tokens(&items).unwrap();
        let code = result.to_string();
        assert!(code.contains("HashSet"));
        assert!(code.contains("insert"));
        assert!(code.contains("Arc"));
    }

    // Tests for set_comp_to_rust_tokens
    #[test]
    fn test_set_comp_to_rust_tokens_no_condition() {
        let element = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let iter = HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(5))],
            kwargs: vec![],
        };
        let result = set_comp_to_rust_tokens(&element, "x", &iter, &None).unwrap();
        let code = result.to_string();
        assert!(code.contains("into_iter"));
        assert!(code.contains("map"));
        assert!(code.contains("HashSet"));
    }

    #[test]
    fn test_set_comp_to_rust_tokens_with_condition() {
        let element = HirExpr::Var("x".to_string());
        let iter = HirExpr::Var("items".to_string());
        let condition = Some(Box::new(HirExpr::Binary {
            op: BinOp::Gt,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(0))),
        }));
        let result = set_comp_to_rust_tokens(&element, "x", &iter, &condition).unwrap();
        let code = result.to_string();
        assert!(code.contains("into_iter"));
        assert!(code.contains("filter"));
        assert!(code.contains("map"));
        assert!(code.contains("HashSet"));
    }

    // Tests for dict_comp_to_rust_tokens
    #[test]
    fn test_dict_comp_to_rust_tokens_no_condition() {
        let key = HirExpr::Var("x".to_string());
        let value = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Var("x".to_string())),
        };
        let iter = HirExpr::Var("items".to_string());
        let result = dict_comp_to_rust_tokens(&key, &value, "x", &iter, &None).unwrap();
        let code = result.to_string();
        assert!(code.contains("into_iter"));
        assert!(code.contains("map"));
        assert!(code.contains("HashMap"));
    }

    #[test]
    fn test_dict_comp_to_rust_tokens_with_condition() {
        let key = HirExpr::Var("k".to_string());
        let value = HirExpr::Var("v".to_string());
        let iter = HirExpr::Var("pairs".to_string());
        let condition = Some(Box::new(HirExpr::Binary {
            op: BinOp::NotEq,
            left: Box::new(HirExpr::Var("v".to_string())),
            right: Box::new(HirExpr::Literal(Literal::None)),
        }));
        let result = dict_comp_to_rust_tokens(&key, &value, "k", &iter, &condition).unwrap();
        let code = result.to_string();
        assert!(code.contains("into_iter"));
        assert!(code.contains("filter"));
        assert!(code.contains("map"));
        assert!(code.contains("HashMap"));
    }

    // Tests for binop_to_rust_tokens
    #[test]
    fn test_binop_to_rust_tokens_all_ops() {
        assert_eq!(binop_to_rust_tokens(&BinOp::Add).to_string(), "+");
        assert_eq!(binop_to_rust_tokens(&BinOp::Sub).to_string(), "-");
        assert_eq!(binop_to_rust_tokens(&BinOp::Mul).to_string(), "*");
        assert_eq!(binop_to_rust_tokens(&BinOp::Div).to_string(), "/");
        assert_eq!(binop_to_rust_tokens(&BinOp::FloorDiv).to_string(), "/");
        assert_eq!(binop_to_rust_tokens(&BinOp::Mod).to_string(), "%");
        assert!(binop_to_rust_tokens(&BinOp::Pow)
            .to_string()
            .contains("pow"));
        assert_eq!(binop_to_rust_tokens(&BinOp::Eq).to_string(), "==");
        assert_eq!(binop_to_rust_tokens(&BinOp::NotEq).to_string(), "!=");
        assert_eq!(binop_to_rust_tokens(&BinOp::Lt).to_string(), "<");
        assert_eq!(binop_to_rust_tokens(&BinOp::LtEq).to_string(), "<=");
        assert_eq!(binop_to_rust_tokens(&BinOp::Gt).to_string(), ">");
        assert_eq!(binop_to_rust_tokens(&BinOp::GtEq).to_string(), ">=");
        assert_eq!(binop_to_rust_tokens(&BinOp::And).to_string(), "&&");
        assert_eq!(binop_to_rust_tokens(&BinOp::Or).to_string(), "||");
        assert_eq!(binop_to_rust_tokens(&BinOp::BitAnd).to_string(), "&");
        assert_eq!(binop_to_rust_tokens(&BinOp::BitOr).to_string(), "|");
        assert_eq!(binop_to_rust_tokens(&BinOp::BitXor).to_string(), "^");
        assert_eq!(binop_to_rust_tokens(&BinOp::LShift).to_string(), "<<");
        assert_eq!(binop_to_rust_tokens(&BinOp::RShift).to_string(), ">>");
        assert!(binop_to_rust_tokens(&BinOp::In)
            .to_string()
            .contains("contains"));
        assert!(binop_to_rust_tokens(&BinOp::NotIn)
            .to_string()
            .contains("contains"));
    }

    // Tests for unaryop_to_rust_tokens
    #[test]
    fn test_unaryop_to_rust_tokens_all_ops() {
        assert_eq!(unaryop_to_rust_tokens(&UnaryOp::Not).to_string(), "!");
        assert_eq!(unaryop_to_rust_tokens(&UnaryOp::Neg).to_string(), "-");
        assert_eq!(unaryop_to_rust_tokens(&UnaryOp::Pos).to_string(), "+");
        assert_eq!(unaryop_to_rust_tokens(&UnaryOp::BitNot).to_string(), "!");
    }

    // Tests for literal_to_rust_tokens
    #[test]
    fn test_literal_to_rust_tokens_bytes() {
        let bytes = Literal::Bytes(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]); // "Hello"
        let result = literal_to_rust_tokens(&bytes).unwrap();
        let code = result.to_string();
        // Should generate a byte string
        assert!(code.contains("b\"") || code.contains("Hello"));
    }

    #[test]
    fn test_literal_to_rust_tokens_all_types() {
        assert_eq!(
            literal_to_rust_tokens(&Literal::Int(42))
                .unwrap()
                .to_string(),
            "42i64"
        );
        assert!(literal_to_rust_tokens(&Literal::Float(3.15))
            .unwrap()
            .to_string()
            .contains("3.15"));
        assert!(literal_to_rust_tokens(&Literal::String("test".to_string()))
            .unwrap()
            .to_string()
            .contains("test"));
        assert_eq!(
            literal_to_rust_tokens(&Literal::Bool(true))
                .unwrap()
                .to_string(),
            "true"
        );
        assert_eq!(
            literal_to_rust_tokens(&Literal::Bool(false))
                .unwrap()
                .to_string(),
            "false"
        );
        assert_eq!(
            literal_to_rust_tokens(&Literal::None).unwrap().to_string(),
            "None"
        );
    }

    // Tests for type_to_rust_type edge cases
    #[test]
    fn test_type_to_rust_type_tuple() {
        let tuple = Type::Tuple(vec![Type::Int, Type::String, Type::Bool]);
        let tokens = type_to_rust_type(&tuple);
        let code = tokens.to_string();
        assert!(code.contains("i64") || code.contains("String") || code.contains("bool"));
    }

    #[test]
    fn test_type_to_rust_type_optional() {
        let optional = Type::Optional(Box::new(Type::String));
        let tokens = type_to_rust_type(&optional);
        let code = tokens.to_string();
        assert!(code.contains("Option"));
    }

    #[test]
    fn test_type_to_rust_type_list() {
        let list = Type::List(Box::new(Type::Int));
        let tokens = type_to_rust_type(&list);
        let code = tokens.to_string();
        assert!(code.contains("Vec"));
    }

    // Tests for uses_hashmap edge cases
    #[test]
    fn test_uses_hashmap_in_optional() {
        let ty = Type::Optional(Box::new(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int),
        )));
        assert!(uses_hashmap(&ty));
    }

    #[test]
    fn test_uses_hashmap_in_tuple() {
        let ty = Type::Tuple(vec![
            Type::Int,
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
        ]);
        assert!(uses_hashmap(&ty));
    }

    #[test]
    fn test_uses_hashmap_false_for_simple() {
        assert!(!uses_hashmap(&Type::Int));
        assert!(!uses_hashmap(&Type::String));
        assert!(!uses_hashmap(&Type::Bool));
        assert!(!uses_hashmap(&Type::Float));
    }

    // Tests for expr_uses_hashmap edge cases
    #[test]
    fn test_expr_uses_hashmap_direct_dict() {
        let expr = HirExpr::Dict(vec![]);
        assert!(expr_uses_hashmap(&expr));
    }

    #[test]
    fn test_expr_uses_hashmap_nested_in_list() {
        let expr = HirExpr::List(vec![HirExpr::Dict(vec![])]);
        assert!(expr_uses_hashmap(&expr));
    }

    #[test]
    fn test_expr_uses_hashmap_in_index_base() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Dict(vec![])),
            index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
        };
        assert!(expr_uses_hashmap(&expr));
    }

    #[test]
    fn test_expr_uses_hashmap_unary() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Call {
                func: "test".to_string(),
                args: vec![HirExpr::Dict(vec![])],
                kwargs: vec![],
            }),
        };
        assert!(expr_uses_hashmap(&expr));
    }

    // Tests for stmt_uses_hashmap edge cases
    #[test]
    fn test_stmt_uses_hashmap_in_assign() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Dict(vec![]),
            type_annotation: None,
        };
        assert!(stmt_uses_hashmap(&stmt));
    }

    #[test]
    fn test_stmt_uses_hashmap_in_expr() {
        let stmt = HirStmt::Expr(HirExpr::Dict(vec![]));
        assert!(stmt_uses_hashmap(&stmt));
    }

    #[test]
    fn test_stmt_uses_hashmap_in_for_iter() {
        let stmt = HirStmt::For {
            target: AssignTarget::Symbol("k".to_string()),
            iter: HirExpr::Call {
                func: "keys".to_string(),
                args: vec![HirExpr::Dict(vec![])],
                kwargs: vec![],
            },
            body: vec![],
        };
        assert!(stmt_uses_hashmap(&stmt));
    }
}
