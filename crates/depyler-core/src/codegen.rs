use crate::hir::*;
use anyhow::Result;
use quote::{quote, ToTokens};
use std::collections::HashSet;
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

struct ScopeTracker {
    declared_vars: Vec<HashSet<String>>,
}

impl ScopeTracker {
    fn new() -> Self {
        Self {
            declared_vars: vec![HashSet::new()],
        }
    }

    fn enter_scope(&mut self) {
        self.declared_vars.push(HashSet::new());
    }

    fn exit_scope(&mut self) {
        self.declared_vars.pop();
    }

    fn is_declared(&self, var_name: &str) -> bool {
        self.declared_vars
            .iter()
            .any(|scope| scope.contains(var_name))
    }

    fn declare_var(&mut self, var_name: &str) {
        if let Some(current_scope) = self.declared_vars.last_mut() {
            current_scope.insert(var_name.to_string());
        }
    }
}

fn convert_function_to_rust(func: &HirFunction) -> Result<proc_macro2::TokenStream> {
    let name = syn::Ident::new(&func.name, proc_macro2::Span::call_site());

    // Convert parameters
    let params: Vec<_> = func
        .params
        .iter()
        .map(|param| {
            let param_ident = syn::Ident::new(&param.name, proc_macro2::Span::call_site());
            let rust_type = type_to_rust_type(&param.ty);
            quote! { #param_ident: #rust_type }
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
        Type::Function { params, ret } => {
            let param_types: Vec<_> = params.iter().map(type_to_rust_type).collect();
            let ret_type = type_to_rust_type(ret);
            quote! { fn(#(#param_types),*) -> #ret_type }
        }
        Type::Custom(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! { #ident }
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
        Type::Union(_) => quote! { UnionType }, // Placeholder, will be handled by enum generation
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
    }
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
                        symbols.iter().for_each(|s| scope_tracker.declare_var(s));
                        let idents: Vec<_> = symbols
                            .iter()
                            .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                            .collect();
                        Ok(quote! { let (mut #(#idents),*) = #value_tokens; })
                    }
                }
                None => {
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
    target: &str,
    iter: &HirExpr,
    body: &[HirStmt],
    scope_tracker: &mut ScopeTracker,
) -> Result<proc_macro2::TokenStream> {
    let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
    let iter_tokens = expr_to_rust_tokens(iter)?;
    scope_tracker.enter_scope();
    scope_tracker.declare_var(target);
    let body_stmts: Vec<_> = body
        .iter()
        .map(|stmt| stmt_to_rust_tokens_with_scope(stmt, scope_tracker))
        .collect::<Result<Vec<_>>>()?;
    scope_tracker.exit_scope();
    Ok(quote! {
        for #target_ident in #iter_tokens {
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

                if let Some(finally_code) = finally_stmts {
                    Ok(quote! {
                        {
                            let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                #(#try_stmts)*
                                Ok(())
                            })();
                            if let Err(_e) = _result {
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
                            if let Err(_e) = _result {
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
        HirStmt::Pass => {
            // Pass statement generates no code
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
            Ok(quote! {
                {
                    let a = #left_tokens;
                    let b = #right_tokens;
                    let q = a / b;
                    let r = a % b;
                    if (r != 0) && ((r < 0) != (b < 0)) { q - 1 } else { q }
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
    Ok(quote! {
        {
            let mut map = HashMap::new();
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
    if params.is_empty() {
        Ok(quote! { || #body_tokens })
    } else {
        Ok(quote! { |#(#param_idents),*| #body_tokens })
    }
}

/// Convert set literal to Rust HashSet
/// Complexity: 1 (within ≤10 target)
fn set_literal_to_rust_tokens(items: &[HirExpr]) -> Result<proc_macro2::TokenStream> {
    let item_tokens: Vec<_> = items
        .iter()
        .map(expr_to_rust_tokens)
        .collect::<Result<Vec<_>>>()?;
    Ok(quote! {
        {
            let mut set = HashSet::new();
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
    Ok(quote! {
        {
            let mut set = HashSet::new();
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

    if let Some(cond) = condition {
        // With condition: iter().filter().map().collect()
        let cond_tokens = expr_to_rust_tokens(cond)?;
        Ok(quote! {
            #iter_tokens
                .into_iter()
                .filter(|#target_ident| #cond_tokens)
                .map(|#target_ident| #element_tokens)
                .collect::<HashSet<_>>()
        })
    } else {
        // Without condition: iter().map().collect()
        Ok(quote! {
            #iter_tokens
                .into_iter()
                .map(|#target_ident| #element_tokens)
                .collect::<HashSet<_>>()
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
        HirExpr::Call { func, args } => call_expr_to_rust_tokens(func, args),
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
        } => method_call_to_rust_tokens(object, method, args),
        HirExpr::Slice {
            base,
            start,
            stop,
            step,
        } => slice_expr_to_rust_tokens(base, start, stop, step),
        HirExpr::ListComp {
            element,
            target,
            iter,
            condition,
        } => list_comp_to_rust_tokens(element, target, iter, condition),
        HirExpr::Lambda { params, body } => lambda_to_rust_tokens(params, body),
        HirExpr::Set(items) => set_literal_to_rust_tokens(items),
        HirExpr::FrozenSet(items) => frozen_set_to_rust_tokens(items),
        HirExpr::SetComp {
            element,
            target,
            iter,
            condition,
        } => set_comp_to_rust_tokens(element, target, iter, condition),
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
    }
}

fn literal_to_rust_tokens(lit: &Literal) -> Result<proc_macro2::TokenStream> {
    match lit {
        Literal::Int(i) => Ok(quote! { #i }),
        Literal::Float(f) => Ok(quote! { #f }),
        Literal::String(s) => Ok(quote! { #s.to_string() }),
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
    matches!(expr, HirExpr::Call { func, args } if func == "len" && args.len() == 1)
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
        assert!(code.contains("if (r != 0) && ((r < 0) != (b < 0))"));
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
            target: "i".to_string(),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(5))],
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
            },
            target: Some("f".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "read".to_string(),
                args: vec![HirExpr::Var("f".to_string())],
            })],
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
            },
            target: None,
            body: vec![HirStmt::Expr(HirExpr::Literal(Literal::String(
                "critical".to_string(),
            )))],
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
}
