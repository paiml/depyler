//! Query: High-level API for type lookups.
//!
//! Provides a convenient interface for querying the type database
//! with support for fuzzy matching and caching.

use crate::{KnowledgeError, Result, TypeDatabase, TypeFact, TypeFactKind};
use std::collections::HashMap;
use std::path::Path;

/// Query interface for the type database.
pub struct TypeQuery {
    /// The underlying database
    db: TypeDatabase,
    /// In-memory cache for fast lookups
    cache: HashMap<String, TypeFact>,
    /// Whether the cache has been populated
    cache_populated: bool,
}

impl TypeQuery {
    /// Create a new TypeQuery from a database path.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = TypeDatabase::new(path)?;
        Ok(Self {
            db,
            cache: HashMap::new(),
            cache_populated: false,
        })
    }

    /// Load all facts into the in-memory cache for O(1) lookups.
    pub fn warm_cache(&mut self) -> Result<()> {
        if self.cache_populated {
            return Ok(());
        }

        let facts = self.db.read_all()?;
        for fact in facts {
            let key = format!("{}.{}", fact.module, fact.symbol);
            self.cache.insert(key, fact);
        }

        self.cache_populated = true;
        Ok(())
    }

    /// Look up a function signature by fully qualified name.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sig = query.find_signature("requests", "get")?;
    /// // Returns: "(url: str, **kwargs) -> Response"
    /// ```
    pub fn find_signature(&mut self, module: &str, symbol: &str) -> Result<String> {
        self.warm_cache()?;

        let key = format!("{module}.{symbol}");
        self.cache
            .get(&key)
            .map(|f| f.signature.clone())
            .ok_or_else(|| KnowledgeError::SymbolNotFound {
                module: module.to_string(),
                symbol: symbol.to_string(),
            })
    }

    /// Look up a return type by fully qualified name.
    pub fn find_return_type(&mut self, module: &str, symbol: &str) -> Result<String> {
        self.warm_cache()?;

        let key = format!("{module}.{symbol}");
        self.cache
            .get(&key)
            .map(|f| f.return_type.clone())
            .ok_or_else(|| KnowledgeError::SymbolNotFound {
                module: module.to_string(),
                symbol: symbol.to_string(),
            })
    }

    /// Look up a complete TypeFact by fully qualified name.
    pub fn find_fact(&mut self, module: &str, symbol: &str) -> Result<TypeFact> {
        self.warm_cache()?;

        let key = format!("{module}.{symbol}");
        self.cache
            .get(&key)
            .cloned()
            .ok_or_else(|| KnowledgeError::SymbolNotFound {
                module: module.to_string(),
                symbol: symbol.to_string(),
            })
    }

    /// Find all functions in a module.
    pub fn find_functions(&mut self, module: &str) -> Result<Vec<TypeFact>> {
        self.warm_cache()?;

        Ok(self
            .cache
            .values()
            .filter(|f| f.module == module && f.kind == TypeFactKind::Function)
            .cloned()
            .collect())
    }

    /// Find all classes in a module.
    pub fn find_classes(&mut self, module: &str) -> Result<Vec<TypeFact>> {
        self.warm_cache()?;

        Ok(self
            .cache
            .values()
            .filter(|f| f.module == module && f.kind == TypeFactKind::Class)
            .cloned()
            .collect())
    }

    /// Find all methods of a class.
    pub fn find_methods(&mut self, module: &str, class_name: &str) -> Result<Vec<TypeFact>> {
        self.warm_cache()?;

        let prefix = format!("{class_name}.");
        Ok(self
            .cache
            .values()
            .filter(|f| f.module == module && f.kind == TypeFactKind::Method && f.symbol.starts_with(&prefix))
            .cloned()
            .collect())
    }

    /// Check if a symbol exists in the database.
    pub fn has_symbol(&mut self, module: &str, symbol: &str) -> bool {
        self.warm_cache().is_ok() && self.cache.contains_key(&format!("{module}.{symbol}"))
    }

    /// Get the total number of facts in the database.
    pub fn count(&mut self) -> usize {
        if self.warm_cache().is_ok() {
            self.cache.len()
        } else {
            0
        }
    }

    /// Search for symbols matching a pattern.
    pub fn search(&mut self, pattern: &str) -> Result<Vec<TypeFact>> {
        self.warm_cache()?;

        let pattern_lower = pattern.to_lowercase();
        Ok(self
            .cache
            .values()
            .filter(|f| {
                f.symbol.to_lowercase().contains(&pattern_lower)
                    || f.module.to_lowercase().contains(&pattern_lower)
            })
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_db() -> (TempDir, TypeQuery) {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test.parquet");
        let db = TypeDatabase::new(&db_path).unwrap();

        let facts = vec![
            TypeFact::function("requests", "get", "(url: str, **kwargs) -> Response", "Response"),
            TypeFact::function("requests", "post", "(url: str, data: dict) -> Response", "Response"),
            TypeFact::class("requests.models", "Response"),
            TypeFact::method("requests.models", "Response", "json", "(self) -> dict", "dict"),
            TypeFact::method("requests.models", "Response", "text", "(self) -> str", "str"),
        ];

        db.write(&facts).unwrap();

        let query = TypeQuery::new(&db_path).unwrap();
        (temp, query)
    }

    #[test]
    fn test_find_signature() {
        let (_temp, mut query) = setup_test_db();

        let sig = query.find_signature("requests", "get").unwrap();
        assert!(sig.contains("url: str"));
        assert!(sig.contains("**kwargs"));
    }

    #[test]
    fn test_find_return_type() {
        let (_temp, mut query) = setup_test_db();

        let ret = query.find_return_type("requests", "get").unwrap();
        assert_eq!(ret, "Response");
    }

    #[test]
    fn test_find_methods() {
        let (_temp, mut query) = setup_test_db();

        let methods = query.find_methods("requests.models", "Response").unwrap();
        assert_eq!(methods.len(), 2);

        let method_names: Vec<_> = methods.iter().map(|m| m.symbol.as_str()).collect();
        assert!(method_names.contains(&"Response.json"));
        assert!(method_names.contains(&"Response.text"));
    }

    #[test]
    fn test_has_symbol() {
        let (_temp, mut query) = setup_test_db();

        assert!(query.has_symbol("requests", "get"));
        assert!(!query.has_symbol("requests", "put")); // Not in test data
    }

    #[test]
    fn test_search() {
        let (_temp, mut query) = setup_test_db();

        let results = query.search("json").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].symbol, "Response.json");
    }

    #[test]
    fn test_symbol_not_found() {
        let (_temp, mut query) = setup_test_db();

        let result = query.find_signature("unknown", "function");
        assert!(result.is_err());
        assert!(matches!(result, Err(KnowledgeError::SymbolNotFound { .. })));
    }
}
