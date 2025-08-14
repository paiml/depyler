use depyler_core::hir::{Import, ImportItem};
use depyler_core::module_mapper::ModuleMapper;
use proptest::prelude::*;

// Generate valid Python module names
prop_compose! {
    fn arb_module_name()(base in "[a-z][a-z0-9_]*",
                         submodules in prop::collection::vec("[a-z][a-z0-9_]*", 0..3)) -> String {
        if submodules.is_empty() {
            base
        } else {
            format!("{}.{}", base, submodules.join("."))
        }
    }
}

// Generate valid Python identifiers
prop_compose! {
    fn arb_identifier()(s in "[a-zA-Z_][a-zA-Z0-9_]*") -> String {
        s
    }
}

// Generate import items
prop_compose! {
    fn arb_import_item()(
        is_aliased in any::<bool>(),
        name in arb_identifier(),
        alias in arb_identifier()
    ) -> ImportItem {
        if is_aliased {
            ImportItem::Aliased { name, alias }
        } else {
            ImportItem::Named(name)
        }
    }
}

// Generate imports
prop_compose! {
    fn arb_import()(
        module in arb_module_name(),
        items in prop::collection::vec(arb_import_item(), 0..5)
    ) -> Import {
        Import { module, items }
    }
}

proptest! {
    #[test]
    fn test_module_mapper_never_panics(import in arb_import()) {
        let mapper = ModuleMapper::new();

        // Should never panic on any input
        let _ = mapper.map_import(&import);
    }

    #[test]
    fn test_get_dependencies_is_deterministic(imports in prop::collection::vec(arb_import(), 0..10)) {
        let mapper = ModuleMapper::new();

        // Running get_dependencies multiple times should give same result
        let deps1 = mapper.get_dependencies(&imports);
        let deps2 = mapper.get_dependencies(&imports);

        prop_assert_eq!(deps1, deps2);
    }

    #[test]
    fn test_map_import_produces_valid_rust_paths(import in arb_import()) {
        let mapper = ModuleMapper::new();
        let rust_imports = mapper.map_import(&import);

        for rust_import in rust_imports {
            // All paths should be non-empty
            prop_assert!(!rust_import.path.is_empty());

            // If it's a TODO comment, it should contain the module name
            if rust_import.path.contains("TODO") {
                prop_assert!(rust_import.path.contains(&import.module));
            }
        }
    }

    #[test]
    fn test_known_modules_have_correct_properties(
        module_name in prop::sample::select(vec![
            "os", "sys", "json", "re", "datetime", "typing",
            "collections", "math", "random", "itertools"
        ])
    ) {
        let mapper = ModuleMapper::new();

        if let Some(mapping) = mapper.get_mapping(module_name) {
            // All mappings should have non-empty rust_path (except typing which maps to "")
            if module_name != "typing" {
                prop_assert!(!mapping.rust_path.is_empty());
            }

            // External modules should have versions
            if mapping.is_external {
                prop_assert!(mapping.version.is_some());
            }

            // Standard library modules should not be external
            if matches!(module_name, "os" | "sys" | "math" | "collections") {
                prop_assert!(!mapping.is_external);
            }
        }
    }

    #[test]
    fn test_import_mapping_consistency(
        import in arb_import()
    ) {
        let mapper = ModuleMapper::new();
        let rust_imports1 = mapper.map_import(&import);
        let rust_imports2 = mapper.map_import(&import);

        // Same import should always produce same result
        prop_assert_eq!(rust_imports1.len(), rust_imports2.len());
        for (r1, r2) in rust_imports1.iter().zip(rust_imports2.iter()) {
            prop_assert_eq!(&r1.path, &r2.path);
            prop_assert_eq!(&r1.alias, &r2.alias);
            prop_assert_eq!(r1.is_external, r2.is_external);
        }
    }

    #[test]
    fn test_dependencies_are_unique(
        imports in prop::collection::vec(arb_import(), 1..20)
    ) {
        let mapper = ModuleMapper::new();
        let deps = mapper.get_dependencies(&imports);

        // Check that dependencies are unique
        let mut seen = std::collections::HashSet::new();
        for (crate_name, _) in &deps {
            prop_assert!(seen.insert(crate_name), "Duplicate dependency: {}", crate_name);
        }
    }

    #[test]
    fn test_item_mapping_correctness(
        item_name in arb_identifier()
    ) {
        let mapper = ModuleMapper::new();

        // For known modules, if an item is in the item_map, it should map correctly
        if let Some(os_mapping) = mapper.get_mapping("os") {
            if let Some(rust_name) = os_mapping.item_map.get(&item_name) {
                prop_assert!(!rust_name.is_empty());
            }
        }
    }

    #[test]
    fn test_external_crates_have_versions(
        module in arb_module_name()
    ) {
        let mapper = ModuleMapper::new();

        if let Some(mapping) = mapper.get_mapping(&module) {
            // If it's external, it must have a version
            if mapping.is_external {
                prop_assert!(mapping.version.is_some());
                prop_assert!(!mapping.version.as_ref().unwrap().is_empty());
            }
        }
    }
}

// Additional deterministic property tests
#[test]
fn test_default_and_new_are_equivalent() {
    let mapper1 = ModuleMapper::new();
    let mapper2 = ModuleMapper::default();

    // Test a few known modules
    for module in &["os", "json", "re", "datetime"] {
        let m1 = mapper1.get_mapping(module);
        let m2 = mapper2.get_mapping(module);

        match (m1, m2) {
            (Some(map1), Some(map2)) => {
                assert_eq!(map1.rust_path, map2.rust_path);
                assert_eq!(map1.is_external, map2.is_external);
                assert_eq!(map1.version, map2.version);
            }
            (None, None) => {}
            _ => panic!("Inconsistent mapping for module: {}", module),
        }
    }
}

#[test]
fn test_module_mapping_completeness() {
    let mapper = ModuleMapper::new();

    // Essential Python modules that should be mapped
    let essential_modules = vec![
        "os",
        "sys",
        "json",
        "re",
        "datetime",
        "typing",
        "collections",
        "math",
        "random",
        "itertools",
        "functools",
        "hashlib",
        "base64",
        "pathlib",
        "tempfile",
        "csv",
    ];

    for module in essential_modules {
        assert!(
            mapper.get_mapping(module).is_some(),
            "Missing mapping for essential module: {}",
            module
        );
    }
}
