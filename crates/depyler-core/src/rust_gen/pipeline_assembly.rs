//! DEPYLER-DECOMPOSE: Module assembly pipeline phase
//!
//! Extracted from `generate_rust_file_internal` in `rust_gen.rs` (lines 1058-1285).
//! Assembles all generated code fragments into the final ordered `Vec<TokenStream>`:
//! imports, type aliases, interned strings, constants, conditional imports,
//! error types, CompletedProcess struct, union enums, DepylerValue, runtime types,
//! phantom bindings, module alias stubs, classes, argparse structs, stub functions,
//! functions, main generation, and test generation.

use anyhow::Result;
use quote::quote;

use super::context::{CodeGenContext, RustCodeGen};
use super::{constant_gen, depyler_value_gen, import_gen, module_gen, runtime_types_gen};
#[cfg(feature = "sovereign-types")]
use super::binding_gen;
use crate::hir::*;

/// Assemble all module-level items into a final ordered token stream vector.
///
/// This is the "items assembly" phase of code generation. It takes pre-converted
/// classes and functions plus the module HIR and context, then assembles them
/// in the correct order with all supporting infrastructure (imports, stubs, main, tests).
///
/// # Arguments
/// * `module` - The HIR module being transpiled
/// * `ctx` - Mutable code generation context (tracks needs_completed_process, etc.)
/// * `classes` - Pre-converted class token streams from `class_gen`
/// * `functions` - Pre-converted function token streams from `convert_functions_to_rust`
/// * `unresolved_imports` - Imports that could not be resolved to known modules
/// * `nasa_mode` - Whether NASA mode (standalone, no external crates) is enabled
///
/// # Returns
/// Ordered `Vec<TokenStream>` ready to be quoted into `#(#items)*`
pub(super) fn assemble_module_items(
    module: &HirModule,
    ctx: &mut CodeGenContext,
    classes: Vec<proc_macro2::TokenStream>,
    functions: Vec<proc_macro2::TokenStream>,
    unresolved_imports: &[import_gen::UnresolvedImport],
    nasa_mode: bool,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut items = Vec::new();

    // --- Phase 1: Imports and type setup ---
    append_imports(&mut items, module, nasa_mode);
    append_type_aliases(&mut items, module, ctx);
    append_interned_strings(&mut items, ctx);
    append_constants(&mut items, module, ctx)?;
    append_conditional_imports(&mut items, ctx);
    deduplicate_imports(&mut items);

    // --- Phase 2: Infrastructure types ---
    append_error_types(&mut items, ctx);
    append_completed_process(&mut items, ctx);
    append_union_enums(&mut items, ctx);
    append_depyler_value(&mut items, ctx, nasa_mode);
    append_runtime_types(&mut items, ctx);
    append_phantom_bindings(&mut items, module, ctx);
    append_module_alias_stubs(&mut items, ctx);

    // --- Phase 3: User code ---
    items.extend(classes);
    append_argparse_structs(&mut items, ctx);
    append_stub_functions(&mut items, unresolved_imports);
    items.extend(functions);

    // --- Phase 4: Entry point and tests ---
    append_main_function(&mut items, module, ctx)?;
    append_test_module(&mut items, module)?;

    Ok(items)
}

/// Add module imports (create new mapper for token generation).
/// DEPYLER-1016: Pass NASA mode to skip external crate imports.
fn append_imports(
    items: &mut Vec<proc_macro2::TokenStream>,
    module: &HirModule,
    nasa_mode: bool,
) {
    let import_mapper = crate::module_mapper::ModuleMapper::new();
    items.extend(module_gen::generate_import_tokens(
        &module.imports,
        &import_mapper,
        nasa_mode,
    ));
}

/// DEPYLER-197: Add type aliases (before constants, after imports).
/// Python type aliases like `EventHandler = Callable[[str], None]`
/// must be transpiled as Rust type aliases.
fn append_type_aliases(
    items: &mut Vec<proc_macro2::TokenStream>,
    module: &HirModule,
    ctx: &CodeGenContext,
) {
    items.extend(module_gen::generate_type_alias_tokens(
        &module.type_aliases,
        ctx.type_mapper,
    ));
}

/// Add interned string constants.
fn append_interned_strings(
    items: &mut Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
) {
    items.extend(module_gen::generate_interned_string_tokens(
        &ctx.string_optimizer,
    ));
}

/// Add module-level constants.
fn append_constants(
    items: &mut Vec<proc_macro2::TokenStream>,
    module: &HirModule,
    ctx: &mut CodeGenContext,
) -> Result<()> {
    items.extend(constant_gen::generate_constant_tokens(
        &module.constants,
        ctx,
    )?);
    Ok(())
}

/// Add collection imports if needed.
fn append_conditional_imports(
    items: &mut Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
) {
    items.extend(module_gen::generate_conditional_imports(ctx));
}

/// DEPYLER-0335 FIX #1: Deduplicate imports across all sources.
/// Both generate_import_tokens and generate_conditional_imports can add HashMap.
fn deduplicate_imports(items: &mut Vec<proc_macro2::TokenStream>) {
    let deduped = module_gen::deduplicate_use_statements(std::mem::take(items));
    *items = deduped;
}

/// Add error type definitions if needed.
fn append_error_types(
    items: &mut Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
) {
    items.extend(super::error_gen::generate_error_type_definitions(ctx));
}

/// DEPYLER-0627: Add CompletedProcess struct if subprocess.run is used.
/// DEPYLER-0931: Added Default derive for hoisting support in try/except blocks.
fn append_completed_process(
    items: &mut Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
) {
    if ctx.needs_completed_process {
        let completed_process_struct = quote! {
            /// Result of subprocess.run()
            #[derive(Debug, Clone, Default)]
            pub struct CompletedProcess {
                pub returncode: i32,
                pub stdout: String,
                pub stderr: String,
            }
        };
        items.push(completed_process_struct);
    }
}

/// Add generated union enums.
fn append_union_enums(
    items: &mut Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
) {
    items.extend(ctx.generated_enums.clone());
}

/// DEPYLER-FIX-RC2: Inject DepylerValue enum if heterogeneous dicts were detected
/// OR if we are in NASA mode (since TypeMapper now defaults 'Any' to DepylerValue).
/// DEPYLER-1043: Added trait implementations for Display, len, chars, insert, Index.
/// DEPYLER-1040b/1051: Added Hash/Eq for dict keys (Point 14 falsification fix).
fn append_depyler_value(
    items: &mut Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
    nasa_mode: bool,
) {
    if ctx.needs_depyler_value_enum || nasa_mode {
        let depyler_value_enum = depyler_value_gen::generate_depyler_value_tokens();
        items.push(depyler_value_enum);
    }
}

/// DEPYLER-DECOMPOSE: Inject runtime type items (PythonIntOps, date/time, regex)
/// via extracted module.
fn append_runtime_types(
    items: &mut Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
) {
    items.extend(runtime_types_gen::generate_runtime_type_items(ctx));
}

/// DEPYLER-1115: Generate phantom bindings for external library types.
/// This must come BEFORE classes so external type references resolve.
#[allow(unused_variables)]
fn append_phantom_bindings(
    items: &mut Vec<proc_macro2::TokenStream>,
    module: &HirModule,
    ctx: &CodeGenContext,
) {
    #[cfg(feature = "sovereign-types")]
    {
        if let Some(ref tq) = ctx.type_query {
            let mut type_query_guard = tq.lock().unwrap();
            let mut gen = binding_gen::BindingGenerator::new(&mut type_query_guard);
            gen.collect_symbols(module);
            if let Ok(phantom_bindings) = gen.generate_bindings() {
                items.push(phantom_bindings);
            }
        }
    }
}

/// DEPYLER-1136: Generate module alias stubs.
/// DEPYLER-1137: Use DepylerValue for semantic proxy types (not serde_json::Value).
/// DEPYLER-1139: Use minimal required args - accept anything via impl traits.
/// For `import xml.etree.ElementTree as ET`, generate `mod ET { ... }` stubs.
fn append_module_alias_stubs(
    items: &mut Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
) {
    for alias in ctx.module_aliases.keys() {
        let alias_ident = syn::Ident::new(alias, proc_macro2::Span::call_site());
        let alias_stub = quote! {
            /// DEPYLER-1136: Module alias stub for external library
            /// DEPYLER-1137: Uses DepylerValue for dynamic dispatch compatibility
            /// DEPYLER-1139: Minimal required args to avoid E0061
            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            pub mod #alias_ident {
                use super::DepylerValue;

                /// Phantom function stub - parses XML from string (1 arg)
                pub fn fromstring<S: AsRef<str>>(_s: S) -> DepylerValue {
                    DepylerValue::None
                }

                /// Phantom function stub - parses XML from file (1 arg)
                pub fn parse<S: AsRef<str>>(_source: S) -> DepylerValue {
                    DepylerValue::None
                }

                /// Phantom function stub - creates Element (1 arg only)
                pub fn Element<S: Into<String>>(_tag: S) -> DepylerValue {
                    DepylerValue::None
                }

                /// Phantom function stub - creates SubElement (2 args)
                pub fn SubElement<P, S: Into<String>>(_parent: P, _tag: S) -> DepylerValue {
                    DepylerValue::None
                }

                /// Phantom function stub - converts to string (1-2 args via generic)
                pub fn tostring<E>(_elem: E) -> String {
                    String::new()
                }

                /// Phantom function stub - tostring with encoding (2 args)
                pub fn tostring_with_encoding<E, S: AsRef<str>>(
                    _elem: E,
                    _encoding: S,
                ) -> String {
                    String::new()
                }

                /// Phantom function stub - creates ElementTree (1 arg)
                pub fn ElementTree<E>(_element: E) -> DepylerValue {
                    DepylerValue::None
                }

                /// Phantom function stub - iterparse (1 arg)
                pub fn iterparse<S: AsRef<str>>(_source: S) -> DepylerValue {
                    DepylerValue::None
                }

                /// DEPYLER-1139: Generic get function (like dict.get)
                pub fn get<K, D>(_key: K, _default: D) -> DepylerValue {
                    DepylerValue::None
                }
            }
        };
        items.push(alias_stub);
    }
}

/// DEPYLER-0424: Add ArgumentParser-generated structs at module level
/// (before functions so handler functions can reference Args type).
fn append_argparse_structs(
    items: &mut Vec<proc_macro2::TokenStream>,
    ctx: &CodeGenContext,
) {
    if let Some(ref commands_enum) = ctx.generated_commands_enum {
        items.push(commands_enum.clone());
    }
    if let Some(ref args_struct) = ctx.generated_args_struct {
        items.push(args_struct.clone());
    }
}

/// DEPYLER-0615: Generate stub functions for unresolved local imports.
/// This allows test files importing from local modules to compile standalone.
fn append_stub_functions(
    items: &mut Vec<proc_macro2::TokenStream>,
    unresolved_imports: &[import_gen::UnresolvedImport],
) {
    items.extend(super::generate_stub_functions(unresolved_imports));
}

/// DEPYLER-1216: Generate main() for scripts without an explicit entry point.
/// A Rust binary MUST have fn main(). If the Python script has no main() or
/// `if __name__ == "__main__":` block, we generate one that wraps top-level statements.
fn append_main_function(
    items: &mut Vec<proc_macro2::TokenStream>,
    module: &HirModule,
    ctx: &mut CodeGenContext,
) -> Result<()> {
    let has_main = module.functions.iter().any(|f| f.name == "main");
    if has_main {
        return Ok(());
    }

    if !module.top_level_stmts.is_empty() {
        let main_tokens = generate_semantic_main(&module.top_level_stmts, ctx);
        items.push(main_tokens);
    } else {
        items.push(generate_stub_main());
    }
    Ok(())
}

/// Generate a semantic main() that wraps top-level script statements.
fn generate_semantic_main(
    stmts: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> proc_macro2::TokenStream {
    let mut main_body_tokens = Vec::new();
    for stmt in stmts {
        match stmt.to_rust_tokens(ctx) {
            Ok(tokens) => main_body_tokens.push(tokens),
            Err(e) => {
                // Log warning but continue - fallback to stub behavior for failed conversion
                eprintln!(
                    "DEPYLER-1216: Warning - failed to convert top-level statement: {}",
                    e
                );
            }
        }
    }
    if main_body_tokens.is_empty() {
        return generate_stub_main();
    }
    quote! {
        /// DEPYLER-1216: Auto-generated entry point wrapping top-level script statements
        /// This file was transpiled from a Python script with executable top-level code.
        pub fn main() -> Result<(), Box<dyn std::error::Error>> {
            #(#main_body_tokens)*
            Ok(())
        }
    }
}

/// Generate a stub main() for standalone compilation.
fn generate_stub_main() -> proc_macro2::TokenStream {
    quote! {
        /// DEPYLER-1216: Auto-generated entry point for standalone compilation
        /// This file was transpiled from a Python module without an explicit main.
        /// Add a main() function or `if __name__ == "__main__":` block to customize.
        pub fn main() -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }
}

/// Generate tests for all functions in a single test module.
/// DEPYLER-0280 FIX: Use generate_tests_module() to create a single `mod tests {}` block
/// instead of one per function, which caused "the name `tests` is defined multiple times" errors.
fn append_test_module(
    items: &mut Vec<proc_macro2::TokenStream>,
    module: &HirModule,
) -> Result<()> {
    let test_gen = crate::test_generation::TestGenerator::new(Default::default());
    if let Some(test_module) = test_gen.generate_tests_module(&module.functions)? {
        items.push(test_module);
    }
    Ok(())
}

