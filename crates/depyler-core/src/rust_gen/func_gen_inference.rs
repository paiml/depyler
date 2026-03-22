//! Advanced function codegen: nested functions, type inference helpers
//!
//! DEPYLER-COVERAGE-95: Extracted from func_gen.rs to reduce file size
//! and improve testability. Contains return type inference and nested function detection.

use crate::hir::*;
use crate::lifetime_analysis::LifetimeInference;
use crate::rust_gen::context::{CodeGenContext, RustCodeGen};
use crate::rust_gen::control_flow_analysis::stmt_always_returns;
use crate::rust_gen::func_gen::{
    build_var_type_env, codegen_function_body, codegen_function_params,
    collect_return_types_with_env, function_returns_owned_string,
    function_returns_string_concatenation, infer_expr_type_with_env, infer_param_type_from_body,
    infer_return_type_from_body_with_params, rewrite_adt_child_type,
};
use crate::rust_gen::func_gen_helpers::{
    codegen_function_attrs, codegen_generic_params, codegen_where_clause,
};
use crate::rust_gen::generator_gen::codegen_generator_function;
use crate::rust_gen::keywords::is_rust_keyword;
use crate::rust_gen::rust_type_to_syn;
use crate::rust_gen::type_gen::update_import_needs;
use anyhow::Result;
use quote::quote;
use syn::parse_quote;

/// GH-70: Detect if function returns a nested function/closure
/// Returns Some((nested_fn_name, params, ret_type)) if detected
/// Stores inferred params in ctx.nested_function_params for use during code generation
pub(crate) fn detect_returns_nested_function(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
) -> Option<(String, Vec<HirParam>, Type)> {
    // Look for pattern: function contains nested FunctionDef and ends with returning that name
    let mut nested_functions: std::collections::HashMap<String, (Vec<HirParam>, Type)> =
        std::collections::HashMap::new();

    // Collect nested function definitions with type inference
    for stmt in &func.body {
        if let HirStmt::FunctionDef { name, params, ret_type, body, .. } = stmt {
            // GH-70: Apply type inference to parameters
            // DEPYLER-0737: Also handle Optional(Unknown) for params with default=None
            let mut inferred_params = params.to_vec();
            for param in &mut inferred_params {
                if matches!(param.ty, Type::Unknown) {
                    // Try to infer from body usage
                    if let Some(inferred_ty) = infer_param_type_from_body(&param.name, body) {
                        param.ty = inferred_ty;
                    }
                } else if let Type::Optional(inner) = &param.ty {
                    // DEPYLER-0737: If param is Optional(Unknown), infer inner type and wrap
                    if matches!(inner.as_ref(), Type::Unknown) {
                        if let Some(inferred_ty) = infer_param_type_from_body(&param.name, body) {
                            param.ty = Type::Optional(Box::new(inferred_ty));
                        }
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
            ctx.nested_function_params.insert(name.clone(), inferred_params.clone());

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
pub(crate) fn function_returns_heterogeneous_io(func: &HirFunction) -> bool {
    let mut has_file_return = false;
    let mut has_stdio_return = false;

    collect_io_return_types(&func.body, &mut has_file_return, &mut has_stdio_return);

    has_file_return && has_stdio_return
}

/// DEPYLER-0626: Helper to collect IO return types from statements
pub(crate) fn collect_io_return_types(
    stmts: &[HirStmt],
    has_file: &mut bool,
    has_stdio: &mut bool,
) {
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
pub(crate) fn is_file_creating_return_expr(expr: &HirExpr) -> bool {
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
pub(crate) fn is_stdio_return_expr(expr: &HirExpr) -> bool {
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
    if let Some(result) = try_nested_function_return(func, ctx) {
        return Ok(result);
    }

    if let Some(result) = try_heterogeneous_io_return(func, ctx) {
        return Ok(result);
    }

    let effective_ret_type = compute_effective_return_type(func, ctx);

    let rust_ret_type = map_and_resolve_return_type(func, ctx, &effective_ret_type);

    update_import_needs(ctx, &rust_ret_type);

    let can_fail = func.properties.can_fail;
    let error_type_str = compute_error_type_string(func, ctx);

    let error_type = if can_fail {
        Some(if error_type_str.contains("Box<dyn") {
            crate::rust_gen::context::ErrorType::DynBox
        } else {
            crate::rust_gen::context::ErrorType::Concrete(error_type_str.clone())
        })
    } else {
        None
    };

    mark_error_type_needs(ctx, &error_type_str, &func.properties.error_types);

    let return_type = build_return_type_tokens(
        func,
        lifetime_result,
        ctx,
        &rust_ret_type,
        can_fail,
        &error_type_str,
    )?;

    Ok((return_type, rust_ret_type, can_fail, error_type))
}

fn try_nested_function_return(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
) -> Option<(
    proc_macro2::TokenStream,
    crate::type_mapper::RustType,
    bool,
    Option<crate::rust_gen::context::ErrorType>,
)> {
    let (_nested_name, params, nested_ret_type) = detect_returns_nested_function(func, ctx)?;

    let param_types: Vec<proc_macro2::TokenStream> =
        params.iter().map(|p| crate::rust_gen::type_tokens::hir_type_to_tokens(&p.ty)).collect();

    let ret_ty_tokens = crate::rust_gen::type_tokens::hir_type_to_tokens(&nested_ret_type);

    let fn_type = if params.is_empty() {
        quote! { -> Box<dyn Fn() -> #ret_ty_tokens> }
    } else {
        quote! { -> Box<dyn Fn(#(#param_types),*) -> #ret_ty_tokens> }
    };

    Some((
        fn_type.clone(),
        crate::type_mapper::RustType::Custom("BoxedFn".to_string()),
        false,
        None,
    ))
}

fn try_heterogeneous_io_return(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
) -> Option<(
    proc_macro2::TokenStream,
    crate::type_mapper::RustType,
    bool,
    Option<crate::rust_gen::context::ErrorType>,
)> {
    if !function_returns_heterogeneous_io(func) {
        return None;
    }

    ctx.function_returns_boxed_write = true;
    ctx.needs_io_write = true;

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

    Some((
        return_type,
        crate::type_mapper::RustType::Custom("BoxedWrite".to_string()),
        can_fail,
        error_type,
    ))
}

fn compute_effective_return_type(func: &HirFunction, ctx: &mut CodeGenContext) -> Type {
    let should_infer = matches!(func.ret_type, Type::Unknown | Type::None)
        || matches!(&func.ret_type, Type::Tuple(elems) if elems.is_empty() || elems.iter().any(|t| matches!(t, Type::Unknown)))
        || matches!(&func.ret_type, Type::List(elem) if matches!(**elem, Type::Unknown | Type::UnificationVar(_)))
        || matches!(&func.ret_type, Type::Custom(name) if name == "tuple");

    let effective_ret_type = if should_infer {
        infer_return_type_from_body_with_params(func, ctx).unwrap_or_else(|| func.ret_type.clone())
    } else {
        func.ret_type.clone()
    };

    if should_infer && effective_ret_type != func.ret_type {
        ctx.function_return_types.insert(func.name.clone(), effective_ret_type.clone());
    }

    let effective_ret_type = if !ctx.type_substitutions.is_empty() {
        crate::generic_inference::TypeVarRegistry::apply_substitutions(
            &effective_ret_type,
            &ctx.type_substitutions,
        )
    } else {
        effective_ret_type
    };

    rewrite_adt_child_type(&effective_ret_type, &ctx.adt_child_to_parent)
}

fn map_and_resolve_return_type(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
    effective_ret_type: &Type,
) -> crate::type_mapper::RustType {
    let mapped_ret_type = ctx
        .annotation_aware_mapper
        .map_return_type_with_annotations(effective_ret_type, &func.annotations);

    let rust_ret_type = if let crate::type_mapper::RustType::Enum { name, .. } = &mapped_ret_type {
        if name == "UnionType" {
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

    if matches!(func.ret_type, Type::String) && function_returns_owned_string(func) {
        crate::type_mapper::RustType::String
    } else {
        rust_ret_type
    }
}

fn compute_error_type_string(func: &HirFunction, ctx: &CodeGenContext) -> String {
    let mut error_type_str = if func.properties.can_fail && !func.properties.error_types.is_empty()
    {
        if func.properties.error_types.len() == 1 {
            func.properties.error_types[0].clone()
        } else {
            "Box<dyn std::error::Error>".to_string()
        }
    } else {
        "Box<dyn std::error::Error>".to_string()
    };

    error_type_str = map_python_error_to_rust(&error_type_str);

    if ctx.validator_functions.contains(&func.name) {
        error_type_str = "Box<dyn std::error::Error>".to_string();
    }

    error_type_str
}

fn map_python_error_to_rust(error_type_str: &str) -> String {
    match error_type_str {
        "OSError" | "IOError" | "FileNotFoundError" | "PermissionError" => {
            "std::io::Error".to_string()
        }
        "Exception"
        | "BaseException"
        | "ValueError"
        | "TypeError"
        | "KeyError"
        | "IndexError"
        | "RuntimeError"
        | "AttributeError"
        | "NotImplementedError"
        | "AssertionError"
        | "StopIteration"
        | "ZeroDivisionError"
        | "OverflowError"
        | "ArithmeticError" => "Box<dyn std::error::Error>".to_string(),
        _ => error_type_str.to_string(),
    }
}

fn mark_error_type_needs(
    ctx: &mut CodeGenContext,
    error_type_str: &str,
    property_error_types: &[String],
) {
    mark_single_error_type(ctx, error_type_str);

    for err_type in property_error_types {
        mark_single_error_type(ctx, err_type);
    }
}

fn mark_single_error_type(ctx: &mut CodeGenContext, err_type: &str) {
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

fn build_return_type_tokens(
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
    rust_ret_type: &crate::type_mapper::RustType,
    can_fail: bool,
    error_type_str: &str,
) -> Result<proc_macro2::TokenStream> {
    if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
        return build_unit_return_tokens(func, ctx, can_fail, error_type_str);
    }

    let mut ty = rust_type_to_syn(rust_ret_type)?;
    ty = apply_cow_or_lifetime(func, lifetime_result, ctx, rust_ret_type, ty)?;

    if can_fail {
        let error_type: syn::Type = syn::parse_str(error_type_str)
            .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });
        if func.name == "main" {
            Ok(quote! { -> Result<(), #error_type> })
        } else {
            Ok(quote! { -> Result<#ty, #error_type> })
        }
    } else if func.name == "main" && matches!(func.ret_type, Type::Int) {
        Ok(quote! {})
    } else {
        Ok(quote! { -> #ty })
    }
}

fn build_unit_return_tokens(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
    can_fail: bool,
    error_type_str: &str,
) -> Result<proc_macro2::TokenStream> {
    if !can_fail {
        return Ok(quote! {});
    }

    let error_type: syn::Type = syn::parse_str(error_type_str)
        .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });

    if let Some(inferred_type) = infer_return_type_from_body_with_params(func, ctx) {
        let inferred_rust_type = ctx
            .annotation_aware_mapper
            .map_return_type_with_annotations(&inferred_type, &func.annotations);

        if let Ok(ty) = rust_type_to_syn(&inferred_rust_type) {
            if func.name == "main" {
                return Ok(quote! { -> Result<(), #error_type> });
            }
            return Ok(quote! { -> Result<#ty, #error_type> });
        }
    }

    Ok(quote! { -> Result<(), #error_type> })
}

fn apply_cow_or_lifetime(
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
    rust_ret_type: &crate::type_mapper::RustType,
    mut ty: syn::Type,
) -> Result<syn::Type> {
    let returns_concatenation = matches!(func.ret_type, crate::hir::Type::String)
        && function_returns_string_concatenation(func);

    let uses_cow_return = !returns_concatenation && check_cow_return(func, lifetime_result);

    if uses_cow_return && !returns_concatenation {
        ctx.needs_cow = true;
        if let Some(ref return_lt) = lifetime_result.return_lifetime {
            let lt = syn::Lifetime::new(return_lt.as_str(), proc_macro2::Span::call_site());
            ty = parse_quote! { Cow<#lt, str> };
        } else {
            ty = parse_quote! { Cow<'static, str> };
        }
    } else {
        let returns_owned_string =
            matches!(func.ret_type, Type::String) && function_returns_owned_string(func);

        if let Some(ref return_lt) = lifetime_result.return_lifetime {
            if matches!(
                rust_ret_type,
                crate::type_mapper::RustType::Str { .. }
                    | crate::type_mapper::RustType::Reference { .. }
            ) && !returns_owned_string
            {
                let lt = syn::Lifetime::new(return_lt.as_str(), proc_macro2::Span::call_site());
                match rust_ret_type {
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
    }

    Ok(ty)
}

fn check_cow_return(
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
) -> bool {
    for param in &func.params {
        if let Some(strategy) = lifetime_result.borrowing_strategies.get(&param.name) {
            if matches!(strategy, crate::borrowing_context::BorrowingStrategy::UseCow { .. }) {
                if let Some(_usage) = lifetime_result.param_lifetimes.get(&param.name) {
                    if matches!(func.ret_type, crate::hir::Type::String) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

// ========== Phase 3c: Generator Implementation ==========
// (Moved to generator_gen.rs in v3.18.0 Phase 4)

/// DEPYLER-1181: Preload type annotations from HIR statements into var_types context
/// This ensures that inferred types from the constraint solver are available during
/// expression code generation. Without this, the "Neural Link" (DEPYLER-1180) propagation
/// of types to HIR annotations is ignored by the code generator.
///
/// The flow is:
/// 1. Constraint solver infers types (DEPYLER-1173)
/// 2. apply_substitutions writes types to HIR type_annotation fields (DEPYLER-1180)
/// 3. THIS function reads those annotations into ctx.var_types (DEPYLER-1181)
/// 4. Expression codegen can now access inferred types
pub(crate) fn preload_hir_type_annotations(body: &[HirStmt], ctx: &mut CodeGenContext) {
    for stmt in body {
        preload_stmt_type_annotations(stmt, ctx);
    }
}

/// DEPYLER-1181: Recursively extract type annotations from a single statement
fn preload_stmt_type_annotations(stmt: &HirStmt, ctx: &mut CodeGenContext) {
    match stmt {
        HirStmt::Assign {
            target: AssignTarget::Symbol(var_name),
            type_annotation: Some(ty),
            ..
        } => {
            // Only preload non-Unknown types to avoid overwriting better inferences
            if !matches!(ty, Type::Unknown) {
                // DEPYLER-99MODE-S9: Don't overwrite concrete parameter types with
                // incorrect HM-inferred types. E.g., `prefix: str` should not be
                // overwritten by HM inference of `prefix = prefix[:-1]` as List(Int).
                // Only overwrite if existing type is Unknown or if new type is the same kind.
                let should_overwrite = match ctx.var_types.get(var_name) {
                    None => true,
                    Some(Type::Unknown) => true,
                    Some(existing) => {
                        // Don't overwrite concrete types (String, Int, etc.) with
                        // structurally different inferred types from HM
                        std::mem::discriminant(existing) == std::mem::discriminant(ty)
                    }
                };
                if should_overwrite {
                    ctx.var_types.insert(var_name.clone(), ty.clone());
                }
            }
        }
        // Recursively handle nested statements
        HirStmt::If { then_body, else_body, .. } => {
            for s in then_body {
                preload_stmt_type_annotations(s, ctx);
            }
            if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    preload_stmt_type_annotations(s, ctx);
                }
            }
        }
        HirStmt::While { body, .. } | HirStmt::Block(body) => {
            for s in body {
                preload_stmt_type_annotations(s, ctx);
            }
        }
        HirStmt::For { body, .. } => {
            for s in body {
                preload_stmt_type_annotations(s, ctx);
            }
        }
        HirStmt::Try { body, handlers, orelse, finalbody } => {
            for s in body {
                preload_stmt_type_annotations(s, ctx);
            }
            for handler in handlers {
                for s in &handler.body {
                    preload_stmt_type_annotations(s, ctx);
                }
            }
            if let Some(orelse_stmts) = orelse {
                for s in orelse_stmts {
                    preload_stmt_type_annotations(s, ctx);
                }
            }
            if let Some(final_stmts) = finalbody {
                for s in final_stmts {
                    preload_stmt_type_annotations(s, ctx);
                }
            }
        }
        HirStmt::With { body, .. } => {
            for s in body {
                preload_stmt_type_annotations(s, ctx);
            }
        }
        HirStmt::FunctionDef { body, .. } => {
            // Note: Nested functions have their own scope, but we still preload
            // for consistency. The function's own codegen will clear and repopulate.
            for s in body {
                preload_stmt_type_annotations(s, ctx);
            }
        }
        _ => {}
    }
}

/// DEPYLER-0839/1075: Fix E0700 "hidden type captures lifetime" for impl Fn/Iterator returns.
/// When a function returns `impl Fn(...)` or `impl Iterator<...>` and captures reference parameters,
/// the return type must include a lifetime bound: `impl Fn(...) + 'a`.
#[allow(clippy::too_many_arguments)]
fn fixup_impl_trait_lifetimes(
    rust_ret_type: &crate::type_mapper::RustType,
    type_params: &[String],
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    generic_params: proc_macro2::TokenStream,
    return_type: proc_macro2::TokenStream,
    params: Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
    fn_name: &str,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    Vec<proc_macro2::TokenStream>,
) {
    let type_str = if let crate::type_mapper::RustType::Custom(ref s) = rust_ret_type {
        s.as_str()
    } else {
        return (generic_params, return_type, params);
    };

    let is_impl_trait = type_str.contains("impl Fn")
        || type_str.contains("impl Iterator")
        || type_str.contains("impl IntoIterator");
    if !is_impl_trait {
        return (generic_params, return_type, params);
    }

    let has_ref_params = ctx
        .function_param_borrows
        .get(fn_name)
        .map(|borrows| borrows.iter().any(|&b| b))
        .unwrap_or(false);
    if !has_ref_params {
        return (generic_params, return_type, params);
    }

    // DEPYLER-1080: Use single lifetime 'a for all reference params
    let mut lifetime_params_with_a = lifetime_result.lifetime_params.clone();
    if !lifetime_params_with_a.contains(&"'a".to_string()) {
        lifetime_params_with_a.push("'a".to_string());
    }
    lifetime_params_with_a.retain(|lt| lt == "'a");
    let new_generic_params = codegen_generic_params(type_params, &lifetime_params_with_a);

    // Modify return type to add + 'a bound
    let return_str = return_type.to_string();
    let modified_return = if return_str.contains("impl Fn")
        || return_str.contains("impl Iterator")
        || return_str.contains("impl IntoIterator")
    {
        let modified = format!("{} + 'a", return_str.trim());
        syn::parse_str::<proc_macro2::TokenStream>(&modified).unwrap_or(return_type.clone())
    } else {
        return_type.clone()
    };

    // DEPYLER-0839/1075: Add 'a lifetime to reference parameter types
    let modified_params: Vec<proc_macro2::TokenStream> = params
        .into_iter()
        .map(|p| {
            let param_str = p.to_string();
            let modified_param = param_str
                .replace("& 'b ", "& 'a ")
                .replace("& 'c ", "& 'a ")
                .replace("& 'd ", "& 'a ")
                .replace("& 'e ", "& 'a ");
            let modified_param =
                if modified_param.contains("& ") && !modified_param.contains("& '") {
                    modified_param
                        .replace("& mut ", "& 'a mut ")
                        .replace("& Vec", "& 'a Vec")
                        .replace("& str", "& 'a str")
                } else {
                    modified_param
                };
            syn::parse_str::<proc_macro2::TokenStream>(&modified_param).unwrap_or(p)
        })
        .collect();

    (new_generic_params, modified_return, modified_params)
}

/// Post-process body statements for argparser: generate Args struct, Commands enum,
/// inject precompute statements, and apply option field substitutions.
fn postprocess_argparser(
    ctx: &mut CodeGenContext,
    body_stmts: &mut Vec<proc_macro2::TokenStream>,
) {
    if !ctx.argparser_tracker.has_parsers() {
        return;
    }
    let Some(parser_info) = ctx.argparser_tracker.get_first_parser() else {
        return;
    };

    ctx.needs_clap = true;

    // Generate Commands enum if subcommands exist
    let commands_enum =
        crate::rust_gen::argparse_transform::generate_commands_enum(&ctx.argparser_tracker);
    if !commands_enum.is_empty() {
        ctx.generated_commands_enum = Some(commands_enum);
    }

    // Generate the Args struct definition
    let args_struct = crate::rust_gen::argparse_transform::generate_args_struct(
        parser_info,
        &ctx.argparser_tracker,
    );
    ctx.generated_args_struct = Some(args_struct);

    // Inject precompute statements for Option fields
    let precompute_stmts =
        crate::rust_gen::argparse_transform::generate_option_precompute(parser_info);
    if precompute_stmts.is_empty() {
        return;
    }

    // FIRST post-process body to replace args.<field>.is_some() with has_<field>
    let option_fields: Vec<String> = parser_info
        .arguments
        .iter()
        .filter(|arg| arg.rust_type().starts_with("Option<"))
        .map(|arg| arg.rust_field_name().to_string())
        .collect();

    if !option_fields.is_empty() {
        *body_stmts = body_stmts
            .iter()
            .map(|stmt| {
                let mut stmt_str = stmt.to_string();
                for field in &option_fields {
                    let pattern = format!("args . {} . is_some ()", field);
                    let replacement = format!("has_{}", field);
                    stmt_str = stmt_str.replace(&pattern, &replacement);
                    let pattern_none = format!("args . {} . is_none ()", field);
                    let replacement_none = format!("! has_{}", field);
                    stmt_str = stmt_str.replace(&pattern_none, &replacement_none);
                }
                syn::parse_str(&stmt_str).unwrap_or_else(|_| stmt.clone())
            })
            .collect();
    }

    // THEN inject precompute statements after replacement
    let insert_idx = body_stmts
        .iter()
        .position(|s| s.to_string().contains("Args :: parse"))
        .map(|i| i + 1)
        .unwrap_or(0);
    for (offset, stmt) in precompute_stmts.into_iter().enumerate() {
        body_stmts.insert(insert_idx + offset, stmt);
    }
}

/// CB-200 Batch 10: Infer and refine parameter types from body usage.
fn infer_and_refine_params(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
) -> Vec<crate::hir::HirParam> {
    let mut inferred_params = func.params.clone();
    for param in &mut inferred_params {
        if matches!(param.ty, Type::Unknown) {
            if let Some(inferred_ty) = infer_param_type_from_body(&param.name, &func.body) {
                param.ty = inferred_ty.clone();
                ctx.var_types.insert(param.name.clone(), inferred_ty);
            }
        } else if let Type::Optional(inner) = &param.ty {
            if matches!(inner.as_ref(), Type::Unknown) {
                if let Some(inferred_ty) = infer_param_type_from_body(&param.name, &func.body) {
                    let new_ty = Type::Optional(Box::new(inferred_ty));
                    param.ty = new_ty.clone();
                    ctx.var_types.insert(param.name.clone(), new_ty);
                }
            }
        }
    }
    for param in &mut inferred_params {
        if crate::container_element_inference::has_unknown_inner_type(&param.ty) {
            if let Some(refined) =
                crate::container_element_inference::infer_container_element_type(
                    &param.name,
                    &param.ty,
                    &func.body,
                )
            {
                param.ty = refined.clone();
                ctx.var_types.insert(param.name.clone(), refined);
            }
        }
    }
    inferred_params
}

/// CB-200 Batch 10: Track parameter borrowing, mutability, optionality, and special patterns.
fn track_param_metadata(
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) {
    // Borrow tracking
    let param_borrows: Vec<bool> = func
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
    ctx.ref_params.clear();
    for (p, &is_borrowed) in func.params.iter().zip(param_borrows.iter()) {
        if is_borrowed {
            ctx.ref_params.insert(p.name.clone());
        }
    }
    ctx.function_param_borrows.insert(func.name.clone(), param_borrows.clone());

    // Mutability tracking
    let param_muts: Vec<bool> = func
        .params
        .iter()
        .map(|p| {
            let is_mutated = ctx.mutable_vars.contains(&p.name);
            let should_borrow = lifetime_result
                .param_lifetimes
                .get(&p.name)
                .map(|inf| inf.should_borrow)
                .unwrap_or(false);
            is_mutated && should_borrow
        })
        .collect();
    ctx.mut_ref_params.clear();
    for (p, &needs_mut) in func.params.iter().zip(param_muts.iter()) {
        if needs_mut {
            ctx.mut_ref_params.insert(p.name.clone());
        }
    }
    ctx.function_param_muts.insert(func.name.clone(), param_muts);

    // Optionality tracking
    let param_optionals: Vec<bool> = func
        .params
        .iter()
        .map(|p| {
            let type_is_optional = matches!(p.ty, Type::Optional(_));
            let default_is_none = matches!(p.default, Some(HirExpr::Literal(Literal::None)));
            type_is_optional || default_is_none
        })
        .collect();
    ctx.function_param_optionals.insert(func.name.clone(), param_optionals);

    // Vararg tracking
    if func.params.iter().any(|p| p.is_vararg) {
        ctx.vararg_functions.insert(func.name.clone());
    }

    // Option dict/option param tracking
    for param in &func.params {
        let is_dict = matches!(&param.ty, Type::Dict { .. })
            || matches!(&param.ty, Type::Custom(name) if name == "dict");
        let has_none_default = matches!(&param.default, Some(HirExpr::Literal(Literal::None)));
        let is_optional_dict = matches!(
            &param.ty,
            Type::Optional(inner) if matches!(inner.as_ref(), Type::Dict { .. })
        );
        if (is_dict && has_none_default) || is_optional_dict {
            ctx.mut_option_dict_params.insert(param.name.clone());
        }
        let is_optional = matches!(&param.ty, Type::Optional(_))
            || matches!(&param.ty, Type::Union(types) if types.iter().any(|t| matches!(t, Type::None)));
        let inferred_needs_mut = lifetime_result
            .param_lifetimes
            .get(&param.name)
            .map(|ip| ip.needs_mut)
            .unwrap_or(false);
        if (is_optional || has_none_default) && inferred_needs_mut {
            ctx.mut_option_params.insert(param.name.clone());
        }
    }
}

/// CB-200 Batch 10: Post-process function body (empty-body, unit-return, closure boxing, argparser).
fn postprocess_function_body(
    func: &HirFunction,
    body_stmts: &mut Vec<proc_macro2::TokenStream>,
    rust_ret_type: &crate::type_mapper::RustType,
    can_fail: bool,
    subcommand_info: Option<(String, Vec<String>)>,
    ctx: &mut CodeGenContext,
) {
    // Handle empty body with non-unit return
    {
        use quote::quote;
        let body_is_empty = body_stmts.iter().all(|stmt| stmt.is_empty());
        let is_non_unit_return = !matches!(rust_ret_type, crate::type_mapper::RustType::Unit);
        if body_is_empty && is_non_unit_return {
            body_stmts.push(quote! { unimplemented!() });
        }
    }

    // Discard trailing expression for unit return
    if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
        if let Some(last) = body_stmts.last_mut() {
            let last_str = last.to_string();
            if !last_str.is_empty()
                && !last_str.trim_end().ends_with(';')
                && !last_str.trim_end().ends_with('}')
            {
                use quote::quote;
                let tokens = std::mem::take(last);
                *last = quote! { let _ = #tokens; };
            }
        }
    }

    // Wrap returned closure in Box::new()
    if let Some((nested_name, _, _)) = detect_returns_nested_function(func, ctx) {
        if let Some(last_stmt) = body_stmts.last_mut() {
            use quote::quote;
            let nested_ident = syn::Ident::new(&nested_name, proc_macro2::Span::call_site());
            let last_stmt_str = last_stmt.to_string();
            if last_stmt_str.trim() == nested_name {
                *last_stmt = quote! { Box::new(#nested_ident) };
            }
        }
    }

    ctx.current_subcommand_fields = None;
    postprocess_argparser(ctx, body_stmts);

    // Wrap with subcommand pattern matching
    if let Some((variant_name, fields)) = subcommand_info {
        if !ctx.in_cmd_handler {
            if let Some(args_param) = func.params.first() {
                let args_param_name = args_param.name.as_ref();
                *body_stmts =
                    crate::rust_gen::argparse_transform::wrap_body_with_subcommand_pattern(
                        std::mem::take(body_stmts),
                        &variant_name,
                        &fields,
                        args_param_name,
                    );
            }
        }
    }

    // Add Ok(()) for Result-returning functions
    if can_fail {
        let needs_ok = func.body.last().is_none_or(|stmt| !stmt_always_returns(stmt));
        if needs_ok && matches!(func.ret_type, Type::None | Type::Unknown) {
            body_stmts.push(parse_quote! { Ok(()) });
        }
    }
}

impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        ctx.var_types.clear();
        ctx.type_substitutions.clear();
        ctx.slice_params.clear();

        let name = if is_rust_keyword(&self.name) {
            syn::Ident::new_raw(&self.name, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(&self.name, proc_macro2::Span::call_site())
        };

        ctx.function_return_types.insert(self.name.clone(), self.ret_type.clone());

        let param_defaults: Vec<Option<crate::hir::HirExpr>> =
            self.params.iter().map(|p| p.default.clone()).collect();
        ctx.function_param_defaults.insert(self.name.clone(), param_defaults);

        // Generic type inference and substitutions
        let mut generic_registry = crate::generic_inference::TypeVarRegistry::new();
        let type_substitutions = generic_registry.infer_type_substitutions(self)?;
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
            ctx.type_substitutions = type_substitutions;
        }

        // Infer and refine parameter types
        let inferred_params = infer_and_refine_params(self, ctx);
        let inferred_self = HirFunction { params: inferred_params, ..self.clone() };
        let type_params = generic_registry.infer_function_generics(&inferred_self)?;

        // Lifetime analysis
        let mut lifetime_inference = LifetimeInference::new();
        let lifetime_result = lifetime_inference
            .apply_elision_rules(self, ctx.type_mapper)
            .unwrap_or_else(|| lifetime_inference.analyze_function(self, ctx.type_mapper));

        let generic_params = codegen_generic_params(&type_params, &lifetime_result.lifetime_params);
        let where_clause = codegen_where_clause(&lifetime_result.lifetime_bounds);

        crate::rust_gen::analyze_mutable_vars(&self.body, ctx, &self.params);
        let params = codegen_function_params(self, &lifetime_result, ctx)?;

        // Track all parameter metadata (borrows, muts, optionals, etc.)
        track_param_metadata(self, &lifetime_result, ctx);

        // Return type generation
        let (return_type, rust_ret_type, can_fail, error_type) =
            codegen_return_type(self, &lifetime_result, ctx)?;

        let (generic_params, return_type, params) = fixup_impl_trait_lifetimes(
            &rust_ret_type,
            &type_params,
            &lifetime_result,
            generic_params,
            return_type,
            params,
            ctx,
            &self.name,
        );

        // Subcommand and argparser setup
        let subcommand_info = if ctx.argparser_tracker.has_subcommands() {
            crate::rust_gen::argparse_transform::analyze_subcommand_field_access(
                self,
                &ctx.argparser_tracker,
            )
        } else {
            None
        };
        if let Some((_, ref fields)) = subcommand_info {
            ctx.current_subcommand_fields = Some(fields.iter().cloned().collect());
        }
        crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(
            self,
            &mut ctx.argparser_tracker,
        );
        if ctx.argparser_tracker.has_parsers() {
            if let Some(parser_info) = ctx.argparser_tracker.get_first_parser() {
                for arg in &parser_info.arguments {
                    if arg.rust_type().starts_with("Option<") {
                        ctx.precomputed_option_fields.insert(arg.rust_field_name().to_string());
                    }
                }
            }
        }

        // Generate body
        let was_main = ctx.is_main_function;
        ctx.is_main_function = self.name == "main";

        for param in &self.params {
            if !matches!(param.ty, Type::Unknown) {
                ctx.var_types.insert(param.name.clone(), param.ty.clone());
            }
        }
        preload_hir_type_annotations(&self.body, ctx);

        let mut body_stmts = codegen_function_body(self, can_fail, error_type, ctx)?;
        ctx.is_main_function = was_main;

        // Post-process body
        postprocess_function_body(
            self,
            &mut body_stmts,
            &rust_ret_type,
            can_fail,
            subcommand_info,
            ctx,
        );

        // Generate function attributes and tokens
        let attrs = codegen_function_attrs(
            &self.docstring,
            &self.properties,
            &self.annotations.custom_attributes,
        );

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
            let nasa_mode = ctx.type_mapper.nasa_mode;
            if nasa_mode {
                quote! {
                    #(#attrs)*
                    pub fn #name #generic_params(#(#params),*) #return_type #where_clause {
                        #(#body_stmts)*
                    }
                }
            } else if self.name == "main" {
                ctx.needs_tokio = true;
                quote! {
                    #(#attrs)*
                    #[tokio::main]
                    pub async fn #name #generic_params(#(#params),*) #return_type #where_clause {
                        #(#body_stmts)*
                    }
                }
            } else {
                quote! {
                    #(#attrs)*
                    pub async fn #name #generic_params(#(#params),*) #return_type #where_clause {
                        #(#body_stmts)*
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    // === is_file_creating_return_expr tests ===

    #[test]
    fn test_is_file_creating_open_call() {
        let expr = HirExpr::Call {
            func: "open".to_string(),
            args: vec![HirExpr::Literal(Literal::String("test.txt".to_string()))],
            kwargs: vec![],
        };
        assert!(is_file_creating_return_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_other_call() {
        let expr = HirExpr::Call { func: "read".to_string(), args: vec![], kwargs: vec![] };
        assert!(!is_file_creating_return_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_file_create_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "create".to_string(),
            args: vec![HirExpr::Literal(Literal::String("out.txt".to_string()))],
            kwargs: vec![],
        };
        assert!(is_file_creating_return_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_file_open_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "open".to_string(),
            args: vec![HirExpr::Literal(Literal::String("in.txt".to_string()))],
            kwargs: vec![],
        };
        assert!(is_file_creating_return_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_attribute_file() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("io".to_string())),
                attr: "File".to_string(),
            }),
            method: "create".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_file_creating_return_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_non_file_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("List".to_string())),
            method: "create".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_file_creating_return_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_non_create_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("File".to_string())),
            method: "read".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_file_creating_return_expr(&expr));
    }

    #[test]
    fn test_is_file_creating_var_expr() {
        let expr = HirExpr::Var("f".to_string());
        assert!(!is_file_creating_return_expr(&expr));
    }

    // === is_stdio_return_expr tests ===

    #[test]
    fn test_is_stdio_stdout() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "stdout".to_string(),
        };
        assert!(is_stdio_return_expr(&expr));
    }

    #[test]
    fn test_is_stdio_stderr() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "stderr".to_string(),
        };
        assert!(is_stdio_return_expr(&expr));
    }

    #[test]
    fn test_is_stdio_not_sys() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("os".to_string())),
            attr: "stdout".to_string(),
        };
        assert!(!is_stdio_return_expr(&expr));
    }

    #[test]
    fn test_is_stdio_not_stdout() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "path".to_string(),
        };
        assert!(!is_stdio_return_expr(&expr));
    }

    #[test]
    fn test_is_stdio_plain_var() {
        let expr = HirExpr::Var("stdout".to_string());
        assert!(!is_stdio_return_expr(&expr));
    }

    // === collect_io_return_types tests ===

    #[test]
    fn test_collect_io_return_types_empty() {
        let mut has_file = false;
        let mut has_stdio = false;
        collect_io_return_types(&[], &mut has_file, &mut has_stdio);
        assert!(!has_file);
        assert!(!has_stdio);
    }

    #[test]
    fn test_collect_io_return_file() {
        let stmts = vec![HirStmt::Return(Some(HirExpr::Call {
            func: "open".to_string(),
            args: vec![HirExpr::Literal(Literal::String("f.txt".to_string()))],
            kwargs: vec![],
        }))];
        let mut has_file = false;
        let mut has_stdio = false;
        collect_io_return_types(&stmts, &mut has_file, &mut has_stdio);
        assert!(has_file);
        assert!(!has_stdio);
    }

    #[test]
    fn test_collect_io_return_stdio() {
        let stmts = vec![HirStmt::Return(Some(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("sys".to_string())),
            attr: "stdout".to_string(),
        }))];
        let mut has_file = false;
        let mut has_stdio = false;
        collect_io_return_types(&stmts, &mut has_file, &mut has_stdio);
        assert!(!has_file);
        assert!(has_stdio);
    }

    #[test]
    fn test_collect_io_return_types_in_if_branch() {
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Var("flag".to_string()),
            then_body: vec![HirStmt::Return(Some(HirExpr::Call {
                func: "open".to_string(),
                args: vec![],
                kwargs: vec![],
            }))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("sys".to_string())),
                attr: "stderr".to_string(),
            }))]),
        }];
        let mut has_file = false;
        let mut has_stdio = false;
        collect_io_return_types(&stmts, &mut has_file, &mut has_stdio);
        assert!(has_file);
        assert!(has_stdio);
    }

    #[test]
    fn test_collect_io_return_types_in_loop() {
        let stmts = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Return(Some(HirExpr::Call {
                func: "open".to_string(),
                args: vec![],
                kwargs: vec![],
            }))],
        }];
        let mut has_file = false;
        let mut has_stdio = false;
        collect_io_return_types(&stmts, &mut has_file, &mut has_stdio);
        assert!(has_file);
    }

    #[test]
    fn test_collect_io_return_types_in_for() {
        let stmts = vec![HirStmt::For {
            target: AssignTarget::Symbol("x".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![HirStmt::Return(Some(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("sys".to_string())),
                attr: "stdout".to_string(),
            }))],
        }];
        let mut has_file = false;
        let mut has_stdio = false;
        collect_io_return_types(&stmts, &mut has_file, &mut has_stdio);
        assert!(has_stdio);
    }

    #[test]
    fn test_collect_io_return_none() {
        let stmts = vec![HirStmt::Return(None)];
        let mut has_file = false;
        let mut has_stdio = false;
        collect_io_return_types(&stmts, &mut has_file, &mut has_stdio);
        assert!(!has_file);
        assert!(!has_stdio);
    }

    // === function_returns_heterogeneous_io tests ===

    #[test]
    fn test_heterogeneous_io_both_types() {
        let func = HirFunction {
            name: "get_output".to_string(),
            params: smallvec::smallvec![HirParam::new("use_file".to_string(), Type::Bool)],
            ret_type: Type::Unknown,
            body: vec![HirStmt::If {
                condition: HirExpr::Var("use_file".to_string()),
                then_body: vec![HirStmt::Return(Some(HirExpr::Call {
                    func: "open".to_string(),
                    args: vec![],
                    kwargs: vec![],
                }))],
                else_body: Some(vec![HirStmt::Return(Some(HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("sys".to_string())),
                    attr: "stdout".to_string(),
                }))]),
            }],
            properties: FunctionProperties::default(),
            annotations: depyler_annotations::TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(function_returns_heterogeneous_io(&func));
    }

    #[test]
    fn test_heterogeneous_io_only_file() {
        let func = HirFunction {
            name: "get_file".to_string(),
            params: smallvec::smallvec![],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Return(Some(HirExpr::Call {
                func: "open".to_string(),
                args: vec![],
                kwargs: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: depyler_annotations::TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!function_returns_heterogeneous_io(&func));
    }

    #[test]
    fn test_heterogeneous_io_neither() {
        let func = HirFunction {
            name: "get_val".to_string(),
            params: smallvec::smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
            properties: FunctionProperties::default(),
            annotations: depyler_annotations::TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!function_returns_heterogeneous_io(&func));
    }

    // === preload_hir_type_annotations tests ===

    #[test]
    fn test_preload_simple_assign() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: Some(Type::Int),
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        assert_eq!(ctx.var_types.get("x"), Some(&Type::Int));
    }

    #[test]
    fn test_preload_skips_unknown_type() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: Some(Type::Unknown),
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        assert!(!ctx.var_types.contains_key("x"));
    }

    #[test]
    fn test_preload_no_annotation() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        assert!(!ctx.var_types.contains_key("x"));
    }

    #[test]
    fn test_preload_in_if_branches() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("a".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: Some(Type::Int),
            }],
            else_body: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("b".to_string()),
                value: HirExpr::Literal(Literal::String("hi".to_string())),
                type_annotation: Some(Type::String),
            }]),
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        assert_eq!(ctx.var_types.get("a"), Some(&Type::Int));
        assert_eq!(ctx.var_types.get("b"), Some(&Type::String));
    }

    #[test]
    fn test_preload_in_while_loop() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("counter".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: Some(Type::Int),
            }],
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        assert_eq!(ctx.var_types.get("counter"), Some(&Type::Int));
    }

    #[test]
    fn test_preload_in_for_loop() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("total".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: Some(Type::Float),
            }],
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        assert_eq!(ctx.var_types.get("total"), Some(&Type::Float));
    }

    #[test]
    fn test_preload_in_try_block() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::Try {
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("result".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: Some(Type::Int),
            }],
            handlers: vec![ExceptHandler {
                exception_type: Some("ValueError".to_string()),
                name: None,
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("err_msg".to_string()),
                    value: HirExpr::Literal(Literal::String("error".to_string())),
                    type_annotation: Some(Type::String),
                }],
            }],
            orelse: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("ok".to_string()),
                value: HirExpr::Literal(Literal::Bool(true)),
                type_annotation: Some(Type::Bool),
            }]),
            finalbody: Some(vec![HirStmt::Assign {
                target: AssignTarget::Symbol("done".to_string()),
                value: HirExpr::Literal(Literal::Bool(true)),
                type_annotation: Some(Type::Bool),
            }]),
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        assert_eq!(ctx.var_types.get("result"), Some(&Type::Int));
        assert_eq!(ctx.var_types.get("err_msg"), Some(&Type::String));
        assert_eq!(ctx.var_types.get("ok"), Some(&Type::Bool));
        assert_eq!(ctx.var_types.get("done"), Some(&Type::Bool));
    }

    #[test]
    fn test_preload_in_with_block() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::With {
            context: HirExpr::Call { func: "open".to_string(), args: vec![], kwargs: vec![] },
            target: Some("f".to_string()),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("data".to_string()),
                value: HirExpr::Literal(Literal::String(String::new())),
                type_annotation: Some(Type::String),
            }],
            is_async: false,
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        assert_eq!(ctx.var_types.get("data"), Some(&Type::String));
    }

    #[test]
    fn test_preload_in_nested_function() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec::smallvec![]),
            ret_type: Type::None,
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("local".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: Some(Type::Int),
            }],
            docstring: None,
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        assert_eq!(ctx.var_types.get("local"), Some(&Type::Int));
    }

    #[test]
    fn test_preload_in_block() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::Block(vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: Some(Type::Int),
        }])];
        preload_hir_type_annotations(&body, &mut ctx);
        assert_eq!(ctx.var_types.get("x"), Some(&Type::Int));
    }

    #[test]
    fn test_preload_empty_body() {
        let mut ctx = CodeGenContext::default();
        preload_hir_type_annotations(&[], &mut ctx);
        assert!(ctx.var_types.is_empty());
    }

    #[test]
    fn test_preload_tuple_target_ignored() {
        let mut ctx = CodeGenContext::default();
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Tuple(vec![
                AssignTarget::Symbol("a".to_string()),
                AssignTarget::Symbol("b".to_string()),
            ]),
            value: HirExpr::Tuple(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ]),
            type_annotation: Some(Type::Int),
        }];
        preload_hir_type_annotations(&body, &mut ctx);
        // Tuple targets don't match Symbol pattern
        assert!(!ctx.var_types.contains_key("a"));
    }

    // === Transpile-based integration tests ===

    fn transpile(python_code: &str) -> String {
        use crate::ast_bridge::AstBridge;
        use crate::rust_gen::generate_rust_file;
        use crate::type_mapper::TypeMapper;
        use rustpython_parser::{parse, Mode};

        let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
        let (module, _) =
            AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
        let tm = TypeMapper::default();
        let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
        result
    }

    #[test]
    fn test_transpile_nested_function_return() {
        let code = r"
def make_adder(n: int) -> int:
    def adder(x: int) -> int:
        return n + x
    return adder
";
        let rust = transpile(code);
        assert!(rust.contains("fn make_adder"));
    }

    #[test]
    fn test_transpile_function_with_type_annotations() {
        let code = r#"
def greet(name: str) -> str:
    result: str = "Hello, " + name
    return result
"#;
        let rust = transpile(code);
        assert!(rust.contains("fn greet"));
        assert!(rust.contains("String") || rust.contains("str"));
    }

    #[test]
    fn test_transpile_function_return_type_inferred() {
        let code = r"
def double(x: int) -> int:
    return x * 2
";
        let rust = transpile(code);
        assert!(rust.contains("fn double"));
        assert!(rust.contains("i64") || rust.contains("i32"));
    }

    #[test]
    fn test_transpile_async_function() {
        let code = r"
async def fetch_data(url: str) -> str:
    return url
";
        let rust = transpile(code);
        // The transpiler currently generates synchronous functions for async Python defs.
        // Verify the function is emitted correctly (async support is not yet implemented).
        assert!(rust.contains("fn fetch_data"));
        assert!(rust.contains("String") || rust.contains("str"));
    }

    // === detect_returns_nested_function tests ===

    fn make_function(
        name: &str,
        params: Vec<HirParam>,
        ret_type: Type,
        body: Vec<HirStmt>,
    ) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: params.into_iter().collect(),
            ret_type,
            body,
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        }
    }

    #[test]
    fn test_detect_nested_no_nested_functions() {
        let mut ctx = CodeGenContext::default();
        let func = make_function(
            "simple",
            vec![],
            Type::Int,
            vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        );
        let result = detect_returns_nested_function(&func, &mut ctx);
        assert!(result.is_none());
    }

    #[test]
    fn test_detect_nested_with_explicit_return() {
        let mut ctx = CodeGenContext::default();
        let func = make_function(
            "make_adder",
            vec![HirParam {
                name: "n".to_string(),
                ty: Type::Int,
                default: None,
                is_vararg: false,
            }],
            Type::Unknown,
            vec![
                HirStmt::FunctionDef {
                    name: "adder".to_string(),
                    params: Box::new(smallvec::smallvec![HirParam {
                        name: "x".to_string(),
                        ty: Type::Int,
                        default: None,
                        is_vararg: false,
                    }]),
                    ret_type: Type::Int,
                    body: vec![HirStmt::Return(Some(HirExpr::Binary {
                        left: Box::new(HirExpr::Var("n".to_string())),
                        op: BinOp::Add,
                        right: Box::new(HirExpr::Var("x".to_string())),
                    }))],
                    docstring: None,
                },
                HirStmt::Return(Some(HirExpr::Var("adder".to_string()))),
            ],
        );
        let result = detect_returns_nested_function(&func, &mut ctx);
        assert!(result.is_some());
        let (name, params, ret_type) = result.unwrap();
        assert_eq!(name, "adder");
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].name, "x");
        assert!(matches!(ret_type, Type::Int));
    }

    #[test]
    fn test_detect_nested_with_implicit_return() {
        let mut ctx = CodeGenContext::default();
        let func = make_function(
            "make_fn",
            vec![],
            Type::Unknown,
            vec![
                HirStmt::FunctionDef {
                    name: "inner".to_string(),
                    params: Box::new(smallvec::smallvec![]),
                    ret_type: Type::String,
                    body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String(
                        "hello".to_string(),
                    ))))],
                    docstring: None,
                },
                HirStmt::Expr(HirExpr::Var("inner".to_string())),
            ],
        );
        let result = detect_returns_nested_function(&func, &mut ctx);
        assert!(result.is_some());
        let (name, _, ret_type) = result.unwrap();
        assert_eq!(name, "inner");
        assert!(matches!(ret_type, Type::String));
    }

    #[test]
    fn test_detect_nested_returns_wrong_name() {
        let mut ctx = CodeGenContext::default();
        let func = make_function(
            "make_fn",
            vec![],
            Type::Unknown,
            vec![
                HirStmt::FunctionDef {
                    name: "inner".to_string(),
                    params: Box::new(smallvec::smallvec![]),
                    ret_type: Type::Int,
                    body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
                    docstring: None,
                },
                HirStmt::Return(Some(HirExpr::Var("other_fn".to_string()))),
            ],
        );
        let result = detect_returns_nested_function(&func, &mut ctx);
        assert!(result.is_none());
    }

    #[test]
    fn test_detect_nested_stores_params_in_ctx() {
        let mut ctx = CodeGenContext::default();
        let func = make_function(
            "factory",
            vec![],
            Type::Unknown,
            vec![
                HirStmt::FunctionDef {
                    name: "product".to_string(),
                    params: Box::new(smallvec::smallvec![
                        HirParam {
                            name: "a".to_string(),
                            ty: Type::Int,
                            default: None,
                            is_vararg: false,
                        },
                        HirParam {
                            name: "b".to_string(),
                            ty: Type::Float,
                            default: None,
                            is_vararg: false,
                        },
                    ]),
                    ret_type: Type::Float,
                    body: vec![],
                    docstring: None,
                },
                HirStmt::Return(Some(HirExpr::Var("product".to_string()))),
            ],
        );
        let _ = detect_returns_nested_function(&func, &mut ctx);
        assert!(ctx.nested_function_params.contains_key("product"));
        let params = ctx.nested_function_params.get("product").unwrap();
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_detect_nested_unknown_param_inference() {
        let mut ctx = CodeGenContext::default();
        let func = make_function(
            "make_fn",
            vec![],
            Type::Unknown,
            vec![
                HirStmt::FunctionDef {
                    name: "inner".to_string(),
                    params: Box::new(smallvec::smallvec![HirParam {
                        name: "x".to_string(),
                        ty: Type::Unknown,
                        default: None,
                        is_vararg: false,
                    }]),
                    ret_type: Type::Unknown,
                    body: vec![HirStmt::Return(Some(HirExpr::Binary {
                        left: Box::new(HirExpr::Var("x".to_string())),
                        op: BinOp::Add,
                        right: Box::new(HirExpr::Literal(Literal::Int(1))),
                    }))],
                    docstring: None,
                },
                HirStmt::Return(Some(HirExpr::Var("inner".to_string()))),
            ],
        );
        let result = detect_returns_nested_function(&func, &mut ctx);
        assert!(result.is_some());
    }

    #[test]
    fn test_detect_nested_optional_unknown_param() {
        let mut ctx = CodeGenContext::default();
        let func = make_function(
            "make_fn",
            vec![],
            Type::Unknown,
            vec![
                HirStmt::FunctionDef {
                    name: "inner".to_string(),
                    params: Box::new(smallvec::smallvec![HirParam {
                        name: "val".to_string(),
                        ty: Type::Optional(Box::new(Type::Unknown)),
                        default: None,
                        is_vararg: false,
                    }]),
                    ret_type: Type::Int,
                    body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0))))],
                    docstring: None,
                },
                HirStmt::Return(Some(HirExpr::Var("inner".to_string()))),
            ],
        );
        let result = detect_returns_nested_function(&func, &mut ctx);
        assert!(result.is_some());
    }

    #[test]
    fn test_detect_nested_empty_body() {
        let mut ctx = CodeGenContext::default();
        let func = make_function("empty", vec![], Type::Unknown, vec![]);
        let result = detect_returns_nested_function(&func, &mut ctx);
        assert!(result.is_none());
    }

    #[test]
    fn test_detect_nested_non_var_return() {
        let mut ctx = CodeGenContext::default();
        let func = make_function(
            "make_fn",
            vec![],
            Type::Unknown,
            vec![
                HirStmt::FunctionDef {
                    name: "inner".to_string(),
                    params: Box::new(smallvec::smallvec![]),
                    ret_type: Type::Int,
                    body: vec![],
                    docstring: None,
                },
                HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42)))),
            ],
        );
        let result = detect_returns_nested_function(&func, &mut ctx);
        assert!(result.is_none()); // Returns literal, not nested fn name
    }

    // === Transpile-based tests for nested function patterns ===

    #[test]
    fn test_transpile_closure_returning_function() {
        let code = r"
def make_multiplier(factor: int):
    def multiply(x: int) -> int:
        return x * factor
    return multiply
";
        let rust = transpile(code);
        assert!(rust.contains("fn make_multiplier"));
    }

    #[test]
    fn test_transpile_function_with_default_params() {
        let code = r#"
def greet(name: str, greeting: str = "Hello") -> str:
    return greeting + " " + name
"#;
        let rust = transpile(code);
        assert!(rust.contains("fn greet"));
    }

    #[test]
    fn test_transpile_function_returning_bool() {
        let code = r"
def is_even(n: int) -> bool:
    return n % 2 == 0
";
        let rust = transpile(code);
        assert!(rust.contains("fn is_even"));
        assert!(rust.contains("bool"));
    }

    #[test]
    fn test_transpile_function_returning_list() {
        let code = r"
def make_list(n: int) -> list:
    result = []
    for i in range(n):
        result.append(i)
    return result
";
        let rust = transpile(code);
        assert!(rust.contains("fn make_list"));
        assert!(rust.contains("Vec"));
    }

    #[test]
    fn test_transpile_function_returning_optional() {
        let code = r"
def find_item(items: list, target: int):
    for item in items:
        if item == target:
            return item
    return None
";
        let rust = transpile(code);
        assert!(rust.contains("fn find_item"));
        assert!(rust.contains("Option") || rust.contains("None"));
    }

    #[test]
    fn test_transpile_function_with_multiple_returns() {
        let code = r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
        let rust = transpile(code);
        assert!(rust.contains("fn classify"));
        assert!(rust.contains("positive"));
        assert!(rust.contains("negative"));
        assert!(rust.contains("zero"));
    }

    // === Session 9 Batch 6: Targeted coverage ===

    #[test]
    fn test_s9b6_infer_dict_return() {
        let code = r#"
def make_dict() -> dict:
    d = {}
    d["a"] = 1
    d["b"] = 2
    return d
"#;
        let rust = transpile(code);
        assert!(rust.contains("fn make_dict"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_tuple_return() {
        let code = r"
def pair(a: int, b: int) -> tuple:
    return a, b
";
        let rust = transpile(code);
        assert!(rust.contains("fn pair"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_set_return() {
        let code = r"
def unique_chars(s: str) -> set:
    result = set()
    for c in s:
        result.add(c)
    return result
";
        let rust = transpile(code);
        assert!(rust.contains("fn unique_chars"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_binary_op() {
        let code = r"
def compute(a: int, b: int):
    return a * b + a - b
";
        let rust = transpile(code);
        assert!(rust.contains("fn compute"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_comparison() {
        let code = r"
def check(x: int, y: int):
    return x > y
";
        let rust = transpile(code);
        assert!(rust.contains("fn check"), "output: {}", rust);
        assert!(rust.contains("bool"), "should infer bool: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_method_call() {
        let code = r"
def upper(s: str):
    return s.upper()
";
        let rust = transpile(code);
        assert!(rust.contains("fn upper"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_len() {
        let code = r"
def count(items: list):
    return len(items)
";
        let rust = transpile(code);
        assert!(rust.contains("fn count"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_sum() {
        let code = r"
def total(nums: list):
    return sum(nums)
";
        let rust = transpile(code);
        assert!(rust.contains("fn total"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_min_max() {
        let code = r"
def largest(nums: list):
    return max(nums)
";
        let rust = transpile(code);
        assert!(rust.contains("fn largest"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_str_join() {
        let code = r#"
def combine(parts: list):
    return ",".join(parts)
"#;
        let rust = transpile(code);
        assert!(rust.contains("fn combine"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_str_split() {
        let code = r#"
def split_csv(line: str):
    return line.split(",")
"#;
        let rust = transpile(code);
        assert!(rust.contains("fn split_csv"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_void_function() {
        let code = r"
def log(msg: str):
    print(msg)
";
        let rust = transpile(code);
        assert!(rust.contains("fn log"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_list_comprehension() {
        let code = r"
def doubles(nums: list):
    return [x * 2 for x in nums]
";
        let rust = transpile(code);
        assert!(rust.contains("fn doubles"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_nested_return() {
        let code = r"
def safe_div(a: int, b: int):
    if b == 0:
        return None
    return a // b
";
        let rust = transpile(code);
        assert!(rust.contains("fn safe_div"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_from_isinstance() {
        let code = r"
def is_str(x) -> bool:
    return isinstance(x, str)
";
        let rust = transpile(code);
        assert!(rust.contains("fn is_str"), "output: {}", rust);
    }

    #[test]
    fn test_s9b6_infer_multiple_exit_paths() {
        let code = r#"
def analyze(x: int):
    if x > 0:
        return "positive"
    return "non-positive"
"#;
        let rust = transpile(code);
        assert!(rust.contains("fn analyze"), "output: {}", rust);
    }
}
