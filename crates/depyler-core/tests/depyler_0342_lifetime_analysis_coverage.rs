//! DEPYLER-0342: lifetime_analysis.rs Coverage Tests
//!
//! **EXTREME TDD Protocol - Coverage Boost**
//!
//! Target: lifetime_analysis.rs 35.45% → 85%+ coverage
//! TDG Score: 77.1/100 (B) - High complexity (cyclomatic: 114, cognitive: 144)
//!
//! This test suite validates the lifetime inference functionality:
//! - LifetimeInference initialization and lifetime generation
//! - ParamUsage tracking (mutated, moved, escaped flags)
//! - LifetimeInfo and LifetimeSource structures
//! - Basic constraint tracking

#![allow(non_snake_case)]

use depyler_core::lifetime_analysis::*;

// ============================================================================
// BASIC INITIALIZATION TESTS
// ============================================================================

#[test]
fn test_lifetime_inference_new() {
    let inference = LifetimeInference::new();

    // Should create successfully
    assert!(format!("{:?}", inference).contains("LifetimeInference"));
}

// ============================================================================
// LIFETIME SOURCE TESTS
// ============================================================================

#[test]
fn test_lifetime_source_parameter() {
    let source = LifetimeSource::Parameter("x".to_string());
    assert_eq!(source, LifetimeSource::Parameter("x".to_string()));
    assert!(matches!(source, LifetimeSource::Parameter(_)));
}

#[test]
fn test_lifetime_source_static_literal() {
    let source = LifetimeSource::StaticLiteral;
    assert_eq!(source, LifetimeSource::StaticLiteral);
}

#[test]
fn test_lifetime_source_local() {
    let source = LifetimeSource::Local;
    assert_eq!(source, LifetimeSource::Local);
}

#[test]
fn test_lifetime_source_return() {
    let source = LifetimeSource::Return;
    assert_eq!(source, LifetimeSource::Return);
}

#[test]
fn test_lifetime_source_field() {
    let source = LifetimeSource::Field("name".to_string());
    assert_eq!(source, LifetimeSource::Field("name".to_string()));
}

// ============================================================================
// PARAM USAGE TESTS
// ============================================================================

#[test]
fn test_param_usage_default() {
    let usage = ParamUsage::default();

    assert!(!usage.is_mutated);
    assert!(!usage.is_moved);
    assert!(!usage.escapes);
    assert!(!usage.is_read_only);
    assert!(!usage.used_in_loop);
    assert!(!usage.has_nested_borrows);
}

#[test]
fn test_param_usage_mutated() {
    let usage = ParamUsage {
        is_mutated: true,
        ..Default::default()
    };

    assert!(usage.is_mutated);
    assert!(!usage.is_moved);
}

#[test]
fn test_param_usage_moved() {
    let usage = ParamUsage {
        is_moved: true,
        ..Default::default()
    };

    assert!(usage.is_moved);
    assert!(!usage.is_mutated);
}

#[test]
fn test_param_usage_escapes() {
    let usage = ParamUsage {
        escapes: true,
        ..Default::default()
    };

    assert!(usage.escapes);
}

#[test]
fn test_param_usage_read_only() {
    let usage = ParamUsage {
        is_read_only: true,
        ..Default::default()
    };

    assert!(usage.is_read_only);
    assert!(!usage.is_mutated);
}

#[test]
fn test_param_usage_used_in_loop() {
    let usage = ParamUsage {
        used_in_loop: true,
        ..Default::default()
    };

    assert!(usage.used_in_loop);
}

#[test]
fn test_param_usage_nested_borrows() {
    let usage = ParamUsage {
        has_nested_borrows: true,
        ..Default::default()
    };

    assert!(usage.has_nested_borrows);
}

#[test]
fn test_param_usage_complex() {
    let usage = ParamUsage {
        is_mutated: true,
        escapes: true,
        used_in_loop: true,
        has_nested_borrows: true,
        is_moved: false,
        is_read_only: false,
    };

    assert!(usage.is_mutated);
    assert!(usage.escapes);
    assert!(usage.used_in_loop);
    assert!(usage.has_nested_borrows);
    assert!(!usage.is_moved);
    assert!(!usage.is_read_only);
}

// ============================================================================
// LIFETIME INFO TESTS
// ============================================================================

#[test]
fn test_lifetime_info_basic() {
    use std::collections::HashSet;

    let info = LifetimeInfo {
        name: "'a".to_string(),
        is_static: false,
        outlives: HashSet::new(),
        source: LifetimeSource::Parameter("x".to_string()),
    };

    assert_eq!(info.name, "'a");
    assert!(!info.is_static);
    assert!(info.outlives.is_empty());
    assert_eq!(info.source, LifetimeSource::Parameter("x".to_string()));
}

#[test]
fn test_lifetime_info_static() {
    use std::collections::HashSet;

    let info = LifetimeInfo {
        name: "'static".to_string(),
        is_static: true,
        outlives: HashSet::new(),
        source: LifetimeSource::StaticLiteral,
    };

    assert_eq!(info.name, "'static");
    assert!(info.is_static);
    assert_eq!(info.source, LifetimeSource::StaticLiteral);
}

#[test]
fn test_lifetime_info_with_outlives() {
    use std::collections::HashSet;

    let mut outlives = HashSet::new();
    outlives.insert("'b".to_string());
    outlives.insert("'c".to_string());

    let info = LifetimeInfo {
        name: "'a".to_string(),
        is_static: false,
        outlives,
        source: LifetimeSource::Parameter("x".to_string()),
    };

    assert_eq!(info.name, "'a");
    assert_eq!(info.outlives.len(), 2);
    assert!(info.outlives.contains("'b"));
    assert!(info.outlives.contains("'c"));
}

// ============================================================================
// LIFETIME CONSTRAINT TESTS
// ============================================================================

#[test]
fn test_lifetime_constraint_outlives() {
    let constraint = LifetimeConstraint::Outlives;
    assert!(matches!(constraint, LifetimeConstraint::Outlives));
}

#[test]
fn test_lifetime_constraint_equal() {
    let constraint = LifetimeConstraint::Equal;
    assert!(matches!(constraint, LifetimeConstraint::Equal));
}

#[test]
fn test_lifetime_constraint_at_least() {
    let constraint = LifetimeConstraint::AtLeast;
    assert!(matches!(constraint, LifetimeConstraint::AtLeast));
}

// ============================================================================
// INFERRED PARAM TESTS
// ============================================================================

#[test]
fn test_inferred_param_owned() {
    use depyler_core::type_mapper::RustType;

    let param = InferredParam {
        should_borrow: false,
        needs_mut: false,
        lifetime: None,
        rust_type: RustType::String,
    };

    assert!(!param.should_borrow);
    assert!(!param.needs_mut);
    assert!(param.lifetime.is_none());
}

#[test]
fn test_inferred_param_borrowed() {
    use depyler_core::type_mapper::RustType;

    let param = InferredParam {
        should_borrow: true,
        needs_mut: false,
        lifetime: Some("'a".to_string()),
        rust_type: RustType::String,
    };

    assert!(param.should_borrow);
    assert!(!param.needs_mut);
    assert_eq!(param.lifetime, Some("'a".to_string()));
}

#[test]
fn test_inferred_param_mutable_borrow() {
    use depyler_core::type_mapper::RustType;

    let param = InferredParam {
        should_borrow: true,
        needs_mut: true,
        lifetime: Some("'a".to_string()),
        rust_type: RustType::String,
    };

    assert!(param.should_borrow);
    assert!(param.needs_mut);
    assert_eq!(param.lifetime, Some("'a".to_string()));
}

// ============================================================================
// LIFETIME RESULT TESTS
// ============================================================================

#[test]
fn test_lifetime_result_empty() {
    use indexmap::IndexMap;

    let result = LifetimeResult {
        param_lifetimes: IndexMap::new(),
        return_lifetime: None,
        lifetime_params: vec![],
        lifetime_bounds: vec![],
        borrowing_strategies: IndexMap::new(),
    };

    assert!(result.param_lifetimes.is_empty());
    assert!(result.return_lifetime.is_none());
    assert!(result.lifetime_params.is_empty());
    assert!(result.lifetime_bounds.is_empty());
}

#[test]
fn test_lifetime_result_with_params() {
    use depyler_core::type_mapper::RustType;
    use indexmap::IndexMap;

    let mut param_lifetimes = IndexMap::new();
    param_lifetimes.insert(
        "x".to_string(),
        InferredParam {
            should_borrow: true,
            needs_mut: false,
            lifetime: Some("'a".to_string()),
            rust_type: RustType::String,
        },
    );

    let result = LifetimeResult {
        param_lifetimes,
        return_lifetime: Some("'a".to_string()),
        lifetime_params: vec!["'a".to_string()],
        lifetime_bounds: vec![],
        borrowing_strategies: IndexMap::new(),
    };

    assert_eq!(result.param_lifetimes.len(), 1);
    assert_eq!(result.return_lifetime, Some("'a".to_string()));
    assert_eq!(result.lifetime_params.len(), 1);
    assert_eq!(result.lifetime_params[0], "'a");
}

#[test]
fn test_lifetime_result_with_bounds() {
    use indexmap::IndexMap;

    let result = LifetimeResult {
        param_lifetimes: IndexMap::new(),
        return_lifetime: None,
        lifetime_params: vec!["'a".to_string(), "'b".to_string()],
        lifetime_bounds: vec![("'a".to_string(), "'b".to_string())],
        borrowing_strategies: IndexMap::new(),
    };

    assert_eq!(result.lifetime_params.len(), 2);
    assert_eq!(result.lifetime_bounds.len(), 1);
    assert_eq!(
        result.lifetime_bounds[0],
        ("'a".to_string(), "'b".to_string())
    );
}

// ============================================================================
// PROPERTY TESTS - Lifetime Robustness
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_lifetime_inference_initializes(
            _seed in 0u64..1000,
        ) {
            let inference = LifetimeInference::new();
            // Should always create successfully
            let debug_str = format!("{:?}", inference);
            assert!(debug_str.contains("LifetimeInference"));
        }

        #[test]
        fn prop_param_usage_flags_independent(
            is_mutated in prop::bool::ANY,
            is_moved in prop::bool::ANY,
            escapes in prop::bool::ANY,
        ) {
            let usage = ParamUsage {
                is_mutated,
                is_moved,
                escapes,
                is_read_only: false,
                used_in_loop: false,
                has_nested_borrows: false,
            };

            assert_eq!(usage.is_mutated, is_mutated);
            assert_eq!(usage.is_moved, is_moved);
            assert_eq!(usage.escapes, escapes);
        }

        #[test]
        fn prop_lifetime_info_name_preserved(
            name in "[a-z]{1,3}",
            is_static in prop::bool::ANY,
        ) {
            use std::collections::HashSet;

            let lifetime_name = format!("'{}", name);
            let info = LifetimeInfo {
                name: lifetime_name.clone(),
                is_static,
                outlives: HashSet::new(),
                source: LifetimeSource::Local,
            };

            assert_eq!(info.name, lifetime_name);
            assert_eq!(info.is_static, is_static);
        }
    }
}
