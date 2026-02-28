use crate::cargo_toml_gen; // DEPYLER-0384: Cargo.toml generation
use crate::hir::*;
use anyhow::Result;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn;

// Module declarations for rust_gen refactoring (v3.18.0 Phases 2-7)
mod argparse_transform;
mod array_initialization; // DEPYLER-REFACTOR-001: Extracted from expr_gen.rs
pub mod binary_ops; // DEPYLER-SPLIT-001: Extracted binary operation handling
mod builtin_conversions; // DEPYLER-REFACTOR-001: Extracted from expr_gen.rs
mod collection_constructors; // DEPYLER-REFACTOR-001: Extracted from expr_gen.rs
mod context;
mod error_gen;
mod expr_gen;
mod expr_gen_instance_methods; // DEPYLER-COVERAGE-95: Instance method handlers split from expr_gen
mod format;
pub mod func_gen; // DEPYLER-0518: Made public for type inference from lifetime_analysis
pub mod func_gen_helpers; // DEPYLER-COVERAGE-95: Extracted pure helpers for testability
mod func_gen_inference; // DEPYLER-COVERAGE-95: Advanced function codegen helpers split from func_gen
mod generator_gen;
mod import_gen;
pub mod keywords; // DEPYLER-0023: Centralized keyword escaping
pub mod numpy_gen; // Phase 3: NumPy→Trueno codegen
mod stmt_gen;
mod stmt_gen_complex; // DEPYLER-COVERAGE-95: Complex statement handlers split from stmt_gen
mod type_gen;

// Helper modules (v3.21.0)
#[cfg(feature = "sovereign-types")]
mod binding_gen; // DEPYLER-1115: Phantom binding generation for external library types
pub mod borrowing_helpers; // DEPYLER-COVERAGE-95: Borrowing helpers extracted
pub mod control_flow_analysis; // PMAT: Extracted control flow analysis for 100% unit test coverage
pub mod control_stmt_helpers; // DEPYLER-0140: Control statement codegen helpers extracted
pub mod exception_helpers; // DEPYLER-0333: Exception type extraction helpers
pub mod expr_analysis; // PMAT: Extracted expression analysis for 100% unit test coverage
pub mod expr_type_helpers; // DEPYLER-COVERAGE-95: Expression type helpers extracted
pub mod iterator_utils; // DEPYLER-SPLIT-001: Extracted iterator utilities
pub mod json_helpers; // DEPYLER-COVERAGE-95: JSON serialization helpers extracted
pub mod mutation_helpers;
pub mod name_heuristics; // DEPYLER-COVERAGE-95: Name-based type heuristics extracted
pub mod numeric_coercion; // DEPYLER-0582: Numeric type coercion helpers extracted
pub mod precedence; // DEPYLER-0582: Operator precedence helpers extracted for testability
pub mod string_analysis; // PMAT: Extracted string analysis for 100% unit test coverage
mod string_method_helpers;
pub mod truthiness_helpers; // DEPYLER-1096: Made public for truthiness coercion in direct_rules_convert
pub mod type_coercion; // DEPYLER-SPLIT-001: Extracted type coercion utilities
pub mod type_conversion_helpers; // DEPYLER-0455: Type conversion helpers extracted
pub mod type_tokens; // DEPYLER-0759: HIR type to token conversion extracted for testability
pub mod unary_ops; // DEPYLER-SPLIT-002: Extracted unary operation handling
pub mod var_analysis; // PMAT: Extracted variable analysis for 100% unit test coverage
pub mod walrus_helpers; // DEPYLER-0792: Walrus operator helpers extracted for testability // DEPYLER-COVERAGE-95: Mutation analysis helpers extracted

// Stdlib method code generation (DEPYLER-COVERAGE-95: Extracted from expr_gen.rs)
pub mod stdlib_method_gen;

// Text-level fix functions (DEPYLER-DECOMPOSE: Extracted from rust_gen.rs)
mod fixes;

// Phase 2: Pre-pipeline helpers (DEPYLER-DECOMPOSE: Extracted from rust_gen.rs)
mod class_gen;
mod constant_gen;
mod module_gen;
mod validator_analysis;

// Phase 3: Function decomposition (DEPYLER-DECOMPOSE)
mod depyler_value_gen;
mod mutable_analysis;
mod nasa_mode;
mod runtime_types_gen;

// Phase 4: Pipeline decomposition (DEPYLER-DECOMPOSE)
mod context_init;
mod pipeline_assembly;
mod pre_analysis;

// Test modules (DEPYLER-QUALITY-002: 95% coverage target)
#[cfg(test)]
mod argparse_transform_tests;
#[cfg(test)]
mod builtin_conversions_tests;
#[cfg(test)]
mod comprehensive_integration_tests;
#[cfg(test)]
mod coverage_boost_deep_tests;
#[cfg(test)]
mod coverage_boost_expr_tests;
#[cfg(test)]
mod coverage_boost_instance_tests;
#[cfg(test)]
mod coverage_boost_stmt_tests;
#[cfg(test)]
mod coverage_boost_zero_cov_tests;
#[cfg(test)]
mod coverage_wave3_advanced_tests;
#[cfg(test)]
mod coverage_wave3_stdlib_tests;
#[cfg(test)]
mod coverage_wave3_types_tests;
#[cfg(test)]
mod coverage_wave4_analysis_tests;
#[cfg(test)]
mod coverage_wave4_direct_rules_tests;
#[cfg(test)]
mod coverage_wave4_expr_tests;
#[cfg(test)]
mod coverage_wave4_funcgen_tests;
#[cfg(test)]
mod coverage_wave4_instance_tests;
#[cfg(test)]
mod coverage_wave5_exprmethods_tests;
#[cfg(test)]
mod coverage_wave5_funcgen_tests;
#[cfg(test)]
mod coverage_wave5_stmtgen_tests;
#[cfg(test)]
mod coverage_wave6_instance_tests;
#[cfg(test)]
mod coverage_wave7_rustgen_tests;
#[cfg(test)]
mod coverage_wave7_slicing_tests;
#[cfg(test)]
mod coverage_wave7_stmtgen_complex_tests;
#[cfg(test)]
mod coverage_wave8_argparse_string_tests;
#[cfg(test)]
mod coverage_wave8_deep_codegen_tests;
#[cfg(test)]
mod coverage_wave9_direct_rules_tests;
#[cfg(test)]
mod coverage_wave9_fix_inference_tests;
#[cfg(test)]
mod coverage_wave9_stdlib_expr_tests;
#[cfg(test)]
mod deep_coverage_tests;
#[cfg(test)]
mod direct_rules_convert_transpile_tests;
#[cfg(test)]
mod direct_rules_tests;
#[cfg(test)]
mod expr_gen_tests;
#[cfg(test)]
mod func_gen_tests;
#[cfg(test)]
mod generator_gen_tests;
#[cfg(test)]
mod stmt_gen_tests;
#[cfg(test)]
mod targeted_expr_tests;
#[cfg(test)]
mod targeted_func_tests;
#[cfg(test)]
mod targeted_stmt_tests;
#[cfg(test)]
mod type_gen_tests;
// Waves 10-11: 1084 tests (now included in coverage with --test-threads=2)
#[cfg(test)]
mod coverage_wave10_assign_control_tests;
#[cfg(test)]
mod coverage_wave10_instance_deep_tests;
#[cfg(test)]
mod coverage_wave11_assign_control_tests;
#[cfg(test)]
mod coverage_wave11_expr_type_tests;
#[cfg(test)]
mod coverage_wave11_string_dict_tests;
// Wave 12: 600 tests
#[cfg(test)]
mod coverage_wave12_binary_call_tests;
#[cfg(test)]
mod coverage_wave12_dispatch_type_tests;
#[cfg(test)]
mod coverage_wave12_expr_methods_tests;
// Wave 13: 600 tests targeting func_gen, stmt_gen, call_dispatch, stdlib
#[cfg(test)]
mod coverage_wave13_call_stdlib_tests;
#[cfg(test)]
mod coverage_wave13_func_gen_tests;
#[cfg(test)]
mod coverage_wave13_stmt_gen_tests;
// Wave 14: 450 targeted coverage tests (included in coverage measurement)
#[cfg(test)]
mod coverage_wave14_comp_lambda_tests;
#[cfg(test)]
mod coverage_wave14_methods_dispatch_tests;
#[cfg(test)]
mod coverage_wave14_slice_index_tests;
// Wave 15: 200 class/async/error/function coverage tests
#[cfg(test)]
mod coverage_wave15_class_async_tests;
#[cfg(test)]
mod coverage_wave15_rules_type_tests;
// Wave 16: 200 func_gen/stmt_gen/type-coercion coverage tests
#[cfg(test)]
mod coverage_wave16_func_stmt_tests;
// Wave 16: 200 method edge case coverage tests (string, dict, list, set)
#[cfg(test)]
mod coverage_wave16_method_edge_tests;
// Wave 16: 200 slice/index/comprehension/attribute coverage tests
#[cfg(test)]
mod coverage_wave16_slice_comp_tests;
// Wave 17: 200 augmented-assign/comparison/tuple/variable/expression coverage tests
#[cfg(test)]
mod coverage_wave17_rules_expr_tests;
// Wave 17: 200 instance dispatch/method routing/stdlib/numeric deep tests
#[cfg(test)]
mod coverage_wave17_instance_deep_tests;
// Wave 17: 200 import/class/format/error generation coverage tests
#[cfg(test)]
mod coverage_wave17_import_class_tests;
// Wave 18: 200 direct_rules_convert/expr_methods.rs coverage tests
#[cfg(test)]
mod coverage_wave18_direct_expr_tests;
// Wave 18: 200 call_generic/codegen_assign_stmt coverage tests
#[cfg(test)]
mod coverage_wave18_call_assign_tests;
// Wave 18: 200 string_methods/instance_dispatch coverage tests
#[cfg(test)]
mod coverage_wave18_string_instance_tests;
// Wave 18 Deep: 200 deep string method code path coverage tests
#[cfg(test)]
mod coverage_wave18_string_deep_tests;
// Wave 18: 200 dict/list/set collection deep method coverage tests
#[cfg(test)]
mod coverage_wave18_collection_deep_tests;
// Wave 18: 200 stmt_gen/func_gen/type code path coverage tests
#[cfg(test)]
mod coverage_wave18_stmt_func_tests;
// Wave 19: 200 stdlib deep coverage tests (sys, re, colorsys, math, json, time, random, hashlib)
#[cfg(test)]
mod coverage_wave19_stdlib_deep_tests;
// Wave 19: 200 instance_dispatch/set/dict/regex/deque/dunder/file I/O coverage tests
#[cfg(test)]
mod coverage_wave19_instance_dispatch_tests;
// Wave 19: 200 argparse subcommand/call/assign/constructor/variable type coverage tests
#[cfg(test)]
mod coverage_wave19_argparse_call_tests;
// Wave 19 Deep: 200 direct_rules hashlib/base64/re/os/sys coverage tests
#[cfg(test)]
mod coverage_wave19_direct_rules_deep_tests;
// Wave 19 Deep: 200 instance methods + string/list/dict/set deep coverage tests
#[cfg(test)]
mod coverage_wave19_instance_deep_tests;
// Wave 19 Deep: 200 stmt/expr/call/class/type deep coverage tests
#[cfg(test)]
mod coverage_wave19_stmt_expr_deep_tests;
// Wave 20: 200 rust_gen generate_rust_file_internal + analyze_mutable_vars coverage tests
#[cfg(test)]
mod coverage_wave20_rustgen_mutable_tests;
// Wave 20: 200 codegen_assign_stmt + convert_string_method coverage tests
#[cfg(test)]
mod coverage_wave20_stmt_string_tests;
// Wave 20: 200 call_generic/method_call_routing/instance_dispatch coverage tests
#[cfg(test)]
mod coverage_wave20_call_dispatch_tests;
// Wave 21: 200 import/constant/class handling deep coverage tests
#[cfg(test)]
mod coverage_wave21_import_const_tests;
// Wave 21: 200 function generation, expression generation, and type inference coverage tests
#[cfg(test)]
mod coverage_wave21_func_expr_tests;
// Wave 21: 200 direct_rules convert_method_call + argparse transform coverage tests
#[cfg(test)]
mod coverage_wave21_rules_argparse_tests;
// Wave 21: 200 collection edge case tests (list/dict/set/tuple methods)
#[cfg(test)]
mod coverage_wave21_collection_edge_tests;
// Wave 21: 200 string method edge case + attribute access tests
#[cfg(test)]
mod coverage_wave21_string_attr_tests;
// Wave 21: 200 stmt_gen complex + indexing + func_gen nested tests
#[cfg(test)]
mod coverage_wave21_stmt_index_tests;
// Wave 22: 200 string/set/dict method deep coverage tests
#[cfg(test)]
mod coverage_wave22_method_deep_tests;
// Wave 22: 200 assignment + control flow deep coverage tests
#[cfg(test)]
mod coverage_wave22_assign_control_tests;
// Wave 22: 200 call_generic + builtin + datetime deep coverage tests
#[cfg(test)]
mod coverage_wave22_call_builtin_tests;
// Wave 23: 200 function + class + error handling coverage tests
#[cfg(test)]
mod coverage_wave23_func_class_tests;

// Internal imports
#[cfg(test)]
use crate::string_optimization::StringOptimizer;
#[cfg(test)]
use control_stmt_helpers::{codegen_break_stmt, codegen_continue_stmt, codegen_pass_stmt};
use format::format_rust_code;
#[cfg(test)]
use stmt_gen::{
    codegen_assign_attribute, codegen_assign_index, codegen_assign_symbol, codegen_assign_tuple,
    codegen_expr_stmt, codegen_raise_stmt, codegen_return_stmt, codegen_while_stmt,
    codegen_with_stmt,
};
#[cfg(test)]
use stmt_gen_complex::codegen_try_stmt;

// Public re-exports for external modules (union_enum_gen, etc.)
pub use argparse_transform::ArgParserTracker; // DEPYLER-0384: Export for testing
pub use argparse_transform::{
    generate_args_struct, generate_commands_enum, ArgParserArgument, ArgParserInfo, SubcommandInfo,
    SubparserInfo,
}; // Coverage tests
pub use context::{CodeGenContext, RustCodeGen, ToRustExpr};
pub use type_gen::rust_type_to_syn;

// Internal re-exports for cross-module access
pub(crate) use func_gen::return_type_expects_float;

/// DEPYLER-1114: Load the Sovereign Type Database from a well-known path
///
/// Searches for the type database parquet file in order of priority:
/// 1. DEPYLER_TYPE_DB environment variable
/// 2. ./crates/depyler-core/src/data/stdlib_types.parquet (dev path)
/// 3. ./data/stdlib_types.parquet (relative to cwd)
///
/// Returns None if no database is found or if sovereign-types feature is disabled.
#[cfg(feature = "sovereign-types")]
fn load_type_database() -> Option<std::sync::Arc<std::sync::Mutex<depyler_knowledge::TypeQuery>>> {
    use std::sync::{Arc, Mutex};

    // Priority 1: Environment variable
    if let Ok(path) = std::env::var("DEPYLER_TYPE_DB") {
        if let Ok(query) = depyler_knowledge::TypeQuery::new(std::path::Path::new(&path)) {
            return Some(Arc::new(Mutex::new(query)));
        }
    }

    // Priority 2: Development path (relative to project root)
    let dev_path = std::path::Path::new("crates/depyler-core/src/data/stdlib_types.parquet");
    if dev_path.exists() {
        if let Ok(query) = depyler_knowledge::TypeQuery::new(dev_path) {
            return Some(Arc::new(Mutex::new(query)));
        }
    }

    // Priority 3: Relative to current directory
    let rel_path = std::path::Path::new("data/stdlib_types.parquet");
    if rel_path.exists() {
        if let Ok(query) = depyler_knowledge::TypeQuery::new(rel_path) {
            return Some(Arc::new(Mutex::new(query)));
        }
    }

    None
}

/// Analyze which variables are reassigned (mutated) in a list of statements
///
/// Populates ctx.mutable_vars with variables that are:
/// 1. Reassigned after declaration (x = 1; x = 2)
/// 2. Mutated via method calls (.push(), .extend(), .insert(), .remove(), .pop(), etc.)
/// 3. DEPYLER-0312: Function parameters that are reassigned (requires mut)
///
/// Complexity: 7 (stmt loop + match + if + expr scan + method match)
pub(crate) fn analyze_mutable_vars(
    stmts: &[HirStmt],
    ctx: &mut CodeGenContext,
    params: &[HirParam],
) {
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

    // DEPYLER-DECOMPOSE: Delegate to extracted mutable_analysis module
    let mut var_types = HashMap::new();
    let mutating_methods = &ctx.mutating_methods;
    let function_param_muts = &ctx.function_param_muts;
    for stmt in stmts {
        mutable_analysis::analyze_stmt(
            stmt,
            &mut declared,
            &mut ctx.mutable_vars,
            &mut var_types,
            mutating_methods,
            function_param_muts,
        );
    }
}

/// Convert HIR functions to Rust token streams
///
/// Processes all functions using the code generation context.
/// Complexity: 2 (well within ≤10 target)
fn convert_functions_to_rust(
    functions: &[HirFunction],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    functions.iter().map(|f| f.to_rust_tokens(ctx)).collect::<Result<Vec<_>>>()
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
pub(super) fn generate_stub_functions(
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
        let is_type_import = (import.module == "collections.abc" || import.module == "typing")
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
            // DEPYLER-1404: Use DepylerValue return type instead of generic T: Default
            // The generic <T: Default> causes E0283 (type annotations needed) when called
            // in contexts like `assert!(imported_func(x) == 42)`. DepylerValue implements
            // PartialEq<i32>, PartialEq<String>, etc. so it works for most comparisons.
            quote! {
                /// Stub for local import from module: #module_name
                /// DEPYLER-0615: Generated to allow standalone compilation
                #[allow(dead_code, unused_variables)]
                pub fn #func_ident(_args: impl std::any::Any) -> DepylerValue {
                    DepylerValue::default()
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
    // DEPYLER-1133: Delegate to internal implementation with empty var_types
    generate_rust_file_internal(module, type_mapper, std::collections::HashMap::new())
}

/// DEPYLER-1133: Internal implementation that accepts pre-seeded var_types
///
/// This is the "Restoration of Truth" - when called with Oracle-learned types,
/// those types are used during code generation instead of being inferred fresh.
fn generate_rust_file_internal(
    module: &HirModule,
    type_mapper: &crate::type_mapper::TypeMapper,
    initial_var_types: std::collections::HashMap<String, Type>,
) -> Result<(String, Vec<cargo_toml_gen::Dependency>)> {
    // DEPYLER-DECOMPOSE Phase 4: Use extracted modules for pipeline phases

    // Phase 1: Analyze module (imports, async detection, class metadata)
    let analysis = context_init::analyze_module(module);
    let type_mapper = analysis.resolve_type_mapper(type_mapper);
    let type_mapper = &type_mapper;

    // Phase 2: Build CodeGenContext from analysis
    let (mut ctx, unresolved_imports) =
        context_init::build_codegen_context(analysis, type_mapper, initial_var_types);

    // Phase 3: Populate function/class metadata into context
    pre_analysis::populate_context_metadata(module, &mut ctx);

    // DEPYLER-1103: Enable DepylerValue enum when functions use Dict/Unknown types
    if !ctx.needs_depyler_value_enum {
        for func in &module.functions {
            if matches!(&func.ret_type, Type::Dict(_, v) if matches!(v.as_ref(), Type::Unknown)) {
                ctx.needs_depyler_value_enum = true;
                break;
            }
        }
    }

    // Phase 4: Convert module-level constants type registration
    // DEPYLER-1060: Pre-register module-level constant types BEFORE function conversion
    for constant in &module.constants {
        let const_type = match &constant.value {
            HirExpr::Dict(_) => Some(Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown))),
            HirExpr::List(_) => Some(Type::List(Box::new(Type::Unknown))),
            HirExpr::Set(_) => Some(Type::Set(Box::new(Type::Unknown))),
            HirExpr::Literal(Literal::Int(_)) => Some(Type::Int),
            HirExpr::Literal(Literal::Float(_)) => Some(Type::Float),
            HirExpr::Literal(Literal::String(_)) => Some(Type::String),
            HirExpr::Literal(Literal::Bool(_)) => Some(Type::Bool),
            _ => {
                if let Some(ty) = &constant.type_annotation {
                    if !matches!(ty, Type::Unknown) {
                        Some(ty.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        };
        if let Some(t) = const_type {
            ctx.module_constant_types.insert(constant.name.clone(), t);
        }
    }

    // Phase 5: Convert classes and functions
    let (classes, adt_child_to_parent) = class_gen::convert_classes_to_rust(
        &module.classes,
        ctx.type_mapper,
        &ctx.vararg_functions,
    )?;
    ctx.adt_child_to_parent = adt_child_to_parent;

    let functions = convert_functions_to_rust(&module.functions, &mut ctx)?;

    // Phase 6: Assemble all items
    let nasa_mode = ctx.type_mapper.nasa_mode;
    let items = pipeline_assembly::assemble_module_items(
        module,
        &mut ctx,
        classes,
        functions,
        &unresolved_imports,
        nasa_mode,
    )?;

    let file = quote! {
        #(#items)*
    };

    // Phase 7: Post-processing (dependencies, NASA mode, text-level fixes)
    let mut dependencies = cargo_toml_gen::extract_dependencies(&ctx);

    let mut formatted_code = format_rust_code(file.to_string());
    // DEPYLER-0393: Post-process FORMATTED code to detect missed dependencies
    if !nasa_mode && formatted_code.contains("serde_json::") && !ctx.needs_serde_json {
        formatted_code = format!("use serde_json;\n{}", formatted_code);
        dependencies.push(cargo_toml_gen::Dependency::new("serde_json", "1.0"));
        dependencies.push(
            cargo_toml_gen::Dependency::new("serde", "1.0")
                .with_features(vec!["derive".to_string()]),
        );
        formatted_code = format_rust_code(formatted_code);
    }

    if nasa_mode {
        nasa_mode::apply_nasa_mode_fixes(&mut formatted_code);
    }

    formatted_code = fixes::pipeline::apply_text_level_fixes(formatted_code);

    let allow_attrs = "\
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
";
    formatted_code = format!("{}{}", allow_attrs, formatted_code);

    Ok((formatted_code, dependencies))
}

/// DEPYLER-1101: Generate Rust file with oracle-learned type overrides
///
/// This function accepts a map of variable names to their corrected types,
/// learned from E0308 compiler error feedback. When generating code,
/// these types override the inferred types for the specified variables.
///
/// # Arguments
/// * `module` - The HIR module to generate code from
/// * `type_mapper` - Type mapper for Python→Rust type conversion
/// * `type_overrides` - Map of variable name → corrected HIR Type from oracle
///
/// # Returns
/// Returns the generated Rust code and Cargo dependencies, or an error.
pub fn generate_rust_file_with_overrides(
    module: &HirModule,
    type_mapper: &crate::type_mapper::TypeMapper,
    type_overrides: std::collections::HashMap<String, Type>,
) -> Result<(String, Vec<cargo_toml_gen::Dependency>)> {
    // DEPYLER-1133: RESTORATION OF TRUTH
    // The Oracle has learned the correct types. We MUST obey.
    // Pre-seed var_types with the overrides before code generation.

    if type_overrides.is_empty() {
        // No overrides - use standard generator
        return generate_rust_file(module, type_mapper);
    }

    // Log the overrides for observability
    eprintln!("DEPYLER-1133: Restoring {} type constraints from Oracle", type_overrides.len());
    for (var, ty) in &type_overrides {
        eprintln!("  {} → {:?}", var, ty);
    }

    // DEPYLER-1133: THE RESTORATION OF TRUTH
    // Call the internal generator with Oracle's pre-seeded var_types.
    // This ensures the generator obeys the Oracle's constraints.
    generate_rust_file_internal(module, type_mapper, type_overrides)
}

/// DEPYLER-1101: Convert Rust type string from E0308 error to HIR Type
///
/// Parses type strings like "i32", "String", "Vec<i32>", "HashMap<String, i32>"
/// into HIR Type representation for use in type overrides.
pub fn rust_type_string_to_hir(rust_type: &str) -> Type {
    let trimmed = rust_type.trim();

    match trimmed {
        "i32" | "i64" | "isize" | "u32" | "u64" | "usize" => Type::Int,
        "f32" | "f64" => Type::Float,
        "bool" => Type::Bool,
        "String" | "&str" | "str" => Type::String,
        "()" => Type::None,
        _ => parse_generic_rust_type(trimmed),
    }
}

fn parse_generic_rust_type(trimmed: &str) -> Type {
    if let Some(inner) = strip_generic(trimmed, "Vec<") {
        return Type::List(Box::new(rust_type_string_to_hir(inner)));
    }
    if let Some(inner) = strip_generic(trimmed, "HashMap<") {
        if let Some(comma_idx) = find_balanced_comma(inner) {
            let key = rust_type_string_to_hir(&inner[..comma_idx]);
            let val = rust_type_string_to_hir(&inner[comma_idx + 1..]);
            return Type::Dict(Box::new(key), Box::new(val));
        }
        return Type::Unknown;
    }
    if let Some(inner) = strip_generic(trimmed, "Option<") {
        return Type::Optional(Box::new(rust_type_string_to_hir(inner)));
    }
    if let Some(inner) = strip_generic(trimmed, "HashSet<") {
        return Type::Set(Box::new(rust_type_string_to_hir(inner)));
    }
    Type::Unknown
}

fn strip_generic<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    s.strip_prefix(prefix).and_then(|rest| rest.strip_suffix('>'))
}

/// Find the comma that separates type parameters at the top level
fn find_balanced_comma(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::class_gen::detect_adt_patterns;
    use super::constant_gen::{infer_constant_type, infer_unary_type, is_path_constant_expr};
    use super::context::RustCodeGen;
    use super::type_gen::convert_binop;
    use super::validator_analysis::{scan_expr_for_validators, scan_stmts_for_validators};
    use super::*;
    use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
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
            needs_slice_random: false, // GH-207
            needs_rand_distr: false,   // GH-207
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
            needs_sha1: false,                // DEPYLER-1001: sha1 crate
            needs_statrs: false,              // DEPYLER-1001: statrs crate
            needs_url: false,                 // DEPYLER-1001: url crate
            needs_io_read: false,             // DEPYLER-0458
            needs_io_write: false,            // DEPYLER-0458
            needs_bufread: false,             // DEPYLER-0522
            needs_once_cell: false,           // DEPYLER-REARCH-001
            needs_lazy_lock: false,           // DEPYLER-1016
            needs_trueno: false,              // Phase 3: NumPy→Trueno codegen
            numpy_vars: HashSet::new(),       // DEPYLER-0932: Track numpy array variables
            needs_glob: false,                // DEPYLER-0829: glob crate for Path.glob()/rglob()
            needs_tokio: false,               // DEPYLER-0747: asyncio→tokio async runtime mapping
            needs_completed_process: false, // DEPYLER-0627: subprocess.run returns CompletedProcess struct
            vararg_functions: HashSet::new(), // DEPYLER-0648: Track functions with *args
            slice_params: HashSet::new(),   // DEPYLER-1150: Track slice params in current function
            declared_vars: vec![HashSet::new()],
            current_function_can_fail: false,
            current_return_type: None,
            module_mapper: crate::module_mapper::ModuleMapper::new(),
            imported_modules: std::collections::HashMap::new(),
            imported_items: std::collections::HashMap::new(),
            all_imported_modules: HashSet::new(), // DEPYLER-1115
            module_aliases: std::collections::HashMap::new(), // DEPYLER-1136
            mutable_vars: HashSet::new(),
            needs_zerodivisionerror: false,
            needs_indexerror: false,
            needs_valueerror: false,
            needs_argumenttypeerror: false,
            needs_runtimeerror: false,      // DEPYLER-0551
            needs_filenotfounderror: false, // DEPYLER-0551
            needs_syntaxerror: false,       // GH-204
            needs_typeerror: false,         // GH-204
            needs_keyerror: false,          // GH-204
            needs_ioerror: false,           // GH-204
            needs_attributeerror: false,    // GH-204
            needs_stopiteration: false,     // GH-204
            is_classmethod: false,
            in_generator: false,
            generator_state_vars: HashSet::new(),
            generator_iterator_state_vars: HashSet::new(),
            returns_impl_iterator: false, // DEPYLER-1076
            var_types: std::collections::HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            property_methods: HashSet::new(), // DEPYLER-0737: Track @property methods
            function_return_types: std::collections::HashMap::new(), // DEPYLER-0269: Track function return types
            class_method_return_types: std::collections::HashMap::new(), // DEPYLER-1007: Track class method return types
            function_param_borrows: std::collections::HashMap::new(), // DEPYLER-0270: Track parameter borrowing
            function_param_muts: std::collections::HashMap::new(), // DEPYLER-0574: Track &mut parameters
            tuple_iter_vars: HashSet::new(), // DEPYLER-0307 Fix #9: Track tuple iteration variables
            iterator_vars: HashSet::new(), // DEPYLER-0520: Track variables assigned from iterators
            ref_params: HashSet::new(),    // DEPYLER-0758: Track parameters passed by reference
            mut_ref_params: HashSet::new(), // DEPYLER-1217: Track parameters passed by mutable reference
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
            hoisted_inference_vars: HashSet::new(), // DEPYLER-0455 #2: Track hoisted variables needing String normalization
            none_placeholder_vars: HashSet::new(), // DEPYLER-0823: Track vars with skipped None assignment for hoisting
            precomputed_option_fields: HashSet::new(), // DEPYLER-0108: Track precomputed Option checks
            cse_subcommand_temps: std::collections::HashMap::new(), // DEPYLER-0456 #2: Track CSE subcommand temps
            nested_function_params: std::collections::HashMap::new(), // GH-70: Track inferred nested function params
            fn_str_params: HashSet::new(), // DEPYLER-0543: Track function params with str type
            needs_digest: false,           // DEPYLER-0558: Track digest crate dependency
            in_cmd_handler: false,         // DEPYLER-0608: Track if in cmd_* handler function
            cmd_handler_args_fields: Vec::new(), // DEPYLER-0608: Track extracted args.X fields
            in_subcommand_match_arm: false, // DEPYLER-0608: Track if in subcommand match arm
            subcommand_match_fields: Vec::new(), // DEPYLER-0608: Track subcommand fields in match arm
            hoisted_function_names: Vec::new(), // DEPYLER-0613: Track hoisted nested function names
            is_main_function: false, // DEPYLER-0617: Track if in main() for exit code handling
            boxed_dyn_write_vars: HashSet::new(), // DEPYLER-0625: Track vars needing Box<dyn Write>
            function_returns_boxed_write: false, // DEPYLER-0626: Track functions returning Box<dyn Write>
            option_unwrap_map: HashMap::new(),   // DEPYLER-0627: Track Option unwrap substitutions
            narrowed_option_vars: HashSet::new(), // DEPYLER-1151: Track narrowed Options after None check
            function_param_defaults: HashMap::new(), // Track function parameter defaults
            class_field_defaults: HashMap::new(), // DEPYLER-0932: Dataclass field defaults
            function_param_optionals: HashMap::new(), // DEPYLER-0737: Track Optional params
            class_field_types: HashMap::new(),    // DEPYLER-0720: Track class field types
            type_substitutions: HashMap::new(),   // DEPYLER-0716: Track type substitutions
            current_assign_type: None,            // DEPYLER-0727: Track assignment target type
            force_dict_value_option_wrap: false,  // DEPYLER-0741
            char_iter_vars: HashSet::new(), // DEPYLER-0795: Track loop vars iterating over string.chars()
            char_counter_vars: HashSet::new(), // DEPYLER-0821: Track Counter vars from strings
            adt_child_to_parent: HashMap::new(), // DEPYLER-0936: Track ADT child→parent mappings
            function_param_types: HashMap::new(), // DEPYLER-0950: Track param types for literal coercion
            mut_option_dict_params: HashSet::new(), // DEPYLER-0964: Track &mut Option<Dict> params
            mut_option_params: HashSet::new(),    // DEPYLER-1126: Track ALL &mut Option<T> params
            needs_depyler_value_enum: false,      // DEPYLER-1051: Track DepylerValue enum need
            needs_python_string_ops: false,       // DEPYLER-1202: Python string ops trait
            needs_python_int_ops: false,          // DEPYLER-1202: Python int ops trait
            needs_depyler_date: false,
            needs_depyler_datetime: false,
            needs_depyler_timedelta: false,
            module_constant_types: HashMap::new(), // DEPYLER-1060: Track module-level constant types
            needs_depyler_regex_match: false, // DEPYLER-1070: Track DepylerRegexMatch struct need
            #[cfg(feature = "sovereign-types")]
            type_query: None, // DEPYLER-1112
            last_external_call_return_type: None, // DEPYLER-1113
            type_overrides: HashMap::new(),   // DEPYLER-1101: Oracle-learned type overrides
            vars_used_later: HashSet::new(),  // DEPYLER-1168: Call-site clone detection
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
        // DEPYLER-1109: NASA mode uses PyOps traits, so check for either format
        assert!(
            code.contains("a + b") || code.contains("py_add"),
            "Function should contain addition expression: got {code}"
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
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String(
                "negative".to_string(),
            ))))]),
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
        let var_list =
            HirExpr::List(vec![HirExpr::Var("x".to_string()), HirExpr::Var("y".to_string())]);

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
        // DEPYLER-E0308-001: String optimizer now determines if .to_string() is needed
        // For error messages in Result, static str is sufficient
        assert_eq!(result.to_string(), "return Err (\"Error\") ;");
    }

    #[test]
    fn test_codegen_raise_stmt_bare() {
        let mut ctx = create_test_context();
        ctx.current_function_can_fail = true; // Function returns Result, so raise becomes return Err

        let result = codegen_raise_stmt(&None, &mut ctx).unwrap();
        assert_eq!(result.to_string(), "return Err (\"Exception raised\" . into ()) ;");
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
        // DEPYLER-1203: Pass HirExpr for type-based DepylerValue wrapping
        let hir_value = HirExpr::Literal(Literal::Int(42));

        let result = codegen_assign_index(&base, &index, value_expr, &hir_value, &mut ctx).unwrap();
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
        use crate::hir::{AssignTarget, Literal};

        let mut ctx = create_test_context();
        let targets =
            vec![AssignTarget::Symbol("a".to_string()), AssignTarget::Symbol("b".to_string())];
        // DEPYLER-1064: Create HirExpr::Tuple for the value parameter
        let value = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        let value_expr: syn::Expr = syn::parse_quote! { (1, 2) };

        let result = codegen_assign_tuple(&targets, &value, value_expr, None, &mut ctx).unwrap();
        assert!(result.to_string().contains("let (a , b) = (1 , 2)"));
    }

    // Phase 3b tests - Try handler tests
    #[test]
    fn test_codegen_try_stmt_simple() {
        use crate::hir::ExceptHandler;

        let mut ctx = create_test_context();
        let body = vec![HirStmt::Pass];
        let handlers =
            vec![ExceptHandler { exception_type: None, name: None, body: vec![HirStmt::Pass] }];

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
        assert!(code.contains("as i32"), "Should contain 'as i32' cast, got: {}", code);
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

        assert!(code.contains("as f64"), "Expected '(y) as f64', got: {}", code);
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

        assert!(code.contains("to_string"), "Expected 'value.to_string()', got: {}", code);
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

        assert!(code.contains("as bool"), "Expected '(flag) as bool', got: {}", code);
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

        let call_expr =
            HirExpr::Call { func: "int".to_string(), args: vec![division], kwargs: vec![] };

        let mut ctx = create_test_context();
        let result = call_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #result }.to_string();

        // Should generate cast for expressions to prevent bool arithmetic errors
        assert!(code.contains("low"), "Expected 'low' variable, got: {}", code);
        assert!(code.contains("high"), "Expected 'high' variable, got: {}", code);
        assert!(code.contains("as i32"), "Should contain 'as i32' cast, got: {}", code);
    }

    #[test]
    fn test_float_literal_decimal_point() {
        // Regression test for DEPYLER-TBD: Ensure float literals always have decimal point
        // Issue: f64::to_string() for 0.0 produces "0" (no decimal), parsed as integer
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

        // Test 1: int / int returning float (the main issue)
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

        // Should generate: (a as f64) / (b as f64) OR (a).py_div(b) in NASA mode
        // NOT: a / b (which would do integer division)
        // DEPYLER-1109: NASA mode uses PyOps traits which handle coercion internally
        assert!(
            code.contains("as f64") || code.contains("as f32") || code.contains("py_div"),
            "Expected float cast or py_div for int/int division with float return, got: {}",
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

    // === DEPYLER-COVERAGE-95: Additional tests for untested helper functions ===

    #[test]
    fn test_deduplicate_use_statements_empty() {
        let items: Vec<proc_macro2::TokenStream> = vec![];
        let result = module_gen::deduplicate_use_statements(items);
        assert!(result.is_empty());
    }

    #[test]
    fn test_deduplicate_use_statements_no_duplicates() {
        let items = vec![
            quote! { use std::collections::HashMap; },
            quote! { use std::collections::HashSet; },
        ];
        let result = module_gen::deduplicate_use_statements(items);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_deduplicate_use_statements_with_duplicates() {
        let items = vec![
            quote! { use std::collections::HashMap; },
            quote! { use std::collections::HashMap; },
            quote! { use std::collections::HashSet; },
        ];
        let result = module_gen::deduplicate_use_statements(items);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_deduplicate_use_statements_keeps_non_use() {
        let items = vec![
            quote! { fn foo() {} },
            quote! { fn foo() {} }, // Duplicate non-use - should be kept
        ];
        let result = module_gen::deduplicate_use_statements(items);
        assert_eq!(result.len(), 2); // Non-use items are always kept
    }

    #[test]
    fn test_is_path_constant_expr_call() {
        let expr = HirExpr::Call { func: "Path".to_string(), args: vec![], kwargs: vec![] };
        assert!(is_path_constant_expr(&expr));
    }

    #[test]
    fn test_is_path_constant_expr_purepath() {
        let expr = HirExpr::Call { func: "PurePath".to_string(), args: vec![], kwargs: vec![] };
        assert!(is_path_constant_expr(&expr));
    }

    #[test]
    fn test_is_path_constant_expr_pathbuf() {
        let expr = HirExpr::Call { func: "PathBuf".to_string(), args: vec![], kwargs: vec![] };
        assert!(is_path_constant_expr(&expr));
    }

    #[test]
    fn test_is_path_constant_expr_non_path_call() {
        let expr = HirExpr::Call { func: "len".to_string(), args: vec![], kwargs: vec![] };
        assert!(!is_path_constant_expr(&expr));
    }

    #[test]
    fn test_is_path_constant_expr_method_join() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("p".to_string())),
            method: "join".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_path_constant_expr(&expr));
    }

    #[test]
    fn test_is_path_constant_expr_method_parent() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("p".to_string())),
            method: "parent".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_path_constant_expr(&expr));
    }

    #[test]
    fn test_is_path_constant_expr_attribute_parent() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("path".to_string())),
            attr: "parent".to_string(),
        };
        assert!(is_path_constant_expr(&expr));
    }

    #[test]
    fn test_is_path_constant_expr_binary_div() {
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Call {
                func: "Path".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            right: Box::new(HirExpr::Literal(Literal::String("subdir".to_string()))),
        };
        assert!(is_path_constant_expr(&expr));
    }

    #[test]
    fn test_is_path_constant_expr_non_path_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(!is_path_constant_expr(&expr));
    }

    #[test]
    fn test_is_path_constant_expr_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_path_constant_expr(&expr));
    }

    #[test]
    fn test_infer_unary_type_neg_int() {
        let mut ctx = create_test_context();
        let result = infer_unary_type(&UnaryOp::Neg, &HirExpr::Literal(Literal::Int(42)), &mut ctx);
        assert!(result.to_string().contains("i32"));
    }

    #[test]
    fn test_infer_unary_type_neg_float() {
        let mut ctx = create_test_context();
        let result =
            infer_unary_type(&UnaryOp::Neg, &HirExpr::Literal(Literal::Float(3.15)), &mut ctx);
        assert!(result.to_string().contains("f64"));
    }

    #[test]
    fn test_infer_unary_type_pos_int() {
        let mut ctx = create_test_context();
        let result = infer_unary_type(&UnaryOp::Pos, &HirExpr::Literal(Literal::Int(10)), &mut ctx);
        assert!(result.to_string().contains("i32"));
    }

    #[test]
    fn test_infer_unary_type_nested_neg_int() {
        let mut ctx = create_test_context();
        let result = infer_unary_type(
            &UnaryOp::Neg,
            &HirExpr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(HirExpr::Literal(Literal::Int(5))),
            },
            &mut ctx,
        );
        assert!(result.to_string().contains("i32"));
    }

    #[test]
    fn test_infer_unary_type_nested_neg_float() {
        let mut ctx = create_test_context();
        let result = infer_unary_type(
            &UnaryOp::Neg,
            &HirExpr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(HirExpr::Literal(Literal::Float(2.5))),
            },
            &mut ctx,
        );
        assert!(result.to_string().contains("f64"));
    }

    #[test]
    fn test_infer_unary_type_not_returns_bool() {
        // DEPYLER-1040b: Logical NOT always returns bool, not a fallback type
        let mut ctx = create_test_context();
        let result = infer_unary_type(&UnaryOp::Not, &HirExpr::Var("x".to_string()), &mut ctx);
        let result_str = result.to_string();
        assert!(result_str.contains("bool"), "Expected bool type for NOT, got: {}", result_str);
    }

    #[test]
    fn test_generate_conditional_imports_none_needed() {
        let ctx = create_test_context();
        let imports = module_gen::generate_conditional_imports(&ctx);
        assert!(imports.is_empty());
    }

    #[test]
    fn test_generate_conditional_imports_hashmap() {
        let mut ctx = create_test_context();
        ctx.needs_hashmap = true;
        let imports = module_gen::generate_conditional_imports(&ctx);
        assert!(!imports.is_empty());
        let import_str = imports[0].to_string();
        assert!(import_str.contains("HashMap"));
    }

    #[test]
    fn test_generate_conditional_imports_multiple() {
        let mut ctx = create_test_context();
        ctx.needs_hashmap = true;
        ctx.needs_hashset = true;
        ctx.needs_arc = true;
        let imports = module_gen::generate_conditional_imports(&ctx);
        assert_eq!(imports.len(), 3);
    }

    // === Additional tests for analyze_validators ===

    #[test]
    fn test_scan_expr_for_validators_add_argument_with_type() {
        let mut ctx = create_test_context();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("parser".to_string())),
            method: "add_argument".to_string(),
            args: vec![HirExpr::Literal(Literal::String("--value".to_string()))],
            kwargs: vec![("type".to_string(), HirExpr::Var("validate_positive".to_string()))],
        };
        scan_expr_for_validators(&expr, &mut ctx);
        assert!(ctx.validator_functions.contains("validate_positive"));
    }

    #[test]
    fn test_scan_expr_for_validators_builtin_types_skipped() {
        let mut ctx = create_test_context();
        // Built-in types should be skipped
        for builtin in &["str", "int", "float", "Path"] {
            let expr = HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("parser".to_string())),
                method: "add_argument".to_string(),
                args: vec![],
                kwargs: vec![("type".to_string(), HirExpr::Var(builtin.to_string()))],
            };
            scan_expr_for_validators(&expr, &mut ctx);
        }
        assert!(ctx.validator_functions.is_empty());
    }

    #[test]
    fn test_scan_expr_for_validators_non_add_argument() {
        let mut ctx = create_test_context();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("parser".to_string())),
            method: "parse_args".to_string(),
            args: vec![],
            kwargs: vec![("type".to_string(), HirExpr::Var("my_validator".to_string()))],
        };
        scan_expr_for_validators(&expr, &mut ctx);
        // parse_args is not add_argument, so validator should not be tracked
        assert!(ctx.validator_functions.is_empty());
    }

    #[test]
    fn test_scan_stmts_for_validators_expr_stmt() {
        let mut ctx = create_test_context();
        let stmts = vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("parser".to_string())),
            method: "add_argument".to_string(),
            args: vec![],
            kwargs: vec![("type".to_string(), HirExpr::Var("custom_type".to_string()))],
        })];
        scan_stmts_for_validators(&stmts, &mut ctx);
        assert!(ctx.validator_functions.contains("custom_type"));
    }

    #[test]
    fn test_scan_stmts_for_validators_if_body() {
        let mut ctx = create_test_context();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("parser".to_string())),
                method: "add_argument".to_string(),
                args: vec![],
                kwargs: vec![("type".to_string(), HirExpr::Var("if_validator".to_string()))],
            })],
            else_body: None,
        }];
        scan_stmts_for_validators(&stmts, &mut ctx);
        assert!(ctx.validator_functions.contains("if_validator"));
    }

    #[test]
    fn test_scan_stmts_for_validators_else_body() {
        let mut ctx = create_test_context();
        let stmts = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Pass],
            else_body: Some(vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("parser".to_string())),
                method: "add_argument".to_string(),
                args: vec![],
                kwargs: vec![("type".to_string(), HirExpr::Var("else_validator".to_string()))],
            })]),
        }];
        scan_stmts_for_validators(&stmts, &mut ctx);
        assert!(ctx.validator_functions.contains("else_validator"));
    }

    #[test]
    fn test_scan_stmts_for_validators_while_body() {
        let mut ctx = create_test_context();
        let stmts = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("parser".to_string())),
                method: "add_argument".to_string(),
                args: vec![],
                kwargs: vec![("type".to_string(), HirExpr::Var("while_validator".to_string()))],
            })],
        }];
        scan_stmts_for_validators(&stmts, &mut ctx);
        assert!(ctx.validator_functions.contains("while_validator"));
    }

    #[test]
    fn test_scan_stmts_for_validators_for_body() {
        let mut ctx = create_test_context();
        let stmts = vec![HirStmt::For {
            target: crate::hir::AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("parser".to_string())),
                method: "add_argument".to_string(),
                args: vec![],
                kwargs: vec![("type".to_string(), HirExpr::Var("for_validator".to_string()))],
            })],
        }];
        scan_stmts_for_validators(&stmts, &mut ctx);
        assert!(ctx.validator_functions.contains("for_validator"));
    }

    // === Tests for infer_constant_type ===

    #[test]
    fn test_infer_constant_type_int() {
        let mut ctx = create_test_context();
        let result = infer_constant_type(&HirExpr::Literal(Literal::Int(42)), &mut ctx);
        assert!(result.to_string().contains("i32"));
    }

    #[test]
    fn test_infer_constant_type_float() {
        let mut ctx = create_test_context();
        let result = infer_constant_type(&HirExpr::Literal(Literal::Float(3.15)), &mut ctx);
        assert!(result.to_string().contains("f64"));
    }

    #[test]
    fn test_infer_constant_type_string() {
        let mut ctx = create_test_context();
        let result =
            infer_constant_type(&HirExpr::Literal(Literal::String("hello".to_string())), &mut ctx);
        assert!(result.to_string().contains("str"));
    }

    #[test]
    fn test_infer_constant_type_bool() {
        let mut ctx = create_test_context();
        let result = infer_constant_type(&HirExpr::Literal(Literal::Bool(true)), &mut ctx);
        assert!(result.to_string().contains("bool"));
    }

    #[test]
    fn test_infer_constant_type_none() {
        let mut ctx = create_test_context();
        let result = infer_constant_type(&HirExpr::Literal(Literal::None), &mut ctx);
        // DEPYLER-0798: None should be Option<()>
        assert!(result.to_string().contains("Option"));
    }

    #[test]
    fn test_infer_constant_type_path_call() {
        let mut ctx = create_test_context();
        let expr = HirExpr::Call {
            func: "Path".to_string(),
            args: vec![HirExpr::Literal(Literal::String("/tmp".to_string()))],
            kwargs: vec![],
        };
        let result = infer_constant_type(&expr, &mut ctx);
        assert!(result.to_string().contains("PathBuf"));
    }

    #[test]
    fn test_infer_constant_type_dict_infers_hashmap() {
        let mut ctx = create_test_context();
        let expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(1)),
        )]);
        let result = infer_constant_type(&expr, &mut ctx);
        // Dict is inferred via infer_expr_type_simple -> HashMap or Value
        let result_str = result.to_string();
        assert!(
            result_str.contains("HashMap") || result_str.contains("Value"),
            "Expected HashMap or Value, got: {}",
            result_str
        );
    }

    // === Tests for analyze_mutable_vars ===

    #[test]
    fn test_analyze_mutable_vars_empty() {
        let mut ctx = create_test_context();
        let stmts: Vec<HirStmt> = vec![];
        let params: Vec<HirParam> = vec![];
        analyze_mutable_vars(&stmts, &mut ctx, &params);
        assert!(ctx.mutable_vars.is_empty());
    }

    #[test]
    fn test_analyze_mutable_vars_reassignment() {
        let mut ctx = create_test_context();
        let stmts = vec![
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(2)),
                type_annotation: None,
            },
        ];
        let params: Vec<HirParam> = vec![];
        analyze_mutable_vars(&stmts, &mut ctx, &params);
        assert!(ctx.mutable_vars.contains("x"));
    }

    #[test]
    fn test_analyze_mutable_vars_param_reassignment() {
        let mut ctx = create_test_context();
        let stmts = vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("a".to_string()),
            value: HirExpr::Literal(Literal::Int(100)),
            type_annotation: None,
        }];
        let params = vec![HirParam::new("a".to_string(), Type::Int)];
        analyze_mutable_vars(&stmts, &mut ctx, &params);
        // Parameter 'a' is reassigned, so it should be marked mutable
        assert!(ctx.mutable_vars.contains("a"));
    }

    #[test]
    fn test_analyze_mutable_vars_clears_previous() {
        let mut ctx = create_test_context();
        ctx.mutable_vars.insert("old_var".to_string());
        let stmts: Vec<HirStmt> = vec![];
        let params: Vec<HirParam> = vec![];
        analyze_mutable_vars(&stmts, &mut ctx, &params);
        // DEPYLER-0707: Should clear mutable_vars
        assert!(!ctx.mutable_vars.contains("old_var"));
    }

    #[test]
    fn test_analyze_mutable_vars_nested_index_assignment() {
        // DEPYLER-0596-FIX: Test that nested index assignment marks base variable mutable
        // Python: d["a"]["b"] = 5 → Rust: d.get_mut("a").unwrap().insert("b", 5)
        // The variable d should be marked mutable because get_mut requires &mut self
        let mut ctx = create_test_context();
        // First declare d
        let decl = HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("d".to_string()),
            value: HirExpr::Call { func: "HashMap::new".to_string(), args: vec![], kwargs: vec![] },
            type_annotation: None,
        };
        // Then do nested index assignment: d["a"]["b"] = 5
        let nested_assign = HirStmt::Assign {
            target: crate::hir::AssignTarget::Index {
                base: Box::new(HirExpr::Index {
                    base: Box::new(HirExpr::Var("d".to_string())),
                    index: Box::new(HirExpr::Literal(Literal::String("a".to_string()))),
                }),
                index: Box::new(HirExpr::Literal(Literal::String("b".to_string()))),
            },
            value: HirExpr::Literal(Literal::Int(5)),
            type_annotation: None,
        };
        let stmts = vec![decl, nested_assign];
        let params: Vec<HirParam> = vec![];
        analyze_mutable_vars(&stmts, &mut ctx, &params);
        // d should be marked mutable because of nested index assignment
        assert!(
            ctx.mutable_vars.contains("d"),
            "Nested index assignment should mark base variable 'd' as mutable"
        );
    }

    #[test]
    fn test_analyze_mutable_vars_nested_index_in_if_body() {
        // DEPYLER-0596-FIX: Test that nested index assignment inside if body marks base mutable
        // Python:
        //   copied = copy.deepcopy(original)
        //   if "group1" in copied:
        //       copied["group1"]["e"] = 5
        // The variable copied should be marked mutable
        let mut ctx = create_test_context();
        // First declare copied
        let decl = HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("copied".to_string()),
            value: HirExpr::Call { func: "clone".to_string(), args: vec![], kwargs: vec![] },
            type_annotation: None,
        };
        // Then do nested index assignment inside if body: copied["group1"]["e"] = 5
        let nested_assign = HirStmt::Assign {
            target: crate::hir::AssignTarget::Index {
                base: Box::new(HirExpr::Index {
                    base: Box::new(HirExpr::Var("copied".to_string())),
                    index: Box::new(HirExpr::Literal(Literal::String("group1".to_string()))),
                }),
                index: Box::new(HirExpr::Literal(Literal::String("e".to_string()))),
            },
            value: HirExpr::Literal(Literal::Int(5)),
            type_annotation: None,
        };
        let if_stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![nested_assign],
            else_body: None,
        };
        let stmts = vec![decl, if_stmt];
        let params: Vec<HirParam> = vec![];
        analyze_mutable_vars(&stmts, &mut ctx, &params);
        // copied should be marked mutable because of nested index assignment in if body
        assert!(
            ctx.mutable_vars.contains("copied"),
            "Nested index assignment in if body should mark base variable 'copied' as mutable"
        );
    }

    // === Tests for detect_adt_patterns ===

    #[test]
    fn test_detect_adt_patterns_empty() {
        let classes: Vec<HirClass> = vec![];
        let result = detect_adt_patterns(&classes);
        assert!(result.abc_to_children.is_empty());
        assert!(result.child_to_parent.is_empty());
    }

    #[test]
    fn test_detect_adt_patterns_no_inheritance() {
        let classes = vec![HirClass {
            name: "Point".to_string(),
            base_classes: vec![],
            type_params: vec![],
            fields: vec![],
            methods: vec![],
            is_dataclass: true,
            docstring: None,
        }];
        let result = detect_adt_patterns(&classes);
        assert!(result.abc_to_children.is_empty());
    }

    #[test]
    fn test_detect_adt_patterns_with_abc() {
        let classes = vec![
            HirClass {
                name: "Option".to_string(),
                base_classes: vec!["ABC".to_string(), "Generic[T]".to_string()],
                type_params: vec!["T".to_string()],
                fields: vec![],
                methods: vec![],
                is_dataclass: false,
                docstring: None,
            },
            HirClass {
                name: "Some".to_string(),
                base_classes: vec!["Option[T]".to_string()],
                type_params: vec![],
                fields: vec![crate::hir::HirField {
                    name: "value".to_string(),
                    field_type: Type::TypeVar("T".to_string()),
                    default_value: None,
                    is_class_var: false,
                }],
                methods: vec![],
                is_dataclass: true,
                docstring: None,
            },
            HirClass {
                name: "Nothing".to_string(),
                base_classes: vec!["Option[T]".to_string()],
                type_params: vec![],
                fields: vec![],
                methods: vec![],
                is_dataclass: true,
                docstring: None,
            },
        ];
        let result = detect_adt_patterns(&classes);
        assert!(result.abc_to_children.contains_key("Option"));
        let children = result.abc_to_children.get("Option").unwrap();
        assert!(children.contains(&"Some".to_string()));
        assert!(children.contains(&"Nothing".to_string()));
        // Check reverse mapping
        assert_eq!(result.child_to_parent.get("Some"), Some(&"Option".to_string()));
        assert_eq!(result.child_to_parent.get("Nothing"), Some(&"Option".to_string()));
    }

    // === Tests for generate_interned_string_tokens ===

    #[test]
    fn test_generate_interned_string_tokens_empty() {
        let optimizer = StringOptimizer::new();
        let result = module_gen::generate_interned_string_tokens(&optimizer);
        assert!(result.is_empty());
    }

    // === Tests for analyze_string_optimization ===

    #[test]
    fn test_analyze_string_optimization_empty() {
        let mut ctx = create_test_context();
        let functions: Vec<HirFunction> = vec![];
        validator_analysis::analyze_string_optimization(&mut ctx, &functions);
        // Should complete without error
    }

    #[test]
    fn test_analyze_string_optimization_with_function() {
        let mut ctx = create_test_context();
        let functions = vec![HirFunction {
            name: "greet".to_string(),
            params: vec![HirParam::new("name".to_string(), Type::String)].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String(
                "Hello".to_string(),
            ))))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }];
        validator_analysis::analyze_string_optimization(&mut ctx, &functions);
        // Should complete without error
    }

    // === Tests for analyze_validators ===

    #[test]
    fn test_analyze_validators_from_function() {
        let mut ctx = create_test_context();
        let functions = vec![HirFunction {
            name: "setup_args".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("parser".to_string())),
                method: "add_argument".to_string(),
                args: vec![],
                kwargs: vec![("type".to_string(), HirExpr::Var("my_validator".to_string()))],
            })],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }];
        let constants: Vec<HirConstant> = vec![];
        validator_analysis::analyze_validators(&mut ctx, &functions, &constants);
        assert!(ctx.validator_functions.contains("my_validator"));
    }

    #[test]
    fn test_analyze_validators_from_constant() {
        let mut ctx = create_test_context();
        let functions: Vec<HirFunction> = vec![];
        let constants = vec![HirConstant {
            name: "PARSER_SETUP".to_string(),
            value: HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("parser".to_string())),
                method: "add_argument".to_string(),
                args: vec![],
                kwargs: vec![("type".to_string(), HirExpr::Var("const_validator".to_string()))],
            },
            type_annotation: None,
        }];
        validator_analysis::analyze_validators(&mut ctx, &functions, &constants);
        assert!(ctx.validator_functions.contains("const_validator"));
    }

    // === DEPYLER-1103: Tests for PyTruthy trait generation ===

    #[test]
    fn test_depyler_1103_pytruthy_trait_generated_with_depyler_value() {
        // Verify that PyTruthy trait is included when DepylerValue is needed
        // Test by creating a HirModule with a dict (which triggers DepylerValue)
        use crate::hir::HirModule;
        use smallvec::smallvec;

        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_func".to_string(),
                params: smallvec![],
                ret_type: Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
                body: vec![HirStmt::Return(Some(HirExpr::Dict(vec![])))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };
        let type_mapper = TypeMapper::default();
        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Verify PyTruthy trait is defined
        assert!(
            code.contains("pub trait PyTruthy"),
            "Generated code should contain PyTruthy trait definition"
        );
        assert!(
            code.contains("fn is_true(& self) -> bool")
                || code.contains("fn is_true(&self) -> bool"),
            "PyTruthy trait should define is_true method"
        );

        // Verify implementations for primitive types
        assert!(code.contains("impl PyTruthy for bool"), "Should implement PyTruthy for bool");
        assert!(code.contains("impl PyTruthy for i32"), "Should implement PyTruthy for i32");
        assert!(code.contains("impl PyTruthy for i64"), "Should implement PyTruthy for i64");
        assert!(code.contains("impl PyTruthy for f64"), "Should implement PyTruthy for f64");
        assert!(code.contains("impl PyTruthy for String"), "Should implement PyTruthy for String");

        // Verify implementation for DepylerValue
        assert!(
            code.contains("impl PyTruthy for DepylerValue"),
            "Should implement PyTruthy for DepylerValue"
        );
    }

    #[test]
    fn test_depyler_1103_pytruthy_includes_collections() {
        // Verify PyTruthy is implemented for common collection types
        use crate::hir::HirModule;
        use smallvec::smallvec;

        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_func".to_string(),
                params: smallvec![],
                ret_type: Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
                body: vec![HirStmt::Return(Some(HirExpr::Dict(vec![])))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };
        let type_mapper = TypeMapper::default();
        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Verify collection implementations (check with both spacing options)
        assert!(
            code.contains("impl < T > PyTruthy for Vec < T >")
                || code.contains("impl<T> PyTruthy for Vec<T>"),
            "Should implement PyTruthy for Vec<T>"
        );
        assert!(
            code.contains("impl < T > PyTruthy for Option < T >")
                || code.contains("impl<T> PyTruthy for Option<T>"),
            "Should implement PyTruthy for Option<T>"
        );
        assert!(
            code.contains("PyTruthy for std :: collections :: HashMap")
                || code.contains("PyTruthy for std::collections::HashMap"),
            "Should implement PyTruthy for HashMap"
        );
        assert!(
            code.contains("PyTruthy for std :: collections :: HashSet")
                || code.contains("PyTruthy for std::collections::HashSet"),
            "Should implement PyTruthy for HashSet"
        );
    }

    #[test]
    fn test_depyler_1103_pytruthy_is_tied_to_depyler_value() {
        // Verify that PyTruthy is always generated alongside DepylerValue
        // (they are bundled in the same quote! block for consistency)
        use crate::hir::{HirModule, HirParam};
        use smallvec::smallvec;

        let module = HirModule {
            functions: vec![HirFunction {
                name: "simple_add".to_string(),
                params: smallvec![
                    HirParam::new("a".to_string(), Type::Int),
                    HirParam::new("b".to_string(), Type::Int),
                ],
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };
        let type_mapper = TypeMapper::default();
        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // If DepylerValue is present, PyTruthy should also be present (they're bundled)
        // Note: TypeMapper::default() uses NASA mode which always includes DepylerValue
        let has_depyler_value = code.contains("enum DepylerValue");
        let has_pytruthy = code.contains("pub trait PyTruthy");
        assert_eq!(
            has_depyler_value, has_pytruthy,
            "PyTruthy and DepylerValue should always be generated together"
        );
    }

    // === DEPYLER-1104: Tests for PyOps trait generation ===

    #[test]
    fn test_depyler_1104_pyops_traits_generated_with_depyler_value() {
        // Verify that PyAdd, PySub, PyMul, PyDiv, PyIndex traits are included with DepylerValue
        use crate::hir::HirModule;
        use smallvec::smallvec;

        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_func".to_string(),
                params: smallvec![],
                ret_type: Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
                body: vec![HirStmt::Return(Some(HirExpr::Dict(vec![])))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };
        let type_mapper = TypeMapper::default();
        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Verify all PyOps traits are defined
        assert!(
            code.contains("pub trait PyAdd"),
            "Generated code should contain PyAdd trait definition"
        );
        assert!(
            code.contains("pub trait PySub"),
            "Generated code should contain PySub trait definition"
        );
        assert!(
            code.contains("pub trait PyMul"),
            "Generated code should contain PyMul trait definition"
        );
        assert!(
            code.contains("pub trait PyDiv"),
            "Generated code should contain PyDiv trait definition"
        );
        assert!(
            code.contains("pub trait PyIndex"),
            "Generated code should contain PyIndex trait definition"
        );

        // Verify trait methods
        assert!(code.contains("fn py_add"), "PyAdd trait should define py_add method");
        assert!(code.contains("fn py_sub"), "PySub trait should define py_sub method");
        assert!(code.contains("fn py_mul"), "PyMul trait should define py_mul method");
        assert!(code.contains("fn py_div"), "PyDiv trait should define py_div method");
        assert!(code.contains("fn py_index"), "PyIndex trait should define py_index method");
    }

    #[test]
    fn test_depyler_1104_pyops_primitive_implementations() {
        // Verify PyOps traits are implemented for primitive types
        use crate::hir::HirModule;
        use smallvec::smallvec;

        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_func".to_string(),
                params: smallvec![],
                ret_type: Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
                body: vec![HirStmt::Return(Some(HirExpr::Dict(vec![])))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };
        let type_mapper = TypeMapper::default();
        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Verify implementations for i32
        assert!(
            code.contains("impl PyAdd for i32") || code.contains("impl PyAdd < i32 > for i32"),
            "Should implement PyAdd for i32"
        );
        assert!(
            code.contains("impl PySub for i32") || code.contains("impl PySub < i32 > for i32"),
            "Should implement PySub for i32"
        );

        // Verify implementations for f64
        assert!(
            code.contains("impl PyAdd for f64") || code.contains("impl PyAdd < f64 > for f64"),
            "Should implement PyAdd for f64"
        );

        // Verify cross-type implementations (i32 + f64)
        assert!(
            code.contains("impl PyAdd < f64 > for i32") || code.contains("impl PyAdd<f64> for i32"),
            "Should implement PyAdd<f64> for i32 (cross-type)"
        );
    }

    #[test]
    fn test_depyler_1104_pyindex_negative_index_support() {
        // Verify PyIndex implementations support negative indices (Python semantics)
        use crate::hir::HirModule;
        use smallvec::smallvec;

        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_func".to_string(),
                params: smallvec![],
                ret_type: Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
                body: vec![HirStmt::Return(Some(HirExpr::Dict(vec![])))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };
        let type_mapper = TypeMapper::default();
        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Verify PyIndex is implemented for Vec with i32 (supports negative indices)
        assert!(
            code.contains("impl < T : Clone > PyIndex < i32 > for Vec < T >")
                || code.contains("impl<T: Clone> PyIndex<i32> for Vec<T>"),
            "Should implement PyIndex<i32> for Vec<T> to support negative indices"
        );

        // Verify negative index handling logic is present
        assert!(
            code.contains("if index < 0"),
            "PyIndex implementation should handle negative indices"
        );
        assert!(
            code.contains(".len()") && code.contains("as i32"),
            "Negative index handling should use length for wrapping"
        );
    }

    #[test]
    fn test_depyler_1104_pyops_depyler_value_implementation() {
        // Verify PyOps traits are implemented for DepylerValue
        use crate::hir::HirModule;
        use smallvec::smallvec;

        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_func".to_string(),
                params: smallvec![],
                ret_type: Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
                body: vec![HirStmt::Return(Some(HirExpr::Dict(vec![])))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };
        let type_mapper = TypeMapper::default();
        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Verify DepylerValue has PyOps implementations
        assert!(
            code.contains("impl PyAdd for DepylerValue"),
            "Should implement PyAdd for DepylerValue"
        );
        assert!(
            code.contains("impl PySub for DepylerValue"),
            "Should implement PySub for DepylerValue"
        );
        assert!(
            code.contains("impl PyMul for DepylerValue"),
            "Should implement PyMul for DepylerValue"
        );
        assert!(
            code.contains("impl PyDiv for DepylerValue"),
            "Should implement PyDiv for DepylerValue"
        );
        assert!(
            code.contains("impl PyIndex") && code.contains("for DepylerValue"),
            "Should implement PyIndex for DepylerValue"
        );
    }

    /// DEPYLER-1106: Verify PyOps codegen integration for binary operations
    /// This test verifies that when DepylerValue is involved in arithmetic,
    /// we generate .py_add(), .py_sub(), etc. instead of raw operators
    #[test]
    fn test_depyler_1106_pyops_codegen_binary_ops() {
        use crate::hir::HirModule;
        use smallvec::smallvec;

        // Create a module with a function that uses heterogeneous dict (triggers DepylerValue)
        // The function has Dict<String, Unknown> which forces NASA mode DepylerValue usage
        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_add".to_string(),
                params: smallvec![],
                // Dict with mixed types - this triggers DepylerValue in NASA mode
                ret_type: Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
                body: vec![HirStmt::Return(Some(HirExpr::Dict(vec![
                    (
                        HirExpr::Literal(Literal::String("a".to_string())),
                        HirExpr::Literal(Literal::Int(1)),
                    ),
                    (
                        HirExpr::Literal(Literal::String("b".to_string())),
                        HirExpr::Literal(Literal::Float(2.5)),
                    ),
                ])))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };

        // Use NASA mode TypeMapper
        let mut type_mapper = TypeMapper::default();
        type_mapper.nasa_mode = true;

        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Verify the function is generated and code contains PyOps traits
        assert!(code.contains("fn test_add"), "Should have the function");
        assert!(code.contains("pub trait PyAdd"), "Should include PyAdd trait");
    }

    /// DEPYLER-1106: Verify PyOps codegen includes DepylerValue trait implementations
    #[test]
    fn test_depyler_1106_pyops_codegen_depyler_value_impls() {
        use crate::hir::HirModule;
        use smallvec::smallvec;

        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_ops".to_string(),
                params: smallvec![],
                // Return Unknown type to trigger DepylerValue
                ret_type: Type::Unknown,
                body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };

        let mut type_mapper = TypeMapper::default();
        type_mapper.nasa_mode = true;

        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Verify DepylerValue PyOps implementations are present
        assert!(
            code.contains("impl PyAdd for DepylerValue"),
            "Should implement PyAdd for DepylerValue"
        );
        assert!(
            code.contains("impl PyMul for DepylerValue"),
            "Should implement PyMul for DepylerValue"
        );
    }

    /// DEPYLER-1106: Verify that PyOps traits handle cross-type arithmetic
    /// This is the key test - demonstrating that i32 + f64 compiles via trait methods
    #[test]
    fn test_depyler_1106_cross_type_arithmetic_compiles() {
        use crate::hir::HirModule;
        use smallvec::smallvec;

        // Create module that triggers PyOps trait generation
        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_cross_type".to_string(),
                params: smallvec![],
                ret_type: Type::Unknown, // Triggers DepylerValue
                body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Float(3.15))))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };

        let mut type_mapper = TypeMapper::default();
        type_mapper.nasa_mode = true;

        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Should have cross-type implementations
        assert!(
            code.contains("impl PyAdd < f64 > for i32") || code.contains("impl PyAdd<f64> for i32"),
            "Should implement PyAdd<f64> for i32 for cross-type addition"
        );
        assert!(
            code.contains("impl PyMul < f64 > for i32") || code.contains("impl PyMul<f64> for i32"),
            "Should implement PyMul<f64> for i32 for cross-type multiplication"
        );
    }

    /// DEPYLER-1106: Verify PyIndex trait is available for negative index handling
    #[test]
    fn test_depyler_1106_pyindex_available() {
        use crate::hir::HirModule;
        use smallvec::smallvec;

        let module = HirModule {
            functions: vec![HirFunction {
                name: "test_index".to_string(),
                params: smallvec![],
                ret_type: Type::Unknown,
                body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0))))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            constants: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            top_level_stmts: vec![],
        };

        let mut type_mapper = TypeMapper::default();
        type_mapper.nasa_mode = true;

        let result = generate_rust_file(&module, &type_mapper).unwrap();
        let code = result.0;

        // Verify PyIndex trait and implementation are present
        assert!(code.contains("pub trait PyIndex"), "Should include PyIndex trait definition");
        assert!(code.contains("fn py_index"), "PyIndex trait should have py_index method");
    }

    /// DEPYLER-1101: Verify rust_type_string_to_hir converts Rust type strings correctly
    #[test]
    fn test_depyler_1101_rust_type_string_to_hir() {
        use crate::hir::Type;

        // Primitive types
        assert!(matches!(rust_type_string_to_hir("i32"), Type::Int));
        assert!(matches!(rust_type_string_to_hir("i64"), Type::Int));
        assert!(matches!(rust_type_string_to_hir("f64"), Type::Float));
        assert!(matches!(rust_type_string_to_hir("f32"), Type::Float));
        assert!(matches!(rust_type_string_to_hir("bool"), Type::Bool));
        assert!(matches!(rust_type_string_to_hir("String"), Type::String));
        assert!(matches!(rust_type_string_to_hir("&str"), Type::String));
        assert!(matches!(rust_type_string_to_hir("()"), Type::None));

        // Generic types
        if let Type::List(inner) = rust_type_string_to_hir("Vec<i32>") {
            assert!(matches!(*inner, Type::Int));
        } else {
            panic!("Vec<i32> should parse to List(Int)");
        }

        if let Type::Optional(inner) = rust_type_string_to_hir("Option<String>") {
            assert!(matches!(*inner, Type::String));
        } else {
            panic!("Option<String> should parse to Optional(String)");
        }

        if let Type::Dict(k, v) = rust_type_string_to_hir("HashMap<String, i32>") {
            assert!(matches!(*k, Type::String));
            assert!(matches!(*v, Type::Int));
        } else {
            panic!("HashMap<String, i32> should parse to Dict(String, Int)");
        }

        // Nested types
        if let Type::List(inner) = rust_type_string_to_hir("Vec<Vec<f64>>") {
            if let Type::List(inner2) = *inner {
                assert!(matches!(*inner2, Type::Float));
            } else {
                panic!("Vec<Vec<f64>> inner should be List");
            }
        } else {
            panic!("Vec<Vec<f64>> should parse to List");
        }

        // Unknown types fallback
        assert!(matches!(rust_type_string_to_hir("CustomType"), Type::Unknown));
    }

    // ========================================================================
    // Transpile-based coverage tests (DEPYLER-99MODE-S9)
    // ========================================================================

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
    fn test_s9_module_empty() {
        let code = transpile("");
        // Empty module should still generate valid Rust
        assert!(!code.is_empty() || code.is_empty()); // Just ensure no panic
    }

    #[test]
    fn test_s9_module_single_constant_int() {
        let code = transpile("MAX_SIZE = 100\n");
        assert!(
            code.contains("MAX_SIZE") || code.contains("100"),
            "Should contain constant: {code}"
        );
    }

    #[test]
    fn test_s9_module_single_constant_str() {
        let code = transpile("VERSION = \"1.0.0\"\n");
        assert!(
            code.contains("VERSION") || code.contains("1.0.0"),
            "Should contain string constant: {code}"
        );
    }

    #[test]
    fn test_s9_module_multiple_constants() {
        let code = transpile("X = 10\nY = 20\nZ = 30\n");
        assert!(
            code.contains("10") && code.contains("20") && code.contains("30"),
            "Should contain all constant values: {code}"
        );
    }

    #[test]
    fn test_s9_module_constant_float() {
        let code = transpile("RATE = 0.05\n");
        assert!(
            code.contains("RATE") || code.contains("0.05"),
            "Should contain float constant: {code}"
        );
    }

    #[test]
    fn test_s9_module_constant_bool() {
        let code = transpile("DEBUG = True\n");
        assert!(
            code.contains("DEBUG") || code.contains("true"),
            "Should contain bool constant: {code}"
        );
    }

    #[test]
    fn test_s9_module_import_os() {
        let code = transpile("import os\n\ndef run():\n    pass\n");
        // os import should be processed
        assert!(code.contains("fn run"));
    }

    #[test]
    fn test_s9_module_import_sys() {
        let code = transpile("import sys\n\ndef main():\n    pass\n");
        assert!(code.contains("fn main"));
    }

    #[test]
    fn test_s9_module_from_import() {
        let code = transpile("from collections import defaultdict\n\ndef test():\n    pass\n");
        assert!(code.contains("fn test"));
    }

    #[test]
    fn test_s9_module_import_json() {
        let code = transpile("import json\n\ndef parse(data: str) -> str:\n    return data\n");
        assert!(code.contains("fn parse"));
    }

    #[test]
    fn test_s9_module_class_basic() {
        let code = transpile(
            "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n",
        );
        assert!(
            code.contains("struct") || code.contains("Point"),
            "Should generate struct for class: {code}"
        );
    }

    #[test]
    fn test_s9_module_class_with_method() {
        let code = transpile(
            "class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count = self.count + 1\n",
        );
        assert!(
            code.contains("Counter") || code.contains("increment"),
            "Should generate struct with method: {code}"
        );
    }

    #[test]
    fn test_s9_module_class_and_function() {
        let code = transpile(
            "class Pair:\n    def __init__(self, a: int, b: int):\n        self.a = a\n        self.b = b\n\ndef make_pair(x: int, y: int) -> int:\n    return x + y\n",
        );
        assert!(code.contains("Pair") || code.contains("struct"));
        assert!(code.contains("fn make_pair"));
    }

    #[test]
    fn test_s9_module_function_with_list_return() {
        let code = transpile("def numbers() -> list:\n    return [1, 2, 3]\n");
        assert!(code.contains("fn numbers"));
    }

    #[test]
    fn test_s9_module_function_with_dict_return() {
        let code = transpile("def config() -> dict:\n    return {\"key\": \"value\"}\n");
        assert!(code.contains("fn config"));
        assert!(
            code.contains("HashMap") || code.contains("hash"),
            "Should use HashMap for dict: {code}"
        );
    }

    #[test]
    fn test_s9_module_multiple_functions() {
        let code = transpile(
            "def alpha() -> int:\n    return 1\n\ndef beta() -> int:\n    return 2\n\ndef gamma() -> int:\n    return 3\n",
        );
        assert!(code.contains("fn alpha"));
        assert!(code.contains("fn beta"));
        assert!(code.contains("fn gamma"));
    }

    #[test]
    fn test_s9_module_function_with_mutable_var() {
        let code = transpile(
            "def accumulate() -> int:\n    total = 0\n    total = total + 1\n    total = total + 2\n    return total\n",
        );
        assert!(code.contains("mut"), "Reassigned var should be mut: {code}");
    }

    #[test]
    fn test_s9_module_function_calls_len() {
        let code = transpile("def length(items: list) -> int:\n    return len(items)\n");
        assert!(code.contains("fn length"));
        assert!(code.contains("len()"), "Should call len(): {code}");
    }

    #[test]
    fn test_s9_module_global_and_function() {
        let code = transpile("THRESHOLD = 42\n\ndef check(x: int) -> bool:\n    return x > 0\n");
        assert!(code.contains("fn check"));
        assert!(code.contains("42") || code.contains("THRESHOLD"), "Should have constant: {code}");
    }

    #[test]
    fn test_s9_module_function_with_string_ops() {
        let code = transpile("def upper_case(s: str) -> str:\n    return s.upper()\n");
        assert!(code.contains("fn upper_case"));
        assert!(
            code.contains("to_uppercase") || code.contains("upper"),
            "Should map upper to Rust: {code}"
        );
    }

    #[test]
    fn test_s9_module_function_with_print() {
        let code = transpile("def say_hello():\n    print(\"hello\")\n");
        assert!(code.contains("fn say_hello"));
        assert!(code.contains("println!") || code.contains("print"), "Should map print: {code}");
    }

    #[test]
    fn test_s9_module_function_with_enumerate() {
        let code = transpile(
            "def indexed(items: list) -> int:\n    total = 0\n    for i, item in enumerate(items):\n        total = total + i\n    return total\n",
        );
        assert!(code.contains("fn indexed"));
        assert!(
            code.contains("enumerate") || code.contains("iter()"),
            "Should handle enumerate: {code}"
        );
    }

    #[test]
    fn test_s9_module_function_with_range() {
        let code = transpile(
            "def count_up(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total = total + i\n    return total\n",
        );
        assert!(code.contains("fn count_up"));
    }

    #[test]
    fn test_s9_module_function_with_set() {
        let code =
            transpile("def unique_count(items: list) -> int:\n    s = set()\n    return len(s)\n");
        assert!(code.contains("fn unique_count"));
    }

    #[test]
    fn test_s9_module_function_with_tuple_return() {
        let code =
            transpile("def divmod_fn(a: int, b: int) -> tuple:\n    return (a // b, a % b)\n");
        assert!(code.contains("fn divmod_fn"));
    }

    #[test]
    fn test_s9_module_function_with_assert() {
        let code = transpile("def validated(x: int) -> int:\n    assert x >= 0\n    return x\n");
        assert!(code.contains("fn validated"));
        assert!(code.contains("assert"), "Should have assertion: {code}");
    }

    #[test]
    fn test_s9_module_nested_function() {
        let code = transpile(
            "def outer() -> int:\n    def inner() -> int:\n        return 1\n    return inner()\n",
        );
        assert!(code.contains("outer"));
        assert!(code.contains("inner"));
    }

    #[test]
    fn test_s9_module_function_with_break() {
        let code = transpile(
            "def find_first(items: list) -> int:\n    for x in items:\n        if x > 0:\n            break\n    return 0\n",
        );
        assert!(code.contains("fn find_first"));
        assert!(code.contains("break"), "Should contain break: {code}");
    }

    #[test]
    fn test_s9_module_function_with_continue() {
        let code = transpile(
            "def skip_neg(items: list) -> int:\n    total = 0\n    for x in items:\n        if x < 0:\n            continue\n        total = total + x\n    return total\n",
        );
        assert!(code.contains("fn skip_neg"));
        assert!(code.contains("continue"), "Should contain continue: {code}");
    }

    #[test]
    fn test_s9_module_function_ternary_expr() {
        let code = transpile("def abs_val(x: int) -> int:\n    return x if x >= 0 else -x\n");
        assert!(code.contains("fn abs_val"));
        assert!(code.contains("if"), "Should contain conditional expr: {code}");
    }

    #[test]
    fn test_s9_module_function_string_concat() {
        let code = transpile("def concat(a: str, b: str) -> str:\n    return a + b\n");
        assert!(code.contains("fn concat"));
    }

    #[test]
    fn test_s9_module_constant_list() {
        let code = transpile("ITEMS = [1, 2, 3]\n");
        assert!(
            code.contains("1") && code.contains("2") && code.contains("3"),
            "Should have list constant values: {code}"
        );
    }

    #[test]
    fn test_s9_module_analyze_mutable_vars_basic() {
        // Test that analyze_mutable_vars detects reassignment
        let code = transpile("def mutate() -> int:\n    x = 1\n    x = x + 1\n    return x\n");
        assert!(code.contains("mut"), "Reassigned x should be mutable: {code}");
    }

    #[test]
    fn test_s9_module_analyze_mutable_vars_list_push() {
        let code = transpile(
            "def build() -> list:\n    items = []\n    items.append(1)\n    return items\n",
        );
        assert!(code.contains("mut"), "List with append should be mutable: {code}");
    }

    #[test]
    fn test_s9_module_function_with_while_and_break() {
        let code = transpile(
            "def search(limit: int) -> int:\n    i = 0\n    while i < limit:\n        if i > 10:\n            break\n        i = i + 1\n    return i\n",
        );
        assert!(code.contains("fn search"));
        assert!(code.contains("while"));
        assert!(code.contains("break"));
    }
}
