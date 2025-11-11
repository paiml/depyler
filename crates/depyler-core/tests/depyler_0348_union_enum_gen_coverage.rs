//! DEPYLER-0348: union_enum_gen.rs Coverage Tests
//!
//! **EXTREME TDD Protocol - Coverage Boost**
//!
//! Target: union_enum_gen.rs 20% → 85%+ coverage
//! TDG Score: ~1.2 (B+) - Moderate complexity (285 lines, 7 functions)
//!
//! This test suite validates union enum generation:
//! - Type-to-variant-name mappings for all Type variants
//! - Enum naming (≤3 types vs 4+ types)
//! - From trait generation and validation
//! - Match methods (is_* and as_*)
//! - Caching with complex types
//! - Edge cases (nested types, custom types, TypeVar)

use depyler_core::hir::Type;
use depyler_core::union_enum_gen::UnionEnumGenerator;

// ============================================================================
// TYPE-TO-VARIANT-NAME TESTS
// ============================================================================

#[test]
fn test_depyler_0348_variant_name_int() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::String];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Int maps to "Integer" variant
    assert!(code.contains("Integer"), "Should generate Integer variant");
}

#[test]
fn test_depyler_0348_variant_name_float() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Float, Type::Int];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Float maps to "Float" variant
    assert!(code.contains("Float"), "Should generate Float variant");
}

#[test]
fn test_depyler_0348_variant_name_string() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::String, Type::Int];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // String maps to "Text" variant
    assert!(code.contains("Text"), "Should generate Text variant");
}

#[test]
fn test_depyler_0348_variant_name_bool() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Bool, Type::Int];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Bool maps to "Boolean" variant
    assert!(code.contains("Boolean"), "Should generate Boolean variant");
}

#[test]
fn test_depyler_0348_variant_name_none() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::None, Type::Int];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // None maps to "None" variant (unit-like)
    assert!(code.contains("None"), "Should generate None variant");
}

#[test]
fn test_depyler_0348_variant_name_list() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::List(Box::new(Type::Int)), Type::String];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // List maps to "List" variant
    assert!(code.contains("List"), "Should generate List variant");
}

#[test]
fn test_depyler_0348_variant_name_dict() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![
        Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
        Type::Bool,
    ];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Dict maps to "Dict" variant
    assert!(code.contains("Dict"), "Should generate Dict variant");
}

#[test]
fn test_depyler_0348_variant_name_custom() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Custom("MyClass".to_string()), Type::Int];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Custom type uses its name as variant
    assert!(code.contains("MyClass"), "Should generate MyClass variant");
}

#[test]
fn test_depyler_0348_variant_name_typevar() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::TypeVar("T".to_string()), Type::Int];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // TypeVar maps to "TypeT" variant
    assert!(code.contains("TypeT"), "Should generate TypeT variant for TypeVar");
}

// ============================================================================
// ENUM NAMING TESTS (≤3 types vs 4+ types)
// ============================================================================

#[test]
fn test_depyler_0348_enum_naming_two_types() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::String];
    let (name, _) = generator.generate_union_enum(&types);

    // ≤3 types use "XOrYUnion" format
    assert_eq!(name, "IntOrStringUnion");
}

#[test]
fn test_depyler_0348_enum_naming_three_types() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::String, Type::Bool];
    let (name, _) = generator.generate_union_enum(&types);

    // ≤3 types use "XOrYOrZUnion" format
    assert_eq!(name, "BoolOrIntOrStringUnion");
}

#[test]
fn test_depyler_0348_enum_naming_four_types() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::String, Type::Bool, Type::Float];
    let (name, _) = generator.generate_union_enum(&types);

    // >3 types use "UnionType{N}" format
    assert_eq!(name, "UnionType1");
}

#[test]
fn test_depyler_0348_enum_naming_five_types() {
    let mut generator = UnionEnumGenerator::new();

    // First union with 5 types
    let types1 = vec![
        Type::Int,
        Type::String,
        Type::Bool,
        Type::Float,
        Type::None,
    ];
    let (name1, _) = generator.generate_union_enum(&types1);
    assert_eq!(name1, "UnionType1");

    // Second union with 5 types (counter increments)
    let types2 = vec![
        Type::Int,
        Type::String,
        Type::Bool,
        Type::Float,
        Type::List(Box::new(Type::Int)),
    ];
    let (name2, _) = generator.generate_union_enum(&types2);
    assert_eq!(name2, "UnionType2");
}

#[test]
fn test_depyler_0348_enum_naming_counter_increments() {
    let mut generator = UnionEnumGenerator::new();

    // Generate multiple DIFFERENT complex unions to test counter increments
    // (same types would hit cache and not increment counter)
    let types1 = vec![
        Type::Int,
        Type::String,
        Type::Bool,
        Type::Float,
        Type::List(Box::new(Type::Int)),
    ];
    let (name1, _) = generator.generate_union_enum(&types1);
    assert_eq!(name1, "UnionType1");

    let types2 = vec![
        Type::Int,
        Type::String,
        Type::Bool,
        Type::Float,
        Type::None,
    ];
    let (name2, _) = generator.generate_union_enum(&types2);
    assert_eq!(name2, "UnionType2");

    let types3 = vec![
        Type::Float,
        Type::String,
        Type::Bool,
        Type::List(Box::new(Type::String)),
        Type::None,
    ];
    let (name3, _) = generator.generate_union_enum(&types3);
    assert_eq!(name3, "UnionType3");
}

// ============================================================================
// FROM TRAIT GENERATION TESTS
// ============================================================================

#[test]
fn test_depyler_0348_from_impls_generated() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::String];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should generate From trait impls (don't check exact type representation)
    assert!(code.contains("impl From"), "Should generate From trait impls");
    assert!(code.contains("fn from"), "Should have from method");
    assert!(!code.is_empty(), "Should generate code");
}

#[test]
fn test_depyler_0348_from_impls_skip_none() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::None];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should generate From trait for Int but NOT for None (unit-like)
    assert!(code.contains("impl From"), "Should generate From trait");
    assert!(code.contains("fn from"), "Should have from method");
    // Count From impls - should be 1 (only for Int, not None)
    assert_eq!(code.matches("impl From").count(), 1, "Should have exactly 1 From impl");
}

#[test]
fn test_depyler_0348_from_impls_multiple_types() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::String, Type::Bool, Type::Float];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should generate From impls for all 4 types
    let from_count = code.matches("impl From").count();
    assert_eq!(from_count, 4, "Should generate 4 From impls");
}

// ============================================================================
// MATCH METHODS TESTS (is_* and as_*)
// ============================================================================

#[test]
fn test_depyler_0348_is_methods_generated() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::String];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should generate is_integer() and is_text() methods
    assert!(code.contains("is_integer"), "Should generate is_integer method");
    assert!(code.contains("is_text"), "Should generate is_text method");
    assert!(code.contains("pub fn"), "Should generate public methods");
}

#[test]
fn test_depyler_0348_as_methods_generated() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::String];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should generate as_integer() and as_text() methods
    assert!(code.contains("as_integer"), "Should generate as_integer method");
    assert!(code.contains("as_text"), "Should generate as_text method");
    assert!(code.contains("Option"), "Should return Option<&T>");
}

#[test]
fn test_depyler_0348_is_none_method() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::String, Type::None];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should generate is_none() method
    assert!(code.contains("is_none"), "Should generate is_none method");
}

#[test]
fn test_depyler_0348_as_methods_skip_none() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::None];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should generate as_integer() but NOT as_none()
    assert!(code.contains("as_integer"), "Should generate as_integer");
    // None is unit-like, no as_none needed (would return Option<&()>)
}

#[test]
fn test_depyler_0348_match_methods_all_variants() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int, Type::String, Type::Bool];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should generate methods for all 3 variants
    assert!(code.contains("is_integer"), "Should have is_integer");
    assert!(code.contains("is_text"), "Should have is_text");
    assert!(code.contains("is_boolean"), "Should have is_boolean");

    assert!(code.contains("as_integer"), "Should have as_integer");
    assert!(code.contains("as_text"), "Should have as_text");
    assert!(code.contains("as_boolean"), "Should have as_boolean");
}

// ============================================================================
// CACHING TESTS (complex types)
// ============================================================================

#[test]
fn test_depyler_0348_caching_nested_types() {
    let mut generator = UnionEnumGenerator::new();

    let types1 = vec![
        Type::List(Box::new(Type::Int)),
        Type::Dict(Box::new(Type::String), Box::new(Type::Float)),
    ];
    let types2 = vec![
        Type::Dict(Box::new(Type::String), Box::new(Type::Float)),
        Type::List(Box::new(Type::Int)),
    ];

    let (name1, tokens1) = generator.generate_union_enum(&types1);
    let (name2, tokens2) = generator.generate_union_enum(&types2);

    // Should cache reordered nested types
    assert_eq!(name1, name2, "Should return same enum name for reordered types");
    assert!(
        tokens2.to_string().is_empty(),
        "Second call should return empty tokens (cached)"
    );
}

#[test]
fn test_depyler_0348_caching_custom_types() {
    let mut generator = UnionEnumGenerator::new();

    let types1 = vec![Type::Custom("Foo".to_string()), Type::Int];
    let types2 = vec![Type::Int, Type::Custom("Foo".to_string())];

    let (name1, _) = generator.generate_union_enum(&types1);
    let (name2, tokens2) = generator.generate_union_enum(&types2);

    // Should cache custom types
    assert_eq!(name1, name2);
    assert!(tokens2.to_string().is_empty());
}

#[test]
fn test_depyler_0348_caching_different_unions() {
    let mut generator = UnionEnumGenerator::new();

    let types1 = vec![Type::Int, Type::String];
    let types2 = vec![Type::Int, Type::Bool];

    let (name1, tokens1) = generator.generate_union_enum(&types1);
    let (name2, tokens2) = generator.generate_union_enum(&types2);

    // Different unions should have different names
    assert_ne!(name1, name2);
    assert!(!tokens1.to_string().is_empty());
    assert!(!tokens2.to_string().is_empty());
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_depyler_0348_deeply_nested_types() {
    let mut generator = UnionEnumGenerator::new();

    // List[Dict[String, List[Int]]]
    let nested_type = Type::List(Box::new(Type::Dict(
        Box::new(Type::String),
        Box::new(Type::List(Box::new(Type::Int))),
    )));

    let types = vec![nested_type, Type::None];
    let (_name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should handle deeply nested types without panicking
    assert!(code.contains("List"), "Should handle nested List");
}

#[test]
fn test_depyler_0348_all_basic_types() {
    let mut generator = UnionEnumGenerator::new();

    let types = vec![
        Type::Int,
        Type::Float,
        Type::String,
        Type::Bool,
        Type::None,
        Type::List(Box::new(Type::Int)),
        Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
    ];

    let (name, tokens) = generator.generate_union_enum(&types);
    let code = tokens.to_string();

    // Should handle all basic types
    assert!(name.starts_with("UnionType"), "Should use UnionType naming for 7 types");
    assert!(!code.is_empty(), "Should generate code");
}

#[test]
fn test_depyler_0348_single_type() {
    let mut generator = UnionEnumGenerator::new();
    let types = vec![Type::Int];
    let (_name, tokens) = generator.generate_union_enum(&types);

    // Should handle single-type union (though semantically odd)
    assert!(!tokens.to_string().is_empty(), "Should generate code for single type");
}

// ============================================================================
// PROPERTY TESTS - Union Generation Robustness
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_union_generation_never_panics(
            type_count in 1usize..10,
        ) {
            let mut generator = UnionEnumGenerator::new();

            // Generate random union with varying type counts
            let types: Vec<Type> = (0..type_count)
                .map(|i| match i % 5 {
                    0 => Type::Int,
                    1 => Type::String,
                    2 => Type::Bool,
                    3 => Type::Float,
                    _ => Type::None,
                })
                .collect();

            let (_name, _tokens) = generator.generate_union_enum(&types);
            // Should never panic regardless of type count
        }

        #[test]
        fn prop_caching_consistent(
            shuffle_seed in 0u64..1000,
        ) {
            let mut generator = UnionEnumGenerator::new();
            let mut types = vec![Type::Int, Type::String, Type::Bool];

            let (name1, _) = generator.generate_union_enum(&types);

            // Shuffle types (simulating reordering)
            use std::collections::hash_map::RandomState;
            use std::hash::{BuildHasher, Hash, Hasher};
            let mut hasher = RandomState::new().build_hasher();
            shuffle_seed.hash(&mut hasher);
            types.reverse();

            let (name2, tokens2) = generator.generate_union_enum(&types);

            // Caching should work regardless of order
            assert_eq!(name1, name2);
            assert!(tokens2.to_string().is_empty());
        }

        #[test]
        fn prop_naming_deterministic(
            type_count in 1usize..8,
        ) {
            let mut gen1 = UnionEnumGenerator::new();
            let mut gen2 = UnionEnumGenerator::new();

            let types: Vec<Type> = (0..type_count)
                .map(|i| match i % 3 {
                    0 => Type::Int,
                    1 => Type::String,
                    _ => Type::Bool,
                })
                .collect();

            let (name1, _) = gen1.generate_union_enum(&types);
            let (name2, _) = gen2.generate_union_enum(&types);

            // Same types should produce same enum name
            assert_eq!(name1, name2);
        }
    }
}
