use crate::hir::{Import, ImportItem};
use crate::module_mapper::{ModuleMapper, ModuleMapping, RustImport};
use std::collections::HashMap;

#[test]
fn test_module_mapper_creation() {
    let mapper = ModuleMapper::new();
    // Should create without panic
    assert!(mapper.get_mapping("os").is_some());
    assert!(mapper.get_mapping("sys").is_some());
    assert!(mapper.get_mapping("json").is_some());
}

#[test]
fn test_default_trait() {
    let mapper1 = ModuleMapper::new();
    let mapper2 = ModuleMapper::default();

    // Both should have the same mappings
    assert_eq!(
        mapper1.get_mapping("os").unwrap().rust_path,
        mapper2.get_mapping("os").unwrap().rust_path
    );
}

#[test]
fn test_stdlib_mappings() {
    let mapper = ModuleMapper::new();

    // Test os module
    let os_mapping = mapper.get_mapping("os").unwrap();
    assert_eq!(os_mapping.rust_path, "std");
    assert!(!os_mapping.is_external);
    assert!(os_mapping.version.is_none());
    assert_eq!(
        os_mapping.item_map.get("getcwd").unwrap(),
        "env::current_dir"
    );
    assert_eq!(os_mapping.item_map.get("environ").unwrap(), "env::vars");

    // Test sys module
    let sys_mapping = mapper.get_mapping("sys").unwrap();
    assert_eq!(sys_mapping.rust_path, "std");
    assert_eq!(sys_mapping.item_map.get("argv").unwrap(), "env::args");
    assert_eq!(sys_mapping.item_map.get("exit").unwrap(), "process::exit");

    // Test math module
    let math_mapping = mapper.get_mapping("math").unwrap();
    assert_eq!(math_mapping.rust_path, "std::f64");
    assert_eq!(math_mapping.item_map.get("sqrt").unwrap(), "sqrt");
    assert_eq!(math_mapping.item_map.get("pi").unwrap(), "consts::PI");
}

#[test]
fn test_external_crate_mappings() {
    let mapper = ModuleMapper::new();

    // Test json module
    let json_mapping = mapper.get_mapping("json").unwrap();
    assert_eq!(json_mapping.rust_path, "serde_json");
    assert!(json_mapping.is_external);
    assert_eq!(json_mapping.version.as_ref().unwrap(), "1.0");
    assert_eq!(json_mapping.item_map.get("loads").unwrap(), "from_str");

    // Test regex module
    let re_mapping = mapper.get_mapping("re").unwrap();
    assert_eq!(re_mapping.rust_path, "regex");
    assert!(re_mapping.is_external);
    assert_eq!(re_mapping.version.as_ref().unwrap(), "1.0");
    assert_eq!(re_mapping.item_map.get("compile").unwrap(), "Regex::new");

    // Test chrono module
    let datetime_mapping = mapper.get_mapping("datetime").unwrap();
    assert_eq!(datetime_mapping.rust_path, "chrono");
    assert!(datetime_mapping.is_external);
    assert_eq!(datetime_mapping.version.as_ref().unwrap(), "0.4");
}

#[test]
fn test_typing_module_mapping() {
    let mapper = ModuleMapper::new();

    let typing_mapping = mapper.get_mapping("typing").unwrap();
    assert_eq!(typing_mapping.rust_path, "");
    assert!(!typing_mapping.is_external);
    assert_eq!(typing_mapping.item_map.get("List").unwrap(), "Vec");
    assert_eq!(typing_mapping.item_map.get("Dict").unwrap(), "HashMap");
    assert_eq!(typing_mapping.item_map.get("Optional").unwrap(), "Option");
}

#[test]
fn test_map_simple_import() {
    let mapper = ModuleMapper::new();

    // Test import with named items
    let import = Import {
        module: "os".to_string(),
        items: vec![ImportItem::Named("getcwd".to_string())],
    };

    let rust_imports = mapper.map_import(&import);
    assert_eq!(rust_imports.len(), 1);
    assert_eq!(rust_imports[0].path, "std::env::current_dir");
    assert!(!rust_imports[0].is_external);
    assert!(rust_imports[0].alias.is_none());
}

#[test]
fn test_map_aliased_import() {
    let mapper = ModuleMapper::new();

    let import = Import {
        module: "json".to_string(),
        items: vec![ImportItem::Aliased {
            name: "loads".to_string(),
            alias: "parse_json".to_string(),
        }],
    };

    let rust_imports = mapper.map_import(&import);
    assert_eq!(rust_imports.len(), 1);
    assert_eq!(rust_imports[0].path, "serde_json::from_str");
    assert!(rust_imports[0].is_external);
    assert_eq!(rust_imports[0].alias.as_ref().unwrap(), "parse_json");
}

#[test]
fn test_map_whole_module_import() {
    let mapper = ModuleMapper::new();

    // Test "import os" style
    // DEPYLER-0363: Now generates actual use statement with alias
    let import = Import {
        module: "os".to_string(),
        items: vec![],
    };

    let rust_imports = mapper.map_import(&import);
    assert_eq!(rust_imports.len(), 1);
    assert_eq!(rust_imports[0].path, "std");
    assert_eq!(rust_imports[0].alias, Some("os".to_string()));
    assert!(!rust_imports[0].is_external);
}

#[test]
fn test_map_unknown_module() {
    let mapper = ModuleMapper::new();

    let import = Import {
        module: "unknown_module".to_string(),
        items: vec![ImportItem::Named("something".to_string())],
    };

    let rust_imports = mapper.map_import(&import);
    assert_eq!(rust_imports.len(), 1);
    assert!(rust_imports[0].path.contains("NOTE: Map Python module"));
}

#[test]
fn test_map_multiple_items() {
    let mapper = ModuleMapper::new();

    let import = Import {
        module: "os".to_string(),
        items: vec![
            ImportItem::Named("getcwd".to_string()),
            ImportItem::Named("environ".to_string()),
            ImportItem::Aliased {
                name: "path".to_string(),
                alias: "os_path".to_string(),
            },
        ],
    };

    let rust_imports = mapper.map_import(&import);
    assert_eq!(rust_imports.len(), 3);
    assert_eq!(rust_imports[0].path, "std::env::current_dir");
    assert_eq!(rust_imports[1].path, "std::env::vars");
    assert_eq!(rust_imports[2].path, "std::path::Path");
    assert_eq!(rust_imports[2].alias.as_ref().unwrap(), "os_path");
}

#[test]
fn test_get_dependencies() {
    let mapper = ModuleMapper::new();

    let imports = vec![
        Import {
            module: "json".to_string(),
            items: vec![ImportItem::Named("loads".to_string())],
        },
        Import {
            module: "re".to_string(),
            items: vec![ImportItem::Named("compile".to_string())],
        },
        Import {
            module: "os".to_string(),
            items: vec![ImportItem::Named("getcwd".to_string())],
        },
        Import {
            module: "json".to_string(), // Duplicate, should be deduped
            items: vec![ImportItem::Named("dumps".to_string())],
        },
    ];

    let deps = mapper.get_dependencies(&imports);
    assert_eq!(deps.len(), 2); // json and re (os is stdlib)
    assert!(deps.contains(&("serde_json".to_string(), "1.0".to_string())));
    assert!(deps.contains(&("regex".to_string(), "1.0".to_string())));
}

#[test]
fn test_complex_module_paths() {
    let mapper = ModuleMapper::new();

    // Test os.path
    let os_path_mapping = mapper.get_mapping("os.path").unwrap();
    assert_eq!(os_path_mapping.rust_path, "std::path");
    assert_eq!(os_path_mapping.item_map.get("join").unwrap(), "Path::join");

    // Test urllib.parse
    let urllib_mapping = mapper.get_mapping("urllib.parse").unwrap();
    assert_eq!(urllib_mapping.rust_path, "url");
    assert!(urllib_mapping.is_external);
}

#[test]
fn test_collections_mapping() {
    let mapper = ModuleMapper::new();

    let collections_mapping = mapper.get_mapping("collections").unwrap();
    assert_eq!(collections_mapping.rust_path, "std::collections");
    assert_eq!(
        collections_mapping.item_map.get("deque").unwrap(),
        "VecDeque"
    );
    assert_eq!(
        collections_mapping.item_map.get("OrderedDict").unwrap(),
        "IndexMap"
    );
}

#[test]
fn test_itertools_mapping() {
    let mapper = ModuleMapper::new();

    let itertools_mapping = mapper.get_mapping("itertools").unwrap();
    assert_eq!(itertools_mapping.rust_path, "itertools");
    assert!(itertools_mapping.is_external);
    assert_eq!(itertools_mapping.version.as_ref().unwrap(), "0.11");
    assert_eq!(itertools_mapping.item_map.get("chain").unwrap(), "chain");
    assert_eq!(
        itertools_mapping.item_map.get("product").unwrap(),
        "iproduct"
    );
}

#[test]
fn test_unmapped_item_fallback() {
    let mapper = ModuleMapper::new();

    // Test an item that's not in the item_map
    let import = Import {
        module: "os".to_string(),
        items: vec![ImportItem::Named("unmapped_function".to_string())],
    };

    let rust_imports = mapper.map_import(&import);
    assert_eq!(rust_imports.len(), 1);
    // Should use direct mapping when not found in item_map
    assert_eq!(rust_imports[0].path, "std::unmapped_function");
}

#[test]
fn test_crypto_mappings() {
    let mapper = ModuleMapper::new();

    // Test hashlib
    let hashlib_mapping = mapper.get_mapping("hashlib").unwrap();
    assert_eq!(hashlib_mapping.rust_path, "sha2");
    assert!(hashlib_mapping.is_external);
    assert_eq!(hashlib_mapping.item_map.get("sha256").unwrap(), "Sha256");

    // Test base64
    let base64_mapping = mapper.get_mapping("base64").unwrap();
    assert_eq!(base64_mapping.rust_path, "base64");
    assert!(base64_mapping.is_external);
    assert_eq!(base64_mapping.item_map.get("b64encode").unwrap(), "encode");
}

#[test]
fn test_tempfile_mapping() {
    let mapper = ModuleMapper::new();

    let tempfile_mapping = mapper.get_mapping("tempfile").unwrap();
    assert_eq!(tempfile_mapping.rust_path, "tempfile");
    assert!(tempfile_mapping.is_external);
    assert_eq!(
        tempfile_mapping.item_map.get("NamedTemporaryFile").unwrap(),
        "NamedTempFile"
    );
}

#[test]
fn test_module_mapping_clone() {
    let mapping = ModuleMapping {
        rust_path: "test".to_string(),
        is_external: true,
        version: Some("1.0".to_string()),
        item_map: HashMap::from([("foo".to_string(), "bar".to_string())]),
    };

    let cloned = mapping.clone();
    assert_eq!(cloned.rust_path, mapping.rust_path);
    assert_eq!(cloned.is_external, mapping.is_external);
    assert_eq!(cloned.version, mapping.version);
    assert_eq!(cloned.item_map, mapping.item_map);
}

#[test]
fn test_rust_import_struct() {
    let import = RustImport {
        path: "std::env::args".to_string(),
        alias: Some("cmdline_args".to_string()),
        is_external: false,
    };

    assert_eq!(import.path, "std::env::args");
    assert_eq!(import.alias.as_ref().unwrap(), "cmdline_args");
    assert!(!import.is_external);

    // Test Debug trait
    let debug_str = format!("{:?}", import);
    assert!(debug_str.contains("RustImport"));
    assert!(debug_str.contains("cmdline_args"));
}

#[test]
fn test_empty_imports_dependencies() {
    let mapper = ModuleMapper::new();
    let imports: Vec<Import> = vec![];
    let deps = mapper.get_dependencies(&imports);
    assert!(deps.is_empty());
}

// ============================================================================
// DEPYLER-0170: Fix HashMap Import Path Generation
// ============================================================================

#[test]
fn test_collections_hashmap_import_path() {
    // DEPYLER-0170: HashMap should be imported as a type, not HashMap::new
    let mapper = ModuleMapper::new();

    let import = Import {
        module: "collections".to_string(),
        items: vec![
            ImportItem::Named("Counter".to_string()),
            ImportItem::Named("defaultdict".to_string()),
        ],
    };

    let rust_imports = mapper.map_import(&import);

    // Verify Counter mapping
    assert_eq!(rust_imports.len(), 2);
    assert_eq!(rust_imports[0].path, "std::collections::HashMap");
    assert!(!rust_imports[0].is_external);

    // Verify defaultdict mapping
    assert_eq!(rust_imports[1].path, "std::collections::HashMap");
    assert!(!rust_imports[1].is_external);

    // CRITICAL: Path should NOT be "std::collections::HashMap::new"
    assert!(!rust_imports[0].path.ends_with("::new"));
    assert!(!rust_imports[1].path.ends_with("::new"));
}

#[test]
fn test_collections_deque_import_path() {
    // DEPYLER-0173: VecDeque should be imported correctly
    let mapper = ModuleMapper::new();

    let import = Import {
        module: "collections".to_string(),
        items: vec![ImportItem::Named("deque".to_string())],
    };

    let rust_imports = mapper.map_import(&import);

    assert_eq!(rust_imports.len(), 1);
    assert_eq!(rust_imports[0].path, "std::collections::VecDeque");
    assert!(!rust_imports[0].is_external);
}
