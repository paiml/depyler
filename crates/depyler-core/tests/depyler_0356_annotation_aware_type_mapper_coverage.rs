//! DEPYLER-0356: annotation_aware_type_mapper.rs Coverage Tests
//!
//! **EXTREME TDD Protocol - Coverage Boost**
//!
//! Target: annotation_aware_type_mapper.rs ~50% → 85%+ coverage
//! TDG Score: Excellent (A+) - Clean annotation-aware type mapping infrastructure
//!
//! This test suite complements the existing 6 module tests with additional coverage for:
//! - Constructor methods (new, with_base_mapper, Default)
//! - needs_reference_with_annotations() for all ownership models
//! - Conservative string strategy
//! - Borrowed/Shared ownership variants for collections
//! - Nested type mapping
//! - Fallback to base_mapper for non-annotated types
//! - Property-based tests for annotation combinations
//! - Edge cases and complex scenarios

use depyler_core::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use depyler_core::hir::Type as PythonType;
use depyler_core::type_mapper::{PrimitiveType, RustType, TypeMapper};
use depyler_annotations::*;

// ============================================================================
// CONSTRUCTOR TESTS
// ============================================================================

#[test]
fn test_DEPYLER_0356_new_constructor() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    // Verify it works with basic type mapping
    let rust_type = mapper.map_type_with_annotations(&PythonType::Int, &annotations);
    assert_eq!(rust_type, RustType::Primitive(PrimitiveType::I32));
}

#[test]
fn test_DEPYLER_0356_default_trait() {
    let mapper = AnnotationAwareTypeMapper::default();
    let annotations = TranspilationAnnotations::default();

    // Verify Default trait works identically to new()
    let rust_type = mapper.map_type_with_annotations(&PythonType::Bool, &annotations);
    assert_eq!(rust_type, RustType::Primitive(PrimitiveType::Bool));
}

#[test]
fn test_DEPYLER_0356_with_base_mapper() {
    let custom_base = TypeMapper::new();
    let mapper = AnnotationAwareTypeMapper::with_base_mapper(custom_base);
    let annotations = TranspilationAnnotations::default();

    // Verify custom base mapper is used
    let rust_type = mapper.map_type_with_annotations(&PythonType::Float, &annotations);
    assert_eq!(rust_type, RustType::Primitive(PrimitiveType::F64));
}

// ============================================================================
// STRING STRATEGY TESTS - CONSERVATIVE
// ============================================================================

#[test]
fn test_DEPYLER_0356_conservative_string_borrowed() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.string_strategy = StringStrategy::Conservative;
    annotations.ownership_model = OwnershipModel::Borrowed;

    let rust_type = mapper.map_type_with_annotations(&PythonType::String, &annotations);
    assert_eq!(
        rust_type,
        RustType::Str {
            lifetime: Some("'a".to_string())
        }
    );
}

#[test]
fn test_DEPYLER_0356_conservative_string_owned() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.string_strategy = StringStrategy::Conservative;
    annotations.ownership_model = OwnershipModel::Owned;

    let rust_type = mapper.map_type_with_annotations(&PythonType::String, &annotations);
    assert_eq!(rust_type, RustType::String);
}

#[test]
fn test_DEPYLER_0356_conservative_string_shared() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.string_strategy = StringStrategy::Conservative;
    annotations.ownership_model = OwnershipModel::Shared;

    let rust_type = mapper.map_type_with_annotations(&PythonType::String, &annotations);
    assert_eq!(rust_type, RustType::String);
}

// ============================================================================
// LIST OWNERSHIP TESTS - SHARED WITHOUT THREAD SAFETY
// ============================================================================

#[test]
fn test_DEPYLER_0356_list_shared_no_thread_safety() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Shared;
    annotations.thread_safety = ThreadSafety::NotRequired;

    let list_type = PythonType::List(Box::new(PythonType::String));
    let rust_type = mapper.map_type_with_annotations(&list_type, &annotations);

    assert_eq!(rust_type, RustType::Custom("Rc<Vec<String>>".to_string()));
}

#[test]
fn test_DEPYLER_0356_list_shared_with_thread_safety() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Shared;
    annotations.thread_safety = ThreadSafety::Required;

    let list_type = PythonType::List(Box::new(PythonType::Int));
    let rust_type = mapper.map_type_with_annotations(&list_type, &annotations);

    assert_eq!(rust_type, RustType::Custom("Arc<Vec<i32>>".to_string()));
}

// ============================================================================
// DICT OWNERSHIP TESTS - BORROWED AND SHARED
// ============================================================================

#[test]
fn test_DEPYLER_0356_dict_borrowed() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Borrowed;

    let dict_type = PythonType::Dict(Box::new(PythonType::String), Box::new(PythonType::Int));
    let rust_type = mapper.map_type_with_annotations(&dict_type, &annotations);

    match rust_type {
        RustType::Reference {
            lifetime,
            mutable,
            inner,
        } => {
            assert_eq!(lifetime, Some("'a".to_string()));
            assert!(!mutable);
            // Annotations apply recursively to nested types - String keys become &'a str
            assert_eq!(
                *inner,
                RustType::Custom("HashMap<&'a str, i32>".to_string())
            );
        }
        _ => panic!("Expected reference type, got {:?}", rust_type),
    }
}

#[test]
fn test_DEPYLER_0356_dict_shared_thread_safe() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Shared;
    annotations.thread_safety = ThreadSafety::Required;

    let dict_type = PythonType::Dict(Box::new(PythonType::String), Box::new(PythonType::Bool));
    let rust_type = mapper.map_type_with_annotations(&dict_type, &annotations);

    assert_eq!(
        rust_type,
        RustType::Custom("Arc<HashMap<String, bool>>".to_string())
    );
}

#[test]
fn test_DEPYLER_0356_dict_shared_not_thread_safe() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Shared;
    annotations.thread_safety = ThreadSafety::NotRequired;

    let dict_type = PythonType::Dict(Box::new(PythonType::Int), Box::new(PythonType::String));
    let rust_type = mapper.map_type_with_annotations(&dict_type, &annotations);

    assert_eq!(
        rust_type,
        RustType::Custom("Rc<HashMap<i32, String>>".to_string())
    );
}

// ============================================================================
// NEEDS_REFERENCE_WITH_ANNOTATIONS TESTS
// ============================================================================

#[test]
fn test_DEPYLER_0356_needs_reference_borrowed_non_copy() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Borrowed;

    let rust_type = RustType::String; // Non-Copy type
    assert!(mapper.needs_reference_with_annotations(&rust_type, &annotations));
}

#[test]
fn test_DEPYLER_0356_needs_reference_borrowed_copy() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Borrowed;

    let rust_type = RustType::Primitive(PrimitiveType::I32); // Copy type
    assert!(!mapper.needs_reference_with_annotations(&rust_type, &annotations));
}

#[test]
fn test_DEPYLER_0356_needs_reference_owned() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Owned;

    let rust_type = RustType::String;
    assert!(!mapper.needs_reference_with_annotations(&rust_type, &annotations));
}

#[test]
fn test_DEPYLER_0356_needs_reference_shared() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Shared;

    let rust_type = RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)));
    assert!(!mapper.needs_reference_with_annotations(&rust_type, &annotations));
}

// ============================================================================
// RETURN TYPE TESTS
// ============================================================================

#[test]
fn test_DEPYLER_0356_return_type_none_default_strategy() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let rust_type = mapper.map_return_type_with_annotations(&PythonType::None, &annotations);
    assert_eq!(rust_type, RustType::Unit);
}

#[test]
fn test_DEPYLER_0356_return_type_none_result_strategy() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.error_strategy = ErrorStrategy::ResultType;

    let rust_type = mapper.map_return_type_with_annotations(&PythonType::None, &annotations);
    assert_eq!(
        rust_type,
        RustType::Result(
            Box::new(RustType::Unit),
            Box::new(RustType::Custom("Error".to_string()))
        )
    );
}

#[test]
fn test_DEPYLER_0356_return_type_unknown() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let rust_type = mapper.map_return_type_with_annotations(&PythonType::Unknown, &annotations);
    assert_eq!(rust_type, RustType::Unit);
}

#[test]
fn test_DEPYLER_0356_return_type_concrete() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let rust_type = mapper.map_return_type_with_annotations(&PythonType::String, &annotations);
    assert_eq!(rust_type, RustType::String);
}

// ============================================================================
// FALLBACK TO BASE_MAPPER TESTS
// ============================================================================

#[test]
fn test_DEPYLER_0356_base_mapper_fallback_int() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let rust_type = mapper.map_type_with_annotations(&PythonType::Int, &annotations);
    assert_eq!(rust_type, RustType::Primitive(PrimitiveType::I32));
}

#[test]
fn test_DEPYLER_0356_base_mapper_fallback_float() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let rust_type = mapper.map_type_with_annotations(&PythonType::Float, &annotations);
    assert_eq!(rust_type, RustType::Primitive(PrimitiveType::F64));
}

#[test]
fn test_DEPYLER_0356_base_mapper_fallback_bool() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let rust_type = mapper.map_type_with_annotations(&PythonType::Bool, &annotations);
    assert_eq!(rust_type, RustType::Primitive(PrimitiveType::Bool));
}

#[test]
fn test_DEPYLER_0356_base_mapper_fallback_none() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let rust_type = mapper.map_type_with_annotations(&PythonType::None, &annotations);
    assert_eq!(rust_type, RustType::Unit);
}

#[test]
fn test_DEPYLER_0356_base_mapper_fallback_custom() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let custom_type = PythonType::Custom("MyClass".to_string());
    let rust_type = mapper.map_type_with_annotations(&custom_type, &annotations);
    assert_eq!(rust_type, RustType::Custom("MyClass".to_string()));
}

// ============================================================================
// NESTED TYPE TESTS
// ============================================================================

#[test]
fn test_DEPYLER_0356_nested_list_of_lists() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let nested_type = PythonType::List(Box::new(PythonType::List(Box::new(PythonType::Int))));
    let rust_type = mapper.map_type_with_annotations(&nested_type, &annotations);

    assert_eq!(
        rust_type,
        RustType::Vec(Box::new(RustType::Vec(Box::new(RustType::Primitive(
            PrimitiveType::I32
        )))))
    );
}

#[test]
fn test_DEPYLER_0356_nested_dict_with_list_values() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let complex_type = PythonType::Dict(
        Box::new(PythonType::String),
        Box::new(PythonType::List(Box::new(PythonType::Int))),
    );
    let rust_type = mapper.map_type_with_annotations(&complex_type, &annotations);

    assert_eq!(
        rust_type,
        RustType::Custom("HashMap<String, Vec<i32>>".to_string())
    );
}

#[test]
fn test_DEPYLER_0356_nested_optional_list() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let optional_list = PythonType::Optional(Box::new(PythonType::List(Box::new(
        PythonType::String,
    ))));
    let rust_type = mapper.map_type_with_annotations(&optional_list, &annotations);

    assert_eq!(
        rust_type,
        RustType::Option(Box::new(RustType::Vec(Box::new(RustType::String))))
    );
}

#[test]
fn test_DEPYLER_0356_nested_optional_dict() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    let optional_dict = PythonType::Optional(Box::new(PythonType::Dict(
        Box::new(PythonType::Int),
        Box::new(PythonType::Bool),
    )));
    let rust_type = mapper.map_type_with_annotations(&optional_dict, &annotations);

    assert_eq!(
        rust_type,
        RustType::Option(Box::new(RustType::Custom(
            "HashMap<i32, bool>".to_string()
        )))
    );
}

// ============================================================================
// COMPLEX ANNOTATION COMBINATION TESTS
// ============================================================================

#[test]
fn test_DEPYLER_0356_zero_copy_borrowed_list() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.string_strategy = StringStrategy::ZeroCopy;
    annotations.ownership_model = OwnershipModel::Borrowed;

    let list_of_strings = PythonType::List(Box::new(PythonType::String));
    let rust_type = mapper.map_type_with_annotations(&list_of_strings, &annotations);

    match rust_type {
        RustType::Reference {
            lifetime,
            mutable,
            inner,
        } => {
            assert_eq!(lifetime, Some("'a".to_string()));
            assert!(!mutable);
            // Inner should be Vec<&str>
            match *inner {
                RustType::Vec(inner_type) => match *inner_type {
                    RustType::Str { lifetime } => {
                        assert_eq!(lifetime, Some("'a".to_string()));
                    }
                    _ => panic!("Expected Str type"),
                },
                _ => panic!("Expected Vec type"),
            }
        }
        _ => panic!("Expected reference type"),
    }
}

#[test]
fn test_DEPYLER_0356_result_type_optional_string() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.error_strategy = ErrorStrategy::ResultType;

    let optional_string = PythonType::Optional(Box::new(PythonType::String));
    let rust_type = mapper.map_type_with_annotations(&optional_string, &annotations);

    assert_eq!(
        rust_type,
        RustType::Result(
            Box::new(RustType::String),
            Box::new(RustType::Custom("Error".to_string()))
        )
    );
}

#[test]
fn test_DEPYLER_0356_shared_thread_safe_nested_dict() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Shared;
    annotations.thread_safety = ThreadSafety::Required;

    let nested_dict = PythonType::Dict(
        Box::new(PythonType::String),
        Box::new(PythonType::Dict(
            Box::new(PythonType::Int),
            Box::new(PythonType::String),
        )),
    );
    let rust_type = mapper.map_type_with_annotations(&nested_dict, &annotations);

    // Shared ownership with thread safety applies recursively to nested collections
    assert_eq!(
        rust_type,
        RustType::Custom("Arc<HashMap<String, Arc<HashMap<i32, String>>>>".to_string())
    );
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_DEPYLER_0356_deeply_nested_types() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    // List<Optional<Dict<String, List<Int>>>>
    let deep_type = PythonType::List(Box::new(PythonType::Optional(Box::new(
        PythonType::Dict(
            Box::new(PythonType::String),
            Box::new(PythonType::List(Box::new(PythonType::Int))),
        ),
    ))));

    let rust_type = mapper.map_type_with_annotations(&deep_type, &annotations);

    assert_eq!(
        rust_type,
        RustType::Vec(Box::new(RustType::Option(Box::new(RustType::Custom(
            "HashMap<String, Vec<i32>>".to_string()
        )))))
    );
}

#[test]
fn test_DEPYLER_0356_mixed_ownership_annotations() {
    let mapper = AnnotationAwareTypeMapper::new();

    // Test that inner types inherit annotations
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Borrowed;
    annotations.string_strategy = StringStrategy::ZeroCopy;

    let dict_type = PythonType::Dict(Box::new(PythonType::String), Box::new(PythonType::String));
    let rust_type = mapper.map_type_with_annotations(&dict_type, &annotations);

    match rust_type {
        RustType::Reference { inner, .. } => {
            // Both key and value should be &str due to annotations
            let s = inner.to_rust_string();
            assert!(s.contains("&'a str"));
        }
        _ => panic!("Expected reference type"),
    }
}

#[test]
fn test_DEPYLER_0356_empty_annotations() {
    let mapper = AnnotationAwareTypeMapper::new();
    let annotations = TranspilationAnnotations::default();

    // Default annotations should still work
    let rust_type = mapper.map_type_with_annotations(&PythonType::String, &annotations);
    assert_eq!(rust_type, RustType::String);
}

// ============================================================================
// PROPERTY-BASED TESTS - Annotation Combinations
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn arb_ownership_model()(ownership in 0..3u8) -> OwnershipModel {
            match ownership {
                0 => OwnershipModel::Owned,
                1 => OwnershipModel::Borrowed,
                _ => OwnershipModel::Shared,
            }
        }
    }

    prop_compose! {
        fn arb_string_strategy()(strategy in 0..3u8) -> StringStrategy {
            match strategy {
                0 => StringStrategy::AlwaysOwned,
                1 => StringStrategy::ZeroCopy,
                _ => StringStrategy::Conservative,
            }
        }
    }

    prop_compose! {
        fn arb_thread_safety()(safety in 0..2u8) -> ThreadSafety {
            match safety {
                0 => ThreadSafety::NotRequired,
                _ => ThreadSafety::Required,
            }
        }
    }

    proptest! {
        #[test]
        fn prop_string_type_always_produces_valid_output(
            ownership in arb_ownership_model(),
            strategy in arb_string_strategy(),
        ) {
            let mapper = AnnotationAwareTypeMapper::new();
            let mut annotations = TranspilationAnnotations::default();
            annotations.ownership_model = ownership;
            annotations.string_strategy = strategy;

            let rust_type = mapper.map_type_with_annotations(&PythonType::String, &annotations);

            // Should always produce a valid RustType
            match rust_type {
                RustType::String | RustType::Str { .. } => {},
                _ => panic!("String type should map to String or Str, got {:?}", rust_type),
            }
        }

        #[test]
        fn prop_list_type_valid_with_all_ownership_models(
            ownership in arb_ownership_model(),
            thread_safety in arb_thread_safety(),
        ) {
            let mapper = AnnotationAwareTypeMapper::new();
            let mut annotations = TranspilationAnnotations::default();
            annotations.ownership_model = ownership;
            annotations.thread_safety = thread_safety;

            let list_type = PythonType::List(Box::new(PythonType::Int));
            let rust_type = mapper.map_type_with_annotations(&list_type, &annotations);

            // Should always produce a valid collection type
            match rust_type {
                RustType::Vec(_) | RustType::Reference { .. } | RustType::Custom(_) => {},
                _ => panic!("List type should produce valid collection type, got {:?}", rust_type),
            }
        }

        #[test]
        fn prop_dict_type_always_uses_hashmap(
            ownership in arb_ownership_model(),
            thread_safety in arb_thread_safety(),
        ) {
            let mapper = AnnotationAwareTypeMapper::new();
            let mut annotations = TranspilationAnnotations::default();
            annotations.ownership_model = ownership;
            annotations.thread_safety = thread_safety;

            let dict_type = PythonType::Dict(Box::new(PythonType::String), Box::new(PythonType::Int));
            let rust_type = mapper.map_type_with_annotations(&dict_type, &annotations);

            // Should always use HashMap (DEPYLER-0278)
            let rust_str = rust_type.to_rust_string();
            prop_assert!(
                rust_str.contains("HashMap"),
                "Dict should always use HashMap, got: {}", rust_str
            );
        }

        #[test]
        fn prop_needs_reference_consistency(
            ownership in arb_ownership_model(),
        ) {
            let mapper = AnnotationAwareTypeMapper::new();
            let mut annotations = TranspilationAnnotations::default();
            let ownership_copy = ownership.clone();
            annotations.ownership_model = ownership;

            let rust_type = RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)));
            let needs_ref = mapper.needs_reference_with_annotations(&rust_type, &annotations);

            // Borrowed ownership with non-Copy types should need reference
            match ownership_copy {
                OwnershipModel::Borrowed => prop_assert!(needs_ref),
                OwnershipModel::Owned | OwnershipModel::Shared => prop_assert!(!needs_ref),
            }
        }
    }
}

// ============================================================================
// INTEGRATION TESTS - End-to-End Scenarios
// ============================================================================

#[test]
fn test_DEPYLER_0356_integration_api_response_type() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Owned;
    annotations.error_strategy = ErrorStrategy::ResultType;

    // Simulate API response: Optional<Dict<String, List<String>>>
    let api_response_type = PythonType::Optional(Box::new(PythonType::Dict(
        Box::new(PythonType::String),
        Box::new(PythonType::List(Box::new(PythonType::String))),
    )));

    let rust_type = mapper.map_type_with_annotations(&api_response_type, &annotations);

    // Should map to Result<HashMap<String, Vec<String>>, Error>
    assert_eq!(
        rust_type,
        RustType::Result(
            Box::new(RustType::Custom("HashMap<String, Vec<String>>".to_string())),
            Box::new(RustType::Custom("Error".to_string()))
        )
    );
}

#[test]
fn test_DEPYLER_0356_integration_zero_copy_parser() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.string_strategy = StringStrategy::ZeroCopy;
    annotations.ownership_model = OwnershipModel::Borrowed;

    // Simulate parser return: Dict<String, String>
    let parser_return = PythonType::Dict(Box::new(PythonType::String), Box::new(PythonType::String));
    let rust_type = mapper.map_type_with_annotations(&parser_return, &annotations);

    // Should produce borrowed HashMap with &str keys and values
    match rust_type {
        RustType::Reference { inner, .. } => {
            let s = inner.to_rust_string();
            assert!(s.contains("HashMap"));
            assert!(s.contains("&'a str"));
        }
        _ => panic!("Expected reference to HashMap with &str"),
    }
}

#[test]
fn test_DEPYLER_0356_integration_thread_safe_cache() {
    let mapper = AnnotationAwareTypeMapper::new();
    let mut annotations = TranspilationAnnotations::default();
    annotations.ownership_model = OwnershipModel::Shared;
    annotations.thread_safety = ThreadSafety::Required;

    // Simulate thread-safe cache: Dict<String, Optional<List<Int>>>
    let cache_type = PythonType::Dict(
        Box::new(PythonType::String),
        Box::new(PythonType::Optional(Box::new(PythonType::List(Box::new(
            PythonType::Int,
        ))))),
    );

    let rust_type = mapper.map_type_with_annotations(&cache_type, &annotations);

    // Annotations apply recursively - nested List gets Arc-wrapped for thread safety
    assert_eq!(
        rust_type,
        RustType::Custom("Arc<HashMap<String, Option<Arc<Vec<i32>>>>>".to_string())
    );
}
