//! Module-level code generation: imports, type aliases, and interned strings
//!
//! This module handles generation of top-level module items that appear
//! before function and struct definitions in the generated Rust output:
//!
//! - **Import deduplication** (`deduplicate_use_statements`): Prevents E0252
//!   errors when multiple sources generate the same `use` statement.
//! - **Conditional imports** (`generate_conditional_imports`): Adds `use`
//!   statements for collections, smart pointers, and external crates based
//!   on what the generated code actually needs (NASA-mode aware).
//! - **Type aliases** (`generate_type_alias_tokens`): Maps Python type aliases
//!   to Rust `type` aliases or newtype structs.
//! - **Import mapping** (`generate_import_tokens`): Translates Python imports
//!   to Rust `use` statements via the module mapper.
//! - **Interned strings** (`generate_interned_string_tokens`): Emits constant
//!   definitions for compile-time interned string literals.

use super::context::CodeGenContext;
use super::{keywords, type_gen};
use crate::hir::*;
use crate::string_optimization::StringOptimizer;
use quote::quote;
use std::collections::HashSet;
use syn::parse_quote;

/// Deduplicate use statements to avoid E0252 errors
///
/// DEPYLER-0335 FIX #1: Multiple sources can generate the same import.
/// For example, both generate_import_tokens and generate_conditional_imports
/// might add `use std::collections::HashMap;`.
///
/// # Complexity
/// ~6 (loop + if + string ops)
pub(super) fn deduplicate_use_statements(
    items: Vec<proc_macro2::TokenStream>,
) -> Vec<proc_macro2::TokenStream> {
    let mut seen = HashSet::new();
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

/// Generate conditional imports based on code generation context
///
/// Adds imports for collections and smart pointers as needed.
/// Complexity: 1 (data-driven approach, well within <=10 target)
/// DEPYLER-1016: Added nasa_mode to skip external crate imports
pub(super) fn generate_conditional_imports(ctx: &CodeGenContext) -> Vec<proc_macro2::TokenStream> {
    let mut imports = Vec::new();
    let nasa_mode = ctx.type_mapper.nasa_mode;

    // DEPYLER-1016: Define std-only imports (always safe)
    let std_imports = [
        (ctx.needs_hashmap, quote! { use std::collections::HashMap; }),
        (ctx.needs_hashset, quote! { use std::collections::HashSet; }),
        (ctx.needs_vecdeque, quote! { use std::collections::VecDeque; }),
        (ctx.needs_arc, quote! { use std::sync::Arc; }),
        (ctx.needs_rc, quote! { use std::rc::Rc; }),
        (ctx.needs_cow, quote! { use std::borrow::Cow; }),
        (ctx.needs_io_read, quote! { use std::io::Read; }), // DEPYLER-0458
        (ctx.needs_io_write, quote! { use std::io::Write; }), // DEPYLER-0458
        (ctx.needs_bufread, quote! { use std::io::BufRead; }), // DEPYLER-0522
        (ctx.needs_lazy_lock, quote! { use std::sync::LazyLock; }), // DEPYLER-1016: NASA mode std-only
    ];

    // DEPYLER-1016: External crate imports (skip in NASA mode)
    let external_imports = [
        (ctx.needs_fnv_hashmap, quote! { use fnv::FnvHashMap; }),
        (ctx.needs_ahash_hashmap, quote! { use ahash::AHashMap; }),
        (ctx.needs_serde_json, quote! { use serde_json; }),
        (ctx.needs_base64, quote! { use base64::Engine; }), // DEPYLER-0664: Engine trait needed for .encode()/.decode() methods
        (ctx.needs_once_cell, quote! { use once_cell::sync::Lazy; }), // DEPYLER-REARCH-001
        (ctx.needs_trueno, quote! { use trueno::Vector; }), // Phase 3: NumPy->Trueno
        // DEPYLER-1004: chrono methods like .month(), .minute() need Datelike/Timelike traits
        (ctx.needs_chrono, quote! { use chrono::{Datelike, Timelike}; }),
    ];

    // Add std imports (always)
    for (needed, import_tokens) in std_imports {
        if needed {
            imports.push(import_tokens);
        }
    }

    // Add external imports only if not in NASA mode
    if !nasa_mode {
        for (needed, import_tokens) in external_imports {
            if needed {
                imports.push(import_tokens);
            }
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
pub(super) fn generate_type_alias_tokens(
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
/// Complexity: ~7-8 (within <=10 target)
/// DEPYLER-1016: Added nasa_mode to skip external crate imports
pub(super) fn generate_import_tokens(
    imports: &[Import],
    module_mapper: &crate::module_mapper::ModuleMapper,
    nasa_mode: bool,
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
                // DEPYLER-1016: Skip external imports in NASA mode
                if !nasa_mode {
                    external_imports.push(rust_import);
                }
            } else {
                std_imports.push(rust_import);
            }
        }
    }

    // DEPYLER-0335 FIX #1: Deduplicate imports using HashSet
    // Multiple Python imports can map to same Rust type (e.g., defaultdict + Counter -> HashMap)
    let mut seen_paths = HashSet::new();

    // Add external imports (deduplicated)
    for import in external_imports {
        // DEPYLER-0936: Skip hashlib module alias
        // hashlib maps to sha2, but hashlib.md5() uses md-5 crate, hashlib.sha256() uses sha2, etc.
        // The method calls are handled inline in expr_gen.rs with correct crate imports.
        // Generating `use sha2 as hashlib;` causes E0432 when only md5 is used.
        if import.alias.as_deref() == Some("hashlib") {
            continue;
        }

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
/// Complexity: 2 (well within <=10 target)
pub(super) fn generate_interned_string_tokens(
    optimizer: &StringOptimizer,
) -> Vec<proc_macro2::TokenStream> {
    let interned_constants = optimizer.generate_interned_constants();
    interned_constants.into_iter().filter_map(|constant| constant.parse().ok()).collect()
}
