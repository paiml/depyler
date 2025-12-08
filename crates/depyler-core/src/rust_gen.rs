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
mod array_initialization; // DEPYLER-REFACTOR-001: Extracted from expr_gen.rs
mod builtin_conversions; // DEPYLER-REFACTOR-001: Extracted from expr_gen.rs
pub mod numpy_gen; // Phase 3: NumPy→Trueno codegen
mod collection_constructors; // DEPYLER-REFACTOR-001: Extracted from expr_gen.rs
mod context;
mod error_gen;
mod expr_gen;
mod format;
pub mod func_gen; // DEPYLER-0518: Made public for type inference from lifetime_analysis
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
pub use argparse_transform::{
    ArgParserArgument, ArgParserInfo, SubcommandInfo, SubparserInfo,
    generate_args_struct, generate_commands_enum,
}; // Coverage tests
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
pub(crate) fn analyze_mutable_vars(stmts: &[HirStmt], ctx: &mut CodeGenContext, params: &[HirParam]) {
    // DEPYLER-0707: Clear mutable_vars before analyzing each function
    // Without this, variables from previous functions leak to subsequent ones,
    // causing false positives (e.g., `p` in test_point() leaking to test_person())
    ctx.mutable_vars.clear();

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
            "add" | "discard" | "difference_update" | "intersection_update" |
            // DEPYLER-0529: File I/O methods that require mutable access
            "write" | "write_all" | "writelines" | "flush" | "seek" | "truncate" |
            // DEPYLER-0549: CSV reader/writer methods that require mutable access
            // csv::Reader requires &mut self for headers(), records(), deserialize()
            // csv::Writer requires &mut self for write_record(), serialize()
            "headers" | "records" | "deserialize" | "serialize" | "write_record" |
            "writeheader" | "writerow"
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

                        // DEPYLER-0549: Mark csv readers/writers as mutable
                        // In Rust, csv::Reader and csv::Writer require &mut self for most operations
                        // Detection: variable names or call patterns involving csv/reader/writer
                        let needs_csv_mut =
                            if let HirExpr::MethodCall { object, method, .. } = value {
                                // csv.DictReader() or csv.reader()
                                if let HirExpr::Var(module) = object.as_ref() {
                                    module == "csv"
                                        && (method.contains("Reader")
                                            || method.contains("reader")
                                            || method.contains("Writer")
                                            || method.contains("writer"))
                                } else {
                                    false
                                }
                            } else if let HirExpr::Call { func, .. } = value {
                                // DictReader(f) or csv.ReaderBuilder...
                                func.contains("Reader")
                                    || func.contains("Writer")
                                    || func.contains("reader")
                                    || func.contains("writer")
                            } else {
                                // Name heuristic: variables named reader/writer
                                name == "reader"
                                    || name == "writer"
                                    || name.contains("reader")
                                    || name.contains("writer")
                            };

                        if needs_csv_mut {
                            mutable.insert(name.clone());
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
            // DEPYLER-0549: Handle WITH statements - analyze body for mutations
            HirStmt::With { body, .. } => {
                for stmt in body {
                    analyze_stmt(stmt, declared, mutable, var_types, mutating_methods);
                }
            }
            // DEPYLER-0549: Handle Try - analyze all branches
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
                ..
            } => {
                for stmt in body {
                    analyze_stmt(stmt, declared, mutable, var_types, mutating_methods);
                }
                for handler in handlers {
                    for stmt in &handler.body {
                        analyze_stmt(stmt, declared, mutable, var_types, mutating_methods);
                    }
                }
                if let Some(else_stmts) = orelse {
                    for stmt in else_stmts {
                        analyze_stmt(stmt, declared, mutable, var_types, mutating_methods);
                    }
                }
                if let Some(final_stmts) = finalbody {
                    for stmt in final_stmts {
                        analyze_stmt(stmt, declared, mutable, var_types, mutating_methods);
                    }
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
    vararg_functions: &std::collections::HashSet<String>, // DEPYLER-0648: Track vararg functions
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut class_items = Vec::new();
    for class in classes {
        // DEPYLER-0648: Pass vararg_functions for proper call site generation in methods
        let items = crate::direct_rules::convert_class_to_struct(class, type_mapper, vararg_functions)?;
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
        (ctx.needs_base64, quote! { use base64::Engine; }), // DEPYLER-0664: Engine trait needed for .encode()/.decode() methods
        (ctx.needs_io_read, quote! { use std::io::Read; }), // DEPYLER-0458
        (ctx.needs_io_write, quote! { use std::io::Write; }), // DEPYLER-0458
        (ctx.needs_bufread, quote! { use std::io::BufRead; }), // DEPYLER-0522
        (ctx.needs_once_cell, quote! { use once_cell::sync::Lazy; }), // DEPYLER-REARCH-001
        (ctx.needs_trueno, quote! { use trueno::Vector; }), // Phase 3: NumPy→Trueno
    ];

    // Add imports where needed
    for (needed, import_tokens) in conditional_imports {
        if needed {
            imports.push(import_tokens);
        }
    }

    imports
}

/// DEPYLER-197: Generate Rust type aliases from Python type aliases
///
/// Maps Python type aliases like `EventHandler = Callable[[str], None]`
/// to Rust type aliases like `type EventHandler = Box<dyn Fn(String)>;`
///
/// # Arguments
/// * `type_aliases` - Vector of TypeAlias structs from HIR
/// * `type_mapper` - TypeMapper for converting Python types to Rust types
///
/// # Returns
/// Vector of TokenStreams containing type alias declarations
fn generate_type_alias_tokens(
    type_aliases: &[TypeAlias],
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Vec<proc_macro2::TokenStream> {
    let mut items = Vec::new();

    for type_alias in type_aliases {
        // Get the Rust type for this alias
        let rust_type = type_mapper.map_type(&type_alias.target_type);
        let target_type = match type_gen::rust_type_to_syn(&rust_type) {
            Ok(ty) => ty,
            Err(_) => continue, // Skip if type conversion fails
        };

        // Create identifier for the alias name
        let alias_name = if keywords::is_rust_keyword(&type_alias.name) {
            syn::Ident::new_raw(&type_alias.name, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(&type_alias.name, proc_macro2::Span::call_site())
        };

        // Generate either a newtype struct or a type alias
        let alias_item = if type_alias.is_newtype {
            // Generate a NewType struct: pub struct UserId(pub i32);
            quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub struct #alias_name(pub #target_type);
            }
        } else {
            // Generate a type alias: pub type UserId = i32;
            quote! {
                pub type #alias_name = #target_type;
            }
        };

        items.push(alias_item);
    }

    items
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

        // DEPYLER-0593: Skip os/os.path module aliases
        // These modules are handled specially in expr_gen.rs (try_convert_os_path_method)
        // Generating `use std as os;` breaks the module recognition
        // DEPYLER-0691: Also skip sys module aliases since we use fully qualified std::env, std::process paths
        if import.alias.as_deref() == Some("os")
            || import.alias.as_deref() == Some("os_path")
            || import.alias.as_deref() == Some("sys")
        {
            continue;
        }

        // Create unique key from path + alias
        let key = format!("{}:{:?}", import.path, import.alias);
        if !seen_paths.insert(key) {
            continue; // Skip duplicate
        }

        // DEPYLER-0702: Skip struct method imports that can't be valid `use` statements
        // e.g., `from os.path import join` maps to `std::path::Path::join` which is invalid
        // because Path is a struct, not a module. These are handled at call site.
        // DEPYLER-0721: Also skip bare struct types like `std::path::Path` for inline-handled
        // functions (splitext, normpath, etc.) that don't need imports
        // DEPYLER-0771: Skip std::f64::isqrt - it doesn't exist; handled inline in expr_gen.rs
        // Also skip any path ending with ::isqrt since Rust has no such function in std::f64
        if import.path.contains("::Path::")
            || import.path.contains("::File::")
            || import.path.ends_with("::Path")
            || import.path == "std::f64::isqrt"
            || import.path.ends_with("::isqrt")
        {
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

/// Generate a single runtime-initialized constant (Lazy)
///
/// Used for complex constants like Dict/List that need runtime initialization.
/// Complexity: 6 (nested if-else with match arms)
fn generate_lazy_constant(
    constant: &HirConstant,
    name_ident: syn::Ident,
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    ctx.needs_once_cell = true;

    let type_annotation = if let Some(ref ty) = constant.type_annotation {
        let rust_type = ctx.type_mapper.map_type(ty);
        let syn_type = type_gen::rust_type_to_syn(&rust_type)?;
        quote! { #syn_type }
    } else {
        // DEPYLER-0107: Infer type from value expression
        infer_lazy_constant_type(&constant.value, ctx)
    };

    // DEPYLER-0107: Dict/List literals return HashMap/Vec, convert to Value type
    // DEPYLER-0714: Function calls may return Result, unwrap them
    let final_expr = if constant.type_annotation.is_none() {
        match &constant.value {
            HirExpr::Dict(_) | HirExpr::List(_) => {
                ctx.needs_serde_json = true;
                quote! { serde_json::to_value(#value_expr).unwrap() }
            }
            HirExpr::Call { .. } => {
                // DEPYLER-0714: Function calls may return Result - unwrap them
                // Python semantics expect the value, not Result
                quote! { #value_expr.unwrap() }
            }
            _ => quote! { #value_expr }
        }
    } else {
        quote! { #value_expr }
    };

    Ok(quote! {
        pub static #name_ident: once_cell::sync::Lazy<#type_annotation> = once_cell::sync::Lazy::new(|| #final_expr);
    })
}

/// DEPYLER-0107: Infer type for Lazy constants based on value expression
///
/// Most complex constants default to serde_json::Value for compatibility.
/// DEPYLER-0188: Path expressions return std::path::PathBuf.
/// DEPYLER-0714: Function calls use the function's return type (unwrapped if Result).
fn infer_lazy_constant_type(
    value: &HirExpr,
    ctx: &mut CodeGenContext,
) -> proc_macro2::TokenStream {
    // DEPYLER-0188: Path expressions should be typed as PathBuf
    if is_path_constant_expr(value) {
        return quote! { std::path::PathBuf };
    }

    // DEPYLER-0714: Function calls - look up return type
    // For Unknown return types, fall through to serde_json::Value default
    if let HirExpr::Call { func, .. } = value {
        if let Some(ret_type) = ctx.function_return_types.get(func) {
            // Skip Unknown - fall through to default
            if !matches!(ret_type, crate::hir::Type::Unknown) {
                if let Ok(syn_type) = type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(ret_type)) {
                    return quote! { #syn_type };
                }
            }
        }
    }

    // Default: use serde_json::Value for Lazy constants
    ctx.needs_serde_json = true;
    quote! { serde_json::Value }
}

/// Generate a single simple constant (pub const)
///
/// Used for literals and simple expressions that can be const-evaluated.
/// Complexity: 4 (if-else with helper call)
///
/// DEPYLER-0599: Fixed string literal const type mismatch.
/// String literals at module level should be `&str` without `.to_string()`.
fn generate_simple_constant(
    constant: &HirConstant,
    name_ident: syn::Ident,
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let type_annotation = if let Some(ref ty) = constant.type_annotation {
        // DEPYLER-0714: Skip Unknown type annotation - would generate TypeParam("T")
        // which is undefined. Fall through to inference to get proper type.
        if matches!(ty, crate::hir::Type::Unknown) {
            infer_constant_type(&constant.value, ctx)
        } else {
            let rust_type = ctx.type_mapper.map_type(ty);
            let syn_type = type_gen::rust_type_to_syn(&rust_type)?;
            quote! { : #syn_type }
        }
    } else {
        infer_constant_type(&constant.value, ctx)
    };

    // DEPYLER-0599: For string literals assigned to const, use raw literal (no .to_string())
    // The string optimizer may have added .to_string() but for const &str we need the bare literal
    let final_value_expr = if let HirExpr::Literal(Literal::String(s)) = &constant.value {
        // Generate raw string literal for const &str
        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
        syn::parse_quote! { #lit }
    } else {
        value_expr
    };

    Ok(quote! {
        pub const #name_ident #type_annotation = #final_value_expr;
    })
}

/// DEPYLER-0516: Infer type annotation for constant expression
///
/// Determines the Rust type for module-level constant expressions.
/// Complexity: 7 (match with 6 arms + default)
fn infer_constant_type(value: &HirExpr, ctx: &mut CodeGenContext) -> proc_macro2::TokenStream {
    match value {
        // Literal types
        HirExpr::Literal(Literal::Int(_)) => quote! { : i32 },
        HirExpr::Literal(Literal::Float(_)) => quote! { : f64 },
        HirExpr::Literal(Literal::String(_)) => quote! { : &str },
        HirExpr::Literal(Literal::Bool(_)) => quote! { : bool },

        // DEPYLER-0516: Unary operations preserve type (helper extracts unary logic)
        HirExpr::Unary { op, operand } => infer_unary_type(op, operand, ctx),

        // DEPYLER-0188: Path expressions should be typed as PathBuf
        // Detect Path() calls, .parent, .join method chains, and path / segment division
        _ if is_path_constant_expr(value) => {
            quote! { : std::path::PathBuf }
        }

        // DEPYLER-0713: Function calls - look up return type from function signatures
        // This prevents fallback to serde_json::Value for typed function results
        HirExpr::Call { func, .. } => {
            if let Some(ret_type) = ctx.function_return_types.get(func) {
                // DEPYLER-0714: Skip Unknown return type - would generate TypeParam("T")
                // Fall through to inference instead
                if matches!(ret_type, crate::hir::Type::Unknown) {
                    ctx.needs_serde_json = true;
                    quote! { : serde_json::Value }
                } else {
                    // Use the function's return type
                    match type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(ret_type)) {
                        Ok(syn_type) => quote! { : #syn_type },
                        Err(_) => {
                            ctx.needs_serde_json = true;
                            quote! { : serde_json::Value }
                        }
                    }
                }
            } else {
                // DEPYLER-0713: Try infer_expr_type_simple for builtin calls
                let inferred = func_gen::infer_expr_type_simple(value);
                if !matches!(inferred, crate::hir::Type::Unknown) {
                    match type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(&inferred)) {
                        Ok(syn_type) => quote! { : #syn_type },
                        Err(_) => {
                            ctx.needs_serde_json = true;
                            quote! { : serde_json::Value }
                        }
                    }
                } else {
                    ctx.needs_serde_json = true;
                    quote! { : serde_json::Value }
                }
            }
        }

        // DEPYLER-0713: Variable references - look up tracked type
        HirExpr::Var(name) => {
            if let Some(var_type) = ctx.var_types.get(name) {
                match type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(var_type)) {
                    Ok(syn_type) => quote! { : #syn_type },
                    Err(_) => {
                        ctx.needs_serde_json = true;
                        quote! { : serde_json::Value }
                    }
                }
            } else {
                ctx.needs_serde_json = true;
                quote! { : serde_json::Value }
            }
        }

        // Default fallback
        _ => {
            // DEPYLER-0713: Try infer_expr_type_simple before falling back to Value
            let inferred = func_gen::infer_expr_type_simple(value);
            if !matches!(inferred, crate::hir::Type::Unknown) {
                match type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(&inferred)) {
                    Ok(syn_type) => quote! { : #syn_type },
                    Err(_) => {
                        ctx.needs_serde_json = true;
                        quote! { : serde_json::Value }
                    }
                }
            } else {
                ctx.needs_serde_json = true;
                quote! { : serde_json::Value }
            }
        }
    }
}

/// DEPYLER-0188: Check if expression is a pathlib Path constant
///
/// Detects Path expressions for correct type inference in module-level constants.
fn is_path_constant_expr(value: &HirExpr) -> bool {
    match value {
        // Path() or pathlib.Path() call
        HirExpr::Call { func, .. } => {
            matches!(func.as_str(), "Path" | "PurePath" | "PathBuf")
        }
        // .parent, .join, etc. method calls return paths
        HirExpr::MethodCall { method, object, .. } => {
            matches!(method.as_str(), "parent" | "join" | "resolve" | "absolute" |
                     "with_name" | "with_suffix" | "to_path_buf")
                || is_path_constant_expr(object)
        }
        // .parent attribute access
        HirExpr::Attribute { attr, value, .. } => {
            matches!(attr.as_str(), "parent" | "root" | "anchor")
                || is_path_constant_expr(value)
        }
        // path / segment division
        HirExpr::Binary { left, op: BinOp::Div, .. } => is_path_constant_expr(left),
        _ => false,
    }
}

/// DEPYLER-0516: Infer type annotation for unary expressions (negative/positive literals)
///
/// Handles type inference for unary operations like -1, +1, --1, -1.5, etc.
/// Complexity: 5 (recursive pattern matching with early returns)
fn infer_unary_type(
    op: &UnaryOp,
    operand: &HirExpr,
    ctx: &mut CodeGenContext,
) -> proc_macro2::TokenStream {
    match (op, operand) {
        // Negation/Positive of int literal → i32
        (UnaryOp::Neg | UnaryOp::Pos, HirExpr::Literal(Literal::Int(_))) => {
            quote! { : i32 }
        }
        // Negation/Positive of float literal → f64
        (UnaryOp::Neg | UnaryOp::Pos, HirExpr::Literal(Literal::Float(_))) => {
            quote! { : f64 }
        }
        // Nested unary (e.g., --1) - recursively check inner operand
        (UnaryOp::Neg | UnaryOp::Pos, HirExpr::Unary { operand: inner, .. }) => {
            match inner.as_ref() {
                HirExpr::Literal(Literal::Int(_)) => quote! { : i32 },
                HirExpr::Literal(Literal::Float(_)) => quote! { : f64 },
                _ => {
                    ctx.needs_serde_json = true;
                    quote! { : serde_json::Value }
                }
            }
        }
        // Other unary operations - fallback
        _ => {
            ctx.needs_serde_json = true;
            quote! { : serde_json::Value }
        }
    }
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
    use std::collections::HashMap;

    // DEPYLER-0201: Deduplicate constants by name, keeping LAST occurrence (Python semantics)
    // Python allows reassignment at module level: NAME = "old"; NAME = "new"
    // We must emit only the last value to avoid Rust error E0428 (duplicate definitions)
    let mut last_by_name: HashMap<&str, &HirConstant> = HashMap::new();
    for constant in constants {
        last_by_name.insert(&constant.name, constant);
    }

    let mut items = Vec::new();

    // Process in original order but only emit constants that are the "last" for each name
    let mut emitted: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for constant in constants {
        // Skip if we already emitted this name or if this isn't the last occurrence
        if emitted.contains(constant.name.as_str()) {
            continue;
        }
        // Check if this is the last occurrence of this name
        if let Some(&last) = last_by_name.get(constant.name.as_str()) {
            if !std::ptr::eq(constant, last) {
                // Not the last occurrence, skip
                continue;
            }
        }
        emitted.insert(&constant.name);

        let name_ident = syn::Ident::new(&constant.name, proc_macro2::Span::call_site());

        // DEPYLER-0188: Lambdas at module level should become functions, not consts
        // Closures cannot be assigned to const in Rust
        if let HirExpr::Lambda { params, body } = &constant.value {
            let token = generate_lambda_as_function(&constant.name, params, body, ctx)?;
            items.push(token);
            continue;
        }

        // DEPYLER-0673: Skip TypeVar calls - they're for type checking, not runtime code
        // Python: T = TypeVar("T")
        // This is only used for static type checking, skip in Rust code generation
        if let HirExpr::Call { func, .. } = &constant.value {
            if func == "TypeVar" {
                continue;
            }
        }

        let value_expr = constant.value.to_rust_expr(ctx)?;

        // DEPYLER-REARCH-001: Complex types need runtime initialization (Lazy)
        // DEPYLER-0188: PathBuf expressions also need runtime init (not const-evaluable)
        // DEPYLER-0714: Function calls also need runtime init - can't be const
        let needs_runtime_init = matches!(
            &constant.value,
            HirExpr::Dict(_) | HirExpr::List(_) | HirExpr::Set(_) | HirExpr::Tuple(_) | HirExpr::Call { .. }
        ) || is_path_constant_expr(&constant.value);

        let token = if needs_runtime_init {
            generate_lazy_constant(constant, name_ident, value_expr, ctx)?
        } else {
            generate_simple_constant(constant, name_ident, value_expr, ctx)?
        };

        items.push(token);
    }

    Ok(items)
}

/// DEPYLER-0188: Convert module-level lambda to a function
///
/// Python: `f = lambda x: x * 2`
/// Rust: `pub fn f(x: i32) -> i32 { x * 2 }`
///
/// Complexity: 5 (param mapping + body conversion)
fn generate_lambda_as_function(
    name: &str,
    params: &[String],
    body: &HirExpr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    use crate::rust_gen::context::ToRustExpr;

    let fn_name = syn::Ident::new(name, proc_macro2::Span::call_site());

    // Generate parameters - use i32 as default type (can be improved with type inference)
    let param_tokens: Vec<proc_macro2::TokenStream> = params
        .iter()
        .map(|p| {
            let param_ident = syn::Ident::new(p, proc_macro2::Span::call_site());
            quote! { #param_ident: i32 }
        })
        .collect();

    let body_expr = body.to_rust_expr(ctx)?;

    Ok(quote! {
        pub fn #fn_name(#(#param_tokens),*) -> i32 {
            #body_expr
        }
    })
}

/// DEPYLER-0615: Generate stub functions for unresolved local imports
///
/// When a Python file imports from a local module (e.g., `from nested_function_cli import X`),
/// and we're compiling standalone, we need stub functions to prevent E0425 errors.
/// These stubs allow compilation without the actual implementation.
///
/// Complexity: 4 (loop + ident creation + quote)
fn generate_stub_functions(
    unresolved_imports: &[import_gen::UnresolvedImport],
) -> Vec<proc_macro2::TokenStream> {
    let mut stubs = Vec::new();
    let mut seen = HashSet::new();

    for import in unresolved_imports {
        // Skip duplicates (same function imported from different locations)
        if !seen.insert(import.item_name.clone()) {
            continue;
        }

        // DEPYLER-0680: Handle imports that are used as types, not functions
        // These need type aliases, not function stubs
        let is_type_import =
            (import.module == "collections.abc" || import.module == "typing")
                && import.item_name == "AsyncIterator";

        if is_type_import {
            // Generate type alias + related stubs instead of function
            let async_stubs = quote! {
                /// AsyncIterator type alias for Python async iteration
                /// DEPYLER-0680: Generated to allow standalone compilation
                #[allow(dead_code)]
                pub type AsyncIterator<T> = std::iter::Once<T>;

                /// StopAsyncIteration exception for Python async iteration
                /// DEPYLER-0680: Generated to allow standalone compilation
                #[allow(dead_code)]
                pub struct StopAsyncIteration;
            };
            stubs.push(async_stubs);
            continue; // Skip function stub generation
        }

        // Create a valid Rust identifier (escape if keyword)
        let func_ident = keywords::safe_ident(&import.item_name);
        let _module_name = &import.module; // Stored for potential doc comment use

        // Generate a stub function that accepts any args
        // DEPYLER-0600: Use () return type for contextlib functions to avoid type inference issues
        let stub = if import.module == "contextlib" {
            quote! {
                /// Stub for local import from module: #module_name
                /// DEPYLER-0615: Generated to allow standalone compilation
                #[allow(dead_code, unused_variables)]
                pub fn #func_ident(_args: impl std::any::Any) -> () {
                }
            }
        } else {
            quote! {
                /// Stub for local import from module: #module_name
                /// DEPYLER-0615: Generated to allow standalone compilation
                #[allow(dead_code, unused_variables)]
                pub fn #func_ident<T: Default>(_args: impl std::any::Any) -> T {
                    Default::default()
                }
            }
        };

        stubs.push(stub);

        // DEPYLER-0600: If importing from contextlib (e.g., suppress), add Python exception stubs
        // These are commonly used as arguments to suppress()
        if import.module == "contextlib" {
            let exception_stubs = quote! {
                /// Python FileNotFoundError stub for contextlib.suppress()
                #[allow(dead_code)]
                pub struct FileNotFoundError;

                /// Python PermissionError stub for contextlib.suppress()
                #[allow(dead_code)]
                pub struct PermissionError;

                /// Python OSError stub for contextlib.suppress()
                #[allow(dead_code)]
                pub struct OSError;
            };
            stubs.push(exception_stubs);
        }
    }

    stubs
}

/// Generate a complete Rust file from HIR module
pub fn generate_rust_file(
    module: &HirModule,
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<(String, Vec<cargo_toml_gen::Dependency>)> {
    let module_mapper = crate::module_mapper::ModuleMapper::new();

    // Process imports to populate the context
    // DEPYLER-0615: Also track unresolved local imports for stub generation
    let (imported_modules, imported_items, unresolved_imports) =
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

    // DEPYLER-0737: Collect property method names from all classes
    // In Python, @property allows method access without (), but Rust requires ()
    let mut property_methods: HashSet<String> = HashSet::new();
    for class in &module.classes {
        for method in &class.methods {
            if method.is_property {
                property_methods.insert(method.name.clone());
            }
        }
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
        needs_digest: false, // DEPYLER-0558
        needs_blake2: false,
        needs_hex: false,
        needs_uuid: false,
        needs_hmac: false,
        needs_crc32: false,
        needs_url_encoding: false,
        needs_io_read: false,   // DEPYLER-0458
        needs_io_write: false,  // DEPYLER-0458
        needs_bufread: false,   // DEPYLER-0522
        needs_once_cell: false, // DEPYLER-REARCH-001
        needs_trueno: false,    // Phase 3: NumPy→Trueno codegen
        needs_tokio: false,     // DEPYLER-0747: asyncio→tokio async runtime mapping
        needs_completed_process: false, // DEPYLER-0627: subprocess.run returns CompletedProcess struct
        vararg_functions: HashSet::new(), // DEPYLER-0648: Track functions with *args
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
        needs_runtimeerror: false,      // DEPYLER-0551
        needs_filenotfounderror: false, // DEPYLER-0551
        in_generator: false,
        is_classmethod: false,
        generator_state_vars: HashSet::new(),
        var_types: std::collections::HashMap::new(),
        class_names,
        mutating_methods,
        property_methods, // DEPYLER-0737: Track @property methods for parenthesis insertion
        function_return_types: std::collections::HashMap::new(), // DEPYLER-0269: Track function return types
        function_param_borrows: std::collections::HashMap::new(), // DEPYLER-0270: Track parameter borrowing
        function_param_muts: std::collections::HashMap::new(), // DEPYLER-0574: Track &mut parameters
        function_param_defaults: std::collections::HashMap::new(),
        function_param_optionals: std::collections::HashMap::new(), // DEPYLER-0737: Track Optional params
        class_field_types: std::collections::HashMap::new(), // DEPYLER-0621: Track default param values
        tuple_iter_vars: HashSet::new(), // DEPYLER-0307 Fix #9: Track tuple iteration variables
        iterator_vars: HashSet::new(),   // DEPYLER-0520: Track variables assigned from iterators
        ref_params: HashSet::new(),      // DEPYLER-0758: Track parameters passed by reference
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
        precomputed_option_fields: HashSet::new(), // DEPYLER-0108: Track precomputed Option checks for argparse
        nested_function_params: std::collections::HashMap::new(), // GH-70: Track inferred nested function params
        fn_str_params: HashSet::new(), // DEPYLER-0543: Track function params with str type (become &str in Rust)
        in_cmd_handler: false, // DEPYLER-0608: Track if in cmd_* handler function
        cmd_handler_args_fields: Vec::new(), // DEPYLER-0608: Track extracted args.X fields
        in_subcommand_match_arm: false, // DEPYLER-0608: Track if in subcommand match arm
        subcommand_match_fields: Vec::new(), // DEPYLER-0608: Track subcommand fields for match arm
        hoisted_function_names: Vec::new(), // DEPYLER-0613: Track hoisted nested function names
        is_main_function: false, // DEPYLER-0617: Track if in main() for exit code handling
        boxed_dyn_write_vars: HashSet::new(), // DEPYLER-0625: Track vars needing Box<dyn Write>
        function_returns_boxed_write: false, // DEPYLER-0626: Track functions returning Box<dyn Write>
        option_unwrap_map: HashMap::new(), // DEPYLER-0627: Track Option unwrap substitutions
        type_substitutions: HashMap::new(), // DEPYLER-0716: Track type substitutions for generic inference
        current_assign_type: None, // DEPYLER-0727: Track assignment target type for dict Value wrapping
        force_dict_value_option_wrap: false, // DEPYLER-0741: Force dict values to use Option wrapping
        char_iter_vars: HashSet::new(), // DEPYLER-0795: Track loop vars iterating over string.chars()
    };

    // Analyze all functions first for string optimization
    analyze_string_optimization(&mut ctx, &module.functions);

    // Finalize interned string names (resolve collisions)
    ctx.string_optimizer.finalize_interned_names();

    // DEPYLER-0447: Scan all function bodies and constants for argparse validators
    // Must run BEFORE function conversion so validator parameter types are correct
    analyze_validators(&mut ctx, &module.functions, &module.constants);

    // DEPYLER-0789: Pre-register ALL argparse subcommands from ALL functions
    // This ensures cmd_* functions have access to argument type info (e.g., store_true → bool)
    // even when defined before the main() function that sets up argparse
    for func in &module.functions {
        argparse_transform::preregister_subcommands_from_hir(func, &mut ctx.argparser_tracker);
    }

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

    // DEPYLER-0648: Pre-populate vararg functions before codegen
    // Python *args functions become fn(args: &[String]) in Rust
    // Call sites need to wrap arguments in &[...] slices
    for func in &module.functions {
        if func.params.iter().any(|p| p.is_vararg) {
            ctx.vararg_functions.insert(func.name.clone());
        }
    }

    // DEPYLER-0737: Pre-populate Optional parameters for call site wrapping
    // When a parameter has Optional type (from =None default), call sites need Some() wrapping
    for func in &module.functions {
        let optionals: Vec<bool> = func
            .params
            .iter()
            .map(|p| matches!(p.ty, Type::Optional(_)))
            .collect();
        // Only track if any param is Optional
        if optionals.iter().any(|&b| b) {
            ctx.function_param_optionals
                .insert(func.name.clone(), optionals);
        }
    }

    // DEPYLER-0720: Pre-populate class field types for self.X attribute access
    // This enables expr_returns_float() to recognize self.balance as float
    for class in &module.classes {
        for field in &class.fields {
            ctx.class_field_types
                .insert(field.name.clone(), field.field_type.clone());
        }
    }

    // Convert classes first (they might be used by functions)
    // DEPYLER-0648: Pass vararg_functions for proper call site generation
    let classes = convert_classes_to_rust(&module.classes, ctx.type_mapper, &ctx.vararg_functions)?;

    // Convert all functions to detect what imports we need
    let functions = convert_functions_to_rust(&module.functions, &mut ctx)?;

    // Build items list with all generated code
    let mut items = Vec::new();

    // Add module imports (create new mapper for token generation)
    let import_mapper = crate::module_mapper::ModuleMapper::new();
    items.extend(generate_import_tokens(&module.imports, &import_mapper));

    // DEPYLER-197: Add type aliases (before constants, after imports)
    // Python type aliases like `EventHandler = Callable[[str], None]`
    // must be transpiled as Rust type aliases
    items.extend(generate_type_alias_tokens(&module.type_aliases, ctx.type_mapper));

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

    // DEPYLER-0627: Add CompletedProcess struct if subprocess.run is used
    if ctx.needs_completed_process {
        let completed_process_struct = quote::quote! {
            /// Result of subprocess.run()
            #[derive(Debug, Clone)]
            pub struct CompletedProcess {
                pub returncode: i32,
                pub stdout: String,
                pub stderr: String,
            }
        };
        items.push(completed_process_struct);
    }

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

    // DEPYLER-0615: Generate stub functions for unresolved local imports
    // This allows test files importing from local modules to compile standalone
    items.extend(generate_stub_functions(&unresolved_imports));

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
            needs_bufread: false,   // DEPYLER-0522
            needs_once_cell: false, // DEPYLER-REARCH-001
            needs_trueno: false,    // Phase 3: NumPy→Trueno codegen
            needs_tokio: false,     // DEPYLER-0747: asyncio→tokio async runtime mapping
            needs_completed_process: false, // DEPYLER-0627: subprocess.run returns CompletedProcess struct
            vararg_functions: HashSet::new(), // DEPYLER-0648: Track functions with *args
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
            needs_runtimeerror: false,      // DEPYLER-0551
            needs_filenotfounderror: false, // DEPYLER-0551
            is_classmethod: false,
            in_generator: false,
            generator_state_vars: HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            property_methods: HashSet::new(), // DEPYLER-0737: Track @property methods
            function_return_types: std::collections::HashMap::new(), // DEPYLER-0269: Track function return types
            function_param_borrows: std::collections::HashMap::new(), // DEPYLER-0270: Track parameter borrowing
            function_param_muts: std::collections::HashMap::new(), // DEPYLER-0574: Track &mut parameters
            tuple_iter_vars: HashSet::new(), // DEPYLER-0307 Fix #9: Track tuple iteration variables
            iterator_vars: HashSet::new(), // DEPYLER-0520: Track variables assigned from iterators
            ref_params: HashSet::new(),      // DEPYLER-0758: Track parameters passed by reference
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
            precomputed_option_fields: HashSet::new(), // DEPYLER-0108: Track precomputed Option checks
            cse_subcommand_temps: std::collections::HashMap::new(), // DEPYLER-0456 Bug #2: Track CSE subcommand temps
            nested_function_params: std::collections::HashMap::new(), // GH-70: Track inferred nested function params
            fn_str_params: HashSet::new(), // DEPYLER-0543: Track function params with str type
            needs_digest: false,           // DEPYLER-0558: Track digest crate dependency
            in_cmd_handler: false, // DEPYLER-0608: Track if in cmd_* handler function
            cmd_handler_args_fields: Vec::new(), // DEPYLER-0608: Track extracted args.X fields
            in_subcommand_match_arm: false, // DEPYLER-0608: Track if in subcommand match arm
            subcommand_match_fields: Vec::new(), // DEPYLER-0608: Track subcommand fields in match arm
            hoisted_function_names: Vec::new(), // DEPYLER-0613: Track hoisted nested function names
            is_main_function: false, // DEPYLER-0617: Track if in main() for exit code handling
            boxed_dyn_write_vars: HashSet::new(), // DEPYLER-0625: Track vars needing Box<dyn Write>
            function_returns_boxed_write: false, // DEPYLER-0626: Track functions returning Box<dyn Write>
            option_unwrap_map: HashMap::new(), // DEPYLER-0627: Track Option unwrap substitutions
            function_param_defaults: HashMap::new(), // Track function parameter defaults
            function_param_optionals: HashMap::new(), // DEPYLER-0737: Track Optional params
            class_field_types: HashMap::new(), // DEPYLER-0720: Track class field types
            type_substitutions: HashMap::new(), // DEPYLER-0716: Track type substitutions
            current_assign_type: None, // DEPYLER-0727: Track assignment target type
            force_dict_value_option_wrap: false, // DEPYLER-0741
            char_iter_vars: HashSet::new(), // DEPYLER-0795: Track loop vars iterating over string.chars()
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
        // DEPYLER-0701: Pure expressions are wrapped in `let _ =` to avoid
        // "path statement with no effect" warnings
        assert_eq!(result.to_string(), "let _ = 42 ;");
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
        // DEPYLER-0698: `while True:` now generates idiomatic `loop {}`
        // (Rust warns: "denote infinite loops with `loop { ... }`")
        assert!(result.to_string().contains("loop"));
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

        let result = codegen_with_stmt(&context, &target, &body, false, &mut ctx).unwrap();
        assert!(result.to_string().contains("let mut file"));
    }

    #[test]
    fn test_codegen_with_stmt_no_target() {
        use crate::hir::Literal;

        let mut ctx = create_test_context();
        let context = HirExpr::Literal(Literal::Int(42));
        let body = vec![HirStmt::Pass];

        let result = codegen_with_stmt(&context, &None, &body, false, &mut ctx).unwrap();
        // DEPYLER-0602: Context variable is mutable for __enter__()
        assert!(result.to_string().contains("let mut _context"));
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
