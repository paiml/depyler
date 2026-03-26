//! Advanced function codegen: `impl RustCodeGen for HirFunction` and helpers.
//!
//! DEPYLER-COVERAGE-95: Extracted from func_gen.rs to reduce file size
//! and improve testability.

use crate::hir::*;
use crate::lifetime_analysis::LifetimeInference;
use crate::rust_gen::context::{CodeGenContext, ErrorType, RustCodeGen};
use crate::rust_gen::func_gen::{
    codegen_function_body, codegen_function_params, infer_return_type_from_body_with_params,
    propagate_return_type_to_vars,
};
use crate::rust_gen::func_gen_helpers::{
    codegen_function_attrs, codegen_generic_params, codegen_where_clause,
};
use crate::rust_gen::generator_gen::codegen_generator_function;
use crate::rust_gen::keywords::is_rust_keyword;
use crate::rust_gen::type_tokens::hir_type_to_tokens;
use anyhow::Result;
use quote::quote;

/// Detect if a function body contains nested function definitions that
/// return values, enabling the outer function to capture their types.
pub(crate) fn detect_returns_nested_function(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
) -> Option<(String, Vec<HirParam>, Type)> {
    let mut nested_functions: std::collections::HashMap<String, (Vec<HirParam>, Type)> =
        std::collections::HashMap::new();

    for stmt in &func.body {
        if let HirStmt::FunctionDef { name, params, ret_type, body, .. } = stmt {
            let mut inferred_params = params.to_vec();
            for param in &mut inferred_params {
                if matches!(param.ty, Type::Unknown) {
                    if let Some(inferred_ty) =
                        crate::param_type_inference::infer_param_type_from_body(&param.name, body)
                    {
                        param.ty = inferred_ty;
                    }
                }
            }
            ctx.nested_function_params.insert(name.clone(), inferred_params.clone());
            nested_functions.insert(name.clone(), (inferred_params, ret_type.clone()));
        }
    }

    // Check if last statement returns a nested function name
    if let Some(HirStmt::Return(Some(HirExpr::Var(name)))) = func.body.last() {
        if let Some((params, ret_type)) = nested_functions.get(name) {
            return Some((name.clone(), params.clone(), ret_type.clone()));
        }
    }

    None
}

/// Pre-load HIR type annotations from assignment statements into ctx.var_types.
fn preload_hir_type_annotations(body: &[HirStmt], ctx: &mut CodeGenContext) {
    for stmt in body {
        preload_stmt_type_annotations(stmt, ctx);
    }
}

fn preload_stmt_type_annotations(stmt: &HirStmt, ctx: &mut CodeGenContext) {
    match stmt {
        HirStmt::Assign { target, type_annotation: Some(ty), .. } => {
            if let AssignTarget::Symbol(name) = target {
                if !matches!(ty, Type::Unknown) {
                    ctx.var_types.insert(name.clone(), ty.clone());
                }
            }
        }
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
        HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
            for s in body {
                preload_stmt_type_annotations(s, ctx);
            }
        }
        HirStmt::Try { body, handlers, finalbody, .. } => {
            for s in body {
                preload_stmt_type_annotations(s, ctx);
            }
            for handler in handlers {
                for s in &handler.body {
                    preload_stmt_type_annotations(s, ctx);
                }
            }
            if let Some(finally) = finalbody {
                for s in finally {
                    preload_stmt_type_annotations(s, ctx);
                }
            }
        }
        HirStmt::With { body, .. } => {
            for s in body {
                preload_stmt_type_annotations(s, ctx);
            }
        }
        HirStmt::Block(stmts) => {
            for s in stmts {
                preload_stmt_type_annotations(s, ctx);
            }
        }
        _ => {}
    }
}

/// Compute effective return type, inferring from body if needed.
fn compute_effective_return_type(func: &HirFunction, ctx: &mut CodeGenContext) -> Type {
    let mut effective_ret = func.ret_type.clone();

    if matches!(effective_ret, Type::Unknown | Type::None) {
        // Try to infer return type from body
        if let Some(inferred) = infer_return_type_from_body_with_params(func, ctx) {
            if !matches!(inferred, Type::Unknown) {
                effective_ret = inferred;
            }
        }
    }

    // Store for use during body codegen
    ctx.current_return_type = Some(effective_ret.clone());
    effective_ret
}

/// Build return type tokens from HirType.
fn build_return_type_tokens(
    effective_ret: &Type,
    can_fail: bool,
    error_type: &Option<ErrorType>,
) -> proc_macro2::TokenStream {
    if matches!(effective_ret, Type::None | Type::Unknown) && !can_fail {
        return quote! {};
    }

    let base_tokens = hir_type_to_tokens(effective_ret);

    if can_fail {
        match error_type {
            Some(ErrorType::Concrete(ty_str)) => {
                if let Ok(err_ty) = syn::parse_str::<syn::Type>(ty_str) {
                    if matches!(effective_ret, Type::None | Type::Unknown) {
                        quote! { -> Result<(), #err_ty> }
                    } else {
                        quote! { -> Result<#base_tokens, #err_ty> }
                    }
                } else {
                    quote! { -> Result<#base_tokens> }
                }
            }
            Some(ErrorType::DynBox) => {
                if matches!(effective_ret, Type::None | Type::Unknown) {
                    quote! { -> Result<(), Box<dyn std::error::Error>> }
                } else {
                    quote! { -> Result<#base_tokens, Box<dyn std::error::Error>> }
                }
            }
            None => {
                quote! { -> Result<#base_tokens> }
            }
        }
    } else {
        quote! { -> #base_tokens }
    }
}

/// Determine error type from function properties.
fn determine_error_type(func: &HirFunction) -> Option<ErrorType> {
    if !func.properties.can_fail {
        return None;
    }
    if func.properties.error_types.len() > 1 {
        Some(ErrorType::DynBox)
    } else if let Some(et) = func.properties.error_types.first() {
        let rust_err = map_python_error_to_rust(et);
        Some(ErrorType::Concrete(rust_err))
    } else {
        Some(ErrorType::DynBox)
    }
}

fn map_python_error_to_rust(error_type_str: &str) -> String {
    match error_type_str {
        "ValueError" => "ValueError".to_string(),
        "TypeError" => "TypeError".to_string(),
        "KeyError" => "KeyError".to_string(),
        "IndexError" => "IndexError".to_string(),
        "IOError" | "OSError" => "std::io::Error".to_string(),
        "FileNotFoundError" => "std::io::Error".to_string(),
        "ZeroDivisionError" => "ZeroDivisionError".to_string(),
        other => other.to_string(),
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

        let param_defaults: Vec<Option<HirExpr>> =
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

        let type_params = generic_registry.infer_function_generics(self)?;

        // Lifetime analysis
        let mut lifetime_inference = LifetimeInference::new();
        let lifetime_result = lifetime_inference
            .apply_elision_rules(self, ctx.type_mapper)
            .unwrap_or_else(|| lifetime_inference.analyze_function(self, ctx.type_mapper));

        let generic_params = codegen_generic_params(&type_params, &lifetime_result.lifetime_params);
        let where_clause = codegen_where_clause(&lifetime_result.lifetime_bounds);

        crate::rust_gen::analyze_mutable_vars(&self.body, ctx, &self.params);
        let params = codegen_function_params(self, &lifetime_result, ctx)?;

        // Track parameter metadata
        for param in &self.params {
            if !matches!(param.ty, Type::Unknown) {
                ctx.var_types.insert(param.name.clone(), param.ty.clone());
            }
        }

        // Return type generation
        let effective_ret = compute_effective_return_type(self, ctx);
        let can_fail = self.properties.can_fail;
        let error_type = determine_error_type(self);
        let return_type = build_return_type_tokens(&effective_ret, can_fail, &error_type);

        // Argparser setup
        if ctx.argparser_tracker.has_parsers() {
            if let Some(parser_info) = ctx.argparser_tracker.get_first_parser() {
                for arg in &parser_info.arguments {
                    if arg.rust_type().starts_with("Option<") {
                        ctx.precomputed_option_fields.insert(arg.rust_field_name().to_string());
                    }
                }
            }
        }
        crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(
            self,
            &mut ctx.argparser_tracker,
        );

        // Generate body
        let was_main = ctx.is_main_function;
        ctx.is_main_function = self.name == "main";

        for param in &self.params {
            if !matches!(param.ty, Type::Unknown) {
                ctx.var_types.insert(param.name.clone(), param.ty.clone());
            }
        }
        preload_hir_type_annotations(&self.body, ctx);

        propagate_return_type_to_vars(&self.body, &mut ctx.var_types, &effective_ret);

        let body_stmts = codegen_function_body(self, can_fail, error_type, ctx)?;
        ctx.is_main_function = was_main;

        // Generate function attributes
        let attrs = codegen_function_attrs(
            &self.docstring,
            &self.properties,
            &self.annotations.custom_attributes,
        );

        // Dummy RustType for generator function (parameter is unused)
        let dummy_rust_ret = crate::type_mapper::RustType::Unit;

        let func_tokens = if self.properties.is_generator {
            codegen_generator_function(
                self,
                &name,
                &generic_params,
                &where_clause,
                &params,
                &attrs,
                &dummy_rust_ret,
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
