//! DEPYLER-0903: Test Suite for Enterprise Library Mapping System
//!
//! EXTREME TDD: Tests written FIRST, implementation follows.
//!
//! Test categories:
//! 1. Unit tests for core structs
//! 2. Registry priority tests
//! 3. Transform pattern tests
//! 4. TOML parsing tests
//! 5. Property tests for O(1) guarantees

// Allow ticket-number naming convention (DEPYLER-XXXX)
#![allow(non_snake_case)]

use super::*;

// ============================================================================
// Unit Tests: Core Data Structures
// ============================================================================

#[test]
fn test_DEPYLER_0903_library_mapping_creation() {
    let mapping = LibraryMapping {
        python_module: "pandas".to_string(),
        rust_crate: "polars".to_string(),
        python_version_req: ">=3.8".to_string(),
        rust_crate_version: "0.35".to_string(),
        items: HashMap::new(),
        features: vec!["lazy".to_string()],
        confidence: MappingConfidence::Community,
        provenance: "https://docs.rs/polars/".to_string(),
    };

    assert_eq!(mapping.python_module, "pandas");
    assert_eq!(mapping.rust_crate, "polars");
    assert_eq!(mapping.python_version_req, ">=3.8");
    assert_eq!(mapping.rust_crate_version, "0.35");
    assert_eq!(mapping.features, vec!["lazy"]);
    assert_eq!(mapping.confidence, MappingConfidence::Community);
}

#[test]
fn test_DEPYLER_0903_item_mapping_direct() {
    let item = ItemMapping {
        rust_name: "DataFrame".to_string(),
        pattern: TransformPattern::Direct,
        type_transform: None,
    };

    assert_eq!(item.rust_name, "DataFrame");
    assert_eq!(item.pattern, TransformPattern::Direct);
}

#[test]
fn test_DEPYLER_0903_item_mapping_method_call() {
    let item = ItemMapping {
        rust_name: "head".to_string(),
        pattern: TransformPattern::MethodCall {
            extra_args: vec!["None".to_string()],
        },
        type_transform: None,
    };

    match &item.pattern {
        TransformPattern::MethodCall { extra_args } => {
            assert_eq!(extra_args, &vec!["None".to_string()]);
        }
        _ => panic!("Expected MethodCall pattern"),
    }
}

#[test]
fn test_DEPYLER_0903_item_mapping_constructor() {
    let item = ItemMapping {
        rust_name: "DataFrame".to_string(),
        pattern: TransformPattern::Constructor {
            method: "new".to_string(),
        },
        type_transform: None,
    };

    match &item.pattern {
        TransformPattern::Constructor { method } => {
            assert_eq!(method, "new");
        }
        _ => panic!("Expected Constructor pattern"),
    }
}

// ============================================================================
// Unit Tests: ReorderArgs Pattern (Section 11.1)
// ============================================================================

#[test]
fn test_DEPYLER_0903_reorder_args_valid_permutation() {
    // Valid: [0, 2, 1] is a permutation of [0, 1, 2]
    let result = TransformPattern::validate_reorder_args(&[0, 2, 1]);
    assert!(result.is_ok());
}

#[test]
fn test_DEPYLER_0903_reorder_args_identity_permutation() {
    // Valid: [0, 1, 2] is identity permutation
    let result = TransformPattern::validate_reorder_args(&[0, 1, 2]);
    assert!(result.is_ok());
}

#[test]
fn test_DEPYLER_0903_reorder_args_empty_valid() {
    // Valid: empty is trivial permutation
    let result = TransformPattern::validate_reorder_args(&[]);
    assert!(result.is_ok());
}

#[test]
fn test_DEPYLER_0903_reorder_args_single_element() {
    // Valid: [0] is only valid single-element permutation
    let result = TransformPattern::validate_reorder_args(&[0]);
    assert!(result.is_ok());
}

#[test]
fn test_DEPYLER_0903_reorder_args_out_of_bounds() {
    // Invalid: index 5 out of bounds for 3 elements
    let result = TransformPattern::validate_reorder_args(&[0, 5, 1]);
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("out of bounds"));
}

#[test]
fn test_DEPYLER_0903_reorder_args_duplicate_index() {
    // Invalid: duplicate index 1
    let result = TransformPattern::validate_reorder_args(&[0, 1, 1]);
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("Duplicate"));
}

// ============================================================================
// Unit Tests: TypedTemplate Pattern (Section 11.3)
// ============================================================================

#[test]
fn test_DEPYLER_0903_typed_template_valid() {
    let result = TransformPattern::validate_typed_template(
        "{client}.put_object().bucket({bucket}).key({key})",
        &[
            "client".to_string(),
            "bucket".to_string(),
            "key".to_string(),
        ],
        &[ParamType::Expr, ParamType::String, ParamType::String],
    );
    assert!(result.is_ok());
}

#[test]
fn test_DEPYLER_0903_typed_template_missing_param() {
    let result = TransformPattern::validate_typed_template(
        "{client}.put_object()",
        &["client".to_string(), "missing".to_string()],
        &[ParamType::Expr, ParamType::String],
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("not found"));
}

#[test]
fn test_DEPYLER_0903_typed_template_count_mismatch() {
    let result = TransformPattern::validate_typed_template(
        "{client}.method()",
        &["client".to_string()],
        &[ParamType::Expr, ParamType::String], // 2 types but 1 param
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("count"));
}

// ============================================================================
// Unit Tests: Registry Priority Chain
// ============================================================================

#[test]
fn test_DEPYLER_0903_registry_core_lookup() {
    let registry = MappingRegistry::with_defaults();

    // json.loads should be in core mappings
    let item = registry.lookup("json", "loads");
    assert!(item.is_some());
    assert_eq!(item.unwrap().rust_name, "from_str");
}

#[test]
fn test_DEPYLER_0903_registry_override_takes_priority() {
    let mut registry = MappingRegistry::with_defaults();

    // Add override for json.loads
    registry.register_override(LibraryMapping {
        python_module: "json".to_string(),
        rust_crate: "custom_json".to_string(),
        python_version_req: "*".to_string(),
        rust_crate_version: "2.0".to_string(),
        items: HashMap::from([(
            "loads".to_string(),
            ItemMapping {
                rust_name: "custom_from_str".to_string(),
                pattern: TransformPattern::Direct,
                type_transform: None,
            },
        )]),
        features: vec![],
        confidence: MappingConfidence::Verified,
        provenance: "internal".to_string(),
    });

    // Override should take priority over core
    let item = registry.lookup("json", "loads");
    assert!(item.is_some());
    assert_eq!(item.unwrap().rust_name, "custom_from_str");
}

#[test]
fn test_DEPYLER_0903_registry_extension_over_core() {
    let mut registry = MappingRegistry::with_defaults();

    // Add extension for json.loads
    registry.register_extension(LibraryMapping {
        python_module: "json".to_string(),
        rust_crate: "enterprise_json".to_string(),
        python_version_req: "*".to_string(),
        rust_crate_version: "1.5".to_string(),
        items: HashMap::from([(
            "loads".to_string(),
            ItemMapping {
                rust_name: "enterprise_from_str".to_string(),
                pattern: TransformPattern::Direct,
                type_transform: None,
            },
        )]),
        features: vec![],
        confidence: MappingConfidence::Verified,
        provenance: "enterprise".to_string(),
    });

    // Extension should take priority over core
    let item = registry.lookup("json", "loads");
    assert!(item.is_some());
    assert_eq!(item.unwrap().rust_name, "enterprise_from_str");
}

#[test]
fn test_DEPYLER_0903_registry_override_over_extension() {
    let mut registry = MappingRegistry::with_defaults();

    // Add extension
    registry.register_extension(LibraryMapping {
        python_module: "json".to_string(),
        rust_crate: "enterprise_json".to_string(),
        python_version_req: "*".to_string(),
        rust_crate_version: "1.5".to_string(),
        items: HashMap::from([(
            "loads".to_string(),
            ItemMapping {
                rust_name: "enterprise_from_str".to_string(),
                pattern: TransformPattern::Direct,
                type_transform: None,
            },
        )]),
        features: vec![],
        confidence: MappingConfidence::Verified,
        provenance: "enterprise".to_string(),
    });

    // Add override (should win)
    registry.register_override(LibraryMapping {
        python_module: "json".to_string(),
        rust_crate: "user_json".to_string(),
        python_version_req: "*".to_string(),
        rust_crate_version: "3.0".to_string(),
        items: HashMap::from([(
            "loads".to_string(),
            ItemMapping {
                rust_name: "user_from_str".to_string(),
                pattern: TransformPattern::Direct,
                type_transform: None,
            },
        )]),
        features: vec![],
        confidence: MappingConfidence::Verified,
        provenance: "user".to_string(),
    });

    // Override should win over extension
    let item = registry.lookup("json", "loads");
    assert!(item.is_some());
    assert_eq!(item.unwrap().rust_name, "user_from_str");
}

#[test]
fn test_DEPYLER_0903_registry_module_lookup() {
    let registry = MappingRegistry::with_defaults();

    let module = registry.lookup_module("json");
    assert!(module.is_some());
    assert_eq!(module.unwrap().rust_crate, "serde_json");
}

#[test]
fn test_DEPYLER_0903_registry_nonexistent_module() {
    let registry = MappingRegistry::with_defaults();

    let item = registry.lookup("nonexistent", "anything");
    assert!(item.is_none());
}

#[test]
fn test_DEPYLER_0903_registry_nonexistent_item() {
    let registry = MappingRegistry::with_defaults();

    let item = registry.lookup("json", "nonexistent_function");
    assert!(item.is_none());
}

#[test]
fn test_DEPYLER_0903_registry_module_count() {
    let registry = MappingRegistry::with_defaults();

    // Should have at least json, os, re from defaults
    assert!(registry.module_count() >= 3);
}

// ============================================================================
// Unit Tests: MappingConfidence
// ============================================================================

#[test]
fn test_DEPYLER_0903_confidence_levels() {
    assert_eq!(
        MappingConfidence::default(),
        MappingConfidence::Experimental
    );

    let verified = MappingConfidence::Verified;
    let community = MappingConfidence::Community;
    let experimental = MappingConfidence::Experimental;

    // Ensure distinct
    assert_ne!(verified, community);
    assert_ne!(community, experimental);
    assert_ne!(verified, experimental);
}

// ============================================================================
// Serialization Tests
// ============================================================================

#[test]
fn test_DEPYLER_0903_library_mapping_serde_roundtrip() {
    let original = LibraryMapping {
        python_module: "numpy".to_string(),
        rust_crate: "ndarray".to_string(),
        python_version_req: ">=3.7".to_string(),
        rust_crate_version: "0.15".to_string(),
        items: HashMap::from([(
            "array".to_string(),
            ItemMapping {
                rust_name: "Array".to_string(),
                pattern: TransformPattern::Constructor {
                    method: "from_vec".to_string(),
                },
                type_transform: None,
            },
        )]),
        features: vec!["blas".to_string()],
        confidence: MappingConfidence::Community,
        provenance: "https://docs.rs/ndarray/".to_string(),
    };

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: LibraryMapping = serde_json::from_str(&json).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_DEPYLER_0903_transform_pattern_serde() {
    // Test each variant serializes correctly
    let patterns = vec![
        TransformPattern::Direct,
        TransformPattern::MethodCall {
            extra_args: vec!["arg1".to_string()],
        },
        TransformPattern::PropertyToMethod,
        TransformPattern::Constructor {
            method: "new".to_string(),
        },
        TransformPattern::ReorderArgs {
            indices: vec![2, 0, 1],
        },
        TransformPattern::TypedTemplate {
            pattern: "{x}.method()".to_string(),
            params: vec!["x".to_string()],
            param_types: vec![ParamType::Expr],
        },
    ];

    for pattern in patterns {
        let json = serde_json::to_string(&pattern).unwrap();
        let deserialized: TransformPattern = serde_json::from_str(&json).unwrap();
        assert_eq!(pattern, deserialized);
    }
}

// ============================================================================
// Integration Tests: Full Workflow
// ============================================================================

#[test]
fn test_DEPYLER_0903_full_enterprise_workflow() {
    // Simulate enterprise usage:
    // 1. Start with defaults
    // 2. Load enterprise extension
    // 3. Apply user override
    // 4. Lookup should respect priority

    let mut registry = MappingRegistry::with_defaults();

    // Step 2: Enterprise extension for internal library
    registry.register_extension(LibraryMapping {
        python_module: "company.ml".to_string(),
        rust_crate: "company_ml_rs".to_string(),
        python_version_req: ">=3.9".to_string(),
        rust_crate_version: "2.0".to_string(),
        items: HashMap::from([(
            "train".to_string(),
            ItemMapping {
                rust_name: "train_model".to_string(),
                pattern: TransformPattern::MethodCall { extra_args: vec![] },
                type_transform: None,
            },
        )]),
        features: vec!["cuda".to_string()],
        confidence: MappingConfidence::Verified,
        provenance: "internal://docs/ml-rs".to_string(),
    });

    // Step 3: User override for specific behavior
    registry.register_override(LibraryMapping {
        python_module: "company.ml".to_string(),
        rust_crate: "company_ml_rs".to_string(),
        python_version_req: ">=3.9".to_string(),
        rust_crate_version: "2.0".to_string(),
        items: HashMap::from([(
            "train".to_string(),
            ItemMapping {
                rust_name: "train_model_v2".to_string(), // User prefers v2
                pattern: TransformPattern::MethodCall { extra_args: vec![] },
                type_transform: None,
            },
        )]),
        features: vec!["cuda".to_string()],
        confidence: MappingConfidence::Verified,
        provenance: "user config".to_string(),
    });

    // Step 4: Verify priority
    let item = registry.lookup("company.ml", "train");
    assert!(item.is_some());
    assert_eq!(item.unwrap().rust_name, "train_model_v2"); // Override wins
}
