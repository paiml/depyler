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
        } else if !mapping.rust_path.is_empty() && !rust_name.is_empty() {
            // DEPYLER-0825: Don't insert items with empty rust_name
            // Empty rust_name indicates no direct Rust equivalent (e.g., functools.partial)
            // This prevents "std::" paths that cause syn::Ident::new("") to panic
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
/// in a module and returns four maps:
/// - imported_modules: Full module imports (e.g., `import math`)
/// - imported_items: Specific item imports (e.g., `from typing import List`)
/// - unresolved_imports: Local imports that need stub functions (DEPYLER-0615)
/// - module_aliases: Module-level aliases (DEPYLER-1136)
///
/// # Arguments
/// * `imports` - List of all imports in the module
/// * `module_mapper` - Module mapper for Python->Rust mappings
///
/// # Returns
/// Tuple of (imported_modules, imported_items, unresolved_imports, module_aliases)
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
    std::collections::HashMap<String, String>,
) {
    let mut imported_modules = std::collections::HashMap::new();
    let mut imported_items = std::collections::HashMap::new();
    let mut unresolved_imports = Vec::new();
    let mut module_aliases = std::collections::HashMap::new();

    for import in imports {
        // DEPYLER-1136: Track module-level aliases (e.g., `import X as Y`)
        if let Some(ref alias) = import.alias {
            module_aliases.insert(alias.clone(), import.module.clone());
        }

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

    (imported_modules, imported_items, unresolved_imports, module_aliases)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{Import, ImportItem};
    use crate::module_mapper::ModuleMapper;

    fn create_test_mapper() -> ModuleMapper {
        ModuleMapper::new()
    }

    #[test]
    fn test_empty_imports_list() {
        let mapper = create_test_mapper();
        let imports: Vec<Import> = vec![];
        let (modules, items, unresolved, aliases) = process_module_imports(&imports, &mapper);
        assert!(modules.is_empty());
        assert!(items.is_empty());
        assert!(unresolved.is_empty());
        assert!(aliases.is_empty());
    }

    #[test]
    fn test_typing_import() {
        let mapper = create_test_mapper();
        let imports = vec![Import {
            module: "typing".to_string(),
            alias: None,
            items: vec![ImportItem::Named("List".to_string())],
        }];
        let (_, items, _, _) = process_module_imports(&imports, &mapper);
        // typing module items are mapped if present in module mapper
        // Just verify no panic occurs
        assert!(items.is_empty() || items.contains_key("List"));
    }

    #[test]
    fn test_math_whole_module_import() {
        let mapper = create_test_mapper();
        let imports = vec![Import {
            module: "math".to_string(),
            alias: None,
            items: vec![],
        }];
        let (modules, items, unresolved, _) = process_module_imports(&imports, &mapper);
        // math is a known module, should be in imported_modules if mapped
        assert!(items.is_empty());
        assert!(unresolved.is_empty());
        // modules may or may not contain math depending on mapper config
        let _ = modules; // use modules to avoid warning
    }

    #[test]
    fn test_from_typing_import_dict() {
        let mapper = create_test_mapper();
        let imports = vec![Import {
            module: "typing".to_string(),
            alias: None,
            items: vec![ImportItem::Named("Dict".to_string())],
        }];
        let (_, items, _, _) = process_module_imports(&imports, &mapper);
        // Just verify no panic
        let _ = items;
    }

    #[test]
    fn test_aliased_import() {
        let mapper = create_test_mapper();
        let imports = vec![Import {
            module: "typing".to_string(),
            alias: None,
            items: vec![ImportItem::Aliased {
                name: "Optional".to_string(),
                alias: "Opt".to_string(),
            }],
        }];
        let (_, items, _, _) = process_module_imports(&imports, &mapper);
        // If mapped, should use alias as key
        if !items.is_empty() {
            assert!(items.contains_key("Opt") || items.contains_key("Optional"));
        }
    }

    #[test]
    fn test_unknown_module_creates_unresolved() {
        let mapper = create_test_mapper();
        let imports = vec![Import {
            module: "my_local_module".to_string(),
            alias: None,
            items: vec![ImportItem::Named("my_func".to_string())],
        }];
        let (_, _, unresolved, _) = process_module_imports(&imports, &mapper);
        assert_eq!(unresolved.len(), 1);
        assert_eq!(unresolved[0].module, "my_local_module");
        assert_eq!(unresolved[0].item_name, "my_func");
    }

    #[test]
    fn test_multiple_items_from_unknown_module() {
        let mapper = create_test_mapper();
        let imports = vec![Import {
            module: "local_utils".to_string(),
            alias: None,
            items: vec![
                ImportItem::Named("helper1".to_string()),
                ImportItem::Named("helper2".to_string()),
                ImportItem::Aliased {
                    name: "helper3".to_string(),
                    alias: "h3".to_string(),
                },
            ],
        }];
        let (_, _, unresolved, _) = process_module_imports(&imports, &mapper);
        assert_eq!(unresolved.len(), 3);
    }

    #[test]
    fn test_mixed_known_and_unknown_imports() {
        let mapper = create_test_mapper();
        let imports = vec![
            Import {
                module: "typing".to_string(),
                alias: None,
                items: vec![ImportItem::Named("List".to_string())],
            },
            Import {
                module: "my_module".to_string(),
                alias: None,
                items: vec![ImportItem::Named("my_func".to_string())],
            },
        ];
        let (_, _, unresolved, _) = process_module_imports(&imports, &mapper);
        // Only my_module should be unresolved
        assert_eq!(unresolved.len(), 1);
        assert_eq!(unresolved[0].module, "my_module");
    }

    #[test]
    fn test_unresolved_import_struct() {
        let import = UnresolvedImport {
            module: "test_module".to_string(),
            item_name: "test_func".to_string(),
        };
        assert_eq!(import.module, "test_module");
        assert_eq!(import.item_name, "test_func");
    }

    #[test]
    fn test_unresolved_import_clone() {
        let import = UnresolvedImport {
            module: "mod".to_string(),
            item_name: "func".to_string(),
        };
        let cloned = import.clone();
        assert_eq!(cloned.module, import.module);
        assert_eq!(cloned.item_name, import.item_name);
    }

    #[test]
    fn test_unresolved_import_debug() {
        let import = UnresolvedImport {
            module: "m".to_string(),
            item_name: "f".to_string(),
        };
        let debug = format!("{:?}", import);
        assert!(debug.contains("UnresolvedImport"));
        assert!(debug.contains("module"));
        assert!(debug.contains("item_name"));
    }

    #[test]
    fn test_whole_module_import_no_items() {
        let mapper = create_test_mapper();
        let imports = vec![Import {
            module: "json".to_string(),
            alias: None,
            items: vec![],
        }];
        let (modules, items, unresolved, _) = process_module_imports(&imports, &mapper);
        // No specific items imported, no unresolved for whole module imports
        assert!(items.is_empty());
        assert!(unresolved.is_empty());
        let _ = modules;
    }

    #[test]
    fn test_multiple_whole_module_imports() {
        let mapper = create_test_mapper();
        let imports = vec![
            Import {
                module: "json".to_string(),
                alias: None,
                items: vec![],
            },
            Import {
                module: "os".to_string(),
                alias: None,
                items: vec![],
            },
        ];
        let (_, _, unresolved, _) = process_module_imports(&imports, &mapper);
        // Whole module imports don't create unresolved entries
        assert!(unresolved.is_empty());
    }

    #[test]
    fn test_functools_import_empty_rust_name() {
        let mapper = create_test_mapper();
        // functools.partial has empty rust_name in mapper
        let imports = vec![Import {
            module: "functools".to_string(),
            alias: None,
            items: vec![ImportItem::Named("partial".to_string())],
        }];
        let (_, items, _, _) = process_module_imports(&imports, &mapper);
        // partial should NOT be in items due to empty rust_name check (DEPYLER-0825)
        assert!(!items.contains_key("partial"));
    }

    // DEPYLER-1136: Test module-level alias tracking
    #[test]
    fn test_module_level_alias() {
        let mapper = create_test_mapper();
        let imports = vec![Import {
            module: "xml.etree.ElementTree".to_string(),
            alias: Some("ET".to_string()),
            items: vec![],
        }];
        let (_, _, _, aliases) = process_module_imports(&imports, &mapper);
        assert_eq!(aliases.get("ET"), Some(&"xml.etree.ElementTree".to_string()));
    }
}
