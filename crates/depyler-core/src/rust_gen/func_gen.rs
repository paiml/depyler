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
                    let bound: syn::Path =
                        syn::parse_str(b).unwrap_or_else(|_| parse_quote! { Clone });
                    quote! { #bound }
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

/// Generate function attributes (doc comments, panic-free, termination proofs)
#[inline]
pub(crate) fn codegen_function_attrs(
    docstring: &Option<String>,
    properties: &crate::hir::FunctionProperties,
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

    attrs
}

// ============================================================================
// DEPYLER-0141 Phase 2: Medium Complexity Helpers
// ============================================================================

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
    ctx.current_return_type = Some(func.ret_type.clone());
    // DEPYLER-0310: Set error type for raise statement wrapping
    ctx.current_error_type = error_type;

    for param in &func.params {
        ctx.declare_var(&param.name);
        // Store parameter type information for set/dict disambiguation
        ctx.var_types.insert(param.name.clone(), param.ty.clone());
    }

    // DEPYLER-0312 NOTE: analyze_mutable_vars is now called in impl RustCodeGen BEFORE
    // codegen_function_params, so ctx.mutable_vars is already populated here

    // DEPYLER-0271: Convert body, marking final statement for expression-based returns
    let body_len = func.body.len();
    let body_stmts: Vec<_> = func
        .body
        .iter()
        .enumerate()
        .map(|(i, stmt)| {
            // Mark final statement for idiomatic expression-based return
            ctx.is_final_statement = i == body_len - 1;
            stmt.to_rust_tokens(ctx)
        })
        .collect::<Result<Vec<_>>>()?;

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
    let param_ident = syn::Ident::new(&param.name, proc_macro2::Span::call_site());

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

    // Get the inferred parameter info
    if let Some(inferred) = lifetime_result.param_lifetimes.get(&param.name) {
        let rust_type = &inferred.rust_type;

        // Handle Union type placeholders
        let actual_rust_type =
            if let crate::type_mapper::RustType::Enum { name, variants: _ } = rust_type {
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
        // Fallback to original mapping
        let rust_type = ctx
            .annotation_aware_mapper
            .map_type_with_annotations(&param.ty, &func.annotations);
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
        HirExpr::Call { .. }
        | HirExpr::Var(_)
        | HirExpr::Literal(_)
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
        | HirExpr::FString { .. }
        | HirExpr::Yield { .. }
        | HirExpr::SortByKey { .. }
        | HirExpr::GeneratorExp { .. } => false,
    }
}

/// Check if the function's return expressions contain owned-returning string methods
fn function_returns_owned_string(func: &HirFunction) -> bool {
    // Check all return statements in the function body
    for stmt in &func.body {
        if let HirStmt::Return(Some(expr)) = stmt {
            if contains_owned_string_method(expr) {
                return true;
            }
        }
    }
    false
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

// ========== Phase 3b: Return Type Generation ==========

/// Generate return type with Result wrapper and lifetime handling
///
/// DEPYLER-0310: Now returns ErrorType (4th tuple element) for raise statement wrapping
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
    // Convert return type using annotation-aware mapping
    let mapped_ret_type = ctx
        .annotation_aware_mapper
        .map_return_type_with_annotations(&func.ret_type, &func.annotations);

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
    let error_type_str = if can_fail && !func.properties.error_types.is_empty() {
        // Use first error type or generic for mixed types
        if func.properties.error_types.len() == 1 {
            func.properties.error_types[0].clone()
        } else {
            "Box<dyn std::error::Error>".to_string()
        }
    } else {
        "Box<dyn std::error::Error>".to_string()
    };

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

    // Mark error types as needed for type generation
    if error_type_str.contains("ZeroDivisionError") {
        ctx.needs_zerodivisionerror = true;
    }
    if error_type_str.contains("IndexError") {
        ctx.needs_indexerror = true;
    }
    // DEPYLER-0295: Add ValueError support
    if error_type_str.contains("ValueError") {
        ctx.needs_valueerror = true;
    }

    let return_type = if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
        if can_fail {
            let error_type: syn::Type = syn::parse_str(&error_type_str)
                .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });
            quote! { -> Result<(), #error_type> }
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
            quote! { -> Result<#ty, #error_type> }
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
        // DEPYLER-0306 FIX: Use raw identifiers for function names that are Rust keywords
        let name = if is_rust_keyword(&self.name) {
            syn::Ident::new_raw(&self.name, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(&self.name, proc_macro2::Span::call_site())
        };

        // Perform generic type inference
        let mut generic_registry = crate::generic_inference::TypeVarRegistry::new();
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

        // Convert parameters using lifetime analysis results
        let params = codegen_function_params(self, &lifetime_result, ctx)?;

        // Generate return type with Result wrapper and lifetime handling
        let (return_type, rust_ret_type, can_fail, error_type) =
            codegen_return_type(self, &lifetime_result, ctx)?;

        // Process function body with proper scoping
        let body_stmts = codegen_function_body(self, can_fail, error_type, ctx)?;

        // Add documentation
        let attrs = codegen_function_attrs(&self.docstring, &self.properties);

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
