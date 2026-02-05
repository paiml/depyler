//! Sovereign Type Database for Python Library Type Extraction
//!
//! This crate provides a "Type Truth Database" for Python libraries, enabling
//! the Depyler transpiler to **know** types instead of **guessing** them.
//!
//! # Architecture (The Sovereign Stack)
//!
//! 1. **Harvester**: Uses `uv pip install --target` for deterministic package fetching
//! 2. **Extractor**: Uses `rustpython_parser` for `.pyi` stub parsing
//! 3. **Database**: Uses Apache Parquet via `arrow` crate for efficient queries
//!
//! # Peer-Reviewed Foundation
//!
//! - PEP 484 (van Rossum, Lehtosalo, 2014): Type Hints
//! - PEP 561 (Smith, 2017): Stub Distribution (.pyi format)
//! - PEP 585 (Langa, 2019): Generic Syntax
//! - Apache Parquet Spec (2013): Columnar storage format
//!
//! # Example
//!
//! ```ignore
//! use depyler_knowledge::{Harvester, Extractor, TypeDatabase};
//!
//! // Harvest the requests package
//! let harvest = Harvester::new("/tmp/harvest")?.fetch("requests")?;
//!
//! // Extract type facts from .pyi stubs
//! let facts = Extractor::new().extract_all(&harvest)?;
//!
//! // Store in Parquet database
//! let db = TypeDatabase::new("types.parquet")?;
//! db.write(&facts)?;
//!
//! // Query: Get signature for requests.get
//! let sig = db.find_signature("requests", "get");
//! assert!(sig.unwrap().contains("url: str"));
//! ```

#[cfg(feature = "parquet-storage")]
pub mod database;
pub mod error;
pub mod extractor;
pub mod harvester;
#[cfg(feature = "parquet-storage")]
pub mod query;

#[cfg(feature = "parquet-storage")]
pub use database::TypeDatabase;
pub use error::{KnowledgeError, Result};
pub use extractor::Extractor;
pub use harvester::{HarvestResult, Harvester};
#[cfg(feature = "parquet-storage")]
pub use query::TypeQuery;

use serde::{Deserialize, Serialize};

/// The kind of symbol extracted from Python stubs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeFactKind {
    /// A function (top-level or module-level)
    Function,
    /// A class definition
    Class,
    /// A method within a class
    Method,
    /// A class or module attribute
    Attribute,
}

impl std::fmt::Display for TypeFactKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function => write!(f, "function"),
            Self::Class => write!(f, "class"),
            Self::Method => write!(f, "method"),
            Self::Attribute => write!(f, "attribute"),
        }
    }
}

impl std::str::FromStr for TypeFactKind {
    type Err = KnowledgeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "function" => Ok(Self::Function),
            "class" => Ok(Self::Class),
            "method" => Ok(Self::Method),
            "attribute" => Ok(Self::Attribute),
            _ => Err(KnowledgeError::InvalidKind(s.to_string())),
        }
    }
}

/// A single type fact extracted from Python stubs.
///
/// This is the core data structure of the Sovereign Type Database.
/// Each fact represents a symbol (function, class, method, attribute)
/// with its full type signature.
///
/// # Schema Rationale
///
/// - `module`: Fully qualified module path (e.g., "requests.api")
/// - `symbol`: Symbol name (e.g., "get")
/// - `kind`: Discriminant for symbol type
/// - `signature`: Full signature string for display/debugging
/// - `return_type`: Parsed return type for codegen integration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeFact {
    /// Fully qualified module path (e.g., "requests.api")
    pub module: String,
    /// Symbol name (e.g., "get")
    pub symbol: String,
    /// The kind of symbol (function, class, method, attribute)
    pub kind: TypeFactKind,
    /// Full signature string (e.g., "(url: str, params: dict = None) -> Response")
    pub signature: String,
    /// Return type for functions/methods (e.g., "requests.models.Response")
    pub return_type: String,
}

impl TypeFact {
    /// Create a new TypeFact for a function.
    pub fn function(module: &str, symbol: &str, signature: &str, return_type: &str) -> Self {
        Self {
            module: module.to_string(),
            symbol: symbol.to_string(),
            kind: TypeFactKind::Function,
            signature: signature.to_string(),
            return_type: return_type.to_string(),
        }
    }

    /// Create a new TypeFact for a class.
    pub fn class(module: &str, symbol: &str) -> Self {
        Self {
            module: module.to_string(),
            symbol: symbol.to_string(),
            kind: TypeFactKind::Class,
            signature: String::new(),
            return_type: format!("{module}.{symbol}"),
        }
    }

    /// Create a new TypeFact for a method.
    pub fn method(
        module: &str,
        class: &str,
        method: &str,
        signature: &str,
        return_type: &str,
    ) -> Self {
        Self {
            module: module.to_string(),
            symbol: format!("{class}.{method}"),
            kind: TypeFactKind::Method,
            signature: signature.to_string(),
            return_type: return_type.to_string(),
        }
    }

    /// Get the fully qualified name of this symbol.
    pub fn fqn(&self) -> String {
        format!("{}.{}", self.module, self.symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_fact_function() {
        let fact = TypeFact::function(
            "requests",
            "get",
            "(url: str, **kwargs) -> Response",
            "requests.models.Response",
        );
        assert_eq!(fact.module, "requests");
        assert_eq!(fact.symbol, "get");
        assert_eq!(fact.kind, TypeFactKind::Function);
        assert_eq!(fact.fqn(), "requests.get");
    }

    #[test]
    fn test_type_fact_class() {
        let fact = TypeFact::class("requests.models", "Response");
        assert_eq!(fact.kind, TypeFactKind::Class);
        assert_eq!(fact.return_type, "requests.models.Response");
    }

    #[test]
    fn test_type_fact_method() {
        let fact = TypeFact::method(
            "requests.models",
            "Response",
            "json",
            "(self) -> dict",
            "dict",
        );
        assert_eq!(fact.symbol, "Response.json");
        assert_eq!(fact.kind, TypeFactKind::Method);
    }

    #[test]
    fn test_type_fact_kind_display() {
        assert_eq!(TypeFactKind::Function.to_string(), "function");
        assert_eq!(TypeFactKind::Class.to_string(), "class");
        assert_eq!(TypeFactKind::Method.to_string(), "method");
        assert_eq!(TypeFactKind::Attribute.to_string(), "attribute");
    }

    #[test]
    fn test_type_fact_kind_from_str() {
        assert_eq!(
            "function".parse::<TypeFactKind>().unwrap(),
            TypeFactKind::Function
        );
        assert_eq!(
            "class".parse::<TypeFactKind>().unwrap(),
            TypeFactKind::Class
        );
        assert!("invalid".parse::<TypeFactKind>().is_err());
    }

    #[test]
    fn test_type_fact_kind_from_str_method() {
        assert_eq!(
            "method".parse::<TypeFactKind>().unwrap(),
            TypeFactKind::Method
        );
    }

    #[test]
    fn test_type_fact_kind_from_str_attribute() {
        assert_eq!(
            "attribute".parse::<TypeFactKind>().unwrap(),
            TypeFactKind::Attribute
        );
    }

    #[test]
    fn test_type_fact_kind_from_str_invalid_returns_error() {
        let result = "unknown_kind".parse::<TypeFactKind>();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("unknown_kind"));
    }

    #[test]
    fn test_type_fact_kind_from_str_empty_string() {
        let result = "".parse::<TypeFactKind>();
        assert!(result.is_err());
    }

    #[test]
    fn test_type_fact_kind_from_str_case_sensitive() {
        // "Function" with capital F should fail
        let result = "Function".parse::<TypeFactKind>();
        assert!(result.is_err());
    }

    #[test]
    fn test_type_fact_kind_clone_and_eq() {
        let kind = TypeFactKind::Function;
        let cloned = kind.clone();
        assert_eq!(kind, cloned);
    }

    #[test]
    fn test_type_fact_kind_debug() {
        let kind = TypeFactKind::Function;
        let debug_str = format!("{kind:?}");
        assert_eq!(debug_str, "Function");
    }

    #[test]
    fn test_type_fact_function_fqn() {
        let fact = TypeFact::function(
            "os.path",
            "join",
            "(path: str, *paths: str) -> str",
            "str",
        );
        assert_eq!(fact.fqn(), "os.path.join");
    }

    #[test]
    fn test_type_fact_class_return_type_format() {
        let fact = TypeFact::class("collections", "OrderedDict");
        assert_eq!(fact.return_type, "collections.OrderedDict");
        assert_eq!(fact.signature, "");
    }

    #[test]
    fn test_type_fact_method_symbol_format() {
        let fact = TypeFact::method(
            "http.client",
            "HTTPConnection",
            "request",
            "(self, method: str, url: str) -> None",
            "None",
        );
        assert_eq!(fact.symbol, "HTTPConnection.request");
        assert_eq!(fact.fqn(), "http.client.HTTPConnection.request");
    }

    #[test]
    fn test_type_fact_clone() {
        let original = TypeFact::function("mod", "func", "(x: int) -> int", "int");
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_type_fact_serialization_json() {
        let fact = TypeFact::function("math", "sqrt", "(x: float) -> float", "float");
        let json = serde_json::to_string(&fact).unwrap();
        let deserialized: TypeFact = serde_json::from_str(&json).unwrap();
        assert_eq!(fact, deserialized);
    }

    #[test]
    fn test_type_fact_kind_serialization_json() {
        let kind = TypeFactKind::Method;
        let json = serde_json::to_string(&kind).unwrap();
        let deserialized: TypeFactKind = serde_json::from_str(&json).unwrap();
        assert_eq!(kind, deserialized);
    }

    #[test]
    fn test_type_fact_with_empty_strings() {
        let fact = TypeFact::function("", "", "", "");
        assert_eq!(fact.module, "");
        assert_eq!(fact.symbol, "");
        assert_eq!(fact.fqn(), ".");
    }

    #[test]
    fn test_type_fact_with_unicode_symbol() {
        let fact = TypeFact::function("mymod", "calc_\u{03c0}", "(n: int) -> float", "float");
        assert_eq!(fact.symbol, "calc_\u{03c0}");
        assert_eq!(fact.fqn(), "mymod.calc_\u{03c0}");
    }

    #[test]
    fn test_type_fact_method_class_with_dots() {
        let fact = TypeFact::method(
            "pkg.submod",
            "MyClass",
            "do_thing",
            "(self) -> None",
            "None",
        );
        assert_eq!(fact.fqn(), "pkg.submod.MyClass.do_thing");
    }

    #[test]
    fn test_type_fact_kind_display_roundtrip() {
        let kinds = [
            TypeFactKind::Function,
            TypeFactKind::Class,
            TypeFactKind::Method,
            TypeFactKind::Attribute,
        ];
        for kind in &kinds {
            let display_str = kind.to_string();
            let parsed: TypeFactKind = display_str.parse().unwrap();
            assert_eq!(*kind, parsed);
        }
    }
}
