//! Import processing and module mapping
//!
//! This module handles Python import statements and maps them to Rust
//! use statements through the module mapper.

use crate::hir::{Import, ImportItem};

/// Process a whole module import (e.g., `import math`)
///
/// Adds the module mapping to imported_modules if found in the module mapper.
///
/// # Complexity
/// 2 (single if let)
fn process_whole_module_import(
    import: &Import,
    module_mapper: &crate::module_mapper::ModuleMapper,
    imported_modules: &mut std::collections::HashMap<String, crate::module_mapper::ModuleMapping>,
) {
    if let Some(mapping) = module_mapper.get_mapping(&import.module) {
        imported_modules.insert(import.module.clone(), mapping.clone());
    }
}

/// Process a single import item and add to imported_items
///
/// Handles special case for typing module (no full path needed).
/// Maps Python names to Rust paths using the module mapper.
///
/// # Arguments
/// * `import_module` - The Python module being imported from
/// * `item_name` - The name of the item being imported
/// * `import_key` - The key to use in imported_items (name or alias)
/// * `mapping` - The module mapping for this import
/// * `imported_items` - Map to populate with import->Rust path mappings
///
/// # Complexity
/// 4 (if let + 2 if checks for typing/empty path)
fn process_import_item(
    import_module: &str,
    item_name: &str,
    import_key: &str,
    mapping: &crate::module_mapper::ModuleMapping,
    imported_items: &mut std::collections::HashMap<String, String>,
) {
    if let Some(rust_name) = mapping.item_map.get(item_name) {
        // Special handling for typing module
        if import_module == "typing" && !rust_name.is_empty() {
            // Types from typing module don't need full paths
            imported_items.insert(import_key.to_string(), rust_name.clone());
        } else if !mapping.rust_path.is_empty() {
            imported_items.insert(
                import_key.to_string(),
                format!("{}::{}", mapping.rust_path, rust_name),
            );
        }
    }
}

/// Process specific items import (e.g., `from typing import List, Dict`)
///
/// Handles both Named and Aliased import items by delegating to process_import_item.
///
/// # Complexity
/// 5 (if let + loop + match with 2 arms)
fn process_specific_items_import(
    import: &Import,
    module_mapper: &crate::module_mapper::ModuleMapper,
    imported_items: &mut std::collections::HashMap<String, String>,
) {
    if let Some(mapping) = module_mapper.get_mapping(&import.module) {
        for item in &import.items {
            match item {
                ImportItem::Named(name) => {
                    process_import_item(&import.module, name, name, mapping, imported_items);
                }
                ImportItem::Aliased { name, alias } => {
                    process_import_item(&import.module, name, alias, mapping, imported_items);
                }
            }
        }
    }
}

/// DEPYLER-0615: Track unresolved local imports
/// These are imports from local modules (no stdlib mapping) that need stub functions
#[derive(Debug, Clone)]
pub struct UnresolvedImport {
    pub module: String,
    pub item_name: String,
}

/// Process module imports and populate import mappings
///
/// This is the main entry point for import processing. It processes all imports
/// in a module and returns three maps:
/// - imported_modules: Full module imports (e.g., `import math`)
/// - imported_items: Specific item imports (e.g., `from typing import List`)
/// - unresolved_imports: Local imports that need stub functions (DEPYLER-0615)
///
/// # Arguments
/// * `imports` - List of all imports in the module
/// * `module_mapper` - Module mapper for Python->Rust mappings
///
/// # Returns
/// Tuple of (imported_modules, imported_items, unresolved_imports)
///
/// # Complexity
/// 4 (loop + if/else + inner loop for unresolved)
pub fn process_module_imports(
    imports: &[Import],
    module_mapper: &crate::module_mapper::ModuleMapper,
) -> (
    std::collections::HashMap<String, crate::module_mapper::ModuleMapping>,
    std::collections::HashMap<String, String>,
    Vec<UnresolvedImport>,
) {
    let mut imported_modules = std::collections::HashMap::new();
    let mut imported_items = std::collections::HashMap::new();
    let mut unresolved_imports = Vec::new();

    for import in imports {
        if import.items.is_empty() {
            process_whole_module_import(import, module_mapper, &mut imported_modules);
        } else {
            // DEPYLER-0615: Track unresolved imports for stub generation
            if module_mapper.get_mapping(&import.module).is_none() {
                // This is a local module import - track for stub generation
                for item in &import.items {
                    let item_name = match item {
                        ImportItem::Named(name) => name.clone(),
                        ImportItem::Aliased { name, .. } => name.clone(),
                    };
                    unresolved_imports.push(UnresolvedImport {
                        module: import.module.clone(),
                        item_name,
                    });
                }
            }
            process_specific_items_import(import, module_mapper, &mut imported_items);
        }
    }

    (imported_modules, imported_items, unresolved_imports)
}
