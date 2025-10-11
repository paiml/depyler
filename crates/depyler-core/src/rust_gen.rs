use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use crate::hir::*;
use crate::lifetime_analysis::LifetimeInference;
use crate::string_optimization::StringOptimizer;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{self, parse_quote};

// Module declarations for rust_gen refactoring (v3.18.0 Phases 2-5)
mod context;
mod error_gen;
mod expr_gen;
mod format;
mod generator_gen;
mod import_gen;
mod type_gen;

// Internal imports
use error_gen::generate_error_type_definitions;
use format::format_rust_code;
use generator_gen::codegen_generator_function;
use import_gen::process_module_imports;
use type_gen::update_import_needs;

// Public re-exports for external modules (union_enum_gen, etc.)
pub use context::{CodeGenContext, RustCodeGen, ToRustExpr};
pub use type_gen::rust_type_to_syn;

/// Analyze functions for string optimization
///
/// Performs string optimization analysis on all functions.
/// Complexity: 2 (well within ≤10 target)
fn analyze_string_optimization(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        ctx.string_optimizer.analyze_function(func);
    }
}

/// Analyze which variables are reassigned (mutated) in a list of statements
///
/// Populates ctx.mutable_vars with variables that are reassigned after declaration.
/// Complexity: 4 (loop + match + if)
fn analyze_mutable_vars(stmts: &[HirStmt], ctx: &mut CodeGenContext) {
    let mut declared = HashSet::new();

    fn analyze_stmt(stmt: &HirStmt, declared: &mut HashSet<String>, mutable: &mut HashSet<String>) {
        match stmt {
            HirStmt::Assign { target, .. } => {
                match target {
                    AssignTarget::Symbol(name) => {
                        if declared.contains(name) {
                            // Variable is being reassigned - mark as mutable
                            mutable.insert(name.clone());
                        } else {
                            // First declaration
                            declared.insert(name.clone());
                        }
                    }
                    AssignTarget::Tuple(targets) => {
                        // Tuple assignment - analyze each element
                        for t in targets {
                            if let AssignTarget::Symbol(name) = t {
                                if declared.contains(name) {
                                    // Variable is being reassigned - mark as mutable
                                    mutable.insert(name.clone());
                                } else {
                                    // First declaration
                                    declared.insert(name.clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                for stmt in then_body {
                    analyze_stmt(stmt, declared, mutable);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        analyze_stmt(stmt, declared, mutable);
                    }
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                for stmt in body {
                    analyze_stmt(stmt, declared, mutable);
                }
            }
            _ => {}
        }
    }

    for stmt in stmts {
        analyze_stmt(stmt, &mut declared, &mut ctx.mutable_vars);
    }
}

/// Convert Python classes to Rust structs
///
/// Processes all classes and generates token streams.
/// Complexity: 3 (well within ≤10 target)
fn convert_classes_to_rust(
    classes: &[HirClass],
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut class_items = Vec::new();
    for class in classes {
        let items = crate::direct_rules::convert_class_to_struct(class, type_mapper)?;
        for item in items {
            let tokens = item.to_token_stream();
            class_items.push(tokens);
        }
    }
    Ok(class_items)
}

/// Convert HIR functions to Rust token streams
///
/// Processes all functions using the code generation context.
/// Complexity: 2 (well within ≤10 target)
fn convert_functions_to_rust(
    functions: &[HirFunction],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    functions
        .iter()
        .map(|f| f.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()
}

/// Generate conditional imports based on code generation context
///
/// Adds imports for collections and smart pointers as needed.
/// Complexity: 1 (data-driven approach, well within ≤10 target)
fn generate_conditional_imports(ctx: &CodeGenContext) -> Vec<proc_macro2::TokenStream> {
    let mut imports = Vec::new();

    // Define all possible conditional imports
    let conditional_imports = [
        (ctx.needs_hashmap, quote! { use std::collections::HashMap; }),
        (ctx.needs_hashset, quote! { use std::collections::HashSet; }),
        (ctx.needs_fnv_hashmap, quote! { use fnv::FnvHashMap; }),
        (ctx.needs_ahash_hashmap, quote! { use ahash::AHashMap; }),
        (ctx.needs_arc, quote! { use std::sync::Arc; }),
        (ctx.needs_rc, quote! { use std::rc::Rc; }),
        (ctx.needs_cow, quote! { use std::borrow::Cow; }),
    ];

    // Add imports where needed
    for (needed, import_tokens) in conditional_imports {
        if needed {
            imports.push(import_tokens);
        }
    }

    imports
}

/// Generate import token streams from Python imports
///
/// Maps Python imports to Rust use statements.
/// Complexity: ~7-8 (within ≤10 target)
fn generate_import_tokens(
    imports: &[Import],
    module_mapper: &crate::module_mapper::ModuleMapper,
) -> Vec<proc_macro2::TokenStream> {
    let mut items = Vec::new();
    let mut external_imports = Vec::new();
    let mut std_imports = Vec::new();

    // Categorize imports
    for import in imports {
        let rust_imports = module_mapper.map_import(import);
        for rust_import in rust_imports {
            if rust_import.path.starts_with("//") {
                // Comment for unmapped imports
                let comment = &rust_import.path;
                items.push(quote! { #[doc = #comment] });
            } else if rust_import.is_external {
                external_imports.push(rust_import);
            } else {
                std_imports.push(rust_import);
            }
        }
    }

    // Add external imports
    for import in external_imports {
        let path: syn::Path =
            syn::parse_str(&import.path).unwrap_or_else(|_| parse_quote! { unknown });
        if let Some(alias) = import.alias {
            let alias_ident = syn::Ident::new(&alias, proc_macro2::Span::call_site());
            items.push(quote! { use #path as #alias_ident; });
        } else {
            items.push(quote! { use #path; });
        }
    }

    // Add standard library imports
    for import in std_imports {
        // Skip typing imports as they're handled by the type system
        if import.path.starts_with("::") || import.path.is_empty() {
            continue;
        }
        let path: syn::Path = syn::parse_str(&import.path).unwrap_or_else(|_| parse_quote! { std });
        if let Some(alias) = import.alias {
            let alias_ident = syn::Ident::new(&alias, proc_macro2::Span::call_site());
            items.push(quote! { use #path as #alias_ident; });
        } else {
            items.push(quote! { use #path; });
        }
    }

    items
}

/// Generate interned string constant tokens
///
/// Generates constant definitions for interned strings.
/// Complexity: 2 (well within ≤10 target)
fn generate_interned_string_tokens(optimizer: &StringOptimizer) -> Vec<proc_macro2::TokenStream> {
    let interned_constants = optimizer.generate_interned_constants();
    interned_constants
        .into_iter()
        .filter_map(|constant| constant.parse().ok())
        .collect()
}

/// Generate a complete Rust file from HIR module
pub fn generate_rust_file(
    module: &HirModule,
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<String> {
    let module_mapper = crate::module_mapper::ModuleMapper::new();

    // Process imports to populate the context
    let (imported_modules, imported_items) =
        process_module_imports(&module.imports, &module_mapper);

    let mut ctx = CodeGenContext {
        type_mapper,
        annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(type_mapper.clone()),
        string_optimizer: StringOptimizer::new(),
        union_enum_generator: crate::union_enum_gen::UnionEnumGenerator::new(),
        generated_enums: Vec::new(),
        needs_hashmap: false,
        needs_hashset: false,
        needs_fnv_hashmap: false,
        needs_ahash_hashmap: false,
        needs_arc: false,
        needs_rc: false,
        needs_cow: false,
        declared_vars: vec![HashSet::new()],
        current_function_can_fail: false,
        current_return_type: None,
        module_mapper,
        imported_modules,
        imported_items,
        mutable_vars: HashSet::new(),
        needs_zerodivisionerror: false,
        in_generator: false,
        needs_indexerror: false,
        is_classmethod: false,
        generator_state_vars: HashSet::new(),
    };

    // Analyze all functions first for string optimization
    analyze_string_optimization(&mut ctx, &module.functions);

    // Convert classes first (they might be used by functions)
    let classes = convert_classes_to_rust(&module.classes, ctx.type_mapper)?;

    // Convert all functions to detect what imports we need
    let functions = convert_functions_to_rust(&module.functions, &mut ctx)?;

    // Build items list with all generated code
    let mut items = Vec::new();

    // Add module imports (create new mapper for token generation)
    let import_mapper = crate::module_mapper::ModuleMapper::new();
    items.extend(generate_import_tokens(&module.imports, &import_mapper));

    // Add interned string constants
    items.extend(generate_interned_string_tokens(&ctx.string_optimizer));

    // Add collection imports if needed
    items.extend(generate_conditional_imports(&ctx));

    // Add error type definitions if needed
    items.extend(generate_error_type_definitions(&ctx));

    // Add generated union enums
    items.extend(ctx.generated_enums.clone());

    // Add classes
    items.extend(classes);

    // Add all functions
    items.extend(functions);

    // Generate tests for functions if applicable
    let test_gen = crate::test_generation::TestGenerator::new(Default::default());
    let mut test_modules = Vec::new();

    for func in &module.functions {
        if let Some(test_module) = test_gen.generate_tests(func)? {
            test_modules.push(test_module);
        }
    }

    // Add test modules
    items.extend(test_modules);

    let file = quote! {
        #(#items)*
    };

    Ok(format_rust_code(file.to_string()))
}

// ============================================================================
// DEPYLER-0141 Phase 1: HirFunction Helper Functions
// ============================================================================

/// Generate combined generic parameters (<'a, 'b, T, U: Bound>)
#[inline]
fn codegen_generic_params(
    type_params: &[crate::generic_inference::TypeParameter],
    lifetime_params: &[String],
) -> proc_macro2::TokenStream {
    if type_params.is_empty() && lifetime_params.is_empty() {
        return quote! {};
    }

    let mut all_params = Vec::new();

    // Add lifetime parameters first
    for lt in lifetime_params {
        let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
        all_params.push(quote! { #lt_ident });
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
fn codegen_where_clause(lifetime_bounds: &[(String, String)]) -> proc_macro2::TokenStream {
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
fn codegen_function_attrs(
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
fn codegen_function_body(
    func: &HirFunction,
    can_fail: bool,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    // Enter function scope and declare parameters
    ctx.enter_scope();
    ctx.current_function_can_fail = can_fail;
    ctx.current_return_type = Some(func.ret_type.clone());

    for param in &func.params {
        ctx.declare_var(&param.name);
    }

    // Analyze which variables are mutated in the function body
    analyze_mutable_vars(&func.body, ctx);

    // Convert body
    let body_stmts: Vec<_> = func
        .body
        .iter()
        .map(|stmt| stmt.to_rust_tokens(ctx))
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
fn codegen_function_params(
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

    // Check if parameter is mutated
    let is_param_mutated = matches!(
        lifetime_result.borrowing_strategies.get(&param.name),
        Some(crate::borrowing_context::BorrowingStrategy::TakeOwnership)
    ) && func.body.iter().any(
        |stmt| matches!(stmt, HirStmt::Assign { target: AssignTarget::Symbol(s), .. } if s == &param.name),
    );

    // Get the inferred parameter info
    if let Some(inferred) = lifetime_result.param_lifetimes.get(&param.name) {
        let rust_type = &inferred.rust_type;

        // Handle Union type placeholders
        let actual_rust_type = if let crate::type_mapper::RustType::Enum { name, variants: _ } = rust_type {
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
        let ty = apply_param_borrowing_strategy(
            &param.name,
            &actual_rust_type,
            inferred,
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

    // Check if we have a borrowing strategy
    if let Some(strategy) = lifetime_result.borrowing_strategies.get(param_name) {
        match strategy {
            crate::borrowing_context::BorrowingStrategy::UseCow { lifetime } => {
                ctx.needs_cow = true;
                let lt = syn::Lifetime::new(lifetime, proc_macro2::Span::call_site());
                ty = parse_quote! { Cow<#lt, str> };
            }
            _ => {
                // Apply normal borrowing if needed
                if inferred.should_borrow {
                    ty = apply_borrowing_to_type(ty, rust_type, inferred)?;
                }
            }
        }
    } else {
        // Fallback to normal borrowing
        if inferred.should_borrow {
            ty = apply_borrowing_to_type(ty, rust_type, inferred)?;
        }
    }

    Ok(ty)
}

/// Apply borrowing (&, &mut, with lifetime) to a type
fn apply_borrowing_to_type(
    mut ty: syn::Type,
    rust_type: &crate::type_mapper::RustType,
    inferred: &crate::lifetime_analysis::InferredParam,
) -> Result<syn::Type> {
    // Special case for strings: use &str instead of &String
    if matches!(rust_type, crate::type_mapper::RustType::String) {
        if let Some(ref lifetime) = inferred.lifetime {
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
        if let Some(ref lifetime) = inferred.lifetime {
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
        "upper" | "lower" | "strip" | "lstrip" | "rstrip"
        | "replace" | "format" | "title" | "capitalize"
        | "swapcase" | "expandtabs" | "center" | "ljust"
        | "rjust" | "zfill" => StringMethodReturnType::Owned,

        // Query/test methods that return bool or &str (borrowed)
        "startswith" | "endswith" | "isalpha" | "isdigit"
        | "isalnum" | "isspace" | "islower" | "isupper"
        | "istitle" | "isascii" | "isprintable"
        | "find" | "rfind" | "index" | "rindex"
        | "count" => StringMethodReturnType::Borrowed,

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
        HirExpr::Unary { operand, .. } => {
            contains_owned_string_method(operand)
        }
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
#[inline]
fn codegen_return_type(
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) -> Result<(proc_macro2::TokenStream, crate::type_mapper::RustType, bool)> {
    // Convert return type using annotation-aware mapping
    let mapped_ret_type = ctx
        .annotation_aware_mapper
        .map_return_type_with_annotations(&func.ret_type, &func.annotations);

    // Check if this is a placeholder Union enum that needs proper generation
    let rust_ret_type =
        if let crate::type_mapper::RustType::Enum { name, .. } = &mapped_ret_type {
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
    let rust_ret_type = if matches!(func.ret_type, Type::String)
        && function_returns_owned_string(func)
    {
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

    // Mark error types as needed for type generation
    if error_type_str.contains("ZeroDivisionError") {
        ctx.needs_zerodivisionerror = true;
    }
    if error_type_str.contains("IndexError") {
        ctx.needs_indexerror = true;
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

        // Check if any parameter escapes through return and uses Cow
        let mut uses_cow_return = false;
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

        if uses_cow_return {
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
            let returns_owned_string = matches!(func.ret_type, Type::String)
                && function_returns_owned_string(func);

            // Apply return lifetime if needed (unless returning owned String)
            if let Some(ref return_lt) = lifetime_result.return_lifetime {
                // Check if the return type needs lifetime substitution
                if matches!(
                    rust_ret_type,
                    crate::type_mapper::RustType::Str { .. }
                        | crate::type_mapper::RustType::Reference { .. }
                ) && !returns_owned_string {
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

    Ok((return_type, rust_ret_type, can_fail))
}

// ========== Phase 3c: Generator Implementation ==========
// (Moved to generator_gen.rs in v3.18.0 Phase 4)

impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        // Perform generic type inference
        let mut generic_registry = crate::generic_inference::TypeVarRegistry::new();
        let type_params = generic_registry.infer_function_generics(self)?;

        // Perform lifetime analysis
        let mut lifetime_inference = LifetimeInference::new();
        let lifetime_result = lifetime_inference.analyze_function(self, ctx.type_mapper);

        // Generate combined generic parameters (lifetimes + type params)
        let generic_params = codegen_generic_params(&type_params, &lifetime_result.lifetime_params);

        // Generate lifetime bounds
        let where_clause = codegen_where_clause(&lifetime_result.lifetime_bounds);

        // Convert parameters using lifetime analysis results
        let params = codegen_function_params(self, &lifetime_result, ctx)?;

        // Generate return type with Result wrapper and lifetime handling
        let (return_type, rust_ret_type, can_fail) = codegen_return_type(self, &lifetime_result, ctx)?;

        // Process function body with proper scoping
        let body_stmts = codegen_function_body(self, can_fail, ctx)?;

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

/// Check if a type annotation requires explicit conversion
///
/// For numeric types like Int, we always apply conversion to ensure correctness
/// even after optimizations like CSE transform the expression.
/// Complexity: 2 (simple match)
fn needs_type_conversion(target_type: &Type) -> bool {
    // For Int annotations, always apply conversion to handle cases where
    // the value might be usize (from len(), range, etc.)
    matches!(target_type, Type::Int)
}

/// Apply type conversion to value expression
///
/// Wraps the expression with appropriate conversion (e.g., `as i32`)
/// Complexity: 2 (simple match)
fn apply_type_conversion(value_expr: syn::Expr, target_type: &Type) -> syn::Expr {
    match target_type {
        Type::Int => {
            // Convert to i32 using 'as' cast
            // This handles usize->i32 conversions and is a no-op if already i32
            // Note: Removed defensive parens - Rust's precedence rules handle this correctly
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
fn codegen_pass_stmt() -> Result<proc_macro2::TokenStream> {
    Ok(quote! {})
}

/// Generate code for Break statement with optional label
#[inline]
fn codegen_break_stmt(label: &Option<String>) -> Result<proc_macro2::TokenStream> {
    if let Some(label_name) = label {
        let label_ident = syn::Lifetime::new(
            &format!("'{}", label_name),
            proc_macro2::Span::call_site(),
        );
        Ok(quote! { break #label_ident; })
    } else {
        Ok(quote! { break; })
    }
}

/// Generate code for Continue statement with optional label
#[inline]
fn codegen_continue_stmt(label: &Option<String>) -> Result<proc_macro2::TokenStream> {
    if let Some(label_name) = label {
        let label_ident = syn::Lifetime::new(
            &format!("'{}", label_name),
            proc_macro2::Span::call_site(),
        );
        Ok(quote! { continue #label_ident; })
    } else {
        Ok(quote! { continue; })
    }
}

/// Generate code for expression statement
#[inline]
fn codegen_expr_stmt(expr: &HirExpr, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
    let expr_tokens = expr.to_rust_expr(ctx)?;
    Ok(quote! { #expr_tokens; })
}

// ============================================================================
// Statement Code Generation Helpers (DEPYLER-0140 Phase 2)
// Medium-complexity handlers extracted from HirStmt::to_rust_tokens
// ============================================================================

/// Generate code for Return statement with optional expression
#[inline]
fn codegen_return_stmt(
    expr: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    if let Some(e) = expr {
        let expr_tokens = e.to_rust_expr(ctx)?;

        // Check if return type is Optional and wrap value in Some()
        let is_optional_return =
            matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));

        // Check if the expression is None literal
        let is_none_literal = matches!(e, HirExpr::Literal(Literal::None));

        if ctx.current_function_can_fail {
            if is_optional_return && !is_none_literal {
                // Wrap value in Some() for Optional return types
                Ok(quote! { return Ok(Some(#expr_tokens)); })
            } else {
                Ok(quote! { return Ok(#expr_tokens); })
            }
        } else if is_optional_return && !is_none_literal {
            // Wrap value in Some() for Optional return types
            Ok(quote! { return Some(#expr_tokens); })
        } else {
            Ok(quote! { return #expr_tokens; })
        }
    } else if ctx.current_function_can_fail {
        // No expression - check if return type is Optional
        let is_optional_return =
            matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));
        if is_optional_return {
            Ok(quote! { return Ok(None); })
        } else {
            Ok(quote! { return Ok(()); })
        }
    } else {
        Ok(quote! { return; })
    }
}

/// Generate code for While loop statement
#[inline]
fn codegen_while_stmt(
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
#[inline]
fn codegen_raise_stmt(
    exception: &Option<HirExpr>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // For V1, we'll implement basic error handling
    if let Some(exc) = exception {
        let exc_expr = exc.to_rust_expr(ctx)?;
        Ok(quote! { return Err(#exc_expr); })
    } else {
        // Re-raise or bare raise - use generic error
        Ok(quote! { return Err("Exception raised".into()); })
    }
}

/// Generate code for With (context manager) statement
#[inline]
fn codegen_with_stmt(
    context: &HirExpr,
    target: &Option<String>,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Convert context expression
    let context_expr = context.to_rust_expr(ctx)?;

    // Convert body statements
    let body_stmts: Vec<_> = body
        .iter()
        .map(|stmt| stmt.to_rust_tokens(ctx))
        .collect::<Result<_>>()?;

    // Note: Currently generates a simple scope block for context managers.
    // Proper RAII pattern with Drop trait implementation is not yet supported.
    // This is a known limitation - __enter__/__exit__ methods are not translated.
    if let Some(var_name) = target {
        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
        ctx.declare_var(var_name);
        Ok(quote! {
            {
                let mut #var_ident = #context_expr;
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

/// Generate code for If statement with optional else clause
#[inline]
fn codegen_if_stmt(
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let cond = condition.to_rust_expr(ctx)?;
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

/// Generate code for For loop statement
#[inline]
fn codegen_for_stmt(
    target: &str,
    iter: &HirExpr,
    body: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
    let mut iter_expr = iter.to_rust_expr(ctx)?;

    // Check if we're iterating over a borrowed collection
    // If iter is a simple variable that refers to a borrowed collection (e.g., &Vec<T>),
    // we need to add .iter() to properly iterate over it
    if let HirExpr::Var(_var_name) = iter {
        // This is a simple heuristic: if the expression is just a variable name,
        // it's likely a parameter or local var that might be borrowed
        // The generated code already has the variable as borrowed (e.g., data: &Vec<T>)
        // so we need to call .iter() on it
        iter_expr = parse_quote! { #iter_expr.iter() };
    }

    ctx.enter_scope();
    ctx.declare_var(target); // for loop variable is declared in the loop scope
    let body_stmts: Vec<_> = body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();
    Ok(quote! {
        for #target_ident in #iter_expr {
            #(#body_stmts)*
        }
    })
}

/// Generate code for Assign statement (variable/index/attribute/tuple assignment)
#[inline]
fn codegen_assign_stmt(
    target: &AssignTarget,
    value: &HirExpr,
    type_annotation: &Option<Type>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let mut value_expr = value.to_rust_expr(ctx)?;

    // If there's a type annotation, handle type conversions
    let type_annotation_tokens = if let Some(target_type) = type_annotation {
        let target_rust_type = ctx.type_mapper.map_type(target_type);
        let target_syn_type = rust_type_to_syn(&target_rust_type)?;

        // Check if we need type conversion (e.g., usize to i32)
        if needs_type_conversion(target_type) {
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
        AssignTarget::Index { base, index } => {
            codegen_assign_index(base, index, value_expr, ctx)
        }
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
fn codegen_assign_symbol(
    symbol: &str,
    value_expr: syn::Expr,
    type_annotation_tokens: Option<proc_macro2::TokenStream>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let target_ident = syn::Ident::new(symbol, proc_macro2::Span::call_site());

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
fn codegen_assign_index(
    base: &HirExpr,
    index: &HirExpr,
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let final_index = index.to_rust_expr(ctx)?;

    // Extract the base and all intermediate indices
    let (base_expr, indices) = extract_nested_indices_tokens(base, ctx)?;

    if indices.is_empty() {
        // Simple assignment: d[k] = v
        Ok(quote! { #base_expr.insert(#final_index, #value_expr); })
    } else {
        // Nested assignment: build chain of get_mut calls
        let mut chain = quote! { #base_expr };
        for idx in &indices {
            chain = quote! {
                #chain.get_mut(&#idx).unwrap()
            };
        }

        Ok(quote! { #chain.insert(#final_index, #value_expr); })
    }
}

/// Generate code for attribute (struct field) assignment
#[inline]
fn codegen_assign_attribute(
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
fn codegen_assign_tuple(
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
                    .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                    .collect();
                Ok(quote! { (#(#idents),*) = #value_expr; })
            } else {
                // First declaration - mark each variable individually
                symbols.iter().for_each(|s| ctx.declare_var(s));
                let idents_with_mut: Vec<_> = symbols
                    .iter()
                    .map(|s| {
                        let ident = syn::Ident::new(s, proc_macro2::Span::call_site());
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
fn codegen_try_stmt(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // Convert try body to statements
    ctx.enter_scope();
    let try_stmts: Vec<_> = body
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.exit_scope();

    // Generate except handler code
    let mut handler_tokens = Vec::new();
    for handler in handlers {
        ctx.enter_scope();

        // If there's a name binding, declare it in scope
        if let Some(var_name) = &handler.name {
            ctx.declare_var(var_name);
        }

        let handler_stmts: Vec<_> = handler
            .body
            .iter()
            .map(|s| s.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;
        ctx.exit_scope();

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
    } else if handlers.len() == 1 {
        let handler_code = &handler_tokens[0];
        if let Some(finally_code) = finally_stmts {
            Ok(quote! {
                {
                    let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                        #(#try_stmts)*
                        Ok(())
                    })();
                    if let Err(_e) = _result {
                        #handler_code
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
                        #handler_code
                    }
                }
            })
        }
    } else {
        // Multiple handlers
        if let Some(finally_code) = finally_stmts {
            Ok(quote! {
                {
                    let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                        #(#try_stmts)*
                        Ok(())
                    })();
                    if let Err(_e) = _result {
                        #(#handler_tokens)*
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
                        #(#handler_tokens)*
                    }
                }
            })
        }
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
            HirStmt::Pass => codegen_pass_stmt(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
    use crate::rust_gen::context::RustCodeGen;
    use crate::type_mapper::TypeMapper;
    use depyler_annotations::TranspilationAnnotations;
    use std::collections::HashSet;
    use crate::rust_gen::type_gen::convert_binop;

    fn create_test_context() -> CodeGenContext<'static> {
        // This is a bit of a hack for testing - in real use, the TypeMapper would have a longer lifetime
        let type_mapper: &'static TypeMapper = Box::leak(Box::new(TypeMapper::default()));
        CodeGenContext {
            type_mapper,
            annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(
                type_mapper.clone(),
            ),
            string_optimizer: StringOptimizer::new(),
            union_enum_generator: crate::union_enum_gen::UnionEnumGenerator::new(),
            generated_enums: Vec::new(),
            needs_hashmap: false,
            needs_hashset: false,
            needs_fnv_hashmap: false,
            needs_ahash_hashmap: false,
            needs_arc: false,
            needs_rc: false,
            needs_cow: false,
            declared_vars: vec![HashSet::new()],
            current_function_can_fail: false,
            current_return_type: None,
            module_mapper: crate::module_mapper::ModuleMapper::new(),
            imported_modules: std::collections::HashMap::new(),
            imported_items: std::collections::HashMap::new(),
            mutable_vars: HashSet::new(),
            needs_zerodivisionerror: false,
            needs_indexerror: false,
            is_classmethod: false,
            in_generator: false,
            generator_state_vars: HashSet::new(),
        }
    }

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

        let mut ctx = create_test_context();
        let tokens = func.to_rust_tokens(&mut ctx).unwrap();
        let code = tokens.to_string();

        assert!(code.contains("pub fn add"));
        assert!(code.contains("i32"));
        assert!(code.contains("return"));
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

        let mut ctx = create_test_context();
        let tokens = if_stmt.to_rust_tokens(&mut ctx).unwrap();
        let code = tokens.to_string();

        assert!(code.contains("if"));
        assert!(code.contains("else"));
        assert!(code.contains("return"));
    }

    #[test]
    fn test_list_generation() {
        // Test literal array generation
        let list_expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);

        let mut ctx = create_test_context();
        let expr = list_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #expr }.to_string();

        // Small literal lists should generate arrays
        assert!(code.contains("[") && code.contains("]"));
        assert!(code.contains("1"));
        assert!(code.contains("2"));
        assert!(code.contains("3"));

        // Test non-literal list still uses vec!
        let var_list = HirExpr::List(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Var("y".to_string()),
        ]);

        let expr2 = var_list.to_rust_expr(&mut ctx).unwrap();
        let code2 = quote! { #expr2 }.to_string();
        assert!(code2.contains("vec !"));
    }

    #[test]
    fn test_dict_generation_sets_needs_hashmap() {
        let dict_expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);

        let mut ctx = create_test_context();
        assert!(!ctx.needs_hashmap);

        let _ = dict_expr.to_rust_expr(&mut ctx).unwrap();

        assert!(ctx.needs_hashmap);
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
            let result = convert_binop(op).unwrap();
            assert_eq!(quote! { #result }.to_string(), expected);
        }
    }

    #[test]
    fn test_unsupported_operators() {
        assert!(convert_binop(BinOp::Pow).is_err());
        assert!(convert_binop(BinOp::In).is_err());
        assert!(convert_binop(BinOp::NotIn).is_err());
    }

    // ========================================================================
    // DEPYLER-0140 Phase 1: Tests for extracted statement handlers
    // ========================================================================

    #[test]
    fn test_codegen_pass_stmt() {
        let result = codegen_pass_stmt().unwrap();
        assert!(result.is_empty(), "Pass statement should generate no code");
    }

    #[test]
    fn test_codegen_break_stmt_simple() {
        let result = codegen_break_stmt(&None).unwrap();
        assert_eq!(result.to_string(), "break ;");
    }

    #[test]
    fn test_codegen_break_stmt_with_label() {
        let result = codegen_break_stmt(&Some("outer".to_string())).unwrap();
        assert_eq!(result.to_string(), "break 'outer ;");
    }

    #[test]
    fn test_codegen_continue_stmt_simple() {
        let result = codegen_continue_stmt(&None).unwrap();
        assert_eq!(result.to_string(), "continue ;");
    }

    #[test]
    fn test_codegen_continue_stmt_with_label() {
        let result = codegen_continue_stmt(&Some("outer".to_string())).unwrap();
        assert_eq!(result.to_string(), "continue 'outer ;");
    }

    #[test]
    fn test_codegen_expr_stmt() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let expr = HirExpr::Literal(Literal::Int(42));

        let result = codegen_expr_stmt(&expr, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "42 ;");
    }

    // ========================================================================
    // DEPYLER-0140 Phase 2: Tests for medium-complexity statement handlers
    // ========================================================================

    #[test]
    fn test_codegen_return_stmt_simple() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let expr = Some(HirExpr::Literal(Literal::Int(42)));

        let result = codegen_return_stmt(&expr, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "return 42 ;");
    }

    #[test]
    fn test_codegen_return_stmt_none() {
        let mut ctx = create_test_context();

        let result = codegen_return_stmt(&None, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "return ;");
    }

    #[test]
    fn test_codegen_while_stmt() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let condition = HirExpr::Literal(Literal::Bool(true));
        let body = vec![HirStmt::Pass];

        let result = codegen_while_stmt(&condition, &body, &mut ctx).unwrap();
        assert!(result.to_string().contains("while true"));
    }

    #[test]
    fn test_codegen_raise_stmt_with_exception() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let exc = Some(HirExpr::Literal(Literal::String("Error".to_string())));

        let result = codegen_raise_stmt(&exc, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "return Err (\"Error\" . to_string ()) ;");
    }

    #[test]
    fn test_codegen_raise_stmt_bare() {
        let mut ctx = create_test_context();

        let result = codegen_raise_stmt(&None, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "return Err (\"Exception raised\" . into ()) ;");
    }

    #[test]
    fn test_codegen_with_stmt_with_target() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let context = HirExpr::Literal(Literal::Int(42));
        let target = Some("file".to_string());
        let body = vec![HirStmt::Pass];

        let result = codegen_with_stmt(&context, &target, &body, &mut ctx).unwrap();
        assert!(result.to_string().contains("let mut file"));
    }

    #[test]
    fn test_codegen_with_stmt_no_target() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let context = HirExpr::Literal(Literal::Int(42));
        let body = vec![HirStmt::Pass];

        let result = codegen_with_stmt(&context, &None, &body, &mut ctx).unwrap();
        assert!(result.to_string().contains("let _context"));
    }

    // Phase 3b tests - Assign handler tests
    #[test]
    fn test_codegen_assign_symbol_new_var() {
        let mut ctx = create_test_context();
        let value_expr = syn::parse_quote! { 42 };

        let result = codegen_assign_symbol("x", value_expr, None, &mut ctx).unwrap();
        assert!(result.to_string().contains("let x = 42"));
    }

    #[test]
    fn test_codegen_assign_symbol_with_type() {
        let mut ctx = create_test_context();
        let value_expr = syn::parse_quote! { 42 };
        let type_ann = Some(quote! { : i32 });

        let result = codegen_assign_symbol("x", value_expr, type_ann, &mut ctx).unwrap();
        assert!(result.to_string().contains("let x : i32 = 42"));
    }

    #[test]
    fn test_codegen_assign_symbol_existing_var() {
        let mut ctx = create_test_context();
        ctx.declare_var("x");
        let value_expr = syn::parse_quote! { 100 };

        let result = codegen_assign_symbol("x", value_expr, None, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "x = 100 ;");
    }

    #[test]
    fn test_codegen_assign_index() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let base = HirExpr::Var("dict".to_string());
        let index = HirExpr::Literal(Literal::String("key".to_string()));
        let value_expr = syn::parse_quote! { 42 };

        let result = codegen_assign_index(&base, &index, value_expr, &mut ctx).unwrap();
        assert!(result.to_string().contains("dict . insert"));
    }

    #[test]
    fn test_codegen_assign_attribute() {
        let mut ctx = create_test_context();
        let base = HirExpr::Var("obj".to_string());
        let value_expr = syn::parse_quote! { 42 };

        let result = codegen_assign_attribute(&base, "field", value_expr, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "obj . field = 42 ;");
    }

    #[test]
    fn test_codegen_assign_tuple_new_vars() {
        use crate::hir::AssignTarget;

        let mut ctx = create_test_context();
        let targets = vec![
            AssignTarget::Symbol("a".to_string()),
            AssignTarget::Symbol("b".to_string()),
        ];
        let value_expr = syn::parse_quote! { (1, 2) };

        let result = codegen_assign_tuple(&targets, value_expr, None, &mut ctx).unwrap();
        assert!(result.to_string().contains("let (a , b) = (1 , 2)"));
    }

    // Phase 3b tests - Try handler tests
    #[test]
    fn test_codegen_try_stmt_simple() {
        use crate::hir::ExceptHandler;

        let mut ctx = create_test_context();
        let body = vec![HirStmt::Pass];
        let handlers = vec![ExceptHandler {
            exception_type: None,
            name: None,
            body: vec![HirStmt::Pass],
        }];

        let result = codegen_try_stmt(&body, &handlers, &None, &mut ctx).unwrap();
        let result_str = result.to_string();
        assert!(result_str.contains("let _result"));
        assert!(result_str.contains("if let Err (_e) = _result"));
    }

    #[test]
    fn test_codegen_try_stmt_with_finally() {
        let mut ctx = create_test_context();
        let body = vec![HirStmt::Pass];
        let handlers = vec![];
        let finally = Some(vec![HirStmt::Pass]);

        let result = codegen_try_stmt(&body, &handlers, &finally, &mut ctx).unwrap();
        assert!(!result.to_string().is_empty());
    }

    #[test]
    fn test_codegen_try_stmt_except_and_finally() {
        use crate::hir::ExceptHandler;

        let mut ctx = create_test_context();
        let body = vec![HirStmt::Pass];
        let handlers = vec![ExceptHandler {
            exception_type: None,
            name: Some("e".to_string()),
            body: vec![HirStmt::Pass],
        }];
        let finally = Some(vec![HirStmt::Pass]);

        let result = codegen_try_stmt(&body, &handlers, &finally, &mut ctx).unwrap();
        let result_str = result.to_string();
        assert!(result_str.contains("let _result"));
        assert!(result_str.contains("if let Err (_e) = _result"));
    }

    // Phase 1b/1c tests - Type conversion functions (DEPYLER-0149)
    #[test]
    fn test_int_cast_conversion() {
        // Python: int(x) → Rust: x (no cast, let type inference handle it)
        // Phase 1c change: Removed cast to allow usize/i32 inference based on context
        let call_expr = HirExpr::Call {
            func: "int".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        // Should just be 'x' without cast
        assert!(code.contains("x"), "Expected 'x', got: {}", code);
        assert!(!code.contains("as"), "Should not contain cast, got: {}", code);
    }

    #[test]
    fn test_float_cast_conversion() {
        // Python: float(x) → Rust: (x) as f64
        let call_expr = HirExpr::Call {
            func: "float".to_string(),
            args: vec![HirExpr::Var("y".to_string())],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        assert!(code.contains("as f64"), "Expected '(y) as f64', got: {}", code);
    }

    #[test]
    fn test_str_conversion() {
        // Python: str(x) → Rust: x.to_string()
        let call_expr = HirExpr::Call {
            func: "str".to_string(),
            args: vec![HirExpr::Var("value".to_string())],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        assert!(code.contains("to_string"), "Expected 'value.to_string()', got: {}", code);
    }

    #[test]
    fn test_bool_cast_conversion() {
        // Python: bool(x) → Rust: (x) as bool
        let call_expr = HirExpr::Call {
            func: "bool".to_string(),
            args: vec![HirExpr::Var("flag".to_string())],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        assert!(code.contains("as bool"), "Expected '(flag) as bool', got: {}", code);
    }

    #[test]
    fn test_int_cast_with_expression() {
        // Python: int((low + high) / 2) → Rust: (low + high) / 2 (no cast)
        // Phase 1c: Integer division already works in Rust, let type inference handle it
        let division = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("low".to_string())),
                right: Box::new(HirExpr::Var("high".to_string())),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };

        let call_expr = HirExpr::Call {
            func: "int".to_string(),
            args: vec![division],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        // Should preserve the expression without cast
        assert!(code.contains("low"), "Expected 'low' variable, got: {}", code);
        assert!(code.contains("high"), "Expected 'high' variable, got: {}", code);
        assert!(!code.contains("as"), "Should not contain cast, got: {}", code);
    }

    #[test]
    fn test_float_literal_decimal_point() {
        // Regression test for DEPYLER-TBD: Ensure float literals always have decimal point
        // Bug: f64::to_string() for 0.0 produces "0" (no decimal), parsed as integer
        // Fix: Always ensure ".0" suffix for floats without decimal/exponent
        let mut ctx = create_test_context();

        // Test 0.0 → should generate "0.0" not "0"
        let zero_float = HirExpr::Literal(Literal::Float(0.0));
        let result = zero_float.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(code.contains("0.0") || code.contains("0 ."),
                "Expected '0.0' for float zero, got: {}", code);

        // Test 42.0 → should generate "42.0" not "42"
        let forty_two = HirExpr::Literal(Literal::Float(42.0));
        let result = forty_two.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(code.contains("42.0") || code.contains("42 ."),
                "Expected '42.0' for float, got: {}", code);

        // Test 1.5 → should preserve "1.5" (already has decimal)
        let one_half = HirExpr::Literal(Literal::Float(1.5));
        let result = one_half.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(code.contains("1.5"), "Expected '1.5', got: {}", code);

        // Test scientific notation: 1e10 → should preserve (has 'e')
        let scientific = HirExpr::Literal(Literal::Float(1e10));
        let result = scientific.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(code.contains("e") || code.contains("E") || code.contains("."),
                "Expected scientific notation or decimal, got: {}", code);
    }

    #[test]
    fn test_string_method_return_types() {
        // Regression test for v3.16.0 Phase 1
        // String transformation methods (.upper(), .lower(), .strip()) return owned String
        // Function signatures should reflect this: `fn f(s: &str) -> String` not `-> &str`

        // Test 1: .upper() should generate String return type
        let upper_func = HirFunction {
            name: "to_upper".to_string(),
            params: vec![HirParam::new("text".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "upper".to_string(),
                args: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = upper_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        // Should generate: fn to_upper(text: &str) -> String
        // NOT: fn to_upper<'a>(text: &'a str) -> &'a str
        assert!(code.contains("-> String"),
                "Expected '-> String' for .upper() method, got: {}", code);
        assert!(!code.contains("-> & ") && !code.contains("-> &'"),
                "Should not generate borrowed return for .upper(), got: {}", code);

        // Test 2: .lower() should also generate String return type
        let lower_func = HirFunction {
            name: "to_lower".to_string(),
            params: vec![HirParam::new("text".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "lower".to_string(),
                args: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = lower_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        assert!(code.contains("-> String"),
                "Expected '-> String' for .lower() method, got: {}", code);

        // Test 3: .strip() should also generate String return type
        let strip_func = HirFunction {
            name: "trim_text".to_string(),
            params: vec![HirParam::new("text".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "strip".to_string(),
                args: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = strip_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        assert!(code.contains("-> String"),
                "Expected '-> String' for .strip() method, got: {}", code);
    }

    #[test]
    fn test_int_float_division_semantics() {
        // Regression test for v3.16.0 Phase 2
        // Python's `/` operator always returns float, even with int operands
        // Rust's `/` does integer division with int operands
        // We need to cast to float when the context expects float

        // Test 1: int / int returning float (the main bug)
        let divide_func = HirFunction {
            name: "safe_divide".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int),
            ].into(),
            ret_type: Type::Float,  // Expects float return!
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Div,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = divide_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        // Should generate: (a as f64) / (b as f64)
        // NOT: a / b (which would do integer division)
        assert!(code.contains("as f64") || code.contains("as f32"),
                "Expected float cast for int/int division with float return, got: {}", code);
        assert!(code.contains("-> f64") || code.contains("-> f32"),
                "Expected float return type, got: {}", code);

        // Test 2: int // int returning int (floor division - should NOT cast)
        let floor_div_func = HirFunction {
            name: "floor_divide".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int),
            ].into(),
            ret_type: Type::Int,  // Expects int return
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::FloorDiv,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = floor_div_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        // Floor division should NOT add float casts
        assert!(code.contains("-> i32") || code.contains("-> i64"),
                "Expected int return type for floor division, got: {}", code);

        // Test 3: float / float should work without changes
        let float_div_func = HirFunction {
            name: "divide_floats".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Float),
                HirParam::new("b".to_string(), Type::Float),
            ].into(),
            ret_type: Type::Float,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Div,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = float_div_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        assert!(code.contains("-> f64") || code.contains("-> f32"),
                "Expected float return type, got: {}", code);
    }
}
