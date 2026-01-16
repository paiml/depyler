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

    // Test regex module (DEPYLER-EXTDEPS-001: updated to 1.10)
    let re_mapping = mapper.get_mapping("re").unwrap();
    assert_eq!(re_mapping.rust_path, "regex");
    assert!(re_mapping.is_external);
    assert_eq!(re_mapping.version.as_ref().unwrap(), "1.10");
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
    let import = Import { alias: None,
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

    let import = Import { alias: None,
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
    let import = Import { alias: None,
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

    let import = Import { alias: None,
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

    let import = Import { alias: None,
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
        Import { alias: None,
            module: "json".to_string(),
            items: vec![ImportItem::Named("loads".to_string())],
        },
        Import { alias: None,
            module: "re".to_string(),
            items: vec![ImportItem::Named("compile".to_string())],
        },
        Import { alias: None,
            module: "os".to_string(),
            items: vec![ImportItem::Named("getcwd".to_string())],
        },
        Import { alias: None,
            module: "json".to_string(), // Duplicate, should be deduped
            items: vec![ImportItem::Named("dumps".to_string())],
        },
    ];

    let deps = mapper.get_dependencies(&imports);
    assert_eq!(deps.len(), 2); // json and re (os is stdlib)
    assert!(deps.contains(&("serde_json".to_string(), "1.0".to_string())));
    // DEPYLER-EXTDEPS-001: regex updated to 1.10
    assert!(deps.contains(&("regex".to_string(), "1.10".to_string())));
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
    // DEPYLER-0936: OrderedDict maps to HashMap (Rust 1.36+ preserves insertion order)
    assert_eq!(
        collections_mapping.item_map.get("OrderedDict").unwrap(),
        "HashMap"
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
    let import = Import { alias: None,
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
        constructor_patterns: HashMap::new(),
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

    let import = Import { alias: None,
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

    let import = Import { alias: None,
        module: "collections".to_string(),
        items: vec![ImportItem::Named("deque".to_string())],
    };

    let rust_imports = mapper.map_import(&import);

    assert_eq!(rust_imports.len(), 1);
    assert_eq!(rust_imports[0].path, "std::collections::VecDeque");
    assert!(!rust_imports[0].is_external);
}

// ============================================================================
// EXTREME TDD: Comprehensive module coverage tests
// ============================================================================

use crate::module_mapper::ConstructorPattern;

#[test]
fn test_numpy_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("numpy").unwrap();
    assert_eq!(mapping.rust_path, "trueno");
    assert!(mapping.is_external);
    assert_eq!(mapping.item_map.get("array").unwrap(), "Vector::from_slice");
    assert_eq!(mapping.item_map.get("zeros").unwrap(), "Vector::zeros");
    assert_eq!(mapping.item_map.get("dot").unwrap(), "Vector::dot");
    assert_eq!(mapping.item_map.get("sum").unwrap(), "Vector::sum");
}

#[test]
fn test_numpy_linalg_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("numpy.linalg").unwrap();
    assert_eq!(mapping.rust_path, "trueno::linalg");
    assert!(mapping.is_external);
    assert_eq!(mapping.item_map.get("norm").unwrap(), "norm");
    assert_eq!(mapping.item_map.get("inv").unwrap(), "inv");
}

#[test]
fn test_sklearn_linear_model_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("sklearn.linear_model").unwrap();
    assert_eq!(mapping.rust_path, "aprender::linear");
    assert!(mapping.is_external);
    assert_eq!(mapping.item_map.get("LinearRegression").unwrap(), "LinearRegression");
    assert_eq!(mapping.constructor_patterns.get("LinearRegression"), Some(&ConstructorPattern::New));
}

#[test]
fn test_sklearn_cluster_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("sklearn.cluster").unwrap();
    assert_eq!(mapping.rust_path, "aprender::cluster");
    assert_eq!(mapping.item_map.get("KMeans").unwrap(), "KMeans");
}

#[test]
fn test_sklearn_tree_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("sklearn.tree").unwrap();
    assert_eq!(mapping.rust_path, "aprender::tree");
    assert_eq!(mapping.item_map.get("DecisionTreeClassifier").unwrap(), "DecisionTree");
}

#[test]
fn test_sklearn_ensemble_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("sklearn.ensemble").unwrap();
    assert_eq!(mapping.rust_path, "aprender::ensemble");
    assert_eq!(mapping.item_map.get("RandomForestClassifier").unwrap(), "RandomForest");
}

#[test]
fn test_sklearn_preprocessing_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("sklearn.preprocessing").unwrap();
    assert_eq!(mapping.rust_path, "aprender::preprocessing");
    assert_eq!(mapping.item_map.get("StandardScaler").unwrap(), "StandardScaler");
}

#[test]
fn test_sklearn_decomposition_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("sklearn.decomposition").unwrap();
    assert_eq!(mapping.item_map.get("PCA").unwrap(), "PCA");
}

#[test]
fn test_sklearn_model_selection_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("sklearn.model_selection").unwrap();
    assert_eq!(mapping.item_map.get("train_test_split").unwrap(), "train_test_split");
    assert_eq!(mapping.item_map.get("KFold").unwrap(), "KFold");
}

#[test]
fn test_sklearn_metrics_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("sklearn.metrics").unwrap();
    assert_eq!(mapping.item_map.get("accuracy_score").unwrap(), "accuracy");
    assert_eq!(mapping.item_map.get("f1_score").unwrap(), "f1");
}

#[test]
fn test_subprocess_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("subprocess").unwrap();
    assert_eq!(mapping.rust_path, "std::process");
    assert!(!mapping.is_external);
    assert_eq!(mapping.item_map.get("run").unwrap(), "Command");
    assert_eq!(mapping.item_map.get("PIPE").unwrap(), "Stdio::piped");
    assert_eq!(mapping.constructor_patterns.get("Command"), Some(&ConstructorPattern::Method("new".to_string())));
}

#[test]
fn test_argparse_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("argparse").unwrap();
    assert_eq!(mapping.rust_path, "clap");
    assert!(mapping.is_external);
    assert_eq!(mapping.item_map.get("ArgumentParser").unwrap(), "Parser");
}

#[test]
fn test_threading_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("threading").unwrap();
    assert_eq!(mapping.rust_path, "std::thread");
    assert!(!mapping.is_external);
    assert_eq!(mapping.item_map.get("Thread").unwrap(), "spawn");
    assert_eq!(mapping.item_map.get("Lock").unwrap(), "Mutex");
}

#[test]
fn test_asyncio_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("asyncio").unwrap();
    assert_eq!(mapping.rust_path, "tokio");
    assert!(mapping.is_external);
    assert_eq!(mapping.item_map.get("sleep").unwrap(), "time::sleep");
    assert_eq!(mapping.item_map.get("gather").unwrap(), "join!");
}

#[test]
fn test_struct_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("struct").unwrap();
    assert_eq!(mapping.rust_path, "byteorder");
    assert!(mapping.is_external);
    assert_eq!(mapping.item_map.get("pack").unwrap(), "WriteBytesExt");
    assert_eq!(mapping.item_map.get("unpack").unwrap(), "ReadBytesExt");
}

#[test]
fn test_statistics_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("statistics").unwrap();
    assert_eq!(mapping.rust_path, "statrs");
    assert!(mapping.is_external);
    assert_eq!(mapping.item_map.get("mean").unwrap(), "statistics::Statistics::mean");
}

#[test]
fn test_io_module_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("io").unwrap();
    assert_eq!(mapping.rust_path, "std::io");
    assert!(!mapping.is_external);
    assert_eq!(mapping.item_map.get("BufferedReader").unwrap(), "BufReader");
    assert_eq!(mapping.constructor_patterns.get("BufReader"), Some(&ConstructorPattern::New));
}

#[test]
fn test_functools_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("functools").unwrap();
    assert_eq!(mapping.rust_path, "std");
    assert_eq!(mapping.item_map.get("reduce").unwrap(), "iter::Iterator::fold");
}

#[test]
fn test_random_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("random").unwrap();
    assert_eq!(mapping.rust_path, "rand");
    assert!(mapping.is_external);
    assert_eq!(mapping.item_map.get("random").unwrap(), "random");
    assert_eq!(mapping.item_map.get("randint").unwrap(), "gen_range");
}

#[test]
fn test_csv_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("csv").unwrap();
    assert_eq!(mapping.rust_path, "csv");
    assert!(mapping.is_external);
    assert_eq!(mapping.item_map.get("reader").unwrap(), "Reader");
}

#[test]
fn test_pathlib_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("pathlib").unwrap();
    assert_eq!(mapping.rust_path, "std::path");
    assert!(!mapping.is_external);
    assert_eq!(mapping.item_map.get("Path").unwrap(), "PathBuf");
}

#[test]
fn test_os_path_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("os.path").unwrap();
    assert_eq!(mapping.rust_path, "std::path");
    assert_eq!(mapping.item_map.get("join").unwrap(), "Path::join");
    assert_eq!(mapping.item_map.get("exists").unwrap(), "Path::exists");
    assert_eq!(mapping.item_map.get("isfile").unwrap(), "Path::is_file");
}

#[test]
fn test_re_flags_mapping() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("re").unwrap();
    assert_eq!(mapping.item_map.get("IGNORECASE").unwrap(), "(?i)");
    assert_eq!(mapping.item_map.get("I").unwrap(), "(?i)");
    assert_eq!(mapping.item_map.get("MULTILINE").unwrap(), "(?m)");
    assert_eq!(mapping.item_map.get("DOTALL").unwrap(), "(?s)");
}

#[test]
fn test_constructor_pattern_debug() {
    let patterns = vec![
        ConstructorPattern::New,
        ConstructorPattern::Function,
        ConstructorPattern::Method("open".to_string()),
    ];
    for p in patterns {
        let debug_str = format!("{:?}", p);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_constructor_pattern_clone() {
    let p1 = ConstructorPattern::New;
    let p2 = p1.clone();
    assert_eq!(p1, p2);

    let p3 = ConstructorPattern::Method("test".to_string());
    let p4 = p3.clone();
    assert_eq!(p3, p4);
}

#[test]
fn test_constructor_pattern_eq() {
    assert_eq!(ConstructorPattern::New, ConstructorPattern::New);
    assert_eq!(ConstructorPattern::Function, ConstructorPattern::Function);
    assert_eq!(ConstructorPattern::Method("x".to_string()), ConstructorPattern::Method("x".to_string()));
    assert_ne!(ConstructorPattern::New, ConstructorPattern::Function);
    assert_ne!(ConstructorPattern::Method("x".to_string()), ConstructorPattern::Method("y".to_string()));
}

#[test]
fn test_map_argparse_import_whole_module() {
    let mapper = ModuleMapper::new();
    let import = Import { alias: None,
        module: "argparse".to_string(),
        items: vec![],
    };
    let rust_imports = mapper.map_import(&import);
    assert_eq!(rust_imports.len(), 1);
    assert_eq!(rust_imports[0].path, "clap::Parser");
    assert!(rust_imports[0].is_external);
}

#[test]
fn test_map_typing_whole_module() {
    let mapper = ModuleMapper::new();
    let import = Import { alias: None,
        module: "typing".to_string(),
        items: vec![],
    };
    let rust_imports = mapper.map_import(&import);
    // typing has empty rust_path, should generate comment
    assert!(rust_imports[0].path.contains("no Rust equivalent"));
}

#[test]
fn test_tempfile_constructor_patterns() {
    let mapper = ModuleMapper::new();
    let mapping = mapper.get_mapping("tempfile").unwrap();
    assert_eq!(mapping.constructor_patterns.get("NamedTempFile"), Some(&ConstructorPattern::New));
    assert_eq!(mapping.constructor_patterns.get("TempDir"), Some(&ConstructorPattern::New));
    assert_eq!(mapping.constructor_patterns.get("tempfile"), Some(&ConstructorPattern::Function));
    assert_eq!(mapping.constructor_patterns.get("tempdir"), Some(&ConstructorPattern::Function));
}

#[test]
fn test_module_mapping_debug() {
    let mapping = ModuleMapping {
        rust_path: "test".to_string(),
        is_external: false,
        version: None,
        item_map: HashMap::new(),
        constructor_patterns: HashMap::new(),
    };
    let debug_str = format!("{:?}", mapping);
    assert!(debug_str.contains("ModuleMapping"));
}

#[test]
fn test_rust_import_clone() {
    let import = RustImport {
        path: "test::path".to_string(),
        alias: Some("alias".to_string()),
        is_external: true,
    };
    let cloned = import.clone();
    assert_eq!(import.path, cloned.path);
    assert_eq!(import.alias, cloned.alias);
    assert_eq!(import.is_external, cloned.is_external);
}

#[test]
fn test_multiple_sklearn_imports_dependencies() {
    let mapper = ModuleMapper::new();
    let imports = vec![
        Import { alias: None,
            module: "sklearn.linear_model".to_string(),
            items: vec![ImportItem::Named("LinearRegression".to_string())],
        },
        Import { alias: None,
            module: "sklearn.cluster".to_string(),
            items: vec![ImportItem::Named("KMeans".to_string())],
        },
        Import { alias: None,
            module: "sklearn.metrics".to_string(),
            items: vec![ImportItem::Named("accuracy_score".to_string())],
        },
    ];
    let deps = mapper.get_dependencies(&imports);
    // Each sklearn submodule maps to different aprender submodules
    assert_eq!(deps.len(), 3);
    assert!(deps.contains(&("aprender::linear".to_string(), "0.14".to_string())));
    assert!(deps.contains(&("aprender::cluster".to_string(), "0.14".to_string())));
    assert!(deps.contains(&("aprender::metrics".to_string(), "0.14".to_string())));
}

#[test]
fn test_all_mapped_modules_have_valid_data() {
    let mapper = ModuleMapper::new();
    let modules = vec![
        "os", "os.path", "sys", "io", "json", "re", "datetime", "typing",
        "collections", "math", "random", "itertools", "functools", "hashlib",
        "base64", "urllib.parse", "pathlib", "tempfile", "csv", "numpy",
        "numpy.linalg", "sklearn.linear_model", "sklearn.cluster", "sklearn.tree",
        "sklearn.ensemble", "sklearn.preprocessing", "sklearn.decomposition",
        "sklearn.model_selection", "sklearn.metrics", "subprocess", "argparse",
        "threading", "asyncio", "struct", "statistics",
    ];

    for module in modules {
        let mapping = mapper.get_mapping(module);
        assert!(mapping.is_some(), "Module {} should have mapping", module);
    }
}
