use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use crate::cargo_toml_gen; // DEPYLER-0384: Cargo.toml generation
use crate::hir::*;
use crate::string_optimization::StringOptimizer;
use anyhow::Result;
use quote::{quote, ToTokens};
use std::collections::{HashMap, HashSet};
use syn::{self, parse_quote};

// Module declarations for rust_gen refactoring (v3.18.0 Phases 2-7)
mod argparse_transform;
mod builtin_conversions; // DEPYLER-REFACTOR-001: Extracted from expr_gen.rs
mod collection_constructors; // DEPYLER-REFACTOR-001: Extracted from expr_gen.rs
mod context;
mod error_gen;
mod expr_gen;
mod format;
mod func_gen;
mod generator_gen;
mod import_gen;
pub mod keywords; // DEPYLER-0023: Centralized keyword escaping
mod stmt_gen;
mod type_gen;

// Internal imports
use error_gen::generate_error_type_definitions;
use format::format_rust_code;
use import_gen::process_module_imports;
#[cfg(test)]
use stmt_gen::{
    codegen_assign_attribute, codegen_assign_index, codegen_assign_symbol, codegen_assign_tuple,
    codegen_break_stmt, codegen_continue_stmt, codegen_expr_stmt, codegen_pass_stmt,
    codegen_raise_stmt, codegen_return_stmt, codegen_try_stmt, codegen_while_stmt,
    codegen_with_stmt,
};

// Public re-exports for external modules (union_enum_gen, etc.)
pub use argparse_transform::ArgParserTracker; // DEPYLER-0384: Export for testing
pub use context::{CodeGenContext, RustCodeGen, ToRustExpr};
pub use type_gen::rust_type_to_syn;

// Internal re-exports for cross-module access
pub(crate) use func_gen::return_type_expects_float;

/// Analyze functions for string optimization
///
/// Performs string optimization analysis on all functions.
/// Complexity: 2 (well within ≤10 target)
fn analyze_string_optimization(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        ctx.string_optimizer.analyze_function(func);
    }
}

/// DEPYLER-0447: Analyze function bodies AND constants to find argparse validators
///
/// Scans all statements in function bodies and constant expressions to find
/// add_argument(type=validator_func) calls. Populates ctx.validator_functions
/// with function names used as type= parameters.
/// This must run BEFORE function signature generation so parameter types can be corrected.
///
/// Complexity: 8 (func loop + const loop + stmt loop + match + expr match + kwargs loop + filter)
fn analyze_validators(
    ctx: &mut CodeGenContext,
    functions: &[HirFunction],
    constants: &[HirConstant],
) {
    // Scan function bodies
    for func in functions {
        scan_stmts_for_validators(&func.body, ctx);
    }

    // Scan constant expressions (module-level code)
    for constant in constants {
        scan_expr_for_validators(&constant.value, ctx);
    }
}

/// Helper: Recursively scan statements for add_argument(type=...) calls
fn scan_stmts_for_validators(stmts: &[HirStmt], ctx: &mut CodeGenContext) {
    for stmt in stmts {
        match stmt {
            HirStmt::Expr(expr) => {
                scan_expr_for_validators(expr, ctx);
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                scan_stmts_for_validators(then_body, ctx);
                if let Some(ref else_stmts) = else_body {
                    scan_stmts_for_validators(else_stmts, ctx);
                }
            }
            HirStmt::While { body, .. } => {
                scan_stmts_for_validators(body, ctx);
            }
            HirStmt::For { body, .. } => {
                scan_stmts_for_validators(body, ctx);
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                scan_stmts_for_validators(body, ctx);
                for handler in handlers {
                    scan_stmts_for_validators(&handler.body, ctx);
                }
                if let Some(ref else_stmts) = orelse {
                    scan_stmts_for_validators(else_stmts, ctx);
                }
                if let Some(ref final_stmts) = finalbody {
                    scan_stmts_for_validators(final_stmts, ctx);
                }
            }
            _ => {}
        }
    }
}

/// Helper: Scan expression for add_argument method calls
fn scan_expr_for_validators(expr: &HirExpr, ctx: &mut CodeGenContext) {
    match expr {
        HirExpr::MethodCall { method, kwargs, .. } if method == "add_argument" => {
            // Check for type= parameter
            for (kw_name, kw_value) in kwargs {
                if kw_name == "type" {
                    if let HirExpr::Var(type_name) = kw_value {
                        // Skip built-in types
                        if !matches!(type_name.as_str(), "str" | "int" | "float" | "Path") {
                            ctx.validator_functions.insert(type_name.clone());
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// Analyze which variables are reassigned (mutated) in a list of statements
///
/// Populates ctx.mutable_vars with variables that are:
/// 1. Reassigned after declaration (x = 1; x = 2)
/// 2. Mutated via method calls (.push(), .extend(), .insert(), .remove(), .pop(), etc.)
/// 3. DEPYLER-0312: Function parameters that are reassigned (requires mut)
///
/// Complexity: 7 (stmt loop + match + if + expr scan + method match)
fn analyze_mutable_vars(stmts: &[HirStmt], ctx: &mut CodeGenContext, params: &[HirParam]) {
    let mut declared = HashSet::new();

    // DEPYLER-0312: Pre-populate declared with function parameters
    // This allows the reassignment detection logic below to catch parameter mutations
    // Example: def gcd(a, b): a = temp  # Now detected as reassignment → mut a
    for param in params {
        declared.insert(param.name.clone());
    }

    fn analyze_expr_for_mutations(
        expr: &HirExpr,
        mutable: &mut HashSet<String>,
        var_types: &HashMap<String, String>,
        mutating_methods: &HashMap<String, HashSet<String>>,
    ) {
        match expr {
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => {
                // Check if this is a mutating method call
                let is_mut = if is_mutating_method(method) {
                    // Built-in mutating method
                    true
                } else if let HirExpr::Var(var_name) = &**object {
                    // Check if this is a user-defined mutating method
                    if let Some(class_name) = var_types.get(var_name) {
                        if let Some(mut_methods) = mutating_methods.get(class_name) {
                            mut_methods.contains(method)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                if is_mut {
                    if let HirExpr::Var(var_name) = &**object {
                        mutable.insert(var_name.clone());
                    }
                }
                // Recursively check nested expressions
                analyze_expr_for_mutations(object, mutable, var_types, mutating_methods);
                for arg in args {
                    analyze_expr_for_mutations(arg, mutable, var_types, mutating_methods);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                analyze_expr_for_mutations(left, mutable, var_types, mutating_methods);
                analyze_expr_for_mutations(right, mutable, var_types, mutating_methods);
            }
            HirExpr::Unary { operand, .. } => {
                analyze_expr_for_mutations(operand, mutable, var_types, mutating_methods);
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    analyze_expr_for_mutations(arg, mutable, var_types, mutating_methods);
                }
            }
            HirExpr::IfExpr { test, body, orelse } => {
                analyze_expr_for_mutations(test, mutable, var_types, mutating_methods);
                analyze_expr_for_mutations(body, mutable, var_types, mutating_methods);
                analyze_expr_for_mutations(orelse, mutable, var_types, mutating_methods);
            }
            HirExpr::List(items)
            | HirExpr::Tuple(items)
            | HirExpr::Set(items)
            | HirExpr::FrozenSet(items) => {
                for item in items {
                    analyze_expr_for_mutations(item, mutable, var_types, mutating_methods);
                }
            }
            HirExpr::Dict(pairs) => {
                for (key, value) in pairs {
                    analyze_expr_for_mutations(key, mutable, var_types, mutating_methods);
                    analyze_expr_for_mutations(value, mutable, var_types, mutating_methods);
                }
            }
            HirExpr::Index { base, index } => {
                analyze_expr_for_mutations(base, mutable, var_types, mutating_methods);
                analyze_expr_for_mutations(index, mutable, var_types, mutating_methods);
            }
            HirExpr::Attribute { value, .. } => {
                analyze_expr_for_mutations(value, mutable, var_types, mutating_methods);
            }
            _ => {}
        }
    }

    fn is_mutating_method(method: &str) -> bool {
        matches!(
            method,
            // List methods
            "append" | "extend" | "insert" | "remove" | "pop" | "clear" | "reverse" | "sort" |
            // Dict methods
            "update" | "setdefault" | "popitem" |
            // Set methods
            "add" | "discard" | "difference_update" | "intersection_update"
        )
    }

    fn analyze_stmt(
        stmt: &HirStmt,
        declared: &mut HashSet<String>,
        mutable: &mut HashSet<String>,
        var_types: &mut HashMap<String, String>,
        mutating_methods: &HashMap<String, HashSet<String>>,
    ) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                // Check if the value expression contains method calls that mutate variables
                analyze_expr_for_mutations(value, mutable, var_types, mutating_methods);

                match target {
                    AssignTarget::Symbol(name) => {
                        // Track variable type if assigned from class constructor
                        if let HirExpr::Call { func, .. } = value {
                            // Store the type (class name) for this variable
                            var_types.insert(name.clone(), func.clone());
                        }

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
                    AssignTarget::Attribute { value, .. } => {
                        // DEPYLER-0235 FIX: Property writes require the base object to be mutable
                        // e.g., `b.size = 20` requires `let mut b = ...`
                        if let HirExpr::Var(var_name) = value.as_ref() {
                            mutable.insert(var_name.clone());
                        }
                    }
                    AssignTarget::Index { base, .. } => {
                        // DEPYLER-0235 FIX: Index assignments also require mutability
                        // e.g., `arr[i] = value` requires `let mut arr = ...`
                        if let HirExpr::Var(var_name) = base.as_ref() {
                            mutable.insert(var_name.clone());
                        }
                    }
                }
            }
            HirStmt::Expr(expr) => {
                // Check standalone expressions for method calls (e.g., numbers.push(4))
                analyze_expr_for_mutations(expr, mutable, var_types, mutating_methods);
            }
            HirStmt::Return(Some(expr)) => {
                analyze_expr_for_mutations(expr, mutable, var_types, mutating_methods);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                analyze_expr_for_mutations(condition, mutable, var_types, mutating_methods);
                for stmt in then_body {
                    analyze_stmt(stmt, declared, mutable, var_types, mutating_methods);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        analyze_stmt(stmt, declared, mutable, var_types, mutating_methods);
                    }
                }
            }
            HirStmt::While {
                condition, body, ..
            } => {
                analyze_expr_for_mutations(condition, mutable, var_types, mutating_methods);
                for stmt in body {
                    analyze_stmt(stmt, declared, mutable, var_types, mutating_methods);
                }
            }
            HirStmt::For { body, .. } => {
                for stmt in body {
                    analyze_stmt(stmt, declared, mutable, var_types, mutating_methods);
                }
            }
            _ => {}
        }
    }

    let mut var_types = HashMap::new();
    let mutating_methods = &ctx.mutating_methods;
    for stmt in stmts {
        analyze_stmt(
            stmt,
            &mut declared,
            &mut ctx.mutable_vars,
            &mut var_types,
            mutating_methods,
        );
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
/// Deduplicate use statements to avoid E0252 errors
///
/// DEPYLER-0335 FIX #1: Multiple sources can generate the same import.
/// For example, both generate_import_tokens and generate_conditional_imports
/// might add `use std::collections::HashMap;`.
///
/// # Complexity
/// ~6 (loop + if + string ops)
fn deduplicate_use_statements(
    items: Vec<proc_macro2::TokenStream>,
) -> Vec<proc_macro2::TokenStream> {
    let mut seen = std::collections::HashSet::new();
    let mut deduped = Vec::new();

    for item in items {
        let item_str = item.to_string();
        // Only deduplicate use statements
        if item_str.starts_with("use ") {
            if seen.insert(item_str) {
                deduped.push(item);
            }
            // else: skip duplicate
        } else {
            // Non-import items: always keep
            deduped.push(item);
        }
    }

    deduped
}

fn generate_conditional_imports(ctx: &CodeGenContext) -> Vec<proc_macro2::TokenStream> {
    let mut imports = Vec::new();

    // Define all possible conditional imports
    let conditional_imports = [
        (ctx.needs_hashmap, quote! { use std::collections::HashMap; }),
        (ctx.needs_hashset, quote! { use std::collections::HashSet; }),
        (
            ctx.needs_vecdeque,
            quote! { use std::collections::VecDeque; },
        ),
        (ctx.needs_fnv_hashmap, quote! { use fnv::FnvHashMap; }),
        (ctx.needs_ahash_hashmap, quote! { use ahash::AHashMap; }),
        (ctx.needs_arc, quote! { use std::sync::Arc; }),
        (ctx.needs_rc, quote! { use std::rc::Rc; }),
        (ctx.needs_cow, quote! { use std::borrow::Cow; }),
        (ctx.needs_serde_json, quote! { use serde_json; }),
        (ctx.needs_io_read, quote! { use std::io::Read; }), // DEPYLER-0458
        (ctx.needs_io_write, quote! { use std::io::Write; }), // DEPYLER-0458
        (ctx.needs_once_cell, quote! { use once_cell::sync::Lazy; }), // DEPYLER-REARCH-001
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

    // DEPYLER-0335 FIX #1: Deduplicate imports using HashSet
    // Multiple Python imports can map to same Rust type (e.g., defaultdict + Counter -> HashMap)
    let mut seen_paths = std::collections::HashSet::new();

    // Add external imports (deduplicated)
    for import in external_imports {
        // Create unique key from path + alias
        let key = format!("{}:{:?}", import.path, import.alias);
        if !seen_paths.insert(key) {
            continue; // Skip duplicate
        }

        let path: syn::Path =
            syn::parse_str(&import.path).unwrap_or_else(|_| parse_quote! { unknown });
        if let Some(alias) = import.alias {
            let alias_ident = syn::Ident::new(&alias, proc_macro2::Span::call_site());
            items.push(quote! { use #path as #alias_ident; });
        } else {
            items.push(quote! { use #path; });
        }
    }

    // Add standard library imports (deduplicated)
    for import in std_imports {
        // Skip typing imports as they're handled by the type system
        if import.path.starts_with("::") || import.path.is_empty() {
            continue;
        }

        // Create unique key from path + alias
        let key = format!("{}:{:?}", import.path, import.alias);
        if !seen_paths.insert(key) {
            continue; // Skip duplicate
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

/// Generate module-level constant tokens
///
/// Generates `pub const` declarations for module-level constants.
/// For simple literal values (int, float, string, bool), generates const.
/// For complex expressions (Dict, List), uses once_cell::Lazy for runtime init.
///
/// # DEPYLER-REARCH-001: Phase 3.2 - Fix const initialization
/// HashMap::new() and .insert() are not const-evaluable, so we use Lazy for
/// complex collections.
fn generate_constant_tokens(
    constants: &[HirConstant],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    use crate::rust_gen::context::ToRustExpr;

    let mut items = Vec::new();

    for constant in constants {
        let name_ident = syn::Ident::new(&constant.name, proc_macro2::Span::call_site());

        // Generate the value expression
        let value_expr = constant.value.to_rust_expr(ctx)?;

        // DEPYLER-REARCH-001: Check if this value needs runtime initialization
        let needs_runtime_init = matches!(
            &constant.value,
            HirExpr::Dict(_) | HirExpr::List(_) | HirExpr::Set(_) | HirExpr::Tuple(_)
        );

        if needs_runtime_init {
            // Use once_cell::Lazy for runtime-initialized constants
            ctx.needs_once_cell = true;

            // Generate type annotation
            let type_annotation = if let Some(ref ty) = constant.type_annotation {
                let rust_type = ctx.type_mapper.map_type(ty);
                let syn_type = type_gen::rust_type_to_syn(&rust_type)?;
                quote! { #syn_type }
            } else {
                // Infer type from expression
                match &constant.value {
                    HirExpr::Dict { .. } => {
                        ctx.needs_serde_json = true;
                        quote! { serde_json::Value }
                    }
                    HirExpr::List { .. } | HirExpr::Tuple(_) | HirExpr::Set(_) => {
                        ctx.needs_serde_json = true;
                        quote! { serde_json::Value }
                    }
                    _ => {
                        ctx.needs_serde_json = true;
                        quote! { serde_json::Value }
                    }
                }
            };

            // Generate: pub static NAME: Lazy<Type> = Lazy::new(|| { ... });
            items.push(quote! {
                pub static #name_ident: once_cell::sync::Lazy<#type_annotation> = once_cell::sync::Lazy::new(|| #value_expr);
            });
        } else {
            // Simple literals can use pub const
            let type_annotation = if let Some(ref ty) = constant.type_annotation {
                let rust_type = ctx.type_mapper.map_type(ty);
                let syn_type = type_gen::rust_type_to_syn(&rust_type)?;
                quote! { : #syn_type }
            } else {
                // DEPYLER-0448: Infer type from expression (not just literals)
                match &constant.value {
                    // Literal types
                    HirExpr::Literal(Literal::Int(_)) => quote! { : i32 },
                    HirExpr::Literal(Literal::Float(_)) => quote! { : f64 },
                    HirExpr::Literal(Literal::String(_)) => quote! { : &str },
                    HirExpr::Literal(Literal::Bool(_)) => quote! { : bool },

                    // Default fallback
                    _ => {
                        ctx.needs_serde_json = true;
                        quote! { : serde_json::Value }
                    }
                }
            };

            // Generate: pub const NAME: Type = value;
            items.push(quote! {
                pub const #name_ident #type_annotation = #value_expr;
            });
        }
    }

    Ok(items)
}

/// Generate a complete Rust file from HIR module
pub fn generate_rust_file(
    module: &HirModule,
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<(String, Vec<cargo_toml_gen::Dependency>)> {
    let module_mapper = crate::module_mapper::ModuleMapper::new();

    // Process imports to populate the context
    let (imported_modules, imported_items) =
        process_module_imports(&module.imports, &module_mapper);

    // DEPYLER-0490/0491: Set needs_* flags based on imported modules AND items
    // This ensures Cargo.toml dependencies are generated for external crates
    // Check both whole module imports (import X) and specific imports (from X import Y)
    let needs_chrono = imported_modules.contains_key("datetime")
        || imported_items
            .values()
            .any(|path| path.starts_with("chrono::"));
    let needs_tempfile = imported_modules.contains_key("tempfile")
        || imported_items
            .values()
            .any(|path| path.starts_with("tempfile::"));
    let needs_itertools = imported_modules.contains_key("itertools")
        || imported_items
            .values()
            .any(|path| path.starts_with("itertools::"));

    // Extract class names from module (DEPYLER-0230: distinguish user classes from builtins)
    let class_names: HashSet<String> = module
        .classes
        .iter()
        .map(|class| class.name.clone())
        .collect();

    // DEPYLER-0231: Build map of mutating methods (class_name -> set of method names)
    let mut mutating_methods: std::collections::HashMap<String, HashSet<String>> =
        std::collections::HashMap::new();
    for class in &module.classes {
        let mut mut_methods = HashSet::new();
        for method in &class.methods {
            if crate::direct_rules::method_mutates_self(method) {
                mut_methods.insert(method.name.clone());
            }
        }
        mutating_methods.insert(class.name.clone(), mut_methods);
    }

    let mut ctx = CodeGenContext {
        type_mapper,
        annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(type_mapper.clone()),
        string_optimizer: StringOptimizer::new(),
        union_enum_generator: crate::union_enum_gen::UnionEnumGenerator::new(),
        generated_enums: Vec::new(),
        needs_hashmap: false,
        needs_hashset: false,
        needs_vecdeque: false,
        needs_fnv_hashmap: false,
        needs_ahash_hashmap: false,
        needs_arc: false,
        needs_rc: false,
        needs_cow: false,
        needs_rand: false,
        needs_serde_json: false,
        needs_regex: false,
        needs_chrono,    // DEPYLER-0490: Set from imports
        needs_tempfile,  // DEPYLER-0490: Set from imports
        needs_itertools, // DEPYLER-0490: Set from imports
        needs_clap: false,
        needs_csv: false,
        needs_rust_decimal: false,
        needs_num_rational: false,
        needs_base64: false,
        needs_md5: false,
        needs_sha2: false,
        needs_sha3: false,
        needs_blake2: false,
        needs_hex: false,
        needs_uuid: false,
        needs_hmac: false,
        needs_crc32: false,
        needs_url_encoding: false,
        needs_io_read: false,   // DEPYLER-0458
        needs_io_write: false,  // DEPYLER-0458
        needs_once_cell: false, // DEPYLER-REARCH-001
        declared_vars: vec![HashSet::new()],
        current_function_can_fail: false,
        current_return_type: None,
        module_mapper,
        imported_modules,
        imported_items,
        mutable_vars: HashSet::new(),
        needs_zerodivisionerror: false,
        needs_indexerror: false,
        needs_valueerror: false,
        needs_argumenttypeerror: false,
        in_generator: false,
        is_classmethod: false,
        generator_state_vars: HashSet::new(),
        var_types: std::collections::HashMap::new(),
        class_names,
        mutating_methods,
        function_return_types: std::collections::HashMap::new(), // DEPYLER-0269: Track function return types
        function_param_borrows: std::collections::HashMap::new(), // DEPYLER-0270: Track parameter borrowing
        tuple_iter_vars: HashSet::new(), // DEPYLER-0307 Fix #9: Track tuple iteration variables
        is_final_statement: false, // DEPYLER-0271: Track final statement for expression-based returns
        result_bool_functions: HashSet::new(), // DEPYLER-0308: Track functions returning Result<bool>
        result_returning_functions: HashSet::new(), // DEPYLER-0270: Track ALL Result-returning functions
        option_returning_functions: HashSet::new(), // DEPYLER-0497: Track functions returning Option<T>
        current_error_type: None, // DEPYLER-0310: Track error type for raise statement wrapping
        exception_scopes: Vec::new(), // DEPYLER-0333: Exception scope tracking stack
        argparser_tracker: argparse_transform::ArgParserTracker::new(), // DEPYLER-0363: Track ArgumentParser patterns
        generated_args_struct: None, // DEPYLER-0424: Args struct (hoisted to module level)
        generated_commands_enum: None, // DEPYLER-0424: Commands enum (hoisted to module level)
        current_subcommand_fields: None, // DEPYLER-0425: Subcommand field extraction
        validator_functions: HashSet::new(), // DEPYLER-0447: Track argparse validator functions
        in_json_context: false,      // DEPYLER-0461: Track json!() macro context for nested dicts
        stdlib_mappings: crate::stdlib_mappings::StdlibMappings::new(), // DEPYLER-0452: Stdlib API mappings
        hoisted_inference_vars: HashSet::new(), // DEPYLER-0455 Bug 2: Track hoisted variables needing String normalization
        cse_subcommand_temps: std::collections::HashMap::new(), // DEPYLER-0456 Bug #2: Track CSE subcommand temps
        nested_function_params: std::collections::HashMap::new(), // GH-70: Track inferred nested function params
    };

    // Analyze all functions first for string optimization
    analyze_string_optimization(&mut ctx, &module.functions);

    // Finalize interned string names (resolve collisions)
    ctx.string_optimizer.finalize_interned_names();

    // DEPYLER-0447: Scan all function bodies and constants for argparse validators
    // Must run BEFORE function conversion so validator parameter types are correct
    analyze_validators(&mut ctx, &module.functions, &module.constants);

    // DEPYLER-0270: Populate Result-returning functions map
    // All functions that can_fail return Result<T, E> and need unwrapping at call sites
    for func in &module.functions {
        if func.properties.can_fail {
            ctx.result_returning_functions.insert(func.name.clone());
        }
    }

    // DEPYLER-0308: Populate Result<bool> functions map
    // Functions that can_fail and return Bool need unwrapping in boolean contexts
    for func in &module.functions {
        if func.properties.can_fail && matches!(func.ret_type, Type::Bool) {
            ctx.result_bool_functions.insert(func.name.clone());
        }
    }

    // DEPYLER-0497: Populate Option-returning functions map and function return types
    // Functions that return Option<T> need unwrapping in format! and other Display contexts
    for func in &module.functions {
        // Store all function return types for type tracking
        ctx.function_return_types
            .insert(func.name.clone(), func.ret_type.clone());

        // Track Option-returning functions specifically
        if matches!(func.ret_type, Type::Optional(_)) {
            ctx.option_returning_functions.insert(func.name.clone());
        }
    }

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

    // Add module-level constants
    items.extend(generate_constant_tokens(&module.constants, &mut ctx)?);

    // Add collection imports if needed
    items.extend(generate_conditional_imports(&ctx));

    // DEPYLER-0335 FIX #1: Deduplicate imports across all sources
    // Both generate_import_tokens and generate_conditional_imports can add HashMap
    items = deduplicate_use_statements(items);

    // Add error type definitions if needed
    items.extend(generate_error_type_definitions(&ctx));

    // Add generated union enums
    items.extend(ctx.generated_enums.clone());

    // Add classes
    items.extend(classes);

    // DEPYLER-0424: Add ArgumentParser-generated structs at module level
    // (before functions so handler functions can reference Args type)
    if let Some(ref commands_enum) = ctx.generated_commands_enum {
        items.push(commands_enum.clone());
    }
    if let Some(ref args_struct) = ctx.generated_args_struct {
        items.push(args_struct.clone());
    }

    // Add all functions
    items.extend(functions);

    // Generate tests for all functions in a single test module
    // DEPYLER-0280 FIX: Use generate_tests_module() to create a single `mod tests {}` block
    // instead of one per function, which caused "the name `tests` is defined multiple times" errors
    let test_gen = crate::test_generation::TestGenerator::new(Default::default());
    if let Some(test_module) = test_gen.generate_tests_module(&module.functions)? {
        items.push(test_module);
    }

    let file = quote! {
        #(#items)*
    };

    // DEPYLER-0384: Extract dependencies from context (BEFORE post-processing)
    let mut dependencies = cargo_toml_gen::extract_dependencies(&ctx);

    // Format the code first (this is when tokens become readable strings)
    let mut formatted_code = format_rust_code(file.to_string());

    // DEPYLER-0393: Post-process FORMATTED code to detect missed dependencies
    // TokenStreams don't have literal strings - must scan AFTER formatting
    if formatted_code.contains("serde_json::") && !ctx.needs_serde_json {
        // Add missing import at the beginning
        formatted_code = format!("use serde_json;\n{}", formatted_code);
        // Add missing Cargo.toml dependencies
        dependencies.push(cargo_toml_gen::Dependency::new("serde_json", "1.0"));
        dependencies.push(
            cargo_toml_gen::Dependency::new("serde", "1.0")
                .with_features(vec!["derive".to_string()]),
        );
        // Re-format to ensure imports are properly ordered
        formatted_code = format_rust_code(formatted_code);
    }

    Ok((formatted_code, dependencies))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
    use crate::rust_gen::context::RustCodeGen;
    use crate::rust_gen::type_gen::convert_binop;
    use crate::type_mapper::TypeMapper;
    use depyler_annotations::TranspilationAnnotations;
    use std::collections::HashSet;

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
            needs_vecdeque: false,
            needs_fnv_hashmap: false,
            needs_ahash_hashmap: false,
            needs_arc: false,
            needs_rc: false,
            needs_cow: false,
            needs_rand: false,
            needs_serde_json: false,
            needs_regex: false,
            needs_chrono: false,
            needs_tempfile: false,  // DEPYLER-0493
            needs_itertools: false, // DEPYLER-0493
            needs_clap: false,
            needs_csv: false,
            needs_rust_decimal: false,
            needs_num_rational: false,
            needs_base64: false,
            needs_md5: false,
            needs_sha2: false,
            needs_sha3: false,
            needs_blake2: false,
            needs_hex: false,
            needs_uuid: false,
            needs_hmac: false,
            needs_crc32: false,
            needs_url_encoding: false,
            needs_io_read: false,   // DEPYLER-0458
            needs_io_write: false,  // DEPYLER-0458
            needs_once_cell: false, // DEPYLER-REARCH-001
            declared_vars: vec![HashSet::new()],
            current_function_can_fail: false,
            current_return_type: None,
            module_mapper: crate::module_mapper::ModuleMapper::new(),
            imported_modules: std::collections::HashMap::new(),
            imported_items: std::collections::HashMap::new(),
            mutable_vars: HashSet::new(),
            needs_zerodivisionerror: false,
            needs_indexerror: false,
            needs_valueerror: false,
            needs_argumenttypeerror: false,
            is_classmethod: false,
            in_generator: false,
            generator_state_vars: HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            function_return_types: std::collections::HashMap::new(), // DEPYLER-0269: Track function return types
            function_param_borrows: std::collections::HashMap::new(), // DEPYLER-0270: Track parameter borrowing
            tuple_iter_vars: HashSet::new(), // DEPYLER-0307 Fix #9: Track tuple iteration variables
            is_final_statement: false, // DEPYLER-0271: Track final statement for expression-based returns
            result_bool_functions: HashSet::new(), // DEPYLER-0308: Track functions returning Result<bool>
            result_returning_functions: HashSet::new(), // DEPYLER-0270: Track ALL Result-returning functions
            option_returning_functions: HashSet::new(), // DEPYLER-0497: Track functions returning Option<T>
            current_error_type: None, // DEPYLER-0310: Track error type for raise statement wrapping
            exception_scopes: Vec::new(), // DEPYLER-0333: Exception scope tracking stack
            argparser_tracker: argparse_transform::ArgParserTracker::new(), // DEPYLER-0363: Track ArgumentParser patterns
            generated_args_struct: None, // DEPYLER-0424: Args struct (hoisted to module level)
            generated_commands_enum: None, // DEPYLER-0424: Commands enum (hoisted to module level)
            current_subcommand_fields: None, // DEPYLER-0425: Subcommand field extraction
            validator_functions: HashSet::new(), // DEPYLER-0447: Track argparse validator functions
            in_json_context: false, // DEPYLER-0461: Track json!() macro context for nested dicts
            stdlib_mappings: crate::stdlib_mappings::StdlibMappings::new(), // DEPYLER-0452
            hoisted_inference_vars: HashSet::new(), // DEPYLER-0455 Bug 2: Track hoisted variables needing String normalization
            cse_subcommand_temps: std::collections::HashMap::new(), // DEPYLER-0456 Bug #2: Track CSE subcommand temps
            nested_function_params: std::collections::HashMap::new(), // GH-70: Track inferred nested function params
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
        // DEPYLER-0271: Final return statements use expression-based returns (no `return` keyword)
        // The function body should contain the expression result without explicit `return`
        assert!(
            code.contains("a + b"),
            "Function should contain expression 'a + b'"
        );
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
        ctx.current_function_can_fail = true; // Function returns Result, so raise becomes return Err
        let exc = Some(HirExpr::Literal(Literal::String("Error".to_string())));

        let result = codegen_raise_stmt(&exc, &mut ctx).unwrap();
        assert_eq!(
            result.to_string(),
            "return Err (\"Error\" . to_string ()) ;"
        );
    }

    #[test]
    fn test_codegen_raise_stmt_bare() {
        let mut ctx = create_test_context();
        ctx.current_function_can_fail = true; // Function returns Result, so raise becomes return Err

        let result = codegen_raise_stmt(&None, &mut ctx).unwrap();
        assert_eq!(
            result.to_string(),
            "return Err (\"Exception raised\" . into ()) ;"
        );
    }

    // NOTE: With statement with target incomplete - requires full implementation (tracked in DEPYLER-0424)
    // This test was written ahead of implementation (aspirational test)
    // Tracked in roadmap: Complete with statement target binding support
    #[test]
    #[ignore = "Incomplete feature: With statement target binding not yet implemented"]
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

        let result = codegen_assign_symbol("x", value_expr, None, false, &mut ctx).unwrap();
        assert!(result.to_string().contains("let x = 42"));
    }

    #[test]
    fn test_codegen_assign_symbol_with_type() {
        let mut ctx = create_test_context();
        let value_expr = syn::parse_quote! { 42 };
        let type_ann = Some(quote! { : i32 });

        let result = codegen_assign_symbol("x", value_expr, type_ann, false, &mut ctx).unwrap();
        assert!(result.to_string().contains("let x : i32 = 42"));
    }

    #[test]
    fn test_codegen_assign_symbol_existing_var() {
        let mut ctx = create_test_context();
        ctx.declare_var("x");
        let value_expr = syn::parse_quote! { 100 };

        let result = codegen_assign_symbol("x", value_expr, None, false, &mut ctx).unwrap();
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
        // DEPYLER-0257 REFACTOR v3: Simplified try/except (no Result wrapper)
        // Just executes try block statements directly
        assert!(!result_str.is_empty(), "Should generate code");
        // Code should be simple block execution (no complex patterns for now)
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
        // DEPYLER-0257 REFACTOR v3: Simplified try/except with finally
        // Executes try block then finally block
        assert!(!result_str.is_empty(), "Should generate code");
        // Code should execute try block and finally block
    }

    // Phase 1b/1c tests - Type conversion functions (DEPYLER-0149, DEPYLER-0216)
    #[test]
    fn test_int_cast_conversion() {
        // DEPYLER-0216 FIX: Python: int(x) → Rust: (x) as i32 (always cast variables)
        // Previous behavior (no cast) caused "cannot add bool to bool" errors
        // when x is a bool variable: int(flag1) + int(flag2) → flag1 + flag2 (ERROR!)
        let call_expr = HirExpr::Call {
            func: "int".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        // Should generate cast for variables to prevent bool arithmetic errors
        assert!(code.contains("x"), "Expected 'x', got: {}", code);
        assert!(
            code.contains("as i32"),
            "Should contain 'as i32' cast, got: {}",
            code
        );
    }

    #[test]
    fn test_float_cast_conversion() {
        // Python: float(x) → Rust: (x) as f64
        let call_expr = HirExpr::Call {
            func: "float".to_string(),
            args: vec![HirExpr::Var("y".to_string())],
            kwargs: vec![],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        assert!(
            code.contains("as f64"),
            "Expected '(y) as f64', got: {}",
            code
        );
    }

    #[test]
    fn test_str_conversion() {
        // Python: str(x) → Rust: x.to_string()
        let call_expr = HirExpr::Call {
            func: "str".to_string(),
            args: vec![HirExpr::Var("value".to_string())],
            kwargs: vec![],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        assert!(
            code.contains("to_string"),
            "Expected 'value.to_string()', got: {}",
            code
        );
    }

    // NOTE: Boolean casting incomplete - requires type cast implementation (tracked in DEPYLER-0424)
    // This test was written ahead of implementation (aspirational test)
    // Tracked in roadmap: Implement bool() builtin casting
    #[test]
    #[ignore = "Incomplete feature: bool() casting not yet implemented"]
    fn test_bool_cast_conversion() {
        // Python: bool(x) → Rust: (x) as bool
        let call_expr = HirExpr::Call {
            func: "bool".to_string(),
            args: vec![HirExpr::Var("flag".to_string())],
            kwargs: vec![],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        assert!(
            code.contains("as bool"),
            "Expected '(flag) as bool', got: {}",
            code
        );
    }

    #[test]
    fn test_int_cast_with_expression() {
        // DEPYLER-0216 FIX: Python: int((low + high) / 2) → Rust: ((low + high) / 2) as i32
        // Previous behavior (no cast) caused "cannot add bool to bool" errors
        // when expression might be bool: int(x > 0) + int(y > 0) → (x > 0) + (y > 0) (ERROR!)
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
            kwargs: vec![],
        };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        // Should generate cast for expressions to prevent bool arithmetic errors
        assert!(
            code.contains("low"),
            "Expected 'low' variable, got: {}",
            code
        );
        assert!(
            code.contains("high"),
            "Expected 'high' variable, got: {}",
            code
        );
        assert!(
            code.contains("as i32"),
            "Should contain 'as i32' cast, got: {}",
            code
        );
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
        assert!(
            code.contains("0.0") || code.contains("0 ."),
            "Expected '0.0' for float zero, got: {}",
            code
        );

        // Test 42.0 → should generate "42.0" not "42"
        let forty_two = HirExpr::Literal(Literal::Float(42.0));
        let result = forty_two.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(
            code.contains("42.0") || code.contains("42 ."),
            "Expected '42.0' for float, got: {}",
            code
        );

        // Test 1.5 → should preserve "1.5" (already has decimal)
        let one_half = HirExpr::Literal(Literal::Float(1.5));
        let result = one_half.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(code.contains("1.5"), "Expected '1.5', got: {}", code);

        // Test scientific notation: 1e10 → should preserve (has 'e')
        let scientific = HirExpr::Literal(Literal::Float(1e10));
        let result = scientific.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();
        assert!(
            code.contains("e") || code.contains("E") || code.contains("."),
            "Expected scientific notation or decimal, got: {}",
            code
        );
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
                kwargs: vec![],
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
        assert!(
            code.contains("-> String"),
            "Expected '-> String' for .upper() method, got: {}",
            code
        );
        assert!(
            !code.contains("-> & ") && !code.contains("-> &'"),
            "Should not generate borrowed return for .upper(), got: {}",
            code
        );

        // Test 2: .lower() should also generate String return type
        let lower_func = HirFunction {
            name: "to_lower".to_string(),
            params: vec![HirParam::new("text".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "lower".to_string(),
                args: vec![],
                kwargs: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = lower_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        assert!(
            code.contains("-> String"),
            "Expected '-> String' for .lower() method, got: {}",
            code
        );

        // Test 3: .strip() should also generate String return type
        let strip_func = HirFunction {
            name: "trim_text".to_string(),
            params: vec![HirParam::new("text".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("text".to_string())),
                method: "strip".to_string(),
                args: vec![],
                kwargs: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let result = strip_func.to_rust_tokens(&mut ctx).unwrap();
        let code = result.to_string();

        assert!(
            code.contains("-> String"),
            "Expected '-> String' for .strip() method, got: {}",
            code
        );
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
            ]
            .into(),
            ret_type: Type::Float, // Expects float return!
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
        assert!(
            code.contains("as f64") || code.contains("as f32"),
            "Expected float cast for int/int division with float return, got: {}",
            code
        );
        assert!(
            code.contains("-> f64") || code.contains("-> f32"),
            "Expected float return type, got: {}",
            code
        );

        // Test 2: int // int returning int (floor division - should NOT cast)
        let floor_div_func = HirFunction {
            name: "floor_divide".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int),
            ]
            .into(),
            ret_type: Type::Int, // Expects int return
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
        assert!(
            code.contains("-> i32") || code.contains("-> i64"),
            "Expected int return type for floor division, got: {}",
            code
        );

        // Test 3: float / float should work without changes
        let float_div_func = HirFunction {
            name: "divide_floats".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Float),
                HirParam::new("b".to_string(), Type::Float),
            ]
            .into(),
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

        assert!(
            code.contains("-> f64") || code.contains("-> f32"),
            "Expected float return type, got: {}",
            code
        );
    }
}
