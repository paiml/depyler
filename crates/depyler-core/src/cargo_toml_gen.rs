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

    if ctx.needs_sha3 {
        deps.push(Dependency::new("sha3", "0.10"));
    }

    if ctx.needs_blake2 {
        deps.push(Dependency::new("blake2", "0.10"));
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

    // DEPYLER-0384: Check if ArgumentParser was used (needs clap)
    if ctx.needs_clap {
        deps.push(Dependency::new("clap", "4.5").with_features(vec!["derive".to_string()]));
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

        let type_mapper: &'static TypeMapper = Box::leak(Box::new(TypeMapper::default()));
        let mut ctx = CodeGenContext {
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
            needs_serde_json: true,
            needs_regex: false,
            needs_chrono: true,
            needs_csv: false,
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
            in_generator: false,
            is_classmethod: false,
            generator_state_vars: std::collections::HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: std::collections::HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            function_return_types: std::collections::HashMap::new(),
            function_param_borrows: std::collections::HashMap::new(),
            tuple_iter_vars: std::collections::HashSet::new(),
            is_final_statement: false,
            result_bool_functions: std::collections::HashSet::new(),
            result_returning_functions: std::collections::HashSet::new(),
            current_error_type: None,
            exception_scopes: Vec::new(),
            argparser_tracker: crate::rust_gen::ArgParserTracker::new(),
            generated_args_struct: None,
            generated_commands_enum: None,
            current_subcommand_fields: None,
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
        let mut ctx = CodeGenContext {
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
            in_generator: false,
            is_classmethod: false,
            generator_state_vars: HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            function_return_types: std::collections::HashMap::new(),
            function_param_borrows: std::collections::HashMap::new(),
            tuple_iter_vars: HashSet::new(),
            is_final_statement: false,
            result_bool_functions: HashSet::new(),
            result_returning_functions: HashSet::new(),
            current_error_type: None,
            exception_scopes: Vec::new(),
            argparser_tracker: crate::rust_gen::ArgParserTracker::new(),
            generated_args_struct: None,
            generated_commands_enum: None,
            current_subcommand_fields: None,
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
        let mut ctx = CodeGenContext {
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
            needs_serde_json: true, // Enable serde_json
            needs_regex: false,
            needs_chrono: false,
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
            needs_clap: false,
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
            in_generator: false,
            is_classmethod: false,
            generator_state_vars: HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            function_return_types: std::collections::HashMap::new(),
            function_param_borrows: std::collections::HashMap::new(),
            tuple_iter_vars: HashSet::new(),
            is_final_statement: false,
            result_bool_functions: HashSet::new(),
            result_returning_functions: HashSet::new(),
            current_error_type: None,
            exception_scopes: Vec::new(),
            argparser_tracker: crate::rust_gen::ArgParserTracker::new(),
            generated_args_struct: None,
            generated_commands_enum: None,
            current_subcommand_fields: None,
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

    /// Integration Test: Verify clap has derive feature
    #[test]
    fn test_integration_clap_has_derive_feature() {
        use crate::rust_gen::CodeGenContext;
        use crate::type_mapper::TypeMapper;
        use std::collections::HashSet;

        let type_mapper: &'static TypeMapper = Box::leak(Box::new(TypeMapper::default()));
        let mut ctx = CodeGenContext {
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
            needs_serde_json: false,
            needs_regex: false,
            needs_chrono: false,
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
            needs_clap: true, // Enable clap
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
            in_generator: false,
            is_classmethod: false,
            generator_state_vars: HashSet::new(),
            var_types: std::collections::HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: std::collections::HashMap::new(),
            function_return_types: std::collections::HashMap::new(),
            function_param_borrows: std::collections::HashMap::new(),
            tuple_iter_vars: HashSet::new(),
            is_final_statement: false,
            result_bool_functions: HashSet::new(),
            result_returning_functions: HashSet::new(),
            current_error_type: None,
            exception_scopes: Vec::new(),
            argparser_tracker: crate::rust_gen::ArgParserTracker::new(),
            generated_args_struct: None,
            generated_commands_enum: None,
            current_subcommand_fields: None,
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
