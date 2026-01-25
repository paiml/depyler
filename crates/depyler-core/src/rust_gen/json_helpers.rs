//! JSON serialization helper functions extracted from expr_gen.rs
//! DEPYLER-COVERAGE-95: Extracted for testability

use crate::hir::Type;

/// Helper: Check if a type should use serde_json::Value
/// DEPYLER-0726: Also check for Type::Custom("Any") after DEPYLER-0725 fix
/// DEPYLER-0773: Also check for "object" which is Python's top-level type
pub fn is_json_value_type(ty: &Type) -> bool {
    matches!(ty, Type::Unknown)
        || matches!(ty, Type::Custom(s) if s == "serde_json::Value" || s == "Value" || s == "Any" || s == "object")
}

/// Check if a type name indicates JSON value type
pub fn is_json_value_type_name(type_name: &str) -> bool {
    matches!(
        type_name,
        "serde_json::Value" | "Value" | "Any" | "object" | "unknown"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_is_json_value() {
        assert!(is_json_value_type(&Type::Unknown));
    }

    #[test]
    fn test_serde_json_value_is_json_value() {
        assert!(is_json_value_type(&Type::Custom(
            "serde_json::Value".to_string()
        )));
    }

    #[test]
    fn test_value_is_json_value() {
        assert!(is_json_value_type(&Type::Custom("Value".to_string())));
    }

    #[test]
    fn test_any_is_json_value() {
        assert!(is_json_value_type(&Type::Custom("Any".to_string())));
    }

    #[test]
    fn test_object_is_json_value() {
        assert!(is_json_value_type(&Type::Custom("object".to_string())));
    }

    #[test]
    fn test_string_is_not_json_value() {
        assert!(!is_json_value_type(&Type::String));
    }

    #[test]
    fn test_int_is_not_json_value() {
        assert!(!is_json_value_type(&Type::Int));
    }

    #[test]
    fn test_custom_type_is_not_json_value() {
        assert!(!is_json_value_type(&Type::Custom("MyStruct".to_string())));
    }

    #[test]
    fn test_list_is_not_json_value() {
        assert!(!is_json_value_type(&Type::List(Box::new(Type::String))));
    }

    #[test]
    fn test_dict_is_not_json_value() {
        assert!(!is_json_value_type(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int)
        )));
    }

    // Type name tests
    #[test]
    fn test_type_name_serde_json_value() {
        assert!(is_json_value_type_name("serde_json::Value"));
    }

    #[test]
    fn test_type_name_value() {
        assert!(is_json_value_type_name("Value"));
    }

    #[test]
    fn test_type_name_any() {
        assert!(is_json_value_type_name("Any"));
    }

    #[test]
    fn test_type_name_object() {
        assert!(is_json_value_type_name("object"));
    }

    #[test]
    fn test_type_name_unknown() {
        assert!(is_json_value_type_name("unknown"));
    }

    #[test]
    fn test_type_name_string_not_json() {
        assert!(!is_json_value_type_name("String"));
    }

    #[test]
    fn test_type_name_custom_not_json() {
        assert!(!is_json_value_type_name("MyCustomType"));
    }
}
