//! External Dependencies Mapping Tests (DEPYLER-EXTDEPS-001)
//!
//! TDD tests for the external dependency mapping strategy spec.
//! Following Toyota Way Jidoka principle: build quality in through comprehensive testing.

use depyler_core::hir::{Import, ImportItem};
use depyler_core::module_mapper::ModuleMapper;

// =============================================================================
// Phase 1: Batuta Stack Mappings (P0 - Highest Priority)
// =============================================================================

mod numpy_to_trueno {
    use super::*;

    /// Test: numpy module maps to trueno crate
    /// Spec Section 2.3: NumPy → Trueno Mapping
    #[test]
    fn test_numpy_module_maps_to_trueno() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("numpy").expect("numpy mapping should exist");

        assert_eq!(mapping.rust_path, "trueno");
        assert!(mapping.is_external, "trueno is an external crate");
        assert_eq!(
            mapping.version.as_ref().unwrap(),
            "0.7",
            "trueno version should be 0.7"
        );
    }

    /// Test: np.array maps to Vector::from_slice
    #[test]
    fn test_numpy_array_maps_to_vector_from_slice() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("numpy").expect("numpy mapping should exist");

        assert_eq!(
            mapping.item_map.get("array"),
            Some(&"Vector::from_slice".to_string())
        );
    }

    /// Test: np.zeros maps to Vector::zeros
    #[test]
    fn test_numpy_zeros_maps_to_vector_zeros() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("numpy").expect("numpy mapping should exist");

        assert_eq!(
            mapping.item_map.get("zeros"),
            Some(&"Vector::zeros".to_string())
        );
    }

    /// Test: np.ones maps to Vector::ones
    #[test]
    fn test_numpy_ones_maps_to_vector_ones() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("numpy").expect("numpy mapping should exist");

        assert_eq!(
            mapping.item_map.get("ones"),
            Some(&"Vector::ones".to_string())
        );
    }

    /// Test: np.add maps to Vector::add
    #[test]
    fn test_numpy_add_maps_to_vector_add() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("numpy").expect("numpy mapping should exist");

        assert_eq!(
            mapping.item_map.get("add"),
            Some(&"Vector::add".to_string())
        );
    }

    /// Test: np.dot maps to Vector::dot
    #[test]
    fn test_numpy_dot_maps_to_vector_dot() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("numpy").expect("numpy mapping should exist");

        assert_eq!(
            mapping.item_map.get("dot"),
            Some(&"Vector::dot".to_string())
        );
    }

    /// Test: np.sum maps to Vector::sum
    #[test]
    fn test_numpy_sum_maps_to_vector_sum() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("numpy").expect("numpy mapping should exist");

        assert_eq!(
            mapping.item_map.get("sum"),
            Some(&"Vector::sum".to_string())
        );
    }

    /// Test: np.mean maps to Vector::mean
    #[test]
    fn test_numpy_mean_maps_to_vector_mean() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("numpy").expect("numpy mapping should exist");

        assert_eq!(
            mapping.item_map.get("mean"),
            Some(&"Vector::mean".to_string())
        );
    }

    /// Test: np.matmul maps to Matrix::matmul
    #[test]
    fn test_numpy_matmul_maps_to_matrix_matmul() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("numpy").expect("numpy mapping should exist");

        assert_eq!(
            mapping.item_map.get("matmul"),
            Some(&"Matrix::matmul".to_string())
        );
    }

    /// Test: import numpy generates correct Rust import
    #[test]
    fn test_numpy_import_generates_trueno_use() {
        let mapper = ModuleMapper::new();
        let import = Import {
            module: "numpy".to_string(),
            alias: None,
            items: vec![ImportItem::Named("array".to_string())],
        };

        let rust_imports = mapper.map_import(&import);
        assert!(!rust_imports.is_empty());
        assert!(rust_imports[0].path.contains("trueno"));
        assert!(rust_imports[0].is_external);
    }

    /// Test: numpy generates trueno dependency
    #[test]
    fn test_numpy_generates_trueno_dependency() {
        let mapper = ModuleMapper::new();
        let imports = vec![Import {
            module: "numpy".to_string(),
            alias: None,
            items: vec![ImportItem::Named("array".to_string())],
        }];

        let deps = mapper.get_dependencies(&imports);
        assert!(
            deps.iter().any(|(name, _)| name == "trueno"),
            "Should generate trueno dependency"
        );
    }
}

mod sklearn_to_aprender {
    use super::*;

    /// Test: sklearn module maps to aprender crate
    /// Spec Section 2.4: Sklearn → Aprender Mapping
    #[test]
    fn test_sklearn_module_maps_to_aprender() {
        let mapper = ModuleMapper::new();

        // Check various sklearn submodules
        let linear_mapping = mapper.get_mapping("sklearn.linear_model");
        assert!(
            linear_mapping.is_some(),
            "sklearn.linear_model mapping should exist"
        );

        let mapping = linear_mapping.unwrap();
        assert_eq!(mapping.rust_path, "aprender::linear");
        assert!(mapping.is_external);
    }

    /// Test: LinearRegression maps to aprender::linear::LinearRegression
    #[test]
    fn test_sklearn_linear_regression_maps_to_aprender() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("sklearn.linear_model")
            .expect("sklearn.linear_model should exist");

        assert_eq!(
            mapping.item_map.get("LinearRegression"),
            Some(&"LinearRegression".to_string())
        );
    }

    /// Test: KMeans maps to aprender::cluster::KMeans
    #[test]
    fn test_sklearn_kmeans_maps_to_aprender() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("sklearn.cluster")
            .expect("sklearn.cluster should exist");

        assert_eq!(
            mapping.item_map.get("KMeans"),
            Some(&"KMeans".to_string())
        );
    }

    /// Test: sklearn generates aprender dependency
    #[test]
    fn test_sklearn_generates_aprender_dependency() {
        let mapper = ModuleMapper::new();
        let imports = vec![Import {
            module: "sklearn.linear_model".to_string(),
            alias: None,
            items: vec![ImportItem::Named("LinearRegression".to_string())],
        }];

        let deps = mapper.get_dependencies(&imports);
        assert!(
            deps.iter().any(|(name, _)| name.contains("aprender")),
            "Should generate aprender dependency"
        );
    }
}

// =============================================================================
// Phase 2: High-Impact Standard Library Mappings
// =============================================================================

mod subprocess_to_std_process {
    use super::*;

    /// Test: subprocess module maps to std::process
    /// Spec Section 4.2: subprocess → std::process (146 examples)
    #[test]
    fn test_subprocess_module_maps_to_std_process() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("subprocess")
            .expect("subprocess mapping should exist");

        assert_eq!(mapping.rust_path, "std::process");
        assert!(!mapping.is_external, "std::process is in stdlib");
    }

    /// Test: subprocess.run maps to Command::new().status()
    #[test]
    fn test_subprocess_run_maps_to_command() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("subprocess")
            .expect("subprocess should exist");

        assert_eq!(
            mapping.item_map.get("run"),
            Some(&"Command".to_string())
        );
    }

    /// Test: subprocess.Popen maps to Command
    #[test]
    fn test_subprocess_popen_maps_to_command() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("subprocess")
            .expect("subprocess should exist");

        assert_eq!(
            mapping.item_map.get("Popen"),
            Some(&"Command".to_string())
        );
    }

    /// Test: subprocess.PIPE maps to Stdio::piped
    #[test]
    fn test_subprocess_pipe_maps_to_stdio_piped() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("subprocess")
            .expect("subprocess should exist");

        assert_eq!(
            mapping.item_map.get("PIPE"),
            Some(&"Stdio::piped".to_string())
        );
    }

    /// Test: subprocess.call maps to Command status
    #[test]
    fn test_subprocess_call_maps_to_command_status() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("subprocess")
            .expect("subprocess should exist");

        assert_eq!(
            mapping.item_map.get("call"),
            Some(&"Command".to_string())
        );
    }

    /// Test: subprocess.check_output maps to Command output
    #[test]
    fn test_subprocess_check_output_maps_to_command_output() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("subprocess")
            .expect("subprocess should exist");

        assert_eq!(
            mapping.item_map.get("check_output"),
            Some(&"Command".to_string())
        );
    }
}

mod re_to_regex_enhanced {
    use super::*;

    /// Test: re.sub maps to Regex::replace_all
    #[test]
    fn test_re_sub_maps_to_replace_all() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("re").expect("re mapping should exist");

        assert_eq!(
            mapping.item_map.get("sub"),
            Some(&"Regex::replace_all".to_string())
        );
    }

    /// Test: re.split maps to Regex::split
    #[test]
    fn test_re_split_maps_to_regex_split() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("re").expect("re mapping should exist");

        assert_eq!(
            mapping.item_map.get("split"),
            Some(&"Regex::split".to_string())
        );
    }

    /// Test: re.IGNORECASE maps to RegexBuilder with case_insensitive
    #[test]
    fn test_re_ignorecase_mapping() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("re").expect("re mapping should exist");

        // Flags should have some mapping indicator
        assert!(
            mapping.item_map.contains_key("IGNORECASE")
                || mapping.item_map.contains_key("I"),
            "Should have IGNORECASE flag mapping"
        );
    }
}

// =============================================================================
// Phase 3: Dependency Version Validation
// =============================================================================

mod dependency_versions {
    use super::*;

    /// Test: All Batuta stack crates have correct versions per spec
    #[test]
    fn test_batuta_stack_versions() {
        let mapper = ModuleMapper::new();

        // trueno should be 0.7
        if let Some(numpy) = mapper.get_mapping("numpy") {
            assert_eq!(numpy.version.as_ref().unwrap(), "0.7");
        }

        // aprender should be 0.14
        if let Some(sklearn) = mapper.get_mapping("sklearn.linear_model") {
            assert!(sklearn.version.as_ref().unwrap().starts_with("0.14"));
        }
    }

    /// Test: External crates have valid semver versions
    #[test]
    fn test_external_crate_versions_are_semver() {
        let mapper = ModuleMapper::new();

        let external_modules = [
            "json",
            "re",
            "datetime",
            "random",
            "tempfile",
            "csv",
            "base64",
        ];

        for module in external_modules {
            if let Some(mapping) = mapper.get_mapping(module) {
                if mapping.is_external {
                    assert!(
                        mapping.version.is_some(),
                        "{} should have a version",
                        module
                    );
                    let version = mapping.version.as_ref().unwrap();
                    // Basic semver check: should contain a number
                    assert!(
                        version.chars().any(|c| c.is_ascii_digit()),
                        "{} version '{}' should be semver-like",
                        module,
                        version
                    );
                }
            }
        }
    }
}

// =============================================================================
// Integration Tests
// =============================================================================

// =============================================================================
// Phase 2: P1 Mappings (Medium Impact)
// =============================================================================

mod random_to_rand {
    use super::*;

    /// Test: random module maps to rand crate
    #[test]
    fn test_random_module_maps_to_rand() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("random").expect("random mapping should exist");

        assert_eq!(mapping.rust_path, "rand");
        assert!(mapping.is_external);
        assert_eq!(mapping.version.as_ref().unwrap(), "0.8");
    }

    /// Test: random.random maps to rand::random
    #[test]
    fn test_random_random_maps_correctly() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("random").expect("random should exist");

        assert_eq!(
            mapping.item_map.get("random"),
            Some(&"random".to_string())
        );
    }

    /// Test: random.randint maps to gen_range
    #[test]
    fn test_random_randint_maps_to_gen_range() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("random").expect("random should exist");

        assert_eq!(
            mapping.item_map.get("randint"),
            Some(&"gen_range".to_string())
        );
    }

    /// Test: random.uniform maps to gen_range for floats
    #[test]
    fn test_random_uniform_maps_correctly() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("random").expect("random should exist");

        assert!(
            mapping.item_map.contains_key("uniform"),
            "uniform should be mapped"
        );
    }

    /// Test: random.seed maps to thread_rng seed
    #[test]
    fn test_random_seed_maps_correctly() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("random").expect("random should exist");

        assert!(
            mapping.item_map.contains_key("seed"),
            "seed should be mapped"
        );
    }
}

mod threading_to_std_thread {
    use super::*;

    /// Test: threading module maps to std::thread
    #[test]
    fn test_threading_module_maps_to_std_thread() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("threading")
            .expect("threading mapping should exist");

        assert_eq!(mapping.rust_path, "std::thread");
        assert!(!mapping.is_external);
    }

    /// Test: threading.Thread maps to std::thread::spawn
    #[test]
    fn test_threading_thread_maps_to_spawn() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("threading").expect("threading should exist");

        assert_eq!(
            mapping.item_map.get("Thread"),
            Some(&"spawn".to_string())
        );
    }

    /// Test: threading.Lock maps to std::sync::Mutex
    #[test]
    fn test_threading_lock_maps_to_mutex() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("threading").expect("threading should exist");

        assert_eq!(
            mapping.item_map.get("Lock"),
            Some(&"Mutex".to_string())
        );
    }

    /// Test: threading.Event maps to Condvar
    #[test]
    fn test_threading_event_maps_to_condvar() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("threading").expect("threading should exist");

        assert_eq!(
            mapping.item_map.get("Event"),
            Some(&"Condvar".to_string())
        );
    }

    /// Test: threading.Semaphore maps to Semaphore
    #[test]
    fn test_threading_semaphore_maps_correctly() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("threading").expect("threading should exist");

        assert!(
            mapping.item_map.contains_key("Semaphore"),
            "Semaphore should be mapped"
        );
    }
}

mod asyncio_to_tokio {
    use super::*;

    /// Test: asyncio module maps to tokio
    #[test]
    fn test_asyncio_module_maps_to_tokio() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("asyncio")
            .expect("asyncio mapping should exist");

        assert_eq!(mapping.rust_path, "tokio");
        assert!(mapping.is_external);
        assert!(mapping.version.as_ref().unwrap().starts_with("1."));
    }

    /// Test: asyncio.run maps to tokio runtime
    #[test]
    fn test_asyncio_run_maps_to_runtime() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("asyncio").expect("asyncio should exist");

        assert!(
            mapping.item_map.contains_key("run"),
            "run should be mapped"
        );
    }

    /// Test: asyncio.sleep maps to tokio::time::sleep
    #[test]
    fn test_asyncio_sleep_maps_to_tokio_sleep() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("asyncio").expect("asyncio should exist");

        assert_eq!(
            mapping.item_map.get("sleep"),
            Some(&"time::sleep".to_string())
        );
    }

    /// Test: asyncio.gather maps to tokio::join
    #[test]
    fn test_asyncio_gather_maps_to_join() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("asyncio").expect("asyncio should exist");

        assert!(
            mapping.item_map.contains_key("gather"),
            "gather should be mapped"
        );
    }

    /// Test: asyncio.Queue maps to tokio channel
    #[test]
    fn test_asyncio_queue_maps_to_channel() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("asyncio").expect("asyncio should exist");

        assert!(
            mapping.item_map.contains_key("Queue"),
            "Queue should be mapped"
        );
    }
}

mod struct_to_byteorder {
    use super::*;

    /// Test: struct module maps to byteorder
    #[test]
    fn test_struct_module_maps_to_byteorder() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("struct")
            .expect("struct mapping should exist");

        assert_eq!(mapping.rust_path, "byteorder");
        assert!(mapping.is_external);
    }

    /// Test: struct.pack maps to WriteBytesExt
    #[test]
    fn test_struct_pack_maps_correctly() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("struct").expect("struct should exist");

        assert!(
            mapping.item_map.contains_key("pack"),
            "pack should be mapped"
        );
    }

    /// Test: struct.unpack maps to ReadBytesExt
    #[test]
    fn test_struct_unpack_maps_correctly() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("struct").expect("struct should exist");

        assert!(
            mapping.item_map.contains_key("unpack"),
            "unpack should be mapped"
        );
    }
}

mod statistics_to_statrs {
    use super::*;

    /// Test: statistics module maps to statrs
    #[test]
    fn test_statistics_module_maps_to_statrs() {
        let mapper = ModuleMapper::new();
        let mapping = mapper
            .get_mapping("statistics")
            .expect("statistics mapping should exist");

        assert_eq!(mapping.rust_path, "statrs");
        assert!(mapping.is_external);
    }

    /// Test: statistics.mean maps correctly
    #[test]
    fn test_statistics_mean_maps_correctly() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("statistics").expect("statistics should exist");

        assert!(
            mapping.item_map.contains_key("mean"),
            "mean should be mapped"
        );
    }

    /// Test: statistics.stdev maps correctly
    #[test]
    fn test_statistics_stdev_maps_correctly() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("statistics").expect("statistics should exist");

        assert!(
            mapping.item_map.contains_key("stdev"),
            "stdev should be mapped"
        );
    }

    /// Test: statistics.median maps correctly
    #[test]
    fn test_statistics_median_maps_correctly() {
        let mapper = ModuleMapper::new();
        let mapping = mapper.get_mapping("statistics").expect("statistics should exist");

        assert!(
            mapping.item_map.contains_key("median"),
            "median should be mapped"
        );
    }
}

mod integration {
    use super::*;

    /// Test: Complete numpy workflow generates correct imports and deps
    #[test]
    fn test_complete_numpy_workflow() {
        let mapper = ModuleMapper::new();

        let imports = vec![
            Import {
                module: "numpy".to_string(),
                alias: None,
                items: vec![
                    ImportItem::Named("array".to_string()),
                    ImportItem::Named("zeros".to_string()),
                    ImportItem::Named("dot".to_string()),
                ],
            },
        ];

        // Check imports are generated
        for import in &imports {
            let rust_imports = mapper.map_import(import);
            assert!(!rust_imports.is_empty());
            for ri in &rust_imports {
                assert!(
                    ri.path.contains("trueno") || ri.path.contains("Vector"),
                    "Import path should reference trueno: {}",
                    ri.path
                );
            }
        }

        // Check dependency is generated
        let deps = mapper.get_dependencies(&imports);
        assert!(deps.iter().any(|(name, _)| name == "trueno"));
    }

    /// Test: Mixed Python imports generate correct combined dependencies
    #[test]
    fn test_mixed_imports_generate_correct_deps() {
        let mapper = ModuleMapper::new();

        let imports = vec![
            Import {
                module: "numpy".to_string(),
                alias: None,
                items: vec![ImportItem::Named("array".to_string())],
            },
            Import {
                module: "json".to_string(),
                alias: None,
                items: vec![ImportItem::Named("loads".to_string())],
            },
            Import {
                module: "os".to_string(),
                alias: None,
                items: vec![ImportItem::Named("getcwd".to_string())],
            },
            Import {
                module: "re".to_string(),
                alias: None,
                items: vec![ImportItem::Named("compile".to_string())],
            },
        ];

        let deps = mapper.get_dependencies(&imports);

        // Should have trueno, serde_json, regex (os is stdlib - no dep)
        let dep_names: Vec<_> = deps.iter().map(|(n, _)| n.as_str()).collect();

        // External deps should be present
        assert!(
            dep_names.contains(&"trueno") || mapper.get_mapping("numpy").is_none(),
            "trueno should be in deps if numpy mapping exists"
        );
        assert!(dep_names.contains(&"serde_json"));
        assert!(dep_names.contains(&"regex"));

        // stdlib should NOT generate deps
        assert!(
            !dep_names.contains(&"std"),
            "stdlib should not generate dependency"
        );
    }
}
