//! Cargo.toml generation from CodeGenContext dependencies
//!
//! DEPYLER-0384: Automatically generates Cargo.toml with correct dependencies
//! based on the needs_* flags tracked during code generation.

use crate::rust_gen::CodeGenContext;

/// Dependency specification with version and optional features
#[derive(Debug, Clone)]
pub struct Dependency {
    pub crate_name: String,
    pub version: String,
    pub features: Vec<String>,
}

impl Dependency {
    pub fn new(crate_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            crate_name: crate_name.into(),
            version: version.into(),
            features: vec![],
        }
    }

    pub fn with_features(mut self, features: Vec<String>) -> Self {
        self.features = features;
        self
    }

    /// Generate TOML dependency line
    pub fn to_toml_line(&self) -> String {
        if self.features.is_empty() {
            format!("{} = \"{}\"", self.crate_name, self.version)
        } else {
            let features_str = self
                .features
                .iter()
                .map(|f| format!("\"{}\"", f))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "{} = {{ version = \"{}\", features = [{}] }}",
                self.crate_name, self.version, features_str
            )
        }
    }
}

/// Extract dependencies from CodeGenContext needs_* flags
pub fn extract_dependencies(ctx: &CodeGenContext) -> Vec<Dependency> {
    let mut deps = Vec::new();

    // Standard library collections (no external deps needed)
    // HashMap, HashSet, VecDeque are in std::collections

    // External crate mappings
    if ctx.needs_serde_json {
        deps.push(Dependency::new("serde_json", "1.0"));
        deps.push(Dependency::new("serde", "1.0").with_features(vec!["derive".to_string()]));
    }

    if ctx.needs_regex {
        deps.push(Dependency::new("regex", "1.0"));
    }

    if ctx.needs_chrono {
        deps.push(Dependency::new("chrono", "0.4"));
    }

    if ctx.needs_tempfile {
        deps.push(Dependency::new("tempfile", "3.0"));
    }

    if ctx.needs_itertools {
        deps.push(Dependency::new("itertools", "0.12"));
    }

    if ctx.needs_csv {
        deps.push(Dependency::new("csv", "1.0"));
    }

    if ctx.needs_rust_decimal {
        deps.push(Dependency::new("rust_decimal", "1.0"));
    }

    if ctx.needs_num_rational {
        deps.push(Dependency::new("num-rational", "0.4"));
    }

    if ctx.needs_base64 {
        deps.push(Dependency::new("base64", "0.21"));
    }

    if ctx.needs_md5 {
        deps.push(Dependency::new("md-5", "0.10"));
    }

    if ctx.needs_sha2 {
        deps.push(Dependency::new("sha2", "0.10"));
    }

    // DEPYLER-0558: digest crate for DynDigest trait (type-erased hashers)
    if ctx.needs_digest {
        deps.push(Dependency::new("digest", "0.10"));
    }

    if ctx.needs_sha3 {
        deps.push(Dependency::new("sha3", "0.10"));
    }

    if ctx.needs_blake2 {
        deps.push(Dependency::new("blake2", "0.10"));
    }

    // Phase 3: NumPy→Trueno codegen
    if ctx.needs_trueno {
        deps.push(Dependency::new("trueno", "0.7"));
    }

    // DEPYLER-0747: asyncio→tokio async runtime mapping
    if ctx.needs_tokio {
        deps.push(Dependency::new("tokio", "1").with_features(vec!["full".to_string()]));
    }

    if ctx.needs_hex {
        deps.push(Dependency::new("hex", "0.4"));
    }

    if ctx.needs_uuid {
        deps.push(Dependency::new("uuid", "1.0"));
    }

    if ctx.needs_hmac {
        deps.push(Dependency::new("hmac", "0.12"));
    }

    if ctx.needs_crc32 {
        deps.push(Dependency::new("crc32fast", "1.3"));
    }

    if ctx.needs_url_encoding {
        deps.push(Dependency::new("percent-encoding", "2.3"));
    }

    if ctx.needs_rand {
        deps.push(Dependency::new("rand", "0.8"));
    }

    // GH-207: rand_distr for statistical distributions (gauss, triangular, etc.)
    if ctx.needs_rand_distr {
        deps.push(Dependency::new("rand_distr", "0.4"));
    }

    // DEPYLER-0829: glob crate for Path.glob() and Path.rglob()
    if ctx.needs_glob {
        deps.push(Dependency::new("glob", "0.3"));
    }

    // DEPYLER-0384: Check if ArgumentParser was used (needs clap)
    if ctx.needs_clap {
        deps.push(Dependency::new("clap", "4.5").with_features(vec!["derive".to_string()]));
    }

    // DEPYLER-REARCH-001: Check if once_cell is needed (for lazy static initialization)
    if ctx.needs_once_cell {
        deps.push(Dependency::new("once_cell", "1.20"));
    }

    deps
}

/// Generate complete Cargo.toml content
///
/// DEPYLER-0392: Now includes [[bin]] section to ensure generated manifests
/// are complete and can be built by Cargo without manual editing.
pub fn generate_cargo_toml(
    package_name: &str,
    source_file_path: &str,
    dependencies: &[Dependency],
) -> String {
    let mut toml = String::new();

    // Package section
    toml.push_str("[package]\n");
    toml.push_str(&format!("name = \"{}\"\n", package_name));
    toml.push_str("version = \"0.1.0\"\n");
    toml.push_str("edition = \"2021\"\n");
    toml.push('\n');

    // Binary section (DEPYLER-0392: Required for cargo build to work)
    toml.push_str("[[bin]]\n");
    toml.push_str(&format!("name = \"{}\"\n", package_name));
    toml.push_str(&format!("path = \"{}\"\n", source_file_path));
    toml.push('\n');

    // Dependencies section
    if !dependencies.is_empty() {
        toml.push_str("[dependencies]\n");
        for dep in dependencies {
            toml.push_str(&dep.to_toml_line());
            toml.push('\n');
        }
    }

    toml
}

/// Generate Cargo.toml with automatic crate type selection (DEPYLER-0629)
///
/// Selects [lib] for test files (test_*) and [[bin]] for regular files.
/// This ensures CI enforcement of test files → [lib] is handled consistently.
pub fn generate_cargo_toml_auto(
    package_name: &str,
    source_file_path: &str,
    dependencies: &[Dependency],
) -> String {
    if package_name.starts_with("test_") {
        generate_cargo_toml_lib(package_name, source_file_path, dependencies)
    } else {
        generate_cargo_toml(package_name, source_file_path, dependencies)
    }
}

/// Generate Cargo.toml for library crates (DEPYLER-0600)
///
/// Used by oracle improve loop where generated code has no main function.
/// Automatically includes quickcheck as dev-dependency for generated tests.
pub fn generate_cargo_toml_lib(
    package_name: &str,
    source_file_path: &str,
    dependencies: &[Dependency],
) -> String {
    let mut toml = String::new();

    // Package section
    toml.push_str("[package]\n");
    toml.push_str(&format!("name = \"{}\"\n", package_name));
    toml.push_str("version = \"0.1.0\"\n");
    toml.push_str("edition = \"2021\"\n");
    toml.push('\n');

    // Library section (not binary)
    toml.push_str("[lib]\n");
    toml.push_str(&format!("path = \"{}\"\n", source_file_path));
    toml.push('\n');

    // Dependencies section
    if !dependencies.is_empty() {
        toml.push_str("[dependencies]\n");
        for dep in dependencies {
            toml.push_str(&dep.to_toml_line());
            toml.push('\n');
        }
        toml.push('\n');
    }

    // Dev-dependencies for generated tests (DEPYLER-0600)
    toml.push_str("[dev-dependencies]\n");
    toml.push_str("quickcheck = \"1\"\n");

    toml
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_to_toml_simple() {
        let dep = Dependency::new("serde_json", "1.0");
        assert_eq!(dep.to_toml_line(), "serde_json = \"1.0\"");
    }

    #[test]
    fn test_dependency_to_toml_with_features() {
        let dep = Dependency::new("clap", "4.5").with_features(vec!["derive".to_string()]);
        assert_eq!(
            dep.to_toml_line(),
            "clap = { version = \"4.5\", features = [\"derive\"] }"
        );
    }

    #[test]
    fn test_generate_cargo_toml_empty() {
        let toml = generate_cargo_toml("test_pkg", "test_pkg.rs", &[]);
        assert!(toml.contains("name = \"test_pkg\""));
        assert!(toml.contains("version = \"0.1.0\""));
        assert!(toml.contains("edition = \"2021\""));
        // DEPYLER-0392: Verify [[bin]] section exists
        assert!(toml.contains("[[bin]]"));
        assert!(toml.contains("path = \"test_pkg.rs\""));
    }

    #[test]
    fn test_generate_cargo_toml_with_deps() {
        let deps = vec![
            Dependency::new("serde_json", "1.0"),
            Dependency::new("clap", "4.5").with_features(vec!["derive".to_string()]),
        ];
        let toml = generate_cargo_toml("test_pkg", "main.rs", &deps);

        assert!(toml.contains("[[bin]]"));
        assert!(toml.contains("[dependencies]"));
        assert!(toml.contains("serde_json = \"1.0\""));
        assert!(toml.contains("clap = { version = \"4.5\", features = [\"derive\"] }"));
    }

    // ========================================================================
    // DEPYLER-0384: Property Tests for Cargo.toml Generation
    // ========================================================================

    /// Property Test: Generated TOML must be valid TOML syntax
    #[test]
    fn test_property_generated_toml_is_valid() {
        let test_cases = vec![
            (vec![], "empty"),
            (vec![Dependency::new("serde", "1.0")], "single"),
            (
                vec![
                    Dependency::new("serde", "1.0"),
                    Dependency::new("tokio", "1.0"),
                ],
                "multiple",
            ),
            (
                vec![Dependency::new("clap", "4.5")
                    .with_features(vec!["derive".to_string(), "cargo".to_string()])],
                "features",
            ),
        ];

        for (deps, desc) in test_cases {
            let toml = generate_cargo_toml("test_pkg", "test_pkg.rs", &deps);

            // Property: Must parse as valid TOML
            let parsed: Result<toml::Value, _> = toml::from_str(&toml);
            assert!(
                parsed.is_ok(),
                "{} deps: Generated TOML is invalid: {:?}",
                desc,
                parsed.err()
            );
        }
    }

    /// Property Test: Package name must appear in both [package] and [[bin]] sections
    #[test]
    fn test_property_package_name_uniqueness() {
        let toml = generate_cargo_toml("my_app", "my_app.rs", &[]);

        // Property: Package name appears exactly twice (once in [package], once in [[bin]])
        let count = toml.matches("name = \"my_app\"").count();
        assert_eq!(
            count, 2,
            "Package name must appear in [package] and [[bin]] sections"
        );

        // Property: Required sections exist
        assert!(toml.contains("[package]"), "Must have [package] section");
        assert!(toml.contains("[[bin]]"), "Must have [[bin]] section");
    }

    /// Property Test: All dependencies must be in [dependencies] section
    #[test]
    fn test_property_dependencies_in_correct_section() {
        let deps = vec![
            Dependency::new("serde", "1.0"),
            Dependency::new("tokio", "1.0"),
        ];
        let toml = generate_cargo_toml("test", "test.rs", &deps);

        // Property: [dependencies] appears before any dependency
        let deps_idx = toml.find("[dependencies]");
        let serde_idx = toml.find("serde =");
        let tokio_idx = toml.find("tokio =");

        assert!(deps_idx.is_some(), "Must have [dependencies] section");
        assert!(serde_idx.is_some(), "Must have serde dependency");
        assert!(tokio_idx.is_some(), "Must have tokio dependency");

        assert!(
            deps_idx.unwrap() < serde_idx.unwrap(),
            "[dependencies] must come before serde"
        );
        assert!(
            deps_idx.unwrap() < tokio_idx.unwrap(),
            "[dependencies] must come before tokio"
        );
    }

    /// Property Test: extract_dependencies is idempotent
    #[test]
    fn test_property_extract_dependencies_idempotent() {
        use crate::rust_gen::CodeGenContext;
        use crate::type_mapper::TypeMapper;
        use std::collections::HashSet;

        let type_mapper: &'static TypeMapper = Box::leak(Box::new(TypeMapper::default()));
        let ctx = CodeGenContext {
            type_mapper,
            annotation_aware_mapper:
                crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper::with_base_mapper(
                    type_mapper.clone(),
                ),
            string_optimizer: crate::string_optimization::StringOptimizer::new(),
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
            needs_rand: true,
            needs_slice_random: false, // GH-207
            needs_rand_distr: false,   // GH-207
            needs_serde_json: true,
            needs_regex: false,
            needs_chrono: true,
            needs_csv: false,
            needs_itertools: false,
            needs_tempfile: false,
            needs_rust_decimal: false,
            needs_num_rational: false,
            needs_base64: false,
            needs_md5: true,
            needs_sha2: false,
            needs_sha3: false,
            needs_blake2: false,
            needs_hex: true,
            needs_uuid: false,
            needs_hmac: false,
            needs_crc32: false,
            needs_url_encoding: false,
            needs_clap: true,
            needs_io_read: false,   // DEPYLER-0458
            needs_io_write: false,  // DEPYLER-0458
            needs_bufread: false,   // DEPYLER-0522
            needs_once_cell: false, // DEPYLER-REARCH-001
            needs_trueno: false,    // Phase 3: NumPy→Trueno codegen
            numpy_vars: std::collections::HashSet::new(), // DEPYLER-0932
            needs_glob: false,      // DEPYLER-0829: glob crate for Path.glob()/rglob()
            needs_tokio: false,     // DEPYLER-0747: asyncio→tokio async runtime mapping
            declared_vars: vec![std::collections::HashSet::new()],
            current_function_can_fail: false,
            current_return_type: None,
            module_mapper: crate::module_mapper::ModuleMapper::new(),
            imported_modules: std::collections::HashMap::new(),
            imported_items: std::collections::HashMap::new(),
            mutable_vars: std::collections::HashSet::new(),
            needs_zerodivisionerror: false,
            needs_indexerror: false,
            needs_valueerror: false,
            needs_argumenttypeerror: false,
            needs_runtimeerror: false,
            needs_filenotfounderror: false,
            needs_syntaxerror: false,
            needs_typeerror: false,
            needs_keyerror: false,
            needs_ioerror: false,
            needs_attributeerror: false,
            needs_stopiteration: false,
            in_generator: false,
            is_classmethod: false,
            generator_state_vars: std::collections::HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: std::collections::HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            property_methods: std::collections::HashSet::new(), // DEPYLER-0737: Track @property methods
            function_return_types: std::collections::HashMap::new(),
            function_param_borrows: std::collections::HashMap::new(),
            tuple_iter_vars: std::collections::HashSet::new(),
            iterator_vars: std::collections::HashSet::new(), // DEPYLER-0520: Track iterator vars
            ref_params: std::collections::HashSet::new(),      // DEPYLER-0758: Track ref params
            is_final_statement: false,
            result_bool_functions: std::collections::HashSet::new(),
            result_returning_functions: std::collections::HashSet::new(),
            option_returning_functions: std::collections::HashSet::new(),
            current_error_type: None,
            exception_scopes: Vec::new(),
            argparser_tracker: crate::rust_gen::ArgParserTracker::new(),
            generated_args_struct: None,
            generated_commands_enum: None,
            current_subcommand_fields: None,
            validator_functions: std::collections::HashSet::new(), // DEPYLER-0447
            in_json_context: false,                                // DEPYLER-0461
            stdlib_mappings: crate::stdlib_mappings::StdlibMappings::new(), // DEPYLER-0452
            hoisted_inference_vars: std::collections::HashSet::new(), // DEPYLER-0455 Bug 2
            none_placeholder_vars: std::collections::HashSet::new(), // DEPYLER-0823: Track vars with skipped None assignment
            precomputed_option_fields: std::collections::HashSet::new(), // DEPYLER-0108
            cse_subcommand_temps: std::collections::HashMap::new(), // DEPYLER-0456 Bug #2
            nested_function_params: std::collections::HashMap::new(), // GH-70: Track inferred nested function params
            fn_str_params: HashSet::new(), // DEPYLER-0543: Track function params with str type
            function_param_muts: std::collections::HashMap::new(), // DEPYLER-0574: Track &mut parameters
            needs_digest: false, // DEPYLER-0575: Track digest crate dependency
            in_cmd_handler: false, // DEPYLER-0608: Track if in cmd_* handler function
            cmd_handler_args_fields: Vec::new(), // DEPYLER-0608: Track extracted args.X fields
            in_subcommand_match_arm: false, // DEPYLER-0608: Track if in subcommand match arm
            subcommand_match_fields: Vec::new(), // DEPYLER-0608: Track subcommand fields for match arm
            hoisted_function_names: Vec::new(), // DEPYLER-0613: Track hoisted nested function names
            is_main_function: false, // DEPYLER-0617: Track if in main() for exit code handling
            function_param_defaults: std::collections::HashMap::new(),
            class_field_defaults: std::collections::HashMap::new(), // DEPYLER-0932
            function_param_optionals: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
            boxed_dyn_write_vars: std::collections::HashSet::new(),
            function_returns_boxed_write: false,
            option_unwrap_map: std::collections::HashMap::new(),
            needs_completed_process: false,
            vararg_functions: std::collections::HashSet::new(),
            type_substitutions: std::collections::HashMap::new(),
            current_assign_type: None, // DEPYLER-0727
            force_dict_value_option_wrap: false, // DEPYLER-0741
            char_iter_vars: std::collections::HashSet::new(), // DEPYLER-0795
            char_counter_vars: std::collections::HashSet::new(), // DEPYLER-0821
            adt_child_to_parent: std::collections::HashMap::new(), // DEPYLER-0936
            function_param_types: std::collections::HashMap::new(), // DEPYLER-0950
            mut_option_dict_params: std::collections::HashSet::new(), // DEPYLER-0964
        };

        // Property: Calling extract_dependencies multiple times returns same result
        let deps1 = extract_dependencies(&ctx);
        let deps2 = extract_dependencies(&ctx);

        assert_eq!(
            deps1.len(),
            deps2.len(),
            "Must return same number of dependencies"
        );
        for (d1, d2) in deps1.iter().zip(deps2.iter()) {
            assert_eq!(d1.crate_name, d2.crate_name);
            assert_eq!(d1.version, d2.version);
            assert_eq!(d1.features, d2.features);
        }
    }

    /// Property Test: No duplicate dependencies
    #[test]
    fn test_property_no_duplicate_dependencies() {
        use crate::rust_gen::CodeGenContext;
        use crate::type_mapper::TypeMapper;
        use std::collections::HashSet;

        let type_mapper: &'static TypeMapper = Box::leak(Box::new(TypeMapper::default()));
        let ctx = CodeGenContext {
            type_mapper,
            annotation_aware_mapper:
                crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper::with_base_mapper(
                    type_mapper.clone(),
                ),
            string_optimizer: crate::string_optimization::StringOptimizer::new(),
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
            needs_rand: true,
            needs_slice_random: false, // GH-207
            needs_rand_distr: false,   // GH-207
            needs_serde_json: true,
            needs_regex: true,
            needs_chrono: true,
            needs_csv: true,
            needs_rust_decimal: true,
            needs_num_rational: true,
            needs_base64: true,
            needs_md5: true,
            needs_sha2: true,
            needs_sha3: true,
            needs_blake2: true,
            needs_hex: true,
            needs_uuid: true,
            needs_hmac: true,
            needs_crc32: true,
            needs_url_encoding: true,
            needs_clap: true,
            needs_tempfile: true,   // DEPYLER-0493
            needs_itertools: true,  // DEPYLER-0493
            needs_io_read: false,   // DEPYLER-0458
            needs_io_write: false,  // DEPYLER-0458
            needs_bufread: false,   // DEPYLER-0522
            needs_once_cell: false, // DEPYLER-REARCH-001
            needs_trueno: false,    // Phase 3: NumPy→Trueno codegen
            numpy_vars: HashSet::new(), // DEPYLER-0932
            needs_glob: false,      // DEPYLER-0829: glob crate for Path.glob()/rglob()
            needs_tokio: false,     // DEPYLER-0747: asyncio→tokio async runtime mapping
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
            needs_runtimeerror: false,
            needs_filenotfounderror: false,
            needs_syntaxerror: false,
            needs_typeerror: false,
            needs_keyerror: false,
            needs_ioerror: false,
            needs_attributeerror: false,
            needs_stopiteration: false,
            in_generator: false,
            is_classmethod: false,
            generator_state_vars: HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            property_methods: HashSet::new(), // DEPYLER-0737: Track @property methods
            function_return_types: std::collections::HashMap::new(),
            function_param_borrows: std::collections::HashMap::new(),
            tuple_iter_vars: HashSet::new(),
            iterator_vars: HashSet::new(), // DEPYLER-0520: Track iterator vars
            ref_params: HashSet::new(),      // DEPYLER-0758: Track ref params
            is_final_statement: false,
            result_bool_functions: HashSet::new(),
            result_returning_functions: HashSet::new(),
            option_returning_functions: HashSet::new(),
            current_error_type: None,
            exception_scopes: Vec::new(),
            argparser_tracker: crate::rust_gen::ArgParserTracker::new(),
            generated_args_struct: None,
            generated_commands_enum: None,
            current_subcommand_fields: None,
            validator_functions: std::collections::HashSet::new(), // DEPYLER-0447
            in_json_context: false,                                // DEPYLER-0461
            stdlib_mappings: crate::stdlib_mappings::StdlibMappings::new(), // DEPYLER-0452
            hoisted_inference_vars: std::collections::HashSet::new(), // DEPYLER-0455 Bug 2
            none_placeholder_vars: std::collections::HashSet::new(), // DEPYLER-0823: Track vars with skipped None assignment
            precomputed_option_fields: std::collections::HashSet::new(), // DEPYLER-0108
            cse_subcommand_temps: std::collections::HashMap::new(), // DEPYLER-0456 Bug #2
            nested_function_params: std::collections::HashMap::new(), // GH-70: Track inferred nested function params
            fn_str_params: HashSet::new(), // DEPYLER-0543: Track function params with str type
            function_param_muts: std::collections::HashMap::new(), // DEPYLER-0574: Track &mut parameters
            needs_digest: false, // DEPYLER-0575: Track digest crate dependency
            in_cmd_handler: false, // DEPYLER-0608: Track if in cmd_* handler function
            cmd_handler_args_fields: Vec::new(), // DEPYLER-0608: Track extracted args.X fields
            in_subcommand_match_arm: false, // DEPYLER-0608: Track if in subcommand match arm
            subcommand_match_fields: Vec::new(), // DEPYLER-0608: Track subcommand fields for match arm
            hoisted_function_names: Vec::new(), // DEPYLER-0613: Track hoisted nested function names
            is_main_function: false, // DEPYLER-0617: Track if in main() for exit code handling
            function_param_defaults: std::collections::HashMap::new(),
            class_field_defaults: std::collections::HashMap::new(), // DEPYLER-0932
            function_param_optionals: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
            boxed_dyn_write_vars: std::collections::HashSet::new(),
            function_returns_boxed_write: false,
            option_unwrap_map: std::collections::HashMap::new(),
            needs_completed_process: false,
            vararg_functions: std::collections::HashSet::new(),
            type_substitutions: std::collections::HashMap::new(),
            current_assign_type: None, // DEPYLER-0727
            force_dict_value_option_wrap: false, // DEPYLER-0741
            char_iter_vars: std::collections::HashSet::new(), // DEPYLER-0795
            char_counter_vars: std::collections::HashSet::new(), // DEPYLER-0821
            adt_child_to_parent: std::collections::HashMap::new(), // DEPYLER-0936
            function_param_types: std::collections::HashMap::new(), // DEPYLER-0950
            mut_option_dict_params: std::collections::HashSet::new(), // DEPYLER-0964
        };

        let deps = extract_dependencies(&ctx);

        // Property: No duplicate crate names
        let mut seen = HashSet::new();
        for dep in &deps {
            assert!(
                seen.insert(&dep.crate_name),
                "Duplicate dependency: {}",
                dep.crate_name
            );
        }
    }

    /// Integration Test: Verify serde_json always comes with serde
    #[test]
    fn test_integration_serde_json_implies_serde() {
        use crate::rust_gen::CodeGenContext;
        use crate::type_mapper::TypeMapper;
        use std::collections::HashSet;

        let type_mapper: &'static TypeMapper = Box::leak(Box::new(TypeMapper::default()));
        let ctx = CodeGenContext {
            type_mapper,
            annotation_aware_mapper:
                crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper::with_base_mapper(
                    type_mapper.clone(),
                ),
            string_optimizer: crate::string_optimization::StringOptimizer::new(),
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
            needs_serde_json: true, // Enable serde_json
            needs_regex: false,
            needs_chrono: false,
            needs_csv: false,
            needs_itertools: false,
            needs_tempfile: false,
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
            needs_clap: false,
            needs_io_read: false,   // DEPYLER-0458
            needs_io_write: false,  // DEPYLER-0458
            needs_bufread: false,   // DEPYLER-0522
            needs_once_cell: false, // DEPYLER-REARCH-001
            needs_trueno: false,    // Phase 3: NumPy→Trueno codegen
            numpy_vars: HashSet::new(), // DEPYLER-0932
            needs_glob: false,      // DEPYLER-0829: glob crate for Path.glob()/rglob()
            needs_tokio: false,     // DEPYLER-0747: asyncio→tokio async runtime mapping
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
            needs_runtimeerror: false,
            needs_filenotfounderror: false,
            needs_syntaxerror: false,
            needs_typeerror: false,
            needs_keyerror: false,
            needs_ioerror: false,
            needs_attributeerror: false,
            needs_stopiteration: false,
            in_generator: false,
            is_classmethod: false,
            generator_state_vars: HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            property_methods: HashSet::new(), // DEPYLER-0737: Track @property methods
            function_return_types: std::collections::HashMap::new(),
            function_param_borrows: std::collections::HashMap::new(),
            tuple_iter_vars: HashSet::new(),
            iterator_vars: HashSet::new(), // DEPYLER-0520: Track iterator vars
            ref_params: HashSet::new(),      // DEPYLER-0758: Track ref params
            is_final_statement: false,
            result_bool_functions: HashSet::new(),
            result_returning_functions: HashSet::new(),
            option_returning_functions: HashSet::new(),
            current_error_type: None,
            exception_scopes: Vec::new(),
            argparser_tracker: crate::rust_gen::ArgParserTracker::new(),
            generated_args_struct: None,
            generated_commands_enum: None,
            current_subcommand_fields: None,
            validator_functions: std::collections::HashSet::new(), // DEPYLER-0447
            in_json_context: false,                                // DEPYLER-0461
            stdlib_mappings: crate::stdlib_mappings::StdlibMappings::new(), // DEPYLER-0452
            hoisted_inference_vars: std::collections::HashSet::new(), // DEPYLER-0455 Bug 2
            none_placeholder_vars: std::collections::HashSet::new(), // DEPYLER-0823: Track vars with skipped None assignment
            precomputed_option_fields: std::collections::HashSet::new(), // DEPYLER-0108
            cse_subcommand_temps: std::collections::HashMap::new(), // DEPYLER-0456 Bug #2
            nested_function_params: std::collections::HashMap::new(), // GH-70: Track inferred nested function params
            fn_str_params: HashSet::new(), // DEPYLER-0543: Track function params with str type
            function_param_muts: std::collections::HashMap::new(), // DEPYLER-0574: Track &mut parameters
            needs_digest: false, // DEPYLER-0575: Track digest crate dependency
            in_cmd_handler: false, // DEPYLER-0608: Track if in cmd_* handler function
            cmd_handler_args_fields: Vec::new(), // DEPYLER-0608: Track extracted args.X fields
            in_subcommand_match_arm: false, // DEPYLER-0608: Track if in subcommand match arm
            subcommand_match_fields: Vec::new(), // DEPYLER-0608: Track subcommand fields for match arm
            hoisted_function_names: Vec::new(), // DEPYLER-0613: Track hoisted nested function names
            is_main_function: false, // DEPYLER-0617: Track if in main() for exit code handling
            function_param_defaults: std::collections::HashMap::new(),
            class_field_defaults: std::collections::HashMap::new(), // DEPYLER-0932
            function_param_optionals: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
            boxed_dyn_write_vars: std::collections::HashSet::new(),
            function_returns_boxed_write: false,
            option_unwrap_map: std::collections::HashMap::new(),
            needs_completed_process: false,
            vararg_functions: std::collections::HashSet::new(),
            type_substitutions: std::collections::HashMap::new(),
            current_assign_type: None, // DEPYLER-0727
            force_dict_value_option_wrap: false, // DEPYLER-0741
            char_iter_vars: std::collections::HashSet::new(), // DEPYLER-0795
            char_counter_vars: std::collections::HashSet::new(), // DEPYLER-0821
            adt_child_to_parent: std::collections::HashMap::new(), // DEPYLER-0936
            function_param_types: std::collections::HashMap::new(), // DEPYLER-0950
            mut_option_dict_params: std::collections::HashSet::new(), // DEPYLER-0964
        };

        let deps = extract_dependencies(&ctx);

        // Invariant: serde_json requires serde
        let has_serde_json = deps.iter().any(|d| d.crate_name == "serde_json");
        let has_serde = deps.iter().any(|d| d.crate_name == "serde");

        assert!(has_serde_json, "Should have serde_json");
        assert!(has_serde, "serde_json requires serde");

        // Verify serde has derive feature
        let serde_dep = deps.iter().find(|d| d.crate_name == "serde").unwrap();
        assert!(
            serde_dep.features.contains(&"derive".to_string()),
            "serde needs derive feature"
        );
    }

    // === generate_cargo_toml_lib tests (DEPYLER-0600) ===

    #[test]
    fn test_generate_cargo_toml_lib_empty_deps() {
        let toml = generate_cargo_toml_lib("my_lib", "lib.rs", &[]);
        assert!(toml.contains("[package]"));
        assert!(toml.contains("name = \"my_lib\""));
        assert!(toml.contains("[lib]"));
        assert!(toml.contains("path = \"lib.rs\""));
        // No [[bin]] section
        assert!(!toml.contains("[[bin]]"));
        // Has dev-dependencies for quickcheck
        assert!(toml.contains("[dev-dependencies]"));
        assert!(toml.contains("quickcheck = \"1\""));
    }

    #[test]
    fn test_generate_cargo_toml_lib_with_deps() {
        let deps = vec![Dependency::new("serde", "1.0")];
        let toml = generate_cargo_toml_lib("test_lib", "src/lib.rs", &deps);
        assert!(toml.contains("[dependencies]"));
        assert!(toml.contains("serde = \"1.0\""));
        assert!(toml.contains("[dev-dependencies]"));
        assert!(toml.contains("quickcheck = \"1\""));
    }

    #[test]
    fn test_generate_cargo_toml_lib_is_valid_toml() {
        let deps = vec![
            Dependency::new("serde", "1.0").with_features(vec!["derive".to_string()]),
        ];
        let toml_str = generate_cargo_toml_lib("valid_lib", "lib.rs", &deps);
        let parsed: Result<toml::Value, _> = toml::from_str(&toml_str);
        assert!(parsed.is_ok(), "Generated lib TOML must be valid");
    }

    // === generate_cargo_toml_auto tests (DEPYLER-0629) ===

    #[test]
    fn test_generate_cargo_toml_auto_test_file_uses_lib() {
        let toml = generate_cargo_toml_auto("test_my_module", "test_my_module.rs", &[]);
        // test_ prefix should use lib crate type
        assert!(toml.contains("[lib]"));
        assert!(!toml.contains("[[bin]]"));
        assert!(toml.contains("[dev-dependencies]"));
    }

    #[test]
    fn test_generate_cargo_toml_auto_regular_file_uses_bin() {
        let toml = generate_cargo_toml_auto("my_app", "my_app.rs", &[]);
        // Non-test prefix should use bin crate type
        assert!(toml.contains("[[bin]]"));
        assert!(!toml.contains("[lib]"));
        assert!(!toml.contains("[dev-dependencies]"));
    }

    #[test]
    fn test_generate_cargo_toml_auto_test_prefix_edge_cases() {
        // Must start with "test_", not just contain it
        let toml_starts_with = generate_cargo_toml_auto("test_foo", "test_foo.rs", &[]);
        assert!(toml_starts_with.contains("[lib]"), "test_ prefix → lib");

        let toml_contains = generate_cargo_toml_auto("my_test_helper", "my_test_helper.rs", &[]);
        assert!(toml_contains.contains("[[bin]]"), "Contains test but no prefix → bin");
    }

    // === Dependency struct additional tests ===

    #[test]
    fn test_dependency_clone() {
        let dep = Dependency::new("tokio", "1.0").with_features(vec!["full".to_string()]);
        let cloned = dep.clone();
        assert_eq!(cloned.crate_name, dep.crate_name);
        assert_eq!(cloned.version, dep.version);
        assert_eq!(cloned.features, dep.features);
    }

    #[test]
    fn test_dependency_debug() {
        let dep = Dependency::new("rand", "0.8");
        let debug = format!("{:?}", dep);
        assert!(debug.contains("rand"));
        assert!(debug.contains("0.8"));
    }

    #[test]
    fn test_dependency_multiple_features() {
        let dep = Dependency::new("tokio", "1.0")
            .with_features(vec!["full".to_string(), "rt-multi-thread".to_string(), "macros".to_string()]);
        let line = dep.to_toml_line();
        assert!(line.contains("\"full\""));
        assert!(line.contains("\"rt-multi-thread\""));
        assert!(line.contains("\"macros\""));
    }

    #[test]
    fn test_dependency_empty_features_uses_simple_format() {
        let dep = Dependency::new("regex", "1.0").with_features(vec![]);
        assert_eq!(dep.to_toml_line(), "regex = \"1.0\"");
    }

    // === Individual needs_* flag tests ===

    #[test]
    fn test_extract_dependencies_empty_context() {
        use crate::rust_gen::CodeGenContext;

        let ctx = CodeGenContext::default();
        let deps = extract_dependencies(&ctx);
        assert!(deps.is_empty(), "Default context should have no dependencies");
    }

    /// Integration Test: Verify clap has derive feature
    #[test]
    fn test_integration_clap_has_derive_feature() {
        use crate::rust_gen::CodeGenContext;
        use crate::type_mapper::TypeMapper;
        use std::collections::HashSet;

        let type_mapper: &'static TypeMapper = Box::leak(Box::new(TypeMapper::default()));
        let ctx = CodeGenContext {
            type_mapper,
            annotation_aware_mapper:
                crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper::with_base_mapper(
                    type_mapper.clone(),
                ),
            string_optimizer: crate::string_optimization::StringOptimizer::new(),
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
            needs_csv: false,
            needs_itertools: false,
            needs_tempfile: false,
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
            needs_clap: true,       // Enable clap
            needs_io_read: false,   // DEPYLER-0458
            needs_io_write: false,  // DEPYLER-0458
            needs_bufread: false,   // DEPYLER-0522
            needs_once_cell: false, // DEPYLER-REARCH-001
            needs_trueno: false,    // Phase 3: NumPy→Trueno codegen
            numpy_vars: HashSet::new(), // DEPYLER-0932
            needs_glob: false,      // DEPYLER-0829: glob crate for Path.glob()/rglob()
            needs_tokio: false,     // DEPYLER-0747: asyncio→tokio async runtime mapping
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
            needs_runtimeerror: false,
            needs_filenotfounderror: false,
            needs_syntaxerror: false,
            needs_typeerror: false,
            needs_keyerror: false,
            needs_ioerror: false,
            needs_attributeerror: false,
            needs_stopiteration: false,
            in_generator: false,
            is_classmethod: false,
            generator_state_vars: HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            property_methods: HashSet::new(), // DEPYLER-0737: Track @property methods
            function_return_types: std::collections::HashMap::new(),
            function_param_borrows: std::collections::HashMap::new(),
            tuple_iter_vars: HashSet::new(),
            iterator_vars: HashSet::new(), // DEPYLER-0520: Track iterator vars
            ref_params: HashSet::new(),      // DEPYLER-0758: Track ref params
            is_final_statement: false,
            result_bool_functions: HashSet::new(),
            result_returning_functions: HashSet::new(),
            option_returning_functions: HashSet::new(),
            current_error_type: None,
            exception_scopes: Vec::new(),
            argparser_tracker: crate::rust_gen::ArgParserTracker::new(),
            generated_args_struct: None,
            generated_commands_enum: None,
            current_subcommand_fields: None,
            validator_functions: std::collections::HashSet::new(), // DEPYLER-0447
            in_json_context: false,                                // DEPYLER-0461
            stdlib_mappings: crate::stdlib_mappings::StdlibMappings::new(), // DEPYLER-0452
            hoisted_inference_vars: std::collections::HashSet::new(), // DEPYLER-0455 Bug 2
            none_placeholder_vars: std::collections::HashSet::new(), // DEPYLER-0823: Track vars with skipped None assignment
            precomputed_option_fields: std::collections::HashSet::new(), // DEPYLER-0108
            cse_subcommand_temps: std::collections::HashMap::new(), // DEPYLER-0456 Bug #2
            nested_function_params: std::collections::HashMap::new(), // GH-70: Track inferred nested function params
            fn_str_params: HashSet::new(), // DEPYLER-0543: Track function params with str type
            function_param_muts: std::collections::HashMap::new(), // DEPYLER-0574: Track &mut parameters
            needs_digest: false, // DEPYLER-0575: Track digest crate dependency
            in_cmd_handler: false, // DEPYLER-0608: Track if in cmd_* handler function
            cmd_handler_args_fields: Vec::new(), // DEPYLER-0608: Track extracted args.X fields
            in_subcommand_match_arm: false, // DEPYLER-0608: Track if in subcommand match arm
            subcommand_match_fields: Vec::new(), // DEPYLER-0608: Track subcommand fields for match arm
            hoisted_function_names: Vec::new(), // DEPYLER-0613: Track hoisted nested function names
            is_main_function: false, // DEPYLER-0617: Track if in main() for exit code handling
            function_param_defaults: std::collections::HashMap::new(),
            class_field_defaults: std::collections::HashMap::new(), // DEPYLER-0932
            function_param_optionals: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
            boxed_dyn_write_vars: std::collections::HashSet::new(),
            function_returns_boxed_write: false,
            option_unwrap_map: std::collections::HashMap::new(),
            needs_completed_process: false,
            vararg_functions: std::collections::HashSet::new(),
            type_substitutions: std::collections::HashMap::new(),
            current_assign_type: None, // DEPYLER-0727
            force_dict_value_option_wrap: false, // DEPYLER-0741
            char_iter_vars: std::collections::HashSet::new(), // DEPYLER-0795
            char_counter_vars: std::collections::HashSet::new(), // DEPYLER-0821
            adt_child_to_parent: std::collections::HashMap::new(), // DEPYLER-0936
            function_param_types: std::collections::HashMap::new(), // DEPYLER-0950
            mut_option_dict_params: std::collections::HashSet::new(), // DEPYLER-0964
        };

        let deps = extract_dependencies(&ctx);

        // Invariant: clap must have derive feature for #[derive(Parser)]
        let clap_dep = deps.iter().find(|d| d.crate_name == "clap");
        assert!(clap_dep.is_some(), "Should have clap");
        assert!(
            clap_dep.unwrap().features.contains(&"derive".to_string()),
            "clap needs derive feature for ArgumentParser"
        );
    }
}
