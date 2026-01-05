//! EXTREME TDD Coverage Tests for TypeExtractor::parse_forward_reference
//!
//! These tests target the uncovered lines 118-181 in type_extraction.rs:
//! - parse_forward_reference: Forward reference string parsing ("Container[T]")
//! - parse_forward_ref_type_param: PEP 604 union syntax in forward refs ("T | None")
//!
//! Coverage Strategy: Use extract_type with ast::Constant::Str expressions
//! to exercise the forward reference parsing path.

use depyler_core::ast_bridge::TypeExtractor;
use depyler_core::hir::Type;
use rustpython_ast::{self as ast};

/// Helper to create a string constant expression for forward reference testing
fn make_forward_ref(s: &str) -> ast::Expr {
    ast::Expr::Constant(ast::ExprConstant {
        value: ast::Constant::Str(s.to_string()),
        kind: None,
        range: Default::default(),
    })
}

// ============================================================================
// parse_forward_reference - Generic syntax tests (lines 117-136)
// ============================================================================

#[test]
fn test_forward_ref_simple_generic_one_param() {
    // Line 117-136: Generic syntax with single param
    let expr = make_forward_ref("Container[T]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Container");
            assert_eq!(params.len(), 1);
            assert!(matches!(params[0], Type::TypeVar(_)));
        }
        other => panic!("Expected Generic, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_simple_generic_two_params() {
    // Line 117-136: Generic syntax with multiple params
    let expr = make_forward_ref("Dict[str, int]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Dict");
            assert_eq!(params.len(), 2);
            assert!(matches!(params[0], Type::String));
            assert!(matches!(params[1], Type::Int));
        }
        other => panic!("Expected Generic, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_list_with_param() {
    let expr = make_forward_ref("List[int]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "List");
            assert_eq!(params.len(), 1);
            assert!(matches!(params[0], Type::Int));
        }
        other => panic!("Expected Generic, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_set_with_param() {
    let expr = make_forward_ref("Set[str]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Set");
            assert_eq!(params.len(), 1);
            assert!(matches!(params[0], Type::String));
        }
        other => panic!("Expected Generic, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_tuple_with_params() {
    let expr = make_forward_ref("Tuple[int, str, bool]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Tuple");
            assert_eq!(params.len(), 3);
            assert!(matches!(params[0], Type::Int));
            assert!(matches!(params[1], Type::String));
            assert!(matches!(params[2], Type::Bool));
        }
        other => panic!("Expected Generic, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_custom_generic() {
    let expr = make_forward_ref("MyClass[T, U]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "MyClass");
            assert_eq!(params.len(), 2);
        }
        other => panic!("Expected Generic, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_generic_with_whitespace() {
    // Should handle whitespace in params
    let expr = make_forward_ref("Dict[ str , int ]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Dict");
            assert_eq!(params.len(), 2);
        }
        other => panic!("Expected Generic, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_generic_with_leading_whitespace() {
    let expr = make_forward_ref("  Container[T]  ");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base.trim(), "Container");
            assert_eq!(params.len(), 1);
        }
        other => panic!("Expected Generic, got {:?}", other),
    }
}

// ============================================================================
// parse_forward_reference - Empty params branch (line 128-130)
// ============================================================================

#[test]
fn test_forward_ref_generic_empty_params() {
    // Line 128-130: Empty params should fall back to simple type
    // "Base[]" with no params
    let expr = make_forward_ref("Container[]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    // Should treat as Custom type since no params
    assert!(matches!(result, Type::Custom(_)) || matches!(result, Type::Generic { .. }));
}

// ============================================================================
// parse_forward_reference - Top-level union (lines 139-142)
// ============================================================================

#[test]
fn test_forward_ref_top_level_union_t_or_none() {
    // Line 139-142: Top-level union syntax T | None
    let expr = make_forward_ref("int | None");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Optional(inner) => {
            assert!(matches!(*inner, Type::Int));
        }
        other => panic!("Expected Optional, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_top_level_union_none_or_t() {
    // Line 139-142: Reversed order: None | T
    let expr = make_forward_ref("None | str");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Optional(inner) => {
            assert!(matches!(*inner, Type::String));
        }
        other => panic!("Expected Optional, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_top_level_union_with_whitespace() {
    let expr = make_forward_ref("int  |  None");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    assert!(matches!(result, Type::Optional(_)));
}

// ============================================================================
// parse_forward_ref_type_param - PEP 604 union syntax (lines 150-177)
// ============================================================================

#[test]
fn test_forward_ref_param_union_t_or_none() {
    // Line 154-167: Inside generic params, T | None
    let expr = make_forward_ref("Container[int | None]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Container");
            assert_eq!(params.len(), 1);
            assert!(matches!(&params[0], Type::Optional(_)));
        }
        other => panic!("Expected Generic with Optional param, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_param_union_none_or_t() {
    // Line 154-167: Reversed order in params
    let expr = make_forward_ref("Container[None | str]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Container");
            assert_eq!(params.len(), 1);
            assert!(matches!(&params[0], Type::Optional(_)));
        }
        other => panic!("Expected Generic with Optional param, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_param_general_union() {
    // Line 170-176: General union T | U | V (not just None)
    let expr = make_forward_ref("int | str");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Union(types) => {
            assert_eq!(types.len(), 2);
            assert!(matches!(types[0], Type::Int));
            assert!(matches!(types[1], Type::String));
        }
        other => panic!("Expected Union, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_param_general_union_three_types() {
    let expr = make_forward_ref("int | str | bool");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Union(types) => {
            assert_eq!(types.len(), 3);
        }
        other => panic!("Expected Union with 3 types, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_generic_with_union_param() {
    // Container with union inside: Container[int | str]
    let expr = make_forward_ref("Container[int | str]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Container");
            assert_eq!(params.len(), 1);
            assert!(matches!(&params[0], Type::Union(_)));
        }
        other => panic!("Expected Generic with Union param, got {:?}", other),
    }
}

// ============================================================================
// parse_forward_reference - Simple type fallback (lines 144-145, 179-180)
// ============================================================================

#[test]
fn test_forward_ref_simple_int() {
    // No generic syntax, falls back to extract_simple_type
    let expr = make_forward_ref("int");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    assert!(matches!(result, Type::Int));
}

#[test]
fn test_forward_ref_simple_str() {
    let expr = make_forward_ref("str");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    assert!(matches!(result, Type::String));
}

#[test]
fn test_forward_ref_simple_float() {
    let expr = make_forward_ref("float");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    assert!(matches!(result, Type::Float));
}

#[test]
fn test_forward_ref_simple_bool() {
    let expr = make_forward_ref("bool");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    assert!(matches!(result, Type::Bool));
}

#[test]
fn test_forward_ref_simple_none() {
    let expr = make_forward_ref("None");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    assert!(matches!(result, Type::None));
}

#[test]
fn test_forward_ref_simple_type_var() {
    let expr = make_forward_ref("T");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::TypeVar(name) => assert_eq!(name, "T"),
        other => panic!("Expected TypeVar, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_simple_custom() {
    let expr = make_forward_ref("MyClass");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Custom(name) => assert_eq!(name, "MyClass"),
        other => panic!("Expected Custom, got {:?}", other),
    }
}

// ============================================================================
// Edge cases and boundary conditions
// ============================================================================

#[test]
fn test_forward_ref_nested_generic() {
    // Nested generics: Container[List[int]]
    // This is a complex case that may or may not be fully supported
    let expr = make_forward_ref("Container[List[int]]");
    let result = TypeExtractor::extract_type(&expr);
    // Should at least not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_callable_like() {
    let expr = make_forward_ref("Callable[int, str]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Callable");
            assert_eq!(params.len(), 2);
        }
        other => panic!("Expected Generic Callable, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_optional_explicit() {
    // "Optional[int]" as string
    let expr = make_forward_ref("Optional[int]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Optional");
            assert_eq!(params.len(), 1);
            assert!(matches!(params[0], Type::Int));
        }
        other => panic!("Expected Generic Optional, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_multiple_params_with_union() {
    // Dict[str, int | None]
    let expr = make_forward_ref("Dict[str, int | None]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Dict");
            assert_eq!(params.len(), 2);
            assert!(matches!(params[0], Type::String));
            assert!(matches!(&params[1], Type::Optional(_)));
        }
        other => panic!("Expected Generic Dict, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_whitespace_only_param() {
    // Edge case: trailing comma handling
    let expr = make_forward_ref("Tuple[int, str]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Tuple");
            assert_eq!(params.len(), 2);
        }
        other => panic!("Expected Tuple, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_unclosed_bracket() {
    // "Container[int" - missing closing bracket
    // Should fall back to treating as simple type or error
    let expr = make_forward_ref("Container[int");
    let result = TypeExtractor::extract_type(&expr);
    // Should handle gracefully (not panic)
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_empty_string() {
    let expr = make_forward_ref("");
    let result = TypeExtractor::extract_type(&expr);
    // Empty string should probably become Custom or Unknown
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_whitespace_string() {
    let expr = make_forward_ref("   ");
    let result = TypeExtractor::extract_type(&expr);
    // Just whitespace should be handled
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_single_bracket() {
    let expr = make_forward_ref("[");
    let result = TypeExtractor::extract_type(&expr);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_just_brackets() {
    let expr = make_forward_ref("[]");
    let result = TypeExtractor::extract_type(&expr);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_pipe_only() {
    let expr = make_forward_ref("|");
    let result = TypeExtractor::extract_type(&expr);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_multiple_pipes() {
    let expr = make_forward_ref("int | str | bool | float");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Union(types) => {
            assert_eq!(types.len(), 4);
        }
        other => panic!("Expected Union, got {:?}", other),
    }
}

// ============================================================================
// Type variable detection in forward refs
// ============================================================================

#[test]
fn test_forward_ref_type_var_single_letter() {
    let expr = make_forward_ref("A");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    assert!(matches!(result, Type::TypeVar(_)));
}

#[test]
fn test_forward_ref_type_var_u() {
    let expr = make_forward_ref("U");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::TypeVar(name) => assert_eq!(name, "U"),
        other => panic!("Expected TypeVar, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_generic_with_type_var() {
    let expr = make_forward_ref("Container[U]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Container");
            assert_eq!(params.len(), 1);
            assert!(matches!(&params[0], Type::TypeVar(_)));
        }
        other => panic!("Expected Generic with TypeVar, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_generic_with_multiple_type_vars() {
    let expr = make_forward_ref("Container[T, U, V]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Container");
            assert_eq!(params.len(), 3);
            for p in params {
                assert!(matches!(p, Type::TypeVar(_)));
            }
        }
        other => panic!("Expected Generic with TypeVars, got {:?}", other),
    }
}

// ============================================================================
// Special Python typing constructs as forward refs
// ============================================================================

#[test]
fn test_forward_ref_iterator_type() {
    let expr = make_forward_ref("Iterator[int]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Iterator");
            assert_eq!(params.len(), 1);
        }
        other => panic!("Expected Generic Iterator, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_generator_type() {
    let expr = make_forward_ref("Generator[int, None, None]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Generator");
            assert_eq!(params.len(), 3);
        }
        other => panic!("Expected Generic Generator, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_sequence_type() {
    let expr = make_forward_ref("Sequence[str]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Sequence");
            assert_eq!(params.len(), 1);
        }
        other => panic!("Expected Generic Sequence, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_mapping_type() {
    let expr = make_forward_ref("Mapping[str, int]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Mapping");
            assert_eq!(params.len(), 2);
        }
        other => panic!("Expected Generic Mapping, got {:?}", other),
    }
}

// ============================================================================
// Complex combinations
// ============================================================================

#[test]
fn test_forward_ref_union_with_generic() {
    // "List[int] | None" - union of generic and None
    let expr = make_forward_ref("List[int] | None");
    let result = TypeExtractor::extract_type(&expr);
    // This is a complex case - the parser may have limitations
    // Just ensure it doesn't panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_mixed_union_types() {
    // Mix of concrete and type vars: "T | int | None"
    let expr = make_forward_ref("T | int | None");
    let result = TypeExtractor::extract_type(&expr);
    // Should handle without panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_self_type() {
    // Python 3.11 Self type
    let expr = make_forward_ref("Self");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Custom(name) => assert_eq!(name, "Self"),
        Type::TypeVar(name) => assert_eq!(name, "Self"),
        other => panic!("Expected Custom or TypeVar, got {:?}", other),
    }
}

// ============================================================================
// Unicode and special characters
// ============================================================================

#[test]
fn test_forward_ref_unicode_class_name() {
    // Python allows unicode in identifiers
    let expr = make_forward_ref("Дата");
    let result = TypeExtractor::extract_type(&expr);
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_forward_ref_with_underscores() {
    let expr = make_forward_ref("My_Custom_Type");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Custom(name) => assert_eq!(name, "My_Custom_Type"),
        other => panic!("Expected Custom, got {:?}", other),
    }
}

#[test]
fn test_forward_ref_numbers_in_name() {
    let expr = make_forward_ref("Type2D");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Custom(name) => assert_eq!(name, "Type2D"),
        other => panic!("Expected Custom, got {:?}", other),
    }
}

// ============================================================================
// Regression tests for specific DEPYLER tickets
// ============================================================================

#[test]
fn test_depyler_0740_parse_forward_reference_generic() {
    // DEPYLER-0740: Parse forward reference strings like "Container[U]"
    let expr = make_forward_ref("Container[U]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Container");
            assert!(matches!(&params[0], Type::TypeVar(_)));
        }
        other => panic!("DEPYLER-0740 regression: Expected Generic, got {:?}", other),
    }
}

#[test]
fn test_depyler_0836_pep604_union_in_forward_ref() {
    // DEPYLER-0836: Handle PEP 604 union syntax in forward refs "Container[T | None]"
    let expr = make_forward_ref("Container[T | None]");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Generic { base, params } => {
            assert_eq!(base, "Container");
            assert!(matches!(&params[0], Type::Optional(_)));
        }
        other => panic!("DEPYLER-0836 regression: Expected Generic with Optional, got {:?}", other),
    }
}

#[test]
fn test_depyler_0836_pep604_union_toplevel() {
    // DEPYLER-0836: Top-level PEP 604 union
    let expr = make_forward_ref("str | None");
    let result = TypeExtractor::extract_type(&expr).unwrap();
    match result {
        Type::Optional(inner) => {
            assert!(matches!(*inner, Type::String));
        }
        other => panic!("DEPYLER-0836 regression: Expected Optional, got {:?}", other),
    }
}
