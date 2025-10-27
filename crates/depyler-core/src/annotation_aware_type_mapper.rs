use crate::hir::Type as PythonType;
use crate::type_mapper::{RustType, TypeMapper};
use depyler_annotations::{
    OwnershipModel, StringStrategy as AnnotationStringStrategy, TranspilationAnnotations,
};

/// An enhanced type mapper that considers annotations when mapping types
pub struct AnnotationAwareTypeMapper {
    base_mapper: TypeMapper,
}

impl Default for AnnotationAwareTypeMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl AnnotationAwareTypeMapper {
    pub fn new() -> Self {
        Self {
            base_mapper: TypeMapper::new(),
        }
    }

    pub fn with_base_mapper(base_mapper: TypeMapper) -> Self {
        Self { base_mapper }
    }

    /// Maps a Python type to a Rust type, considering the provided annotations
    pub fn map_type_with_annotations(
        &self,
        py_type: &PythonType,
        annotations: &TranspilationAnnotations,
    ) -> RustType {
        match py_type {
            PythonType::String => self.map_string_type(annotations),
            PythonType::List(inner) => self.map_list_type(inner, annotations),
            PythonType::Dict(key, value) => self.map_dict_type(key, value, annotations),
            PythonType::Optional(inner) => self.map_optional_type(inner, annotations),
            _ => self.base_mapper.map_type(py_type),
        }
    }

    /// Maps string types based on annotations
    fn map_string_type(&self, annotations: &TranspilationAnnotations) -> RustType {
        match annotations.string_strategy {
            AnnotationStringStrategy::AlwaysOwned => RustType::String,
            AnnotationStringStrategy::ZeroCopy => match annotations.ownership_model {
                OwnershipModel::Borrowed => RustType::Str {
                    lifetime: Some("'a".to_string()),
                },
                _ => RustType::String, // Can't do zero-copy without borrowing
            },
            AnnotationStringStrategy::Conservative => {
                // Conservative defaults to owned unless explicitly borrowed
                match annotations.ownership_model {
                    OwnershipModel::Borrowed => RustType::Str {
                        lifetime: Some("'a".to_string()),
                    },
                    _ => RustType::String,
                }
            }
        }
    }

    /// Maps list types based on annotations
    fn map_list_type(
        &self,
        inner: &PythonType,
        annotations: &TranspilationAnnotations,
    ) -> RustType {
        let inner_rust = self.map_type_with_annotations(inner, annotations);

        match annotations.ownership_model {
            OwnershipModel::Borrowed => RustType::Reference {
                lifetime: Some("'a".to_string()),
                mutable: false,
                inner: Box::new(RustType::Vec(Box::new(inner_rust))),
            },
            OwnershipModel::Shared => {
                // For thread-safe shared ownership
                if annotations.thread_safety == depyler_annotations::ThreadSafety::Required {
                    RustType::Custom(format!("Arc<Vec<{}>>", inner_rust.to_rust_string()))
                } else {
                    RustType::Custom(format!("Rc<Vec<{}>>", inner_rust.to_rust_string()))
                }
            }
            OwnershipModel::Owned => RustType::Vec(Box::new(inner_rust)),
        }
    }

    /// Maps dictionary types based on annotations
    fn map_dict_type(
        &self,
        key: &PythonType,
        value: &PythonType,
        annotations: &TranspilationAnnotations,
    ) -> RustType {
        let key_rust = self.map_type_with_annotations(key, annotations);
        let value_rust = self.map_type_with_annotations(value, annotations);

        // Choose hash map implementation based on hash strategy
        let hash_map_type = match annotations.hash_strategy {
            depyler_annotations::HashStrategy::Standard => "HashMap",
            depyler_annotations::HashStrategy::Fnv => "FnvHashMap",
            depyler_annotations::HashStrategy::AHash => "AHashMap",
        };

        let base_type = RustType::Custom(format!(
            "{}<{}, {}>",
            hash_map_type,
            key_rust.to_rust_string(),
            value_rust.to_rust_string()
        ));

        match annotations.ownership_model {
            OwnershipModel::Borrowed => RustType::Reference {
                lifetime: Some("'a".to_string()),
                mutable: false,
                inner: Box::new(base_type),
            },
            OwnershipModel::Shared => {
                if annotations.thread_safety == depyler_annotations::ThreadSafety::Required {
                    RustType::Custom(format!("Arc<{}>", base_type.to_rust_string()))
                } else {
                    RustType::Custom(format!("Rc<{}>", base_type.to_rust_string()))
                }
            }
            OwnershipModel::Owned => base_type,
        }
    }

    /// Maps optional types based on annotations
    fn map_optional_type(
        &self,
        inner: &PythonType,
        annotations: &TranspilationAnnotations,
    ) -> RustType {
        let inner_rust = self.map_type_with_annotations(inner, annotations);

        // If error strategy is Result, use Result instead of Option
        match annotations.error_strategy {
            depyler_annotations::ErrorStrategy::ResultType => RustType::Result(
                Box::new(inner_rust),
                Box::new(RustType::Custom("Error".to_string())),
            ),
            _ => RustType::Option(Box::new(inner_rust)),
        }
    }

    /// Determines if a type should be passed by reference based on annotations
    pub fn needs_reference_with_annotations(
        &self,
        rust_type: &RustType,
        annotations: &TranspilationAnnotations,
    ) -> bool {
        match annotations.ownership_model {
            OwnershipModel::Borrowed => !self.base_mapper.can_copy(rust_type),
            OwnershipModel::Owned => false,
            OwnershipModel::Shared => false, // Shared types are already wrapped
        }
    }

    /// Maps return types considering annotations
    pub fn map_return_type_with_annotations(
        &self,
        py_type: &PythonType,
        annotations: &TranspilationAnnotations,
    ) -> RustType {
        match py_type {
            PythonType::None => match annotations.error_strategy {
                depyler_annotations::ErrorStrategy::ResultType => RustType::Result(
                    Box::new(RustType::Unit),
                    Box::new(RustType::Custom("Error".to_string())),
                ),
                _ => RustType::Unit,
            },
            PythonType::Unknown => RustType::Unit, // Functions without return annotation implicitly return None/()
            _ => self.map_type_with_annotations(py_type, annotations),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_mapper::PrimitiveType;
    use depyler_annotations::*;

    fn create_test_annotations() -> TranspilationAnnotations {
        TranspilationAnnotations::default()
    }

    #[test]
    fn test_string_mapping_with_annotations() {
        let mapper = AnnotationAwareTypeMapper::new();

        // Test always owned
        let mut annotations = create_test_annotations();
        annotations.string_strategy = StringStrategy::AlwaysOwned;
        let rust_type = mapper.map_type_with_annotations(&PythonType::String, &annotations);
        assert_eq!(rust_type, RustType::String);

        // Test zero copy with borrowing
        annotations.string_strategy = StringStrategy::ZeroCopy;
        annotations.ownership_model = OwnershipModel::Borrowed;
        let rust_type = mapper.map_type_with_annotations(&PythonType::String, &annotations);
        assert_eq!(
            rust_type,
            RustType::Str {
                lifetime: Some("'a".to_string())
            }
        );

        // Test zero copy without borrowing falls back to owned
        annotations.ownership_model = OwnershipModel::Owned;
        let rust_type = mapper.map_type_with_annotations(&PythonType::String, &annotations);
        assert_eq!(rust_type, RustType::String);
    }

    #[test]
    fn test_list_mapping_with_ownership() {
        let mapper = AnnotationAwareTypeMapper::new();
        let list_type = PythonType::List(Box::new(PythonType::Int));

        // Test owned
        let mut annotations = create_test_annotations();
        annotations.ownership_model = OwnershipModel::Owned;
        let rust_type = mapper.map_type_with_annotations(&list_type, &annotations);
        assert_eq!(
            rust_type,
            RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)))
        );

        // Test borrowed
        annotations.ownership_model = OwnershipModel::Borrowed;
        let rust_type = mapper.map_type_with_annotations(&list_type, &annotations);
        match rust_type {
            RustType::Reference {
                lifetime,
                mutable,
                inner,
            } => {
                assert_eq!(lifetime, Some("'a".to_string()));
                assert!(!mutable);
                assert_eq!(
                    *inner,
                    RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)))
                );
            }
            _ => panic!("Expected reference type"),
        }

        // Test shared with thread safety
        annotations.ownership_model = OwnershipModel::Shared;
        annotations.thread_safety = ThreadSafety::Required;
        let rust_type = mapper.map_type_with_annotations(&list_type, &annotations);
        assert_eq!(rust_type, RustType::Custom("Arc<Vec<i32>>".to_string()));
    }

    #[test]
    fn test_dict_mapping_with_hash_strategy() {
        let mapper = AnnotationAwareTypeMapper::new();
        let dict_type = PythonType::Dict(Box::new(PythonType::String), Box::new(PythonType::Int));

        // Test standard HashMap
        let mut annotations = create_test_annotations();
        annotations.hash_strategy = HashStrategy::Standard;
        let rust_type = mapper.map_type_with_annotations(&dict_type, &annotations);
        assert_eq!(
            rust_type,
            RustType::Custom("HashMap<String, i32>".to_string())
        );

        // Test FnvHashMap
        annotations.hash_strategy = HashStrategy::Fnv;
        let rust_type = mapper.map_type_with_annotations(&dict_type, &annotations);
        assert_eq!(
            rust_type,
            RustType::Custom("FnvHashMap<String, i32>".to_string())
        );

        // Test AHashMap
        annotations.hash_strategy = HashStrategy::AHash;
        let rust_type = mapper.map_type_with_annotations(&dict_type, &annotations);
        assert_eq!(
            rust_type,
            RustType::Custom("AHashMap<String, i32>".to_string())
        );
    }

    #[test]
    fn test_optional_mapping_with_error_strategy() {
        let mapper = AnnotationAwareTypeMapper::new();
        let optional_type = PythonType::Optional(Box::new(PythonType::String));

        // Test default (Option)
        let mut annotations = create_test_annotations();
        let rust_type = mapper.map_type_with_annotations(&optional_type, &annotations);
        assert_eq!(rust_type, RustType::Option(Box::new(RustType::String)));

        // Test Result type
        annotations.error_strategy = ErrorStrategy::ResultType;
        let rust_type = mapper.map_type_with_annotations(&optional_type, &annotations);
        assert_eq!(
            rust_type,
            RustType::Result(
                Box::new(RustType::String),
                Box::new(RustType::Custom("Error".to_string())),
            )
        );
    }

    #[test]
    fn test_return_type_mapping() {
        let mapper = AnnotationAwareTypeMapper::new();

        // Test None return with default strategy
        let mut annotations = create_test_annotations();
        let rust_type = mapper.map_return_type_with_annotations(&PythonType::None, &annotations);
        assert_eq!(rust_type, RustType::Unit);

        // Test None return with Result strategy
        annotations.error_strategy = ErrorStrategy::ResultType;
        let rust_type = mapper.map_return_type_with_annotations(&PythonType::None, &annotations);
        assert_eq!(
            rust_type,
            RustType::Result(
                Box::new(RustType::Unit),
                Box::new(RustType::Custom("Error".to_string())),
            )
        );
    }
}
